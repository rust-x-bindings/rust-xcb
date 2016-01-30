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

use libc::{c_int, c_uint, c_void, c_char};
use ffi::xproto;


pub enum xcb_connection_t {}

pub enum xcb_extension_t {}

pub enum xcb_special_event_t {}

pub const XCB_COPY_FROM_PARENT: u32 = 0;

#[repr(C)]
pub struct xcb_generic_iterator_t {
    data:  *mut c_void,
    rem:   c_int,
    index: c_int
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_generic_reply_t {
    response_type: u8,
    pad0:          u8,
    sequence:      u16,
    length:        u32
}

#[derive(Copy)]
#[repr(C)]
pub struct xcb_generic_event_t {
    pub response_type: u8,
    pub pad0:          u8,
    pub sequence:      u16,
    pub pad:           [u32; 7],
    pub full_sequence: u32
}
impl Clone for xcb_generic_event_t {
    fn clone(&self) -> xcb_generic_event_t { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct xcb_ge_event_t {
    response_type: u8,
    pad0:          u8,
    sequence:      u16,
    length:        u32,
    event_type:    u16,
    pad1:          u16,
    pad:           [u32; 5],
    full_sequence: u32
}
impl Clone for xcb_ge_event_t {
    fn clone(&self) -> xcb_ge_event_t { *self }
}

#[derive(Copy, Debug)]
#[repr(C)]
pub struct xcb_generic_error_t {
    response_type:  u8,
    error_code:     u8,
    sequence:       u16,
    resource_id:    u32,
    minor_code:     u16,
    major_code:     u8,
    pad0:           u8,
    pad:            [u32; 5],
    full_sequence:  u32
}
impl Clone for xcb_generic_error_t {
    fn clone(&self) -> xcb_generic_error_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_void_cookie_t {
    sequence: c_int
}

#[repr(C)]
pub struct xcb_auth_info_t {
    namelen:    c_int,
    name:       *mut c_char,
    datalen:    c_int,
    data:       *mut c_char
}


#[link(name="xcb")]
extern {

    pub fn xcb_flush(c: *mut xcb_connection_t)
            -> c_int;

    pub fn xcb_get_maximum_request_length(c: *mut xcb_connection_t)
            -> u32;

    pub fn xcb_prefetch_maximum_request_length(c: *mut xcb_connection_t);

    pub fn xcb_wait_for_event(c: *mut xcb_connection_t)
            -> *mut xcb_generic_event_t;

    pub fn xcb_poll_for_event(c: *mut xcb_connection_t)
            -> *mut xcb_generic_event_t;

    pub fn xcb_poll_for_queued_event(c: *mut xcb_connection_t)
            -> *mut xcb_generic_event_t;

    pub fn xcb_poll_for_special_event(c: *mut xcb_connection_t,
                                      se: *mut xcb_special_event_t)
            -> *mut xcb_generic_event_t;

    pub fn xcb_wait_for_special_event(c: *mut xcb_connection_t,
                                      se: *mut xcb_special_event_t)
            -> *mut xcb_generic_event_t;

    pub fn xcb_register_for_special_xge(c: *mut xcb_connection_t,
                                        ext: *mut xcb_extension_t,
                                        eid: u32,
                                        stamp: *mut u32)
            -> *mut xcb_special_event_t;

    pub fn xcb_unregister_for_special_event(c: *mut xcb_connection_t,
                                            se: *mut xcb_special_event_t);

    pub fn xcb_request_check(c: *mut xcb_connection_t,
                             cookie: xcb_void_cookie_t)
            -> *mut xcb_generic_error_t;

    pub fn xcb_discard_reply(c: *mut xcb_connection_t,
                             sequence: c_uint);

    pub fn xcb_discard_reply64(c: *mut xcb_connection_t,
                               sequence: u64);

    pub fn xcb_get_extension_data(c: *mut xcb_connection_t,
                                  ext: *mut xcb_extension_t)
            -> *const xproto::xcb_query_extension_reply_t;

    pub fn xcb_prefetch_extension_data(c: *mut xcb_connection_t,
                                       ext: *mut xcb_extension_t);

    pub fn xcb_get_setup(c: *mut xcb_connection_t)
            -> *const xproto::xcb_setup_t;

    pub fn xcb_get_file_descriptor(c: *mut xcb_connection_t)
            -> c_int;

    pub fn xcb_connection_has_error(c: *mut xcb_connection_t)
            -> c_int;

    pub fn xcb_connect_to_fd(fd: c_int,
                             auth_info: *mut xcb_auth_info_t)
            -> *mut xcb_connection_t;

    pub fn xcb_disconnect(c: *mut xcb_connection_t);

    pub fn xcb_parse_display(name: *mut c_char,
                             host: *mut *mut c_char,
                             display: *mut c_int,
                             screen: *mut c_int)
            -> c_int;

    pub fn xcb_connect(displayname: *mut c_char,
                       screenp: *mut c_int)
            -> *mut xcb_connection_t;

    pub fn xcb_connect_to_display_with_auth_info(display: *mut c_char,
                                                 auth: *mut xcb_auth_info_t,
                                                 screen: *mut c_int)
            -> *mut xcb_connection_t;

    pub fn xcb_generate_id(c: *mut xcb_connection_t)
            -> u32;

}

#[allow(non_upper_case_globals)]
pub mod Xlib {
    use super::{xcb_connection_t};
    use libc::{c_void, c_uint};
    pub enum Display {}

    pub type XEventQueueOwner = c_uint;
    pub static XlibOwnsEventQueue : XEventQueueOwner = 0;
    pub static XCBOwnsEventQueue : XEventQueueOwner  = 1;

    #[link(name="X11")]
    #[link(name="X11-xcb")]
    extern {
        pub fn XGetXCBConnection(dpy:*mut Display) -> *mut xcb_connection_t;
        pub fn XSetEventQueueOwner(dpy:*mut Display, owner:XEventQueueOwner) -> c_void;
    }

}
