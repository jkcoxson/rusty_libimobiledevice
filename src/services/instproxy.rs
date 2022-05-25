// jkcoxson

use std::ffi::CString;

use crate::{bindings as unsafe_bindings, error::InstProxyError, idevice::Device};

use log::info;
use plist_plus::Plist;

/// Manages installing, removing and modifying applications on the device
pub struct InstProxyClient<'a> {
    pub(crate) pointer: unsafe_bindings::instproxy_client_t,
    pub label: String,
    phantom: std::marker::PhantomData<&'a Device>,
}

unsafe impl Send for InstProxyClient<'_> {}
unsafe impl Sync for InstProxyClient<'_> {}

impl InstProxyClient<'_> {
    /// Starts a new service with house arrest
    /// # Arguments
    /// * `device` - The device to create the sevice with
    /// * `label` - The label to give the connection
    /// # Returns
    /// A struct containing the handle to the service
    ///
    /// ***Verified:*** False
    pub fn new(device: &Device, label: impl Into<String>) -> Result<Self, InstProxyError> {
        let label = label.into();
        let mut instproxy_client = unsafe { std::mem::zeroed() };
        let label_c_str = std::ffi::CString::new(label.clone()).unwrap();
        info!("Creating instproxy client for {}", device.get_udid());
        let result = unsafe {
            unsafe_bindings::instproxy_client_start_service(
                device.pointer,
                &mut instproxy_client,
                label_c_str.as_ptr(),
            )
        }
        .into();

        if result != InstProxyError::Success {
            return Err(result);
        }

        Ok(InstProxyClient {
            pointer: instproxy_client,
            label,
            phantom: std::marker::PhantomData,
        })
    }

    /// Lists installed applications on the device
    /// # Arguments
    /// * `option` - The browse options to use
    /// # Returns
    /// A plist with a list of applications
    ///
    /// ***Verified:*** False
    pub fn browse(&self, option: BrowseOption) -> Result<Plist, InstProxyError> {
        let mut plist = std::ptr::null_mut();

        let result = if option == BrowseOption::None {
            unsafe {
                unsafe_bindings::instproxy_browse(self.pointer, std::ptr::null_mut(), &mut plist)
            }
        } else {
            let option_plist: Plist = option.into();
            unsafe {
                unsafe_bindings::instproxy_browse(
                    self.pointer,
                    option_plist.get_pointer(),
                    &mut plist,
                )
            }
        }
        .into();

        if result != InstProxyError::Success {
            return Err(result);
        }

        Ok(plist.into())
    }

    /// Creates a Plist containing return attributes for lookup
    /// # Arguments
    /// * `options` - The options for lookup
    /// * `args` - The items to return in the lookup
    /// # Returns
    /// A plist containing the apps found
    ///
    /// ***Verified:*** False
    pub fn create_return_attributes(options: Vec<(String, Plist)>, args: Vec<String>) -> Plist {
        info!("Setting return attributes");
        let mut pointer: Plist = unsafe { unsafe_bindings::instproxy_client_options_new() }.into();

        for (key, value) in options {
            pointer.dict_set_item(&key, value).unwrap();
        }

        let mut return_attributes = Plist::new_array();
        for i in args {
            let t = Plist::new_string(&i);
            return_attributes.array_append_item(t).unwrap();
        }
        match pointer.dict_insert_item("ReturnAttributes", return_attributes) {
            _ => {}
        };

        pointer
    }

    /// Looks up information about apps on the device
    /// # Arguments
    /// * `app_ids` - The bundle ID's of apps to lookup information about
    /// * `client_options` - A plist containing options for the lookup. Create with `create_return_attributes`
    /// # Returns
    /// A plist with the lookup results
    ///
    /// ***Verified:*** False
    pub fn lookup(
        &self,
        app_ids: Vec<String>,
        client_options: Option<Plist>,
    ) -> Result<Plist, InstProxyError> {
        // Convert vector of strings to a slice
        let cstrings = app_ids
            .iter()
            .map(|s| std::ffi::CString::new(s.clone()).unwrap())
            .collect::<Vec<_>>();
        let mut cstring_pointers = cstrings.iter().map(|s| s.as_ptr()).collect::<Vec<_>>();
        cstring_pointers.push(std::ptr::null());
        let mut cstring_pointers_ptr = cstring_pointers.as_mut_ptr();
        if app_ids.len() == 0 {
            cstring_pointers_ptr = std::ptr::null_mut();
        }

        let opt_ptr = if client_options.is_some() {
            let client_options = client_options.unwrap();
            let ptr = client_options.get_pointer();
            client_options.false_drop();
            ptr
        } else {
            std::ptr::null_mut()
        };

        let mut res_plist: unsafe_bindings::plist_t = unsafe { std::mem::zeroed() };
        info!("Instproxy lookup");
        let result = unsafe {
            unsafe_bindings::instproxy_lookup(
                self.pointer,
                cstring_pointers_ptr,
                opt_ptr,
                &mut res_plist,
            )
        }
        .into();
        if result != InstProxyError::Success {
            return Err(result);
        }

        unsafe { unsafe_bindings::instproxy_client_options_free(opt_ptr) };

        info!("Instproxy lookup done");
        Ok(res_plist.into())
    }

    /// Installs a package on the device
    /// # Arguments
    /// * `pkg_path` - The path to the .ipa or other package bundle
    /// * `client_options` - The options in a plist dictionary for install
    /// # Returns
    /// *none*
    ///
    /// ***Verified:*** False
    pub fn install(
        &self,
        pkg_path: impl Into<String>,
        client_options: Option<Plist>,
    ) -> Result<(), InstProxyError> {
        info!("Instproxy install");
        let pkg_path_c_str = std::ffi::CString::new(pkg_path.into()).unwrap();

        let ptr = if client_options.is_some() {
            client_options.unwrap().get_pointer()
        } else {
            std::ptr::null_mut()
        };

        let result = unsafe {
            unsafe_bindings::instproxy_install(
                self.pointer,
                pkg_path_c_str.as_ptr(),
                ptr,
                None, // I feel like this will segfault. The bindings are probably wrong.
                std::ptr::null_mut(),
            )
        }
        .into();
        if result != InstProxyError::Success {
            return Err(result);
        }

        Ok(())
    }

    /// Updates a package on the device
    /// # Arguments
    /// * `pkg_path` - The path to the new package
    /// * `client_options` - The options in a plist dictionary for install
    /// # Returns
    /// *none*
    ///
    /// ***Verified:*** False
    pub fn upgrade(
        &self,
        pkg_path: impl Into<String>,
        client_options: Option<Plist>,
    ) -> Result<(), InstProxyError> {
        info!("Instproxy upgrade");
        let pkg_path_c_str = std::ffi::CString::new(pkg_path.into()).unwrap();

        let ptr = if client_options.is_some() {
            client_options.unwrap().get_pointer()
        } else {
            std::ptr::null_mut()
        };

        let result = unsafe {
            unsafe_bindings::instproxy_upgrade(
                self.pointer,
                pkg_path_c_str.as_ptr(),
                ptr,
                None, // I feel like this will segfault. The bindings are probably wrong.
                std::ptr::null_mut(),
            )
        }
        .into();
        if result != InstProxyError::Success {
            return Err(result);
        }

        Ok(())
    }

    /// Uninstalls an app on the device
    /// # Arguments
    /// * `app_id` - The bundle ID of the app to uninstall
    /// * `client_options` - The options in a plist dictionary for uninstall
    /// # Returns
    /// *none*
    ///
    /// ***Verified:*** False
    pub fn uninstall(
        &self,
        app_id: impl Into<String>,
        client_options: Option<Plist>,
    ) -> Result<(), InstProxyError> {
        info!("Instproxy uninstall");
        let app_id_c_str = std::ffi::CString::new(app_id.into()).unwrap();

        let ptr = if client_options.is_some() {
            client_options.unwrap().get_pointer()
        } else {
            std::ptr::null_mut()
        };

        let result = unsafe {
            unsafe_bindings::instproxy_uninstall(
                self.pointer,
                app_id_c_str.as_ptr(),
                ptr,
                None, // I feel like this will segfault. The bindings are probably wrong.
                std::ptr::null_mut(),
            )
        }
        .into();
        if result != InstProxyError::Success {
            return Err(result);
        }

        Ok(())
    }

    /// Gets a list of all the archives on the device
    /// # Arguments
    /// * `client_options` - Currently no known use for this, pass None if unsure.
    /// # Returns
    /// *none*
    ///
    /// ***Verified:*** False
    pub fn lookup_archives(&self, client_options: Option<Plist>) -> Result<Plist, InstProxyError> {
        let mut res_plist: unsafe_bindings::plist_t = unsafe { std::mem::zeroed() };
        info!("Instproxy lookup archives");

        let ptr = if client_options.is_some() {
            client_options.unwrap().get_pointer()
        } else {
            std::ptr::null_mut()
        };
        let result = unsafe {
            unsafe_bindings::instproxy_lookup_archives(self.pointer, ptr, &mut res_plist)
        }
        .into();
        if result != InstProxyError::Success {
            return Err(result);
        }
        Ok(res_plist.into())
    }

    /// Creates an archive of the app
    /// # Arguments
    /// * `app_id` - The bundle ID of the app to archive
    /// * `client_options` - The options for archive.
    ///     Current known options for plist dictionaries are `SkipUninstall: bool` and `ArchiveType: "ApplicationOnly"`
    /// # Returns
    /// *none*
    ///
    /// ***Verified:*** False
    pub fn archive(
        &self,
        app_id: impl Into<String>,
        client_options: Option<Plist>,
    ) -> Result<(), InstProxyError> {
        info!("Instproxy archive");
        let app_id_c_str = std::ffi::CString::new(app_id.into()).unwrap();

        let ptr = if client_options.is_some() {
            client_options.unwrap().get_pointer()
        } else {
            std::ptr::null_mut()
        };

        let result = unsafe {
            unsafe_bindings::instproxy_archive(
                self.pointer,
                app_id_c_str.as_ptr(),
                ptr,
                None, // I feel like this will segfault. The bindings are probably wrong.
                std::ptr::null_mut(),
            )
        }
        .into();
        if result != InstProxyError::Success {
            return Err(result);
        }
        Ok(())
    }

    /// Restore an archived application back to the device
    /// # Arguments
    /// * `app_id` - The bundle ID of the app to restore
    /// * `client_options` - The options for restoring the app
    /// # Returns
    /// *none*
    ///
    /// ***Verified:*** False
    pub fn restore(
        &self,
        app_id: impl Into<String>,
        client_options: Option<Plist>,
    ) -> Result<(), InstProxyError> {
        info!("Instproxy restore");
        let app_id_c_str = std::ffi::CString::new(app_id.into()).unwrap();

        let ptr = if client_options.is_some() {
            client_options.unwrap().get_pointer()
        } else {
            std::ptr::null_mut()
        };

        let result = unsafe {
            unsafe_bindings::instproxy_restore(
                self.pointer,
                app_id_c_str.as_ptr(),
                ptr,
                None, // I feel like this will segfault. The bindings are probably wrong.
                std::ptr::null_mut(),
            )
        }
        .into();
        if result != InstProxyError::Success {
            return Err(result);
        }
        Ok(())
    }

    /// Removes an archive from the device
    /// # Arguments
    /// * `app_id` - The app bundle ID of the archive to remove
    /// * `client_options` - The options to use for removal. There are no known options, so pass None if unsure.
    /// # Returns
    /// *none*
    ///
    /// ***Verified:*** False
    pub fn remove_archive(
        &self,
        app_id: impl Into<String>,
        client_options: Option<Plist>,
    ) -> Result<(), InstProxyError> {
        info!("Instproxy remove archive");
        let app_id_c_str = std::ffi::CString::new(app_id.into()).unwrap();

        let ptr = if client_options.is_some() {
            client_options.unwrap().get_pointer()
        } else {
            std::ptr::null_mut()
        };

        let result = unsafe {
            unsafe_bindings::instproxy_remove_archive(
                self.pointer,
                app_id_c_str.as_ptr(),
                ptr,
                None, // I feel like this will segfault. The bindings are probably wrong.
                std::ptr::null_mut(),
            )
        }
        .into();
        if result != InstProxyError::Success {
            return Err(result);
        }
        Ok(())
    }

    /// Check if the device has certain capabilities
    /// # Arguments
    /// * `capabilities` - A list of capabilities to check
    /// * `client_options` - The options for checking. There are no known uses of this, pass None.
    /// # Returns
    /// A plist with the results of the check
    ///
    /// ***Verified:*** False
    pub fn check_capabilities_match(
        &self,
        capabilities: Vec<String>,
        client_options: Option<Plist>,
    ) -> Result<Plist, InstProxyError> {
        let mut res_plist = unsafe { std::mem::zeroed() };
        let mut capabilities_c_str = vec![];
        for capability in capabilities {
            capabilities_c_str.push(std::ffi::CString::new(capability).unwrap());
        }

        let mut capabilities_c_str_ptrs = vec![];
        for capability in capabilities_c_str {
            capabilities_c_str_ptrs.push(capability.as_ptr())
        }
        capabilities_c_str_ptrs.push(std::ptr::null());

        let ptr = if client_options.is_some() {
            client_options.unwrap().get_pointer()
        } else {
            std::ptr::null_mut()
        };

        let cap_ptr = capabilities_c_str_ptrs.as_mut_ptr();
        let result = unsafe {
            unsafe_bindings::instproxy_check_capabilities_match(
                self.pointer,
                cap_ptr,
                ptr,
                &mut res_plist,
            )
        }
        .into();
        if result != InstProxyError::Success {
            return Err(result);
        }
        Ok(res_plist.into())
    }

    /// Gets the path for an app's bundle ID
    /// # Arguments
    /// * `bundle_identifier` - The bundle identifier of the app
    /// # Returns
    /// The path as a string
    ///
    /// ***Verified:*** False
    pub fn get_path_for_bundle_identifier(
        &self,
        bundle_identifier: impl Into<String>,
    ) -> Result<String, InstProxyError> {
        let bundle_id = std::ffi::CString::new(bundle_identifier.into()).unwrap();
        // This is kinda horrifying, could use a refractor
        let to_fill = CString::new("").unwrap();
        let mut to_fill_bytes = to_fill.into_raw();
        let to_fill_ptr = &mut to_fill_bytes;

        info!("Instproxy get_path_for_bundle_identifier");
        let result = unsafe {
            unsafe_bindings::instproxy_client_get_path_for_bundle_identifier(
                self.pointer,
                bundle_id.as_ptr(),
                to_fill_ptr,
            )
        }
        .into();

        if result != InstProxyError::Success {
            return Err(result);
        }

        info!("Instproxy get_path_for_bundle_identifier done");
        Ok(unsafe { CString::from_raw(to_fill_bytes).into_string().unwrap() })
    }
}

