// jkcoxson

use std::ffi::CString;

use crate::{
    bindings as unsafe_bindings, error::MobileActivationError, idevice::Device,
    services::lockdownd::LockdowndService,
};

use plist_plus::Plist;

pub struct MobileActivationClient<'a> {
    pub(crate) pointer: unsafe_bindings::mobileactivation_client_t,
    phantom: std::marker::PhantomData<&'a Device>,
}

impl MobileActivationClient<'_> {
    /// Creates a new mobile activation service connection to the device
    /// The use of this function is unknown
    /// # Arguments
    /// * `device` - The device to create the service with
    /// # Returns
    /// The lockdownd service
    ///
    /// ***Verified:*** False
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

    /// Starts a mobile activation service connection to the device
    /// # Arguments
    /// * `device` - The device to create the service with
    /// * `service_name` - The name of the service to start
    /// # Returns
    /// An afc service connection
    ///
    /// ***Verified:*** False
    pub fn start_service(
        device: &Device,
        label: impl Into<String>,
    ) -> Result<Self, MobileActivationError> {
        let label_c_string = CString::new(label.into()).unwrap();
        let mut client = unsafe { std::mem::zeroed() };

        let result = unsafe {
            unsafe_bindings::mobileactivation_client_start_service(
                device.pointer,
                &mut client,
                label_c_string.as_ptr(),
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

    /// Gets the activation state of the device
    /// # Arguments
    /// *none*
    /// # Returns
    /// A plist with the results
    ///
    /// ***Verified:*** False
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

    /// Gets a session blob for the device requied for activation.
    /// Requires an internet connection as it queries albert.apple.com for the value
    /// # Arguments
    /// *none*
    /// # Returns
    /// A plist with the activation session info
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

    /// Gets the activation info from Apple's servers
    /// # Arguments
    /// *none*
    /// # Returns
    /// Both the handshake reponse and the activation info
    ///
    /// ***Verified:*** False
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

    /// Activates a device
    /// # Arguments
    /// * `record` - A plist containing the activation record fetched from Apple
    /// * `session` - A plist containing session blobs. Pass None if not required.
    /// # Returns
    /// *none*
    ///
    /// ***Verified:*** False
    pub fn activate(
        &self,
        record: Plist,
        session: Option<Plist>,
    ) -> Result<(), MobileActivationError> {
        let result = if let Some(session) = session {
            unsafe {
                unsafe_bindings::mobileactivation_activate_with_session(
                    self.pointer,
                    record.get_pointer(),
                    session.get_pointer(),
                )
            }
            .into()
        } else {
            unsafe {
                unsafe_bindings::mobileactivation_activate(self.pointer, record.get_pointer())
            }
            .into()
        };

        if result != MobileActivationError::Success {
            return Err(result);
        }

        Ok(())
    }

    /// Deactivates a device, requiring it to be reactivated by Apple's servers
    /// # Arguments
    /// *none*
    /// # Returns
    /// *none*
    ///
    /// ***Verified:*** False
    pub fn deactivate(&self) -> Result<(), MobileActivationError> {
        let result = unsafe { unsafe_bindings::mobileactivation_deactivate(self.pointer) }.into();

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
