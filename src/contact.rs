use std::collections::BTreeMap;
use std::string::ToString;
use std::default::Default;
use rustc_serialize::json::{Json,ToJson};

use parse::*;
use parse::Presence::*;
use record;
use record::{Record, PartialRecord};
use types::File;


// basic three-part Date type, YYYY-MM-DD
// individual components may be 0, which will present as None
// only minimal range sanity checks are done
#[derive(Clone, PartialEq, Debug)]
pub struct OptionDate {
    pub y: Option<u16>,
    pub m: Option<u8>,
    pub d: Option<u8>,
}

impl Default for OptionDate {
    fn default() -> OptionDate {
        OptionDate { y: None, m: None, d: None }
    }
}

impl ToString for OptionDate {
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

impl ToJson for OptionDate {
    fn to_json(&self) -> Json {
        Json::String(self.to_string())
    }
}

impl FromJson for OptionDate {
    fn from_json(json: &Json) -> Result<OptionDate,ParseError> {
        match *json {
            Json::String(ref v) => {
                let (ok, err): (Vec<Result<u16,_>>,Vec<_>) = v.split('-').map(|ref s| s.parse::<u16>()).partition(|ref r| match **r { Ok(_) => true, Err(_) => false });
                if let false = err.is_empty() {
                    return Err(ParseError::InvalidStructure("OptionDate".to_string()));
                }

                let dv: Vec<Option<u16>> = ok.into_iter().map(|n| match n.ok().unwrap() {
                    0  => None,
                    nn => Some(nn),
                }).collect();

                if let false = dv.len() == 3 {
                    return Err(ParseError::InvalidStructure("OptionDate".to_string()));
                }

                Ok(OptionDate {
                    y: dv[0],
                    m: match dv[1] {
                        Some(n) if n > 12 => Err(ParseError::InvalidStructure("OptionDate".to_string())),
                        Some(n) => Ok(Some(n as u8)),
                        None => Ok(None),
                    }?,
                    d: match dv[2] {
                        Some(n) if n > 31 => Err(ParseError::InvalidStructure("OptionDate".to_string())),
                        Some(n) => Ok(Some(n as u8)),
                        None => Ok(None),
                    }?,
                })
            },
            _ => Err(ParseError::InvalidJsonType("OptionDate".to_string())),
        }
    }
}


// marker trait for contact types
pub trait ContactType: ToString + ToJson + FromJson + Default { }

make_prop_enum_type!(EmailType, "EmailType", Other,
    Personal => "personal",
    Work     => "work",
    Other    => "other"
);
impl ContactType for EmailType { }

make_prop_enum_type!(PhoneType, "PhoneType", Other,
    Home   => "home",
    Work   => "work",
    Mobile => "mobile",
    Fax    => "fax",
    Pager  => "pager",
    Other  => "other"
);
impl ContactType for PhoneType { }

make_prop_enum_type!(OnlineType, "OnlineType", Other,
    Uri      => "uri",
    Username => "username",
    Other    => "other"
);
impl ContactType for OnlineType { }

make_prop_enum_type!(AddressType, "AddressType", Other,
    Home    => "home",
    Work    => "work",
    Billing => "billing",
    Postal  => "postal",
    Other   => "other"
);
impl ContactType for AddressType { }


#[derive(Clone, PartialEq, Debug)]
pub struct ContactInformation<T: ContactType> {
    pub typ:        T,
    pub value:      String,
    pub label:      Option<String>,
    pub is_default: bool,
}

impl<T> Default for ContactInformation<T> where T: ContactType {
    fn default() -> ContactInformation<T> {
        ContactInformation {
            typ:        T::default(),
            value:      "".to_string(),
            label:      None,
            is_default: false,
        }
    }
}

impl<T> ToJson for ContactInformation<T> where T: ContactType {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::<String,Json>::new();
        self.typ.to_json_field(&mut d, "type");
        self.value.to_json_field(&mut d, "value");
        self.label.to_json_field(&mut d, "label");
        self.is_default.to_json_field(&mut d, "isDefault");
        Json::Object(d)
    }
}

impl<T> FromJson for ContactInformation<T> where T: ContactType {
    fn from_json(json: &Json) -> Result<ContactInformation<T>,ParseError> {
        match *json {
            Json::Object(ref o) => {
                let mut ci = ContactInformation::<T>::default();
                ci.typ        = FromJsonField::from_json_field(o, "type")?;
                ci.value      = FromJsonField::from_json_field(o, "value")?;
                ci.label      = FromJsonField::from_json_field(o, "label")?;
                ci.is_default = FromJsonField::from_json_field(o, "isDefault")?;
                Ok(ci)
            },
            _ => Err(ParseError::InvalidJsonType("ContactInformation".to_string())),
        }
    }
}


make_prop_type!(Address, "Address",
    typ:        AddressType    => "type",
    label:      Option<String> => "label",
    street:     String         => "street",
    locality:   String         => "locality",
    region:     String         => "region",
    postcode:   String         => "postcode",
    country:    String         => "country",
    is_default: String         => "isDefault"
);


make_record_type!(Contact, PartialContact, "Contact",
    is_flagged:          bool                                => "isFlagged",
    avatar:              Option<File>                        => "avatar",
    prefix:              String                              => "prefix",
    first_name:          String                              => "firstName",
    last_name:           String                              => "lastName",
    suffix:              String                              => "suffix",
    nickname:            String                              => "nickname",
    birthday:            OptionDate                          => "birthday",
    anniversary:         OptionDate                          => "anniversary",
    company:             String                              => "company",
    department:          String                              => "department",
    job_title:           String                              => "jobTitle",
    emails:              Vec<ContactInformation<EmailType>>  => "emails",
    phones:              Vec<ContactInformation<PhoneType>>  => "phones",
    online:              Vec<ContactInformation<OnlineType>> => "online",
    addresses:           Vec<Address>                        => "addresses",
    notes:               String                              => "notes"
);
