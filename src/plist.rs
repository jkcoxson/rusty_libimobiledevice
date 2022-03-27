// jkcoxson

use std::{convert::TryInto, ffi::CString, fmt::Formatter, time::SystemTime};

use crate::bindings as unsafe_bindings;
use crate::debug;
use rand::Rng;

pub struct Plist {
    pub(crate) plist_t: unsafe_bindings::plist_t,
    pub plist_type: PlistType,
    pub(crate) dependent_plists: Vec<unsafe_bindings::plist_t>,
    pub(crate) id: u32,
}

unsafe impl Send for Plist {}
unsafe impl Sync for Plist {}

pub struct PlistArrayIter {
    plist_array_iter: unsafe_bindings::plist_array_iter,
    plist: Plist,
}

unsafe impl Send for PlistDictIter {}
unsafe impl Sync for PlistDictIter {}

pub struct PlistDictIter {
    plist_dict_iter: unsafe_bindings::plist_dict_iter,
    plist: Plist,
}

#[derive(PartialEq, Debug)]
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

impl From<PlistType> for String {
    fn from(plist_type: PlistType) -> String {
        match plist_type {
            PlistType::Boolean => "Boolean".to_string(),
            PlistType::Integer => "Integer".to_string(),
            PlistType::Real => "Real".to_string(),
            PlistType::Date => "Date".to_string(),
            PlistType::Data => "Data".to_string(),
            PlistType::String => "String".to_string(),
            PlistType::Array => "Array".to_string(),
            PlistType::Dictionary => "Dictionary".to_string(),
            PlistType::Unknown => "Unknown".to_string(),
            PlistType::Key => "Key".to_string(),
            PlistType::Uid => "Uid".to_string(),
            PlistType::None => "None".to_string(),
        }
    }
}

impl Plist {
    pub fn new_dict() -> Plist {
        debug!("Generating new dictionary plist");
        unsafe { unsafe_bindings::plist_new_dict() }.into()
    }
    pub fn new_array() -> Plist {
        debug!("Generating new array plist");
        unsafe { unsafe_bindings::plist_new_array() }.into()
    }
    pub fn new_string(string: &str) -> Plist {
        debug!("Generating new string plist");
        let string = match CString::new(string) {
            Ok(s) => s,
            Err(_) => {
                panic!("Could not convert string to CString");
            }
        };
        unsafe { unsafe_bindings::plist_new_string(string.as_ptr() as *const i8) }.into()
    }
    pub fn new_bool(bool: bool) -> Plist {
        debug!("Generating new bool plist");
        unsafe {
            unsafe_bindings::plist_new_bool(match bool == true {
                true => 1,
                false => 0,
            })
        }
        .into()
    }
    pub fn new_uint(uint: u64) -> Plist {
        debug!("Generating new uint plist");
        unsafe { unsafe_bindings::plist_new_uint(uint) }.into()
    }
    pub fn new_real(real: f64) -> Plist {
        debug!("Generating new float plist");
        unsafe { unsafe_bindings::plist_new_real(real) }.into()
    }
    pub fn new_data(data: &[u8]) -> Plist {
        debug!("Generating new data plist");
        unsafe {
            unsafe_bindings::plist_new_data(
                data.as_ptr() as *const i8,
                std::convert::TryInto::try_into(data.len()).unwrap(),
            )
        }
        .into()
    }
    pub fn new_date(_date: SystemTime) -> Plist {
        unimplemented!() // I am too tired to implement this right now
                         // Turns out I'm too tired to impliment this ever
    }
    pub fn new_plist_uid(uid: u64) -> Plist {
        debug!("Generating new plist uid");
        unsafe { unsafe_bindings::plist_new_uid(uid) }.into()
    }
    pub fn from_xml(xml: String) -> Result<Plist, ()> {
        let xml = match CString::new(xml) {
            Ok(s) => s,
            Err(_) => {
                debug!("Could not convert string to CString");
                return Err(());
            }
        };
        let xml_len = std::convert::TryInto::try_into(xml.as_bytes().len()).unwrap();
        let mut plist_t = unsafe { std::mem::zeroed() };
        debug!("Parsing xml");
        unsafe {
            unsafe_bindings::plist_from_xml(xml.as_ptr() as *const i8, xml_len, &mut plist_t)
        };
        Ok(plist_t.into())
    }
    pub fn array_get_size(&self) -> Result<u32, ()> {
        if self.plist_type != PlistType::Array {
            return Err(());
        }
        debug!("Getting array size");
        Ok(unsafe { unsafe_bindings::plist_array_get_size(self.plist_t) })
    }
    pub fn array_get_item(&self, index: u32) -> Result<Plist, ()> {
        if self.plist_type != PlistType::Array {
            return Err(());
        }
        debug!("Getting array item");
        Ok(unsafe { unsafe_bindings::plist_array_get_item(self.plist_t, index) }.into())
    }
    pub fn array_get_item_index(&self) -> Result<u32, ()> {
        if self.plist_type != PlistType::Array {
            return Err(());
        }
        debug!("Getting array item index");
        Ok(unsafe {
            unsafe_bindings::plist_array_get_item_index(self.plist_t) // ???
        })
    }

