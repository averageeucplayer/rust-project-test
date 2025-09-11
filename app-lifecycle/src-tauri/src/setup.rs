use ipc_server_lib::IpcClient;
use log::*;
use std::{error::Error, process::Command, time::Duration};
use tauri::{App, Manager};
use tokio::time::sleep;

use crate::{
    background::{BackgroundWorker, BackgroundWorkerArgs}, context::AppContext, emitter::AppEmitter, updater::check_updates
};

pub fn setup(app: &mut App) -> Result<(), Box<dyn Error>> {
    info!("setup");

    let app_handle = app.handle();
    let context = app_handle.state::<AppContext>();
    let package_info = app_handle.package_info();
    let version = package_info.version.to_string();

    let mut background = BackgroundWorker::new();
    let emitter = AppEmitter::new(app_handle.clone());
    let ipc_client = IpcClient::new();

    check_updates(ipc_client.clone());

    let args = BackgroundWorkerArgs {
        ipc_client,
        filter: context.filter.clone(),
        emitter,
        version,
    };

    background.run(args)?;

    app_handle.manage(background);

    Ok(())
}
