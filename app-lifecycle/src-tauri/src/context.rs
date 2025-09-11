use std::{fs, path::PathBuf};
use anyhow::Result;

pub struct AppContext {
    pub filter: String,
    pub exe_path: PathBuf,
    pub current_dir: PathBuf
}

impl AppContext {
    pub fn new() -> Result<Self> {
        let exe_path = std::env::current_exe().unwrap();
        let current_dir = exe_path.parent().unwrap().to_owned();

        // let capture_path = current_dir.join(r"..\..\ipc-server\target\debug\capture.exe");
        // let capture_path = fs::canonicalize(&capture_path)?;

        let filter = env!("FILTER").to_string();

        Ok(Self {
            filter,
            exe_path,
            current_dir
        })
    }
}