use std::{error::Error, sync::Arc};

use lib::{DefaultRecordManager, RecordManager};
use tauri::{App, Manager};

use crate::{abstractions::DefaultEventEmitter, background_worker, updater};

pub fn setup_app(app: &mut App) -> Result<(), Box<dyn Error>> {
    let handle = app.handle().clone();

    let record_manager: Arc<dyn RecordManager> = Arc::new(DefaultRecordManager::new());
    app.manage(record_manager);

    updater::update(handle.clone());

    let main_emitter = Arc::new(DefaultEventEmitter::new(handle.clone(), "main".into()));
    let other_emitter = DefaultEventEmitter::new(handle.clone(), "other".into());

    app.manage(main_emitter.clone());
    app.manage(other_emitter);

    background_worker::start(main_emitter);

    Ok(())
}