use axum::http::StatusCode;
use axum::response::IntoResponse;

/// The public homepage.
pub async fn home() -> impl IntoResponse {
    (StatusCode::OK, "filipnone.eu")
}
