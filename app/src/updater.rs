use log::{error, info};
use tauri::{async_runtime, AppHandle};
use tauri_plugin_updater::UpdaterExt;

pub fn update(handle: AppHandle) {
    async_runtime::spawn(async move {
        match update_inner(handle).await {
            Err(err) => {
                error!("{}", err)
            }
            _ => {}
        }
    });
}

fn on_chunk(mut downloaded: u64, chunk_length: usize, content_length: Option<u64>) {
    downloaded += chunk_length as u64;
    info!("downloaded {downloaded} from {content_length:?}");
}

fn on_finish() {
    info!("download finished");
}

async fn update_inner(app: AppHandle) -> tauri_plugin_updater::Result<()> {

    if let Some(update) = app.updater()?.check().await? {
        let mut downloaded = 0;

        update.download_and_install(
            |chunk_length, content_length| on_chunk(downloaded, chunk_length, content_length),
            on_finish).await?;

        info!("update installed");
        app.restart();
    }

    Ok(())
}
