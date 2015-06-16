use std::collections::BTreeMap;
use std::string::ToString;
use std::default::Default;
use rustc_serialize::json::{Json,ToJson};

use parse::*;

// subtypes shared across record types

make_prop_type!(File, "File",
    url:  String => "url",
    typ:  Option<String> => "type",
    name: Option<String> => "name",
    size: Option<u64> => "size"
);


