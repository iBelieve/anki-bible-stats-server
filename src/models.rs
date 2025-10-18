use serde::Serialize;
use tabled::Tabled;
use utoipa::ToSchema;

/// Statistics for a single Bible book
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct BookStats {
    pub book: String,
    pub mature_passages: i64,
    pub young_passages: i64,
    pub unseen_passages: i64,
    pub suspended_passages: i64,
    pub mature_verses: i64,
    pub young_verses: i64,
    pub unseen_verses: i64,
    pub suspended_verses: i64,
}

/// Display wrapper for BookStats that formats passages and verses as "P / V"
#[derive(Debug, Clone, Tabled)]
pub struct BookStatsDisplay {
    #[tabled(rename = "Book")]
    pub book: String,

    #[tabled(rename = "Mature")]
    pub mature: String,

    #[tabled(rename = "Young")]
    pub young: String,

    #[tabled(rename = "Unseen")]
    pub unseen: String,

    #[tabled(rename = "Suspended")]
    pub suspended: String,
}

impl From<&BookStats> for BookStatsDisplay {
    fn from(stats: &BookStats) -> Self {
        Self {
            book: stats.book.clone(),
            mature: format!("{} / {}", stats.mature_passages, stats.mature_verses),
            young: format!("{} / {}", stats.young_passages, stats.young_verses),
            unseen: format!("{} / {}", stats.unseen_passages, stats.unseen_verses),
            suspended: format!("{} / {}", stats.suspended_passages, stats.suspended_verses),
        }
    }
}

impl BookStats {
    pub fn total_passages(&self) -> i64 {
        self.mature_passages + self.young_passages + self.unseen_passages + self.suspended_passages
    }

    pub fn total_verses(&self) -> i64 {
        self.mature_verses + self.young_verses + self.unseen_verses + self.suspended_verses
    }
}

/// Aggregated statistics for a collection of books
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct AggregateStats {
    pub label: String,
    pub mature_passages: i64,
    pub young_passages: i64,
    pub unseen_passages: i64,
    pub suspended_passages: i64,
    pub mature_verses: i64,
    pub young_verses: i64,
    pub unseen_verses: i64,
    pub suspended_verses: i64,
    pub book_stats: Vec<BookStats>,
}

impl AggregateStats {
    pub fn new(label: String) -> Self {
        Self {
            label,
            mature_passages: 0,
            young_passages: 0,
            unseen_passages: 0,
            suspended_passages: 0,
            mature_verses: 0,
            young_verses: 0,
            unseen_verses: 0,
            suspended_verses: 0,
            book_stats: Vec::new(),
        }
    }

    pub fn add_book(&mut self, stats: BookStats) {
        self.mature_passages += stats.mature_passages;
        self.young_passages += stats.young_passages;
        self.unseen_passages += stats.unseen_passages;
        self.suspended_passages += stats.suspended_passages;
        self.mature_verses += stats.mature_verses;
        self.young_verses += stats.young_verses;
        self.unseen_verses += stats.unseen_verses;
        self.suspended_verses += stats.suspended_verses;
        self.book_stats.push(stats);
    }

    pub fn total_passages(&self) -> i64 {
        self.mature_passages + self.young_passages + self.unseen_passages + self.suspended_passages
    }

    pub fn total_verses(&self) -> i64 {
        self.mature_verses + self.young_verses + self.unseen_verses + self.suspended_verses
    }
}

/// Complete Bible statistics report
#[derive(Debug, Serialize, ToSchema)]
pub struct BibleStats {
    pub old_testament: AggregateStats,
    pub new_testament: AggregateStats,
}

impl BibleStats {
    pub fn new() -> Self {
        Self {
            old_testament: AggregateStats::new("Old Testament".to_string()),
            new_testament: AggregateStats::new("New Testament".to_string()),
        }
    }

    pub fn total_mature_passages(&self) -> i64 {
        self.old_testament.mature_passages + self.new_testament.mature_passages
    }

