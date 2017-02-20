extern crate serde;

#[macro_use]
extern crate serde_derive;

extern crate serde_json;

mod parser;
mod generator;

use std::fs::File;
use std::io::Read;
use std::error::Error;

// TODO: Use error_chain!
pub fn translate(input_filename: &str, output_filename: &str) -> Result<(), Box<Error>> {
    let mut f = File::open(input_filename)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;

    let s: parser::Schema = s.parse()?;

    generator::generate(output_filename, s)?;

    Ok(())
}
