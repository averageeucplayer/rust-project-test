use std::{io::Write, marker::PhantomData, sync::atomic::Ordering};

use etherparse::PacketHeaders;
use log::*;
use anyhow::{anyhow, Result};
use pcap::{Capture, Device};
use crate::types::*;

pub struct NcapPacketCapture {
    _marker: PhantomData<()>,
}

fn find_active_device() -> anyhow::Result<Device> {
    let devices = Device::list()?;
    for dev in &devices {
        
        if dev.flags.connection_status != pcap::ConnectionStatus::Connected {
            continue;
        }

        if dev.addresses.iter().find(|a| {
            matches!(a.addr, std::net::IpAddr::V4(ip) if !ip.octets().starts_with(&[169, 254]))
        }).is_some() {
            return Ok(dev.clone());
        }
    }

    anyhow::bail!("No active Ethernet/Wi-Fi device found");
}

impl PacketCapture for NcapPacketCapture {
    fn start<W: Write>(&mut self, args: StartArgs<W>) -> Result<()> {
        let StartArgs {
            filter,
            should_stop,
            mut writer,
            timeout
        } = args;

        info!("Packet capture: ncap");
        info!("Filter {:?}", filter);

        // let device = Device::lookup()?.ok_or_else(|| anyhow!("Could not find device"))?;
        let device = find_active_device()?;

        info!("{:?}", device);

        let mut capture = Capture::from_device(device)?
            .promisc(true)
            .immediate_mode(true)
            .timeout(timeout.as_millis() as i32)
            .open()?;

        capture.filter(&filter, true)?;

        let mut packet;

        loop {
            match capture.next_packet() {
                Ok(value) => packet = value,
                Err(err) => {
                    match err {
                        pcap::Error::TimeoutExpired => {
                            if should_stop.load(Ordering::Relaxed) {
                                info!("Closing...");
                                return Ok(());
                            }
                            continue;
                        },
                        _ => {
                            should_stop.store(true, Ordering::Relaxed);
                            return Ok(());
                        }
                    }
                },
            };

            let data = packet.data;
            let headers = match PacketHeaders::from_ethernet_slice(&data) {
                Ok(headers) => {
                    headers
                },
                Err(err) => {
                    info!("invalid {:?} - data - {:?}", err, data);
                    continue;
                },
            };

            writer.write_all(&data)?;
            // match headers.payload {
            //     etherparse::PayloadSlice::Tcp(data) => {
            //         // let data = data.to_vec();
            //         writer.write_all(&data)?;
            //     },
            //     _ => {}
            // }

            // let headers = PacketHeaders::from_ip_slice(&data)?;
            // let data = headers.payload.slice().to_vec();
            
            // writer.write_all(&data)?;
        }

        Ok(())
    }
}

impl NcapPacketCapture {
    pub fn new() -> Self {
        Self {
            _marker: PhantomData
        }
    }
}
