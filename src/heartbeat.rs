// jkcoxson

use std::ffi::CString;

use crate::{
    bindings as unsafe_bindings, error::HeartbeatError, libimobiledevice::Device, plist::Plist,
};

pub struct HeartbeatClient {
    pub(crate) pointer: unsafe_bindings::heartbeat_client_t,
    // phantom: std::marker::PhantomData<&'a Device>,
}

unsafe impl Send for HeartbeatClient {}
unsafe impl Sync for HeartbeatClient {}

impl HeartbeatClient {
    pub fn new(device: &Device, label: String) -> Result<Self, HeartbeatError> {
        let mut pointer = unsafe { std::mem::zeroed() };
        let label_c_str = CString::new(label).unwrap();
        let label_ptr = label_c_str.as_ptr();
        let result = unsafe {
            unsafe_bindings::heartbeat_client_start_service(device.pointer, &mut pointer, label_ptr)
        }
        .into();
        if result != HeartbeatError::Success {
            return Err(result);
        }
        Ok(Self {
            pointer: pointer,
            // phantom: std::marker::PhantomData,
        })
    }

    pub fn send(&self, message: Plist) -> Result<(), HeartbeatError> {
        let result =
            unsafe { unsafe_bindings::heartbeat_send(self.pointer, message.plist_t) }.into();
        if result != HeartbeatError::Success {
            return Err(result);
        }
        Ok(())
    }

    pub fn receive(&self) -> Result<Plist, HeartbeatError> {
        let mut plist_ptr = unsafe { std::mem::zeroed() };
        let result =
            unsafe { unsafe_bindings::heartbeat_receive(self.pointer, &mut plist_ptr) }.into();
        if result != HeartbeatError::Success {
            return Err(result);
        }
        Ok(plist_ptr.into())
    }

    pub fn receive_with_timeout(&self, timeout: u32) -> Result<Plist, HeartbeatError> {
        let mut plist_ptr = unsafe { std::mem::zeroed() };
        let result = unsafe {
            unsafe_bindings::heartbeat_receive_with_timeout(self.pointer, &mut plist_ptr, timeout)
        }
        .into();
        if result != HeartbeatError::Success {
            return Err(result);
        }
        Ok(plist_ptr.into())
    }
}

impl Drop for HeartbeatClient {
    fn drop(&mut self) {
        unsafe {
            unsafe_bindings::heartbeat_client_free(self.pointer);
        }
    }
}
