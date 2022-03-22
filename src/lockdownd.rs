// jkcoxson

use std::ffi::CStr;
use std::io::Read;

use libc::c_void;

pub use crate::bindings as unsafe_bindings;
use crate::debug;
use crate::error::{LockdowndError, MobileImageMounterError};
use crate::libimobiledevice::Device;
use crate::plist::Plist;

pub struct LockdowndClient<'a> {
    pub(crate) pointer: unsafe_bindings::lockdownd_client_t,
    pub label: String,
    phantom: std::marker::PhantomData<&'a Device>,
}

pub struct LockdowndPairRecord {
    pub device_certificate: String,
    pub host_certificate: String,
    pub root_certificate: String,
    pub host_id: String,
    pub system_buid: String,
}

unsafe impl Send for LockdowndClient<'_> {}
unsafe impl Sync for LockdowndClient<'_> {}

pub struct LockdowndService<'a> {
    pub(crate) pointer: unsafe_bindings::lockdownd_service_descriptor_t,
    pub label: String,
    pub port: u32,
    phantom: std::marker::PhantomData<&'a LockdowndClient<'a>>,
}

unsafe impl Send for LockdowndService<'_> {}
unsafe impl Sync for LockdowndService<'_> {}

pub struct MobileImageMounter<'a> {
    pub(crate) pointer: unsafe_bindings::mobile_image_mounter_client_t,
    pub(crate) phantom: std::marker::PhantomData<&'a LockdowndService<'a>>,
}

unsafe impl Send for MobileImageMounter<'_> {}
unsafe impl Sync for MobileImageMounter<'_> {}

impl LockdowndClient<'_> {
    pub fn new(device: &Device, label: String) -> Result<Self, LockdowndError> {
        let mut client: unsafe_bindings::lockdownd_client_t = unsafe { std::mem::zeroed() };
        let client_ptr: *mut unsafe_bindings::lockdownd_client_t = &mut client;

        let label_c_str = std::ffi::CString::new(label.clone()).unwrap();

        debug!("Creating lockdownd client for {}", device.udid);
        let result = unsafe {
            unsafe_bindings::lockdownd_client_new_with_handshake(
                device.pointer,
                client_ptr,
                label_c_str.as_ptr(),
            )
        }
        .into();

        if result != LockdowndError::Success {
            return Err(result);
        }

        Ok(LockdowndClient {
            pointer: unsafe { *client_ptr },
            label: label,
            phantom: std::marker::PhantomData,
        })
    }

    /// Gets a value from the device
    pub fn get_value(&self, key: String, domain: String) -> Result<Plist, LockdowndError> {
        let domain_c_str = std::ffi::CString::new(domain.clone()).unwrap();
        let domain_c_str = if domain == "".to_string() {
            std::ptr::null()
        } else {
            domain_c_str.as_ptr()
        };
        let key_c_str = std::ffi::CString::new(key.clone()).unwrap();
        let key_c_str = if key == "".to_string() {
            std::ptr::null()
        } else {
            key_c_str.as_ptr()
        };

        let mut value: unsafe_bindings::plist_t = unsafe { std::mem::zeroed() };

        debug!("Getting value for {}", key);
        let result = unsafe {
            unsafe_bindings::lockdownd_get_value(self.pointer, domain_c_str, key_c_str, &mut value)
        }
        .into();

        if result != LockdowndError::Success {
            return Err(result);
        }

        Ok(value.into())
    }

    pub fn start_service(&mut self, label: String) -> Result<LockdowndService, LockdowndError> {
        let label_c_str = std::ffi::CString::new(label.clone()).unwrap();
        let label_c_str = if label == "".to_string() {
            std::ptr::null()
        } else {
            label_c_str.as_ptr()
        };

        let mut service: unsafe_bindings::lockdownd_service_descriptor_t =
            unsafe { std::mem::zeroed() };

        debug!("Starting lockdown service");
        let result = unsafe {
            unsafe_bindings::lockdownd_start_service(self.pointer, label_c_str, &mut service)
        }
        .into();

        if result != LockdowndError::Success {
            return Err(result);
        }

        Ok(LockdowndService {
            pointer: service,
            label: label,
            port: 0,
            phantom: std::marker::PhantomData,
        })
    }

    pub fn pair(&self) -> Result<LockdowndPairRecord, LockdowndError> {
        let to_fill = unsafe { std::mem::zeroed() };
        let result = unsafe { unsafe_bindings::lockdownd_pair(self.pointer, to_fill) }.into();
        if result != LockdowndError::Success {
            return Err(result);
        }
        Ok(to_fill.into())
    }
}

