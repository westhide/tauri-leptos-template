use std::sync::Arc;

use leptos::config::LeptosOptions;
use service::{axum::extract::FromRef, config::Config};

#[derive(Debug, Clone)]
pub struct State {
    pub config: Arc<Config>,
    pub options: LeptosOptions,
}

impl State {
    pub fn new(config: Config, options: LeptosOptions) -> Self {
        Self { config: Arc::new(config), options }
    }
}

impl FromRef<State> for LeptosOptions {
    fn from_ref(state: &State) -> Self {
        state.options.clone()
    }
}

#[cfg(feature = "ssr")]
const _: () = {
    use service::server::context::ShutdownTimeout;
    impl ShutdownTimeout for State {
        fn shutdown_timeout(&self) -> u64 {
            self.config.server.shutdown_timeout
        }
    }
};
