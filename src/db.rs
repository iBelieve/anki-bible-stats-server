use anyhow::{Context, Result};
use chrono::{Datelike, Duration, Local, TimeZone};
use chrono_tz::Tz;
use rusqlite::{Connection, OpenFlags};
use std::collections::HashMap;

use crate::book_name_parser;
use crate::config::TIMEZONE;
use crate::models::{BookStats, DailyStudyTime};
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

/// Gets a config value from the Anki database config table
/// Values are stored as UTF-8 strings in blobs
fn get_config_value(conn: &Connection, key: &str) -> Result<String> {
    let blob: Vec<u8> = conn
        .query_row("SELECT val FROM config WHERE key = ?1", [key], |row| {
            row.get(0)
        })
        .context(format!("Failed to read config key '{}' from database", key))?;

    // Convert blob to UTF-8 string
    let s = String::from_utf8(blob)
        .context(format!("Config value for '{}' is not valid UTF-8", key))?;

    Ok(s)
}

/// Gets the rollover hour from the Anki database configuration
pub fn get_rollover_hour(conn: &Connection) -> Result<i64> {
    let s = get_config_value(conn, "rollover")?;

    // Parse as integer
    let hour = s
        .parse::<i64>()
        .context(format!("Failed to parse rollover value '{}' as integer", s))?;

    Ok(hour)
}

/// Helper function to calculate the start of today in epoch milliseconds
/// accounting for timezone and rollover hour
fn get_today_start_ms(rollover_hour: i64) -> Result<i64> {
    let tz: Tz = TIMEZONE
        .parse()
        .context("Failed to parse timezone from config")?;

    let now = Local::now();
    let now_in_tz = tz
        .from_local_datetime(&now.naive_local())
        .single()
        .context("Failed to convert current time to configured timezone")?;

    // Get start of today at midnight in the configured timezone
    let today_midnight = tz
        .with_ymd_and_hms(
            now_in_tz.year(),
            now_in_tz.month(),
            now_in_tz.day(),
            0,
            0,
            0,
        )
        .single()
        .context("Failed to create today's midnight")?;

    // Add rollover hours
    let today_start = today_midnight + Duration::hours(rollover_hour);

    // Convert to epoch milliseconds
    Ok(today_start.timestamp_millis())
}

/// Helper function to calculate day boundaries for a specific day offset
fn get_day_boundaries(day_offset: i32, rollover_hour: i64) -> Result<(i64, i64, String)> {
    let tz: Tz = TIMEZONE
        .parse()
        .context("Failed to parse timezone from config")?;

    let now = Local::now();
    let now_in_tz = tz
        .from_local_datetime(&now.naive_local())
        .single()
        .context("Failed to convert current time to configured timezone")?;

    // Calculate the target date (today - day_offset)
    let target_date = now_in_tz - Duration::days(day_offset as i64);

    // Get start of that day at midnight
    let day_midnight = tz
        .with_ymd_and_hms(
            target_date.year(),
            target_date.month(),
            target_date.day(),
            0,
            0,
            0,
        )
        .single()
        .context("Failed to create target day's midnight")?;

    // Add rollover hours for day start
    let day_start = day_midnight + Duration::hours(rollover_hour);

    // Next day start
    let next_day_start = day_start + Duration::days(1);

    // Format date for output
    let date_str = target_date.format("%Y-%m-%d").to_string();

    Ok((
        day_start.timestamp_millis(),
        next_day_start.timestamp_millis(),
        date_str,
    ))
}

/// Gets the total study time for today in minutes
pub fn get_today_study_minutes(conn: &Connection) -> Result<f64> {
    let rollover_hour = get_rollover_hour(conn)?;
    let today_start_ms = get_today_start_ms(rollover_hour)?;

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

/// Gets study time for each of the last 30 days in minutes
pub fn get_last_30_days_study_minutes(conn: &Connection) -> Result<Vec<DailyStudyTime>> {
    let rollover_hour = get_rollover_hour(conn)?;
    let deck_id = get_deck_id(conn)?;

    let mut results = Vec::new();

    // Query each day individually
    for day_offset in 0..30 {
        let (day_start_ms, day_end_ms, date_str) = get_day_boundaries(day_offset, rollover_hour)?;

        let query = r#"
            SELECT COALESCE(SUM(r.time), 0) as total_ms
            FROM revlog r
            JOIN cards c ON c.id = r.cid
            WHERE c.did = ?1 AND r.id >= ?2 AND r.id < ?3
        "#;

        let total_ms: i64 =
            conn.query_row(query, [deck_id, day_start_ms, day_end_ms], |row| row.get(0))?;

        let minutes = total_ms as f64 / 60000.0;
        results.push(DailyStudyTime::new(date_str, minutes));
    }

    // Reverse so most recent day is last
    results.reverse();

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
