use std::time::Duration;

use axum::extract::FromRef;
use leptos::config::LeptosOptions;
use tokio::time::timeout;
use tokio_util::{sync::CancellationToken, task::TaskTracker};

use crate::{
    config::Config,
    server::shutdown::ShutdownSignal,
    shared::{
        Null,
        error::Result,
        logger::{error, info},
    },
};

#[derive(Debug, Clone)]
pub struct Context {
    pub config: Config,
    pub leptos: LeptosOptions,
    pub task_tracker: TaskTracker,
    pub cancellation: CancellationToken,
}

impl Context {
    pub fn new(config: Config, leptos: LeptosOptions) -> Self {
        Self {
            config,
            leptos,
            task_tracker: TaskTracker::new(),
            cancellation: CancellationToken::new(),
        }
    }

    pub fn graceful_shutdown_signal(&self) -> Result<impl Future<Output = Null> + use<>> {
        let Self { config, task_tracker, cancellation, .. } = self.clone();

        let signal = ShutdownSignal::new()?;
        let shutdown_timeout = config.server.shutdown_timeout;

        Ok(async move {
            info!("Register graceful shutdown signal");
            signal.wait_with_cancel(cancellation.clone()).await;

            info!("Shuting down...");

            // TODO: stop server
            task_tracker.close();
            cancellation.cancel();

            info!("Shutdown tasks...");
            let duration = Duration::from_secs(shutdown_timeout);
            if let Err(err) = timeout(duration, task_tracker.wait()).await {
                error!("Shutdown timeout: {err}, forcing shutdown");
            }

            info!("Shutdown complete");
        })
    }
}

impl FromRef<Context> for Config {
    fn from_ref(ctx: &Context) -> Self {
        ctx.config.clone()
    }
}

impl FromRef<Context> for LeptosOptions {
    fn from_ref(ctx: &Context) -> Self {
        ctx.leptos.clone()
    }
}
