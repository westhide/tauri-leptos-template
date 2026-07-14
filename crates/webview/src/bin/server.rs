use leptos::config::get_config_from_env;
use service::{axum::serve, server::router::router, shared::logger::init_logger};
use tokio::net::TcpListener;
use tracing::info;
use webview::{
    shared::{NULL, Null, error::Error},
    shell,
};

#[tokio::main]
async fn main() -> Result<Null, Error> {
    init_logger("debug")?;

    info!("Server start");
    let options = get_config_from_env()?.leptos_options;
    info!("SSR options: {options:?}");

    let listener = TcpListener::bind(options.site_addr).await?;

    let router = router(shell, options).await?;

    info!("Server listen on http://{}", listener.local_addr()?);
    serve(listener, router).await?;

    info!("Server shutdown");
    Ok(NULL)
}
