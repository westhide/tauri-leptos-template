use axum::{Extension, Json};
use surrealdb::types::RecordId;

use crate::{
    models::database::user::{USER_DB, USER_TABLE, User},
    server::extensions::database::Client,
    shared::{
        NULL, Null,
        error::Result,
        logger::{Level, instrument},
    },
};

impl Client {
    pub async fn select_user(&self, username: &str) -> Result<Option<User>> {
        self.use_db(USER_DB).await?;
        let sql = format!("SELECT * FROM {USER_TABLE} WHERE username = $username");
        let user = self.query(sql).bind(("username", username)).await?.check()?.take(0)?;
        Ok(user)
    }

    pub async fn select_users(&self) -> Result<Vec<User>> {
        self.use_db(USER_DB).await?;
        let users: Vec<User> = self.select(USER_TABLE).await?;
        Ok(users)
    }

    pub async fn insert_user(&self, user: User) -> Result<Null> {
        self.use_db(USER_DB).await?;
        let rid = RecordId::new(USER_TABLE, user.user_id);
        let sql = format!("CREATE $rid CONTENT $user");
        self.query(sql).bind(("rid", rid)).bind(("user", user)).await?.check()?;
        Ok(NULL)
    }
}

#[instrument(level = Level::DEBUG, skip_all, ret, err)]
pub async fn users(db: Extension<Client>) -> Result<Json<Vec<User>>> {
    Ok(Json(db.select_users().await?))
}
