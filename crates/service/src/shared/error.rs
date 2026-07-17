use std::{
    env::VarError as StdEnvVarError, io::Error as StdIoError,
    net::AddrParseError as StdNetAddrParseError,
};

use config::ConfigError;
use leptos::server_fn::{
    codec::JsonEncoding,
    error::{FromServerFnError, ServerFnErrorErr},
};
use libgrpc::tonic::Status as GrpcStatus;
#[cfg(feature = "server")]
use libgrpc::tonic::transport::Error as GrpcTransportError;
use serde::{Deserialize, Serialize};
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
    GrpcStatus(#[from] GrpcStatus),

    #[cfg(feature = "server")]
    #[error(transparent)]
    GrpcTransportError(#[from] GrpcTransportError),

    #[error(transparent)]
    TracingFilterFromEnvError(#[from] TracingFilterFromEnvError),

    #[error(transparent)]
    TracingFilterParseError(#[from] TracingFilterParseError),

    #[error("{0}")]
    Error(String),
}

/// ServerFnError
#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
pub enum ServerFnError {
    #[error(transparent)]
    ServerFnError(#[from] ServerFnErrorErr),

    #[error("{0}")]
    Error(String),
}

impl From<Error> for ServerFnError {
    fn from(err: Error) -> Self {
        Self::Error(err.to_string())
    }
}

impl FromServerFnError for ServerFnError {
    type Encoder = JsonEncoding;

    fn from_server_fn_error(err: ServerFnErrorErr) -> Self {
        Self::ServerFnError(err)
    }
}

#[cfg(feature = "server")]
const _: () = {
    use axum::{
        http::StatusCode,
        response::{IntoResponse, Response},
    };

    impl IntoResponse for ServerFnError {
        fn into_response(self) -> Response {
            const CODE: StatusCode = StatusCode::INTERNAL_SERVER_ERROR;
            (CODE, self.to_string()).into_response()
        }
    }
};

macro_rules! err {
    ($($arg:tt)*) => {
        Err($crate::shared::error::Error::Error(format!($($arg)*)))
    }
}

pub(crate) use err;

pub(crate) type Result<T, E = Error> = std::result::Result<T, E>;
