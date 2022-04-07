// jkcoxson

use crate::bindings as unsafe_bindings;
use crate::error::NpError;
use crate::idevice::Device;
use crate::lockdownd::LockdowndService;

pub struct NotificationProxyClient<'a> {
    pub(crate) pointer: unsafe_bindings::np_client_t,
    phantom: std::marker::PhantomData<&'a Device>,
}

impl NotificationProxyClient<'_> {
    pub fn new(device: &Device, descriptor: LockdowndService) -> Result<Self, NpError> {
        let mut pointer = std::ptr::null_mut();
        let result = unsafe {
            unsafe_bindings::np_client_new(device.pointer, descriptor.pointer, &mut pointer)
        }
        .into();

        if result != NpError::Success {
            return Err(result);
        }

        Ok(Self {
            pointer,
            phantom: std::marker::PhantomData,
        })
    }

    pub fn start_service(device: &Device, label: String) -> Result<Self, NpError> {
        let mut pointer = std::ptr::null_mut();
        let result = unsafe {
            unsafe_bindings::np_client_start_service(
                device.pointer,
                &mut pointer,
                label.as_ptr() as *const std::os::raw::c_char,
            )
        }
        .into();

        if result != NpError::Success {
            return Err(result);
        }

        Ok(Self {
            pointer,
            phantom: std::marker::PhantomData,
        })
    }

    pub fn post_notification(&self, notification: &str) -> Result<(), NpError> {
        let result = unsafe {
            unsafe_bindings::np_post_notification(
                self.pointer,
                notification.as_ptr() as *const std::os::raw::c_char,
            )
        }
        .into();

        if result != NpError::Success {
            return Err(result);
        }

        Ok(())
    }

    pub fn observe_notification(&self, notification: &str) -> Result<(), NpError> {
        let result = unsafe {
            unsafe_bindings::np_observe_notification(
                self.pointer,
                notification.as_ptr() as *const std::os::raw::c_char,
            )
        }
        .into();

        if result != NpError::Success {
            return Err(result);
        }

        Ok(())
    }

    pub fn observe_notifications(&self, notifications: Vec<&str>) -> Result<(), NpError> {
        let mut not_ptrs = Vec::new();
        for notification in notifications {
            not_ptrs.push(notification.as_ptr() as *const std::os::raw::c_char);
        }

        let result = unsafe {
            unsafe_bindings::np_observe_notifications(self.pointer, not_ptrs.as_mut_ptr())
        }
        .into();

        if result != NpError::Success {
            return Err(result);
        }

        Ok(())
    }
}

impl Drop for NotificationProxyClient<'_> {
    fn drop(&mut self) {
        unsafe {
            unsafe_bindings::np_client_free(self.pointer);
        }
    }
}
