#![feature(convert,slice_patterns)]

extern crate rustc_serialize;

mod contact;
mod jmaputil;

use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::io::Read;
use jmaputil::FromJson;
use contact::{Contact,PartialContact};
use rustc_serialize::json::{Json,ToJson};

fn load_json(filename: &str) -> Json {
    let path = Path::new(filename);
    let mut file = match File::open(&path) {
        Ok(f) => f,
        Err(e) => panic!("couldn't open {}: {}", path.display(), Error::description(&e)),
    };

    let mut raw = String::new();
    if let Err(e) = file.read_to_string(&mut raw) {
        panic!("couldn't read {}: {}", path.display(), Error::description(&e));
    };

    match Json::from_str(raw.as_ref()) {
        Ok(j) => j,
        Err(e) => panic!("couldn't parse {}: {}", path.display(), Error::description(&e)),
    }
}

fn main() {
    let contact = match Contact::from_json(&load_json("contact.json")) {
        Ok(c) => c,
        Err(e) => panic!("contact parse error: {}", e),
    };

    println!("{:?}", contact);
    println!("{}", contact.to_json().to_string());

    let update = match PartialContact::from_json(&load_json("update.json")) {
        Ok(u) => u,
        Err(e) => panic!("contact update parse error: {}", e),
    };

    let updated = contact.updated_with(&update);

    println!("{:?}", updated);
    println!("{}", updated.to_json().to_string());
}

