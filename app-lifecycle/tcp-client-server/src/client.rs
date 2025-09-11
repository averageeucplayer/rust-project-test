use std::time::Duration;

use log::*;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use tokio::time::sleep;

pub async fn run(addr: &str) -> anyhow::Result<()> {

    info!("Connecting to {}", addr);
    let mut stream = TcpStream::connect(addr).await?;
    info!("Connected");
    let mut buf = vec![0u8; 1024];

    loop {
        let read = stream.read(&mut buf).await?;
        sleep(Duration::from_millis(100)).await;
    }

    Ok(())
}
