mod get_records;

#[cfg(test)]
mod test_utils;

use tauri::{generate_handler, ipc::Invoke};

pub fn generate_handlers() -> Box<dyn Fn(Invoke) -> bool + Send + Sync> {
    Box::new(generate_handler![get_records::get_records])
}
