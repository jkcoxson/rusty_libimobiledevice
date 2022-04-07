// jkcoxson

use std::os::raw::{c_char, c_int};

use crate::{
    bindings as unsafe_bindings,
    error::{MobileBackup2Error, MobileBackupError},
    idevice::Device,
    lockdownd::LockdowndService,
    plist::Plist,
};

/// This is only for old versions of iOS, you are probably looking for MobileBackup2
pub struct MobileBackupClient<'a> {
    pub(crate) pointer: unsafe_bindings::mobilebackup_client_t,
    phantom: std::marker::PhantomData<&'a Device>,
}

pub struct MobileBackup2Client<'a> {
    pub(crate) pointer: unsafe_bindings::mobilebackup2_client_t,
    phantom: std::marker::PhantomData<&'a Device>,
}

impl MobileBackupClient<'_> {
    pub fn new(device: &Device, service: LockdowndService) -> Result<Self, MobileBackupError> {
        let mut client = unsafe { std::mem::zeroed() };

        let result = unsafe {
            unsafe_bindings::mobilebackup_client_new(device.pointer, service.pointer, &mut client)
        }
        .into();

        if result != MobileBackupError::Success {
            return Err(result);
        }

        Ok(MobileBackupClient {
            pointer: client,
            phantom: std::marker::PhantomData,
        })
    }

    pub fn start_service(device: &Device, label: String) -> Result<Self, MobileBackupError> {
        let mut client = unsafe { std::mem::zeroed() };

        let result = unsafe {
            unsafe_bindings::mobilebackup_client_start_service(
                device.pointer,
                &mut client,
                label.as_ptr() as *const std::os::raw::c_char,
            )
        }
        .into();

        if result != MobileBackupError::Success {
            return Err(result);
        }

        Ok(MobileBackupClient {
            pointer: client,
            phantom: std::marker::PhantomData,
        })
    }

    pub fn recieve(&self) -> Result<Plist, MobileBackupError> {
        let mut plist = unsafe { std::mem::zeroed() };

        let result =
            unsafe { unsafe_bindings::mobilebackup_receive(self.pointer, &mut plist) }.into();

        if result != MobileBackupError::Success {
            return Err(result);
        }

        Ok(plist.into())
    }

    pub fn send(&self, plist: &Plist) -> Result<(), MobileBackupError> {
        let result =
            unsafe { unsafe_bindings::mobilebackup_send(self.pointer, plist.plist_t) }.into();

        if result != MobileBackupError::Success {
            return Err(result);
        }

        Ok(())
    }

    pub fn request_backup(
        &self,
        manifest: Plist,
        base_path: String,
        backup_version: String,
    ) -> Result<(), MobileBackupError> {
        let result = unsafe {
            unsafe_bindings::mobilebackup_request_backup(
                self.pointer,
                manifest.plist_t,
                base_path.as_ptr() as *const std::os::raw::c_char,
                backup_version.as_ptr() as *const std::os::raw::c_char,
            )
        }
        .into();

        if result != MobileBackupError::Success {
            return Err(result);
        }

        Ok(())
    }

    pub fn send_backup_file_received(&self) -> Result<(), MobileBackupError> {
        let result =
            unsafe { unsafe_bindings::mobilebackup_send_backup_file_received(self.pointer) }.into();

        if result != MobileBackupError::Success {
            return Err(result);
        }

        Ok(())
    }

    pub fn request_restore(
        &self,
        manifest: Plist,
        flags: unsafe_bindings::mobilebackup_flags_t,
        backup_version: String,
    ) -> Result<(), MobileBackupError> {
        let result = unsafe {
            unsafe_bindings::mobilebackup_request_restore(
                self.pointer,
                manifest.plist_t,
                flags,
                backup_version.as_ptr() as *const std::os::raw::c_char,
            )
        }
        .into();

        if result != MobileBackupError::Success {
            return Err(result);
        }

        Ok(())
    }

    pub fn receive_restore_file_received(&self) -> Result<Plist, MobileBackupError> {
        let mut plist = unsafe { std::mem::zeroed() };

        let result = unsafe {
            unsafe_bindings::mobilebackup_receive_restore_file_received(self.pointer, &mut plist)
        }
        .into();

        if result != MobileBackupError::Success {
            return Err(result);
        }

        Ok(plist.into())
    }

    pub fn receive_restore_application_received(&self) -> Result<Plist, MobileBackupError> {
        let mut plist = unsafe { std::mem::zeroed() };

        let result = unsafe {
            unsafe_bindings::mobilebackup_receive_restore_application_received(
                self.pointer,
                &mut plist,
            )
        }
        .into();

        if result != MobileBackupError::Success {
            return Err(result);
        }

        Ok(plist.into())
    }

    pub fn send_restore_complete(&self) -> Result<(), MobileBackupError> {
        let result =
            unsafe { unsafe_bindings::mobilebackup_send_restore_complete(self.pointer) }.into();

        if result != MobileBackupError::Success {
            return Err(result);
        }

        Ok(())
    }

    pub fn send_error(&self, error: String) -> Result<(), MobileBackupError> {
        let result = unsafe {
            unsafe_bindings::mobilebackup_send_error(
                self.pointer,
                error.as_ptr() as *const std::os::raw::c_char,
            )
        }
        .into();

        if result != MobileBackupError::Success {
            return Err(result);
        }

        Ok(())
    }
}

