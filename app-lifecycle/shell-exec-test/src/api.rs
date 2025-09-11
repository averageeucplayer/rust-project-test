
use sysinfo::System;
use winapi::um::shellapi::ShellExecuteExW;
use winapi::um::shellapi::ShellExecuteW;
use winapi::um::shellapi::SEE_MASK_NOCLOSEPROCESS;
use winapi::um::shellapi::SHELLEXECUTEINFOW;
use winapi::um::winuser::SW_SHOWNORMAL;
use std::ffi::OsStr;
use std::path::PathBuf;
use std::ptr;
use std::fs;
use winapi::um::winuser::SW_SHOW;
use log::*;
use anyhow::*;
use std::os::windows::ffi::OsStrExt;

pub fn to_wstring(os_str: &OsStr) -> Vec<u16> {
    os_str.encode_wide().chain(Some(0)).collect()
}

pub fn process_exists_by_name(name: &str) -> bool {
    let mut sys = System::new_all();
    sys.refresh_processes();

    sys.processes()
        .values()
        .any(|process| process.name().eq_ignore_ascii_case(name))
}

pub fn launch_process(capture_path: PathBuf, filter: &str) -> Result<()> {
    let exe_w = to_wstring(capture_path.as_os_str());
    let args = to_wstring(&OsStr::new(filter));
    let run_as = to_wstring(OsStr::new("runas")).as_ptr();

    let mut exec_info: SHELLEXECUTEINFOW = unsafe { std::mem::zeroed() };
    exec_info.cbSize = std::mem::size_of::<SHELLEXECUTEINFOW>() as u32;
    exec_info.fMask = SEE_MASK_NOCLOSEPROCESS;
    exec_info.hwnd = std::ptr::null_mut();
    exec_info.lpVerb = run_as;
    exec_info.lpFile = exe_w.as_ptr();
    exec_info.lpParameters = args.as_ptr();
    exec_info.lpDirectory = ptr::null();
    exec_info.nShow = SW_SHOW as i32;

    let success = unsafe { ShellExecuteExW(&mut exec_info as *mut _) } == 1;

    if !success {
        return Err(anyhow!("Could not launch capture server"));
    }

    Ok(())
}