use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};

use crate::templates;

/// The public homepage, rendered from the index template.
pub async fn home() -> impl IntoResponse {
    let tera = templates::engine();
    let body = tera
        .render("index.html", &tera::Context::new())
        .unwrap_or_else(|e| {
            tracing::error!("failed to render index.html: {e}");
            "internal server error".to_string()
        });
    (StatusCode::OK, Html(body))
}
