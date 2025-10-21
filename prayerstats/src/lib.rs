pub mod db;
pub mod models;

use anyhow::Result;

pub use models::{DayStats, WeekStats};

/// Gets the total prayer time for today in minutes
///
/// # Arguments
/// * `db_path` - Path to the Proseuche SQLite database file
///
/// # Returns
/// Total prayer time in minutes for today
///
/// # Errors
/// Returns an error if the database cannot be opened or queried
///
/// # Example
/// ```ignore
/// use prayerstats::get_today_prayer_time;
///
/// let minutes = get_today_prayer_time("/path/to/database.sqlite")?;
/// println!("Prayer time today: {:.1} minutes", minutes);
/// ```
pub fn get_today_prayer_time(db_path: &str) -> Result<f64> {
    let conn = db::open_database(db_path)?;
    db::get_today_prayer_minutes(&conn)
}

/// Gets prayer time for each of the last 30 days
///
/// # Arguments
/// * `db_path` - Path to the Proseuche SQLite database file
///
/// # Returns
/// Vector of DayStats with date and minutes for each of the last 30 days
///
/// # Errors
/// Returns an error if the database cannot be opened or queried
///
/// # Example
/// ```ignore
/// use prayerstats::get_last_30_days_stats;
///
/// let stats = get_last_30_days_stats("/path/to/database.sqlite")?;
/// for day in stats {
///     println!("{}: {:.1} minutes", day.date, day.minutes);
/// }
/// ```
pub fn get_last_30_days_stats(db_path: &str) -> Result<Vec<DayStats>> {
    let conn = db::open_database(db_path)?;
    db::get_last_30_days_stats(&conn)
}

/// Gets prayer time for each of the last 12 weeks
///
/// # Arguments
/// * `db_path` - Path to the Proseuche SQLite database file
///
/// # Returns
/// Vector of WeekStats with week start date and minutes for each of the last 12 weeks
///
/// # Errors
/// Returns an error if the database cannot be opened or queried
///
/// # Example
/// ```ignore
/// use prayerstats::get_last_12_weeks_stats;
///
/// let stats = get_last_12_weeks_stats("/path/to/database.sqlite")?;
/// for week in stats {
///     println!("{}: {:.1} minutes", week.week_start, week.minutes);
/// }
/// ```
pub fn get_last_12_weeks_stats(db_path: &str) -> Result<Vec<WeekStats>> {
    let conn = db::open_database(db_path)?;
    db::get_last_12_weeks_stats(&conn)
}
