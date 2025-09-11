#![allow(unreachable_code)]

mod app;
mod client;
mod server;
mod simulation;
mod args;

use anyhow::*;
use log::*;
use clap::Parser;
use flexi_logger::{DeferredNow, Duplicate, Logger};
use single_instance::SingleInstance;

use crate::{app::App, args::CommandLineArgs};


fn log_format(
    w: &mut dyn std::io::Write,
    now: &mut DeferredNow,
    record: &log::Record,
) -> std::io::Result<()> {
    write!(
        w,
        "{} [{}] {}: {}",
        now.format("%Y-%m-%d %H:%M:%S"),
        record.level(),
        record.module_path().unwrap_or("<unnamed>"),
        &record.args()
    )
}

#[tokio::main]
// #[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() -> Result<()> {
    let args = CommandLineArgs::parse();
    let crate_name = env!("CARGO_PKG_NAME");
    let instance = SingleInstance::new(crate_name).unwrap();

    if !instance.is_single() {
        error!("TCP server is already running");
        return Ok(());
    }
    
    Logger::try_with_str("debug")?
        .duplicate_to_stderr(Duplicate::Warn)
        .format(log_format)
        .start()?;

    let app = App::new(args.ip_addr, args.port)?;
    app.run().await?;

    Ok(())
}
