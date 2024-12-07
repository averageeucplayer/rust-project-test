use std::{error::Error, fmt::Display, sync::Arc};

use lib::ThreadSafe;
use tauri::{AppHandle, Emitter, Runtime};
use std::ops::Deref;
use strum_macros::{Display, EnumString};

#[derive(Display, EnumString)]
pub enum AppEvent {
    #[strum(serialize = "new-record")]
    NewRecord {
        id: u64,
        name: String
    },

    #[strum(serialize = "record-update")]
    RecordUpdate {
        id: u64,
        name: String
    }
}

pub trait EventEmitter: ThreadSafe {
    fn emit(&self, event: AppEvent) -> Result<(), Box<dyn Error>>;
}

pub struct DefaultEventEmitter<R: Runtime> {
    handle: AppHandle<R>,
    target: tauri::EventTarget
}


impl<R: Runtime> ThreadSafe for DefaultEventEmitter<R> {}

impl<R: Runtime> EventEmitter for DefaultEventEmitter<R> {
    fn emit(&self, event: AppEvent) -> Result<(), Box<dyn Error>> {

        // let event_target = match target {
        //     EventTarget::WebviewWindow { label } => {
        //         tauri::EventTarget::webview_window(label)
        //     },
        //     _ => {
        //         panic!();
        //     }
        // };

        // Emitter::emit_to(&self.handle, event_target, event, payload)
        //     .map_err(|err| Box::new(err) as Box<dyn Error>)

        Ok(())
    }
}

impl<R: Runtime> DefaultEventEmitter<R> {
    pub fn new(handle: AppHandle<R>, target: String) -> Self {
        let target = tauri::EventTarget::WebviewWindow { 
            label: target
        };

        Self {
            handle,
            target
        }
    }
}

pub struct MainEmitter(Arc<dyn EventEmitter>);

impl MainEmitter {
    pub fn new(event_emitter: Arc<dyn EventEmitter>) -> Self {
        Self(event_emitter)
    }
}

impl Deref for MainEmitter {
    type Target = Arc<dyn EventEmitter>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct OtherEmitter(Arc<dyn EventEmitter>);

pub type SharedEventEmitter = Arc<dyn EventEmitter>;