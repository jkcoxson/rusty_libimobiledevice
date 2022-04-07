// jkcoxson

use std::os::raw::c_char;

use crate::{bindings as unsafe_bindings, idevice::Device, lockdownd::LockdowndService, error::ServiceError};

pub struct ServiceClient<'a> {
    pub(crate) pointer: unsafe_bindings::service_client_t,
    phantom: std::marker::PhantomData<&'a Device>,
}

impl ServiceClient<'_> {
    pub fn new(device: &Device, descriptor: LockdowndService) -> Result<Self, ServiceError> {
        let mut pointer = std::ptr::null_mut();
        let result = unsafe {
            unsafe_bindings::service_client_new(
                device.pointer,
                descriptor.pointer,
                &mut pointer,
            )
        }.into();

        if result != ServiceError::Success {
            return Err(result);
        }

        Ok(ServiceClient {
            pointer,
            phantom: std::marker::PhantomData,
        })
    }

    pub fn factory_start_service(device: &Device, service_name: String, label: String) -> Result<(Self, i32), ServiceError> {
        let mut pointer = std::ptr::null_mut();
        let mut error_code = 0;
        let result = unsafe {
            unsafe_bindings::service_client_factory_start_service(
                device.pointer,
                service_name.as_ptr() as *const c_char,
                &mut pointer,
                label.as_ptr() as *const c_char,
                None,
                &mut error_code,

            )
        }.into();

        if result != ServiceError::Success {
            return Err(result);
        }

        let pointer = pointer as  unsafe_bindings::service_client_t;

        Ok((ServiceClient {
            pointer,
            phantom: std::marker::PhantomData,
        }, error_code))
    }

    pub fn send(&self, data: Vec<c_char>) -> Result<u32, ServiceError> {
        let mut sent = 0;
        let result = unsafe {
            unsafe_bindings::service_send(
                self.pointer,
                data.as_ptr() as *const c_char,
                data.len() as u32,
                &mut sent,
            )
        }.into();

        if result != ServiceError::Success {
            return Err(result);
        }

        Ok(sent)
    }

    pub fn receive(&self, size: u32) -> Result<Vec<c_char>, ServiceError> {
        let mut data = Vec::new();
        let mut received = 0;
        let result = unsafe {
            unsafe_bindings::service_receive(
                self.pointer,
                data.as_mut_ptr() as *mut c_char,
                size,
                &mut received,
            )
        }.into();

        if result != ServiceError::Success {
            return Err(result);
        }

        Ok(data)
    }

    pub fn receive_with_timeout(&self, size: u32, timeout: u32) -> Result<Vec<c_char>, ServiceError> {
        let mut data = Vec::new();
        let mut received = 0;
        let result = unsafe {
            unsafe_bindings::service_receive_with_timeout(
                self.pointer,
                data.as_mut_ptr() as *mut c_char,
                size,
                &mut received,
                timeout,
            )
        }.into();

        if result != ServiceError::Success {
            return Err(result);
        }

        Ok(data)
    }

    pub fn enable_ssl(&self) -> Result<(), ServiceError> {
        let result = unsafe {
            unsafe_bindings::service_enable_ssl(self.pointer)
        }.into();

        if result != ServiceError::Success {
            return Err(result);
        }

        Ok(())
    }

    pub fn disable_ssl(&self) -> Result<(), ServiceError> {
        let result = unsafe {
            unsafe_bindings::service_disable_ssl(self.pointer)
        }.into();

        if result != ServiceError::Success {
            return Err(result);
        }

        Ok(())
    }

    pub fn disable_bypass_ssl(&self, bypass: u8) -> Result<(), ServiceError> {
        let result = unsafe {
            unsafe_bindings::service_disable_bypass_ssl(self.pointer, bypass)
        }.into();

        if result != ServiceError::Success {
            return Err(result);
        }

        Ok(())
    }
}

impl Drop for ServiceClient<'_> {
    fn drop(&mut self) {
        unsafe {
            unsafe_bindings::service_client_free(self.pointer);
        }
    }
}
