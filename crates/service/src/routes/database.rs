use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
use surrealdb::types::SurrealValue;

#[cfg(feature = "server")]
use crate::server::extension::database::Database;
use crate::shared::{
    error::Result,
    logger::{Level, instrument},
};

#[derive(Debug, Serialize, Deserialize, SurrealValue)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseInfo {
    pub id: u32,
    pub name: String,
    pub comment: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, SurrealValue)]
#[serde(rename_all = "camelCase")]
pub struct NamespaceInfo {
    // pub users: todo!(),
    // pub accesses: todo!(),
    pub databases: Vec<DatabaseInfo>,
}

// #[derive(Debug, Serialize, Deserialize, SurrealValue)]
// #[serde(rename_all = "camelCase")]
// pub struct TableInfo {
//     pub events: BTreeMap<String, String>,
//     pub fields: BTreeMap<String, String>,
//     pub indexes: BTreeMap<String, String>,
//     pub lives: BTreeMap<String, String>,
//     pub tables: BTreeMap<String, String>,
// }

#[cfg(feature = "server")]
#[instrument(level = Level::DEBUG, skip_all, ret, err)]
pub async fn schemas(db: Extension<Database>) -> Result<Json<Option<NamespaceInfo>>> {
    let data: Option<NamespaceInfo> = db.query("INFO FOR NAMESPACE STRUCTURE").await?.take(0)?;

    Ok(data.into())
}
