#![feature(i128_type)]

extern crate byteorder;
extern crate serde;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate error_chain;

pub mod ser;
pub mod de;
pub mod errors;
pub mod schema;
