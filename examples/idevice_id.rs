// jkcoxson

use rusty_libimobiledevice::libimobiledevice;

fn main() {
    // Get all devices attatched
    let devices = match libimobiledevice::get_devices() {
        Ok(devices) => devices,
        Err(e) => {
            println!("Error getting devices: {:?}", e);
            return;
        }
    };

    for i in &devices {
        println!("{} ({})", i.udid, match i.network {
            true => "Network",
            false => "USB"
        });
    }
}