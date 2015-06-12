extern crate rustc_serialize;
extern crate uuid;
extern crate chrono;

#[macro_use] pub mod macros;

pub mod parse;
pub mod method;
pub mod record;
pub mod types;

pub mod mailbox;
pub mod calendar;
pub mod calendarevent;
pub mod contact;
pub mod contactgroup;
