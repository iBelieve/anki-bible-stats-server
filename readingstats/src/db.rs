use anyhow::{Context, Result};
use rusqlite::{Connection, OpenFlags};

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
