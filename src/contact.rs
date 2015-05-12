use std::collections::BTreeMap;
use std::string::ToString;
use std::default::Default;
use rustc_serialize::json::{Json,ToJson};

use parse::*;
use parse::Presence::*;

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


// bringing it all together, a contact
#[derive(Clone, PartialEq, Debug)]
pub struct Contact {
    pub id:                  String,
    pub is_flagged:          bool,
    pub avatar:              Option<File>,
    pub prefix:              String,
    pub first_name:          String,
    pub last_name:           String,
    pub suffix:              String,
    pub nickname:            String,
    pub birthday:            Date,
    pub anniversary:         Date,
    pub company:             String,
    pub department:          String,
    pub job_title:           String,
    pub emails:              Vec<ContactInformation<EmailType>>,
    pub default_email_index: u64,
    pub phones:              Vec<ContactInformation<PhoneType>>,
    pub online:              Vec<ContactInformation<OnlineType>>,
    pub addresses:           Vec<Address>,
    pub notes:               String,
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
        self.id.to_json_field(&mut d, "id");
        self.is_flagged.to_json_field(&mut d, "isFlagged");
        self.avatar.to_json_field(&mut d, "avatar");
        self.prefix.to_json_field(&mut d, "prefix");
        self.first_name.to_json_field(&mut d, "firstName");
        self.last_name.to_json_field(&mut d, "lastName");
        self.suffix.to_json_field(&mut d, "suffix");
        self.nickname.to_json_field(&mut d, "nickname");
        self.birthday.to_json_field(&mut d, "birthday");
        self.anniversary.to_json_field(&mut d, "anniversary");
        self.company.to_json_field(&mut d, "company");
        self.department.to_json_field(&mut d, "department");
        self.job_title.to_json_field(&mut d, "jobTitle");
        self.emails.to_json_field(&mut d, "emails");
        self.default_email_index.to_json_field(&mut d, "defaultEmailIndex");
        self.phones.to_json_field(&mut d, "phones");
        self.online.to_json_field(&mut d, "online");
        self.addresses.to_json_field(&mut d, "addresses");
        self.notes.to_json_field(&mut d, "notes");
        Json::Object(d)
    }
}

impl FromJson for Contact {
    fn from_json(json: &Json) -> Result<Contact,ParseError> {
        match *json {
            Json::Object(ref o) => {
                let mut contact = Contact::default();
                contact.id                  = try!(FromJsonField::from_json_field(o, "id"));
                contact.is_flagged          = try!(FromJsonField::from_json_field(o, "isFlagged"));
                contact.avatar              = try!(FromJsonField::from_json_field(o, "avatar"));
                contact.prefix              = try!(FromJsonField::from_json_field(o, "prefix"));
                contact.first_name          = try!(FromJsonField::from_json_field(o, "firstName"));
                contact.last_name           = try!(FromJsonField::from_json_field(o, "lastName"));
                contact.suffix              = try!(FromJsonField::from_json_field(o, "suffix"));
                contact.nickname            = try!(FromJsonField::from_json_field(o, "nickname"));
                contact.birthday            = try!(FromJsonField::from_json_field(o, "birthday"));
                contact.anniversary         = try!(FromJsonField::from_json_field(o, "anniversary"));
                contact.company             = try!(FromJsonField::from_json_field(o, "company"));
                contact.department          = try!(FromJsonField::from_json_field(o, "department"));
                contact.job_title           = try!(FromJsonField::from_json_field(o, "jobTitle"));
                contact.emails              = try!(FromJsonField::from_json_field(o, "emails"));
                contact.default_email_index = try!(FromJsonField::from_json_field(o, "defaultEmailIndex"));
                contact.phones              = try!(FromJsonField::from_json_field(o, "phones"));
                contact.online              = try!(FromJsonField::from_json_field(o, "online"));
                contact.addresses           = try!(FromJsonField::from_json_field(o, "addresses"));
                contact.notes               = try!(FromJsonField::from_json_field(o, "notes"));
                Ok(contact)
            }
            _ => Err(ParseError::InvalidJsonType("Contact".to_string())),
        }
    }
}


