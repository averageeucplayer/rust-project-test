use std::ptr::null_mut;

use anyhow::{bail, Result};
use interprocess::os::windows::security_descriptor::{AsSecurityDescriptorExt, AsSecurityDescriptorMut, SecurityDescriptor};
use windows_sys::Win32::Security::{SetSecurityDescriptorDacl, SECURITY_ATTRIBUTES};

/// Creates a new `SecurityDescriptor` with an unrestricted DACL.
///
/// The returned security descriptor allows full access to everyone
/// because the DACL (Discretionary Access Control List) is unset. This is
/// typically used for objects that should have no access restrictions.
///
/// # Returns
///
/// * `Ok(SecurityDescriptor)` – a security descriptor with an unrestricted DACL.
/// * `Err(_)` – if the descriptor could not be created.
///
/// # Safety
///
/// This descriptor grants **full access to all users**, so it should
/// only be used in controlled scenarios where unrestricted access is safe.
pub fn create_unrestricted_descriptor() -> Result<SecurityDescriptor> {
    let mut descriptor  = SecurityDescriptor::new()?;
    let descriptor_ptr = descriptor.as_sd_mut() as *mut _;

    let result = unsafe {
        SetSecurityDescriptorDacl(descriptor_ptr, 1, null_mut(), 0) == 1
    };

    if !result {
        bail!("Could not set security descriptor");
    }

    let mut attributes = SECURITY_ATTRIBUTES {
        nLength: std::mem::size_of::<SECURITY_ATTRIBUTES>() as u32,
        lpSecurityDescriptor: descriptor_ptr as *mut _,
        bInheritHandle: 0,
    };

    descriptor.write_to_security_attributes(&mut attributes);

    Ok(descriptor)
}