# jmap-rs

A JMAP parser/generator in Rust

## what is JMAP?

JMAP is the JSON Mail Access Protocol, an API for accessing mail, calendars and
contacts on a server. In simple terms its a HTTP+JSON replacement for IMAP,
CalDAV and CardDAV. See [jmap.io](http://jmap.io/) for more info.

## what is jmap-rs?

Everything you need to parse, generate and manipulate JMAP data structures from
Rust. Its what you need if you're writing a JMAP server or client.

(salada)[https://github.com/robn/salada] is a JMAP server that uses this
library.

## status

So far it can round-trip a JMAP Contact object and its related methods into the
corresponding Rust structures and back. Most of my efforts so far are in
learning the language and type system and getting a good set of tools and
idioms established for building out the entire protocol.

## credits and license

Copyright (c) 2015 Robert Norris. MIT license. See LICENSE.

## contributing

Please hack on this and send pull requests :)

