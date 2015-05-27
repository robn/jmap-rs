use std::collections::BTreeMap;
use std::string::ToString;
use std::default::Default;
use rustc_serialize::json::{Json,ToJson};

use parse::*;
use parse::Presence::*;
use record;
use record::{Record, PartialRecord};


#[derive(Clone, PartialEq, Debug)]
pub enum MailboxRole {
    Inbox,
    Archive,
    Drafts,
    Outbox,
    Sent,
    Trash,
    Spam,
    Templates,
    Custom(String),
}

impl ToString for MailboxRole {
    fn to_string(&self) -> String {
        match *self {
            MailboxRole::Inbox         => "inbox".to_string(),
            MailboxRole::Archive       => "archive".to_string(),
            MailboxRole::Drafts        => "drafts".to_string(),
            MailboxRole::Outbox        => "outbox".to_string(),
            MailboxRole::Sent          => "sent".to_string(),
            MailboxRole::Trash         => "trash".to_string(),
            MailboxRole::Spam          => "spam".to_string(),
            MailboxRole::Templates     => "templates".to_string(),
            MailboxRole::Custom(ref r) => {
                let mut s = "x-".to_string();
                s.push_str(r.as_ref());
                s
            },
        }
    }
}

impl ToJson for MailboxRole {
    fn to_json(&self) -> Json {
        Json::String(self.to_string())
    }
}

impl FromJson for MailboxRole {
    fn from_json(json: &Json) -> Result<MailboxRole,ParseError> {
        match *json {
            Json::String(ref v) => match v.as_ref() {
                "inbox"     => Ok(MailboxRole::Inbox),
                "archive"   => Ok(MailboxRole::Archive),
                "drafts"    => Ok(MailboxRole::Drafts),
                "outbox"    => Ok(MailboxRole::Outbox),
                "sent"      => Ok(MailboxRole::Sent),
                "trash"     => Ok(MailboxRole::Trash),
                "spam"      => Ok(MailboxRole::Spam),
                "templates" => Ok(MailboxRole::Templates),
                r => {
                    let bits: Vec<&str> = r.splitn(2, '-').collect();
                    match bits.len() {
                        2 => Ok(MailboxRole::Custom(bits[1].to_string())),
                        _ => Err(ParseError::InvalidStructure("MailboxRole".to_string())),
                    }
                }
            },
            _ => Err(ParseError::InvalidJsonType("MailboxRole".to_string())),
        }
    }
}


#[derive(Clone, PartialEq, Debug)]
pub struct Mailbox {
    pub id:                   String,
    pub name:                 String,
    pub parent_id:            Option<String>,
    pub role:                 Option<MailboxRole>,
    pub precedence:           i32,
    pub must_be_only_mailbox: bool,
    pub may_read_items:       bool,
    pub may_add_items:        bool,
    pub may_remove_items:     bool,
    pub may_create_child:     bool,
    pub may_rename:           bool,
    pub may_delete:           bool,
    pub total_messages:       usize,
    pub unread_messages:      usize,
    pub total_threads:        usize,
    pub unread_threads:       usize,
}

impl Default for Mailbox {
    fn default() -> Mailbox {
        Mailbox {
            id:                   record::new_id(),
            name:                 "".to_string(),
            parent_id:            None,
            role:                 None,
            precedence:           0,
            must_be_only_mailbox: false,
            may_read_items:       true,
            may_add_items:        true,
            may_remove_items:     true,
            may_create_child:     true,
            may_rename:           true,
            may_delete:           true,
            total_messages:       0,
            unread_messages:      0,
            total_threads:        0,
            unread_threads:       0,
        }
    }
}


impl ToJson for Mailbox {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::<String,Json>::new();
        self.id.to_json_field(&mut d, "id");
        self.name.to_json_field(&mut d, "name");
        self.parent_id.to_json_field(&mut d, "parentId");
        self.role.to_json_field(&mut d, "role");
        self.precedence.to_json_field(&mut d, "precedence");
        self.must_be_only_mailbox.to_json_field(&mut d, "mustBeOnlyMailbox");
        self.may_read_items.to_json_field(&mut d, "mayReadItems");
        self.may_add_items.to_json_field(&mut d, "mayAddItems");
        self.may_remove_items.to_json_field(&mut d, "mayRemoveItems");
        self.may_create_child.to_json_field(&mut d, "mayCreateChild");
        self.may_rename.to_json_field(&mut d, "mayRename");
        self.may_delete.to_json_field(&mut d, "mayDelete");
        self.total_messages.to_json_field(&mut d, "totalMessages");
        self.unread_messages.to_json_field(&mut d, "unreadMessages");
        self.total_threads.to_json_field(&mut d, "totalThreads");
        self.unread_threads.to_json_field(&mut d, "unreadThreads");
        Json::Object(d)
    }
}

