// jkcoxson

use rusty_libimobiledevice::libimobiledevice;
use rusty_libimobiledevice::libimobiledevice::Device;

fn main() {
    const VERSION: &str = "0.1.0";
    let mut udid = "".to_string();

    // Collect options
    let mut i = 1;
    let arguments = std::env::args().collect::<Vec<String>>();

    while i < arguments.len() {
        match arguments[i].as_str() {
            "-u" | "--udid" => {
                i += 1;
                udid = arguments[i].clone();
            }
            "-v" | "--version" => {
                println!("v{}", VERSION);
                return;
            }
            "-h" | "--help" => {
                println!("Options:");
                println!("  -u, --udid <udid>    Device UDID");
                println!("  -n, --network        Use network connection");
                println!("  -v, --version        Print version");
                println!("  -h, --help           Show this help");
                return;
            }
            _ => {
                panic!("Unknown argument: {}", &arguments[i]);
            }
        }
        i += 1;
    }
    if udid == "".to_string() {
        panic!("No device UDID specified");
    }

    let mut devices = match libimobiledevice::get_devices() {
        Ok(devices) => devices,
        Err(e) => {
            println!("Error getting device list: {:?}", e);
            return;
        }
    };
    
    let mut device = match find_device(udid, devices) {
        Some(device) => device,
        None => {
            println!("Device not found");
            return;
        }
    };
    

    match device.start_lockdownd_service("yeet".to_string()) {
        Ok(()) => {}
        Err(e) => {
            println!("Error starting lockdown service: {:?}", e);
            return;
        }
    }

    todo!();
}

fn find_device(udid: String, list: Vec<Device>) -> Option<Device> {
    for dev in list {
        if dev.udid == udid {
            return Some(dev);
        }
    }
    None
}