use std::io::Error as StdIoError;

use leptos::config::errors::LeptosConfigError;
use tracing_subscriber::filter::{
    FromEnvError as TracingFilterFromEnvError, ParseError as TracingFilterParseError,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    StdIoError(#[from] StdIoError),

    #[error(transparent)]
    LeptosConfigError(#[from] LeptosConfigError),

    #[error(transparent)]
    TracingFilterFromEnvError(#[from] TracingFilterFromEnvError),

    #[error(transparent)]
    TracingFilterParseError(#[from] TracingFilterParseError),

    #[error("{0}")]
    Error(String),
}

macro_rules! err {
    ($($arg:tt)*) => {
        Err($crate::server::error::Error::Error(format!($($arg)*)))
    }
}

pub(crate) use err;

pub type Result<T, E = Error> = std::result::Result<T, E>;
