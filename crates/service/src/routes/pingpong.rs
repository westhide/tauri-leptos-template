use libgrpc::{
    protowire::{Ping, Pong, ping_pong_service_server::PingPongService},
    tonic::{Request, Response, Status, async_trait},
};

use crate::shared::{
    error::Result,
    logger::{Level, instrument},
};

#[derive(Debug, Default)]
pub struct PingPongServiceImpl {}

#[async_trait]
impl PingPongService for PingPongServiceImpl {
    #[instrument(level = Level::DEBUG, skip(self), ret, err)]
    async fn pingpong(&self, ping: Request<Ping>) -> Result<Response<Pong>, Status> {
        let Ping { id } = ping.into_inner();
        Ok(Response::new(Pong { id: id + 1 }))
    }
}
