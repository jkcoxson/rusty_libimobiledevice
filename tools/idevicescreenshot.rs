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

    let mut device = match libimobiledevice::get_device(udid) {
        Ok(d) => d,
        Err(e) => {
            println!("Error: {:?}", e);
            return;
        }
    };

    

    match device.new_lockdownd_client("yeet".to_string()) {
        Ok(_) => {}
        Err(e) => {
            println!("Error starting lockdown service: {:?}", e);
            return;
        }
    }

    todo!();
}

