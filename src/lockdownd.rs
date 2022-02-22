// jkcoxson

pub use crate::bindings as unsafe_bindings;
use crate::error::LockdowndError;
use crate::libimobiledevice::Device;
use crate::plist::Plist;

pub struct LockdowndClient {
    pointer: unsafe_bindings::lockdownd_client_t,
    pub label: String,
}

impl LockdowndClient {
    pub fn new(device: &mut Device, label: String) -> Result<Self, LockdowndError> {
        let mut client: unsafe_bindings::lockdownd_client_t = unsafe { std::mem::zeroed() };
        let client_ptr: *mut unsafe_bindings::lockdownd_client_t = &mut client;

        let label_c_str = std::ffi::CString::new(label.clone()).unwrap();

        let result = unsafe {
            unsafe_bindings::lockdownd_client_new_with_handshake(
                device.device,
                client_ptr,
                label_c_str.as_ptr(),
            )
        }.into();

        if result != LockdowndError::Success {
            return Err(result);
        }

        Ok(LockdowndClient { pointer: client, label: label })
    }

    pub fn get_value(&self, key: String, domain: String) -> Result<Plist, LockdowndError> {
        let domain_c_str = std::ffi::CString::new(domain.clone()).unwrap();
        let domain_c_str = if domain == "".to_string() {
            std::ptr::null()
        } else {
            domain_c_str.as_ptr()
        };
        let key_c_str = std::ffi::CString::new(key.clone()).unwrap();
        let key_c_str = if key == "".to_string() {
            std::ptr::null()
        } else {
            key_c_str.as_ptr()
        };
        
        let mut value: unsafe_bindings::plist_t = unsafe { std::mem::zeroed() };

        let result = unsafe {
            unsafe_bindings::lockdownd_get_value(
                self.pointer,
                domain_c_str,
                key_c_str,
                &mut value,
            )
        }.into();

        if result != LockdowndError::Success {
            return Err(result);
        }

        Ok(value.into())
    }
}

impl Drop for LockdowndClient {
    fn drop(&mut self) {
        unsafe {
            unsafe_bindings::lockdownd_client_free(self.pointer);
        }
    }
}