use std::{io::Write, sync::{atomic::AtomicBool, Arc}, time::Duration};

use anyhow::Result;

pub struct StartArgs<W: Write> {
    pub timeout: Duration,
    pub writer: W,
    pub filter: String,
    pub should_stop: Arc<AtomicBool>
}

pub trait PacketCapture {
    fn start<W: Write>(&mut self, args: StartArgs<W>) -> Result<()>;
}