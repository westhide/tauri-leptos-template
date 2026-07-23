use std::ops::Deref;

use surrealdb::{
    Error, Surreal,
    engine::any::{Any, connect},
};

use crate::{config::server::Database as Config, shared::logger::debug};

const INIT_SQL: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/sql/init.surql"));

#[derive(Debug, Clone)]
pub struct Client {
    inner: Surreal<Any>,
}

impl Deref for Client {
    type Target = Surreal<Any>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Client {
    pub async fn new(config: &Config) -> Result<Self, Error> {
        let Config { url, namespace } = config;

        let client = connect(url).await?;
        client.health().await?;
        debug!("SurrealDB connected: {url}");

        client.use_ns(namespace).await?;
        client.query(INIT_SQL).await?.check()?;
        debug!("SurrealDB initialized");

        Ok(Self { inner: client })
    }
}
