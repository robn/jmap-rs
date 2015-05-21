use std::collections::BTreeMap;
use std::string::ToString;
use std::default::Default;
use rustc_serialize::json::{Json,ToJson};

use parse::*;
use parse::Presence::*;
use record;
use record::{Record, PartialRecord};


#[derive(Clone, PartialEq, Debug)]
pub struct Calendar {
    pub id:                 String,
    pub name:               String,
    pub colour:             String, // XXX CSS colour type?
    pub is_visible:         bool,
    pub may_read_free_busy: bool,
    pub may_read_items:     bool,
    pub may_add_items:      bool,
    pub may_write_items:    bool,
    pub may_remove_items:   bool,
    pub may_rename:         bool,
    pub may_delete:         bool,
}

impl Default for Calendar {
    fn default() -> Calendar {
        Calendar {
            id:                 record::new_id(),
            name:               "".to_string(),
            colour:             "".to_string(),
            is_visible:         true,
            may_read_free_busy: true,
            may_read_items:     true,
            may_add_items:      true,
            may_write_items:    true,
            may_remove_items:   true,
            may_rename:         true,
            may_delete:         true,
        }
    }
}

impl ToJson for Calendar {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::<String,Json>::new();
        self.id.to_json_field(&mut d, "id");
        self.name.to_json_field(&mut d, "name");
        self.colour.to_json_field(&mut d, "colour");
        self.is_visible.to_json_field(&mut d, "isVisible");
        self.may_read_free_busy.to_json_field(&mut d, "mayReadFreeBusy");
        self.may_read_items.to_json_field(&mut d, "mayReadItems");
        self.may_add_items.to_json_field(&mut d, "mayAddItems");
        self.may_write_items.to_json_field(&mut d, "mayWriteItems");
        self.may_remove_items.to_json_field(&mut d, "mayRemoveItems");
        self.may_rename.to_json_field(&mut d, "mayRename");
        self.may_delete.to_json_field(&mut d, "mayDelete");
        Json::Object(d)
    }
}

impl FromJson for Calendar {
    fn from_json(json: &Json) -> Result<Calendar,ParseError> {
        match *json {
            Json::Object(ref o) => {
                let mut cal = Calendar::default();
                cal.id                 = try!(FromJsonField::from_json_field(o, "id"));
                cal.name               = try!(FromJsonField::from_json_field(o, "name"));
                cal.colour             = try!(FromJsonField::from_json_field(o, "colour"));
                cal.is_visible         = try!(FromJsonField::from_json_field(o, "isVisible"));
                cal.may_read_free_busy = try!(FromJsonField::from_json_field(o, "mayReadFreeBusy"));
                cal.may_read_items     = try!(FromJsonField::from_json_field(o, "mayReadItems"));
                cal.may_add_items      = try!(FromJsonField::from_json_field(o, "mayAddItems"));
                cal.may_write_items    = try!(FromJsonField::from_json_field(o, "mayWriteItems"));
                cal.may_remove_items   = try!(FromJsonField::from_json_field(o, "mayRemoveItems"));
                cal.may_rename         = try!(FromJsonField::from_json_field(o, "mayRename"));
                cal.may_delete         = try!(FromJsonField::from_json_field(o, "mayDelete"));
                Ok(cal)
            }
            _ => Err(ParseError::InvalidJsonType("Calendar".to_string())),
        }
    }
}


#[derive(Clone, PartialEq, Debug)]
pub struct PartialCalendar {
    pub id:                 Presence<String>,
    pub name:               Presence<String>,
    pub colour:             Presence<String>, // XXX CSS colour type?
    pub is_visible:         Presence<bool>,
    pub may_read_free_busy: Presence<bool>,
    pub may_read_items:     Presence<bool>,
    pub may_add_items:      Presence<bool>,
    pub may_write_items:    Presence<bool>,
    pub may_remove_items:   Presence<bool>,
    pub may_rename:         Presence<bool>,
    pub may_delete:         Presence<bool>,
}

impl PartialRecord for PartialCalendar {
    partial_record_methods!();
}

impl Default for PartialCalendar {
    fn default() -> PartialCalendar {
        PartialCalendar {
            id:                 Absent,
            name:               Absent,
            colour:             Absent,
            is_visible:         Absent,
            may_read_free_busy: Absent,
            may_read_items:     Absent,
            may_add_items:      Absent,
            may_write_items:    Absent,
            may_remove_items:   Absent,
            may_rename:         Absent,
            may_delete:         Absent,
        }
    }
}

