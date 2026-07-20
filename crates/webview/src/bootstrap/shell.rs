use leptos::prelude::*;
use leptos_meta::{HashedStylesheet, Meta, MetaTags, Title, provide_meta_context};
use service::{config::Config, traits::from_ctx::FromCtx};

use crate::{
    shared::{error::Result, logger::debug},
    views::Main,
};

fn csp_policies(enable: bool) -> String {
    if !enable {
        debug!("csp disable");
        return String::new();
    }

    let nonce = Nonce::from_ctx();
    debug!(?nonce);

    let styles_rule = format!("'self' 'nonce-{nonce}'");
    let script_rule = format!("'strict-dynamic' 'nonce-{nonce}' 'wasm-unsafe-eval'");
    format!("default-src 'self'; style-src {styles_rule}; script-src {script_rule};")
}

pub fn shell(options: LeptosOptions) -> Result<impl IntoView> {
    provide_meta_context();

    let title = options.output_name.clone();

    let config = Config::from_ctx();
    let csp_policies = csp_policies(config.client.csp.enable);

    Ok(view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <Title text=title />
                <Meta http_equiv="Content-Security-Policy" content=csp_policies />
                <AutoReload options=options.clone() />
                <HydrationScripts options=options.clone() />
                <HashedStylesheet options />
                <MetaTags />
            </head>
            <body class="h-screen">
                <Main />
            </body>
        </html>
    })
}
