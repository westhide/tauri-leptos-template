use leptos::config::get_config_from_env;
use tokio::net::TcpListener;
use webview::{
    server::{error::Result, logger::init_logger, router::router},
    shared::{NULL, Null, logger::log::info},
};

#[tokio::main]
async fn main() -> Result<Null> {
    init_logger("debug")?;

    info!("Server start");
    let options = get_config_from_env()?.leptos_options;
    info!("SSR options: {options:?}");

    let listener = TcpListener::bind(options.site_addr).await?;

    let router = router(options).await?;

    info!("Server listen on http://{}", listener.local_addr()?);
    axum::serve(listener, router).await?;

    info!("Server shutdown");
    Ok(NULL)
}
