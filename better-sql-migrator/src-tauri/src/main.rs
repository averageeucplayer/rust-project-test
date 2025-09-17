

#[tokio::main]
async fn main() -> Result<()> {

    tauri::Builder::default()
        .manage(settings_manager)
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_single_instance::init(|_app, _argv, _cwd| {}))
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(
            tauri_plugin_window_state::Builder::new()
                .with_state_flags(WINDOW_STATE_FLAGS)
                .build(),
        )
        .on_window_event(handler)
        .setup(|app| {
            let app_handle = app.handle();
            info!("starting app v{}", app.package_info().version);
            app::panic::set_hook(app.handle());

            if let Err(e) = setup_db(app.handle()) {
                warn!("error setting up database: {e}");
            }

            setup_tray(app_handle)?;

            let app_path = std::env::current_exe()?.display().to_string();
            app.manage(AutoLaunchManager::new(&app.package_info().name, &app_path));

            let update_checked = Arc::new(AtomicBool::new(false));
            let checked_clone = update_checked.clone();
            let cloned_app_handle = app_handle.clone();
            tauri::async_runtime::spawn(async move {
                match cloned_app_handle.updater().unwrap().check().await {
                    #[cfg(not(debug_assertions))]
                    Ok(Some(update)) => {
                        info!("update available, downloading update: v{}", update.version);

                        unload_driver();
                        remove_driver();

                        if let Err(e) = update.download_and_install(|_, _| {}, || {}).await {
                            error!("failed to download update: {}", e);
                        }
                    }
                    Err(e) => {
                        warn!("failed to get update: {e}");
                        checked_clone.store(true, Ordering::Relaxed);
                    }
                    _ => {
                        info!("no update available");
                        checked_clone.store(true, Ordering::Relaxed);
                    }
                }
            });

            let settings_manager = app_handle.state::<SettingsManager>();
            let settings = settings_manager.read()?;

            let meter_window = app.get_webview_window(METER_WINDOW_LABEL).unwrap();
            meter_window
                .restore_state(WINDOW_STATE_FLAGS)
                .expect("failed to restore window state");

            let mini_window = app.get_webview_window(METER_MINI_WINDOW_LABEL).unwrap();
            meter_window
                .restore_state(WINDOW_STATE_FLAGS)
                .expect("failed to restore window state");
            // #[cfg(debug_assertions)]
            // {
            //     meter_window.open_devtools();
            // }

            let logs_window = app.get_webview_window(LOGS_WINDOW_LABEL).unwrap();
            logs_window
                .restore_state(WINDOW_STATE_FLAGS)
                .expect("failed to restore window state");

            let mut port = 6040;

            if let Some(settings) = settings.clone() {
                info!("settings loaded");
                if settings.general.mini {
                    mini_window.show().unwrap();
                } else if !settings.general.hide_meter_on_start && !settings.general.mini {
                    meter_window.show().unwrap();
                }
                if !settings.general.hide_logs_on_start {
                    logs_window.show().unwrap();
                }
                if !settings.general.always_on_top {
                    meter_window.set_always_on_top(false).unwrap();
                    mini_window.set_always_on_top(false).unwrap();
                } else {
                    meter_window.set_always_on_top(true).unwrap();
                    mini_window.set_always_on_top(true).unwrap();
                }

                if settings.general.auto_iface && settings.general.port > 0 {
                    port = settings.general.port;
                }

                if settings.general.start_loa_on_start {
                    info!("auto launch game enabled");
                    start_loa_process(app.handle().clone());
                }
            } else {
                meter_window.show().unwrap();
                logs_window.show().unwrap();
            }

            remove_driver();

            // only start listening if we have live meter
            #[cfg(feature = "meter-core")]
            {
                let app = app.app_handle().clone();
                tokio::task::spawn_blocking(move || {
                    // only start listening when there's no update, otherwise unable to remove driver
                    while !update_checked.load(Ordering::Relaxed) {
                        std::thread::sleep(std::time::Duration::from_millis(100));
                    }
                    info!("listening on port: {port}");
                    live::start(app, port, settings).map_err(|e| {
                        error!("unexpected error occurred in parser: {e}");
                    })
                });
            }

            // #[cfg(debug_assertions)]
            // {
            //     _logs_window.open_devtools();
            // }

            Ok(())
        })
        .on_window_event(|window, event| match event {
            WindowEvent::CloseRequested { api, .. } => {
                api.prevent_close();

                if window.label() == METER_WINDOW_LABEL || window.label() == METER_MINI_WINDOW_LABEL
                {
                    let app_handle = window.app_handle();
                    let meter_window = app_handle.get_webview_window(METER_WINDOW_LABEL).unwrap();
                    let logs_window = app_handle.get_webview_window(LOGS_WINDOW_LABEL).unwrap();

                    if logs_window.is_minimized().unwrap() {
                        logs_window.unminimize().unwrap();
                    }

                    if meter_window.is_minimized().unwrap() {
                        meter_window.unminimize().unwrap();
                    }

                    app_handle
                        .save_window_state(WINDOW_STATE_FLAGS)
                        .expect("failed to save window state");
                    unload_driver();
                    app_handle.exit(0);
                } else if window.label() == LOGS_WINDOW_LABEL {
                    window.hide().unwrap();
                }
            }
            WindowEvent::Focused(focused) if !focused => {
                window
                    .app_handle()
                    .save_window_state(WINDOW_STATE_FLAGS)
                    .expect("failed to save window state");
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            load_encounters_preview,
            load_encounter,
            get_encounter_count,
            open_most_recent_encounter,
            delete_encounter,
            delete_encounters,
            toggle_meter_window,
            toggle_logs_window,
            open_url,
            save_settings,
            open_db_path,
            delete_encounters_below_min_duration,
            get_db_info,
            disable_blur,
            enable_blur,
            write_log,
            toggle_encounter_favorite,
            delete_all_encounters,
            delete_all_uncleared_encounters,
            enable_aot,
            disable_aot,
            set_clickthrough,
            optimize_database,
            check_start_on_boot,
            set_start_on_boot,
            check_loa_running,
            start_loa_process,
            get_sync_candidates,
            sync,
            remove_driver,
            unload_driver,
        ])
        .run(tauri_context)
        .expect("error while running application");

    Ok(())
}
