// jkcoxson

use std::time::SystemTime;

use crate::libimobiledevice::*;

pub struct Plist {
    pub plist_t: unsafe_bindings::plist_t,
}

pub struct PlistArrayIter {
    plist_array_iter: unsafe_bindings::plist_array_iter,
    plist: Plist,
}

pub struct PlistDictIter {
    plist_dict_iter: unsafe_bindings::plist_dict_iter,
    plist: Plist,
}

pub enum PlistType {
    Boolean,
    Integer,
    Real,
    Date,
    Data,
    String,
    Array,
    Dictionary,
    Unknown,
    Key,
    Uid,
    None,
}

impl Plist {
    // New plist functions
    pub fn new_dict() -> Plist {
        let plist_t = unsafe {
            unsafe_bindings::plist_new_dict()
        };
        Plist { plist_t }
    }
    pub fn new_array() -> Plist {
        let plist_t = unsafe {
            unsafe_bindings::plist_new_array()
        };
        Plist { plist_t }
    }
    pub fn new_string(string: &str) -> Plist {
        let plist_t = unsafe {
            unsafe_bindings::plist_new_string(string.as_ptr() as *const i8)
        };
        Plist { plist_t }
    }
    pub fn new_bool(bool: bool) -> Plist {
        let plist_t = unsafe {
            unsafe_bindings::plist_new_bool(match bool == true {
                true => 1,
                false => 0,
            })
        };
        Plist { plist_t }
    }
    pub fn new_uint(uint: u64) -> Plist {
        let plist_t = unsafe {
            unsafe_bindings::plist_new_uint(uint)
        };
        Plist { plist_t }
    }
    pub fn new_real(real: f64) -> Plist {
        let plist_t = unsafe {
            unsafe_bindings::plist_new_real(real)
        };
        Plist { plist_t }
    }
    pub fn new_data(data: &[u8]) -> Plist {
        let plist_t = unsafe {
            unsafe_bindings::plist_new_data(data.as_ptr() as *const i8, std::convert::TryInto::try_into(data.len()).unwrap())
        };
        Plist { plist_t }
    }
    pub fn new_date(_date: SystemTime) -> Plist {
        unimplemented!() // I am too tired to implement this right now
    }
    pub fn new_plist_uid(uid: u64) -> Plist {
        let plist_t = unsafe {
            unsafe_bindings::plist_new_uid(uid)
        };
        Plist { plist_t }
    }

    // Getters
    pub fn array_get_size(&self) -> u32 {
        unsafe {
            unsafe_bindings::plist_array_get_size(self.plist_t)
        }
    }
    pub fn array_get_item(&self, index: u32) -> Plist {
        let plist_t = unsafe {
            unsafe_bindings::plist_array_get_item(self.plist_t, index)
        };
        Plist { plist_t }
    }
    pub fn array_get_item_index(&self) -> u32 {
        unsafe {
            unsafe_bindings::plist_array_get_item_index(self.plist_t) // ???
        }
    }

