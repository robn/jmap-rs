#![feature(convert,slice_patterns)]

extern crate rustc_serialize;

mod method;
mod contact;
mod jmaputil;

use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::io::Read;

use rustc_serialize::json::{Json,ToJson};

use jmaputil::FromJson;
use contact::{Contact,PartialContact};
use method::RequestMethod;

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

fn test_contacts() {
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

    println!("{:?}", update);
    println!("{}", update.to_json().to_string());

    let updated = contact.updated_with(&update);

    println!("{:?}", updated);
    println!("{}", updated.to_json().to_string());
}

fn test_methods() {
    let method = match RequestMethod::from_json(&load_json("method.json")) {
        Ok(m) => m,
        Err(e) => panic!("method parse error: {}", e),
    };

    println!("{:?}", method);
    println!("{}", method.to_json().to_string());
}

fn main() {
    //test_contacts();
    test_methods();
}
