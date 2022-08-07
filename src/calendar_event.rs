use std::collections::BTreeMap;
use std::string::ToString;
use std::default::Default;
use std::ops::Deref;
use rustc_serialize::json::{Json,ToJson};
use chrono::NaiveDateTime;

use parse::*;
use parse::Presence::*;
use record;
use record::{Record, PartialRecord};
use types::{File,Date};


#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct LocalDate(pub NaiveDateTime);

impl Deref for LocalDate  {
    type Target = NaiveDateTime;
    fn deref<'a>(&'a self) -> &'a Self::Target {
        &self.0
    }
}

impl Default for LocalDate {
    fn default() -> LocalDate {
        LocalDate(NaiveDateTime::from_timestamp(0, 0))
    }
}

impl ToString for LocalDate {
    fn to_string(&self) -> String {
        self.format("%Y-%m-%d").to_string()
    }
}

impl ToJson for LocalDate {
    fn to_json(&self) -> Json {
        Json::String(self.to_string())
    }
}

impl FromJson for LocalDate {
    fn from_json(json: &Json) -> Result<LocalDate,ParseError> {
        match *json {
            Json::String(ref v) => {
                match v.parse::<NaiveDateTime>() {
                    Ok(dt) => Ok(LocalDate(dt)),
                    _      => Err(ParseError::InvalidStructure("LocalDate".to_string())),
                }
            },
            _ => Err(ParseError::InvalidJsonType("LocalDate".to_string())),
        }
    }
}


make_prop_enum_type!(Frequency, "Frequency", Secondly, // XXX don't really want a default
    Yearly   => "yearly",
    Monthly  => "monthly",
    Weekly   => "weekly",
    Daily    => "daily",
    Hourly   => "hourly",
    Minutely => "minutely",
    Secondly => "secondly"
);

make_prop_type!(Recurrence, "Recurrence",
    frequency:         Frequency        => "frequency",
    interval:          Option<i32>      => "interval",
    first_day_of_week: Option<i32>      => "firstDayOfWeek", // XXX how to represent value rules?
    by_day:            Option<Vec<i32>> => "byDay",
    by_date:           Option<Vec<i32>> => "byDate",
    by_month:          Option<Vec<i32>> => "byMonth",
    by_year_day:       Option<Vec<i32>> => "byYearDay",
    by_week_no:        Option<Vec<i32>> => "byWeekNo",
    by_hour:           Option<Vec<i32>> => "byHour",
    by_minute:         Option<Vec<i32>> => "byMinute",
    by_second:         Option<Vec<i32>> => "bySecond",
    by_set_position:   Option<Vec<i32>> => "bySetPosition",
    count:             Option<u64>      => "count",
    until:             LocalDate        => "until"
);

make_prop_enum_type!(AlertType, "AlertType", Alert,
    Email => "email",
    Alert => "alert"
);

make_prop_type!(Alert, "Alert",
    minutes_before: i32       => "minutesBefore",
    typ:            AlertType => "type"
);

make_prop_enum_type!(Rsvp, "Rsvp", None,
    None  => "",
    Yes   => "yes",
    Maybe => "maybe",
    No    => "no"
);

make_prop_type!(Participant, "Participant",
    name:   String => "name",
    email:  String => "email",
    is_you: bool   => "isYou",
    rsvp:   Rsvp   => "rsvp"
);

#[derive(Clone, PartialEq, Debug)]
pub struct ExceptionMap(pub BTreeMap<LocalDate,Option<PartialCalendarEvent>>);

impl Deref for ExceptionMap  {
    type Target = BTreeMap<LocalDate,Option<PartialCalendarEvent>>;
    fn deref<'a>(&'a self) -> &'a Self::Target {
        &self.0
    }
}

impl ToJson for ExceptionMap {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::<String,Json>::new();
        for (k,v) in self.iter() {
            d.insert(k.to_string(), v.to_json());
        }
        Json::Object(d)
    }
}

impl FromJson for ExceptionMap {
    fn from_json(json: &Json) -> Result<ExceptionMap,ParseError> {
        match *json {
            Json::Object(ref v) => {
                let mut d = BTreeMap::<LocalDate,Option<PartialCalendarEvent>>::new();
                for (k,v) in v.iter() {
                    let date = LocalDate::from_json(&Json::String(k.clone()))?; // XXX awkward
                    let obj = match *v { // XXX prefer FromJson for Option<T> but meh, compiler
                        Json::Null => None,
                        _ => {
                            let p = PartialCalendarEvent::from_json(v)?;
                            Some(p)
                        },
                    };
                    d.insert(date, obj);

                }
                Ok(ExceptionMap(d))
            },
            _ => Err(ParseError::InvalidJsonType("ExceptionMap".to_string()))
        }
    }
}

make_record_type!(CalendarEvent, PartialCalendarEvent, "CalendarEvent",
    calendar_id:     String                   => "calendarId",
    summary:         String                   => "summary",
    description:     String                   => "description",
    location:        String                   => "location",
    show_as_free:    bool                     => "showAsFree",
    is_all_day:      bool                     => "isAllDay",
    start:           Date                     => "start",
    end:             Date                     => "end",
    start_time_zone: Option<String>           => "startTimeZone",
    end_time_zone:   Option<String>           => "endTimeZone",
    recurrence:      Option<Recurrence>       => "recurrence",
    inclusions:      Option<Vec<LocalDate>>   => "inclusions",
    exceptions:      Option<ExceptionMap>     => "exceptions",
    alerts:          Option<Vec<Alert>>       => "alerts",
    organizer:       Option<Participant>      => "organizer",
    attendees:       Option<Vec<Participant>> => "attendees",
    attachments:     Option<Vec<File>>        => "attachments"
);
