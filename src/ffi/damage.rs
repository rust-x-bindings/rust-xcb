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

pub type damage = u32;
/**
 * @brief damage_iterator
 **/
#[repr(C)]
pub struct damage_iterator {
    pub data : *mut damage,
    pub rem  : c_int,
    pub index: c_int
}



#[repr(C)]
pub struct bad_damage_error {
     pub response_type :   u8,
     pub error_code :      u8,
     pub sequence :        u16
}

impl Copy for bad_damage_error {}
impl Clone for bad_damage_error {
    fn clone(&self) -> bad_damage_error { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct query_version_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct query_version_request {
     pub major_opcode :           u8,
     pub minor_opcode :           u8,
     pub length :                 u16,
     pub client_major_version :   u32,
     pub client_minor_version :   u32
}

impl Copy for query_version_request {}
impl Clone for query_version_request {
    fn clone(&self) -> query_version_request { *self }
}

#[repr(C)]
pub struct query_version_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub major_version :   u32,
     pub minor_version :   u32,
     pub pad1 :            [u8; 16]
}

impl Copy for query_version_reply {}
impl Clone for query_version_reply {
    fn clone(&self) -> query_version_reply { *self }
}


#[repr(C)]
pub struct create_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub damage :         damage,
     pub drawable :       ffi::xproto::drawable,
     pub level :          u8,
     pub pad0 :           [u8; 3]
}

impl Copy for create_request {}
impl Clone for create_request {
    fn clone(&self) -> create_request { *self }
}


#[repr(C)]
pub struct destroy_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub damage :         damage
}

impl Copy for destroy_request {}
impl Clone for destroy_request {
    fn clone(&self) -> destroy_request { *self }
}


#[repr(C)]
pub struct subtract_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub damage :         damage,
     pub repair :         ffi::xfixes::region,
     pub parts :          ffi::xfixes::region
}

impl Copy for subtract_request {}
impl Clone for subtract_request {
    fn clone(&self) -> subtract_request { *self }
}


#[repr(C)]
pub struct add_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub drawable :       ffi::xproto::drawable,
     pub region :         ffi::xfixes::region
}

impl Copy for add_request {}
impl Clone for add_request {
    fn clone(&self) -> add_request { *self }
}


#[repr(C)]
pub struct notify_event {
     pub response_type :   u8,
     pub level :           u8,
     pub sequence :        u16,
     pub drawable :        ffi::xproto::drawable,
     pub damage :          damage,
     pub timestamp :       ffi::xproto::timestamp,
     pub area :            ffi::xproto::rectangle,
     pub geometry :        ffi::xproto::rectangle
}

impl Copy for notify_event {}
impl Clone for notify_event {
    fn clone(&self) -> notify_event { *self }
}
#[link(name="xcb-damage")]
extern "C" {

/**
 * Get the next element of the iterator
 * @param i Pointer to a damage_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(damage)
 *
 *
 */
pub fn xcb_damage_damage_next (i:*mut damage_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An damage_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_damage_damage_end (i:damage_iterator) -> ffi::base::generic_iterator;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_damage_query_version (c : *mut ffi::base::connection,
                                    client_major_version :  u32,
                                    client_minor_version :  u32) -> query_version_cookie;

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
pub fn xcb_damage_query_version_unchecked (c : *mut ffi::base::connection,
                                              client_major_version :  u32,
                                              client_minor_version :  u32) -> query_version_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_damage_query_version_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_damage_query_version_reply (c : *mut ffi::base::connection,
                                          cookie : query_version_cookie,
                                          e : *mut *mut ffi::base::generic_error) -> *mut query_version_reply;

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
pub fn xcb_damage_create_checked (c : *mut ffi::base::connection,
                                     damage :  damage,
                                     drawable :  ffi::xproto::drawable,
                                     level :  u8) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_damage_create (c : *mut ffi::base::connection,
                             damage :  damage,
                             drawable :  ffi::xproto::drawable,
                             level :  u8) -> ffi::base::void_cookie;

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
pub fn xcb_damage_destroy_checked (c : *mut ffi::base::connection,
                                      damage :  damage) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_damage_destroy (c : *mut ffi::base::connection,
                              damage :  damage) -> ffi::base::void_cookie;

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
pub fn xcb_damage_subtract_checked (c : *mut ffi::base::connection,
                                       damage :  damage,
                                       repair :  ffi::xfixes::region,
                                       parts :  ffi::xfixes::region) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_damage_subtract (c : *mut ffi::base::connection,
                               damage :  damage,
                               repair :  ffi::xfixes::region,
                               parts :  ffi::xfixes::region) -> ffi::base::void_cookie;

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
pub fn xcb_damage_add_checked (c : *mut ffi::base::connection,
                                  drawable :  ffi::xproto::drawable,
                                  region :  ffi::xfixes::region) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_damage_add (c : *mut ffi::base::connection,
                          drawable :  ffi::xproto::drawable,
                          region :  ffi::xfixes::region) -> ffi::base::void_cookie;
}

