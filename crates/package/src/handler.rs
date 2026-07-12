use tauri::{Runtime, command, generate_handler, ipc::Invoke};

use crate::shared::logger::{Level, instrument};

#[command]
#[instrument(level = Level::DEBUG, skip_all, ret)]
fn version() -> String {
    env!("CARGO_PKG_VERSION").to_owned()
}

pub fn handler<R>() -> impl Fn(Invoke<R>) -> bool
where
    R: Runtime,
{
    generate_handler![version]
}
