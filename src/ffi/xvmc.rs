/*
 * This file generated automatically from xvmc.xml by r_client.py.
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
use ffi::shm;
use ffi::xv;

pub static XVMC_MAJOR_VERSION : c_uint = 1;
pub static XVMC_MINOR_VERSION : c_uint = 1;

pub type context = u32;
/**
 * @brief context_iterator
 **/
pub struct context_iterator {
    data : *mut context,
    rem  : c_int,
    index: c_int
}


pub type surface = u32;
/**
 * @brief surface_iterator
 **/
pub struct surface_iterator {
    data : *mut surface,
    rem  : c_int,
    index: c_int
}


pub type subpicture = u32;
/**
 * @brief subpicture_iterator
 **/
pub struct subpicture_iterator {
    data : *mut subpicture,
    rem  : c_int,
    index: c_int
}


pub struct surface_info {
    id :                      surface,
    chroma_format :           u16,
    pad0 :                    u16,
    max_width :               u16,
    max_height :              u16,
    subpicture_max_width :    u16,
    subpicture_max_height :   u16,
    mc_type :                 u32,
    flags :                   u32
}

/**
 * @brief surface_info_iterator
 **/
pub struct surface_info_iterator {
    data : *mut surface_info,
    rem  : c_int,
    index: c_int
}


pub struct query_version_cookie {
    sequence : c_uint
}


pub struct query_version_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16
}


pub struct query_version_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    major :           u32,
    minor :           u32
}


pub struct list_surface_types_cookie {
    sequence : c_uint
}


pub struct list_surface_types_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    port_id :        ffi::xv::port
}


pub struct list_surface_types_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    num :             u32,
    pad1 :            [u8,..20]
}


pub struct create_context_cookie {
    sequence : c_uint
}


pub struct create_context_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_id :     context,
    port_id :        ffi::xv::port,
    surface_id :     surface,
    width :          u16,
    height :         u16,
    flags :          u32
}


pub struct create_context_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    width_actual :    u16,
    height_actual :   u16,
    flags_return :    u32,
    pad1 :            [u8,..20]
}



pub struct destroy_context_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_id :     context
}


pub struct create_surface_cookie {
    sequence : c_uint
}


pub struct create_surface_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    surface_id :     surface,
    context_id :     context
}


pub struct create_surface_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..24]
}



pub struct destroy_surface_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    surface_id :     surface
}


pub struct create_subpicture_cookie {
    sequence : c_uint
}


pub struct create_subpicture_request {
    major_opcode :    u8,
    minor_opcode :    u8,
    length :          u16,
    subpicture_id :   subpicture,
    context :         context,
    xvimage_id :      u32,
    width :           u16,
    height :          u16
}


pub struct create_subpicture_reply {
    response_type :         u8,
    pad0 :                  u8,
    sequence :              u16,
    length :                u32,
    width_actual :          u16,
    height_actual :         u16,
    num_palette_entries :   u16,
    entry_bytes :           u16,
    component_order :       [u8,..4],
    pad1 :                  [u8,..12]
}



pub struct destroy_subpicture_request {
    major_opcode :    u8,
    minor_opcode :    u8,
    length :          u16,
    subpicture_id :   subpicture
}


pub struct list_subpicture_types_cookie {
    sequence : c_uint
}


pub struct list_subpicture_types_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    port_id :        ffi::xv::port,
    surface_id :     surface
}


pub struct list_subpicture_types_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    num :             u32,
    pad1 :            [u8,..20]
}

