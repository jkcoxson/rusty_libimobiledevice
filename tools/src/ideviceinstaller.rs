// jkcoxson

use std::time;

use rusty_libimobiledevice::{
    idevice,
    services::{afc::AfcFileMode, instproxy::InstProxyClient},
};

const PKG_PATH: &str = "PublicStaging";
const VERSION: &str = "0.1.0";

fn main() {
    let mut mode = Usage::None;
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

    // Make sure we have a usage
    if mode == Usage::None {
        println!("Specify a usage! (Install, Upgrade, List, Uninstall)");
        return;
    }

    // Check the IPA
    if (mode == Usage::Install || mode == Usage::Upgrade) && path.is_empty() {
        println!("Specify a path to the IPA");
        return;
    }

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

    // Check if PublicStaging exists
    match afc.get_file_info(format!("./{}", PKG_PATH)) {
        Ok(_) => {}
        Err(_) => match afc.make_directory(format!("./{}", PKG_PATH)) {
            Ok(_) => match afc.get_file_info(format!("./{}", PKG_PATH)) {
                Ok(_) => {}
                Err(e) => {
                    println!("Unable to read PublicStaging info: {:?}", e);
                    return;
                }
            },
            Err(e) => {
                println!("Unable to make PublicStaging directory: {:?}", e);
                return;
            }
        },
    };
    println!("Created PublicStaging directory");

    // Unzip the ipa and get the bundle ID
    let opened_file = match std::fs::File::open(path.clone()) {
        Ok(f) => f,
        Err(_) => {
            println!("Unable to read ipa into memory");
            return;
        }
    };

    let mut archive = match zip::ZipArchive::new(opened_file) {
        Ok(a) => a,
        Err(_) => {
            println!("Unable to read the archive");
            return;
        }
    };

    let mut bundle_id = None;
    let info_re = regex::Regex::new("Payload/[^/]*/Info.plist").unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        if info_re.is_match(outpath.to_str().unwrap()) {
            // Read to a plist
            let buf = vec![];
            let mut buf_writer = std::io::BufWriter::new(buf);
            match std::io::copy(&mut file, &mut buf_writer) {
                Ok(_) => {}
                Err(e) => {
                    println!("Unable to write to buffer {e}");
                    return;
                }
            }

            let buf = match buf_writer.into_inner() {
                Ok(b) => b,
                Err(e) => {
                    println!("Buf writer ate the buffer and won't give it back: {e}");
                    return;
                }
            };
            let info_plist = match plist_plus::Plist::from_bin(buf) {
                Ok(p) => p,
                Err(_) => {
                    println!("Error converting extracted file to Plist!!");
                    return;
                }
            };

            let bid = match info_plist.dict_get_item("CFBundleIdentifier") {
                Ok(b) => b,
                Err(_) => {
                    println!("Plist does not contain bundle ID");
                    return;
                }
            };
            let bid = match bid.get_string_val() {
                Ok(b) => b,
                Err(_) => {
                    println!("Plist does not contain bundle ID");
                    return;
                }
            };
            bundle_id = Some(bid)
        }
    }

    if bundle_id.is_none() {
        println!("Archive did not contain the bundle ID");
        return;
    }
    let bundle_id = bundle_id.unwrap();

    // Create a folder in PublicStaging
    match afc.get_file_info(format!("./{}/{}", PKG_PATH, bundle_id)) {
        Ok(_) => {}
        Err(_) => match afc.make_directory(format!("./{}/{}", PKG_PATH, bundle_id)) {
            Ok(_) => match afc.get_file_info(format!("./{}/{}", PKG_PATH, bundle_id)) {
                Ok(_) => {}
                Err(e) => {
                    println!("Unable to read PublicStaging info: {:?}", e);
                    return;
                }
            },
            Err(e) => {
                println!("Unable to make PublicStaging directory: {:?}", e);
                return;
            }
        },
    };
    println!("Created bundle ID directory");

    // Transfer the ipa
    let handle = match afc.file_open(
        format!("./{}/{}/app.ipa", PKG_PATH, bundle_id),
        AfcFileMode::WriteOnly,
    ) {
        Ok(h) => h,
        Err(e) => {
            println!("Unable to open file on device: {:?}", e);
            return;
        }
    };

    let ipa_bytes = match std::fs::read(path) {
        Ok(b) => b,
        Err(e) => {
            println!("Unable to read bytes into memory: {e}");
            return;
        }
    };

    println!("Sending bytes of ipa");
    match afc.file_write(handle, ipa_bytes) {
        Ok(_) => {}
        Err(e) => {
            println!("Unable to write ipa: {:?}", e);
            return;
        }
    }

    let mut client_opts = InstProxyClient::client_options_new();
    client_opts
        .dict_set_item("CFBundleIdentifier", bundle_id.clone().into())
        .unwrap();

    let inst_client = match device.new_instproxy_client("ideviceinstaller") {
        Ok(i) => i,
        Err(e) => {
            println!("Unable to start instproxy: {:?}", e);
            return;
        }
    };

    println!("Installing...");
    match inst_client.install(
        format!("./{}/{}/app.ipa", PKG_PATH, bundle_id),
        Some(client_opts.clone()), // nobody understands libplist, but clone is necessary I guess
    ) {
        Ok(_) => {}
        Err(e) => {
            println!("Unable to install app: {:?}", e);
            return;
        }
    }

    println!("Done!");
}

#[derive(PartialEq, Eq)]
enum Usage {
    None,
    Install,
    Uninstall,
    Upgrade,
    List,
}
