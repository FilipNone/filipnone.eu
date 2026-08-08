pub mod config;
pub mod routes;
pub mod state;
pub mod templates;

use axum::Router;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

use crate::state::AppState;

/// Build the application router with the given shared state.
pub fn app(state: AppState) -> Router {
    Router::new()
        .route("/", axum::routing::get(routes::home))
        .route("/panel", axum::routing::get(routes::panel))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(state)
        .layer(TraceLayer::new_for_http())
}
