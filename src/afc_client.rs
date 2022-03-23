// jkcoxson

use std::ffi::CStr;

use crate::{bindings as unsafe_bindings, error::AfcError, libimobiledevice::Device};

pub struct AfcClient<'a> {
    pub(crate) pointer: unsafe_bindings::afc_client_t,
    phantom: std::marker::PhantomData<&'a Device>,
}

pub struct LockdowndService<'a> {
    pub(crate) pointer: unsafe_bindings::lockdownd_service_descriptor_t,
    pub port: u16,
    pub ssl_enabled: bool,
    pub identifier: String,
    phantom: std::marker::PhantomData<&'a AfcClient<'a>>,
}

impl AfcClient<'_> {
    pub fn new(device: &Device) -> Result<(Self, LockdowndService), String> {
        let mut pointer = unsafe { std::mem::zeroed() };
        let mut client_pointer = unsafe { std::mem::zeroed() };
        let result = unsafe {
            unsafe_bindings::afc_client_new(device.pointer, &mut pointer, &mut client_pointer)
        };
        if result != 0 {
            return Err(format!("afc_client_new failed: {}", result));
        }
        Ok((
            AfcClient {
                pointer: client_pointer,
                phantom: std::marker::PhantomData,
            },
            LockdowndService {
                pointer: &mut pointer,
                port: pointer.port,
                ssl_enabled: match pointer.ssl_enabled {
                    0 => false,
                    _ => true,
                },
                identifier: unsafe { CStr::from_ptr(pointer.identifier) }
                    .to_string_lossy()
                    .into_owned(),
                phantom: std::marker::PhantomData,
            },
        ))
    }

    pub fn start_service(&mut self, device: &Device, service_name: &str) -> Result<(), AfcError> {
        let result = unsafe {
            unsafe_bindings::afc_client_start_service(
                device.pointer,
                &mut self.pointer,
                service_name.as_ptr() as *const i8,
            )
        }
        .into();
        if result != AfcError::Success {
            return Err(result);
        }
        Ok(())
    }
}

impl Drop for AfcClient<'_> {
    fn drop(&mut self) {
        unsafe {
            unsafe_bindings::afc_client_free(self.pointer);
        }
    }
}

impl Drop for LockdowndService<'_> {
    fn drop(&mut self) {
        unsafe {
            unsafe_bindings::lockdownd_service_descriptor_free(self.pointer);
        }
    }
}
