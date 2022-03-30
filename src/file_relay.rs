// jkcoxson

use std::ffi::CString;

use crate::{
    bindings as unsafe_bindings, connection::DeviceConnection, error::FileRelayError,
    libimobiledevice::Device, lockdownd::LockdowndService,
};

pub struct FileRelay<'a> {
    pub pointer: unsafe_bindings::file_relay_client_t,
    phantom: std::marker::PhantomData<&'a Device>,
}

impl FileRelay<'_> {
    pub fn new(device: &Device, service: LockdowndService) -> Result<Self, FileRelayError> {
        let mut pointer = std::ptr::null_mut();
        let result = unsafe {
            unsafe_bindings::file_relay_client_new(device.pointer, service.pointer, &mut pointer)
        }
        .into();

        if result != FileRelayError::Success {
            return Err(result);
        }

        Ok(FileRelay {
            pointer,
            phantom: std::marker::PhantomData,
        })
    }

    pub fn start_service(device: &Device, label: String) -> Result<Self, FileRelayError> {
        let mut pointer = std::ptr::null_mut();
        let result = unsafe {
            unsafe_bindings::file_relay_client_start_service(
                device.pointer,
                &mut pointer,
                label.as_ptr() as *const i8,
            )
        }
        .into();

        if result != FileRelayError::Success {
            return Err(result);
        }

        Ok(FileRelay {
            pointer,
            phantom: std::marker::PhantomData,
        })
    }

    pub fn request_sources(
        &self,
        sources: Vec<FileRelaySources>,
        mut connection: DeviceConnection,
    ) -> Result<(), FileRelayError> {
        let sources: Vec<FileRelaySources> = sources.into();
        let mut source_ptrs = vec![];
        for source in sources {
            let source: CString = source.into();
            source_ptrs.push(source.into_raw() as *const i8);
        }
        let ptrs_ptr = source_ptrs.as_mut_ptr();
        let result = unsafe {
            unsafe_bindings::file_relay_request_sources(
                self.pointer,
                ptrs_ptr,
                &mut connection.pointer,
            )
        }
        .into();

        if result != FileRelayError::Success {
            return Err(result);
        }

        Ok(())
    }

    pub fn request_sources_timeout(
        &self,
        sources: Vec<FileRelaySources>,
        mut connection: DeviceConnection,
        timeout: u32,
    ) -> Result<(), FileRelayError> {
        let sources: Vec<FileRelaySources> = sources.into();
        let mut source_ptrs = vec![];
        for source in sources {
            let source: CString = source.into();
            source_ptrs.push(source.into_raw() as *const i8);
        }
        let ptrs_ptr = source_ptrs.as_mut_ptr();
        let result = unsafe {
            unsafe_bindings::file_relay_request_sources_timeout(
                self.pointer,
                ptrs_ptr,
                &mut connection.pointer,
                timeout,
            )
        }
        .into();

        if result != FileRelayError::Success {
            return Err(result);
        }

        Ok(())
    }

    
}

pub enum FileRelaySources {
    AppleSupport,
    Network,
    VPN,
    WiFi,
    UserDatabases,
    CrashReporter,
    Tmp,
    SystemConfiguration,
}

impl From<FileRelaySources> for CString {
    fn from(source: FileRelaySources) -> Self {
        match source {
            FileRelaySources::AppleSupport => CString::new("AppleSupport").unwrap(),
            FileRelaySources::Network => CString::new("Network").unwrap(),
            FileRelaySources::VPN => CString::new("VPN").unwrap(),
            FileRelaySources::WiFi => CString::new("WiFi").unwrap(),
            FileRelaySources::UserDatabases => CString::new("UserDatabases").unwrap(),
            FileRelaySources::CrashReporter => CString::new("CrashReporter").unwrap(),
            FileRelaySources::Tmp => CString::new("Tmp").unwrap(),
            FileRelaySources::SystemConfiguration => CString::new("SystemConfiguration").unwrap(),
        }
    }
}

impl Drop for FileRelay<'_> {
    fn drop(&mut self) {
        unsafe {
            unsafe_bindings::file_relay_client_free(self.pointer);
        }
    }
}
