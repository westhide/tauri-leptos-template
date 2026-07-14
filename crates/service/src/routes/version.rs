use leptos::{prelude::*, server_fn::codec::Json};

use crate::shared::{
    error::ServerFnError,
    logger::{Level, instrument},
};

// TODO: route #[get("/version")]
// TODO: Fix macro order
#[instrument(level = Level::DEBUG, skip_all, ret)]
#[server(input= Json)]
pub async fn version() -> Result<String, ServerFnError> {
    Ok(env!("CARGO_PKG_VERSION").to_owned())
}
