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

pub static SCREENSAVER_MAJOR_VERSION : c_uint = 1;
pub static SCREENSAVER_MINOR_VERSION : c_uint = 1;

pub struct query_version_cookie {
    sequence : c_uint
}


pub struct query_version_request {
     pub major_opcode :           u8,
     pub minor_opcode :           u8,
     pub length :                 u16,
     pub client_major_version :   u8,
     pub client_minor_version :   u8,
     pub pad0 :                   [u8,..2]
}


pub struct query_version_reply {
     pub response_type :          u8,
     pub pad0 :                   u8,
     pub sequence :               u16,
     pub length :                 u32,
     pub server_major_version :   u16,
     pub server_minor_version :   u16,
     pub pad1 :                   [u8,..20]
}


pub struct query_info_cookie {
    sequence : c_uint
}


pub struct query_info_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub drawable :       ffi::xproto::drawable
}


pub struct query_info_reply {
     pub response_type :         u8,
     pub state :                 u8,
     pub sequence :              u16,
     pub length :                u32,
     pub saver_window :          ffi::xproto::window,
     pub ms_until_server :       u32,
     pub ms_since_user_input :   u32,
     pub event_mask :            u32,
     pub kind :                  u8,
     pub pad0 :                  [u8,..7]
}



pub struct select_input_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub drawable :       ffi::xproto::drawable,
     pub event_mask :     u32
}



pub struct set_attributes_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub drawable :       ffi::xproto::drawable,
     pub x :              i16,
     pub y :              i16,
     pub width :          u16,
     pub height :         u16,
     pub border_width :   u16,
     pub class :          u8,
     pub depth :          u8,
     pub visual :         ffi::xproto::visualid,
     pub value_mask :     u32
}



pub struct unset_attributes_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub drawable :       ffi::xproto::drawable
}



pub struct suspend_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub suspend :        u8,
     pub pad0 :           [u8,..3]
}



pub struct notify_event {
     pub response_type :     u8,
     pub code :              u8,
     pub sequence :          u16,
     pub state :             u8,
     pub pad0 :              u8,
     pub sequence_number :   u16,
     pub time :              ffi::xproto::timestamp,
     pub root :              ffi::xproto::window,
     pub window :            ffi::xproto::window,
     pub kind :              u8,
     pub forced :            u8,
     pub pad1 :              [u8,..14]
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
pub fn xcb_screensaver_query_version (c : *mut ffi::base::connection,
                                         client_major_version :  u8,
                                         client_minor_version :  u8) -> query_version_cookie;

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
pub fn xcb_screensaver_query_version_unchecked (c : *mut ffi::base::connection,
                                                   client_major_version :  u8,
                                                   client_minor_version :  u8) -> query_version_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_screensaver_query_version_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_screensaver_query_version_reply (c : *mut ffi::base::connection,
                                               cookie : query_version_cookie,
                                               e : *mut *mut ffi::base::generic_error) -> *mut query_version_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_screensaver_query_info (c : *mut ffi::base::connection,
                                      drawable :  ffi::xproto::drawable) -> query_info_cookie;

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
pub fn xcb_screensaver_query_info_unchecked (c : *mut ffi::base::connection,
                                                drawable :  ffi::xproto::drawable) -> query_info_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_screensaver_query_info_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_screensaver_query_info_reply (c : *mut ffi::base::connection,
                                            cookie : query_info_cookie,
                                            e : *mut *mut ffi::base::generic_error) -> *mut query_info_reply;

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
pub fn xcb_screensaver_select_input_checked (c : *mut ffi::base::connection,
                                                drawable :  ffi::xproto::drawable,
                                                event_mask :  u32) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_screensaver_select_input (c : *mut ffi::base::connection,
                                        drawable :  ffi::xproto::drawable,
                                        event_mask :  u32) -> ffi::base::void_cookie;

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
pub fn xcb_screensaver_set_attributes_checked (c : *mut ffi::base::connection,
                                                  drawable :  ffi::xproto::drawable,
                                                  x :  i16,
                                                  y :  i16,
                                                  width :  u16,
                                                  height :  u16,
                                                  border_width :  u16,
                                                  class :  u8,
                                                  depth :  u8,
                                                  visual :  ffi::xproto::visualid,
                                                  value_mask :  u32,
                                                  value_list : *mut u32) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_screensaver_set_attributes (c : *mut ffi::base::connection,
                                          drawable :  ffi::xproto::drawable,
                                          x :  i16,
                                          y :  i16,
                                          width :  u16,
                                          height :  u16,
                                          border_width :  u16,
                                          class :  u8,
                                          depth :  u8,
                                          visual :  ffi::xproto::visualid,
                                          value_mask :  u32,
                                          value_list : *mut u32) -> ffi::base::void_cookie;

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
pub fn xcb_screensaver_unset_attributes_checked (c : *mut ffi::base::connection,
                                                    drawable :  ffi::xproto::drawable) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_screensaver_unset_attributes (c : *mut ffi::base::connection,
                                            drawable :  ffi::xproto::drawable) -> ffi::base::void_cookie;

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
pub fn xcb_screensaver_suspend_checked (c : *mut ffi::base::connection,
                                           suspend :  u8) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_screensaver_suspend (c : *mut ffi::base::connection,
                                   suspend :  u8) -> ffi::base::void_cookie;
}

