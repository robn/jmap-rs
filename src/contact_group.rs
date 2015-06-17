use std::collections::BTreeMap;
use std::string::ToString;
use std::default::Default;
use rustc_serialize::json::{Json,ToJson};

use parse::*;
use parse::Presence::*;
use record;
use record::{Record, PartialRecord};


make_record_type!(ContactGroup, PartialContactGroup, "ContactGroup",
    name:        String      => "name",
    contact_ids: Vec<String> => "contactIds"
);
