use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterParams {
    pub username: String,
    pub nickname: String,
    pub password: String,
    pub captcha_verification: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response<T = ()> {
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterData {
    user_id: i64,
    access_token: String,
    expires_time: String,
    refresh_token: String,
    password_expired: Option<bool>,
    must_change_password: Option<bool>,
}
