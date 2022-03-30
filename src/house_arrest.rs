// jkcoxson

use crate::{
    bindings as unsafe_bindings, error::HouseArrestError, libimobiledevice::Device,
    lockdownd::LockdowndService, plist::Plist,
};

pub struct HouseArrest<'a> {
    pub(crate) pointer: unsafe_bindings::house_arrest_client_t,
    phantom: std::marker::PhantomData<&'a Device>,
}

impl HouseArrest<'_> {
    pub fn new(device: &Device, service: &LockdowndService) -> Result<Self, HouseArrestError> {
        let mut pointer = std::ptr::null_mut();
        let result = unsafe {
            unsafe_bindings::house_arrest_client_new(device.pointer, service.pointer, &mut pointer)
        }
        .into();

        if result != HouseArrestError::Success {
            return Err(result);
        }

        Ok(HouseArrest {
            pointer,
            phantom: std::marker::PhantomData,
        })
    }

    pub fn start_service(device: &Device, label: String) -> Result<Self, HouseArrestError> {
        let mut pointer = std::ptr::null_mut();
        let result = unsafe {
            unsafe_bindings::house_arrest_client_start_service(
                device.pointer,
                &mut pointer,
                label.as_ptr() as *const std::os::raw::c_char,
            )
        }
        .into();

        if result != HouseArrestError::Success {
            return Err(result);
        }

        Ok(HouseArrest {
            pointer,
            phantom: std::marker::PhantomData,
        })
    }

    pub fn send_request(&self, request: Plist) -> Result<(), HouseArrestError> {
        let result =
            unsafe { unsafe_bindings::house_arrest_send_request(self.pointer, request.plist_t) }
                .into();

        if result != HouseArrestError::Success {
            return Err(result);
        }

        Ok(())
    }

    pub fn send_command(&self, command: String, app_id: String) -> Result<(), HouseArrestError> {
        let result = unsafe {
            unsafe_bindings::house_arrest_send_command(
                self.pointer,
                command.as_ptr() as *const std::os::raw::c_char,
                app_id.as_ptr() as *const std::os::raw::c_char,
            )
        }
        .into();

        if result != HouseArrestError::Success {
            return Err(result);
        }

        Ok(())
    }

    pub fn get_result(&self) -> Result<Plist, HouseArrestError> {
        let mut plist_t = std::ptr::null_mut();
        let result =
            unsafe { unsafe_bindings::house_arrest_get_result(self.pointer, &mut plist_t) }.into();

        if result != HouseArrestError::Success {
            return Err(result);
        }

        Ok(plist_t.into())
    }

    
}

impl Drop for HouseArrest<'_> {
    fn drop(&mut self) {
        unsafe {
            unsafe_bindings::house_arrest_client_free(self.pointer);
        }
    }
}
