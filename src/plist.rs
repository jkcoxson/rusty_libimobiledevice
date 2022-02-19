// jkcoxson

use std::{time::SystemTime, convert::TryInto};

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
    pub fn get_key_val(&self) -> String {
        let mut key = std::ptr::null_mut();
        unsafe {
            unsafe_bindings::plist_get_key_val(self.plist_t, &mut key)
        };
        let key = unsafe {
            std::ffi::CStr::from_ptr(key).to_string_lossy().into_owned()
        };
        key
    }
    pub fn get_string_val(&self) -> String {
        let mut val = std::ptr::null_mut();
        unsafe {
            unsafe_bindings::plist_get_string_val(self.plist_t, &mut val)
        };
        let val = unsafe {
            std::ffi::CStr::from_ptr(val).to_string_lossy().into_owned()
        };
        val
    }
    pub unsafe fn get_string_ptr(&self) -> *const i8 {
        unsafe_bindings::plist_get_string_ptr(self.plist_t, std::ptr::null_mut())
    }
    pub fn get_bool_val(&self) -> bool {
        let val = unsafe { std::mem::zeroed() };
        unsafe {
            unsafe_bindings::plist_get_bool_val(self.plist_t, val);
            match *val {
                0 => false,
                _ => true,
            }
        }
    }
    pub fn get_uint_val(&self) -> u64 {
        let val = unsafe { std::mem::zeroed() };
        unsafe {
            unsafe_bindings::plist_get_uint_val(self.plist_t, val);
            *val
        }
    }
    pub fn get_real_val(&self) -> f64 {
        let val = unsafe { std::mem::zeroed() };
        unsafe {
            unsafe_bindings::plist_get_real_val(self.plist_t, val);
            *val
        }
    }
    pub fn get_data_val(&self) -> Vec<i8> {
        let mut val = std::ptr::null_mut();
        let mut size = 0;
        unsafe {
            unsafe_bindings::plist_get_data_val(self.plist_t, &mut val, &mut size);
        }
        let val = unsafe {
            std::slice::from_raw_parts(val, size as usize)
        };
        val.to_vec()
    }
    pub fn get_date_val(&self) {
        unimplemented!();
    }
    pub fn get_uid_val(&self) -> u64 {
        let mut val = unsafe { std::mem::zeroed() };
        unsafe {
            unsafe_bindings::plist_get_uid_val(self.plist_t, &mut val);
        }
        val
    }
    pub fn set_key_val(&self, key: &str) {
        unsafe {
            unsafe_bindings::plist_set_key_val(self.plist_t, key.as_ptr() as *const i8)
        }
    }
    pub fn set_string_val(&self, val: &str) {
        unsafe {
            unsafe_bindings::plist_set_string_val(self.plist_t, val.as_ptr() as *const i8)
        }
    }
    pub fn set_bool_val(&self, val: bool) {
        let val = if val { 1 } else { 0 };
        unsafe {
            unsafe_bindings::plist_set_bool_val(self.plist_t, val)
        }
    }
    pub fn set_uint_val(&self, val: u64) {
        unsafe {
            unsafe_bindings::plist_set_uint_val(self.plist_t, val)
        }
    }
    pub fn set_real_val(&self, val: f64) {
        unsafe {
            unsafe_bindings::plist_set_real_val(self.plist_t, val)
        }
    }
    pub fn set_data_val(&self, val: &[i8]) {
        unsafe {
            unsafe_bindings::plist_set_data_val(self.plist_t, val.as_ptr(), val.len() as u64)
        }
    }
    pub fn set_date_val(&self) {
        unimplemented!();
    }
    pub fn set_uid_val(&self, val: u64) {
        unsafe {
            unsafe_bindings::plist_set_uid_val(self.plist_t, val)
        }
    }
    pub fn is_binary(&self) -> bool {
        let plist_data = unsafe {std::mem::zeroed() };
        let plist_len = unsafe {std::mem::zeroed() };
        unsafe {
            unsafe_bindings::plist_get_data_val(self.plist_t, plist_data, plist_len);
        }
        match unsafe {
            unsafe_bindings::plist_is_binary(*plist_data, (*plist_len).try_into().unwrap())
        } {
            0 => false,
            _ => true,
        }
    }
    pub fn access_path(&self, len: u32) -> Plist {
        let mut plist_t = std::ptr::null_mut();
        unsafe {
            unsafe_bindings::plist_access_path(self.plist_t, len, &mut plist_t);
        }
        Plist { plist_t }
    }

}

impl From<unsafe_bindings::plist_t> for Plist {
    fn from(plist_t: unsafe_bindings::plist_t) -> Self {
        Plist { plist_t }
    }
}

impl From<Plist> for String {
    fn from(plist: Plist) -> Self {
        let plist_t = plist.plist_t;
        let mut plist_data = std::ptr::null_mut();
        let mut plist_size = 0;
        unsafe {
            unsafe_bindings::plist_to_xml(
                plist_t,
                &mut plist_data,
                &mut plist_size
            );
            unsafe_bindings::plist_mem_free(plist.plist_t);
        }
        let plist_data = unsafe {
            std::slice::from_raw_parts(plist_data as *const u8, plist_size.try_into().unwrap())
        };
        let plist_data = std::str::from_utf8(plist_data).unwrap();
        let plist_data = String::from(plist_data);
        plist_data
    }
}

impl From<String> for Plist {
    fn from(plist_data: String) -> Self {
        let len = plist_data.len();
        let plist_data = plist_data.as_ptr() as *const i8;
        let plist_t = unsafe { std::mem::zeroed() };
        unsafe {
            unsafe_bindings::plist_from_xml(plist_data, len as u32, plist_t)
        };
        Plist { plist_t: unsafe {*plist_t} }
    }
}

impl From<Plist> for Vec<u8> {
    fn from(plist: Plist) -> Self {
        let plist_t = plist.plist_t;
        let mut plist_data = std::ptr::null_mut();
        let mut plist_size = 0;
        unsafe {
            unsafe_bindings::plist_to_bin(
                plist_t,
                &mut plist_data,
                &mut plist_size
            );
            unsafe_bindings::plist_mem_free(plist.plist_t);
        }
        let plist_data = unsafe {
            std::slice::from_raw_parts(plist_data as *const u8, plist_size.try_into().unwrap())
        };
        let plist_data = plist_data.to_vec();
        plist_data
    }
}

impl From<Vec<u8>> for Plist {
    fn from(plist_data: Vec<u8>) -> Self {
        let len = plist_data.len();
        let plist_data = plist_data.as_ptr() as *const i8;
        let plist_t = unsafe { std::mem::zeroed() };
        unsafe {
            unsafe_bindings::plist_from_bin(plist_data, len as u32, plist_t)
        };
        Plist { plist_t: unsafe {*plist_t} }
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

pub fn compare_node_values(node_l: Plist, node_r: Plist) -> bool {
    match unsafe {
        unsafe_bindings::plist_compare_node_value(node_l.plist_t, node_r.plist_t)
    }.to_string().as_str() {
        "TRUE" => true,
        _ => false
    }
}