    pub fn array_set_item(&mut self, item: Plist, index: u32) -> Result<(), ()> {
        if self.plist_type != PlistType::Array {
            return Err(());
        }
        debug!("Setting array item");
        unsafe { unsafe_bindings::plist_array_set_item(self.plist_t, item.plist_t, index) };
        self.dependent_plists.push(item.plist_t);
        item.false_drop();
        Ok(())
    }
    pub fn array_append_item(&mut self, item: Plist) -> Result<(), ()> {
        if self.plist_type != PlistType::Array {
            return Err(());
        }
        debug!("Appending array item");
        unsafe { unsafe_bindings::plist_array_append_item(self.plist_t, item.plist_t) };
        self.dependent_plists.push(item.plist_t);
        item.false_drop();
        Ok(())
    }
    pub fn array_insert_item(&mut self, item: Plist, index: u32) -> Result<(), ()> {
        if self.plist_type != PlistType::Array {
            return Err(());
        }
        debug!("Inserting array item");
        unsafe { unsafe_bindings::plist_array_insert_item(self.plist_t, item.plist_t, index) }
        self.dependent_plists.push(item.plist_t);
        item.false_drop();
        Ok(())
    }
    pub fn array_remove_item(&self, index: u32) -> Result<(), ()> {
        if self.plist_type != PlistType::Array {
            return Err(());
        }
        debug!("Removing array item");
        unsafe { unsafe_bindings::plist_array_remove_item(self.plist_t, index) };
        Ok(())
    }
    pub fn array_item_remove(&self) -> Result<(), ()> {
        if self.plist_type != PlistType::Array {
            return Err(());
        }
        debug!("Removing array item");
        unsafe { unsafe_bindings::plist_array_item_remove(self.plist_t) }
        Ok(())
    }
    pub fn dict_get_size(&self) -> Result<u32, ()> {
        if self.plist_type != PlistType::Dictionary {
            return Err(());
        }
        debug!("Getting dict size");
        Ok(unsafe { unsafe_bindings::plist_dict_get_size(self.plist_t) })
    }
    pub fn dict_get_item_key(&self) -> Result<String, ()> {
        if self.plist_type != PlistType::Dictionary {
            return Err(());
        }
        let mut key = std::ptr::null_mut();
        debug!("Getting dict item key");
        unsafe { unsafe_bindings::plist_dict_get_item_key(self.plist_t, &mut key) };
        debug!("Converting key to string");
        let key = unsafe { std::ffi::CStr::from_ptr(key).to_string_lossy().into_owned() };
        Ok(key)
    }
    pub fn dict_get_item(&self, key: &str) -> Result<Plist, ()> {
        if self.plist_type != PlistType::Dictionary {
            return Err(());
        }
        let key_c_string = CString::new(key).unwrap();
        debug!("Getting dict item");
        Ok(
            unsafe { unsafe_bindings::plist_dict_get_item(self.plist_t, key_c_string.as_ptr()) }
                .into(),
        )
    }
    pub fn dict_item_get_key(&self) -> Result<Plist, ()> {
        if self.plist_type != PlistType::Dictionary {
            return Err(());
        }
        debug!("Getting dict item key");
        Ok(unsafe { unsafe_bindings::plist_dict_item_get_key(self.plist_t) }.into())
    }
    pub fn dict_set_item(&mut self, key: &str, item: Plist) -> Result<(), ()> {
        let key = CString::new(key).unwrap();
        if self.plist_type != PlistType::Dictionary {
            return Err(());
        }
        debug!("Setting dict item");
        unsafe { unsafe_bindings::plist_dict_set_item(self.plist_t, key.as_ptr(), item.plist_t) }
        self.dependent_plists.push(item.plist_t);
        item.false_drop();
        Ok(())
    }
    pub fn dict_insert_item(&mut self, key: &str, item: Plist) -> Result<(), ()> {
        let key = CString::new(key).unwrap();
        if self.plist_type != PlistType::Dictionary {
            return Err(());
        }
        debug!("Inserting dict item");
        unsafe {
            unsafe_bindings::plist_dict_insert_item(
                self.plist_t,
                key.as_ptr() as *const i8,
                item.plist_t,
            )
        }
        self.dependent_plists.push(item.plist_t);
        item.false_drop();
        Ok(())
    }
    pub fn dict_remove_item(&self, key: &str) -> Result<(), ()> {
        let key = CString::new(key).unwrap();
        if self.plist_type != PlistType::Dictionary {
            return Err(());
        }
        debug!("Removing dict item");
        unsafe { unsafe_bindings::plist_dict_remove_item(self.plist_t, key.as_ptr() as *const i8) }
        Ok(())
    }
    pub fn dict_merge(&mut self, dict: Plist) -> Result<(), ()> {
        if self.plist_type != PlistType::Dictionary {
            return Err(());
        }
        debug!("Merging dict");
        unsafe { unsafe_bindings::plist_dict_merge(&mut self.plist_t, dict.plist_t) }
        self.dependent_plists.push(dict.plist_t);
        dict.false_drop();
        Ok(())
    }
    pub fn get_parent(&self) -> Plist {
        debug!("Getting parent");
        unsafe { unsafe_bindings::plist_get_parent(self.plist_t) }.into()
    }
    pub fn get_node_type(&self) -> PlistType {
        debug!("Getting node type");
        unsafe { unsafe_bindings::plist_get_node_type(self.plist_t) }.into() // puts on sunglasses
    }
    pub fn get_key_val(&self) -> Result<String, ()> {
        if self.plist_type != PlistType::Key {
            return Err(());
        }
        let mut key = std::ptr::null_mut();
        debug!("Getting key value");
        unsafe { unsafe_bindings::plist_get_key_val(self.plist_t, &mut key) };
        debug!("Converting key to string");
        let key = unsafe { std::ffi::CStr::from_ptr(key).to_string_lossy().into_owned() };
        Ok(key)
    }
    pub fn get_string_val(&self) -> Result<String, ()> {
        if self.plist_type != PlistType::String {
            return Err(());
        }
        let mut val = std::ptr::null_mut();
        debug!("Getting string value");
        unsafe { unsafe_bindings::plist_get_string_val(self.plist_t, &mut val) };
        debug!("Converting cstring to string");
        let val = unsafe { std::ffi::CStr::from_ptr(val).to_string_lossy().into_owned() };
        Ok(val)
    }
    /// Don't use this unless you want to shoot yourself in the foot
    pub fn get_string_ptr(&self) -> *const i8 {
        unsafe { unsafe_bindings::plist_get_string_ptr(self.plist_t, std::ptr::null_mut()) }
    }
    pub fn get_bool_val(&self) -> Result<bool, ()> {
        if self.plist_type != PlistType::Boolean {
            return Err(());
        }
        let val = unsafe { std::mem::zeroed() };
        debug!("Getting bool value");
        Ok(unsafe {
            unsafe_bindings::plist_get_bool_val(self.plist_t, val);
            match *val {
                0 => false,
                _ => true,
            }
        })
    }
    pub fn get_uint_val(&self) -> Result<u64, ()> {
        if self.plist_type != PlistType::Integer {
            return Err(());
        }
        let val = unsafe { std::mem::zeroed() };
        debug!("Getting uint value");
        Ok(unsafe {
            unsafe_bindings::plist_get_uint_val(self.plist_t, val);
            *val
        })
    }
    pub fn get_real_val(&self) -> Result<f64, ()> {
        let val = unsafe { std::mem::zeroed() };
        debug!("Getting float value");
        Ok(unsafe {
            unsafe_bindings::plist_get_real_val(self.plist_t, val);
            *val
        })
    }
    pub fn get_data_val(&self) -> Result<Vec<i8>, ()> {
        let mut val = std::ptr::null_mut();
        let mut size = 0;
        debug!("Getting data value");
        unsafe {
            unsafe_bindings::plist_get_data_val(self.plist_t, &mut val, &mut size);
        }
        let val = unsafe { std::slice::from_raw_parts(val, size as usize) };
        Ok(val.to_vec())
    }
    pub fn get_date_val(&self) {
        unimplemented!();
    }
    pub fn get_uid_val(&self) -> Result<u64, ()> {
        let mut val = unsafe { std::mem::zeroed() };
        debug!("Getting uid value");
        unsafe {
            unsafe_bindings::plist_get_uid_val(self.plist_t, &mut val);
        }
        Ok(val)
    }

