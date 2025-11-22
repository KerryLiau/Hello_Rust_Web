mod api;
mod app_state;
mod config;
mod core;
mod data_source;
mod middleware;  // 新增：自訂 middleware

use crate::api::employee;
use axum::Router;
use rust_web_sdk::{
    middleware::MiddlewareBuilder,
    server::RustWebApplication,
};

#[tokio::main]
async fn main() {
    // ════════════════════════════════════════════════════════════
    // 🎯 方案 A：完全使用 SDK（最簡潔）
    // ════════════════════════════════════════════════════════════

    println!("Hello World");
    run_with_sdk().await;

    // ════════════════════════════════════════════════════════════
    // 🎯 方案 B：保留原有 config 和 app_state（更彈性）
    // ════════════════════════════════════════════════════════════

    // run_with_custom_config().await;
}

/// 方案 A：完全使用 SDK
///
/// 優點：
/// - 程式碼最簡潔（~10 行）
/// - SDK 自動處理所有基礎設施
/// - 符合 Spring Boot 風格
///
/// 缺點：
/// - 需要調整 config 檔案格式（從 otel → telemetry）
async fn run_with_sdk() {
    println!("🎯 使用 SDK 完全自動化方案\n");

    // 1️⃣ 載入原本的 config（保留你的業務邏輯需要）
    let settings = config::Settings::load()
        .expect("Failed to load configuration");

    // 2️⃣ 初始化 app state（保留你的業務邏輯）
    let state = app_state::init(&settings).await;

    // 3️⃣ 定義路由
    let router = Router::new()
        .nest("/employee", employee::router(state.clone()));

    // 4️⃣ 註冊自訂 middleware
    let router = MiddlewareBuilder::new()
        .with_logger(middleware::HelloRustWebLogger::new())
        .with_auth(middleware::HelloRustWebAuth::new())
        .enable_tracing(true)
        .enable_panic_handler(true)
        .apply(router);

    // 5️⃣ 啟動！
    // SDK 自動處理：config 載入、telemetry 初始化、graceful shutdown
    println!("🚀 Starting with SDK!");
    println!("📍 Listening on {}:{}\n", settings.server.host, settings.server.port);

    RustWebApplication::run(router)
        .await
        .expect("Failed to run server");
}

/// 方案 B：保留原有的 config 和初始化流程
///
/// 優點：
/// - 完全相容現有架構
/// - 不需要修改 config 檔案
/// - 漸進式遷移
///
/// 缺點：
/// - 需要手動初始化 telemetry 和 database
#[allow(dead_code)]
async fn run_with_custom_config() {
    println!("🎯 使用自訂 config + SDK middleware 方案\n");

    // 1️⃣ 使用原本的 config
    let settings = config::Settings::load()
        .expect("Failed to load configuration");

    // 2️⃣ 手動初始化 telemetry (保留原本的邏輯)
    core::error::init_panic_handling();
    let _tracer_provider = core::otel::init(&settings.otel);

    // 3️⃣ 初始化 app state
    let state = app_state::init(&settings).await;

    // 4️⃣ 定義路由
    let router = Router::new()
        .nest("/employee", employee::router(state.clone()));

    // 5️⃣ 只使用 SDK 的 middleware builder
    let router = MiddlewareBuilder::new()
        .with_logger(middleware::HelloRustWebLogger::new())
        .with_auth(middleware::HelloRustWebAuth::new())
        .enable_tracing(true)
        .enable_panic_handler(false)  // 我們已經有自己的了
        .apply(router);

    // 6️⃣ 手動啟動 server
    let bind_addr = format!("{}:{}", settings.server.host, settings.server.port);
    let listener = tokio::net::TcpListener::bind(&bind_addr)
        .await
        .expect("Failed to bind");

    println!("🚀 Starting with custom config!");
    println!("📍 Listening on {}\n", bind_addr);

    axum::serve(listener, router)
        .await
        .expect("Server error");

    // 7️⃣ Shutdown
    println!("Shutting down telemetry...");
    _tracer_provider
        .shutdown()
        .expect("Failed to shutdown tracer");
}
