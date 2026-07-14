use libgrpc::{
    fetch::Fetch,
    protowire::{
        Ping, Pong, ping_pong_service_client::PingPongServiceClient,
        ping_pong_service_server::PingPongService,
    },
    tonic::{
        Request, Response, Status, async_trait, codec::CompressionEncoding::Gzip,
        codegen::http::Uri,
    },
    web::GrpcWebClientService,
};

use crate::shared::error::Result;

#[derive(Debug, Default)]
pub struct PingPongServiceImpl {}

#[async_trait]
impl PingPongService for PingPongServiceImpl {
    async fn pingpong(&self, ping: Request<Ping>) -> Result<Response<Pong>, Status> {
        let Ping { id } = ping.into_inner();
        Ok(Response::new(Pong { id: id + 1 }))
    }
}

pub fn client(uri: Uri) -> PingPongServiceClient<GrpcWebClientService<Fetch>>
// where GrpcWebClientService<T>: GrpcService<GrpcBody, ResponseBody = GrpcBody>,
{
    let inner = GrpcWebClientService::new(Fetch {});
    PingPongServiceClient::with_origin(inner, uri).accept_compressed(Gzip).send_compressed(Gzip)
}
