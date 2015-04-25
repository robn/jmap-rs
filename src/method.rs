use std::collections::BTreeMap;
use std::default::Default;
use rustc_serialize::json::{Json,ToJson};

use jmaputil::*;
use jmaputil::Presence::*;

use method::RequestMethod::*;

use contact::PartialContact;

#[derive(Clone, PartialEq, Debug)]
pub struct GetRequestArgs {
    ids:         Presence<Vec<String>>,
    properties:  Presence<Vec<String>>,
    since_state: Presence<String>,
}

impl Default for GetRequestArgs {
    fn default() -> GetRequestArgs {
        GetRequestArgs {
            ids:         Absent,
            properties:  Absent,
            since_state: Absent,
        }
    }
}

impl ToJson for GetRequestArgs {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::<String,Json>::new();
        self.ids.to_json_field(&mut d, "ids");
        self.properties.to_json_field(&mut d, "properties");
        self.since_state.to_json_field(&mut d, "sinceState");
        Json::Object(d)
    }
}

impl FromJson for GetRequestArgs {
    fn from_json(json: &Json) -> Result<GetRequestArgs,ParseError> {
        match *json {
            Json::Object(ref o) => {
                let mut args = GetRequestArgs::default();
                args.ids         = try!(FromJsonField::from_json_field(o, "ids"));
                args.properties  = try!(FromJsonField::from_json_field(o, "properties"));
                args.since_state = try!(FromJsonField::from_json_field(o, "sinceState"));
                Ok(args)
            },
            _ => Err(ParseError::InvalidJsonType("GetRequestArgs".to_string())),
        }
    }
}


#[derive(Clone, PartialEq, Debug)]
pub struct GetResponseArgs {
    state:     String,
    ids:       Option<Vec<PartialContact>>,
    not_found: Option<Vec<String>>,
}

impl Default for GetResponseArgs {
    fn default() -> GetResponseArgs {
        GetResponseArgs {
            state:     "".to_string(),
            ids:       None,
            not_found: None,
        }
    }
}

impl ToJson for GetResponseArgs {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::<String,Json>::new();
        self.state.to_json_field(&mut d, "state");
        self.ids.to_json_field(&mut d, "ids");
        self.not_found.to_json_field(&mut d, "notFound");
        Json::Object(d)
    }
}

impl FromJson for GetResponseArgs {
    fn from_json(json: &Json) -> Result<GetResponseArgs,ParseError> {
        match *json {
            Json::Object(ref o) => {
                let mut args = GetResponseArgs::default();
                args.state       = try!(FromJsonField::from_json_field(o, "state"));
                args.ids         = try!(FromJsonField::from_json_field(o, "ids"));
                args.not_found   = try!(FromJsonField::from_json_field(o, "notFound"));
                Ok(args)
            },
            _ => Err(ParseError::InvalidJsonType("GetResponseArgs".to_string())),
        }
    }
}


#[derive(Clone, PartialEq, Debug)]
pub struct GetUpdatesRequestArgs {
    since_state:             String,
    max_changes:             Presence<u64>,
    fetch_records:           Presence<bool>,
    fetch_record_properties: Presence<Vec<String>>,
}

impl Default for GetUpdatesRequestArgs {
    fn default() -> GetUpdatesRequestArgs {
        GetUpdatesRequestArgs {
            since_state:             "".to_string(),
            max_changes:             Absent,
            fetch_records:           Absent,
            fetch_record_properties: Absent,
        }
    }
}

impl ToJson for GetUpdatesRequestArgs {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::<String,Json>::new();
        self.since_state.to_json_field(&mut d, "sinceState");
        self.max_changes.to_json_field(&mut d, "maxChanges");
        self.fetch_records.to_json_field(&mut d, "fetchRecords");
        self.fetch_record_properties.to_json_field(&mut d, "fetchRecordProperties");
        Json::Object(d)
    }
}

impl FromJson for GetUpdatesRequestArgs {
    fn from_json(json: &Json) -> Result<GetUpdatesRequestArgs,ParseError> {
        match *json {
            Json::Object(ref o) => {
                let mut args = GetUpdatesRequestArgs::default();
                args.since_state             = try!(FromJsonField::from_json_field(o, "sinceState"));
                args.max_changes             = try!(FromJsonField::from_json_field(o, "maxChanges"));
                args.fetch_records           = try!(FromJsonField::from_json_field(o, "fetchRecords"));
                args.fetch_record_properties = try!(FromJsonField::from_json_field(o, "fetchRecordProperties"));
                Ok(args)
            },
            _ => Err(ParseError::InvalidJsonType("GetUpdatesRequestArgs".to_string())),
        }
    }
}


