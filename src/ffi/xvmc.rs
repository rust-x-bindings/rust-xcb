//
// This file generated automatically from xvmc.xml by r_client.py.
// Edit at your peril.
//

//Make the compiler quiet
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use std;
use libc::*;
use ffi;
use ffi::xproto;
use ffi::shm;
use ffi::xv;

pub const XVMC_MAJOR_VERSION : c_uint = 1;
pub const XVMC_MINOR_VERSION : c_uint = 1;

pub type xcb_xvmc_context_t = u32;
#[repr(C)]
pub struct xcb_xvmc_context_iterator_t {
    pub data : *mut xcb_xvmc_context_t,
    pub rem  : c_int,
    pub index: c_int
}


pub type xcb_xvmc_surface_t = u32;
#[repr(C)]
pub struct xcb_xvmc_surface_iterator_t {
    pub data : *mut xcb_xvmc_surface_t,
    pub rem  : c_int,
    pub index: c_int
}


pub type xcb_xvmc_subpicture_t = u32;
#[repr(C)]
pub struct xcb_xvmc_subpicture_iterator_t {
    pub data : *mut xcb_xvmc_subpicture_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_xvmc_surface_info_t {
    pub id :                      xcb_xvmc_surface_t,
    pub chroma_format :           u16,
    pub pad0 :                    u16,
    pub max_width :               u16,
    pub max_height :              u16,
    pub subpicture_max_width :    u16,
    pub subpicture_max_height :   u16,
    pub mc_type :                 u32,
    pub flags :                   u32
}

impl Copy for xcb_xvmc_surface_info_t {}
impl Clone for xcb_xvmc_surface_info_t {
    fn clone(&self) -> xcb_xvmc_surface_info_t { *self }
}
#[repr(C)]
pub struct xcb_xvmc_surface_info_iterator_t {
    pub data : *mut xcb_xvmc_surface_info_t,
    pub rem  : c_int,
    pub index: c_int
}


#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xvmc_query_version_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_xvmc_query_version_request_t {
    pub major_opcode :   u8,
    pub minor_opcode :   u8,
    pub length :         u16
}

impl Copy for xcb_xvmc_query_version_request_t {}
impl Clone for xcb_xvmc_query_version_request_t {
    fn clone(&self) -> xcb_xvmc_query_version_request_t { *self }
}

#[repr(C)]
pub struct xcb_xvmc_query_version_reply_t {
    pub response_type :   u8,
    pub pad0 :            u8,
    pub sequence :        u16,
    pub length :          u32,
    pub major :           u32,
    pub minor :           u32
}

