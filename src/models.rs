use tabled::Tabled;

/// Statistics for a single Bible book
#[derive(Debug, Clone, Tabled)]
pub struct BookStats {
    #[tabled(rename = "Book")]
    pub book: String,

    #[tabled(rename = "Mature")]
    pub mature_count: i64,

    #[tabled(rename = "Young")]
    pub young_count: i64,

    #[tabled(rename = "Unseen")]
    pub unseen_count: i64,

    #[tabled(rename = "Suspended")]
    pub suspended_count: i64,
}

impl BookStats {
    pub fn new(book: String, mature: i64, young: i64, unseen: i64, suspended: i64) -> Self {
        Self {
            book,
            mature_count: mature,
            young_count: young,
            unseen_count: unseen,
            suspended_count: suspended,
        }
    }

    pub fn total_cards(&self) -> i64 {
        self.mature_count + self.young_count + self.unseen_count + self.suspended_count
    }
}

/// Aggregated statistics for a collection of books
#[derive(Debug, Clone)]
pub struct AggregateStats {
    pub label: String,
    pub mature_count: i64,
    pub young_count: i64,
    pub unseen_count: i64,
    pub suspended_count: i64,
    pub book_stats: Vec<BookStats>,
}

impl AggregateStats {
    pub fn new(label: String) -> Self {
        Self {
            label,
            mature_count: 0,
            young_count: 0,
            unseen_count: 0,
            suspended_count: 0,
            book_stats: Vec::new(),
        }
    }

    pub fn add_book(&mut self, stats: BookStats) {
        self.mature_count += stats.mature_count;
        self.young_count += stats.young_count;
        self.unseen_count += stats.unseen_count;
        self.suspended_count += stats.suspended_count;
        self.book_stats.push(stats);
    }

    pub fn total_cards(&self) -> i64 {
        self.mature_count + self.young_count + self.unseen_count + self.suspended_count
    }
}

/// Complete Bible statistics report
#[derive(Debug)]
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

    pub fn total_mature(&self) -> i64 {
        self.old_testament.mature_count + self.new_testament.mature_count
    }

    pub fn total_young(&self) -> i64 {
        self.old_testament.young_count + self.new_testament.young_count
    }

    pub fn total_unseen(&self) -> i64 {
        self.old_testament.unseen_count + self.new_testament.unseen_count
    }

    pub fn total_suspended(&self) -> i64 {
        self.old_testament.suspended_count + self.new_testament.suspended_count
    }

    pub fn total_cards(&self) -> i64 {
        self.old_testament.total_cards() + self.new_testament.total_cards()
    }
}

impl Default for BibleStats {
    fn default() -> Self {
        Self::new()
    }
}

/// Study time statistics for a single day
#[derive(Debug, Clone)]
pub struct DailyStudyTime {
    pub date: String,
    pub minutes: f64,
}

impl DailyStudyTime {
    pub fn new(date: String, minutes: f64) -> Self {
        Self { date, minutes }
    }
}
