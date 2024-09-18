// jkcoxson

use std::{ffi::CString, os::raw::c_char};

use crate::{
    bindings as unsafe_bindings, connection::DeviceConnection, error::FileRelayError,
    idevice::Device, services::lockdownd::LockdowndService,
};

/// Relays files from the iOS device to the host
pub struct FileRelay<'a> {
    pub pointer: unsafe_bindings::file_relay_client_t,
    phantom: std::marker::PhantomData<&'a Device>,
}

impl FileRelay<'_> {
    /// Creates a new file relay service from a lockdown service
    /// # Arguments
    /// * `device` - The device to create the sevice with
    /// * `service` - The lockdown service to connect on
    /// # Returns
    /// A struct containing the handle to the service
    ///
    /// ***Verified:*** False
    pub fn new(device: &Device, service: LockdowndService) -> Result<Self, FileRelayError> {
        let mut pointer = std::ptr::null_mut();
        let result = unsafe {
            unsafe_bindings::file_relay_client_new(device.pointer, service.pointer, &mut pointer)
        }
        .into();

        if result != FileRelayError::Success {
            return Err(result);
        }

        Ok(FileRelay {
            pointer,
            phantom: std::marker::PhantomData,
        })
    }

    /// Starts a new service with file relay
    /// # Arguments
    /// * `device` - The device to create the sevice with
    /// * `label` - The label to give the connection
    /// # Returns
    /// A struct containing the handle to the service
    ///
    /// ***Verified:*** False
    pub fn start_service(
        device: &Device,
        label: impl Into<String>,
    ) -> Result<Self, FileRelayError> {
        let mut pointer = std::ptr::null_mut();
        let label_c_string = CString::new(label.into()).unwrap();
        let result = unsafe {
            unsafe_bindings::file_relay_client_start_service(
                device.pointer,
                &mut pointer,
                label_c_string.as_ptr(),
            )
        }
        .into();

        if result != FileRelayError::Success {
            return Err(result);
        }

        Ok(FileRelay {
            pointer,
            phantom: std::marker::PhantomData,
        })
    }

    /// Request data for network interfaces.
    /// Data will be placed in /tmp/mobile_file_relay.XXXX and must be removed manually.
    /// # Arguments
    /// * `sources` - A list of sources to request data for
    /// * `connection` - A connection to the device
    /// * `timeout` - How long to wait for a response. If 0, this will block indefinitely.
    /// # Returns
    /// *none*
    pub fn request_sources(
        &self,
        sources: Vec<FileRelaySources>,
        mut connection: DeviceConnection,
        timeout: u32,
    ) -> Result<(), FileRelayError> {
        let mut source_c_strings: Vec<CString> = Vec::with_capacity(sources.len());
        let mut source_c_strings_ptrs: Vec<*const c_char> = Vec::with_capacity(sources.len() + 1);
        for source in sources {
            source_c_strings.push(source.into());
            source_c_strings_ptrs.push(source_c_strings.last().unwrap().as_ptr());
        }
        source_c_strings_ptrs.push(std::ptr::null());

        if timeout == 0 {
            let result = unsafe {
                unsafe_bindings::file_relay_request_sources(
                    self.pointer,
                    source_c_strings_ptrs.as_mut_ptr(),
                    &mut connection.pointer,
                )
            }
            .into();

            if result != FileRelayError::Success {
                return Err(result);
            }
        } else {
            let result = unsafe {
                unsafe_bindings::file_relay_request_sources_timeout(
                    self.pointer,
                    source_c_strings_ptrs.as_mut_ptr(),
                    &mut connection.pointer,
                    timeout,
                )
            }
            .into();

            if result != FileRelayError::Success {
                return Err(result);
            }
        }

        Ok(())
    }
}

/// The different types of interface sources that can have data requested for
pub enum FileRelaySources {
    AppleSupport,
    Network,
    VPN,
    WiFi,
    UserDatabases,
    CrashReporter,
    Tmp,
    SystemConfiguration,
}

impl From<FileRelaySources> for CString {
    fn from(source: FileRelaySources) -> Self {
        match source {
            FileRelaySources::AppleSupport => CString::new("AppleSupport").unwrap(),
            FileRelaySources::Network => CString::new("Network").unwrap(),
            FileRelaySources::VPN => CString::new("VPN").unwrap(),
            FileRelaySources::WiFi => CString::new("WiFi").unwrap(),
            FileRelaySources::UserDatabases => CString::new("UserDatabases").unwrap(),
            FileRelaySources::CrashReporter => CString::new("CrashReporter").unwrap(),
            FileRelaySources::Tmp => CString::new("Tmp").unwrap(),
            FileRelaySources::SystemConfiguration => CString::new("SystemConfiguration").unwrap(),
        }
    }
}

impl Drop for FileRelay<'_> {
    fn drop(&mut self) {
        unsafe {
            unsafe_bindings::file_relay_client_free(self.pointer);
        }
    }
}
