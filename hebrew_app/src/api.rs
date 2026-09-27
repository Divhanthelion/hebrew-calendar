//! The web server: the calendar's page at `/`, and a JSON API under `/api/v1`.

use axum::{
    extract::{RawQuery, State},
    http::{header, StatusCode},
    response::{IntoResponse, Json, Response},
    routing::get,
    Router,
};
use hebrew_core::CalendarError;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

use crate::config::AppConfig;
use crate::frontend;
use crate::query::{self, DayQuery, DaysQuery, HebrewDateQuery, HolidaysQuery};

type Shared = State<Arc<AppConfig>>;

/// The router, separate from [`serve`] so tests can call it directly.
pub fn router(config: AppConfig) -> Router {
    Router::new()
        .route(
            "/",
            get(|| async { page(frontend::INDEX, "text/html; charset=utf-8") }),
        )
        .route(
            "/app.js",
            get(|| async { page(frontend::APP_JS, "text/javascript; charset=utf-8") }),
        )
        .route(
            "/app.css",
            get(|| async { page(frontend::APP_CSS, "text/css; charset=utf-8") }),
        )
        .route(
            "/icon.svg",
            get(|| async { page(frontend::ICON, "image/svg+xml") }),
        )
        .route("/api/v1/health", get(health))
        .route("/api/v1/day", get(day))
        .route("/api/v1/days", get(days))
        .route("/api/v1/holidays", get(holidays))
        .route("/api/v1/hebrew-to-gregorian", get(hebrew_to_gregorian))
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any))
        .with_state(Arc::new(config))
}

/// Serve on `host:port` until stopped.
pub async fn serve(config: AppConfig, host: &str, port: u16) -> anyhow::Result<()> {
    let listener = tokio::net::TcpListener::bind((host, port)).await?;
    let address = listener.local_addr()?;
    println!("Hebrew Calendar is running at http://{address}");
    println!("API: http://{address}/api/v1/days?start=2025-09-01&end=2025-09-30&lat=40.71&lng=-74.01&tz=America/New_York");
    axum::serve(listener, router(config)).await?;
    Ok(())
}

fn page(body: &'static str, content_type: &'static str) -> Response {
    ([(header::CONTENT_TYPE, content_type)], body).into_response()
}

async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({ "status": "ok", "version": env!("CARGO_PKG_VERSION") }))
}

async fn day(
    State(config): Shared,
    RawQuery(raw): RawQuery,
) -> Result<Json<impl Serialize>, ApiError> {
    let q: DayQuery = from_query(raw)?;
    Ok(Json(query::day(&q, &config)?))
}

async fn days(
    State(config): Shared,
    RawQuery(raw): RawQuery,
) -> Result<Json<impl Serialize>, ApiError> {
    let q: DaysQuery = from_query(raw)?;
    Ok(Json(query::days(&q, &config)?))
}

async fn holidays(
    State(config): Shared,
    RawQuery(raw): RawQuery,
) -> Result<Json<impl Serialize>, ApiError> {
    let q: HolidaysQuery = from_query(raw)?;
    Ok(Json(query::holidays(&q, &config)?))
}

async fn hebrew_to_gregorian(RawQuery(raw): RawQuery) -> Result<Json<impl Serialize>, ApiError> {
    let q: HebrewDateQuery = from_query(raw)?;
    Ok(Json(query::hebrew_to_gregorian(&q)?))
}

/// Read a query string into the same request types the desktop app sends
/// as JSON: numbers and true/false become JSON numbers and booleans, except
/// for fields that are always text.
fn from_query<T: DeserializeOwned>(raw: Option<String>) -> Result<T, ApiError> {
    const TEXT: [&str; 7] = ["start", "end", "date", "tz", "name", "month", "place"];
    let pairs: Vec<(String, String)> = serde_urlencoded::from_str(raw.as_deref().unwrap_or(""))
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;
    let mut object = serde_json::Map::new();
    for (key, value) in pairs {
        let json = if TEXT.contains(&key.as_str()) {
            serde_json::Value::String(value)
        } else if let Ok(b) = value.parse::<bool>() {
            serde_json::Value::Bool(b)
        } else if let Ok(n) = value.parse::<i64>() {
            n.into()
        } else if let Some(n) = value
            .parse::<f64>()
            .ok()
            .and_then(serde_json::Number::from_f64)
        {
            serde_json::Value::Number(n)
        } else {
            serde_json::Value::String(value)
        };
        object.insert(key, json);
    }
    serde_json::from_value(serde_json::Value::Object(object))
        .map_err(|e| ApiError::BadRequest(e.to_string()))
}

