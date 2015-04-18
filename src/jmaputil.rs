use std::string::ToString;
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;
use rustc_serialize::json::{Json,ToJson};


#[derive(Clone, PartialEq, Debug)]
pub enum ParseError {
    InvalidJsonType(String),
    InvalidStructure(String),
    MissingField(String),
}

impl Error for ParseError {
    fn description(&self) -> &str {
        match *self {
            ParseError::InvalidJsonType(_)  => "invalid JSON type for conversion",
            ParseError::InvalidStructure(_) => "invalid value structure for conversion",
            ParseError::MissingField(_)     => "missing field",
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            ParseError::InvalidJsonType(ref e)  => format!("invalid JSON type for conversion to {}", e),
            ParseError::InvalidStructure(ref e) => format!("invalid value structure for conversion to {}", e),
            ParseError::MissingField(ref e)     => format!("missing field \"{}\"", e),
        }.to_string())
    }
}


// trait for things that can be created from a JSON fragment
pub trait FromJson {
    fn from_json(json: &Json) -> Result<Self,ParseError>;
}


// conversions for system types
impl FromJson for String {
    fn from_json(json: &Json) -> Result<String,ParseError> {
        match *json {
            Json::String(ref s) => Ok(s.to_string()),
            _                   => Err(ParseError::InvalidJsonType("String".to_string())),
        }
    }
}
impl FromJson for u64 {
    fn from_json(json: &Json) -> Result<u64,ParseError> {
        match *json {
            Json::U64(n) => Ok(n),
            Json::I64(n) => Ok(n as u64),
            _            => Err(ParseError::InvalidJsonType("u64".to_string())),
        }
    }
}
impl FromJson for bool {
    fn from_json(json: &Json) -> Result<bool,ParseError> {
        match *json {
            Json::Boolean(b) => Ok(b),
            _                => Err(ParseError::InvalidJsonType("bool".to_string())),
        }
    }
}

impl<T> FromJson for Vec<T> where T: FromJson {
    fn from_json(json: &Json) -> Result<Vec<T>,ParseError> {
        match *json {
            Json::Array(ref a) => {
                let (ok, mut err): (Vec<_>,Vec<_>)  = a.iter().map(|ref j| T::from_json(j)).partition(|ref r| match **r { Ok(_) => true, Err(_) => false });
                match err.len() {
                    0 => Ok(ok.into_iter().map(|r| r.ok().unwrap()).collect()),
                    _ => Err(err.remove(0).err().unwrap()),
                }
            }
            _ => Err(ParseError::InvalidJsonType("Vec".to_string())),
        }
    }
}


// helpers to extract native value from JSON object field
#[inline]
pub fn from_json_field<T>(json: &BTreeMap<String,Json>, field: &str) -> Result<T,ParseError> where T: FromJson {
    match json.get(field) {
        Some(ref v) => T::from_json(&v),
        None        => Err(ParseError::MissingField(field.to_string())),
    }
}
#[inline]
pub fn from_json_field_opt<T>(json: &BTreeMap<String,Json>, field: &str) -> Result<Option<T>,ParseError> where T: FromJson {
    match json.get(field) {
        Some(ref v) => match **v {
            Json::Null => Ok(None),
            _ => match T::from_json(&v) {
                Ok(vv) => Ok(Some(vv)),
                Err(e) => Err(e),
            }
        },
        None => Ok(None),
    }
}


// helpers to install native value into JSON object field
#[inline]
pub fn to_json_field<T>(json: &mut BTreeMap<String,Json>, field: &str, v: &T) where T: ToJson {
    json.insert(field.to_string(), v.to_json());
}

#[inline]
pub fn to_json_field_opt<T>(json: &mut BTreeMap<String,Json>, field: &str, v: &Option<T>) where T: ToJson {
    json.insert(field.to_string(), match *v {
        Some(ref vv) => vv.to_json(),
        None         => Json::Null,
    });
}
