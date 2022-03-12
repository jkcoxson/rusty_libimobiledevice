// jkcoxson

use std::ffi::CString;

use crate::{
    bindings as unsafe_bindings, error::InstProxyError, libimobiledevice::Device, plist::Plist,
};

pub struct InstProxyClient<'a> {
    pub(crate) pointer: unsafe_bindings::instproxy_client_t,
    pub label: String,
    phantom: std::marker::PhantomData<&'a Device>,
}

impl InstProxyClient<'_> {
    pub fn new(device: &Device, label: String) -> Result<Self, InstProxyError> {
        let mut instproxy_client = unsafe { std::mem::zeroed() };
        let label_c_str = std::ffi::CString::new(label.clone()).unwrap();
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

    pub fn options_new() -> Plist {
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
        mut client_options: Plist,
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
        let result = unsafe {
            unsafe_bindings::instproxy_lookup(
                self.pointer,
                cstring_pointers_ptr,
                client_options.plist_t,
                &mut res_plist,
            )
        }
        .into();
        if result != InstProxyError::Success {
            return Err(result);
        }

        // todo make this not a hack (which means it'll never happen)
        // This is because when the default plist impl drop fires, it will segfault on this specific plist type
        unsafe { unsafe_bindings::instproxy_client_options_free(client_options.plist_t) };
        std::mem::forget(client_options);

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

        Ok(unsafe { CString::from_raw(to_fill_bytes).into_string().unwrap() })
    }
}

impl Drop for InstProxyClient<'_> {
    fn drop(&mut self) {
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
