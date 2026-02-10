// schema.rs — P-MATRIX Runtime State Record (D1-A Schema v1.0.0)
//
// This module defines the canonical runtime state record structure.
// Field names, types, and constraints follow D1-A_Runtime_State_Schema_v1_0_frozen.md exactly.

use serde::{Deserialize, Serialize};

/// The four evaluation functions that characterize an agent's runtime posture.
/// Each produces a normalized scalar in [0.0, 1.0].
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Functions {
    pub baseline: f64,
    pub norm: f64,
    pub stability: f64,
    pub meta_control: f64,
}

/// A single P-MATRIX runtime state record.
/// Represents the operational posture of an autonomous agent at one instant in time.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct RuntimeStateRecord {
    pub spec_version: String,
    pub schema_version: String,
    pub timestamp: u64,
    pub functions: Functions,
    pub stability_score: f64,
    pub risk_score: f64,
    pub mode: String,
    pub risk_level: String,
}

/// The five discrete operating modes.
pub const MODES: [&str; 5] = ["Optimal", "Normal", "Caution", "Alert", "Halt"];

/// The five risk classification levels.
pub const RISK_LEVELS: [&str; 5] = ["L1", "L2", "L3", "L4", "L5"];

/// Current spec version (D1-A §3.1).
pub const SPEC_VERSION: &str = "pmatrix-3.5";

/// Current schema version (D1-A §3.1).
pub const SCHEMA_VERSION: &str = "1.0.0";
