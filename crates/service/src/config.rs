pub mod client;
pub mod logger;
pub mod server;

use std::{ops::Deref, sync::Arc};

use config::{ConfigBuilder, File, builder::DefaultState};
use serde::{Deserialize, Serialize};

use crate::{
    config::{client::Client, logger::Logger, server::Server},
    impl_from_ctx,
    shared::error::Result,
};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ConfigInner {
    pub logger: Logger,
    pub client: Client,
    pub server: Server,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
#[serde(transparent)]
pub struct Config {
    inner: Arc<ConfigInner>,
}

impl Deref for Config {
    type Target = ConfigInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Config {
    pub fn try_from_file(path: &str) -> Result<Self> {
        let source = File::with_name(path).required(true);
        let config = ConfigBuilder::<DefaultState>::default().add_source(source).build()?;
        Ok(config.try_deserialize()?)
    }
}

// Unsafe: must call provide_context()
impl_from_ctx!(Config);
