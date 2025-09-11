use anyhow::*;
use byteorder::{LittleEndian, ReadBytesExt};
use ipc_server_lib::{IpcClient, IpcClientError};
use log::*;
use std::io::{Cursor, Read};
use std::result::Result::Ok;
use std::thread::{self, sleep, JoinHandle};
use std::time::Duration;

use crate::emitter::Event;
use crate::models::Kind;
use crate::{emitter::AppEmitter};

pub struct BackgroundWorkerArgs {
    pub ipc_client: IpcClient,
    pub emitter: AppEmitter,
    pub filter: String,
    pub version: String,
}

pub struct BackgroundWorker(Option<JoinHandle<()>>);

impl BackgroundWorker {
    pub fn new() -> Self {
        Self(None)
    }

    pub fn run(&mut self, args: BackgroundWorkerArgs) -> Result<()> {
        info!("starting background-worker");

        let mut builder = thread::Builder::new();

        builder = builder.name("background-worker".to_owned());

        let handle = builder.spawn(|| 
            if let Err(err) = Self::inner(args) {
                error!("{:?}", err);
            }
        )?;

        self.0 = Some(handle);

        Ok(())
    }

    pub fn is_running(&self) -> bool {
        self.0.as_ref().is_some_and(|pr| !pr.is_finished())
    }

    fn inner(args: BackgroundWorkerArgs) -> Result<()> {
        info!("background-worker separate thread");

        let BackgroundWorkerArgs {
            mut ipc_client,
            filter,
            emitter,
            version,
        } = args;

        loop {
            let mut reader = retry(
                || ipc_client.start(&filter),
                10,
                Duration::from_secs(5),
            )?;

            loop {
                match reader.read() {
                    Ok(data) => Self::on_data(data, &emitter)?,
                    Err(err) => match err {
                        IpcClientError::ClosedConnection => {
                            sleep(Duration::from_secs(5));
                            info!("Closed connection! Restarting capture after 5sec..");
                            continue;
                        },
                        _ => return Err(err.into())
                    },
                }
            }
        }

        Ok(())
    }

    fn on_data(data: &[u8], emitter: &AppEmitter) -> Result<()> {
        let kind = match data.first().cloned().unwrap_or_else(|| u8::MAX) {
            0 => Kind::Spawn,
            1 => Kind::Damage,
            2 => Kind::ZoneChange,
            _ => Kind::Unknown,
        };

        match kind {
            Kind::Unknown => {
                info!("unknown");
            },
            Kind::Spawn => {
                let data = &data[1..];
                let mut cursor = Cursor::new(data);
                cursor.read_u8()?;
                let id = cursor.read_u64::<LittleEndian>()?;
                let str_len = cursor.read_u8()? as usize;
                let mut buf = vec![0; str_len];
                cursor.read_exact(&mut buf)?;
                let name = std::str::from_utf8(&buf).unwrap().to_string();

                let event = Event::Spawn { id, name };
                emitter.emit(event);
            },
            Kind::Damage => {
                let data = &data[1..];
                let mut cursor = Cursor::new(data);
                cursor.read_u8()?;
                let id = cursor.read_u64::<LittleEndian>()?;
                cursor.read_u8()?;
                let damage = cursor.read_i16::<LittleEndian>()?;

                let event = Event::Damage { id: id, value: damage as i64 };
                emitter.emit(event);
            },
            Kind::ZoneChange => {
                let event = Event::ZoneChange;
                emitter.emit(event);
            },
        }

        Ok(())
    }
}

fn retry<F, T>(mut op: F, max_attempts: u32, delay: Duration) -> Result<T>
where
    F: FnMut() -> Result<T>,
{
    let mut last_err = None;

    for attempt in 1..=max_attempts {
        match op() {
            Ok(value) => return Ok(value),
            Err(e) => {
                last_err = Some(e);
                warn!("Attempt {attempt}/{max_attempts} failed. Retrying in {:?}...", delay);
                sleep(delay);
            }
        }
    }

    Err(anyhow!("All {} attempts failed. Last error: {:?}", max_attempts, last_err))
}