#[derive(Clone, PartialEq, Debug)]
pub struct PartialContact {
    pub id:                  Presence<String>,
    pub is_flagged:          Presence<bool>,
    pub avatar:              Presence<Option<File>>,
    pub prefix:              Presence<String>,
    pub first_name:          Presence<String>,
    pub last_name:           Presence<String>,
    pub suffix:              Presence<String>,
    pub nickname:            Presence<String>,
    pub birthday:            Presence<Date>,
    pub anniversary:         Presence<Date>,
    pub company:             Presence<String>,
    pub department:          Presence<String>,
    pub job_title:           Presence<String>,
    pub emails:              Presence<Vec<ContactInformation<EmailType>>>,
    pub default_email_index: Presence<u64>,
    pub phones:              Presence<Vec<ContactInformation<PhoneType>>>,
    pub online:              Presence<Vec<ContactInformation<OnlineType>>>,
    pub addresses:           Presence<Vec<Address>>,
    pub notes:               Presence<String>,
}

impl Default for PartialContact {
    fn default() -> PartialContact {
        PartialContact {
            id:                  Absent,
            is_flagged:          Absent,
            avatar:              Absent,
            prefix:              Absent,
            first_name:          Absent,
            last_name:           Absent,
            suffix:              Absent,
            nickname:            Absent,
            birthday:            Absent,
            anniversary:         Absent,
            company:             Absent,
            department:          Absent,
            job_title:           Absent,
            emails:              Absent,
            default_email_index: Absent,
            phones:              Absent,
            online:              Absent,
            addresses:           Absent,
            notes:               Absent,
        }
    }
}

impl ToJson for PartialContact {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::<String,Json>::new();
        self.id.to_json_field(&mut d, "id");
        self.is_flagged.to_json_field(&mut d, "isFlagged");
        self.avatar.to_json_field(&mut d, "avatar");
        self.prefix.to_json_field(&mut d, "prefix");
        self.first_name.to_json_field(&mut d, "firstName");
        self.last_name.to_json_field(&mut d, "lastName");
        self.suffix.to_json_field(&mut d, "suffix");
        self.nickname.to_json_field(&mut d, "nickname");
        self.birthday.to_json_field(&mut d, "birthday");
        self.anniversary.to_json_field(&mut d, "anniversary");
        self.company.to_json_field(&mut d, "company");
        self.department.to_json_field(&mut d, "department");
        self.job_title.to_json_field(&mut d, "jobTitle");
        self.emails.to_json_field(&mut d, "emails");
        self.default_email_index.to_json_field(&mut d, "defaultEmailIndex");
        self.phones.to_json_field(&mut d, "phones");
        self.online.to_json_field(&mut d, "online");
        self.addresses.to_json_field(&mut d, "addresses");
        self.notes.to_json_field(&mut d, "notes");
        Json::Object(d)
    }
}

impl FromJson for PartialContact {
    fn from_json(json: &Json) -> Result<PartialContact,ParseError> {
        match *json {
            Json::Object(ref o) => {
                let mut contact = PartialContact::default();
                contact.id                  = try!(FromJsonField::from_json_field(o, "id"));
                contact.is_flagged          = try!(FromJsonField::from_json_field(o, "isFlagged"));
                contact.avatar              = try!(FromJsonField::from_json_field(o, "avatar"));
                contact.prefix              = try!(FromJsonField::from_json_field(o, "prefix"));
                contact.first_name          = try!(FromJsonField::from_json_field(o, "firstName"));
                contact.last_name           = try!(FromJsonField::from_json_field(o, "lastName"));
                contact.suffix              = try!(FromJsonField::from_json_field(o, "suffix"));
                contact.nickname            = try!(FromJsonField::from_json_field(o, "nickname"));
                contact.birthday            = try!(FromJsonField::from_json_field(o, "birthday"));
                contact.anniversary         = try!(FromJsonField::from_json_field(o, "anniversary"));
                contact.company             = try!(FromJsonField::from_json_field(o, "company"));
                contact.department          = try!(FromJsonField::from_json_field(o, "department"));
                contact.job_title           = try!(FromJsonField::from_json_field(o, "jobTitle"));
                contact.emails              = try!(FromJsonField::from_json_field(o, "emails"));
                contact.default_email_index = try!(FromJsonField::from_json_field(o, "defaultEmailIndex"));
                contact.phones              = try!(FromJsonField::from_json_field(o, "phones"));
                contact.online              = try!(FromJsonField::from_json_field(o, "online"));
                contact.addresses           = try!(FromJsonField::from_json_field(o, "addresses"));
                contact.notes               = try!(FromJsonField::from_json_field(o, "notes"));
                Ok(contact)
            }
            _ => Err(ParseError::InvalidJsonType("Contact".to_string())),
        }
    }
}


