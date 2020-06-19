use rustc_serialize::json::{Json, ToJson};
use std::collections::BTreeMap;
use std::default::Default;
use std::string::ToString;

use crate::parse::Presence::*;
use crate::parse::*;
use crate::record;
use crate::record::{PartialRecord, Record};

make_record_type!(Calendar, PartialCalendar, "Calendar",
    name:               String => "name",
    color:              String => "color", // XXX CSS colour type?
    sort_order:         u64    => "sortOrder",
    is_visible:         bool   => "isVisible",
    may_read_free_busy: bool   => "mayReadFreeBusy",
    may_read_items:     bool   => "mayReadItems",
    may_add_items:      bool   => "mayAddItems",
    may_modify_items:   bool   => "mayModifyItems",
    may_remove_items:   bool   => "mayRemoveItems",
    may_rename:         bool   => "mayRename",
    may_delete:         bool   => "mayDelete"
);
