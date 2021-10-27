// jkcoxson

extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    //println!("cargo:rustc-link-lib=bz2");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Include in clang build
        .clang_arg("-I./libimobiledevice/include")
        .clang_arg("-I./libplist/include")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

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
