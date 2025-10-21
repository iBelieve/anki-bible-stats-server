pub mod db;
pub mod models;

use anyhow::Result;
use crate::models::DayStats;

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
