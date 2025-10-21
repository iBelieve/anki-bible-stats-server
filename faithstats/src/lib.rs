pub mod models;

use anyhow::Result;

use crate::models::{FaithDailyStats, FaithDayStats};

/// Gets unified faith statistics for the last 30 days, combining Anki Bible memorization
/// and KOReader Bible reading data.
///
/// # Arguments
/// * `anki_db_path` - Path to the Anki collection.anki2 database file
/// * `koreader_db_path` - Path to the KOReader statistics.sqlite3 database file
///
/// # Returns
/// FaithDailyStats containing daily breakdown and summary statistics
///
/// # Errors
/// Returns an error if either database is unavailable or cannot be queried
///
/// # Example
/// ```ignore
/// use faithstats::get_faith_daily_stats;
///
/// let stats = get_faith_daily_stats(
///     "/path/to/collection.anki2",
///     "/path/to/statistics.sqlite3"
/// )?;
/// println!("Total faith time: {:.2} hours", stats.summary.total_hours);
/// ```
pub fn get_faith_daily_stats(
    anki_db_path: &str,
    koreader_db_path: &str,
) -> Result<FaithDailyStats> {
    // Query both databases - will return error if either is unavailable
    let anki_stats = ankistats::get_last_30_days_stats(anki_db_path)?;
    let reading_stats = readingstats::get_last_30_days_stats(koreader_db_path)?;

    // Both functions return the same 30 dates in the same order (guaranteed by DatePeriod),
    // so we can simply zip them together
    let merged_days: Vec<FaithDayStats> = anki_stats
        .into_iter()
        .zip(reading_stats)
        .map(|(anki_day, reading_day)| FaithDayStats {
            date: anki_day.date,
            anki_minutes: anki_day.minutes,
            anki_matured_passages: anki_day.matured_passages,
            anki_lost_passages: anki_day.lost_passages,
            anki_cumulative_passages: anki_day.cumulative_passages,
            reading_minutes: reading_day.minutes,
            prayer_minutes: 0.0, // Future enhancement
        })
        .collect();

    Ok(FaithDailyStats::new(merged_days))
}
