// jkcoxson
// Note: incomplete

use rusty_libimobiledevice::idevice;

fn main() {
    const VERSION: &str = "0.1.0";
    let mut udid = "".to_string();

    // Parse arguments
    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "-u" | "--udid" => {
                udid = args[i + 1].clone();
            }
            "-h" | "--help" => {
                println!("Usage: ideviceimagemounter <DMG Path> [options]");
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
                if args[i].starts_with('-') {
                    println!("Unknown flag: {}", args[i]);
                    return;
                }
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

    let lckd = match device.new_lockdownd_client("ideviceimagemounter".to_string()) {
        Ok(lckd) => {
            println!("Successfully connected to lockdownd.");
            lckd
        }
        Err(e) => {
            println!("Error starting lockdown service: {:?}", e);
            return;
        }
    };
    let output = match lckd.get_value("".to_string(), "".to_string()) {
        Ok(output) => output,
        Err(e) => {
            println!("Error: {:?}", e);
            return;
        }
    };
    for line in output.into_iter() {
        println!(
            "{}: {}",
            line.key.unwrap(),
            line.plist.clone().get_display_value().unwrap()
        );
    }
}
