use std::ops::Deref;

use reqwest::{Client, RequestBuilder, StatusCode};
use serde::{Deserialize, Serialize};

use crate::shared::error::{Result, err};

#[derive(Debug, Clone)]
pub struct HttpClient {
    inner: Client,
}

impl Deref for HttpClient {
    type Target = Client;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl HttpClient {
    pub fn new() -> Self {
        Self { inner: Client::new() }
    }

    pub async fn fetch<T, R>(&self, builder: RequestBuilder, params: &T) -> Result<R>
    where
        T: Serialize,
        R: for<'de> Deserialize<'de>,
    {
        let req = builder.json(params).build()?;

        let res = self.inner.execute(req).await?;
        if res.status() != StatusCode::OK {
            return err!("http client request failed, status={}", res.status())
        }

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct Response<T> {
            pub code: i32,
            pub data: Option<T>,
            pub msg: String,
        }

        let ret: Response<R> = res.json().await?;
        if ret.code != 0 {
            return err!("request fail: {}, code={}", ret.msg, ret.code);
        }

        match ret.data {
            Some(data) => Ok(data),
            None => err!("http client fetch return None"),
        }
    }
}
