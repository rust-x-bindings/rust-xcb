/*
 * This file generated automatically from xselinux.xml by r_client.py.
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
use ffi::xproto;

pub static SELINUX_MAJOR_VERSION : c_uint = 1;
pub static SELINUX_MINOR_VERSION : c_uint = 0;

pub struct query_version_cookie {
    sequence : c_uint
}


pub struct query_version_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    client_major :   u8,
    client_minor :   u8
}


pub struct query_version_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    server_major :    u16,
    server_minor :    u16
}



pub struct set_device_create_context_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_len :    u32
}


pub struct get_device_create_context_cookie {
    sequence : c_uint
}


pub struct get_device_create_context_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16
}


pub struct get_device_create_context_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    context_len :     u32,
    pad1 :            [u8,..20]
}



pub struct set_device_context_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    device :         u32,
    context_len :    u32
}


pub struct get_device_context_cookie {
    sequence : c_uint
}


pub struct get_device_context_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    device :         u32
}


pub struct get_device_context_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    context_len :     u32,
    pad1 :            [u8,..20]
}



pub struct set_window_create_context_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_len :    u32
}


pub struct get_window_create_context_cookie {
    sequence : c_uint
}


pub struct get_window_create_context_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16
}


pub struct get_window_create_context_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    context_len :     u32,
    pad1 :            [u8,..20]
}


pub struct get_window_context_cookie {
    sequence : c_uint
}


pub struct get_window_context_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    window :         ffi::xproto::window
}


pub struct get_window_context_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    context_len :     u32,
    pad1 :            [u8,..20]
}


pub struct list_item {
    name :                 ffi::xproto::atom,
    object_context_len :   u32,
    data_context_len :     u32
}

/**
 * @brief list_item_iterator
 **/
pub struct list_item_iterator {
    data : *mut list_item,
    rem  : c_int,
    index: c_int
}



pub struct set_property_create_context_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_len :    u32
}


pub struct get_property_create_context_cookie {
    sequence : c_uint
}


pub struct get_property_create_context_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16
}


pub struct get_property_create_context_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    context_len :     u32,
    pad1 :            [u8,..20]
}



pub struct set_property_use_context_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_len :    u32
}


pub struct get_property_use_context_cookie {
    sequence : c_uint
}


pub struct get_property_use_context_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16
}


pub struct get_property_use_context_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    context_len :     u32,
    pad1 :            [u8,..20]
}


pub struct get_property_context_cookie {
    sequence : c_uint
}


pub struct get_property_context_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    window :         ffi::xproto::window,
    property :       ffi::xproto::atom
}


pub struct get_property_context_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    context_len :     u32,
    pad1 :            [u8,..20]
}


pub struct get_property_data_context_cookie {
    sequence : c_uint
}


pub struct get_property_data_context_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    window :         ffi::xproto::window,
    property :       ffi::xproto::atom
}


pub struct get_property_data_context_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    context_len :     u32,
    pad1 :            [u8,..20]
}


pub struct list_properties_cookie {
    sequence : c_uint
}


pub struct list_properties_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    window :         ffi::xproto::window
}


pub struct list_properties_reply {
    response_type :    u8,
    pad0 :             u8,
    sequence :         u16,
    length :           u32,
    properties_len :   u32,
    pad1 :             [u8,..20]
}



pub struct set_selection_create_context_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_len :    u32
}


pub struct get_selection_create_context_cookie {
    sequence : c_uint
}


pub struct get_selection_create_context_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16
}


pub struct get_selection_create_context_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    context_len :     u32,
    pad1 :            [u8,..20]
}



pub struct set_selection_use_context_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_len :    u32
}


pub struct get_selection_use_context_cookie {
    sequence : c_uint
}


pub struct get_selection_use_context_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16
}


pub struct get_selection_use_context_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    context_len :     u32,
    pad1 :            [u8,..20]
}


pub struct get_selection_context_cookie {
    sequence : c_uint
}


pub struct get_selection_context_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    selection :      ffi::xproto::atom
}


pub struct get_selection_context_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    context_len :     u32,
    pad1 :            [u8,..20]
}


pub struct get_selection_data_context_cookie {
    sequence : c_uint
}


pub struct get_selection_data_context_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    selection :      ffi::xproto::atom
}


pub struct get_selection_data_context_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    context_len :     u32,
    pad1 :            [u8,..20]
}


pub struct list_selections_cookie {
    sequence : c_uint
}


