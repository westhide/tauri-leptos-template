pub fn init_logger<L: Into<log::Level>>(level: L) {
    console_error_panic_hook::set_once();
    console_log::init_with_level(level.into()).ok();
}

pub(crate) use tracing::*;
