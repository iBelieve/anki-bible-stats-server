use serde::Serialize;
use tabled::Tabled;

/// Statistics for a single Bible book
#[derive(Debug, Clone, Serialize)]
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
#[derive(Debug, Clone, Serialize)]
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
#[derive(Debug, Serialize)]
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

/// Study time statistics for a single day
#[derive(Debug, Clone, Serialize)]
pub struct DailyStudyTime {
    pub date: String,
    pub minutes: f64,
}

impl DailyStudyTime {
    pub fn new(date: String, minutes: f64) -> Self {
        Self { date, minutes }
    }
}
