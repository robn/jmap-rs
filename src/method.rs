use std::collections::BTreeMap;
use std::default::Default;
use std::error::Error;
use std::marker::PhantomData;
use std::fmt;
use rustc_serialize::json::{Json,ToJson};

use parse::*;
use parse::Presence::*;
use record::Record;

use calendar::Calendar;
use calendar_event::CalendarEvent;
use contact::Contact;
use contact_group::ContactGroup;
use mailbox::Mailbox;

use self::RequestMethod::*;
use self::ResponseMethod::*;


make_method_args_type!(GetRequestArgs, "GetRequestArgs",
    ids:         Presence<Vec<String>> => "ids",
    properties:  Presence<Vec<String>> => "properties",
    since_state: Presence<String>      => "sinceState"
);

make_method_args_type!(GetResponseArgs, "GetResponseArgs",
    state:     String                  => "state",
    list:      Option<Vec<R::Partial>> => "list",
    not_found: Option<Vec<String>>     => "notFound"
);

make_method_args_type!(GetUpdatesRequestArgs, "GetUpdatesRequestArgs",
    since_state:             String                => "sinceState",
    max_changes:             Presence<u64>         => "maxChanges",
    fetch_records:           Presence<bool>        => "fetchRecords",
    fetch_record_properties: Presence<Vec<String>> => "fetchRecordProperties"
);

make_method_args_type!(GetUpdatesResponseArgs, "GetUpdatesResponseArgs",
    old_state: String      => "oldState",
    new_state: String      => "newState",
    changed:   Vec<String> => "changed",
    removed:   Vec<String> => "removed"
);

make_method_args_type!(SetRequestArgs, "SetRequestArgs",
    if_in_state: Presence<String>                      => "ifInState",
    create:      Presence<BTreeMap<String,R::Partial>> => "create",
    update:      Presence<BTreeMap<String,R::Partial>> => "update",
    destroy:     Presence<Vec<String>>                 => "destroy"
);

make_method_args_type!(SetResponseArgs, "SetResponseArgs",
    old_state:     Option<String>              => "oldState",
    new_state:     String                      => "newState",
    created:       BTreeMap<String,R::Partial> => "created",
    updated:       Vec<String>                 => "updated",
    destroyed:     Vec<String>                 => "destroyed",
    not_created:   BTreeMap<String,SetError>   => "notCreated",
    not_updated:   BTreeMap<String,SetError>   => "notUpdated",
    not_destroyed: BTreeMap<String,SetError>   => "notDestroyed"
);


make_prop_type!(SetError, "SetError",
    typ:         String         => "type",
    description: Option<String> => "description"
);


#[derive(Clone, PartialEq, Debug)]
pub struct ErrorDescription(pub String);

impl ToJson for ErrorDescription {
    fn to_json(&self) -> Json {
        self.0.to_json()
    }
}

impl FromJson for ErrorDescription {
    fn from_json(json: &Json) -> Result<ErrorDescription,ParseError> {
        match String::from_json(json) {
            Ok(v) => Ok(ErrorDescription(v)),
            Err(e) => Err(e),
        }
    }
}


#[derive(Clone, PartialEq, Debug)]
pub enum MethodError {
    UnknownMethod(Presence<ErrorDescription>),
    InvalidArguments(Presence<ErrorDescription>),
    TooManyChanges,
    CannotCalculateChanges,
    StateMismatch,
    AccountNotFound,
    AccountNoMail,
    AccountReadOnly,
    AnchorNotFound,
    NotFound,
    InvalidMailboxes,
    MaxQuotaReached,
    FromAccountNotFound,
    ToAccountNotFound,
    FromAccountNoMail,
    ToAccountNoMail,
    AccountNoContacts,
    AccountNoCalendars,
    UnsupportedSort,
    InternalError(Presence<ErrorDescription>), // XXX not in spec
}

