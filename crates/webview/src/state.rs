use leptos::config::LeptosOptions;
use service::{axum::extract::FromRef, config::Config};

#[derive(Debug, Clone)]
pub struct State {
    pub config: Config,
    pub options: LeptosOptions,
}

impl State {
    pub fn new(config: Config, options: LeptosOptions) -> Self {
        Self { config, options }
    }
}

impl FromRef<State> for Config {
    fn from_ref(state: &State) -> Self {
        state.config.clone()
    }
}

impl FromRef<State> for LeptosOptions {
    fn from_ref(state: &State) -> Self {
        state.options.clone()
    }
}

#[cfg(server)]
const _: () = {
    use service::{impl_from_ctx, server::context::ShutdownTimeout};

    impl ShutdownTimeout for State {
        fn shutdown_timeout(&self) -> u64 {
            self.config.server.shutdown_timeout
        }
    }

    // Unsafe: must call provide_context()
    impl_from_ctx!(State);
};
