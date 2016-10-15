// Assumedly to ensure this doens't run with the nightly
#![cfg_attr(feature = "nightly", feature(proc_macro))]

#[macro_use] extern crate diesel;
#[cfg(feature = "nightly")]
#[macro_use] extern crate diesel_codegen;
extern crate dotenv;

#[cfg(feature = "nightly")]
include!("lib.in.rs");

#[cfg(feature = "with-syntex")]
include!(concat!(env!("OUT_DIR"), "/lib.rs"));
