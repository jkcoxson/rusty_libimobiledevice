// jkcoxson

use std::os::raw::c_char;

use crate::{
    bindings as unsafe_bindings, error::WebInspectorError, idevice::Device,
    services::lockdownd::LockdowndService
};

use plist_plus::Plist;

pub struct WebInspectorClient<'a> {
    pub(crate) pointer: unsafe_bindings::webinspector_client_t,
    phantom: std::marker::PhantomData<&'a Device>,
}

impl WebInspectorClient<'_> {
    pub fn new(device: &Device, descriptor: LockdowndService) -> Result<Self, WebInspectorError> {
        let mut pointer = std::ptr::null_mut();

        let result = unsafe {
            unsafe_bindings::webinspector_client_new(
                device.pointer,
                descriptor.pointer,
                &mut pointer,
            )
        }
        .into();

        if result != WebInspectorError::Success {
            return Err(result);
        }

        Ok(WebInspectorClient {
            pointer,
            phantom: std::marker::PhantomData,
        })
    }

    pub fn start_service(device: &Device, label: String) -> Result<Self, WebInspectorError> {
        let mut pointer = std::ptr::null_mut();

        let result = unsafe {
            unsafe_bindings::webinspector_client_start_service(
                device.pointer,
                &mut pointer,
                label.as_ptr() as *const c_char,
            )
        }
        .into();

        if result != WebInspectorError::Success {
            return Err(result);
        }

        Ok(WebInspectorClient {
            pointer,
            phantom: std::marker::PhantomData,
        })
    }

    pub fn send(&self, data: Plist) -> Result<(), WebInspectorError> {
        let result =
            unsafe { unsafe_bindings::webinspector_send(self.pointer, data.get_pointer()) }.into();

        if result != WebInspectorError::Success {
            return Err(result);
        }

        Ok(())
    }

    pub fn receive(&self) -> Result<Plist, WebInspectorError> {
        let mut plist = std::ptr::null_mut();

        let result =
            unsafe { unsafe_bindings::webinspector_receive(self.pointer, &mut plist) }.into();

        if result != WebInspectorError::Success {
            return Err(result);
        }

        Ok(plist.into())
    }

    pub fn receive_with_timeout(&self, timeout: u32) -> Result<Plist, WebInspectorError> {
        let mut plist = std::ptr::null_mut();

        let result = unsafe {
            unsafe_bindings::webinspector_receive_with_timeout(
                self.pointer,
                &mut plist,
                timeout,
            )
        }
        .into();

        if result != WebInspectorError::Success {
            return Err(result);
        }

        Ok(plist.into())
    }
}

impl Drop for WebInspectorClient<'_> {
    fn drop(&mut self) {
        unsafe {
            unsafe_bindings::webinspector_client_free(self.pointer);
        }
    }
}
