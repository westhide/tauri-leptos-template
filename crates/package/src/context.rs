use service::{cli::Cli, server::startup::startup_grpc};
use tauri::{App, Context, Manager, Runtime, async_runtime::spawn, generate_context};

use crate::shared::{
    NULL, Null,
    logger::{info, warn},
};

pub fn context<R>() -> Context<R>
where
    R: Runtime,
{
    generate_context!()
}

#[cfg(debug_assertions)]
fn open_devtools(app: &mut App) {
    if let Some(window) = app.get_webview_window("main") {
        info!("open devtools");
        window.open_devtools();
    } else {
        warn!("skip devtools");
    }
}

pub fn setup(app: &mut App) -> Result<Null, Box<dyn std::error::Error>> {
    #[cfg(debug_assertions)]
    open_devtools(app);

    info!("Application setup");

    let config = Cli::load_config()?;

    if !config.server.start_grpc {
        spawn(startup_grpc(config));
    }

    Ok(NULL)
}
