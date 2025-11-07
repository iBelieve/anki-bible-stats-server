# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust library crate that analyzes Anki flashcard databases to generate statistics about Bible verse memorization progress. It queries an Anki SQLite database to count cards by status (mature, young, unseen, suspended) for each Bible book, separated into Old Testament and New Testament.

The crate provides:
- **Public library API**: Functions like `get_bible_stats()` that can be used by other crates
- **CLI binary**: Command-line tool for analyzing statistics locally and displaying formatted tables

## Development Commands

### Build and Run

#### CLI Tool (ankistats binary)
```bash
# Run the CLI with a specific command
cargo run -- books /path/to/collection.anki2   # Show book statistics for OT/NT
cargo run -- today /path/to/collection.anki2   # Show today's study time
cargo run -- daily /path/to/collection.anki2   # Show study time for last 30 days
cargo run -- weekly /path/to/collection.anki2  # Show study time for last 12 weeks
cargo run -- refs /path/to/collection.anki2    # List all Bible references in database

# Build and run release version
cargo build --release
./target/release/ankistats books ~/.local/share/Anki2/User/collection.anki2
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

## Public Library API

The crate exports the following public functions (defined in `src/lib.rs`):

- **`get_bible_stats(db_path: &str) -> Result<BibleStats>`** - Get Bible book statistics for Old and New Testament
- **`get_today_study_time(db_path: &str) -> Result<f64>`** - Get today's study time in minutes
- **`get_last_30_days_stats(db_path: &str) -> Result<Vec<DayStats>>`** - Get daily study stats for last 30 days
- **`get_last_12_weeks_stats(db_path: &str) -> Result<Vec<WeekStats>>`** - Get weekly study stats for last 12 weeks

These functions are used by both the CLI binary and the backend web server crate.

## Architecture

### Module Structure

- **`src/lib.rs`**: Public library API exposing functions like `get_bible_stats()` used by other crates and the CLI
- **`src/main.rs`**: CLI binary that parses arguments and formats output as tables
- **`src/models.rs`**: Data structures with both `Serialize` (for JSON API) and `Tabled` (for CLI display) support
- **`src/db.rs`**: All database interaction logic with Anki's SQLite schema, including custom SQLite function registration
- **`src/bible.rs`**: Canonical lists of Bible books (`OLD_TESTAMENT` and `NEW_TESTAMENT` constants)
- **`src/verse_parser.rs`**: Parses Bible references and counts verses (e.g., "Genesis 1:1-5" → 5 verses)
- **`src/book_name_parser.rs`**: Extracts book names from Bible references (e.g., "2 Timothy 3:16" → "2 Timothy")
- **`src/config.rs`**: Configuration constants like timezone settings

### Database Query Logic

The tool expects a specific Anki setup:
- Deck name: `Bible<UNIT_SEPARATOR>Verses` (where `UNIT_SEPARATOR` is `\x1F`)
- Note type: `Bible Verse`
- Card matching: Uses custom SQLite function `parse_book_name()` to extract book names from references

#### Custom SQLite Functions

The database module registers two custom SQLite functions in `db.rs`:

1. **`count_verses(reference)`**: Counts verses in a Bible reference
   - Single verse: "Genesis 1:1" → 1
   - Range: "Genesis 1:1-5" → 5
   - Handles verse suffixes: "Proverbs 12:4a" → 1
   - Single-chapter books: "Jude 24-25" → 2

2. **`parse_book_name(reference)`**: Extracts book name from a reference
   - Multi-chapter: "Genesis 1:1" → "Genesis"
   - Numbered books: "2 Timothy 3:16" → "2 Timothy"
   - Single-chapter: "Jude 24" → "Jude"

These functions are used directly in SQL queries to aggregate both passage counts (number of cards) and verse counts (total verses covered) for better statistics.

#### Card Status Classification

Card status is determined by queue type and interval (see Anki schema constants defined at the top of `db.rs`):
- **Mature**: BOTH cards (ord=0 and ord=1) must be review/buried cards with `ivl >= 21` days
- **Young**: At least one card is learning OR review/buried with `ivl < 21` days (and not mature)
- **Unseen**: At least one card is new (`queue = 0`)
- **Suspended**: At least one card has `queue < 0`

Queue type constants are based on Anki's internal schema (see link referenced in `db.rs`).

**Important**: Bible verse notes contain two cards:
- Card 0 (ord=0): Front → Back (e.g., reference → verse text)
- Card 1 (ord=1): Back → Front (e.g., verse text → reference)

A note is only counted as mature when BOTH cards have been learned to maturity (ivl >= 21 days).

### Data Flow

#### Library Function Flow
1. Public function (e.g., `get_bible_stats()`) is called with database path
2. Function opens SQLite connection and retrieves deck/model IDs based on hardcoded deck name and note type
3. For book statistics: `db::get_all_books_stats()` executes a single optimized GROUP BY query
4. The query uses custom SQLite functions (`parse_book_name` and `count_verses`) to aggregate data for all books in one pass
5. Results are looked up from the HashMap and accumulated into `BibleStats` with `AggregateStats` for each testament
6. Structured data is returned to caller

#### CLI Usage Flow
1. `src/main.rs` parses command-line arguments using clap
2. Based on subcommand, calls appropriate library function (e.g., `get_bible_stats()`)
3. CLI formats and prints results as tables using the `tabled` crate

### Key Implementation Details

- The tool joins both cards (ord=0 and ord=1) for each note to check maturity status
- A note is only counted as mature if BOTH cards are mature (ivl >= 21 days)
- Book matching uses `parse_book_name()` custom SQLite function to extract book names from the `sfld` field
- Note: "Psalm" is singular in `bible.rs` constants to match typical reference format
- The query optimization uses a single `GROUP BY` query instead of one query per book (66 queries → 1 query)
- Both passage counts (number of notes) and verse counts (using `count_verses()` function) are tracked
- Verse counting handles ranges, verse suffixes (e.g., "4a"), and single-chapter books
- Unicode formatting characters are stripped from references before parsing
- The `tabled` crate provides formatted table output with rounded borders
- All database errors use `anyhow` for context-aware error messages
- SQLite functions are deterministic and UTF-8 safe for reliable caching and performance

## Dependencies

- **rusqlite**: SQLite database access with bundled SQLite and custom function support
- **anyhow**: Error handling with context
- **chrono** / **chrono-tz**: Timezone-aware date/time handling for study time calculations
- **clap**: Command-line argument parsing with derive macros (CLI binary only)
- **tabled**: Table formatting for terminal output (CLI binary only)
- **serde** / **serde_json**: JSON serialization (for library consumers and models)
- **utoipa**: OpenAPI schema generation attributes for data structures
