use chrono::{DateTime, Utc};
use service::models::database::user::User;

#[derive(Debug)]
pub struct Credential {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub access_token: String,
    pub refresh_token: String,
    pub expires_time: DateTime<Utc>,
}

// TODO
const USER_EMAIL: &str = "medclaw@medclaw.com";

impl Default for Credential {
    fn default() -> Self {
        Self {
            id: 0,
            name: "medclaw".into(),
            email: USER_EMAIL.into(),
            access_token: Default::default(),
            refresh_token: Default::default(),
            expires_time: Default::default(),
        }
    }
}

impl From<User> for Credential {
    fn from(user: User) -> Self {
        Self {
            id: user.user_id,
            name: user.username,
            email: USER_EMAIL.into(),
            access_token: user.access_token,
            refresh_token: user.refresh_token,
            expires_time: user.expires_time,
        }
    }
}
