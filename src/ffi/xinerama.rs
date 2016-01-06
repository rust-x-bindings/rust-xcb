/*
 * This file generated automatically from xinerama.xml by r_client.py.
 * Edit at your peril.
 */

//Make the compiler quiet
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use std;
use libc::*;
use ffi;
use ffi::xproto;

pub static XINERAMA_MAJOR_VERSION : c_uint = 1;
pub static XINERAMA_MINOR_VERSION : c_uint = 1;

#[repr(C)]
pub struct screen_info {
     pub x_org :    i16,
     pub y_org :    i16,
     pub width :    u16,
     pub height :   u16
}

impl Copy for screen_info {}
impl Clone for screen_info {
    fn clone(&self) -> screen_info { *self }
}
/**
 * @brief screen_info_iterator
 **/
#[repr(C)]
pub struct screen_info_iterator {
    pub data : *mut screen_info,
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
     pub length :         u16,
     pub major :          u8,
     pub minor :          u8
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
     pub major :           u16,
     pub minor :           u16
}

impl Copy for query_version_reply {}
impl Clone for query_version_reply {
    fn clone(&self) -> query_version_reply { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct get_state_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct get_state_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub window :         ffi::xproto::window
}

impl Copy for get_state_request {}
impl Clone for get_state_request {
    fn clone(&self) -> get_state_request { *self }
}

#[repr(C)]
pub struct get_state_reply {
     pub response_type :   u8,
     pub state :           u8,
     pub sequence :        u16,
     pub length :          u32,
     pub window :          ffi::xproto::window
}

impl Copy for get_state_reply {}
impl Clone for get_state_reply {
    fn clone(&self) -> get_state_reply { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct get_screen_count_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct get_screen_count_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub window :         ffi::xproto::window
}

impl Copy for get_screen_count_request {}
impl Clone for get_screen_count_request {
    fn clone(&self) -> get_screen_count_request { *self }
}

#[repr(C)]
pub struct get_screen_count_reply {
     pub response_type :   u8,
     pub screen_count :    u8,
     pub sequence :        u16,
     pub length :          u32,
     pub window :          ffi::xproto::window
}

impl Copy for get_screen_count_reply {}
impl Clone for get_screen_count_reply {
    fn clone(&self) -> get_screen_count_reply { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct get_screen_size_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct get_screen_size_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub window :         ffi::xproto::window,
     pub screen :         u32
}

impl Copy for get_screen_size_request {}
impl Clone for get_screen_size_request {
    fn clone(&self) -> get_screen_size_request { *self }
}

#[repr(C)]
pub struct get_screen_size_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub width :           u32,
     pub height :          u32,
     pub window :          ffi::xproto::window,
     pub screen :          u32
}

impl Copy for get_screen_size_reply {}
impl Clone for get_screen_size_reply {
    fn clone(&self) -> get_screen_size_reply { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct is_active_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct is_active_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16
}

impl Copy for is_active_request {}
impl Clone for is_active_request {
    fn clone(&self) -> is_active_request { *self }
}

#[repr(C)]
pub struct is_active_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub state :           u32
}

impl Copy for is_active_reply {}
impl Clone for is_active_reply {
    fn clone(&self) -> is_active_reply { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct query_screens_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct query_screens_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16
}

impl Copy for query_screens_request {}
impl Clone for query_screens_request {
    fn clone(&self) -> query_screens_request { *self }
}

#[repr(C)]
pub struct query_screens_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub number :          u32,
     pub pad1 :            [u8; 20]
}

impl Copy for query_screens_reply {}
impl Clone for query_screens_reply {
    fn clone(&self) -> query_screens_reply { *self }
}
#[link(name="xcb-xinerama")]
extern "C" {

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
pub fn xcb_xinerama_screen_info_next (i:*mut screen_info_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An screen_info_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_xinerama_screen_info_end (i:screen_info_iterator) -> ffi::base::generic_iterator;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xinerama_query_version (c : *mut ffi::base::connection,
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
pub fn xcb_xinerama_query_version_unchecked (c : *mut ffi::base::connection,
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
pub fn xcb_xinerama_query_version_reply (c : *mut ffi::base::connection,
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
pub fn xcb_xinerama_get_state (c : *mut ffi::base::connection,
                                  window :  ffi::xproto::window) -> get_state_cookie;

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
pub fn xcb_xinerama_get_state_unchecked (c : *mut ffi::base::connection,
                                            window :  ffi::xproto::window) -> get_state_cookie;

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
pub fn xcb_xinerama_get_state_reply (c : *mut ffi::base::connection,
                                        cookie : get_state_cookie,
                                        e : *mut *mut ffi::base::generic_error) -> *mut get_state_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xinerama_get_screen_count (c : *mut ffi::base::connection,
                                         window :  ffi::xproto::window) -> get_screen_count_cookie;

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
pub fn xcb_xinerama_get_screen_count_unchecked (c : *mut ffi::base::connection,
                                                   window :  ffi::xproto::window) -> get_screen_count_cookie;

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
pub fn xcb_xinerama_get_screen_count_reply (c : *mut ffi::base::connection,
                                               cookie : get_screen_count_cookie,
                                               e : *mut *mut ffi::base::generic_error) -> *mut get_screen_count_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xinerama_get_screen_size (c : *mut ffi::base::connection,
                                        window :  ffi::xproto::window,
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
pub fn xcb_xinerama_get_screen_size_unchecked (c : *mut ffi::base::connection,
                                                  window :  ffi::xproto::window,
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
pub fn xcb_xinerama_get_screen_size_reply (c : *mut ffi::base::connection,
                                              cookie : get_screen_size_cookie,
                                              e : *mut *mut ffi::base::generic_error) -> *mut get_screen_size_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xinerama_is_active (c : *mut ffi::base::connection) -> is_active_cookie;

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
pub fn xcb_xinerama_is_active_unchecked (c : *mut ffi::base::connection) -> is_active_cookie;

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
pub fn xcb_xinerama_is_active_reply (c : *mut ffi::base::connection,
                                        cookie : is_active_cookie,
                                        e : *mut *mut ffi::base::generic_error) -> *mut is_active_reply;

pub fn xcb_xinerama_query_screens_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xinerama_query_screens (c : *mut ffi::base::connection) -> query_screens_cookie;

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
pub fn xcb_xinerama_query_screens_unchecked (c : *mut ffi::base::connection) -> query_screens_cookie;

pub fn xcb_xinerama_query_screens_screen_info (R : *mut query_screens_reply) -> *mut screen_info;


pub fn xcb_xinerama_query_screens_screen_info_length (R : *mut query_screens_reply) -> c_int;

pub fn xcb_xinerama_query_screens_screen_info_iterator (R : *mut query_screens_reply) -> screen_info_iterator;

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
pub fn xcb_xinerama_query_screens_reply (c : *mut ffi::base::connection,
                                            cookie : query_screens_cookie,
                                            e : *mut *mut ffi::base::generic_error) -> *mut query_screens_reply;
}

