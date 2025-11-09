use axum::{
    extract::Request,
    http::{header},
    middleware::Next,
    response::Response,
};
use serde::{Deserialize, Serialize};
use tokio::task_local;
use crate::core::error::ApiError;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Auth {
    pub id: String,
}

task_local! {
    pub static USER: Auth;
}

pub async fn process(req: Request, next: Next) -> Result<Response, ApiError> {
    let auth_header = find_auth_from_header(&req);
    if auth_header.is_err() {
        return Err(ApiError::Unauthorized("invalid auth header".to_string()));
    }
    let auth_header = auth_header?;
    if let Some(auth_data) = auth(auth_header) {
        tracing::info!("auth data: {:?}", auth_data);
        Ok(USER.scope(auth_data, next.run(req)).await)
    } else {
        tracing::warn!("invalid auth data");
        Err(ApiError::Unauthorized("unauthorized".to_string()))
    }
}

fn find_auth_from_header(req: &Request) -> Result<String, ApiError> {
    let data = req.headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .ok_or(ApiError::Unauthorized("No auth header".to_string()))?
        .strip_prefix("Bearer ")
        .ok_or(ApiError::Unauthorized("Incorrect auth header format".to_string()))?
        .to_string();
    Ok(data)
}

fn auth(auth_header: String) -> Option<Auth> {
    Some(Auth{ id: auth_header })
}