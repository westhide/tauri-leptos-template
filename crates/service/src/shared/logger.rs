use tracing_subscriber::{EnvFilter, fmt};

use crate::shared::{NULL, Null, error::Result};

pub fn init_logger<L: Into<Level>>(level: L) -> Result<Null> {
    let level = level.into();
    let builder = EnvFilter::builder().with_default_directive(level.into());
    let filter = builder.from_env()?;
    fmt().with_env_filter(filter).init();
    Ok(NULL)
}

pub(crate) use tracing::*;
