// jkcoxson

use rusty_libimobiledevice::error::InstProxyError;
use rusty_libimobiledevice::instproxy::InstProxyClient;
use rusty_libimobiledevice::libimobiledevice;
use rusty_libimobiledevice::plist::Plist;

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
                i += 1;
            }
            "-h" | "--help" => {
                println!("Usage: ideviceimagemounter [options] <DMG Path>");
                println!("");
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
                if args[i].starts_with("-") {
                    println!("Unknown flag: {}", args[i]);
                    return;
                }
            }
        }
        i += 1;
    }
    if udid == "" {
        println!("Error: No UDID specified. Use -u or --udid to specify a device.");
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

    let mut lockdown_client = match device.new_lockdownd_client("idevicedebug".to_string()) {
        Ok(lckd) => {
            println!("Successfully connected to lockdownd");
            lckd
        }
        Err(e) => {
            println!("Error starting lockdown service: {:?}", e);
            return;
        }
    };

    let instproxy_client = match device.new_instproxy_client("idevicedebug".to_string()) {
        Ok(instproxy) => {
            println!("Successfully started instproxy");
            instproxy
        }
        Err(e) => {
            println!("Error starting instproxy: {:?}", e);
            return;
        }
    };

    let mut client_opts = InstProxyClient::options_new();
    println!("Client options: {}", client_opts.to_string());
    InstProxyClient::options_add(
        &mut client_opts,
        vec![("ApplicationType".to_string(), Plist::new_string("Any"))],
    );
    println!(
        "Successfully created client options: {}",
        client_opts.clone().to_string()
    );
    InstProxyClient::options_set_return_attributes(
        &mut client_opts,
        vec![
            "CFBundleIdentifier".to_string(),
            "CFBundleExecutable".to_string(),
        ],
    );
    println!("Successfully set return attributes");
    println!("client opts: {:?}", client_opts.to_string());

    let apps =
        match instproxy_client.lookup(vec!["com.google.ios.youtube".to_string()], client_opts) {
            Ok(apps) => {
                println!("Successfully looked up apps");
                apps
            }
            Err(e) => {
                println!("Error looking up apps: {:?}", e);
                return;
            }
        };
    println!("apps: {:?}", apps.to_string());
    println!("apps type: {:?}", apps.plist_type);

    let app_found = match apps.dict_get_item("Container") {
        Ok(p) => p,
        Err(_) => {
            println!("App not found");
            return;
        }
    };
    println!("app_found: {:?}", app_found.to_string());
    println!("app_found type: {:?}", app_found.plist_type);

    let bundle_path = match instproxy_client
        .get_path_for_bundle_identifier("com.google.ios.youtube".to_string())
    {
        Ok(p) => {
            println!("Successfully found bundle path");
            p
        }
        Err(e) => {
            println!("Error getting path for bundle identifier: {:?}", e);
            return;
        }
    };

    println!("app type: {:?}", app_found.plist_type);

    let object = match app_found.dict_get_item("Container") {
        Ok(o) => o,
        Err(e) => {
            println!("Error getting object: {:?}", e);
            return;
        }
    };
    let working_directory = object.get_string_val().unwrap();
}
