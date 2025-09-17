use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let tauri_context = tauri::generate_context!();

    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_single_instance::init(|_app, _argv, _cwd| {}))
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(
            tauri_plugin_window_state::Builder::new()
                .build(),
        )
        .run(tauri_context)
        .expect("error while running application");

    Ok(())
}
