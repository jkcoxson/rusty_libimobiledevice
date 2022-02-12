// jkcoxson

extern crate bindgen;

use std::{env, path::PathBuf, fs::canonicalize, fs::create_dir };

fn main() {
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    ////////////////////////////
    //   BINDGEN GENERATION   //
    ////////////////////////////

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

    // // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // Check if folder ./override exists
    let override_path = PathBuf::from("./override");
    if override_path.exists() {
        println!("cargo:rustc-link-search={}", canonicalize(&override_path).unwrap().display());
        // Search in every folder within override
        for entry in override_path.read_dir().unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                println!("cargo:rustc-link-search={}", canonicalize(path).unwrap().display());
            }
        }

    } else {
        // Get the target triple
        let target_triple = env::var("TARGET").unwrap();
        let host_triple = env::var("HOST").unwrap();

        let ldlibs = "";

        ////////////////////////////
        //     LIBPLIST BUILD     //
        ////////////////////////////

        // Change directory to libimobiledevice
        let _ = env::set_current_dir("./submodules/libplist");
        // Create build folder
        let _ = create_dir("./build");
        let build_path = PathBuf::from("./build");
        // Run ./autogen.sh
        let _ = std::process::Command::new("sh")
            .arg(format!("./autogen.sh"))
            .arg("--enable-static")
            .arg(format!("--prefix={}", canonicalize(build_path.clone()).unwrap().display()))
            .arg(format!("--build={}", host_triple))
            .arg(format!("--host={}", target_triple))
            .status();
        // Run make
        let _ = std::process::Command::new("make").status();
        let _ = std::process::Command::new("make").arg("install").status();
        // Add the include path to $CLFLAGS
        add_cflag(&format!("-I{}", canonicalize(&build_path.join("include")).unwrap().display()));
        // Add lib to ldlibs
        add_ldlib(&format!("-L{}", canonicalize(&build_path.join("lib")).unwrap().display()));
        // Change directory back to project root
        let _ = env::set_current_dir("../../");

        ////////////////////////////
        //    LIBUSBMUXD BUILD    //
        ////////////////////////////

        // Change directory to libimobiledevice
        let _ = env::set_current_dir("./submodules/libusbmuxd");
        // Create build folder
        let _ = create_dir("./build");
        let build_path = PathBuf::from("./build");
        // Run ./autogen.sh
        let _ = std::process::Command::new("sh")
            .arg(format!("./autogen.sh"))
            .arg("--enable-static")
            .arg(format!("--prefix={}", canonicalize(build_path.clone()).unwrap().display()))
            .arg(format!("--build={}", host_triple))
            .arg(format!("--host={}", target_triple))
            .status();
        // Run make
        let _ = std::process::Command::new("make").status();
        let _ = std::process::Command::new("make").arg("install").status();
        // Add the include path to $CLFLAGS
        add_cflag(&format!("-I{}", canonicalize(build_path.join("include")).unwrap().display()));
        // Add lib to ldlibs
        add_ldlib(&format!("-L{}", canonicalize(build_path.join("lib")).unwrap().display()));
        // Change directory back to project root
        let _ = env::set_current_dir("../../");

        ////////////////////////////
        // LIBIMOBILED-GLUE BUILD //
        ////////////////////////////

        // Change directory to libimobiledevice
        let _ = env::set_current_dir("./submodules/libimobiledevice-glue");
        // Create build folder
        let _ = create_dir("./build");
        let build_path = PathBuf::from("./build");
        // Run ./autogen.sh
        let _ = std::process::Command::new("sh")
            .arg(format!("./autogen.sh"))
            .arg("--enable-static")
            .arg(format!("--prefix={}", canonicalize(build_path.clone()).unwrap().display()))
            .arg(format!("--build={}", host_triple))
            .arg(format!("--host={}", target_triple))
            .status();
        // Run make
        let _ = std::process::Command::new("make").status();
        let _ = std::process::Command::new("make").arg("install").status();
        // Add the include path to $CLFLAGS
        add_cflag(&format!("-I{}", canonicalize(build_path.join("include")).unwrap().display()));
        // Add lib to ldlibs
        add_ldlib(&format!("-L{}", canonicalize(build_path.join("lib")).unwrap().display()));
        // Change directory back to project root
        let _ = env::set_current_dir("../../");

        ////////////////////////////
        // LIBIMOBILEDEVICE BUILD //
        ////////////////////////////

        // Change directory to libimobiledevice
        let _ = env::set_current_dir("./submodules/libimobiledevice");
        // Create build folder
        let _ = create_dir("./build");
        let build_path = PathBuf::from("./build");
        // Run ./autogen.sh
        let _ = std::process::Command::new("sh")
            .arg(format!("./autogen.sh"))
            .arg("--enable-static")
            .arg(format!("--prefix={}", canonicalize(build_path.clone()).unwrap().display()))
            .arg(format!("--build={}", host_triple))
            .arg(format!("--host={}", target_triple))
            .status();
        // Run make
        let _ = std::process::Command::new("make").arg(format!("LDLIBS={}", ldlibs)).status();
        let _ = std::process::Command::new("make").arg("install").status();
        // Add the include path to $CLFLAGS
        add_cflag(&format!("-I{}", canonicalize(build_path.join("include")).unwrap().display()));
        // Change directory back to project root
        let _ = env::set_current_dir("../../");

        ////////////////////////////
        //      GNUTLS BUILD      //
        ////////////////////////////

        // Change directory to libimobiledevice
        let _ = env::set_current_dir("./submodules/gnutls");
        // Create build folder
        let _ = create_dir("./build");
        let build_path = PathBuf::from("./build");
        // Run boostrap
        let _ = std::process::Command::new("sh")
            .arg(format!("./bootstrap.sh")).status();
        // Run ./configure.sh
        let _ = std::process::Command::new("sh")
            .arg(format!("./configure.sh"))
            .arg("--enable-static")
            .arg(format!("--prefix={}", canonicalize(build_path.clone()).unwrap().display()))
            .arg(format!("--build={}", host_triple))
            .arg(format!("--host={}", target_triple))
            .status();
        // Run make
        let _ = std::process::Command::new("make").status();
        let _ = std::process::Command::new("make").arg("install").status();
        // Add the include path to $CLFLAGS
        add_cflag(&format!("-I{}", canonicalize(build_path.join("include")).unwrap().display()));
        // Change directory back to project root
        let _ = env::set_current_dir("../../");

        panic!();
    }

    // Link libi* deps
    println!("cargo:rustc-link-lib=static=imobiledevice-1.0");
    println!("cargo:rustc-link-lib=static=plist-2.0");
    println!("cargo:rustc-link-lib=static=usbmuxd-2.0");
    println!("cargo:rustc-link-lib=static=imobiledevice-glue-1.0");

    // Link ancient tech deps
    println!("cargo:rustc-link-lib=static=crypto");
    println!("cargo:rustc-link-lib=static=ssl");
    println!("cargo:rustc-link-lib=static=gnutls");

    // Link to stupid openssl
    match env::consts::OS {
        "macos" => println!("cargo:rustc-link-search=/opt/homebrew/opt/openssl@3/lib"),
        _ => panic!("Unsupported OS"),
    };
}

fn add_cflag(flag: &str) {
    let mut flags = env::var("CFLAGS").unwrap_or_default();
    flags.push_str(" ");
    flags.push_str(flag);
    env::set_var("CFLAGS", flags);
}

fn add_ldlib(flag: &str) {
    let mut flags = env::var("LDFLAGS").unwrap_or_default();
    flags.push_str(" ");
    flags.push_str(flag);
    env::set_var("LDFLAGS", flags);
}