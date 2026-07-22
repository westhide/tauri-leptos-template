use chrono::{DateTime, Utc, serde::ts_milliseconds};
use serde::{Deserialize, Serialize};
use surrealdb::types::SurrealValue;

#[derive(Debug, Serialize, Deserialize, SurrealValue)]
#[serde(rename_all = "camelCase")]
pub struct UserTable {
    pub username: String,
    pub nickname: String,
    pub password: String,
    pub captcha_verification: Option<String>,

    pub user_id: i64,
    pub access_token: String,
    pub refresh_token: String,
    #[serde(with = "ts_milliseconds")]
    pub expires_time: DateTime<Utc>,
    pub password_expired: Option<bool>,
    pub must_change_password: Option<bool>,

    #[serde(with = "ts_milliseconds")]
    pub create_time: DateTime<Utc>,
    #[serde(with = "ts_milliseconds")]
    pub update_time: DateTime<Utc>,
}