impl FromJson for Mailbox {
    fn from_json(json: &Json) -> Result<Mailbox,ParseError> {
        match *json {
            Json::Object(ref o) => {
                let mut m = Mailbox::default();
                m.id                   = try!(FromJsonField::from_json_field(o, "id"));
                m.name                 = try!(FromJsonField::from_json_field(o, "name"));
                m.parent_id            = try!(FromJsonField::from_json_field(o, "parentId"));
                m.role                 = try!(FromJsonField::from_json_field(o, "role"));
                m.precedence           = try!(FromJsonField::from_json_field(o, "precedence"));
                m.must_be_only_mailbox = try!(FromJsonField::from_json_field(o, "mustBeOnlyMailbox"));
                m.may_read_items       = try!(FromJsonField::from_json_field(o, "mayReadItems"));
                m.may_add_items        = try!(FromJsonField::from_json_field(o, "mayAddItems"));
                m.may_remove_items     = try!(FromJsonField::from_json_field(o, "mayRemoveItems"));
                m.may_create_child     = try!(FromJsonField::from_json_field(o, "mayCreateChild"));
                m.may_rename           = try!(FromJsonField::from_json_field(o, "mayRename"));
                m.may_delete           = try!(FromJsonField::from_json_field(o, "mayDelete"));
                m.total_messages       = try!(FromJsonField::from_json_field(o, "totalMessages"));
                m.unread_messages      = try!(FromJsonField::from_json_field(o, "unreadMessages"));
                m.total_threads        = try!(FromJsonField::from_json_field(o, "totalThreads"));
                m.unread_threads       = try!(FromJsonField::from_json_field(o, "unreadThreads"));
                Ok(m)
            }
            _ => Err(ParseError::InvalidJsonType("Mailbox".to_string())),
        }
    }
}


#[derive(Clone, PartialEq, Debug)]
pub struct PartialMailbox {
    pub id:                   Presence<String>,
    pub name:                 Presence<String>,
    pub parent_id:            Presence<Option<String>>,
    pub role:                 Presence<Option<MailboxRole>>,
    pub precedence:           Presence<i32>,
    pub must_be_only_mailbox: Presence<bool>,
    pub may_read_items:       Presence<bool>,
    pub may_add_items:        Presence<bool>,
    pub may_remove_items:     Presence<bool>,
    pub may_create_child:     Presence<bool>,
    pub may_rename:           Presence<bool>,
    pub may_delete:           Presence<bool>,
    pub total_messages:       Presence<usize>,
    pub unread_messages:      Presence<usize>,
    pub total_threads:        Presence<usize>,
    pub unread_threads:       Presence<usize>,
}

impl PartialRecord for PartialMailbox {
    partial_record_methods!();
}

impl Default for PartialMailbox {
    fn default() -> PartialMailbox {
        PartialMailbox {
            id:                   Absent,
            name:                 Absent,
            parent_id:            Absent,
            role:                 Absent,
            precedence:           Absent,
            must_be_only_mailbox: Absent,
            may_read_items:       Absent,
            may_add_items:        Absent,
            may_remove_items:     Absent,
            may_create_child:     Absent,
            may_rename:           Absent,
            may_delete:           Absent,
            total_messages:       Absent,
            unread_messages:      Absent,
            total_threads:        Absent,
            unread_threads:       Absent,
        }
    }
}

impl ToJson for PartialMailbox {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::<String,Json>::new();
        self.id.to_json_field(&mut d, "id");
        self.name.to_json_field(&mut d, "name");
        self.parent_id.to_json_field(&mut d, "parentId");
        self.role.to_json_field(&mut d, "role");
        self.precedence.to_json_field(&mut d, "precedence");
        self.must_be_only_mailbox.to_json_field(&mut d, "mustBeOnlyMailbox");
        self.may_read_items.to_json_field(&mut d, "mayReadItems");
        self.may_add_items.to_json_field(&mut d, "mayAddItems");
        self.may_remove_items.to_json_field(&mut d, "mayRemoveItems");
        self.may_create_child.to_json_field(&mut d, "mayCreateChild");
        self.may_rename.to_json_field(&mut d, "mayRename");
        self.may_delete.to_json_field(&mut d, "mayDelete");
        self.total_messages.to_json_field(&mut d, "totalMessages");
        self.unread_messages.to_json_field(&mut d, "unreadMessages");
        self.total_threads.to_json_field(&mut d, "totalThreads");
        self.unread_threads.to_json_field(&mut d, "unreadThreads");
        Json::Object(d)
    }
}

