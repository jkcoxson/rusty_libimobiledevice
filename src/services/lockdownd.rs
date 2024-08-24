// jkcoxson

use std::os::raw::c_char;

use crate::bindings as unsafe_bindings;
use crate::error::LockdowndError;
use crate::idevice::Device;

use log::info;
use plist_plus::Plist;

/// A jumping point for other services.
/// Lockdownd is in charge of starting other services and opening ports for them.
/// Lockdown can be used for simple data transactions, but most requests will be done through other services.
pub struct LockdowndClient<'a> {
    pub(crate) pointer: unsafe_bindings::lockdownd_client_t,
    phantom: std::marker::PhantomData<&'a Device>,
}

/// A pair record for lockdown
pub struct LockdowndPairRecord {
    pub device_certificate: String,
    pub host_certificate: String,
    pub root_certificate: String,
    pub host_id: String,
    pub system_buid: String,
}

unsafe impl Send for LockdowndClient<'_> {}
unsafe impl Sync for LockdowndClient<'_> {}

pub struct LockdowndService<'a> {
    pub(crate) pointer: unsafe_bindings::lockdownd_service_descriptor_t,
    pub port: u32,
    pub(crate) phantom: std::marker::PhantomData<&'a LockdowndClient<'a>>,
}

unsafe impl Send for LockdowndService<'_> {}
unsafe impl Sync for LockdowndService<'_> {}

