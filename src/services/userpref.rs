// jkcoxson

use std::ffi::CString;

use crate::bindings as unsafe_bindings;
use crate::error::UserPrefError;

use plist_plus::Plist;

pub fn read_pair_record(udid: String) -> Result<Plist, UserPrefError> {
    let udid = CString::new(udid).unwrap();
    let mut to_fill = unsafe { std::mem::zeroed() };
    let results =
        unsafe { unsafe_bindings::userpref_read_pair_record(udid.as_ptr(), &mut to_fill) }.into();
    if results != UserPrefError::Success {
        return Err(results);
    }
    Ok(to_fill.into())
}
