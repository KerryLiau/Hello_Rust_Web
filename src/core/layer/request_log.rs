use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
use tracing::{info, instrument};

#[instrument(skip_all)]
pub async fn process(req: Request, next: Next) -> Response {
    info!("incoming request: {}", req.uri()); // 這行會自動綁定 span
    next.run(req).await
}