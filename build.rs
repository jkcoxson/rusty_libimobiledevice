// jkcoxson

extern crate bindgen;

use std::{env, path::PathBuf, fs::canonicalize };

fn main() {
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    ////////////////////////////
    //   BINDGEN GENERATION   //
    ////////////////////////////

    if cfg!(feature = "pls-generate") {
        // Get gnutls path per OS
        let gnutls_path = match env::consts::OS {
            "linux" => "/usr/include",
            "macos" => "/opt/homebrew/include",
            "windows" => "/mingw64/include", // ?
            _ => panic!("Unsupported OS"),
        };

        let bindings = bindgen::Builder::default()
            // The input header we would like to generate
            // bindings for.
            .header("wrapper.h")
            // Include in clang build
            .clang_arg("-I./submodules/libimobiledevice/include")
            .clang_arg("-I./submodules/libplist/include")
            .clang_arg("-I./submodules/libimobiledevice")
            .clang_arg(format!("-I{}", gnutls_path))
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
    }

    // Check if folder ./override exists
    let override_path = PathBuf::from("./override").join(env::var("TARGET").unwrap());
    if override_path.exists() {
        println!("cargo:rustc-link-search={}", canonicalize(&override_path).unwrap().display());
    } 

    // Link libi* deps
    println!("cargo:rustc-link-lib=static=imobiledevice-1.0");
    println!("cargo:rustc-link-lib=static=plist-2.0");
    println!("cargo:rustc-link-lib=static=usbmuxd-2.0");
    println!("cargo:rustc-link-lib=static=imobiledevice-glue-1.0");

    // Link ancient tech deps
    println!("cargo:rustc-link-lib=static=crypto");
    println!("cargo:rustc-link-lib=static=ssl");

    if env::var("TARGET").unwrap().contains("windows") {
        println!("cargo:rustc-link-lib=dylib=unistring");
        println!("cargo:rustc-link-lib=dylib=Iphlpapi");
        println!("cargo:rustc-link-lib=static=intl");
        println!("cargo:rustc-link-lib=static=iconv");
        println!("cargo:rustc-link-lib=static=gmp");
        println!("cargo:rustc-link-lib=static=gnutls");
        println!("cargo:rustc-link-lib=static=tasn1");
        println!("cargo:rustc-link-lib=static=idn2");
        println!("cargo:rustc-link-lib=dylib=p11-kit");
        println!("cargo:rustc-link-lib=static=hogweed");
        println!("cargo:rustc-link-lib=static=nettle");
    }
    

}
