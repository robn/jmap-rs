extern crate rustc_serialize;
extern crate uuid;
extern crate chrono;

#[macro_use] mod macros;

pub mod parse;
pub mod method;
pub mod record;
pub mod types;

pub use self::mailbox::Mailbox;
pub use self::message::Message;
pub use self::calendar::Calendar;
pub use self::calendar_event::CalendarEvent;
pub use self::contact::Contact;
pub use self::contact_group::ContactGroup;

pub mod mailbox;
pub mod message;
pub mod message_list;
pub mod message_import;
pub mod calendar;
pub mod calendar_event;
pub mod contact;
pub mod contact_group;