impl From<*mut unsafe_bindings::lockdownd_pair_record> for LockdowndPairRecord {
    fn from(l: *mut unsafe_bindings::lockdownd_pair_record) -> Self {
        let device_certificate = unsafe { CStr::from_ptr((*l).device_certificate) }
            .to_str()
            .unwrap()
            .to_string();
        let host_certificate = unsafe { CStr::from_ptr((*l).host_certificate) }
            .to_str()
            .unwrap()
            .to_string();
        let root_certificate = unsafe { CStr::from_ptr((*l).root_certificate) }
            .to_str()
            .unwrap()
            .to_string();
        let host_id = unsafe { CStr::from_ptr((*l).host_id) }
            .to_str()
            .unwrap()
            .to_string();
        let system_buid = unsafe { CStr::from_ptr((*l).system_buid) }
            .to_str()
            .unwrap()
            .to_string();

        Self {
            device_certificate,
            host_certificate,
            root_certificate,
            host_id,
            system_buid,
        }
    }
}

#[cfg(target_os = "windows")]
type ImageMounterPointerSize = u32;
#[cfg(target_os = "windows")]
type ImageMounterReturnType = i32;
#[cfg(not(target_os = "windows"))]
type ImageMounterPointerSize = u64;
#[cfg(not(target_os = "windows"))]
type ImageMounterReturnType = i64;

