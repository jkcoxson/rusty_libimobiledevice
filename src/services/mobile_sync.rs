// jkcoxson

use std::{
    ffi::CString,
    os::raw::{c_char, c_uint},
};

use crate::{
    bindings as unsafe_bindings,
    error::MobileSyncError,
    idevice::Device,
    services::lockdownd::LockdowndService,
};

use plist_plus::{Plist, PlistType};

pub struct MobileSyncClient<'a> {
    pub(crate) pointer: unsafe_bindings::mobilesync_client_t,
    phantom: std::marker::PhantomData<&'a Device>,
}

pub struct MobileSyncAnchor {
    device_anchor: String,
    computer_anchor: String,
}

impl MobileSyncClient<'_> {
    pub fn new(device: Device, descriptor: LockdowndService) -> Result<Self, MobileSyncError> {
        let mut pointer: unsafe_bindings::mobilesync_client_t = std::ptr::null_mut();
        let result = unsafe {
            unsafe_bindings::mobilesync_client_new(device.pointer, descriptor.pointer, &mut pointer)
        }
        .into();

        if result != MobileSyncError::Success {
            return Err(result);
        }

        Ok(MobileSyncClient {
            pointer,
            phantom: std::marker::PhantomData,
        })
    }

    pub fn start_service(device: Device, label: String) -> Result<Self, MobileSyncError> {
        let mut pointer: unsafe_bindings::mobilesync_client_t = std::ptr::null_mut();
        let result = unsafe {
            unsafe_bindings::mobilesync_client_start_service(
                device.pointer,
                &mut pointer,
                label.as_ptr() as *mut c_char,
            )
        }
        .into();

        if result != MobileSyncError::Success {
            return Err(result);
        }

        Ok(MobileSyncClient {
            pointer,
            phantom: std::marker::PhantomData,
        })
    }

    pub fn recieve(&self) -> Result<Plist, MobileSyncError> {
        let mut plist: unsafe_bindings::plist_t = std::ptr::null_mut();
        let result =
            unsafe { unsafe_bindings::mobilesync_receive(self.pointer, &mut plist) }.into();

        if result != MobileSyncError::Success {
            return Err(result);
        }

        Ok(plist.into())
    }

    pub fn send(&self, plist: Plist) -> Result<(), MobileSyncError> {
        let result =
            unsafe { unsafe_bindings::mobilesync_send(self.pointer, plist.get_pointer()) }.into();

        if result != MobileSyncError::Success {
            return Err(result);
        }

        Ok(())
    }

    pub fn start(
        &self,
        data_class: String,
        anchors: Vec<MobileSyncAnchor>,
        computer_data_class_version: u64,
        sync_type: MobileSyncType,
    ) -> Result<(), (String, MobileSyncError)> {
        let data_class = CString::new(data_class).unwrap();
        let data_class_ptr = data_class.as_ptr();

        let mut anchor_ptrs = Vec::new();
        for i in anchors {
            anchor_ptrs.push(unsafe_bindings::mobilesync_anchors_t::from(i));
        }
        anchor_ptrs.push(std::ptr::null_mut());

        let mut device_data_class_version = 0;

        let mut error_description = std::ptr::null_mut();

        let result = unsafe {
            unsafe_bindings::mobilesync_start(
                self.pointer,
                data_class_ptr,
                anchor_ptrs[0],
                computer_data_class_version,
                &mut sync_type.into(),
                &mut device_data_class_version,
                &mut error_description,
            )
        }
        .into();

        if result != MobileSyncError::Success {
            return Err((
                unsafe { CString::from_raw(error_description) }
                    .into_string()
                    .unwrap(),
                result,
            ));
        }

        Ok(())
    }

    pub fn cancel(&self, reason: String) -> Result<(), MobileSyncError> {
        let reason = CString::new(reason).unwrap();
        let reason_ptr = reason.as_ptr();

        let result = unsafe { unsafe_bindings::mobilesync_cancel(self.pointer, reason_ptr) }.into();

        if result != MobileSyncError::Success {
            return Err(result);
        }

        Ok(())
    }

    pub fn finish(&self) -> Result<(), MobileSyncError> {
        let result = unsafe { unsafe_bindings::mobilesync_finish(self.pointer) }.into();

        if result != MobileSyncError::Success {
            return Err(result);
        }

        Ok(())
    }

    pub fn get_all_records_from_device(&self) -> Result<(), MobileSyncError> {
        let result =
            unsafe { unsafe_bindings::mobilesync_get_all_records_from_device(self.pointer) }.into();

        if result != MobileSyncError::Success {
            return Err(result);
        }

        Ok(())
    }

    pub fn get_changes_from_device(&self) -> Result<(), MobileSyncError> {
        let result =
            unsafe { unsafe_bindings::mobilesync_get_changes_from_device(self.pointer) }.into();

        if result != MobileSyncError::Success {
            return Err(result);
        }

        Ok(())
    }

    pub fn clear_all_records_on_device(&self) -> Result<(), MobileSyncError> {
        let result =
            unsafe { unsafe_bindings::mobilesync_clear_all_records_on_device(self.pointer) }.into();

        if result != MobileSyncError::Success {
            return Err(result);
        }

        Ok(())
    }

    pub fn receive_changes(&self) -> Result<(Plist, bool, Plist), MobileSyncError> {
        let mut plist: unsafe_bindings::plist_t = std::ptr::null_mut();
        let mut has_more_changes = 0;
        let mut anchor: unsafe_bindings::plist_t = std::ptr::null_mut();

        let result = unsafe {
            unsafe_bindings::mobilesync_receive_changes(
                self.pointer,
                &mut plist,
                &mut has_more_changes,
                &mut anchor,
            )
        }
        .into();

        if result != MobileSyncError::Success {
            return Err(result);
        }

        Ok((plist.into(), has_more_changes != 0, anchor.into()))
    }

    pub fn acknowledge_changes_from_device(&self) -> Result<(), MobileSyncError> {
        let result =
            unsafe { unsafe_bindings::mobilesync_acknowledge_changes_from_device(self.pointer) }
                .into();

        if result != MobileSyncError::Success {
            return Err(result);
        }

        Ok(())
    }

    pub fn ready_to_send_changes_from_computer(&self) -> Result<(), MobileSyncError> {
        let result = unsafe {
            unsafe_bindings::mobilesync_ready_to_send_changes_from_computer(self.pointer)
        }
        .into();

        if result != MobileSyncError::Success {
            return Err(result);
        }

        Ok(())
    }

    pub fn send_changes(
        &self,
        entities: Plist,
        is_last: bool,
        actions: Option<Plist>,
    ) -> Result<(), MobileSyncError> {
        let actions = actions.map(|x| x.get_pointer()).unwrap_or(std::ptr::null_mut());

        let result = unsafe {
            unsafe_bindings::mobilesync_send_changes(
                self.pointer,
                entities.get_pointer(),
                is_last.into(),
                actions,
            )
        }
        .into();

        if result != MobileSyncError::Success {
            return Err(result);
        }

        Ok(())
    }

    pub fn remap_identifiers(&self, mut mapping: Plist) -> Result<(), MobileSyncError> {
        if mapping.plist_type != PlistType::Array {
            return Err(MobileSyncError::InvalidArg);
        }

        let result =
            unsafe { unsafe_bindings::mobilesync_remap_identifiers(self.pointer, &mut mapping.get_pointer()) }
                .into();

        if result != MobileSyncError::Success {
            return Err(result);
        }

        Ok(())
    }
}

impl MobileSyncAnchor {
    pub fn new(device_anchor: String, computer_anchor: String) -> Self {
        MobileSyncAnchor {
            device_anchor,
            computer_anchor,
        }
    }
}

impl From<MobileSyncAnchor> for unsafe_bindings::mobilesync_anchors_t {
    fn from(anchor: MobileSyncAnchor) -> Self {
        let x = unsafe_bindings::mobilesync_anchors {
            device_anchor: anchor.device_anchor.as_ptr() as *mut c_char,
            computer_anchor: anchor.computer_anchor.as_ptr() as *mut c_char,
        };
        Box::into_raw(Box::new(x))
    }
}

pub enum MobileSyncType {
    Fast,
    Slow,
    Reset,
}

impl From<MobileSyncType> for c_uint {
    fn from(type_: MobileSyncType) -> Self {
        match type_ {
            MobileSyncType::Fast => 0,
            MobileSyncType::Slow => 1,
            MobileSyncType::Reset => 2,
        }
    }
}

impl Drop for MobileSyncClient<'_> {
    fn drop(&mut self) {
        unsafe {
            unsafe_bindings::mobilesync_client_free(self.pointer);
        }
    }
}