impl MobileBackup2Client<'_> {
    pub fn new(device: &Device, service: LockdowndService) -> Result<Self, MobileBackup2Error> {
        let mut client = unsafe { std::mem::zeroed() };

        let result = unsafe {
            unsafe_bindings::mobilebackup2_client_new(device.pointer, service.pointer, &mut client)
        }
        .into();

        if result != MobileBackup2Error::Success {
            return Err(result);
        }

        Ok(MobileBackup2Client {
            pointer: client,
            phantom: std::marker::PhantomData,
        })
    }

    pub fn start_service(device: &Device, label: String) -> Result<Self, MobileBackup2Error> {
        let mut client = unsafe { std::mem::zeroed() };

        let result = unsafe {
            unsafe_bindings::mobilebackup2_client_start_service(
                device.pointer,
                &mut client,
                label.as_ptr() as *const std::os::raw::c_char,
            )
        }
        .into();

        if result != MobileBackup2Error::Success {
            return Err(result);
        }

        Ok(MobileBackup2Client {
            pointer: client,
            phantom: std::marker::PhantomData,
        })
    }

    pub fn send_message(&self, message: String, options: Plist) -> Result<(), MobileBackup2Error> {
        let result = unsafe {
            unsafe_bindings::mobilebackup2_send_message(
                self.pointer,
                message.as_ptr() as *const std::os::raw::c_char,
                options.plist_t,
            )
        }
        .into();

        if result != MobileBackup2Error::Success {
            return Err(result);
        }

        Ok(())
    }

    pub fn receive_message(&self) -> Result<(String, Plist), MobileBackup2Error> {
        let mut message = unsafe { std::mem::zeroed() };
        let mut options = unsafe { std::mem::zeroed() };

        let result = unsafe {
            unsafe_bindings::mobilebackup2_receive_message(self.pointer, &mut options, &mut message)
        }
        .into();

        if result != MobileBackup2Error::Success {
            return Err(result);
        }

        Ok((
            unsafe { std::ffi::CStr::from_ptr(message) }
                .to_string_lossy()
                .into_owned(),
            options.into(),
        ))
    }

    pub fn send_raw(&self, data: Vec<u8>) -> Result<u32, MobileBackup2Error> {
        let mut sent = 0;
        let result = unsafe {
            unsafe_bindings::mobilebackup2_send_raw(
                self.pointer,
                data.as_ptr() as *const c_char,
                data.len() as u32,
                &mut sent,
            )
        }
        .into();

        if result != MobileBackup2Error::Success {
            return Err(result);
        }

        Ok(sent)
    }

    pub fn recieve_raw(&self, len: u32) -> Result<Vec<i8>, MobileBackup2Error> {
        let mut data = unsafe { std::mem::zeroed() };
        let mut received = 0;

        let result = unsafe {
            unsafe_bindings::mobilebackup2_receive_raw(self.pointer, &mut data, len, &mut received)
        }
        .into();

        if result != MobileBackup2Error::Success {
            return Err(result);
        }

        Ok(unsafe { std::slice::from_raw_parts(&data as *const i8, received as usize).to_vec() })
    }

    pub fn version_exchange(&self, mut versions: Vec<f64>) -> Result<f64, MobileBackup2Error> {
        let mut version = 0.0;
        let result = unsafe {
            unsafe_bindings::mobilebackup2_version_exchange(
                self.pointer,
                versions.as_mut_ptr(),
                versions.len() as c_char,
                &mut version,
            )
        }
        .into();

        if result != MobileBackup2Error::Success {
            return Err(result);
        }

        Ok(version)
    }

    pub fn send_request(
        &self,
        request: MobileBackupRequest,
        target: String,
        source: String,
        options: Plist,
    ) -> Result<(), MobileBackup2Error> {
        let result = unsafe {
            unsafe_bindings::mobilebackup2_send_request(
                self.pointer,
                request.into(),
                target.as_ptr() as *const std::os::raw::c_char,
                source.as_ptr() as *const std::os::raw::c_char,
                options.plist_t,
            )
        }
        .into();

        if result != MobileBackup2Error::Success {
            return Err(result);
        }

        Ok(())
    }

    pub fn send_status_response(
        &self,
        code: c_int,
        status_string: Option<String>,
        status_plist: Option<Plist>,
    ) -> Result<(), MobileBackup2Error> {
        let result = unsafe {
            unsafe_bindings::mobilebackup2_send_status_response(
                self.pointer,
                code,
                status_string
                    .map(|s| s.as_ptr() as *const std::os::raw::c_char)
                    .unwrap_or(std::ptr::null()),
                status_plist
                    .map(|p| p.plist_t)
                    .unwrap_or(0 as *mut std::os::raw::c_void), // idk
            )
        }
        .into();

        if result != MobileBackup2Error::Success {
            return Err(result);
        }

        Ok(())
    }
}

pub enum MobileBackupRequest {
    Backup,
    Restore,
    Info,
    List,
}

impl From<MobileBackupRequest> for *const c_char {
    fn from(request: MobileBackupRequest) -> Self {
        match request {
            MobileBackupRequest::Backup => "Backup".as_ptr() as *const c_char,
            MobileBackupRequest::Restore => "Restore".as_ptr() as *const c_char,
            MobileBackupRequest::Info => "Info".as_ptr() as *const c_char,
            MobileBackupRequest::List => "List".as_ptr() as *const c_char,
        }
    }
}

impl Drop for MobileBackupClient<'_> {
    fn drop(&mut self) {
        unsafe {
            unsafe_bindings::mobilebackup_client_free(self.pointer);
        }
    }
}

impl Drop for MobileBackup2Client<'_> {
    fn drop(&mut self) {
        unsafe {
            unsafe_bindings::mobilebackup2_client_free(self.pointer);
        }
    }
}
