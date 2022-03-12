// jkcoxson

use rusty_libimobiledevice::debug_server::DebugServerCommand;
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
    InstProxyClient::options_add(
        &mut client_opts,
        vec![("ApplicationType".to_string(), Plist::new_string("Any"))],
    );
    InstProxyClient::options_set_return_attributes(
        &mut client_opts,
        vec![
            "CFBundleIdentifier".to_string(),
            "CFBundleExecutable".to_string(),
            "Container".to_string(),
        ],
    );
    let lookup_results =
        match instproxy_client.lookup(vec!["com.jkcoxson.DolphiniOS".to_string()], client_opts) {
            Ok(apps) => {
                println!("Successfully looked up apps");
                apps
            }
            Err(e) => {
                println!("Error looking up apps: {:?}", e);
                return;
            }
        };
    let lookup_results = lookup_results
        .dict_get_item("com.jkcoxson.DolphiniOS")
        .unwrap();

    let working_directory = match lookup_results.dict_get_item("Container") {
        Ok(p) => p,
        Err(_) => {
            println!("App not found");
            return;
        }
    };

    let working_directory = match working_directory.get_string_val() {
        Ok(p) => p,
        Err(_) => {
            println!("App not found");
            return;
        }
    };
    println!("Working directory: {}", working_directory);

    let bundle_path = match instproxy_client
        .get_path_for_bundle_identifier("com.jkcoxson.DolphiniOS".to_string())
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

    println!("Bundle Path: {}", bundle_path);

    let debug_server = match device.new_debug_server("idevicedebug") {
        Ok(d) => {
            println!("Successfully started debug server");
            d
        }
        Err(e) => {
            println!("Error starting debug server: {:?}", e);
            println!("Maybe mount the Developer DMG?");
            return;
        }
    };

    match debug_server.send_command("QSetMaxPacketSize: 1024".into()) {
        Ok(res) => println!("Successfully set max packet size: {:?}", res),
        Err(e) => {
            println!("Error setting max packet size: {:?}", e);
            return;
        }
    }

    match debug_server.send_command(format!("QSetWorkingDir: {}", working_directory).into()) {
        Ok(res) => println!("Successfully set working directory: {:?}", res),
        Err(e) => {
            println!("Error setting working directory: {:?}", e);
            return;
        }
    }

    match debug_server.set_argv(vec![bundle_path.clone(), bundle_path.clone()]) {
        Ok(res) => println!("Successfully set argv: {:?}", res),
        Err(e) => {
            println!("Error setting argv: {:?}", e);
            return;
        }
    }

    match debug_server.send_command("qLaunchSuccess".into()) {
        Ok(res) => println!("Got launch response: {:?}", res),
        Err(e) => {
            println!("Error checking if app launched: {:?}", e);
            return;
        }
    }

    match debug_server.send_command("D".into()) {
        Ok(res) => println!("Detaching: {:?}", res),
        Err(e) => {
            println!("Error detaching: {:?}", e);
            return;
        }
    }
}
