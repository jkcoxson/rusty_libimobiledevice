// jkcoxson

use crate::{
    bindings as unsafe_bindings, error::CompanionProxyError, idevice::Device, plist::Plist,
    services::lockdownd::LockdowndService,
};

pub struct CompanionProxy<'a> {
    pub(crate) pointer: unsafe_bindings::companion_proxy_client_t,
    phantom: std::marker::PhantomData<&'a Device>,
}

impl CompanionProxy<'_> {
    pub fn new(device: &Device, descriptor: LockdowndService) -> Result<Self, CompanionProxyError> {
        let mut pointer = unsafe { std::mem::zeroed() };
        let result = unsafe {
            unsafe_bindings::companion_proxy_client_new(
                device.pointer,
                descriptor.pointer,
                &mut pointer,
            )
        }
        .into();
        if result != CompanionProxyError::Success {
            return Err(result);
        }

        Ok(CompanionProxy {
            pointer,
            phantom: std::marker::PhantomData,
        })
    }

    pub fn start_service(device: &Device, label: String) -> Result<Self, CompanionProxyError> {
        let mut pointer = unsafe { std::mem::zeroed() };
        let result = unsafe {
            unsafe_bindings::companion_proxy_client_start_service(
                device.pointer,
                &mut pointer,
                label.as_ptr() as *const i8,
            )
        }
        .into();
        if result != CompanionProxyError::Success {
            return Err(result);
        }

        Ok(CompanionProxy {
            pointer,
            phantom: std::marker::PhantomData,
        })
    }

    pub fn send(&self, message: Plist) -> Result<(), CompanionProxyError> {
        let result =
            unsafe { unsafe_bindings::companion_proxy_send(self.pointer, message.plist_t) }.into();
        if result != CompanionProxyError::Success {
            return Err(result);
        }

        Ok(())
    }

    pub fn receive(&self) -> Result<Plist, CompanionProxyError> {
        let mut plist = unsafe { std::mem::zeroed() };
        let result =
            unsafe { unsafe_bindings::companion_proxy_receive(self.pointer, &mut plist) }.into();
        if result != CompanionProxyError::Success {
            return Err(result);
        }

        Ok(plist.into())
    }

    pub fn get_device_registry(self) -> Result<Plist, CompanionProxyError> {
        let mut plist = unsafe { std::mem::zeroed() };
        let result = unsafe {
            unsafe_bindings::companion_proxy_get_device_registry(self.pointer, &mut plist)
        }
        .into();
        if result != CompanionProxyError::Success {
            return Err(result);
        }

        Ok(plist.into())
    }

    pub fn get_value_from_registry(
        self,
        udid: String,
        key: String,
    ) -> Result<Plist, CompanionProxyError> {
        let mut plist = unsafe { std::mem::zeroed() };
        let result = unsafe {
            unsafe_bindings::companion_proxy_get_value_from_registry(
                self.pointer,
                udid.as_ptr() as *const i8,
                key.as_ptr() as *const i8,
                &mut plist,
            )
        }
        .into();
        if result != CompanionProxyError::Success {
            return Err(result);
        }

        Ok(plist.into())
    }

    pub fn start_forwarding_service_port(
        &self,
        port: u16,
        service_name: String,
        options: Plist,
    ) -> Result<u16, CompanionProxyError> {
        let mut result_port = 0;
        let result = unsafe {
            unsafe_bindings::companion_proxy_start_forwarding_service_port(
                self.pointer,
                port,
                service_name.as_ptr() as *const i8,
                &mut result_port,
                options.plist_t,
            )
        }
        .into();
        if result != CompanionProxyError::Success {
            return Err(result);
        }

        Ok(result_port)
    }

    pub fn stop_forwarding_service_port(&self, port: u16) -> Result<(), CompanionProxyError> {
        let result = unsafe {
            unsafe_bindings::companion_proxy_stop_forwarding_service_port(self.pointer, port)
        }
        .into();
        if result != CompanionProxyError::Success {
            return Err(result);
        }

        Ok(())
    }
}

impl Drop for CompanionProxy<'_> {
    fn drop(&mut self) {
        unsafe {
            unsafe_bindings::companion_proxy_client_free(self.pointer);
        }
    }
}
