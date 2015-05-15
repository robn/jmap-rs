use std::collections::BTreeMap;
use std::string::ToString;
use std::default::Default;
use rustc_serialize::json::{Json,ToJson};

use parse::*;
use parse::Presence::*;
use record::{Record, PartialRecord};


#[derive(Clone, PartialEq, Debug)]
pub struct ContactGroup {
    pub id:          String,
    pub name:        String,
    pub contact_ids: Vec<String>,
}

impl Default for ContactGroup {
    fn default() -> ContactGroup {
        ContactGroup {
            id:          "".to_string(),
            name:        "".to_string(),
            contact_ids: vec!(),
        }
    }
}

impl ToJson for ContactGroup {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::<String,Json>::new();
        self.id.to_json_field(&mut d, "id");
        self.name.to_json_field(&mut d, "name");
        self.contact_ids.to_json_field(&mut d, "contactIds");
        Json::Object(d)
    }
}

impl FromJson for ContactGroup {
    fn from_json(json: &Json) -> Result<ContactGroup,ParseError> {
        match *json {
            Json::Object(ref o) => {
                let mut group = ContactGroup::default();
                group.id          = try!(FromJsonField::from_json_field(o, "id"));
                group.name        = try!(FromJsonField::from_json_field(o, "name"));
                group.contact_ids = try!(FromJsonField::from_json_field(o, "contactIds"));
                Ok(group)
            }
            _ => Err(ParseError::InvalidJsonType("ContactGroup".to_string())),
        }
    }
}


#[derive(Clone, PartialEq, Debug)]
pub struct PartialContactGroup {
    pub id:          Presence<String>,
    pub name:        Presence<String>,
    pub contact_ids: Presence<Vec<String>>,
}

impl PartialRecord for PartialContactGroup { }

impl Default for PartialContactGroup {
    fn default() -> PartialContactGroup {
        PartialContactGroup {
            id:          Absent,
            name:        Absent,
            contact_ids: Absent,
        }
    }
}

impl ToJson for PartialContactGroup {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::<String,Json>::new();
        self.id.to_json_field(&mut d, "id");
        self.name.to_json_field(&mut d, "name");
        self.contact_ids.to_json_field(&mut d, "contactIds");
        Json::Object(d)
    }
}

impl FromJson for PartialContactGroup {
    fn from_json(json: &Json) -> Result<PartialContactGroup,ParseError> {
        match *json {
            Json::Object(ref o) => {
                let mut group = PartialContactGroup::default();
                group.id          = try!(FromJsonField::from_json_field(o, "id"));
                group.name        = try!(FromJsonField::from_json_field(o, "name"));
                group.contact_ids = try!(FromJsonField::from_json_field(o, "contactIds"));
                Ok(group)
            }
            _ => Err(ParseError::InvalidJsonType("ContactGroup".to_string())),
        }
    }
}


impl Record for ContactGroup {
    type Partial = PartialContactGroup;

    fn updated_with(&self, p: &PartialContactGroup) -> ContactGroup {
        let mut g = self.clone();
        let u = p.clone();
        if let Present(v) = u.id          { g.id = v };
        if let Present(v) = u.name        { g.name = v };
        if let Present(v) = u.contact_ids { g.contact_ids = v };
        g
    }

    fn to_partial(&self) -> PartialContactGroup {
        PartialContactGroup {
            id:          Present(self.id.clone()),
            name:        Present(self.name.clone()),
            contact_ids: Present(self.contact_ids.clone()),
        }
    }

    fn to_filtered_partial(&self, properties: &Vec<String>) -> PartialContactGroup {
        let mut p = PartialContactGroup::default();
        p.id = Present(self.id.clone());
        for prop in properties.iter() {
            match prop.as_ref() {
                "name"       => p.name =        Present(self.name.clone()),
                "contactIds" => p.contact_ids = Present(self.contact_ids.clone()),
                _            => (),
            }
        }
        p
    }
}
