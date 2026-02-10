// lib.rs — P-MATRIX Reference Encoder Public API
//
// Reference encoder for schema conformance. Not an execution engine.

pub mod schema;
pub mod mode;
pub mod invariants;
pub mod demo;

use schema::{Functions, RuntimeStateRecord, SCHEMA_VERSION, SPEC_VERSION};
use mode::{demo_partition_map, mode_to_risk_level};
use demo::{demo_stability_score, demo_risk_score};
use invariants::{validate_all, is_valid, InvariantResult};

use std::time::{SystemTime, UNIX_EPOCH};

/// Emits a demonstration runtime state record from four function values.
///
/// WARNING: Uses demonstration aggregation logic only.
/// Production implementations use proprietary evaluation pipelines.
pub fn emit_demo_record(
    baseline: f64,
    norm: f64,
    stability: f64,
    meta_control: f64,
    timestamp: Option<u64>,
) -> Result<RuntimeStateRecord, String> {
    // Validate input ranges
    for (name, val) in [
        ("baseline", baseline),
        ("norm", norm),
        ("stability", stability),
        ("meta_control", meta_control),
    ] {
        if val.is_nan() || val.is_infinite() {
            return Err(format!("{} is NaN or infinite", name));
        }
        if !(0.0..=1.0).contains(&val) {
            return Err(format!("{} = {} is outside [0.0, 1.0]", name, val));
        }
    }

    let functions = Functions {
        baseline,
        norm,
        stability,
        meta_control,
    };

    let stability_score = demo_stability_score(&functions);
    let risk_score = demo_risk_score(stability_score);

    let mode = demo_partition_map(risk_score)
        .ok_or_else(|| format!("risk_score {} out of range", risk_score))?;
    let risk_level = mode_to_risk_level(mode)
        .ok_or_else(|| format!("unknown mode {}", mode))?;

    let ts = timestamp.unwrap_or_else(|| {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    });

    Ok(RuntimeStateRecord {
        spec_version: SPEC_VERSION.to_string(),
        schema_version: SCHEMA_VERSION.to_string(),
        timestamp: ts,
        functions,
        stability_score,
        risk_score,
        mode: mode.to_string(),
        risk_level: risk_level.to_string(),
    })
}

/// Validates a runtime state record against all 12 invariants (D1-A §4).
///
/// Returns a list of invariant check results.
pub fn validate_record(record: &RuntimeStateRecord) -> Vec<InvariantResult> {
    validate_all(record)
}

/// Returns true if the record satisfies all invariants.
pub fn is_record_valid(record: &RuntimeStateRecord) -> bool {
    is_valid(record)
}
