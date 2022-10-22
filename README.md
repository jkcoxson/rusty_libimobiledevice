# rusty_libimobiledevice

Rusty Libimobiledevice - An ergonomic library to communicate with iOS devices.

**Tested level: 0.00%** - Contribute to this library by battle testing methods!
All functions should be tested for possible segfaults and memory leaks.

If you see the `Verified: False`, that means that function needs your help to make sure it's safe.
Open a PR with your testing code to change a function's status.

## Building
Build and install the following packages:
- [libusbmuxd](https://github.com/libimobiledevice/libusbmuxd)
- [libimobiledevice](https://github.com/libimobiledevice/libimobiledevice)
- [libimobiledevice-glue](https://github.com/libimobiledevice/libimobiledevice-glue)
- [openssl](https://github.com/openssl/openssl)

**Note:** Package managers do not ship static libraries, so you will need to build them yourself
if compiling statically.

To cross compile this crate, you can use the ``vendored`` feature and the build script will attempt to clone and build them for the specified target.

## Usage
Add the crate and path to your cargo.toml, and add either ``static`` or ``dynamic`` to the features list. This will determine how the library is linked. By default this is dynamic. You can also use the ``vendored`` feature to build libimobiledevice at compile time.

Check the [tools](tools) directory for full examples of how to use this library. It has many common use-cases.

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

