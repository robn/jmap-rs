use std::collections::BTreeMap;
use rustc_serialize::json::{Json,ToJson};

use parse::*;

make_method_args_type!(ReportMessagesRequestArgs, "ReportMessagesRequestArgs",
    account_id:  Presence<String> => "accountId",
    message_ids: Vec<String>      => "messageIds",
    as_spam:     bool             => "asSpam"
);

make_method_args_type!(ReportMessagesResponseArgs, "ReportMessagesResponseArgs",
    account_id:  String              => "accountId",
    as_spam:     bool                => "asSpam",
    reported:    Vec<String>         => "reported",
    not_found:   Option<Vec<String>> => "notFound"
);
