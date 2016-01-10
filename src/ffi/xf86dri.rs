//
// This file generated automatically from xf86dri.xml by r_client.py.
// Edit at your peril.
//

//Make the compiler quiet
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use std;
use libc::*;
use ffi;

pub const XF86DRI_MAJOR_VERSION : c_uint = 4;
pub const XF86DRI_MINOR_VERSION : c_uint = 1;

#[repr(C)]
pub struct xcb_xf86dri_drm_clip_rect_t {
     pub x1 :   i16,
     pub y1 :   i16,
     pub x2 :   i16,
     pub x3 :   i16
}

impl Copy for xcb_xf86dri_drm_clip_rect_t {}
impl Clone for xcb_xf86dri_drm_clip_rect_t {
    fn clone(&self) -> xcb_xf86dri_drm_clip_rect_t { *self }
}
#[repr(C)]
pub struct xcb_xf86dri_drm_clip_rect_iterator_t {
    pub data : *mut xcb_xf86dri_drm_clip_rect_t,
    pub rem  : c_int,
    pub index: c_int
}


#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xf86dri_query_version_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_xf86dri_query_version_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16
}

impl Copy for xcb_xf86dri_query_version_request_t {}
impl Clone for xcb_xf86dri_query_version_request_t {
    fn clone(&self) -> xcb_xf86dri_query_version_request_t { *self }
}

#[repr(C)]
pub struct xcb_xf86dri_query_version_reply_t {
     pub response_type :       u8,
     pub pad0 :                u8,
     pub sequence :            u16,
     pub length :              u32,
     pub dri_major_version :   u16,
     pub dri_minor_version :   u16,
     pub dri_minor_patch :     u32
}

impl Copy for xcb_xf86dri_query_version_reply_t {}
impl Clone for xcb_xf86dri_query_version_reply_t {
    fn clone(&self) -> xcb_xf86dri_query_version_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xf86dri_query_direct_rendering_capable_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_xf86dri_query_direct_rendering_capable_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub screen :         u32
}

impl Copy for xcb_xf86dri_query_direct_rendering_capable_request_t {}
impl Clone for xcb_xf86dri_query_direct_rendering_capable_request_t {
    fn clone(&self) -> xcb_xf86dri_query_direct_rendering_capable_request_t { *self }
}

#[repr(C)]
pub struct xcb_xf86dri_query_direct_rendering_capable_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub is_capable :      u8
}

impl Copy for xcb_xf86dri_query_direct_rendering_capable_reply_t {}
impl Clone for xcb_xf86dri_query_direct_rendering_capable_reply_t {
    fn clone(&self) -> xcb_xf86dri_query_direct_rendering_capable_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xf86dri_open_connection_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_xf86dri_open_connection_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub screen :         u32
}

impl Copy for xcb_xf86dri_open_connection_request_t {}
impl Clone for xcb_xf86dri_open_connection_request_t {
    fn clone(&self) -> xcb_xf86dri_open_connection_request_t { *self }
}

#[repr(C)]
pub struct xcb_xf86dri_open_connection_reply_t {
     pub response_type :       u8,
     pub pad0 :                u8,
     pub sequence :            u16,
     pub length :              u32,
     pub sarea_handle_low :    u32,
     pub sarea_handle_high :   u32,
     pub bus_id_len :          u32,
     pub pad1 :                [u8; 12]
}

impl Copy for xcb_xf86dri_open_connection_reply_t {}
impl Clone for xcb_xf86dri_open_connection_reply_t {
    fn clone(&self) -> xcb_xf86dri_open_connection_reply_t { *self }
}


#[repr(C)]
pub struct xcb_xf86dri_close_connection_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub screen :         u32
}

impl Copy for xcb_xf86dri_close_connection_request_t {}
impl Clone for xcb_xf86dri_close_connection_request_t {
    fn clone(&self) -> xcb_xf86dri_close_connection_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xf86dri_get_client_driver_name_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_xf86dri_get_client_driver_name_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub screen :         u32
}

