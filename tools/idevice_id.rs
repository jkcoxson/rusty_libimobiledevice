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
        println!(
            "{} ({})",
            i.get_udid(),
            match i.get_network() {
                true => "Network",
                false => "USB",
            }
        );
        println!("IP: {:?}", i.get_ip_address());
    }
}
