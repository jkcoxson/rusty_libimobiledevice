// jkcoxson

use crate::{
    bindings as unsafe_bindings, error::PropertyListServiceError, idevice::Device,
    lockdownd::LockdowndService, plist::Plist,
};

pub struct PropertyListServiceClient<'a> {
    pub(crate) pointer: unsafe_bindings::property_list_service_client_t,
    phantom: std::marker::PhantomData<&'a Device>,
}

impl PropertyListServiceClient<'_> {
    pub fn new(
        device: &Device,
        descriptor: LockdowndService,
    ) -> Result<Self, PropertyListServiceError> {
        let mut pointer = std::ptr::null_mut();
        let result = unsafe {
            unsafe_bindings::property_list_service_client_new(
                device.pointer,
                descriptor.pointer,
                &mut pointer,
            )
        }
        .into();

        if result != PropertyListServiceError::Success {
            return Err(result);
        }

        Ok(PropertyListServiceClient {
            pointer,
            phantom: std::marker::PhantomData,
        })
    }

    pub fn send_xml_plist(&self, data: Plist) -> Result<(), PropertyListServiceError> {
        let result = unsafe {
            unsafe_bindings::property_list_service_send_xml_plist(self.pointer, data.plist_t)
        }
        .into();

        if result != PropertyListServiceError::Success {
            return Err(result);
        }

        Ok(())
    }

    pub fn send_binary_plist(&self, data: Plist) -> Result<(), PropertyListServiceError> {
        let result = unsafe {
            unsafe_bindings::property_list_service_send_binary_plist(self.pointer, data.plist_t)
        }
        .into();

        if result != PropertyListServiceError::Success {
            return Err(result);
        }

        Ok(())
    }

    pub fn receive_plist(&self) -> Result<Plist, PropertyListServiceError> {
        let mut plist_t = std::ptr::null_mut();
        let result = unsafe {
            unsafe_bindings::property_list_service_receive_plist(self.pointer, &mut plist_t)
        }
        .into();

        if result != PropertyListServiceError::Success {
            return Err(result);
        }

        Ok(plist_t.into())
    }

    pub fn receive_plist_with_timeout(
        &self,
        timeout: u32,
    ) -> Result<Plist, PropertyListServiceError> {
        let mut plist_t = std::ptr::null_mut();
        let result = unsafe {
            unsafe_bindings::property_list_service_receive_plist_with_timeout(
                self.pointer,
                &mut plist_t,
                timeout,
            )
        }
        .into();

        if result != PropertyListServiceError::Success {
            return Err(result);
        }

        Ok(plist_t.into())
    }

    pub fn enable_ssl(&self) -> Result<(), PropertyListServiceError> {
        let result = unsafe { unsafe_bindings::property_list_service_enable_ssl(self.pointer) }
            .into();

        if result != PropertyListServiceError::Success {
            return Err(result);
        }

        Ok(())
    }

    pub fn disable_ssl(&self) -> Result<(), PropertyListServiceError> {
        let result = unsafe { unsafe_bindings::property_list_service_disable_ssl(self.pointer) }
            .into();

        if result != PropertyListServiceError::Success {
            return Err(result);
        }

        Ok(())
    }
}

impl Drop for PropertyListServiceClient<'_> {
    fn drop(&mut self) {
        unsafe {
            unsafe_bindings::property_list_service_client_free(self.pointer);
        }
    }
}
