use anyhow::{bail, Result};
use interprocess::local_socket::{traits::Stream as StreamExt, RecvHalf, SendHalf, Stream};
use log::*;
use std::{
    io::{Read, Write},
    sync::{atomic::{AtomicBool, Ordering}, Arc, Mutex},
    thread::JoinHandle,
};

use crate::get_socket_name;

#[derive(Debug, thiserror::Error)]
pub enum IpcClientError {
    #[error("Shutdown requested by client")]
    Shutdown,

    #[error("Connection closed by server")]
    ClosedConnection,

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub struct IpcClient {
    stop_handle: Arc<AtomicBool>,
    sender: Arc<Mutex<Option<SendHalf>>>,
    handle: Arc<Mutex<Option<JoinHandle<()>>>>,
}

pub struct IpcClientReader {
    recv: Stream,
    stop_handle: Arc<AtomicBool>,
    buffer: Vec<u8>,
}

impl IpcClientReader {
    pub fn new(recv: Stream, stop_handle: Arc<AtomicBool>) -> Self {
        Self {
            recv,
            stop_handle,
            buffer: vec![0; 1000],
        }
    }

    pub fn read(&mut self) -> Result<&[u8], IpcClientError> {
        let read = self.recv.read(&mut self.buffer)?;

        if self.stop_handle.load(Ordering::Relaxed) {
            return Err(IpcClientError::Shutdown);
        }

        if read == 0 {
            return Err(IpcClientError::ClosedConnection);
        }

        Ok(&self.buffer[..read])
    }
}

impl IpcClient {
    pub fn new() -> Self {
        Self { 
            stop_handle: Arc::new(AtomicBool::new(false)),
            sender: Arc::new(Mutex::new(None)),
            handle: Arc::new(Mutex::new(None)),
        }
    }

    pub fn start(&mut self, filter: &str) -> Result<IpcClientReader> {
        info!("Filter: {}", filter);

        let name = get_socket_name(filter)?;

        info!("Socket name: {:?}", name);

        let client = Stream::connect(name)?;

        // let (recv, sender) = client.split();

        // *self.sender.lock().unwrap() = Some(sender);

        Ok(IpcClientReader::new(client, self.stop_handle.clone()))
    }

    // pub fn start(&mut self, filter: String) -> Result<Receiver<(Kind, Vec<u8>)>> {

    //     let name = get_socket_name(&filter)?;
    //     let client = Stream::connect(name)?;

    //     let (tx, rx) = channel();

    //     let builder = thread::Builder::new();

    //     let stop_handle = self.stop_handle.clone();
    //     let sender = self.sender.clone();
    //     let handle = builder.spawn(move || Self::inner(filter, tx, stop_handle, sender))?;

    //     *self.handle.lock().unwrap() = Some(handle);

    //     Ok(rx)
    // }

    pub fn stop(&self) -> Result<()> {
        self.stop_handle.store(true, Ordering::Relaxed);

        // if let Some(mut sender) = self.sender.lock().unwrap().take() {
        //     sender.write(b"STOP").unwrap();
        // }

        if let Some(handle) = self.handle.lock().unwrap().take() {
            handle.join().unwrap();
        }

        Ok(())
    }
}

impl Clone for IpcClient {
    fn clone(&self) -> Self {
        Self {
            stop_handle: self.stop_handle.clone(),
            sender: self.sender.clone(),
            handle: self.handle.clone(),
        }
    }
}