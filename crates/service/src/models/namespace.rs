pub mod register;

// use chrono::{DateTime, Utc, serde::ts_milliseconds};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetNamespaceParams {
    pub website: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetNamespaceData {
    pub id: i64,
    pub name: String,
    pub icon: Option<String>,
    pub hospital_name: Option<String>,
    pub contact_name: Option<String>,
    pub contact_mobile: Option<String>,
    pub status: Option<i32>,
    pub website: Option<String>,
    pub package_id: Option<i64>,
    // #[serde(with = "ts_milliseconds")]
    // pub expire_time: DateTime<Utc>,
    pub account_count: Option<i32>,
    // #[serde(with = "ts_milliseconds")]
    // pub create_time: DateTime<Utc>,
}
