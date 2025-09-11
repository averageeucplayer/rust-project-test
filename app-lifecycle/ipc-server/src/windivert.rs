use std::{io::Write, marker::PhantomData, sync::atomic::Ordering};

use etherparse::PacketHeaders;
use log::*;
use anyhow::Result;
use windivert::{error::WinDivertError, WinDivert};
use windivert_sys::WinDivertFlags;
use crate::types::*;

pub struct WindivertPacketCapture {
    _marker: PhantomData<()>,
}

impl PacketCapture for WindivertPacketCapture {
    fn start<W: Write>(&mut self, args: StartArgs<W>) -> Result<()> {
        let StartArgs {
            filter,
            should_stop,
            mut writer,
            timeout
        } = args;

        info!("Packet capture: windivert");
        info!("Filter {:?}", filter);

        let mut buffer = vec![0u8; 65535];
        
        let flags = WinDivertFlags::new().set_recv_only().set_sniff();
        let windivert = WinDivert::network(&filter, 0, flags)?;

        loop {
            let packet;

            match windivert.recv_wait(&mut buffer, timeout.as_millis() as u32) {
                Ok(value) => packet = value,
                Err(err) => {
                    match err {
                        WinDivertError::Timeout => {
                            if should_stop.load(Ordering::Relaxed) {
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
            }

            let data = packet.data;
            let headers = PacketHeaders::from_ip_slice(&data)?;
            let data = headers.payload.slice();

            writer.write_all(&data)?;
        }

        Ok(())
    }
}

impl WindivertPacketCapture {
    pub fn new() -> Self {
        Self {
            _marker: PhantomData
        }
    }
}