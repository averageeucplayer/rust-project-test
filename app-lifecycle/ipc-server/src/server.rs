use anyhow::Result;
use interprocess::{
    local_socket::{
        traits::{Listener as ListenerExt, Stream}, Listener, ListenerOptions, RecvHalf
    },
};
use log::*;
use std::{
    io::Read, sync::{atomic::{AtomicBool, Ordering}, Arc}, thread::{self}, time::Duration 
};

macro_rules! check_stop {
    ($should_stop:expr) => {
        if $should_stop.load(Ordering::Relaxed) {
            info!("Closing...");
            return Ok(());
        }
    };
}

use crate::{logger::LoggerWriter, types::{PacketCapture, StartArgs}, utils::get_socket_name};

pub struct IpcServer<PC: PacketCapture> {
    filter: String,
    should_stop: Arc<AtomicBool>,
    handle: Option<thread::JoinHandle<()>>,
    listener: Listener,
    packet_capture: PC,
}

impl<PC: PacketCapture> IpcServer<PC> {
    pub fn new(filter: String, packet_capture: PC) -> Result<Self> {
        let should_stop: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
        let listener = Self::setup_listener(&filter)?;

        Ok(Self {
            filter,
            should_stop,
            handle: None,
            listener,
            packet_capture
        })
    }

    fn setup_listener(filter: &str) -> Result<Listener> {
        let name = get_socket_name(filter)?;
        info!("socket name: {:?}", name);

        let mut builder = ListenerOptions::new().name(name);

        #[cfg(target_os = "windows")]
        {
            info!("windows: applying unrestricted access");
            use interprocess::os::windows::local_socket::ListenerOptionsExt;
            use crate::security::create_unrestricted_descriptor;

            let descriptor = create_unrestricted_descriptor()?;
            builder = builder.security_descriptor(descriptor);
        }

        let listener: Listener = builder.create_sync()?;

        Ok(listener)
    }

    // fn run_test(&self, filter: &str) -> Result<()> {
    //     info!("Test run mode");
    //     // let filter = format!("tcp.SrcPort == 42068");
    //     // let filter = "loopback";
    //     // let filter = format!("tcp.DstPort == 42068 || tcp.SrcPort == {} || tcp.SrcPort == 42068", port);
    //     // let filter = format!("outbound && (tcp.SrcPort == {} || tcp.SrcPort == 42068)", port);
    //     info!("Filter: {}", filter);

    //     let mut buffer = vec![0u8; 65535];
    //     let flags = WinDivertFlags::new().set_recv_only().set_sniff();
    //     let mut windivert = WinDivert::network(&filter, 0, flags)?;

    //     loop {
    //         let packet = windivert.recv(&mut buffer)?;

    //         let headers = PacketHeaders::from_ip_slice(&packet.data)?;
    //         let payload = headers.payload.slice().to_vec();

    //         info!("{:?}", payload);
    //     }
    // }
    
    pub fn run(&mut self, test_run: bool) -> Result<()> {
        if test_run {
            info!("Test run");

            let args = StartArgs {
                timeout: Duration::from_secs(5),
                filter: self.filter.to_string(),
                should_stop: self.should_stop.clone(),
                writer: LoggerWriter
            };

            self.packet_capture.start(args)?;

            return Ok(());
        }

        self.run_pipe_server(&self.filter.clone())?;

        Ok(())
    }

    /// concurrent I/O with a named pipe attempted – 
    /// this leads to deadlocks due to the synchronization used by named pipes on Windows internally, and was prevented
    /// because it would have caused a deadlock stack backtrace:
    // fn setup_server(&mut self, mut recv: RecvHalf) {
    //     let should_stop = self.should_stop.clone();

    //     let handle: thread::JoinHandle<()> = thread::spawn(move || {
    //         let mut buffer = vec![0u8; 10];

    //         match recv.read(&mut buffer) {
    //             Ok(0) => {
    //                 should_stop.store(true, Ordering::Release);
    //             }
    //             Ok(n) => {
    //                 let message = &buffer[..n];
    //                 if message == b"STOP" {
    //                     should_stop.store(true, Ordering::Release);
    //                 }
    //             }
    //             Err(err) => {
    //                 error!("Error reading from socket: {:?}", err);
    //             }
    //         }
    //     });

    //     self.handle = Some(handle);
    // }

    fn run_pipe_server(&mut self, filter: &str) -> Result<()> {
       
        
        loop {
            info!("Waiting for client...");

            check_stop!(self.should_stop);

            let stream = match self.listener.accept() {
                Ok(stream) => stream,
                Err(err) => {
                    error!("Failed to accept client: {err}");
                    continue;
                }
            };

            check_stop!(self.should_stop);

            // let (recv, send) = stream.split();
            // self.setup_server(recv);

            info!("Client connected");

            let args = StartArgs {
                timeout: Duration::from_secs(5),
                filter: filter.to_string(),
                should_stop: self.should_stop.clone(),
                writer: stream
            };

            self.packet_capture.start(args)?;
        }
    }
}