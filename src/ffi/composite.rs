//
// This file generated automatically from composite.xml by r_client.py.
// Edit at your peril.
//

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

pub const COMPOSITE_MAJOR_VERSION : c_uint = 0;
pub const COMPOSITE_MINOR_VERSION : c_uint = 3;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_composite_query_version_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_composite_query_version_request_t {
     pub major_opcode :           u8,
     pub minor_opcode :           u8,
     pub length :                 u16,
     pub client_major_version :   u32,
     pub client_minor_version :   u32
}

impl Copy for xcb_composite_query_version_request_t {}
impl Clone for xcb_composite_query_version_request_t {
    fn clone(&self) -> xcb_composite_query_version_request_t { *self }
}

#[repr(C)]
pub struct xcb_composite_query_version_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub major_version :   u32,
     pub minor_version :   u32,
     pub pad1 :            [u8; 16]
}

impl Copy for xcb_composite_query_version_reply_t {}
impl Clone for xcb_composite_query_version_reply_t {
    fn clone(&self) -> xcb_composite_query_version_reply_t { *self }
}


#[repr(C)]
pub struct xcb_composite_redirect_window_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub window :         ffi::xproto::xcb_window_t,
     pub update :         u8,
     pub pad0 :           [u8; 3]
}

impl Copy for xcb_composite_redirect_window_request_t {}
impl Clone for xcb_composite_redirect_window_request_t {
    fn clone(&self) -> xcb_composite_redirect_window_request_t { *self }
}


#[repr(C)]
pub struct xcb_composite_redirect_subwindows_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub window :         ffi::xproto::xcb_window_t,
     pub update :         u8,
     pub pad0 :           [u8; 3]
}

impl Copy for xcb_composite_redirect_subwindows_request_t {}
impl Clone for xcb_composite_redirect_subwindows_request_t {
    fn clone(&self) -> xcb_composite_redirect_subwindows_request_t { *self }
}


#[repr(C)]
pub struct xcb_composite_unredirect_window_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub window :         ffi::xproto::xcb_window_t,
     pub update :         u8,
     pub pad0 :           [u8; 3]
}

impl Copy for xcb_composite_unredirect_window_request_t {}
impl Clone for xcb_composite_unredirect_window_request_t {
    fn clone(&self) -> xcb_composite_unredirect_window_request_t { *self }
}


#[repr(C)]
pub struct xcb_composite_unredirect_subwindows_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub window :         ffi::xproto::xcb_window_t,
     pub update :         u8,
     pub pad0 :           [u8; 3]
}

impl Copy for xcb_composite_unredirect_subwindows_request_t {}
impl Clone for xcb_composite_unredirect_subwindows_request_t {
    fn clone(&self) -> xcb_composite_unredirect_subwindows_request_t { *self }
}


#[repr(C)]
pub struct xcb_composite_create_region_from_border_clip_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub region :         ffi::xfixes::xcb_xfixes_region_t,
     pub window :         ffi::xproto::xcb_window_t
}

impl Copy for xcb_composite_create_region_from_border_clip_request_t {}
impl Clone for xcb_composite_create_region_from_border_clip_request_t {
    fn clone(&self) -> xcb_composite_create_region_from_border_clip_request_t { *self }
}


#[repr(C)]
pub struct xcb_composite_name_window_pixmap_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub window :         ffi::xproto::xcb_window_t,
     pub pixmap :         ffi::xproto::xcb_pixmap_t
}

impl Copy for xcb_composite_name_window_pixmap_request_t {}
impl Clone for xcb_composite_name_window_pixmap_request_t {
    fn clone(&self) -> xcb_composite_name_window_pixmap_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_composite_get_overlay_window_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_composite_get_overlay_window_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub window :         ffi::xproto::xcb_window_t
}

impl Copy for xcb_composite_get_overlay_window_request_t {}
impl Clone for xcb_composite_get_overlay_window_request_t {
    fn clone(&self) -> xcb_composite_get_overlay_window_request_t { *self }
}

#[repr(C)]
pub struct xcb_composite_get_overlay_window_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub overlay_win :     ffi::xproto::xcb_window_t,
     pub pad1 :            [u8; 20]
}