#[derive(Clone, PartialEq, Debug)]
pub struct GetUpdatesResponseArgs {
    old_state: String,
    new_state: String,
    changed:   Vec<String>,
    removed:   Vec<String>,
}

impl Default for GetUpdatesResponseArgs {
    fn default() -> GetUpdatesResponseArgs {
        GetUpdatesResponseArgs {
            old_state: "".to_string(),
            new_state: "".to_string(),
            changed:   vec!(),
            removed:   vec!(),
        }
    }
}

impl ToJson for GetUpdatesResponseArgs {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::<String,Json>::new();
        self.old_state.to_json_field(&mut d, "oldState");
        self.new_state.to_json_field(&mut d, "newState");
        self.changed.to_json_field(&mut d, "changed");
        self.removed.to_json_field(&mut d, "removed");
        Json::Object(d)
    }
}

impl FromJson for GetUpdatesResponseArgs {
    fn from_json(json: &Json) -> Result<GetUpdatesResponseArgs,ParseError> {
        match *json {
            Json::Object(ref o) => {
                let mut args = GetUpdatesResponseArgs::default();
                args.old_state = try!(FromJsonField::from_json_field(o, "oldState"));
                args.new_state = try!(FromJsonField::from_json_field(o, "newState"));
                args.changed   = try!(FromJsonField::from_json_field(o, "changed"));
                args.removed   = try!(FromJsonField::from_json_field(o, "removed"));
                Ok(args)
            },
            _ => Err(ParseError::InvalidJsonType("GetUpdatesResponseArgs".to_string())),
        }
    }
}


#[derive(Clone, PartialEq, Debug)]
pub struct SetRequestArgs {
    if_in_state: Presence<String>,
    create:      Presence<BTreeMap<String,PartialContact>>,
    update:      Presence<BTreeMap<String,PartialContact>>,
    destroy:     Presence<Vec<String>>,
}

impl Default for SetRequestArgs {
    fn default() -> SetRequestArgs {
        SetRequestArgs {
            if_in_state: Absent,
            create:      Absent,
            update:      Absent,
            destroy:     Absent,
        }
    }
}

impl ToJson for SetRequestArgs {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::<String,Json>::new();
        self.if_in_state.to_json_field(&mut d, "ifInState");
        self.create.to_json_field(&mut d, "create");
        self.update.to_json_field(&mut d, "update");
        self.destroy.to_json_field(&mut d, "destroy");
        Json::Object(d)
    }
}

impl FromJson for SetRequestArgs {
    fn from_json(json: &Json) -> Result<SetRequestArgs,ParseError> {
        match *json {
            Json::Object(ref o) => {
                let mut args = SetRequestArgs::default();
                args.if_in_state = try!(FromJsonField::from_json_field(o, "ifInState"));
                args.create      = try!(FromJsonField::from_json_field(o, "create"));
                args.update      = try!(FromJsonField::from_json_field(o, "update"));
                args.destroy     = try!(FromJsonField::from_json_field(o, "destroy"));
                Ok(args)
            },
            _ => Err(ParseError::InvalidJsonType("SetRequestArgs".to_string())),
        }
    }
}


#[derive(Clone, PartialEq, Debug)]
pub struct SetResponseArgs {
    old_state:     Option<String>,
    new_state:     String,
    created:       BTreeMap<String,PartialContact>,
    updated:       Vec<String>,
    destroyed:     Vec<String>,
    not_created:   BTreeMap<String,SetError>,
    not_updated:   BTreeMap<String,SetError>,
    not_destroyed: BTreeMap<String,SetError>,
}

impl Default for SetResponseArgs {
    fn default() -> SetResponseArgs {
        SetResponseArgs {
            old_state:     None,
            new_state:     "".to_string(),
            created:       BTreeMap::new(),
            updated:       vec!(),
            destroyed:     vec!(),
            not_created:   BTreeMap::new(),
            not_updated:   BTreeMap::new(),
            not_destroyed: BTreeMap::new(),
        }
    }
}

