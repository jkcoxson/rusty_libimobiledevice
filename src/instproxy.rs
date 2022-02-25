// jkcoxson

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
            options.dict_set_item(&key, &value).unwrap();
        }
    }

    /// A rough translation of what I think the C library does.
    /// Rust doesn't support function overloading...
    pub fn options_set_return_attributes(options: &Plist, args: Vec<String>) {
        let return_attributes = Plist::new_array();
        for i in args {
            return_attributes.array_append_item(&i.into()).unwrap();
        }
        options.dict_set_item("ReturnAttributes", &return_attributes).unwrap();
    }

    pub fn lookup(&self, app_ids: Vec<String>, client_options: Plist) -> Result<Plist, InstProxyError> {
        if let Ok(pointer) = self.pointer.check() {
            let mut plist = unsafe { std::mem::zeroed() };
            let mut raw_app_ids = vec![];
            let mut id_pointers = vec![];
            for i in app_ids {
                let raw = std::ffi::CString::new(i).unwrap();
                let ptr = raw.as_ptr();
                raw_app_ids.push(raw);
                id_pointers.push(ptr);
            }
            let result = unsafe {
                unsafe_bindings::instproxy_lookup(pointer, id_pointers.as_mut_ptr(), client_options.plist_t, &mut plist)
            }.into();

            if result != InstProxyError::Success {
                return Err(result)
            }
            return Ok(plist.into())
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