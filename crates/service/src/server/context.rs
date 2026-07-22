use std::{ops::Deref, time::Duration};

use axum::extract::FromRef;
use leptos::config::LeptosOptions;
use tokio::time::timeout;
use tokio_util::{sync::CancellationToken, task::TaskTracker};

use crate::{
    server::shutdown::ShutdownSignal,
    shared::{
        Null,
        error::Result,
        logger::{error, info},
    },
};

#[derive(Debug, Clone)]
pub struct Context<S> {
    pub state: S,
    pub task_tracker: TaskTracker,
    pub cancellation: CancellationToken,
}

impl<S> Deref for Context<S> {
    type Target = S;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

pub trait ShutdownTimeout {
    fn shutdown_timeout(&self) -> u64;
}

impl<S> Context<S> {
    pub fn new(state: S) -> Self {
        Self { state, task_tracker: TaskTracker::new(), cancellation: CancellationToken::new() }
    }

    pub fn graceful_shutdown_signal(&self) -> Result<impl Future<Output = Null> + use<S>>
    where
        S: Clone + ShutdownTimeout,
    {
        let Self { state, task_tracker, cancellation } = self.clone();

        let signal = ShutdownSignal::new()?;

        Ok(async move {
            info!("Register graceful shutdown signal");
            signal.wait_with_cancel(cancellation.clone()).await;

            info!("Shuting down...");

            // TODO: stop server
            task_tracker.close();
            cancellation.cancel();

            info!("Shutdown tasks...");
            let duration = Duration::from_secs(state.shutdown_timeout());
            if let Err(err) = timeout(duration, task_tracker.wait()).await {
                error!("Shutdown timeout: {err}, forcing shutdown");
            }

            info!("Shutdown complete");
        })
    }
}

impl<S> FromRef<Context<S>> for LeptosOptions
where
    LeptosOptions: FromRef<S>,
{
    fn from_ref(ctx: &Context<S>) -> Self {
        Self::from_ref(&ctx.state)
    }
}
