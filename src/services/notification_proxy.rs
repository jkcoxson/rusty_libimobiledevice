// jkcoxson

use std::ffi::CString;

use crate::bindings as unsafe_bindings;
use crate::error::NpError;
use crate::idevice::Device;
use crate::services::lockdownd::LockdowndService;

/// A service to proxy notifications to the device
pub struct NotificationProxyClient<'a> {
    pub(crate) pointer: unsafe_bindings::np_client_t,
    phantom: std::marker::PhantomData<&'a Device>,
}

impl NotificationProxyClient<'_> {
    /// Creates a new notification proxy from a lockdown service
    /// # Arguments
    /// * `device` - The device to connect to
    /// * `descriptor` - The lockdown service to connect on
    /// # Returns
    /// A struct containing the handle to the connection
    ///
    /// ***Verified:*** False
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

    /// Starts a new connection and adds a notification proxy to it
    /// # Arguments
    /// * `device` - The device to connect to
    /// * `label` - The label for the connection
    /// # Returns
    /// A struct containing the handle to the connection
    ///
    /// ***Verified:*** False
    pub fn start_service(device: &Device, label: impl Into<String>) -> Result<Self, NpError> {
        let label_c_string = CString::new(label.into()).unwrap();

        let mut pointer = std::ptr::null_mut();
        let result = unsafe {
            unsafe_bindings::np_client_start_service(
                device.pointer,
                &mut pointer,
                label_c_string.as_ptr(),
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

    /// Sends a notification to the device
    /// # Arguments
    /// * `notification` - The contents of the notification
    /// # Returns
    /// *none*
    ///
    /// ***Verified:*** False
    pub fn post_notification(&self, notification: &str) -> Result<(), NpError> {
        let notification_c_string = CString::new(notification).unwrap();
        let result = unsafe {
            unsafe_bindings::np_post_notification(self.pointer, notification_c_string.as_ptr())
        }
        .into();

        if result != NpError::Success {
            return Err(result);
        }

        Ok(())
    }

    /// Tells the proxy to send a notification when an event occurs
    /// # Arguments
    /// * `notification` - The contents of the notification
    /// # Returns
    /// *none*
    ///
    /// ***Verified:*** False
    pub fn observe_notification(&self, notification: &str) -> Result<(), NpError> {
        let notification_c_string = CString::new(notification).unwrap();
        let result = unsafe {
            unsafe_bindings::np_observe_notification(self.pointer, notification_c_string.as_ptr())
        }
        .into();

        if result != NpError::Success {
            return Err(result);
        }

        Ok(())
    }

    /// Tells the proxy to send notifications when an event occurs
    /// # Arguments
    /// * `notifications` - The contents of the notifications
    /// # Returns
    /// *none*
    ///
    /// ***Verified:*** False
    pub fn observe_notifications(&self, notifications: Vec<&str>) -> Result<(), NpError> {
        let mut not_c_strings = Vec::with_capacity(notifications.len());
        let mut not_ptrs = Vec::with_capacity(not_c_strings.len() + 1);

        for notification in notifications {
            not_c_strings.push(CString::new(notification).unwrap());
            not_ptrs.push(not_c_strings.last().unwrap().as_ptr());

        }
        not_ptrs.push(std::ptr::null());

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
