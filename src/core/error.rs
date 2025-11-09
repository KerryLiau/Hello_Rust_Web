use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

pub enum ApiError {
    Unauthorized(String),
    Forbidden(String),
    NotFound(String),
    BadRequest(String),
    Conflict(String),
    Locked(String),
    InternalServerError(String),
    BadGateway(String),
}

impl ApiError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            ApiError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            ApiError::Forbidden(_) => StatusCode::FORBIDDEN,
            ApiError::NotFound(_) => StatusCode::NOT_FOUND,
            ApiError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ApiError::Conflict(_) => StatusCode::CONFLICT,
            ApiError::Locked(_) => StatusCode::LOCKED,
            ApiError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::BadGateway(_) => StatusCode::BAD_GATEWAY,
        }
    }

    pub fn message(&self) -> String {
        match self {
            ApiError::Unauthorized(msg) => msg.to_string(),
            ApiError::Forbidden(msg) => msg.to_string(),
            ApiError::NotFound(msg) => msg.to_string(),
            ApiError::BadRequest(msg) => msg.to_string(),
            ApiError::Conflict(msg) => msg.to_string(),
            ApiError::Locked(msg) => msg.to_string(),
            ApiError::InternalServerError(msg) => msg.to_string(),
            ApiError::BadGateway(msg) => msg.to_string(),
        }
    }

    pub fn message_for_output(&self) -> String {
        match self {
            ApiError::InternalServerError(_msg) => "Internal Server Error".to_string(),
            _ => self.message()
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let body = Json(json!({
            "error": self.message_for_output()
        }));
        (self.status_code(), body).into_response()
    }
}
