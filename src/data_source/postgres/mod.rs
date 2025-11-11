pub mod users;

use std::time::Duration;
use sqlx::Pool;
use sqlx::postgres::PgPoolOptions;

pub async fn init() -> Result<Pool<sqlx::Postgres>, sqlx::Error> {
    let db_conn_str = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:@localhost:5432".to_string());
    let pool = PgPoolOptions::new()
        .max_connections(3)
        .min_connections(1)
        .idle_timeout(Duration::from_mins(5))
        .max_lifetime(Duration::from_mins(30))
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_conn_str)
        .await?;
    Ok(pool)
}
