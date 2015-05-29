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
use contact::Contact;
use contactgroup::ContactGroup;
use mailbox::Mailbox;

use self::RequestMethod::*;
use self::ResponseMethod::*;

#[derive(Clone, PartialEq, Debug)]
pub struct GetRequestArgs<R> where R: Record {
    pub ids:         Presence<Vec<String>>,
    pub properties:  Presence<Vec<String>>,
    pub since_state: Presence<String>,
    pub _marker:     PhantomData<R>,
}

impl<R: Record> Default for GetRequestArgs<R> {
    fn default() -> GetRequestArgs<R> {
        GetRequestArgs::<R> {
            ids:         Absent,
            properties:  Absent,
            since_state: Absent,
            _marker:     PhantomData,
        }
    }
}

impl<R: Record> ToJson for GetRequestArgs<R> {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::<String,Json>::new();
        self.ids.to_json_field(&mut d, "ids");
        self.properties.to_json_field(&mut d, "properties");
        self.since_state.to_json_field(&mut d, "sinceState");
        Json::Object(d)
    }
}

impl<R: Record> FromJson for GetRequestArgs<R> {
    fn from_json(json: &Json) -> Result<GetRequestArgs<R>,ParseError> {
        match *json {
            Json::Object(ref o) => {
                let mut args = GetRequestArgs::<R>::default();
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
pub struct GetResponseArgs<R> where R: Record {
    pub state:     String,
    pub list:      Option<Vec<R::Partial>>,
    pub not_found: Option<Vec<String>>,
}

impl<R: Record> Default for GetResponseArgs<R> {
    fn default() -> GetResponseArgs<R> {
        GetResponseArgs::<R> {
            state:     "".to_string(),
            list:      None,
            not_found: None,
        }
    }
}

impl<R: Record> ToJson for GetResponseArgs<R> {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::<String,Json>::new();
        self.state.to_json_field(&mut d, "state");
        self.list.to_json_field(&mut d, "list");
        self.not_found.to_json_field(&mut d, "notFound");
        Json::Object(d)
    }
}

impl<R: Record> FromJson for GetResponseArgs<R> {
    fn from_json(json: &Json) -> Result<GetResponseArgs<R>,ParseError> {
        match *json {
            Json::Object(ref o) => {
                let mut args = GetResponseArgs::<R>::default();
                args.state       = try!(FromJsonField::from_json_field(o, "state"));
                args.list        = try!(FromJsonField::from_json_field(o, "list"));
                args.not_found   = try!(FromJsonField::from_json_field(o, "notFound"));
                Ok(args)
            },
            _ => Err(ParseError::InvalidJsonType("GetResponseArgs".to_string())),
        }
    }
}


#[derive(Clone, PartialEq, Debug)]
pub struct GetUpdatesRequestArgs<R> where R: Record {
    pub since_state:             String,
    pub max_changes:             Presence<u64>,
    pub fetch_records:           Presence<bool>,
    pub fetch_record_properties: Presence<Vec<String>>,
    pub _marker:                 PhantomData<R>,
}

impl<R: Record> Default for GetUpdatesRequestArgs<R> {
    fn default() -> GetUpdatesRequestArgs<R> {
        GetUpdatesRequestArgs::<R> {
            since_state:             "".to_string(),
            max_changes:             Absent,
            fetch_records:           Absent,
            fetch_record_properties: Absent,
            _marker:                 PhantomData,
        }
    }
}

impl<R: Record> ToJson for GetUpdatesRequestArgs<R> {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::<String,Json>::new();
        self.since_state.to_json_field(&mut d, "sinceState");
        self.max_changes.to_json_field(&mut d, "maxChanges");
        self.fetch_records.to_json_field(&mut d, "fetchRecords");
        self.fetch_record_properties.to_json_field(&mut d, "fetchRecordProperties");
        Json::Object(d)
    }
}

impl<R: Record> FromJson for GetUpdatesRequestArgs<R> {
    fn from_json(json: &Json) -> Result<GetUpdatesRequestArgs<R>,ParseError> {
        match *json {
            Json::Object(ref o) => {
                let mut args = GetUpdatesRequestArgs::<R>::default();
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
pub struct GetUpdatesResponseArgs<R> where R: Record {
    pub old_state: String,
    pub new_state: String,
    pub changed:   Vec<String>,
    pub removed:   Vec<String>,
    pub _marker:   PhantomData<R>,
}

impl<R: Record> Default for GetUpdatesResponseArgs<R> {
    fn default() -> GetUpdatesResponseArgs<R> {
        GetUpdatesResponseArgs::<R> {
            old_state: "".to_string(),
            new_state: "".to_string(),
            changed:   vec!(),
            removed:   vec!(),
            _marker:   PhantomData,
        }
    }
}

impl<R: Record> ToJson for GetUpdatesResponseArgs<R> {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::<String,Json>::new();
        self.old_state.to_json_field(&mut d, "oldState");
        self.new_state.to_json_field(&mut d, "newState");
        self.changed.to_json_field(&mut d, "changed");
        self.removed.to_json_field(&mut d, "removed");
        Json::Object(d)
    }
}

impl<R: Record> FromJson for GetUpdatesResponseArgs<R> {
    fn from_json(json: &Json) -> Result<GetUpdatesResponseArgs<R>,ParseError> {
        match *json {
            Json::Object(ref o) => {
                let mut args = GetUpdatesResponseArgs::<R>::default();
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
pub struct SetRequestArgs<R> where R: Record {
    pub if_in_state: Presence<String>,
    pub create:      Presence<BTreeMap<String,R::Partial>>,
    pub update:      Presence<BTreeMap<String,R::Partial>>,
    pub destroy:     Presence<Vec<String>>,
}

impl<R: Record> Default for SetRequestArgs<R> {
    fn default() -> SetRequestArgs<R> {
        SetRequestArgs {
            if_in_state: Absent,
            create:      Absent,
            update:      Absent,
            destroy:     Absent,
        }
    }
}

impl<R: Record> ToJson for SetRequestArgs<R> {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::<String,Json>::new();
        self.if_in_state.to_json_field(&mut d, "ifInState");
        self.create.to_json_field(&mut d, "create");
        self.update.to_json_field(&mut d, "update");
        self.destroy.to_json_field(&mut d, "destroy");
        Json::Object(d)
    }
}

impl<R: Record> FromJson for SetRequestArgs<R> {
    fn from_json(json: &Json) -> Result<SetRequestArgs<R>,ParseError> {
        match *json {
            Json::Object(ref o) => {
                let mut args = SetRequestArgs::<R>::default();
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
pub struct SetResponseArgs<R> where R: Record {
    pub old_state:     Option<String>,
    pub new_state:     String,
    pub created:       BTreeMap<String,R::Partial>,
    pub updated:       Vec<String>,
    pub destroyed:     Vec<String>,
    pub not_created:   BTreeMap<String,SetError>,
    pub not_updated:   BTreeMap<String,SetError>,
    pub not_destroyed: BTreeMap<String,SetError>,
}

impl<R: Record> Default for SetResponseArgs<R> {
    fn default() -> SetResponseArgs<R> {
        SetResponseArgs::<R> {
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

impl<R: Record> ToJson for SetResponseArgs<R> {
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

impl<R: Record> FromJson for SetResponseArgs<R> {
    fn from_json(json: &Json) -> Result<SetResponseArgs<R>,ParseError> {
        match *json {
            Json::Object(ref o) => {
                let mut args = SetResponseArgs::<R>::default();
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
    pub typ:         String,
    pub description: Option<String>,
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


macro_rules! make_methods {
    ($set: ident, $setname: expr, $error: ident, $($method: ident, $args: ty => $methodname: expr),*) => {
        #[derive(Clone, PartialEq, Debug)]
        pub enum $set {
            $($method($args, String),)*
        }

        impl ToJson for $set {
            fn to_json(&self) -> Json {
                Json::Array(
                    match *self {
                        $($method(ref args, ref client_id) =>
                            vec!($methodname.to_json(), args.to_json(), client_id.to_json()),)*
                    }
                )
            }
        }

        impl FromJson for $set {
            fn from_json(json: &Json) -> Result<$set,ParseError> {
                match *json {
                    Json::Array(ref a) => {
                        if let false = a.len() == 3 {
                            return Err(ParseError::InvalidStructure($setname.to_string()));
                        }
                        let method = try!(String::from_json(&a[0]));
                        let client_id = try!(String::from_json(&a[2]));
                        match method.as_ref() {
                            $($methodname => Ok($method(try!(<$args>::from_json(&a[1])), client_id)),)*
                            _ => Ok($error(MethodError::UnknownMethod(Present(ErrorDescription(method))), client_id)),
                        }
                    },
                    _ => Err(ParseError::InvalidJsonType($setname.to_string())),
                }
            }
        }

        impl ClientId for $set {
            fn client_id(&self) -> String {
                match *self {
                    $($method(_, ref id) => id,)*
                }.clone()
            }
        }
    }
}

make_methods!(RequestMethod, "RequestMethod", RequestError,
    GetCalendars,           GetRequestArgs<Calendar>            => "getCalendars",
    GetCalendarUpdates,     GetUpdatesRequestArgs<Calendar>     => "getCalendarUpdates",
    SetCalendars,           SetRequestArgs<Calendar>            => "setCalendars",

    GetContacts,            GetRequestArgs<Contact>             => "getContacts",
    GetContactUpdates,      GetUpdatesRequestArgs<Contact>      => "getContactUpdates",
    SetContacts,            SetRequestArgs<Contact>             => "setContacts",

    GetContactGroups,       GetRequestArgs<ContactGroup>        => "getContactGroups",
    GetContactGroupUpdates, GetUpdatesRequestArgs<ContactGroup> => "getContactGroupUpdates",
    SetContactGroups,       SetRequestArgs<ContactGroup>        => "setContactGroups",

    GetMailboxes,           GetRequestArgs<Mailbox>             => "getMailboxes",
    GetMailboxUpdates,      GetUpdatesRequestArgs<Mailbox>      => "getMailboxUpdates",
    SetMailboxes,           SetRequestArgs<Mailbox>             => "setMailboxes",

    RequestError,           MethodError                         => "error"
);

make_methods!(ResponseMethod, "ResponseMethod", ResponseError,
    Calendars,           GetResponseArgs<Calendar>           => "calendars",
    CalendarUpdates,     GetUpdatesResponseArgs<Calendar>    => "calendarUpdates",
    CalendarsSet,        SetResponseArgs<Calendar>           => "calendersSet",

    Contacts,            GetResponseArgs<Contact>             => "contacts",
    ContactUpdates,      GetUpdatesResponseArgs<Contact>      => "contactUpdates",
    ContactsSet,         SetResponseArgs<Contact>             => "contactsSet",

    ContactGroups,       GetResponseArgs<ContactGroup>        => "contactGroups",
    ContactGroupUpdates, GetUpdatesResponseArgs<ContactGroup> => "contactGroupUpdates",
    ContactGroupsSet,    SetResponseArgs<ContactGroup>        => "contactGroupsSet",

    Mailboxes,           GetResponseArgs<Mailbox>             => "mailboxes",
    MailboxUpdates,      GetUpdatesResponseArgs<Mailbox>      => "mailboxUpdates",
    MailboxesSet,        SetResponseArgs<Mailbox>             => "mailboxesSet",

    ResponseError,       MethodError                          => "error"
);


macro_rules! make_batch {
    ($batch: ident, $method: ty) => {
        #[derive(Clone, PartialEq, Debug)]
        pub struct $batch(pub Vec<$method>);

        impl Default for $batch {
            fn default() -> $batch {
                $batch(vec!())
            }
        }

        impl ToJson for $batch {
            fn to_json(&self) -> Json {
                self.0.to_json()
            }
        }

        impl FromJson for $batch {
            fn from_json(json: &Json) -> Result<$batch,ParseError> {
                match Vec::<$method>::from_json(json) {
                    Ok(v) => Ok($batch(v)),
                    Err(e) => Err(e),
                }
            }
        }
    }
}

make_batch!(RequestBatch,  RequestMethod);
make_batch!(ResponseBatch, ResponseMethod);
