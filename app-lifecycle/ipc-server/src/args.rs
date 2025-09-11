use clap::Parser;
use clap::ValueEnum;
use anyhow::*;

#[derive(Parser)]
#[command(author, version, about)]
pub struct CommandLineArgs {
    // let filter = "tcp and src port {}";
    #[arg(long, default_value = "inbound && tcp.SrcPort == 6040")]
    pub filter: String,

    #[arg(long, default_value = "false")]
    pub test: bool
}