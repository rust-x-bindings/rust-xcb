/*
 * This file generated automatically from xinerama.xml by r_client.py.
 * Edit at your peril.
 */

//Make the compiler quiet
#[allow(unused_imports)];
#[allow(non_camel_case_types)];
use std;
use std::libc::*;
use std::{cast,num,ptr,str,libc};
use std::to_bytes::ToBytes;
use ll::base::*;
use ll;
use ll::xproto;

pub static XINERAMA_MAJOR_VERSION : c_uint = 1;
pub static XINERAMA_MINOR_VERSION : c_uint = 1;

pub struct screen_info {
    x_org :    i16,
    y_org :    i16,
    width :    u16,
    height :   u16
}

/**
 * @brief screen_info_iterator
 **/
pub struct screen_info_iterator {
    data : *screen_info,
    rem  : c_int,
    index: c_int
}


pub struct query_version_cookie {
    sequence : c_uint
}


pub struct query_version_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    major :          u8,
    minor :          u8
}


pub struct query_version_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    major :           u16,
    minor :           u16
}


pub struct get_state_cookie {
    sequence : c_uint
}


pub struct get_state_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    window :         ll::xproto::window
}


pub struct get_state_reply {
    response_type :   u8,
    state :           u8,
    sequence :        u16,
    length :          u32,
    window :          ll::xproto::window
}


pub struct get_screen_count_cookie {
    sequence : c_uint
}


pub struct get_screen_count_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    window :         ll::xproto::window
}


pub struct get_screen_count_reply {
    response_type :   u8,
    screen_count :    u8,
    sequence :        u16,
    length :          u32,
    window :          ll::xproto::window
}


pub struct get_screen_size_cookie {
    sequence : c_uint
}


pub struct get_screen_size_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    window :         ll::xproto::window,
    screen :         u32
}


pub struct get_screen_size_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    width :           u32,
    height :          u32,
    window :          ll::xproto::window,
    screen :          u32
}


pub struct is_active_cookie {
    sequence : c_uint
}


pub struct is_active_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16
}


pub struct is_active_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    state :           u32
}


pub struct query_screens_cookie {
    sequence : c_uint
}


pub struct query_screens_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16
}


pub struct query_screens_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    number :          u32,
    pad1 :            [u8,..20]
}

#[link_args="-lxcb-xinerama"]
pub extern "C" {

/**
 * Get the next element of the iterator
 * @param i Pointer to a screen_info_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(screen_info)
 *
 *
 */
pub unsafe fn xcb_xinerama_screen_info_next (i:*screen_info_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An screen_info_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub unsafe fn xcb_xinerama_screen_info_end (i:screen_info_iterator) -> generic_iterator;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_xinerama_query_version (c : *connection,
                                      major :  u8,
                                      minor :  u8) -> query_version_cookie;

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
pub unsafe fn xcb_xinerama_query_version_unchecked (c : *connection,
                                                major :  u8,
                                                minor :  u8) -> query_version_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xinerama_query_version_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub unsafe fn xcb_xinerama_query_version_reply (c : *connection,
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
pub unsafe fn xcb_xinerama_get_state (c : *connection,
                                  window :  ll::xproto::window) -> get_state_cookie;

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
pub unsafe fn xcb_xinerama_get_state_unchecked (c : *connection,
                                            window :  ll::xproto::window) -> get_state_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xinerama_get_state_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub unsafe fn xcb_xinerama_get_state_reply (c : *connection,
                                        cookie : get_state_cookie,
                                        e : **generic_error) -> *get_state_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_xinerama_get_screen_count (c : *connection,
                                         window :  ll::xproto::window) -> get_screen_count_cookie;

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
pub unsafe fn xcb_xinerama_get_screen_count_unchecked (c : *connection,
                                                   window :  ll::xproto::window) -> get_screen_count_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xinerama_get_screen_count_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub unsafe fn xcb_xinerama_get_screen_count_reply (c : *connection,
                                               cookie : get_screen_count_cookie,
                                               e : **generic_error) -> *get_screen_count_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_xinerama_get_screen_size (c : *connection,
                                        window :  ll::xproto::window,
                                        screen :  u32) -> get_screen_size_cookie;

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
pub unsafe fn xcb_xinerama_get_screen_size_unchecked (c : *connection,
                                                  window :  ll::xproto::window,
                                                  screen :  u32) -> get_screen_size_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xinerama_get_screen_size_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub unsafe fn xcb_xinerama_get_screen_size_reply (c : *connection,
                                              cookie : get_screen_size_cookie,
                                              e : **generic_error) -> *get_screen_size_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_xinerama_is_active (c : *connection) -> is_active_cookie;

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
pub unsafe fn xcb_xinerama_is_active_unchecked (c : *connection) -> is_active_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xinerama_is_active_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub unsafe fn xcb_xinerama_is_active_reply (c : *connection,
                                        cookie : is_active_cookie,
                                        e : **generic_error) -> *is_active_reply;

pub unsafe fn xcb_xinerama_query_screens_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_xinerama_query_screens (c : *connection) -> query_screens_cookie;

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
pub unsafe fn xcb_xinerama_query_screens_unchecked (c : *connection) -> query_screens_cookie;

pub unsafe fn xcb_xinerama_query_screens_screen_info (R : *query_screens_reply) -> *screen_info;


pub unsafe fn xcb_xinerama_query_screens_screen_info_length (R : *query_screens_reply) -> c_int;

pub unsafe fn xcb_xinerama_query_screens_screen_info_iterator (R : *query_screens_reply) -> screen_info_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xinerama_query_screens_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub unsafe fn xcb_xinerama_query_screens_reply (c : *connection,
                                            cookie : query_screens_cookie,
                                            e : **generic_error) -> *query_screens_reply;
}

