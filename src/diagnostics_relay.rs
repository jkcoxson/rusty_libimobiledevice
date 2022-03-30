// jkcoxson

use libc::c_uint;

use crate::{
    bindings as unsafe_bindings, error::DiagnosticsRelayError, libimobiledevice::Device,
    lockdownd::LockdowndService, plist::Plist,
};

pub struct DiagnosticsRelay<'a> {
    pub(crate) pointer: unsafe_bindings::diagnostics_relay_client_t,
    phantom: std::marker::PhantomData<&'a Device>,
}

impl DiagnosticsRelay<'_> {
    pub fn new(device: &Device, service: LockdowndService) -> Result<Self, DiagnosticsRelayError> {
        let mut pointer = std::ptr::null_mut();
        let result = unsafe {
            unsafe_bindings::diagnostics_relay_client_new(
                device.pointer,
                service.pointer,
                &mut pointer,
            )
        }
        .into();

        if result != DiagnosticsRelayError::Success {
            return Err(result);
        }

        Ok(DiagnosticsRelay {
            pointer: pointer,
            phantom: std::marker::PhantomData,
        })
    }

    pub fn start_service(device: &Device, label: String) -> Result<Self, DiagnosticsRelayError> {
        let mut pointer = std::ptr::null_mut();
        let result = unsafe {
            unsafe_bindings::diagnostics_relay_client_start_service(
                device.pointer,
                &mut pointer,
                label.as_ptr() as *const std::os::raw::c_char,
            )
        }
        .into();

        if result != DiagnosticsRelayError::Success {
            return Err(result);
        }

        Ok(DiagnosticsRelay {
            pointer: pointer,
            phantom: std::marker::PhantomData,
        })
    }

    pub fn goodbye(self) -> Result<(), DiagnosticsRelayError> {
        let result = unsafe { unsafe_bindings::diagnostics_relay_goodbye(self.pointer) }.into();

        if result != DiagnosticsRelayError::Success {
            return Err(result);
        }

        Ok(())
    }

    pub fn sleep(self) -> Result<(), DiagnosticsRelayError> {
        let result = unsafe { unsafe_bindings::diagnostics_relay_sleep(self.pointer) }.into();

        if result != DiagnosticsRelayError::Success {
            return Err(result);
        }

        Ok(())
    }

    pub fn restart(self, flag: c_uint) -> Result<(), DiagnosticsRelayError> {
        let result =
            unsafe { unsafe_bindings::diagnostics_relay_restart(self.pointer, flag) }.into();

        if result != DiagnosticsRelayError::Success {
            return Err(result);
        }

        Ok(())
    }

    pub fn shutdown(self, flag: c_uint) -> Result<(), DiagnosticsRelayError> {
        let result =
            unsafe { unsafe_bindings::diagnostics_relay_shutdown(self.pointer, flag) }.into();

        if result != DiagnosticsRelayError::Success {
            return Err(result);
        }

        Ok(())
    }

    pub fn request_diagnostics(&self, type_: String) -> Result<Plist, DiagnosticsRelayError> {
        let mut plist = std::ptr::null_mut();
        let result = unsafe {
            unsafe_bindings::diagnostics_relay_request_diagnostics(
                self.pointer,
                type_.as_ptr() as *const std::os::raw::c_char,
                &mut plist,
            )
        }
        .into();

        if result != DiagnosticsRelayError::Success {
            return Err(result);
        }

        Ok(plist.into())
    }

    pub fn query_mobilegestalt(&self, keys: Plist) -> Result<Plist, DiagnosticsRelayError> {
        let mut plist = std::ptr::null_mut();
        let result = unsafe {
            unsafe_bindings::diagnostics_relay_query_mobilegestalt(
                self.pointer,
                keys.plist_t,
                &mut plist,
            )
        }
        .into();

        if result != DiagnosticsRelayError::Success {
            return Err(result);
        }

        Ok(plist.into())
    }

    pub fn query_ioregistry_entry(&self, entry_name: String, entry_class: String) -> Result<Plist, DiagnosticsRelayError> {
        let mut plist = std::ptr::null_mut();
        let result = unsafe {
            unsafe_bindings::diagnostics_relay_query_ioregistry_entry(
                self.pointer,
                entry_name.as_ptr() as *const std::os::raw::c_char,
                entry_class.as_ptr() as *const std::os::raw::c_char,
                &mut plist,
            )
        }
        .into();

        if result != DiagnosticsRelayError::Success {
            return Err(result);
        }

        Ok(plist.into())
    }

    pub fn query_ioregistry_plane(&self, plane: String) -> Result<Plist, DiagnosticsRelayError> {
        let mut plist = std::ptr::null_mut();
        let result = unsafe {
            unsafe_bindings::diagnostics_relay_query_ioregistry_plane(
                self.pointer,
                plane.as_ptr() as *const std::os::raw::c_char,
                &mut plist,
            )
        }
        .into();

        if result != DiagnosticsRelayError::Success {
            return Err(result);
        }

        Ok(plist.into())
    }

    
}

pub enum DiagnosticsRelayAction {
    WaitForDisconnect,
    DisplayPass,
    DisplayFail,
}

impl From<DiagnosticsRelayAction> for c_uint {
    fn from(action: DiagnosticsRelayAction) -> Self {
        match action {
            DiagnosticsRelayAction::WaitForDisconnect => 2,
            DiagnosticsRelayAction::DisplayPass => 4,
            DiagnosticsRelayAction::DisplayFail => 8,
        }
    }
}

impl Drop for DiagnosticsRelay<'_> {
    fn drop(&mut self) {
        unsafe {
            unsafe_bindings::diagnostics_relay_client_free(self.pointer);
        }
    }
}
