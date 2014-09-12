/*
 * This file generated automatically from dpms.xml by r_client.py.
 * Edit at your peril.
 */

//Make the compiler quiet
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use std;
use std::libc::*;
use std::{cast,num,ptr,str,libc};
use std::to_bytes::ToBytes;
use ffi::base::*;
use ffi;

pub static DPMS_MAJOR_VERSION : c_uint = 0;
pub static DPMS_MINOR_VERSION : c_uint = 0;

pub struct get_version_cookie {
    sequence : c_uint
}


pub struct get_version_request {
    major_opcode :           u8,
    minor_opcode :           u8,
    length :                 u16,
    client_major_version :   u16,
    client_minor_version :   u16
}


pub struct get_version_reply {
    response_type :          u8,
    pad0 :                   u8,
    sequence :               u16,
    length :                 u32,
    server_major_version :   u16,
    server_minor_version :   u16
}


pub struct capable_cookie {
    sequence : c_uint
}


pub struct capable_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16
}


pub struct capable_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    capable :         u8,
    pad1 :            [u8,..23]
}


pub struct get_timeouts_cookie {
    sequence : c_uint
}


pub struct get_timeouts_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16
}


pub struct get_timeouts_reply {
    response_type :     u8,
    pad0 :              u8,
    sequence :          u16,
    length :            u32,
    standby_timeout :   u16,
    suspend_timeout :   u16,
    off_timeout :       u16,
    pad1 :              [u8,..18]
}



pub struct set_timeouts_request {
    major_opcode :      u8,
    minor_opcode :      u8,
    length :            u16,
    standby_timeout :   u16,
    suspend_timeout :   u16,
    off_timeout :       u16
}



pub struct enable_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16
}



pub struct disable_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16
}



pub struct force_level_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    power_level :    u16
}


pub struct info_cookie {
    sequence : c_uint
}


pub struct info_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16
}


pub struct info_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    power_level :     u16,
    state :           u8,
    pad1 :            [u8,..21]
}

#[link(name="lxcb-dpms")]
pub extern "C" {

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_dpms_get_version (c : *mut connection,
                                client_major_version :  u16,
                                client_minor_version :  u16) -> get_version_cookie;

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
pub fn xcb_dpms_get_version_unchecked (c : *mut connection,
                                          client_major_version :  u16,
                                          client_minor_version :  u16) -> get_version_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_dpms_get_version_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_dpms_get_version_reply (c : *mut connection,
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
pub fn xcb_dpms_capable (c : *mut connection) -> capable_cookie;

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
pub fn xcb_dpms_capable_unchecked (c : *mut connection) -> capable_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_dpms_capable_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_dpms_capable_reply (c : *mut connection,
                                  cookie : capable_cookie,
                                  e : *mut *mut generic_error) -> *mut capable_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_dpms_get_timeouts (c : *mut connection) -> get_timeouts_cookie;

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
pub fn xcb_dpms_get_timeouts_unchecked (c : *mut connection) -> get_timeouts_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_dpms_get_timeouts_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_dpms_get_timeouts_reply (c : *mut connection,
                                       cookie : get_timeouts_cookie,
                                       e : *mut *mut generic_error) -> *mut get_timeouts_reply;

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
pub fn xcb_dpms_set_timeouts_checked (c : *mut connection,
                                         standby_timeout :  u16,
                                         suspend_timeout :  u16,
                                         off_timeout :  u16) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_dpms_set_timeouts (c : *mut connection,
                                 standby_timeout :  u16,
                                 suspend_timeout :  u16,
                                 off_timeout :  u16) -> void_cookie;

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
pub fn xcb_dpms_enable_checked (c : *mut connection) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_dpms_enable (c : *mut connection) -> void_cookie;

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
pub fn xcb_dpms_disable_checked (c : *mut connection) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_dpms_disable (c : *mut connection) -> void_cookie;

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
pub fn xcb_dpms_force_level_checked (c : *mut connection,
                                        power_level :  u16) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_dpms_force_level (c : *mut connection,
                                power_level :  u16) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_dpms_info (c : *mut connection) -> info_cookie;

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
pub fn xcb_dpms_info_unchecked (c : *mut connection) -> info_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_dpms_info_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_dpms_info_reply (c : *mut connection,
                               cookie : info_cookie,
                               e : *mut *mut generic_error) -> *mut info_reply;
}

