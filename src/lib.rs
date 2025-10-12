pub mod bible;
pub mod db;
pub mod models;

use anyhow::Result;

use crate::bible::{NEW_TESTAMENT, OLD_TESTAMENT};
use crate::models::BibleStats;

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
