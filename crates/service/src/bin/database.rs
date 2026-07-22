use service::{
    cli::Cli,
    config::server::Database,
    shared::{error::Error, logger::init_logger},
};
use surrealdb::engine::any::connect;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = Cli::load_config()?;

    init_logger(config.logger.level)?;

    let Database { url, namespace } = &config.server.database;
    let client = connect(url).await?;
    client.health().await?;
    info!("SurrealDB connected: {url}");

    let ret = client.use_ns(namespace).await?;
    info!(?ret);
    let ret = client
        .query("INFO FOR NAMESPACE STRUCTURE; USE DATABASE user_db; INFO FOR DB; INFO FOR TABLE users;")
        .await?;
    info!("{ret:#?}");

    info!("SurrealDB done");
    Ok(())
}
