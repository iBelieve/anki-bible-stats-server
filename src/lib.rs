pub mod bible;
pub mod book_name_parser;
pub mod config;
pub mod db;
pub mod models;
pub mod verse_parser;

use anyhow::Result;

use crate::bible::{NEW_TESTAMENT, OLD_TESTAMENT};
use crate::models::{BibleStats, DayStats, WeekStats};

/// Retrieves statistics for all Bible books from an Anki database
pub fn get_bible_stats(db_path: &str) -> Result<BibleStats> {
    let conn = db::open_database(db_path)?;
    let deck_id = db::get_deck_id(&conn)?;
    let model_id = db::get_model_id(&conn)?;

    // Get all book stats in a single query
    let books_map = db::get_all_books_stats(&conn, deck_id, model_id)?;

    let mut stats = BibleStats::new();

    // Get Old Testament stats - lookup from HashMap or create zero-filled stats
    for &book in OLD_TESTAMENT {
        let book_stats = books_map
            .get(book)
            .cloned()
            .unwrap_or_else(|| models::BookStats {
                book: book.to_string(),
                mature_passages: 0,
                young_passages: 0,
                unseen_passages: 0,
                suspended_passages: 0,
                mature_verses: 0,
                young_verses: 0,
                unseen_verses: 0,
                suspended_verses: 0,
            });
        stats.old_testament.add_book(book_stats);
    }

    // Get New Testament stats - lookup from HashMap or create zero-filled stats
    for &book in NEW_TESTAMENT {
        let book_stats = books_map
            .get(book)
            .cloned()
            .unwrap_or_else(|| models::BookStats {
                book: book.to_string(),
                mature_passages: 0,
                young_passages: 0,
                unseen_passages: 0,
                suspended_passages: 0,
                mature_verses: 0,
                young_verses: 0,
                unseen_verses: 0,
                suspended_verses: 0,
            });
        stats.new_testament.add_book(book_stats);
    }

    Ok(stats)
}

/// Gets the total study time for today in minutes
pub fn get_today_study_time(db_path: &str) -> Result<f64> {
    let conn = db::open_database(db_path)?;
    db::get_today_study_minutes(&conn)
}

/// Gets study time and learning progress for each of the last 30 days
pub fn get_last_30_days_stats(db_path: &str) -> Result<Vec<DayStats>> {
    let conn = db::open_database(db_path)?;
    db::get_last_30_days_stats(&conn)
}

/// Gets study time and learning progress for each of the last 12 weeks
pub fn get_last_12_weeks_stats(db_path: &str) -> Result<Vec<WeekStats>> {
    let conn = db::open_database(db_path)?;
    db::get_last_12_weeks_stats(&conn)
}

/// Gets all Bible references from the database, sorted alphabetically
pub fn get_bible_references(db_path: &str) -> Result<Vec<String>> {
    let conn = db::open_database(db_path)?;
    let deck_id = db::get_deck_id(&conn)?;
    let model_id = db::get_model_id(&conn)?;
    db::get_all_references(&conn, deck_id, model_id)
}
