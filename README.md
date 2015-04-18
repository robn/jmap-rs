# jmap-rs

A JMAP parser/generator in Rust

## what is JMAP?

JMAP is the JSON Mail Access Protocol, an API for accessing mail, calendars and
contacts on a server. In simple terms its a HTTP+JSON replacement for IMAP,
CalDAV and CardDAV. See [jmap.io](http://jmap.io/) for more info.

## what is jmap-rs?

A Rust implementation of JMAP. Initially, its just the parser and generator
portions. Later, a server will be built on top of this, but I'm not yet sure if
that will be part of jmap-rs directly or use it as a library.

## status

So far it can round-trip a JMAP Contact object into the correspond Rust
structures and back. Most of my efforts so far are in learning the language and
type system and getting a good set of tools and idioms established for building
out the entire protocol.

## credits and license

Copyright (c) 2015 Robert Norris. MIT license. See LICENSE.

## contributing

Please hack on this and send pull requests :)

