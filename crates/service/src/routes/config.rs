use leptos::{prelude::*, server_fn::codec::Json};

use crate::{
    config::Config,
    shared::{
        error::ServerFnError,
        logger::{Level, instrument},
    },
    traits::from_ctx::FromCtx,
};

#[instrument(level = Level::DEBUG, skip_all, ret)]
#[server(input= Json)]
pub async fn get_config() -> Result<Config, ServerFnError> {
    Ok(Config::from_ctx())
}
