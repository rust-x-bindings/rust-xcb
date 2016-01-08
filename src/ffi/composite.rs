/*
 * This file generated automatically from composite.xml by r_client.py.
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

pub static COMPOSITE_MAJOR_VERSION : c_uint = 0;
pub static COMPOSITE_MINOR_VERSION : c_uint = 3;

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
pub struct redirect_window_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub window :         ffi::xproto::window,
     pub update :         u8,
     pub pad0 :           [u8; 3]
}

impl Copy for redirect_window_request {}
impl Clone for redirect_window_request {
    fn clone(&self) -> redirect_window_request { *self }
}


#[repr(C)]
pub struct redirect_subwindows_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub window :         ffi::xproto::window,
     pub update :         u8,
     pub pad0 :           [u8; 3]
}

impl Copy for redirect_subwindows_request {}
impl Clone for redirect_subwindows_request {
    fn clone(&self) -> redirect_subwindows_request { *self }
}


#[repr(C)]
pub struct unredirect_window_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub window :         ffi::xproto::window,
     pub update :         u8,
     pub pad0 :           [u8; 3]
}

impl Copy for unredirect_window_request {}
impl Clone for unredirect_window_request {
    fn clone(&self) -> unredirect_window_request { *self }
}


#[repr(C)]
pub struct unredirect_subwindows_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub window :         ffi::xproto::window,
     pub update :         u8,
     pub pad0 :           [u8; 3]
}

impl Copy for unredirect_subwindows_request {}
impl Clone for unredirect_subwindows_request {
    fn clone(&self) -> unredirect_subwindows_request { *self }
}


#[repr(C)]
pub struct create_region_from_border_clip_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub region :         ffi::xfixes::region,
     pub window :         ffi::xproto::window
}

impl Copy for create_region_from_border_clip_request {}
impl Clone for create_region_from_border_clip_request {
    fn clone(&self) -> create_region_from_border_clip_request { *self }
}


#[repr(C)]
pub struct name_window_pixmap_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub window :         ffi::xproto::window,
     pub pixmap :         ffi::xproto::pixmap
}

impl Copy for name_window_pixmap_request {}
impl Clone for name_window_pixmap_request {
    fn clone(&self) -> name_window_pixmap_request { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct get_overlay_window_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct get_overlay_window_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub window :         ffi::xproto::window
}

impl Copy for get_overlay_window_request {}
impl Clone for get_overlay_window_request {
    fn clone(&self) -> get_overlay_window_request { *self }
}

#[repr(C)]
pub struct get_overlay_window_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub overlay_win :     ffi::xproto::window,
     pub pad1 :            [u8; 20]
}

impl Copy for get_overlay_window_reply {}
impl Clone for get_overlay_window_reply {
    fn clone(&self) -> get_overlay_window_reply { *self }
}


#[repr(C)]
pub struct release_overlay_window_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub window :         ffi::xproto::window
}

impl Copy for release_overlay_window_request {}
impl Clone for release_overlay_window_request {
    fn clone(&self) -> release_overlay_window_request { *self }
}
#[link(name="xcb-composite")]
extern "C" {

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_composite_query_version (c : *mut ffi::base::connection,
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
pub fn xcb_composite_query_version_unchecked (c : *mut ffi::base::connection,
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
 * xcb_composite_query_version_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_composite_query_version_reply (c : *mut ffi::base::connection,
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
pub fn xcb_composite_redirect_window_checked (c : *mut ffi::base::connection,
                                                 window :  ffi::xproto::window,
                                                 update :  u8) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_composite_redirect_window (c : *mut ffi::base::connection,
                                         window :  ffi::xproto::window,
                                         update :  u8) -> ffi::base::void_cookie;

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
pub fn xcb_composite_redirect_subwindows_checked (c : *mut ffi::base::connection,
                                                     window :  ffi::xproto::window,
                                                     update :  u8) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_composite_redirect_subwindows (c : *mut ffi::base::connection,
                                             window :  ffi::xproto::window,
                                             update :  u8) -> ffi::base::void_cookie;

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
pub fn xcb_composite_unredirect_window_checked (c : *mut ffi::base::connection,
                                                   window :  ffi::xproto::window,
                                                   update :  u8) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_composite_unredirect_window (c : *mut ffi::base::connection,
                                           window :  ffi::xproto::window,
                                           update :  u8) -> ffi::base::void_cookie;

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
pub fn xcb_composite_unredirect_subwindows_checked (c : *mut ffi::base::connection,
                                                       window :  ffi::xproto::window,
                                                       update :  u8) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_composite_unredirect_subwindows (c : *mut ffi::base::connection,
                                               window :  ffi::xproto::window,
                                               update :  u8) -> ffi::base::void_cookie;

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
pub fn xcb_composite_create_region_from_border_clip_checked (c : *mut ffi::base::connection,
                                                                region :  ffi::xfixes::region,
                                                                window :  ffi::xproto::window) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_composite_create_region_from_border_clip (c : *mut ffi::base::connection,
                                                        region :  ffi::xfixes::region,
                                                        window :  ffi::xproto::window) -> ffi::base::void_cookie;

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
pub fn xcb_composite_name_window_pixmap_checked (c : *mut ffi::base::connection,
                                                    window :  ffi::xproto::window,
                                                    pixmap :  ffi::xproto::pixmap) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_composite_name_window_pixmap (c : *mut ffi::base::connection,
                                            window :  ffi::xproto::window,
                                            pixmap :  ffi::xproto::pixmap) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_composite_get_overlay_window (c : *mut ffi::base::connection,
                                            window :  ffi::xproto::window) -> get_overlay_window_cookie;

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
pub fn xcb_composite_get_overlay_window_unchecked (c : *mut ffi::base::connection,
                                                      window :  ffi::xproto::window) -> get_overlay_window_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_composite_get_overlay_window_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_composite_get_overlay_window_reply (c : *mut ffi::base::connection,
                                                  cookie : get_overlay_window_cookie,
                                                  e : *mut *mut ffi::base::generic_error) -> *mut get_overlay_window_reply;

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
pub fn xcb_composite_release_overlay_window_checked (c : *mut ffi::base::connection,
                                                        window :  ffi::xproto::window) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_composite_release_overlay_window (c : *mut ffi::base::connection,
                                                window :  ffi::xproto::window) -> ffi::base::void_cookie;
}

