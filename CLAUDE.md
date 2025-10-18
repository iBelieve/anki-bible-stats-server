# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust project that analyzes Anki flashcard databases to generate statistics about Bible verse memorization progress. It queries an Anki SQLite database to count cards by status (mature, young, unseen, suspended) for each Bible book, separated into Old Testament and New Testament.

The project includes two binaries:
- **Web API Server**: REST API built with Axum that exposes Bible statistics endpoints
- **CLI Tool**: Command-line interface for analyzing statistics locally

## Development Commands

### Build and Run

#### Web API Server (main binary)
```bash
# Set required environment variables
export ANKI_DATABASE_PATH="/path/to/collection.anki2"
export API_KEY="your-secure-api-key-here"

# Run the server in development (default binary)
cargo run

# Build and run release version
cargo build --release
./target/release/anki-bible-stats

# The server will start on http://0.0.0.0:3000
```

#### CLI Tool
```bash
# Run the CLI with a specific command
cargo run --bin cli -- books /path/to/collection.anki2   # Show book statistics for OT/NT
cargo run --bin cli -- today /path/to/collection.anki2   # Show today's study time
cargo run --bin cli -- daily /path/to/collection.anki2   # Show study time for last 30 days
cargo run --bin cli -- refs /path/to/collection.anki2    # List all Bible references in database

# Build and run release version
cargo build --release --bin cli
./target/release/cli books ~/.local/share/Anki2/User/collection.anki2
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

## API Endpoints

The web API server exposes the following REST endpoints:

### `GET /health`
Health check endpoint (no authentication required)

**Response:**
```json
{
  "status": "ok",
  "service": "anki-bible-stats"
}
```

### `GET /api/stats/books`
Get Bible book statistics for Old and New Testament

**Authentication:** Required (Bearer token)

**Response:** Returns a `BibleStats` object with detailed counts per book and testament aggregates

### `GET /api/stats/today`
Get today's study time

**Authentication:** Required (Bearer token)

**Response:**
```json
{
  "minutes": 45.5,
  "hours": 0.758
}
```

### `GET /api/stats/daily`
Get study time for each of the last 30 days

**Authentication:** Required (Bearer token)

**Response:**
```json
{
  "daily": [
    {"date": "2025-09-14", "minutes": 30.2},
    {"date": "2025-09-15", "minutes": 45.8}
  ],
  "summary": {
    "total_minutes": 720.5,
    "total_hours": 12.0,
    "average_minutes_per_day": 24.0,
    "average_hours_per_day": 0.4,
    "days_studied": 22,
    "total_days": 30
  }
}
```

### Authentication

All API endpoints (except `/health`) require authentication via Bearer token:

```bash
curl -H "Authorization: Bearer your-api-key-here" http://localhost:3000/api/stats/today
```

The API key must match the `API_KEY` environment variable set when running the server.

## Architecture

### Module Structure

The codebase is organized into the following structure:

- **`src/main.rs`**: Axum web server (main binary) with REST API endpoints and authentication middleware
- **`src/bin/cli.rs`**: CLI helper utility that parses arguments and formats output as tables
- **`src/lib.rs`**: Public API exposing functions like `get_bible_stats()` used by both binaries
- **`src/models.rs`**: Data structures with both `Serialize` (for JSON) and `Tabled` (for CLI) support
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
- **Mature**: Review/buried cards with `ivl >= 21` days
- **Young**: Learning cards OR review/buried cards with `ivl < 21` days
- **Unseen**: New cards (`queue = 0`)
- **Suspended**: Cards with `queue < 0` (excluding unseen)

Queue type constants are based on Anki's internal schema (see link referenced in `db.rs`).

### Data Flow

#### Web API Flow
1. Client makes authenticated request to API endpoint (e.g., `/api/stats/books`)
2. Axum middleware validates Bearer token against `API_KEY` environment variable
3. Handler function calls library function (e.g., `get_bible_stats()`) with database path from `ANKI_DATABASE_PATH`
4. Library function queries Anki database and returns structured data
5. Handler serializes result to JSON and returns HTTP response

#### CLI Flow
1. `src/bin/cli.rs` parses command-line arguments using clap
2. Based on subcommand, calls appropriate library function (e.g., `get_bible_stats()`)
3. Library function opens database and retrieves deck/model IDs
4. For book statistics: `db::get_all_books_stats()` executes a single optimized GROUP BY query
5. The query uses custom SQLite functions (`parse_book_name` and `count_verses`) to aggregate data for all books in one pass
6. Results are looked up from the HashMap and accumulated into `BibleStats` with `AggregateStats` for each testament
7. CLI formats and prints tables using the `tabled` crate

### Key Implementation Details

- The tool only counts cards where `ord = 0` (first card in the note) to avoid double-counting
- Book matching uses `parse_book_name()` custom SQLite function to extract book names from the `sfld` field
- Note: "Psalm" is singular in `bible.rs` constants to match typical reference format
- The query optimization uses a single `GROUP BY` query instead of one query per book (66 queries → 1 query)
- Both passage counts (number of cards) and verse counts (using `count_verses()` function) are tracked
- Verse counting handles ranges, verse suffixes (e.g., "4a"), and single-chapter books
- Unicode formatting characters are stripped from references before parsing
- The `tabled` crate provides formatted table output with rounded borders
- All database errors use `anyhow` for context-aware error messages
- SQLite functions are deterministic and UTF-8 safe for reliable caching and performance

## Dependencies

### Core Dependencies
- **rusqlite**: SQLite database access with bundled SQLite and custom function support
- **anyhow**: Error handling with context
- **chrono** / **chrono-tz**: Timezone-aware date/time handling for study time calculations

### Web API Dependencies
- **axum**: Web framework for REST API
- **tokio**: Async runtime
- **tower** / **tower-http**: Middleware support (CORS, etc.)
- **serde** / **serde_json**: JSON serialization

### CLI Dependencies
- **clap**: Command-line argument parsing with derive macros
- **tabled**: Table formatting for terminal output

## Environment Variables

### Web API Server
- **`ANKI_DATABASE_PATH`** (required): Full path to the Anki collection database file
- **`API_KEY`** (required): Secret key for API authentication via Bearer token

### CLI Tool
No environment variables required. Database path is passed as a command-line argument.
