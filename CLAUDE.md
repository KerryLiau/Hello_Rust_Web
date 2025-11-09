# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust web application built with Axum framework, featuring PostgreSQL database integration, OpenTelemetry tracing with Jaeger, and a layered architecture with middleware-based authentication and request logging.

## Development Commands

### Running the Application
```bash
cargo run
```
Server runs on `http://127.0.0.1:3000`

### Building
```bash
cargo build          # Debug build
cargo build --release # Production build
```

### Testing
```bash
cargo test           # Run all tests
cargo test <test_name> # Run specific test
```

### Database Setup
The application expects a PostgreSQL database. Configure via environment variable:
```bash
export DATABASE_URL="postgres://postgres:@localhost:5432"
```

### Jaeger/OpenTelemetry Setup
Required for tracing. Quick start:
```bash
docker rm -f jaeger
docker run -d --rm --name jaeger \
  -e COLLECTOR_OTLP_ENABLED=true \
  -p 16686:16686 \
  -p 4317:4317 \
  -p 4318:4318 \
  -p 5778:5778 \
  -p 9411:9411 \
  cr.jaegertracing.io/jaegertracing/jaeger:2.11.0
```
Jaeger UI available at `http://localhost:16686`

## Architecture

### Layer-Based Request Processing
Requests flow through middleware layers applied in `main.rs`:
1. **TraceLayer** - HTTP tracing via tower-http
2. **request_log** middleware (`core::layer::request_log`) - Logs incoming requests with OpenTelemetry spans
3. **auth** middleware (`core::layer::auth`) - Bearer token authentication, stores user data in task-local storage

Middleware order matters - they are applied in reverse order of declaration in `route_layer()`.

### Authentication Pattern
The auth layer uses Tokio's `task_local!` to store authenticated user data:
- `USER` task-local variable (`core::layer::auth::USER`) contains `Auth` struct with user ID
- Access in handlers via `USER.with(|auth| auth.id.clone())`
- All routes under `/employee` require Bearer token authentication
- Token is currently stored as-is in the `Auth.id` field (simplified auth)

### Module Structure
- **`api/`** - API endpoints organized by domain (e.g., `employee`)
  - Each domain has: `model/`, `services/`, and router setup in `mod.rs`
  - Services layer calls data_source layer
- **`core/`** - Shared infrastructure
  - `layer/` - Axum middleware (auth, request logging)
  - `error.rs` - Centralized `ApiError` enum with `IntoResponse` implementation
- **`data_source/`** - Database access layer
  - `postgres/` - SQLx-based data access, organized by entity (e.g., `users/`)
  - Database pool initialization and connection management

### State Management
`AppState` struct in `main.rs` contains shared application state:
- Database connection pool (`Arc<sqlx::Pool<sqlx::Postgres>>`)
- Resource string (example state)
- Wrapped in `Arc` and passed to routers via `.with_state()`

### Error Handling Pattern
- `ApiError` enum in `core/error.rs` provides structured error responses
- Implements `IntoResponse` for automatic JSON error responses
- Database errors are mapped to appropriate HTTP status codes (e.g., `RowNotFound` → `404`)
- Internal errors sanitize messages in output via `message_for_output()`

### Database Layer Pattern
Each entity (e.g., `users`) has:
- Entity struct with `#[derive(sqlx::FromRow)]` in `data_source/postgres/entity/`
- Query functions returning `Result<Entity, ApiError>`
- SQLx error mapping to `ApiError` variants

### OpenTelemetry Setup
Configured in `init_server()` in `main.rs`:
- OTLP exporter connects to Jaeger at `http://localhost:4317`
- Tracer provider with batch exporter
- TraceContext propagation for distributed tracing
- Tracing subscriber with debug-level filtering
