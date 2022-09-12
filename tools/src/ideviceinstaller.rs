// jkcoxson

use std::time;

use rusty_libimobiledevice::{error::AfcError, idevice, services::afc::AfcClient};

const PKG_PATH: &'static str = "PublicStaging";
const VERSION: &str = "0.1.0";

fn main() {
    let mut mode = Usage::List;
    let mut udid = "".to_string();
    let mut path = "".to_string();

    // Parse arguments
    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "-u" | "--udid" => {
                udid = args[i + 1].clone();
            }
            "-p" | "--path" => {
                path = args[i + 1].clone();
            }
            "-l" | "--list" => {
                mode = Usage::List;
            }
            "-i" | "--install" => {
                mode = Usage::Install;
            }
            "r" | "--remove" => {
                mode = Usage::Uninstall;
            }
            "--upgrade" => {
                mode = Usage::Upgrade;
            }
            "-h" | "--help" => {
                println!("Usage: ideviceimagemounter <DMG Path> [options]");
                println!();
                println!("Options:");
                println!("  -u, --udid <udid>    : udid of the device to mount");
                println!("  -p, --path <path>    : path to the image to install");
                println!("  -l, --list           : list all installed images");
                println!("  -i, --install        : install an image");
                println!("      --upgrade        : upgrade an image");
                println!("  -r, --remove         : remove an image");
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

    // Get the current epoch time to append to the afc label
    // iOS is bad at releasing old afc clients
    let now = time::SystemTime::now();
    let now = now.duration_since(time::UNIX_EPOCH).unwrap();
    let now = now.as_secs();

    // Start an AFC client
    let afc = match device.new_afc_client(format!("ideviceinstaller-{}", now)) {
        Ok(afc) => afc,
        Err(e) => {
            println!("Error: Could not start AFC service: {:?}", e);
            return;
        }
    };

    let d_info = match afc.read_directory(".") {
        Ok(d_info) => d_info,
        Err(e) => {
            println!("Error: Could not read dir: {:?}", e);
            return;
        }
    };
    println!("Dir: {:?}", d_info);
    let d_info = &d_info[2..];

    for i in d_info {
        println!("{}", i);
        let i = match afc.get_file_info("./".to_string() + i) {
            Ok(i) => i,
            Err(e) => {
                println!("Error: Could not get file info: {:?}", e);
                continue;
            }
        };
        println!("File: {:?}", i);
    }

    todo!();
}

enum Usage {
    Install,
    Uninstall,
    Upgrade,
    List,
}
