// jkcoxson

use crate::{
    bindings as unsafe_bindings, error::MisagentError, idevice::Device,
    services::lockdownd::LockdowndService,
};
use std::{ffi::CString, os::raw::c_char};

use plist_plus::Plist;

/// Manges and checks provisioning profiles
pub struct MisagentClient<'a> {
    pub(crate) pointer: unsafe_bindings::misagent_client_t,
    phantom: std::marker::PhantomData<&'a Device>,
}

impl MisagentClient<'_> {
    /// Creates a new misagent service connection to the device
    /// The use of this function is unknown
    /// # Arguments
    /// * `device` - The device to create the service with
    /// # Returns
    /// The lockdownd service
    ///
    /// ***Verified:*** False
    pub fn new(device: &Device, descriptor: LockdowndService) -> Result<Self, MisagentError> {
        let mut pointer = unsafe { std::mem::zeroed() };
        let result = unsafe {
            unsafe_bindings::misagent_client_new(device.pointer, descriptor.pointer, &mut pointer)
        }
        .into();
        if result != MisagentError::Success {
            return Err(result);
        }

        Ok(MisagentClient {
            pointer,
            phantom: std::marker::PhantomData,
        })
    }

    /// Starts an misagent service connection to the device
    /// # Arguments
    /// * `device` - The device to create the service with
    /// * `service_name` - The name of the service to start
    /// # Returns
    /// An misagent service connection
    ///
    /// ***Verified:*** False
    pub fn start_service(device: &Device, label: impl Into<String>) -> Result<Self, MisagentError> {
        let mut pointer = unsafe { std::mem::zeroed() };
        let result = unsafe {
            unsafe_bindings::misagent_client_start_service(
                device.pointer,
                &mut pointer,
                label.into().as_ptr() as *const c_char,
            )
        }
        .into();
        if result != MisagentError::Success {
            return Err(result);
        }

        Ok(MisagentClient {
            pointer,
            phantom: std::marker::PhantomData,
        })
    }

    /// Installs a provisioning profile on the device
    /// # Arguments
    /// * `profile` - The profile as a plist
    /// # Returns
    /// *none*
    ///
    /// ***Verified:*** False
    pub fn install(&self, profile: Plist) -> Result<(), MisagentError> {
        let result =
            unsafe { unsafe_bindings::misagent_install(self.pointer, profile.get_pointer()) }
                .into();
        if result != MisagentError::Success {
            return Err(result);
        }

        Ok(())
    }

    /// Retrieves provisioning profiles from the device
    /// # Arguments
    /// * `low_version` - Whether the device verion is lower than iOS 9.3
    /// # Returns
    /// A plist containing the results
    ///
    /// ***Verified:*** False
    pub fn copy(&self, low_version: bool) -> Result<Plist, MisagentError> {
        let mut plist = unsafe { std::mem::zeroed() };
        let result = if low_version {
            unsafe { unsafe_bindings::misagent_copy(self.pointer, &mut plist) }.into()
        } else {
            unsafe { unsafe_bindings::misagent_copy_all(self.pointer, &mut plist) }.into()
        };
        if result != MisagentError::Success {
            return Err(result);
        }

        Ok(plist.into())
    }

    /// Removes a provisioning profile from the device
    /// # Arguments
    /// * `id` - The ID of the provisioning profile
    /// # Returns
    /// *none*
    ///
    /// ***Verified:*** False
    pub fn remove(&self, id: impl Into<String>) -> Result<(), MisagentError> {
        let id_c_string = CString::new(id.into()).unwrap();
        let result =
            unsafe { unsafe_bindings::misagent_remove(self.pointer, id_c_string.as_ptr()) }.into();
        if result != MisagentError::Success {
            return Err(result);
        }

        Ok(())
    }

    /// Gets the status code of the last operation
    /// # Arguments
    /// *none*
    /// # Returns
    /// The status code
    pub fn get_status_code(&self) -> Result<i32, MisagentError> {
        let result = unsafe { unsafe_bindings::misagent_get_status_code(self.pointer) };
        if result == -1 {
            return Err(MisagentError::InvalidArg);
        }

        Ok(result)
    }
}

impl Drop for MisagentClient<'_> {
    fn drop(&mut self) {
        unsafe {
            unsafe_bindings::misagent_client_free(self.pointer);
        }
    }
}
