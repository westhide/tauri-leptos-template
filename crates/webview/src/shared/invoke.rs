use leptos::{
    __reexports::wasm_bindgen_futures,
    serde_json::{from_str, to_string as into_str},
    wasm_bindgen,
    wasm_bindgen::prelude::*,
    web_sys::js_sys::JSON,
};
use serde::{Deserialize, Serialize};

use crate::shared::error::Error;

pub trait JsExt: Sized {
    fn from_wasm(value: JsValue) -> Result<Self, Error>;

    fn into_wasm(value: &Self) -> Result<JsValue, Error>;
}

impl<T> JsExt for T
where
    T: Serialize + for<'a> Deserialize<'a>,
{
    fn from_wasm(value: JsValue) -> Result<T, Error> {
        // JSON.stringify(undefined) === undefined, reinterpret 'undefined' as 'null'
        let text = if value.is_undefined() {
            String::from("null")
        } else {
            JSON::stringify(&value).map(String::from)?
        };
        Ok(from_str(&text)?)
    }

    fn into_wasm(value: &Self) -> Result<JsValue, Error> {
        let text = into_str(value)?;
        Ok(JSON::parse(&text)?)
    }
}

#[wasm_bindgen]
unsafe extern "C" {
    #[wasm_bindgen(catch ,js_namespace = ["window", "__TAURI_INTERNALS__"], js_name = invoke)]
    async fn call(
        cmd: &str,
        args: Option<JsValue>,
        opts: Option<JsValue>,
    ) -> Result<JsValue, JsValue>;
}

pub async fn invoke_impl<T, A, O>(cmd: &str, args: Option<&A>, opts: Option<&O>) -> Result<T, Error>
where
    A: JsExt,
    O: JsExt,
    T: JsExt,
{
    let args = args.map(JsExt::into_wasm).transpose()?;
    let opts = opts.map(JsExt::into_wasm).transpose()?;
    T::from_wasm(call(cmd, args, opts).await?)
}

macro_rules! invoke {
    ($cmd:expr) => {
        $crate::shared::invoke::invoke_impl::<_, Null, Null>($cmd, None, None).await
    };

    ($cmd:expr, $args:expr) => {
        $crate::shared::invoke::invoke_impl::<_, _, Null>($cmd, Some($args), None).await
    };

    ($cmd:expr, $args:expr, $opts:expr) => {
        $crate::shared::invoke::invoke_impl($cmd, Some($args), Some($opts)).await
    };
}

pub(crate) use invoke;
