// jkcoxson

use std::{net::IpAddr, str::FromStr};

use rusty_libimobiledevice::idevice::Device;

fn main() {
    // Create a new device
    let device = Device::new(
        "00008101-00156C420221001E".to_string(),
        true,
        Some(IpAddr::from_str("127.0.0.1").unwrap()),
        1263,
    )
    .unwrap();

    // Create a lockdown client
    let lockdown_client = device.new_lockdownd_client("letsgo".to_string()).unwrap();
    println!("\nConnected to lockdownd");
    println!(
        "\nDevice name: {}",
        lockdown_client.get_device_name().unwrap()
    );

    println!("\nLadies and gentlement, we gottem");
}
