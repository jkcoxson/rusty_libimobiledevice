# rusty_libimobiledevice

Rusty Libimobiledevice - An ergonomic library to communicate with iOS devices.

**Tested level: 0.00%** - Contribute to this library by battle testing methods!
All functions should be tested for possible segfaults and memory leaks.

## Building
Build or install the following packages:
- libplist
- libusbmuxd
- libimobiledevice
- libimobiledevice-glue
- gnutls
- openssl

Either include these packages in a path where the linker can link them, or place them in ``./override/[platform-triple]``.

**Note:** Package managers do not ship static libraries, so you will need to build them yourself
if compiling statically.

## Usage
Add the crate and path to your cargo.toml, and add either ``static`` or ``dynamic`` to the features list. This will determine how the library is linked. By default this is dynamic.

To list devices detected by a usbmuxd daemon, you can use the following example.
```rust
// Include the idevice module. Will be needed in most scenarios.
use rusty_libimobiledevice::idevice;

fn main() {
    // Get all devices attatched from the usbmuxd daemon
    let devices = match idevice::get_devices() {
        Ok(devices) => devices,
        Err(e) => {
            // If the daemon is not running or does not behave as expected, this returns an error
            println!("Error getting devices: {:?}", e);
            return;
        }
    };

    // Devices support the display trait and can be viewed as such
    println!("Devices found: {:?}", devices);
}
```

More complicated code can skip fetching devices from usbmuxd and attach straight to a network device.

## Services
This library implements methods for a handful of an iOS device's [services](https://www.theiphonewiki.com/wiki/Services)
These can be useful for manipulating functions on the device. For example, you can get a list of apps
installed on a device using the following example
```rust
use rusty_libimobiledevice::idevice;

fn print_apps(udid: String) {
    // Get the device from usbmuxd using the given UDID
    let device = match idevice::get_device(udid.to_string()) {
        Ok(device) => device,
        Err(e) => {
            println!("Error: Could not find device: {:?}", e);
            return;
        }
    };

    // Start an instproxy service on the device
    let instproxy_client = match device.new_instproxy_client("idevicelistapps".to_string()) {
        Ok(instproxy) => {
            println!("Successfully started instproxy");
            instproxy
        }
        Err(e) => {
            println!("Error starting instproxy: {:?}", e);
            return;
        }
    };

    // Create a request to be sent using the service
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

    // Send the request and get the lookup results as a plist
    let lookup_results = match instproxy_client.lookup(vec![], client_opts) {
        Ok(apps) => {
            println!("Successfully looked up apps");
            apps
        }
        Err(e) => {
            println!("Error looking up apps: {:?}", e);
            return;
        }
    };

    println!("{}", lookup_results.to_string());
}
```

Plist will be moving to its own library soon for ease of use