impl Copy for xcb_composite_get_overlay_window_reply_t {}
impl Clone for xcb_composite_get_overlay_window_reply_t {
    fn clone(&self) -> xcb_composite_get_overlay_window_reply_t { *self }
}


#[repr(C)]
pub struct xcb_composite_release_overlay_window_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub window :         ffi::xproto::xcb_window_t
}

impl Copy for xcb_composite_release_overlay_window_request_t {}
impl Clone for xcb_composite_release_overlay_window_request_t {
    fn clone(&self) -> xcb_composite_release_overlay_window_request_t { *self }
}
#[link(name="xcb-composite")]
extern "C" {

/// Delivers a request to the X server.
///
pub fn xcb_composite_query_version (c : *mut ffi::base::xcb_connection_t,
                                       client_major_version :  u32,
                                       client_minor_version :  u32) -> xcb_composite_query_version_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_composite_query_version_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                 client_major_version :  u32,
                                                 client_minor_version :  u32) -> xcb_composite_query_version_cookie_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_composite_query_version_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_composite_query_version_reply (c : *mut ffi::base::xcb_connection_t,
                                             cookie : xcb_composite_query_version_cookie_t,
                                             e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_composite_query_version_reply_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_composite_redirect_window_checked (c : *mut ffi::base::xcb_connection_t,
                                                 window :  ffi::xproto::xcb_window_t,
                                                 update :  u8) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_composite_redirect_window (c : *mut ffi::base::xcb_connection_t,
                                         window :  ffi::xproto::xcb_window_t,
                                         update :  u8) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_composite_redirect_subwindows_checked (c : *mut ffi::base::xcb_connection_t,
                                                     window :  ffi::xproto::xcb_window_t,
                                                     update :  u8) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_composite_redirect_subwindows (c : *mut ffi::base::xcb_connection_t,
                                             window :  ffi::xproto::xcb_window_t,
                                             update :  u8) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_composite_unredirect_window_checked (c : *mut ffi::base::xcb_connection_t,
                                                   window :  ffi::xproto::xcb_window_t,
                                                   update :  u8) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_composite_unredirect_window (c : *mut ffi::base::xcb_connection_t,
                                           window :  ffi::xproto::xcb_window_t,
                                           update :  u8) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_composite_unredirect_subwindows_checked (c : *mut ffi::base::xcb_connection_t,
                                                       window :  ffi::xproto::xcb_window_t,
                                                       update :  u8) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_composite_unredirect_subwindows (c : *mut ffi::base::xcb_connection_t,
                                               window :  ffi::xproto::xcb_window_t,
                                               update :  u8) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_composite_create_region_from_border_clip_checked (c : *mut ffi::base::xcb_connection_t,
                                                                region :  ffi::xfixes::xcb_xfixes_region_t,
                                                                window :  ffi::xproto::xcb_window_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_composite_create_region_from_border_clip (c : *mut ffi::base::xcb_connection_t,
                                                        region :  ffi::xfixes::xcb_xfixes_region_t,
                                                        window :  ffi::xproto::xcb_window_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_composite_name_window_pixmap_checked (c : *mut ffi::base::xcb_connection_t,
                                                    window :  ffi::xproto::xcb_window_t,
                                                    pixmap :  ffi::xproto::xcb_pixmap_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_composite_name_window_pixmap (c : *mut ffi::base::xcb_connection_t,
                                            window :  ffi::xproto::xcb_window_t,
                                            pixmap :  ffi::xproto::xcb_pixmap_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_composite_get_overlay_window (c : *mut ffi::base::xcb_connection_t,
                                            window :  ffi::xproto::xcb_window_t) -> xcb_composite_get_overlay_window_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_composite_get_overlay_window_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                      window :  ffi::xproto::xcb_window_t) -> xcb_composite_get_overlay_window_cookie_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_composite_get_overlay_window_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_composite_get_overlay_window_reply (c : *mut ffi::base::xcb_connection_t,
                                                  cookie : xcb_composite_get_overlay_window_cookie_t,
                                                  e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_composite_get_overlay_window_reply_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_composite_release_overlay_window_checked (c : *mut ffi::base::xcb_connection_t,
                                                        window :  ffi::xproto::xcb_window_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_composite_release_overlay_window (c : *mut ffi::base::xcb_connection_t,
                                                window :  ffi::xproto::xcb_window_t) -> ffi::base::xcb_void_cookie_t;
}

