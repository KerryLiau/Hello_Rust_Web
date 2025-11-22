//! Hello Rust Web 的自訂認證 middleware
//!
//! 這是把原本 core::layer::auth 的邏輯包裝成 SDK trait 實作

use axum::{
    extract::Request,
    http::header,
    middleware::Next,
    response::Response,
};
use axum::response::IntoResponse;
use rust_web_sdk::middleware::traits::{AuthMiddleware, MiddlewareFuture};
use serde::{Deserialize, Serialize};
use tokio::task_local;
use tracing::{instrument, info, warn};
use crate::core::error::ApiError;

/// 認證資料 - 與原本的 core::layer::auth::Auth 相同
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Auth {
    pub id: String,
}

task_local! {
    /// Task-local 儲存認證用戶
    pub static USER: Auth;
}

/// 取得當前認證用戶的 ID
pub fn get_current_user_id() -> String {
    USER.with(|auth| auth.id.clone())
}

/// Hello Rust Web 的認證 middleware
///
/// 保留原本的 Bearer token 認證邏輯
pub struct HelloRustWebAuth;

impl HelloRustWebAuth {
    pub fn new() -> Self {
        Self
    }

    /// 從 header 提取 Bearer token
    fn extract_bearer_token(req: &Request) -> Result<String, ApiError> {
        let data = req
            .headers()
            .get(header::AUTHORIZATION)
            .and_then(|header| header.to_str().ok())
            .ok_or(ApiError::Unauthorized("No auth header".to_string()))?
            .strip_prefix("Bearer ")
            .ok_or(ApiError::Unauthorized(
                "Incorrect auth header format".to_string(),
            ))?
            .to_string();
        Ok(data)
    }

    /// 認證邏輯 - 目前簡化為直接使用 token 作為 user ID
    fn authenticate(token: String) -> Option<Auth> {
        // TODO: 在這裡加入你的認證邏輯
        // 例如：驗證 JWT、查詢資料庫等
        Some(Auth { id: token })
    }
}

impl Default for HelloRustWebAuth {
    fn default() -> Self {
        Self::new()
    }
}

/// 實作 SDK 的 AuthMiddleware trait
impl AuthMiddleware for HelloRustWebAuth {
    #[instrument(skip_all)]
    fn process(&self, req: Request, next: Next) -> MiddlewareFuture<'_> {
        Box::pin(async move {
            // 提取 Bearer token
            match Self::extract_bearer_token(&req) {
                Ok(token) => {
                    // 認證
                    if let Some(auth_data) = Self::authenticate(token) {
                        info!("auth data: {:?}", auth_data);
                        // 認證成功，使用 task_local 儲存用戶資訊
                        USER.scope(auth_data, next.run(req)).await
                    } else {
                        warn!("invalid auth data");
                        // 認證失敗，返回 401
                        ApiError::Unauthorized("unauthorized".to_string()).into_response()
                    }
                }
                Err(err) => {
                    // 缺少或格式錯誤的 header
                    err.into_response()
                }
            }
        })
    }
}
