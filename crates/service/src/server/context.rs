use std::{ops::Deref, sync::Arc, time::Duration};

use tokio::time::timeout;
use tokio_util::{sync::CancellationToken, task::TaskTracker};

use crate::{
    server::shutdown::ShutdownSignal,
    shared::{
        Null,
        config::Config,
        error::Result,
        logger::{error, info},
    },
};

#[derive(Debug)]
pub struct ContextInner {
    pub config: Config,
    pub task_tracker: TaskTracker,
    pub cancellation: CancellationToken,
}

impl ContextInner {
    pub fn new(config: Config) -> Self {
        Self { config, task_tracker: TaskTracker::new(), cancellation: CancellationToken::new() }
    }
}

#[derive(Debug, Clone)]
pub struct Context {
    inner: Arc<ContextInner>,
}

impl Context {
    pub fn new(config: Config) -> Self {
        let inner = ContextInner::new(config);
        Self { inner: Arc::new(inner) }
    }

    pub fn graceful_shutdown_signal(&self) -> Result<impl Future<Output = Null> + use<>> {
        let signal = ShutdownSignal::try_new(self.cancellation.clone())?;
        let ctx = self.inner.clone();

        Ok(async move {
            info!("Register graceful shutdown signal");
            signal.wait().await;

            info!("Shuting down...");

            // TODO: stop server
            ctx.task_tracker.close();
            ctx.cancellation.cancel();

            info!("Shutdown tasks...");
            let duration = Duration::from_secs(ctx.config.server.shutdown_timeout);
            if let Err(err) = timeout(duration, ctx.task_tracker.wait()).await {
                error!("Shutdown timeout: {err}, forcing shutdown");
            }

            info!("Shutdown complete");
        })
    }
}

impl Deref for Context {
    type Target = ContextInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
