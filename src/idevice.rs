// jkcoxson

use crate::bindings as unsafe_bindings;
use crate::bindings::idevice_info_t;
use crate::callback::IDeviceEventCallback;
use crate::error::{
    self, AfcError, DebugServerError, HeartbeatError, IdeviceError, InstProxyError, LockdowndError,
    MisagentError, MobileImageMounterError, ScreenshotrError,
};
use crate::services::afc::AfcClient;
use crate::services::heartbeat::HeartbeatClient;
use crate::services::lockdownd::LockdowndClient;
use crate::services::misagent::MisagentClient;
use crate::services::mobile_image_mounter::MobileImageMounter;
use core::fmt;
use libc::c_void;
use log::{info, trace, warn};
use std::ffi::CStr;
use std::net::IpAddr;
use std::os::raw::c_char;
use std::{fmt::Debug, fmt::Formatter, ptr::null_mut};

/// Get a list of UDIDs
/// # Arguments
/// *none*
/// # Returns
/// A vector of UDIDs
///
/// ***Verified:*** False
pub fn get_udid_list() -> Result<Vec<String>, IdeviceError> {
    let mut device_list: *mut idevice_info_t = null_mut();
    let mut device_count: i32 = 0;
    info!("Getting all devices from the muxer");
    let result: error::IdeviceError = unsafe {
        unsafe_bindings::idevice_get_device_list_extended(&mut device_list, &mut device_count)
    }
    .into();
    if result != error::IdeviceError::Success {
        return Err(result);
    }

    // Create slice of mutable references to idevice_info_t from device_list and device_count
    info!("Getting device list from slice");
    let device_list_slice =
        unsafe { std::slice::from_raw_parts_mut(device_list, device_count as usize) };

    let mut to_return = Vec::with_capacity(device_list_slice.len());
    for device in device_list_slice {
        to_return.push(unsafe {
            std::ffi::CStr::from_ptr((*(*device)).udid)
                .to_string_lossy()
                .into_owned()
        });
    }
    info!("Returning device list");
    Ok(to_return)
}

/// Gets all devices detected by usbmuxd
/// An abstraction that fetches the device list and connects to it
/// # Arguments
/// *none*
/// # Returns
/// A vector of devices
///
/// ***Verified:*** False
pub fn get_devices() -> Result<Vec<Device>, IdeviceError> {
    let mut device_list: *mut idevice_info_t = null_mut();
    let mut device_count: i32 = 0;
    info!("Getting device list from the muxer");
    let result: error::IdeviceError = unsafe {
        unsafe_bindings::idevice_get_device_list_extended(&mut device_list, &mut device_count)
    }
    .into();

    if result != error::IdeviceError::Success {
        return Err(result);
    }

    info!("Determining devices from slice");
    // Create slice of mutable references to idevice_info_t from device_list and device_count
    let device_list_slice =
        unsafe { std::slice::from_raw_parts_mut(device_list, device_count as usize) };

    let mut to_return = Vec::with_capacity(device_list_slice.len());
    for i in device_list_slice.iter_mut() {
        let network = unsafe { (*(*i)).conn_type != 1 };

        let mut device_info: unsafe_bindings::idevice_t = unsafe { std::mem::zeroed() };
        let device_info_ptr: *mut unsafe_bindings::idevice_t = &mut device_info;
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
            trace!("Failed to create device struct");
            continue;
        }
        let to_push = device_info.into();
        to_return.push(to_push);
    }

    // Drop the memory that the C library allocated
    info!("Freeing device list");
    let device_list_ptr = device_list as *mut *mut std::os::raw::c_char;
    unsafe {
        unsafe_bindings::idevice_device_list_free(device_list_ptr);
    }
    info!("Returning device structs");
    Ok(to_return)
}

