pub mod user;

use axum::{Extension, Json};
use serde::{Deserialize, Serialize};

use crate::{
    models::database::{Database, Namespace, NamespaceDatabase},
    server::extension::database::DbClient,
    shared::{
        error::{Result, err},
        logger::{Level, instrument},
    },
};

impl DbClient {
    async fn namespace_info(&self) -> Result<Namespace> {
        const SQL: &str = "INFO FOR NAMESPACE STRUCTURE";
        let Some(namespace) = self.query(SQL).await?.check()?.take(0)? else {
            return err!("info for namespace return none");
        };
        Ok(namespace)
    }

    async fn database_info(self: &DbClient, db: &str) -> Result<Database> {
        const SQL: &str = "INFO FOR DATABASE STRUCTURE";
        self.use_db(db).await?;
        let Some(database) = self.query(SQL).await?.check()?.take(0)? else {
            return err!("info for database return none");
        };
        Ok(database)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schemas {
    pub namespace: Namespace,
    pub databases: Vec<Database>,
}

#[instrument(level = Level::DEBUG, skip_all, ret, err)]
pub async fn schemas(db: Extension<DbClient>) -> Result<Json<Schemas>> {
    let namespace = db.namespace_info().await?;

    let mut databases = vec![];
    for NamespaceDatabase { name, .. } in &namespace.databases {
        let database = db.database_info(name).await?;
        databases.push(database);
    }

    Ok(Json(Schemas { namespace, databases }))
}
