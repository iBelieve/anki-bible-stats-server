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
