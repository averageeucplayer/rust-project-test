mod handlers;
mod setup_app;
mod updater;
mod background_worker;
mod abstractions;

use std::panic;

use handlers::generate_handlers;
use setup_app::setup_app;
use tauri::{generate_context, Context};
use tauri_plugin_log::{Target, TargetKind};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    panic::set_hook(Box::new(|info| {
        let location = info
            .location()
            .map(|loc| format!("file '{}' at line {}", loc.file(), loc.line()))
            .unwrap_or_else(|| "unknown location".to_string());

        let message = if let Some(s) = info.payload().downcast_ref::<&str>() {
            format!("panic occurred: {}", s)
        } else {
            "panic occurred, but can't retrieve the message...".to_string()
        };

        eprintln!("Custom panic handler triggered!");
        eprintln!("Error: {}\nLocation: {}", message, location);
    }));

    let context: Context = generate_context!();

    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new()
        .targets([
            Target::new(TargetKind::Stdout),
            Target::new(TargetKind::LogDir {
                file_name: Some("logs".to_string()),
            })
        ])
        .build())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .setup(setup_app)
        .invoke_handler(generate_handlers())
        .run(context)
        .expect("error while running tauri application");
}
