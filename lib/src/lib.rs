mod utils;
use std::sync::{Arc, Mutex};
use serde::Serialize;
use utils::generate_random_string;

#[derive(Serialize, Clone)]
pub struct Record {
    pub id: u64,
    pub name: String
}

pub trait ThreadSafe: Send + Sync + 'static {}

pub struct DefaultRecordManager {
    last_index: u64,
    records: Vec<Record>
}

pub trait RecordManager: ThreadSafe {
    fn add(&mut self) -> Record;
    fn get(&self) -> Vec<Record>;
}

impl ThreadSafe for DefaultRecordManager {}

impl RecordManager for DefaultRecordManager {
    fn get(&self) -> Vec<Record> {
       self.records.clone()
    }
    
    fn add(&mut self) -> Record {
        self.last_index += 1;

        let record = Record {
            id: self.last_index,
            name: generate_random_string(8),
        };

        self.records.push(record.clone());

        record
    }
}

impl DefaultRecordManager {
    pub fn new() -> Self {
        let records =  vec![
            Record{
                id: 1,
                name: generate_random_string(8),
            },
            Record{
                id: 2,
                name: generate_random_string(8),
            },
            Record{
                id: 3,
                name: generate_random_string(8),
            }
        ];

        Self {
            last_index: 3,
            records
        }
    }
}

pub type SharedRecordManager = Arc<Mutex<dyn RecordManager>>;