/// The options that can be used when browsing installed apps
#[derive(PartialEq, Debug)]
pub enum BrowseOption {
    System,
    User,
    Internal,
    All,
    None,
}

impl From<BrowseOption> for Plist {
    fn from(option: BrowseOption) -> Self {
        let mut dict = Plist::new_dict();
        match option {
            BrowseOption::System => dict
                .dict_set_item("ApplicationType", "System".into())
                .unwrap(),
            BrowseOption::User => dict
                .dict_set_item("ApplicationType", "User".into())
                .unwrap(),
            BrowseOption::Internal => dict
                .dict_set_item("ApplicationType", "Internal".into())
                .unwrap(),
            BrowseOption::All => dict.dict_set_item("ApplicationType", "All".into()).unwrap(),
            BrowseOption::None => dict
                .dict_set_item("ApplicationType", "None".into())
                .unwrap(),
        }
        dict
    }
}

impl Drop for InstProxyClient<'_> {
    fn drop(&mut self) {
        info!("Dropping instproxy client");
        unsafe {
            unsafe_bindings::instproxy_client_free(self.pointer);
        }
    }
}

extern "C" {
    #[allow(clashing_extern_declarations)] // this one is better
    pub fn instproxy_client_options_set_return_attributes(
        client_options: unsafe_bindings::plist_t,
        key: *const ::std::os::raw::c_char,
        value: *const ::std::os::raw::c_char,
        null: *const u8,
    );
}
