// jkcoxson

use core::fmt;
use std::ffi::CString;
use std::net::{IpAddr, SocketAddr};
use std::{fmt::Debug, fmt::Formatter, ptr::null_mut};

use libc::c_void;

use crate::bindings as unsafe_bindings;
use crate::bindings::idevice_info_t;
use crate::debug;
use crate::error::{
    self, DebugServerError, IdeviceError, InstProxyError, LockdowndError, MobileImageMounterError,
};
use crate::lockdownd::{LockdowndClient, LockdowndService, MobileImageMounter};

// The end goal here is to create a safe library that can wrap the unsafe C code

/////////////////////
// Smexy Functions //
/////////////////////

/// Get a list of UDIDs
pub fn get_udid_list() -> Result<Vec<String>, IdeviceError> {
    let mut device_list: *mut idevice_info_t = null_mut();
    let mut device_count: i32 = 0;
    debug!("Getting all devices from the muxer");
    let result: error::IdeviceError = unsafe {
        unsafe_bindings::idevice_get_device_list_extended(&mut device_list, &mut device_count)
    }
    .into();
    if result != error::IdeviceError::Success {
        return Err(result);
    }

    // Create slice of mutable references to idevice_info_t from device_list and device_count
    debug!("Getting device list from slice");
    let device_list_slice =
        unsafe { std::slice::from_raw_parts_mut(device_list, device_count as usize) };

    let mut to_return = vec![];
    for device in device_list_slice {
        to_return.push(unsafe {
            std::ffi::CStr::from_ptr((*(*device)).udid)
                .to_string_lossy()
                .into_owned()
        });
    }
    debug!("Returning device list");
    Ok(to_return)
}

/// Gets all devices detected by usbmuxd
/// An abstraction that fetches the device list and connects to it
pub fn get_devices() -> Result<Vec<Device>, IdeviceError> {
    let mut device_list: *mut idevice_info_t = null_mut();
    let mut device_count: i32 = 0;
    debug!("Getting device list from the muxer");
    let result: error::IdeviceError = unsafe {
        unsafe_bindings::idevice_get_device_list_extended(&mut device_list, &mut device_count)
    }
    .into();

    if result != error::IdeviceError::Success {
        return Err(result);
    }

    debug!("Determining devices from slice");
    // Create slice of mutable references to idevice_info_t from device_list and device_count
    let device_list_slice =
        unsafe { std::slice::from_raw_parts_mut(device_list, device_count as usize) };

    let mut to_return = vec![];
    for i in device_list_slice.iter_mut() {
        // Print pointer address
        let udid = unsafe {
            std::ffi::CStr::from_ptr((*(*i)).udid)
                .to_string_lossy()
                .to_string()
        };
        let network = unsafe {
            if (*(*i)).conn_type == 1 {
                false
            } else {
                true
            }
        };

        let mut device_info: unsafe_bindings::idevice_t = unsafe { std::mem::zeroed() };
        let device_info_ptr: *mut unsafe_bindings::idevice_t = &mut device_info;
        debug!("Creating device struct connection to {}", udid);
        let result = unsafe {
            unsafe_bindings::idevice_new_with_options(
                device_info_ptr,
                (*(*i)).udid,
                if network {
                    unsafe_bindings::idevice_options_IDEVICE_LOOKUP_NETWORK
                } else {
                    unsafe_bindings::idevice_options_IDEVICE_LOOKUP_USBMUX
                },
            )
        };
        if result != 0 {
            debug!("Failed to create device struct to {}", udid);
            continue;
        }
        let to_push = device_info.into();
        to_return.push(to_push);
    }

    // Drop the memory that the C library allocated
    debug!("Freeing device list");
    let device_list_ptr = device_list as *mut *mut std::os::raw::c_char;
    unsafe {
        unsafe_bindings::idevice_device_list_free(device_list_ptr);
    }
    debug!("Returning device structs");
    Ok(to_return)
}

pub fn get_device(udid: String) -> Result<Device, IdeviceError> {
    let devices = match get_devices() {
        Ok(devices) => devices,
        Err(e) => return Err(e),
    };
    for device in devices {
        if device.get_udid() == udid {
            return Ok(device);
        }
    }
    Err(error::IdeviceError::NoDevice)
}

pub fn set_debug(debug: bool) {
    let debug = match debug {
        true => 1,
        false => 0,
    };
    debug!("Setting debug mode to {}", debug);
    unsafe { unsafe_bindings::idevice_set_debug_level(debug) }
}

// Structs
pub struct Device {
    pub(crate) pointer: unsafe_bindings::idevice_t,
}

unsafe impl Send for Device {}
unsafe impl Sync for Device {}

