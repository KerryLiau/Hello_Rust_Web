use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::env;

/// Database configuration
#[derive(Debug, Clone, Deserialize)]
pub struct Database {
    /// Database connection URL (e.g., postgres://user:password@localhost:5432/dbname)
    #[serde(default = "default_url")]
    pub url: String,
    /// Maximum number of connections in the pool
    pub max_connections: u32,
    /// Minimum number of connections in the pool
    pub min_connections: u32,
    /// Idle timeout in seconds
    pub idle_timeout_secs: u64,
    /// Maximum lifetime in seconds
    pub max_lifetime_secs: u64,
    /// Acquire timeout in seconds
    pub acquire_timeout_secs: u64,
}

fn default_url() -> String {
    "postgres://postgres:@localhost:5432".to_string()
}

/// Server configuration
#[derive(Debug, Clone, Deserialize)]
pub struct Server {
    /// Server host address (e.g., 127.0.0.1, 0.0.0.0)
    pub host: String,
    /// Server port number
    pub port: u16,
}

/// OpenTelemetry configuration
#[derive(Debug, Clone, Deserialize)]
pub struct OpenTelemetry {
    /// OTLP endpoint URL (e.g., http://localhost:4317)
    pub endpoint: String,
    /// Service name for tracing
    pub service_name: String,
    /// Export timeout in seconds
    pub timeout_secs: u64,
    /// Log level (e.g., info, debug, warn, error)
    pub log_level: String,
}

/// Application settings
#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    /// Database configuration
    pub database: Database,
    /// Server configuration
    pub server: Server,
    /// OpenTelemetry configuration
    pub otel: OpenTelemetry,
    /// Application resource string
    pub app_resource: String,
}

impl Settings {
    /// Load configuration from files and environment variables
    ///
    /// Configuration is layered in the following order (later sources override earlier ones):
    /// 1. config/default.toml - Default configuration
    /// 2. config/{RUN_MODE}.toml - Environment-specific config (e.g., development, production)
    /// 3. config/local.toml - Local overrides (optional, not in version control)
    /// 4. Environment variables with APP_ prefix
    ///
    /// Example environment variables:
    /// - APP_DATABASE_URL=postgres://localhost/mydb
    /// - APP_SERVER_PORT=9000
    /// - APP_OTEL_ENDPOINT=http://jaeger:4317
    pub fn load() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let config = Config::builder()
            // Load base configuration
            .add_source(File::with_name("config/default"))
            // Load environment-specific config (optional)
            .add_source(
                File::with_name(&format!("config/{}", run_mode))
                    .required(false)
            )
            // Load local overrides (optional, not in version control)
            .add_source(File::with_name("config/local").required(false))
            // Override with environment variables prefixed with APP_
            // Example: APP_DATABASE_URL=postgres://localhost/mydb
            .add_source(
                Environment::with_prefix("APP")
                    .separator("_")
                    .try_parsing(true)
            )
            .build()?;

        config.try_deserialize()
    }
}
