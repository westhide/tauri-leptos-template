use std::io::Error as StdIoError;

use leptos::{
    config::errors::LeptosConfigError, serde_json::Error as JsonError, wasm_bindgen::JsValue,
};
use libgrpc::tonic::Status as GrpcStatus;
use service::{
    axum::http::uri::InvalidUri as HttpInvalidUri,
    shared::error::{Error as ServiceError, ServerFnError},
};

use crate::shared::logger::log::ParseLevelError as LogParseLevelError;

/// Error
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    StdIoError(#[from] StdIoError),

    #[error(transparent)]
    GrpcStatus(#[from] GrpcStatus),

    #[error(transparent)]
    HttpInvalidUri(#[from] HttpInvalidUri),

    #[error("{0}")]
    JsError(String),

    #[error(transparent)]
    JsonError(#[from] JsonError),

    #[error(transparent)]
    LeptosConfigError(#[from] LeptosConfigError),

    #[error(transparent)]
    LogParseLevelError(#[from] LogParseLevelError),

    #[error("{0}")]
    ServerFnError(String),

    #[error(transparent)]
    ServiceError(#[from] ServiceError),

    #[error("{0}")]
    Error(String),
}

impl From<JsValue> for Error {
    fn from(err: JsValue) -> Self {
        Self::JsError(format!("{err:?}"))
    }
}

impl From<Error> for JsValue {
    fn from(err: Error) -> Self {
        JsValue::from_str(&err.to_string())
    }
}

impl From<ServerFnError> for Error {
    fn from(err: ServerFnError) -> Self {
        Self::ServerFnError(err.to_string())
    }
}

macro_rules! err {
    ($($arg:tt)*) => {
        Err($crate::shared::error::Error::Error(format!($($arg)*)))
    }
}

pub(crate) use err;

pub(crate) type Result<T, E = Error> = std::result::Result<T, E>;
