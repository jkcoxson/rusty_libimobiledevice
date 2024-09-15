// jkcoxson

use std::{ffi::CString, future::Future, pin::Pin};

use crate::{bindings as unsafe_bindings, error::HeartbeatError, idevice::Device};

use log::info;
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
}

unsafe impl Send for HeartbeatClient {}
unsafe impl Sync for HeartbeatClient {}

unsafe impl Send for HeartbeatClientFuture {}
unsafe impl Sync for HeartbeatClientFuture {}

impl HeartbeatClient {
    /// Starts a new service with heartbeat
    /// # Arguments
    /// * `device` - The device to create the sevice with
    /// * `label` - The label to give the connection
    /// # Returns
    /// A struct containing the handle to the service
    ///
    /// ***Verified:*** False
    pub fn new(device: &Device, label: impl Into<String>) -> Result<Self, HeartbeatError> {
        let mut pointer = unsafe { std::mem::zeroed() };
        let label_c_string = CString::new(label.into()).unwrap();
        let result = unsafe {
            unsafe_bindings::heartbeat_client_start_service(
                device.pointer,
                &mut pointer,
                label_c_string.as_ptr(),
            )
        }
        .into();
        if result != HeartbeatError::Success {
            return Err(result);
        }
        Ok(Self {
            pointer,
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

    /// Receive data from the heartbeat service as a future.
    /// If the error is a MuxError, this usually means that the device has disconnected.
    /// # Arguments
    /// * `timeout` - How long to wait for a message. If 0, this will block indefinitely.
    /// # Returns
    /// The message as a plist
    ///
    /// ***Verified:*** False
    pub fn receive_async(&self, timeout: u32) -> HeartbeatClientFuture {
        HeartbeatClientFuture {
            pointer: self.pointer,
            start_time: std::time::Instant::now(),
            timeout,
        }
    }
}

pub struct HeartbeatClientFuture {
    pointer: unsafe_bindings::heartbeat_client_t,
    start_time: std::time::Instant,
    timeout: u32,
}

impl HeartbeatClientFuture {
    fn receive(&self) -> Result<Plist, HeartbeatError> {
        let mut plist_ptr = unsafe { std::mem::zeroed() };
        let result = unsafe {
            unsafe_bindings::heartbeat_receive_with_timeout(self.pointer, &mut plist_ptr, 1000)
        }
        .into();
        if result != HeartbeatError::Success {
            return Err(result);
        }
        Ok(plist_ptr.into())
    }
}

impl Future for HeartbeatClientFuture {
    type Output = Result<Plist, HeartbeatError>;

    fn poll(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        // Determine if we have passed the timeout + 5 seconds
        let now = std::time::Instant::now();
        let elapsed = now.duration_since(self.start_time);
        let timeout = self.timeout as u128;
        let elapsed_ms = elapsed.as_millis();
        if elapsed_ms > timeout + 5000 {
            // Query the heartbeat service for a message
            match self.receive() {
                Ok(plist) => {
                    info!("Received heartbeat message");
                    return std::task::Poll::Ready(Ok(plist));
                }
                Err(error) => {
                    info!("Received heartbeat error: {:?}", error);
                    return std::task::Poll::Ready(Err(error));
                }
            }
        }
        cx.waker().wake_by_ref();
        std::task::Poll::Pending
    }
}

impl Drop for HeartbeatClient {
    fn drop(&mut self) {
        info!("Dropping heartbeat client");
        unsafe {
            unsafe_bindings::heartbeat_client_free(self.pointer);
        }
    }
}
