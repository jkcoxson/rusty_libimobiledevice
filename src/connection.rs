// jkcoxson

use std::convert::TryInto;
use std::marker::PhantomData;
use std::os::raw::c_char;

use crate::bindings as unsafe_bindings;
use crate::error::IdeviceError;
use crate::idevice::Device;

pub struct DeviceConnection<'a> {
    pub(crate) pointer: *mut unsafe_bindings::idevice_connection_private,
    phantom: PhantomData<&'a Device>,
}

pub struct SslData {}

pub enum DeviceConnectionType {
    Usbmuxd,
    Network,
}

impl DeviceConnection<'_> {
    /// Create a connection to an iOS device
    /// This is NOT a lockdown connection, for things like debugging use a specific service
    /// # Arguments
    /// * `device` - The device to create a connection to
    /// * `port` - The port to connect to
    /// # Returns
    /// A handle for the connection
    ///
    /// ***Verified:*** False
    pub fn connect(device: Device, port: u16) -> Result<Self, IdeviceError> {
        let mut to_fill = unsafe { std::mem::zeroed() };

        let result =
            unsafe { unsafe_bindings::idevice_connect(device.pointer, port, &mut to_fill) }.into();

        if result != IdeviceError::Success {
            return Err(result);
        }

        Ok(DeviceConnection {
            pointer: to_fill,
            phantom: std::marker::PhantomData,
        })
    }

    /// Sends data to the device
    /// # Arguments
    /// * `data` - The data to send
    /// # Returns
    /// The number of bytes sent
    ///
    /// ***Verified:*** False
    pub fn send(&self, data: Vec<u8>) -> Result<u32, IdeviceError> {
        let mut to_fill = unsafe { std::mem::zeroed() };
        let result = unsafe {
            unsafe_bindings::idevice_connection_send(
                self.pointer,
                data.as_ptr() as *const c_char,
                data.len().try_into().unwrap(),
                &mut to_fill,
            )
        }
        .into();

        if result != IdeviceError::Success {
            return Err(result);
        }

        Ok(to_fill)
    }

    /// Receives data from the device
    /// # Arguments
    /// * `len` - The number of bytes to receive
    /// * `timeout` - The timeout in milliseconds
    /// # Returns
    /// The received data
    ///
    /// ***Verified:*** False
    pub fn receive(&self, len: u32, timeout: Option<u32>) -> Result<Vec<u8>, IdeviceError> {
        let mut buffer = vec![0 as u8; len as usize];
        let mut received = 0;

        let result = match timeout {
            Some(timeout) => unsafe {
                unsafe_bindings::idevice_connection_receive_timeout(
                    self.pointer,
                    buffer.as_mut_ptr() as *mut c_char,
                    len,
                    &mut received,
                    timeout,
                )
            },
            None => unsafe {
                unsafe_bindings::idevice_connection_receive(
                    self.pointer,
                    buffer.as_mut_ptr() as *mut c_char,
                    len,
                    &mut received,
                )
            },
        }
        .into();

        if result != IdeviceError::Success {
            return Err(result);
        }

        buffer.truncate(received as usize);

        Ok(buffer)
    }

    /// Toggles SSL on the connection
    /// # Arguments
    /// * `enable` - Whether to enable SSL
    /// # Returns
    /// *none*
    ///
    /// ***Verified:*** False
    pub fn enable_ssl(&self, enable: bool) -> Result<(), IdeviceError> {
        let result = match enable {
            true => unsafe { unsafe_bindings::idevice_connection_enable_ssl(self.pointer) },
            false => unsafe { unsafe_bindings::idevice_connection_disable_ssl(self.pointer) },
        }
        .into();

        if result != IdeviceError::Success {
            return Err(result);
        }

        Ok(())
    }

    /// Bypasses the SSL on the iOS device
    /// # Arguments
    /// * `bypass` - Whether to close the connection or not
    /// # Returns
    /// *none*
    ///
    /// ***Verified:*** False
    pub fn disable_bypass_ssl(&self, bypass: bool) -> Result<(), IdeviceError> {
        let result = unsafe {
            unsafe_bindings::idevice_connection_disable_bypass_ssl(self.pointer, bypass as u8)
        }
        .into();

        if result != IdeviceError::Success {
            return Err(result);
        }

        Ok(())
    }

    /// Gets the file descriptor of the connection
    /// # Arguments
    /// *none*
    /// # Returns
    /// The file descriptor
    ///
    /// ***Verified:*** False
    pub fn get_fd(&self) -> i32 {
        let mut to_fill = unsafe { std::mem::zeroed() };
        unsafe { unsafe_bindings::idevice_connection_get_fd(self.pointer, &mut to_fill) };
        to_fill
    }
}

impl Drop for DeviceConnection<'_> {
    fn drop(&mut self) {
        unsafe { unsafe_bindings::idevice_disconnect(self.pointer) };
    }
}

impl From<u32> for DeviceConnectionType {
    fn from(value: u32) -> Self {
        match value {
            0 => DeviceConnectionType::Usbmuxd,
            1 => DeviceConnectionType::Network,
            _ => panic!("Unknown connection type"),
        }
    }
}
