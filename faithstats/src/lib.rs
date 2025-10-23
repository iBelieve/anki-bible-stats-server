pub mod models;

use anyhow::Result;

use crate::models::{
    FaithDailyStats, FaithDayStats, FaithTodayStats, FaithWeekStats, FaithWeeklyStats,
};

/// Gets unified faith statistics for the last 30 days, combining Anki Bible memorization,
/// KOReader Bible reading, and prayer time data.
///
/// # Arguments
/// * `anki_db_path` - Path to the Anki collection.anki2 database file
/// * `koreader_db_path` - Path to the KOReader statistics.sqlite3 database file
/// * `proseuche_db_path` - Path to the Proseuche database.sqlite file
///
/// # Returns
/// FaithDailyStats containing daily breakdown and summary statistics
///
/// # Errors
/// Returns an error if any database is unavailable or cannot be queried
///
/// # Example
/// ```ignore
/// use faithstats::get_faith_daily_stats;
///
/// let stats = get_faith_daily_stats(
///     "/path/to/collection.anki2",
///     "/path/to/statistics.sqlite3",
///     "/path/to/database.sqlite"
/// )?;
/// println!("Total faith time: {:.2} hours", stats.summary.total_hours);
/// ```
pub fn get_faith_daily_stats(
    anki_db_path: &str,
    koreader_db_path: &str,
    proseuche_db_path: &str,
) -> Result<FaithDailyStats> {
    // Query all databases - will return error if any is unavailable
    let anki_stats = ankistats::get_last_30_days_stats(anki_db_path)?;
    let reading_stats = readingstats::get_last_30_days_stats(koreader_db_path)?;
    let prayer_stats = prayerstats::get_last_30_days_stats(proseuche_db_path)?;

    // All functions return the same 30 dates in the same order (guaranteed by DatePeriod),
    // so we can simply zip them together
    let merged_days: Vec<FaithDayStats> = anki_stats
        .into_iter()
        .zip(reading_stats)
        .zip(prayer_stats)
        .map(|((anki_day, reading_day), prayer_day)| FaithDayStats {
            date: anki_day.date,
            anki_minutes: anki_day.minutes,
            anki_matured_passages: anki_day.matured_passages,
            anki_lost_passages: anki_day.lost_passages,
            anki_cumulative_passages: anki_day.cumulative_passages,
            reading_minutes: reading_day.minutes,
            prayer_minutes: prayer_day.minutes,
        })
        .collect();

    Ok(FaithDailyStats::new(merged_days))
}

/// Gets unified faith statistics for today, combining Anki Bible memorization,
/// KOReader Bible reading, and prayer time data.
///
/// # Arguments
/// * `anki_db_path` - Path to the Anki collection.anki2 database file
/// * `koreader_db_path` - Path to the KOReader statistics.sqlite3 database file
/// * `proseuche_db_path` - Path to the Proseuche database.sqlite file
///
/// # Returns
/// FaithTodayStats containing today's combined statistics
///
/// # Errors
/// Returns an error if any database is unavailable or cannot be queried
///
/// # Example
/// ```ignore
/// use faithstats::get_faith_today_stats;
///
/// let stats = get_faith_today_stats(
///     "/path/to/collection.anki2",
///     "/path/to/statistics.sqlite3",
///     "/path/to/database.sqlite"
/// )?;
/// println!("Total faith time today: {:.2} hours", stats.total_hours);
/// ```
pub fn get_faith_today_stats(
    anki_db_path: &str,
    koreader_db_path: &str,
    proseuche_db_path: &str,
) -> Result<FaithTodayStats> {
    // Query all databases - will return error if any is unavailable
    let anki_minutes = ankistats::get_today_study_time(anki_db_path)?;
    let reading_minutes = readingstats::get_today_reading_time(koreader_db_path)?;
    let prayer_minutes = prayerstats::get_today_prayer_time(proseuche_db_path)?;

    Ok(FaithTodayStats::new(
        anki_minutes,
        reading_minutes,
        prayer_minutes,
    ))
}

/// Gets unified faith statistics for the last 12 weeks, combining Anki Bible memorization,
/// KOReader Bible reading, Arc church attendance, and prayer time data.
///
/// # Arguments
/// * `anki_db_path` - Path to the Anki collection.anki2 database file
/// * `koreader_db_path` - Path to the KOReader statistics.sqlite3 database file
/// * `arcstats_export_path` - Path to the Arc Timeline export directory
/// * `proseuche_db_path` - Path to the Proseuche database.sqlite file
///
/// # Returns
/// FaithWeeklyStats containing weekly breakdown and summary statistics
///
/// # Errors
/// Returns an error if any database/export is unavailable or cannot be queried
///
/// # Example
/// ```ignore
/// use faithstats::get_faith_weekly_stats;
///
/// let stats = get_faith_weekly_stats(
///     "/path/to/collection.anki2",
///     "/path/to/statistics.sqlite3",
///     "/path/to/arc/export",
///     "/path/to/database.sqlite"
/// )?;
/// println!("Total faith time: {:.2} hours", stats.summary.total_hours);
/// ```
pub fn get_faith_weekly_stats(
    anki_db_path: &str,
    koreader_db_path: &str,
    arcstats_export_path: &str,
    proseuche_db_path: &str,
) -> Result<FaithWeeklyStats> {
    // Query all databases - will return error if any is unavailable
    let anki_stats = ankistats::get_last_12_weeks_stats(anki_db_path)?;
    let reading_stats = readingstats::get_last_12_weeks_stats(koreader_db_path)?;
    let church_stats = arcstats::get_last_12_weeks_stats(arcstats_export_path)?;
    let prayer_stats = prayerstats::get_last_12_weeks_stats(proseuche_db_path)?;

    // All functions return the same 12 weeks in the same order (guaranteed by DatePeriod),
    // so we can simply zip them together
    let merged_weeks: Vec<FaithWeekStats> = anki_stats
        .into_iter()
        .zip(reading_stats)
        .zip(church_stats)
        .zip(prayer_stats)
        .map(|(((anki_week, reading_week), church_week), prayer_week)| FaithWeekStats {
            week_start: anki_week.week_start,
            anki_minutes: anki_week.minutes,
            anki_matured_passages: anki_week.matured_passages,
            anki_lost_passages: anki_week.lost_passages,
            anki_cumulative_passages: anki_week.cumulative_passages,
            reading_minutes: reading_week.minutes,
            at_church_minutes: church_week.minutes,
            prayer_minutes: prayer_week.minutes,
        })
        .collect();

    Ok(FaithWeeklyStats::new(merged_weeks))
}
