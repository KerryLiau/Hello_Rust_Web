use crate::core::error::ApiError;
use crate::employee::model::user::Resp as UserResp;
use crate::{data_source::postgres};
use axum::extract::{Path, State};
use std::sync::Arc;
use axum::Json;
use tracing::instrument;
use crate::app_state::AppState;

#[instrument(skip(state))]
pub async fn get_by_id(
    Path(id): Path<i32>,
    State(state): State<Arc<AppState>>
) -> Result<UserResp , ApiError> {
    get_by_id_private(Path(id), State(state)).await
}

// rust 的 opentelemetry 因為是使用 macro 於編譯期靜態展開
// 所以連私有方法都可以被紀錄！！！
#[instrument(skip(state))]
async fn get_by_id_private(
    Path(id): Path<i32>,
    State(state): State<Arc<AppState>>
) -> Result<UserResp, ApiError> {
    tracing::info!("Getting user by ID");
    let user = postgres::users::get_by_id(id, &state.pool).await?;
    Ok(UserResp::from(user))
}

#[instrument(skip(state))]
#[axum::debug_handler]
pub async fn update_by_id(
    Path(id): Path<i32>,
    State(state): State<Arc<AppState>>,
    Json(content): Json<postgres::users::UpdateModel>,
) -> Result<UserResp, ApiError> {
    let user = postgres::users::update_by_id(id, content, &state.pool).await?;
    Ok(UserResp::from(user))
}