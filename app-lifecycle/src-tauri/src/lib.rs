#![allow(unreachable_code)]

use log::{error, LevelFilter};

use crate::assets::AssetsPreloader;
use crate::context::AppContext;
use crate::handlers::generate_handlers;
use crate::setup::setup;

mod context;
mod assets;
mod background;
mod emitter;
mod handlers;
mod setup;
mod models;
mod updater;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let tauri_context = tauri::generate_context!();
    let context = AppContext::new().expect("Could not setup context");
    let assets_preloader = AssetsPreloader::new();

    std::panic::set_hook(Box::new(|info| {
        let payload = if let Some(s) = info.payload().downcast_ref::<&str>() {
            (*s).to_string()
        } else if let Some(s) = info.payload().downcast_ref::<String>() {
            s.clone()
        } else {
            "non-string panic payload".to_string()
        };
        
        error!("Panicked: {:?}, location: {:?}", payload, info.location());

        log::logger().flush();
    }));


    let log_builder = tauri_plugin_log::Builder::new()
        .level(log::LevelFilter::Info)
        .level_for(
            "tao::platform_impl::platform::event_loop::runner",
            LevelFilter::Error,
        )
        .max_file_size(5_000_000)
        .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepAll)
        .target(tauri_plugin_log::Target::new(
            tauri_plugin_log::TargetKind::Folder {
                path: context.current_dir.clone(),
                file_name: Some("app_lifecycle".to_string()),
            },
        ));

    tauri::Builder::default()
        .manage(context)
        .manage(assets_preloader)
        .plugin(log_builder.build())
        .plugin(tauri_plugin_opener::init())
        .setup(setup)
        .invoke_handler(generate_handlers())
        .run(tauri_context)
        .expect("error while running tauri application");
}
