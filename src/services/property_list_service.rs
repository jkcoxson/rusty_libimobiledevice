// jkcoxson

use crate::{
    bindings as unsafe_bindings, error::PropertyListServiceError, idevice::Device,
    services::lockdownd::LockdowndService,
};

pub struct PropertyListServiceClient<'a> {
    pub(crate) pointer: unsafe_bindings::property_list_service_client_t,
    phantom: std::marker::PhantomData<&'a Device>,
}

use plist_plus::Plist;

impl PropertyListServiceClient<'_> {
    /// Creates a preboard client from a property list service
    /// # Arguments
    /// * `device` - The device to connect to
    /// * `descriptor` - The lockdown service to connect on
    /// # Returns
    /// A struct containing the handle to the connection
    ///
    /// ***Verified:*** False
    pub fn new(
        device: &Device,
        descriptor: LockdowndService,
    ) -> Result<Self, PropertyListServiceError> {
        let mut pointer = std::ptr::null_mut();
        let result = unsafe {
            unsafe_bindings::property_list_service_client_new(
                device.pointer,
                descriptor.pointer,
                &mut pointer,
            )
        }
        .into();

        if result != PropertyListServiceError::Success {
            return Err(result);
        }

        Ok(PropertyListServiceClient {
            pointer,
            phantom: std::marker::PhantomData,
        })
    }

    /// Sends a plist to the device
    /// # Arguments
    /// * `data` - The plist to send
    /// # Returns
    /// *none*
    ///
    /// ***Verified:*** False
    pub fn send_xml_plist(&self, data: Plist) -> Result<(), PropertyListServiceError> {
        let result = unsafe {
            unsafe_bindings::property_list_service_send_xml_plist(self.pointer, data.get_pointer())
        }
        .into();

        if result != PropertyListServiceError::Success {
            return Err(result);
        }

        Ok(())
    }

    /// Sends a plist as a binary
    /// # Arguments
    /// * `data` - The plist to send
    /// # Returns
    /// *none*
    ///
    /// ***Verified:*** False
    pub fn send_binary_plist(&self, data: Plist) -> Result<(), PropertyListServiceError> {
        let result = unsafe {
            unsafe_bindings::property_list_service_send_binary_plist(
                self.pointer,
                data.get_pointer(),
            )
        }
        .into();

        if result != PropertyListServiceError::Success {
            return Err(result);
        }

        Ok(())
    }

    /// Receives a plist from the service
    /// # Arguments
    /// * `timeout` - The timeout to wait for, 0 will wait indefinitely
    /// # Returns
    /// *none*
    ///
    /// ***Verified:*** False
    pub fn receive_plist(&self, timeout: u32) -> Result<Plist, PropertyListServiceError> {
        let mut plist_t = std::ptr::null_mut();
        let result = unsafe {
            if timeout == 0 {
                unsafe_bindings::property_list_service_receive_plist(self.pointer, &mut plist_t)
            } else {
                unsafe_bindings::property_list_service_receive_plist_with_timeout(
                    self.pointer,
                    &mut plist_t,
                    timeout,
                )
            }
        }
        .into();

        if result != PropertyListServiceError::Success {
            return Err(result);
        }

        Ok(plist_t.into())
    }

    /// Enables SSL on the service connection
    /// # Arguments
    /// *none*
    /// # Returns
    /// *none*
    ///
    /// ***Verified:*** False
    pub fn enable_ssl(&self) -> Result<(), PropertyListServiceError> {
        let result =
            unsafe { unsafe_bindings::property_list_service_enable_ssl(self.pointer) }.into();

        if result != PropertyListServiceError::Success {
            return Err(result);
        }

        Ok(())
    }

    /// Disables SSL on the service connection
    /// # Arguments
    /// *none*
    /// # Returns
    /// *none*
    ///
    /// ***Verified:*** False
    pub fn disable_ssl(&self) -> Result<(), PropertyListServiceError> {
        let result =
            unsafe { unsafe_bindings::property_list_service_disable_ssl(self.pointer) }.into();

        if result != PropertyListServiceError::Success {
            return Err(result);
        }

        Ok(())
    }
}

impl Drop for PropertyListServiceClient<'_> {
    fn drop(&mut self) {
        unsafe {
            unsafe_bindings::property_list_service_client_free(self.pointer);
        }
    }
}
