use service::{
    cli::Cli,
    server::startup::startup_grpc,
    shared::{error::Error, logger::init_logger},
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = Cli::load_config()?;

    init_logger(config.logger.level)?;

    startup_grpc(config).await
}
