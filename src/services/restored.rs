// jkcoxson

use std::ffi::CString;

use crate::{bindings as unsafe_bindings, error::RestoredError, idevice::Device};

use plist_plus::Plist;

/// Restores an iDevice to a specific backup or iOS version
pub struct RestoredClient<'a> {
    pub(crate) pointer: unsafe_bindings::restored_client_t,
    phantom: std::marker::PhantomData<&'a Device>,
}

impl RestoredClient<'_> {
    /// Starts a new connection and adds a restored client to it
    /// # Arguments
    /// * `device` - The device to connect to
    /// * `label` - The label for the connection
    /// # Returns
    /// A struct containing the handle to the connection
    ///
    /// ***Verified:*** False
    pub fn new(device: &Device, label: impl Into<String>) -> Result<Self, RestoredError> {
        let mut pointer = unsafe { std::mem::zeroed() };
        let label_c_string = CString::new(label.into()).unwrap();
        let result = unsafe {
            unsafe_bindings::restored_client_new(
                device.pointer,
                &mut pointer,
                label_c_string.as_ptr(),
            )
        }
        .into();
        if result != RestoredError::Success {
            return Err(result);
        }

        Ok(Self {
            pointer,
            phantom: std::marker::PhantomData,
        })
    }

    /// Get the type of restored client
    /// # Arguments
    /// *none*
    /// # Returns
    /// A type and version of the client
    ///
    /// ***Verified:*** False
    pub fn query_type(&self) -> Result<(String, u64), RestoredError> {
        let mut type_ = std::ptr::null_mut();
        let mut version = 0;
        let result =
            unsafe { unsafe_bindings::restored_query_type(self.pointer, &mut type_, &mut version) }
                .into();
        if result != RestoredError::Success {
            return Err(result);
        }

        let type_ = unsafe {
            std::ffi::CStr::from_ptr(type_)
                .to_string_lossy()
                .into_owned()
        };
        Ok((type_, version))
    }

    /// Queries a value from the client
    /// # Arguments
    /// * `key` - The key to get the value for
    /// # Returns
    /// A plist with the returned value
    ///
    /// ***Verified:*** False
    pub fn query_value(&self, key: impl Into<String>) -> Result<Plist, RestoredError> {
        let mut value = std::ptr::null_mut();
        let key_c_string = CString::new(key.into()).unwrap();
        let result = unsafe {
            unsafe_bindings::restored_query_value(self.pointer, key_c_string.as_ptr(), &mut value)
        }
        .into();
        if result != RestoredError::Success {
            return Err(result);
        }

        Ok(value.into())
    }

    /// Gets a value from the client
    /// # Arguments
    /// * `key` - The key to get the value for
    /// # Returns
    /// A plist with the returned value
    ///
    /// ***Verified:*** False
    pub fn get_value(&self, key: impl Into<String>) -> Result<Plist, RestoredError> {
        let mut value = std::ptr::null_mut();
        let key_c_string = CString::new(key.into()).unwrap();
        let result = unsafe {
            unsafe_bindings::restored_get_value(self.pointer, key_c_string.as_ptr(), &mut value)
        }
        .into();
        if result != RestoredError::Success {
            return Err(result);
        }

        Ok(value.into())
    }

    /// Sends a message to the client
    /// # Arguments
    /// * `data` - The data to send as a plist
    /// # Returns
    /// *none*
    ///
    /// ***Verified:*** False
    pub fn send(&self, data: Plist) -> Result<(), RestoredError> {
        let result =
            unsafe { unsafe_bindings::restored_send(self.pointer, data.get_pointer()) }.into();
        if result != RestoredError::Success {
            return Err(result);
        }

        Ok(())
    }

    /// Receives a message from client
    /// # Arguments
    /// *none*
    /// # Returns
    /// A plist containing the response
    ///
    /// ***Verified:*** False
    pub fn receive(&self) -> Result<Plist, RestoredError> {
        let mut value = std::ptr::null_mut();
        let result = unsafe { unsafe_bindings::restored_receive(self.pointer, &mut value) }.into();
        if result != RestoredError::Success {
            return Err(result);
        }

        Ok(value.into())
    }

    /// Sends a goodbye, terminating the connection
    /// # Arguments
    /// *none*
    /// # Returns
    /// *none*
    ///
    /// ***Verified:*** False
    pub fn goodbye(self) -> Result<(), RestoredError> {
        let result = unsafe { unsafe_bindings::restored_goodbye(self.pointer) }.into();
        if result != RestoredError::Success {
            return Err(result);
        }

        Ok(())
    }

    /// Starts a restore of the device
    /// # Arguments
    /// * `options` - The options for the restore
    /// * `version` - The restore protocol version
    /// # Returns
    /// *none*
    ///
    /// ***Verified:*** False
    pub fn start_restore(&self, options: Option<Plist>, version: u64) -> Result<(), RestoredError> {
        let ptr = options.as_ref().map_or(std::ptr::null_mut(), |v| v.get_pointer());

        let result =
            unsafe { unsafe_bindings::restored_start_restore(self.pointer, ptr, version) }.into();
        if result != RestoredError::Success {
            return Err(result);
        }

        Ok(())
    }

    /// Reboots the device
    /// # Arguments
    /// *none*
    /// # Returns
    /// *none*
    ///
    /// ***Verified:*** False
    pub fn reboot(self) -> Result<(), RestoredError> {
        let result = unsafe { unsafe_bindings::restored_reboot(self.pointer) }.into();
        if result != RestoredError::Success {
            return Err(result);
        }

        Ok(())
    }

    /// Sets the label for the connection
    /// # Arguments
    /// * `label` - The label to use for the connection
    /// # Returns
    /// *none*
    ///
    /// ***Verified:*** False
    pub fn set_label(&self, label: impl Into<String>) {
        let label_c_string = CString::new(label.into()).unwrap();
        unsafe {
            unsafe_bindings::restored_client_set_label(self.pointer, label_c_string.as_ptr())
        };
    }
}

impl Drop for RestoredClient<'_> {
    fn drop(&mut self) {
        unsafe {
            unsafe_bindings::restored_client_free(self.pointer);
        }
    }
}
