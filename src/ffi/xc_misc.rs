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
pub struct xcb_xc_misc_get_version_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_xc_misc_get_version_request_t {
     pub major_opcode :           u8,
     pub minor_opcode :           u8,
     pub length :                 u16,
     pub client_major_version :   u16,
     pub client_minor_version :   u16
}

impl Copy for xcb_xc_misc_get_version_request_t {}
impl Clone for xcb_xc_misc_get_version_request_t {
    fn clone(&self) -> xcb_xc_misc_get_version_request_t { *self }
}

#[repr(C)]
pub struct xcb_xc_misc_get_version_reply_t {
     pub response_type :          u8,
     pub pad0 :                   u8,
     pub sequence :               u16,
     pub length :                 u32,
     pub server_major_version :   u16,
     pub server_minor_version :   u16
}

impl Copy for xcb_xc_misc_get_version_reply_t {}
impl Clone for xcb_xc_misc_get_version_reply_t {
    fn clone(&self) -> xcb_xc_misc_get_version_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xc_misc_get_xid_range_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_xc_misc_get_xid_range_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16
}

impl Copy for xcb_xc_misc_get_xid_range_request_t {}
impl Clone for xcb_xc_misc_get_xid_range_request_t {
    fn clone(&self) -> xcb_xc_misc_get_xid_range_request_t { *self }
}

#[repr(C)]
pub struct xcb_xc_misc_get_xid_range_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub start_id :        u32,
     pub count :           u32
}

impl Copy for xcb_xc_misc_get_xid_range_reply_t {}
impl Clone for xcb_xc_misc_get_xid_range_reply_t {
    fn clone(&self) -> xcb_xc_misc_get_xid_range_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xc_misc_get_xid_list_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_xc_misc_get_xid_list_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub count :          u32
}

impl Copy for xcb_xc_misc_get_xid_list_request_t {}
impl Clone for xcb_xc_misc_get_xid_list_request_t {
    fn clone(&self) -> xcb_xc_misc_get_xid_list_request_t { *self }
}

#[repr(C)]
pub struct xcb_xc_misc_get_xid_list_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub ids_len :         u32,
     pub pad1 :            [u8; 20]
}

impl Copy for xcb_xc_misc_get_xid_list_reply_t {}
impl Clone for xcb_xc_misc_get_xid_list_reply_t {
    fn clone(&self) -> xcb_xc_misc_get_xid_list_reply_t { *self }
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
pub fn xcb_xc_misc_get_version (c : *mut ffi::base::xcb_connection_t,
                                   client_major_version :  u16,
                                   client_minor_version :  u16) -> xcb_xc_misc_get_version_cookie_t;

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
pub fn xcb_xc_misc_get_version_unchecked (c : *mut ffi::base::xcb_connection_t,
                                             client_major_version :  u16,
                                             client_minor_version :  u16) -> xcb_xc_misc_get_version_cookie_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xc_misc_get_version_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_xc_misc_get_version_reply (c : *mut ffi::base::xcb_connection_t,
                                         cookie : xcb_xc_misc_get_version_cookie_t,
                                         e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_xc_misc_get_version_reply_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_xc_misc_get_xid_range (c : *mut ffi::base::xcb_connection_t) -> xcb_xc_misc_get_xid_range_cookie_t;

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
pub fn xcb_xc_misc_get_xid_range_unchecked (c : *mut ffi::base::xcb_connection_t) -> xcb_xc_misc_get_xid_range_cookie_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xc_misc_get_xid_range_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_xc_misc_get_xid_range_reply (c : *mut ffi::base::xcb_connection_t,
                                           cookie : xcb_xc_misc_get_xid_range_cookie_t,
                                           e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_xc_misc_get_xid_range_reply_t;

pub fn xcb_xc_misc_get_xid_list_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_xc_misc_get_xid_list (c : *mut ffi::base::xcb_connection_t,
                                    count :  u32) -> xcb_xc_misc_get_xid_list_cookie_t;

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
pub fn xcb_xc_misc_get_xid_list_unchecked (c : *mut ffi::base::xcb_connection_t,
                                              count :  u32) -> xcb_xc_misc_get_xid_list_cookie_t;

pub fn xcb_xc_misc_get_xid_list_ids (R : *mut xcb_xc_misc_get_xid_list_reply_t) -> *mut u32;


pub fn xcb_xc_misc_get_xid_list_ids_length (R : *mut xcb_xc_misc_get_xid_list_reply_t) -> c_int;


pub fn xcb_xc_misc_get_xid_list_ids_end (R : *mut xcb_xc_misc_get_xid_list_reply_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xc_misc_get_xid_list_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_xc_misc_get_xid_list_reply (c : *mut ffi::base::xcb_connection_t,
                                          cookie : xcb_xc_misc_get_xid_list_cookie_t,
                                          e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_xc_misc_get_xid_list_reply_t;
}

