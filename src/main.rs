use anki_bible_stats::{
    get_bible_stats, get_study_time_last_30_days, get_today_study_time,
    models::{BibleStats, DailyStats, HealthCheck, TodayStats},
};
use axum::{
    Router,
    extract::Request,
    http::StatusCode,
    middleware::{self, Next},
    response::{IntoResponse, Json, Response},
    routing::get,
};
use serde_json::json;
use std::env;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    // Get configuration from environment variables
    let db_path = env::var("ANKI_DATABASE_PATH").unwrap_or_else(|_| {
        eprintln!("Error: ANKI_DATABASE_PATH environment variable is required");
        std::process::exit(1);
    });

    let api_key = env::var("API_KEY").unwrap_or_else(|_| {
        eprintln!("Error: API_KEY environment variable is required");
        std::process::exit(1);
    });

    // Validate that the database path exists
    if !std::path::Path::new(&db_path).exists() {
        eprintln!("Error: Database file not found at: {}", db_path);
        std::process::exit(1);
    }

    println!("Starting anki-bible-stats API server...");
    println!("Database: {}", db_path);

    // Build the router with routes
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/stats/books", get(get_books_stats))
        .route("/api/stats/today", get(get_today_stats))
        .route("/api/stats/daily", get(get_daily_stats))
        .layer(middleware::from_fn(move |req, next| {
            auth_middleware(req, next, api_key.clone())
        }))
        .layer(CorsLayer::permissive())
        .with_state(db_path);

    // Start the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to port 3000");

    println!("Server listening on http://0.0.0.0:3000");

    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
}

/// Authentication middleware that validates the API key
async fn auth_middleware(
    req: Request,
    next: Next,
    expected_api_key: String,
) -> Result<Response, StatusCode> {
    // Skip auth for health check endpoint
    if req.uri().path() == "/health" {
        return Ok(next.run(req).await);
    }

    let headers = req.headers();
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("");

    if let Some(token) = auth_header.strip_prefix("Bearer ")
        && token == expected_api_key
    {
        return Ok(next.run(req).await);
    }

    Err(StatusCode::UNAUTHORIZED)
}

/// Health check endpoint
async fn health_check() -> impl IntoResponse {
    Json(HealthCheck::new())
}

/// Get Bible book statistics
async fn get_books_stats(
    axum::extract::State(db_path): axum::extract::State<String>,
) -> Result<Json<BibleStats>, AppError> {
    let stats = get_bible_stats(&db_path)?;
    Ok(Json(stats))
}

/// Get today's study time
async fn get_today_stats(
    axum::extract::State(db_path): axum::extract::State<String>,
) -> Result<Json<TodayStats>, AppError> {
    let minutes = get_today_study_time(&db_path)?;
    Ok(Json(TodayStats::new(minutes)))
}

/// Get daily study time for last 30 days
async fn get_daily_stats(
    axum::extract::State(db_path): axum::extract::State<String>,
) -> Result<Json<DailyStats>, AppError> {
    let daily_stats = get_study_time_last_30_days(&db_path)?;
    Ok(Json(DailyStats::new(daily_stats)))
}

/// Custom error type for API errors
struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": format!("{:#}", self.0)
            })),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
