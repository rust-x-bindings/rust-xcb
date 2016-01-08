/*
 * This file generated automatically from xtest.xml by r_client.py.
 * Edit at your peril.
 */

//Make the compiler quiet
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use std;
use libc::*;
use ffi;
use ffi::xproto;

pub static TEST_MAJOR_VERSION : c_uint = 2;
pub static TEST_MINOR_VERSION : c_uint = 1;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct get_version_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct get_version_request {
     pub major_opcode :    u8,
     pub minor_opcode :    u8,
     pub length :          u16,
     pub major_version :   u8,
     pub pad0 :            u8,
     pub minor_version :   u16
}

impl Copy for get_version_request {}
impl Clone for get_version_request {
    fn clone(&self) -> get_version_request { *self }
}

#[repr(C)]
pub struct get_version_reply {
     pub response_type :   u8,
     pub major_version :   u8,
     pub sequence :        u16,
     pub length :          u32,
     pub minor_version :   u16
}

impl Copy for get_version_reply {}
impl Clone for get_version_reply {
    fn clone(&self) -> get_version_reply { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct compare_cursor_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct compare_cursor_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub window :         ffi::xproto::window,
     pub cursor :         ffi::xproto::cursor
}

impl Copy for compare_cursor_request {}
impl Clone for compare_cursor_request {
    fn clone(&self) -> compare_cursor_request { *self }
}

#[repr(C)]
pub struct compare_cursor_reply {
     pub response_type :   u8,
     pub same :            u8,
     pub sequence :        u16,
     pub length :          u32
}

impl Copy for compare_cursor_reply {}
impl Clone for compare_cursor_reply {
    fn clone(&self) -> compare_cursor_reply { *self }
}


#[repr(C)]
pub struct fake_input_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub type_ :          u8,
     pub detail :         u8,
     pub pad0 :           [u8; 2],
     pub time :           u32,
     pub root :           ffi::xproto::window,
     pub pad1 :           [u8; 8],
     pub rootX :          i16,
     pub rootY :          i16,
     pub pad2 :           [u8; 7],
     pub deviceid :       u8
}

impl Copy for fake_input_request {}
impl Clone for fake_input_request {
    fn clone(&self) -> fake_input_request { *self }
}


#[repr(C)]
pub struct grab_control_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub impervious :     u8,
     pub pad0 :           [u8; 3]
}

impl Copy for grab_control_request {}
impl Clone for grab_control_request {
    fn clone(&self) -> grab_control_request { *self }
}
#[link(name="xcb-xtest")]
extern "C" {

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_test_get_version (c : *mut ffi::base::connection,
                                major_version :  u8,
                                minor_version :  u16) -> get_version_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_test_get_version_unchecked (c : *mut ffi::base::connection,
                                          major_version :  u8,
                                          minor_version :  u16) -> get_version_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_test_get_version_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_test_get_version_reply (c : *mut ffi::base::connection,
                                      cookie : get_version_cookie,
                                      e : *mut *mut ffi::base::generic_error) -> *mut get_version_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_test_compare_cursor (c : *mut ffi::base::connection,
                                   window :  ffi::xproto::window,
                                   cursor :  ffi::xproto::cursor) -> compare_cursor_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_test_compare_cursor_unchecked (c : *mut ffi::base::connection,
                                             window :  ffi::xproto::window,
                                             cursor :  ffi::xproto::cursor) -> compare_cursor_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_test_compare_cursor_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_test_compare_cursor_reply (c : *mut ffi::base::connection,
                                         cookie : compare_cursor_cookie,
                                         e : *mut *mut ffi::base::generic_error) -> *mut compare_cursor_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_test_fake_input_checked (c : *mut ffi::base::connection,
                                       type_ :  u8,
                                       detail :  u8,
                                       time :  u32,
                                       root :  ffi::xproto::window,
                                       rootX :  i16,
                                       rootY :  i16,
                                       deviceid :  u8) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_test_fake_input (c : *mut ffi::base::connection,
                               type_ :  u8,
                               detail :  u8,
                               time :  u32,
                               root :  ffi::xproto::window,
                               rootX :  i16,
                               rootY :  i16,
                               deviceid :  u8) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_test_grab_control_checked (c : *mut ffi::base::connection,
                                         impervious :  u8) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_test_grab_control (c : *mut ffi::base::connection,
                                 impervious :  u8) -> ffi::base::void_cookie;
}

