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

extern crate libc;

use libc::{c_int, c_uint, c_void};

use ffi::xproto;

pub enum connection {}

pub enum extension {}

#[repr(C)]
pub struct generic_iterator {
    data : *mut u8,
    rem : c_int,
    index : c_int
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct generic_reply {
    response_type : u8,
    pad0 : u8,
    sequence : u16,
    length : u32
}

#[derive(Copy)]
#[repr(C)]
pub struct generic_event {
    pub response_type : u8,
    pad0 : u8,
    sequence : u16,
    pad : [u32; 7],
    full_sequence : u32
}
impl Clone for generic_event {
    fn clone(&self) -> generic_event { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct ge_event {
    response_type : u8,
    pad0 : u8,
    sequence : u16,
    length : u32,
    event_type : u16,
    pad1 : u16,
    pad : [u32; 5],
    full_sequence : u32
}
impl Clone for ge_event {
    fn clone(&self) -> ge_event { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct generic_error {
    response_type : u8,
    error_code : u8,
    sequence : u16,
    resource_id : u32,
    minor_code : u16,
    major_code : u8,
    pad0 : u8,
    pad : [u32; 5],
    full_sequence : u32
}
impl Clone for generic_error {
    fn clone(&self) -> generic_error { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct void_cookie {
    sequence : c_int
}

#[repr(C)]
pub struct auth_info {
    namelen : c_int,
    name : *mut u8,
    datalen : c_int,
    data : *mut u8
}

#[link(name = "xcb")]
extern {
    pub fn xcb_flush(c : *mut connection) -> c_int;

    pub fn xcb_get_maximum_request_length(c:*mut connection) -> u32;
    pub fn xcb_prefetch_maximum_request_length(c:*mut connection) -> c_void;

    pub fn xcb_wait_for_event(c:*mut connection) -> *mut generic_event;
    pub fn xcb_poll_for_event(c:*mut connection) -> *mut generic_event;
    pub fn xcb_poll_for_queued_event(c:*mut connection) -> *mut generic_event;
    pub fn xcb_request_check(c:*mut connection, cookie:void_cookie) -> *mut generic_error;
    pub fn xcb_discard_reply(c:*mut connection, sequence:c_uint) -> c_void;

    pub fn xcb_get_extension_data(c:*mut connection, ext:*mut extension) -> *mut xproto::query_extension_reply;
    pub fn xcb_prefetch_extension_data(c:*mut connection, ext:*mut extension) -> c_void;

    pub fn xcb_get_setup(c:*mut connection) -> *mut xproto::setup;
    pub fn xcb_get_file_descriptor(c:*mut connection) -> c_int;
    pub fn xcb_connection_has_error(c:*mut connection) -> c_int;
    pub fn xcb_connect_to_fd(fd:c_int, auth_info:*mut auth_info) -> *mut connection;
    pub fn xcb_disconnect(c:*mut connection) -> c_void;

    pub fn xcb_parse_display(name:*mut u8, host:*mut *mut u8, display:*mut c_int, screen:*mut c_int) -> c_int;
    pub fn xcb_connect(displayname:*mut u8, screenp:*mut c_int) -> *mut connection;
    pub fn xcb_connect_to_display_with_auth_info(display:*mut u8, auth:*mut auth_info, screen:*mut c_int) -> *mut connection;

    pub fn xcb_generate_id(c:*mut connection) -> u32;
}

#[allow(non_upper_case_globals)]
pub mod Xlib {
    use super::{connection};
    use libc::{c_void, c_uint};
    pub enum Display {}

    pub type XEventQueueOwner = c_uint;
    pub static XlibOwnsEventQueue : XEventQueueOwner = 0;
    pub static XCBOwnsEventQueue : XEventQueueOwner  = 1;

    #[link(name="X11")]
    #[link(name="X11-xcb")]
    extern {
        pub fn XGetXCBConnection(dpy:*mut Display) -> *mut connection;
        pub fn XSetEventQueueOwner(dpy:*mut Display, owner:XEventQueueOwner) -> c_void;
    }

}
