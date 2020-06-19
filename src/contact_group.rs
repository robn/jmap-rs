use rustc_serialize::json::{Json, ToJson};
use std::collections::BTreeMap;
use std::default::Default;
use std::string::ToString;

use crate::parse::Presence::*;
use crate::parse::*;
use crate::record;
use crate::record::{PartialRecord, Record};

make_record_type!(ContactGroup, PartialContactGroup, "ContactGroup",
    name:        String      => "name",
    contact_ids: Vec<String> => "contactIds"
);
