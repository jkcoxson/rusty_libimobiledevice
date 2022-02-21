// jkcoxson

use core::fmt;
use std::{convert::TryInto, ffi::CString, fmt::Debug, fmt::Formatter, ptr::null_mut};

pub use crate::bindings as unsafe_bindings;
use crate::bindings::idevice_info_t;
use crate::error::{self, IdeviceError, LockdownError, InstProxyError, DebugServerError};
use crate::plist::Plist;

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
    }.into();

    if result != error::IdeviceError::Success {
        return Err(result);
    }

    // Create slice of mutable references to idevice_info_t from device_list and device_count
    let device_list_slice =
        unsafe { std::slice::from_raw_parts_mut(device_list, device_count as usize) };

    let mut to_return = vec![];
    for i in device_list_slice.iter_mut() {
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

/////////////////////
// Yucky Functions //
// To be replaced  //
/////////////////////

pub fn instproxy_client_options_new() -> Plist {
    unsafe { unsafe_bindings::instproxy_client_options_new() }.into()
}

pub fn instproxy_client_options_add(options: Plist, key: String, value: String) {
    let key_c_str = CString::new(key).unwrap();
    let value_c_str = CString::new(value).unwrap();
    let null_ptr: *mut CString = std::ptr::null_mut();

    unsafe {
        unsafe_bindings::instproxy_client_options_add(
            options.plist_t,
            key_c_str.as_ptr(),
            value_c_str.as_ptr(),
            null_ptr,
        )
    };
}

pub fn instproxy_client_options_set_return_attributes(options: Plist, attribute: String) {
    let attributes_c_str = CString::new(attribute).unwrap();
    let null_ptr: *mut CString = std::ptr::null_mut();

    unsafe {
        unsafe_bindings::instproxy_client_options_set_return_attributes(
            options.plist_t,
            attributes_c_str.as_ptr(),
            null_ptr,
        )
    };
}

pub fn instproxy_lookup(
    client: instproxy_client_t,
    appid: String,
    client_opts: Plist,
) -> Option<Plist> {
    let mut apps: unsafe_bindings::plist_t = unsafe { std::mem::zeroed() };

    let appid_c_str = CString::new(appid).unwrap();
    let appid_c_str_ptr: *const std::os::raw::c_char = appid_c_str.as_ptr();
    let appid_c_str_ptr_ptr = appid_c_str_ptr as *mut *const std::os::raw::c_char;

    let results = unsafe {
        unsafe_bindings::instproxy_lookup(
            client.client,
            appid_c_str_ptr_ptr,
            client_opts.plist_t,
            &mut apps,
        )
    };

    if results < 0 {
        return None;
    }

    Some(apps.into())
}

pub fn instproxy_client_options_free(options: Plist) {
    unsafe { unsafe_bindings::instproxy_client_options_free(options.plist_t) };
}

pub fn plist_access_path(apps: Plist, length: u32, appid: String) -> Plist {
    let appid_c_str = CString::new(appid).unwrap();
    return unsafe {
        (unsafe_bindings::plist_access_path(
            apps.plist_t,
            length,
            appid_c_str,
        )).into()
    };
}

pub fn plist_dict_get_item(apps: Plist, key: String) -> Plist {
    let key_c_str = CString::new(key).unwrap();
    return unsafe {
        (unsafe_bindings::plist_dict_get_item(
            apps.plist_t,
            key_c_str.as_ptr(),
        )).into()
    };
}

// Structs
pub struct Device {
    // Front facing properties
    pub name: String,
    pub udid: String,
    pub network: bool,
    // Raw properties
    conn_data: *mut std::os::raw::c_void,
    device: *mut unsafe_bindings::idevice_private,
    lockdown_client: Option<unsafe_bindings::lockdownd_client_t>,
    proxy_client: Option<unsafe_bindings::instproxy_client_t>,
    debug_server: Option<unsafe_bindings::debugserver_client_t>,
}

impl Device {
    pub fn new(
        udid: String,
        network: bool,
        conn_data: *mut std::os::raw::c_void,
        device: *mut unsafe_bindings::idevice_private,
    ) -> Device {
        return Device {
            name: String::new(),
            udid,
            network,
            conn_data,
            device,
            lockdown_client: None,
            proxy_client: None,
            debug_server: None,
        };
    }
    /// Starts the lockdown service for the device
    /// This allows things like debuggers to be attached
    pub fn start_lockdownd_service(&mut self, label: String) -> Result<(), LockdownError> {
        let mut client: unsafe_bindings::lockdownd_client_t = unsafe { std::mem::zeroed() };
        let client_ptr: *mut unsafe_bindings::lockdownd_client_t = &mut client;

        let label_c_str = std::ffi::CString::new(label).unwrap();

        let result = unsafe {
            unsafe_bindings::lockdownd_client_new_with_handshake(
                self.device,
                client_ptr,
                label_c_str.as_ptr(),
            )
        }.into();

        if result != LockdownError::Success {
            return Err(result);
        }

        self.lockdown_client = Some(client);
        Ok(())
    }

    /// Gets a plist value from the device
    pub fn lockdownd_get_value(&mut self, key: String, domain: String) -> Result<Plist, LockdownError> {
        let domain_c_str = std::ffi::CString::new(domain.clone()).unwrap();
        let domain_c_str = if domain == "".to_string() {
            std::ptr::null()
        } else {
            domain_c_str.as_ptr()
        };
        let key_c_str = std::ffi::CString::new(key).unwrap();
        let mut value: unsafe_bindings::plist_t = unsafe { std::mem::zeroed() };

        let result = unsafe {
            unsafe_bindings::lockdownd_get_value(
                self.lockdown_client.unwrap(),
                domain_c_str,
                key_c_str.as_ptr(),
                &mut value,
            )
        }.into();

        if result != LockdownError::Success {
            return Err(result);
        }

        Ok(value.into())
    }

    pub fn get_ios_version(&mut self) -> Result<String, String> {
        let value = self.lockdownd_get_value("ProductVersion".to_string(), "".to_string())?;
        let value = value.get_string_val();
        Ok(value)
    }


    /// Gets the preference plist from the lockdown service
    /// Temporarily returns a string until we can parse it
    pub fn get_preference_plist(&mut self) -> Result<Plist, LockdownError> {
        if self.lockdown_client.is_none() {
            self.start_lockdownd_service(String::from("com.apple.mobile.lockdown"))?;
        }
        let mut plist: unsafe_bindings::plist_t = unsafe { std::mem::zeroed() };
        let plist_ptr: *mut unsafe_bindings::plist_t = &mut plist;
        let domain_ptr: *mut std::os::raw::c_char = std::ptr::null_mut();
        let key_ptr: *mut std::os::raw::c_char = std::ptr::null_mut();
        // Create domain variable
        let result = unsafe {
            unsafe_bindings::lockdownd_get_value(
                self.lockdown_client.unwrap(),
                domain_ptr,
                key_ptr,
                plist_ptr,
            )
        }.into();
        if result != LockdownError::Success {
            return Err(result);
        }

        Ok(plist.into())
    }

    /// Starts the instproxy service for the device
    pub fn start_instproxy_service(&mut self, label: String) -> Result<(), InstProxyError> {
        let mut client: unsafe_bindings::instproxy_client_t = unsafe { std::mem::zeroed() };
        let client_ptr: *mut unsafe_bindings::instproxy_client_t = &mut client;

        let label_c_str = std::ffi::CString::new(label).unwrap();

        let result = unsafe {
            unsafe_bindings::instproxy_client_start_service(
                self.device,
                client_ptr,
                label_c_str.as_ptr(),
            )
        }.into();
        if result != InstProxyError::Success {
            return Err(result);
        }

        self.proxy_client = Some(client);
        Ok(())
    }

    /// Starts the debugserver service for the device
    pub fn start_debug_server(&mut self, label: String) -> Result<(), DebugServerError> {
        let mut client: unsafe_bindings::debugserver_client_t = unsafe { std::mem::zeroed() };
        let client_ptr: *mut unsafe_bindings::debugserver_client_t = &mut client;

        let label_c_str = std::ffi::CString::new(label).unwrap();

        let result = unsafe {
            unsafe_bindings::debugserver_client_start_service(
                self.device,
                client_ptr,
                label_c_str.as_ptr(),
            )
        }.into();
        if result != DebugServerError::Success {
            return Err(result);
        }

        self.debug_server = Some(client);
        Ok(())
    }

    /// Sends a DebugServerCommand to the device
    pub fn send_command(&mut self, command: DebugServerCommand) -> Result<String, DebugServerError> {
        if self.debug_server.is_none() {
            self.start_debug_server(String::from("com.apple.debugserver"))?;
        }
        let mut response: std::os::raw::c_char = unsafe { std::mem::zeroed() };
        let mut response_ptr: *mut std::os::raw::c_char = &mut response;
        let response_ptr_ptr: *mut *mut std::os::raw::c_char = &mut response_ptr;

        let response_size = std::ptr::null_mut();

        let result = unsafe {
            unsafe_bindings::debugserver_client_send_command(
                self.debug_server.unwrap(),
                command.command,
                response_ptr_ptr,
                response_size,
            )
        }.into();
        if result != DebugServerError::Success {
            return Err(result);
        }

        // Convert response to String
        let response_str = unsafe {
            std::ffi::CStr::from_ptr(response_ptr)
                .to_string_lossy()
                .to_string()
        };

        Ok(response_str)
    }

}

impl Debug for Device {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Device {{ name: {}, udid: {}, network: {} }}",
            self.name, self.udid, self.network
        )
    }
}