/// Errors as `{"error": "..."}` with a 400 status.
#[derive(Debug)]
pub enum ApiError {
    BadRequest(String),
}

impl From<CalendarError> for ApiError {
    fn from(e: CalendarError) -> Self {
        ApiError::BadRequest(e.to_string())
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let ApiError::BadRequest(message) = self;
        (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": message })),
        )
            .into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::ServiceExt;

    async fn get(uri: &str) -> (StatusCode, String) {
        let response = router(AppConfig::default())
            .oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
            .await
            .unwrap();
        let status = response.status();
        let body = axum::body::to_bytes(response.into_body(), 1 << 22)
            .await
            .unwrap();
        (status, String::from_utf8(body.to_vec()).unwrap())
    }

    fn json(body: &str) -> serde_json::Value {
        serde_json::from_str(body).unwrap()
    }

    #[tokio::test]
    async fn serves_the_page() {
        let (status, body) = get("/").await;
        assert_eq!(status, StatusCode::OK);
        assert!(body.contains("<title>Hebrew Calendar</title>"));
        assert_eq!(get("/app.js").await.0, StatusCode::OK);
        assert_eq!(get("/app.css").await.0, StatusCode::OK);
    }

    #[tokio::test]
    async fn health() {
        let (status, body) = get("/api/v1/health").await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(json(&body)["status"], "ok");
    }

    #[tokio::test]
    async fn a_day_in_new_york_uses_daylight_time() {
        let (status, body) = get(
            "/api/v1/day?date=2025-07-04&lat=40.7128&lng=-74.006&tz=America/New_York&israel=false",
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{body}");
        let day = json(&body);
        assert_eq!(day["zmanim"]["utc_offset"], "-04:00");
        assert_eq!(day["candle_lighting"], "20:12");
        assert_eq!(day["hebrew_display"], "8 Tammuz 5785");
    }

    #[tokio::test]
    async fn a_month_of_days() {
        let (status, body) = get("/api/v1/days?start=2025-09-01&end=2025-09-30&nowhere=true").await;
        assert_eq!(status, StatusCode::OK, "{body}");
        let days = json(&body);
        assert_eq!(days.as_array().unwrap().len(), 30);
        assert!(days[0]["zmanim"].is_null());
    }

    #[tokio::test]
    async fn bad_requests_say_why() {
        let (status, body) = get("/api/v1/days?start=2025-01-01&end=2027-01-01").await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert!(json(&body)["error"].as_str().unwrap().contains("400 days"));
        let (status, body) = get("/api/v1/day?date=2025-01-01&lat=1&lng=1&tz=Nowhere/Land").await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert!(json(&body)["error"].as_str().unwrap().contains("time zone"));
        assert_eq!(
            get("/api/v1/day?date=yesterday").await.0,
            StatusCode::BAD_REQUEST
        );
    }

    #[tokio::test]
    async fn holidays_and_hebrew_dates() {
        let (status, body) =
            get("/api/v1/holidays?start=2026-04-01&end=2026-04-10&israel=true").await;
        assert_eq!(status, StatusCode::OK, "{body}");
        let names: Vec<String> = json(&body)
            .as_array()
            .unwrap()
            .iter()
            .map(|h| h["name"].as_str().unwrap().to_string())
            .collect();
        assert!(names.contains(&"Pesach I".to_string()));
        assert!(
            !names.contains(&"Pesach VIII".to_string()),
            "Israel keeps seven days"
        );
        let (status, body) = get("/api/v1/hebrew-to-gregorian?year=5786&month=Nisan&day=15").await;
        assert_eq!(status, StatusCode::OK, "{body}");
        assert_eq!(json(&body)["date"], "2026-04-02");
    }
}
