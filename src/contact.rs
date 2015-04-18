use std::collections::BTreeMap;
use std::string::ToString;
use std::default::Default;
use rustc_serialize::json::{Json,ToJson};

use jmaputil::{FromJson,ParseError,from_json_field,from_json_field_opt,to_json_field,to_json_field_opt};


// basic three-part Date type, YYYY-MM-DD
// individual components may be 0, which will present as None
// only minimal range sanity checks are done
#[derive(Clone, PartialEq, Debug)]
pub struct Date {
    y: Option<u16>,
    m: Option<u8>,
    d: Option<u8>,
}

impl Default for Date {
    fn default() -> Date {
        Date { y: None, m: None, d: None }
    }
}

impl ToString for Date {
    fn to_string(&self) -> String {
        format!("{:04}-{:02}-{:02}",
            match self.y {
                Some(n) => n,
                None    => 0,
            },
            match self.m {
                Some(n) => n,
                None    => 0,
            },
            match self.d {
                Some(n) => n,
                None    => 0,
            },
        )
    }
}

impl ToJson for Date {
    fn to_json(&self) -> Json {
        Json::String(self.to_string())
    }
}

impl FromJson for Date {
    fn from_json(json: &Json) -> Result<Date,ParseError> {
        match *json {
            Json::String(ref v) => {
                let (ok, err): (Vec<Result<u16,_>>,Vec<_>) = v.split('-').map(|ref s| s.parse::<u16>()).partition(|ref r| match **r { Ok(_) => true, Err(_) => false });
                if let false = err.is_empty() {
                    return Err(ParseError::InvalidStructure("Date".to_string()));
                }

                let dv: Vec<Option<u16>> = ok.into_iter().map(|n| match n.ok().unwrap() {
                    0  => None,
                    nn => Some(nn),
                }).collect();

                match dv.as_slice() {
                    [y,m,d] => {
                        Ok(Date {
                            y: y,
                            m: try!(match m {
                                Some(n) if n > 12 => Err(ParseError::InvalidStructure("Date".to_string())),
                                Some(n) => Ok(Some(n as u8)),
                                None => Ok(None),
                            }),
                            d: try!(match d {
                                Some(n) if n > 31 => Err(ParseError::InvalidStructure("Date".to_string())),
                                Some(n) => Ok(Some(n as u8)),
                                None => Ok(None),
                            }),
                        })
                    }
                    _ => Err(ParseError::InvalidStructure("Date".to_string())),
                }
            },
            _ => Err(ParseError::InvalidJsonType("Date".to_string())),
        }
    }
}


// marker trait for contact types
pub trait ContactType: ToString + ToJson + FromJson + Default { }


#[derive(Copy, Clone, PartialEq, Debug)]
pub enum EmailType {
    Personal,
    Work,
    Other,
}

impl ContactType for EmailType { }

impl Default for EmailType {
    fn default() -> EmailType { EmailType::Other }
}

impl ToString for EmailType {
    fn to_string(&self) -> String {
        match *self {
            EmailType::Personal => "personal",
            EmailType::Work     => "work",
            EmailType::Other    => "other",
        }.to_string()
    }
}

impl ToJson for EmailType {
    fn to_json(&self) -> Json {
        Json::String(self.to_string())
    }
}

impl FromJson for EmailType {
    fn from_json(json: &Json) -> Result<EmailType,ParseError> {
        match *json {
            Json::String(ref v) => match v.as_str() {
                "personal" => Ok(EmailType::Personal),
                "work"     => Ok(EmailType::Work),
                "other"    => Ok(EmailType::Other),
                _          => Err(ParseError::InvalidStructure("EmailType".to_string())),
            },
            _ => Err(ParseError::InvalidJsonType("EmailType".to_string())),
        }
    }
}


#[derive(Copy, Clone, PartialEq, Debug)]
pub enum PhoneType {
    Home,
    Work,
    Mobile,
    Fax,
    Pager,
    Other,
}

impl ContactType for PhoneType { }

impl Default for PhoneType {
    fn default() -> PhoneType { PhoneType::Other }
}

