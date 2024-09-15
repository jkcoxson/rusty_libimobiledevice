// jkcoxson

use std::ffi::CString;

use crate::{
    bindings as unsafe_bindings, error::HouseArrestError, idevice::Device,
    services::lockdownd::LockdowndService,
};

use plist_plus::Plist;

/// iTunes file transfer service.
/// This differs from AFC in that this is for managing files in app specific storage accessable by iTunes.
pub struct HouseArrest<'a> {
    pub(crate) pointer: unsafe_bindings::house_arrest_client_t,
    phantom: std::marker::PhantomData<&'a Device>,
}

impl HouseArrest<'_> {
    /// Creates a new house arrest service from a lockdown service
    /// # Arguments
    /// * `device` - The device to create the sevice with
    /// * `service` - The lockdown service to connect on
    /// # Returns
    /// A struct containing the handle to the service
    ///
    /// ***Verified:*** False
    pub fn new(device: &Device, service: &LockdowndService) -> Result<Self, HouseArrestError> {
        let mut pointer = std::ptr::null_mut();
        let result = unsafe {
            unsafe_bindings::house_arrest_client_new(device.pointer, service.pointer, &mut pointer)
        }
        .into();

        if result != HouseArrestError::Success {
            return Err(result);
        }

        Ok(HouseArrest {
            pointer,
            phantom: std::marker::PhantomData,
        })
    }

    /// Starts a new service with house arrest
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
    ) -> Result<Self, HouseArrestError> {
        let mut pointer = std::ptr::null_mut();
        let label_c_string = CString::new(label.into()).unwrap();
        let result = unsafe {
            unsafe_bindings::house_arrest_client_start_service(
                device.pointer,
                &mut pointer,
                label_c_string.as_ptr(),
            )
        }
        .into();

        if result != HouseArrestError::Success {
            return Err(result);
        }

        Ok(HouseArrest {
            pointer,
            phantom: std::marker::PhantomData,
        })
    }

    /// Send a request to the house arrest service
    /// # Arguments
    /// * `request` - A plist containing the request
    /// # Returns
    /// A plist containing the result of the request
    ///
    /// ***Verified:*** False
    pub fn send_request(&self, request: Plist) -> Result<Plist, HouseArrestError> {
        let result = unsafe {
            unsafe_bindings::house_arrest_send_request(self.pointer, request.get_pointer())
        }
        .into();

        if result != HouseArrestError::Success {
            return Err(result);
        }

        // Get result
        let mut plist_t = std::ptr::null_mut();
        let result =
            unsafe { unsafe_bindings::house_arrest_get_result(self.pointer, &mut plist_t) }.into();

        if result != HouseArrestError::Success {
            return Err(result);
        }

        Ok(plist_t.into())
    }

    /// Send a command to house arrest
    /// # Arguments
    /// * `command` - The command to send. Currently, only VendContainer and VendDocuments are known.
    /// * `app_id` - The bundle identifier to pass along with the command
    /// # Returns
    /// A plist containing the result of the request
    ///
    /// ***Verified:*** False
    pub fn send_command(
        &self,
        command: impl Into<String>,
        app_id: impl Into<String>,
    ) -> Result<Plist, HouseArrestError> {
        let command_c_string = CString::new(command.into()).unwrap();
        let app_id_c_string = CString::new(app_id.into()).unwrap();

        let result = unsafe {
            unsafe_bindings::house_arrest_send_command(
                self.pointer,
                command_c_string.as_ptr(),
                app_id_c_string.as_ptr(),
            )
        }
        .into();

        if result != HouseArrestError::Success {
            return Err(result);
        }

        // Get result
        let mut plist_t = std::ptr::null_mut();
        let result =
            unsafe { unsafe_bindings::house_arrest_get_result(self.pointer, &mut plist_t) }.into();

        if result != HouseArrestError::Success {
            return Err(result);
        }

        Ok(plist_t.into())
    }
}

impl Drop for HouseArrest<'_> {
    fn drop(&mut self) {
        unsafe {
            unsafe_bindings::house_arrest_client_free(self.pointer);
        }
    }
}