    // Setters
    pub fn array_set_item(&self, item: &Plist, index: u32) {
        unsafe {
            unsafe_bindings::plist_array_set_item(self.plist_t, item.plist_t, index)
        }
    }
    pub fn array_append_item(&self, item: &Plist) {
        unsafe {
            unsafe_bindings::plist_array_append_item(self.plist_t, item.plist_t)
        }
    }
    pub fn array_insert_item(&self, item: &Plist, index: u32) {
        unsafe {
            unsafe_bindings::plist_array_insert_item(self.plist_t, item.plist_t, index)
        }
    }
    pub fn array_remove_item(&self, index: u32) {
        unsafe {
            unsafe_bindings::plist_array_remove_item(self.plist_t, index)
        }
    }
    pub fn array_item_remove(&self) {
        unsafe {
            unsafe_bindings::plist_array_item_remove(self.plist_t)
        }
    }
    pub fn dict_get_size(&self) -> u32 {
        unsafe {
            unsafe_bindings::plist_dict_get_size(self.plist_t)
        }
    }
    pub fn dict_get_item_key(&self) -> String {
        let mut key = std::ptr::null_mut();
        unsafe {
            unsafe_bindings::plist_dict_get_item_key(self.plist_t, &mut key)
        };
        let key = unsafe {
            std::ffi::CStr::from_ptr(key).to_string_lossy().into_owned()
        };
        key
    }
    pub fn dict_get_item(&self, key: &str) -> Plist {
        let plist_t = unsafe {
            unsafe_bindings::plist_dict_get_item(self.plist_t, key.as_ptr() as *const i8)
        };
        Plist { plist_t }
    }
    pub fn dict_item_get_key(&self) -> Plist {
        let plist_t = unsafe {
            unsafe_bindings::plist_dict_item_get_key(self.plist_t)
        };
        Plist { plist_t }
    }
    pub fn dict_set_item(&self, key: &str, item: &Plist) {
        unsafe {
            unsafe_bindings::plist_dict_set_item(self.plist_t, key.as_ptr() as *const i8, item.plist_t)
        }
    }
    pub fn dict_insert_item(&self, key: &str, item: &Plist) {
        unsafe {
            unsafe_bindings::plist_dict_insert_item(self.plist_t, key.as_ptr() as *const i8, item.plist_t)
        }
    }
    pub fn dict_remove_item(&self, key: &str) {
        unsafe {
            unsafe_bindings::plist_dict_remove_item(self.plist_t, key.as_ptr() as *const i8)
        }
    }
    pub fn dict_merge(&mut self, dict: &Plist) {
        unsafe {
            unsafe_bindings::plist_dict_merge(&mut self.plist_t, dict.plist_t)
        }
    }
    pub fn get_parent(&self) -> Plist {
        let plist_t = unsafe {
            unsafe_bindings::plist_get_parent(self.plist_t)
        };
        Plist { plist_t }
    }
    pub fn get_node_type(&self) -> PlistType {
        unsafe {
            unsafe_bindings::plist_get_node_type(self.plist_t)
        }.into() // puts on sunglasses
    }
}

impl From<unsafe_bindings::plist_t> for Plist {
    fn from(plist_t: unsafe_bindings::plist_t) -> Self {
        Plist { plist_t }
    }
}

impl Drop for Plist {
    fn drop(&mut self) {
        unsafe {
            unsafe_bindings::plist_free(self.plist_t)
        }
    }
}

impl PlistArrayIter {
    pub fn next_item(&mut self) -> Option<Plist> {
        let to_fill = unsafe { std::mem::zeroed() };
        unsafe {
            unsafe_bindings::plist_array_next_item(self.plist.plist_t, self.plist_array_iter, to_fill)
        };
        if to_fill.is_null() {
            None
        } else {
            Some(Plist { plist_t: unsafe { *to_fill } }) // yeet
        }
    }
}

impl From<Plist> for PlistArrayIter {
    fn from(plist: Plist) -> Self {
        let mut plist_array_iter = unsafe {
            std::mem::zeroed()
        };
        unsafe {
            unsafe_bindings::plist_array_new_iter(plist.plist_t, &mut plist_array_iter)
        };
        PlistArrayIter { plist_array_iter, plist }
    }
}

impl PlistDictIter {
    pub fn next_item(&mut self) -> Option<(String, Plist)> {
        let key = unsafe { std::mem::zeroed() };
        let to_fill = unsafe { std::mem::zeroed() };
        unsafe {
            unsafe_bindings::plist_dict_next_item(self.plist.plist_t, self.plist_dict_iter, key, to_fill)
        };
        if to_fill.is_null() {
            None
        } else {
            let key_str = unsafe {
                std::ffi::CStr::from_ptr(*key).to_string_lossy().into_owned()
            };
            Some((key_str, Plist { plist_t: unsafe { *to_fill } }))
        }
    }
}

impl From<Plist> for PlistDictIter {
    fn from(plist: Plist) -> Self {
        let mut plist_dict_iter = unsafe {
            std::mem::zeroed()
        };
        unsafe {
            unsafe_bindings::plist_dict_new_iter(plist.plist_t, &mut plist_dict_iter)
        };
        PlistDictIter { plist_dict_iter, plist }
    }
}

impl From<u32> for PlistType {
    fn from(i: u32) -> Self {
        match i {
            0 => PlistType::Boolean,
            1 => PlistType::Integer,
            2 => PlistType::Real,
            3 => PlistType::String,
            4 => PlistType::Array,
            5 => PlistType::Dictionary,
            6 => PlistType::Date,
            7 => PlistType::Data,
            8 => PlistType::Key,
            9 => PlistType::Uid,
            10 => PlistType::None,
            _ => PlistType::Unknown
        }
    }
}