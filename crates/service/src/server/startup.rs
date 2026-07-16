use libgrpc::{
    protowire::ping_pong_service_server::PingPongServiceServer,
    tonic::{codec::CompressionEncoding::Gzip, transport::Server},
    web::GrpcWebLayer,
};
use tokio_util::sync::CancellationToken;
use tower_http::cors::CorsLayer;

use crate::{
    config::Config,
    routes::pingpong::PingPongServiceImpl,
    server::shutdown::ShutdownSignal,
    shared::{
        NULL, Null,
        error::Result,
        logger::{debug, info},
    },
};

pub async fn startup_grpc(config: Config) -> Result<Null> {
    info!("Grpc server startup");
    debug!(?config);

    let grpc_url = config.server.grpc_url;
    info!("Grpc server listen on http://{grpc_url}");

    let service = PingPongServiceServer::new(PingPongServiceImpl::default())
        .accept_compressed(Gzip)
        .send_compressed(Gzip);

    let cancellation = CancellationToken::new();
    let shutdown_signal = ShutdownSignal::new()?.wait_with_cancel(cancellation);

    Server::builder()
        .accept_http1(true)
        .layer(CorsLayer::permissive())
        .layer(GrpcWebLayer::new())
        .add_service(service)
        .serve_with_shutdown(grpc_url, shutdown_signal)
        .await?;

    info!("Grpc server shutdown");
    Ok(NULL)
}
