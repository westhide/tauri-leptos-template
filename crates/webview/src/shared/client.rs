use std::{
    net::SocketAddr,
    ops::{Deref, DerefMut},
};

use gloo::net::http::{Request, RequestBuilder};
use libgrpc::{
    fetch::Fetch, protowire::ping_pong_service_client::PingPongServiceClient,
    tonic::codec::CompressionEncoding::Gzip, web::GrpcWebClientService,
};
use serde::Deserialize;
use service::impl_from_ctx;

use crate::shared::error::Result;

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
pub struct HttpClient {}

impl HttpClient {
    pub fn new(url: &str) -> RequestBuilder {
        RequestBuilder::new(url)
    }

    pub async fn fetch<R>(req: Request) -> Result<R>
    where
        R: for<'de> Deserialize<'de>,
    {
        Ok(req.send().await?.json().await?)
    }
}
