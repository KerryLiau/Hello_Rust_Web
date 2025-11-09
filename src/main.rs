mod api;
pub mod core;
mod data_source;

use crate::api::employee;
use axum::{middleware, Router};
use opentelemetry::global;
use opentelemetry::trace::TracerProvider;
use std::sync::Arc;
use std::time::Duration;
use tower::ServiceBuilder;

#[derive(Debug)]
struct AppState {
    resource: String,
    pool: Arc<sqlx::Pool<sqlx::Postgres>>,
}

#[tokio::main]
async fn main() {
    let tracer_provider = init_server();
    let state = init_state().await;
    let app = Router::new()
        .layer(TraceLayer::new_for_http())
        .nest("/employee", employee::router(state.clone()))
        .route_layer(
            ServiceBuilder::new()
                .layer(middleware::from_fn(core::layer::request_log::process))
                .layer(middleware::from_fn(core::layer::auth::process))
        );
    run(app).await;

    // CRITICAL: Shutdown tracer provider to flush remaining spans
    tracer_provider.shutdown().expect("Failed to shutdown tracer provider");
}

use opentelemetry_otlp::SpanExporter;
use opentelemetry_sdk::trace::SdkTracerProvider;
use tower_http::trace::TraceLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

fn init_server() -> SdkTracerProvider {
    use opentelemetry_sdk::Resource;
    use opentelemetry_otlp::WithExportConfig;

    let exporter = SpanExporter::builder()
        .with_tonic()
        .with_timeout(Duration::from_secs(3))
        .with_endpoint("http://localhost:4317")
        .build()
        .expect("failed to init exporter");

    // Create resource with service name - this is CRITICAL for Jaeger to identify traces
    let resource = Resource::builder()
        .with_service_name("hello-rust-web")
        .build();

    // Use simple_exporter for immediate sending (testing)
    let provider = SdkTracerProvider::builder()
        .with_batch_exporter(exporter)
        .with_resource(resource)
        .build();

    let tracer = provider.tracer("hello-world");

    // Clone provider before setting globally so we can shutdown later
    global::set_tracer_provider(provider.clone());
    global::set_text_map_propagator(opentelemetry_sdk::propagation::TraceContextPropagator::new());

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new("info"))  // debug to see export logs
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_opentelemetry::layer().with_tracer(tracer))
        .init();

    provider
}

async fn run(app: Router) {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn init_state() -> Arc<AppState> {
    let pool = data_source::postgres::init().await.expect("failed to init db pool");
    Arc::new(AppState {
        resource: "Hello World".to_string(),
        pool: Arc::new(pool),
    })
}
