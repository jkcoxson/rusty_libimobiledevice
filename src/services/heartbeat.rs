// jkcoxson

use std::ffi::CString;

use crate::{bindings as unsafe_bindings, debug, error::HeartbeatError, idevice::Device};

use plist_plus::Plist;

/// A required service for most other services.
/// iOS will close other connections if there is no active heartbeat client
///
/// # Protocol
/// The hearbeat protocol goes as follows
/// * The host requests a connection through lockdown
/// * The iOS connection accepts the connection
/// * The host will wait for a hearbeat packet from the iDevice
/// * The host will echo back the message at the interval defined in the message (give buffer time)
///
/// **Note** The device will kill the heartbeat connection if packets are echoed too frequently
pub struct HeartbeatClient {
    pub(crate) pointer: unsafe_bindings::heartbeat_client_t,
    // phantom: std::marker::PhantomData<&'a Device>,
}

unsafe impl Send for HeartbeatClient {}
unsafe impl Sync for HeartbeatClient {}

impl HeartbeatClient {
    /// Starts a new service with heartbeat
    /// # Arguments
    /// * `device` - The device to create the sevice with
    /// * `label` - The label to give the connection
    /// # Returns
    /// A struct containing the handle to the service
    ///
    /// ***Verified:*** False
    pub fn new(device: &Device, label: String) -> Result<Self, HeartbeatError> {
        let mut pointer = unsafe { std::mem::zeroed() };
        let label_c_str = CString::new(label).unwrap();
        let label_ptr = label_c_str.as_ptr();
        let result = unsafe {
            unsafe_bindings::heartbeat_client_start_service(device.pointer, &mut pointer, label_ptr)
        }
        .into();
        if result != HeartbeatError::Success {
            return Err(result);
        }
        Ok(Self {
            pointer: pointer,
            // phantom: std::marker::PhantomData,
        })
    }

    /// Send data to the hearbeat service
    /// # Arguments
    /// * `message` - A plist containing the message
    /// # Returns
    /// *none*
    ///
    /// ***Verified:*** False
    pub fn send(&self, message: Plist) -> Result<(), HeartbeatError> {
        let result =
            unsafe { unsafe_bindings::heartbeat_send(self.pointer, message.get_pointer()) }.into();
        if result != HeartbeatError::Success {
            return Err(result);
        }
        Ok(())
    }

    /// Receive data from the heartbeat service.
    /// If the error is a MuxError, this usually means that the device has disconnected.
    /// # Arguments
    /// * `timeout` - How long to wait for a message. If 0, this will block indefinitely.
    /// # Returns
    /// The message as a plist
    ///
    /// ***Verified:*** False
    pub fn receive(&self, timeout: u32) -> Result<Plist, HeartbeatError> {
        let mut plist_ptr = unsafe { std::mem::zeroed() };

        if timeout == 0 {
            let result =
                unsafe { unsafe_bindings::heartbeat_receive(self.pointer, &mut plist_ptr) }.into();
            if result != HeartbeatError::Success {
                return Err(result);
            }
        } else {
            let result = unsafe {
                unsafe_bindings::heartbeat_receive_with_timeout(
                    self.pointer,
                    &mut plist_ptr,
                    timeout,
                )
            }
            .into();
            if result != HeartbeatError::Success {
                return Err(result);
            }
        }

        Ok(plist_ptr.into())
    }
}

impl Drop for HeartbeatClient {
    fn drop(&mut self) {
        debug!("Dropping heartbeat client");
        unsafe {
            unsafe_bindings::heartbeat_client_free(self.pointer);
        }
    }
}
