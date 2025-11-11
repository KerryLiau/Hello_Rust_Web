pub mod users;

use std::time::Duration;
use sqlx::Pool;
use sqlx::postgres::PgPoolOptions;
use crate::config::Database;

pub async fn init(db_config: &Database) -> Result<Pool<sqlx::Postgres>, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(db_config.max_connections)
        .min_connections(db_config.min_connections)
        .idle_timeout(Duration::from_secs(db_config.idle_timeout_secs))
        .max_lifetime(Duration::from_secs(db_config.max_lifetime_secs))
        .acquire_timeout(Duration::from_secs(db_config.acquire_timeout_secs))
        .connect(&db_config.url)
        .await?;
    Ok(pool)
}
