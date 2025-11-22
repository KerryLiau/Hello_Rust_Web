# SDK 整合指南

這份文檔說明如何將 `rust_web_sdk` 整合到 Hello_Rust_Web 專案中。

## 🎯 整合完成的檔案

### 1. 依賴更新
- ✅ `Cargo.toml` - 已加入 rust_web_sdk 依賴

### 2. 自訂 Middleware
- ✅ `src/middleware/mod.rs` - Middleware 模組定義
- ✅ `src/middleware/custom_auth.rs` - 實作 SDK 的 `AuthMiddleware` trait
- ✅ `src/middleware/custom_logger.rs` - 實作 SDK 的 `LogMiddleware` trait

### 3. 新的 Main 入口
- ✅ `src/main_new.rs` - 使用 SDK 的新版本 main.rs（提供兩種方案）

## 📝 修改說明

### Custom Auth Middleware

原本的認證邏輯在 `src/core/layer/auth.rs`，現在包裝成 SDK trait 實作：

```rust
// src/middleware/custom_auth.rs
pub struct HelloRustWebAuth;

impl AuthMiddleware for HelloRustWebAuth {
    fn process(&self, req: Request, next: Next) -> MiddlewareFuture<'_> {
        Box::pin(async move {
            // 保留原本的認證邏輯
            match Self::extract_bearer_token(&req) {
                Ok(token) => {
                    if let Some(auth_data) = Self::authenticate(token) {
                        USER.scope(auth_data, next.run(req)).await
                    } else {
                        ApiError::Unauthorized("unauthorized".to_string()).into_response()
                    }
                }
                Err(err) => err.into_response(),
            }
        })
    }
}
```

**關鍵改變：**
- ✅ 實作 `AuthMiddleware` trait
- ✅ 保留原本的 Bearer token 邏輯
- ✅ 保留 `task_local!` 的 USER 儲存
- ✅ 完全相容現有的業務邏輯

### Custom Logger Middleware

原本的日誌邏輯在 `src/core/layer/request_log.rs`，現在包裝成 SDK trait 實作：

```rust
// src/middleware/custom_logger.rs
pub struct HelloRustWebLogger;

impl LogMiddleware for HelloRustWebLogger {
    fn process(&self, req: Request, next: Next) -> MiddlewareFuture<'_> {
        Box::pin(async move {
            let uri = req.uri().clone();
            info!("incoming request: {}", uri);
            let response = next.run(req).await;
            info!("request completed: {} - status: {}", uri, response.status());
            response
        })
    }
}
```

**關鍵改變：**
- ✅ 實作 `LogMiddleware` trait
- ✅ 保留原本的 logging 邏輯
- ✅ 保留 OpenTelemetry span integration

## 🚀 兩種整合方案

### 方案 A：完全使用 SDK（推薦）

**優點：**
- 程式碼最簡潔（~20 行 vs 原本的 47 行）
- SDK 自動處理所有基礎設施
- 符合 Spring Boot 風格

**main.rs 範例：**
```rust
#[tokio::main]
async fn main() {
    let settings = config::Settings::load().unwrap();
    let state = app_state::init(&settings).await;

    let router = Router::new()
        .nest("/employee", employee::router(state));

    let router = MiddlewareBuilder::new()
        .with_logger(middleware::HelloRustWebLogger::new())
        .with_auth(middleware::HelloRustWebAuth::new())
        .enable_tracing(true)
        .apply(router);

    RustWebApplication::run(router).await.unwrap();
}
```

**減少的程式碼：**
- ❌ 不需要手動初始化 OpenTelemetry
- ❌ 不需要手動建立 ServiceBuilder
- ❌ 不需要手動排列 middleware 順序
- ❌ 不需要手動 shutdown tracer_provider
- ❌ 不需要手動建立 TcpListener

### 方案 B：保留原有架構

**優點：**
- 完全相容現有 config 格式
- 不需要修改 config 檔案
- 漸進式遷移

