use libgrpc::{
    tonic::{codec::CompressionEncoding::Gzip, transport::Server},
    web::GrpcWebLayer,
};
use service::{
    cli::Cli,
    routes::pingpong,
    shared::{
        NULL, Null,
        error::Result,
        logger::{info, init_logger},
    },
};

#[tokio::main]
async fn main() -> Result<Null, Box<dyn std::error::Error>> {
    let config = Cli::load_config()?;

    init_logger(&config.logger.level)?;

    let service = pingpong::service().accept_compressed(Gzip).send_compressed(Gzip);

    info!("Service startup");
    info!("Server listen on http://{}", config.server.grpc_url);

    Server::builder()
        .accept_http1(true)
        .layer(GrpcWebLayer::new())
        .add_service(service)
        .serve(config.server.grpc_url)
        .await?;

    info!("Service shutdown");
    Ok(NULL)
}