impl Copy for xcb_xvmc_query_version_reply_t {}
impl Clone for xcb_xvmc_query_version_reply_t {
    fn clone(&self) -> xcb_xvmc_query_version_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xvmc_list_surface_types_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_xvmc_list_surface_types_request_t {
    pub major_opcode :   u8,
    pub minor_opcode :   u8,
    pub length :         u16,
    pub port_id :        ffi::xv::xcb_xv_port_t
}

impl Copy for xcb_xvmc_list_surface_types_request_t {}
impl Clone for xcb_xvmc_list_surface_types_request_t {
    fn clone(&self) -> xcb_xvmc_list_surface_types_request_t { *self }
}

#[repr(C)]
pub struct xcb_xvmc_list_surface_types_reply_t {
    pub response_type :   u8,
    pub pad0 :            u8,
    pub sequence :        u16,
    pub length :          u32,
    pub num :             u32,
    pub pad1 :            [u8; 20]
}

impl Copy for xcb_xvmc_list_surface_types_reply_t {}
impl Clone for xcb_xvmc_list_surface_types_reply_t {
    fn clone(&self) -> xcb_xvmc_list_surface_types_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xvmc_create_context_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_xvmc_create_context_request_t {
    pub major_opcode :   u8,
    pub minor_opcode :   u8,
    pub length :         u16,
    pub context_id :     xcb_xvmc_context_t,
    pub port_id :        ffi::xv::xcb_xv_port_t,
    pub surface_id :     xcb_xvmc_surface_t,
    pub width :          u16,
    pub height :         u16,
    pub flags :          u32
}

impl Copy for xcb_xvmc_create_context_request_t {}
impl Clone for xcb_xvmc_create_context_request_t {
    fn clone(&self) -> xcb_xvmc_create_context_request_t { *self }
}

#[repr(C)]
pub struct xcb_xvmc_create_context_reply_t {
    pub response_type :   u8,
    pub pad0 :            u8,
    pub sequence :        u16,
    pub length :          u32,
    pub width_actual :    u16,
    pub height_actual :   u16,
    pub flags_return :    u32,
    pub pad1 :            [u8; 20]
}

impl Copy for xcb_xvmc_create_context_reply_t {}
impl Clone for xcb_xvmc_create_context_reply_t {
    fn clone(&self) -> xcb_xvmc_create_context_reply_t { *self }
}


#[repr(C)]
pub struct xcb_xvmc_destroy_context_request_t {
    pub major_opcode :   u8,
    pub minor_opcode :   u8,
    pub length :         u16,
    pub context_id :     xcb_xvmc_context_t
}

impl Copy for xcb_xvmc_destroy_context_request_t {}
impl Clone for xcb_xvmc_destroy_context_request_t {
    fn clone(&self) -> xcb_xvmc_destroy_context_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xvmc_create_surface_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_xvmc_create_surface_request_t {
    pub major_opcode :   u8,
    pub minor_opcode :   u8,
    pub length :         u16,
    pub surface_id :     xcb_xvmc_surface_t,
    pub context_id :     xcb_xvmc_context_t
}

impl Copy for xcb_xvmc_create_surface_request_t {}
impl Clone for xcb_xvmc_create_surface_request_t {
    fn clone(&self) -> xcb_xvmc_create_surface_request_t { *self }
}

#[repr(C)]
pub struct xcb_xvmc_create_surface_reply_t {
    pub response_type :   u8,
    pub pad0 :            u8,
    pub sequence :        u16,
    pub length :          u32,
    pub pad1 :            [u8; 24]
}

impl Copy for xcb_xvmc_create_surface_reply_t {}
impl Clone for xcb_xvmc_create_surface_reply_t {
    fn clone(&self) -> xcb_xvmc_create_surface_reply_t { *self }
}


#[repr(C)]
pub struct xcb_xvmc_destroy_surface_request_t {
    pub major_opcode :   u8,
    pub minor_opcode :   u8,
    pub length :         u16,
    pub surface_id :     xcb_xvmc_surface_t
}

impl Copy for xcb_xvmc_destroy_surface_request_t {}
impl Clone for xcb_xvmc_destroy_surface_request_t {
    fn clone(&self) -> xcb_xvmc_destroy_surface_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xvmc_create_subpicture_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_xvmc_create_subpicture_request_t {
    pub major_opcode :    u8,
    pub minor_opcode :    u8,
    pub length :          u16,
    pub subpicture_id :   xcb_xvmc_subpicture_t,
    pub context :         xcb_xvmc_context_t,
    pub xvimage_id :      u32,
    pub width :           u16,
    pub height :          u16
}

impl Copy for xcb_xvmc_create_subpicture_request_t {}
impl Clone for xcb_xvmc_create_subpicture_request_t {
    fn clone(&self) -> xcb_xvmc_create_subpicture_request_t { *self }
}

#[repr(C)]
pub struct xcb_xvmc_create_subpicture_reply_t {
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

impl Copy for xcb_xvmc_create_subpicture_reply_t {}
impl Clone for xcb_xvmc_create_subpicture_reply_t {
    fn clone(&self) -> xcb_xvmc_create_subpicture_reply_t { *self }
}


#[repr(C)]
pub struct xcb_xvmc_destroy_subpicture_request_t {
    pub major_opcode :    u8,
    pub minor_opcode :    u8,
    pub length :          u16,
    pub subpicture_id :   xcb_xvmc_subpicture_t
}

impl Copy for xcb_xvmc_destroy_subpicture_request_t {}
impl Clone for xcb_xvmc_destroy_subpicture_request_t {
    fn clone(&self) -> xcb_xvmc_destroy_subpicture_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xvmc_list_subpicture_types_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_xvmc_list_subpicture_types_request_t {
    pub major_opcode :   u8,
    pub minor_opcode :   u8,
    pub length :         u16,
    pub port_id :        ffi::xv::xcb_xv_port_t,
    pub surface_id :     xcb_xvmc_surface_t
}

impl Copy for xcb_xvmc_list_subpicture_types_request_t {}
impl Clone for xcb_xvmc_list_subpicture_types_request_t {
    fn clone(&self) -> xcb_xvmc_list_subpicture_types_request_t { *self }
}

#[repr(C)]
pub struct xcb_xvmc_list_subpicture_types_reply_t {
    pub response_type :   u8,
    pub pad0 :            u8,
    pub sequence :        u16,
    pub length :          u32,
    pub num :             u32,
    pub pad1 :            [u8; 20]
}

impl Copy for xcb_xvmc_list_subpicture_types_reply_t {}
impl Clone for xcb_xvmc_list_subpicture_types_reply_t {
    fn clone(&self) -> xcb_xvmc_list_subpicture_types_reply_t { *self }
}
#[link(name="xcb-xvmc")]
extern "C" {

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xvmc_context_t)
///
pub fn xcb_xvmc_context_next (i:*mut xcb_xvmc_context_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xvmc_context_end (i:xcb_xvmc_context_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xvmc_surface_t)
///
pub fn xcb_xvmc_surface_next (i:*mut xcb_xvmc_surface_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xvmc_surface_end (i:xcb_xvmc_surface_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xvmc_subpicture_t)
///
pub fn xcb_xvmc_subpicture_next (i:*mut xcb_xvmc_subpicture_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xvmc_subpicture_end (i:xcb_xvmc_subpicture_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xvmc_surface_info_t)
///
pub fn xcb_xvmc_surface_info_next (i:*mut xcb_xvmc_surface_info_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xvmc_surface_info_end (i:xcb_xvmc_surface_info_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/// Delivers a request to the X server.
///
pub fn xcb_xvmc_query_version (c : *mut ffi::base::xcb_connection_t) -> xcb_xvmc_query_version_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_xvmc_query_version_unchecked (c : *mut ffi::base::xcb_connection_t) -> xcb_xvmc_query_version_cookie_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_xvmc_query_version_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_xvmc_query_version_reply (c : *mut ffi::base::xcb_connection_t,
                                        cookie : xcb_xvmc_query_version_cookie_t,
                                        e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_xvmc_query_version_reply_t;

pub fn xcb_xvmc_list_surface_types_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
///
pub fn xcb_xvmc_list_surface_types (c : *mut ffi::base::xcb_connection_t,
                                       port_id :  ffi::xv::xcb_xv_port_t) -> xcb_xvmc_list_surface_types_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_xvmc_list_surface_types_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                 port_id :  ffi::xv::xcb_xv_port_t) -> xcb_xvmc_list_surface_types_cookie_t;

pub fn xcb_xvmc_list_surface_types_surfaces (R : *mut xcb_xvmc_list_surface_types_reply_t) -> *mut xcb_xvmc_surface_info_t;


pub fn xcb_xvmc_list_surface_types_surfaces_length (R : *mut xcb_xvmc_list_surface_types_reply_t) -> c_int;

pub fn xcb_xvmc_list_surface_types_surfaces_iterator (R : *mut xcb_xvmc_list_surface_types_reply_t) -> xcb_xvmc_surface_info_iterator_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_xvmc_list_surface_types_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_xvmc_list_surface_types_reply (c : *mut ffi::base::xcb_connection_t,
                                             cookie : xcb_xvmc_list_surface_types_cookie_t,
                                             e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_xvmc_list_surface_types_reply_t;

pub fn xcb_xvmc_create_context_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
///
pub fn xcb_xvmc_create_context (c : *mut ffi::base::xcb_connection_t,
                                   context_id :  xcb_xvmc_context_t,
                                   port_id :  ffi::xv::xcb_xv_port_t,
                                   surface_id :  xcb_xvmc_surface_t,
                                   width :  u16,
                                   height :  u16,
                                   flags :  u32) -> xcb_xvmc_create_context_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_xvmc_create_context_unchecked (c : *mut ffi::base::xcb_connection_t,
                                             context_id :  xcb_xvmc_context_t,
                                             port_id :  ffi::xv::xcb_xv_port_t,
                                             surface_id :  xcb_xvmc_surface_t,
                                             width :  u16,
                                             height :  u16,
                                             flags :  u32) -> xcb_xvmc_create_context_cookie_t;

pub fn xcb_xvmc_create_context_priv_data (R : *mut xcb_xvmc_create_context_reply_t) -> *mut u32;


pub fn xcb_xvmc_create_context_priv_data_length (R : *mut xcb_xvmc_create_context_reply_t) -> c_int;


pub fn xcb_xvmc_create_context_priv_data_end (R : *mut xcb_xvmc_create_context_reply_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_xvmc_create_context_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_xvmc_create_context_reply (c : *mut ffi::base::xcb_connection_t,
                                         cookie : xcb_xvmc_create_context_cookie_t,
                                         e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_xvmc_create_context_reply_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_xvmc_destroy_context_checked (c : *mut ffi::base::xcb_connection_t,
                                            context_id :  xcb_xvmc_context_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_xvmc_destroy_context (c : *mut ffi::base::xcb_connection_t,
                                    context_id :  xcb_xvmc_context_t) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_xvmc_create_surface_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
///
pub fn xcb_xvmc_create_surface (c : *mut ffi::base::xcb_connection_t,
                                   surface_id :  xcb_xvmc_surface_t,
                                   context_id :  xcb_xvmc_context_t) -> xcb_xvmc_create_surface_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_xvmc_create_surface_unchecked (c : *mut ffi::base::xcb_connection_t,
                                             surface_id :  xcb_xvmc_surface_t,
                                             context_id :  xcb_xvmc_context_t) -> xcb_xvmc_create_surface_cookie_t;

pub fn xcb_xvmc_create_surface_priv_data (R : *mut xcb_xvmc_create_surface_reply_t) -> *mut u32;


pub fn xcb_xvmc_create_surface_priv_data_length (R : *mut xcb_xvmc_create_surface_reply_t) -> c_int;


pub fn xcb_xvmc_create_surface_priv_data_end (R : *mut xcb_xvmc_create_surface_reply_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_xvmc_create_surface_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_xvmc_create_surface_reply (c : *mut ffi::base::xcb_connection_t,
                                         cookie : xcb_xvmc_create_surface_cookie_t,
                                         e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_xvmc_create_surface_reply_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_xvmc_destroy_surface_checked (c : *mut ffi::base::xcb_connection_t,
                                            surface_id :  xcb_xvmc_surface_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_xvmc_destroy_surface (c : *mut ffi::base::xcb_connection_t,
                                    surface_id :  xcb_xvmc_surface_t) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_xvmc_create_subpicture_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
///
pub fn xcb_xvmc_create_subpicture (c : *mut ffi::base::xcb_connection_t,
                                      subpicture_id :  xcb_xvmc_subpicture_t,
                                      context :  xcb_xvmc_context_t,
                                      xvimage_id :  u32,
                                      width :  u16,
                                      height :  u16) -> xcb_xvmc_create_subpicture_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_xvmc_create_subpicture_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                subpicture_id :  xcb_xvmc_subpicture_t,
                                                context :  xcb_xvmc_context_t,
                                                xvimage_id :  u32,
                                                width :  u16,
                                                height :  u16) -> xcb_xvmc_create_subpicture_cookie_t;

pub fn xcb_xvmc_create_subpicture_priv_data (R : *mut xcb_xvmc_create_subpicture_reply_t) -> *mut u32;


pub fn xcb_xvmc_create_subpicture_priv_data_length (R : *mut xcb_xvmc_create_subpicture_reply_t) -> c_int;


pub fn xcb_xvmc_create_subpicture_priv_data_end (R : *mut xcb_xvmc_create_subpicture_reply_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_xvmc_create_subpicture_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_xvmc_create_subpicture_reply (c : *mut ffi::base::xcb_connection_t,
                                            cookie : xcb_xvmc_create_subpicture_cookie_t,
                                            e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_xvmc_create_subpicture_reply_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_xvmc_destroy_subpicture_checked (c : *mut ffi::base::xcb_connection_t,
                                               subpicture_id :  xcb_xvmc_subpicture_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_xvmc_destroy_subpicture (c : *mut ffi::base::xcb_connection_t,
                                       subpicture_id :  xcb_xvmc_subpicture_t) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_xvmc_list_subpicture_types_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
///
pub fn xcb_xvmc_list_subpicture_types (c : *mut ffi::base::xcb_connection_t,
                                          port_id :  ffi::xv::xcb_xv_port_t,
                                          surface_id :  xcb_xvmc_surface_t) -> xcb_xvmc_list_subpicture_types_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_xvmc_list_subpicture_types_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                    port_id :  ffi::xv::xcb_xv_port_t,
                                                    surface_id :  xcb_xvmc_surface_t) -> xcb_xvmc_list_subpicture_types_cookie_t;

pub fn xcb_xvmc_list_subpicture_types_types (R : *mut xcb_xvmc_list_subpicture_types_reply_t) -> *mut ffi::xv::xcb_xv_image_format_info_t;


pub fn xcb_xvmc_list_subpicture_types_types_length (R : *mut xcb_xvmc_list_subpicture_types_reply_t) -> c_int;

pub fn xcb_xvmc_list_subpicture_types_types_iterator (R : *mut xcb_xvmc_list_subpicture_types_reply_t) -> ffi::xv::xcb_xv_image_format_info_iterator_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_xvmc_list_subpicture_types_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_xvmc_list_subpicture_types_reply (c : *mut ffi::base::xcb_connection_t,
                                                cookie : xcb_xvmc_list_subpicture_types_cookie_t,
                                                e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_xvmc_list_subpicture_types_reply_t;
}

