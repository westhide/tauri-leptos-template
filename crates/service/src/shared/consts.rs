use axum::http::HeaderName;

pub const PKG_NAME: &str = env!("CARGO_PKG_NAME");

pub const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const SERVER_SHUTDOWN_TIMEOUT: u64 = 300; // 5min

pub const NAMESPACE_HEADER: HeaderName = HeaderName::from_static("tenant-id");
