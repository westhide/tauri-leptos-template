use std::{
    net::SocketAddr,
    ops::{Deref, DerefMut},
};

use gloo::net::http::{Method, RequestBuilder};
use libgrpc::{
    fetch::Fetch, protowire::ping_pong_service_client::PingPongServiceClient,
    tonic::codec::CompressionEncoding::Gzip, web::GrpcWebClientService,
};
use serde::{Deserialize, Serialize};
use service::impl_from_ctx;

use crate::shared::error::{Result, err};

type GrpcClientImpl = PingPongServiceClient<GrpcWebClientService<Fetch>>;

/// GrpcClient
#[derive(Debug, Clone)]
pub struct GrpcClient {
    inner: GrpcClientImpl,
}

impl Deref for GrpcClient {
    type Target = GrpcClientImpl;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for GrpcClient {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl GrpcClient {
    pub fn new(url: SocketAddr) -> Result<Self> {
        let uri = format!("http://{url}").parse()?;

        let fetch = GrpcWebClientService::new(Fetch {});
        let inner = PingPongServiceClient::with_origin(fetch, uri)
            .accept_compressed(Gzip)
            .send_compressed(Gzip);

        Ok(Self { inner })
    }
}

// Unsafe: must call provide_context()
#[cfg(client)]
impl_from_ctx!(GrpcClient);

/// HttpClient
#[derive(Debug)]
pub struct HttpClient {
    builder: RequestBuilder,
}

impl HttpClient {
    pub fn new(url: &str) -> Self {
        let builder = RequestBuilder::new(url);
        Self { builder }
    }

    pub fn method(self, method: Method) -> Self {
        let builder = self.builder.method(method);
        Self { builder }
    }

    pub fn header(self, key: &str, value: &str) -> Self {
        let builder = self.builder.header(key, value);
        Self { builder }
    }

    pub async fn fetch<T, R>(self, params: &T) -> Result<R>
    where
        T: Serialize,
        R: for<'de> Deserialize<'de>,
    {
        let Self { builder } = self;
        let req = builder.json(params)?;

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct Response<T> {
            pub code: i32,
            pub data: Option<T>,
            pub msg: String,
        }

        let ret: Response<R> = req.send().await?.json().await?;
        if ret.code != 0 {
            return err!("request fail: {}, code={}", ret.msg, ret.code);
        }

        match ret.data {
            Some(data) => Ok(data),
            None => err!("http client fetch return None"),
        }
    }
}
