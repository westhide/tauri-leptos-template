pub mod logger;
pub mod server;

use config::{ConfigBuilder, File, builder::DefaultState};
use serde::{Deserialize, Serialize};

use crate::{
    config::{logger::Logger, server::Server},
    shared::error::Result,
};

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
