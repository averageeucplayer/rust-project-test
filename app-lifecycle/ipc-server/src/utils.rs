use blake3::hash;
use log::*;

use anyhow::{anyhow, Result};
use interprocess::local_socket::{GenericFilePath, GenericNamespaced, Name, NameType, ToFsName, ToNsName
};
use std::{ffi::OsStr, os::windows::ffi::OsStrExt, path::Path, ptr};

use winapi::um::{shellapi::{ShellExecuteExW, SEE_MASK_NOCLOSEPROCESS, SHELLEXECUTEINFOW}, winuser::SW_SHOW};

fn to_wstring(os_str: &OsStr) -> Vec<u16> {
    os_str.encode_wide().chain(Some(0)).collect()
}

pub fn get_socket_name<'a>(filter: &str) -> Result<Name<'a>> {

    let version = env!("CARGO_PKG_VERSION").replace(".", "_");
    let filter_hash = &hash(filter.as_bytes()).to_hex()[..8];
    
    let full_name = if cfg!(windows) {
        format!("windows_capture_{}_{}.sock", version, filter_hash)
    }
    else {
        format!("/tmp/linux_capture_{}_{}.sock", version, filter_hash)
    };

    let name: Name<'_> = if GenericNamespaced::is_supported() {
        full_name.to_ns_name::<GenericNamespaced>()?
    } else {
        full_name.to_fs_name::<GenericFilePath>()?
    };
  
    Ok(name)
}

pub fn spawn_process(exe_path: &Path, filter: String) -> Result<()> {
    
    let exe_w = to_wstring(exe_path.as_os_str());
    let args = to_wstring(&OsStr::new(&filter.to_string()));
    let run_as = to_wstring(OsStr::new("runas"));

    let mut exec_info: SHELLEXECUTEINFOW = unsafe { std::mem::zeroed() };
    exec_info.cbSize = std::mem::size_of::<SHELLEXECUTEINFOW>() as u32;
    exec_info.fMask = SEE_MASK_NOCLOSEPROCESS;
    exec_info.hwnd = std::ptr::null_mut();
    exec_info.lpVerb = run_as.as_ptr();
    exec_info.lpFile = exe_w.as_ptr();
    exec_info.lpParameters = args.as_ptr();
    exec_info.lpDirectory = ptr::null();
    exec_info.nShow = SW_SHOW as i32;

    // TO-DO Investigate weird quirks about wrong file class sometimes???
    let success = unsafe { ShellExecuteExW(&mut exec_info as *mut _) } == 1;

    if !success {
        return Err(anyhow!("Could not launch capture server"));
    }

    Ok(())
}

#[cfg(all(windows, feature = "windivert"))]
mod windivert_utils {
    use std::ptr;

    use log::info;
    use winapi::{shared::{ntdef::ULONG, winerror::{ERROR_BUFFER_OVERFLOW, NO_ERROR}, ws2def::AF_UNSPEC}, um::{iphlpapi::GetAdaptersAddresses, iptypes::{GAA_FLAG_INCLUDE_PREFIX, IP_ADAPTER_ADDRESSES_LH}}};
    use windivert_sys::ChecksumFlags;
    use windivert::{error::WinDivertError, layer::NetworkLayer, packet::WinDivertPacket, prelude::WinDivertFlags, WinDivert};


    pub fn interface_name_from_index(index: u32) -> Option<String> {
        let mut size: ULONG = 0;
        
        unsafe {
            let mut size: ULONG = 0;

            let ret = GetAdaptersAddresses(
                AF_UNSPEC as u32,
                GAA_FLAG_INCLUDE_PREFIX,
                ptr::null_mut(),
                ptr::null_mut(),
                &mut size,
            );

            if ret != ERROR_BUFFER_OVERFLOW {
                return None;
            }

            let mut buf = vec![0u8; size as usize];
            let adapter = buf.as_mut_ptr() as *mut IP_ADAPTER_ADDRESSES_LH;

            if GetAdaptersAddresses(
                AF_UNSPEC as u32,
                GAA_FLAG_INCLUDE_PREFIX,
                ptr::null_mut(),
                adapter,
                &mut size,
            ) != NO_ERROR
            {
                return None;
            }

            let mut current = adapter;
            while !current.is_null() {
                let interface = *current;

                if interface.Ipv6IfIndex == index {
                    if !(*current).FriendlyName.is_null() {
                        let wide = std::slice::from_raw_parts((*current).FriendlyName, 256);
                        let name_ptr = &wide[..wide.iter().position(|&c| c == 0).unwrap_or(0)];
                        let name = String::from_utf16_lossy(name_ptr);
                        
                        info!("{:?} {:?}", name, interface.Ipv6IfIndex);

                        return Some(name);
                    }
                }

                current = (*current).Next;
            }
        }

        None
    }

    pub fn create_custom_packet(
        src_ip: [u8; 4],
        dst_ip: [u8; 4],
        src_port: u16,
        dst_port: u16,
        payload: &[u8],
    ) -> Result<WinDivertPacket<'static, NetworkLayer>, WinDivertError> {
        let total_length = 20 + 20 + payload.len();
        let mut packet_data: Vec<u8> = Vec::with_capacity(total_length);

        packet_data.extend_from_slice(&[
            0x45,                      // Version + IHL
            0x00,                      // Type of Service
            (total_length >> 8) as u8, // Total length high byte
            (total_length & 0xFF) as u8, // Total length low byte
            0x12, 0x34,                // Identification
            0x40, 0x00,                // Flags + Fragment Offset
            0x40,                      // TTL
            0x06,                      // Protocol = TCP
            0x00, 0x00,                // Header checksum (will be recalculated)
            src_ip[0], src_ip[1], src_ip[2], src_ip[3], // Source IP
            dst_ip[0], dst_ip[1], dst_ip[2], dst_ip[3], // Destination IP
        ]);

        // TCP header
        packet_data.extend_from_slice(&[
            (src_port >> 8) as u8, (src_port & 0xFF) as u8, // Source port
            (dst_port >> 8) as u8, (dst_port & 0xFF) as u8, // Destination port
            0x12, 0x34, 0x56, 0x78, // Sequence number
            0x00, 0x00, 0x00, 0x00, // Acknowledgment number
            0x50, 0x02, 0x20, 0x00, // Data offset + flags + window
            0x00, 0x00, 0x00, 0x00, // Checksum + urgent pointer
        ]);
        packet_data.extend_from_slice(payload);

        let mut packet = unsafe { WinDivertPacket::<NetworkLayer>::new(packet_data) };

        packet.recalculate_checksums(ChecksumFlags::new())?;

        Ok(packet.into_owned())
    }
}

#[cfg(all(windows, feature = "windivert"))]
pub use windivert_utils::*;