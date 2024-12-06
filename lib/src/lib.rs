use std::sync::Arc;
use serde::Serialize;

#[derive(Serialize)]
pub struct Record {
    id: u64,
    name: String
}

pub struct DefaultRecordManager {}

pub trait RecordManager: Send + Sync + 'static {
    fn get_records(&self) -> Vec<Record>;
}

impl RecordManager for DefaultRecordManager {
    fn get_records(&self) -> Vec<Record> {
        vec![
            Record{
                id: 1,
                name: "test".to_string(),
            },
            Record{
                id: 2,
                name: "test".to_string(),
            },
            Record{
                id: 3,
                name: "test".to_string(),
            }
        ]
    }
}

impl DefaultRecordManager {
    pub fn new() -> Self {
        Self {

        }
    }
}

pub type SharedRecordManager = Arc<dyn RecordManager>;