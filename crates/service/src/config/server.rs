use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

use crate::shared::consts::SERVER_SHUTDOWN_TIMEOUT;

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Server {
    pub host_url: SocketAddr,
    pub grpc_url: SocketAddr,
    pub start_grpc: bool,
    pub shutdown_timeout: u64,
}

impl Default for Server {
    fn default() -> Self {
        Self {
            host_url: SocketAddr::from(([127, 0, 0, 1], 1420)),
            grpc_url: SocketAddr::from(([127, 0, 0, 1], 1520)),
            start_grpc: true,
            shutdown_timeout: SERVER_SHUTDOWN_TIMEOUT,
        }
    }
}
