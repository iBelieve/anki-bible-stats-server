pub mod bible;
pub mod config;
pub mod db;
pub mod models;

use anyhow::Result;

use crate::bible::{NEW_TESTAMENT, OLD_TESTAMENT};
use crate::models::{BibleStats, DailyStudyTime};

/// Retrieves statistics for all Bible books from an Anki database
pub fn get_bible_stats(db_path: &str) -> Result<BibleStats> {
    let conn = db::open_database(db_path)?;
    let deck_id = db::get_deck_id(&conn)?;
    let model_id = db::get_model_id(&conn)?;

    let mut stats = BibleStats::new();

    // Get Old Testament stats
    for &book in OLD_TESTAMENT {
        let book_stats = db::get_book_stats(&conn, book, deck_id, model_id)?;
        stats.old_testament.add_book(book_stats);
    }

    // Get New Testament stats
    for &book in NEW_TESTAMENT {
        let book_stats = db::get_book_stats(&conn, book, deck_id, model_id)?;
        stats.new_testament.add_book(book_stats);
    }

    Ok(stats)
}

/// Gets the total study time for today in minutes
pub fn get_today_study_time(db_path: &str) -> Result<f64> {
    let conn = db::open_database(db_path)?;
    db::get_today_study_minutes(&conn)
}

/// Gets study time for each of the last 30 days in minutes
pub fn get_study_time_last_30_days(db_path: &str) -> Result<Vec<DailyStudyTime>> {
    let conn = db::open_database(db_path)?;
    db::get_last_30_days_study_minutes(&conn)
}
