//
// This file generated automatically from xtest.xml by r_client.py.
// Edit at your peril.
//

//Make the compiler quiet
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use std;
use libc::*;
use ffi;
use ffi::xproto;

pub const TEST_MAJOR_VERSION : c_uint = 2;
pub const TEST_MINOR_VERSION : c_uint = 1;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_test_get_version_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_test_get_version_request_t {
     pub major_opcode :    u8,
     pub minor_opcode :    u8,
     pub length :          u16,
     pub major_version :   u8,
     pub pad0 :            u8,
     pub minor_version :   u16
}

impl Copy for xcb_test_get_version_request_t {}
impl Clone for xcb_test_get_version_request_t {
    fn clone(&self) -> xcb_test_get_version_request_t { *self }
}

#[repr(C)]
pub struct xcb_test_get_version_reply_t {
     pub response_type :   u8,
     pub major_version :   u8,
     pub sequence :        u16,
     pub length :          u32,
     pub minor_version :   u16
}

impl Copy for xcb_test_get_version_reply_t {}
impl Clone for xcb_test_get_version_reply_t {
    fn clone(&self) -> xcb_test_get_version_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_test_compare_cursor_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_test_compare_cursor_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub window :         ffi::xproto::xcb_window_t,
     pub cursor :         ffi::xproto::xcb_cursor_t
}

impl Copy for xcb_test_compare_cursor_request_t {}
impl Clone for xcb_test_compare_cursor_request_t {
    fn clone(&self) -> xcb_test_compare_cursor_request_t { *self }
}

#[repr(C)]
pub struct xcb_test_compare_cursor_reply_t {
     pub response_type :   u8,
     pub same :            u8,
     pub sequence :        u16,
     pub length :          u32
}

impl Copy for xcb_test_compare_cursor_reply_t {}
impl Clone for xcb_test_compare_cursor_reply_t {
    fn clone(&self) -> xcb_test_compare_cursor_reply_t { *self }
}


#[repr(C)]
pub struct xcb_test_fake_input_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub type_ :          u8,
     pub detail :         u8,
     pub pad0 :           [u8; 2],
     pub time :           u32,
     pub root :           ffi::xproto::xcb_window_t,
     pub pad1 :           [u8; 8],
     pub rootX :          i16,
     pub rootY :          i16,
     pub pad2 :           [u8; 7],
     pub deviceid :       u8
}

impl Copy for xcb_test_fake_input_request_t {}
impl Clone for xcb_test_fake_input_request_t {
    fn clone(&self) -> xcb_test_fake_input_request_t { *self }
}


#[repr(C)]
pub struct xcb_test_grab_control_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub impervious :     u8,
     pub pad0 :           [u8; 3]
}

impl Copy for xcb_test_grab_control_request_t {}
impl Clone for xcb_test_grab_control_request_t {
    fn clone(&self) -> xcb_test_grab_control_request_t { *self }
}
#[link(name="xcb-xtest")]
extern "C" {

/// Delivers a request to the X server.
///
pub fn xcb_test_get_version (c : *mut ffi::base::xcb_connection_t,
                                major_version :  u8,
                                minor_version :  u16) -> xcb_test_get_version_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_test_get_version_unchecked (c : *mut ffi::base::xcb_connection_t,
                                          major_version :  u8,
                                          minor_version :  u16) -> xcb_test_get_version_cookie_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_test_get_version_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_test_get_version_reply (c : *mut ffi::base::xcb_connection_t,
                                      cookie : xcb_test_get_version_cookie_t,
                                      e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_test_get_version_reply_t;

/// Delivers a request to the X server.
///
pub fn xcb_test_compare_cursor (c : *mut ffi::base::xcb_connection_t,
                                   window :  ffi::xproto::xcb_window_t,
                                   cursor :  ffi::xproto::xcb_cursor_t) -> xcb_test_compare_cursor_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_test_compare_cursor_unchecked (c : *mut ffi::base::xcb_connection_t,
                                             window :  ffi::xproto::xcb_window_t,
                                             cursor :  ffi::xproto::xcb_cursor_t) -> xcb_test_compare_cursor_cookie_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_test_compare_cursor_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_test_compare_cursor_reply (c : *mut ffi::base::xcb_connection_t,
                                         cookie : xcb_test_compare_cursor_cookie_t,
                                         e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_test_compare_cursor_reply_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_test_fake_input_checked (c : *mut ffi::base::xcb_connection_t,
                                       type_ :  u8,
                                       detail :  u8,
                                       time :  u32,
                                       root :  ffi::xproto::xcb_window_t,
                                       rootX :  i16,
                                       rootY :  i16,
                                       deviceid :  u8) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_test_fake_input (c : *mut ffi::base::xcb_connection_t,
                               type_ :  u8,
                               detail :  u8,
                               time :  u32,
                               root :  ffi::xproto::xcb_window_t,
                               rootX :  i16,
                               rootY :  i16,
                               deviceid :  u8) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_test_grab_control_checked (c : *mut ffi::base::xcb_connection_t,
                                         impervious :  u8) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_test_grab_control (c : *mut ffi::base::xcb_connection_t,
                                 impervious :  u8) -> ffi::base::xcb_void_cookie_t;
}

