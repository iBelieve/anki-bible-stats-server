# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust library and CLI tool that combines faith-related statistics from multiple data sources into a unified view. It aggregates data from:
- **ankistats**: Bible verse memorization progress from Anki
- **readingstats**: Bible reading time from KOReader
- **prayerstats**: Prayer time tracking (future enhancement)

The crate provides both a library API for programmatic access and a CLI tool for viewing combined statistics.

## Architecture

### Purpose and Design

`faithstats` serves as an **aggregation layer** that sits above the individual stats crates. It:
1. Queries multiple databases (Anki, KOReader, etc.)
2. Merges data by date, using zero values when data is missing for a particular day
3. Returns errors if any required database is unavailable
4. Provides unified data structures combining all sources

This design keeps the backend crate thin (just HTTP wrapper) while making the aggregation logic reusable across CLI, API, and future applications.

### Module Structure

- **`src/lib.rs`**: Public library API exposing functions like `get_faith_daily_stats()`
- **`src/main.rs`**: CLI binary that loads config from .env and displays formatted tables
- **`src/models.rs`**: Data structures with `Serialize` (for API), `ToSchema` (for OpenAPI), and `Tabled` (for CLI)

### Dependencies

- **ankistats, readingstats, prayerstats**: Source data crates (path dependencies)
- **statsutils**: Shared date/time utilities
- **anyhow**: Error handling
- **serde**: JSON serialization
- **utoipa**: OpenAPI schema generation
- **tabled**: CLI table formatting
- **clap**: CLI argument parsing
- **dotenvy**: .env file loading

## Development Commands

### Build and Test

```bash
# Build the faithstats crate
cargo build -p faithstats

# Build release version
cargo build -p faithstats --release

# Run tests
cargo test -p faithstats

# Check code
cargo check -p faithstats
```

### Running the CLI

The CLI loads database paths from environment variables rather than command-line arguments. Create a `.env` file in the project root or export the variables:

```bash
# Create .env file with database paths
cat > .env <<EOF
ANKI_DATABASE_PATH=/path/to/collection.anki2
KOREADER_DATABASE_PATH=/path/to/statistics.sqlite3
EOF

# Run the daily command
cargo run -p faithstats -- daily
```

Or export environment variables directly:

```bash
export ANKI_DATABASE_PATH="/path/to/collection.anki2"
export KOREADER_DATABASE_PATH="/path/to/statistics.sqlite3"
cargo run -p faithstats -- daily
```

### CLI Commands

Currently, the CLI provides one subcommand:

- **`faithstats daily`**: Show faith statistics for the last 30 days with combined view

Future commands may include weekly, monthly, or custom date ranges.

## Library API

### `get_faith_daily_stats(anki_db_path: &str, koreader_db_path: &str) -> Result<FaithDailyStats>`

Retrieves unified faith statistics for the last 30 days.

**Parameters:**
- `anki_db_path`: Path to Anki collection.anki2 database
- `koreader_db_path`: Path to KOReader statistics.sqlite3 database

**Returns:**
- `FaithDailyStats` containing:
  - `days`: Vec of `FaithDayStats` with per-day breakdown
  - `summary`: `FaithDailySummary` with aggregate statistics

**Error Handling:**
- Returns error if either database is unavailable or cannot be queried
- Uses zero values for days where one source has no data, but still requires both databases to be accessible

**Example:**
```rust
use faithstats::get_faith_daily_stats;

let stats = get_faith_daily_stats(
    "/path/to/collection.anki2",
    "/path/to/statistics.sqlite3"
)?;

println!("Total faith time: {:.2} hours", stats.summary.total_hours);
println!("Anki study: {:.2} min/day", stats.summary.anki_average_minutes_per_day);
println!("Reading: {:.2} min/day", stats.summary.reading_average_minutes_per_day);
```

## Data Structures

### FaithDayStats

Combined statistics for a single day:

```rust
pub struct FaithDayStats {
    pub date: String,                      // YYYY-MM-DD format

    // Anki Bible memorization stats
    pub anki_minutes: f64,
    pub anki_matured_passages: i64,
    pub anki_lost_passages: i64,
    pub anki_cumulative_passages: i64,

    // KOReader Bible reading stats
    pub reading_minutes: f64,

    // Prayer stats (future)
    pub prayer_minutes: f64,
}
```

### FaithDailySummary

Aggregate statistics across all sources:

- Per-source totals: minutes, hours, average per day, days active
- Anki-specific: matured/lost passages, net progress
- Combined: total time, average, days with any activity

### FaithDailyStats

Container with daily breakdown and summary:

```rust
pub struct FaithDailyStats {
    pub days: Vec<FaithDayStats>,
    pub summary: FaithDailySummary,
}
```

## Data Merging Algorithm

The `get_faith_daily_stats()` function merges data from multiple sources:

1. **Query both databases**: Calls `ankistats::get_last_30_days_stats()` and `readingstats::get_last_30_days_stats()`
2. **Create lookup maps**: Builds HashMap by date for efficient lookup
3. **Collect unique dates**: Gets all dates from both sources
4. **Merge by date**: For each date:
   - Look up stats from both sources
   - Use zero values if a source has no data for that date
   - Combine into unified `FaithDayStats`
5. **Compute summary**: Aggregate statistics across all days

This approach ensures:
- All dates from both sources are included
- No data is lost
- Missing data is represented as zeros (not omitted)
- Errors bubble up if databases are unavailable

## Environment Variables

### Required (for CLI)

- **`ANKI_DATABASE_PATH`**: Path to Anki collection database file
- **`KOREADER_DATABASE_PATH`**: Path to KOReader statistics database file

### Optional

The CLI will attempt to load a `.env` file from the current directory using the `dotenvy` crate. If the file doesn't exist, it falls back to checking environment variables directly.

## Integration with Backend

The backend crate imports `faithstats` as a dependency and wraps its functions in HTTP endpoints:

- `/api/faith/daily`: Returns `FaithDailyStats` as JSON
- Future: `/api/faith/weekly`, etc.

The backend loads database paths from the same environment variables and passes them to the library functions.

## Future Enhancements

### Planned Features

1. **Prayer statistics integration**: When `prayerstats` is implemented, automatically include prayer data
2. **Weekly aggregation**: `get_faith_weekly_stats()` for 12-week view
3. **Custom date ranges**: Allow querying specific time periods
4. **Additional metrics**: Total verses read, prayer topics, etc.
5. **Unified CLI commands**: More subcommands (weekly, monthly, summary)

### Architecture Notes

- The crate is designed to gracefully handle new data sources
- Adding a new source requires:
  1. Update `FaithDayStats` model with new fields
  2. Update `FaithDailySummary` with new aggregates
  3. Update merging logic in `get_faith_daily_stats()`
  4. Update CLI display and summary output
- Prayer stats are already scaffolded in the models with `prayer_minutes` field

## Testing

When adding tests, consider:
- Testing merge logic with different date ranges
- Testing zero-value handling when sources have gaps
- Testing error propagation when databases are unavailable
- Testing summary calculations with various data patterns
