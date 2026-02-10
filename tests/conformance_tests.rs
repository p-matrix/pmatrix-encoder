// conformance_tests.rs — D1-A Invariant Conformance Tests
//
// Tests cover:
// - All 12 invariants (positive and negative cases)
// - Boundary values: 0.0, 0.2, 0.4, 0.6, 0.8, 1.0
// - Edge cases: NaN, out-of-range, empty strings
// - D1-A §5 Example record verification
// - Stream-level INV-T1 validation

use pmatrix_encoder::schema::*;
use pmatrix_encoder::mode::*;
use pmatrix_encoder::invariants::*;
use pmatrix_encoder::demo::*;
use pmatrix_encoder::{emit_demo_record, is_record_valid};

// ============================================================
// Helper
// ============================================================

fn make_record(
    baseline: f64, norm: f64, stability: f64, meta_control: f64,
    stability_score: f64, risk_score: f64,
    mode: &str, risk_level: &str,
    timestamp: u64,
) -> RuntimeStateRecord {
    RuntimeStateRecord {
        spec_version: SPEC_VERSION.to_string(),
        schema_version: SCHEMA_VERSION.to_string(),
        timestamp,
        functions: Functions { baseline, norm, stability, meta_control },
        stability_score,
        risk_score,
        mode: mode.to_string(),
        risk_level: risk_level.to_string(),
    }
}

// ============================================================
// D1-A §5 Example Record (verbatim from spec)
// ============================================================

#[test]
fn test_spec_example_record() {
    let record = make_record(
        0.25, 0.70, 0.30, 0.20,
        0.58, 0.42,
        "Caution", "L3",
        1707500000,
    );
    assert!(is_record_valid(&record));
}

// ============================================================
// Boundary Value Tests — 5-mode partition edges
// ============================================================

#[test]
fn test_boundary_optimal_lower() {
    // risk_score = 0.0 → Optimal / L1
    assert_eq!(demo_partition_map(0.0), Some("Optimal"));
    let record = make_record(1.0, 1.0, 1.0, 1.0, 1.0, 0.0, "Optimal", "L1", 1000);
    assert!(is_record_valid(&record));
}

#[test]
fn test_boundary_optimal_upper() {
    // risk_score = 0.19999... → Optimal / L1
    assert_eq!(demo_partition_map(0.19999999), Some("Optimal"));
}

#[test]
fn test_boundary_normal_lower() {
    // risk_score = 0.2 → Normal / L2
    assert_eq!(demo_partition_map(0.2), Some("Normal"));
    let record = make_record(0.5, 0.5, 0.5, 0.5, 0.8, 0.2, "Normal", "L2", 1000);
    assert!(is_record_valid(&record));
}

#[test]
fn test_boundary_caution_lower() {
    // risk_score = 0.4 → Caution / L3
    assert_eq!(demo_partition_map(0.4), Some("Caution"));
}

#[test]
fn test_boundary_alert_lower() {
    // risk_score = 0.6 → Alert / L4
    assert_eq!(demo_partition_map(0.6), Some("Alert"));
}

#[test]
fn test_boundary_halt_lower() {
    // risk_score = 0.8 → Halt / L5
    assert_eq!(demo_partition_map(0.8), Some("Halt"));
}

#[test]
fn test_boundary_halt_upper() {
    // risk_score = 1.0 → Halt / L5 (closed interval)
    assert_eq!(demo_partition_map(1.0), Some("Halt"));
    let record = make_record(0.0, 0.0, 0.0, 0.0, 0.0, 1.0, "Halt", "L5", 1000);
    assert!(is_record_valid(&record));
}

// ============================================================
// INV-R1: Function values in [0.0, 1.0]
// ============================================================

#[test]
fn test_inv_r1_valid_zeros() {
    let record = make_record(0.0, 0.0, 0.0, 0.0, 0.0, 1.0, "Halt", "L5", 1000);
    let results = validate_all(&record);
    assert!(results.iter().find(|r| r.id == "INV-R1").unwrap().passed);
}

#[test]
fn test_inv_r1_valid_ones() {
    let record = make_record(1.0, 1.0, 1.0, 1.0, 1.0, 0.0, "Optimal", "L1", 1000);
    let results = validate_all(&record);
    assert!(results.iter().find(|r| r.id == "INV-R1").unwrap().passed);
}

#[test]
fn test_inv_r1_fail_negative() {
    let record = make_record(-0.1, 0.5, 0.5, 0.5, 0.5, 0.5, "Caution", "L3", 1000);
    let results = validate_all(&record);
    assert!(!results.iter().find(|r| r.id == "INV-R1").unwrap().passed);
}

