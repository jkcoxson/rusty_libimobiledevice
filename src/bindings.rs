// jkcoxson

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(deref_nullptr)]
#![allow(improper_ctypes)]
#![allow(dead_code)]
#![allow(unused_imports)]

// This was mostly done for rust-analyzer, but it's useful to have it here

#[cfg(feature = "pls-generate")]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(not(feature = "pls-generate"))]
include!("pregenerated.rs");
