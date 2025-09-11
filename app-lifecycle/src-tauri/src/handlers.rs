use log::info;
use tauri::{State, command, generate_handler, ipc::Invoke};

use crate::{assets::AssetsPreloader, background::BackgroundWorker};

pub fn generate_handlers() -> Box<dyn Fn(Invoke) -> bool + Send + Sync> {
    Box::new(generate_handler![load, heartbeat])
}

#[command]
pub fn load(assets_preloader: State<AssetsPreloader>) {
    // assets_preloader.wait();

    info!("load() endpoint");
}

#[command]
pub fn heartbeat(background: State<BackgroundWorker>) -> bool {
    // assets_preloader.wait();

    background.is_running()
}
