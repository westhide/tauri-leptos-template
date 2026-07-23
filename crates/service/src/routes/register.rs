use chrono::Utc;
use leptos::{prelude::*, server_fn::codec::Json};

use crate::{
    config::Config,
    models::{
        database::user::User,
        namespace::register::{RegisterData, RegisterParams},
    },
    shared::{
        consts::NAMESPACE_HEADER,
        error::{Result, ServerFnError},
        logger::{Level, instrument},
    },
    traits::from_ctx::FromCtx,
};

#[instrument(level = Level::DEBUG, skip_all, ret, err)]
#[server(input= Json)]
pub async fn register(id: String, params: RegisterParams) -> Result<RegisterData, ServerFnError> {
    use crate::server::extension::{database::DbClient, fetch::HttpClient};

    let config = Config::from_ctx();
    let client = HttpClient::from_ctx();

    let base_url = &config.server.saas_platform.base_url;
    let url = format!("{base_url}/admin-api/system/auth/register");

    let req = client.post(url).header(NAMESPACE_HEADER, id);
    let data: RegisterData = client.fetch(req, &params).await?;

    let db = DbClient::from_ctx();
    let now = Utc::now();
    let user = User {
        username: params.username,
        nickname: params.nickname,
        password: params.password,
        captcha_verification: params.captcha_verification,
        user_id: data.user_id,
        access_token: data.access_token.clone(),
        refresh_token: data.refresh_token.clone(),
        expires_time: data.expires_time,
        password_expired: data.password_expired,
        must_change_password: data.must_change_password,
        create_time: now,
        update_time: now,
    };
    db.insert_user(user).await?;

    Ok(data)
}