impl Device {
    pub fn new(
        udid: String,
        network: bool,
        ip_addr: Option<SocketAddr>,
        mux_id: u32,
    ) -> Result<Device, ()> {
        // Convert the udid to a CString
        let udid_cstring = CString::new(udid).unwrap();
        if network && ip_addr.is_none() {
            return Err(());
        }
        // Convert the ip_addr into bytes
        let ip_addr: Vec<u8> = match network {
            true => match ip_addr.unwrap().ip() {
                IpAddr::V4(ip) => {
                    let mut to_return = vec![0x10, 0x02, 0x00, 0x00];
                    to_return.extend(ip.octets().iter().cloned());
                    to_return.extend(vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
                    to_return
                }
                IpAddr::V6(ip) => {
                    let mut to_return = vec![0x1C, 0x1E, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
                    to_return.extend(ip.octets().iter().cloned());
                    to_return.extend(vec![0x00, 0x00, 0x00, 0x00, 0x00]);
                    to_return
                }
            },
            false => vec![],
        };
        // Create a pointer to the ip_addr bytes
        let ip_addr_ptr = ip_addr.as_ptr() as *const u8;
        std::mem::forget(ip_addr);
        // Create udid pointer
        let udid_ptr = udid_cstring.as_ptr();
        std::mem::forget(udid_cstring);
        // Create the device struct
        let mut i_private = unsafe_bindings::idevice_private {
            udid: udid_ptr as *mut i8,
            mux_id,
            conn_type: match network {
                true => 1,
                false => 0,
            },
            conn_data: ip_addr_ptr as *mut c_void,
            version: 0,
            device_class: 0,
        };
        // Create pointer to the device struct
        let i_private_ptr = &mut i_private as *mut unsafe_bindings::idevice_private;
        std::mem::forget(i_private);
        Ok(i_private_ptr.into())
    }

    pub fn get_handle(&self) -> Result<u32, IdeviceError> {
        let mut handle: u32 = 0;
        let result =
            unsafe { unsafe_bindings::idevice_get_handle(self.pointer, &mut handle) }.into();
        if result != IdeviceError::Success {
            return Err(result);
        }
        Ok(handle)
    }

    pub fn get_udid(&self) -> String {
        unsafe {
            std::ffi::CStr::from_ptr((*self.pointer).udid)
                .to_string_lossy()
                .to_string()
        }
    }

    pub fn get_network(&self) -> bool {
        unsafe {
            if (*self.pointer).conn_type == 1 {
                false
            } else {
                true
            }
        }
    }

    pub fn get_ip_address(&self) -> Option<String> {
        if !self.get_network() {
            return None;
        }
        let data_pointer = unsafe { (*(self.pointer)).conn_data } as *mut u8;
        // Determine how many bytes long the data is
        let data_length = unsafe { *(data_pointer) };
        let data = unsafe { std::slice::from_raw_parts(data_pointer, data_length.into()) };
        // Determine if the data is IPv4 or IPv6
        match data[1] {
            0x02 => {
                // IPv4
                let mut ip_addr = [0u8; 4];
                ip_addr.copy_from_slice(&data[4..8]);
                let ip_addr = std::net::Ipv4Addr::from(ip_addr);
                Some(ip_addr.to_string())
            }
            0x1E => {
                // IPv6
                let mut ip_addr = [0u8; 16];
                ip_addr.copy_from_slice(&data[7..23]);
                let ip_addr = std::net::Ipv6Addr::from(ip_addr);
                Some(ip_addr.to_string())
            }
            _ => None,
        }
    }

    /// Starts the lockdown service for the device
    /// This allows things like debuggers to be attached
    pub fn new_lockdownd_client(&self, label: String) -> Result<LockdowndClient, LockdowndError> {
        Ok(LockdowndClient::new(self, label)?)
    }

    /// Creates an image mounter for the device
    pub fn new_mobile_image_mounter(
        &self,
        service: &LockdowndService,
    ) -> Result<MobileImageMounter, MobileImageMounterError> {
        let mut mobile_image_mounter: unsafe_bindings::mobile_image_mounter_client_t =
            unsafe { std::mem::zeroed() };

        debug!("Creating mobile image mounter for {}", self.get_udid());
        let error = unsafe {
            unsafe_bindings::mobile_image_mounter_new(
                self.pointer,
                service.pointer,
                &mut mobile_image_mounter,
            )
        }
        .into();

        if error != MobileImageMounterError::Success {
            return Err(error);
        }

        let mobile_image_mounter = MobileImageMounter {
            pointer: mobile_image_mounter,
            phantom: std::marker::PhantomData,
        };

        Ok(mobile_image_mounter)
    }

    /// Creates an instproxy client for the device
    pub fn new_instproxy_client(
        &self,
        label: String,
    ) -> Result<crate::instproxy::InstProxyClient, InstProxyError> {
        crate::instproxy::InstProxyClient::new(self, label)
    }

    /// Creates a new debug server for the device
    pub fn new_debug_server(
        &self,
        label: &str,
    ) -> Result<crate::debug_server::DebugServer, DebugServerError> {
        crate::debug_server::DebugServer::new(self, label)
    }
}

impl From<unsafe_bindings::idevice_t> for Device {
    fn from(device: unsafe_bindings::idevice_t) -> Device {
        return Device { pointer: device };
    }
}

impl Debug for Device {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Device {{ udid: {}, network: {} }}",
            self.get_udid(),
            self.get_network()
        )
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        debug!("Dropping device {}", self.get_udid());
        unsafe {
            unsafe_bindings::idevice_free(self.pointer);
        }
    }
}
