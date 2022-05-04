// jkcoxson

use rusty_libimobiledevice::idevice::Device;

fn main() {
    // Create a new device
    let device = Device::new("00008101-001E30590C08001E".to_string(), false, None, 1263).unwrap();

    // Create a lockdown client
    let lockdown_client = device.new_lockdownd_client("asdf".to_string()).unwrap();
}
