use std::{error::Error, sync::Arc};

use lib::{DefaultRecordManager, RecordManager};
use log::error;
use tauri::{async_runtime, App, AppHandle, Manager};
use tauri_plugin_updater::UpdaterExt;

pub fn setup_app(app: &mut App) -> Result<(), Box<dyn Error>> {
    let handle = app.handle().clone();

    let record_manager: Arc<dyn RecordManager> = Arc::new(DefaultRecordManager::new());
    app.manage(record_manager);

    async_runtime::spawn(async move {
        match update(handle).await {
            Err(err) => {
                error!("{}", err)
            }
            _ => {}
        }
    });

    Ok(())
}

async fn update(app: AppHandle) -> tauri_plugin_updater::Result<()> {
    if let Some(update) = app.updater()?.check().await? {
        let mut downloaded = 0;

        update
            .download_and_install(
                |chunk_length, content_length| {
                    downloaded += chunk_length;
                    println!("downloaded {downloaded} from {content_length:?}");
                },
                || {
                    println!("download finished");
                },
            )
            .await?;

        println!("update installed");
        app.restart();
    }

    Ok(())
}
