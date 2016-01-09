/*
 * This file generated automatically from screensaver.xml by r_client.py.
 * Edit at your peril.
 */

//Make the compiler quiet
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use std;
use libc::*;
use ffi;
use ffi::xproto;

pub const SCREENSAVER_MAJOR_VERSION : c_uint = 1;
pub const SCREENSAVER_MINOR_VERSION : c_uint = 1;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_screensaver_query_version_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_screensaver_query_version_request_t {
     pub major_opcode :           u8,
     pub minor_opcode :           u8,
     pub length :                 u16,
     pub client_major_version :   u8,
     pub client_minor_version :   u8,
     pub pad0 :                   [u8; 2]
}

impl Copy for xcb_screensaver_query_version_request_t {}
impl Clone for xcb_screensaver_query_version_request_t {
    fn clone(&self) -> xcb_screensaver_query_version_request_t { *self }
}

#[repr(C)]
pub struct xcb_screensaver_query_version_reply_t {
     pub response_type :          u8,
     pub pad0 :                   u8,
     pub sequence :               u16,
     pub length :                 u32,
     pub server_major_version :   u16,
     pub server_minor_version :   u16,
     pub pad1 :                   [u8; 20]
}

impl Copy for xcb_screensaver_query_version_reply_t {}
impl Clone for xcb_screensaver_query_version_reply_t {
    fn clone(&self) -> xcb_screensaver_query_version_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_screensaver_query_info_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_screensaver_query_info_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub drawable :       ffi::xproto::xcb_drawable_t
}

impl Copy for xcb_screensaver_query_info_request_t {}
impl Clone for xcb_screensaver_query_info_request_t {
    fn clone(&self) -> xcb_screensaver_query_info_request_t { *self }
}

#[repr(C)]
pub struct xcb_screensaver_query_info_reply_t {
     pub response_type :         u8,
     pub state :                 u8,
     pub sequence :              u16,
     pub length :                u32,
     pub saver_window :          ffi::xproto::xcb_window_t,
     pub ms_until_server :       u32,
     pub ms_since_user_input :   u32,
     pub event_mask :            u32,
     pub kind :                  u8,
     pub pad0 :                  [u8; 7]
}

impl Copy for xcb_screensaver_query_info_reply_t {}
impl Clone for xcb_screensaver_query_info_reply_t {
    fn clone(&self) -> xcb_screensaver_query_info_reply_t { *self }
}


#[repr(C)]
pub struct xcb_screensaver_select_input_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub drawable :       ffi::xproto::xcb_drawable_t,
     pub event_mask :     u32
}

impl Copy for xcb_screensaver_select_input_request_t {}
impl Clone for xcb_screensaver_select_input_request_t {
    fn clone(&self) -> xcb_screensaver_select_input_request_t { *self }
}


#[repr(C)]
pub struct xcb_screensaver_set_attributes_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub drawable :       ffi::xproto::xcb_drawable_t,
     pub x :              i16,
     pub y :              i16,
     pub width :          u16,
     pub height :         u16,
     pub border_width :   u16,
     pub class :          u8,
     pub depth :          u8,
     pub visual :         ffi::xproto::xcb_visualid_t,
     pub value_mask :     u32
}

impl Copy for xcb_screensaver_set_attributes_request_t {}
impl Clone for xcb_screensaver_set_attributes_request_t {
    fn clone(&self) -> xcb_screensaver_set_attributes_request_t { *self }
}


#[repr(C)]
pub struct xcb_screensaver_unset_attributes_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub drawable :       ffi::xproto::xcb_drawable_t
}

impl Copy for xcb_screensaver_unset_attributes_request_t {}
impl Clone for xcb_screensaver_unset_attributes_request_t {
    fn clone(&self) -> xcb_screensaver_unset_attributes_request_t { *self }
}


#[repr(C)]
pub struct xcb_screensaver_suspend_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub suspend :        u8,
     pub pad0 :           [u8; 3]
}

impl Copy for xcb_screensaver_suspend_request_t {}
impl Clone for xcb_screensaver_suspend_request_t {
    fn clone(&self) -> xcb_screensaver_suspend_request_t { *self }
}


