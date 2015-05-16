use std::default::Default;
use rustc_serialize::json::ToJson;
use parse::{FromJson, Presence};

pub trait PartialRecord: Default + ToJson + FromJson {
    fn id(&self) -> Presence<String>;
    fn set_id(&mut self, id: Presence<String>);
}

pub trait Record: Default + ToJson + FromJson {
    type Partial: PartialRecord;

    fn id(&self) -> String;
    fn set_id(&mut self, id: String);

    fn updated_with(&self, p: &Self::Partial) -> Self;
    fn to_partial(&self) -> Self::Partial;
    fn to_filtered_partial(&self, properties: &Vec<String>) -> Self::Partial;
}


macro_rules! record_methods(
    () => {
        fn id(&self) -> String {
            self.id.clone()
        }
        fn set_id(&mut self, id: String) {
            self.id = id;
        }
    }
);
macro_rules! partial_record_methods(
    () => {
        fn id(&self) -> Presence<String> {
            self.id.clone()
        }
        fn set_id(&mut self, id: Presence<String>) {
            self.id = id;
        }
    }
);
