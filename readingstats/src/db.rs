use anyhow::{Context, Result};
use rusqlite::{Connection, OpenFlags};
use statsutils::get_day_boundaries;

use crate::models::DayStats;

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
    let mut results = Vec::new();

    // Query each day individually (from oldest to newest)
    for day_offset in (0..30).rev() {
        let (day_start_ms, day_end_ms, date_str) = get_day_boundaries(day_offset)?;

        // Convert milliseconds to seconds for KOReader database (uses Unix seconds)
        let day_start_sec = day_start_ms / 1000;
        let day_end_sec = day_end_ms / 1000;

        // Query reading time for books matching Bible or Treasury of Daily Prayer
        let query = r#"
            SELECT COALESCE(SUM(psd.duration), 0) as total_seconds
            FROM page_stat_data psd
            JOIN book b ON b.id = psd.id_book
            WHERE (b.title LIKE '%Bible%' OR b.title LIKE 'Treasury of Daily Prayer%')
                AND psd.start_time >= ?1
                AND psd.start_time < ?2
        "#;

        let total_seconds: i64 = conn.query_row(query, [day_start_sec, day_end_sec], |row| {
            row.get(0)
        })?;

        let minutes = total_seconds as f64 / 60.0;

        results.push(DayStats {
            date: date_str,
            minutes,
        });
    }

    Ok(results)
}
