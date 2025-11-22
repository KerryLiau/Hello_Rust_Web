//! 應用端的自訂 middleware
//!
//! 這裡實作 rust_web_sdk 的 middleware traits，
//! 把原本 core::layer 的邏輯包裝成 SDK 的 trait 實作

pub mod custom_auth;
pub mod custom_logger;

pub use custom_auth::HelloRustWebAuth;
pub use custom_logger::HelloRustWebLogger;
