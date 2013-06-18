/*
 * This file generated automatically from bigreq.xml by r_client.py.
 * Edit at your peril.
 */

//Make the compiler quiet
#[allow(unused_imports)];
#[allow(non_camel_case_types)];
use std;
use std::libc::*;
use std::{cast,num,ptr,str,libc};
use std::to_bytes::ToBytes;
use ll::base::*;
use ll;

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
pub unsafe fn xcb_big_requests_enable (c : *connection) -> enable_cookie;

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
pub unsafe fn xcb_big_requests_enable_unchecked (c : *connection) -> enable_cookie;

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
pub unsafe fn xcb_big_requests_enable_reply (c : *connection,
                                         cookie : enable_cookie,
                                         e : **generic_error) -> *enable_reply;
}

