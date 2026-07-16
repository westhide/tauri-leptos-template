use std::{
    net::SocketAddr,
    ops::{Deref, DerefMut},
};

use libgrpc::{
    fetch::Fetch, protowire::ping_pong_service_client::PingPongServiceClient,
    tonic::codec::CompressionEncoding::Gzip, web::GrpcWebClientService,
};
use service::impl_from_ctx;

use crate::shared::error::Result;

type ClientImpl = PingPongServiceClient<GrpcWebClientService<Fetch>>;

#[derive(Debug, Clone)]
pub struct GrpcClient {
    inner: ClientImpl,
}

impl Deref for GrpcClient {
    type Target = ClientImpl;

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
