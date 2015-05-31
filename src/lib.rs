extern crate rustc_serialize;
extern crate uuid;

#[macro_use] pub mod macros;

pub mod parse;
pub mod method;
pub mod record;

pub mod mailbox;
pub mod calendar;
pub mod contact;
pub mod contactgroup;
