# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is the backend web server crate that provides a REST API for the ankistats library. It's built with Axum and provides HTTP endpoints with Bearer token authentication, CORS support, and interactive OpenAPI documentation via Swagger UI.

## Development Commands

### Running the Server

The server requires environment variables for configuration. You can provide them in two ways:

**Option 1: Using a .env file (recommended)**

```bash
# Copy the example file and update with your values
cp .env.example .env
# Edit .env with your actual paths and API key

# Run the server (it will automatically load .env)
cargo run -p backend

# Or from the backend directory
cargo run

# The server will start on http://0.0.0.0:3000
```

**Option 2: Export environment variables manually**

```bash
# Set required environment variables
export ANKI_DATABASE_PATH="/path/to/collection.anki2"
export KOREADER_DATABASE_PATH="/path/to/statistics.sqlite3"
export API_KEY="your-secure-api-key"

# Run in development
cargo run -p backend
```

### Using Makefile (from workspace root)

If you have a `collection.anki2` file in the `ankistats/` directory:

```bash
make backend  # Runs with ANKI_DATABASE_PATH=ankistats/collection.anki2 and API_KEY=test
```

### Testing

```bash
# Run tests
cargo test

# Test endpoints with curl
curl http://localhost:3000/health

# Test authenticated endpoints
curl -H "Authorization: Bearer your-api-key" http://localhost:3000/api/anki/books
curl -H "Authorization: Bearer your-api-key" http://localhost:3000/api/anki/today
curl -H "Authorization: Bearer your-api-key" http://localhost:3000/api/anki/daily
curl -H "Authorization: Bearer your-api-key" http://localhost:3000/api/anki/weekly
```

### Accessing Documentation

Once the server is running:
- **Swagger UI**: http://localhost:3000/swagger-ui/
- **OpenAPI JSON**: http://localhost:3000/openapi.json

## API Endpoints

All endpoints are defined in `src/main.rs` with utoipa macros for OpenAPI documentation.

### Public Endpoints (No Authentication)

#### `GET /health`
Health check endpoint that returns service status.

**Response:**
```json
{
  "status": "ok",
  "service": "life-stats"
}
```

#### `GET /swagger-ui/`
Interactive Swagger UI for API documentation and testing. You can test all authenticated endpoints directly from the browser by entering your Bearer token.

#### `GET /openapi.json`
Raw OpenAPI 3.0 specification in JSON format describing all endpoints, request/response schemas, and authentication requirements.

### Authenticated Endpoints (Bearer Token Required)

All `/api/anki/*` endpoints require authentication via Bearer token in the Authorization header:
```
Authorization: Bearer <your-api-key>
```

#### `GET /api/anki/books`
Get Bible book statistics for Old and New Testament.

**Response:** `BibleStats` object with detailed counts per book and testament aggregates.

#### `GET /api/anki/today`
Get today's study time in minutes and hours.

**Response:**
```json
{
  "minutes": 45.5,
  "hours": 0.758
}
```

#### `GET /api/anki/daily`
Get daily study time for each of the last 30 days.

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

#### `GET /api/anki/weekly`
Get weekly study time for each of the last 12 weeks.

**Response:** `WeeklyStats` object with weekly breakdown and summary statistics.

### Error Responses

Failed requests return appropriate HTTP status codes:
- **401 Unauthorized**: Missing or invalid API key
- **500 Internal Server Error**: Database errors or other server issues

Error responses include a JSON body with details:
```json
{
  "error": "error message here"
}
```

## Architecture

### Request Flow

1. Client makes HTTP request to an endpoint
2. Axum routing layer matches the request to a handler
3. **Authentication middleware** (`auth_middleware`) runs before the handler:
   - Public endpoints (`/health`, `/swagger-ui`, `/openapi.json`) bypass auth
   - Other endpoints require valid Bearer token matching `API_KEY` env var
   - Returns 401 Unauthorized if auth fails
4. Handler function extracts database path from Axum state
5. Handler calls ankistats library function (e.g., `get_bible_stats(&db_path)`)
6. Library returns data or error
7. Handler wraps result:
   - **Success**: Serializes to JSON via `Json(data)` wrapper
   - **Error**: Converts to `AppError` which becomes 500 response with error JSON