impl LockdowndClient<'_> {
    /// Creates a new lockdown service and starts initial handshake
    /// # Arguments
    /// * `device` - The device to start the service on
    /// * `label` - The label to give the connection
    /// # Returns
    /// A struct containing the handle to the service
    ///
    /// ***Verified:*** False
    pub fn new(device: &Device, label: impl Into<String>) -> Result<Self, LockdowndError> {
        let mut client: unsafe_bindings::lockdownd_client_t = unsafe { std::mem::zeroed() };
        let client_ptr: *mut unsafe_bindings::lockdownd_client_t = &mut client;

        let label_c_str = std::ffi::CString::new(label.into()).unwrap();

        info!("Creating lockdownd client for {}", device.get_udid());
        let result = unsafe {
            unsafe_bindings::lockdownd_client_new_with_handshake(
                device.pointer,
                client_ptr,
                label_c_str.as_ptr(),
            )
        }
        .into();

        if result != LockdowndError::Success {
            return Err(result);
        }

        Ok(LockdowndClient {
            pointer: unsafe { *client_ptr },
            phantom: std::marker::PhantomData,
        })
    }

    /// Gets a preference value from the lockdown service
    /// # Arguments
    /// * `key` - The key of the value to fetch. Pass "" to query all keys.
    /// * `domain` - The domain that the value exists in. Pass "" to query the gloabl domain.
    /// # Returns
    /// A plist containing the value
    ///
    /// ***Verified:*** False
    pub fn get_value(
        &self,
        key: impl Into<String>,
        domain: impl Into<String>,
    ) -> Result<Plist, LockdowndError> {
        let domain = domain.into();
        let key = key.into();
        let domain_c_str = std::ffi::CString::new(domain.clone()).unwrap();
        let domain_c_str = if domain == *"" {
            std::ptr::null()
        } else {
            domain_c_str.as_ptr()
        };
        let key_c_str = std::ffi::CString::new(key.clone()).unwrap();
        let key_c_str = if key == *"" {
            std::ptr::null()
        } else {
            key_c_str.as_ptr()
        };

        let mut value: unsafe_bindings::plist_t = unsafe { std::mem::zeroed() };

        info!("Getting value for {}", key);
        let result = unsafe {
            unsafe_bindings::lockdownd_get_value(self.pointer, domain_c_str, key_c_str, &mut value)
        }
        .into();

        if result != LockdowndError::Success {
            return Err(result);
        }

        Ok(value.into())
    }

    /// Sets a preference value on the device
    /// # Arguments
    /// * `key` - The key of the value to set
    /// * `domain` - The domain to set the value in. Pass "" for the global domain.
    /// * `value` - The value to set
    /// # Returns
    /// *none*
    ///
    /// ***Verified:***
    pub fn set_value(
        &self,
        key: impl Into<String>,
        domain: impl Into<String>,
        value: Plist,
    ) -> Result<(), LockdowndError> {
        let domain: String = domain.into();
        let key: String = key.into();
        let domain_c_str = std::ffi::CString::new(domain.clone()).unwrap();
        let domain_ptr = if domain.is_empty() {
            std::ptr::null()
        } else {
            domain_c_str.as_ptr()
        };
        let key_c_str = std::ffi::CString::new(key.clone()).unwrap();
        let key_ptr = if key.is_empty() {
            std::ptr::null()
        } else {
            key_c_str.as_ptr()
        };

        info!("Setting value for {}", key);
        let result = unsafe {
            unsafe_bindings::lockdownd_set_value(
                self.pointer,
                domain_ptr,
                key_ptr,
                value.get_pointer(),
            )
        }
        .into();

        value.false_drop();

        if result != LockdowndError::Success {
            return Err(result);
        }

        Ok(())
    }

    /// Removes a preference value from the device
    /// # Arguments
    /// * `key` - The key to remove. Pass "" to remove all keys in the current domain.
    /// * 'domain' - The domain to remove the key in. Pass "" for the global domain.
    /// # Returns
    /// *none*
    ///
    /// ***Verified:*** False
    pub fn remove_value(
        &self,
        key: impl Into<String>,
        domain: impl Into<String>,
    ) -> Result<(), LockdowndError> {
        let domain = domain.into();
        let key = key.into();
        let domain_c_str = std::ffi::CString::new(domain.clone()).unwrap();
        let domain_c_str = if domain == *"" {
            std::ptr::null()
        } else {
            domain_c_str.as_ptr()
        };
        let key_c_str = std::ffi::CString::new(key.clone()).unwrap();
        let key_c_str = if key == *"" {
            std::ptr::null()
        } else {
            key_c_str.as_ptr()
        };

        info!("Removing value for {}", key);
        let result = unsafe {
            unsafe_bindings::lockdownd_remove_value(self.pointer, domain_c_str, key_c_str)
        }
        .into();

        if result != LockdowndError::Success {
            return Err(result);
        }

        Ok(())
    }

    /// Commands lockdownd to start a service.
    /// This will not have a Rust type, and will only be able to use basic methods.
    /// # Arguments
    /// * `service` - The identifier of the service to start
    /// * `escrow_bag` - Whether to start the service with escrow_bag
    /// # Returns
    /// A raw lockdownd service
    ///
    /// ***Verified:*** False
    pub fn start_service(
        &mut self,
        service: impl Into<String>,
        escrow_bag: bool,
    ) -> Result<LockdowndService, LockdowndError> {
        let service = service.into();
        let label_c_str = std::ffi::CString::new(service.clone()).unwrap();
        let label_c_str = if service == *"" {
            std::ptr::null()
        } else {
            label_c_str.as_ptr()
        };

        let mut service: unsafe_bindings::lockdownd_service_descriptor_t =
            unsafe { std::mem::zeroed() };

        info!("Starting lockdown service");
        let result = if escrow_bag {
            unsafe {
                unsafe_bindings::lockdownd_start_service(self.pointer, label_c_str, &mut service)
            }
            .into()
        } else {
            unsafe {
                unsafe_bindings::lockdownd_start_service_with_escrow_bag(
                    self.pointer,
                    label_c_str,
                    &mut service,
                )
            }
            .into()
        };

        if result != LockdowndError::Success {
            return Err(result);
        }

        let service_struct: &unsafe_bindings::lockdownd_service_descriptor =
            unsafe { &*service };

        Ok(LockdowndService {
            pointer: service,
            port: service_struct.port as u32,
            phantom: std::marker::PhantomData,
        })
    }

    /// Opens a session with lockdownd and switches to SSL if requested by the device
    /// # Arguments
    /// * `host_id` - The ID of the host
    /// # Returns
    /// The session ID and whether SSL was enabled
    ///
    /// ***Verified:*** False
    pub fn start_session(
        &self,
        host_id: impl Into<String>,
    ) -> Result<(String, bool), LockdowndError> {
        let host_id = host_id.into();
        let host_id_c_str = std::ffi::CString::new(host_id).unwrap();
        let mut session_id = unsafe { std::mem::zeroed() };
        let mut ssl_enabled = unsafe { std::mem::zeroed() };

        let result = unsafe {
            unsafe_bindings::lockdownd_start_session(
                self.pointer,
                host_id_c_str.as_ptr(),
                &mut session_id,
                &mut ssl_enabled,
            )
        }
        .into();

        if result != LockdowndError::Success {
            return Err(result);
        }

        unsafe {
            Ok((
                std::ffi::CStr::from_ptr(session_id)
                    .to_string_lossy()
                    .into_owned(),
                ssl_enabled != 0,
            ))
        }
    }

    /// Stops a lockdownd session started by `start_session`
    /// # Arguments
    /// * `session` - The ID of the session created to stop
    /// # Returns
    /// *none*
    ///
    /// ***Verified:*** False
    pub fn stop_session(&self, session_id: impl Into<String>) -> Result<(), LockdowndError> {
        let session_id = session_id.into();
        let session_id_c_str = std::ffi::CString::new(session_id.clone()).unwrap();
        let session_id_c_str = if session_id == *"" {
            std::ptr::null()
        } else {
            session_id_c_str.as_ptr()
        };

        let result =
            unsafe { unsafe_bindings::lockdownd_stop_session(self.pointer, session_id_c_str) }
                .into();

        if result != LockdowndError::Success {
            return Err(result);
        }

        Ok(())
    }

    /// Sends a message to lockdownd
    /// # Arguments
    /// * `message` - The message to send
    /// # Returns
    /// *none*
    ///
    /// ***Verified:*** False
    pub fn send(&self, message: Plist) -> Result<(), LockdowndError> {
        let result =
            unsafe { unsafe_bindings::lockdownd_send(self.pointer, message.get_pointer()) }.into();

        if result != LockdowndError::Success {
            return Err(result);
        }

        Ok(())
    }

    /// Receives a message from lockdownd.
    /// Blocks until a full plist is received
    /// # Arguments
    /// *none*
    /// # Returns
    /// A plist with the message received
    ///
    /// ***Verified:*** False
    pub fn receive(&self) -> Result<Plist, LockdowndError> {
        let mut plist: unsafe_bindings::plist_t = unsafe { std::mem::zeroed() };

        let result = unsafe { unsafe_bindings::lockdownd_receive(self.pointer, &mut plist) }.into();

        if result != LockdowndError::Success {
            return Err(result);
        }

        Ok(plist.into())
    }

    /// Attempts to pair with the device.
    /// This will only succeed on USB devices, and will add the pairing file to usbmuxd's pairing file storage.
    /// # Arguments
    /// * `pair_record` - A pair record for the host. If None, usbmuxd will fetch the host's record.
    /// * `options` - The options for pairing
    /// # Returns
    /// *none*
    ///
    /// ***Verified:*** False
    pub fn pair(
        &self,
        pairing_record: Option<LockdowndPairRecord>,
        options: Option<Plist>,
    ) -> Result<(), LockdowndError> {
        let pair_ptr: unsafe_bindings::lockdownd_pair_record_t =
            if let Some(pairing_record) = pairing_record {
                &mut pairing_record.into()
            } else {
                std::ptr::null_mut()
            };

        let mut response = unsafe { std::mem::zeroed() };

        let result = if let Some(options) = options {
            unsafe {
                unsafe_bindings::lockdownd_pair_with_options(
                    self.pointer,
                    pair_ptr,
                    options.get_pointer(),
                    &mut response,
                )
            }
            .into()
        } else {
            unsafe { unsafe_bindings::lockdownd_pair(self.pointer, pair_ptr) }.into()
        };

        if result != LockdowndError::Success {
            return Err(result);
        }

        Ok(())
    }

    /// Validates that the device is paired with a specified host
    /// # Arguments
    /// * `pairing_record` - The host pairing record
    /// # Returns
    /// *none*
    ///
    /// ***Verified:*** False
    pub fn validate_pair(&self, pairing_record: LockdowndPairRecord) -> Result<(), LockdowndError> {
        let mut pairing_record = pairing_record.into();
        let result =
            unsafe { unsafe_bindings::lockdownd_validate_pair(self.pointer, &mut pairing_record) }
                .into();

        if result != LockdowndError::Success {
            return Err(result);
        }

        Ok(())
    }

    /// Unpairs the device from the host
    /// # Arguments
    /// * `pairing_record` - The host pairing record
    /// # Returns
    /// *none*
    ///
    /// ***Verified:*** False
    pub fn unpair(&self, pairing_record: LockdowndPairRecord) -> Result<(), LockdowndError> {
        let mut pairing_record = pairing_record.into();
        let result =
            unsafe { unsafe_bindings::lockdownd_unpair(self.pointer, &mut pairing_record) }.into();

        if result != LockdowndError::Success {
            return Err(result);
        }

        Ok(())
    }

    /// Activates the device. You will need an activation record from Apple's servers. Only works with an open session.
    /// # Arguments
    /// * `activation_record` - The activation record from Apple's servers
    /// # Returns
    /// *none*
    ///
    /// ***Verified:*** False
    pub fn activate(&self, activation_record: Plist) -> Result<(), LockdowndError> {
        let result = unsafe {
            unsafe_bindings::lockdownd_activate(self.pointer, activation_record.get_pointer())
        }
        .into();

        if result != LockdowndError::Success {
            return Err(result);
        }

        Ok(())
    }

    /// Deactivates a device, forcing it to show the "Activate with iTunes screen".
    /// # Arguments
    /// *none*
    /// # Returns
    /// *none*
    ///
    /// ***Verified:*** False
    pub fn deactivate(&self) -> Result<(), LockdowndError> {
        let result = unsafe { unsafe_bindings::lockdownd_deactivate(self.pointer) }.into();

        if result != LockdowndError::Success {
            return Err(result);
        }

        Ok(())
    }

    /// Forces the device to enter recovery mode immediately
    /// # Arguments
    /// *none*
    /// # Returns
    /// *none*
    ///
    /// ***Verified:*** False
    pub fn enter_recovery(&self) -> Result<(), LockdowndError> {
        let result = unsafe { unsafe_bindings::lockdownd_enter_recovery(self.pointer) }.into();

        if result != LockdowndError::Success {
            return Err(result);
        }

        Ok(())
    }

    /// Sends a goodbye to lockdown, terminating the connection
    /// # Arguments
    /// *none*
    /// # Returns
    /// *none*
    ///
    /// ***Verified:*** False
    pub fn goodbye(self) -> Result<(), LockdowndError> {
        let result = unsafe { unsafe_bindings::lockdownd_goodbye(self.pointer) }.into();

        if result != LockdowndError::Success {
            return Err(result);
        }

        Ok(())
    }

    /// Sets the label for lockdownd requests
    /// # Arguments
    /// * `label` - The label to use
    /// # Returns
    /// *none*
    ///
    /// ***Verified:*** False
    pub fn client_set_label(&self, label: impl Into<String>) {
        let label = label.into();
        let label_c_str = std::ffi::CString::new(label.clone()).unwrap();
        let label_c_str = if label == *"" {
            std::ptr::null()
        } else {
            label_c_str.as_ptr()
        };

        unsafe { unsafe_bindings::lockdownd_client_set_label(self.pointer, label_c_str) };
    }

    /// Get the UDID of the device
    /// # Arguments
    /// *none*
    /// # Returns
    /// The UDID as a string
    ///
    /// ***Verified:*** False
    pub fn get_device_udid(&self) -> Result<String, LockdowndError> {
        let mut udid_c_str = unsafe { std::mem::zeroed() };

        let result =
            unsafe { unsafe_bindings::lockdownd_get_device_udid(self.pointer, &mut udid_c_str) }
                .into();

        if result != LockdowndError::Success {
            return Err(result);
        }

        Ok(unsafe {
            std::ffi::CStr::from_ptr(udid_c_str)
                .to_string_lossy()
                .into_owned()
        })
    }

    /// Gets the device's name
    /// # Arguments
    /// *none*
    /// # Returns
    /// *none*
    ///
    /// ***Verified:*** False
    pub fn get_device_name(&self) -> Result<String, LockdowndError> {
        let mut name_c_str = unsafe { std::mem::zeroed() };

        let result =
            unsafe { unsafe_bindings::lockdownd_get_device_name(self.pointer, &mut name_c_str) }
                .into();

        if result != LockdowndError::Success {
            return Err(result);
        }

        Ok(unsafe {
            std::ffi::CStr::from_ptr(name_c_str)
                .to_string_lossy()
                .into_owned()
        })
    }

    /// Get the data classes the device supports
    /// # Arguments
    /// *none*
    /// # Returns
    /// A list of class names that are supported
    ///
    /// ***Verified:*** False
    pub fn get_sync_data_classes(&self) -> Result<Vec<String>, LockdowndError> {
        let mut classes_c_str = unsafe { std::mem::zeroed() };
        let mut count = unsafe { std::mem::zeroed() };

        let result = unsafe {
            unsafe_bindings::lockdownd_get_sync_data_classes(
                self.pointer,
                &mut classes_c_str,
                &mut count,
            )
        }
        .into();

        if result != LockdowndError::Success {
            return Err(result);
        }

        let classes = unsafe { std::ffi::CStr::from_ptr(*classes_c_str) }
            .to_str()
            .unwrap()
            .split(',')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let result = unsafe { unsafe_bindings::lockdownd_data_classes_free(classes_c_str) }.into();

        if result != LockdowndError::Success {
            return Err(result);
        }

        Ok(classes)
    }

    /// Get the current type of the service daemon
    /// # Arguments
    /// *none*
    /// # Returns
    /// The daemon type as a string
    ///
    /// ***Verified:*** False
    pub fn query_type(&self) -> Result<String, LockdowndError> {
        let mut type_c_str: *mut c_char = std::ptr::null_mut();
        let result =
            unsafe { unsafe_bindings::lockdownd_query_type(self.pointer, &mut type_c_str) }.into();
        if result != LockdowndError::Success {
            return Err(result);
        }

        let type_str = unsafe { std::ffi::CStr::from_ptr(type_c_str as *const c_char) }
            .to_str()
            .unwrap()
            .to_string();

        Ok(type_str)
    }
}

