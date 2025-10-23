use anyhow::{Context, Result};
use rusqlite::{Connection, OpenFlags};
use statsutils::{DatePeriod, get_today_start_ms, register_date_functions};
use std::collections::HashMap;

use crate::models::{DayStats, WeekStats};

/// Opens a connection to a Proseuche database in read-only mode
///
/// # Arguments
/// * `path` - Path to the Proseuche SQLite database file
///
/// # Returns
/// A read-only SQLite connection to the database
///
/// # Database Schema
/// The Proseuche database contains prayer_sessions table with:
/// - started_at: datetime when prayer session started
/// - ended_at: datetime when prayer session ended
/// - duration_minutes: computed column with session duration in minutes
///
/// # Example
/// ```ignore
/// use prayerstats::db::open_database;
///
/// let conn = open_database("/path/to/database.sqlite")?;
/// ```
pub fn open_database(path: &str) -> Result<Connection> {
    let conn = Connection::open_with_flags(
        path,
        OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX,
    )
    .context("Failed to open Proseuche database in read-only mode")?;

    // Register date functions from statsutils
    register_date_functions(&conn)?;

    Ok(conn)
}

/// Gets the total prayer time for today in minutes
pub fn get_today_prayer_minutes(conn: &Connection) -> Result<f64> {
    let today_start_ms = get_today_start_ms()?;
    let today_start_sec = today_start_ms / 1000;

    let query = r#"
        SELECT COALESCE(SUM(duration_minutes), 0) as total_minutes
        FROM prayer_sessions
        WHERE started_at IS NOT NULL
            AND ended_at IS NOT NULL
            AND CAST(strftime('%s', started_at) AS INTEGER) >= ?1
    "#;

    let total_minutes: f64 = conn.query_row(query, [today_start_sec], |row| row.get(0))?;

    Ok(total_minutes)
}

/// Gets prayer time for each of the last 30 days
///
/// # Arguments
/// * `conn` - Database connection to Proseuche database
///
/// # Returns
/// Vector of DayStats with date and minutes for each of the last 30 days
pub fn get_last_30_days_stats(conn: &Connection) -> Result<Vec<DayStats>> {
    // Get the period data for the last 30 days
    let period = DatePeriod::last_30_days()?;

    // Convert milliseconds to seconds for SQL query (strftime works with seconds)
    let start_sec = period.start_ms / 1000;
    let end_sec = period.end_ms / 1000;

    // Query prayer time grouped by date
    let query = r#"
        SELECT date_str_from_sec(CAST(strftime('%s', started_at) AS INTEGER)) as date,
               SUM(duration_minutes) as total_minutes
        FROM prayer_sessions
        WHERE started_at IS NOT NULL
            AND ended_at IS NOT NULL
            AND CAST(strftime('%s', started_at) AS INTEGER) >= ?1
            AND CAST(strftime('%s', started_at) AS INTEGER) < ?2
        GROUP BY date_str_from_sec(CAST(strftime('%s', started_at) AS INTEGER))
    "#;

    let mut stmt = conn.prepare(query)?;
    let prayer_results = stmt
        .query_map([start_sec, end_sec], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, f64>(1)?))
        })?
        .collect::<Result<HashMap<String, f64>, _>>()?;

    let results = period.build_results(prayer_results, |date, total_minutes| DayStats {
        date,
        minutes: total_minutes,
    });

    Ok(results)
}

/// Gets prayer time for each of the last 12 weeks
pub fn get_last_12_weeks_stats(conn: &Connection) -> Result<Vec<WeekStats>> {
    // Get the period data for the last 12 weeks
    let period = DatePeriod::last_12_weeks()?;

    // Convert milliseconds to seconds for SQL query
    let start_sec = period.start_ms / 1000;
    let end_sec = period.end_ms / 1000;

    // Query prayer time grouped by week
    let query = r#"
        SELECT week_str_from_sec(CAST(strftime('%s', started_at) AS INTEGER)) as week,
               SUM(duration_minutes) as total_minutes
        FROM prayer_sessions
        WHERE started_at IS NOT NULL
            AND ended_at IS NOT NULL
            AND CAST(strftime('%s', started_at) AS INTEGER) >= ?1
            AND CAST(strftime('%s', started_at) AS INTEGER) < ?2
        GROUP BY week_str_from_sec(CAST(strftime('%s', started_at) AS INTEGER))
    "#;

    let mut stmt = conn.prepare(query)?;
    let prayer_results = stmt
        .query_map([start_sec, end_sec], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, f64>(1)?))
        })?
        .collect::<Result<HashMap<String, f64>, _>>()?;

    let results = period.build_results(prayer_results, |week_start, total_minutes| WeekStats {
        week_start,
        minutes: total_minutes,
    });

    Ok(results)
}