#[test]
fn test_inv_r1_fail_above_one() {
    let record = make_record(0.5, 1.1, 0.5, 0.5, 0.5, 0.5, "Caution", "L3", 1000);
    let results = validate_all(&record);
    assert!(!results.iter().find(|r| r.id == "INV-R1").unwrap().passed);
}

#[test]
fn test_inv_r1_fail_nan() {
    let record = make_record(f64::NAN, 0.5, 0.5, 0.5, 0.5, 0.5, "Caution", "L3", 1000);
    let results = validate_all(&record);
    assert!(!results.iter().find(|r| r.id == "INV-R1").unwrap().passed);
}

// ============================================================
// INV-R2, INV-R3: Derived score ranges
// ============================================================

#[test]
fn test_inv_r2_fail_out_of_range() {
    let record = make_record(0.5, 0.5, 0.5, 0.5, 1.5, 0.5, "Caution", "L3", 1000);
    let results = validate_all(&record);
    assert!(!results.iter().find(|r| r.id == "INV-R2").unwrap().passed);
}

#[test]
fn test_inv_r3_fail_nan() {
    let record = make_record(0.5, 0.5, 0.5, 0.5, 0.5, f64::NAN, "Caution", "L3", 1000);
    let results = validate_all(&record);
    assert!(!results.iter().find(|r| r.id == "INV-R3").unwrap().passed);
}

// ============================================================
// INV-R4: Timestamp positive
// ============================================================

#[test]
fn test_inv_r4_fail_zero() {
    let record = make_record(0.5, 0.5, 0.5, 0.5, 0.5, 0.5, "Caution", "L3", 0);
    let results = validate_all(&record);
    assert!(!results.iter().find(|r| r.id == "INV-R4").unwrap().passed);
}

// ============================================================
// INV-C1: mode = threshold_map(risk_score)
// ============================================================

#[test]
fn test_inv_c1_fail_wrong_mode() {
    // risk_score 0.35 should be "Normal", not "Caution" (D1-B §5.2 example)
    let record = make_record(0.5, 0.5, 0.5, 0.5, 0.65, 0.35, "Caution", "L3", 1000);
    let results = validate_all(&record);
    assert!(!results.iter().find(|r| r.id == "INV-C1").unwrap().passed);
}

// ============================================================
// INV-C2: risk_level = level_map(mode)
// ============================================================

#[test]
fn test_inv_c2_fail_wrong_level() {
    let record = make_record(0.5, 0.5, 0.5, 0.5, 0.5, 0.5, "Caution", "L4", 1000);
    let results = validate_all(&record);
    assert!(!results.iter().find(|r| r.id == "INV-C2").unwrap().passed);
}

// ============================================================
// INV-S3: spec_version must be "pmatrix-3.5"
// ============================================================

#[test]
fn test_inv_s3_fail_wrong_version() {
    let mut record = make_record(0.5, 0.5, 0.5, 0.5, 0.5, 0.5, "Caution", "L3", 1000);
    record.spec_version = "pmatrix-4.0".to_string();
    let results = validate_all(&record);
    assert!(!results.iter().find(|r| r.id == "INV-S3").unwrap().passed);
}

// ============================================================
// INV-S4: schema_version valid semver
// ============================================================

#[test]
fn test_inv_s4_fail_invalid_semver() {
    let mut record = make_record(0.5, 0.5, 0.5, 0.5, 0.5, 0.5, "Caution", "L3", 1000);
    record.schema_version = "1.0".to_string();
    let results = validate_all(&record);
    assert!(!results.iter().find(|r| r.id == "INV-S4").unwrap().passed);
}

// ============================================================
// INV-T1: Stream-level temporal monotonicity
// ============================================================

#[test]
fn test_inv_t1_valid_stream() {
    let records = vec![
        make_record(0.5, 0.5, 0.5, 0.5, 0.5, 0.5, "Caution", "L3", 1000),
        make_record(0.5, 0.5, 0.5, 0.5, 0.5, 0.5, "Caution", "L3", 1000), // equal OK
        make_record(0.5, 0.5, 0.5, 0.5, 0.5, 0.5, "Caution", "L3", 1001),
    ];
    assert_eq!(validate_stream_t1(&records), None);
}

#[test]
fn test_inv_t1_fail_decreasing() {
    let records = vec![
        make_record(0.5, 0.5, 0.5, 0.5, 0.5, 0.5, "Caution", "L3", 1001),
        make_record(0.5, 0.5, 0.5, 0.5, 0.5, 0.5, "Caution", "L3", 1000), // violation
    ];
    assert_eq!(validate_stream_t1(&records), Some(1));
}

