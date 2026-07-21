use axum::http::HeaderName;

pub const SERVER_SHUTDOWN_TIMEOUT: u64 = 300; // 5min

pub const NAMESPACE_HEADER: HeaderName = HeaderName::from_static("tenant-id");
