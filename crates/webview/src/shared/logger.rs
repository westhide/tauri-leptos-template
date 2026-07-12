use tracing::log::Level::Debug;

pub fn init_console_log() {
    console_error_panic_hook::set_once();
    // TODO: log level
    console_log::init_with_level(Debug).ok();
}

pub use tracing::*;