    // These don't need type checks because they set the type of the plist

    pub fn set_key_val(&self, key: &str) {
        let key = CString::new(key).unwrap();
        debug!("Setting key value");
        unsafe { unsafe_bindings::plist_set_key_val(self.plist_t, key.as_ptr() as *const i8) }
    }
    pub fn set_string_val(&self, val: &str) {
        let val = CString::new(val).unwrap();
        debug!("Setting string value");
        unsafe { unsafe_bindings::plist_set_string_val(self.plist_t, val.as_ptr() as *const i8) }
    }
    pub fn set_bool_val(&self, val: bool) {
        let val = if val { 1 } else { 0 };
        debug!("Setting bool value");
        unsafe { unsafe_bindings::plist_set_bool_val(self.plist_t, val) }
    }
    pub fn set_uint_val(&self, val: u64) {
        debug!("Setting uint value");
        unsafe { unsafe_bindings::plist_set_uint_val(self.plist_t, val) }
    }
    pub fn set_real_val(&self, val: f64) {
        debug!("Setting float value");
        unsafe { unsafe_bindings::plist_set_real_val(self.plist_t, val) }
    }
    pub fn set_data_val(&self, val: &[i8]) {
        debug!("Setting data value");
        unsafe { unsafe_bindings::plist_set_data_val(self.plist_t, val.as_ptr(), val.len() as u64) }
    }
    pub fn set_date_val(&self) {
        unimplemented!();
    }
    pub fn set_uid_val(&self, val: u64) {
        debug!("Setting uid value");
        unsafe { unsafe_bindings::plist_set_uid_val(self.plist_t, val) }
    }