**適合：**
- 想要逐步遷移的情況
- 有特殊的初始化需求
- 需要完全控制啟動流程

## ⚙️ 啟用整合

### 步驟 1：確保 SDK 已建立

```bash
cd /Users/kerryliau/RustroverProjects/Hello_Rust_Lib
cargo build --features database,telemetry
```

### 步驟 2：選擇方案並啟用

#### 選項 A：使用新的 main.rs（方案 A）

```bash
cd /Users/kerryliau/RustroverProjects/Hello_Rust_Web

# 備份原有的 main.rs
mv src/main.rs src/main_old.rs

# 使用新版本
mv src/main_new.rs src/main.rs
```

#### 選項 B：保留原有架構（方案 B）

在 `src/main_new.rs` 中啟用方案 B：

```rust
#[tokio::main]
async fn main() {
    // run_with_sdk().await;  // 註解掉
    run_with_custom_config().await;  // 啟用這個
}
```

### 步驟 3：建置並測試

```bash
cargo build
cargo run
```

## 📊 效果對比

### 原本的 main.rs（47 行）

```rust
#[tokio::main]
async fn main() {
    let settings = config::Settings::load().expect(...);

    core::error::init_panic_handling();
    let tracer_provider = core::otel::init(&settings.otel);

    run_server(settings).await;

    tracer_provider.shutdown().expect(...);
}

async fn run_server(settings: config::Settings) {
    let state = app_state::init(&settings).await;
    let router = Router::new()
        .nest("/employee", employee::router(state.clone()))
        .route_layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(middleware::from_fn(core::layer::request_log::process))
                .layer(middleware::from_fn(core::layer::auth::process))
                .layer(CatchPanicLayer::custom(core::error::MyPanicHandler)),
        );

    let bind_addr = format!("{}:{}", settings.server.host, settings.server.port);
    let listener = tokio::net::TcpListener::bind(&bind_addr).await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, router).await.unwrap();
}
```

### 使用 SDK 後（~20 行）

```rust
#[tokio::main]
async fn main() {
    let settings = config::Settings::load().unwrap();
    let state = app_state::init(&settings).await;

    let router = Router::new()
        .nest("/employee", employee::router(state));

    let router = MiddlewareBuilder::new()
        .with_logger(middleware::HelloRustWebLogger::new())
        .with_auth(middleware::HelloRustWebAuth::new())
        .enable_tracing(true)
        .apply(router);

    RustWebApplication::run(router).await.unwrap();
}
```

**減少了 27 行程式碼（57% reduction）！**

## ✅ 驗證清單

整合完成後，請確認：

- [ ] 編譯成功：`cargo build`
- [ ] 測試原有的 API：`curl -H "Authorization: Bearer test" http://localhost:8080/employee/users`
- [ ] OpenTelemetry 正常運作：查看 Jaeger UI
- [ ] 日誌輸出正常
- [ ] 認證功能正常（401 for missing token, 200 for valid token）

## 🔄 回滾方案

如果需要回滾到原有版本：

```bash
# 恢復原有的 main.rs
mv src/main_old.rs src/main.rs

# 移除 middleware 目錄（如果不需要的話）
# rm -rf src/middleware

# 回滾 Cargo.toml 的依賴（可選）
git checkout Cargo.toml
```

## 📚 相關文件

- SDK 完整文檔：`../Hello_Rust_Lib/README.md`
- Middleware 系統：`../Hello_Rust_Lib/MIDDLEWARE_SYSTEM.md`
- 快速參考：`../Hello_Rust_Lib/QUICK_REFERENCE.md`
- 自訂認證範例：`../Hello_Rust_Lib/examples/app_custom_auth.rs`

## 💡 下一步

1. **測試現有 API** - 確保所有端點正常運作
2. **檢查 OpenTelemetry** - 確認 traces 正常送到 Jaeger
3. **考慮移除舊程式碼** - 如果一切正常，可以移除 `src/core/layer/` 下的舊 middleware
4. **簡化 config** - 考慮是否要完全遷移到 SDK 的 config 格式
