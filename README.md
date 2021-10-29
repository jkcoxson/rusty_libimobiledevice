# rusty_libimobiledevice
A Rust wrapper and implimentation for [Libimobiledevice](https://github.com/libimobiledevice/libimobiledevice)

This library builds libimobiledevice and sends it to rustc for linking, all automatically (almost)

## Building
Atm there are no instructions because of the viscosity of this project. 
Basically, clone the submodules and it will work fine on MacOS/Linux.
Windows people will have to fetch dylibs from GitHub Actions or build it yourself, then place them where ``build.rs`` expects them

### **This build process will be made better in the future, it's just not a priority atm**

## Usage
Add the crate and path to your cargo.toml, then ``use`` the crate.
The crate has a module called ``libimobiledevice`` and ``unsafe_bindings``, ``libimobiledevice`` is a safe wrapper for the C library, and can be used in safe code.
``unsafe_bindings`` will have to be wrapped in a block or with a custom implimentation.
