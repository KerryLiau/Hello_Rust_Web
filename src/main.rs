mod api;
mod app_state;
mod core;
mod data_source;

use crate::api::employee;
use axum::{Router, middleware};
use tower::ServiceBuilder;
use tower_http::{catch_panic::CatchPanicLayer, trace::TraceLayer};

#[tokio::main]
async fn main() {
    core::error::init_panic_handling();
    let tracer_provider = core::otel::init();
    run_server().await;
    // CRITICAL: Shutdown tracer provider to flush remaining spans
    tracer_provider
        .shutdown()
        .expect("Failed to shutdown tracer provider");
}

async fn run_server() {
    let state = app_state::init().await;
    let router = Router::new()
        // routing api
        .nest("/employee", employee::router(state.clone()))
        // attach middleware to all routes
        .route_layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(middleware::from_fn(core::layer::request_log::process))
                .layer(middleware::from_fn(core::layer::auth::process))
                .layer(CatchPanicLayer::custom(core::error::MyPanicHandler)),
        );
    // start server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, router).await.unwrap();
}
