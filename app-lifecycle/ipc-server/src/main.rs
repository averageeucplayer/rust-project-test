use ipc_server_lib::{IpcServer, DefaultPacketCapture};
use clap::Parser;
use log::*;
use anyhow::Result;
use flexi_logger::{Duplicate, Logger};
use single_instance::SingleInstance;

use crate::args::CommandLineArgs;

mod args;

fn main() -> Result<()> {
    let crate_name = env!("CARGO_PKG_NAME");
    let instance = SingleInstance::new(crate_name).unwrap();

    Logger::try_with_str("debug")?
        .duplicate_to_stderr(Duplicate::Warn)
        .start()?;

    if !instance.is_single() {
        error!("IPC server is already running");
        return Ok(());
    }

    let CommandLineArgs {
        mut filter,
        test
    } = CommandLineArgs::parse();

    filter = filter.replace("\"", "");

    let packet_capture = DefaultPacketCapture::new();
    let mut server = IpcServer::new(filter, packet_capture)?;
    server.run(test)?;

    Ok(())
}
