use rustc_serialize::json::{Json, ToJson};
use std::collections::BTreeMap;
use std::default::Default;
use std::string::ToString;

use crate::parse::Presence::*;
use crate::parse::*;
use crate::record;
use crate::record::{PartialRecord, Record};
use crate::types::Date;

make_prop_type!(Emailer, "Emailer",
    name:  String => "name",
    email: String => "email"
);

make_prop_type!(Attachment, "Attachment",
    blob_id:   String         => "blobId",
    typ:       String         => "type",
    name:      String         => "name",
    size:      u64            => "size",
    cid:       Option<String> => "cid",
    is_inline: bool           => "isInline",
    width:     Option<u64>    => "width",
    height:    Option<u64>    => "height"
);

make_record_type!(Message, PartialMessage, "Message",
    blob_id:                String                              => "blobId",
    thread_id:              String                              => "threadId",
    mailbox_ids:            Vec<String>                         => "mailboxIds",
    in_reply_to_message_id: Option<String>                      => "inReplyToMessageId",
    is_unread:              bool                                => "isUnread",
    is_flagged:             bool                                => "isFlagged",
    is_answered:            bool                                => "isAnswered",
    is_draft:               bool                                => "isDraft",
    has_attachment:         bool                                => "hasAttachment",
    headers:                BTreeMap<String,String>             => "headers",
    sender:                 Option<Emailer>                     => "sender",
    from:                   Option<Vec<Emailer>>                => "from",
    to:                     Option<Vec<Emailer>>                => "to",
    cc:                     Option<Vec<Emailer>>                => "cc",
    bcc:                    Option<Vec<Emailer>>                => "bcc",
    reply_to:               Option<Vec<Emailer>>                => "replyTo",
    subject:                String                              => "subject",
    date:                   Date                                => "date",
    size:                   u64                                 => "size",
    preview:                String                              => "preview",
    text_body:              Option<String>                      => "textBody",
    html_body:              Option<String>                      => "htmlBody",
    attachments:            Option<Vec<Attachment>>             => "attachments",
    attached_messages:      Option<BTreeMap<String,Message>>    => "attachedMessages"
);
