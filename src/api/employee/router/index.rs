use crate::core::layer::auth::USER;
use crate::AppState;
use axum::{
    Json, Router,
    routing::get,
    response::{IntoResponse, Response},
    extract::State,
    http::StatusCode
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub fn router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(index))
        .with_state(state)
}

#[derive(Serialize, Deserialize)]
struct Resp {
    val: String,
    user: String,
}

impl IntoResponse for Resp {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

async fn index(
    State(state): State<Arc<AppState>>
) -> Result<Resp, ()> {
    tracing::info!("employee index processing");
    let resp = Resp {
        val: state.resource.clone(),
        user: USER.with(|auth| auth.id.clone()),
    };
    Ok(resp)
}