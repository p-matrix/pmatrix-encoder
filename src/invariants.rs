// invariants.rs — P-MATRIX Runtime State Invariant Validator
//
// Implements all 12 invariants from D1-A §4 / D1-B §5.
// A record that violates any invariant is malformed.

use crate::mode::{demo_partition_map, mode_to_risk_level};
use crate::schema::{RuntimeStateRecord, SPEC_VERSION};

/// Result of validating a single invariant.
#[derive(Debug, Clone)]
pub struct InvariantResult {
    pub id: &'static str,
    pub passed: bool,
    pub detail: String,
}

/// Validates all 12 invariants against a runtime state record.
/// Returns a Vec of results — one per invariant.
pub fn validate_all(record: &RuntimeStateRecord) -> Vec<InvariantResult> {
    vec![
        check_inv_r1(record),
        check_inv_r2(record),
        check_inv_r3(record),
        check_inv_r4(record),
        check_inv_c1(record),
        check_inv_c2(record),
        check_inv_c3(record),
        check_inv_s1(record),
        check_inv_s2(record),
        check_inv_s3(record),
        check_inv_s4(record),
        // INV-T1 is a stream-level invariant; validated separately.
        check_inv_t1_note(),
    ]
}

/// Returns true only if all invariants pass.
pub fn is_valid(record: &RuntimeStateRecord) -> bool {
    validate_all(record).iter().all(|r| r.passed)
}

// --- Range Invariants ---

fn check_inv_r1(r: &RuntimeStateRecord) -> InvariantResult {
    let f = &r.functions;
    let in_range = |v: f64| (0.0..=1.0).contains(&v) && !v.is_nan();
    let ok = in_range(f.baseline) && in_range(f.norm)
        && in_range(f.stability) && in_range(f.meta_control);
    InvariantResult {
        id: "INV-R1",
        passed: ok,
        detail: if ok {
            "All function values in [0.0, 1.0].".into()
        } else {
            format!(
                "Function value(s) out of range: baseline={}, norm={}, stability={}, meta_control={}",
                f.baseline, f.norm, f.stability, f.meta_control
            )
        },
    }
}

fn check_inv_r2(r: &RuntimeStateRecord) -> InvariantResult {
    let ok = (0.0..=1.0).contains(&r.stability_score) && !r.stability_score.is_nan();
    InvariantResult {
        id: "INV-R2",
        passed: ok,
        detail: format!("stability_score={}", r.stability_score),
    }
}

fn check_inv_r3(r: &RuntimeStateRecord) -> InvariantResult {
    let ok = (0.0..=1.0).contains(&r.risk_score) && !r.risk_score.is_nan();
    InvariantResult {
        id: "INV-R3",
        passed: ok,
        detail: format!("risk_score={}", r.risk_score),
    }
}

fn check_inv_r4(r: &RuntimeStateRecord) -> InvariantResult {
    let ok = r.timestamp > 0;
    InvariantResult {
        id: "INV-R4",
        passed: ok,
        detail: format!("timestamp={}", r.timestamp),
    }
}

// --- Consistency Invariants ---

fn check_inv_c1(r: &RuntimeStateRecord) -> InvariantResult {
    let expected = demo_partition_map(r.risk_score);
    let ok = expected.map_or(false, |m| m == r.mode);
    InvariantResult {
        id: "INV-C1",
        passed: ok,
        detail: format!(
            "risk_score={} → expected mode={:?}, actual mode={}",
            r.risk_score, expected, r.mode
        ),
    }
}

fn check_inv_c2(r: &RuntimeStateRecord) -> InvariantResult {
    let expected = mode_to_risk_level(&r.mode);
    let ok = expected.map_or(false, |l| l == r.risk_level);
    InvariantResult {
        id: "INV-C2",
        passed: ok,
        detail: format!(
            "mode={} → expected risk_level={:?}, actual risk_level={}",
            r.mode, expected, r.risk_level
        ),
    }
}

fn check_inv_c3(r: &RuntimeStateRecord) -> InvariantResult {
    // INV-C3: INV-C1 ∧ INV-C2 → risk_level fully determined by risk_score.
    let c1 = check_inv_c1(r).passed;
    let c2 = check_inv_c2(r).passed;
    let ok = c1 && c2;
    InvariantResult {
        id: "INV-C3",
        passed: ok,
        detail: if ok {
            "mode and risk_level are mutually consistent with risk_score.".into()
        } else {
            "Mutual consistency violation: mode/risk_level not determined by risk_score.".into()
        },
    }
}

// --- Structural Invariants ---

fn check_inv_s1(r: &RuntimeStateRecord) -> InvariantResult {
    // All 8 required fields present — guaranteed by struct deserialization,
    // but we verify no empty strings for string fields.
    let ok = !r.spec_version.is_empty()
        && !r.schema_version.is_empty()
        && !r.mode.is_empty()
        && !r.risk_level.is_empty();
    InvariantResult {
        id: "INV-S1",
        passed: ok,
        detail: "All eight required fields present.".into(),
    }
}

fn check_inv_s2(_r: &RuntimeStateRecord) -> InvariantResult {
    // No additional fields — enforced by #[serde(deny_unknown_fields)] at parse time.
    // At validation time on a parsed struct, this is inherently satisfied.
    InvariantResult {
        id: "INV-S2",
        passed: true,
        detail: "No additional fields (enforced by strict deserialization).".into(),
    }
}

fn check_inv_s3(r: &RuntimeStateRecord) -> InvariantResult {
    let ok = r.spec_version == SPEC_VERSION;
    InvariantResult {
        id: "INV-S3",
        passed: ok,
        detail: format!("spec_version={}, expected={}", r.spec_version, SPEC_VERSION),
    }
}

fn check_inv_s4(r: &RuntimeStateRecord) -> InvariantResult {
    // Valid semver: MAJOR.MINOR.PATCH, all numeric.
    let parts: Vec<&str> = r.schema_version.split('.').collect();
    let ok = parts.len() == 3 && parts.iter().all(|p| p.parse::<u32>().is_ok());
    InvariantResult {
        id: "INV-S4",
        passed: ok,
        detail: format!("schema_version={}", r.schema_version),
    }
}

// --- Temporal Invariant ---

fn check_inv_t1_note() -> InvariantResult {
    // INV-T1 requires sequential records from a single emitter.
    // Single-record validation cannot check this; noted as informational.
    InvariantResult {
        id: "INV-T1",
        passed: true,
        detail: "Stream-level invariant. Not checkable on a single record. \
                 Use validate_stream() for sequential validation.".into(),
    }
}

/// Validates INV-T1 across a sequence of records.
/// Returns the index of the first violation, or None if all pass.
pub fn validate_stream_t1(records: &[RuntimeStateRecord]) -> Option<usize> {
    for i in 1..records.len() {
        if records[i].timestamp < records[i - 1].timestamp {
            return Some(i);
        }
    }
    None
}