impl ToJson for PartialCalendar {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::<String,Json>::new();
        self.id.to_json_field(&mut d, "id");
        self.name.to_json_field(&mut d, "name");
        self.colour.to_json_field(&mut d, "colour");
        self.is_visible.to_json_field(&mut d, "isVisible");
        self.may_read_free_busy.to_json_field(&mut d, "mayReadFreeBusy");
        self.may_read_items.to_json_field(&mut d, "mayReadItems");
        self.may_add_items.to_json_field(&mut d, "mayAddItems");
        self.may_write_items.to_json_field(&mut d, "mayWriteItems");
        self.may_remove_items.to_json_field(&mut d, "mayRemoveItems");
        self.may_rename.to_json_field(&mut d, "mayRename");
        self.may_delete.to_json_field(&mut d, "mayDelete");
        Json::Object(d)
    }
}

impl FromJson for PartialCalendar {
    fn from_json(json: &Json) -> Result<PartialCalendar,ParseError> {
        match *json {
            Json::Object(ref o) => {
                let mut cal = PartialCalendar::default();
                cal.id                 = try!(FromJsonField::from_json_field(o, "id"));
                cal.name               = try!(FromJsonField::from_json_field(o, "name"));
                cal.colour             = try!(FromJsonField::from_json_field(o, "colour"));
                cal.is_visible         = try!(FromJsonField::from_json_field(o, "isVisible"));
                cal.may_read_free_busy = try!(FromJsonField::from_json_field(o, "mayReadFreeBusy"));
                cal.may_read_items     = try!(FromJsonField::from_json_field(o, "mayReadItems"));
                cal.may_add_items      = try!(FromJsonField::from_json_field(o, "mayAddItems"));
                cal.may_write_items    = try!(FromJsonField::from_json_field(o, "mayWriteItems"));
                cal.may_remove_items   = try!(FromJsonField::from_json_field(o, "mayRemoveItems"));
                cal.may_rename         = try!(FromJsonField::from_json_field(o, "mayRename"));
                cal.may_delete         = try!(FromJsonField::from_json_field(o, "mayDelete"));
                Ok(cal)
            }
            _ => Err(ParseError::InvalidJsonType("Calendar".to_string())),
        }
    }
}


impl Record for Calendar {
    type Partial = PartialCalendar;

    record_methods!();

    fn updated_with(&self, p: &PartialCalendar) -> Calendar {
        let mut c = self.clone();
        let u = p.clone();
        if let Present(v) = u.id                 { c.id = v };
        if let Present(v) = u.name               { c.name = v };
        if let Present(v) = u.colour             { c.colour = v };
        if let Present(v) = u.is_visible         { c.is_visible = v };
        if let Present(v) = u.may_read_free_busy { c.may_read_free_busy = v };
        if let Present(v) = u.may_read_items     { c.may_read_items = v };
        if let Present(v) = u.may_add_items      { c.may_add_items = v };
        if let Present(v) = u.may_write_items    { c.may_write_items = v };
        if let Present(v) = u.may_remove_items   { c.may_remove_items = v };
        if let Present(v) = u.may_rename         { c.may_rename = v };
        if let Present(v) = u.may_delete         { c.may_delete = v };
        c
    }

    fn to_partial(&self) -> PartialCalendar {
        PartialCalendar {
            id:                 Present(self.id.clone()),
            name:               Present(self.name.clone()),
            colour:             Present(self.name.clone()),
            is_visible:         Present(self.is_visible),
            may_read_free_busy: Present(self.may_read_free_busy),
            may_read_items:     Present(self.may_read_items),
            may_add_items:      Present(self.may_add_items),
            may_write_items:    Present(self.may_write_items),
            may_remove_items:   Present(self.may_remove_items),
            may_rename:         Present(self.may_rename),
            may_delete:         Present(self.may_delete),
        }
    }

    fn to_filtered_partial(&self, properties: &Vec<String>) -> PartialCalendar {
        let mut p = PartialCalendar::default();
        p.id = Present(self.id.clone());
        for prop in properties.iter() {
            match prop.as_ref() {
                "name"            => p.name =               Present(self.name.clone()),
                "colour"          => p.colour =             Present(self.name.clone()),
                "isVisible"       => p.is_visible =         Present(self.is_visible),
                "mayReadFreeBusy" => p.may_read_free_busy = Present(self.may_read_free_busy),
                "mayReadItems"    => p.may_read_items =     Present(self.may_read_items),
                "mayAddItems"     => p.may_add_items =      Present(self.may_add_items),
                "mayWriteItems"   => p.may_write_items =    Present(self.may_write_items),
                "mayRemoveItems"  => p.may_remove_items =   Present(self.may_remove_items),
                "mayRename"       => p.may_rename =         Present(self.may_rename),
                "mayDelete"       => p.may_delete =         Present(self.may_delete),
                _                 => (),
            }
        }
        p
    }
}