impl ToString for PhoneType {
    fn to_string(&self) -> String {
        match *self {
            PhoneType::Home   => "home",
            PhoneType::Work   => "work",
            PhoneType::Mobile => "mobile",
            PhoneType::Fax    => "fax",
            PhoneType::Pager  => "pager",
            PhoneType::Other  => "other",
        }.to_string()
    }
}

impl ToJson for PhoneType {
    fn to_json(&self) -> Json {
        Json::String(self.to_string())
    }
}

impl FromJson for PhoneType {
    fn from_json(json: &Json) -> Result<PhoneType,ParseError> {
        match *json {
            Json::String(ref v) => match v.as_str() {
                "home"   => Ok(PhoneType::Home),
                "work"   => Ok(PhoneType::Work),
                "mobile" => Ok(PhoneType::Mobile),
                "fax"    => Ok(PhoneType::Fax),
                "pager"  => Ok(PhoneType::Pager),
                "other"  => Ok(PhoneType::Other),
                _        => Err(ParseError::InvalidStructure("PhoneType".to_string())),
            },
            _ => Err(ParseError::InvalidJsonType("PhoneType".to_string())),
        }
    }
}


#[derive(Copy, Clone, PartialEq, Debug)]
pub enum OnlineType {
    Uri,
    Username,
    Other,
}

impl ContactType for OnlineType { }

impl Default for OnlineType {
    fn default() -> OnlineType { OnlineType::Other }
}

impl ToString for OnlineType {
    fn to_string(&self) -> String {
        match *self {
            OnlineType::Uri      => "uri",
            OnlineType::Username => "username",
            OnlineType::Other    => "other",
        }.to_string()
    }
}

impl ToJson for OnlineType {
    fn to_json(&self) -> Json {
        Json::String(self.to_string())
    }
}

impl FromJson for OnlineType {
    fn from_json(json: &Json) -> Result<OnlineType,ParseError> {
        match *json {
            Json::String(ref v) => match v.as_str() {
                "uri"      => Ok(OnlineType::Uri),
                "username" => Ok(OnlineType::Username),
                "other"    => Ok(OnlineType::Other),
                _          => Err(ParseError::InvalidStructure("OnlineType".to_string())),
            },
            _ => Err(ParseError::InvalidJsonType("OnlineType".to_string())),
        }
    }
}


#[derive(Copy, Clone, PartialEq, Debug)]
pub enum AddressType {
    Home,
    Work,
    Billing,
    Postal,
    Other,
}

impl ContactType for AddressType { }

impl Default for AddressType {
    fn default() -> AddressType { AddressType::Other }
}

impl ToString for AddressType {
    fn to_string(&self) -> String {
        match *self {
            AddressType::Home    => "home",
            AddressType::Work    => "work",
            AddressType::Billing => "billing",
            AddressType::Postal  => "postal",
            AddressType::Other   => "other",
        }.to_string()
    }
}

impl ToJson for AddressType {
    fn to_json(&self) -> Json {
        Json::String(self.to_string())
    }
}

impl FromJson for AddressType {
    fn from_json(json: &Json) -> Result<AddressType,ParseError> {
        match *json {
            Json::String(ref v) => match v.as_str() {
                "home"    => Ok(AddressType::Home),
                "work"    => Ok(AddressType::Work),
                "billing" => Ok(AddressType::Billing),
                "postal"  => Ok(AddressType::Postal),
                "other"   => Ok(AddressType::Other),
                _         => Err(ParseError::InvalidStructure("AddressType".to_string())),
            },
            _ => Err(ParseError::InvalidJsonType("AddressType".to_string())),
        }
    }
}


// ContactInformation is a type, a value and an optional label
#[derive(Clone, PartialEq, Debug)]
pub struct ContactInformation<T: ContactType> {
    typ:   T,
    value: String,
    label: Option<String>,
}

impl<T> Default for ContactInformation<T> where T: ContactType {
    fn default() -> ContactInformation<T> {
        ContactInformation { typ: T::default(), value: "".to_string(), label: None }
    }
}

