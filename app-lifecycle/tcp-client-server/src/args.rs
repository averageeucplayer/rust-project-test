use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
pub struct CommandLineArgs {
    // let filter = "tcp and src port {}";
    #[arg(long, default_value = "127.0.0.1")]
    pub ip_addr: String,

    #[arg(long, default_value = "42069")]
    pub port: u16
}