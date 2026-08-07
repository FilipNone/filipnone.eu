pub mod config;
pub mod routes;

use axum::Router;
use tower_http::trace::TraceLayer;

/// Build the application router.
pub fn app() -> Router {
    Router::new()
        .route("/", axum::routing::get(routes::home))
        .layer(TraceLayer::new_for_http())
}