impl Error for MethodError {
    fn description(&self) -> &str {
        match *self {
            MethodError::UnknownMethod(_)       => "unknown method",
            MethodError::InvalidArguments(_)    => "invalid arguments for method",
            MethodError::TooManyChanges         => "number of available changes is higher than requested max",
            MethodError::CannotCalculateChanges => "can't calculate changes from supplied state",
            MethodError::StateMismatch          => "supplied state does not match current state",
            MethodError::AccountNotFound        => "account not found",
            MethodError::AccountNoMail          => "account does not contain any mail data",
            MethodError::AccountReadOnly        => "account is read-only",
            MethodError::AnchorNotFound         => "requested anchor not found in message list",
            MethodError::NotFound               => "requested file not found",
            MethodError::InvalidMailboxes       => "mailbox not found or invalid mailbox combination",
            MethodError::MaxQuotaReached        => "max quota reached",
            MethodError::FromAccountNotFound    => "from account not found",
            MethodError::ToAccountNotFound      => "to account not found",
            MethodError::FromAccountNoMail      => "from account does not contain any mail data",
            MethodError::ToAccountNoMail        => "to account does not contain any mail data",
            MethodError::AccountNoContacts      => "account does not contain any contact data",
            MethodError::AccountNoCalendars     => "account does not contain any calendar data",
            MethodError::UnsupportedSort        => "unable to sort on requested properties",
            MethodError::InternalError(_)       => "internal error",
        }
    }
}

impl fmt::Display for MethodError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            MethodError::UnknownMethod(Present(ref d)) => format!("unknown method ({})", d.0),
            MethodError::InvalidArguments(Present(ref d)) => format!("invalid arguments for method ({})", d.0),
            MethodError::InternalError(Present(ref d)) => format!("internal error ({})", d.0),
            ref e => e.description().to_string(),
        })
    }
}

impl ToJson for MethodError {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::<String,Json>::new();

        match *self {
            MethodError::UnknownMethod(_)       => "unknownMethod",
            MethodError::InvalidArguments(_)    => "invalidArguments",
            MethodError::TooManyChanges         => "tooManyChanges",
            MethodError::CannotCalculateChanges => "cannotCalculateChanges",
            MethodError::StateMismatch          => "stateMismatch",
            MethodError::AccountNotFound        => "accountNotFound",
            MethodError::AccountNoMail          => "accountNoMail",
            MethodError::AccountReadOnly        => "accountReadOnly",
            MethodError::AnchorNotFound         => "anchorNotFound",
            MethodError::NotFound               => "notFound",
            MethodError::InvalidMailboxes       => "invalidMailboxes",
            MethodError::MaxQuotaReached        => "maxQuotaReached",
            MethodError::FromAccountNotFound    => "fromAccountNotFound",
            MethodError::ToAccountNotFound      => "toAccountNotFound",
            MethodError::FromAccountNoMail      => "fromAccountNoMail",
            MethodError::ToAccountNoMail        => "toAccountNoMail",
            MethodError::AccountNoContacts      => "accountNoContacts",
            MethodError::AccountNoCalendars     => "accountNoCalendars",
            MethodError::UnsupportedSort        => "unsupportedSort",
            MethodError::InternalError(_)       => "internalError",
        }.to_string().to_json_field(&mut d, "type");

        match *self {
            MethodError::UnknownMethod(ref desc)    |
            MethodError::InvalidArguments(ref desc) |
            MethodError::InternalError(ref desc) =>
                desc.to_json_field(&mut d, "description"),
            _ => (),
        };

        Json::Object(d)
    }
}

impl FromJson for MethodError {
    fn from_json(json: &Json) -> Result<MethodError,ParseError> {
        match *json {
            Json::Object(ref o) => {
                let typ: String = try!(FromJsonField::from_json_field(o, "type"));
                match typ.as_ref() {
                    "unknownMethod"          => Ok(MethodError::UnknownMethod(try!(FromJsonField::from_json_field(o, "description")))),
                    "invalidArguments"       => Ok(MethodError::InvalidArguments(try!(FromJsonField::from_json_field(o, "description")))),
                    "tooManyChanges"         => Ok(MethodError::TooManyChanges),
                    "cannotCalculateChanges" => Ok(MethodError::CannotCalculateChanges),
                    "stateMismatch"          => Ok(MethodError::StateMismatch),
                    "accountNotFound"        => Ok(MethodError::AccountNotFound),
                    "accountNoMail"          => Ok(MethodError::AccountNoMail),
                    "accountReadOnly"        => Ok(MethodError::AccountReadOnly),
                    "anchorNotFound"         => Ok(MethodError::AnchorNotFound),
                    "notFound"               => Ok(MethodError::NotFound),
                    "invalidMailboxes"       => Ok(MethodError::InvalidMailboxes),
                    "maxQuotaReached"        => Ok(MethodError::MaxQuotaReached),
                    "fromAccountNotFound"    => Ok(MethodError::FromAccountNotFound),
                    "toAccountNotFound"      => Ok(MethodError::ToAccountNotFound),
                    "fromAccountNoMail"      => Ok(MethodError::FromAccountNoMail),
                    "toAccountNoMail"        => Ok(MethodError::ToAccountNoMail),
                    "accountNoContacts"      => Ok(MethodError::AccountNoContacts),
                    "accountNoCalendars"     => Ok(MethodError::AccountNoCalendars),
                    "unsupportedSort"        => Ok(MethodError::UnsupportedSort),
                    "internalError"          => Ok(MethodError::InternalError(try!(FromJsonField::from_json_field(o, "description")))),

                    _                        => Err(ParseError::InvalidStructure("MethodError".to_string())),
                }
            },
            _ => Err(ParseError::InvalidJsonType("MethodError".to_string())),
        }
    }
}


