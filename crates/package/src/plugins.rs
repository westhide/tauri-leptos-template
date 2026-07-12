use tauri::{Runtime, plugin::Plugin};

pub fn logger<R>() -> impl Plugin<R>
where
    R: Runtime,
{
    // TODO: log level
    use tauri_plugin_log::{Builder, log::LevelFilter::Debug};
    Builder::new().level(Debug).with_colors(Default::default()).build()
}

pub use tauri_plugin_opener::init as opener;
