use std::net::SocketAddr;

use config::{ConfigBuilder, File, builder::DefaultState};
use serde::{Deserialize, Serialize};

use crate::shared::{consts::SERVER_SHUTDOWN_TIMEOUT, error::Result};

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Logger {
    pub level: String,
}

impl Default for Logger {
    fn default() -> Self {
        Self { level: "info".into() }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Server {
    pub host_url: SocketAddr,
    pub grpc_url: SocketAddr,
    pub shutdown_timeout: u64,
}

impl Default for Server {
    fn default() -> Self {
        Self {
            host_url: SocketAddr::from(([127, 0, 0, 1], 3000)),
            grpc_url: SocketAddr::from(([127, 0, 0, 1], 3001)),
            shutdown_timeout: SERVER_SHUTDOWN_TIMEOUT,
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub logger: Logger,
    pub server: Server,
}

impl Config {
    pub fn try_from_file(path: &str) -> Result<Self> {
        let source = File::with_name(path).required(true);
        let config = ConfigBuilder::<DefaultState>::default().add_source(source).build()?;
        Ok(config.try_deserialize()?)
    }
}
