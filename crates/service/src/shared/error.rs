use std::{
    env::VarError as StdEnvVarError, io::Error as StdIoError,
    net::AddrParseError as StdNetAddrParseError,
};

use axum::{
    Error as AxumError,
    http::{
        Error as HttpError, header::InvalidHeaderValue as HttpInvalidHeaderValue,
        uri::InvalidUri as HttpInvalidUri,
    },
};
use config::ConfigError;
use leptos::{
    serde_json::Error as JsonError,
    server_fn::{
        codec::JsonEncoding,
        error::{FromServerFnError, ServerFnErrorErr},
    },
};
use libgrpc::tonic::Status as GrpcStatus;
#[cfg(feature = "server")]
use libgrpc::tonic::transport::Error as GrpcTransportError;
use reqwest::Error as ReqwestError;
use serde::{Deserialize, Serialize};
use surrealdb::Error as SurrealdbError;
use tracing_subscriber::filter::{
    FromEnvError as TracingFilterFromEnvError, ParseError as TracingFilterParseError,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    StdEnvVar(#[from] StdEnvVarError),

    #[error(transparent)]
    StdIo(#[from] StdIoError),

    #[error(transparent)]
    StdNetAddrParse(#[from] StdNetAddrParseError),

    #[error(transparent)]
    Axum(#[from] AxumError),

    #[error(transparent)]
    Config(#[from] ConfigError),

    #[error(transparent)]
    GrpcStatus(#[from] GrpcStatus),

    #[cfg(feature = "server")]
    #[error(transparent)]
    GrpcTransport(#[from] GrpcTransportError),

    #[error(transparent)]
    Http(#[from] HttpError),

    #[error(transparent)]
    HttpInvalidHeaderValue(#[from] HttpInvalidHeaderValue),

    #[error(transparent)]
    HttpInvalidUri(#[from] HttpInvalidUri),

    #[error(transparent)]
    Json(#[from] JsonError),

    #[error(transparent)]
    Reqwest(#[from] ReqwestError),

    #[error(transparent)]
    Surrealdb(#[from] SurrealdbError),

    #[error(transparent)]
    TracingFilterFromEnv(#[from] TracingFilterFromEnvError),

    #[error(transparent)]
    TracingFilterParse(#[from] TracingFilterParseError),

    #[error("{0}")]
    Error(String),
}

/// ServerFnError
#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
pub enum ServerFnError {
    #[error(transparent)]
    ServerFn(#[from] ServerFnErrorErr),

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
        Self::ServerFn(err)
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
