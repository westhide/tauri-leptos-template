use leptos::{prelude::*, server_fn::codec::Json};

use crate::{
    config::Config,
    models::namespace::{GetNamespaceData, GetNamespaceParams},
    shared::{
        NULL,
        error::{Result, ServerFnError},
        logger::{Level, instrument},
    },
    traits::from_ctx::FromCtx,
};

#[instrument(level = Level::DEBUG, skip_all, ret, err)]
#[server(input= Json)]
pub async fn get(domain: String) -> Result<GetNamespaceData, ServerFnError> {
    use crate::server::extension::client::HttpClient;

    let config = Config::from_ctx();
    let client = HttpClient::from_ctx();

    let base_url = &config.server.saas_platform.base_url;
    let url = format!("{base_url}/admin-api/system/tenant/get-by-website");

    let params = GetNamespaceParams { website: domain };
    let req = client.get(url).query(&params);

    Ok(client.fetch(req, &NULL).await?)
}
