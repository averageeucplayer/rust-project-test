use std::fs;

use anyhow::Result;
use flexi_logger::Duplicate;
use flexi_logger::Logger;
use log::*;

use crate::api::*;

mod api;

fn main() -> Result<()> {
    Logger::try_with_str("debug")?
        .duplicate_to_stderr(Duplicate::Warn)
        .start()?;

    if process_exists_by_name("capture.exe") {
        info!("process is already running");
        return Ok(())
    }

    let exe_path = std::env::current_exe().unwrap();
    let current_dir = exe_path.parent().unwrap().to_owned();

    let capture_path = current_dir.join(r"..\..\capture\target\debug\capture.exe");
    let capture_path = fs::canonicalize(&capture_path)?;

    info!("capture_path: {:?}", capture_path.to_string_lossy().to_string());

    let filter = "";
    launch_process(capture_path, filter)?;

    Ok(())
}