#[repr(C)]
pub struct xcb_screensaver_notify_event_t {
     pub response_type :     u8,
     pub code :              u8,
     pub sequence :          u16,
     pub state :             u8,
     pub pad0 :              u8,
     pub sequence_number :   u16,
     pub time :              ffi::xproto::xcb_timestamp_t,
     pub root :              ffi::xproto::xcb_window_t,
     pub window :            ffi::xproto::xcb_window_t,
     pub kind :              u8,
     pub forced :            u8,
     pub pad1 :              [u8; 14]
}

impl Copy for xcb_screensaver_notify_event_t {}
impl Clone for xcb_screensaver_notify_event_t {
    fn clone(&self) -> xcb_screensaver_notify_event_t { *self }
}
#[link(name="xcb-screensaver")]
extern "C" {

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_screensaver_query_version (c : *mut ffi::base::xcb_connection_t,
                                         client_major_version :  u8,
                                         client_minor_version :  u8) -> xcb_screensaver_query_version_cookie_t;

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
pub fn xcb_screensaver_query_version_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                   client_major_version :  u8,
                                                   client_minor_version :  u8) -> xcb_screensaver_query_version_cookie_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_screensaver_query_version_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_screensaver_query_version_reply (c : *mut ffi::base::xcb_connection_t,
                                               cookie : xcb_screensaver_query_version_cookie_t,
                                               e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_screensaver_query_version_reply_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_screensaver_query_info (c : *mut ffi::base::xcb_connection_t,
                                      drawable :  ffi::xproto::xcb_drawable_t) -> xcb_screensaver_query_info_cookie_t;

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
pub fn xcb_screensaver_query_info_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                drawable :  ffi::xproto::xcb_drawable_t) -> xcb_screensaver_query_info_cookie_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_screensaver_query_info_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_screensaver_query_info_reply (c : *mut ffi::base::xcb_connection_t,
                                            cookie : xcb_screensaver_query_info_cookie_t,
                                            e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_screensaver_query_info_reply_t;

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
pub fn xcb_screensaver_select_input_checked (c : *mut ffi::base::xcb_connection_t,
                                                drawable :  ffi::xproto::xcb_drawable_t,
                                                event_mask :  u32) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_screensaver_select_input (c : *mut ffi::base::xcb_connection_t,
                                        drawable :  ffi::xproto::xcb_drawable_t,
                                        event_mask :  u32) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_screensaver_set_attributes_sizeof (_buffer :  *mut c_void) -> c_int;

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
pub fn xcb_screensaver_set_attributes_checked (c : *mut ffi::base::xcb_connection_t,
                                                  drawable :  ffi::xproto::xcb_drawable_t,
                                                  x :  i16,
                                                  y :  i16,
                                                  width :  u16,
                                                  height :  u16,
                                                  border_width :  u16,
                                                  class :  u8,
                                                  depth :  u8,
                                                  visual :  ffi::xproto::xcb_visualid_t,
                                                  value_mask :  u32,
                                                  value_list : *mut u32) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_screensaver_set_attributes (c : *mut ffi::base::xcb_connection_t,
                                          drawable :  ffi::xproto::xcb_drawable_t,
                                          x :  i16,
                                          y :  i16,
                                          width :  u16,
                                          height :  u16,
                                          border_width :  u16,
                                          class :  u8,
                                          depth :  u8,
                                          visual :  ffi::xproto::xcb_visualid_t,
                                          value_mask :  u32,
                                          value_list : *mut u32) -> ffi::base::xcb_void_cookie_t;

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
pub fn xcb_screensaver_unset_attributes_checked (c : *mut ffi::base::xcb_connection_t,
                                                    drawable :  ffi::xproto::xcb_drawable_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_screensaver_unset_attributes (c : *mut ffi::base::xcb_connection_t,
                                            drawable :  ffi::xproto::xcb_drawable_t) -> ffi::base::xcb_void_cookie_t;

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
pub fn xcb_screensaver_suspend_checked (c : *mut ffi::base::xcb_connection_t,
                                           suspend :  u8) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_screensaver_suspend (c : *mut ffi::base::xcb_connection_t,
                                   suspend :  u8) -> ffi::base::xcb_void_cookie_t;
}

