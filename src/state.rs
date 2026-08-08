use std::sync::Arc;

/// Shared application state passed to handlers via extractors.
#[derive(Clone)]
pub struct AppState {
    /// Public domain the site is served under.
    pub domain: Arc<str>,
}
