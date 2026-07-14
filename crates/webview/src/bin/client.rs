use leptos::prelude::*;
use webview::{
    shared::logger::{info, init_console_log},
    views::Main,
};

// #[tokio::main(flavor = "current_thread")] async
fn main() {
    init_console_log("info").ok();

    info!("Webview startup");

    mount_to_body(Main)
}