impl<T> ToJson for ContactInformation<T> where T: ContactType {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::<String,Json>::new();
        to_json_field(&mut d, "type", &self.typ);
        to_json_field(&mut d, "value", &self.value);
        to_json_field_opt(&mut d, "label", &self.label);
        Json::Object(d)
    }
}

impl<T> FromJson for ContactInformation<T> where T: ContactType {
    fn from_json(json: &Json) -> Result<ContactInformation<T>,ParseError> {
        match *json {
            Json::Object(ref o) => {
                let mut ci = ContactInformation::<T>::default();
                ci.typ   = try!(from_json_field(o, "type"));
                ci.value = try!(from_json_field(o, "value"));
                ci.label = try!(from_json_field_opt(o, "label"));
                Ok(ci)
            },
            _ => Err(ParseError::InvalidJsonType("ContactInformation".to_string())),
        }
    }
}


// Address is much like ContactInformation, but with multiple values
#[derive(Clone, PartialEq, Debug)]
pub struct Address {
    typ:      AddressType,
    label:    Option<String>,
    street:   String,
    locality: String,
    region:   String,
    postcode: String,
    country:  String,
}

impl Default for Address {
    fn default() -> Address {
        Address {
            typ:      AddressType::Other,
            label:    None,
            street:   "".to_string(),
            locality: "".to_string(),
            region:   "".to_string(),
            postcode: "".to_string(),
            country:  "".to_string(),
        }
    }
}

impl ToJson for Address {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::<String,Json>::new();
        to_json_field(&mut d, "type", &self.typ);
        to_json_field(&mut d, "street", &self.street);
        to_json_field(&mut d, "locality", &self.locality);
        to_json_field(&mut d, "region", &self.region);
        to_json_field(&mut d, "postcode", &self.postcode);
        to_json_field(&mut d, "country", &self.country);
        to_json_field_opt(&mut d, "label", &self.label);
        Json::Object(d)
    }
}

impl FromJson for Address {
    fn from_json(json: &Json) -> Result<Address,ParseError> {
        match *json {
            Json::Object(ref o) => {
                let mut address = Address::default();
                address.typ      = try!(from_json_field(o, "type"));
                address.street   = try!(from_json_field(o, "street"));
                address.locality = try!(from_json_field(o, "locality"));
                address.region   = try!(from_json_field(o, "region"));
                address.postcode = try!(from_json_field(o, "postcode"));
                address.country  = try!(from_json_field(o, "country"));
                address.label    = try!(from_json_field_opt(o, "label"));
                Ok(address)
            },
            _ => Err(ParseError::InvalidJsonType("Address".to_string())),
        }
    }
}


// representation of some file. most fields are optional
// type is a MIME type this time, so a freeform string
#[derive(Clone, PartialEq, Debug)]
pub struct File {
    url:  String,
    typ:  Option<String>,
    name: Option<String>,
    size: Option<u64>,
}

impl Default for File {
    fn default() -> File {
        File {
            url:  "".to_string(),
            typ:  None,
            name: None,
            size: None,
        }
    }
}

impl ToJson for File {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::<String,Json>::new();
        to_json_field(&mut d, "url", &self.url);
        to_json_field_opt(&mut d, "type", &self.typ);
        to_json_field_opt(&mut d, "name", &self.name);
        to_json_field_opt(&mut d, "size", &self.size);
        Json::Object(d)
    }
}

impl FromJson for File {
    fn from_json(json: &Json) -> Result<File,ParseError> {
        match *json {
            Json::Object(ref o) => {
                let mut file = File::default();
                file.url  = try!(from_json_field(o, "url"));
                file.typ  = try!(from_json_field_opt(o, "type"));
                file.name = try!(from_json_field_opt(o, "name"));
                file.size = try!(from_json_field_opt(o, "size"));
                Ok(file)
            }
            _ => Err(ParseError::InvalidJsonType("File".to_string())),
        }
    }
}


