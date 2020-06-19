extern crate chrono;
extern crate rustc_serialize;
extern crate uuid;

#[macro_use]
mod macros;

pub mod method;
pub mod parse;
pub mod record;
pub mod types;

pub use self::calendar::Calendar;
pub use self::calendar_event::CalendarEvent;
pub use self::contact::Contact;
pub use self::contact_group::ContactGroup;
pub use self::mailbox::Mailbox;
pub use self::message::Message;

pub mod calendar;
pub mod calendar_event;
pub mod contact;
pub mod contact_group;
pub mod mailbox;
pub mod message;
pub mod message_copy;
pub mod message_import;
pub mod message_list;
