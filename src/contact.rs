use std::collections::BTreeMap;
use std::string::ToString;
use std::default::Default;
use rustc_serialize::json::{Json,ToJson};

use parse::*;
use parse::Presence::*;
use record;
use record::{Record, PartialRecord};


// basic three-part Date type, YYYY-MM-DD
// individual components may be 0, which will present as None
// only minimal range sanity checks are done
#[derive(Clone, PartialEq, Debug)]
pub struct Date {
    pub y: Option<u16>,
    pub m: Option<u8>,
    pub d: Option<u8>,
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

                if let false = dv.len() == 3 {
                    return Err(ParseError::InvalidStructure("Date".to_string()));
                }

                Ok(Date {
                    y: dv[0],
                    m: try!(match dv[1] {
                        Some(n) if n > 12 => Err(ParseError::InvalidStructure("Date".to_string())),
                        Some(n) => Ok(Some(n as u8)),
                        None => Ok(None),
                    }),
                    d: try!(match dv[2] {
                        Some(n) if n > 31 => Err(ParseError::InvalidStructure("Date".to_string())),
                        Some(n) => Ok(Some(n as u8)),
                        None => Ok(None),
                    }),
                })
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
            Json::String(ref v) => match v.as_ref() {
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
            Json::String(ref v) => match v.as_ref() {
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
            Json::String(ref v) => match v.as_ref() {
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
            Json::String(ref v) => match v.as_ref() {
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
    pub typ:   T,
    pub value: String,
    pub label: Option<String>,
}

impl<T> Default for ContactInformation<T> where T: ContactType {
    fn default() -> ContactInformation<T> {
        ContactInformation { typ: T::default(), value: "".to_string(), label: None }
    }
}

impl<T> ToJson for ContactInformation<T> where T: ContactType {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::<String,Json>::new();
        self.typ.to_json_field(&mut d, "type");
        self.value.to_json_field(&mut d, "value");
        self.label.to_json_field(&mut d, "label");
        Json::Object(d)
    }
}

impl<T> FromJson for ContactInformation<T> where T: ContactType {
    fn from_json(json: &Json) -> Result<ContactInformation<T>,ParseError> {
        match *json {
            Json::Object(ref o) => {
                let mut ci = ContactInformation::<T>::default();
                ci.typ   = try!(FromJsonField::from_json_field(o, "type"));
                ci.value = try!(FromJsonField::from_json_field(o, "value"));
                ci.label = try!(FromJsonField::from_json_field(o, "label"));
                Ok(ci)
            },
            _ => Err(ParseError::InvalidJsonType("ContactInformation".to_string())),
        }
    }
}


// Address is much like ContactInformation, but with multiple values
#[derive(Clone, PartialEq, Debug)]
pub struct Address {
    pub typ:      AddressType,
    pub label:    Option<String>,
    pub street:   String,
    pub locality: String,
    pub region:   String,
    pub postcode: String,
    pub country:  String,
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
        self.typ.to_json_field(&mut d, "type");
        self.street.to_json_field(&mut d, "street");
        self.locality.to_json_field(&mut d, "locality");
        self.region.to_json_field(&mut d, "region");
        self.postcode.to_json_field(&mut d, "postcode");
        self.country.to_json_field(&mut d, "country");
        self.label.to_json_field(&mut d, "label");
        Json::Object(d)
    }
}

impl FromJson for Address {
    fn from_json(json: &Json) -> Result<Address,ParseError> {
        match *json {
            Json::Object(ref o) => {
                let mut address = Address::default();
                address.typ      = try!(FromJsonField::from_json_field(o, "type"));
                address.street   = try!(FromJsonField::from_json_field(o, "street"));
                address.locality = try!(FromJsonField::from_json_field(o, "locality"));
                address.region   = try!(FromJsonField::from_json_field(o, "region"));
                address.postcode = try!(FromJsonField::from_json_field(o, "postcode"));
                address.country  = try!(FromJsonField::from_json_field(o, "country"));
                address.label    = try!(FromJsonField::from_json_field(o, "label"));
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
    pub url:  String,
    pub typ:  Option<String>,
    pub name: Option<String>,
    pub size: Option<u64>,
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
        self.url.to_json_field(&mut d, "url");
        self.typ.to_json_field(&mut d, "type");
        self.name.to_json_field(&mut d, "name");
        self.size.to_json_field(&mut d, "size");
        Json::Object(d)
    }
}

impl FromJson for File {
    fn from_json(json: &Json) -> Result<File,ParseError> {
        match *json {
            Json::Object(ref o) => {
                let mut file = File::default();
                file.url  = try!(FromJsonField::from_json_field(o, "url"));
                file.typ  = try!(FromJsonField::from_json_field(o, "type"));
                file.name = try!(FromJsonField::from_json_field(o, "name"));
                file.size = try!(FromJsonField::from_json_field(o, "size"));
                Ok(file)
            }
            _ => Err(ParseError::InvalidJsonType("File".to_string())),
        }
    }
}


make_record_type!(Contact, PartialContact, "Contact",
    is_flagged:          bool                                => "isFlagged",
    avatar:              Option<File>                        => "avatar",
    prefix:              String                              => "prefix",
    first_name:          String                              => "firstName",
    last_name:           String                              => "lastName",
    suffix:              String                              => "suffix",
    nickname:            String                              => "nickname",
    birthday:            Date                                => "birthday",
    anniversary:         Date                                => "anniversary",
    company:             String                              => "company",
    department:          String                              => "department",
    job_title:           String                              => "jobTitle",
    emails:              Vec<ContactInformation<EmailType>>  => "emails",
    default_email_index: u64                                 => "defaultEmailIndex",
    phones:              Vec<ContactInformation<PhoneType>>  => "phones",
    online:              Vec<ContactInformation<OnlineType>> => "online",
    addresses:           Vec<Address>                        => "addresses",
    notes:               String                              => "notes"
);
