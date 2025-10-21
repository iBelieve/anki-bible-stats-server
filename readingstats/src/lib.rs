pub mod db;
pub mod models;

use crate::models::{DayStats, WeekStats};
use anyhow::Result;

/// Gets reading time for each of the last 30 days for Bible and Treasury of Daily Prayer books
///
/// # Arguments
/// * `db_path` - Path to the KOReader statistics.sqlite3 database file
///
/// # Returns
/// Vector of DayStats with date and minutes for each of the last 30 days
///
/// # Example
/// ```ignore
/// use readingstats::get_last_30_days_stats;
///
/// let daily_stats = get_last_30_days_stats("/path/to/statistics.sqlite3")?;
/// for day in daily_stats {
///     println!("{}: {:.2} minutes", day.date, day.minutes);
/// }
/// ```
pub fn get_last_30_days_stats(db_path: &str) -> Result<Vec<DayStats>> {
    let conn = db::open_database(db_path)?;
    db::get_last_30_days_stats(&conn)
}

/// Gets the total reading time for today in minutes
///
/// # Arguments
/// * `db_path` - Path to the KOReader statistics.sqlite3 database file
///
/// # Returns
/// Total reading time in minutes for today
///
/// # Example
/// ```ignore
/// use readingstats::get_today_reading_time;
///
/// let minutes = get_today_reading_time("/path/to/statistics.sqlite3")?;
/// println!("Today's reading time: {:.2} minutes", minutes);
/// ```
pub fn get_today_reading_time(db_path: &str) -> Result<f64> {
    let conn = db::open_database(db_path)?;
    db::get_today_reading_minutes(&conn)
}

/// Gets reading time for each of the last 12 weeks for Bible and Treasury of Daily Prayer books
///
/// # Arguments
/// * `db_path` - Path to the KOReader statistics.sqlite3 database file
///
/// # Returns
/// Vector of WeekStats with week_start and minutes for each of the last 12 weeks
///
/// # Example
/// ```ignore
/// use readingstats::get_last_12_weeks_stats;
///
/// let weekly_stats = get_last_12_weeks_stats("/path/to/statistics.sqlite3")?;
/// for week in weekly_stats {
///     println!("{}: {:.2} minutes", week.week_start, week.minutes);
/// }
/// ```
pub fn get_last_12_weeks_stats(db_path: &str) -> Result<Vec<WeekStats>> {
    let conn = db::open_database(db_path)?;
    db::get_last_12_weeks_stats(&conn)
}
