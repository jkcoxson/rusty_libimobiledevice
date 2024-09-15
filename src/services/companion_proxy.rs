// jkcoxson

use crate::{
    bindings as unsafe_bindings, error::CompanionProxyError, idevice::Device,
    services::lockdownd::LockdowndService,
};
use std::ffi::CString;

use plist_plus::Plist;

/// A proxy for interoping with devices paired with the iOS device
/// This includes the Apple Watch
pub struct CompanionProxy<'a> {
    pub(crate) pointer: unsafe_bindings::companion_proxy_client_t,
    phantom: std::marker::PhantomData<&'a Device>,
}

impl CompanionProxy<'_> {
    /// Creates a new companion proxy from a lockdown connection
    /// # Arguments
    /// * `device` - The device of which to connect to
    /// * `descriptor` - The service to start the companion proxy on
    /// # Returns
    /// A companion proxy struct
    ///
    /// ***Verified:*** False
    pub fn new(device: &Device, descriptor: LockdowndService) -> Result<Self, CompanionProxyError> {
        let mut pointer = unsafe { std::mem::zeroed() };
        let result = unsafe {
            unsafe_bindings::companion_proxy_client_new(
                device.pointer,
                descriptor.pointer,
                &mut pointer,
            )
        }
        .into();
        if result != CompanionProxyError::Success {
            return Err(result);
        }

        Ok(CompanionProxy {
            pointer,
            phantom: std::marker::PhantomData,
        })
    }

    /// Starts a new service on the device and starts a companion proxy on top of it
    /// # Arguments
    /// * `device` - The device of which to connect to
    /// * `label` - The label to give the service
    /// # Returns
    /// A companion proxy struct
    ///
    /// ***Verified:*** False
    pub fn start_service(
        device: &Device,
        label: impl Into<String>,
    ) -> Result<Self, CompanionProxyError> {
        let label_c_string = CString::new(label.into()).unwrap();

        let mut pointer = unsafe { std::mem::zeroed() };
        let result = unsafe {
            unsafe_bindings::companion_proxy_client_start_service(
                device.pointer,
                &mut pointer,
                label_c_string.as_ptr(),
            )
        }
        .into();
        if result != CompanionProxyError::Success {
            return Err(result);
        }

        Ok(CompanionProxy {
            pointer,
            phantom: std::marker::PhantomData,
        })
    }

    /// Sends a message to the companion proxy service
    /// # Arguments
    /// * `message` -  A plist containing the desired message
    /// # Returns
    /// *none*
    ///
    /// ***Verified:*** False
    pub fn send(&self, message: Plist) -> Result<(), CompanionProxyError> {
        let result =
            unsafe { unsafe_bindings::companion_proxy_send(self.pointer, message.get_pointer()) }
                .into();
        if result != CompanionProxyError::Success {
            return Err(result);
        }

        Ok(())
    }

    /// Receives a message from the companion proxy service.
    /// Blocks until a full plist is received
    /// # Arguments
    /// *none*
    /// # Returns
    /// A plist containing the message
    ///
    /// ***Verified:*** False
    pub fn receive(&self) -> Result<Plist, CompanionProxyError> {
        let mut plist = unsafe { std::mem::zeroed() };
        let result =
            unsafe { unsafe_bindings::companion_proxy_receive(self.pointer, &mut plist) }.into();
        if result != CompanionProxyError::Success {
            return Err(result);
        }

        Ok(plist.into())
    }

    /// Fetches the registry from the iOS device.
    /// Closes the connection after a reply, so this consumes the companion proxy.
    /// # Arguments
    /// *none*
    /// # Returns
    /// A plist containing the device registry
    ///
    /// ***Verified:*** False
    pub fn get_device_registry(self) -> Result<Plist, CompanionProxyError> {
        let mut plist = unsafe { std::mem::zeroed() };
        let result = unsafe {
            unsafe_bindings::companion_proxy_get_device_registry(self.pointer, &mut plist)
        }
        .into();
        if result != CompanionProxyError::Success {
            return Err(result);
        }

        Ok(plist.into())
    }

    /// Gets a value from the device's registry.
    /// Closes the connection after a reply, so this consumes the companion proxy.
    /// # Arguments
    /// * `udid` - The UDID of the paired device
    /// * `key` - The value to fetch from the registry
    pub fn get_value_from_registry(
        self,
        udid: impl Into<String>,
        key: impl Into<String>,
    ) -> Result<Plist, CompanionProxyError> {
        let udid_c_string = CString::new(udid.into()).unwrap();
        let key_c_string = CString::new(key.into()).unwrap();

        let mut plist = unsafe { std::mem::zeroed() };
        let result = unsafe {
            unsafe_bindings::companion_proxy_get_value_from_registry(
                self.pointer,
                udid_c_string.as_ptr(),
                key_c_string.as_ptr(),
                &mut plist,
            )
        }
        .into();
        if result != CompanionProxyError::Success {
            return Err(result);
        }

        Ok(plist.into())
    }

    /// Starts a port forwarding service for a paired device
    /// # Arguments
    /// * `port` - The internal port to open to
    /// * `service_name` - The name of the service
    /// * `options` - Options for the port forward
    /// # Returns
    /// The external port that was opened
    ///
    /// ***Verified:*** False
    pub fn start_forwarding_service_port(
        &self,
        port: u16,
        service_name: impl Into<String>,
        options: Plist,
    ) -> Result<u16, CompanionProxyError> {
        let mut result_port = 0;
        let service_name_c_string = CString::new(service_name.into()).unwrap();

        let result = unsafe {
            unsafe_bindings::companion_proxy_start_forwarding_service_port(
                self.pointer,
                port,
                service_name_c_string.as_ptr(),
                &mut result_port,
                options.get_pointer(),
            )
        }
        .into();
        if result != CompanionProxyError::Success {
            return Err(result);
        }

        Ok(result_port)
    }

    /// Closes an opened port
    /// # Arguments
    /// * `port` - The opened port to close (somebody figure out if this is the internal or external port pls)
    /// # Returns
    /// *none*
    ///
    /// ***Verified:*** False
    pub fn stop_forwarding_service_port(&self, port: u16) -> Result<(), CompanionProxyError> {
        let result = unsafe {
            unsafe_bindings::companion_proxy_stop_forwarding_service_port(self.pointer, port)
        }
        .into();
        if result != CompanionProxyError::Success {
            return Err(result);
        }

        Ok(())
    }
}

impl Drop for CompanionProxy<'_> {
    fn drop(&mut self) {
        unsafe {
            unsafe_bindings::companion_proxy_client_free(self.pointer);
        }
    }
}
