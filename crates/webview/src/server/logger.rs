use tracing_subscriber::{EnvFilter, fmt};

use crate::{
    server::error::Result,
    shared::{NULL, Null},
};

pub fn init_logger(level: &str) -> Result<Null> {
    let directive = level.parse()?;
    let builder = EnvFilter::builder().with_default_directive(directive);
    let filter = builder.from_env()?;
    fmt().with_env_filter(filter).init();
    Ok(NULL)
}