    pub fn total_young_passages(&self) -> i64 {
        self.old_testament.young_passages + self.new_testament.young_passages
    }

    pub fn total_unseen_passages(&self) -> i64 {
        self.old_testament.unseen_passages + self.new_testament.unseen_passages
    }

    pub fn total_suspended_passages(&self) -> i64 {
        self.old_testament.suspended_passages + self.new_testament.suspended_passages
    }

    pub fn total_passages(&self) -> i64 {
        self.old_testament.total_passages() + self.new_testament.total_passages()
    }

    pub fn total_mature_verses(&self) -> i64 {
        self.old_testament.mature_verses + self.new_testament.mature_verses
    }

    pub fn total_young_verses(&self) -> i64 {
        self.old_testament.young_verses + self.new_testament.young_verses
    }

    pub fn total_unseen_verses(&self) -> i64 {
        self.old_testament.unseen_verses + self.new_testament.unseen_verses
    }

    pub fn total_suspended_verses(&self) -> i64 {
        self.old_testament.suspended_verses + self.new_testament.suspended_verses
    }

    pub fn total_verses(&self) -> i64 {
        self.old_testament.total_verses() + self.new_testament.total_verses()
    }
}

impl Default for BibleStats {
    fn default() -> Self {
        Self::new()
    }
}

/// Study time and progress statistics for a single day
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct DayStats {
    pub date: String,
    pub minutes: f64,
    pub matured_passages: i64,
    pub lost_passages: i64,
    pub cumulative_passages: i64,
}

/// Health check response
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct HealthCheck {
    pub status: String,
    pub service: String,
}

impl HealthCheck {
    pub fn new() -> Self {
        Self {
            status: "ok".to_string(),
            service: "anki-bible-stats".to_string(),
        }
    }
}

impl Default for HealthCheck {
    fn default() -> Self {
        Self::new()
    }
}

/// Today's study time response
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct TodayStats {
    pub minutes: f64,
    pub hours: f64,
}

impl TodayStats {
    pub fn new(minutes: f64) -> Self {
        Self {
            minutes,
            hours: minutes / 60.0,
        }
    }
}

/// Summary statistics for daily study time and progress
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct DailySummary {
    pub total_minutes: f64,
    pub total_hours: f64,
    pub average_minutes_per_day: f64,
    pub average_hours_per_day: f64,
    pub days_studied: usize,
    pub total_days: usize,
    pub total_matured_passages: i64,
    pub total_lost_passages: i64,
    pub net_progress: i64,
}

impl DailySummary {
    pub fn from_daily_stats(daily: &[DayStats]) -> Self {
        let total_minutes: f64 = daily.iter().map(|d| d.minutes).sum();
        let avg_minutes = total_minutes / daily.len() as f64;
        let days_studied = daily.iter().filter(|d| d.minutes > 0.0).count();
        let total_matured: i64 = daily.iter().map(|d| d.matured_passages).sum();
        let total_lost: i64 = daily.iter().map(|d| d.lost_passages).sum();

        Self {
            total_minutes,
            total_hours: total_minutes / 60.0,
            average_minutes_per_day: avg_minutes,
            average_hours_per_day: avg_minutes / 60.0,
            days_studied,
            total_days: daily.len(),
            total_matured_passages: total_matured,
            total_lost_passages: total_lost,
            net_progress: total_matured - total_lost,
        }
    }
}

/// Daily study time response with summary
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct DailyStats {
    pub days: Vec<DayStats>,
    pub summary: DailySummary,
}

impl DailyStats {
    pub fn new(days: Vec<DayStats>) -> Self {
        let summary = DailySummary::from_daily_stats(&days);
        Self { days, summary }
    }
}

/// Error response
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ErrorResponse {
    pub error: String,
}

impl ErrorResponse {
    pub fn new(error: String) -> Self {
        Self { error }
    }
}