impl ToJson for SetResponseArgs {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::<String,Json>::new();
        self.old_state.to_json_field(&mut d, "oldState");
        self.new_state.to_json_field(&mut d, "newState");
        self.created.to_json_field(&mut d, "created");
        self.updated.to_json_field(&mut d, "updated");
        self.destroyed.to_json_field(&mut d, "destroyed");
        self.not_created.to_json_field(&mut d, "notCreated");
        self.not_updated.to_json_field(&mut d, "notUpdated");
        self.not_destroyed.to_json_field(&mut d, "notDestroyed");
        Json::Object(d)
    }
}

impl FromJson for SetResponseArgs {
    fn from_json(json: &Json) -> Result<SetResponseArgs,ParseError> {
        match *json {
            Json::Object(ref o) => {
                let mut args = SetResponseArgs::default();
                args.old_state     = try!(FromJsonField::from_json_field(o, "oldState"));
                args.new_state     = try!(FromJsonField::from_json_field(o, "newState"));
                args.created       = try!(FromJsonField::from_json_field(o, "created"));
                args.updated       = try!(FromJsonField::from_json_field(o, "updated"));
                args.destroyed     = try!(FromJsonField::from_json_field(o, "destroyed"));
                args.not_created   = try!(FromJsonField::from_json_field(o, "notCreated"));
                args.not_updated   = try!(FromJsonField::from_json_field(o, "notUpdated"));
                args.not_destroyed = try!(FromJsonField::from_json_field(o, "notDestroyed"));
                Ok(args)
            },
            _ => Err(ParseError::InvalidJsonType("SetResponseArgs".to_string())),
        }
    }
}


#[derive(Clone, PartialEq, Debug)]
pub struct SetError {
    typ:         String,
    description: Option<String>,
}

impl Default for SetError {
    fn default() -> SetError {
        SetError {
            typ:         "".to_string(),
            description: None,
        }
    }
}

impl ToJson for SetError {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::<String,Json>::new();
        self.typ.to_json_field(&mut d, "type");
        self.description.to_json_field(&mut d, "description");
        Json::Object(d)
    }
}

impl FromJson for SetError {
    fn from_json(json: &Json) -> Result<SetError,ParseError> {
        match *json {
            Json::Object(ref o) => {
                let mut se = SetError::default();
                se.typ         = try!(FromJsonField::from_json_field(o, "type"));
                se.description = try!(FromJsonField::from_json_field(o, "description"));
                Ok(se)
            },
            _ => Err(ParseError::InvalidJsonType("SetError".to_string())),
        }
    }
}


#[derive(Clone, PartialEq, Debug)]
pub enum RequestMethod {
    GetContacts(GetRequestArgs, String),
    GetContactUpdates(GetUpdatesRequestArgs, String),
    SetContacts(SetRequestArgs, String),
}

impl ToJson for RequestMethod {
    fn to_json(&self) -> Json {
        Json::Array(
            match *self {
                GetContacts(ref args, ref client_id) =>
                    vec!("getContacts".to_json(), args.to_json(), client_id.to_json()),
                GetContactUpdates(ref args, ref client_id) =>
                    vec!("getContactUpdates".to_json(), args.to_json(), client_id.to_json()),
                SetContacts(ref args, ref client_id) =>
                    vec!("setContacts".to_json(), args.to_json(), client_id.to_json()),
            }
        )
    }
}

impl FromJson for RequestMethod {
    fn from_json(json: &Json) -> Result<RequestMethod,ParseError> {
        match *json {
            Json::Array(ref a) => {
                if let false = a.len() == 3 {
                    return Err(ParseError::InvalidStructure("RequestMethod".to_string()));
                }
                let method = try!(String::from_json(&a[0]));
                let client_id = try!(String::from_json(&a[2]));
                match method.as_str() {
                    "getContacts" =>
                        Ok(GetContacts(try!(GetRequestArgs::from_json(&a[1])), client_id)),
                    "getContactUpdates" =>
                        Ok(GetContactUpdates(try!(GetUpdatesRequestArgs::from_json(&a[1])), client_id)),
                    "setContacts" =>
                        Ok(SetContacts(try!(SetRequestArgs::from_json(&a[1])), client_id)),
                    _ => Err(ParseError::UnknownMethod(method)),
                }
            },
            _ => Err(ParseError::InvalidJsonType("RequestMethod".to_string())),
        }
    }
}
