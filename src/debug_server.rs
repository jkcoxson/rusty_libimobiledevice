// jkcoxson

use std::{convert::TryInto, os::raw::c_char};

use libc::c_int;

use crate::{
    bindings as unsafe_bindings, debug, error::DebugServerError, libimobiledevice::Device,
};

pub struct DebugServer<'a> {
    pub(crate) pointer: unsafe_bindings::debugserver_client_t,
    pub(crate) phantom: std::marker::PhantomData<&'a Device>,
}

unsafe impl Send for DebugServer<'_> {}
unsafe impl Sync for DebugServer<'_> {}

pub struct DebugServerCommand {
    command: unsafe_bindings::debugserver_command_t,
}

impl DebugServer<'_> {
    pub fn new(device: &Device, label: &str) -> Result<Self, DebugServerError> {
        let mut client: unsafe_bindings::debugserver_client_t = unsafe { std::mem::zeroed() };
        let client_ptr: *mut unsafe_bindings::debugserver_client_t = &mut client;

        let label_c_str = std::ffi::CString::new(label).unwrap();
        debug!("Creating debug server for {}", device.get_udid());
        let result = unsafe {
            unsafe_bindings::debugserver_client_start_service(
                device.pointer,
                client_ptr,
                label_c_str.as_ptr(),
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

    pub fn send(&self, data: String) -> Result<(), DebugServerError> {
        let data_c_str = std::ffi::CString::new(data).unwrap();
        let mut sent = 0;
        let result = unsafe {
            unsafe_bindings::debugserver_client_send(
                self.pointer,
                data_c_str.as_ptr(),
                data_c_str.as_bytes().len().try_into().unwrap(),
                &mut sent,
            )
        }
        .into();
        if result != DebugServerError::Success {
            return Err(result);
        }

        Ok(())
    }

    pub fn recieve_with_timeout(
        &self,
        size: u32,
        timeout: u32,
    ) -> Result<String, DebugServerError> {
        let mut data = vec![0u8; size as usize];
        let mut received = 0;
        let result = unsafe {
            unsafe_bindings::debugserver_client_receive_with_timeout(
                self.pointer,
                data.as_mut_ptr() as *mut i8,
                size,
                &mut received,
                timeout,
            )
        }
        .into();
        if result != DebugServerError::Success {
            return Err(result);
        }

        Ok(String::from_utf8(data).unwrap())
    }

    pub fn recieve(&self, size: u32) -> Result<String, DebugServerError> {
        let mut data = vec![0u8; size as usize];
        let mut received = 0;
        let result = unsafe {
            unsafe_bindings::debugserver_client_receive(
                self.pointer,
                data.as_mut_ptr() as *mut i8,
                size,
                &mut received,
            )
        }
        .into();
        if result != DebugServerError::Success {
            return Err(result);
        }

        Ok(String::from_utf8(data).unwrap())
    }

    pub fn recieve_response(&self) -> Result<String, DebugServerError> {
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

    pub fn set_environment_hex_encoded(&self, env: String) -> Result<String, DebugServerError> {
        let env_c_str = std::ffi::CString::new(env).unwrap();
        let mut response = unsafe { std::mem::zeroed() };
        let result = unsafe {
            unsafe_bindings::debugserver_client_set_environment_hex_encoded(
                self.pointer,
                env_c_str.as_ptr(),
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

    pub fn send_command(&self, command: DebugServerCommand) -> Result<String, DebugServerError> {
        let mut response: std::os::raw::c_char = unsafe { std::mem::zeroed() };
        let mut response_ptr: *mut std::os::raw::c_char = &mut response;
        let response_ptr_ptr: *mut *mut std::os::raw::c_char = &mut response_ptr;

        let response_size = std::ptr::null_mut();
        debug!("Sending command to debug server");
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

    pub fn set_argv(&self, args: Vec<String>) -> Result<String, DebugServerError> {
        let mut argv: Vec<*mut std::os::raw::c_char> = Vec::new();
        let mut c_strings = vec![];
        for arg in args {
            let arg_c_str = std::ffi::CString::new(arg).unwrap().into_raw();
            argv.push(arg_c_str);
            c_strings.push(arg_c_str);
        }
        argv.push(std::ptr::null_mut());

        let mut response: std::os::raw::c_char = unsafe { std::mem::zeroed() };
        let mut response_ptr: *mut std::os::raw::c_char = &mut response;
        let response_ptr_ptr: *mut *mut std::os::raw::c_char = &mut response_ptr;

        debug!("Setting argv for debug server");
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

    pub fn encode_string(buffer: String) -> (String, u32) {
        let mut encoded_buffer = unsafe { std::mem::zeroed() };
        let mut encoded_buffer_size = 0;
        let buffer_c_str = std::ffi::CString::new(buffer).unwrap();
        let buffer_ptr = buffer_c_str.as_ptr() as *mut i8;
        unsafe {
            unsafe_bindings::debugserver_encode_string(
                buffer_ptr,
                &mut encoded_buffer,
                &mut encoded_buffer_size,
            );
        }
        let encoded_buffer_str = unsafe {
            std::ffi::CStr::from_ptr(encoded_buffer)
                .to_string_lossy()
                .to_string()
        };
        (encoded_buffer_str, encoded_buffer_size as u32)
    }

    pub fn decode_string(buffer: String) -> String {
        let mut decoded_buffer = unsafe { std::mem::zeroed() };
        let buffer_len = buffer.len() as unsafe_bindings::size_t;
        let buffer_c_str = std::ffi::CString::new(buffer).unwrap();
        let buffer_ptr = buffer_c_str.as_ptr() as *mut i8;
        unsafe {
            unsafe_bindings::debugserver_decode_string(buffer_ptr, buffer_len, &mut decoded_buffer);
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
    pub fn new(command: String, arguments: Vec<String>) -> Result<DebugServerCommand, String> {
        let mut command_ptr: unsafe_bindings::debugserver_command_t = unsafe { std::mem::zeroed() };
        let command_ptr_ptr: *mut unsafe_bindings::debugserver_command_t = &mut command_ptr;

        let command_c_str = std::ffi::CString::new(command).unwrap();

        // Create C array
        let mut arguments_c_array: Vec<c_char> = Vec::new();
        for i in arguments.iter() {
            let c_str = std::ffi::CString::new(i.clone()).unwrap();
            arguments_c_array.push(c_str.as_bytes_with_nul()[0].try_into().unwrap());
        }
        // Create pointer to to_fill[0]
        let mut c_array_ptr: *mut std::os::raw::c_char = arguments_c_array.as_mut_ptr();
        let mut c_array_ptr_ptr: *mut *mut std::os::raw::c_char = &mut c_array_ptr;

        if arguments.len() == 0 {
            c_array_ptr_ptr = std::ptr::null_mut();
        }

        debug!("Creating debug server command");
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

impl Into<DebugServerCommand> for String {
    fn into(self) -> DebugServerCommand {
        // Split string into command and arguments
        let mut split = self.split_whitespace();
        let command = split.next().unwrap().to_string();
        let arguments: Vec<String> = split.map(|s| s.to_string()).collect();
        DebugServerCommand::new(command, arguments).unwrap()
    }
}
impl Into<DebugServerCommand> for &str {
    fn into(self) -> DebugServerCommand {
        self.to_string().into()
    }
}
