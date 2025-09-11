use anyhow::Result;
use log::*;
use std::{
    io::Read,
    path::PathBuf,
    sync::mpsc::{channel, Receiver, Sender},
    thread::{self, sleep, JoinHandle}, time::Duration,
};
use interprocess::local_socket::{traits::Stream as StreamExt, GenericFilePath, GenericNamespaced, Name, NameType, Stream, ToFsName, ToNsName};

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum Kind {
    Unknown,
    Spawn,
    Damage,
    ZoneChange
}


pub struct Capture {
    capture_path: PathBuf,
    handle: Option<JoinHandle<()>>,
}

impl Capture {
    pub fn new(capture_path: PathBuf) -> Self {
        Self { 
            capture_path,
            handle: None
        }
    }

    pub fn start(&mut self, port: u16) -> Result<Receiver<(Kind, Vec<u8>)>> {
        let (tx, rx) = channel();

        let builder = thread::Builder::new();

        let exe_path = self.capture_path.clone();
        let handle = builder.spawn(move || Self::inner(exe_path, port, tx))?;

        self.handle = Some(handle);

        Ok(rx)
    }

    fn inner(exe_path: PathBuf, port: u16, tx: Sender<(Kind, Vec<u8>)>) {
        match Self::run(exe_path, port, tx) {
            Ok(_) => {
                warn!("Early return from capture");
            }
            Err(err) => {
                error!("An error occurred whilst capturing {}", err);
            }
        }
    }

    fn run(exe_path: PathBuf, port: u16, tx: Sender<(Kind, Vec<u8>)>) -> Result<()> {

        loop {
            let name = get_socket_name()?;
            let client = Stream::connect(name);

            let mut stream = match client {
                Ok(stream) => stream,
                Err(err) => {
                    error!("Error: {:?} Trying again in 5s", err);
                    sleep(Duration::from_secs(5));
                    continue;
                },
            };
            let mut data = vec![0u8; 1000];

            loop {
                let read = stream.read(&mut data)?;

                if read == 0 {
                    info!("broken connection");
                    break;
                }

                let kind = match data.first().cloned().unwrap_or_else(|| u8::MAX) {
                    0 => Kind::Spawn,
                    1 => Kind::Damage,
                    2 => Kind::ZoneChange,
                    _ => Kind::Unknown,
                };

                info!("received {:?}", data);
                let payload_bytes = data[1..].to_vec();
                tx.send((kind, payload_bytes))?;
            }
        }

        Ok(())
    }
}

pub fn get_socket_name<'a>() -> Result<Name<'a>> {
    let name: Name<'_> = if GenericNamespaced::is_supported() {
        "capture.sock".to_ns_name::<GenericNamespaced>()?
    } else {
        "/tmp/capture.sock".to_fs_name::<GenericFilePath>()?
    };

    Ok(name)
}
