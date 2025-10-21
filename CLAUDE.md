# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust workspace for tracking personal life and faith statistics from multiple data sources. It includes:
- Bible memorization tracking from Anki
- Bible reading time from KOReader
- Prayer time tracking (future)
- A web API and frontend for viewing combined statistics

The project uses a three-tier architecture: data source crates → aggregation layer → presentation layer (API/CLI/frontend).

## Workspace Structure

This repository uses Cargo workspace with the following crates:

### Data Source Crates
- **ankistats**: Anki Bible verse memorization statistics (library + CLI)
- **readingstats**: KOReader Bible reading time statistics (library + CLI)
- **prayerstats**: Prayer time tracking (library + CLI, not yet implemented)
- **statsutils**: Shared date/time utilities used by all stats crates

### Aggregation Layer
- **faithstats**: Combines data from multiple sources into unified statistics (library + CLI)

### Presentation Layer
- **backend**: Axum REST API server exposing all statistics as JSON endpoints
- **frontend**: SvelteKit web application for viewing statistics (separate from Rust workspace)

## Development Commands

### Rust Workspace

#### Build and Test

```bash
# Build the entire workspace
cargo build

# Build specific package
cargo build -p ankistats
cargo build -p faithstats
cargo build -p backend

# Run tests
cargo test

# Run tests for specific package
cargo test -p ankistats

# Check code without building
cargo check

# Run clippy linter
cargo clippy --all-targets --all-features

# Format code
cargo fmt
```

#### Running CLIs

Each stats crate provides its own CLI. See individual CLAUDE.md files for details.

```bash
# Ankistats CLI (individual Anki stats)
cargo run -p ankistats -- books /path/to/collection.anki2
cargo run -p ankistats -- today /path/to/collection.anki2
cargo run -p ankistats -- daily /path/to/collection.anki2
cargo run -p ankistats -- weekly /path/to/collection.anki2

# Readingstats CLI (individual reading stats)
cargo run -p readingstats -- daily /path/to/statistics.sqlite3

# Faithstats CLI (combined stats from all sources)
# Requires .env file with ANKI_DATABASE_PATH and KOREADER_DATABASE_PATH
cargo run -p faithstats -- daily
```

#### Backend API Server

```bash
# Create .env file with required environment variables
cp .env.example .env
# Edit .env with your database paths and API key

# Run the server
cargo run -p backend
```

The server starts on http://0.0.0.0:3000 with Swagger UI at http://localhost:3000/swagger-ui/

#### Using Makefile

The top-level Makefile provides shortcuts when you have a `collection.anki2` file in the ankistats directory:

```bash
make build    # Build ankistats
make test     # Generate test data and run tests
make books    # Run ankistats books command
make today    # Run ankistats today command
make daily    # Run faithstats daily command (combined stats)
make weekly   # Run ankistats weekly command
make backend  # Run backend with test credentials
```

### Frontend (SvelteKit)

```bash
cd frontend

# Install dependencies
npm ci

# Start development server
npm run dev

# Type checking
npm run check              # Run once
npm run check:watch        # Watch mode

# Code quality
npm run lint               # Check formatting and linting
npm run format             # Auto-format with Prettier

# Build for production
npm run build
npm run preview            # Preview production build

# Generate API client types from backend OpenAPI spec
npm run generate-api:local  # From local backend (http://localhost:3000)
npm run generate-api:prod   # From production backend
```

### CI/CD

The repository includes GitHub Actions workflows that run on push and pull requests:

```bash
# Rust checks (format, lint, build, test)
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo build --verbose
cargo test --verbose

# Frontend checks (format, lint, type-check)
cd frontend
npm run format:check
npx eslint .
npm run check
```

## Architecture

### Three-Tier Design

The project is structured in three layers:

1. **Data Source Crates** (ankistats, readingstats, prayerstats)
   - Each crate provides a library API and CLI for a specific data source
   - Libraries expose functions like `get_last_30_days_stats(db_path)` that return structured data
   - CLIs format and display the data as tables
   - No dependencies on web frameworks or other stats crates

2. **Aggregation Layer** (faithstats)
   - Combines data from multiple sources into unified statistics
   - Merges by date, using zero values when data is missing for a particular day
   - Provides both library API and CLI
   - Depends on all data source crates

3. **Presentation Layer** (backend, frontend)
   - **backend**: Axum REST API that wraps library functions in HTTP endpoints with authentication
   - **frontend**: SvelteKit application that consumes the API and displays interactive charts
   - Backend uses `.env` file for configuration (database paths, API key)
   - Frontend generates TypeScript types from backend's OpenAPI spec

### Data Flow Example

For combined faith statistics:
1. Frontend makes authenticated request to `GET /api/faith/daily`
2. Backend calls `faithstats::get_faith_daily_stats(anki_path, koreader_path)`
3. Faithstats calls `ankistats::get_last_30_days_stats()` and `readingstats::get_last_30_days_stats()`
4. Each stats crate queries its respective SQLite database
5. Faithstats merges the results by date
6. Backend serializes to JSON and returns to frontend
7. Frontend renders charts using the unified data

### API Endpoints

The backend exposes these endpoints (see backend/src/main.rs for details):

**Public (no auth):**
- `GET /health` - Health check
- `GET /swagger-ui/` - Interactive API documentation
- `GET /openapi.json` - OpenAPI specification

**Authenticated (Bearer token required):**
- `GET /api/anki/books` - Bible book statistics
- `GET /api/anki/today` - Today's Anki study time
- `GET /api/anki/daily` - Last 30 days Anki study time
- `GET /api/anki/weekly` - Last 12 weeks Anki study time
- `GET /api/faith/daily` - Combined daily stats from all sources (Anki + reading)

Authentication uses Bearer token that must match the `API_KEY` environment variable.

## Environment Variables

### Backend Server and Faithstats CLI

These components require environment variables. Create a `.env` file in the workspace root (see `.env.example`):

- **ANKI_DATABASE_PATH** (required): Path to Anki collection.anki2 database file
- **KOREADER_DATABASE_PATH** (required): Path to KOReader statistics.sqlite3 database file
- **API_KEY** (required, backend only): Secret key for API authentication

### Individual Stats CLIs

The individual crate CLIs (ankistats, readingstats) do not use environment variables. Database paths are passed as command-line arguments:

```bash
cargo run -p ankistats -- daily /path/to/collection.anki2
cargo run -p readingstats -- daily /path/to/statistics.sqlite3
```

## Deployment

The project includes a Dockerfile for containerized deployment. When deploying:

1. Set the required environment variables (`ANKI_DATABASE_PATH`, `KOREADER_DATABASE_PATH`, `API_KEY`)
2. Mount volumes containing the database files
3. Expose port 3000 for the backend API

The frontend is configured for Cloudflare Pages deployment via `@sveltejs/adapter-cloudflare`.

## Additional Documentation

Each crate has its own CLAUDE.md file with detailed implementation information:

- **[ankistats/CLAUDE.md](./ankistats/CLAUDE.md)** - Anki database schema, custom SQLite functions, verse parsing
- **[faithstats/CLAUDE.md](./faithstats/CLAUDE.md)** - Data merging algorithm, aggregation logic
- **[backend/CLAUDE.md](./backend/CLAUDE.md)** - Authentication middleware, OpenAPI documentation
- **[frontend/CLAUDE.md](./frontend/CLAUDE.md)** - SvelteKit configuration, Svelte 5 runes syntax, Tailwind CSS v4
