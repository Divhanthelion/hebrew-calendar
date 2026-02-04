//! API Server Module
//! 
//! Axum-based HTTP API for Hebrew calendar calculations.

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use chrono::Datelike;
use hebrew_core::{CalendarError, DailyData, HebrewCalendar};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

use crate::config::AppConfig;

/// Shared application state
#[derive(Clone)]
pub struct ApiState {
    pub config: AppConfig,
}

/// Build the API router (extracted for testability)
pub fn build_router(config: AppConfig) -> Router {
    let state = Arc::new(ApiState { config });

    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/", get(root))
        .route("/api/v1/health", get(health_check))
        .route("/api/v1/calendar/convert", get(convert_date))
        .route("/api/v1/calendar/range", get(date_range))
        .route("/api/v1/zmanim", get(get_zmanim))
        .route("/api/v1/holidays/upcoming", get(upcoming_holidays))
        .layer(cors)
        .with_state(state)
}

/// Launch the API server
pub async fn launch(config: AppConfig, port: u16) -> anyhow::Result<()> {
    let app = build_router(config);

    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    tracing::info!("🕎 Hebrew Calendar API server running on http://{}", addr);
    tracing::info!("Try: curl 'http://{}/api/v1/calendar/convert?date=2024-01-01&lat=31.77&long=35.21'", addr);

    axum::serve(listener, app).await?;

    Ok(())
}

/// Root endpoint
async fn root() -> &'static str {
    "Hebrew Calendar API\n\nEndpoints:\n\
    - GET /api/v1/health\n\
    - GET /api/v1/calendar/convert?date=YYYY-MM-DD&lat=LAT&long=LNG\n\
    - GET /api/v1/calendar/range?start=YYYY-MM-DD&end=YYYY-MM-DD&lat=LAT&long=LNG\n\
    - GET /api/v1/zmanim?date=YYYY-MM-DD&lat=LAT&long=LNG&elevation=M\n\
    - GET /api/v1/holidays/upcoming?year=YYYY\n"
}

/// Health check endpoint
async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    version: String,
}

/// Date conversion request parameters
#[derive(Deserialize)]
pub struct ConvertRequest {
    /// ISO date string (supports extended format for year 0)
    date: String,
    /// Latitude for zmanim (optional)
    lat: Option<f64>,
    /// Longitude for zmanim (optional)
    long: Option<f64>,
    /// Elevation in meters (optional)
    elevation: Option<f64>,
    /// Candle lighting offset in minutes (default from config)
    candle_offset: Option<i64>,
}

/// Convert a single date
async fn convert_date(
    State(state): State<Arc<ApiState>>,
    Query(params): Query<ConvertRequest>,
) -> Result<Json<DailyData>, ApiError> {
    // Parse date
    let date = HebrewCalendar::parse_date(&params.date)
        .map_err(ApiError::from)?;
    
    // Build location if coordinates provided
    let location = if let (Some(lat), Some(long)) = (params.lat, params.long) {
        let mut loc = hebrew_core::zmanim::GeoLocation::new(lat, long)
            .map_err(ApiError::from)?;
        if let Some(elev) = params.elevation {
            loc = loc.with_elevation(elev);
        }
        Some(loc)
    } else {
        None
    };
    
    let candle_offset = params.candle_offset
        .unwrap_or(state.config.candle_lighting_offset_minutes);
    
    let data = HebrewCalendar::calculate_day(date, location, candle_offset)
        .map_err(ApiError::from)?;
    
    Ok(Json(data))
}

/// Date range request parameters
#[derive(Deserialize)]
pub struct RangeRequest {
    start: String,
    end: String,
    lat: Option<f64>,
    long: Option<f64>,
    elevation: Option<f64>,
    candle_offset: Option<i64>,
}

/// Convert a range of dates
async fn date_range(
    State(state): State<Arc<ApiState>>,
    Query(params): Query<RangeRequest>,
) -> Result<Json<Vec<DailyData>>, ApiError> {
    let start = HebrewCalendar::parse_date(&params.start)
        .map_err(ApiError::from)?;
    let end = HebrewCalendar::parse_date(&params.end)
        .map_err(ApiError::from)?;
    
    if end < start {
        return Err(ApiError::BadRequest("End date must be after start date".to_string()));
    }
    
    // Limit range to prevent abuse
    let days = (end - start).num_days();
    if days > 366 {
        return Err(ApiError::BadRequest(
            format!("Date range too large (max 366 days, requested {})", days)
        ));
    }
    
    let location = if let (Some(lat), Some(long)) = (params.lat, params.long) {
        let mut loc = hebrew_core::zmanim::GeoLocation::new(lat, long)
            .map_err(ApiError::from)?;
        if let Some(elev) = params.elevation {
            loc = loc.with_elevation(elev);
        }
        Some(loc)
    } else {
        None
    };
    
    let candle_offset = params.candle_offset
        .unwrap_or(state.config.candle_lighting_offset_minutes);
    
    let mut results = Vec::with_capacity(days as usize + 1);
    let mut current = start;
    
    while current <= end {
        let data = HebrewCalendar::calculate_day(current, location.clone(), candle_offset)
            .map_err(ApiError::from)?;
        results.push(data);
        current = current.succ_opt().unwrap();
    }
    
    Ok(Json(results))
}

