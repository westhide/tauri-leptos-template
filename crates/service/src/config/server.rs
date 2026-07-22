use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

use crate::shared::consts::{PKG_NAME, PKG_VERSION, SERVER_SHUTDOWN_TIMEOUT};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct SaasPlatform {
    pub base_url: String,
    pub namespace: String,
    pub captcha: Option<String>,
}

impl Default for SaasPlatform {
    fn default() -> Self {
        Self {
            base_url: "http://127.0.0.1:8102/".into(),
            namespace: Default::default(),
            captcha: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Database {
    pub url: String,
    pub namespace: String,
}

impl Default for Database {
    fn default() -> Self {
        Self {
            url: format!("rocksdb:///tmp/{PKG_NAME}-{PKG_VERSION}-db"),
            namespace: "default".into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Server {
    pub host_url: SocketAddr,
    pub grpc_url: SocketAddr,
    pub start_grpc: bool,
    pub database: Database,
    pub saas_platform: SaasPlatform,
    pub shutdown_timeout: u64,
}

impl Default for Server {
    fn default() -> Self {
        Self {
            host_url: SocketAddr::from(([127, 0, 0, 1], 1420)),
            grpc_url: SocketAddr::from(([127, 0, 0, 1], 1520)),
            start_grpc: true,
            database: Default::default(),
            saas_platform: Default::default(),
            shutdown_timeout: SERVER_SHUTDOWN_TIMEOUT,
        }
    }
}
