//
// This file generated automatically from ge.xml by r_client.py.
// Edit at your peril.
//

//Make the compiler quiet
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use std;
use libc::*;
use ffi;

pub const GENERICEVENT_MAJOR_VERSION : c_uint = 1;
pub const GENERICEVENT_MINOR_VERSION : c_uint = 0;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_genericevent_query_version_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_genericevent_query_version_request_t {
    pub major_opcode :           u8,
    pub minor_opcode :           u8,
    pub length :                 u16,
    pub client_major_version :   u16,
    pub client_minor_version :   u16
}

impl Copy for xcb_genericevent_query_version_request_t {}
impl Clone for xcb_genericevent_query_version_request_t {
    fn clone(&self) -> xcb_genericevent_query_version_request_t { *self }
}

#[repr(C)]
pub struct xcb_genericevent_query_version_reply_t {
    pub response_type :   u8,
    pub pad0 :            u8,
    pub sequence :        u16,
    pub length :          u32,
    pub major_version :   u16,
    pub minor_version :   u16,
    pub pad1 :            [u8; 20]
}

impl Copy for xcb_genericevent_query_version_reply_t {}
impl Clone for xcb_genericevent_query_version_reply_t {
    fn clone(&self) -> xcb_genericevent_query_version_reply_t { *self }
}
extern "C" {

/// Delivers a request to the X server.
///
pub fn xcb_genericevent_query_version (c : *mut ffi::base::xcb_connection_t,
                                          client_major_version :  u16,
                                          client_minor_version :  u16) -> xcb_genericevent_query_version_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_genericevent_query_version_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                    client_major_version :  u16,
                                                    client_minor_version :  u16) -> xcb_genericevent_query_version_cookie_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_genericevent_query_version_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_genericevent_query_version_reply (c : *mut ffi::base::xcb_connection_t,
                                                cookie : xcb_genericevent_query_version_cookie_t,
                                                e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_genericevent_query_version_reply_t;
}

