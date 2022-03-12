// jkcoxson

use core::fmt;
use std::{fmt::Debug, fmt::Formatter, ptr::null_mut};

pub use crate::bindings as unsafe_bindings;
use crate::bindings::idevice_info_t;
use crate::error::{
    self, DebugServerError, IdeviceError, InstProxyError, LockdowndError, MobileImageMounterError,
};
use crate::lockdownd::{LockdowndClient, LockdowndService, MobileImageMounter};

// The end goal here is to create a safe library that can wrap the unsafe C code

/////////////////////
// Smexy Functions //
/////////////////////

/// Gets all devices detected by usbmuxd
pub fn get_devices() -> Result<Vec<Device>, IdeviceError> {
    let mut device_list: *mut idevice_info_t = null_mut();
    let mut device_count: i32 = 0;
    let result: error::IdeviceError = unsafe {
        unsafe_bindings::idevice_get_device_list_extended(&mut device_list, &mut device_count)
    }
    .into();

    if result != error::IdeviceError::Success {
        return Err(result);
    }

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
            continue;
        }
        let to_push = Device::new(udid, network, unsafe { (*(*i)).conn_data }, device_info);
        to_return.push(to_push);
    }

    // Drop the memory that the C library allocated
    let device_list_ptr = device_list as *mut *mut std::os::raw::c_char;
    unsafe {
        unsafe_bindings::idevice_device_list_free(device_list_ptr);
    }

    Ok(to_return)
}

pub fn get_device(udid: String) -> Result<Device, IdeviceError> {
    let devices = match get_devices() {
        Ok(devices) => devices,
        Err(e) => return Err(e),
    };
    for device in devices {
        if device.udid == udid {
            return Ok(device);
        }
    }
    Err(error::IdeviceError::NoDevice)
}

// Structs
pub struct Device {
    // Front facing properties
    pub udid: String,
    pub network: bool,
    // Raw properties
    conn_data: *mut std::os::raw::c_void, // tbh what the heck is this
    pub(crate) pointer: unsafe_bindings::idevice_t,
}

impl Device {
    pub fn new(
        udid: String,
        network: bool,
        conn_data: *mut std::os::raw::c_void,
        device: unsafe_bindings::idevice_t,
    ) -> Device {
        return Device {
            udid,
            network,
            conn_data,
            pointer: device,
        };
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

impl Debug for Device {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Device {{ udid: {}, network: {} }}",
            self.udid, self.network
        )
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe {
            unsafe_bindings::idevice_free(self.pointer);
        }
    }
}
