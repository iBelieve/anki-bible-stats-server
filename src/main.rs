use anki_bible_stats::{
    get_bible_stats, get_last_12_weeks_stats, get_last_30_days_stats, get_today_study_time,
    models::{
        AggregateStats, BibleStats, BookStats, DailyStats, DailySummary, DayStats, ErrorResponse,
        HealthCheck, TodayStats, WeekStats, WeeklyStats, WeeklySummary,
    },
};
use axum::{
    Router,
    extract::Request,
    http::StatusCode,
    middleware::{self, Next},
    response::{IntoResponse, Json, Response},
    routing::get,
};
use std::env;
use tower_http::cors::CorsLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

/// OpenAPI documentation structure
#[derive(OpenApi)]
#[openapi(
    paths(
        health_check,
        get_books_stats,
        get_today_stats,
        get_daily_stats,
        get_weekly_stats,
    ),
    components(
        schemas(HealthCheck, BibleStats, TodayStats, DailyStats, WeeklyStats,
                BookStats, AggregateStats, DayStats, DailySummary, WeekStats, WeeklySummary, ErrorResponse)
    ),
    tags(
        (name = "health", description = "Health check endpoints"),
        (name = "statistics", description = "Bible memorization statistics endpoints")
    ),
    info(
        title = "Anki Bible Stats API",
        description = "REST API for analyzing Anki flashcard databases to generate Bible verse memorization statistics",
        license(
            name = "AGPL-3.0-or-later",
            url = "https://www.gnu.org/licenses/agpl-3.0.en.html"
        )
    ),
    modifiers(&SecurityAddon)
)]
struct ApiDoc;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                utoipa::openapi::security::SecurityScheme::Http(
                    utoipa::openapi::security::Http::new(
                        utoipa::openapi::security::HttpAuthScheme::Bearer,
                    ),
                ),
            )
        }
    }
}

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
        .merge(SwaggerUi::new("/swagger-ui").url("/openapi.json", ApiDoc::openapi()))
        .route("/health", get(health_check))
        .route("/api/stats/books", get(get_books_stats))
        .route("/api/stats/today", get(get_today_stats))
        .route("/api/stats/daily", get(get_daily_stats))
        .route("/api/stats/weekly", get(get_weekly_stats))
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
    println!("API Documentation:");
    println!("  - Swagger UI: http://localhost:3000/swagger-ui/");
    println!("  - OpenAPI spec: http://localhost:3000/openapi.json");

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
    let path = req.uri().path();

    // Skip auth for public endpoints
    if path == "/health" || path == "/openapi.json" || path.starts_with("/swagger-ui") {
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
#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Service is healthy", body = HealthCheck)
    ),
    tag = "health"
)]
async fn health_check() -> impl IntoResponse {
    Json(HealthCheck::new())
}

/// Get Bible book statistics
#[utoipa::path(
    get,
    path = "/api/stats/books",
    responses(
        (status = 200, description = "Bible book statistics retrieved successfully", body = BibleStats),
        (status = 401, description = "Unauthorized - invalid or missing API key"),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "statistics"
)]
async fn get_books_stats(
    axum::extract::State(db_path): axum::extract::State<String>,
) -> Result<Json<BibleStats>, AppError> {
    let stats = get_bible_stats(&db_path)?;
    Ok(Json(stats))
}

/// Get today's study time
#[utoipa::path(
    get,
    path = "/api/stats/today",
    responses(
        (status = 200, description = "Today's study time retrieved successfully", body = TodayStats),
        (status = 401, description = "Unauthorized - invalid or missing API key"),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "statistics"
)]
async fn get_today_stats(
    axum::extract::State(db_path): axum::extract::State<String>,
) -> Result<Json<TodayStats>, AppError> {
    let minutes = get_today_study_time(&db_path)?;
    Ok(Json(TodayStats::new(minutes)))
}

/// Get daily study time for last 30 days
#[utoipa::path(
    get,
    path = "/api/stats/daily",
    responses(
        (status = 200, description = "Daily study time for last 30 days retrieved successfully", body = DailyStats),
        (status = 401, description = "Unauthorized - invalid or missing API key"),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "statistics"
)]
async fn get_daily_stats(
    axum::extract::State(db_path): axum::extract::State<String>,
) -> Result<Json<DailyStats>, AppError> {
    let daily_stats = get_last_30_days_stats(&db_path)?;
    Ok(Json(DailyStats::new(daily_stats)))
}

/// Get weekly study time for last 12 weeks
#[utoipa::path(
    get,
    path = "/api/stats/weekly",
    responses(
        (status = 200, description = "Weekly study time for last 12 weeks retrieved successfully", body = WeeklyStats),
        (status = 401, description = "Unauthorized - invalid or missing API key"),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "statistics"
)]
async fn get_weekly_stats(
    axum::extract::State(db_path): axum::extract::State<String>,
) -> Result<Json<WeeklyStats>, AppError> {
    let weekly_stats = get_last_12_weeks_stats(&db_path)?;
    Ok(Json(WeeklyStats::new(weekly_stats)))
}

/// Custom error type for API errors
struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse::new(format!("{:#}", self.0))),
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
