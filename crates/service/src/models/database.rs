pub mod user;

use serde::{Deserialize, Serialize};
use surrealdb::types::SurrealValue;

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