impl FromJson for PartialMailbox {
    fn from_json(json: &Json) -> Result<PartialMailbox,ParseError> {
        match *json {
            Json::Object(ref o) => {
                let mut m = PartialMailbox::default();
                m.id                   = try!(FromJsonField::from_json_field(o, "id"));
                m.name                 = try!(FromJsonField::from_json_field(o, "name"));
                m.parent_id            = try!(FromJsonField::from_json_field(o, "parentId"));
                m.role                 = try!(FromJsonField::from_json_field(o, "role"));
                m.precedence           = try!(FromJsonField::from_json_field(o, "precedence"));
                m.must_be_only_mailbox = try!(FromJsonField::from_json_field(o, "mustBeOnlyMailbox"));
                m.may_read_items       = try!(FromJsonField::from_json_field(o, "mayReadItems"));
                m.may_add_items        = try!(FromJsonField::from_json_field(o, "mayAddItems"));
                m.may_remove_items     = try!(FromJsonField::from_json_field(o, "mayRemoveItems"));
                m.may_create_child     = try!(FromJsonField::from_json_field(o, "mayCreateChild"));
                m.may_rename           = try!(FromJsonField::from_json_field(o, "mayRename"));
                m.may_delete           = try!(FromJsonField::from_json_field(o, "mayDelete"));
                m.total_messages       = try!(FromJsonField::from_json_field(o, "totalMessages"));
                m.unread_messages      = try!(FromJsonField::from_json_field(o, "unreadMessages"));
                m.total_threads        = try!(FromJsonField::from_json_field(o, "totalThreads"));
                m.unread_threads       = try!(FromJsonField::from_json_field(o, "unreadThreads"));
                Ok(m)
            }
            _ => Err(ParseError::InvalidJsonType("Mailbox".to_string())),
        }
    }
}


impl Record for Mailbox {
    type Partial = PartialMailbox;

    record_methods!();

    fn updated_with(&self, p: &PartialMailbox) -> Mailbox {
        let mut m = self.clone();
        let u = p.clone();
        if let Present(v) = u.id                   { m.id = v };
        if let Present(v) = u.name                 { m.name = v };
        if let Present(v) = u.parent_id            { m.parent_id = v };
        if let Present(v) = u.role                 { m.role = v };
        if let Present(v) = u.precedence           { m.precedence = v };
        if let Present(v) = u.must_be_only_mailbox { m.must_be_only_mailbox = v };
        if let Present(v) = u.may_read_items       { m.may_read_items = v };
        if let Present(v) = u.may_add_items        { m.may_add_items = v };
        if let Present(v) = u.may_remove_items     { m.may_remove_items = v };
        if let Present(v) = u.may_create_child     { m.may_create_child = v };
        if let Present(v) = u.may_rename           { m.may_rename = v };
        if let Present(v) = u.may_delete           { m.may_delete = v };
        if let Present(v) = u.total_messages       { m.total_messages = v };
        if let Present(v) = u.unread_messages      { m.unread_messages = v };
        if let Present(v) = u.total_threads        { m.total_threads = v };
        if let Present(v) = u.unread_threads       { m.unread_threads = v };
        m
    }

    fn to_partial(&self) -> PartialMailbox {
        PartialMailbox {
            id:                   Present(self.id.clone()),
            name:                 Present(self.name.clone()),
            parent_id:            Present(self.parent_id.clone()),
            role:                 Present(self.role.clone()),
            precedence:           Present(self.precedence),
            must_be_only_mailbox: Present(self.must_be_only_mailbox),
            may_read_items:       Present(self.may_read_items),
            may_add_items:        Present(self.may_add_items),
            may_remove_items:     Present(self.may_remove_items),
            may_create_child:     Present(self.may_create_child),
            may_rename:           Present(self.may_rename),
            may_delete:           Present(self.may_delete),
            total_messages:       Present(self.total_messages),
            unread_messages:      Present(self.unread_messages),
            total_threads:        Present(self.total_threads),
            unread_threads:       Present(self.unread_threads),
        }
    }

    fn to_filtered_partial(&self, properties: &Vec<String>) -> PartialMailbox {
        let mut p = PartialMailbox::default();
        p.id = Present(self.id.clone());
        for prop in properties.iter() {
            match prop.as_ref() {
                "name"              => p.name =                 Present(self.name.clone()),
                "parentId"          => p.parent_id =            Present(self.parent_id.clone()),
                "role"              => p.role =                 Present(self.role.clone()),
                "precedence"        => p.precedence =           Present(self.precedence),
                "mustBeOnlyMailbox" => p.must_be_only_mailbox = Present(self.must_be_only_mailbox),
                "mayReadItems"      => p.may_read_items =       Present(self.may_read_items),
                "mayAddItems"       => p.may_add_items =        Present(self.may_add_items),
                "mayRemoveItems"    => p.may_remove_items =     Present(self.may_remove_items),
                "mayCreateChild"    => p.may_create_child =     Present(self.may_create_child),
                "mayRename"         => p.may_rename =           Present(self.may_rename),
                "mayDelete"         => p.may_delete =           Present(self.may_delete),
                "totalMessages"     => p.total_messages =       Present(self.total_messages),
                "unreadMessages"    => p.unread_messages =      Present(self.unread_messages),
                "totalThreads"      => p.total_threads =        Present(self.total_threads),
                "unreadThreads"     => p.unread_threads =       Present(self.unread_threads),
                _                   => (),
            }
        }
        p
    }
}