impl Contact {
    pub fn updated_with(&self, p: &PartialContact) -> Contact {
        let mut c = self.clone();
        let u = p.clone();
        if let Present(v) = u.id                  { c.id = v };
        if let Present(v) = u.is_flagged          { c.is_flagged = v };
        if let Present(v) = u.avatar              { c.avatar = v };
        if let Present(v) = u.prefix              { c.prefix = v };
        if let Present(v) = u.first_name          { c.first_name = v };
        if let Present(v) = u.last_name           { c.last_name = v };
        if let Present(v) = u.suffix              { c.suffix = v };
        if let Present(v) = u.nickname            { c.nickname = v };
        if let Present(v) = u.birthday            { c.birthday = v };
        if let Present(v) = u.anniversary         { c.anniversary = v };
        if let Present(v) = u.company             { c.company = v };
        if let Present(v) = u.department          { c.department = v };
        if let Present(v) = u.job_title           { c.job_title = v };
        if let Present(v) = u.emails              { c.emails = v };
        if let Present(v) = u.default_email_index { c.default_email_index = v };
        if let Present(v) = u.phones              { c.phones = v };
        if let Present(v) = u.online              { c.online = v };
        if let Present(v) = u.addresses           { c.addresses = v };
        if let Present(v) = u.notes               { c.notes = v };
        c
    }

    pub fn to_partial(&self) -> PartialContact {
        PartialContact {
            id:                  Present(self.id.clone()),
            is_flagged:          Present(self.is_flagged),
            avatar:              Present(self.avatar.clone()),
            prefix:              Present(self.prefix.clone()),
            first_name:          Present(self.first_name.clone()),
            last_name:           Present(self.last_name.clone()),
            suffix:              Present(self.suffix.clone()),
            nickname:            Present(self.nickname.clone()),
            birthday:            Present(self.birthday.clone()),
            anniversary:         Present(self.anniversary.clone()),
            company:             Present(self.company.clone()),
            department:          Present(self.department.clone()),
            job_title:           Present(self.job_title.clone()),
            emails:              Present(self.emails.clone()),
            default_email_index: Present(self.default_email_index),
            phones:              Present(self.phones.clone()),
            online:              Present(self.online.clone()),
            addresses:           Present(self.addresses.clone()),
            notes:               Present(self.notes.clone()),
        }
    }

    pub fn to_filtered_partial(&self, properties: &Vec<String>) -> PartialContact {
        let mut p = PartialContact::default();
        p.id = Present(self.id.clone());
        for prop in properties.iter() {
            match prop.as_ref() {
                "isFlagged"         => p.is_flagged =          Present(self.is_flagged),
                "avatar"            => p.avatar =              Present(self.avatar.clone()),
                "prefix"            => p.prefix =              Present(self.prefix.clone()),
                "firstName"         => p.first_name =          Present(self.first_name.clone()),
                "lastName"          => p.last_name =           Present(self.last_name.clone()),
                "suffix"            => p.suffix =              Present(self.suffix.clone()),
                "nickname"          => p.nickname =            Present(self.nickname.clone()),
                "birthday"          => p.birthday =            Present(self.birthday.clone()),
                "anniversary"       => p.anniversary =         Present(self.anniversary.clone()),
                "company"           => p.company =             Present(self.company.clone()),
                "department"        => p.department =          Present(self.department.clone()),
                "jobTitle"          => p.job_title =           Present(self.job_title.clone()),
                "emails"            => p.emails =              Present(self.emails.clone()),
                "defaultEmailIndex" => p.default_email_index = Present(self.default_email_index),
                "phones"            => p.phones =              Present(self.phones.clone()),
                "online"            => p.online =              Present(self.online.clone()),
                "addresses"         => p.addresses =           Present(self.addresses.clone()),
                "notes"             => p.notes =               Present(self.notes.clone()),
                _                   => (),
            }
        }
        p
    }
}
