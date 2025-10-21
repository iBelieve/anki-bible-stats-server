use anyhow::{Context, Result};
use rusqlite::{Connection, OpenFlags};
use statsutils::{get_today_start_ms, register_date_functions, DatePeriod};
use std::collections::HashMap;

use crate::models::{DayStats, WeekStats};

/// Opens a connection to a KOReader statistics database in read-only mode
///
/// # Arguments
/// * `path` - Path to the KOReader statistics.sqlite3 file
///
/// # Returns
/// A read-only SQLite connection to the database
///
/// # Database Schema
/// The KOReader statistics database contains the following tables:
/// - `book`: Book metadata (id, title, authors, total_read_time, total_read_pages, etc.)
/// - `page_stat_data`: Raw reading session data (id_book, page, start_time, duration, total_pages)
/// - `page_stat`: Normalized view of reading statistics
/// - `numbers`: Helper table for views
pub fn open_database(path: &str) -> Result<Connection> {
    let conn = Connection::open_with_flags(
        path,
        OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX,
    )
    .context("Failed to open KOReader statistics database in read-only mode")?;

    // Register date functions from statsutils
    register_date_functions(&conn)?;

    Ok(conn)
}

/// Gets reading time for each of the last 30 days for Bible and Treasury of Daily Prayer books
///
/// # Arguments
/// * `conn` - Database connection to KOReader statistics database
///
/// # Returns
/// Vector of DayStats with date and minutes for each of the last 30 days
pub fn get_last_30_days_stats(conn: &Connection) -> Result<Vec<DayStats>> {
    // Get the period data for the last 30 days
    let period = DatePeriod::last_30_days()?;

    // Convert milliseconds to seconds for KOReader database (uses Unix seconds)
    let start_sec = period.start_ms / 1000;
    let end_sec = period.end_ms / 1000;

    // Query reading time grouped by date
    let query = r#"
        SELECT date_str_from_sec(psd.start_time) as date, SUM(psd.duration) as total_seconds
        FROM page_stat_data psd
        JOIN book b ON b.id = psd.id_book
        WHERE (b.title LIKE '%Bible%' OR b.title LIKE 'Treasury of Daily Prayer%')
            AND psd.start_time >= ?1
            AND psd.start_time < ?2
        GROUP BY date_str_from_sec(psd.start_time)
    "#;

    let mut stmt = conn.prepare(query)?;
    let reading_results = stmt
        .query_map([start_sec, end_sec], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))
        })?
        .collect::<Result<HashMap<String, i64>, _>>()?;

    let results = period.build_results(reading_results, |date, total_seconds| DayStats {
        date,
        minutes: total_seconds as f64 / 60.0,
    });

    Ok(results)
}

/// Gets the total reading time for today in minutes
pub fn get_today_reading_minutes(conn: &Connection) -> Result<f64> {
    let today_start_ms = get_today_start_ms()?;
    let today_start_sec = today_start_ms / 1000;

    let query = r#"
        SELECT COALESCE(SUM(psd.duration), 0) as total_seconds
        FROM page_stat_data psd
        JOIN book b ON b.id = psd.id_book
        WHERE (b.title LIKE '%Bible%' OR b.title LIKE 'Treasury of Daily Prayer%')
            AND psd.start_time >= ?1
    "#;

    let total_seconds: i64 = conn.query_row(query, [today_start_sec], |row| row.get(0))?;

    // Convert seconds to minutes
    Ok(total_seconds as f64 / 60.0)
}

/// Gets reading time for each of the last 12 weeks
pub fn get_last_12_weeks_stats(conn: &Connection) -> Result<Vec<WeekStats>> {
    // Get the period data for the last 12 weeks
    let period = DatePeriod::last_12_weeks()?;

    // Convert milliseconds to seconds for KOReader database (uses Unix seconds)
    let start_sec = period.start_ms / 1000;
    let end_sec = period.end_ms / 1000;

    // Query reading time grouped by week
    let query = r#"
        SELECT week_str_from_sec(psd.start_time) as week, SUM(psd.duration) as total_seconds
        FROM page_stat_data psd
        JOIN book b ON b.id = psd.id_book
        WHERE (b.title LIKE '%Bible%' OR b.title LIKE 'Treasury of Daily Prayer%')
            AND psd.start_time >= ?1
            AND psd.start_time < ?2
        GROUP BY week_str_from_sec(psd.start_time)
    "#;

    let mut stmt = conn.prepare(query)?;
    let reading_results = stmt
        .query_map([start_sec, end_sec], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))
        })?
        .collect::<Result<HashMap<String, i64>, _>>()?;

    let results = period.build_results(reading_results, |week_start, total_seconds| WeekStats {
        week_start,
        minutes: total_seconds as f64 / 60.0,
    });

    Ok(results)
}
