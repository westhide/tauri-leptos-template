use leptos::config::get_config_from_env;
use service::{
    axum::serve,
    cli::Cli,
    server::{context::Context, router::router, startup::startup_grpc},
    shared::logger::init_logger,
};
use tokio::{net::TcpListener, spawn};
use tracing::{debug, info};
use webview::{
    bootstrap::shell::shell,
    shared::{NULL, Null, error::Error},
    state::State,
};

#[tokio::main]
async fn main() -> Result<Null, Error> {
    let config = Cli::load_config()?;

    init_logger(config.logger.level)?;

    if config.server.start_grpc {
        spawn(startup_grpc(config.clone()));
    }

    info!("Ssr server start");
    debug!(?config);

    let options = get_config_from_env()?.leptos_options;
    info!("Ssr options: {options:?}");

    let listener = TcpListener::bind(options.site_addr).await?;

    let state = State::new(config, options);
    let context = Context::new(state);
    let shutdown_signal = context.graceful_shutdown_signal()?;

    let router = router(context, shell).await?;

    info!("Ssr server listen on http://{}", listener.local_addr()?);
    serve(listener, router).with_graceful_shutdown(shutdown_signal).await?;

    info!("Ssr server shutdown");
    Ok(NULL)
}
