// jkcoxson

use libc::c_uint;
use std::ffi::CString;

use crate::{
    bindings as unsafe_bindings, error::DiagnosticsRelayError, idevice::Device,
    services::lockdownd::LockdowndService,
};

use plist_plus::Plist;

/// Relays diagnostic logs from the iOS device to the host
pub struct DiagnosticsRelay<'a> {
    pub(crate) pointer: unsafe_bindings::diagnostics_relay_client_t,
    phantom: std::marker::PhantomData<&'a Device>,
}

impl DiagnosticsRelay<'_> {
    /// Creates a new diagnostics relay service from a lockdown service
    /// # Arguments
    /// * `device` - The device to create the sevice with
    /// * `service` - The lockdown service to connect on
    /// # Returns
    /// A struct containing the handle to the service
    ///
    /// ***Verified:*** False
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
            pointer,
            phantom: std::marker::PhantomData,
        })
    }

    /// Starts a new service with diagnostic relay
    /// # Arguments
    /// * `device` - The device to create the sevice with
    /// * `label` - The label to give the connection
    /// # Returns
    /// A struct containing the handle to the service
    ///
    /// ***Verified:*** False
    pub fn start_service(
        device: &Device,
        label: impl Into<String>,
    ) -> Result<Self, DiagnosticsRelayError> {
        let mut pointer = std::ptr::null_mut();
        let label_c_string = CString::new(label.into()).unwrap();
        let result = unsafe {
            unsafe_bindings::diagnostics_relay_client_start_service(
                device.pointer,
                &mut pointer,
                label_c_string.as_ptr(),
            )
        }
        .into();

        if result != DiagnosticsRelayError::Success {
            return Err(result);
        }

        Ok(DiagnosticsRelay {
            pointer,
            phantom: std::marker::PhantomData,
        })
    }

    /// Sends a goodbye to the service, terminating the connection.
    /// This consumes the DiagnosticsRelayClient.
    /// # Arguments
    /// *none*
    /// # Returns
    /// *none*
    ///
    /// ***Verified:*** False
    pub fn goodbye(self) -> Result<(), DiagnosticsRelayError> {
        let result = unsafe { unsafe_bindings::diagnostics_relay_goodbye(self.pointer) }.into();

        if result != DiagnosticsRelayError::Success {
            return Err(result);
        }

        Ok(())
    }

    /// Puts the connected device to sleep, breaking the connection.
    /// # Arguments
    /// *none*
    /// # Returns
    /// *none*
    ///
    /// ***Verified:*** False
    pub fn sleep(self) -> Result<(), DiagnosticsRelayError> {
        let result = unsafe { unsafe_bindings::diagnostics_relay_sleep(self.pointer) }.into();

        if result != DiagnosticsRelayError::Success {
            return Err(result);
        }

        Ok(())
    }

    /// Restarts the connected device, breaking the connection.
    /// # Arguments
    /// * `flag` - A flag to determine actions for the restart
    /// # Returns
    /// *none*
    ///
    /// ***Verified:*** False
    pub fn restart(self, flag: c_uint) -> Result<(), DiagnosticsRelayError> {
        let result =
            unsafe { unsafe_bindings::diagnostics_relay_restart(self.pointer, flag) }.into();

        if result != DiagnosticsRelayError::Success {
            return Err(result);
        }

        Ok(())
    }

    /// Shuts the device off, breaking the connection.
    /// # Arguments
    /// * `flag` - A flag to determine actions for the restart
    /// # Returns
    /// *none*
    ///
    /// ***Verified:*** False
    pub fn shutdown(self, flag: c_uint) -> Result<(), DiagnosticsRelayError> {
        let result =
            unsafe { unsafe_bindings::diagnostics_relay_shutdown(self.pointer, flag) }.into();

        if result != DiagnosticsRelayError::Success {
            return Err(result);
        }

        Ok(())
    }

    /// Requests diagnostics from the device
    /// # Arguments
    /// * `type_` - The type of diagnostics to request
    /// # Returns
    /// A plist containing the diagnostics data
    ///
    /// ***Verified:*** False
    pub fn request_diagnostics(
        &self,
        type_: impl Into<String>,
    ) -> Result<Plist, DiagnosticsRelayError> {
        let mut plist = std::ptr::null_mut();
        let type_c_string = CString::new(type_.into()).unwrap();
        let result = unsafe {
            unsafe_bindings::diagnostics_relay_request_diagnostics(
                self.pointer,
                type_c_string.as_ptr(),
                &mut plist,
            )
        }
        .into();

        if result != DiagnosticsRelayError::Success {
            return Err(result);
        }

        Ok(plist.into())
    }

    /// Usage unknown
    /// # Arguments
    /// `keys` - Unknown
    /// # Returns
    /// A plist with unknown usage
    ///
    /// ***Verified:*** False
    pub fn query_mobilegestalt(&self, keys: Plist) -> Result<Plist, DiagnosticsRelayError> {
        let mut plist = std::ptr::null_mut();
        let result = unsafe {
            unsafe_bindings::diagnostics_relay_query_mobilegestalt(
                self.pointer,
                keys.get_pointer(),
                &mut plist,
            )
        }
        .into();

        if result != DiagnosticsRelayError::Success {
            return Err(result);
        }

        Ok(plist.into())
    }

    /// Requests data from the device's IO registry
    /// # Arguments
    /// * `entry_name` - The name to request
    /// * `entry_class` - The class to request
    /// # Returns
    /// A plist containing the entry
    ///
    /// ***Verified:*** False
    pub fn query_ioregistry_entry(
        &self,
        entry_name: impl Into<String>,
        entry_class: impl Into<String>,
    ) -> Result<Plist, DiagnosticsRelayError> {
        let mut plist = std::ptr::null_mut();
        let entry_name_c_string = CString::new(entry_name.into()).unwrap();
        let entry_class_c_string = CString::new(entry_class.into()).unwrap();

        let result = unsafe {
            unsafe_bindings::diagnostics_relay_query_ioregistry_entry(
                self.pointer,
                entry_name_c_string.as_ptr(),
                entry_class_c_string.as_ptr(),
                &mut plist,
            )
        }
        .into();

        if result != DiagnosticsRelayError::Success {
            return Err(result);
        }

        Ok(plist.into())
    }

    /// Usage unknown
    /// # Arguments
    /// * `plane` - Unknown
    /// # Returns
    /// A plist containing the requested data
    ///
    /// ***Verified:*** False
    pub fn query_ioregistry_plane(
        &self,
        plane: impl Into<String>,
    ) -> Result<Plist, DiagnosticsRelayError> {
        let mut plist = std::ptr::null_mut();
        let plane_c_string = CString::new(plane.into()).unwrap();
        let result = unsafe {
            unsafe_bindings::diagnostics_relay_query_ioregistry_plane(
                self.pointer,
                plane_c_string.as_ptr(),
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