pub struct DebugServerCommand {
    command: unsafe_bindings::debugserver_command_t,
}

impl DebugServerCommand {
    pub fn new(command: String, arguments: Vec<String>) -> Result<DebugServerCommand, String> {
        let mut command_ptr: unsafe_bindings::debugserver_command_t = unsafe { std::mem::zeroed() };
        let command_ptr_ptr: *mut unsafe_bindings::debugserver_command_t = &mut command_ptr;

        let command_c_str = std::ffi::CString::new(command).unwrap();

        // Create C array
        let mut arguments_c_array: Vec<i8> = Vec::new();
        for i in arguments.iter() {
            let c_str = std::ffi::CString::new(i.clone()).unwrap();
            arguments_c_array.push(c_str.as_bytes_with_nul()[0].try_into().unwrap());
        }
        // Create pointer to to_fill[0]
        let mut c_array_ptr: *mut std::os::raw::c_char = arguments_c_array.as_mut_ptr();
        let c_array_ptr_ptr: *mut *mut std::os::raw::c_char = &mut c_array_ptr;

        let result = unsafe {
            unsafe_bindings::debugserver_command_new(
                command_c_str.as_ptr(),
                arguments.len() as i32,
                c_array_ptr_ptr,
                command_ptr_ptr,
            )
        };
        if result < 0 {
            return Err(String::from("Failed to create command"));
        }

        Ok(DebugServerCommand {
            command: command_ptr,
        })
    }
}

impl Into<DebugServerCommand> for String {
    fn into(self) -> DebugServerCommand {
        // Split string into command and arguments
        let mut split = self.split_whitespace();
        let command = split.next().unwrap().to_string();
        let arguments: Vec<String> = split.map(|s| s.to_string()).collect();
        DebugServerCommand::new(command, arguments).unwrap()
    }
}
impl Into<DebugServerCommand> for &str {
    fn into(self) -> DebugServerCommand {
        self.to_string().into()
    }
}
// Raw, bad structs

pub struct instproxy_client_t {
    pub client: unsafe_bindings::instproxy_client_t,
}

impl instproxy_client_t {
    pub fn new(client: unsafe_bindings::instproxy_client_t) -> Self {
        instproxy_client_t { client }
    }
}

pub struct debugserver_client_t {
    client: unsafe_bindings::debugserver_client_t,
}

impl debugserver_client_t {
    pub fn new(client: unsafe_bindings::debugserver_client_t) -> Self {
        debugserver_client_t { client }
    }
}

pub struct debugserver_command_t {
    command: unsafe_bindings::debugserver_command_t,
}

impl debugserver_command_t {
    pub fn new(command: unsafe_bindings::debugserver_command_t) -> Self {
        debugserver_command_t { command }
    }
}

