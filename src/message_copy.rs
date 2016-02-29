use std::collections::BTreeMap;
use rustc_serialize::json::{Json,ToJson};

use parse::*;
use record::Record;

use message::Message;
use method::SetError;

make_prop_type!(MessageCopy, "MessageCopy",
    message_id:  String      => "messageId",
    mailbox_ids: Vec<String> => "mailboxIds",
    is_unread:   bool        => "isUnread",
    is_flagged:  bool        => "isFlagged",
    is_answered: bool        => "isAnswered",
    is_draft:    bool        => "isDraft"
);

make_method_args_type!(CopyMessagesRequestArgs, "CopyMessagesRequestArgs",
    from_account_id: Presence<String>             => "accountId",
    to_account_id:   Presence<String>             => "accountId",
    messages:        BTreeMap<String,MessageCopy> => "messages"
);

make_method_args_type!(CopyMessagesResponseArgs, "CopyMessagesResponseArgs",
    from_account_id: String                                        => "accountId",
    to_account_id:   String                                        => "accountId",
    created:         BTreeMap<String,<Message as Record>::Partial> => "created",
    not_created:     BTreeMap<String,SetError>                     => "notCreated"
);
