#[doc = include_str!("../README.md")]

mod bindings;
/// TODO
pub mod callback;
/// A debug macro used throughout the crate
pub mod debug;
/// A module containing all possible errors produced by the library
pub mod error;
/// Creates connections and manages high level interfaces for iOS devices
pub mod idevice;
/// Plist is Apple's human readable format for storing data. Will be moved soon tm
pub mod plist;
/// A bare bones representation of a service running on a device. 
/// Useful for services that don't have modules or for running raw commands
pub mod service;
/// A module that contains all abstractions for built-in services
pub mod services;
