/*
 * This file generated automatically from xtest.xml by r_client.py.
 * Edit at your peril.
 */

//Make the compiler quiet
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use std;
use libc::*;
use std::{mem,num,ptr,str};
use ffi::base::*;
use ffi;
use ffi::xproto;

pub static TEST_MAJOR_VERSION : c_uint = 2;
pub static TEST_MINOR_VERSION : c_uint = 1;

pub struct get_version_cookie {
    sequence : c_uint
}


pub struct get_version_request {
    major_opcode :    u8,
    minor_opcode :    u8,
    length :          u16,
    major_version :   u8,
    pad0 :            u8,
    minor_version :   u16
}


pub struct get_version_reply {
    response_type :   u8,
    major_version :   u8,
    sequence :        u16,
    length :          u32,
    minor_version :   u16
}


pub struct compare_cursor_cookie {
    sequence : c_uint
}


pub struct compare_cursor_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    window :         ffi::xproto::window,
    cursor :         ffi::xproto::cursor
}


pub struct compare_cursor_reply {
    response_type :   u8,
    same :            u8,
    sequence :        u16,
    length :          u32
}



pub struct fake_input_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    type_ :          u8,
    detail :         u8,
    pad0 :           [u8,..2],
    time :           u32,
    root :           ffi::xproto::window,
    pad1 :           [u8,..8],
    rootX :          i16,
    rootY :          i16,
    pad2 :           [u8,..7],
    deviceid :       u8
}



pub struct grab_control_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    impervious :     u8,
    pad0 :           [u8,..3]
}

#[link(name="lxcb-xtest")]
pub extern "C" {

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_test_get_version (c : *mut connection,
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
pub fn xcb_test_get_version_unchecked (c : *mut connection,
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
pub fn xcb_test_get_version_reply (c : *mut connection,
                                      cookie : get_version_cookie,
                                      e : *mut *mut generic_error) -> *mut get_version_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_test_compare_cursor (c : *mut connection,
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
pub fn xcb_test_compare_cursor_unchecked (c : *mut connection,
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
pub fn xcb_test_compare_cursor_reply (c : *mut connection,
                                         cookie : compare_cursor_cookie,
                                         e : *mut *mut generic_error) -> *mut compare_cursor_reply;

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
pub fn xcb_test_fake_input_checked (c : *mut connection,
                                       type_ :  u8,
                                       detail :  u8,
                                       time :  u32,
                                       root :  ffi::xproto::window,
                                       rootX :  i16,
                                       rootY :  i16,
                                       deviceid :  u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_test_fake_input (c : *mut connection,
                               type_ :  u8,
                               detail :  u8,
                               time :  u32,
                               root :  ffi::xproto::window,
                               rootX :  i16,
                               rootY :  i16,
                               deviceid :  u8) -> void_cookie;

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
pub fn xcb_test_grab_control_checked (c : *mut connection,
                                         impervious :  u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_test_grab_control (c : *mut connection,
                                 impervious :  u8) -> void_cookie;
}

