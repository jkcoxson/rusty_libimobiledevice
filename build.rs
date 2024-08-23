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
        // Change current directory to OUT_DIR
        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        env::set_current_dir(&out_path).unwrap();
        env::set_var("PKG_CONFIG_PATH", &out_path.join("lib/pkgconfig"));

        // Lib path setup
        let lib_path = out_path.join("lib");
        if !lib_path.exists() {
            // Create lib directory
            std::fs::create_dir(&lib_path).unwrap();
        }
        let mut lib_path = lib_path.canonicalize().unwrap().display().to_string();

        // Set the LD_LIBRARY_PATH environment variable to lib_path
        env::set_var("LD_LIBRARY_PATH", lib_path.clone());

        // Include path setup
        let include_path = out_path.join("include");
        if !include_path.exists() {
            // Create include directory
            std::fs::create_dir(&include_path).unwrap();
        }
        let mut include_path = include_path.canonicalize().unwrap().display().to_string();

        // Search for where openssl-src placed my libs
        env::set_current_dir("../../").unwrap();
        let mut openssl_found = false;
        loop {
            for path in std::fs::read_dir(".").unwrap() {
                let path = path.unwrap().path();
                if !path.is_dir() {
                    continue;
                }
                if path.to_str().unwrap().contains("openssl-sys") {
                    let install_path = path.join("out").join("openssl-build").join("install");
                    if install_path.exists() {
                        println!(
                            "cargo:rustc-link-search=native={}",
                            install_path.join("lib").canonicalize().unwrap().display()
                        );
                        include_path = format!(
                            "{} -I{}",
                            include_path,
                            install_path
                                .join("include")
                                .canonicalize()
                                .unwrap()
                                .display()
                        );
                        lib_path = format!(
                            "{} -L{}",
                            lib_path,
                            install_path.join("lib").canonicalize().unwrap().display()
                        );
                        // Copy the pkgconfig files to the OUT_DIR
                        let ssl_pkg_config_path = install_path.join("lib").join("pkgconfig");

                        // Create the path
                        std::fs::create_dir_all(&ssl_pkg_config_path).unwrap();
                        let ssl_pkg_config_path = ssl_pkg_config_path.canonicalize().unwrap();

                        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap())
                            .join("lib")
                            .join("pkgconfig");
                        std::fs::create_dir_all(&out_path).unwrap();
                        let out_path = out_path.canonicalize().unwrap();
                        for path in std::fs::read_dir(&ssl_pkg_config_path).unwrap() {
                            let path = path.unwrap().path();
                            if !path.is_file() {
                                continue;
                            }
                            let file_name = path.file_name().unwrap().to_str().unwrap();
                            std::fs::copy(&path, out_path.join(file_name)).unwrap();
                        }
                        openssl_found = true;
                    }
                }
            }
            if openssl_found {
                break;
            }
            std::thread::sleep(std::time::Duration::from_secs(10));
        }
        if !openssl_found {
            panic!("\nopenssl-src was not found, exiting\n");
        }

        // Clone the vendored libraries
        repo_setup("https://github.com/libimobiledevice/libplist.git");
        repo_setup("https://github.com/libimobiledevice/libimobiledevice-glue.git");
        repo_setup("https://github.com/libimobiledevice/libusbmuxd.git");
        repo_setup("https://github.com/libimobiledevice/libtatsu.git");
        repo_setup("https://github.com/libimobiledevice/libimobiledevice.git");

        // Remove tools from libimobiledevice's makefile. There's no point to build them, and they cause errors on MacOS.
        let mut makefile = std::fs::read_to_string("libimobiledevice/Makefile.in").unwrap();
        makefile = makefile.replace("tools", "");
        std::fs::write("libimobiledevice/Makefile.in", makefile).unwrap();

        let mut c_flags = vec![];
        let mut cxx_flags = vec![];

        // If building for Windows, set the env var for mbedtls
        if env::var("TARGET").unwrap().contains("windows") {
            env::set_var("WINDOWS_BUILD", "1");

            // Windows needs extra crap
            println!("cargo:rustc-link-lib=dylib=iphlpapi");
            println!("cargo:rustc-link-lib=dylib=shell32");
            println!("cargo:rustc-link-lib=dylib=ole32");
        }
        if env::var("TARGET").unwrap().contains("apple") {
            println!("cargo:rustc-env=MACOSX_DEPLOYMENT_TARGET=10.13");
            c_flags.push("-mmacosx-version-min=10.13".to_string());
            cxx_flags.push("-mmacosx-version-min=10.13".to_string());
        }
        c_flags.push(format!("-L{} -I{}", lib_path, include_path));

        // Build those bad bois
        let mut dst = autotools::Config::new("libplist");
        let mut dst = dst.without("cython", None);
        for flag in &c_flags {
            dst = dst.cflag(flag);
        }
        for flag in &cxx_flags {
            dst = dst.cxxflag(flag);
        }
        let dst = dst.build();

        println!(
            "cargo:rustc-link-search=native={}",
            dst.join("lib").display()
        );

        let mut dst = autotools::Config::new("libimobiledevice-glue");
        let dst = dst.without("cython", None);
        let mut dst = dst.env("PKG_CONFIG_PATH", &out_path.join("lib/pkgconfig"));
        for flag in &c_flags {
            dst = dst.cflag(flag);
        }
        for flag in &cxx_flags {
            dst = dst.cxxflag(flag);
        }
        let dst = dst.build();

        println!("cargo:rustc-link-search=native={}", dst.display());

        let mut dst = autotools::Config::new("libusbmuxd");
        let dst = dst.without("cython", None);
        let mut dst = dst.env("PKG_CONFIG_PATH", &out_path.join("lib/pkgconfig"));
        for flag in &c_flags {
            dst = dst.cflag(flag);
        }
        for flag in &cxx_flags {
            dst = dst.cxxflag(flag);
        }
        let dst = dst.build();

        println!(
            "cargo:rustc-link-search=native={}",
            dst.join("lib").display()
        );

        let mut dst = autotools::Config::new("libtatsu");
        let dst = dst.without("cython", None);
        let mut dst = dst.env("PKG_CONFIG_PATH", &out_path.join("lib/pkgconfig"));
        for flag in &c_flags {
            dst = dst.cflag(flag);
        }
        for flag in &cxx_flags {
            dst = dst.cxxflag(flag);
        }
        let dst = dst.build();

        println!(
            "cargo:rustc-link-search=native={}",
            dst.join("lib").display()
        );

        let mut dst = autotools::Config::new("libimobiledevice");
        let dst = dst.without("cython", None);
        let mut dst = dst.env("PKG_CONFIG_PATH", &out_path.join("lib/pkgconfig"));
        for flag in &c_flags {
            dst = dst.cflag(flag);
        }
        for flag in &cxx_flags {
            dst = dst.cxxflag(flag);
        }
        let dst = dst.build();

        println!(
            "cargo:rustc-link-search=native={}",
            dst.join("lib").display()
        );
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
    let location_determinator = if cfg!(feature = "static") {
        "static"
    } else {
        "dylib"
    };

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
    println!("cargo:rustc-link-lib={}=plist-2.0", location_determinator);

    println!("cargo:rustc-link-lib={location_determinator}=crypto");
    println!("cargo:rustc-link-lib={location_determinator}=ssl");
}

fn repo_setup(url: &str) {
    let mut cmd = std::process::Command::new("git");
    cmd.arg("clone");
    cmd.arg("--depth=1");
    cmd.arg(url);
    cmd.output().unwrap();
    env::set_current_dir(url.split('/').last().unwrap().replace(".git", "")).unwrap();
    env::set_var("NOCONFIGURE", "1");
    let mut cmd = std::process::Command::new("./autogen.sh");
    let _ = cmd.output();
    env::remove_var("NOCONFIGURE");
    env::set_current_dir("..").unwrap();
}
