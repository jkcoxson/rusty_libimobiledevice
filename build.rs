// jkcoxson

extern crate bindgen;

use std::env;

fn main() {
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // Change directory to libimobiledevice
    let _ = env::set_current_dir("./libimobiledevice");
    // Run ./autogen.sh
    let _ = std::process::Command::new("sh").arg("autogen.sh").status();
    // Run make
    let _ = std::process::Command::new("make").status();

    // Get path to libimobiledevice/src/.libs
    let libs_path = env::current_dir().unwrap().join("src").join(".libs");

    // Tell cargo to link the library at src/.libs/libimobiledevice.a
    println!("cargo:rustc-link-search={}", libs_path.display());
    println!("cargo:rustc-link-lib=dylib=imobiledevice-1.0");
}
