use anyhow::{Context, Result};
use rusqlite::Connection;

use crate::models::BookStats;

// Anki queue type constants
// See https://github.com/ankitects/anki/blob/76d3237139b3e73b98f5a5b4dfeeeea2f0554644/pylib/anki/consts.py#L22C1-L29
const QUEUE_TYPE_MANUALLY_BURIED: i64 = -3;
const QUEUE_TYPE_SIBLING_BURIED: i64 = -2;
#[allow(dead_code)]
const QUEUE_TYPE_SUSPENDED: i64 = -1;
const QUEUE_TYPE_NEW: i64 = 0;
const QUEUE_TYPE_LRN: i64 = 1;
const QUEUE_TYPE_REV: i64 = 2;
const QUEUE_TYPE_DAY_LEARN_RELEARN: i64 = 3;
#[allow(dead_code)]
const QUEUE_TYPE_PREVIEW: i64 = 4;

/// Unicode unit separator character (used in Anki deck names)
const UNIT_SEPARATOR: char = '\x1F';

/// Opens a connection to an Anki database
pub fn open_database(path: &str) -> Result<Connection> {
    Connection::open(path).context("Failed to open Anki database")
}

/// Looks up the deck ID for "Bible<unit-separator>Verses"
pub fn get_deck_id(conn: &Connection) -> Result<i64> {
    let deck_name = format!("Bible{}Verses", UNIT_SEPARATOR);

    let deck_id: i64 = conn
        .query_row(
            "SELECT id FROM decks WHERE LOWER(name) = LOWER(?1)",
            [&deck_name],
            |row| row.get(0),
        )
        .context(format!("Failed to find deck '{}'", deck_name))?;

    Ok(deck_id)
}

/// Looks up the model ID for the "Bible Verse" note type
pub fn get_model_id(conn: &Connection) -> Result<i64> {
    let model_name = "Bible Verse";

    let model_id: i64 = conn
        .query_row(
            "SELECT id FROM notetypes WHERE LOWER(name) = LOWER(?1)",
            [model_name],
            |row| row.get(0),
        )
        .context(format!("Failed to find note type '{}'", model_name))?;

    Ok(model_id)
}

/// Gets statistics for a specific Bible book
pub fn get_book_stats(
    conn: &Connection,
    book_name: &str,
    deck_id: i64,
    model_id: i64,
) -> Result<BookStats> {
    let search_pattern = format!("{}%", book_name);

    let query = format!(
        r#"
        SELECT
            SUM(CASE WHEN queue IN ({},{},{}) AND ivl >= 21 THEN 1 ELSE 0 END) as mature_count,
            SUM(CASE WHEN queue IN ({},{}) OR
                              (queue IN ({},{},{}) AND ivl < 21) THEN 1 ELSE 0 END) as young_count,
            SUM(CASE WHEN queue={} THEN 1 ELSE 0 END) as unseen_count,
            SUM(CASE WHEN queue<{} THEN 1 ELSE 0 END) as suspended_count
        FROM cards
        JOIN notes ON notes.id = cards.nid
        WHERE ord = 0 AND mid = ?1 AND did = ?2 AND sfld LIKE ?3
        "#,
        QUEUE_TYPE_REV,
        QUEUE_TYPE_SIBLING_BURIED,
        QUEUE_TYPE_MANUALLY_BURIED,
        QUEUE_TYPE_LRN,
        QUEUE_TYPE_DAY_LEARN_RELEARN,
        QUEUE_TYPE_REV,
        QUEUE_TYPE_SIBLING_BURIED,
        QUEUE_TYPE_MANUALLY_BURIED,
        QUEUE_TYPE_NEW,
        QUEUE_TYPE_NEW
    );

    let mut stmt = conn.prepare(&query)?;

    let stats = stmt.query_row(
        rusqlite::params![model_id, deck_id, search_pattern],
        |row| {
            Ok(BookStats::new(
                book_name.to_string(),
                row.get(0).unwrap_or(0),
                row.get(1).unwrap_or(0),
                row.get(2).unwrap_or(0),
                row.get(3).unwrap_or(0),
            ))
        },
    )?;

    Ok(stats)
}
