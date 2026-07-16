use service::{
    cli::Cli,
    server::startup::startup_grpc,
    shared::{Null, error::Error, logger::init_logger},
};

#[tokio::main]
async fn main() -> Result<Null, Error> {
    let config = Cli::load_config()?;

    init_logger(config.logger.level)?;

    startup_grpc(config).await
}