    pub fn is_binary(&self) -> bool {
        let plist_data = unsafe { std::mem::zeroed() };
        let plist_len = unsafe { std::mem::zeroed() };
        debug!("Getting plist data");
        unsafe {
            unsafe_bindings::plist_get_data_val(self.plist_t, plist_data, plist_len);
        }
        debug!("Checking if plist is binary");
        match unsafe {
            unsafe_bindings::plist_is_binary(*plist_data, (*plist_len).try_into().unwrap())
        } {
            0 => false,
            _ => true,
        }
    }

    /// Reimplimented from the C function because function overloading is evil
    pub fn access_path(self, plists: Vec<String>) -> Result<Plist, ()> {
        let mut current = self;
        let mut i = 0;
        while i < plists.len() {
            match current.plist_type {
                PlistType::Array => {
                    current = match current.array_get_item(i as u32) {
                        Ok(item) => item,
                        Err(_) => return Err(()),
                    };
                }
                PlistType::Dictionary => {
                    current = match current.dict_get_item(&plists[i]) {
                        Ok(item) => item,
                        Err(_) => return Err(()),
                    };
                }
                _ => {
                    return Err(());
                }
            }
            i += 1;
        }
        Ok(current.plist_t.into()) // Probably really stupid
    }

    /// This is necessary when a function absorbs another plist.
    /// That way, the rest of the plist struct is dropped, but the pointer is not.
    /// This prevents many segfaults.
    pub fn false_drop(mut self) {
        debug!("False dropping {}", self.id);
        let replacement = unsafe { unsafe_bindings::plist_new_bool(0) };
        self.plist_t = replacement;
    }
}

