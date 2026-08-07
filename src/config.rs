/// Application configuration loaded from the environment.
pub struct Config {
    /// Port the server listens on.
    pub port: u16,
}

impl Config {
    /// Build a config from environment variables, using sensible defaults.
    pub fn from_env() -> Self {
        let port = std::env::var("PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(3000);
        Self { port }
    }
}
