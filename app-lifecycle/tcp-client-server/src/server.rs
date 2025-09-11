use std::time::Duration;

use log::*;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tokio::time::sleep;

use crate::simulation::Simulation;

pub async fn run(addr: &str) -> anyhow::Result<()> {
    let addr = addr.to_string();

    let listener = TcpListener::bind(&addr).await.unwrap();
    info!("Listening on {}", addr);

    let (mut socket, client_addr) = listener.accept().await.unwrap();
    info!("Client connected: {}", client_addr);

    let delay = Duration::from_millis(500);

    let mut simulation = Simulation::new();
    info!("starting simulation");

    loop {
        let packets = simulation.tick();

        if packets.is_empty() {
            info!("starting new simulation");
            simulation = Simulation::new();
            continue;
        }

        for packet in packets {
            socket.write_all(&packet).await.unwrap();
            sleep(delay).await;
        }
    }

    Ok(())
}
