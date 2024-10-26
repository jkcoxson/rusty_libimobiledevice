// Jackson Coxson
// Set the location of an iOS device

use rusty_libimobiledevice::{idevice, service::ServiceClient};

fn main() {
    const VERSION: &str = "0.1.0";

    env_logger::init();

    let mut udid = "".to_string();
    let mut usage = None;
    let mut latitude = "".to_string();
    let mut longitude = "".to_string();

    // Parse arguments
    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "-u" | "--udid" => {
                udid = args[i + 1].clone();
                i += 1;
            }
            "-h" | "--help" => {
                println!("Usage: idevicelocation [options] <start|stop> <latitude> <longitude>");
                println!();
                println!("Options:");
                println!("  -u, --udid <udid>    : udid of the device to mount");
                println!("  -h, --help           : display this help message");
                println!("  -v, --version        : display version");
                return;
            }
            "-v" | "--version" => {
                println!("v{}", VERSION);
                return;
            }
            _ => {
                if usage.is_none() {
                    match args[i].as_str() {
                        "start" => {
                            usage = Some(Usage::Start);
                            i += 1;
                            continue;
                        }
                        "stop" => {
                            usage = Some(Usage::Stop);
                            i += 1;
                            continue;
                        }
                        _ => {
                            println!("Expected start or stop");
                            return;
                        }
                    }
                }
                if latitude.is_empty() {
                    latitude = args[i].to_string();
                    i += 1;
                    continue;
                }
                if longitude.is_empty() {
                    longitude = args[i].to_string();
                    i += 1;
                    continue;
                }
                if args[i].starts_with('-') {
                    println!("Unknown flag: {}", args[i]);
                    return;
                }
            }
        }
        i += 1;
    }

    // Check arguments
    if usage.is_none() {
        println!("No usage specified, pass -h for help");
        return;
    }
    let usage = usage.unwrap();
    if usage == Usage::Start && (latitude.is_empty() || longitude.is_empty()) {
        println!("No coordinates specified, pass -h for help");
        return;
    }

    // Get a device from the muxer. This is so we can get the pairing information and its address.
    let device = if udid.is_empty() {
        match idevice::get_first_device() {
            Ok(device) => device,
            Err(e) => {
                println!("Error getting devices: {:?}", e);
                return;
            }
        }
    } else {
        match idevice::get_device(udid) {
            Ok(device) => device,
            Err(e) => {
                println!("Error getting devices: {:?}", e);
                return;
            }
        }
    };

    // Start a generic service on the device. rusty_libimobiledevice currently doesn't have built in abstractions
    // for location services, but we can manually send packets through a generic service.
    let mut lockdown_client = match device.new_lockdownd_client("idevicelocation") {
        Ok(l) => l,
        Err(e) => {
            println!("Error starting lockdown client: {:?}", e);
            return;
        }
    };
    let service = match lockdown_client.start_service("com.apple.dt.simulatelocation", false) {
        Ok(s) => s,
        Err(e) => {
            println!("Unable to start service: {:?}", e);
            return;
        }
    };
    let service = match ServiceClient::new(&device, service) {
        Ok(s) => s,
        Err(e) => {
            println!("Unable to convert service client: {:?}", e);
            return;
        }
    };

    match usage {
        Usage::Start => {
            // Send the starting bytes
            match service.send([0, 0, 0, 0].to_vec()) {
                Ok(_) => {}
                Err(e) => {
                    println!("Error sending start byte: {:?}", e);
                    return;
                }
            }

            // Send latitude
            let lat_len = (latitude.len() as u32).to_be_bytes();
            let lat_len = lat_len.to_vec();
            match service.send(lat_len) {
                Ok(_) => {}
                Err(e) => {
                    println!("Unable to send latitude length: {:?}", e);
                    return;
                }
            }
            let latitude = latitude.as_bytes();
            match service.send(latitude.to_vec()) {
                Ok(_) => {}
                Err(e) => {
                    println!("Unable to send latitude: {:?}", e);
                    return;
                }
            }

            // Send longitude
            let lon_len = (longitude.len() as u32).to_be_bytes();
            let lon_len = lon_len.to_vec();
            match service.send(lon_len) {
                Ok(_) => {}
                Err(e) => {
                    println!("Unable to send longitude length: {:?}", e);
                    return;
                }
            }
            let longitude = longitude.as_bytes().to_vec();
            match service.send(longitude) {
                Ok(_) => {}
                Err(e) => {
                    println!("Unable to send longitude: {:?}", e);
                    return;
                }
            }

            println!("Done");
        }
        Usage::Stop => match service.send([0, 0, 0, 1].to_vec()) {
            Ok(_) => {
                println!("Stopped successfully")
            }
            Err(e) => {
                println!("Error stopping: {:?}", e);
            }
        },
    }
}

#[derive(PartialEq, Eq)]
enum Usage {
    Start,
    Stop,
}
