pub mod context;
pub mod handler;
pub mod plugins;
pub mod shared;
// pub mod rpc;

use tauri::{Builder, Result};

use crate::{
    context::{context, setup},
    handler::handler,
    plugins::{logger, opener},
    shared::Null,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn startup() -> Result<Null> {
    Builder::default()
        .plugin(opener())
        .plugin(logger())
        .setup(setup)
        .invoke_handler(handler())
        .run(context())
}
