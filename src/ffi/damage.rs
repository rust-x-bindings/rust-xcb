/*
 * This file generated automatically from damage.xml by r_client.py.
 * Edit at your peril.
 */

//Make the compiler quiet
#[allow(unused_imports)];
#[allow(non_camel_case_types)];
use std;
use std::libc::*;
use std::{cast,num,ptr,str,libc};
use std::to_bytes::ToBytes;
use ffi::base::*;
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
pub struct damage_iterator {
    data : *damage,
    rem  : c_int,
    index: c_int
}



pub struct bad_damage_error {
    response_type :   u8,
    error_code :      u8,
    sequence :        u16
}


pub struct query_version_cookie {
    sequence : c_uint
}


pub struct query_version_request {
    major_opcode :           u8,
    minor_opcode :           u8,
    length :                 u16,
    client_major_version :   u32,
    client_minor_version :   u32
}


pub struct query_version_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    major_version :   u32,
    minor_version :   u32,
    pad1 :            [u8,..16]
}



pub struct create_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    damage :         damage,
    drawable :       ffi::xproto::drawable,
    level :          u8,
    pad0 :           [u8,..3]
}



pub struct destroy_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    damage :         damage
}



pub struct subtract_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    damage :         damage,
    repair :         ffi::xfixes::region,
    parts :          ffi::xfixes::region
}



pub struct add_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    drawable :       ffi::xproto::drawable,
    region :         ffi::xfixes::region
}



pub struct notify_event {
    response_type :   u8,
    level :           u8,
    sequence :        u16,
    drawable :        ffi::xproto::drawable,
    damage :          damage,
    timestamp :       ffi::xproto::timestamp,
    area :            ffi::xproto::rectangle,
    geometry :        ffi::xproto::rectangle
}

#[link_args="-lxcb-damage"]
pub extern "C" {

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
pub unsafe fn xcb_damage_damage_next (i:*damage_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An damage_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub unsafe fn xcb_damage_damage_end (i:damage_iterator) -> generic_iterator;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_damage_query_version (c : *connection,
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
pub unsafe fn xcb_damage_query_version_unchecked (c : *connection,
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
pub unsafe fn xcb_damage_query_version_reply (c : *connection,
                                          cookie : query_version_cookie,
                                          e : **generic_error) -> *query_version_reply;

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
pub unsafe fn xcb_damage_create_checked (c : *connection,
                                     damage :  damage,
                                     drawable :  ffi::xproto::drawable,
                                     level :  u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_damage_create (c : *connection,
                             damage :  damage,
                             drawable :  ffi::xproto::drawable,
                             level :  u8) -> void_cookie;

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
pub unsafe fn xcb_damage_destroy_checked (c : *connection,
                                      damage :  damage) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_damage_destroy (c : *connection,
                              damage :  damage) -> void_cookie;

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
pub unsafe fn xcb_damage_subtract_checked (c : *connection,
                                       damage :  damage,
                                       repair :  ffi::xfixes::region,
                                       parts :  ffi::xfixes::region) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_damage_subtract (c : *connection,
                               damage :  damage,
                               repair :  ffi::xfixes::region,
                               parts :  ffi::xfixes::region) -> void_cookie;

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
pub unsafe fn xcb_damage_add_checked (c : *connection,
                                  drawable :  ffi::xproto::drawable,
                                  region :  ffi::xfixes::region) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_damage_add (c : *connection,
                          drawable :  ffi::xproto::drawable,
                          region :  ffi::xfixes::region) -> void_cookie;
}

