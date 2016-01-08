/*
 * This file generated automatically from xvmc.xml by r_client.py.
 * Edit at your peril.
 */

//Make the compiler quiet
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use std;
use libc::*;
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
#[repr(C)]
pub struct context_iterator {
    pub data : *mut context,
    pub rem  : c_int,
    pub index: c_int
}


pub type surface = u32;
/**
 * @brief surface_iterator
 **/
#[repr(C)]
pub struct surface_iterator {
    pub data : *mut surface,
    pub rem  : c_int,
    pub index: c_int
}


pub type subpicture = u32;
/**
 * @brief subpicture_iterator
 **/
#[repr(C)]
pub struct subpicture_iterator {
    pub data : *mut subpicture,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct surface_info {
     pub id :                      surface,
     pub chroma_format :           u16,
     pub pad0 :                    u16,
     pub max_width :               u16,
     pub max_height :              u16,
     pub subpicture_max_width :    u16,
     pub subpicture_max_height :   u16,
     pub mc_type :                 u32,
     pub flags :                   u32
}

impl Copy for surface_info {}
impl Clone for surface_info {
    fn clone(&self) -> surface_info { *self }
}
/**
 * @brief surface_info_iterator
 **/
#[repr(C)]
pub struct surface_info_iterator {
    pub data : *mut surface_info,
    pub rem  : c_int,
    pub index: c_int
}


#[derive(Copy, Clone)]
#[repr(C)]
pub struct query_version_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct query_version_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16
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
     pub major :           u32,
     pub minor :           u32
}

impl Copy for query_version_reply {}
impl Clone for query_version_reply {
    fn clone(&self) -> query_version_reply { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct list_surface_types_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct list_surface_types_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub port_id :        ffi::xv::port
}

impl Copy for list_surface_types_request {}
impl Clone for list_surface_types_request {
    fn clone(&self) -> list_surface_types_request { *self }
}

#[repr(C)]
pub struct list_surface_types_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub num :             u32,
     pub pad1 :            [u8; 20]
}

impl Copy for list_surface_types_reply {}
impl Clone for list_surface_types_reply {
    fn clone(&self) -> list_surface_types_reply { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct create_context_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct create_context_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub context_id :     context,
     pub port_id :        ffi::xv::port,
     pub surface_id :     surface,
     pub width :          u16,
     pub height :         u16,
     pub flags :          u32
}

impl Copy for create_context_request {}
impl Clone for create_context_request {
    fn clone(&self) -> create_context_request { *self }
}

#[repr(C)]
pub struct create_context_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub width_actual :    u16,
     pub height_actual :   u16,
     pub flags_return :    u32,
     pub pad1 :            [u8; 20]
}

impl Copy for create_context_reply {}
impl Clone for create_context_reply {
    fn clone(&self) -> create_context_reply { *self }
}


#[repr(C)]
pub struct destroy_context_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub context_id :     context
}

impl Copy for destroy_context_request {}
impl Clone for destroy_context_request {
    fn clone(&self) -> destroy_context_request { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct create_surface_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct create_surface_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub surface_id :     surface,
     pub context_id :     context
}

impl Copy for create_surface_request {}
impl Clone for create_surface_request {
    fn clone(&self) -> create_surface_request { *self }
}

#[repr(C)]
pub struct create_surface_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub pad1 :            [u8; 24]
}

impl Copy for create_surface_reply {}
impl Clone for create_surface_reply {
    fn clone(&self) -> create_surface_reply { *self }
}


#[repr(C)]
pub struct destroy_surface_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub surface_id :     surface
}

