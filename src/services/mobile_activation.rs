// jkcoxson

use std::os::raw::c_char;

use crate::{
    bindings as unsafe_bindings, error::MobileActivationError, idevice::Device,
    services::lockdownd::LockdowndService
};

use plist_plus::Plist;

pub struct MobileActivationClient<'a> {
    pub(crate) pointer: unsafe_bindings::mobileactivation_client_t,
    phantom: std::marker::PhantomData<&'a Device>,
}

impl MobileActivationClient<'_> {
    pub fn new(
        device: &Device,
        descriptor: LockdowndService,
    ) -> Result<Self, MobileActivationError> {
        let mut client = unsafe { std::mem::zeroed() };

        let result = unsafe {
            unsafe_bindings::mobileactivation_client_new(
                device.pointer,
                descriptor.pointer,
                &mut client,
            )
        }
        .into();

        if result != MobileActivationError::Success {
            return Err(result);
        }

        Ok(MobileActivationClient {
            pointer: client,
            phantom: std::marker::PhantomData,
        })
    }

    pub fn start_service(device: &Device, label: String) -> Result<Self, MobileActivationError> {
        let mut client = unsafe { std::mem::zeroed() };

        let result = unsafe {
            unsafe_bindings::mobileactivation_client_start_service(
                device.pointer,
                &mut client,
                label.as_ptr() as *const c_char,
            )
        }
        .into();

        if result != MobileActivationError::Success {
            return Err(result);
        }

        Ok(MobileActivationClient {
            pointer: client,
            phantom: std::marker::PhantomData,
        })
    }

    pub fn get_activation_state(&self) -> Result<Plist, MobileActivationError> {
        let mut plist = unsafe { std::mem::zeroed() };

        let result = unsafe {
            unsafe_bindings::mobileactivation_get_activation_state(self.pointer, &mut plist)
        }
        .into();

        if result != MobileActivationError::Success {
            return Err(result);
        }

        Ok(plist.into())
    }

    pub fn create_activation_session_info(&self) -> Result<Plist, MobileActivationError> {
        let mut plist = unsafe { std::mem::zeroed() };

        let result = unsafe {
            unsafe_bindings::mobileactivation_create_activation_session_info(
                self.pointer,
                &mut plist,
            )
        }
        .into();

        if result != MobileActivationError::Success {
            return Err(result);
        }

        Ok(plist.into())
    }

    pub fn create_activation_info_with_session(
        &self,
    ) -> Result<(Plist, Plist), MobileActivationError> {
        let plist = unsafe { std::mem::zeroed() };
        let mut session_plist = unsafe { std::mem::zeroed() };

        let result = unsafe {
            unsafe_bindings::mobileactivation_create_activation_info_with_session(
                self.pointer,
                plist,
                &mut session_plist,
            )
        }
        .into();

        if result != MobileActivationError::Success {
            return Err(result);
        }

        Ok((plist.into(), session_plist.into()))
    }

    pub fn activate(&self, record: Plist) -> Result<(), MobileActivationError> {
        let result =
            unsafe { unsafe_bindings::mobileactivation_activate(self.pointer, record.get_pointer()) }
                .into();

        if result != MobileActivationError::Success {
            return Err(result);
        }

        Ok(())
    }

    pub fn activate_with_session(
        &self,
        record: Plist,
        session: Plist,
    ) -> Result<(), MobileActivationError> {
        let result = unsafe {
            unsafe_bindings::mobileactivation_activate_with_session(
                self.pointer,
                record.get_pointer(),
                session.get_pointer(),
            )
        }
        .into();

        if result != MobileActivationError::Success {
            return Err(result);
        }

        Ok(())
    }

    pub fn deactivate(&self) -> Result<(), MobileActivationError> {
        let result = unsafe { unsafe_bindings::mobileactivation_deactivate(self.pointer) }
            .into();

        if result != MobileActivationError::Success {
            return Err(result);
        }

        Ok(())
    }
}

impl Drop for MobileActivationClient<'_> {
    fn drop(&mut self) {
        unsafe {
            unsafe_bindings::mobileactivation_client_free(self.pointer);
        }
    }
}
