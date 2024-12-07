use lib::SharedRecordManager;
use tauri::{command, ipc::InvokeError, State};

use crate::abstractions::{AppEvent, MainEmitter};

#[command]
pub fn add_record(
    record_manager: State<'_, SharedRecordManager>,
    main_event_emitter: State<'_, MainEmitter>) -> Result<(), InvokeError> {

    let mut record_manager = record_manager.lock().unwrap();
    let record = record_manager.add();

    let event = AppEvent::NewRecord {
        id: record.id,
        name: record.name
    };

    main_event_emitter.emit(event).unwrap();

    Ok(())
}


#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use lib::{Record, RecordManager, ThreadSafe};
    use crate::{abstractions::EventEmitter, handlers::test_utils::to_state_unsafe};
    use super::*;
    use mockall::{mock, predicate::always};

    mock!{
        pub RecordManager {}
        impl RecordManager for RecordManager {
            fn add(&mut self) -> Record;
            fn get(&self) -> Vec<Record>;
        }
    }

    impl ThreadSafe for MockRecordManager {}

    mock!{
        pub EventEmitter {}
        impl EventEmitter for EventEmitter {
            fn emit(&self, event: AppEvent) -> Result<(), Box<dyn std::error::Error>>;
        }
    }

    impl ThreadSafe for MockEventEmitter {}

    #[test]
    fn should_add_record() {
        let mut event_emitter = MockEventEmitter::new();

        event_emitter
            .expect_emit()
            .with(always())
            .returning(|_| Ok(()));

        let event_emitter = MainEmitter::new(Arc::new(event_emitter));
        let event_emitter = to_state_unsafe(&event_emitter);

        let mut record_manager = MockRecordManager::new();

        record_manager
            .expect_add()
            .returning(|| Record{ id: 1, name: "test".into() });

        let record_manager: Arc<Mutex<dyn RecordManager>> = Arc::new(Mutex::new(record_manager));
        let record_manager = to_state_unsafe(&record_manager);

        let result = add_record(record_manager, event_emitter);
        assert!(result.is_ok(), "Should return records")
    }
}
