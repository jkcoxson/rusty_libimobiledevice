// jkcoxson

use std::{ffi::CString, ptr::null};

use crate::{bindings as unsafe_bindings, libimobiledevice::Device, error::InstProxyError, memory_lock::InstProxyLock, plist::Plist};

pub struct InstProxyClient {
    pub(crate) pointer: InstProxyLock,
    pub label: String,
}

impl InstProxyClient {
    pub fn new(device: &Device, label: String) -> Result<Self, InstProxyError> {
        let mut instproxy_client = unsafe { std::mem::zeroed() };
        let label_c_str = std::ffi::CString::new(label.clone()).unwrap();
        if let Ok(device_ptr) = device.pointer.check() {

            let result = unsafe {
                unsafe_bindings::instproxy_client_start_service(device_ptr, &mut instproxy_client, label_c_str.as_ptr())
            }.into();

            if result != InstProxyError::Success {
                return Err(result);
            }
            
            Ok(InstProxyClient {
                pointer: InstProxyLock::new(instproxy_client, device.pointer.clone()),
                label,
            })
        } else {
            return Err(InstProxyError::MissingObjectDepenency);
        }
    }

    pub fn options_new() -> Plist {
        unsafe {
            unsafe_bindings::instproxy_client_options_new()
        }.into() // insert sunglasses emoji
    }

    /// A rough translation of what I think the C library does.
    /// Rust doesn't support function overloading...
    pub fn options_add(options: &Plist, args: Vec<(String, Plist)>){
        for (key, value) in args {
            println!("key: {}\n\n", key);
            println!("value: {}\n\n", value.to_string());
            options.dict_set_item(&key, &value).unwrap();
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    /// A rough translation of what I think the C library does.
    /// Rust doesn't support function overloading...
    pub fn options_set_return_attributes(options: &Plist, args: Vec<String>) {
        let return_attributes = Plist::new_array();
        for i in args {
            let t = &Plist::new_string(&i);
            println!("Pushing {:?}\n\n", t.to_string());
            return_attributes.array_append_item(t).unwrap();
        }
        match options.dict_insert_item("ReturnAttributes", &return_attributes) {
            Ok(_) => {},
            Err(_) => panic!("It's brokey!"),
        };
    }

    pub fn lookup(&self, app_ids: Vec<String>, client_options: Plist) -> Result<Plist, InstProxyError> {
        if let Ok(pointer) = self.pointer.check() {
            // Convert vector of strings to a slice
            let cstrings = app_ids.iter().map(|s| std::ffi::CString::new(s.clone()).unwrap()).collect::<Vec<_>>();
            let mut cstring_pointers = cstrings.iter().map(|s| s.as_ptr()).collect::<Vec<_>>();
            cstring_pointers.push(std::ptr::null());
            let cstring_pointers_ptr = cstring_pointers.as_mut_ptr();

            let mut res_plist: unsafe_bindings::plist_t = unsafe { std::mem::zeroed() };
            let result = unsafe {
                unsafe_bindings::instproxy_lookup(pointer, &mut null(), client_options.plist_t, &mut res_plist)
            }.into();

            if result != InstProxyError::Success {
                return Err(result);
            }

            Ok(res_plist.into())
        } else {
            return Err(InstProxyError::MissingObjectDepenency);
        }
    }

    pub fn get_path_for_bundle_identifier(&self, bundle_identifier: String) -> Result<String, InstProxyError> {
        if let Ok(pointer) = self.pointer.check() {
            let bundle_id = std::ffi::CString::new(bundle_identifier).unwrap();
            // This is kinda horrifying, could use a refractor
            let to_fill = CString::new("").unwrap();   
            let mut to_fill_bytes = to_fill.into_raw();
            let to_fill_ptr = &mut to_fill_bytes;

            let result = unsafe {
                unsafe_bindings::instproxy_client_get_path_for_bundle_identifier(pointer, bundle_id.as_ptr(), to_fill_ptr)
            }.into();

            if result != InstProxyError::Success {
                return Err(result);
            }

            Ok(unsafe { CString::from_raw(to_fill_bytes).into_string().unwrap() })
        } else {
            return Err(InstProxyError::MissingObjectDepenency);
        }
    }
}


impl Drop for InstProxyClient {
    fn drop(&mut self) {
        if let Ok(ptr) = self.pointer.check() {
            unsafe {
                unsafe_bindings::instproxy_client_free(ptr);
            }
        }        
        self.pointer.invalidate();
    }
}

extern "C" {
    #[allow(clashing_extern_declarations)] // this one is better
    pub fn instproxy_client_options_set_return_attributes(client_options: unsafe_bindings::plist_t, key: *const ::std::os::raw::c_char, value: *const ::std::os::raw::c_char, null: *const u8);
}