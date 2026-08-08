/// Application configuration loaded from the environment.
pub struct Config {
    /// Port the server listens on.
    pub port: u16,
    /// Public domain the site is served under.
    pub domain: String,
}

impl Config {
    /// Build a config from environment variables, using sensible defaults.
    pub fn from_env() -> Self {
        let port = std::env::var("PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(3000);
        let domain = std::env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string());
        Self { port, domain }
    }
}
