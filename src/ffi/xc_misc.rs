/*
 * This file generated automatically from xc_misc.xml by r_client.py.
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

pub static XCMISC_MAJOR_VERSION : c_uint = 1;
pub static XCMISC_MINOR_VERSION : c_uint = 1;

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


pub struct get_xid_range_cookie {
    sequence : c_uint
}


pub struct get_xid_range_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16
}


pub struct get_xid_range_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    start_id :        u32,
    count :           u32
}


pub struct get_xid_list_cookie {
    sequence : c_uint
}


pub struct get_xid_list_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    count :          u32
}


pub struct get_xid_list_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    ids_len :         u32,
    pad1 :            [u8,..20]
}

pub extern "C" {

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xc_misc_get_version (c : *mut connection,
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
pub fn xcb_xc_misc_get_version_unchecked (c : *mut connection,
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
 * xcb_xc_misc_get_version_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_xc_misc_get_version_reply (c : *mut connection,
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
pub fn xcb_xc_misc_get_xid_range (c : *mut connection) -> get_xid_range_cookie;

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
pub fn xcb_xc_misc_get_xid_range_unchecked (c : *mut connection) -> get_xid_range_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xc_misc_get_xid_range_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_xc_misc_get_xid_range_reply (c : *mut connection,
                                           cookie : get_xid_range_cookie,
                                           e : *mut *mut generic_error) -> *mut get_xid_range_reply;

pub fn xcb_xc_misc_get_xid_list_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xc_misc_get_xid_list (c : *mut connection,
                                    count :  u32) -> get_xid_list_cookie;

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
pub fn xcb_xc_misc_get_xid_list_unchecked (c : *mut connection,
                                              count :  u32) -> get_xid_list_cookie;

pub fn xcb_xc_misc_get_xid_list_ids (R : *mut get_xid_list_reply) -> *mut u32;


pub fn xcb_xc_misc_get_xid_list_ids_length (R : *mut get_xid_list_reply) -> c_int;


pub fn xcb_xc_misc_get_xid_list_ids_end (R : *mut get_xid_list_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xc_misc_get_xid_list_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_xc_misc_get_xid_list_reply (c : *mut connection,
                                          cookie : get_xid_list_cookie,
                                          e : *mut *mut generic_error) -> *mut get_xid_list_reply;
}

