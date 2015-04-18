#![feature(convert,slice_patterns)]

extern crate rustc_serialize;

mod contact;
mod jmaputil;

use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::io::Read;
use jmaputil::FromJson;
use contact::Contact;
use rustc_serialize::json::{Json,ToJson};

fn main() {
    let path = Path::new("contact.json");
    let mut file = match File::open(&path) {
        Ok(f) => f,
        Err(e) => panic!("couldn't open {}: {}", path.display(), Error::description(&e)),
    };

    let mut raw = String::new();
    if let Err(e) = file.read_to_string(&mut raw) {
        panic!("couldn't read {}: {}", path.display(), Error::description(&e));
    };

    let json = match Json::from_str(raw.as_ref()) {
        Ok(j) => j,
        Err(e) => panic!("json parse error: {}", Error::description(&e)),
    };

    let contact = match Contact::from_json(&json) {
        Ok(c) => c,
        Err(e) => panic!("contact parse error: {}", e),
    };

    println!("{:?}", contact);

    println!("{}", contact.to_json().to_string());
}

