use libgrpc::{
    protowire::ping_pong_service_server::PingPongServiceServer,
    tonic::{codec::CompressionEncoding::Gzip, transport::Server},
    web::GrpcWebLayer,
};
use service::{
    cli::Cli,
    routes::pingpong::PingPongServiceImpl,
    shared::{NULL, Null, error::Error, logger::init_logger},
};
use tower_http::cors::CorsLayer;
use tracing::info;

#[tokio::main]
async fn main() -> Result<Null, Error> {
    let config = Cli::load_config()?;

    init_logger(&config.logger.level)?;

    let service = PingPongServiceServer::new(PingPongServiceImpl::default())
        .accept_compressed(Gzip)
        .send_compressed(Gzip);

    info!("Service startup");
    info!("Server listen on http://{}", config.server.grpc_url);

    Server::builder()
        .accept_http1(true)
        .layer(CorsLayer::permissive())
        .layer(GrpcWebLayer::new())
        .add_service(service)
        .serve(config.server.grpc_url)
        .await?;

    info!("Service shutdown");
    Ok(NULL)
}
