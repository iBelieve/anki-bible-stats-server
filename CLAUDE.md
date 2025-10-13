# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust CLI tool that analyzes Anki flashcard databases to generate statistics about Bible verse memorization progress. It queries an Anki SQLite database to count cards by status (mature, young, unseen, suspended) for each Bible book, separated into Old Testament and New Testament.

## Development Commands

### Build and Run
```bash
# Build the project
cargo build

# Run with release optimizations
cargo build --release

# Run the tool (requires path to Anki database)
cargo run -- <path-to-anki-database>
# Example: cargo run -- ~/.local/share/Anki2/User/collection.anki2

# Run release build directly
./target/release/anki-bible-stats <path-to-anki-database>
```

### Testing
```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test <test_name>
```

### Code Quality
```bash
# Check code without building
cargo check

# Run clippy linter
cargo clippy

# Format code
cargo fmt

# Check formatting
cargo fmt -- --check
```

## Architecture

### Module Structure

The codebase is organized into four main modules:

- **`main.rs`**: CLI entry point that parses arguments, calls `get_bible_stats()`, and formats output as tables
- **`lib.rs`**: Public API exposing `get_bible_stats()` which orchestrates the entire statistics gathering process
- **`models.rs`**: Data structures for statistics (`BookStats`, `AggregateStats`, `BibleStats`)
- **`db.rs`**: All database interaction logic with Anki's SQLite schema
- **`bible.rs`**: Canonical lists of Bible books (`OLD_TESTAMENT` and `NEW_TESTAMENT` constants)

### Database Query Logic

The tool expects a specific Anki setup:
- Deck name: `Bible<UNIT_SEPARATOR>Verses` (where `UNIT_SEPARATOR` is `\x1F`)
- Note type: `Bible Verse`
- Card matching: Uses `sfld LIKE 'BookName%'` to find cards for each book

Card status classification (in `db.rs:56-86`):
- **Mature**: Cards with `ivl >= 21` days (review/buried cards)
- **Young**: Learning cards OR review cards with `ivl < 21` days
- **Unseen**: New cards (`queue = 0`)
- **Suspended**: Cards with `queue < 0` (excluding unseen)

Queue type constants are based on Anki's internal schema (see link in `db.rs:7`).

### Data Flow

1. `main.rs` validates CLI args and calls `get_bible_stats()`
2. `lib.rs:get_bible_stats()` opens database and retrieves deck/model IDs
3. For each book in `OLD_TESTAMENT` and `NEW_TESTAMENT`, calls `db::get_book_stats()`
4. `db::get_book_stats()` runs SQL query with card status aggregation
5. Results are accumulated into `BibleStats` with `AggregateStats` for each testament
6. `main.rs` formats and prints tables using the `tabled` crate

### Key Implementation Details

- The tool only counts cards where `ord = 0` (first card in the note) to avoid double-counting
- Book matching uses `LIKE` pattern with the book name prefix (note: "Psalm" is singular to match reference format)
- The `tabled` crate provides formatted table output with rounded borders
- All database errors use `anyhow` for context-aware error messages

## Dependencies

- **rusqlite**: SQLite database access with bundled SQLite
- **anyhow**: Error handling with context
- **tabled**: Table formatting for terminal output
