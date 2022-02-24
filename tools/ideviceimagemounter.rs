// jkcoxson

use rusty_libimobiledevice::libimobiledevice;

fn main() {
    const VERSION: &str = "0.1.0";

    let mut udid = "".to_string();
    let mut dmg_path = "".to_string();
    let mut image_type = "Developer".to_string();
    let mut display_xml = false;
    let mut list_mode = false;

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
            "-l" | "--list" => {
                list_mode = true;
            }
            "-t" | "--imagetype" => {
                image_type = args[i + 1].clone();
                i += 1;
            }
            "-x" | "--xml" => {
                display_xml = true;
            }
            "-h" | "--help" => {
                println!("Usage: ideviceimagemounter [options] <DMG Path>");
                println!("");
                println!("Options:");
                println!("  -u, --udid <udid>    : udid of the device to mount");
                println!("  -l, --list           : list all devices");
                println!("  -t, --imagetype <type> : image type to mount, the default is Developer");
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
    if dmg_path == "" && !list_mode {
        println!("Error: No DMG specified. Use -h for help.");
        return;
    }
    println!("{}", dmg_path);

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
            println!("Successfully connected to lockdownd");
            lckd
        }
        Err(e) => {
            println!("Error starting lockdown service: {:?}", e);
            return;
        }
    };

    let ios_version = match lockdown_client.get_value("ProductVersion".to_string(), "".to_string()) {
        Ok(ios_version) => {
            ios_version.get_string_val().unwrap()
        }
        Err(e) => {
            println!("Error getting iOS version: {:?}", e);
            return;
        }
    };

    let ios_major_version = ios_version.split('.').next().unwrap().parse::<u32>().unwrap();
    if ios_major_version < 8 {
        println!("Error: old versions of iOS are not supported atm because lazy");
        return;
    }

    let mut service = match lockdown_client.start_service("com.apple.mobile.mobile_image_mounter".to_string()) {
        Ok(service) => {
            println!("Successfully started com.apple.mobile.mobile_image_mounter");
            service
        }
        Err(e) => {
            println!("Error starting com.apple.mobile.mobile_image_mounter: {:?}", e);
            return;
        }
    };

    let mim = match device.new_mobile_image_mounter(&service) {
        Ok(mim) => {
            println!("Successfully started mobile_image_mounter");
            mim
        }
        Err(e) => {
            println!("Error starting mobile_image_mounter: {:?}", e);
            return;
        }
    };

    if list_mode {
        match mim.lookup_image(image_type.to_string()) {
            Ok(plist) => {
                println!("{:?}", plist.to_string());
            }
            Err(e) => {
                println!("Error listing images: {:?}", e);
                return;
            }
        }
    } else {
        match mim.upload_image(dmg_path.clone(), image_type.clone(), format!("{}.signature", dmg_path.clone()).to_string()) {
            Ok(_) => {
                println!("Successfully uploaded image");
            }
            Err(e) => {
                println!("Error uploading image: {:?}", e);
                return;
            }
        }
        match mim.mount_image(dmg_path.clone(), image_type, format!("{}.signature", dmg_path.clone()).to_string()) {
            Ok(_) => {
                println!("Successfully mounted image");
            }
            Err(e) => {
                println!("Error mounting image: {:?}", e);
                return;
            }
        }
        
    }
}

