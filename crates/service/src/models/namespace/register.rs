use chrono::{DateTime, Utc, serde::ts_milliseconds};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterParams {
    pub username: String,
    pub nickname: String,
    pub password: String,
    pub captcha_verification: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterData {
    pub user_id: i64,
    pub access_token: String,
    pub refresh_token: String,
    #[serde(with = "ts_milliseconds")]
    pub expires_time: DateTime<Utc>,
    pub password_expired: Option<bool>,
    pub must_change_password: Option<bool>,
}
