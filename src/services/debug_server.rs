// jkcoxson

use std::{convert::TryInto, ffi::CString, os::raw::c_char};

use libc::c_int;
use log::info;

use crate::{bindings as unsafe_bindings, error::DebugServerError, idevice::Device};

pub struct DebugServer<'a> {
    pub(crate) pointer: unsafe_bindings::debugserver_client_t,
    pub(crate) phantom: std::marker::PhantomData<&'a Device>,
}

unsafe impl Send for DebugServer<'_> {}
unsafe impl Sync for DebugServer<'_> {}

/// A command that can be sent to the debug server service
pub struct DebugServerCommand {
    command: unsafe_bindings::debugserver_command_t,
}

unsafe impl Send for DebugServerCommand {}
unsafe impl Sync for DebugServerCommand {}

impl DebugServer<'_> {
    /// Starts a new debug server on the device
    /// # Arguments
    /// * `device` - The device to start the debug server on
    /// * `label` - The label to use for the debug server
    /// # Returns
    /// A debug server struct
    ///
    /// ***Verified:*** False
    pub fn new(device: &Device, label: &str) -> Result<Self, DebugServerError> {
        let mut client: unsafe_bindings::debugserver_client_t = unsafe { std::mem::zeroed() };
        let client_ptr: *mut unsafe_bindings::debugserver_client_t = &mut client;

        let label_c_string = CString::new(label).unwrap();
        info!("Creating debug server for {}", device.get_udid());
        let result = unsafe {
            unsafe_bindings::debugserver_client_start_service(
                device.pointer,
                client_ptr,
                label_c_string.as_ptr(),
            )
        }
        .into();
        if result != DebugServerError::Success {
            return Err(result);
        }

        Ok(DebugServer {
            pointer: unsafe { *client_ptr },
            phantom: std::marker::PhantomData,
        })
    }

    /// Sends a command to the debug server
    /// # Arguments
    /// * `data` - The command to send
    /// # Returns
    /// *none*
    ///
    /// ***Verified:*** False
    pub fn send(&self, data: impl Into<String>) -> Result<(), DebugServerError> {
        let data_c_string = CString::new(data.into()).unwrap();
        let mut sent = 0;
        let result = unsafe {
            unsafe_bindings::debugserver_client_send(
                self.pointer,
                data_c_string.as_ptr(),
                data_c_string.as_bytes().len().try_into().unwrap(),
                &mut sent,
            )
        }
        .into();
        if result != DebugServerError::Success {
            return Err(result);
        }

        Ok(())
    }

    /// Receives a command from the debug server with a timeout
    /// # Arguments
    /// * `size` - The number of bytes to receive
    /// * `timeout` - The timeout in milliseconds. If zero, this will be blocking
    /// # Returns
    /// The bytes received
    ///
    /// ***Verified:*** False
    pub fn receive(&self, size: u32, timeout: u32) -> Result<Vec<u8>, DebugServerError> {
        let mut data = vec![0u8; size as usize];
        let mut received = 0;

        if timeout == 0 {
            let result = unsafe {
                unsafe_bindings::debugserver_client_receive(
                    self.pointer,
                    data.as_mut_ptr() as *mut c_char,
                    size,
                    &mut received,
                )
            }
            .into();
            if result != DebugServerError::Success {
                return Err(result);
            }
        } else {
            let result = unsafe {
                unsafe_bindings::debugserver_client_receive_with_timeout(
                    self.pointer,
                    data.as_mut_ptr() as *mut c_char,
                    size,
                    &mut received,
                    timeout,
                )
            }
            .into();
            if result != DebugServerError::Success {
                return Err(result);
            }
        }

        Ok(data[..received as usize].to_vec())
    }

    /// Receives a response from the debug server
    /// Blocks until a response is received
    /// # Arguments
    /// *none*
    /// # Returns
    /// The response
    ///
    /// ***Verified:*** False
    pub fn receive_response(&self) -> Result<String, DebugServerError> {
        let mut data = unsafe { std::mem::zeroed() };
        let mut size = 0;
        let result = unsafe {
            unsafe_bindings::debugserver_client_receive_response(self.pointer, &mut data, &mut size)
        }
        .into();
        if result != DebugServerError::Success {
            return Err(result);
        }
        let data = data as *mut u8;

        Ok(
            String::from_utf8(unsafe { std::slice::from_raw_parts(data, size as usize).to_vec() })
                .unwrap(),
        )
    }

    /// Sets the ack mode of the debug server
    /// # Arguments
    /// * `enabled` - Whether to enable ack mode
    /// # Returns
    /// *none*
    ///
    /// ***Verified:*** False
    pub fn set_ack_mode(&self, enabled: bool) -> Result<(), DebugServerError> {
        let result = unsafe {
            unsafe_bindings::debugserver_client_set_ack_mode(self.pointer, enabled as c_int)
        }
        .into();
        if result != DebugServerError::Success {
            return Err(result);
        }

        Ok(())
    }

    /// Sets the environment with a hex value
    /// # Arguments
    /// * `env` - The environment variable as 'KEY=VALUE'
    /// # Returns
    /// The response to the request
    ///
    /// ***Verified:*** False
    pub fn set_environment_hex_encoded(
        &self,
        env: impl Into<String>,
    ) -> Result<String, DebugServerError> {
        let env_c_string = CString::new(env.into()).unwrap();
        let mut response = unsafe { std::mem::zeroed() };
        let result = unsafe {
            unsafe_bindings::debugserver_client_set_environment_hex_encoded(
                self.pointer,
                env_c_string.as_ptr(),
                &mut response,
            )
        }
        .into();
        if result != DebugServerError::Success {
            return Err(result);
        }

        Ok(unsafe {
            std::ffi::CStr::from_ptr(response)
                .to_string_lossy()
                .into_owned()
        })
    }

    /// Sends a command to the debug server
    /// # Arguments
    /// * `command` - The command to send as a debug server command
    /// # Returns
    /// The response from the command, usually 'OK'
    ///
    /// ***Verified:*** False
    pub fn send_command(&self, command: DebugServerCommand) -> Result<String, DebugServerError> {
        let mut response: std::os::raw::c_char = unsafe { std::mem::zeroed() };
        let mut response_ptr: *mut std::os::raw::c_char = &mut response;
        let response_ptr_ptr: *mut *mut std::os::raw::c_char = &mut response_ptr;

        let response_size = std::ptr::null_mut();
        info!("Sending command to debug server");
        let result = unsafe {
            unsafe_bindings::debugserver_client_send_command(
                self.pointer,
                command.command,
                response_ptr_ptr,
                response_size,
            )
        }
        .into();
        if result != DebugServerError::Success {
            return Err(result);
        }

        if response == 0 {
            return Ok("".to_string());
        }
        // Convert response to String
        let response_str = unsafe {
            std::ffi::CStr::from_ptr(response_ptr)
                .to_string_lossy()
                .to_string()
        };

        Ok(response_str)
    }

    /// Sets the argument value for a command
    /// # Arguments
    /// * `args` - A list of arguments
    /// # Returns
    /// The response from the command, usually 'OK'
    ///
    /// ***Verified:*** False
    pub fn set_argv(&self, args: Vec<String>) -> Result<String, DebugServerError> {
        let mut argv = Vec::with_capacity(args.len() + 1);
        let mut c_strings = Vec::with_capacity(args.len());
        for arg in args {
            c_strings.push(CString::new(arg).unwrap());
            argv.push(c_strings.last().unwrap().as_ptr() as *mut c_char)
        }
        argv.push(std::ptr::null_mut());

        let mut response: std::os::raw::c_char = unsafe { std::mem::zeroed() };
        let mut response_ptr: *mut std::os::raw::c_char = &mut response;
        let response_ptr_ptr: *mut *mut std::os::raw::c_char = &mut response_ptr;

        info!("Setting argv for debug server");
        let result = unsafe {
            unsafe_bindings::debugserver_client_set_argv(
                self.pointer,
                argv.len() as i32,
                argv.as_mut_ptr(),
                response_ptr_ptr,
            )
        }
        .into();
        if result != DebugServerError::Success {
            return Err(result);
        }

        // Convert response to String
        let response_str = unsafe {
            std::ffi::CStr::from_ptr(response_ptr)
                .to_string_lossy()
                .to_string()
        };

        Ok(response_str)
    }

    /// Encodes a string into hex notation
    /// # Arguments
    /// * `buffer` - The string to encode
    /// # Returns
    /// The encoded bytes
    ///
    /// ***Verified:*** False
    pub fn encode_string(buffer: impl Into<String>) -> Vec<u8> {
        let encoded_buffer: *mut u8 = unsafe { std::mem::zeroed() };
        let mut encoded_buffer_size = 0;
        let buffer_c_string = CString::new(buffer.into()).unwrap();
        unsafe {
            unsafe_bindings::debugserver_encode_string(
                buffer_c_string.as_ptr(),
                &mut (encoded_buffer as *mut c_char),
                &mut encoded_buffer_size,
            );
        }
        unsafe {
            std::vec::Vec::from_raw_parts(
                encoded_buffer,
                encoded_buffer_size as usize,
                encoded_buffer_size as usize,
            )
        }
    }

    /// Decodes a string encoded in hex
    /// # Arguments
    /// * `buffer` - The string to decode
    pub fn decode_string(buffer: impl Into<String>) -> String {
        let buffer = buffer.into();
        let mut decoded_buffer = unsafe { std::mem::zeroed() };
        let buffer_len = buffer.len() as unsafe_bindings::size_t;
        let buffer_c_string = CString::new(buffer).unwrap();
        unsafe {
            unsafe_bindings::debugserver_decode_string(
                buffer_c_string.as_ptr(),
                buffer_len,
                &mut decoded_buffer,
            );
        }
        let decoded_buffer_str = unsafe {
            std::ffi::CStr::from_ptr(decoded_buffer)
                .to_string_lossy()
                .to_string()
        };
        decoded_buffer_str
    }
}