impl Copy for xcb_xf86dri_get_client_driver_name_request_t {}
impl Clone for xcb_xf86dri_get_client_driver_name_request_t {
    fn clone(&self) -> xcb_xf86dri_get_client_driver_name_request_t { *self }
}

#[repr(C)]
pub struct xcb_xf86dri_get_client_driver_name_reply_t {
     pub response_type :                 u8,
     pub pad0 :                          u8,
     pub sequence :                      u16,
     pub length :                        u32,
     pub client_driver_major_version :   u32,
     pub client_driver_minor_version :   u32,
     pub client_driver_patch_version :   u32,
     pub client_driver_name_len :        u32,
     pub pad1 :                          [u8; 8]
}

impl Copy for xcb_xf86dri_get_client_driver_name_reply_t {}
impl Clone for xcb_xf86dri_get_client_driver_name_reply_t {
    fn clone(&self) -> xcb_xf86dri_get_client_driver_name_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xf86dri_create_context_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_xf86dri_create_context_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub screen :         u32,
     pub visual :         u32,
     pub context :        u32
}

impl Copy for xcb_xf86dri_create_context_request_t {}
impl Clone for xcb_xf86dri_create_context_request_t {
    fn clone(&self) -> xcb_xf86dri_create_context_request_t { *self }
}

#[repr(C)]
pub struct xcb_xf86dri_create_context_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub hw_context :      u32
}

impl Copy for xcb_xf86dri_create_context_reply_t {}
impl Clone for xcb_xf86dri_create_context_reply_t {
    fn clone(&self) -> xcb_xf86dri_create_context_reply_t { *self }
}


#[repr(C)]
pub struct xcb_xf86dri_destroy_context_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub screen :         u32,
     pub context :        u32
}

impl Copy for xcb_xf86dri_destroy_context_request_t {}
impl Clone for xcb_xf86dri_destroy_context_request_t {
    fn clone(&self) -> xcb_xf86dri_destroy_context_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xf86dri_create_drawable_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_xf86dri_create_drawable_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub screen :         u32,
     pub drawable :       u32
}

impl Copy for xcb_xf86dri_create_drawable_request_t {}
impl Clone for xcb_xf86dri_create_drawable_request_t {
    fn clone(&self) -> xcb_xf86dri_create_drawable_request_t { *self }
}

#[repr(C)]
pub struct xcb_xf86dri_create_drawable_reply_t {
     pub response_type :        u8,
     pub pad0 :                 u8,
     pub sequence :             u16,
     pub length :               u32,
     pub hw_drawable_handle :   u32
}

impl Copy for xcb_xf86dri_create_drawable_reply_t {}
impl Clone for xcb_xf86dri_create_drawable_reply_t {
    fn clone(&self) -> xcb_xf86dri_create_drawable_reply_t { *self }
}


#[repr(C)]
pub struct xcb_xf86dri_destroy_drawable_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub screen :         u32,
     pub drawable :       u32
}

impl Copy for xcb_xf86dri_destroy_drawable_request_t {}
impl Clone for xcb_xf86dri_destroy_drawable_request_t {
    fn clone(&self) -> xcb_xf86dri_destroy_drawable_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xf86dri_get_drawable_info_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_xf86dri_get_drawable_info_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub screen :         u32,
     pub drawable :       u32
}

impl Copy for xcb_xf86dri_get_drawable_info_request_t {}
impl Clone for xcb_xf86dri_get_drawable_info_request_t {
    fn clone(&self) -> xcb_xf86dri_get_drawable_info_request_t { *self }
}

