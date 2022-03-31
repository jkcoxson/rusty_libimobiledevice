// jkcoxson
// Experimental system for wrapping a C callback in safe Rust

use libc::c_void;

use crate::bindings as unsafe_bindings;
use crate::libimobiledevice::IDeviceEvent;
use std::any::Any;

pub struct IDeviceEventCallback {
    pub(crate) _function_pointer: Box<dyn FnMut(IDeviceEvent, Box<dyn Any>)>,
    pub(crate) _data: Box<dyn Any>,
}

impl IDeviceEventCallback {
    pub fn new(function: Box<dyn FnMut(IDeviceEvent, Box<dyn Any>)>, _data: Box<dyn Any>) -> Self {
        IDeviceEventCallback {
            _function_pointer: function,
            _data,
        }
    }
}

pub extern "C" fn idevice_event_callback(
    _event: *const unsafe_bindings::idevice_event_t,
    _user_data: *mut c_void,
) {
    println!("Got here");
    // Unpack the user data into Box of IDeviceEventCallback
    // let mut user_data = Box::from_raw(user_data as *mut IDeviceEventCallback);
    // Run the callback
    // let event_type: IDeviceEvent = (*event).into();
    // (user_data.function_pointer)(event_type, user_data.data);
}
