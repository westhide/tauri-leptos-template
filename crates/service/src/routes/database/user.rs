use crate::{
    models::database::user::{USER_DB, USER_TABLE, User},
    server::extension::database::DbClient,
    shared::{NULL, Null, error::Result},
};

impl DbClient {
    pub async fn insert_user(&self, user: User) -> Result<Null> {
        self.use_db(USER_DB).await?;
        self.insert::<Option<User>>((USER_TABLE, user.user_id)).content(user).await?;
        Ok(NULL)
    }
}
