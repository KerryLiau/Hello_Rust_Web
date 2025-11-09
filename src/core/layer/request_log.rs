use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
use tracing::info;

pub async fn process(req: Request, next: Next) -> Response {
    info!("incoming request"); // 這行會自動綁定 span
    next.run(req).await
}