use leptos::{
    serde_json::Error as JsonError,
    server_fn::{
        codec::JsonEncoding,
        error::{FromServerFnError, ServerFnErrorErr},
    },
    wasm_bindgen::JsValue,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    ServerFnErrorErr(#[from] ServerFnErrorErr),

    #[error("{0}")]
    JsError(String),

    #[error("{0}")]
    JsonError(String),

    #[error("{0}")]
    Error(String),
}

impl From<JsonError> for Error {
    fn from(err: JsonError) -> Self {
        Self::JsonError(err.to_string())
    }
}

impl From<JsValue> for Error {
    fn from(err: JsValue) -> Self {
        Self::JsError(format!("{err:?}"))
    }
}

impl FromServerFnError for Error {
    type Encoder = JsonEncoding;

    fn from_server_fn_error(err: ServerFnErrorErr) -> Self {
        Self::ServerFnErrorErr(err)
    }
}

#[cfg(feature = "ssr")]
const _: () = {
    use axum::{
        http::StatusCode,
        response::{IntoResponse, Response},
    };

    impl IntoResponse for Error {
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

pub type Result<T, E = Error> = std::result::Result<T, E>;