pub trait ClientId {
    fn client_id(&self) -> String;
}


make_methods!(RequestMethod, "RequestMethod", RequestError,
    GetCalendars,            GetRequestArgs<Calendar>             => "getCalendars",
    GetCalendarUpdates,      GetUpdatesRequestArgs<Calendar>      => "getCalendarUpdates",
    SetCalendars,            SetRequestArgs<Calendar>             => "setCalendars",

    GetCalendarEvents,       GetRequestArgs<CalendarEvent>        => "getCalendarEvents",
    GetCalendarEventUpdates, GetUpdatesRequestArgs<CalendarEvent> => "getCalendarEventUpdates",
    SetCalendarEvents,       SetRequestArgs<CalendarEvent>        => "setCalendarEvents",

    GetContacts,             GetRequestArgs<Contact>              => "getContacts",
    GetContactUpdates,       GetUpdatesRequestArgs<Contact>       => "getContactUpdates",
    SetContacts,             SetRequestArgs<Contact>              => "setContacts",

    GetContactGroups,        GetRequestArgs<ContactGroup>         => "getContactGroups",
    GetContactGroupUpdates,  GetUpdatesRequestArgs<ContactGroup>  => "getContactGroupUpdates",
    SetContactGroups,        SetRequestArgs<ContactGroup>         => "setContactGroups",

    GetMailboxes,            GetRequestArgs<Mailbox>              => "getMailboxes",
    GetMailboxUpdates,       GetUpdatesRequestArgs<Mailbox>       => "getMailboxUpdates",
    SetMailboxes,            SetRequestArgs<Mailbox>              => "setMailboxes",

    RequestError,            MethodError                          => "error"
);

make_methods!(ResponseMethod, "ResponseMethod", ResponseError,
    Calendars,            GetResponseArgs<Calendar>             => "calendars",
    CalendarUpdates,      GetUpdatesResponseArgs<Calendar>      => "calendarUpdates",
    CalendarsSet,         SetResponseArgs<Calendar>             => "calendarsSet",

    CalendarEvents,       GetResponseArgs<CalendarEvent>        => "calendarEvents",
    CalendarEventUpdates, GetUpdatesResponseArgs<CalendarEvent> => "calendarEventUpdates",
    CalendarEventsSet,    SetResponseArgs<CalendarEvent>        => "calendarEventsSet",

    Contacts,             GetResponseArgs<Contact>              => "contacts",
    ContactUpdates,       GetUpdatesResponseArgs<Contact>       => "contactUpdates",
    ContactsSet,          SetResponseArgs<Contact>              => "contactsSet",

    ContactGroups,        GetResponseArgs<ContactGroup>         => "contactGroups",
    ContactGroupUpdates,  GetUpdatesResponseArgs<ContactGroup>  => "contactGroupUpdates",
    ContactGroupsSet,     SetResponseArgs<ContactGroup>         => "contactGroupsSet",

    Mailboxes,            GetResponseArgs<Mailbox>              => "mailboxes",
    MailboxUpdates,       GetUpdatesResponseArgs<Mailbox>       => "mailboxUpdates",
    MailboxesSet,         SetResponseArgs<Mailbox>              => "mailboxesSet",

    ResponseError,        MethodError                           => "error"
);


make_batch!(RequestBatch,  RequestMethod);
make_batch!(ResponseBatch, ResponseMethod);
