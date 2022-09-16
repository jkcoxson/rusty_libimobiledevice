// jkcoxson

use rusty_libimobiledevice::idevice;

fn main() {
    // Create a new device
    let device = idevice::get_first_device().unwrap();
    let mis_client = device.new_misagent_client("yurmom").unwrap();

    let mis_plist = mis_client.copy(false).unwrap();

    let items = mis_plist.array_get_size().unwrap();
    for i in 0..(items - 1) {
        let i = mis_plist.array_get_item(i).unwrap();
        let i: Vec<u8> = i
            .get_data_val()
            .unwrap()
            .into_iter()
            .map(|x| x as u8)
            .collect();

        let s = String::from_utf8_lossy(&i).to_string();
        println!("{s}");
        println!();
        println!();
    }
}
