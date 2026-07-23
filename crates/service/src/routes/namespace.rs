use leptos::{prelude::*, server_fn::codec::Json};

use crate::{
    models::namespace::{GetNamespaceData, GetNamespaceParams},
    shared::{
        error::{Result, ServerFnError},
        logger::{Level, instrument},
    },
    traits::from_ctx::FromCtx,
};

#[instrument(level = Level::DEBUG, skip_all, ret, err)]
#[server(input= Json)]
pub async fn get(domain: String) -> Result<GetNamespaceData, ServerFnError> {
    use crate::server::extensions::platform::SaasPlatform;

    let platform = SaasPlatform::from_ctx();
    let params = GetNamespaceParams { website: domain };
    Ok(platform.get_namespace(&params).await?)
}