/// Fetches a list of devices, but returns one with the given udid
/// # Arguments
/// * `udid` - The udid of the device to return
/// # Returns
/// A device struct
///
/// ***Verified:*** False
pub fn get_device(udid: impl Into<String>) -> Result<Device, IdeviceError> {
    let udid = udid.into();
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

/// Fetches the first device from the muxer
/// # Arguments
/// *none*
/// # Returns
/// A device struct
///
/// ***Verified:*** False
pub fn get_first_device() -> Result<Device, IdeviceError> {
    let devices = match get_devices() {
        Ok(devices) => devices,
        Err(e) => return Err(e),
    };
    if devices.is_empty() {
        return Err(error::IdeviceError::NoDevice);
    }
    Ok(devices[0].clone())
}

/// Toggles usbmuxd's debug mode
/// # Arguments
/// * `debug` - Whether to turn on or off debug mode
/// # Returns
/// ()
///
/// ***Verified:*** False
pub fn set_debug(debug: bool) {
    let debug = match debug {
        true => 1,
        false => 0,
    };
    trace!("Setting debug mode to {}", debug);
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
    /// Creates a new device struct from options
    /// This will sidestep the need for usbmuxd's discovery
    /// # Arguments
    /// * `udid` - The udid of the device to connect to
    /// * `network` - Whether to connect to the device over network or not
    /// * `ip_addr` - The IP address of the device to connect to
    /// * `mux_id` - The ID given to the device by a muxer
    /// # Returns
    /// A device struct
    ///
    /// ***Verified:*** True
    pub fn new(udid: impl Into<String>, ip_addr: Option<IpAddr>, mux_id: u32) -> Device {
        // Convert the udid to a C string
        trace!("Converting udid to C string");
        let mut udid_bytes = udid.into().into_bytes();
        udid_bytes.push(0);
        // Ensure valid C string
        CStr::from_bytes_with_nul(&udid_bytes).unwrap();
        let udid_len = udid_bytes.len();
        let udid_ptr = unsafe { libc::malloc(udid_len) as *mut u8 };

        // SAFETY: udid_cstring has capacity for udid_len bytes, and only need
        // contain valid u8s
        trace!("Creating udid ptr");
        unsafe { udid_ptr.write_bytes(0, udid_len) };

        // SAFETY: udid_cstring points to udid_len bytes, initialized to zero
        let udid_slice = unsafe { std::slice::from_raw_parts_mut(udid_ptr, udid_len) };

        udid_slice.copy_from_slice(&udid_bytes);

        // Convert the ip_addr into bytes
        trace!("Converting ip address into bytes");
        let ip_addr_ptr = if let Some(ip_addr) = ip_addr {
            match ip_addr {
                IpAddr::V4(ip) => {
                    trace!("Encodings ipv4 address");
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
                    trace!("Encodings ipv6 address");
                    let ip_addr = unsafe { libc::malloc(29) as *mut u8 };

                    // SAFETY: ip_addr has capacity for 28 bytes, and only need
                    // contain valid u8s
                    unsafe {
                        ip_addr.write_bytes(0, 29);
                    }

                    // SAFETY: ip_addr points to 29 bytes, initialized to zero
                    let ip_addr_slice = unsafe { std::slice::from_raw_parts_mut(ip_addr, 29) };

                    ip_addr_slice[0..7]
                        .copy_from_slice(&[0x1C, 0x1E, 0x00, 0x00, 0x00, 0x00, 0x00]);
                    ip_addr_slice[8..24].copy_from_slice(&ip.octets());
                    ip_addr_slice[24..29].copy_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00]);

                    ip_addr
                }
            }
        } else {
            trace!("No ip address given");
            std::ptr::null_mut()
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
                conn_type: if ip_addr.is_some() { 2 } else { 1 },
                conn_data: ip_addr_ptr as *mut c_void,
                version: 0,
                device_class: 0,
            });
        }

        i_private_ptr.into()
    }

    /// Get the raw handle to the device
    /// # Returns
    /// The raw handle to the device as a `u32`
    ///
    /// ***Verified:*** False
    pub fn get_handle(&self) -> Result<u32, IdeviceError> {
        let mut handle: u32 = 0;
        let result =
            unsafe { unsafe_bindings::idevice_get_handle(self.pointer, &mut handle) }.into();
        if result != IdeviceError::Success {
            return Err(result);
        }
        Ok(handle)
    }

    /// Get the udid of the device
    /// # Returns
    /// The udid of the device as a `String`
    ///
    /// ***Verified:*** False
    pub fn get_udid(&self) -> String {
        unsafe {
            std::ffi::CStr::from_ptr((*self.pointer).udid)
                .to_string_lossy()
                .to_string()
        }
    }

    /// Return whether the device is connected via network
    /// # Returns
    /// Whether the device is connected via network as a `bool`
    pub fn get_network(&self) -> bool {
        unsafe { (*self.pointer).conn_type != 1 }
    }

    /// Get the ip address of the device if connected over network
    /// # Returns
    /// The ip address of the device
    ///
    /// ***Verified:*** False
    pub fn get_ip_address(&self) -> Option<String> {
        if !self.get_network() {
            warn!("Requested an IP address, but device is not a network device");
            return None;
        }
        let data_pointer = unsafe { (*(self.pointer)).conn_data } as *mut u8;
        // Determine how many bytes long the data is
        let data_length = unsafe { *(data_pointer) };
        info!("Data length is {}", data_length);
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
                warn!("Unknown IP address type");
                None
            }
        }
    }

    /// Get the class of the device
    /// # Returns
    /// The class of the device as a `i32`
    ///
    /// ***Verified:*** False
    pub fn get_device_class(&self) -> DeviceClass {
        unsafe { (*self.pointer).device_class }.into()
    }

    /// Get the version of the device
    /// # Returns
    /// The version of the device as a `i32`
    ///
    /// ***Verified:*** False
    pub fn get_version(&self) -> i32 {
        unsafe { (*self.pointer).version }
    }

    /// Get the mux id of the device
    /// # Returns
    /// The mux id of the device as a `u32`
    ///
    /// ***Verified:*** False
    pub fn get_mux_id(&self) -> u32 {
        unsafe { (*self.pointer).mux_id }
    }

    /// Returns the bytes containing the connection data
    /// This translates to the IP address of the device if connected over network
    /// # Returns
    /// The bytes containing the connection data as a `Vec<u8>`
    ///
    /// ***Verified:*** False
    pub fn get_conn_data(&self) -> Vec<u8> {
        let data_pointer = unsafe { (*(self.pointer)).conn_data } as *mut u8;
        // Determine how many bytes long the data is
        let data_length = unsafe { *(data_pointer) };
        info!("Data length is {}", data_length);
        let data = unsafe { std::slice::from_raw_parts(data_pointer, data_length.into()) };
        data.to_vec()
    }

    /// Starts the lockdown service for the device
    /// This allows things like debuggers to be attached
    /// # Arguments
    /// * `label` - The label to give the underlying service as it starts
    /// # Returns
    /// A lockdown service for the device
    ///
    /// ***Verified:*** False
    pub fn new_lockdownd_client(
        &self,
        label: impl Into<String>,
    ) -> Result<LockdowndClient, LockdowndError> {
        LockdowndClient::new(self, label.into())
    }

    /// Starts the heartbeat service for the device
    /// # Arguments
    /// * `label` - The label to give the underlying service as it starts
    /// # Returns
    /// A heartbeat service for the device
    ///
    /// ***Verified:*** False
    pub fn new_heartbeat_client(
        &self,
        label: impl Into<String>,
    ) -> Result<HeartbeatClient, HeartbeatError> {
        HeartbeatClient::new(self, label.into())
    }

    /// Creates an image mounter for the device
    /// # Arguments
    /// * `label` - The label to give the underlying service as it starts
    /// # Returns
    /// An image mounter service for the device
    ///
    /// ***Verified:*** False
    pub fn new_mobile_image_mounter(
        &self,
        label: impl Into<String>,
    ) -> Result<MobileImageMounter, MobileImageMounterError> {
        crate::services::mobile_image_mounter::MobileImageMounter::start_service(self, label)
    }

    /// Creates an instproxy client for the device
    /// # Arguments
    /// * `label` - The label to give the underlying service as it starts
    /// # Returns
    /// An instproxy client for the device
    ///
    /// ***Verified:*** False
    pub fn new_instproxy_client(
        &self,
        label: impl Into<String>,
    ) -> Result<crate::services::instproxy::InstProxyClient, InstProxyError> {
        crate::services::instproxy::InstProxyClient::new(self, label.into())
    }

    /// Creates a new debug server for the device
    /// # Arguments
    /// * `label` - The label to give the underlying service as it starts
    /// # Returns
    /// A debug server for the device
    ///
    /// ***Verified:*** False
    pub fn new_debug_server(
        &self,
        label: &str,
    ) -> Result<crate::services::debug_server::DebugServer, DebugServerError> {
        crate::services::debug_server::DebugServer::new(self, label)
    }

    /// Creates a new screenshotr service for the device
    /// # Arguments
    /// * `label` - The label to give the underlying service as it starts
    /// # Returns
    /// A screenshot service for the device
    ///
    /// ***Verified:*** False
    pub fn new_screenshot_service(
        &self,
        label: impl Into<String>,
    ) -> Result<crate::services::screenshotr::ScreenshotrClient, ScreenshotrError> {
        crate::services::screenshotr::ScreenshotrClient::start_service(self, label)
    }

    /// Creates a new AFC client for the device
    /// # Arguments
    /// * `label` - The label to give the underlying service as it starts
    /// # Returns
    /// An AFC client for the device
    ///
    /// ***Verified:*** False
    pub fn new_afc_client(&self, label: impl Into<String>) -> Result<AfcClient, AfcError> {
        AfcClient::start_service(self, label)
    }

    /// Creates a new misagent client for the device
    /// # Arguments
    /// * `label` - The label to give the underlying service as it starts
    /// # Returns
    /// An misagent client for the device
    ///
    /// ***Verified:*** False
    pub fn new_misagent_client(
        &self,
        label: impl Into<String>,
    ) -> Result<MisagentClient, MisagentError> {
        MisagentClient::start_service(self, label)
    }
}

