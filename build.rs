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
            "windows" => {
                panic!("Generating bindings on Windows is broken, pls remove the pls-generate feature.");
            }
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

    let location_determinator;
    if cfg!(feature = "static") {
        location_determinator = "static";
    } else if cfg!(feature = "dynamic") {
        location_determinator = "dylib";
    } else {
        panic!("No library type specified! Add 'static' or 'dynamic' to the features list.");
    }
        
    // Link libi* deps
    println!("cargo:rustc-link-lib={}=imobiledevice-1.0", location_determinator);
    println!("cargo:rustc-link-lib={}=plist-2.0", location_determinator);
    println!("cargo:rustc-link-lib={}=usbmuxd-2.0", location_determinator);
    println!("cargo:rustc-link-lib={}=imobiledevice-glue-1.0", location_determinator);

    // Link ancient tech deps
    println!("cargo:rustc-link-lib={}=crypto", location_determinator);
    println!("cargo:rustc-link-lib={}=ssl", location_determinator);

    // This is why we can't have nice things (switch to Mac)
    if env::var("TARGET").unwrap().contains("windows") {
        println!("cargo:rustc-link-lib=dylib=Iphlpapi"); // Microsoft doesn't supply static libs for this
        println!("cargo:rustc-link-lib=dylib=crypt32"); // Microsoft doesn't supply static libs for this
        println!("cargo:rustc-link-lib=dylib=ncrypt"); // Microsoft doesn't supply static libs for this
        println!("cargo:rustc-link-lib=dylib=ole32"); // Microsoft doesn't supply static libs for this
        println!("cargo:rustc-link-lib=dylib=shell32"); // Microsoft doesn't supply static libs for this
    }
}
