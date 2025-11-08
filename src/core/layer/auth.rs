use axum::{
    extract::Request,
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
};
use serde::{Deserialize, Serialize};
use tokio::task_local;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Auth {
    pub id: String,
}

task_local! {
    pub static USER: Auth;
}

pub async fn process(req: Request, next: Next) -> Result<Response, StatusCode> {
    let auth_header = find_auth_from_header(&req);
    if auth_header.is_err() {
        tracing::warn!("invalid auth header");
        return Err(StatusCode::UNAUTHORIZED);
    }
    let auth_header = auth_header?;
    if let Some(auth_data) = auth(auth_header) {
        tracing::info!("auth data: {:?}", auth_data);
        Ok(USER.scope(auth_data, next.run(req)).await)
    } else {
        tracing::warn!("invalid auth data");
        Err(StatusCode::UNAUTHORIZED)
    }
}

fn find_auth_from_header(req: &Request) -> Result<String, StatusCode> {
    let data = req.headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?
        .strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?
        .to_string();
    Ok(data)
}

fn auth(auth_header: String) -> Option<Auth> {
    Some(Auth{ id: auth_header })
}