impl From<unsafe_bindings::plist_t> for Plist {
    fn from(plist_t: unsafe_bindings::plist_t) -> Self {
        let mut rng = rand::thread_rng();
        let id = rng.gen::<u32>();
        debug!("Creating plist from plist_t with id {}", id);
        Plist {
            plist_t,
            plist_type: unsafe { unsafe_bindings::plist_get_node_type(plist_t) }.into(),
            dependent_plists: Vec::new(),
            id: id,
        }
    }
}

impl From<Plist> for bool {
    fn from(plist: Plist) -> Self {
        plist.get_bool_val().expect("Expected boolean type")
    }
}

impl From<Plist> for u64 {
    fn from(plist: Plist) -> Self {
        plist.get_uint_val().expect("Expected integer type")
    }
}

impl From<Plist> for f64 {
    fn from(plist: Plist) -> Self {
        plist.get_real_val().expect("Expected float type")
    }
}

impl From<Plist> for String {
    fn from(plist: Plist) -> Self {
        let plist_t = plist.plist_t;
        let mut plist_data = std::ptr::null_mut();
        let mut plist_size = 0;
        debug!("Converting plist to XML data");
        unsafe {
            unsafe_bindings::plist_to_xml(plist_t, &mut plist_data, &mut plist_size);
        }
        debug!("Assembling XML data");
        let plist_data = unsafe {
            std::slice::from_raw_parts(plist_data as *const u8, plist_size.try_into().unwrap())
        };
        let plist_data = std::str::from_utf8(plist_data).unwrap();
        let plist_data = String::from(plist_data);
        plist_data
    }
}

impl ToString for Plist {
    fn to_string(&self) -> String {
        let plist_t = self.plist_t;
        let mut plist_data = std::ptr::null_mut();
        let mut plist_size = 0;
        debug!("Converting plist to XML data");
        unsafe {
            unsafe_bindings::plist_to_xml(plist_t, &mut plist_data, &mut plist_size);
        }
        debug!("Assembling XML data");
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
        let s = Plist::new_string(&plist_data);
        s
    }
}

impl From<&String> for Plist {
    fn from(plist_data: &String) -> Self {
        Plist::new_string(plist_data)
    }
}

impl From<&str> for Plist {
    fn from(plist_data: &str) -> Self {
        Plist::new_string(plist_data)
    }
}

