use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};

use crate::state::AppState;
use crate::templates;

/// The public homepage, rendered from the index template.
pub async fn home(State(state): State<AppState>) -> impl IntoResponse {
    let tera = templates::engine();
    let mut context = tera::Context::new();
    context.insert("domain", &*state.domain);
    let body = tera.render("index.html", &context).unwrap_or_else(|e| {
        tracing::error!("failed to render index.html: {e}");
        "internal server error".to_string()
    });
    (StatusCode::OK, Html(body))
}

/// The admin panel, always available at /panel even if the homepage breaks.
pub async fn panel(State(state): State<AppState>) -> impl IntoResponse {
    let tera = templates::engine();
    let mut context = tera::Context::new();
    context.insert("domain", &*state.domain);
    let body = tera.render("panel.html", &context).unwrap_or_else(|e| {
        tracing::error!("failed to render panel.html: {e}");
        "internal server error".to_string()
    });
    (StatusCode::OK, Html(body))
}
