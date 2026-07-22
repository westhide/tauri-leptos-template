use leptos::{prelude::*, wasm_bindgen, wasm_bindgen::prelude::*};
use service::routes::config;

use crate::{
    shared::{
        NULL, Null,
        client::GrpcClient,
        error::Result,
        logger::{debug, info, init_logger},
    },
    views::Main,
};

// wasm-bindgen-futures runtime init during hydrate_body function,
// so call this hydrate hook in Main component
pub async fn hydrate_hook() -> Result<Null> {
    let config = config::get().await?;

    init_logger(config.logger.level);

    info!("Client hydration hook start");
    debug!(?config);

    let client = GrpcClient::new(config.server.grpc_url)?;

    provide_context(config);
    provide_context(client);

    info!("Client hydration hook done");
    Ok(NULL)
}

#[wasm_bindgen]
pub fn hydrate() {
    hydrate_body(Main)
}
