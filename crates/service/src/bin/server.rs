use service::{
    cli::Cli,
    server::{context::Context, router::router},
    shared::{
        NULL, Null,
        error::Result,
        logger::{info, init_logger},
    },
};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<Null> {
    let config = Cli::load_config()?;

    init_logger(&config.logger.level)?;

    info!("Service startup");

    let listener = TcpListener::bind(config.server.host_url).await?;

    let ctx = Context::new(config);
    let shutdown_signal = ctx.graceful_shutdown_signal()?;

    let router = router(ctx).await?;

    info!("Server listen on http://{}", listener.local_addr()?);
    axum::serve(listener, router).with_graceful_shutdown(shutdown_signal).await?;

    info!("Service shutdown");
    Ok(NULL)
}
