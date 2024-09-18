// Prepare to be boarded

use std::ffi::CString;

use crate::{
    bindings as unsafe_bindings, error::PreboardError, idevice::Device,
    services::lockdownd::LockdowndService,
};

use plist_plus::Plist;

/// A service that manages data at the first unlock screen after boot.
/// Prepare to be boarded!
pub struct PreboardClient<'a> {
    pub(crate) pointer: unsafe_bindings::preboard_client_t,
    phantom: std::marker::PhantomData<&'a Device>,
}

impl PreboardClient<'_> {
    /// Creates a preboard client from a lockdown service
    /// # Arguments
    /// * `device` - The device to connect to
    /// * `descriptor` - The lockdown service to connect on
    /// # Returns
    /// A struct containing the handle to the connection
    ///
    /// ***Verified:*** False
    pub fn new(device: &Device, descriptor: LockdowndService) -> Result<Self, PreboardError> {
        let mut pointer = std::ptr::null_mut();
        let result = unsafe {
            unsafe_bindings::preboard_client_new(device.pointer, descriptor.pointer, &mut pointer)
        }
        .into();

        if result != PreboardError::Success {
            return Err(result);
        }

        Ok(Self {
            pointer,
            phantom: std::marker::PhantomData,
        })
    }

    /// Starts a new connection and adds a preboard client to it
    /// # Arguments
    /// * `device` - The device to connect to
    /// * `label` - The label for the connection
    /// # Returns
    /// A struct containing the handle to the connection
    ///
    /// ***Verified:*** False
    pub fn start_service(device: &Device, label: impl Into<String>) -> Result<Self, PreboardError> {
        let mut pointer = std::ptr::null_mut();
        let label_c_string = CString::new(label.into()).unwrap();
        let result = unsafe {
            unsafe_bindings::preboard_client_start_service(
                device.pointer,
                &mut pointer,
                label_c_string.as_ptr(),
            )
        }
        .into();

        if result != PreboardError::Success {
            return Err(result);
        }

        Ok(Self {
            pointer,
            phantom: std::marker::PhantomData,
        })
    }

    /// Sends data to the client
    /// # Arguments
    /// * `data` - The data to send
    /// # Returns
    /// *none*
    ///
    /// ***Verified:*** False
    pub fn send(&self, data: Plist) -> Result<(), PreboardError> {
        let result =
            unsafe { unsafe_bindings::preboard_send(self.pointer, data.get_pointer()) }.into();

        if result != PreboardError::Success {
            return Err(result);
        }

        Ok(())
    }

    /// Receives data from the client
    /// Blocks until a full plist is sent
    /// # Arguments
    /// * `timeout` - How long to wait for the data. If 0, this will block indefinitely
    /// # Returns
    /// A plist containing the data
    ///
    /// ***Verified:*** False
    pub fn receive(&self, timeout: u32) -> Result<Plist, PreboardError> {
        let mut plist = std::ptr::null_mut();
        let result = if timeout == 0 {
            unsafe { unsafe_bindings::preboard_receive(self.pointer, &mut plist) }.into()
        } else {
            unsafe {
                unsafe_bindings::preboard_receive_with_timeout(self.pointer, &mut plist, timeout)
            }
            .into()
        };

        if result != PreboardError::Success {
            return Err(result);
        }

        Ok(plist.into())
    }

    /// Creates a stashbag on the device
    /// # Arguments
    /// * `manifest` - The options to use while creating the stashbag
    /// # Returns
    /// *none*
    pub fn create_stashbag(&self, manifest: Option<Plist>) -> Result<(), PreboardError> {
        let result = unsafe {
            unsafe_bindings::preboard_create_stashbag(
                self.pointer,
                manifest.map_or(std::ptr::null_mut(), |p| p.get_pointer()),
                None,
                std::ptr::null_mut(),
            )
        }
        .into();

        if result != PreboardError::Success {
            return Err(result);
        }

        Ok(())
    }

    /// Commands preboard to commit a previously created stashbag
    /// # Arguments
    /// * `manifest` - The manifest used for creating the stashbag
    /// # Returns
    /// *none*
    ///
    /// ***Verified:*** False
    pub fn commit_stashbag(&self, manifest: Option<Plist>) -> Result<(), PreboardError> {
        let result = unsafe {
            unsafe_bindings::preboard_commit_stashbag(
                self.pointer,
                manifest.map_or(std::ptr::null_mut(), |p| p.get_pointer()),
                None,
                std::ptr::null_mut(),
            )
        }
        .into();

        if result != PreboardError::Success {
            return Err(result);
        }

        Ok(())
    }
}

impl Drop for PreboardClient<'_> {
    fn drop(&mut self) {
        unsafe {
            unsafe_bindings::preboard_client_free(self.pointer);
        }
    }
}
