mod api;
pub mod core;

use axum::{middleware, Router};
use std::sync::Arc;
use crate::api::employee;
use tower::ServiceBuilder;

struct AppState {
    resource: String,
}

#[tokio::main]
async fn main() {
    init_server();
    let state = init_state();
    let app = Router::new()
        .nest("/employee", employee::router::index::router(state.clone()))
        .route_layer(
            ServiceBuilder::new()
                .layer(middleware::from_fn(core::layer::request_log::process))
                .layer(middleware::from_fn(core::layer::auth::process))
        );
    run(app).await;
}

fn init_server() {
    tracing_subscriber::fmt::init();
}

async fn run(app: Router) {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

fn init_state() -> Arc<AppState> {
    Arc::new(AppState{
        resource: "Hello World".to_string(),
    })
}
