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
            MailboxRole::Custom(ref r) => r.to_string(),
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
                    match r.starts_with("x-") {
                        true => Ok(MailboxRole::Custom(r.to_string())),
                        _    => Err(ParseError::InvalidStructure("MailboxRole".to_string())),
                    }
                }
            },
            _ => Err(ParseError::InvalidJsonType("MailboxRole".to_string())),
        }
    }
}


make_record_type!(Mailbox, PartialMailbox, "Mailbox",
    name:                 String              => "name",
    parent_id:            Option<String>      => "parentId",
    role:                 Option<MailboxRole> => "role",
    sort_order:           u64                 => "sortOrder",
    must_be_only_mailbox: bool                => "mustBeOnlyMailbox",
    may_read_items:       bool                => "mayReadItems",
    may_add_items:        bool                => "mayAddItems",
    may_remove_items:     bool                => "mayRemoveItems",
    may_create_child:     bool                => "mayCreateChild",
    may_rename:           bool                => "mayRename",
    may_delete:           bool                => "mayDelete",
    total_messages:       u64                 => "totalMessages",
    unread_messages:      u64                 => "unreadMessages",
    total_threads:        u64                 => "totalThreads",
    unread_threads:       u64                 => "unreadThreads"
);
