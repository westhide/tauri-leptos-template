use leptos::{prelude::*, server_fn::codec::Json};

use crate::{
    config::Config,
    models::namespace::register::{RegisterData, RegisterParams},
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
    use crate::server::extension::fetch::HttpClient;

    let config = Config::from_ctx();
    let client = HttpClient::from_ctx();

    let base_url = &config.server.saas_platform.base_url;
    let url = format!("{base_url}/admin-api/system/auth/register");

    let req = client.post(url).header(NAMESPACE_HEADER, id);

    Ok(client.fetch(req, &params).await?)
}
