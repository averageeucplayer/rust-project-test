use anyhow::{Result, anyhow};
use std::{
    sync::Mutex,
    thread::{self, JoinHandle, sleep},
    time::Duration,
};

pub struct AssetsPreloader(Mutex<Option<JoinHandle<()>>>);

impl AssetsPreloader {
    pub fn new() -> Self {
        let mut builder = thread::Builder::new();

        builder = builder.name("assets-loader".to_string());

        let handle = builder
            .spawn(|| {
                sleep(Duration::from_secs(1));
            })
            .unwrap();

        Self(Mutex::new(Some(handle)))
    }

    pub fn wait(&self) -> Result<()> {
        if let Some(handle) = self.0.lock().unwrap().take() {
            handle
                .join()
                .map_err(|err| anyhow!("Could not load assets: {:?}", err))?;
        }

        Ok(())
    }
}
