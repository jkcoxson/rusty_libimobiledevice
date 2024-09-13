// jkcoxson

use std::{
    ffi::CString,
    io::Read,
    os::raw::{c_char, c_long, c_ulong},
    path::PathBuf,
};

use libc::c_void;
use log::{info, trace};
use plist_plus::Plist;

use super::lockdownd::LockdowndService;
use crate::{bindings as unsafe_bindings, error::MobileImageMounterError, idevice::Device};

/// A service for mounting developer disk images on the device
pub struct MobileImageMounter<'a> {
    pub(crate) pointer: unsafe_bindings::mobile_image_mounter_client_t,
    pub(crate) phantom: std::marker::PhantomData<&'a Device>,
}

unsafe impl Send for MobileImageMounter<'_> {}
unsafe impl Sync for MobileImageMounter<'_> {}

impl MobileImageMounter<'_> {
    /// Creates a new mobile image mounter service from a lockdown service
    /// # Arguments
    /// * `device` - The device to connect to
    /// * `descriptor` - The lockdown service to connect on
    /// # Returns
    /// A struct containing the handle to the connection
    ///
    /// ***Verified:*** False
    pub fn new(
        device: &Device,
        descriptor: LockdowndService,
    ) -> Result<Self, MobileImageMounterError> {
        let mut client = unsafe { std::mem::zeroed() };

        let result = unsafe {
            unsafe_bindings::mobile_image_mounter_new(
                device.pointer,
                descriptor.pointer,
                &mut client,
            )
        }
        .into();

        if result != MobileImageMounterError::Success {
            return Err(result);
        }

        Ok(MobileImageMounter {
            pointer: client,
            phantom: std::marker::PhantomData,
        })
    }

    /// Starts a new connection and adds a mobile image mounter to it
    /// # Arguments
    /// * `device` - The device to connect to
    /// * `label` - The label for the connection
    /// # Returns
    /// A struct containing the handle to the connection
    ///
    /// ***Verified:*** False
    pub fn start_service(
        device: &Device,
        label: impl Into<String>,
    ) -> Result<Self, MobileImageMounterError> {
        let mut client = unsafe { std::mem::zeroed() };
        let label_c_string = CString::new(label.into()).unwrap();

        let result = unsafe {
            unsafe_bindings::mobile_image_mounter_start_service(
                device.pointer,
                &mut client,
                label_c_string.as_ptr(),
            )
        }
        .into();

        if result != MobileImageMounterError::Success {
            return Err(result);
        }

        Ok(MobileImageMounter {
            pointer: client,
            phantom: std::marker::PhantomData,
        })
    }

    /// Uploads an image from a path to the device
    /// # Arguments
    /// * `image_path` - The path on the host to the image. Cannot contain spaces. TODO: fix this
    /// * `image_type` - The type of the image to upload, usually "Developer"
    /// * `signature_path` - The path to the signature
    /// # Returns
    /// *none*
    ///
    /// ***Verified:*** False
    pub fn upload_image(
        &self,
        image_path: impl Into<String>,
        image_type: impl Into<String>,
        signature_path: impl Into<String>,
    ) -> Result<(), MobileImageMounterError> {
        let image_path = image_path.into();
        let image_type = image_type.into();
        let signature_path = signature_path.into();
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
        let image_path_c_string = CString::new(image_path).unwrap();
        let mode_c_string = CString::new("rb").unwrap();
        info!("Opening image file");
        let image_buffer =
            unsafe { libc::fopen(image_path_c_string.as_ptr(), mode_c_string.as_ptr()) };
        // Read the signature into a buffer
        let signature_path_c_string = CString::new(signature_path).unwrap();
        info!("Reading signature file");
        let signature_buffer =
            unsafe { libc::fopen(signature_path_c_string.as_ptr(), mode_c_string.as_ptr()) };

        let image_type_c_string = CString::new(image_type.clone()).unwrap();
        let image_type_c_string_ptr = if image_type_c_string.is_empty() {
            std::ptr::null()
        } else {
            image_type_c_string.as_ptr()
        };

        info!("Uploading image");
        let result = unsafe {
            unsafe_bindings::mobile_image_mounter_upload_image(
                self.pointer,
                image_type_c_string_ptr,
                dmg_size as c_ulong,
                signature_buffer as *const c_char,
                signature_size as u16,
                Some(image_mounter_callback),
                image_buffer as *mut c_void,
            )
        }
        .into();

        unsafe {
            libc::fclose(image_buffer);
            libc::fclose(signature_buffer);
        }

        if result != MobileImageMounterError::Success {
            return Err(result);
        }

        Ok(())
    }

    /// Mounts the image on the device
    /// # Arguments
    /// * `image_path` - The path on the host to the image. Cannot contain spaces. TODO: fix this
    /// * `image_type` - The type of the image to upload, usually "Developer"
    /// * `signature_path` - The path to the signature
    /// # Returns
    /// *none*
    ///
    /// ***Verified:*** False
    pub fn mount_image(
        &self,
        image_path: impl Into<String>,
        image_type: impl Into<String>,
        signature_path: impl Into<String>,
    ) -> Result<Plist, MobileImageMounterError> {
        let image_path = image_path.into();
        let image_type = image_type.into();
        let signature_path = signature_path.into();
        // Confirm that the image exists
        let image_path: PathBuf = image_path.into();
        if !image_path.exists() {
            return Err(MobileImageMounterError::DmgNotFound);
        }
        let image_path = CString::new(match image_path.canonicalize() {
            Ok(path) => path.display().to_string(),
            Err(_) => return Err(MobileImageMounterError::DmgNotFound),
        })
        .unwrap();

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

        let image_type_c_string = CString::new(image_type.clone()).unwrap();
        let image_type_c_string_ptr = if image_type_c_string.is_empty() {
            std::ptr::null()
        } else {
            image_type_c_string.as_ptr()
        };

        let mut plist: unsafe_bindings::plist_t = unsafe { std::mem::zeroed() };

        info!("Mounting image");
        let result = unsafe {
            unsafe_bindings::mobile_image_mounter_mount_image(
                self.pointer,
                image_path.as_ptr() as *const c_char,
                signature_buffer.as_ptr() as *const c_char,
                signature_buffer.len() as u16,
                image_type_c_string_ptr,
                &mut plist,
            )
        }
        .into();

        if result != MobileImageMounterError::Success {
            return Err(result);
        }
        Ok(plist.into())
    }

    /// Fetches all images mounted on the device
    /// # Arguments
    /// * `image_type` - The type of images to look for. Pass "" for all images.
    /// # Returns
    /// A plist containing the results. This may return Ok even if failed, check the plist.
    ///
    /// ***Verified:*** False
    pub fn lookup_image(
        &self,
        image_type: impl Into<String>,
    ) -> Result<Plist, MobileImageMounterError> {
        let image_type_c_string = CString::new(image_type.into()).unwrap();
        let image_type_c_string_ptr = if image_type_c_string.is_empty() {
            std::ptr::null()
        } else {
            image_type_c_string.as_ptr()
        };

        let mut plist: unsafe_bindings::plist_t = unsafe { std::mem::zeroed() };

        info!("Looking up image");
        let result = unsafe {
            unsafe_bindings::mobile_image_mounter_lookup_image(
                self.pointer,
                image_type_c_string_ptr,
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

extern "C" fn image_mounter_callback(a: *mut c_void, b: c_ulong, c: *mut c_void) -> c_long {
    trace!("image_mounter_callback called");
    unsafe { libc::fread(a, 1, b as usize, c as *mut libc::FILE) as c_long }
}

impl Drop for MobileImageMounter<'_> {
    fn drop(&mut self) {
        info!("Dropping MobileImageMounter");
        unsafe {
            unsafe_bindings::mobile_image_mounter_free(self.pointer);
        }
    }
}
