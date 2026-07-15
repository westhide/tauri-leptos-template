use tracing_subscriber::{EnvFilter, fmt};

use crate::shared::{NULL, Null, error::Result};

pub fn init_logger<L>(level: L) -> Result<Null>
where
    Level: From<L>,
{
    let level = Level::from(level);
    let builder = EnvFilter::builder().with_default_directive(level.into());
    let filter = builder.from_env()?;
    fmt().with_env_filter(filter).init();
    Ok(NULL)
}

pub(crate) use tracing::*;