/// Zmanim request parameters
#[derive(Deserialize)]
pub struct ZmanimRequest {
    date: String,
    lat: f64,
    long: f64,
    elevation: Option<f64>,
}

/// Get zmanim for a date
async fn get_zmanim(
    Query(params): Query<ZmanimRequest>,
) -> Result<Json<hebrew_core::zmanim::Zmanim>, ApiError> {
    let date = HebrewCalendar::parse_date(&params.date)
        .map_err(ApiError::from)?;
    
    let mut loc = hebrew_core::zmanim::GeoLocation::new(params.lat, params.long)
        .map_err(ApiError::from)?;
    if let Some(elev) = params.elevation {
        loc = loc.with_elevation(elev);
    }
    
    let calc = hebrew_core::zmanim::ZmanimCalculator::new(loc);
    let zmanim = calc.calculate(date)
        .map_err(ApiError::from)?;
    
    Ok(Json(zmanim))
}

/// Upcoming holidays request
#[derive(Deserialize)]
pub struct HolidaysRequest {
    year: Option<i32>,
}

/// Get upcoming holidays for a year
async fn upcoming_holidays(
    Query(params): Query<HolidaysRequest>,
) -> Result<Json<Vec<HolidayInfo>>, ApiError> {
    use chrono::NaiveDate;
    use hebrew_core::calendar::{DateConverter, HebrewDate, HebrewMonth};
    use hebrew_core::holidays::{Holiday, HolidayCalculator};
    
    let year = params.year.unwrap_or_else(|| {
        chrono::Local::now().year()
    });
    
    let mut holidays = Vec::new();
    
    // Get holidays for the entire Hebrew year
    // Find Rosh Hashanah of the Gregorian year
    let rosh_hashanah_gregorian = if year >= 1 {
        NaiveDate::from_ymd_opt(year, 9, 1).unwrap() // Approximate
    } else {
        NaiveDate::from_ymd_opt(0, 9, 1).unwrap()
    };
    
    // This is a simplified implementation
    // A full implementation would iterate through the Hebrew year
    
    Ok(Json(holidays))
}

#[derive(Serialize)]
pub struct HolidayInfo {
    name: String,
    hebrew_date: String,
    gregorian_date: String,
    is_yom_tov: bool,
}

/// API error type
#[derive(Debug)]
pub enum ApiError {
    BadRequest(String),
    Calendar(CalendarError),
}

impl From<CalendarError> for ApiError {
    fn from(err: CalendarError) -> Self {
        ApiError::Calendar(err)
    }
}

impl axum::response::IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::Calendar(err) => {
                let msg = err.to_string();
                let status = match err {
                    CalendarError::DateOutOfRange(_) => StatusCode::BAD_REQUEST,
                    CalendarError::InvalidDateFormat(_) => StatusCode::BAD_REQUEST,
                    CalendarError::InvalidLatitude(_) => StatusCode::BAD_REQUEST,
                    CalendarError::InvalidLongitude(_) => StatusCode::BAD_REQUEST,
                    CalendarError::CalculationError(_) => StatusCode::INTERNAL_SERVER_ERROR,
                };
                (status, msg)
            }
        };
        
        let body = Json(ErrorResponse { error: message });
        (status, body).into_response()
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode as HttpStatusCode};
    use tower::ServiceExt;

    fn test_app() -> Router {
        build_router(AppConfig::default())
    }

    #[tokio::test]
    async fn test_health_check() {
        let app = test_app();
        let response = app
            .oneshot(Request::builder().uri("/api/v1/health").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), HttpStatusCode::OK);
    }

    #[tokio::test]
    async fn test_root_endpoint() {
        let app = test_app();
        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), HttpStatusCode::OK);
    }

    #[tokio::test]
    async fn test_convert_date_happy_path() {
        let app = test_app();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v1/calendar/convert?date=2024-01-01")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), HttpStatusCode::OK);
    }

    #[tokio::test]
    async fn test_convert_date_with_coords() {
        let app = test_app();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v1/calendar/convert?date=2024-01-01&lat=31.77&long=35.21")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), HttpStatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let data: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert!(data.get("zmanim").is_some(), "Should have zmanim with coords");
    }

    #[tokio::test]
    async fn test_convert_date_invalid() {
        let app = test_app();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v1/calendar/convert?date=not-a-date")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), HttpStatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_date_range_happy_path() {
        let app = test_app();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v1/calendar/range?start=2024-01-01&end=2024-01-07")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), HttpStatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let data: Vec<serde_json::Value> = serde_json::from_slice(&body).unwrap();
        assert_eq!(data.len(), 7, "7-day range should return 7 items");
    }

    #[tokio::test]
    async fn test_date_range_too_large() {
        let app = test_app();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v1/calendar/range?start=2024-01-01&end=2026-01-01")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), HttpStatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_zmanim_endpoint() {
        let app = test_app();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v1/zmanim?date=2024-06-15&lat=31.77&long=35.21")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), HttpStatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let data: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert!(data.get("sunrise").is_some());
        assert!(data.get("sunset").is_some());
    }
}
