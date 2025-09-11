use anyhow::Result;
use flexi_logger::Duplicate;
use flexi_logger::Logger;

mod background;
mod capture;

use crate::background::*;
use std::fs;

fn main() -> Result<()> {
    Logger::try_with_str("debug")?
        .duplicate_to_stderr(Duplicate::Warn)
        .start()?;

    let exe_path = std::env::current_exe().unwrap();
    let current_dir = exe_path.parent().unwrap().to_owned();

    let capture_path = current_dir.join(r"..\..\capture\target\debug\capture.exe");
    let capture_path = fs::canonicalize(&capture_path)?;

    let args = BackgroundWorkerArgs {
        capture_path,
        port: 42069,
        version: "0.0.1".to_string(),
    };

    let mut background = BackgroundWorker::new();

    background.run(args)?;

    background.wait();

    Ok(())
}
