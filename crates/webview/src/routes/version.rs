use crate::shared::{
    Null,
    error::Result,
    invoke::invoke,
    logger::{Level, instrument},
};

// TODO: gen #[handler]
#[instrument(level = Level::DEBUG, skip_all, ret, err)]
pub async fn version() -> Result<String> {
    Ok(invoke!("version")?)
}
