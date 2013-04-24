/*
 * This file generated automatically from screensaver.xml by r_client.py.
 * Edit at your peril.
 */

//Make the compiler quiet
#[allow(unused_imports)];
#[allow(non_camel_case_types)];
use core;
use core::libc::*;
use ll::base::*;
use ll;
use ll::xproto;

pub static SCREENSAVER_MAJOR_VERSION : c_uint = 1;
pub static SCREENSAVER_MINOR_VERSION : c_uint = 1;

pub struct query_version_cookie {
    sequence : c_uint
}


pub struct query_version_request {
    major_opcode :           u8,
    minor_opcode :           u8,
    length :                 u16,
    client_major_version :   u8,
    client_minor_version :   u8,
    pad0 :                   [u8,..2]
}


pub struct query_version_reply {
    response_type :          u8,
    pad0 :                   u8,
    sequence :               u16,
    length :                 u32,
    server_major_version :   u16,
    server_minor_version :   u16,
    pad1 :                   [u8,..20]
}


pub struct query_info_cookie {
    sequence : c_uint
}


pub struct query_info_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    drawable :       ll::xproto::drawable
}


pub struct query_info_reply {
    response_type :         u8,
    state :                 u8,
    sequence :              u16,
    length :                u32,
    saver_window :          ll::xproto::window,
    ms_until_server :       u32,
    ms_since_user_input :   u32,
    event_mask :            u32,
    kind :                  u8,
    pad0 :                  [u8,..7]
}



pub struct select_input_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    drawable :       ll::xproto::drawable,
    event_mask :     u32
}



pub struct set_attributes_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    drawable :       ll::xproto::drawable,
    x :              i16,
    y :              i16,
    width :          u16,
    height :         u16,
    border_width :   u16,
    class :          u8,
    depth :          u8,
    visual :         ll::xproto::visualid,
    value_mask :     u32
}



pub struct unset_attributes_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    drawable :       ll::xproto::drawable
}



pub struct suspend_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    suspend :        u8,
    pad0 :           [u8,..3]
}



pub struct notify_event {
    response_type :     u8,
    code :              u8,
    sequence :          u16,
    state :             u8,
    pad0 :              u8,
    sequence_number :   u16,
    time :              ll::xproto::timestamp,
    root :              ll::xproto::window,
    window :            ll::xproto::window,
    kind :              u8,
    forced :            u8,
    pad1 :              [u8,..14]
}

#[link_args="-lxcb-screensaver"]
pub extern "C" {

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_screensaver_query_version (c : *connection,
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
unsafe fn xcb_screensaver_query_version_unchecked (c : *connection,
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
unsafe fn xcb_screensaver_query_version_reply (c : *connection,
                                               cookie : query_version_cookie,
                                               e : **generic_error) -> *query_version_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_screensaver_query_info (c : *connection,
                                      drawable :  ll::xproto::drawable) -> query_info_cookie;

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
unsafe fn xcb_screensaver_query_info_unchecked (c : *connection,
                                                drawable :  ll::xproto::drawable) -> query_info_cookie;

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
unsafe fn xcb_screensaver_query_info_reply (c : *connection,
                                            cookie : query_info_cookie,
                                            e : **generic_error) -> *query_info_reply;

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
unsafe fn xcb_screensaver_select_input_checked (c : *connection,
                                                drawable :  ll::xproto::drawable,
                                                event_mask :  u32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_screensaver_select_input (c : *connection,
                                        drawable :  ll::xproto::drawable,
                                        event_mask :  u32) -> void_cookie;

unsafe fn xcb_screensaver_set_attributes_sizeof (_buffer :  *c_void) -> c_int;

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
unsafe fn xcb_screensaver_set_attributes_checked (c : *connection,
                                                  drawable :  ll::xproto::drawable,
                                                  x :  i16,
                                                  y :  i16,
                                                  width :  u16,
                                                  height :  u16,
                                                  border_width :  u16,
                                                  class :  u8,
                                                  depth :  u8,
                                                  visual :  ll::xproto::visualid,
                                                  value_mask :  u32,
                                                  value_list : *u32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_screensaver_set_attributes (c : *connection,
                                          drawable :  ll::xproto::drawable,
                                          x :  i16,
                                          y :  i16,
                                          width :  u16,
                                          height :  u16,
                                          border_width :  u16,
                                          class :  u8,
                                          depth :  u8,
                                          visual :  ll::xproto::visualid,
                                          value_mask :  u32,
                                          value_list : *u32) -> void_cookie;

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
unsafe fn xcb_screensaver_unset_attributes_checked (c : *connection,
                                                    drawable :  ll::xproto::drawable) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_screensaver_unset_attributes (c : *connection,
                                            drawable :  ll::xproto::drawable) -> void_cookie;

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
unsafe fn xcb_screensaver_suspend_checked (c : *connection,
                                           suspend :  u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_screensaver_suspend (c : *connection,
                                   suspend :  u8) -> void_cookie;
}

