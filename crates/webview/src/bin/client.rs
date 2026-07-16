use leptos::prelude::*;
use tracing::{info, log::Level};
use webview::{shared::logger::init_logger, views::Main};

// #[tokio::main(flavor = "current_thread")] async
fn main() {
    init_logger(Level::Info);

    info!("Webview startup");

    mount_to_body(Main)
}
