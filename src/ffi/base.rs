/*
Copyright (C) 2013 James Miller <james@aatch.net>

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

The LICENSE file provided with the Software is included with all copies or
substantial portions of the Software.

The conditions in the LICENSE file provided with the Software are also in effect.
*/

#![allow(non_camel_case_types)] // C types

use std::libc::{c_int, c_uint, c_void};

use ffi::xproto;

pub struct connection;

pub struct extension;

pub struct generic_iterator {
    data : *mut u8,
    rem : int,
    index : int
}

pub struct generic_reply {
    response_type : u8,
    pad0 : u8,
    sequence : u16,
    length : u32
}

pub struct generic_event {
    response_type : u8,
    pad0 : u8,
    sequence : u16,
    pad : [u32,..7],
    full_sequence : u32
}

pub struct ge_event {
    response_type : u8,
    pad0 : u8,
    sequence : u16,
    length : u32,
    event_type : u16,
    pad1 : u16,
    pad : [u32,..5],
    full_sequence : u32
}

pub struct generic_error {
    response_type : u8,
    error_code : u8,
    sequence : u16,
    resource_id : u32,
    minor_code : u16,
    major_code : u8,
    pad0 : u8,
    pad : [u32,..5],
    full_sequence : u32
}

pub struct void_cookie {
    sequence : int
}

pub struct auth_info {
    namelen : int,
    name : *mut u8,
    datalen : int,
    data : *mut u8
}

#[link(name = "xcb")]
pub extern {
    fn xcb_flush(c : *mut connection) -> c_int;

    fn xcb_get_maximum_request_length(c:*mut connection) -> u32;
    fn xcb_prefetch_maximum_request_length(c:*mut connection) -> c_void;

    fn xcb_wait_for_event(c:*mut connection) -> *mut generic_event;
    fn xcb_poll_for_event(c:*mut connection) -> *mut generic_event;
    fn xcb_poll_for_queued_event(c:*mut connection) -> *mut generic_event;
    fn xcb_request_check(c:*mut connection, cookie:void_cookie) -> *mut generic_error;
    fn xcb_discard_reply(c:*mut connection, sequence:c_uint) -> c_void;

    fn xcb_get_extension_data(c:*mut connection, ext:*mut extension) -> *mut xproto::query_extension_reply;
    fn xcb_prefetch_extension_data(c:*mut connection, ext:*mut extension) -> c_void;

    fn xcb_get_setup(c:*mut connection) -> *mut xproto::setup;
    fn xcb_get_file_descriptor(c:*mut connection) -> c_int;
    fn xcb_connection_has_error(c:*mut connection) -> c_int;
    fn xcb_connect_to_fd(fd:c_int, auth_info:*mut auth_info) -> *mut connection;
    fn xcb_disconnect(c:*mut connection) -> c_void;

    fn xcb_parse_display(name:*mut u8, host:*mut *mut u8, display:*mut c_int, screen:*mut c_int) -> c_int;
    fn xcb_connect(displayname:*mut u8, screenp:*mut c_int) -> *mut connection;
    fn xcb_connect_to_display_with_auth_info(display:*mut u8, auth:*mut auth_info, screen:*mut c_int) -> *mut connection;

    fn xcb_generate_id(c:*mut connection) -> u32;
}

pub mod Xlib {
    use super::{connection};
    use std::libc::{c_void, c_uint};
    struct Display;

    type XEventQueueOwner = c_uint;
    static XlibOwnsEventQueue : XEventQueueOwner = 0;
    static XCBOwnsEventQueue : XEventQueueOwner  = 1;

    #[link(name="X11")]
    #[link(name="X11-xcb")]
    pub extern {
        fn XGetXCBConnection(dpy:*mut Display) -> *mut connection;
        fn XSetEventQueueOwner(dpy:*mut Display, owner:XEventQueueOwner) -> c_void;
    }

}