impl Clone for Device {
    fn clone(&self) -> Self {
        let ip = self.get_ip_address().map(|ip| ip.parse().unwrap());
        Device::new(self.get_udid(), ip, self.get_mux_id())
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceClass {
    IPhone,
    IPad,
    IPod,
    AppleTV,
    Watch,
    Unknown,
}

impl From<i32> for DeviceClass {
    fn from(value: i32) -> Self {
        match value {
            1 => DeviceClass::IPhone,
            2 => DeviceClass::IPad,
            3 => DeviceClass::IPod,
            4 => DeviceClass::AppleTV,
            5 => DeviceClass::Watch,
            _ => DeviceClass::Unknown,
        }
    }
}

impl From<DeviceClass> for i32 {
    fn from(value: DeviceClass) -> Self {
        match value {
            DeviceClass::IPhone => 1,
            DeviceClass::IPad => 2,
            DeviceClass::IPod => 3,
            DeviceClass::AppleTV => 4,
            DeviceClass::Watch => 5,
            DeviceClass::Unknown => 255,
        }
    }
}

impl From<DeviceClass> for String {
    fn from(value: DeviceClass) -> Self {
        match value {
            DeviceClass::IPhone => "iPhone".to_string(),
            DeviceClass::IPad => "iPad".to_string(),
            DeviceClass::IPod => "iPod".to_string(),
            DeviceClass::AppleTV => "AppleTV".to_string(),
            DeviceClass::Watch => "Watch".to_string(),
            DeviceClass::Unknown => "Unknown".to_string(),
        }
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
        Device { pointer: device }
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
            self.get_device_class() as i32,
            self.get_version()
        )
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        info!("Dropping device {}", self.get_udid());
        unsafe {
            unsafe_bindings::idevice_free(self.pointer);
        }
    }
}
