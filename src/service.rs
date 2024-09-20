// jkcoxson

use std::ffi::CString;
use std::os::raw::c_char;

use crate::services::lockdownd::LockdowndService;
use crate::{bindings as unsafe_bindings, error::ServiceError, idevice::Device};

pub struct ServiceClient<'a> {
    pub(crate) pointer: unsafe_bindings::service_client_t,
    phantom: std::marker::PhantomData<&'a Device>,
}

impl ServiceClient<'_> {
    /// Creates a new service on the device
    /// This is useful for services that don't have abstractions and need to be handled manually
    /// # Arguments
    /// * `device` - The device to create the service with
    /// * `descriptor` - The lockdown service to jump off of
    ///
    /// ***Verified:*** False
    pub fn new(device: &Device, descriptor: LockdowndService) -> Result<Self, ServiceError> {
        let mut pointer = std::ptr::null_mut();
        let result = unsafe {
            unsafe_bindings::service_client_new(device.pointer, descriptor.pointer, &mut pointer)
        }
        .into();

        if result != ServiceError::Success {
            return Err(result);
        }

        Ok(ServiceClient {
            pointer,
            phantom: std::marker::PhantomData,
        })
    }

    /// Starts a factory service on the device
    /// # Arguments
    /// * `device` - The device to create the service with
    /// * `service_name` - The name of the service to start
    /// * `label` - The label to use for the service
    ///
    /// ***Verified:*** False
    pub fn factory_start_service(
        device: &Device,
        service_name: impl Into<String>,
        label: impl Into<String>,
    ) -> Result<(Self, i32), ServiceError> {
        let mut pointer = std::ptr::null_mut();
        let service_name_c_string = CString::new(service_name.into()).unwrap();
        let label_c_string = CString::new(label.into()).unwrap();
        let mut error_code = 0;
        let result = unsafe {
            unsafe_bindings::service_client_factory_start_service(
                device.pointer,
                service_name_c_string.as_ptr(),
                &mut pointer,
                label_c_string.as_ptr(),
                None,
                &mut error_code,
            )
        }
        .into();

        if result != ServiceError::Success {
            return Err(result);
        }

        let pointer = pointer as unsafe_bindings::service_client_t;

        Ok((
            ServiceClient {
                pointer,
                phantom: std::marker::PhantomData,
            },
            error_code,
        ))
    }

    /// Send data to the service
    /// # Arguments
    /// * `data` - The data to send
    /// # Returns
    /// The number of bytes sent
    ///
    /// ***Verified:*** False
    pub fn send(&self, data: Vec<c_char>) -> Result<u32, ServiceError> {
        let mut sent = 0;
        let result = unsafe {
            unsafe_bindings::service_send(
                self.pointer,
                data.as_ptr() as *const c_char,
                data.len() as u32,
                &mut sent,
            )
        }
        .into();

        if result != ServiceError::Success {
            return Err(result);
        }

        Ok(sent)
    }

    /// Receives data from the service, blocking until the amount of data is received
    /// # Arguments
    /// * `size` - The size of the buffer to receive
    /// # Returns
    /// The received data as a vector of bytes
    ///
    /// ***Verified:*** False
    pub fn receive(&self, size: u32) -> Result<Vec<c_char>, ServiceError> {
        let mut data = vec![0 as c_char; size as usize];
        let mut received = 0;
        let result = unsafe {
            unsafe_bindings::service_receive(self.pointer, data.as_mut_ptr(), size, &mut received)
        }
        .into();

        if result != ServiceError::Success {
            return Err(result);
        }

        data.truncate(received as usize);

        Ok(data)
    }

    /// Receives data from the service, blocking until the amount of data is received or the timeout is reached
    /// # Arguments
    /// * `size` - The size of the buffer to receive
    /// * `timeout` - The timeout in milliseconds
    /// # Returns
    /// The received data as a vector of bytes
    ///
    /// ***Verified:*** False
    pub fn receive_with_timeout(
        &self,
        size: u32,
        timeout: u32,
    ) -> Result<Vec<c_char>, ServiceError> {
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
        }
        .into();

        if result != ServiceError::Success {
            return Err(result);
        }

        Ok(data)
    }

    /// Toggles on the SSL for the communication between device and service
    ///
    /// ***Verified:*** False
    pub fn enable_ssl(&self) -> Result<(), ServiceError> {
        let result = unsafe { unsafe_bindings::service_enable_ssl(self.pointer) }.into();

        if result != ServiceError::Success {
            return Err(result);
        }

        Ok(())
    }

    /// Toggles off the SSL for the communication between device and service
    ///
    /// ***Verified:*** False
    pub fn disable_ssl(&self) -> Result<(), ServiceError> {
        let result = unsafe { unsafe_bindings::service_disable_ssl(self.pointer) }.into();

        if result != ServiceError::Success {
            return Err(result);
        }

        Ok(())
    }

    /// A hack for bypassing SSL
    ///
    /// ***Verified:*** False
    pub fn disable_bypass_ssl(&self, bypass: u8) -> Result<(), ServiceError> {
        let result =
            unsafe { unsafe_bindings::service_disable_bypass_ssl(self.pointer, bypass) }.into();

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
