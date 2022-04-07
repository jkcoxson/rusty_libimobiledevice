// jkcoxson

use std::os::raw::c_char;

use crate::{bindings as unsafe_bindings, error::RestoredError, idevice::Device, plist::Plist};

pub struct RestoredClient<'a> {
    pub(crate) pointer: unsafe_bindings::restored_client_t,
    phantom: std::marker::PhantomData<&'a Device>,
}

impl RestoredClient<'_> {
    pub fn new(device: &Device, label: String) -> Result<Self, RestoredError> {
        let mut pointer = unsafe { std::mem::zeroed() };
        let result = unsafe {
            unsafe_bindings::restored_client_new(
                device.pointer,
                &mut pointer,
                label.as_ptr() as *const i8,
            )
        }
        .into();
        if result != RestoredError::Success {
            return Err(result);
        }

        Ok(Self {
            pointer,
            phantom: std::marker::PhantomData,
        })
    }

    pub fn query_type(&self) -> Result<(String, u64), RestoredError> {
        let mut type_ = std::ptr::null_mut();
        let mut version = 0;
        let result =
            unsafe { unsafe_bindings::restored_query_type(self.pointer, &mut type_, &mut version) }
                .into();
        if result != RestoredError::Success {
            return Err(result);
        }

        let type_ = unsafe {
            std::ffi::CStr::from_ptr(type_)
                .to_string_lossy()
                .into_owned()
        };
        Ok((type_, version))
    }

    pub fn query_value(&self, key: String) -> Result<Plist, RestoredError> {
        let mut value = std::ptr::null_mut();
        let result = unsafe {
            unsafe_bindings::restored_query_value(
                self.pointer,
                key.as_ptr() as *const i8,
                &mut value,
            )
        }
        .into();
        if result != RestoredError::Success {
            return Err(result);
        }
        
        Ok(value.into())
    }

    pub fn get_value(&self, key: String) -> Result<Plist, RestoredError> {
        let mut value = std::ptr::null_mut();
        let result = unsafe {
            unsafe_bindings::restored_get_value(
                self.pointer,
                key.as_ptr() as *const i8,
                &mut value,
            )
        }
        .into();
        if result != RestoredError::Success {
            return Err(result);
        }

        Ok(value.into())
    }

    pub fn send(&self, data: Plist) -> Result<(), RestoredError> {
        let result = unsafe {
            unsafe_bindings::restored_send(
                self.pointer,
                data.plist_t,
            )
        }
        .into();
        if result != RestoredError::Success {
            return Err(result);
        }

        Ok(())
    }

    pub fn receive(&self) -> Result<Plist, RestoredError> {
        let mut value = std::ptr::null_mut();
        let result = unsafe {
            unsafe_bindings::restored_receive(
                self.pointer,
                &mut value,
            )
        }
        .into();
        if result != RestoredError::Success {
            return Err(result);
        }

        Ok(value.into())
    }

    pub fn goodbye(&self) -> Result<(), RestoredError> {
        let result = unsafe { unsafe_bindings::restored_goodbye(self.pointer) }.into();
        if result != RestoredError::Success {
            return Err(result);
        }

        Ok(())
    }

    pub fn start_restore(&self, options: Plist, version: u64) -> Result<(), RestoredError> {
        let result = unsafe {
            unsafe_bindings::restored_start_restore(
                self.pointer,
                options.plist_t,
                version,
            )
        }
        .into();
        if result != RestoredError::Success {
            return Err(result);
        }

        Ok(())
    }

    pub fn reboot(&self) -> Result<(), RestoredError> {
        let result = unsafe { unsafe_bindings::restored_reboot(self.pointer) }.into();
        if result != RestoredError::Success {
            return Err(result);
        }

        Ok(())
    }

    pub fn set_label(&self, label: String)  {
        unsafe {
            unsafe_bindings::restored_client_set_label(
                self.pointer,
                label.as_ptr() as *const c_char,
            )
        };
    }
}

impl Drop for RestoredClient<'_> {
    fn drop(&mut self) {
        unsafe {
            unsafe_bindings::restored_client_free(self.pointer);
        }
    }
}
