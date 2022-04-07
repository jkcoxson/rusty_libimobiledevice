// Prepare to be boarded

use std::os::raw::c_char;

use crate::{
    bindings as unsafe_bindings, error::PreboardError, idevice::Device,
    lockdownd::LockdowndService, plist::Plist,
};

pub struct PreboardClient<'a> {
    pub(crate) pointer: unsafe_bindings::preboard_client_t,
    phantom: std::marker::PhantomData<&'a Device>,
}

impl PreboardClient<'_> {
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

    pub fn start_service(device: &Device, label: String) -> Result<Self, PreboardError> {
        let mut pointer = std::ptr::null_mut();
        let result = unsafe {
            unsafe_bindings::preboard_client_start_service(
                device.pointer,
                &mut pointer,
                label.as_ptr() as *const c_char,
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

    pub fn send(&self, data: Plist) -> Result<(), PreboardError> {
        let result = unsafe { unsafe_bindings::preboard_send(self.pointer, data.plist_t) }.into();

        if result != PreboardError::Success {
            return Err(result);
        }

        Ok(())
    }

    pub fn receive(&self) -> Result<Plist, PreboardError> {
        let mut plist = std::ptr::null_mut();
        let result = unsafe { unsafe_bindings::preboard_receive(self.pointer, &mut plist) }
            .into();

        if result != PreboardError::Success {
            return Err(result);
        }

        Ok(plist.into())
    }

    pub fn receive_with_timeout(&self, timeout: u32) -> Result<Plist, PreboardError> {
        let mut plist = std::ptr::null_mut();
        let result = unsafe {
            unsafe_bindings::preboard_receive_with_timeout(
                self.pointer,
                &mut plist,
                timeout,
            )
        }
        .into();

        if result != PreboardError::Success {
            return Err(result);
        }

        Ok(plist.into())
    }

    pub fn create_stashbag(&self, manifest: Option<Plist>) -> Result<(), PreboardError> {
        let result = unsafe {
            unsafe_bindings::preboard_create_stashbag(
                self.pointer,
                manifest.map(|p| p.plist_t).unwrap_or(std::ptr::null_mut()),
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

    pub fn commit_stashbag(&self, manifest: Option<Plist>) -> Result<(), PreboardError> {
        let result = unsafe {
            unsafe_bindings::preboard_commit_stashbag(
                self.pointer,
                manifest.map(|p| p.plist_t).unwrap_or(std::ptr::null_mut()),
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
