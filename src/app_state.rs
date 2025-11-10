use std::sync::Arc;
use crate::data_source;

#[derive(Debug)]
pub struct AppState {
    pub resource: String,
    pub pool: Arc<sqlx::Pool<sqlx::Postgres>>,
}

pub async fn init() -> Arc<AppState> {
    let pool = data_source::postgres::init()
        .await
        .expect("failed to init db pool");
    Arc::new(AppState {
        resource: "Hello World".to_string(),
        pool: Arc::new(pool),
    })
}