pub struct list_selections_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16
}


pub struct list_selections_reply {
    response_type :    u8,
    pad0 :             u8,
    sequence :         u16,
    length :           u32,
    selections_len :   u32,
    pad1 :             [u8,..20]
}


pub struct get_client_context_cookie {
    sequence : c_uint
}


pub struct get_client_context_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    resource :       u32
}


pub struct get_client_context_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    context_len :     u32,
    pad1 :            [u8,..20]
}

#[link(name="lxcb-xselinux")]
pub extern "C" {

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_selinux_query_version (c : *mut connection,
                                     client_major :  u8,
                                     client_minor :  u8) -> query_version_cookie;

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
pub fn xcb_selinux_query_version_unchecked (c : *mut connection,
                                               client_major :  u8,
                                               client_minor :  u8) -> query_version_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_selinux_query_version_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_selinux_query_version_reply (c : *mut connection,
                                           cookie : query_version_cookie,
                                           e : *mut *mut generic_error) -> *mut query_version_reply;

pub fn xcb_selinux_set_device_create_context_sizeof (_buffer :  *mut c_void) -> c_int;

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
pub fn xcb_selinux_set_device_create_context_checked (c : *mut connection,
                                                         context_len :  u32,
                                                         context : *mut c_char) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_selinux_set_device_create_context (c : *mut connection,
                                                 context_len :  u32,
                                                 context : *mut c_char) -> void_cookie;

pub fn xcb_selinux_get_device_create_context_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_selinux_get_device_create_context (c : *mut connection) -> get_device_create_context_cookie;

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
pub fn xcb_selinux_get_device_create_context_unchecked (c : *mut connection) -> get_device_create_context_cookie;

pub fn xcb_selinux_get_device_create_context_context (R : *mut get_device_create_context_reply) -> *mut c_char;


pub fn xcb_selinux_get_device_create_context_context_length (R : *mut get_device_create_context_reply) -> c_int;


pub fn xcb_selinux_get_device_create_context_context_end (R : *mut get_device_create_context_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_selinux_get_device_create_context_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_selinux_get_device_create_context_reply (c : *mut connection,
                                                       cookie : get_device_create_context_cookie,
                                                       e : *mut *mut generic_error) -> *mut get_device_create_context_reply;

pub fn xcb_selinux_set_device_context_sizeof (_buffer :  *mut c_void) -> c_int;

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
pub fn xcb_selinux_set_device_context_checked (c : *mut connection,
                                                  device :  u32,
                                                  context_len :  u32,
                                                  context : *mut c_char) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_selinux_set_device_context (c : *mut connection,
                                          device :  u32,
                                          context_len :  u32,
                                          context : *mut c_char) -> void_cookie;

pub fn xcb_selinux_get_device_context_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_selinux_get_device_context (c : *mut connection,
                                          device :  u32) -> get_device_context_cookie;

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
pub fn xcb_selinux_get_device_context_unchecked (c : *mut connection,
                                                    device :  u32) -> get_device_context_cookie;

pub fn xcb_selinux_get_device_context_context (R : *mut get_device_context_reply) -> *mut c_char;


pub fn xcb_selinux_get_device_context_context_length (R : *mut get_device_context_reply) -> c_int;


pub fn xcb_selinux_get_device_context_context_end (R : *mut get_device_context_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_selinux_get_device_context_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_selinux_get_device_context_reply (c : *mut connection,
                                                cookie : get_device_context_cookie,
                                                e : *mut *mut generic_error) -> *mut get_device_context_reply;

pub fn xcb_selinux_set_window_create_context_sizeof (_buffer :  *mut c_void) -> c_int;

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
pub fn xcb_selinux_set_window_create_context_checked (c : *mut connection,
                                                         context_len :  u32,
                                                         context : *mut c_char) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_selinux_set_window_create_context (c : *mut connection,
                                                 context_len :  u32,
                                                 context : *mut c_char) -> void_cookie;

pub fn xcb_selinux_get_window_create_context_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_selinux_get_window_create_context (c : *mut connection) -> get_window_create_context_cookie;

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
pub fn xcb_selinux_get_window_create_context_unchecked (c : *mut connection) -> get_window_create_context_cookie;

pub fn xcb_selinux_get_window_create_context_context (R : *mut get_window_create_context_reply) -> *mut c_char;


pub fn xcb_selinux_get_window_create_context_context_length (R : *mut get_window_create_context_reply) -> c_int;


pub fn xcb_selinux_get_window_create_context_context_end (R : *mut get_window_create_context_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_selinux_get_window_create_context_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_selinux_get_window_create_context_reply (c : *mut connection,
                                                       cookie : get_window_create_context_cookie,
                                                       e : *mut *mut generic_error) -> *mut get_window_create_context_reply;

pub fn xcb_selinux_get_window_context_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_selinux_get_window_context (c : *mut connection,
                                          window :  ffi::xproto::window) -> get_window_context_cookie;

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
pub fn xcb_selinux_get_window_context_unchecked (c : *mut connection,
                                                    window :  ffi::xproto::window) -> get_window_context_cookie;

pub fn xcb_selinux_get_window_context_context (R : *mut get_window_context_reply) -> *mut c_char;


pub fn xcb_selinux_get_window_context_context_length (R : *mut get_window_context_reply) -> c_int;


pub fn xcb_selinux_get_window_context_context_end (R : *mut get_window_context_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_selinux_get_window_context_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_selinux_get_window_context_reply (c : *mut connection,
                                                cookie : get_window_context_cookie,
                                                e : *mut *mut generic_error) -> *mut get_window_context_reply;

pub fn xcb_selinux_list_item_sizeof (_buffer :  *mut c_void) -> c_int;

pub fn xcb_selinux_list_item_object_context (R : *mut list_item) -> *mut c_char;


pub fn xcb_selinux_list_item_object_context_length (R : *mut list_item) -> c_int;


pub fn xcb_selinux_list_item_object_context_end (R : *mut list_item) -> generic_iterator;

pub fn xcb_selinux_list_item_data_context (R : *mut list_item) -> *mut c_char;


pub fn xcb_selinux_list_item_data_context_length (R : *mut list_item) -> c_int;


pub fn xcb_selinux_list_item_data_context_end (R : *mut list_item) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a list_item_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(list_item)
 *
 *
 */
pub fn xcb_selinux_list_item_next (i:*mut list_item_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An list_item_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_selinux_list_item_end (i:list_item_iterator) -> generic_iterator;

pub fn xcb_selinux_set_property_create_context_sizeof (_buffer :  *mut c_void) -> c_int;

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
pub fn xcb_selinux_set_property_create_context_checked (c : *mut connection,
                                                           context_len :  u32,
                                                           context : *mut c_char) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_selinux_set_property_create_context (c : *mut connection,
                                                   context_len :  u32,
                                                   context : *mut c_char) -> void_cookie;

pub fn xcb_selinux_get_property_create_context_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_selinux_get_property_create_context (c : *mut connection) -> get_property_create_context_cookie;

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
pub fn xcb_selinux_get_property_create_context_unchecked (c : *mut connection) -> get_property_create_context_cookie;

pub fn xcb_selinux_get_property_create_context_context (R : *mut get_property_create_context_reply) -> *mut c_char;


pub fn xcb_selinux_get_property_create_context_context_length (R : *mut get_property_create_context_reply) -> c_int;


pub fn xcb_selinux_get_property_create_context_context_end (R : *mut get_property_create_context_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_selinux_get_property_create_context_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_selinux_get_property_create_context_reply (c : *mut connection,
                                                         cookie : get_property_create_context_cookie,
                                                         e : *mut *mut generic_error) -> *mut get_property_create_context_reply;

pub fn xcb_selinux_set_property_use_context_sizeof (_buffer :  *mut c_void) -> c_int;

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
pub fn xcb_selinux_set_property_use_context_checked (c : *mut connection,
                                                        context_len :  u32,
                                                        context : *mut c_char) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_selinux_set_property_use_context (c : *mut connection,
                                                context_len :  u32,
                                                context : *mut c_char) -> void_cookie;

pub fn xcb_selinux_get_property_use_context_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_selinux_get_property_use_context (c : *mut connection) -> get_property_use_context_cookie;

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
pub fn xcb_selinux_get_property_use_context_unchecked (c : *mut connection) -> get_property_use_context_cookie;

pub fn xcb_selinux_get_property_use_context_context (R : *mut get_property_use_context_reply) -> *mut c_char;


pub fn xcb_selinux_get_property_use_context_context_length (R : *mut get_property_use_context_reply) -> c_int;


pub fn xcb_selinux_get_property_use_context_context_end (R : *mut get_property_use_context_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_selinux_get_property_use_context_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_selinux_get_property_use_context_reply (c : *mut connection,
                                                      cookie : get_property_use_context_cookie,
                                                      e : *mut *mut generic_error) -> *mut get_property_use_context_reply;

pub fn xcb_selinux_get_property_context_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_selinux_get_property_context (c : *mut connection,
                                            window :  ffi::xproto::window,
                                            property :  ffi::xproto::atom) -> get_property_context_cookie;

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
pub fn xcb_selinux_get_property_context_unchecked (c : *mut connection,
                                                      window :  ffi::xproto::window,
                                                      property :  ffi::xproto::atom) -> get_property_context_cookie;

pub fn xcb_selinux_get_property_context_context (R : *mut get_property_context_reply) -> *mut c_char;


pub fn xcb_selinux_get_property_context_context_length (R : *mut get_property_context_reply) -> c_int;


pub fn xcb_selinux_get_property_context_context_end (R : *mut get_property_context_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_selinux_get_property_context_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_selinux_get_property_context_reply (c : *mut connection,
                                                  cookie : get_property_context_cookie,
                                                  e : *mut *mut generic_error) -> *mut get_property_context_reply;

pub fn xcb_selinux_get_property_data_context_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_selinux_get_property_data_context (c : *mut connection,
                                                 window :  ffi::xproto::window,
                                                 property :  ffi::xproto::atom) -> get_property_data_context_cookie;

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
pub fn xcb_selinux_get_property_data_context_unchecked (c : *mut connection,
                                                           window :  ffi::xproto::window,
                                                           property :  ffi::xproto::atom) -> get_property_data_context_cookie;

pub fn xcb_selinux_get_property_data_context_context (R : *mut get_property_data_context_reply) -> *mut c_char;


pub fn xcb_selinux_get_property_data_context_context_length (R : *mut get_property_data_context_reply) -> c_int;


pub fn xcb_selinux_get_property_data_context_context_end (R : *mut get_property_data_context_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_selinux_get_property_data_context_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_selinux_get_property_data_context_reply (c : *mut connection,
                                                       cookie : get_property_data_context_cookie,
                                                       e : *mut *mut generic_error) -> *mut get_property_data_context_reply;

pub fn xcb_selinux_list_properties_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_selinux_list_properties (c : *mut connection,
                                       window :  ffi::xproto::window) -> list_properties_cookie;

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
pub fn xcb_selinux_list_properties_unchecked (c : *mut connection,
                                                 window :  ffi::xproto::window) -> list_properties_cookie;


pub fn xcb_selinux_list_properties_properties_length (R : *mut list_properties_reply) -> c_int;

pub fn xcb_selinux_list_properties_properties_iterator (R : *mut list_properties_reply) -> list_item_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_selinux_list_properties_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_selinux_list_properties_reply (c : *mut connection,
                                             cookie : list_properties_cookie,
                                             e : *mut *mut generic_error) -> *mut list_properties_reply;

pub fn xcb_selinux_set_selection_create_context_sizeof (_buffer :  *mut c_void) -> c_int;

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
pub fn xcb_selinux_set_selection_create_context_checked (c : *mut connection,
                                                            context_len :  u32,
                                                            context : *mut c_char) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_selinux_set_selection_create_context (c : *mut connection,
                                                    context_len :  u32,
                                                    context : *mut c_char) -> void_cookie;

pub fn xcb_selinux_get_selection_create_context_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_selinux_get_selection_create_context (c : *mut connection) -> get_selection_create_context_cookie;

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
pub fn xcb_selinux_get_selection_create_context_unchecked (c : *mut connection) -> get_selection_create_context_cookie;

pub fn xcb_selinux_get_selection_create_context_context (R : *mut get_selection_create_context_reply) -> *mut c_char;


pub fn xcb_selinux_get_selection_create_context_context_length (R : *mut get_selection_create_context_reply) -> c_int;


pub fn xcb_selinux_get_selection_create_context_context_end (R : *mut get_selection_create_context_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_selinux_get_selection_create_context_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_selinux_get_selection_create_context_reply (c : *mut connection,
                                                          cookie : get_selection_create_context_cookie,
                                                          e : *mut *mut generic_error) -> *mut get_selection_create_context_reply;

pub fn xcb_selinux_set_selection_use_context_sizeof (_buffer :  *mut c_void) -> c_int;

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
pub fn xcb_selinux_set_selection_use_context_checked (c : *mut connection,
                                                         context_len :  u32,
                                                         context : *mut c_char) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_selinux_set_selection_use_context (c : *mut connection,
                                                 context_len :  u32,
                                                 context : *mut c_char) -> void_cookie;

pub fn xcb_selinux_get_selection_use_context_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_selinux_get_selection_use_context (c : *mut connection) -> get_selection_use_context_cookie;

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
pub fn xcb_selinux_get_selection_use_context_unchecked (c : *mut connection) -> get_selection_use_context_cookie;

pub fn xcb_selinux_get_selection_use_context_context (R : *mut get_selection_use_context_reply) -> *mut c_char;


pub fn xcb_selinux_get_selection_use_context_context_length (R : *mut get_selection_use_context_reply) -> c_int;


pub fn xcb_selinux_get_selection_use_context_context_end (R : *mut get_selection_use_context_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_selinux_get_selection_use_context_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_selinux_get_selection_use_context_reply (c : *mut connection,
                                                       cookie : get_selection_use_context_cookie,
                                                       e : *mut *mut generic_error) -> *mut get_selection_use_context_reply;

pub fn xcb_selinux_get_selection_context_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_selinux_get_selection_context (c : *mut connection,
                                             selection :  ffi::xproto::atom) -> get_selection_context_cookie;

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
pub fn xcb_selinux_get_selection_context_unchecked (c : *mut connection,
                                                       selection :  ffi::xproto::atom) -> get_selection_context_cookie;

pub fn xcb_selinux_get_selection_context_context (R : *mut get_selection_context_reply) -> *mut c_char;


pub fn xcb_selinux_get_selection_context_context_length (R : *mut get_selection_context_reply) -> c_int;


pub fn xcb_selinux_get_selection_context_context_end (R : *mut get_selection_context_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_selinux_get_selection_context_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_selinux_get_selection_context_reply (c : *mut connection,
                                                   cookie : get_selection_context_cookie,
                                                   e : *mut *mut generic_error) -> *mut get_selection_context_reply;

pub fn xcb_selinux_get_selection_data_context_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_selinux_get_selection_data_context (c : *mut connection,
                                                  selection :  ffi::xproto::atom) -> get_selection_data_context_cookie;

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
pub fn xcb_selinux_get_selection_data_context_unchecked (c : *mut connection,
                                                            selection :  ffi::xproto::atom) -> get_selection_data_context_cookie;

pub fn xcb_selinux_get_selection_data_context_context (R : *mut get_selection_data_context_reply) -> *mut c_char;


pub fn xcb_selinux_get_selection_data_context_context_length (R : *mut get_selection_data_context_reply) -> c_int;


pub fn xcb_selinux_get_selection_data_context_context_end (R : *mut get_selection_data_context_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_selinux_get_selection_data_context_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_selinux_get_selection_data_context_reply (c : *mut connection,
                                                        cookie : get_selection_data_context_cookie,
                                                        e : *mut *mut generic_error) -> *mut get_selection_data_context_reply;

pub fn xcb_selinux_list_selections_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_selinux_list_selections (c : *mut connection) -> list_selections_cookie;

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
pub fn xcb_selinux_list_selections_unchecked (c : *mut connection) -> list_selections_cookie;


pub fn xcb_selinux_list_selections_selections_length (R : *mut list_selections_reply) -> c_int;

pub fn xcb_selinux_list_selections_selections_iterator (R : *mut list_selections_reply) -> list_item_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_selinux_list_selections_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_selinux_list_selections_reply (c : *mut connection,
                                             cookie : list_selections_cookie,
                                             e : *mut *mut generic_error) -> *mut list_selections_reply;

pub fn xcb_selinux_get_client_context_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_selinux_get_client_context (c : *mut connection,
                                          resource :  u32) -> get_client_context_cookie;

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
pub fn xcb_selinux_get_client_context_unchecked (c : *mut connection,
                                                    resource :  u32) -> get_client_context_cookie;

pub fn xcb_selinux_get_client_context_context (R : *mut get_client_context_reply) -> *mut c_char;


pub fn xcb_selinux_get_client_context_context_length (R : *mut get_client_context_reply) -> c_int;


pub fn xcb_selinux_get_client_context_context_end (R : *mut get_client_context_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_selinux_get_client_context_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_selinux_get_client_context_reply (c : *mut connection,
                                                cookie : get_client_context_cookie,
                                                e : *mut *mut generic_error) -> *mut get_client_context_reply;
}

