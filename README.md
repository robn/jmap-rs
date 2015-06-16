# jmap-rs

A JMAP parser/generator in Rust

## What is JMAP?

JMAP is the JSON Mail Access Protocol, an API for accessing mail, calendars and
contacts on a server. In simple terms its a HTTP+JSON replacement for IMAP,
CalDAV and CardDAV. See [jmap.io](http://jmap.io/) for more info.

## What is jmap-rs?

Everything you need to parse, generate and manipulate JMAP data structures from
Rust. Its what you need if you're writing a JMAP server or client.

[salada](https://github.com/robn/salada) is a JMAP server that uses this
library.

## Status

Currently can round-trip JMAP Contact, ContactGroup, Calendar, CalendarEvent
and Mailbox objects and CRUD methods into the corresponding Rust structures and
back. More is being added as required by client projects (mainly salada).

## Credits and license

Copyright (c) 2015 Robert Norris. MIT license. See LICENSE.

## Contributing

Pull requests are very welcome! For more general discussions about jmap-rs or
JMAP, try the
[jmap-discuss](https://groups.google.com/forum/#!forum/jmap-discuss) mailing
list or [#jmap on Freenode IRC](http://webchat.freenode.net/?channels=pioneer).

