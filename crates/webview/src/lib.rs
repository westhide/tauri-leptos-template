pub mod routes;
#[cfg(feature = "ssr")]
pub mod server;
pub mod shared;
pub mod views;

use leptos::{prelude::*, wasm_bindgen, wasm_bindgen::prelude::*};
use leptos_meta::{MetaTags, Stylesheet};

use crate::{
    shared::logger::{info, init_console_log},
    views::Main,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    let title = options.output_name.clone();
    let style = options.css_path();

    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
                <title>{title}</title>
                <Stylesheet href=style />
            </head>
            <body>
                <Main />
            </body>
        </html>
    }
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen]
pub fn hydrate() {
    init_console_log();

    info!("Client hydration");

    hydrate_body(Main);
}