#[repr(C)]
pub struct xcb_xf86dri_get_drawable_info_reply_t {
     pub response_type :          u8,
     pub pad0 :                   u8,
     pub sequence :               u16,
     pub length :                 u32,
     pub drawable_table_index :   u32,
     pub drawable_table_stamp :   u32,
     pub drawable_origin_X :      i16,
     pub drawable_origin_Y :      i16,
     pub drawable_size_W :        i16,
     pub drawable_size_H :        i16,
     pub num_clip_rects :         u32,
     pub back_x :                 i16,
     pub back_y :                 i16,
     pub num_back_clip_rects :    u32
}

impl Copy for xcb_xf86dri_get_drawable_info_reply_t {}
impl Clone for xcb_xf86dri_get_drawable_info_reply_t {
    fn clone(&self) -> xcb_xf86dri_get_drawable_info_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xf86dri_get_device_info_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_xf86dri_get_device_info_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub screen :         u32
}

impl Copy for xcb_xf86dri_get_device_info_request_t {}
impl Clone for xcb_xf86dri_get_device_info_request_t {
    fn clone(&self) -> xcb_xf86dri_get_device_info_request_t { *self }
}

#[repr(C)]
pub struct xcb_xf86dri_get_device_info_reply_t {
     pub response_type :               u8,
     pub pad0 :                        u8,
     pub sequence :                    u16,
     pub length :                      u32,
     pub framebuffer_handle_low :      u32,
     pub framebuffer_handle_high :     u32,
     pub framebuffer_origin_offset :   u32,
     pub framebuffer_size :            u32,
     pub framebuffer_stride :          u32,
     pub device_private_size :         u32
}

impl Copy for xcb_xf86dri_get_device_info_reply_t {}
impl Clone for xcb_xf86dri_get_device_info_reply_t {
    fn clone(&self) -> xcb_xf86dri_get_device_info_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xf86dri_auth_connection_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_xf86dri_auth_connection_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub screen :         u32,
     pub magic :          u32
}

impl Copy for xcb_xf86dri_auth_connection_request_t {}
impl Clone for xcb_xf86dri_auth_connection_request_t {
    fn clone(&self) -> xcb_xf86dri_auth_connection_request_t { *self }
}

#[repr(C)]
pub struct xcb_xf86dri_auth_connection_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub authenticated :   u32
}

