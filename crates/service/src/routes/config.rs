use std::sync::OnceLock;

use leptos::{prelude::*, server_fn::codec::Json};

use crate::{
    config::Config,
    shared::{
        error::ServerFnError,
        logger::{Level, instrument},
    },
    traits::from_ctx::FromCtx,
};

static CONFIG: OnceLock<Config> = OnceLock::new();

#[instrument(level = Level::DEBUG, skip_all, ret)]
#[server(input= Json)]
pub async fn get_config() -> Result<Config, ServerFnError> {
    Ok(CONFIG.get_or_init(Config::from_ctx).clone())
}
