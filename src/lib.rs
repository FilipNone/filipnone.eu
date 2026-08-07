pub mod config;
pub mod routes;
pub mod templates;

use axum::Router;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

/// Build the application router.
pub fn app() -> Router {
    Router::new()
        .route("/", axum::routing::get(routes::home))
        .nest_service("/static", ServeDir::new("static"))
        .layer(TraceLayer::new_for_http())
}
