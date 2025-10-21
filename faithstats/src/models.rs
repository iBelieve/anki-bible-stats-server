use serde::Serialize;
use tabled::Tabled;
use utoipa::ToSchema;

/// Combined faith statistics for a single day
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct FaithDayStats {
    /// Date in YYYY-MM-DD format
    pub date: String,

    // Anki Bible memorization stats
    /// Anki study time in minutes
    pub anki_minutes: f64,
    /// Number of passages that matured on this day
    pub anki_matured_passages: i64,
    /// Number of passages that were lost on this day
    pub anki_lost_passages: i64,
    /// Cumulative count of mature passages at end of day
    pub anki_cumulative_passages: i64,

    // KOReader Bible reading stats
    /// Bible reading time in minutes
    pub reading_minutes: f64,

    // Prayer stats (future)
    /// Prayer time in minutes
    pub prayer_minutes: f64,
}

impl FaithDayStats {
    /// Total minutes across all faith activities for this day
    pub fn total_minutes(&self) -> f64 {
        self.anki_minutes + self.reading_minutes + self.prayer_minutes
    }
}

/// Display wrapper for FaithDayStats for CLI table output
#[derive(Debug, Clone, Tabled)]
pub struct FaithDayStatsDisplay {
    #[tabled(rename = "Date")]
    pub date: String,

    #[tabled(rename = "Anki (min)")]
    pub anki_minutes: String,

    #[tabled(rename = "Reading (min)")]
    pub reading_minutes: String,

    #[tabled(rename = "Prayer (min)")]
    pub prayer_minutes: String,

    #[tabled(rename = "Total (min)")]
    pub total_minutes: String,
}

impl From<&FaithDayStats> for FaithDayStatsDisplay {
    fn from(stats: &FaithDayStats) -> Self {
        Self {
            date: stats.date.clone(),
            anki_minutes: format!("{:.1}", stats.anki_minutes),
            reading_minutes: format!("{:.1}", stats.reading_minutes),
            prayer_minutes: format!("{:.1}", stats.prayer_minutes),
            total_minutes: format!("{:.1}", stats.total_minutes()),
        }
    }
}

/// Summary statistics for faith activities over a period
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct FaithDailySummary {
    // Anki stats
    pub anki_total_minutes: f64,
    pub anki_total_hours: f64,
    pub anki_average_minutes_per_day: f64,
    pub anki_days_studied: usize,
    pub anki_total_matured_passages: i64,
    pub anki_total_lost_passages: i64,
    pub anki_net_progress: i64,

    // Reading stats
    pub reading_total_minutes: f64,
    pub reading_total_hours: f64,
    pub reading_average_minutes_per_day: f64,
    pub reading_days_studied: usize,

    // Prayer stats
    pub prayer_total_minutes: f64,
    pub prayer_total_hours: f64,
    pub prayer_average_minutes_per_day: f64,
    pub prayer_days_studied: usize,

    // Combined stats
    pub total_minutes: f64,
    pub total_hours: f64,
    pub average_minutes_per_day: f64,
    pub total_days: usize,
    pub days_with_any_activity: usize,
}

impl FaithDailySummary {
    pub fn from_faith_daily_stats(days: &[FaithDayStats]) -> Self {
        let anki_total: f64 = days.iter().map(|d| d.anki_minutes).sum();
        let reading_total: f64 = days.iter().map(|d| d.reading_minutes).sum();
        let prayer_total: f64 = days.iter().map(|d| d.prayer_minutes).sum();
        let combined_total = anki_total + reading_total + prayer_total;

        let anki_days = days.iter().filter(|d| d.anki_minutes > 0.0).count();
        let reading_days = days.iter().filter(|d| d.reading_minutes > 0.0).count();
        let prayer_days = days.iter().filter(|d| d.prayer_minutes > 0.0).count();
        let any_activity_days = days.iter().filter(|d| d.total_minutes() > 0.0).count();

        let total_days = days.len();
        let anki_avg = anki_total / total_days as f64;
        let reading_avg = reading_total / total_days as f64;
        let prayer_avg = prayer_total / total_days as f64;
        let combined_avg = combined_total / total_days as f64;

        let anki_matured: i64 = days.iter().map(|d| d.anki_matured_passages).sum();
        let anki_lost: i64 = days.iter().map(|d| d.anki_lost_passages).sum();

        Self {
            anki_total_minutes: anki_total,
            anki_total_hours: anki_total / 60.0,
            anki_average_minutes_per_day: anki_avg,
            anki_days_studied: anki_days,
            anki_total_matured_passages: anki_matured,
            anki_total_lost_passages: anki_lost,
            anki_net_progress: anki_matured - anki_lost,

            reading_total_minutes: reading_total,
            reading_total_hours: reading_total / 60.0,
            reading_average_minutes_per_day: reading_avg,
            reading_days_studied: reading_days,

            prayer_total_minutes: prayer_total,
            prayer_total_hours: prayer_total / 60.0,
            prayer_average_minutes_per_day: prayer_avg,
            prayer_days_studied: prayer_days,

            total_minutes: combined_total,
            total_hours: combined_total / 60.0,
            average_minutes_per_day: combined_avg,
            total_days,
            days_with_any_activity: any_activity_days,
        }
    }
}

/// Faith statistics for multiple days with summary
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct FaithDailyStats {
    pub days: Vec<FaithDayStats>,
    pub summary: FaithDailySummary,
}

impl FaithDailyStats {
    pub fn new(days: Vec<FaithDayStats>) -> Self {
        let summary = FaithDailySummary::from_faith_daily_stats(&days);
        Self { days, summary }
    }
}
