use std::sync::Arc;
use crate::{config, data_source};

#[derive(Debug)]
pub struct AppState {
    pub resource: String,
    pub pool: Arc<sqlx::Pool<sqlx::Postgres>>,
}

pub async fn init(settings: &config::Settings) -> Arc<AppState> {
    let pool = data_source::postgres::init(&settings.database)
        .await
        .expect("failed to init db pool");
    Arc::new(AppState {
        resource: settings.app_resource.clone(),
        pool: Arc::new(pool),
    })
}