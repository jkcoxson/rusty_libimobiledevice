// jkcoxson

use std::convert::TryInto;

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
        debug!("Creating debug server for {}", device.udid);
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
}

impl DebugServerCommand {
    pub fn new(command: String, arguments: Vec<String>) -> Result<DebugServerCommand, String> {
        let mut command_ptr: unsafe_bindings::debugserver_command_t = unsafe { std::mem::zeroed() };
        let command_ptr_ptr: *mut unsafe_bindings::debugserver_command_t = &mut command_ptr;

        let command_c_str = std::ffi::CString::new(command).unwrap();

        // Create C array
        let mut arguments_c_array: Vec<i8> = Vec::new();
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
