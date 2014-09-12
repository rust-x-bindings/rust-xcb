/*
 * This file generated automatically from bigreq.xml by r_client.py.
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

pub static BIGREQUESTS_MAJOR_VERSION : c_uint = 0;
pub static BIGREQUESTS_MINOR_VERSION : c_uint = 0;

pub struct enable_cookie {
    sequence : c_uint
}


pub struct enable_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16
}


pub struct enable_reply {
    response_type :            u8,
    pad0 :                     u8,
    sequence :                 u16,
    length :                   u32,
    maximum_request_length :   u32
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
pub fn xcb_big_requests_enable (c : *mut connection) -> enable_cookie;

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
pub fn xcb_big_requests_enable_unchecked (c : *mut connection) -> enable_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_big_requests_enable_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_big_requests_enable_reply (c : *mut connection,
                                         cookie : enable_cookie,
                                         e : *mut *mut generic_error) -> *mut enable_reply;
}

