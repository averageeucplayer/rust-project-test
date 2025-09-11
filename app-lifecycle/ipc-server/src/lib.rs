#![allow(unreachable_code)]

pub mod server;
pub mod utils;
pub mod client;
mod logger;
mod types;

pub use server::*;
pub use utils::*;
pub use client::*;

#[cfg(target_os = "windows")]
mod security;

#[cfg(all(feature = "windivert", feature = "pcap"))]
compile_error!("Features `windivert` and `pcap` cannot be enabled at the same time.");

#[cfg(all(windows, feature = "windivert"))]
mod windivert;

#[cfg(all(feature = "pcap"))]
mod pcap;

#[cfg(all(windows, feature = "windivert"))]
pub type DefaultPacketCapture = crate::windivert::WindivertPacketCapture;

#[cfg(all(feature = "pcap", not(feature = "windivert")))]
pub type DefaultPacketCapture = crate::pcap::NcapPacketCapture;
