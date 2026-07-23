use std::ops::Deref;

use surrealdb::{
    Error, Surreal,
    engine::any::{Any, connect},
};

use crate::{config::server::Database as DBConfig, shared::logger::debug};

const INIT_SQL: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/sql/init.surql"));

#[derive(Debug, Clone)]
pub struct DbClient {
    inner: Surreal<Any>,
}

impl Deref for DbClient {
    type Target = Surreal<Any>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DbClient {
    pub async fn new(config: &DBConfig) -> Result<Self, Error> {
        let DBConfig { url, namespace } = config;

        let client = connect(url).await?;
        client.health().await?;
        debug!("SurrealDB connected: {url}");

        client.use_ns(namespace).await?;
        client.query(INIT_SQL).await?.check()?;
        debug!("SurrealDB initialized");

        Ok(Self { inner: client })
    }
}
