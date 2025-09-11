use anyhow::*;
use tokio::try_join;

use crate::{client, server};

pub struct App {
    addr: String,
}

impl App {
    pub fn new(ip_addr: String, port: u16) -> Result<Self> {
        let addr = format!("{}:{}", ip_addr, port);

        Ok(Self { addr })
    }

    pub async fn run(self) -> Result<()> {
        let addr = self.addr.clone();

        let server_handle = server::run(&addr);
        let client_handle = client::run(&addr);
        let _ = try_join!(server_handle, client_handle);

        Ok(())
    }
}
