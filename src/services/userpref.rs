// jkcoxson

use std::ffi::CString;

use crate::bindings as unsafe_bindings;
use crate::error::UserPrefError;

use plist_plus::Plist;

/// Read the pair record from usbmuxd into a plist
/// # Arguments
/// * `udid` - The UDID of the device to fetch the pairing record of
/// # Returns
/// A plist containing the pair record
pub fn read_pair_record(udid: impl Into<String>) -> Result<Plist, UserPrefError> {
    let udid = CString::new(udid.into()).unwrap();
    let mut to_fill = unsafe { std::mem::zeroed() };
    let results =
        unsafe { unsafe_bindings::userpref_read_pair_record(udid.as_ptr(), &mut to_fill) }.into();
    if results != UserPrefError::Success {
        return Err(results);
    }
    Ok(to_fill.into())
}
