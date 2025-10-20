use anyhow::{Context, Result};
use rusqlite::{Connection, OpenFlags};

/// Opens a connection to a prayer app database in read-only mode
///
/// # Arguments
/// * `path` - Path to the prayer app's SQLite database file
///
/// # Returns
/// A read-only SQLite connection to the database
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
    .context("Failed to open prayer app database in read-only mode")?;

    Ok(conn)
}
