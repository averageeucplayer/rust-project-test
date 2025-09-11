use log::info;
use serde::Serialize;
use tauri::{AppHandle, Emitter};

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "app-event", content = "data")]
pub enum Event {
    Spawn {
        id: u64,
        name: String
    },
    Damage {
        id: u64,
        value: i64
    },
    ZoneChange
}

pub struct AppEmitter(AppHandle);

impl AppEmitter {
    pub fn new(app_handle: AppHandle) -> Self {
        Self(app_handle)
    }

    pub fn emit(&self, event: Event) {
        // info!("sending: {:?}", event);
        unsafe { self.0.emit("app-event", event).unwrap_unchecked() };
    }
}
