// jkcoxson

use std::ffi::CString;

use crate::{
    bindings as unsafe_bindings, error::WebInspectorError, idevice::Device,
    services::lockdownd::LockdowndService,
};

use plist_plus::Plist;

/// First used on MacOS, this service is used to inspect the JavaScript and HTML of a site running on the device
pub struct WebInspectorClient<'a> {
    pub(crate) pointer: unsafe_bindings::webinspector_client_t,
    phantom: std::marker::PhantomData<&'a Device>,
}

impl WebInspectorClient<'_> {
    /// Creates a preboard client from a web inspector service
    /// # Arguments
    /// * `device` - The device to connect to
    /// * `descriptor` - The lockdown service to connect on
    /// # Returns
    /// A struct containing the handle to the connection
    ///
    /// ***Verified:*** False
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

    /// Starts a new connection and adds a web inspector to it
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
    ) -> Result<Self, WebInspectorError> {
        let mut pointer = std::ptr::null_mut();
        let label_c_string = CString::new(label.into()).unwrap();

        let result = unsafe {
            unsafe_bindings::webinspector_client_start_service(
                device.pointer,
                &mut pointer,
                label_c_string.as_ptr(),
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

    /// Sends data to the web inspector
    /// # Arguments
    /// * `data` - The data to send
    /// # Returns
    /// *none*
    ///
    /// ***Verified:*** False
    pub fn send(&self, data: Plist) -> Result<(), WebInspectorError> {
        let result =
            unsafe { unsafe_bindings::webinspector_send(self.pointer, data.get_pointer()) }.into();

        if result != WebInspectorError::Success {
            return Err(result);
        }

        Ok(())
    }

    /// Receives a message from the web inspector
    /// # Arguments
    /// * `timeout` - The time to wait for a message. Pass 0 to wait indefinitely.
    /// # Returns
    /// The message received
    ///
    /// ***Verified:*** False
    pub fn receive(&self, timeout: u32) -> Result<Plist, WebInspectorError> {
        let mut plist = std::ptr::null_mut();

        let result = if timeout == 0 {
            unsafe { unsafe_bindings::webinspector_receive(self.pointer, &mut plist) }.into()
        } else {
            unsafe {
                unsafe_bindings::webinspector_receive_with_timeout(
                    self.pointer,
                    &mut plist,
                    timeout,
                )
            }
            .into()
        };

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
