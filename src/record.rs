use std::default::Default;
use rustc_serialize::json::ToJson;
use uuid::Uuid;
use parse::{FromJson, Presence};

pub trait PartialRecord: Default + ToJson + FromJson {
    fn id(&self) -> Presence<String>;
}

pub trait Record: Default + ToJson + FromJson {
    type Partial: PartialRecord;

    fn id(&self) -> String;

    fn updated_with(&self, p: &Self::Partial) -> Self;
    fn to_partial(&self) -> Self::Partial;
    fn to_filtered_partial(&self, properties: &Vec<String>) -> Self::Partial;
}

pub fn new_id() -> String {
    Uuid::new_v4().to_hyphenated_string()
}

macro_rules! make_record_type {
    ($record: ident, $partialrecord: ident, $recname: expr,
     $($field: ident: $ty: ty => $prop: expr),*) => {
        #[derive(Clone, PartialEq, Debug)]
        pub struct $record {
            id: String,
            $($field: $ty),*
        }

        impl Default for $record {
            fn default() -> $record {
                $record {
                    id: record::new_id(),
                    ..Default::default()
                }
            }
        }

        impl ToJson for $record {
            fn to_json(&self) -> Json {
                let mut d = BTreeMap::<String,Json>::new();
                self.id.to_json_field(&mut d, "id");
                $(self.$field.to_json_field(&mut d, $prop);)*
                Json::Object(d)
            }
        }

        impl FromJson for $record {
            fn from_json(json: &Json) -> Result<$record,ParseError> {
                match *json {
                    Json::Object(ref o) => {
                        let mut r = $record::default();
                        r.id = try!(FromJsonField::from_json_field(o, "id"));
                        $(r.$field = try!(FromJsonField::from_json_field(o, $prop));)*
                        Ok(r)
                    }
                    _ => Err(ParseError::InvalidJsonType($recname.to_string())),
                }
            }
        }


        #[derive(Clone, PartialEq, Debug)]
        pub struct $partialrecord {
            id: Presence<String>,
            $($field: Presence<$ty>),*
        }

        impl PartialRecord for $partialrecord {
            fn id(&self) -> Presence<String> {
                self.id.clone()
            }
        }

        impl Default for $partialrecord {
            fn default() -> $partialrecord {
                $partialrecord {
                    id: Absent,
                    $($field: Absent),*
                }
            }
        }

        impl ToJson for $partialrecord {
            fn to_json(&self) -> Json {
                let mut d = BTreeMap::<String,Json>::new();
                self.id.to_json_field(&mut d, "id");
                $(self.$field.to_json_field(&mut d, $prop);)*
                Json::Object(d)
            }
        }

        impl FromJson for $partialrecord {
            fn from_json(json: &Json) -> Result<$partialrecord,ParseError> {
                match *json {
                    Json::Object(ref o) => {
                        let mut r = $partialrecord::default();
                        r.id = try!(FromJsonField::from_json_field(o, "id"));
                        $(r.$field = try!(FromJsonField::from_json_field(o, $prop));)*
                        Ok(r)
                    }
                    _ => Err(ParseError::InvalidJsonType($recname.to_string())),
                }
            }
        }


        impl Record for $record {
            type Partial = $partialrecord;

            fn id(&self) -> String {
                self.id.clone()
            }

            fn updated_with(&self, p: &$partialrecord) -> $record {
                let mut r = self.clone();
                let u = p.clone();
                if let Present(v) = u.id { r.id = v };
                $(if let Present(v) = u.$field { r.$field = v };)*
                r
            }

            fn to_partial(&self) -> $partialrecord {
                $partialrecord {
                    id: Present(self.id.clone()),
                    $($field: Present(self.$field.clone()),)*
                }
            }

            fn to_filtered_partial(&self, properties: &Vec<String>) -> $partialrecord {
                let mut p = $partialrecord::default();
                p.id = Present(self.id.clone());
                for prop in properties.iter() {
                    match prop.as_ref() {
                        $($prop => p.$field = Present(self.$field.clone()),)*
                        _ => ()
                    }
                }
                p
            }
        }
    }
}