// ============================================================
// emit_demo_record integration
// ============================================================

#[test]
fn test_emit_demo_record_valid() {
    let record = emit_demo_record(0.25, 0.70, 0.30, 0.20, Some(1707500000)).unwrap();
    assert!(is_record_valid(&record));
    assert_eq!(record.spec_version, "pmatrix-3.5");
    assert_eq!(record.schema_version, "1.0.0");
}

#[test]
fn test_emit_demo_record_all_zeros() {
    let record = emit_demo_record(0.0, 0.0, 0.0, 0.0, Some(1000)).unwrap();
    assert!(is_record_valid(&record));
    assert_eq!(record.mode, "Halt");
    assert_eq!(record.risk_level, "L5");
}

#[test]
fn test_emit_demo_record_all_ones() {
    let record = emit_demo_record(1.0, 1.0, 1.0, 1.0, Some(1000)).unwrap();
    assert!(is_record_valid(&record));
    assert_eq!(record.mode, "Optimal");
    assert_eq!(record.risk_level, "L1");
}

#[test]
fn test_emit_demo_record_reject_nan() {
    assert!(emit_demo_record(f64::NAN, 0.5, 0.5, 0.5, Some(1000)).is_err());
}

#[test]
fn test_emit_demo_record_reject_out_of_range() {
    assert!(emit_demo_record(0.5, 1.5, 0.5, 0.5, Some(1000)).is_err());
    assert!(emit_demo_record(-0.1, 0.5, 0.5, 0.5, Some(1000)).is_err());
}

#[test]
fn test_emit_demo_record_reject_infinite() {
    assert!(emit_demo_record(f64::INFINITY, 0.5, 0.5, 0.5, Some(1000)).is_err());
    assert!(emit_demo_record(f64::NEG_INFINITY, 0.5, 0.5, 0.5, Some(1000)).is_err());
}

// ============================================================
// Demo score logic (simple arithmetic — NOT kernel logic)
// ============================================================

#[test]
fn test_demo_stability_score() {
    let f = Functions {
        baseline: 0.25,
        norm: 0.70,
        stability: 0.30,
        meta_control: 0.20,
    };
    let s = demo_stability_score(&f);
    assert!((s - 0.3625).abs() < 1e-10);
}

#[test]
fn test_demo_risk_score() {
    assert!((demo_risk_score(0.3625) - 0.6375).abs() < 1e-10);
}

// ============================================================
// demo_partition_map edge cases
// ============================================================

#[test]
fn test_partition_map_rejects_negative() {
    assert_eq!(demo_partition_map(-0.001), None);
}

#[test]
fn test_partition_map_rejects_above_one() {
    assert_eq!(demo_partition_map(1.001), None);
}

#[test]
fn test_partition_map_rejects_nan() {
    assert_eq!(demo_partition_map(f64::NAN), None);
}

// ============================================================
// mode_to_risk_level
// ============================================================

#[test]
fn test_mode_to_risk_level_all() {
    assert_eq!(mode_to_risk_level("Optimal"), Some("L1"));
    assert_eq!(mode_to_risk_level("Normal"), Some("L2"));
    assert_eq!(mode_to_risk_level("Caution"), Some("L3"));
    assert_eq!(mode_to_risk_level("Alert"), Some("L4"));
    assert_eq!(mode_to_risk_level("Halt"), Some("L5"));
    assert_eq!(mode_to_risk_level("Unknown"), None);
}

// ============================================================
// JSON round-trip
// ============================================================

#[test]
fn test_json_roundtrip() {
    let record = emit_demo_record(0.50, 0.60, 0.40, 0.30, Some(1707500000)).unwrap();
    let json = serde_json::to_string(&record).unwrap();
    let parsed: RuntimeStateRecord = serde_json::from_str(&json).unwrap();
    assert_eq!(record, parsed);
    assert!(is_record_valid(&parsed));
}

#[test]
fn test_json_reject_extra_fields() {
    let json = r#"{
        "spec_version": "pmatrix-3.5",
        "schema_version": "1.0.0",
        "timestamp": 1707500000,
        "functions": {
            "baseline": 0.25,
            "norm": 0.70,
            "stability": 0.30,
            "meta_control": 0.20
        },
        "stability_score": 0.58,
        "risk_score": 0.42,
        "mode": "Caution",
        "risk_level": "L3",
        "extra_field": "should_fail"
    }"#;
    // serde with deny_unknown_fields should reject this
    let result: Result<RuntimeStateRecord, _> = serde_json::from_str(json);
    assert!(result.is_err());
}