impl From<Plist> for Vec<u8> {
    fn from(plist: Plist) -> Self {
        let plist_t = plist.plist_t;
        let mut plist_data = std::ptr::null_mut();
        let mut plist_size = 0;
        debug!("Converting plist to binary data");
        unsafe {
            unsafe_bindings::plist_to_bin(plist_t, &mut plist_data, &mut plist_size);
        }
        debug!("Assembling binary data");
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
        debug!("Creating plist from binary data");
        unsafe { unsafe_bindings::plist_from_bin(plist_data, len as u32, plist_t) };
        unsafe { (*plist_t).into() }
    }
}

impl Clone for Plist {
    fn clone(&self) -> Self {
        debug!("Cloning plist");
        let plist_t = unsafe { unsafe_bindings::plist_copy(self.plist_t) };
        debug!("Getting type of cloned plist");
        plist_t.into()
    }
}

impl std::fmt::Debug for Plist {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let plist_data = self.to_string();
        write!(f, "{:?}: {}", self.plist_type, plist_data)
    }
}

impl Drop for Plist {
    fn drop(&mut self) {
        debug!("Dropping plist {}", self.id);
        // Dependent plists should be freed automatically because this object is being dropped, right?
        if self.plist_t as u8 == 0 {
            debug!("Plist has already been freed");
            return;
        }
        unsafe { unsafe_bindings::plist_free(self.plist_t) }
        debug!("Plist dropped");
    }
}

impl PlistArrayIter {
    pub fn next_item(&mut self) -> Option<Plist> {
        let to_fill = unsafe { std::mem::zeroed() };
        debug!("Getting next item in array");
        unsafe {
            unsafe_bindings::plist_array_next_item(
                self.plist.plist_t,
                self.plist_array_iter,
                to_fill,
            )
        };
        if to_fill.is_null() {
            debug!("No more items in array");
            None
        } else {
            debug!("Getting type of next item in array");
            Some(unsafe { *to_fill }.into())
        }
    }
}

impl From<Plist> for PlistArrayIter {
    fn from(plist: Plist) -> Self {
        let mut plist_array_iter = unsafe { std::mem::zeroed() };
        debug!("Getting iterator for array");
        unsafe { unsafe_bindings::plist_array_new_iter(plist.plist_t, &mut plist_array_iter) };
        PlistArrayIter {
            plist_array_iter,
            plist,
        }
    }
}

impl PlistDictIter {
    pub fn next_item(&mut self) -> Option<(String, Plist)> {
        let mut key = unsafe { std::mem::zeroed() };
        let mut to_fill = unsafe { std::mem::zeroed() };
        debug!("Getting next item in dictionary");
        unsafe {
            unsafe_bindings::plist_dict_next_item(
                self.plist.plist_t,
                self.plist_dict_iter,
                &mut key,
                &mut to_fill,
            )
        };
        if to_fill.is_null() {
            debug!("No more items in dictionary");
            None
        } else {
            let key_str = unsafe { std::ffi::CStr::from_ptr(key).to_string_lossy().into_owned() };
            debug!("Getting type of next item in dictionary");
            Some((key_str, to_fill.into())) // yeet
        }
    }
}

impl From<Plist> for PlistDictIter {
    fn from(plist: Plist) -> Self {
        let mut plist_dict_iter = unsafe { std::mem::zeroed() };
        debug!("Getting iterator for dictionary");
        unsafe { unsafe_bindings::plist_dict_new_iter(plist.plist_t, &mut plist_dict_iter) };
        PlistDictIter {
            plist_dict_iter,
            plist,
        }
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
            _ => PlistType::Unknown,
        }
    }
}

pub fn compare_node_values(node_l: Plist, node_r: Plist) -> bool {
    debug!("Comparing node values");
    match unsafe { unsafe_bindings::plist_compare_node_value(node_l.plist_t, node_r.plist_t) }
        .to_string()
        .as_str()
    {
        "TRUE" => true,
        _ => false,
    }
}
