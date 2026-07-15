pub mod routes;
pub mod shared;
pub mod state;
pub mod views;

use leptos::{
    __reexports::wasm_bindgen_futures, prelude::*, wasm_bindgen, wasm_bindgen::prelude::*,
};
use leptos_meta::{MetaTags, Stylesheet};
use service::routes::config::log_level;

use crate::{
    shared::{
        NULL, Null,
        error::Result,
        logger::{info, init_console_log},
    },
    views::Main,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    // TODO: Fix <!>
    // let title = options.output_name.clone();
    let style = options.css_path();

    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <title>"Leptos"</title>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
                <Stylesheet href=style />
            </head>
            <body>
                <Main />
            </body>
        </html>
    }
}

#[wasm_bindgen]
pub async fn hydrate() -> Result<Null> {
    let level = log_level().await?;

    init_console_log(&level)?;

    info!("Client hydration");

    #[cfg(feature = "hydrate")]
    hydrate_body(Main);
    Ok(NULL)
}
