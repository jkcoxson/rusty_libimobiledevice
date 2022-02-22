// jkcoxson

use rusty_libimobiledevice::libimobiledevice;

fn main() {
    const VERSION: &str = "0.1.0";

    let mut udid = "".to_string();
    let mut dmg_path = "".to_string();
    let mut image_type = "Developer".to_string();
    let mut display_xml = false;

    // Parse arguments
    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "-u" | "--udid" => {
                udid = args[i + 1].clone();
            }
            "-l" | "--list" => {
                todo!();
            }
            "-t" | "--imagetype" => {
                image_type = args[i + 1].clone();
            }
            "-x" | "--xml" => {
                display_xml = true;
            }
            "-h" | "--help" => {
                println!("Usage: ideviceimagemounter <DMG Path> [options]");
                println!("");
                println!("Options:");
                println!("  -u, --udid <udid>    : udid of the device to mount");
                println!("  -l, --list           : list all devices");
                println!("  -t, --imagetype <type> : image type to mount (Developer, Distribution, or Recovery)");
                println!("  -x, --xml            : display xml plist");
                println!("  -h, --help           : display this help message");
                println!("  -v, --version        : display version");
                return;
            }
            "-v" | "--version" => {
                println!("v{}", VERSION);
                return;
            }
            _ => {
                if args[i].starts_with("-") {
                    println!("Unknown flag: {}", args[i]);
                    return;
                }
                dmg_path = args[i].clone();
            }
        }
        i += 1;
    }
    if udid == "" {
        println!("Error: No UDID specified. Use -u or --udid to specify a device.");
        return;
    }
    if dmg_path == "" {
        println!("Error: No DMG specified. Use -h for help.");
        return;
    }

    // Get the device
    let mut device = match libimobiledevice::get_device(udid.to_string()) {
        Ok(device) => device,
        Err(e) => {
            println!("Error: Could not find device: {:?}", e);
            return;
        }
    };

    let mut lockdown_client = match device.new_lockdownd_client("ideviceimagemounter".to_string()) {
        Ok(lckd) => {
            println!("Successfully connected to lockdownd.");
            lckd
        }
        Err(e) => {
            println!("Error starting lockdown service: {:?}", e);
            return;
        }
    };

    let ios_version = match lockdown_client.get_value("ProductVersion".to_string(), "".to_string()) {
        Ok(ios_version) => {
            ios_version.get_string_val()
        }
        Err(e) => {
            println!("Error getting iOS version: {:?}", e);
            return;
        }
    };

    println!("iOS version: {}", ios_version);

}

