// jkcoxson

use rusty_libimobiledevice::idevice;

fn main() {
    // Get all devices attatched
    let devices = match idevice::get_devices() {
        Ok(devices) => devices,
        Err(e) => {
            println!("Error getting devices: {:?}", e);
            return;
        }
    };

    for i in &devices {
        println!(
            "{} ({})",
            i.get_udid(),
            match i.get_network() {
                true => "Network",
                false => "USB",
            }
        );
    }
}