// bringing it all together, a contact
#[derive(Clone, PartialEq, Debug)]
pub struct Contact {
    id:                  String,
    is_flagged:          bool,
    avatar:              Option<File>,
    prefix:              String,
    first_name:          String,
    last_name:           String,
    suffix:              String,
    nickname:            String,
    birthday:            Date,
    anniversary:         Date,
    company:             String,
    department:          String,
    job_title:           String,
    emails:              Vec<ContactInformation<EmailType>>,
    default_email_index: u64,
    phones:              Vec<ContactInformation<PhoneType>>,
    online:              Vec<ContactInformation<OnlineType>>,
    addresses:           Vec<Address>,
    notes:               String,
}

impl Default for Contact {
    fn default() -> Contact {
        Contact {
            id:                  "".to_string(),
            is_flagged:          false,
            avatar:              None,
            prefix:              "".to_string(),
            first_name:          "".to_string(),
            last_name:           "".to_string(),
            suffix:              "".to_string(),
            nickname:            "".to_string(),
            birthday:            Date::default(),
            anniversary:         Date::default(),
            company:             "".to_string(),
            department:          "".to_string(),
            job_title:           "".to_string(),
            emails:              vec!(),
            default_email_index: 0,
            phones:              vec!(),
            online:              vec!(),
            addresses:           vec!(),
            notes:               "".to_string(),
        }
    }
}

impl ToJson for Contact {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::<String,Json>::new();
        to_json_field(&mut d, "id", &self.id);
        to_json_field(&mut d, "isFlagged", &self.is_flagged);
        to_json_field_opt(&mut d, "avatar", &self.avatar);
        to_json_field(&mut d, "prefix", &self.prefix);
        to_json_field(&mut d, "firstName", &self.first_name);
        to_json_field(&mut d, "lastName", &self.last_name);
        to_json_field(&mut d, "suffix", &self.suffix);
        to_json_field(&mut d, "nickname", &self.nickname);
        to_json_field(&mut d, "birthday", &self.birthday);
        to_json_field(&mut d, "anniversary", &self.anniversary);
        to_json_field(&mut d, "company", &self.company);
        to_json_field(&mut d, "department", &self.department);
        to_json_field(&mut d, "jobTitle", &self.job_title);
        to_json_field(&mut d, "emails", &self.emails);
        to_json_field(&mut d, "defaultEmailIndex", &self.default_email_index);
        to_json_field(&mut d, "phones", &self.phones);
        to_json_field(&mut d, "online", &self.online);
        to_json_field(&mut d, "addresses", &self.addresses);
        to_json_field(&mut d, "notes", &self.notes);
        Json::Object(d)
    }
}

impl FromJson for Contact {
    fn from_json(json: &Json) -> Result<Contact,ParseError> {
        match *json {
            Json::Object(ref o) => {
                let mut contact = Contact::default();
                contact.id                  = try!(from_json_field(o, "id"));
                contact.is_flagged          = try!(from_json_field(o, "isFlagged"));
                contact.avatar              = try!(from_json_field_opt(o, "avatar"));
                contact.prefix              = try!(from_json_field(o, "prefix"));
                contact.first_name          = try!(from_json_field(o, "firstName"));
                contact.last_name           = try!(from_json_field(o, "lastName"));
                contact.suffix              = try!(from_json_field(o, "suffix"));
                contact.nickname            = try!(from_json_field(o, "nickname"));
                contact.birthday            = try!(from_json_field(o, "birthday"));
                contact.anniversary         = try!(from_json_field(o, "anniversary"));
                contact.company             = try!(from_json_field(o, "company"));
                contact.department          = try!(from_json_field(o, "department"));
                contact.job_title           = try!(from_json_field(o, "jobTitle"));
                contact.emails              = try!(from_json_field(o, "emails"));
                contact.default_email_index = try!(from_json_field(o, "defaultEmailIndex"));
                contact.phones              = try!(from_json_field(o, "phones"));
                contact.online              = try!(from_json_field(o, "online"));
                contact.addresses           = try!(from_json_field(o, "addresses"));
                contact.notes               = try!(from_json_field(o, "notes"));
                Ok(contact)
            }
            _ => Err(ParseError::InvalidJsonType("Contact".to_string())),
        }
    }
}
