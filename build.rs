// jkcoxson

extern crate bindgen;

use std::{env, fs::canonicalize, path::PathBuf};

fn main() {
    // Tell cargo to invalidate the built crate whenever build files change
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=build.rs");

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

    if cfg!(feature = "vendored") {
        // Clone the vendored libraries
        repo_setup("https://github.com/Mbed-TLS/mbedtls.git");
        repo_setup("https://github.com/libimobiledevice/libplist.git");
        repo_setup("https://github.com/libimobiledevice/libimobiledevice-glue.git");
        repo_setup("https://github.com/libimobiledevice/libusbmuxd.git");
        repo_setup("https://github.com/libimobiledevice/libimobiledevice.git");

        // If building for Windows, set the env var for mbedtls
        if cfg!(target_os = "windows") {
            env::set_var("WINDOWS_BUILD", "1");
        }

        // Build mbedtls
        let dst = cmake::build("mbedtls");
        println!("cargo:rustc-link-search=native={}", dst.display());

        // Set the env var for mbedtls
        let mb_include = dst.join("include");
        env::set_var("CFLAGS", format!("-I{}", mb_include.display()));
        env::set_var("mbedtls_INCLUDES", mb_include.display().to_string());
        env::set_var("mbedtls_LIBDIR", dst.join("lib").display().to_string());

        // Add the out include folder to the include path

        // Build those bad bois
        let dst = autotools::Config::new("libplist")
            .without("cython", None)
            .build();

        println!("cargo:rustc-link-search=native={}", dst.display());

        let dst = autotools::Config::new("libimobiledevice-glue")
            .without("cython", None)
            .with("mbedtls", None)
            .cflag(format!("-I{}", mb_include.display()))
            .build();

        println!("cargo:rustc-link-search=native={}", dst.display());

        let dst = autotools::Config::new("libusbmuxd")
            .without("cython", None)
            .with("mbedtls", None)
            .build();

        println!("cargo:rustc-link-search=native={}", dst.display());

        let dst = autotools::Config::new("libimobiledevice")
            .without("cython", None)
            .with("mbedtls", None)
            .build();

        println!("cargo:rustc-link-search=native={}", dst.display());
    } else {
        // Check if folder ./override exists
        let override_path = PathBuf::from("./override").join(env::var("TARGET").unwrap());
        if override_path.exists() {
            println!(
                "cargo:rustc-link-search={}",
                canonicalize(&override_path).unwrap().display()
            );
        }

        println!("cargo:rustc-link-search=/usr/local/lib");
        println!("cargo:rustc-link-search=/usr/lib");
        println!("cargo:rustc-link-search=/opt/homebrew/lib");
        println!("cargo:rustc-link-search=/usr/local/opt/libimobiledevice/lib");
        println!("cargo:rustc-link-search=/usr/local/opt/libusbmuxd/lib");
        println!("cargo:rustc-link-search=/usr/local/opt/libimobiledevice-glue/lib");
    }
    let location_determinator;
    if cfg!(feature = "static") {
        location_determinator = "static";
    } else if cfg!(feature = "dynamic") {
        location_determinator = "dylib";
    } else {
        location_determinator = "dylib";
    }

    // Link libi* deps
    println!(
        "cargo:rustc-link-lib={}=imobiledevice-1.0",
        location_determinator
    );
    println!("cargo:rustc-link-lib={}=usbmuxd-2.0", location_determinator);
    println!(
        "cargo:rustc-link-lib={}=imobiledevice-glue-1.0",
        location_determinator
    );

    println!("cargo:rustc-link-lib=static=ssl");
    println!("cargo:rustc-link-lib=static=crypto");
}

fn repo_setup(url: &str) {
    let mut cmd = std::process::Command::new("git");
    cmd.arg("clone");
    cmd.arg(url);
    cmd.output().unwrap();
    env::set_current_dir(url.split("/").last().unwrap().replace(".git", "")).unwrap();
    let mut cmd = std::process::Command::new("make");
    cmd.arg("distclean");
    cmd.output().unwrap();
    env::set_var("NOCONFIGURE", "1");
    let mut cmd = std::process::Command::new("./autogen.sh");
    match cmd.output() {
        _ => (),
    }
    env::remove_var("NOCONFIGURE");
    env::set_current_dir("..").unwrap();
}
