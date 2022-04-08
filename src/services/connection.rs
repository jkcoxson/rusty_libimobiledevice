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
    pub fn connect(device: Device, port: u16) -> Result<DeviceConnection<'static>, IdeviceError> {
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
    pub fn disconnect(self) -> IdeviceError {
        unsafe { unsafe_bindings::idevice_disconnect(self.pointer) }.into()
    }
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

    pub fn recieve(&self, len: u32, timeout: u32) -> Result<c_char, IdeviceError> {
        let mut buffer = unsafe { std::mem::zeroed() };
        let mut recieved = unsafe { std::mem::zeroed() };

        let result = match timeout > 0 {
            true => unsafe {
                unsafe_bindings::idevice_connection_receive_timeout(
                    self.pointer,
                    &mut buffer,
                    len.try_into().unwrap(),
                    &mut recieved,
                    timeout.try_into().unwrap(),
                )
            },
            false => unsafe {
                unsafe_bindings::idevice_connection_receive(
                    self.pointer,
                    &mut buffer,
                    len.try_into().unwrap(),
                    &mut recieved,
                )
            },
        }
        .into();

        if result != IdeviceError::Success {
            return Err(result);
        }

        Ok(buffer) // idk if this is correct
    }

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

    pub fn disable_bypass_ssl(&self, bypass: u8) -> Result<(), IdeviceError> {
        let result =
            unsafe { unsafe_bindings::idevice_connection_disable_bypass_ssl(self.pointer, bypass) }
                .into();

        if result != IdeviceError::Success {
            return Err(result);
        }

        Ok(())
    }

    pub fn get_fd(&self) -> i32 {
        let mut to_fill = unsafe { std::mem::zeroed() };
        unsafe { unsafe_bindings::idevice_connection_get_fd(self.pointer, &mut to_fill) };
        to_fill
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
