use std::string::ToString;
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;
use rustc_serialize::json::{Json,ToJson};

use parse::Presence::*;

#[derive(Clone, PartialEq, Debug)]
pub enum ParseError {
    InvalidJsonType(String),
    InvalidStructure(String),
    MissingField(String),
    UnknownMethod(String),
}

impl Error for ParseError {
    fn description(&self) -> &str {
        match *self {
            ParseError::InvalidJsonType(_)  => "invalid JSON type for conversion",
            ParseError::InvalidStructure(_) => "invalid value structure for conversion",
            ParseError::MissingField(_)     => "missing field",
            ParseError::UnknownMethod(_)    => "unknown method",
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            ParseError::InvalidJsonType(ref e)  => format!("invalid JSON type for conversion to {}", e),
            ParseError::InvalidStructure(ref e) => format!("invalid value structure for conversion to {}", e),
            ParseError::MissingField(ref e)     => format!("missing field \"{}\"", e),
            ParseError::UnknownMethod(ref e)    => format!("unknown method \"{}\"", e),
        }.to_string())
    }
}


#[derive(Clone, PartialEq, Debug)]
pub enum Presence<T> {
    Present(T),
    Absent,
}
impl<T> Presence<T> {
    pub fn as_option<'a>(&'a self) -> Option<&'a T> {
        match *self {
            Present(ref tt) => Some(&tt),
            Absent          => None,
        }
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

impl<T> FromJson for BTreeMap<String,T> where T: FromJson {
    fn from_json(json: &Json) -> Result<Self,ParseError> {
        match *json {
            Json::Object(ref o) => {
                let mut m = BTreeMap::<String,T>::new();
                for (k, v) in o.iter() {
                    let vv = try!(T::from_json(v));
                    m.insert(k.clone(), vv);
                }
                Ok(m)
            },
            _ => Err(ParseError::InvalidJsonType("BTreeMap".to_string()))
        }
    }
}



pub trait FromJsonField {
    fn from_json_field(json: &BTreeMap<String,Json>, field: &str) -> Result<Self,ParseError>;
}


impl<T> FromJsonField for T where T: FromJson {
    fn from_json_field(json: &BTreeMap<String,Json>, field: &str) -> Result<Self,ParseError> {
        match json.get(field) {
            Some(ref v) => T::from_json(&v),
            None        => Err(ParseError::MissingField(field.to_string())),
        }
    }
}
impl<T> FromJsonField for Option<T> where T: FromJson {
    fn from_json_field(json: &BTreeMap<String,Json>, field: &str) -> Result<Self,ParseError> {
        match json.get(field) {
            Some(v) => {
                match *v {
                    Json::Null => Ok(None),
                    _ => match T::from_json(&v) {
                        Ok(j)  => Ok(Some(j)),
                        Err(e) => Err(e),
                    }
                }
            }
            None => Ok(None),
        }
    }
}
impl<T> FromJsonField for Presence<T> where T: FromJson {
    fn from_json_field(json: &BTreeMap<String,Json>, field: &str) -> Result<Self,ParseError> {
        match json.get(field) {
            Some(ref v) => {
                match T::from_json(&v) {
                    Ok(j)  => Ok(Present(j)),
                    Err(e) => Err(e),
                }
            }
            None => Ok(Absent),
        }
    }
}
impl <T> FromJsonField for Presence<Option<T>> where T: FromJson {
    fn from_json_field(json: &BTreeMap<String,Json>, field: &str) -> Result<Self,ParseError> {
        match json.get(field) {
            Some(v) => {
                match *v {
                    Json::Null => Ok(Present(None)),
                    _ => match T::from_json(&v) {
                        Ok(j)  => Ok(Present(Some(j))),
                        Err(e) => Err(e),
                    }
                }
            }
            None => Ok(Absent),
        }
    }
}



pub trait ToJsonField {
    fn to_json_field(&self, json: &mut BTreeMap<String,Json>, field: &str);
}

impl<T> ToJsonField for T where T: ToJson {
    fn to_json_field(&self, json: &mut BTreeMap<String,Json>, field: &str) {
        json.insert(field.to_string(), self.to_json());
    }
}

impl<T> ToJsonField for Presence<T> where T: ToJson {
    fn to_json_field(&self, json: &mut BTreeMap<String,Json>, field: &str) {
        if let Present(ref v) = *self {
            json.insert(field.to_string(), v.to_json());
        }
    }
}
