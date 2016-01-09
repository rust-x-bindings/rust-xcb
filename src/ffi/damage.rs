/*
 * This file generated automatically from damage.xml by r_client.py.
 * Edit at your peril.
 */

//Make the compiler quiet
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use std;
use libc::*;
use ffi;
use ffi::xproto;
use ffi::render;
use ffi::shape;
use ffi::xfixes;

pub static DAMAGE_MAJOR_VERSION : c_uint = 1;
pub static DAMAGE_MINOR_VERSION : c_uint = 1;

pub type xcb_damage_damage_t = u32;
/**
 * @brief xcb_damage_damage_iterator_t
 **/
#[repr(C)]
pub struct xcb_damage_damage_iterator_t {
    pub data : *mut xcb_damage_damage_t,
    pub rem  : c_int,
    pub index: c_int
}



#[repr(C)]
pub struct xcb_damage_bad_damage_error_t {
     pub response_type :   u8,
     pub error_code :      u8,
     pub sequence :        u16
}

impl Copy for xcb_damage_bad_damage_error_t {}
impl Clone for xcb_damage_bad_damage_error_t {
    fn clone(&self) -> xcb_damage_bad_damage_error_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_damage_query_version_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_damage_query_version_request_t {
     pub major_opcode :           u8,
     pub minor_opcode :           u8,
     pub length :                 u16,
     pub client_major_version :   u32,
     pub client_minor_version :   u32
}

impl Copy for xcb_damage_query_version_request_t {}
impl Clone for xcb_damage_query_version_request_t {
    fn clone(&self) -> xcb_damage_query_version_request_t { *self }
}

#[repr(C)]
pub struct xcb_damage_query_version_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub major_version :   u32,
     pub minor_version :   u32,
     pub pad1 :            [u8; 16]
}

impl Copy for xcb_damage_query_version_reply_t {}
impl Clone for xcb_damage_query_version_reply_t {
    fn clone(&self) -> xcb_damage_query_version_reply_t { *self }
}


#[repr(C)]
pub struct xcb_damage_create_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub damage :         xcb_damage_damage_t,
     pub drawable :       ffi::xproto::xcb_drawable_t,
     pub level :          u8,
     pub pad0 :           [u8; 3]
}

impl Copy for xcb_damage_create_request_t {}
impl Clone for xcb_damage_create_request_t {
    fn clone(&self) -> xcb_damage_create_request_t { *self }
}


#[repr(C)]
pub struct xcb_damage_destroy_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub damage :         xcb_damage_damage_t
}

impl Copy for xcb_damage_destroy_request_t {}
impl Clone for xcb_damage_destroy_request_t {
    fn clone(&self) -> xcb_damage_destroy_request_t { *self }
}


#[repr(C)]
pub struct xcb_damage_subtract_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub damage :         xcb_damage_damage_t,
     pub repair :         ffi::xfixes::xcb_xfixes_region_t,
     pub parts :          ffi::xfixes::xcb_xfixes_region_t
}

impl Copy for xcb_damage_subtract_request_t {}
impl Clone for xcb_damage_subtract_request_t {
    fn clone(&self) -> xcb_damage_subtract_request_t { *self }
}


#[repr(C)]
pub struct xcb_damage_add_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub drawable :       ffi::xproto::xcb_drawable_t,
     pub region :         ffi::xfixes::xcb_xfixes_region_t
}

impl Copy for xcb_damage_add_request_t {}
impl Clone for xcb_damage_add_request_t {
    fn clone(&self) -> xcb_damage_add_request_t { *self }
}


#[repr(C)]
pub struct xcb_damage_notify_event_t {
     pub response_type :   u8,
     pub level :           u8,
     pub sequence :        u16,
     pub drawable :        ffi::xproto::xcb_drawable_t,
     pub damage :          xcb_damage_damage_t,
     pub timestamp :       ffi::xproto::xcb_timestamp_t,
     pub area :            ffi::xproto::xcb_rectangle_t,
     pub geometry :        ffi::xproto::xcb_rectangle_t
}

impl Copy for xcb_damage_notify_event_t {}
impl Clone for xcb_damage_notify_event_t {
    fn clone(&self) -> xcb_damage_notify_event_t { *self }
}
#[link(name="xcb-damage")]
extern "C" {

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_damage_damage_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_damage_damage_t)
 *
 *
 */
pub fn xcb_damage_damage_next (i:*mut xcb_damage_damage_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_damage_damage_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_damage_damage_end (i:xcb_damage_damage_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_damage_query_version (c : *mut ffi::base::xcb_connection_t,
                                    client_major_version :  u32,
                                    client_minor_version :  u32) -> xcb_damage_query_version_cookie_t;

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
pub fn xcb_damage_query_version_unchecked (c : *mut ffi::base::xcb_connection_t,
                                              client_major_version :  u32,
                                              client_minor_version :  u32) -> xcb_damage_query_version_cookie_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_damage_query_version_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_damage_query_version_reply (c : *mut ffi::base::xcb_connection_t,
                                          cookie : xcb_damage_query_version_cookie_t,
                                          e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_damage_query_version_reply_t;

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
pub fn xcb_damage_create_checked (c : *mut ffi::base::xcb_connection_t,
                                     damage :  xcb_damage_damage_t,
                                     drawable :  ffi::xproto::xcb_drawable_t,
                                     level :  u8) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_damage_create (c : *mut ffi::base::xcb_connection_t,
                             damage :  xcb_damage_damage_t,
                             drawable :  ffi::xproto::xcb_drawable_t,
                             level :  u8) -> ffi::base::xcb_void_cookie_t;

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
pub fn xcb_damage_destroy_checked (c : *mut ffi::base::xcb_connection_t,
                                      damage :  xcb_damage_damage_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_damage_destroy (c : *mut ffi::base::xcb_connection_t,
                              damage :  xcb_damage_damage_t) -> ffi::base::xcb_void_cookie_t;

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
pub fn xcb_damage_subtract_checked (c : *mut ffi::base::xcb_connection_t,
                                       damage :  xcb_damage_damage_t,
                                       repair :  ffi::xfixes::xcb_xfixes_region_t,
                                       parts :  ffi::xfixes::xcb_xfixes_region_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_damage_subtract (c : *mut ffi::base::xcb_connection_t,
                               damage :  xcb_damage_damage_t,
                               repair :  ffi::xfixes::xcb_xfixes_region_t,
                               parts :  ffi::xfixes::xcb_xfixes_region_t) -> ffi::base::xcb_void_cookie_t;

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
pub fn xcb_damage_add_checked (c : *mut ffi::base::xcb_connection_t,
                                  drawable :  ffi::xproto::xcb_drawable_t,
                                  region :  ffi::xfixes::xcb_xfixes_region_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_damage_add (c : *mut ffi::base::xcb_connection_t,
                          drawable :  ffi::xproto::xcb_drawable_t,
                          region :  ffi::xfixes::xcb_xfixes_region_t) -> ffi::base::xcb_void_cookie_t;
}