impl Copy for destroy_surface_request {}
impl Clone for destroy_surface_request {
    fn clone(&self) -> destroy_surface_request { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct create_subpicture_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct create_subpicture_request {
     pub major_opcode :    u8,
     pub minor_opcode :    u8,
     pub length :          u16,
     pub subpicture_id :   subpicture,
     pub context :         context,
     pub xvimage_id :      u32,
     pub width :           u16,
     pub height :          u16
}

impl Copy for create_subpicture_request {}
impl Clone for create_subpicture_request {
    fn clone(&self) -> create_subpicture_request { *self }
}

#[repr(C)]
pub struct create_subpicture_reply {
     pub response_type :         u8,
     pub pad0 :                  u8,
     pub sequence :              u16,
     pub length :                u32,
     pub width_actual :          u16,
     pub height_actual :         u16,
     pub num_palette_entries :   u16,
     pub entry_bytes :           u16,
     pub component_order :       [u8; 4],
     pub pad1 :                  [u8; 12]
}

impl Copy for create_subpicture_reply {}
impl Clone for create_subpicture_reply {
    fn clone(&self) -> create_subpicture_reply { *self }
}


#[repr(C)]
pub struct destroy_subpicture_request {
     pub major_opcode :    u8,
     pub minor_opcode :    u8,
     pub length :          u16,
     pub subpicture_id :   subpicture
}

impl Copy for destroy_subpicture_request {}
impl Clone for destroy_subpicture_request {
    fn clone(&self) -> destroy_subpicture_request { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct list_subpicture_types_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct list_subpicture_types_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub port_id :        ffi::xv::port,
     pub surface_id :     surface
}

impl Copy for list_subpicture_types_request {}
impl Clone for list_subpicture_types_request {
    fn clone(&self) -> list_subpicture_types_request { *self }
}

#[repr(C)]
pub struct list_subpicture_types_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub num :             u32,
     pub pad1 :            [u8; 20]
}

impl Copy for list_subpicture_types_reply {}
impl Clone for list_subpicture_types_reply {
    fn clone(&self) -> list_subpicture_types_reply { *self }
}
#[link(name="xcb-xvmc")]
extern "C" {

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
pub fn xcb_xvmc_context_end (i:context_iterator) -> ffi::base::generic_iterator;

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
pub fn xcb_xvmc_surface_end (i:surface_iterator) -> ffi::base::generic_iterator;

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
pub fn xcb_xvmc_subpicture_end (i:subpicture_iterator) -> ffi::base::generic_iterator;

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
pub fn xcb_xvmc_surface_info_end (i:surface_info_iterator) -> ffi::base::generic_iterator;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_xvmc_query_version (c : *mut ffi::base::connection) -> query_version_cookie;

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
pub fn xcb_xvmc_query_version_unchecked (c : *mut ffi::base::connection) -> query_version_cookie;

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
pub fn xcb_xvmc_query_version_reply (c : *mut ffi::base::connection,
                                        cookie : query_version_cookie,
                                        e : *mut *mut ffi::base::generic_error) -> *mut query_version_reply;

pub fn xcb_xvmc_list_surface_types_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_xvmc_list_surface_types (c : *mut ffi::base::connection,
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
pub fn xcb_xvmc_list_surface_types_unchecked (c : *mut ffi::base::connection,
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
pub fn xcb_xvmc_list_surface_types_reply (c : *mut ffi::base::connection,
                                             cookie : list_surface_types_cookie,
                                             e : *mut *mut ffi::base::generic_error) -> *mut list_surface_types_reply;

pub fn xcb_xvmc_create_context_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_xvmc_create_context (c : *mut ffi::base::connection,
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
pub fn xcb_xvmc_create_context_unchecked (c : *mut ffi::base::connection,
                                             context_id :  context,
                                             port_id :  ffi::xv::port,
                                             surface_id :  surface,
                                             width :  u16,
                                             height :  u16,
                                             flags :  u32) -> create_context_cookie;

pub fn xcb_xvmc_create_context_priv_data (R : *mut create_context_reply) -> *mut u32;


pub fn xcb_xvmc_create_context_priv_data_length (R : *mut create_context_reply) -> c_int;


pub fn xcb_xvmc_create_context_priv_data_end (R : *mut create_context_reply) -> ffi::base::generic_iterator;

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
pub fn xcb_xvmc_create_context_reply (c : *mut ffi::base::connection,
                                         cookie : create_context_cookie,
                                         e : *mut *mut ffi::base::generic_error) -> *mut create_context_reply;

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
pub fn xcb_xvmc_destroy_context_checked (c : *mut ffi::base::connection,
                                            context_id :  context) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_xvmc_destroy_context (c : *mut ffi::base::connection,
                                    context_id :  context) -> ffi::base::void_cookie;

pub fn xcb_xvmc_create_surface_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_xvmc_create_surface (c : *mut ffi::base::connection,
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
pub fn xcb_xvmc_create_surface_unchecked (c : *mut ffi::base::connection,
                                             surface_id :  surface,
                                             context_id :  context) -> create_surface_cookie;

pub fn xcb_xvmc_create_surface_priv_data (R : *mut create_surface_reply) -> *mut u32;


pub fn xcb_xvmc_create_surface_priv_data_length (R : *mut create_surface_reply) -> c_int;


pub fn xcb_xvmc_create_surface_priv_data_end (R : *mut create_surface_reply) -> ffi::base::generic_iterator;

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
pub fn xcb_xvmc_create_surface_reply (c : *mut ffi::base::connection,
                                         cookie : create_surface_cookie,
                                         e : *mut *mut ffi::base::generic_error) -> *mut create_surface_reply;

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
pub fn xcb_xvmc_destroy_surface_checked (c : *mut ffi::base::connection,
                                            surface_id :  surface) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_xvmc_destroy_surface (c : *mut ffi::base::connection,
                                    surface_id :  surface) -> ffi::base::void_cookie;

pub fn xcb_xvmc_create_subpicture_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_xvmc_create_subpicture (c : *mut ffi::base::connection,
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
pub fn xcb_xvmc_create_subpicture_unchecked (c : *mut ffi::base::connection,
                                                subpicture_id :  subpicture,
                                                context :  context,
                                                xvimage_id :  u32,
                                                width :  u16,
                                                height :  u16) -> create_subpicture_cookie;

pub fn xcb_xvmc_create_subpicture_priv_data (R : *mut create_subpicture_reply) -> *mut u32;


pub fn xcb_xvmc_create_subpicture_priv_data_length (R : *mut create_subpicture_reply) -> c_int;


pub fn xcb_xvmc_create_subpicture_priv_data_end (R : *mut create_subpicture_reply) -> ffi::base::generic_iterator;

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
pub fn xcb_xvmc_create_subpicture_reply (c : *mut ffi::base::connection,
                                            cookie : create_subpicture_cookie,
                                            e : *mut *mut ffi::base::generic_error) -> *mut create_subpicture_reply;

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
pub fn xcb_xvmc_destroy_subpicture_checked (c : *mut ffi::base::connection,
                                               subpicture_id :  subpicture) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_xvmc_destroy_subpicture (c : *mut ffi::base::connection,
                                       subpicture_id :  subpicture) -> ffi::base::void_cookie;

pub fn xcb_xvmc_list_subpicture_types_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_xvmc_list_subpicture_types (c : *mut ffi::base::connection,
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
pub fn xcb_xvmc_list_subpicture_types_unchecked (c : *mut ffi::base::connection,
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
pub fn xcb_xvmc_list_subpicture_types_reply (c : *mut ffi::base::connection,
                                                cookie : list_subpicture_types_cookie,
                                                e : *mut *mut ffi::base::generic_error) -> *mut list_subpicture_types_reply;
}

