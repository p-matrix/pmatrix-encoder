// mode.rs — 5-Mode Partition Mapping
//
// Implements the risk_score → mode → risk_level mapping
// as defined in D1-A §3.4 and D1-B §4.
//
// Boundary convention: lower-inclusive, upper-exclusive for L1–L4.
// L5 is closed at both ends: [0.8, 1.0].

/// Maps a risk_score to the corresponding operating mode string.
///
/// Returns None if risk_score is outside [0.0, 1.0].
pub fn demo_partition_map(risk_score: f64) -> Option<&'static str> {
    if risk_score < 0.0 || risk_score > 1.0 || risk_score.is_nan() {
        return None;
    }
    Some(match () {
        _ if risk_score < 0.2 => "Optimal",
        _ if risk_score < 0.4 => "Normal",
        _ if risk_score < 0.6 => "Caution",
        _ if risk_score < 0.8 => "Alert",
        _ => "Halt",
    })
}

/// Maps a mode string to the corresponding risk_level string.
///
/// Returns None if the mode is not one of the five defined modes.
pub fn mode_to_risk_level(mode: &str) -> Option<&'static str> {
    match mode {
        "Optimal" => Some("L1"),
        "Normal" => Some("L2"),
        "Caution" => Some("L3"),
        "Alert" => Some("L4"),
        "Halt" => Some("L5"),
        _ => None,
    }
}
