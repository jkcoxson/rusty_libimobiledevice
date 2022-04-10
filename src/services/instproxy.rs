// jkcoxson

use std::{ffi::CString, os::raw::c_char};

use crate::{
    bindings as unsafe_bindings, debug, error::InstProxyError, idevice::Device,
};

use plist_plus::Plist;

pub struct InstProxyClient<'a> {
    pub(crate) pointer: unsafe_bindings::instproxy_client_t,
    pub label: String,
    phantom: std::marker::PhantomData<&'a Device>,
}

unsafe impl Send for InstProxyClient<'_> {}
unsafe impl Sync for InstProxyClient<'_> {}

impl InstProxyClient<'_> {
    pub fn new(device: &Device, label: String) -> Result<Self, InstProxyError> {
        let mut instproxy_client = unsafe { std::mem::zeroed() };
        let label_c_str = std::ffi::CString::new(label.clone()).unwrap();
        debug!("Creating instproxy client for {}", device.get_udid());
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

    pub fn browse(&self, option: BrowseOption) -> Result<Plist, InstProxyError> {
        let mut plist = std::ptr::null_mut();

        let result = if option == BrowseOption::None {
            unsafe {
                unsafe_bindings::instproxy_browse(self.pointer, std::ptr::null_mut(), &mut plist)
            }
        } else {
            let option_plist: Plist = option.into();
            unsafe {
                unsafe_bindings::instproxy_browse(self.pointer, option_plist.get_pointer(), &mut plist)
            }
        }
        .into();

        if result != InstProxyError::Success {
            return Err(result);
        }

        Ok(plist.into())
    }

    pub fn options_new() -> Plist {
        debug!("Generating new options plist");
        unsafe { unsafe_bindings::instproxy_client_options_new() }.into() // insert sunglasses emoji
    }

    /// A rough translation of what I think the C library does.
    /// Rust doesn't support function overloading...
    pub fn options_add(options: &mut Plist, args: Vec<(String, Plist)>) {
        for (key, value) in args {
            options.dict_set_item(&key, value).unwrap();
        }
    }

    /// A rough translation of what I think the C library does.
    /// Rust doesn't support function overloading...
    pub fn options_set_return_attributes(options: &mut Plist, args: Vec<String>) {
        debug!("Setting return attributes");
        let mut return_attributes = Plist::new_array();
        for i in args {
            let t = Plist::new_string(&i);
            return_attributes.array_append_item(t).unwrap();
        }
        match options.dict_insert_item("ReturnAttributes", return_attributes) {
            Ok(_) => {}
            Err(_) => panic!("It's brokey!"),
        };
    }

    pub fn lookup(
        &self,
        app_ids: Vec<String>,
        client_options: Plist,
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

        let mut res_plist: unsafe_bindings::plist_t = unsafe { std::mem::zeroed() };
        debug!("Instproxy lookup");
        let result = unsafe {
            unsafe_bindings::instproxy_lookup(
                self.pointer,
                cstring_pointers_ptr,
                client_options.get_pointer(),
                &mut res_plist,
            )
        }
        .into();
        if result != InstProxyError::Success {
            return Err(result);
        }

        // todo make this not a hack (which means it'll never happen)
        // This is because when the default plist impl drop fires, it will segfault on this specific plist type
        debug!("Hack dropping the options");
        unsafe { unsafe_bindings::instproxy_client_options_free(client_options.get_pointer()) };
        std::mem::forget(client_options);

        debug!("Instproxy lookup done");
        Ok(res_plist.into())
    }

    pub fn install(&self, pkg_path: String, client_options: Plist) -> Result<(), InstProxyError> {
        let pkg_path_c_str = std::ffi::CString::new(pkg_path).unwrap();
        debug!("Instproxy install");
        let result = unsafe {
            unsafe_bindings::instproxy_install(
                self.pointer,
                pkg_path_c_str.as_ptr(),
                client_options.get_pointer(),
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

    pub fn upgrade(&self, pkg_path: String, client_options: Plist) -> Result<(), InstProxyError> {
        let pkg_path_c_str = std::ffi::CString::new(pkg_path).unwrap();
        debug!("Instproxy upgrade");
        let result = unsafe {
            unsafe_bindings::instproxy_upgrade(
                self.pointer,
                pkg_path_c_str.as_ptr(),
                client_options.get_pointer(),
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

    pub fn uninstall(&self, app_id: String, client_options: Plist) -> Result<(), InstProxyError> {
        let app_id_c_str = std::ffi::CString::new(app_id).unwrap();
        debug!("Instproxy uninstall");
        let result = unsafe {
            unsafe_bindings::instproxy_uninstall(
                self.pointer,
                app_id_c_str.as_ptr(),
                client_options.get_pointer(),
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

    pub fn lookup_archives(&self, client_options: Plist) -> Result<Plist, InstProxyError> {
        let mut res_plist: unsafe_bindings::plist_t = unsafe { std::mem::zeroed() };
        debug!("Instproxy lookup archives");
        let result = unsafe {
            unsafe_bindings::instproxy_lookup_archives(
                self.pointer,
                client_options.get_pointer(),
                &mut res_plist,
            )
        }
        .into();
        if result != InstProxyError::Success {
            return Err(result);
        }
        Ok(res_plist.into())
    }

    pub fn archive(&self, app_id: String, client_options: Plist) -> Result<(), InstProxyError> {
        let app_id_c_str = std::ffi::CString::new(app_id).unwrap();
        debug!("Instproxy archive");
        let result = unsafe {
            unsafe_bindings::instproxy_archive(
                self.pointer,
                app_id_c_str.as_ptr(),
                client_options.get_pointer(),
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

    pub fn restore(&self, app_id: String, client_options: Plist) -> Result<(), InstProxyError> {
        let app_id_c_str = std::ffi::CString::new(app_id).unwrap();
        debug!("Instproxy restore");
        let result = unsafe {
            unsafe_bindings::instproxy_restore(
                self.pointer,
                app_id_c_str.as_ptr(),
                client_options.get_pointer(),
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

    pub fn remove_archive(
        &self,
        app_id: String,
        client_options: Plist,
    ) -> Result<(), InstProxyError> {
        let app_id_c_str = std::ffi::CString::new(app_id).unwrap();
        debug!("Instproxy remove archive");
        let result = unsafe {
            unsafe_bindings::instproxy_remove_archive(
                self.pointer,
                app_id_c_str.as_ptr(),
                client_options.get_pointer(),
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

    pub fn check_capabilities_match(
        &self,
        capabilities: Vec<String>,
        client_options: Plist,
    ) -> Result<Plist, InstProxyError> {
        let mut res_plist = unsafe { std::mem::zeroed() };
        let mut capabilities_c_str = vec![];
        for capability in capabilities {
            capabilities_c_str.push(std::ffi::CString::new(capability).unwrap());
        }
        let capabilities_c_str_ptr = capabilities_c_str.as_mut_ptr();
        let cap_ptr_ptr = capabilities_c_str_ptr as *mut *const c_char;
        let result = unsafe {
            unsafe_bindings::instproxy_check_capabilities_match(
                self.pointer,
                cap_ptr_ptr,
                client_options.get_pointer(),
                &mut res_plist,
            )
        }
        .into();
        if result != InstProxyError::Success {
            return Err(result);
        }
        Ok(res_plist.into())
    }

    pub fn get_path_for_bundle_identifier(
        &self,
        bundle_identifier: String,
    ) -> Result<String, InstProxyError> {
        let bundle_id = std::ffi::CString::new(bundle_identifier).unwrap();
        // This is kinda horrifying, could use a refractor
        let to_fill = CString::new("").unwrap();
        let mut to_fill_bytes = to_fill.into_raw();
        let to_fill_ptr = &mut to_fill_bytes;

        debug!("Instproxy get_path_for_bundle_identifier");
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

        debug!("Instproxy get_path_for_bundle_identifier done");
        Ok(unsafe { CString::from_raw(to_fill_bytes).into_string().unwrap() })
    }
}

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
        debug!("Dropping instproxy client");
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
