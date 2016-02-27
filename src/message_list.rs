use std::collections::BTreeMap;
use std::marker::PhantomData;
use rustc_serialize::json::{Json,ToJson};

use parse::*;
use record::Record;
use types::Date;

make_prop_type!(FilterOperator, "FilterOperator",
    operator:   String      => "operator",
    conditions: Vec<Filter> => "conditions"
);

make_prop_type!(FilterCondition, "FilterCondition",
    in_mailboxes:      Presence<Vec<String>> => "inMailboxes",
    not_in_mailboxes:  Presence<Vec<String>> => "notInnMailboxes",
    before:            Presence<Date>        => "before",
    after:             Presence<Date>        => "after",
    min_size:          Presence<u64>         => "minSize",
    max_size:          Presence<u64>         => "maxSize",
    thread_is_flagged: Presence<bool>        => "threadIsFlagged",
    thread_is_unread:  Presence<bool>        => "threadIsUnread",
    is_flagged:        Presence<bool>        => "isFlagged",
    is_unread:         Presence<bool>        => "isUnread",
    is_answered:       Presence<bool>        => "isAnswered",
    is_draft:          Presence<bool>        => "isDraft",
    has_attachment:    Presence<bool>        => "hasAttachment",
    text:              Presence<String>      => "text",
    from:              Presence<String>      => "from",
    to:                Presence<String>      => "to",
    cc:                Presence<String>      => "cc",
    bcc:               Presence<String>      => "bcc",
    subject:           Presence<String>      => "subject",
    body:              Presence<String>      => "body",
    header:            Presence<Vec<String>> => "header"
);

#[derive(Clone, PartialEq, Debug)]
pub enum Filter {
    Operator(FilterOperator),
    Condition(FilterCondition),
}

impl ToJson for Filter {
    fn to_json(&self) -> Json {
        match *self {
            Filter::Operator(ref o)  => o.to_json(),
            Filter::Condition(ref c) => c.to_json(),
        }
    }
}

impl FromJson for Filter {
    fn from_json(json: &Json) -> Result<Filter,ParseError> {
        match *json {
            Json::Object(ref o) => {
                match o.get("operator") {
                    Some(_) => Ok(Filter::Operator(try!(FilterOperator::from_json(json)))),
                    None    => Ok(Filter::Condition(try!(FilterCondition::from_json(json)))),
                }
            },
            _ => Err(ParseError::InvalidJsonType("Filter".to_string())),
        }
    }
}

make_prop_type!(RemovedItem, "RemovedItem",
    message_id: String => "messageId",
    thread_id:  String => "threadId"
);

make_prop_type!(AddedItem, "AddedItem",
    message_id: String => "messageId",
    thread_id:  String => "threadId",
    index:      u64    => "index"
);

make_method_args_type!(GetMessageListRequestArgs, "GetMessageListRequestArgs",
    account_id:               Presence<String>      => "accountId",
    //filter:
    sort:                     Presence<Vec<String>> => "sort",
    collapse_threads:         Presence<bool>        => "collapseThreads",
    position:                 Presence<u64>         => "position",
    anchor:                   Presence<String>      => "anchor",
    anchor_offset:            Presence<i64>         => "anchorOffset",
    limit:                    Presence<u64>         => "limit",
    fetch_threads:            Presence<bool>        => "fetchThreads",
    fetch_messages:           Presence<bool>        => "fetchMessages",
    fetch_message_properties: Presence<bool>        => "fetchMessageProperties",
    fetch_search_snippets:    Presence<bool>        => "fetchSearchSnippets"
);

make_method_args_type!(GetMessageListResponseArgs, "GetMessageListResponseArgs",
    account_id:            String      => "accountId",
    //filter:
    sort:                  Vec<String> => "sort",
    collapse_threads:      bool        => "collapseThreads",
    state:                 String      => "state",
    can_calculate_updates: bool        => "can_calculate_updates",
    position:              u64         => "position",
    total:                 u64         => "total",
    thread_ids:            Vec<String> => "thread_ids",
    message_ids:           Vec<String> => "message_ids"
);

make_method_args_type!(GetMessageListUpdatesRequestArgs, "GetMessageListRequestArgs",
    account_id:       Presence<String>      => "accountId",
    //filter:
    sort:             Presence<Vec<String>> => "sort",
    collapse_threads: Presence<bool>        => "collapseThreads",
    since_state:      String                => "sinceState",
    upto_message_id:  Presence<String>      => "uptoMessageId",
    max_changes:      Presence<u64>         => "maxChanges"
);

make_method_args_type!(GetMessageListUpdatesResponseArgs, "GetMessageListResponseArgs",
    account_id:       String => "accountId",
    //filter:
    sort:             Vec<String>      => "sort",
    collapse_threads: bool             => "collapseThreads",
    old_state:        String           => "oldState",
    new_state:        String           => "newState",
    upto_message_id:  Presence<String> => "uptoMessageId",
    total:            u64              => "total",
    removed:          Vec<RemovedItem> => "removed",
    added:            Vec<AddedItem>   => "added"
);
