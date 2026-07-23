use chrono::{DateTime, Utc, serde::ts_milliseconds};
use serde::{Deserialize, Serialize};

use crate::models::database::user::User;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginParams {
    pub username: String,
    pub password: String,
    pub social_type: Option<i32>,
    pub social_code: Option<String>,
    pub social_state: Option<String>,
    pub social_code_valid: Option<bool>,
    pub captcha_verification: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginData {
    pub user_id: i64,
    pub access_token: String,
    pub refresh_token: String,
    #[serde(with = "ts_milliseconds")]
    pub expires_time: DateTime<Utc>,
    pub password_expired: Option<bool>,
    pub must_change_password: Option<bool>,
}

impl From<(LoginParams, LoginData)> for User {
    fn from((params, data): (LoginParams, LoginData)) -> Self {
        let now = Utc::now();
        Self {
            username: params.username,
            nickname: None,
            password: params.password,
            captcha_verification: params.captcha_verification,
            user_id: data.user_id,
            access_token: data.access_token,
            refresh_token: data.refresh_token,
            expires_time: data.expires_time,
            password_expired: data.password_expired,
            must_change_password: data.must_change_password,
            create_time: now,
            update_time: now,
        }
    }
}