#[link(name="lxcb-xvmc")]
pub extern "C" {

/**
 * Get the next element of the iterator
 * @param i Pointer to a context_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(context)
 *
 *
 */
pub fn xcb_xvmc_context_next (i:*mut context_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An context_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_xvmc_context_end (i:context_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a surface_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(surface)
 *
 *
 */
pub fn xcb_xvmc_surface_next (i:*mut surface_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An surface_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_xvmc_surface_end (i:surface_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a subpicture_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(subpicture)
 *
 *
 */
pub fn xcb_xvmc_subpicture_next (i:*mut subpicture_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An subpicture_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_xvmc_subpicture_end (i:subpicture_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a surface_info_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(surface_info)
 *
 *
 */
pub fn xcb_xvmc_surface_info_next (i:*mut surface_info_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An surface_info_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_xvmc_surface_info_end (i:surface_info_iterator) -> generic_iterator;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xvmc_query_version (c : *mut connection) -> query_version_cookie;

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
pub fn xcb_xvmc_query_version_unchecked (c : *mut connection) -> query_version_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xvmc_query_version_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_xvmc_query_version_reply (c : *mut connection,
                                        cookie : query_version_cookie,
                                        e : *mut *mut generic_error) -> *mut query_version_reply;

pub fn xcb_xvmc_list_surface_types_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xvmc_list_surface_types (c : *mut connection,
                                       port_id :  ffi::xv::port) -> list_surface_types_cookie;

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
pub fn xcb_xvmc_list_surface_types_unchecked (c : *mut connection,
                                                 port_id :  ffi::xv::port) -> list_surface_types_cookie;

pub fn xcb_xvmc_list_surface_types_surfaces (R : *mut list_surface_types_reply) -> *mut surface_info;


pub fn xcb_xvmc_list_surface_types_surfaces_length (R : *mut list_surface_types_reply) -> c_int;

pub fn xcb_xvmc_list_surface_types_surfaces_iterator (R : *mut list_surface_types_reply) -> surface_info_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xvmc_list_surface_types_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_xvmc_list_surface_types_reply (c : *mut connection,
                                             cookie : list_surface_types_cookie,
                                             e : *mut *mut generic_error) -> *mut list_surface_types_reply;

pub fn xcb_xvmc_create_context_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xvmc_create_context (c : *mut connection,
                                   context_id :  context,
                                   port_id :  ffi::xv::port,
                                   surface_id :  surface,
                                   width :  u16,
                                   height :  u16,
                                   flags :  u32) -> create_context_cookie;

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
pub fn xcb_xvmc_create_context_unchecked (c : *mut connection,
                                             context_id :  context,
                                             port_id :  ffi::xv::port,
                                             surface_id :  surface,
                                             width :  u16,
                                             height :  u16,
                                             flags :  u32) -> create_context_cookie;

pub fn xcb_xvmc_create_context_priv_data (R : *mut create_context_reply) -> *mut u32;


pub fn xcb_xvmc_create_context_priv_data_length (R : *mut create_context_reply) -> c_int;


pub fn xcb_xvmc_create_context_priv_data_end (R : *mut create_context_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xvmc_create_context_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_xvmc_create_context_reply (c : *mut connection,
                                         cookie : create_context_cookie,
                                         e : *mut *mut generic_error) -> *mut create_context_reply;

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
pub fn xcb_xvmc_destroy_context_checked (c : *mut connection,
                                            context_id :  context) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xvmc_destroy_context (c : *mut connection,
                                    context_id :  context) -> void_cookie;

pub fn xcb_xvmc_create_surface_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xvmc_create_surface (c : *mut connection,
                                   surface_id :  surface,
                                   context_id :  context) -> create_surface_cookie;

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
pub fn xcb_xvmc_create_surface_unchecked (c : *mut connection,
                                             surface_id :  surface,
                                             context_id :  context) -> create_surface_cookie;

pub fn xcb_xvmc_create_surface_priv_data (R : *mut create_surface_reply) -> *mut u32;


pub fn xcb_xvmc_create_surface_priv_data_length (R : *mut create_surface_reply) -> c_int;


pub fn xcb_xvmc_create_surface_priv_data_end (R : *mut create_surface_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xvmc_create_surface_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_xvmc_create_surface_reply (c : *mut connection,
                                         cookie : create_surface_cookie,
                                         e : *mut *mut generic_error) -> *mut create_surface_reply;

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
pub fn xcb_xvmc_destroy_surface_checked (c : *mut connection,
                                            surface_id :  surface) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xvmc_destroy_surface (c : *mut connection,
                                    surface_id :  surface) -> void_cookie;

pub fn xcb_xvmc_create_subpicture_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xvmc_create_subpicture (c : *mut connection,
                                      subpicture_id :  subpicture,
                                      context :  context,
                                      xvimage_id :  u32,
                                      width :  u16,
                                      height :  u16) -> create_subpicture_cookie;

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
pub fn xcb_xvmc_create_subpicture_unchecked (c : *mut connection,
                                                subpicture_id :  subpicture,
                                                context :  context,
                                                xvimage_id :  u32,
                                                width :  u16,
                                                height :  u16) -> create_subpicture_cookie;

pub fn xcb_xvmc_create_subpicture_priv_data (R : *mut create_subpicture_reply) -> *mut u32;


pub fn xcb_xvmc_create_subpicture_priv_data_length (R : *mut create_subpicture_reply) -> c_int;


pub fn xcb_xvmc_create_subpicture_priv_data_end (R : *mut create_subpicture_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xvmc_create_subpicture_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_xvmc_create_subpicture_reply (c : *mut connection,
                                            cookie : create_subpicture_cookie,
                                            e : *mut *mut generic_error) -> *mut create_subpicture_reply;

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
pub fn xcb_xvmc_destroy_subpicture_checked (c : *mut connection,
                                               subpicture_id :  subpicture) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xvmc_destroy_subpicture (c : *mut connection,
                                       subpicture_id :  subpicture) -> void_cookie;

pub fn xcb_xvmc_list_subpicture_types_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xvmc_list_subpicture_types (c : *mut connection,
                                          port_id :  ffi::xv::port,
                                          surface_id :  surface) -> list_subpicture_types_cookie;

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
pub fn xcb_xvmc_list_subpicture_types_unchecked (c : *mut connection,
                                                    port_id :  ffi::xv::port,
                                                    surface_id :  surface) -> list_subpicture_types_cookie;

pub fn xcb_xvmc_list_subpicture_types_types (R : *mut list_subpicture_types_reply) -> *mut ffi::xv::image_format_info;


pub fn xcb_xvmc_list_subpicture_types_types_length (R : *mut list_subpicture_types_reply) -> c_int;

pub fn xcb_xvmc_list_subpicture_types_types_iterator (R : *mut list_subpicture_types_reply) -> ffi::xv::image_format_info_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xvmc_list_subpicture_types_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_xvmc_list_subpicture_types_reply (c : *mut connection,
                                                cookie : list_subpicture_types_cookie,
                                                e : *mut *mut generic_error) -> *mut list_subpicture_types_reply;
}

