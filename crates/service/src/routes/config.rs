use leptos::{prelude::*, server_fn::codec::Json};

use crate::shared::{
    error::ServerFnError,
    logger::{Level, instrument},
};

#[instrument(level = Level::DEBUG, skip_all, ret)]
#[server(input= Json)]
pub async fn log_level() -> Result<String, ServerFnError> {
    // TODO
    Ok("debug".into())
}
