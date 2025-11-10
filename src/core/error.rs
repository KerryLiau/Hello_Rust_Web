use std::any::Any;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use axum::body::Body;
use serde_json::json;
use tower_http::catch_panic::ResponseForPanic;

#[allow(dead_code)]
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

impl From<sqlx::Error> for ApiError {
    fn from(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::RowNotFound => ApiError::NotFound(e.to_string()),
            sqlx::Error::Database(db_err) => ApiError::InternalServerError(db_err.to_string()),
            sqlx::Error::Io(io_err) => ApiError::InternalServerError(io_err.to_string()),
            sqlx::Error::ColumnNotFound(_) => ApiError::NotFound(e.to_string()),
            sqlx::Error::PoolTimedOut => ApiError::InternalServerError(e.to_string()),
            sqlx::Error::PoolClosed => ApiError::InternalServerError(e.to_string()),
            sqlx::Error::InvalidArgument(_) => ApiError::BadRequest(e.to_string()),
            _ => ApiError::InternalServerError(e.to_string()),
        }
    }
}

impl From<std::io::Error> for ApiError {
    fn from(value: std::io::Error) -> Self {
        ApiError::InternalServerError(value.to_string())
    }
}

#[derive(Clone)]
pub struct MyPanicHandler;

impl ResponseForPanic for MyPanicHandler {
    type ResponseBody = Body;

    fn response_for_panic(&mut self, _err: Box<dyn Any + Send + 'static>) -> Response<Body> {
        // 不 log，因為 panic hook 已經 log 了
        Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from("Internal Server Error"))
            .unwrap()
    }
}

pub fn init_panic_handling() {
    std::panic::set_hook(Box::new(|panic_info| {
        let location = panic_info.location();
        tracing::error!(
            panic.file = location.map(|l| l.file()),
            panic.line = location.map(|l| l.line()),
            panic.message = %panic_info,
            "panic occurred"
        );
    }));
}
