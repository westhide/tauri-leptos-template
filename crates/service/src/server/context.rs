use std::{ops::Deref, time::Duration};

use axum::extract::FromRef;
use leptos::{config::LeptosOptions, context::provide_context, nonce::Nonce};
use tokio::time::timeout;
use tokio_util::{sync::CancellationToken, task::TaskTracker};

use crate::{
    config::Config,
    impl_from_ctx,
    server::{extension::client::HttpClient, shutdown::ShutdownSignal},
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

    pub fn provide_context_hook(&self) -> impl Fn() + Clone + Send + 'static
    where
        S: Clone + Send + Sync + 'static,
        Config: FromRef<S>,
    {
        let Context { state, task_tracker, cancellation } = self.clone();
        let config = Config::from_ref(&state);
        let client = HttpClient::new();
        move || {
            provide_context(state.clone());
            provide_context(client.clone());
            provide_context(config.clone());
            provide_context(task_tracker.clone());
            provide_context(cancellation.clone());
        }
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

// Safety: Nonce provided
impl_from_ctx!(Nonce);

// Unsafe: must call provide_context() hook
impl_from_ctx!(HttpClient);
impl_from_ctx!(TaskTracker);
impl_from_ctx!(CancellationToken);
