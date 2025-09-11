use anyhow::*;
use byteorder::{LittleEndian, ReadBytesExt};
use log::*;
use std::io::{Cursor, Read};
use std::path::PathBuf;
use std::result::Result::Ok;
use std::thread::{self, JoinHandle};

use crate::capture::Capture;

pub struct BackgroundWorkerArgs {
    pub capture_path: PathBuf,
    pub port: u16,
    pub version: String,
}

pub struct BackgroundWorker(Option<JoinHandle<Result<()>>>);

impl BackgroundWorker {
    pub fn new() -> Self {
        Self(None)
    }

    pub fn run(&mut self, args: BackgroundWorkerArgs) -> Result<()> {
        info!("starting background-worker");

        let mut builder = thread::Builder::new();

        builder = builder.name("background-worker".to_owned());

        let handle = builder.spawn(|| Self::inner(args))?;

        self.0 = Some(handle);

        Ok(())
    }

    fn inner(args: BackgroundWorkerArgs) -> Result<()> {
        info!("background-worker separate thread");

        let BackgroundWorkerArgs {
            capture_path,
            port,
            version,
        } = args;

        info!("running on port: {}", port);
        let mut capture = Capture::new(capture_path);

        let (rx) = capture.start(port)?;

        info!("recv");
        while let Ok((packet, data)) = rx.recv() {
            info!("recv enter");

            // match packet {
            //     capture_lib::models::Kind::Unknown => {
            //         info!("unknown");
            //     },
            //     capture_lib::models::Kind::Spawn => {
            //         info!("spawn {:?}", data);
            //         let mut cursor = Cursor::new(data);
            //         let id = cursor.read_u64::<LittleEndian>()?;
            //         let str_len = cursor.read_u8()? as usize;
            //         let mut buf = Vec::with_capacity(str_len);
            //         cursor.read_exact(&mut buf)?;
            //         let name = std::str::from_utf8(&buf).unwrap().to_string();

            //         info!("spawn {} {}", id, name);
            //     },
            //     capture_lib::models::Kind::Damage => {
            //         info!("damage {:?}", data);
            //         let mut cursor = Cursor::new(data);
            //         let id = cursor.read_u64::<LittleEndian>()?;
            //         let damage = cursor.read_i64::<LittleEndian>()?;

            //     },
            //     capture_lib::models::Kind::ZoneChange => {
            //         info!("zone-change");
            //     },
            // }            
        }

        Ok(())
    }

    pub fn wait(&mut self) {
        if let Some(handle) = self.0.take() {
            handle.join().unwrap().unwrap();
        }
    }
}
