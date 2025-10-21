pub mod models;

use anyhow::Result;
use std::collections::HashMap;

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
pub fn get_faith_daily_stats(anki_db_path: &str, koreader_db_path: &str) -> Result<FaithDailyStats> {
    // Query both databases - will return error if either is unavailable
    let anki_stats = ankistats::get_last_30_days_stats(anki_db_path)?;
    let reading_stats = readingstats::get_last_30_days_stats(koreader_db_path)?;

    // Create lookup maps by date for efficient merging
    let anki_map: HashMap<String, &ankistats::models::DayStats> = anki_stats
        .iter()
        .map(|day| (day.date.clone(), day))
        .collect();

    let reading_map: HashMap<String, &readingstats::models::DayStats> = reading_stats
        .iter()
        .map(|day| (day.date.clone(), day))
        .collect();

    // Get all unique dates from both sources and sort them
    let mut all_dates: Vec<String> = anki_stats
        .iter()
        .map(|d| d.date.clone())
        .chain(reading_stats.iter().map(|d| d.date.clone()))
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();
    all_dates.sort();

    // Merge stats for each date
    let mut merged_days = Vec::new();
    for date in all_dates {
        let anki_day = anki_map.get(&date);
        let reading_day = reading_map.get(&date);

        let faith_day = FaithDayStats {
            date: date.clone(),
            anki_minutes: anki_day.map(|d| d.minutes).unwrap_or(0.0),
            anki_matured_passages: anki_day.map(|d| d.matured_passages).unwrap_or(0),
            anki_lost_passages: anki_day.map(|d| d.lost_passages).unwrap_or(0),
            anki_cumulative_passages: anki_day.map(|d| d.cumulative_passages).unwrap_or(0),
            reading_minutes: reading_day.map(|d| d.minutes).unwrap_or(0.0),
            prayer_minutes: 0.0, // Future enhancement
        };

        merged_days.push(faith_day);
    }

    Ok(FaithDailyStats::new(merged_days))
}
