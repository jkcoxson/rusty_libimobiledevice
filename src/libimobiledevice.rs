#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)] // These are because I want function names to be similar to their C counterparts.
#![allow(non_snake_case)]
#![allow(deref_nullptr)]
#![allow(unaligned_references)]

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
    Some(idevice_t::new(device_info))
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

pub fn lockdownd_get_value(client: lockdownd_client_t) -> Option<String> {
    let mut plist: unsafe_bindings::plist_t = unsafe { std::mem::zeroed() };
    let plist_ptr: *mut unsafe_bindings::plist_t = &mut plist;
    let domain_ptr: *mut std::os::raw::c_char = std::ptr::null_mut();
    let key_ptr: *mut std::os::raw::c_char = std::ptr::null_mut();
    println!("got here 1");
    // Create domain variable
    let result = unsafe {
        unsafe_bindings::lockdownd_get_value(client.client, domain_ptr, key_ptr, plist_ptr)
    };
    println!("got here 2");
    if result < 0 {
        return None;
    }

    // Variables to be filled by C. Honestly, who thought this was the correct way to do this?
    let mut plist_xml: *mut std::os::raw::c_char = std::ptr::null_mut();
    let plist_xml_ptr: *mut *mut std::os::raw::c_char = &mut plist_xml;
    let mut plist_xml_len: u32 = 0;
    let plist_xml_len_ptr: *mut u32 = &mut plist_xml_len;

    unsafe {
        unsafe_bindings::plist_to_xml(*plist_ptr, plist_xml_ptr, plist_xml_len_ptr);
    }
    println!("got here 3");
    // Convert plist_xml to String
    let plist_xml_str = unsafe {
        std::ffi::CStr::from_ptr(plist_xml)
            .to_string_lossy()
            .to_string()
    };
    Some(plist_xml_str)
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
    pub device: *mut unsafe_bindings::idevice_private,
}

impl idevice_t {
    pub fn new(device: *mut unsafe_bindings::idevice_private) -> Self {
        idevice_t { device }
    }
    pub fn get_udid(&self) -> String {
        // Convert self.device.udid to String
        unsafe {
            std::ffi::CStr::from_ptr((*self.device).udid)
                .to_string_lossy()
                .to_string()
        }
    }
}

pub struct lockdownd_client_t {
    client: unsafe_bindings::lockdownd_client_t,
}

impl lockdownd_client_t {
    pub fn new(client: unsafe_bindings::lockdownd_client_t) -> Self {
        lockdownd_client_t { client }
    }
}
