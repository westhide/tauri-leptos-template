use std::{
    env::VarError as StdEnvVarError, io::Error as StdIoError,
    net::AddrParseError as StdNetAddrParseError,
};

use config::ConfigError;
use tracing_subscriber::filter::{
    FromEnvError as TracingFilterFromEnvError, ParseError as TracingFilterParseError,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    StdEnvVarError(#[from] StdEnvVarError),

    #[error(transparent)]
    StdIoError(#[from] StdIoError),

    #[error(transparent)]
    StdNetAddrParseError(#[from] StdNetAddrParseError),

    #[error(transparent)]
    ConfigError(#[from] ConfigError),

    #[error(transparent)]
    TracingFilterFromEnvError(#[from] TracingFilterFromEnvError),

    #[error(transparent)]
    TracingFilterParseError(#[from] TracingFilterParseError),

    #[error("{0}")]
    Error(String),
}

macro_rules! err {
    ($($arg:tt)*) => {
        Err($crate::shared::error::Error::Error(format!($($arg)*)))
    }
}

pub(crate) use err;

pub type Result<T, E = Error> = std::result::Result<T, E>;
