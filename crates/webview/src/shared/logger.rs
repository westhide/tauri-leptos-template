use crate::shared::{NULL, Null, error::Result};

pub fn init_console_log(level: &str) -> Result<Null> {
    console_error_panic_hook::set_once();
    console_log::init_with_level(level.parse()?).ok();
    Ok(NULL)
}

pub(crate) use tracing::*;
