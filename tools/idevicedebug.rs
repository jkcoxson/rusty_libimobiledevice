// jkcoxson

use rusty_libimobiledevice::libimobiledevice;
use rusty_libimobiledevice::instproxy::InstProxyClient;

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

    let client_opts = InstProxyClient::options_new();
    println!("here?");
    InstProxyClient::options_add(&client_opts, vec![("ApplicationType".to_string(), "Any".to_string().into())]);
    println!("Successfully created client options");
    InstProxyClient::options_set_return_attributes(&client_opts, vec!["CFBundleIdentifier".to_string(), "CFBundleExecutable".to_string()]);
    println!("Successfully set return attributes");

    match instproxy_client.lookup(vec!["com.apple.mobile.installation_proxy".to_string()], client_opts) {
        Ok(plist) => {
            println!("Successfully looked up installation_proxy");
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}

