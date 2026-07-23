use leptos::{prelude::*, server_fn::codec::Json};

use crate::{
    models::{database::user::User, namespace::register::RegisterParams},
    shared::{
        error::{Result, ServerFnError},
        logger::{Level, instrument},
    },
    traits::from_ctx::FromCtx,
};

#[instrument(level = Level::DEBUG, skip_all, ret, err)]
#[server(input= Json)]
pub async fn register(params: RegisterParams) -> Result<User, ServerFnError> {
    use crate::server::extensions::{database::Client, platform::SaasPlatform};

    let platform = SaasPlatform::from_ctx();
    let data = platform.register(&params).await?;

    // save to users table
    let db = Client::from_ctx();
    let user = User::from((params, data));
    db.insert_user(user.clone()).await?;

    Ok(user)
}
