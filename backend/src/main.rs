use ankistats::{
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
use faithstats::{
    get_faith_daily_stats,
    models::{FaithDailyStats, FaithDailySummary, FaithDayStats},
};
use std::env;
use tower_http::cors::CorsLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

/// Application configuration holding database paths
#[derive(Clone)]
struct AppConfig {
    anki_db_path: String,
    koreader_db_path: String,
}

/// OpenAPI documentation structure
#[derive(OpenApi)]
#[openapi(
    paths(
        health_check,
        get_books_stats,
        get_today_stats,
        get_daily_stats,
        get_weekly_stats,
        get_faith_daily_stats_endpoint,
    ),
    components(
        schemas(HealthCheck, BibleStats, TodayStats, DailyStats, WeeklyStats,
                BookStats, AggregateStats, DayStats, DailySummary, WeekStats, WeeklySummary,
                FaithDailyStats, FaithDailySummary, FaithDayStats, ErrorResponse)
    ),
    tags(
        (name = "health", description = "Health check endpoints"),
        (name = "anki", description = "Anki Bible memorization statistics endpoints"),
        (name = "faith", description = "Unified faith statistics endpoints combining multiple sources")
    ),
    info(
        title = "Life Stats API",
        description = "REST API for personal life and faith statistics.",
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
    // Load environment variables from .env file if present
    dotenvy::dotenv().ok();

    // Get configuration from environment variables
    let anki_db_path = env::var("ANKI_DATABASE_PATH").unwrap_or_else(|_| {
        eprintln!("Error: ANKI_DATABASE_PATH environment variable is required");
        std::process::exit(1);
    });

    let koreader_db_path = env::var("KOREADER_DATABASE_PATH").unwrap_or_else(|_| {
        eprintln!("Error: KOREADER_DATABASE_PATH environment variable is required");
        std::process::exit(1);
    });

    let api_key = env::var("API_KEY").unwrap_or_else(|_| {
        eprintln!("Error: API_KEY environment variable is required");
        std::process::exit(1);
    });

    // Validate that the database paths exist
    if !std::path::Path::new(&anki_db_path).exists() {
        eprintln!("Error: Anki database file not found at: {}", anki_db_path);
        std::process::exit(1);
    }

    if !std::path::Path::new(&koreader_db_path).exists() {
        eprintln!(
            "Error: KOReader database file not found at: {}",
            koreader_db_path
        );
        std::process::exit(1);
    }

    let config = AppConfig {
        anki_db_path: anki_db_path.clone(),
        koreader_db_path: koreader_db_path.clone(),
    };

    println!("Starting life stats API server...");
    println!("Anki Database: {}", anki_db_path);
    println!("KOReader Database: {}", koreader_db_path);

    // Build the router with routes
    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/openapi.json", ApiDoc::openapi()))
        .route("/health", get(health_check))
        .route("/api/anki/books", get(get_books_stats))
        .route("/api/anki/today", get(get_today_stats))
        .route("/api/anki/daily", get(get_daily_stats))
        .route("/api/anki/weekly", get(get_weekly_stats))
        .route("/api/faith/daily", get(get_faith_daily_stats_endpoint))
        .layer(middleware::from_fn(move |req, next| {
            auth_middleware(req, next, api_key.clone())
        }))
        .layer(CorsLayer::permissive())
        .with_state(config);

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
    path = "/api/anki/books",
    responses(
        (status = 200, description = "Bible book statistics retrieved successfully", body = BibleStats),
        (status = 401, description = "Unauthorized - invalid or missing API key"),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "anki"
)]
async fn get_books_stats(
    axum::extract::State(config): axum::extract::State<AppConfig>,
) -> Result<Json<BibleStats>, AppError> {
    let stats = get_bible_stats(&config.anki_db_path)?;
    Ok(Json(stats))
}

/// Get today's study time
#[utoipa::path(
    get,
    path = "/api/anki/today",
    responses(
        (status = 200, description = "Today's study time retrieved successfully", body = TodayStats),
        (status = 401, description = "Unauthorized - invalid or missing API key"),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "anki"
)]
async fn get_today_stats(
    axum::extract::State(config): axum::extract::State<AppConfig>,
) -> Result<Json<TodayStats>, AppError> {
    let minutes = get_today_study_time(&config.anki_db_path)?;
    Ok(Json(TodayStats::new(minutes)))
}

/// Get daily study time for last 30 days
#[utoipa::path(
    get,
    path = "/api/anki/daily",
    responses(
        (status = 200, description = "Daily study time for last 30 days retrieved successfully", body = DailyStats),
        (status = 401, description = "Unauthorized - invalid or missing API key"),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "anki"
)]
async fn get_daily_stats(
    axum::extract::State(config): axum::extract::State<AppConfig>,
) -> Result<Json<DailyStats>, AppError> {
    let daily_stats = get_last_30_days_stats(&config.anki_db_path)?;
    Ok(Json(DailyStats::new(daily_stats)))
}

/// Get weekly study time for last 12 weeks
#[utoipa::path(
    get,
    path = "/api/anki/weekly",
    responses(
        (status = 200, description = "Weekly study time for last 12 weeks retrieved successfully", body = WeeklyStats),
        (status = 401, description = "Unauthorized - invalid or missing API key"),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "anki"
)]
async fn get_weekly_stats(
    axum::extract::State(config): axum::extract::State<AppConfig>,
) -> Result<Json<WeeklyStats>, AppError> {
    let weekly_stats = get_last_12_weeks_stats(&config.anki_db_path)?;
    Ok(Json(WeeklyStats::new(weekly_stats)))
}

/// Get unified faith statistics for last 30 days
#[utoipa::path(
    get,
    path = "/api/faith/daily",
    responses(
        (status = 200, description = "Unified faith statistics for last 30 days retrieved successfully", body = FaithDailyStats),
        (status = 401, description = "Unauthorized - invalid or missing API key"),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "faith"
)]
async fn get_faith_daily_stats_endpoint(
    axum::extract::State(config): axum::extract::State<AppConfig>,
) -> Result<Json<FaithDailyStats>, AppError> {
    let stats = get_faith_daily_stats(&config.anki_db_path, &config.koreader_db_path)?;
    Ok(Json(stats))
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
