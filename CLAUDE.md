# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust workspace for tracking personal life and faith statistics from multiple data sources, including Bible memorization from Anki, Bible reading from KOReader, and prayer time. The project is structured as a workspace with multiple crates.

## Workspace Structure

This repository uses Cargo workspace with the following crates:

- **ankistats**: Library and CLI for analyzing Anki Bible verse memorization statistics
- **backend**: Axum web server that provides REST API endpoints for the ankistats library

The backend crate depends on ankistats and exposes its functionality via HTTP API.

## Development Commands

### Build and Test

```bash
# Build the entire workspace
cargo build

# Build specific package
cargo build -p ankistats
cargo build -p backend

# Run tests
cargo test

# Run tests for specific package
cargo test -p ankistats

# Check code without building
cargo check

# Run clippy linter
cargo clippy

# Format code
cargo fmt
```

### Running Locally

For package-specific development commands, see the CLAUDE.md file in each subcrate directory.

#### Backend API Server

```bash
# Set environment variables and run
export ANKI_DATABASE_PATH="/path/to/collection.anki2"
export API_KEY="your-api-key"
cargo run -p backend

# Or use make (if you have collection.anki2 in ankistats/)
make backend
```

The server starts on http://0.0.0.0:3000 with Swagger UI at http://localhost:3000/swagger-ui/

#### Ankistats CLI

```bash
# Run CLI commands (requires database path)
cargo run -p ankistats -- books /path/to/collection.anki2
cargo run -p ankistats -- today /path/to/collection.anki2
cargo run -p ankistats -- daily /path/to/collection.anki2
cargo run -p ankistats -- weekly /path/to/collection.anki2

# Or use make shortcuts (if you have collection.anki2 in ankistats/)
make books
make today
make daily
make weekly
```

### Using Makefile

The top-level Makefile provides shortcuts when you have a `collection.anki2` file in the ankistats directory (gitignored):

- `make build` - Build ankistats package
- `make test` - Generate test data and run tests
- `make books/today/daily/weekly` - Run CLI commands
- `make backend` - Run backend server with test credentials

## Architecture

### Workspace Dependencies

The backend crate imports and re-exports types from ankistats, then wraps them in Axum route handlers with authentication middleware. This separation allows:

- ankistats to remain a pure library with no web framework dependencies
- backend to provide HTTP API layer with OpenAPI documentation
- CLI tool (in ankistats) to use the same core logic as the API

### API Endpoints

The backend exposes these endpoints (see backend/src/main.rs for full details):

- `GET /health` - Health check (no auth)
- `GET /api/anki/books` - Bible book statistics (auth required)
- `GET /api/anki/today` - Today's study time (auth required)
- `GET /api/anki/daily` - Last 30 days study time (auth required)
- `GET /api/anki/weekly` - Last 12 weeks study time (auth required)
- `GET /swagger-ui` - Interactive API documentation (no auth)
- `GET /openapi.json` - OpenAPI specification (no auth)

Authentication uses Bearer token that must match the API_KEY environment variable.

## Environment Variables

### Backend Server

- **ANKI_DATABASE_PATH** (required): Path to Anki collection database file
- **API_KEY** (required): Secret key for API authentication

### CLI Tool

No environment variables required. Database path is passed as command-line argument.

## Deployment

The project includes a Dockerfile for containerized deployment. Set the required environment variables and mount a volume containing the Anki database file.

## Additional Documentation

For detailed information about the ankistats implementation, database queries, and data structures, see [ankistats/CLAUDE.md](./ankistats/CLAUDE.md).
