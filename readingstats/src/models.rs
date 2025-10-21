use serde::Serialize;
use utoipa::ToSchema;

/// Reading time statistics for a single day
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct DayStats {
    /// Date in YYYY-MM-DD format
    pub date: String,
    /// Reading time in minutes
    pub minutes: f64,
}

/// Reading time statistics for a single week
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct WeekStats {
    /// Week start date in YYYY-MM-DD format
    pub week_start: String,
    /// Reading time in minutes
    pub minutes: f64,
}
