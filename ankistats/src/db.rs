use anyhow::{Context, Result};
use rusqlite::{Connection, OpenFlags};
use std::collections::HashMap;
use statsutils::{get_day_boundaries, get_today_start_ms, get_week_boundaries};

use crate::book_name_parser;
use crate::models::{BookStats, DayStats, WeekStats};
use crate::verse_parser;

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

/// Opens a connection to an Anki database in read-only mode
pub fn open_database(path: &str) -> Result<Connection> {
    let conn = Connection::open_with_flags(
        path,
        OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX,
    )
    .context("Failed to open Anki database in read-only mode")?;

    // Register custom SQLite function for counting verses in a reference
    conn.create_scalar_function(
        "count_verses",
        1, // number of arguments
        rusqlite::functions::FunctionFlags::SQLITE_UTF8
            | rusqlite::functions::FunctionFlags::SQLITE_DETERMINISTIC,
        |ctx| {
            let reference = ctx.get::<String>(0)?;
            Ok(verse_parser::count_verses_in_reference(&reference))
        },
    )
    .context("Failed to register count_verses SQLite function")?;

    // Register custom SQLite function for parsing book names from references
    conn.create_scalar_function(
        "parse_book_name",
        1, // number of arguments
        rusqlite::functions::FunctionFlags::SQLITE_UTF8
            | rusqlite::functions::FunctionFlags::SQLITE_DETERMINISTIC,
        |ctx| {
            let reference = ctx.get::<String>(0)?;
            Ok(book_name_parser::parse_book_name(&reference))
        },
    )
    .context("Failed to register parse_book_name SQLite function")?;

    Ok(conn)
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

/// Gets statistics for all Bible books in a single query using GROUP BY
/// Returns a HashMap with book names as keys and BookStats as values
pub fn get_all_books_stats(
    conn: &Connection,
    deck_id: i64,
    model_id: i64,
) -> Result<HashMap<String, BookStats>> {
    let query = format!(
        r#"
        SELECT
            parse_book_name(sfld) as book,
            SUM(CASE WHEN queue IN ({QUEUE_TYPE_REV},{QUEUE_TYPE_SIBLING_BURIED},{QUEUE_TYPE_MANUALLY_BURIED}) AND ivl >= 21 THEN 1 ELSE 0 END) as mature_passages,
            SUM(CASE WHEN queue IN ({QUEUE_TYPE_LRN},{QUEUE_TYPE_DAY_LEARN_RELEARN}) OR
                              (queue IN ({QUEUE_TYPE_REV},{QUEUE_TYPE_SIBLING_BURIED},{QUEUE_TYPE_MANUALLY_BURIED}) AND ivl < 21) THEN 1 ELSE 0 END) as young_passages,
            SUM(CASE WHEN queue={QUEUE_TYPE_NEW} THEN 1 ELSE 0 END) as unseen_passages,
            SUM(CASE WHEN queue<{QUEUE_TYPE_NEW} THEN 1 ELSE 0 END) as suspended_passages,
            SUM(CASE WHEN queue IN ({QUEUE_TYPE_REV},{QUEUE_TYPE_SIBLING_BURIED},{QUEUE_TYPE_MANUALLY_BURIED}) AND ivl >= 21 THEN count_verses(sfld) ELSE 0 END) as mature_verses,
            SUM(CASE WHEN queue IN ({QUEUE_TYPE_LRN},{QUEUE_TYPE_DAY_LEARN_RELEARN}) OR
                              (queue IN ({QUEUE_TYPE_REV},{QUEUE_TYPE_SIBLING_BURIED},{QUEUE_TYPE_MANUALLY_BURIED}) AND ivl < 21) THEN count_verses(sfld) ELSE 0 END) as young_verses,
            SUM(CASE WHEN queue={QUEUE_TYPE_NEW} THEN count_verses(sfld) ELSE 0 END) as unseen_verses,
            SUM(CASE WHEN queue<{QUEUE_TYPE_NEW} THEN count_verses(sfld) ELSE 0 END) as suspended_verses
        FROM cards
        JOIN notes ON notes.id = cards.nid
        WHERE ord = 0 AND mid = ?1 AND did = ?2
        GROUP BY book
        HAVING book IS NOT NULL
        "#
    );

    let mut stmt = conn.prepare(&query)?;

    let books_iter = stmt.query_map(rusqlite::params![model_id, deck_id], |row| {
        let book_name: String = row.get(0)?;
        Ok((
            book_name.clone(),
            BookStats {
                book: book_name,
                mature_passages: row.get(1).unwrap_or(0),
                young_passages: row.get(2).unwrap_or(0),
                unseen_passages: row.get(3).unwrap_or(0),
                suspended_passages: row.get(4).unwrap_or(0),
                mature_verses: row.get(5).unwrap_or(0),
                young_verses: row.get(6).unwrap_or(0),
                unseen_verses: row.get(7).unwrap_or(0),
                suspended_verses: row.get(8).unwrap_or(0),
            },
        ))
    })?;

    let mut books_map = HashMap::new();
    for book_result in books_iter {
        let (book_name, stats) = book_result?;
        books_map.insert(book_name, stats);
    }

    Ok(books_map)
}

/// Gets the total study time for today in minutes
pub fn get_today_study_minutes(conn: &Connection) -> Result<f64> {
    let today_start_ms = get_today_start_ms()?;

    let deck_id = get_deck_id(conn)?;

    let query = r#"
        SELECT COALESCE(SUM(r.time), 0) as total_ms
        FROM revlog r
        JOIN cards c ON c.id = r.cid
        WHERE c.did = ?1 AND r.id >= ?2
    "#;

    let total_ms: i64 = conn.query_row(query, [deck_id, today_start_ms], |row| row.get(0))?;

    // Convert milliseconds to minutes
    Ok(total_ms as f64 / 60000.0)
}

/// Gets study time and learning progress for each of the last 30 days
pub fn get_last_30_days_stats(conn: &Connection) -> Result<Vec<DayStats>> {
    let deck_id = get_deck_id(conn)?;
    let model_id = get_model_id(conn)?;

    let mut results = Vec::new();
    let mut cumulative_passages = 0i64;

    // Query each day individually (from oldest to newest)
    for day_offset in (0..30).rev() {
        let (day_start_ms, day_end_ms, date_str) = get_day_boundaries(day_offset)?;

        // Query study time
        let time_query = r#"
            SELECT COALESCE(SUM(r.time), 0) as total_ms
            FROM revlog r
            JOIN cards c ON c.id = r.cid
            WHERE c.did = ?1 AND r.id >= ?2 AND r.id < ?3
        "#;

        let total_ms: i64 =
            conn.query_row(time_query, [deck_id, day_start_ms, day_end_ms], |row| {
                row.get(0)
            })?;

        let minutes = total_ms as f64 / 60000.0;

        // Query progress (maturation and loss)
        let progress_query = format!(
            r#"
            SELECT
                COUNT(CASE WHEN r.lastIvl < 21 AND r.ivl >= 21 THEN 1 END) as matured,
                COUNT(CASE WHEN r.lastIvl >= 21 AND r.ivl < 21 THEN 1 END) as lost
            FROM revlog r
            JOIN cards c ON c.id = r.cid
            JOIN notes n ON n.id = c.nid
            WHERE c.did = ?1 AND n.mid = ?2 AND c.ord = 0
                AND c.queue != {QUEUE_TYPE_SUSPENDED}
                AND r.id >= ?3 AND r.id < ?4
            "#
        );

        let (matured_passages, lost_passages): (i64, i64) = conn.query_row(
            &progress_query,
            [deck_id, model_id, day_start_ms, day_end_ms],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )?;

        // Update cumulative progress
        cumulative_passages += matured_passages - lost_passages;

        results.push(DayStats {
            date: date_str,
            minutes,
            matured_passages,
            lost_passages,
            cumulative_passages,
        });
    }

    Ok(results)
}

/// Gets study time and learning progress for each of the last 12 weeks
pub fn get_last_12_weeks_stats(conn: &Connection) -> Result<Vec<WeekStats>> {
    let deck_id = get_deck_id(conn)?;
    let model_id = get_model_id(conn)?;

    let mut results = Vec::new();
    let mut cumulative_passages = 0i64;

    // Query each week individually (from oldest to newest)
    for week_offset in (0..12).rev() {
        let (week_start_ms, week_end_ms, week_start_str) =
            get_week_boundaries(week_offset)?;

        // Query study time
        let time_query = r#"
            SELECT COALESCE(SUM(r.time), 0) as total_ms
            FROM revlog r
            JOIN cards c ON c.id = r.cid
            WHERE c.did = ?1 AND r.id >= ?2 AND r.id < ?3
        "#;

        let total_ms: i64 =
            conn.query_row(time_query, [deck_id, week_start_ms, week_end_ms], |row| {
                row.get(0)
            })?;

        let minutes = total_ms as f64 / 60000.0;

        // Query progress (maturation and loss)
        let progress_query = format!(
            r#"
            SELECT
                COUNT(CASE WHEN r.lastIvl < 21 AND r.ivl >= 21 THEN 1 END) as matured,
                COUNT(CASE WHEN r.lastIvl >= 21 AND r.ivl < 21 THEN 1 END) as lost
            FROM revlog r
            JOIN cards c ON c.id = r.cid
            JOIN notes n ON n.id = c.nid
            WHERE c.did = ?1 AND n.mid = ?2 AND c.ord = 0
                AND c.queue != {QUEUE_TYPE_SUSPENDED}
                AND r.id >= ?3 AND r.id < ?4
            "#
        );

        let (matured_passages, lost_passages): (i64, i64) = conn.query_row(
            &progress_query,
            [deck_id, model_id, week_start_ms, week_end_ms],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )?;

        // Update cumulative progress
        cumulative_passages += matured_passages - lost_passages;

        results.push(WeekStats {
            week_start: week_start_str,
            minutes,
            matured_passages,
            lost_passages,
            cumulative_passages,
        });
    }

    Ok(results)
}

/// Gets all distinct Bible references from the database, sorted alphabetically
pub fn get_all_references(conn: &Connection, deck_id: i64, model_id: i64) -> Result<Vec<String>> {
    let query = r#"
        SELECT DISTINCT n.sfld
        FROM notes n
        JOIN cards c ON c.nid = n.id
        WHERE c.did = ?1 AND n.mid = ?2
        ORDER BY n.sfld
    "#;

    let mut stmt = conn.prepare(query)?;
    let references = stmt
        .query_map([deck_id, model_id], |row| row.get::<_, String>(0))?
        .collect::<Result<Vec<String>, _>>()?;

    Ok(references)
}
