// jkcoxson

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(deref_nullptr)]
#![allow(unaligned_references)]

// If this has the build feature enabled
#[cfg(feature = "pls-generate")]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(not(feature = "pls-generate"))]
include!("pregenerated.rs");
// Throw a warning if the bindings are not generated
#[cfg(not(feature = "pls-generate"))]
println!("WARNING: The bindings are not generated. This could cause issues in the future, to generate them edit your cargo file and add the `pls-generate` feature.");
