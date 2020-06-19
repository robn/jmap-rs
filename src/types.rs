use chrono::{DateTime, NaiveDateTime, UTC};
use rustc_serialize::json::{Json, ToJson};
use std::collections::BTreeMap;
use std::default::Default;
use std::ops::Deref;
use std::string::ToString;

use crate::parse::*;

// subtypes shared across record types

make_prop_type!(File, "File",
    blob_id:  String         => "blobId",
    typ:      Option<String> => "type",
    name:     Option<String> => "name",
    size:     Option<u64>    => "size"
);

#[derive(Clone, PartialEq, Debug)]
pub struct Date(pub DateTime<UTC>);

impl Deref for Date {
    type Target = DateTime<UTC>;
    fn deref<'a>(&'a self) -> &'a Self::Target {
        &self.0
    }
}

impl Default for Date {
    fn default() -> Date {
        Date(DateTime::<UTC>::from_utc(
            NaiveDateTime::from_timestamp(0, 0),
            UTC,
        ))
    }
}

impl ToJson for Date {
    fn to_json(&self) -> Json {
        Json::String(self.format("%Y-%m-%dT%H:%M:%SZ").to_string())
    }
}

impl FromJson for Date {
    fn from_json(json: &Json) -> Result<Date, ParseError> {
        match *json {
            Json::String(ref v) => match v.parse::<DateTime<UTC>>() {
                Ok(dt) => Ok(Date(dt)),
                _ => Err(ParseError::InvalidStructure("Date".to_string())),
            },
            _ => Err(ParseError::InvalidJsonType("Date".to_string())),
        }
    }
}
