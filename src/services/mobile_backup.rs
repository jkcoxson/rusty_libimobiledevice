// jkcoxson

use std::{
    ffi::CString,
    os::raw::{c_char, c_int, c_uint},
};

use crate::{
    bindings as unsafe_bindings,
    error::{MobileBackup2Error, MobileBackupError},
    idevice::Device,
    services::lockdownd::LockdowndService,
};

use plist_plus::Plist;

/// Manages backups on older devices
/// This is only for old versions of iOS, you are probably looking for MobileBackup2
pub struct MobileBackupClient<'a> {
    pub(crate) pointer: unsafe_bindings::mobilebackup_client_t,
    phantom: std::marker::PhantomData<&'a Device>,
}

/// Manages backups on new devices
pub struct MobileBackup2Client<'a> {
    pub(crate) pointer: unsafe_bindings::mobilebackup2_client_t,
    phantom: std::marker::PhantomData<&'a Device>,
}

impl MobileBackupClient<'_> {
    /// Creates a new mobile backup service connection to the device
    /// The use of this function is unknown
    /// # Arguments
    /// * `device` - The device to create the service with
    /// # Returns
    /// The lockdownd service
    ///
    /// ***Verified:*** False
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

    /// Starts an afc service connection to the device
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
    ) -> Result<Self, MobileBackupError> {
        let mut client = unsafe { std::mem::zeroed() };
        let label_c_string = CString::new(label.into()).unwrap();
        let result = unsafe {
            unsafe_bindings::mobilebackup_client_start_service(
                device.pointer,
                &mut client,
                label_c_string.as_ptr(),
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

    /// Receives a plist from the service
    /// Blocks until a full plist is received
    /// # Arguments
    /// *none*
    /// # Returns
    /// A plist containing the message
    ///
    /// ***Verified:*** False
    pub fn receive(&self) -> Result<Plist, MobileBackupError> {
        let mut plist = unsafe { std::mem::zeroed() };

        let result =
            unsafe { unsafe_bindings::mobilebackup_receive(self.pointer, &mut plist) }.into();

        if result != MobileBackupError::Success {
            return Err(result);
        }

        Ok(plist.into())
    }

    /// Sends a message to the service
    /// # Arguments
    /// * `plist` - The message to send as a plist
    /// # Returns
    /// *none*
    ///
    /// ***Verified:*** False
    pub fn send(&self, message: Plist) -> Result<(), MobileBackupError> {
        let result =
            unsafe { unsafe_bindings::mobilebackup_send(self.pointer, message.get_pointer()) }
                .into();

        if result != MobileBackupError::Success {
            return Err(result);
        }

        Ok(())
    }

    /// Request a backup from the device
    /// # Arguments
    /// * `manifest` - The backup manifest containing the backup state and last backup time. For a first backup, pass None.
    /// * `base_path` - The path to use as the backup's base path, usually '/'.
    /// * `backup_verion` - The version of backup to use. The latest version is 1.6.
    pub fn request_backup(
        &self,
        manifest: Option<Plist>,
        base_path: impl Into<String>,
        backup_version: impl Into<String>,
    ) -> Result<(), MobileBackupError> {
        let ptr = manifest.map_or(std::ptr::null_mut(), |v| v.get_pointer());

        let base_path_c_string = CString::new(base_path.into()).unwrap();
        let backup_version_c_string = CString::new(backup_version.into()).unwrap();

        let result = unsafe {
            unsafe_bindings::mobilebackup_request_backup(
                self.pointer,
                ptr,
                base_path_c_string.as_ptr(),
                backup_version_c_string.as_ptr(),
            )
        }
        .into();

        if result != MobileBackupError::Success {
            return Err(result);
        }

        Ok(())
    }

    /// Sends a confirmation that the backup file was received
    /// # Arguments
    /// *none*
    /// # Returns
    /// *none*
    ///
    /// ***Verified:*** False
    pub fn send_backup_file_received(&self) -> Result<(), MobileBackupError> {
        let result =
            unsafe { unsafe_bindings::mobilebackup_send_backup_file_received(self.pointer) }.into();

        if result != MobileBackupError::Success {
            return Err(result);
        }

        Ok(())
    }

    /// Request the device restore a backup
    /// # Arguments
    /// * `manifest` - The backup manifest containing the backup version
    /// * `flags` - The flag to choose for restoring
    /// * `backup_version` - The backup version to use. The latest known version is 1.6.
    pub fn request_restore(
        &self,
        manifest: Plist,
        flags: MobileBackupRestoreFlags,
        backup_version: impl Into<String>,
    ) -> Result<(), MobileBackupError> {
        let backup_version_c_string = CString::new(backup_version.into()).unwrap();

        let result = unsafe {
            unsafe_bindings::mobilebackup_request_restore(
                self.pointer,
                manifest.get_pointer(),
                flags.into(),
                backup_version_c_string.as_ptr(),
            )
        }
        .into();

        if result != MobileBackupError::Success {
            return Err(result);
        }

        Ok(())
    }

    /// Receive a confirmation that the restore file was received
    /// Blocks until the full plist is received
    /// # Arguments
    /// *none*
    /// # Returns
    /// A plist with the confirmation
    ///
    /// ***Verified:*** False
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

    /// Receive a confirmation that the restore file was received
    /// Blocks until the full plist is received
    /// # Arguments
    /// *none*
    /// # Returns
    /// A plist with the confirmation
    ///
    /// ***Verified:*** False
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

    /// Tells the device that the restore is complete.
    /// The device will close the connection and reboot.
    /// # Arguments
    /// *none*
    /// # Returns
    /// *none*
    ///
    /// ***Verified:*** False
    pub fn send_restore_complete(&self) -> Result<(), MobileBackupError> {
        let result =
            unsafe { unsafe_bindings::mobilebackup_send_restore_complete(self.pointer) }.into();

        if result != MobileBackupError::Success {
            return Err(result);
        }

        Ok(())
    }

    /// Sends an error message to the device
    /// # Arguments
    /// * `error` - The error message to show on the device
    /// # Returns
    /// *none*
    ///
    /// ***Verified:*** False
    pub fn send_error(&self, error: impl Into<String>) -> Result<(), MobileBackupError> {
        let error_c_string = CString::new(error.into()).unwrap();

        let result = unsafe {
            unsafe_bindings::mobilebackup_send_error(self.pointer, error_c_string.as_ptr())
        }
        .into();

        if result != MobileBackupError::Success {
            return Err(result);
        }

        Ok(())
    }
}

impl MobileBackup2Client<'_> {
    /// Creates a new mobile backup service connection to the device
    /// The use of this function is unknown
    /// # Arguments
    /// * `device` - The device to create the service with
    /// # Returns
    /// The lockdownd service
    ///
    /// ***Verified:*** False
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

    /// Starts an afc service connection to the device
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
    ) -> Result<Self, MobileBackup2Error> {
        let mut client = unsafe { std::mem::zeroed() };
        let label_c_string = CString::new(label.into()).unwrap();

        let result = unsafe {
            unsafe_bindings::mobilebackup2_client_start_service(
                device.pointer,
                &mut client,
                label_c_string.as_ptr(),
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

    /// Sends a message to the service
    /// # Arguments
    /// * `message` - The message to send
    /// * `options` - The options to send the message with
    /// # Returns
    /// *none*
    ///
    /// ***Verified:*** False
    pub fn send_message(
        &self,
        message: Option<String>,
        options: Plist,
    ) -> Result<(), MobileBackup2Error> {
        let message_c_string = message.map(|s| CString::new(s).unwrap());
        let message_c_string_ptr = message_c_string.map_or(std::ptr::null(), |s| s.as_ptr());

        let result = unsafe {
            unsafe_bindings::mobilebackup2_send_message(
                self.pointer,
                message_c_string_ptr,
                options.get_pointer(),
            )
        }
        .into();

        if result != MobileBackup2Error::Success {
            return Err(result);
        }

        Ok(())
    }

    /// Receives a message from the service
    /// # Arguments
    /// *none*
    /// # Returns
    /// Receives the DL* string and the message
    ///
    /// ***Verified:*** False
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

    /// Sends raw data through the service connection
    /// # Arguments
    /// * `data` - A vector of bytes to send
    /// # Returns
    /// The bytes sent
    ///
    /// ***Verified:*** False
    pub fn send_raw(&self, data: &[u8]) -> Result<u32, MobileBackup2Error> {
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

    /// Receives raw data from the connection
    /// # Arguments
    /// * `len` - How many bytes to receive
    /// # Returns
    /// A vector of bytes containing the received data
    ///
    /// ***Verified:*** False
    pub fn receive_raw(&self, len: u32) -> Result<Vec<u8>, MobileBackup2Error> {
        let data: u8 = unsafe { std::mem::zeroed() };
        let mut received = 0;

        let result = unsafe {
            unsafe_bindings::mobilebackup2_receive_raw(self.pointer, &mut (data as c_char), len, &mut received)
        }
        .into();

        if result != MobileBackup2Error::Success {
            return Err(result);
        }

        Ok(unsafe {
            std::slice::from_raw_parts(&data, received as usize).to_vec()
        })
    }

    /// Exchanges version with the service
    /// # Arguments
    /// * `versions` - The versions to exchange
    /// # Returns
    /// * The version of the iOS device
    ///
    /// ***Verified:*** False
    pub fn version_exchange(&self, versions: &mut [f64]) -> Result<f64, MobileBackup2Error> {
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

    /// Sends a request to the service
    /// # Arguments
    /// * `request` - The type of request to send
    /// * `target` - The UDID of the target device
    /// * `source` - The UDID of the source device
    /// # Returns
    /// *none*
    ///
    /// ***Verified:*** False
    pub fn send_request(
        &self,
        request: MobileBackupRequest,
        target: impl Into<String>,
        source: impl Into<String>,
        options: Plist,
    ) -> Result<(), MobileBackup2Error> {
        let result = unsafe {
            let target_c_string = CString::new(target.into()).unwrap();
            let source_c_string = CString::new(source.into()).unwrap();
            let request: CString = request.into();
            unsafe_bindings::mobilebackup2_send_request(
                self.pointer,
                request.as_ptr(),
                target_c_string.as_ptr(),
                source_c_string.as_ptr(),
                options.get_pointer(),
            )
        }
        .into();

        if result != MobileBackup2Error::Success {
            return Err(result);
        }

        Ok(())
    }

    /// Sends a status response to the service
    /// # Arguments
    /// * `code` - The status code to send
    /// * `status_string` - The string for the status
    /// * `status_plist` - The plist containing status data
    pub fn send_status_response(
        &self,
        code: c_int,
        status_string: Option<String>,
        status_plist: Option<Plist>,
    ) -> Result<(), MobileBackup2Error> {
        let status_plist = status_plist.map_or(std::ptr::null_mut(), |s| s.get_pointer());
        let status_c_string = status_string.map(|s| CString::new(s).unwrap());
        let status_c_string_ptr = status_c_string.map_or(std::ptr::null(), |s| s.as_ptr());

        let result = unsafe {
            unsafe_bindings::mobilebackup2_send_status_response(
                self.pointer,
                code,
                status_c_string_ptr,
                status_plist,
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

/// Choose what to restore
pub enum MobileBackupRestoreFlags {
    /// Show a restore screen on the device
    Springboard,
    /// Don't overwrite any settings
    Settings,
    /// Don't overwrite the cameraroll
    CameraRoll,
}

impl From<MobileBackupRestoreFlags> for c_uint {
    fn from(flag: MobileBackupRestoreFlags) -> Self {
        match flag {
            MobileBackupRestoreFlags::Springboard => 1,
            MobileBackupRestoreFlags::Settings => 2,
            MobileBackupRestoreFlags::CameraRoll => 4,
        }
    }
}

impl From<MobileBackupRequest> for CString {
    fn from(request: MobileBackupRequest) -> Self {
        CString::new(match request {
            MobileBackupRequest::Backup => "Backup",
            MobileBackupRequest::Restore => "Restore",
            MobileBackupRequest::Info => "Info",
            MobileBackupRequest::List => "List",
        })
        .unwrap()
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
