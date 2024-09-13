// jkcoxson

use std::ffi::CString;

use log::info;

use crate::{
    bindings as unsafe_bindings, error::ScreenshotrError, idevice::Device,
    services::lockdownd::LockdowndService,
};

pub struct ScreenshotrClient<'a> {
    pub(crate) pointer: unsafe_bindings::screenshotr_client_t,
    phantom: std::marker::PhantomData<&'a Device>,
}

impl ScreenshotrClient<'_> {
    /// Creates a preboard client from a screenshotr service
    /// # Arguments
    /// * `device` - The device to connect to
    /// * `descriptor` - The lockdown service to connect on
    /// # Returns
    /// A struct containing the handle to the connection
    ///
    /// ***Verified:*** False
    pub fn new(device: &Device, descriptor: LockdowndService) -> Result<Self, ScreenshotrError> {
        let mut pointer = std::ptr::null_mut();
        let result = unsafe {
            unsafe_bindings::screenshotr_client_new(
                device.pointer,
                descriptor.pointer,
                &mut pointer,
            )
        }
        .into();

        if result != ScreenshotrError::Success {
            return Err(result);
        }

        Ok(ScreenshotrClient {
            pointer,
            phantom: std::marker::PhantomData,
        })
    }

    /// Starts a new connection and adds a screenshotr client to it
    /// # Arguments
    /// * `device` - The device to connect to
    /// * `label` - The label for the connection
    /// # Returns
    /// A struct containing the handle to the connection
    ///
    /// ***Verified:*** False
    pub fn start_service(
        device: &Device,
        label: impl Into<String>,
    ) -> Result<Self, ScreenshotrError> {
        let mut pointer = std::ptr::null_mut();
        let label_c_string = CString::new(label.into()).unwrap();
        let result = unsafe {
            unsafe_bindings::screenshotr_client_start_service(
                device.pointer,
                &mut pointer,
                label_c_string.as_ptr(),
            )
        }
        .into();

        if result != ScreenshotrError::Success {
            return Err(result);
        }

        Ok(ScreenshotrClient {
            pointer,
            phantom: std::marker::PhantomData,
        })
    }

    /// Takes a screenshot on the device
    /// # Arguments
    /// *none*
    /// # Returns
    /// A vector of bytes containing a .png
    ///
    /// ***Verified:*** False
    pub fn take_screenshot(&self) -> Result<Vec<u8>, ScreenshotrError> {
        let mut data = unsafe { std::mem::zeroed() };
        let mut size = 0;
        let result = unsafe {
            unsafe_bindings::screenshotr_take_screenshot(self.pointer, &mut data, &mut size)
        }
        .into();

        if result != ScreenshotrError::Success {
            return Err(result);
        }

        info!("Screenshot size: {}", size);

        Ok(unsafe { std::vec::Vec::from_raw_parts(data as *mut u8, size as usize, size as usize) })
    }
}

impl Drop for ScreenshotrClient<'_> {
    fn drop(&mut self) {
        unsafe {
            unsafe_bindings::screenshotr_client_free(self.pointer);
        }
    }
}
