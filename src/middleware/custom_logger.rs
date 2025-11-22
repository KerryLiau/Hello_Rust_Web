//! Hello Rust Web 的自訂日誌 middleware
//!
//! 保留原本的 request logging 邏輯

use axum::{extract::Request, middleware::Next, response::Response};
use rust_web_sdk::middleware::traits::{LogMiddleware, MiddlewareFuture};
use tracing::{info, instrument};

/// Hello Rust Web 的日誌 middleware
pub struct HelloRustWebLogger;

impl HelloRustWebLogger {
    pub fn new() -> Self {
        Self
    }
}

impl Default for HelloRustWebLogger {
    fn default() -> Self {
        Self::new()
    }
}

/// 實作 SDK 的 LogMiddleware trait
impl LogMiddleware for HelloRustWebLogger {
    fn log_request(&self, req: &Request) {
        info!("incoming request: {}", req.uri());
    }

    fn log_response(&self, _req: &Request, res: &Response) {
        info!("response status: {}", res.status());
    }

    // 使用自訂的 process 實作，加入 OpenTelemetry span
    #[instrument(skip_all, fields(uri = %req.uri(), method = %req.method()))]
    fn process(&self, req: Request, next: Next) -> MiddlewareFuture<'_> {
        Box::pin(async move {
            let uri = req.uri().clone();
            let method = req.method().clone();

            info!("incoming request: {} {}", method, uri);
            let response = next.run(req).await;
            info!(
                "request completed: {} {} - status: {}",
                method,
                uri,
                response.status()
            );

            response
        })
    }
}
