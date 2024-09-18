// jkcoxson

use std::io::Write;

use rusty_libimobiledevice::idevice;

fn main() {
    const VERSION: &str = "0.1.0";
    let mut udid = "".to_string();
    let mut file_name = "".to_string();

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
                if arguments[i].starts_with('-') {
                    println!("Unknown flag: {}", arguments[i]);
                    return;
                }
                file_name = arguments[i].clone();
            }
        }
        i += 1;
    }

    let device = if udid.is_empty() {
        match idevice::get_first_device() {
            Ok(device) => device,
            Err(e) => {
                println!("Error: Could not find device: {:?}", e);
                return;
            }
        }
    } else {
        match idevice::get_device(udid) {
            Ok(device) => device,
            Err(e) => {
                println!("Error: Could not find device: {:?}", e);
                return;
            }
        }
    };
    println!("Using device: {}", device.get_udid());

    let ss = match device.new_screenshot_service("idevicescreenshot") {
        Ok(ss) => ss,
        Err(e) => {
            println!("Error: Could not connect to screenshot service: {:?}", e);
            return;
        }
    };

    let image = match ss.take_screenshot() {
        Ok(image) => image,
        Err(e) => {
            println!("Error: Could not take screenshot: {:?}", e);
            return;
        }
    };
    println!("Got {} bytes of image data", image.len());
    let mut converted = Vec::with_capacity(image.len());
    for i in image {
        converted.push(i as u8);
    }

    let mut file = match std::fs::File::create(file_name) {
        Ok(file) => file,
        Err(e) => {
            println!("Error: Could not create file: {:?}", e);
            return;
        }
    };

    file.write_all(&converted).unwrap();
}
