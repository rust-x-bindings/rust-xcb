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

#[allow(non_camel_case_types)]; // C types

use core::libc::{c_int, c_uint, c_void};

use ll::xproto;

pub struct connection;

pub struct extension;

pub struct generic_iterator {
    data : *u8,
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
    name : *u8,
    datalen : int,
    data : *u8
}

#[link_args="-lxcb"]
pub extern {
    unsafe fn xcb_flush(c : *connection) -> c_int;

    unsafe fn xcb_get_maximum_request_length(c:*connection) -> u32;
    unsafe fn xcb_prefetch_maximum_request_length(c:*connection) -> c_void;

    unsafe fn xcb_wait_for_event(c:*connection) -> *generic_event;
    unsafe fn xcb_poll_for_event(c:*connection) -> *generic_event;
    unsafe fn xcb_poll_for_queued_event(c:*connection) -> *generic_event;
    unsafe fn xcb_request_check(c:*connection, cookie:void_cookie) -> *generic_error;
    unsafe fn xcb_discard_reply(c:*connection, sequence:c_uint) -> c_void;

    unsafe fn xcb_get_extension_data(c:*connection, ext:*extension) -> *xproto::query_extension_reply;
    unsafe fn xcb_prefetch_extension_data(c:*connection, ext:*extension) -> c_void;

    unsafe fn xcb_get_setup(c:*connection) -> *xproto::setup;
    unsafe fn xcb_get_file_descriptor(c:*connection) -> c_int;
    unsafe fn xcb_connection_has_error(c:*connection) -> c_int;
    unsafe fn xcb_connect_to_fd(fd:c_int, auth_info:*auth_info) -> *connection;
    unsafe fn xcb_disconnect(c:*connection) -> c_void;

    unsafe fn xcb_parse_display(name:*u8, host:**u8, display:*c_int, screen:*c_int) -> c_int;
    unsafe fn xcb_connect(displayname:*u8, screenp:*c_int) -> *connection;
    unsafe fn xcb_connect_to_display_with_auth_info(display:*u8, auth:*auth_info, screen:*c_int) -> *connection;

    unsafe fn xcb_generate_id(c:*connection) -> u32;
}

pub mod Xlib {
    use super::{connection};
    use core::libc::{c_void, c_uint};
    struct Display;

    type XEventQueueOwner = c_uint;
    static XlibOwnsEventQueue : XEventQueueOwner = 0;
    static XCBOwnsEventQueue : XEventQueueOwner  = 1;

    #[link_args="-lX11 -lX11-xcb"]
    pub extern {
        unsafe fn XGetXCBConnection(dpy:*Display) -> connection;
        unsafe fn XSetEventQueueOwner(dpy:*Display, owner:XEventQueueOwner) -> c_void;
    }

}
