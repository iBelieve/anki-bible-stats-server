pub mod models;

use anyhow::Result;

use crate::models::{
    FaithDailyStats, FaithDayStats, FaithTodayStats, FaithWeekStats, FaithWeeklyStats,
};

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
            // TODO: Query prayer stats from prayerstats::get_last_30_days_stats() and merge here
            prayer_minutes: 0.0,
        })
        .collect();

    Ok(FaithDailyStats::new(merged_days))
}

/// Gets unified faith statistics for today, combining Anki Bible memorization
/// and KOReader Bible reading data.
///
/// # Arguments
/// * `anki_db_path` - Path to the Anki collection.anki2 database file
/// * `koreader_db_path` - Path to the KOReader statistics.sqlite3 database file
///
/// # Returns
/// FaithTodayStats containing today's combined statistics
///
/// # Errors
/// Returns an error if either database is unavailable or cannot be queried
///
/// # Example
/// ```ignore
/// use faithstats::get_faith_today_stats;
///
/// let stats = get_faith_today_stats(
///     "/path/to/collection.anki2",
///     "/path/to/statistics.sqlite3"
/// )?;
/// println!("Total faith time today: {:.2} hours", stats.total_hours);
/// ```
pub fn get_faith_today_stats(
    anki_db_path: &str,
    koreader_db_path: &str,
) -> Result<FaithTodayStats> {
    // Query both databases - will return error if either is unavailable
    let anki_minutes = ankistats::get_today_study_time(anki_db_path)?;
    let reading_minutes = readingstats::get_today_reading_time(koreader_db_path)?;
    // TODO: Query prayer stats from prayerstats::get_today_prayer_time() when implemented
    let prayer_minutes = 0.0;

    Ok(FaithTodayStats::new(
        anki_minutes,
        reading_minutes,
        prayer_minutes,
    ))
}

/// Gets unified faith statistics for the last 12 weeks, combining Anki Bible memorization
/// and KOReader Bible reading data.
///
/// # Arguments
/// * `anki_db_path` - Path to the Anki collection.anki2 database file
/// * `koreader_db_path` - Path to the KOReader statistics.sqlite3 database file
///
/// # Returns
/// FaithWeeklyStats containing weekly breakdown and summary statistics
///
/// # Errors
/// Returns an error if either database is unavailable or cannot be queried
///
/// # Example
/// ```ignore
/// use faithstats::get_faith_weekly_stats;
///
/// let stats = get_faith_weekly_stats(
///     "/path/to/collection.anki2",
///     "/path/to/statistics.sqlite3"
/// )?;
/// println!("Total faith time: {:.2} hours", stats.summary.total_hours);
/// ```
pub fn get_faith_weekly_stats(
    anki_db_path: &str,
    koreader_db_path: &str,
) -> Result<FaithWeeklyStats> {
    // Query both databases - will return error if either is unavailable
    let anki_stats = ankistats::get_last_12_weeks_stats(anki_db_path)?;
    let reading_stats = readingstats::get_last_12_weeks_stats(koreader_db_path)?;

    // Both functions return the same 12 weeks in the same order (guaranteed by DatePeriod),
    // so we can simply zip them together
    let merged_weeks: Vec<FaithWeekStats> = anki_stats
        .into_iter()
        .zip(reading_stats)
        .map(|(anki_week, reading_week)| FaithWeekStats {
            week_start: anki_week.week_start,
            anki_minutes: anki_week.minutes,
            anki_matured_passages: anki_week.matured_passages,
            anki_lost_passages: anki_week.lost_passages,
            anki_cumulative_passages: anki_week.cumulative_passages,
            reading_minutes: reading_week.minutes,
            // TODO: Query prayer stats from prayerstats::get_last_12_weeks_stats() and merge here
            prayer_minutes: 0.0,
        })
        .collect();

    Ok(FaithWeeklyStats::new(merged_weeks))
}
