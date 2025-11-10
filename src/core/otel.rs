use opentelemetry::global;
use opentelemetry::trace::TracerProvider;
use opentelemetry_otlp::SpanExporter;
use opentelemetry_sdk::trace::SdkTracerProvider;
use std::time::Duration;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub fn init() -> SdkTracerProvider {
    use opentelemetry_otlp::WithExportConfig;
    use opentelemetry_sdk::Resource;

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
        .with(tracing_subscriber::EnvFilter::new("info")) // debug to see export logs
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_opentelemetry::layer().with_tracer(tracer))
        .init();

    provider
}
