use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;

pub async fn process(req: Request, next: Next) -> Response {
    tracing::info!("incoming request: {:?}", req.uri());
    next.run(req).await
}