/*
 * This file generated automatically from xc_misc.xml by r_client.py.
 * Edit at your peril.
 */

//Make the compiler quiet
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use std;
use libc::*;
use ffi;

pub static XCMISC_MAJOR_VERSION : c_uint = 1;
pub static XCMISC_MINOR_VERSION : c_uint = 1;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct get_version_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct get_version_request {
     pub major_opcode :           u8,
     pub minor_opcode :           u8,
     pub length :                 u16,
     pub client_major_version :   u16,
     pub client_minor_version :   u16
}

impl Copy for get_version_request {}
impl Clone for get_version_request {
    fn clone(&self) -> get_version_request { *self }
}

#[repr(C)]
pub struct get_version_reply {
     pub response_type :          u8,
     pub pad0 :                   u8,
     pub sequence :               u16,
     pub length :                 u32,
     pub server_major_version :   u16,
     pub server_minor_version :   u16
}

impl Copy for get_version_reply {}
impl Clone for get_version_reply {
    fn clone(&self) -> get_version_reply { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct get_xid_range_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct get_xid_range_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16
}

impl Copy for get_xid_range_request {}
impl Clone for get_xid_range_request {
    fn clone(&self) -> get_xid_range_request { *self }
}

#[repr(C)]
pub struct get_xid_range_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub start_id :        u32,
     pub count :           u32
}

impl Copy for get_xid_range_reply {}
impl Clone for get_xid_range_reply {
    fn clone(&self) -> get_xid_range_reply { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct get_xid_list_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct get_xid_list_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub count :          u32
}

impl Copy for get_xid_list_request {}
impl Clone for get_xid_list_request {
    fn clone(&self) -> get_xid_list_request { *self }
}

#[repr(C)]
pub struct get_xid_list_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub ids_len :         u32,
     pub pad1 :            [u8; 20]
}

impl Copy for get_xid_list_reply {}
impl Clone for get_xid_list_reply {
    fn clone(&self) -> get_xid_list_reply { *self }
}
extern "C" {

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_xc_misc_get_version (c : *mut ffi::base::connection,
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
pub fn xcb_xc_misc_get_version_unchecked (c : *mut ffi::base::connection,
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
pub fn xcb_xc_misc_get_version_reply (c : *mut ffi::base::connection,
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
pub fn xcb_xc_misc_get_xid_range (c : *mut ffi::base::connection) -> get_xid_range_cookie;

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
pub fn xcb_xc_misc_get_xid_range_unchecked (c : *mut ffi::base::connection) -> get_xid_range_cookie;

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
pub fn xcb_xc_misc_get_xid_range_reply (c : *mut ffi::base::connection,
                                           cookie : get_xid_range_cookie,
                                           e : *mut *mut ffi::base::generic_error) -> *mut get_xid_range_reply;

pub fn xcb_xc_misc_get_xid_list_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_xc_misc_get_xid_list (c : *mut ffi::base::connection,
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
pub fn xcb_xc_misc_get_xid_list_unchecked (c : *mut ffi::base::connection,
                                              count :  u32) -> get_xid_list_cookie;

pub fn xcb_xc_misc_get_xid_list_ids (R : *mut get_xid_list_reply) -> *mut u32;


pub fn xcb_xc_misc_get_xid_list_ids_length (R : *mut get_xid_list_reply) -> c_int;


pub fn xcb_xc_misc_get_xid_list_ids_end (R : *mut get_xid_list_reply) -> ffi::base::generic_iterator;

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
pub fn xcb_xc_misc_get_xid_list_reply (c : *mut ffi::base::connection,
                                          cookie : get_xid_list_cookie,
                                          e : *mut *mut ffi::base::generic_error) -> *mut get_xid_list_reply;
}