impl Copy for xcb_xf86dri_auth_connection_reply_t {}
impl Clone for xcb_xf86dri_auth_connection_reply_t {
    fn clone(&self) -> xcb_xf86dri_auth_connection_reply_t { *self }
}
#[link(name="xcb-xf86dri")]
extern "C" {

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xf86dri_drm_clip_rect_t)
///
pub fn xcb_xf86dri_drm_clip_rect_next (i:*mut xcb_xf86dri_drm_clip_rect_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xf86dri_drm_clip_rect_end (i:xcb_xf86dri_drm_clip_rect_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/// Delivers a request to the X server.
///
pub fn xcb_xf86dri_query_version (c : *mut ffi::base::xcb_connection_t) -> xcb_xf86dri_query_version_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_xf86dri_query_version_unchecked (c : *mut ffi::base::xcb_connection_t) -> xcb_xf86dri_query_version_cookie_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_xf86dri_query_version_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_xf86dri_query_version_reply (c : *mut ffi::base::xcb_connection_t,
                                           cookie : xcb_xf86dri_query_version_cookie_t,
                                           e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_xf86dri_query_version_reply_t;

/// Delivers a request to the X server.
///
pub fn xcb_xf86dri_query_direct_rendering_capable (c : *mut ffi::base::xcb_connection_t,
                                                      screen :  u32) -> xcb_xf86dri_query_direct_rendering_capable_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_xf86dri_query_direct_rendering_capable_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                                screen :  u32) -> xcb_xf86dri_query_direct_rendering_capable_cookie_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_xf86dri_query_direct_rendering_capable_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_xf86dri_query_direct_rendering_capable_reply (c : *mut ffi::base::xcb_connection_t,
                                                            cookie : xcb_xf86dri_query_direct_rendering_capable_cookie_t,
                                                            e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_xf86dri_query_direct_rendering_capable_reply_t;

pub fn xcb_xf86dri_open_connection_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
///
pub fn xcb_xf86dri_open_connection (c : *mut ffi::base::xcb_connection_t,
                                       screen :  u32) -> xcb_xf86dri_open_connection_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_xf86dri_open_connection_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                 screen :  u32) -> xcb_xf86dri_open_connection_cookie_t;

pub fn xcb_xf86dri_open_connection_bus_id (R : *mut xcb_xf86dri_open_connection_reply_t) -> *mut c_char;


pub fn xcb_xf86dri_open_connection_bus_id_length (R : *mut xcb_xf86dri_open_connection_reply_t) -> c_int;


pub fn xcb_xf86dri_open_connection_bus_id_end (R : *mut xcb_xf86dri_open_connection_reply_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_xf86dri_open_connection_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_xf86dri_open_connection_reply (c : *mut ffi::base::xcb_connection_t,
                                             cookie : xcb_xf86dri_open_connection_cookie_t,
                                             e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_xf86dri_open_connection_reply_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_xf86dri_close_connection_checked (c : *mut ffi::base::xcb_connection_t,
                                                screen :  u32) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_xf86dri_close_connection (c : *mut ffi::base::xcb_connection_t,
                                        screen :  u32) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_xf86dri_get_client_driver_name_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
///
pub fn xcb_xf86dri_get_client_driver_name (c : *mut ffi::base::xcb_connection_t,
                                              screen :  u32) -> xcb_xf86dri_get_client_driver_name_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_xf86dri_get_client_driver_name_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                        screen :  u32) -> xcb_xf86dri_get_client_driver_name_cookie_t;

pub fn xcb_xf86dri_get_client_driver_name_client_driver_name (R : *mut xcb_xf86dri_get_client_driver_name_reply_t) -> *mut c_char;


pub fn xcb_xf86dri_get_client_driver_name_client_driver_name_length (R : *mut xcb_xf86dri_get_client_driver_name_reply_t) -> c_int;


pub fn xcb_xf86dri_get_client_driver_name_client_driver_name_end (R : *mut xcb_xf86dri_get_client_driver_name_reply_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_xf86dri_get_client_driver_name_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_xf86dri_get_client_driver_name_reply (c : *mut ffi::base::xcb_connection_t,
                                                    cookie : xcb_xf86dri_get_client_driver_name_cookie_t,
                                                    e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_xf86dri_get_client_driver_name_reply_t;

/// Delivers a request to the X server.
///
pub fn xcb_xf86dri_create_context (c : *mut ffi::base::xcb_connection_t,
                                      screen :  u32,
                                      visual :  u32,
                                      context :  u32) -> xcb_xf86dri_create_context_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_xf86dri_create_context_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                screen :  u32,
                                                visual :  u32,
                                                context :  u32) -> xcb_xf86dri_create_context_cookie_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_xf86dri_create_context_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_xf86dri_create_context_reply (c : *mut ffi::base::xcb_connection_t,
                                            cookie : xcb_xf86dri_create_context_cookie_t,
                                            e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_xf86dri_create_context_reply_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_xf86dri_destroy_context_checked (c : *mut ffi::base::xcb_connection_t,
                                               screen :  u32,
                                               context :  u32) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_xf86dri_destroy_context (c : *mut ffi::base::xcb_connection_t,
                                       screen :  u32,
                                       context :  u32) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_xf86dri_create_drawable (c : *mut ffi::base::xcb_connection_t,
                                       screen :  u32,
                                       drawable :  u32) -> xcb_xf86dri_create_drawable_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_xf86dri_create_drawable_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                 screen :  u32,
                                                 drawable :  u32) -> xcb_xf86dri_create_drawable_cookie_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_xf86dri_create_drawable_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_xf86dri_create_drawable_reply (c : *mut ffi::base::xcb_connection_t,
                                             cookie : xcb_xf86dri_create_drawable_cookie_t,
                                             e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_xf86dri_create_drawable_reply_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_xf86dri_destroy_drawable_checked (c : *mut ffi::base::xcb_connection_t,
                                                screen :  u32,
                                                drawable :  u32) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_xf86dri_destroy_drawable (c : *mut ffi::base::xcb_connection_t,
                                        screen :  u32,
                                        drawable :  u32) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_xf86dri_get_drawable_info_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
///
pub fn xcb_xf86dri_get_drawable_info (c : *mut ffi::base::xcb_connection_t,
                                         screen :  u32,
                                         drawable :  u32) -> xcb_xf86dri_get_drawable_info_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_xf86dri_get_drawable_info_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                   screen :  u32,
                                                   drawable :  u32) -> xcb_xf86dri_get_drawable_info_cookie_t;

pub fn xcb_xf86dri_get_drawable_info_clip_rects (R : *mut xcb_xf86dri_get_drawable_info_reply_t) -> *mut xcb_xf86dri_drm_clip_rect_t;


pub fn xcb_xf86dri_get_drawable_info_clip_rects_length (R : *mut xcb_xf86dri_get_drawable_info_reply_t) -> c_int;

pub fn xcb_xf86dri_get_drawable_info_clip_rects_iterator (R : *mut xcb_xf86dri_get_drawable_info_reply_t) -> xcb_xf86dri_drm_clip_rect_iterator_t;

pub fn xcb_xf86dri_get_drawable_info_back_clip_rects (R : *mut xcb_xf86dri_get_drawable_info_reply_t) -> *mut xcb_xf86dri_drm_clip_rect_t;


pub fn xcb_xf86dri_get_drawable_info_back_clip_rects_length (R : *mut xcb_xf86dri_get_drawable_info_reply_t) -> c_int;

pub fn xcb_xf86dri_get_drawable_info_back_clip_rects_iterator (R : *mut xcb_xf86dri_get_drawable_info_reply_t) -> xcb_xf86dri_drm_clip_rect_iterator_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_xf86dri_get_drawable_info_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_xf86dri_get_drawable_info_reply (c : *mut ffi::base::xcb_connection_t,
                                               cookie : xcb_xf86dri_get_drawable_info_cookie_t,
                                               e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_xf86dri_get_drawable_info_reply_t;

pub fn xcb_xf86dri_get_device_info_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
///
pub fn xcb_xf86dri_get_device_info (c : *mut ffi::base::xcb_connection_t,
                                       screen :  u32) -> xcb_xf86dri_get_device_info_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_xf86dri_get_device_info_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                 screen :  u32) -> xcb_xf86dri_get_device_info_cookie_t;

pub fn xcb_xf86dri_get_device_info_device_private (R : *mut xcb_xf86dri_get_device_info_reply_t) -> *mut u32;


pub fn xcb_xf86dri_get_device_info_device_private_length (R : *mut xcb_xf86dri_get_device_info_reply_t) -> c_int;


pub fn xcb_xf86dri_get_device_info_device_private_end (R : *mut xcb_xf86dri_get_device_info_reply_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_xf86dri_get_device_info_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_xf86dri_get_device_info_reply (c : *mut ffi::base::xcb_connection_t,
                                             cookie : xcb_xf86dri_get_device_info_cookie_t,
                                             e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_xf86dri_get_device_info_reply_t;

/// Delivers a request to the X server.
///
pub fn xcb_xf86dri_auth_connection (c : *mut ffi::base::xcb_connection_t,
                                       screen :  u32,
                                       magic :  u32) -> xcb_xf86dri_auth_connection_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_xf86dri_auth_connection_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                 screen :  u32,
                                                 magic :  u32) -> xcb_xf86dri_auth_connection_cookie_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_xf86dri_auth_connection_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_xf86dri_auth_connection_reply (c : *mut ffi::base::xcb_connection_t,
                                             cookie : xcb_xf86dri_auth_connection_cookie_t,
                                             e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_xf86dri_auth_connection_reply_t;
}

