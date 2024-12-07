use std::{future::Future, sync::Arc};

use tauri::async_runtime;

use crate::abstractions::{AppEvent, EventEmitter};

pub fn start<E: EventEmitter>(event_emitter: Arc<E>) {
    async_runtime::spawn_blocking(|| start_inner(event_emitter));
}

async fn start_inner<E: EventEmitter>(event_emitter: Arc<E>) {

    loop {
        let event = AppEvent::RecordUpdate {
            id: 1,
            name: "new".into()
        };

        match event_emitter.emit(event) {
            _ => {

            }
        };
    }

}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use super::*;

    #[test]
    fn should_return_records() {
        
    }
}
