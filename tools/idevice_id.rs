// jkcoxson

use rusty_libimobiledevice::libimobiledevice;
use std::net::Ipv4Addr;
use std::net::{IpAddr, SocketAddr};

fn main() {
    // Get all devices attatched
    // let devices = match libimobiledevice::get_devices() {
    //     Ok(devices) => devices,
    //     Err(e) => {
    //         println!("Error getting devices: {:?}", e);
    //         return;
    //     }
    // };

    // for i in &devices {
    //     println!(
    //         "{} ({})",
    //         i.get_udid(),
    //         match i.get_network() {
    //             true => "Network",
    //             false => "USB",
    //         }
    //     );
    // }

    // Create a new device
    let x = libimobiledevice::Device::new(
        "00008101-001E30590C08001E".to_string(),
        true,
        Some(SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(192, 168, 1, 14)),
            28000,
        )),
        1290,
    );
    println!("{:?}", x.unwrap().get_ip_address());
}
