// jkcoxson

use rusty_libimobiledevice::idevice;

fn main() {
    // Create a new device
    let device = idevice::get_first_device().unwrap();
    let afc_client = device.new_afc_client("asdfasdfasdf").unwrap();
    let info = afc_client.get_file_info("./PublicStaging").unwrap();
    println!("{:?}", info);
    let dir = afc_client.read_directory("./PublicStaging").unwrap();
    println!("{:?}", dir);
    afc_client
        .remove_path_and_contents("./PublicStaging")
        .unwrap();
}
