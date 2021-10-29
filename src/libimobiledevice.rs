#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)] // These are because I want function names to be similar to their C counterparts.
#![allow(non_snake_case)]
#![allow(deref_nullptr)]
#![allow(unaligned_references)]

use std::ptr::null;

pub use crate::bindings as unsafe_bindings;
use crate::bindings::idevice_info_t;

// The end goal here is to create a safe library that can wrap the unsafe C code

/// Returns a vector of devices found by usbmuxd.
pub fn idevice_get_device_list_extended() -> Option<(Vec<idevice_info>, i32)> {
    // I have no idea how this whole function works, sb wrote it so complain to him.

    // get list of idevice_info_t
    let mut device_list: *mut idevice_info_t = std::ptr::null_mut();
    let mut device_count: i32 = 0;
    let result = unsafe {
        unsafe_bindings::idevice_get_device_list_extended(&mut device_list, &mut device_count)
    };

    // idevice_get_device_list_extended returns an error status code, return None if it's not 0s
    if result < 0 {
        return None;
    }

    // Create slice of mutable references to idevice_info_t from device_list and device_count
    let device_list_slice =
        unsafe { std::slice::from_raw_parts_mut(device_list, device_count as usize) };

    // Package up the found devices into a vector of idevice_info so Rust can manage the memory
    let mut to_return: Vec<idevice_info> = vec![];
    for i in device_list_slice.iter_mut() {
        unsafe {
            to_return.push(idevice_info::new(
                // What the heck C
                std::ffi::CStr::from_ptr((*(*i)).udid)
                    .to_string_lossy()
                    .to_string(),
                (*(*i)).conn_type,
                (*(*i)).conn_data,
            ));
        }
    }

    // Drop the memory that the C library allocated
    let device_list_ptr = device_list as *mut *mut std::os::raw::c_char;
    unsafe {
        unsafe_bindings::idevice_device_list_free(device_list_ptr);
    }

    // All other variables are dropped at return

    Some((to_return, device_count))
}

pub fn idevice_new_with_options(udid: String, network: bool) -> Option<idevice_t> {
    let mut device_info: unsafe_bindings::idevice_t = unsafe { std::mem::zeroed() };
    let device_info_ptr: *mut unsafe_bindings::idevice_t = &mut device_info;

    let udid_c_str = std::ffi::CString::new(udid.clone()).unwrap();
    let network: u32 = if network {
        unsafe_bindings::idevice_options_IDEVICE_LOOKUP_NETWORK
    } else {
        unsafe_bindings::idevice_options_IDEVICE_LOOKUP_USBMUX
    };
    let result = unsafe {
        unsafe_bindings::idevice_new_with_options(device_info_ptr, udid_c_str.as_ptr(), network)
    };
    if result < 0 {
        return None;
    }
    Some(unsafe {
        idevice_t::new(
            device_info
        )
    })
}

pub fn lockdownd_client_new_with_handshake(
    device: idevice_t,
    label: String,
) -> Option<lockdownd_client_t> {
    let mut client: unsafe_bindings::lockdownd_client_t = unsafe { std::mem::zeroed() };
    let client_ptr: *mut unsafe_bindings::lockdownd_client_t = &mut client;

    let label_c_str = std::ffi::CString::new(label).unwrap();

    

    let result = unsafe {
        unsafe_bindings::lockdownd_client_new_with_handshake(
            device.device,
            client_ptr,
            label_c_str.as_ptr(),
        )
    };
    if result < 0 {
        return None;
    }

    Some(lockdownd_client_t::new(client))
}

pub fn lockdownd_get_value(client: lockdownd_client) -> Option<plist> {
    let mut plist_ptr = std::ptr::null_mut();

    let parent = unsafe_bindings::

    let lock_cli = unsafe_bindings::lockdownd_client_private {
        parent: client.parent,
    }


    let result =
        unsafe { unsafe_bindings::lockdownd_get_value(lock_cli, null(), null(), &mut plist_ptr) };
    if result < 0 {
        return None;
    }
    todo!()
    //Some(unsafe { plist::new(plist_ptr) })
}

pub struct idevice_info {
    pub udid: String,
    pub conn_type: u32,
    pub conn_data: *mut std::os::raw::c_void, // What the heck is this?
}

impl idevice_info {
    fn new(udid: String, conn_type: u32, conn_data: *mut std::os::raw::c_void) -> Self {
        idevice_info {
            udid,
            conn_type,
            conn_data,
        }
    }
}

pub struct idevice_t {
    pub device: *mut unsafe_bindings::idevice_private
}

impl idevice_t {
    pub fn new(device: *mut unsafe_bindings::idevice_private) -> Self {
        idevice_t {
            device
        }
    }
}

pub struct lockdownd_client_t {
    client: unsafe_bindings::lockdownd_client_t,
}

impl lockdownd_client_t {
    pub fn new( client: unsafe_bindings::lockdownd_client_t) -> Self {
        lockdownd_client_t {
            client
        }
    }
}

pub struct idevice_connection_t {
    connection: unsafe_bindings::idevice_connection_t,
}

impl idevice_connection_t {
    pub fn new(
        connection: unsafe_bindings::idevice_connection_t,
    ) -> Self {
        idevice_connection_t {
            connection
        }
    }
}