impl DebugServerCommand {
    /// Assembles a new debug server command
    /// # Arguments
    /// * `command` - The command to run
    /// * `arguments` - A list of arguments for the command
    /// # Returns
    /// The struct containing the command
    ///
    /// ***Verified:*** False
    pub fn new(
        command: impl Into<String>,
        arguments: Vec<String>,
    ) -> Result<DebugServerCommand, String> {
        let mut command_ptr: unsafe_bindings::debugserver_command_t = unsafe { std::mem::zeroed() };
        let command_ptr_ptr: *mut unsafe_bindings::debugserver_command_t = &mut command_ptr;

        let command_c_str = std::ffi::CString::new(command.into()).unwrap();

        // Create C array
        let mut arguments_c_array: Vec<c_char> = Vec::new();
        for i in arguments.iter() {
            let c_str = std::ffi::CString::new(i.clone()).unwrap();
            arguments_c_array.push(c_str.as_bytes_with_nul()[0].try_into().unwrap());
        }
        // Create pointer to to_fill[0]
        let mut c_array_ptr: *mut std::os::raw::c_char = arguments_c_array.as_mut_ptr();
        let mut c_array_ptr_ptr: *mut *mut std::os::raw::c_char = &mut c_array_ptr;

        if arguments.is_empty() {
            c_array_ptr_ptr = std::ptr::null_mut();
        }

        info!("Creating debug server command");
        let result = unsafe {
            unsafe_bindings::debugserver_command_new(
                command_c_str.as_ptr(),
                arguments.len() as i32,
                c_array_ptr_ptr,
                command_ptr_ptr,
            )
        };
        if result < 0 {
            return Err(String::from("Failed to create command"));
        }

        Ok(DebugServerCommand {
            command: command_ptr,
        })
    }
}

impl From<String> for DebugServerCommand {
    fn from(s: String) -> Self {
        // Split string into command and arguments
        let mut split = s.split_whitespace();
        let command = split.next().unwrap().to_string();
        let arguments: Vec<String> = split.map(|s| s.to_string()).collect();
        DebugServerCommand::new(command, arguments).unwrap()
    }
}
impl From<&str> for DebugServerCommand {
    fn from(s: &str) -> DebugServerCommand {
        s.to_string().into()
    }
}
