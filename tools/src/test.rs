// jkcoxson

use rusty_libimobiledevice::idevice;

fn main() {
    // Create a new device
    let device = idevice::get_first_device().unwrap();
    device
}
