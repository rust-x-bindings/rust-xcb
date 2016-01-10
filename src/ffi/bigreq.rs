//
// This file generated automatically from bigreq.xml by r_client.py.
// Edit at your peril.
//

//Make the compiler quiet
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use std;
use libc::*;
use ffi;

pub const BIGREQUESTS_MAJOR_VERSION : c_uint = 0;
pub const BIGREQUESTS_MINOR_VERSION : c_uint = 0;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_big_requests_enable_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_big_requests_enable_request_t {
    pub major_opcode :   u8,
    pub minor_opcode :   u8,
    pub length :         u16
}

impl Copy for xcb_big_requests_enable_request_t {}
impl Clone for xcb_big_requests_enable_request_t {
    fn clone(&self) -> xcb_big_requests_enable_request_t { *self }
}

#[repr(C)]
pub struct xcb_big_requests_enable_reply_t {
    pub response_type :            u8,
    pub pad0 :                     u8,
    pub sequence :                 u16,
    pub length :                   u32,
    pub maximum_request_length :   u32
}

impl Copy for xcb_big_requests_enable_reply_t {}
impl Clone for xcb_big_requests_enable_reply_t {
    fn clone(&self) -> xcb_big_requests_enable_reply_t { *self }
}
extern "C" {

/// Delivers a request to the X server.
///
pub fn xcb_big_requests_enable (c : *mut ffi::base::xcb_connection_t) -> xcb_big_requests_enable_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_big_requests_enable_unchecked (c : *mut ffi::base::xcb_connection_t) -> xcb_big_requests_enable_cookie_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_big_requests_enable_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_big_requests_enable_reply (c : *mut ffi::base::xcb_connection_t,
                                         cookie : xcb_big_requests_enable_cookie_t,
                                         e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_big_requests_enable_reply_t;
}

