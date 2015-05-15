use std::default::Default;
use rustc_serialize::json::ToJson;
use parse::FromJson;

pub trait PartialRecord: Default + ToJson + FromJson { }

pub trait Record: Default + ToJson + FromJson {
    type Partial: PartialRecord;

    fn updated_with(&self, p: &Self::Partial) -> Self;
    fn to_partial(&self) -> Self::Partial;
    fn to_filtered_partial(&self, properties: &Vec<String>) -> Self::Partial;
}
