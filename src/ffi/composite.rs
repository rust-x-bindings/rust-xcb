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



pub struct redirect_window_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    window :         ffi::xproto::window,
    update :         u8,
    pad0 :           [u8,..3]
}



pub struct redirect_subwindows_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    window :         ffi::xproto::window,
    update :         u8,
    pad0 :           [u8,..3]
}



pub struct unredirect_window_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    window :         ffi::xproto::window,
    update :         u8,
    pad0 :           [u8,..3]
}



pub struct unredirect_subwindows_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    window :         ffi::xproto::window,
    update :         u8,
    pad0 :           [u8,..3]
}



pub struct create_region_from_border_clip_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    region :         ffi::xfixes::region,
    window :         ffi::xproto::window
}



pub struct name_window_pixmap_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    window :         ffi::xproto::window,
    pixmap :         ffi::xproto::pixmap
}


pub struct get_overlay_window_cookie {
    sequence : c_uint
}


pub struct get_overlay_window_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    window :         ffi::xproto::window
}


pub struct get_overlay_window_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    overlay_win :     ffi::xproto::window,
    pad1 :            [u8,..20]
}



pub struct release_overlay_window_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    window :         ffi::xproto::window
}

#[link(name="lxcb-composite")]
pub extern "C" {

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