impl From<LockdowndPairRecord> for unsafe_bindings::lockdownd_pair_record {
    fn from(l: LockdowndPairRecord) -> Self {
        info!("Converting device certificate");
        let device_certificate = std::ffi::CString::new(l.device_certificate).unwrap();
        info!("Converting host certificate");
        let host_certificate = std::ffi::CString::new(l.host_certificate).unwrap();
        info!("Converting root certificate");
        let root_certificate = std::ffi::CString::new(l.root_certificate).unwrap();
        info!("Converting host id");
        let host_id = std::ffi::CString::new(l.host_id).unwrap();
        info!("Converting system buid");
        let system_buid = std::ffi::CString::new(l.system_buid).unwrap();

        info!("Setting device certificate");
        Self {
            device_certificate: device_certificate.as_ptr() as *mut c_char,
            host_certificate: host_certificate.as_ptr() as *mut c_char,
            root_certificate: root_certificate.as_ptr() as *mut c_char,
            host_id: host_id.as_ptr() as *mut c_char,
            system_buid: system_buid.as_ptr() as *mut c_char,
        }
    }
}

impl Drop for LockdowndClient<'_> {
    fn drop(&mut self) {
        info!("Dropping LockdowndClient");
        unsafe { unsafe_bindings::lockdownd_client_free(self.pointer) };
    }
}

impl Drop for LockdowndService<'_> {
    fn drop(&mut self) {
        info!("Dropping LockdowndService");
        unsafe {
            unsafe_bindings::lockdownd_service_descriptor_free(self.pointer);
        }
    }
}
