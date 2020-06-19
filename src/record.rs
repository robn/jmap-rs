use crate::parse::{FromJson, Presence};
use rustc_serialize::json::ToJson;
use std::default::Default;
use uuid::Uuid;

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
    Uuid::new_v4().hyphenated().to_string()
}
