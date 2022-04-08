// jkcoxson

use crate::{
    bindings as unsafe_bindings, error::MisagentError, idevice::Device,
    services::lockdownd::LockdowndService, plist::Plist,
};

pub struct MisagentClient<'a> {
    pub(crate) pointer: unsafe_bindings::misagent_client_t,
    phantom: std::marker::PhantomData<&'a Device>,
}

impl MisagentClient<'_> {
    pub fn new(device: &Device, descriptor: LockdowndService) -> Result<Self, MisagentError> {
        let mut pointer = unsafe { std::mem::zeroed() };
        let result = unsafe {
            unsafe_bindings::misagent_client_new(device.pointer, descriptor.pointer, &mut pointer)
        }
        .into();
        if result != MisagentError::Success {
            return Err(result);
        }

        Ok(MisagentClient {
            pointer,
            phantom: std::marker::PhantomData,
        })
    }

    pub fn start_service(device: &Device, label: String) -> Result<Self, MisagentError> {
        let mut pointer = unsafe { std::mem::zeroed() };
        let result = unsafe {
            unsafe_bindings::misagent_client_start_service(
                device.pointer,
                &mut pointer,
                label.as_ptr() as *const i8,
            )
        }
        .into();
        if result != MisagentError::Success {
            return Err(result);
        }

        Ok(MisagentClient {
            pointer,
            phantom: std::marker::PhantomData,
        })
    }

    pub fn install(&self, profile: Plist) -> Result<(), MisagentError> {
        let result =
            unsafe { unsafe_bindings::misagent_install(self.pointer, profile.plist_t) }.into();
        if result != MisagentError::Success {
            return Err(result);
        }

        Ok(())
    }

    pub fn copy(&self) -> Result<Plist, MisagentError> {
        let mut plist = unsafe { std::mem::zeroed() };
        let result = unsafe { unsafe_bindings::misagent_copy(self.pointer, &mut plist) }.into();
        if result != MisagentError::Success {
            return Err(result);
        }

        Ok(plist.into())
    }

    pub fn copy_all(&self) -> Result<Plist, MisagentError> {
        let mut plist = unsafe { std::mem::zeroed() };
        let result = unsafe { unsafe_bindings::misagent_copy_all(self.pointer, &mut plist) }.into();
        if result != MisagentError::Success {
            return Err(result);
        }

        Ok(plist.into())
    }

    pub fn remove(&self, id: String) -> Result<(), MisagentError> {
        let result =
            unsafe { unsafe_bindings::misagent_remove(self.pointer, id.as_ptr() as *const i8) }
                .into();
        if result != MisagentError::Success {
            return Err(result);
        }

        Ok(())
    }

    pub fn get_status_code(&self) -> Result<i32, MisagentError> {
        let result = unsafe {
            unsafe_bindings::misagent_get_status_code(self.pointer)
        };

        Ok(result)
    }
}

impl Drop for MisagentClient<'_> {
    fn drop(&mut self) {
        unsafe {
            unsafe_bindings::misagent_client_free(self.pointer);
        }
    }
}
