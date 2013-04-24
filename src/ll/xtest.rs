/*
 * This file generated automatically from xtest.xml by r_client.py.
 * Edit at your peril.
 */

//Make the compiler quiet
#[allow(unused_imports)];
#[allow(non_camel_case_types)];
use core;
use core::libc::*;
use ll::base::*;
use ll;
use ll::xproto;

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
    window :         ll::xproto::window,
    cursor :         ll::xproto::cursor
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
    root :           ll::xproto::window,
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

#[link_args="-lxcb-xtest"]
pub extern "C" {

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_test_get_version (c : *connection,
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
unsafe fn xcb_test_get_version_unchecked (c : *connection,
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
unsafe fn xcb_test_get_version_reply (c : *connection,
                                      cookie : get_version_cookie,
                                      e : **generic_error) -> *get_version_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_test_compare_cursor (c : *connection,
                                   window :  ll::xproto::window,
                                   cursor :  ll::xproto::cursor) -> compare_cursor_cookie;

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
unsafe fn xcb_test_compare_cursor_unchecked (c : *connection,
                                             window :  ll::xproto::window,
                                             cursor :  ll::xproto::cursor) -> compare_cursor_cookie;

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
unsafe fn xcb_test_compare_cursor_reply (c : *connection,
                                         cookie : compare_cursor_cookie,
                                         e : **generic_error) -> *compare_cursor_reply;

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
unsafe fn xcb_test_fake_input_checked (c : *connection,
                                       type_ :  u8,
                                       detail :  u8,
                                       time :  u32,
                                       root :  ll::xproto::window,
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
unsafe fn xcb_test_fake_input (c : *connection,
                               type_ :  u8,
                               detail :  u8,
                               time :  u32,
                               root :  ll::xproto::window,
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
unsafe fn xcb_test_grab_control_checked (c : *connection,
                                         impervious :  u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_test_grab_control (c : *connection,
                                 impervious :  u8) -> void_cookie;
}

