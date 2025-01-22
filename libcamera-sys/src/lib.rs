#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
// this is due to rust-lang/rust-bindgen#1651
#![allow(deref_nullptr)]
// libcamera documentation is incorrectly interpreted as rust code blocks
#![allow(rustdoc::invalid_rust_codeblocks)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/bindings.rs"));
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/bindings_cpp.rs"));