impl MobileImageMounter<'_> {
    /// Uploads an image from a path to the device
    pub fn upload_image(
        &self,
        image_path: String,
        image_type: String,
        signature_path: String,
    ) -> Result<(), MobileImageMounterError> {
        // Determine if files exist
        let dmg_size = match std::fs::File::open(image_path.clone()) {
            Ok(mut file) => {
                let mut temp_buf = vec![];
                file.read_to_end(&mut temp_buf).unwrap();
                temp_buf.len()
            }
            Err(_) => return Err(MobileImageMounterError::DmgNotFound),
        };
        let signature_size = match std::fs::File::open(signature_path.clone()) {
            Ok(mut file) => {
                let mut temp_buf = vec![];
                file.read_to_end(&mut temp_buf).unwrap();
                temp_buf.len()
            }
            Err(_) => return Err(MobileImageMounterError::SignatureNotFound),
        };
        // Read the image into a buffer
        let image_path_c_str = &mut std::ffi::CString::new(image_path.clone()).unwrap();
        let mode_c_str = &mut std::ffi::CString::new("rb").unwrap();
        debug!("Opening image file");
        let image_buffer = unsafe { libc::fopen(image_path_c_str.as_ptr(), mode_c_str.as_ptr()) };
        // Read the signature into a buffer
        let signature_path_c_str = &mut std::ffi::CString::new(signature_path.clone()).unwrap();
        debug!("Reading signature file");
        let signature_buffer =
            unsafe { libc::fopen(signature_path_c_str.as_ptr(), mode_c_str.as_ptr()) };

        let image_type_c_str = std::ffi::CString::new(image_type.clone()).unwrap();
        let image_type_c_str = if image_type == "".to_string() {
            std::ptr::null()
        } else {
            image_type_c_str.as_ptr()
        };

        debug!("Uploading image");
        let result = unsafe {
            unsafe_bindings::mobile_image_mounter_upload_image(
                self.pointer,
                image_type_c_str,
                dmg_size as ImageMounterPointerSize,
                signature_buffer as *const i8,
                signature_size as u16,
                Some(image_mounter_callback),
                image_buffer as *mut c_void,
            )
        }
        .into();

        if result != MobileImageMounterError::Success {
            return Err(result);
        }

        Ok(())
    }

    /// Mounts the image on the device
    pub fn mount_image(
        &self,
        image_path: String,
        image_type: String,
        signature_path: String,
    ) -> Result<Plist, MobileImageMounterError> {
        // Read the image into a buffer
        let mut image_buffer = Vec::new();
        let file = match std::fs::File::open(image_path.clone()) {
            Ok(file) => file,
            Err(_) => return Err(MobileImageMounterError::DmgNotFound),
        };
        let mut reader = std::io::BufReader::new(file);
        match reader.read_to_end(&mut image_buffer) {
            Ok(_) => (),
            Err(_) => return Err(MobileImageMounterError::DmgNotFound),
        };
        // Read the signature into a buffer
        let mut signature_buffer = Vec::new();
        let file = match std::fs::File::open(signature_path) {
            Ok(file) => file,
            Err(_) => return Err(MobileImageMounterError::SignatureNotFound),
        };
        let mut reader = std::io::BufReader::new(file);
        match reader.read_to_end(&mut signature_buffer) {
            Ok(_) => (),
            Err(_) => return Err(MobileImageMounterError::SignatureNotFound),
        };
        let image_type_c_str = std::ffi::CString::new(image_type.clone()).unwrap();
        let image_type_c_str = if image_type == "".to_string() {
            std::ptr::null()
        } else {
            image_type_c_str.as_ptr()
        };

        let mut plist: unsafe_bindings::plist_t = unsafe { std::mem::zeroed() };

        debug!("Mounting image");
        let result = unsafe {
            unsafe_bindings::mobile_image_mounter_mount_image(
                self.pointer,
                image_path.as_ptr() as *const i8,
                signature_buffer.as_ptr() as *const i8,
                signature_buffer.len() as u16,
                image_type_c_str,
                &mut plist,
            )
        }
        .into();

        if result != MobileImageMounterError::Success {
            return Err(result);
        }
        Ok(plist.into())
    }

    pub fn lookup_image(&self, image_type: String) -> Result<Plist, MobileImageMounterError> {
        let image_type_c_str = std::ffi::CString::new(image_type.clone()).unwrap();
        let image_type_c_str = if image_type == "".to_string() {
            std::ptr::null()
        } else {
            image_type_c_str.as_ptr()
        };

        let mut plist: unsafe_bindings::plist_t = unsafe { std::mem::zeroed() };

        debug!("Looking up image");
        let result = unsafe {
            unsafe_bindings::mobile_image_mounter_lookup_image(
                self.pointer,
                image_type_c_str,
                &mut plist,
            )
        }
        .into();

        if result != MobileImageMounterError::Success {
            return Err(result);
        }
        Ok(plist.into())
    }
}

extern "C" fn image_mounter_callback(
    a: *mut c_void,
    b: ImageMounterPointerSize,
    c: *mut c_void,
) -> ImageMounterReturnType {
    debug!("image_mounter_callback called");
    return unsafe { libc::fread(a, 1, b as usize, c as *mut libc::FILE) }
        as ImageMounterReturnType;
}

impl Drop for LockdowndClient<'_> {
    fn drop(&mut self) {
        debug!("Dropping LockdowndClient");
        unsafe { unsafe_bindings::lockdownd_client_free(self.pointer) };
    }
}

impl Drop for LockdowndService<'_> {
    fn drop(&mut self) {
        debug!("Dropping LockdowndService");
        unsafe {
            unsafe_bindings::lockdownd_service_descriptor_free(self.pointer);
        }
    }
}

impl Drop for MobileImageMounter<'_> {
    fn drop(&mut self) {
        debug!("Dropping MobileImageMounter");
        unsafe {
            unsafe_bindings::mobile_image_mounter_free(self.pointer);
        }
    }
}
