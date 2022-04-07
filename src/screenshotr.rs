// jkcoxson

use std::os::raw::c_char;

use crate::{
    bindings as unsafe_bindings, error::ScreenshotrError, idevice::Device,
    lockdownd::LockdowndService,
};

pub struct ScreenshotrClient<'a> {
    pub(crate) pointer: unsafe_bindings::screenshotr_client_t,
    phantom: std::marker::PhantomData<&'a Device>,
}

impl ScreenshotrClient<'_> {
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

    pub fn start_service(device: &Device, label: String) -> Result<Self, ScreenshotrError> {
        let mut pointer = std::ptr::null_mut();
        let result = unsafe {
            unsafe_bindings::screenshotr_client_start_service(
                device.pointer,
                &mut pointer,
                label.as_ptr() as *const c_char,
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

    pub fn take_screenshot(&self) -> Result<Vec<c_char>, ScreenshotrError> {
        let mut data = std::ptr::null_mut();
        let mut size = 0;
        let result = unsafe {
            unsafe_bindings::screenshotr_take_screenshot(self.pointer, &mut data, &mut size)
        }
        .into();

        if result != ScreenshotrError::Success {
            return Err(result);
        }

        let mut buffer = Vec::with_capacity(size as usize);
        unsafe {
            std::ptr::copy_nonoverlapping(data, buffer.as_mut_ptr(), size as usize);
        }

        Ok(buffer)
    }
}

impl Drop for ScreenshotrClient<'_> {
    fn drop(&mut self) {
        unsafe {
            unsafe_bindings::screenshotr_client_free(self.pointer);
        }
    }
}
