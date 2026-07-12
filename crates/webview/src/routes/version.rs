use leptos::{prelude::*, server_fn::codec::Json};

use crate::shared::{
    Null,
    error::Result,
    invoke::invoke,
    logger::{Level, instrument},
};

// TODO: route #[get("/version")]
// TODO: Fix macro order
#[instrument(level = Level::DEBUG, skip_all, ret)]
#[server(input= Json)]
pub async fn version() -> Result<String> {
    Ok(env!("CARGO_PKG_VERSION").to_owned())
}

// TODO: gen #[handler]
#[instrument(level = Level::DEBUG, skip_all, ret, err)]
pub async fn desktop_version() -> Result<String> {
    invoke!("version")
}
