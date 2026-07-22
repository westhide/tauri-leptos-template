use std::ops::Deref;

use surrealdb::{
    Error, Surreal,
    engine::any::{Any, connect},
};

use crate::{config::server::Database as DBConfig, shared::logger::debug};

const INIT_SQL: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/sql/init.surql"));

#[derive(Debug, Clone)]
pub struct Database {
    client: Surreal<Any>,
}

impl Deref for Database {
    type Target = Surreal<Any>;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

impl Database {
    pub async fn new(config: &DBConfig) -> Result<Self, Error> {
        let DBConfig { url, namespace } = config;

        let client = connect(url).await?;
        client.health().await?;
        debug!("SurrealDB connected: {url}");

        client.use_ns(namespace).await?;
        client.query(INIT_SQL).await?;
        debug!("SurrealDB initialized");

        Ok(Self { client })
    }
}