8. Response is sent to client

### Code Structure

- **`src/main.rs`**: Entire backend implementation (single file)
  - `main()`: Server setup, routing, middleware configuration
  - `auth_middleware()`: Bearer token validation
  - Handler functions: `health_check()`, `get_books_stats()`, `get_today_stats()`, `get_daily_stats()`, `get_weekly_stats()`
  - `AppError`: Custom error type that converts to HTTP 500 responses
  - `ApiDoc`: OpenAPI documentation structure with utoipa macros
  - `SecurityAddon`: Adds Bearer auth to OpenAPI spec

### Authentication Implementation

The authentication middleware uses a closure to capture the `api_key` from the environment:

```rust
.layer(middleware::from_fn(move |req, next| {
    auth_middleware(req, next, api_key.clone())
}))
```

The middleware checks the `Authorization` header:
- Extracts Bearer token from `Authorization: Bearer <token>` header
- Compares against the `expected_api_key` parameter
- Allows request through if match, returns 401 if not

### State Management

The Axum router uses `.with_state(db_path)` to inject the database path into all handlers. Each handler extracts it via:

```rust
axum::extract::State(db_path): axum::extract::State<String>
```

### OpenAPI Documentation

The project uses `utoipa` for OpenAPI spec generation:
- Each handler has a `#[utoipa::path(...)]` macro with response schemas
- All data models from ankistats have `#[derive(utoipa::ToSchema)]`
- `ApiDoc` struct lists all paths and components
- `SecurityAddon` adds Bearer auth security scheme to the spec
- Swagger UI is served via `utoipa-swagger-ui` integration

### CORS Configuration

The server uses `CorsLayer::permissive()` to allow cross-origin requests from any domain. This is suitable for development and personal use but should be restricted for production deployments.

## Environment Variables

### Configuration

Environment variables can be provided in two ways:
1. **Using a `.env` file** in the workspace root (recommended for development)
2. **Exporting variables** in your shell before running the server

The server will automatically load variables from a `.env` file if present (using the `dotenvy` crate).

### Required Variables

- **`ANKI_DATABASE_PATH`**: Absolute path to the Anki collection database file (e.g., `/path/to/collection.anki2`)
- **`KOREADER_DATABASE_PATH`**: Absolute path to the KOReader statistics database file (e.g., `/path/to/statistics.sqlite3`)
- **`API_KEY`**: Secret key for API authentication. Clients must send this as a Bearer token in the Authorization header.

See `.env.example` in the workspace root for a template.

### Validation

On startup, the server:
1. Loads variables from `.env` file if present
2. Checks that all required environment variables are set (exits with error if not)
3. Verifies the database files exist at the specified paths (exits with error if not)

## Dependencies

- **ankistats**: The library crate providing all core functionality
- **faithstats**: Library for aggregating faith statistics from multiple sources
- **axum**: Web framework (0.8.x)
- **tokio**: Async runtime with "full" features
- **tower** / **tower-http**: Middleware support (CORS)
- **serde** / **serde_json**: JSON serialization
- **utoipa** / **utoipa-swagger-ui**: OpenAPI documentation and Swagger UI
- **anyhow**: Error handling (used via ankistats and for `AppError`)
- **dotenvy**: Environment variable loading from `.env` files

## Deployment

The workspace includes a Dockerfile for containerized deployment. When deploying:

1. Build the Docker image
2. Set the required environment variables
3. Mount a volume containing the Anki database at the path specified by `ANKI_DATABASE_PATH`
4. Expose port 3000

Example Docker run:
```bash
docker run -p 3000:3000 \
  -e ANKI_DATABASE_PATH=/data/collection.anki2 \
  -e API_KEY=your-secure-key \
  -v /path/to/anki/data:/data \
  lifestats-backend
```

### Using with Self-Hosted Anki Server

If using with the self-hosted Anki server, you may encounter "database is locked" errors because Anki opens the database in exclusive locking mode. The solution is to use a forked version of Anki that removes the exclusive lock. See the ankistats README.md for details.
