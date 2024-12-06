use lib::{Record, SharedRecordManager};
use tauri::{command, State};

#[command]
pub fn get_records(record_manager: State<'_, SharedRecordManager>) -> Vec<Record> {
    record_manager.get_records()
}


#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use lib::{DefaultRecordManager, RecordManager};
    use crate::handlers::test_utils::to_state_unsafe;
    use super::*;

    #[test]
    fn should_return_records() {
        let record_manager: Arc<dyn RecordManager> = Arc::new(DefaultRecordManager::new());
        let record_manager = to_state_unsafe(&record_manager);
        let result = get_records(record_manager);
        assert!(!result.is_empty(), "Should return records")
    }
}
