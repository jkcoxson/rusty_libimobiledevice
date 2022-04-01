// jkcoxson

use core::fmt;
use std::ffi::CStr;
use std::net::IpAddr;
use std::os::raw::c_char;
use std::{fmt::Debug, fmt::Formatter, ptr::null_mut};
use libc::c_void;
use crate::bindings as unsafe_bindings;
use crate::bindings::idevice_info_t;
use crate::callback::IDeviceEventCallback;
use crate::debug;
use crate::error::{
    self, DebugServerError, HeartbeatError, IdeviceError, InstProxyError, LockdowndError,
    MobileImageMounterError,
};
use crate::heartbeat::HeartbeatClient;
use crate::lockdownd::{LockdowndClient, LockdowndService, MobileImageMounter};

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

pub fn event_subscribe(_cb: IDeviceEventCallback) -> Result<(), IdeviceError> {
    todo!()
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
        ip_addr: Option<IpAddr>,
        mux_id: u32,
    ) -> Result<Device, ()> {
        if network && ip_addr.is_none() {
            return Err(());
        }

        // Convert the udid to a C string
        let mut udid_bytes = udid.into_bytes();
        udid_bytes.push(0);
        // Ensure valid C string
        CStr::from_bytes_with_nul(&udid_bytes).unwrap();
        let udid_len = udid_bytes.len();
        let udid_ptr = unsafe { libc::malloc(udid_len) as *mut u8 };

        // SAFETY: udid_cstring has capacity for udid_len bytes, and only need
        // contain valid u8s
        unsafe { udid_ptr.write_bytes(0, udid_len) };

        // SAFETY: udid_cstring points to udid_len bytes, initialized to zero
        let udid_slice = unsafe { std::slice::from_raw_parts_mut(udid_ptr, udid_len) };

        udid_slice.copy_from_slice(&udid_bytes);

        // Convert the ip_addr into bytes
        let ip_addr_ptr = match network {
            true => match ip_addr.unwrap() {
                IpAddr::V4(ip) => {
                    let ip_addr = unsafe { libc::malloc(16) as *mut u8 };

                    // SAFETY: ip_addr has capacity for 16 bytes, and only need
                    // contain valid u8s
                    unsafe {
                        ip_addr.write_bytes(0, 16);
                    }

                    // SAFETY: ip_addr points to 16 bytes, initialized to zero
                    let ip_addr_slice = unsafe { std::slice::from_raw_parts_mut(ip_addr, 16) };

                    ip_addr_slice[0..4].copy_from_slice(&[0x10, 0x02, 0x00, 0x00]);
                    ip_addr_slice[4..8].copy_from_slice(&ip.octets());
                    ip_addr_slice[8..16]
                        .copy_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

                    ip_addr
                }
                IpAddr::V6(ip) => {
                    let ip_addr = unsafe { libc::malloc(29) as *mut u8 };

                    // SAFETY: ip_addr has capacity for 28 bytes, and only need
                    // contain valid u8s
                    unsafe {
                        ip_addr.write_bytes(0, 28);
                    }

                    // SAFETY: ip_addr points to 29 bytes, initialized to zero
                    let ip_addr_slice = unsafe { std::slice::from_raw_parts_mut(ip_addr, 29) };

                    ip_addr_slice[0..8]
                        .copy_from_slice(&[0x1C, 0x1E, 0x00, 0x00, 0x00, 0x00, 0x00]);
                    ip_addr_slice[8..24].copy_from_slice(&ip.octets());
                    ip_addr_slice[24..29].copy_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00]);

                    ip_addr
                }
            },
            false => 0 as *mut u8,
        };

        let i_private_ptr = unsafe {
            libc::malloc(std::mem::size_of::<unsafe_bindings::idevice_private>())
                as *mut unsafe_bindings::idevice_private
        };

        // SAFETY: i_private_ptr has enough capacity for an idevice_private struct
        unsafe {
            i_private_ptr.write(unsafe_bindings::idevice_private {
                udid: udid_ptr as *mut c_char,
                mux_id,
                conn_type: match network {
                    true => 2,
                    false => 1,
                },
                conn_data: ip_addr_ptr as *mut c_void,
                version: 0,
                device_class: 0,
            });
        }

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
            debug!("Requested an IP address, but device is not a network device");
            return None;
        }
        let data_pointer = unsafe { (*(self.pointer)).conn_data } as *mut u8;
        // Determine how many bytes long the data is
        let data_length = unsafe { *(data_pointer) };
        debug!("Data length is {}", data_length);
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
            _ => {
                debug!("Unknown IP address type");
                None
            }
        }
    }

    pub fn get_device_class(&self) -> i32 {
        unsafe { (*self.pointer).device_class }
    }

    pub fn get_version(&self) -> i32 {
        unsafe { (*self.pointer).version }
    }

    pub fn get_conn_data(&self) -> Vec<u8> {
        let data_pointer = unsafe { (*(self.pointer)).conn_data } as *mut u8;
        // Determine how many bytes long the data is
        let data_length = unsafe { *(data_pointer) };
        debug!("Data length is {}", data_length);
        let data = unsafe { std::slice::from_raw_parts(data_pointer, data_length.into()) };
        data.to_vec()
    }

    /// Starts the lockdown service for the device
    /// This allows things like debuggers to be attached
    pub fn new_lockdownd_client(&self, label: String) -> Result<LockdowndClient, LockdowndError> {
        Ok(LockdowndClient::new(self, label)?)
    }

    pub fn new_heartbeat_client(&self, label: String) -> Result<HeartbeatClient, HeartbeatError> {
        Ok(HeartbeatClient::new(self, label)?)
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

pub struct IDeviceEvent {
    pub(crate) _pointer: unsafe_bindings::idevice_event_t,
}

impl From<unsafe_bindings::idevice_event_t> for IDeviceEvent {
    fn from(_pointer: unsafe_bindings::idevice_event_t) -> Self {
        IDeviceEvent { _pointer }
    }
}

pub enum EventType {
    Add,
    Remove,
    Pair,
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
            "Device {{ udid: {}, network: {}, ip_address: {:?}, device_class: {}, version: {} }}",
            self.get_udid(),
            self.get_network(),
            self.get_ip_address(),
            self.get_device_class(),
            self.get_version()
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