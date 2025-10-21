use serde::{Deserialize, Serialize};

/// Statistics for a single day
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayStats {
    /// Date in YYYY-MM-DD format
    pub date: String,
    /// Total prayer time in minutes
    pub minutes: f64,
}

/// Statistics for a single week
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeekStats {
    /// Week start date (Sunday) in YYYY-MM-DD format
    pub week_start: String,
    /// Total prayer time in minutes
    pub minutes: f64,
}
