use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
use surrealdb::types::SurrealValue;

#[cfg(feature = "server")]
use crate::server::extension::database::DbClient;
use crate::shared::{
    error::{Result, err},
    logger::{Level, instrument},
};

#[derive(Debug, Serialize, Deserialize, SurrealValue)]
#[serde(rename_all = "camelCase")]
pub struct NamespaceDatabase {
    pub id: u32,
    pub name: String,
    pub comment: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, SurrealValue)]
#[serde(rename_all = "camelCase")]
pub struct Namespace {
    // pub users: todo!(),
    // pub accesses: todo!(),
    pub databases: Vec<NamespaceDatabase>,
}

#[derive(Debug, Serialize, Deserialize, SurrealValue)]
#[serde(rename_all = "camelCase")]
pub struct Database {
    // pub ...: todo!(),
    pub tables: Vec<DatabaseTable>,
}

#[derive(Debug, Serialize, Deserialize, SurrealValue)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseTable {
    pub id: u32,
    pub name: String,
    pub drop: bool,
    pub schemafull: bool,
    pub comment: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schemas {
    pub namespace: Namespace,
    pub databases: Vec<Database>,
}

#[cfg(feature = "server")]
#[instrument(level = Level::DEBUG, skip_all, ret, err)]
pub async fn schemas(db: Extension<DbClient>) -> Result<Json<Schemas>> {
    async fn get_namespace_info(db: &DbClient) -> Result<Namespace> {
        const SQL: &str = "INFO FOR NAMESPACE STRUCTURE";
        let Some(namespace) = db.query(SQL).await?.check()?.take(0)? else {
            return err!("info for namespace return none");
        };
        Ok(namespace)
    }

    async fn get_database_info(db: &DbClient, name: &str) -> Result<Database> {
        const SQL: &str = "INFO FOR DATABASE STRUCTURE";
        db.use_db(name).await?;
        let Some(database) = db.query(SQL).await?.check()?.take(0)? else {
            return err!("info for database return none");
        };
        Ok(database)
    }

    let namespace = get_namespace_info(&db).await?;

    let mut databases = vec![];
    for NamespaceDatabase { name, .. } in &namespace.databases {
        let database = get_database_info(&db, name).await?;
        databases.push(database);
    }

    Ok(Schemas { namespace, databases }.into())
}
