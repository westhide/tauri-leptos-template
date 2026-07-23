use leptos::{prelude::*, server_fn::codec::Json};

use crate::{
    models::{database::user::User, namespace::login::LoginParams},
    shared::{
        error::{Result, ServerFnError, err},
        logger::{Level, debug, instrument},
    },
    traits::from_ctx::FromCtx,
};

#[instrument(level = Level::DEBUG, skip_all, ret, err)]
#[server(input= Json)]
pub async fn login(params: LoginParams) -> Result<User, ServerFnError> {
    use crate::server::extensions::{database::Client, platform::SaasPlatform};

    // check local user password
    let db = Client::from_ctx();
    if let Some(user) = db.select_user(&params.username).await? {
        if params.password != user.password {
            return err!("密码不正确")?
        }
        // TODO: expires_time
        debug!(user.username, "login with local user password");
        return Ok(user)
    }

    // login with SaaS platform
    let platform = SaasPlatform::from_ctx();
    let data = platform.login(&params).await?;

    // save to users table
    let user = User::from((params, data));
    db.insert_user(user.clone()).await?;

    Ok(user)
}
