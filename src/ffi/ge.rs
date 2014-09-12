/*
 * This file generated automatically from ge.xml by r_client.py.
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

pub static GENERICEVENT_MAJOR_VERSION : c_uint = 1;
pub static GENERICEVENT_MINOR_VERSION : c_uint = 0;

pub struct query_version_cookie {
    sequence : c_uint
}


pub struct query_version_request {
    major_opcode :           u8,
    minor_opcode :           u8,
    length :                 u16,
    client_major_version :   u16,
    client_minor_version :   u16
}


pub struct query_version_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    major_version :   u16,
    minor_version :   u16,
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
pub fn xcb_genericevent_query_version (c : *mut connection,
                                          client_major_version :  u16,
                                          client_minor_version :  u16) -> query_version_cookie;

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
pub fn xcb_genericevent_query_version_unchecked (c : *mut connection,
                                                    client_major_version :  u16,
                                                    client_minor_version :  u16) -> query_version_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_genericevent_query_version_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_genericevent_query_version_reply (c : *mut connection,
                                                cookie : query_version_cookie,
                                                e : *mut *mut generic_error) -> *mut query_version_reply;
}

