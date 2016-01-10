//
// This file generated automatically from xinerama.xml by r_client.py.
// Edit at your peril.
//

//Make the compiler quiet
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use std;
use libc::*;
use ffi;
use ffi::xproto;

pub const XINERAMA_MAJOR_VERSION : c_uint = 1;
pub const XINERAMA_MINOR_VERSION : c_uint = 1;

#[repr(C)]
pub struct xcb_xinerama_screen_info_t {
    pub x_org :    i16,
    pub y_org :    i16,
    pub width :    u16,
    pub height :   u16
}

impl Copy for xcb_xinerama_screen_info_t {}
impl Clone for xcb_xinerama_screen_info_t {
    fn clone(&self) -> xcb_xinerama_screen_info_t { *self }
}
#[repr(C)]
pub struct xcb_xinerama_screen_info_iterator_t {
    pub data : *mut xcb_xinerama_screen_info_t,
    pub rem  : c_int,
    pub index: c_int
}


#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xinerama_query_version_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_xinerama_query_version_request_t {
    pub major_opcode :   u8,
    pub minor_opcode :   u8,
    pub length :         u16,
    pub major :          u8,
    pub minor :          u8
}

impl Copy for xcb_xinerama_query_version_request_t {}
impl Clone for xcb_xinerama_query_version_request_t {
    fn clone(&self) -> xcb_xinerama_query_version_request_t { *self }
}

#[repr(C)]
pub struct xcb_xinerama_query_version_reply_t {
    pub response_type :   u8,
    pub pad0 :            u8,
    pub sequence :        u16,
    pub length :          u32,
    pub major :           u16,
    pub minor :           u16
}

impl Copy for xcb_xinerama_query_version_reply_t {}
impl Clone for xcb_xinerama_query_version_reply_t {
    fn clone(&self) -> xcb_xinerama_query_version_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xinerama_get_state_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_xinerama_get_state_request_t {
    pub major_opcode :   u8,
    pub minor_opcode :   u8,
    pub length :         u16,
    pub window :         ffi::xproto::xcb_window_t
}

impl Copy for xcb_xinerama_get_state_request_t {}
impl Clone for xcb_xinerama_get_state_request_t {
    fn clone(&self) -> xcb_xinerama_get_state_request_t { *self }
}

#[repr(C)]
pub struct xcb_xinerama_get_state_reply_t {
    pub response_type :   u8,
    pub state :           u8,
    pub sequence :        u16,
    pub length :          u32,
    pub window :          ffi::xproto::xcb_window_t
}

impl Copy for xcb_xinerama_get_state_reply_t {}
impl Clone for xcb_xinerama_get_state_reply_t {
    fn clone(&self) -> xcb_xinerama_get_state_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xinerama_get_screen_count_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_xinerama_get_screen_count_request_t {
    pub major_opcode :   u8,
    pub minor_opcode :   u8,
    pub length :         u16,
    pub window :         ffi::xproto::xcb_window_t
}

impl Copy for xcb_xinerama_get_screen_count_request_t {}
impl Clone for xcb_xinerama_get_screen_count_request_t {
    fn clone(&self) -> xcb_xinerama_get_screen_count_request_t { *self }
}

#[repr(C)]
pub struct xcb_xinerama_get_screen_count_reply_t {
    pub response_type :   u8,
    pub screen_count :    u8,
    pub sequence :        u16,
    pub length :          u32,
    pub window :          ffi::xproto::xcb_window_t
}

impl Copy for xcb_xinerama_get_screen_count_reply_t {}
impl Clone for xcb_xinerama_get_screen_count_reply_t {
    fn clone(&self) -> xcb_xinerama_get_screen_count_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xinerama_get_screen_size_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_xinerama_get_screen_size_request_t {
    pub major_opcode :   u8,
    pub minor_opcode :   u8,
    pub length :         u16,
    pub window :         ffi::xproto::xcb_window_t,
    pub screen :         u32
}

impl Copy for xcb_xinerama_get_screen_size_request_t {}
impl Clone for xcb_xinerama_get_screen_size_request_t {
    fn clone(&self) -> xcb_xinerama_get_screen_size_request_t { *self }
}

#[repr(C)]
pub struct xcb_xinerama_get_screen_size_reply_t {
    pub response_type :   u8,
    pub pad0 :            u8,
    pub sequence :        u16,
    pub length :          u32,
    pub width :           u32,
    pub height :          u32,
    pub window :          ffi::xproto::xcb_window_t,
    pub screen :          u32
}

impl Copy for xcb_xinerama_get_screen_size_reply_t {}
impl Clone for xcb_xinerama_get_screen_size_reply_t {
    fn clone(&self) -> xcb_xinerama_get_screen_size_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xinerama_is_active_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_xinerama_is_active_request_t {
    pub major_opcode :   u8,
    pub minor_opcode :   u8,
    pub length :         u16
}

impl Copy for xcb_xinerama_is_active_request_t {}
impl Clone for xcb_xinerama_is_active_request_t {
    fn clone(&self) -> xcb_xinerama_is_active_request_t { *self }
}

#[repr(C)]
pub struct xcb_xinerama_is_active_reply_t {
    pub response_type :   u8,
    pub pad0 :            u8,
    pub sequence :        u16,
    pub length :          u32,
    pub state :           u32
}

impl Copy for xcb_xinerama_is_active_reply_t {}
impl Clone for xcb_xinerama_is_active_reply_t {
    fn clone(&self) -> xcb_xinerama_is_active_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xinerama_query_screens_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_xinerama_query_screens_request_t {
    pub major_opcode :   u8,
    pub minor_opcode :   u8,
    pub length :         u16
}

impl Copy for xcb_xinerama_query_screens_request_t {}
impl Clone for xcb_xinerama_query_screens_request_t {
    fn clone(&self) -> xcb_xinerama_query_screens_request_t { *self }
}

#[repr(C)]
pub struct xcb_xinerama_query_screens_reply_t {
    pub response_type :   u8,
    pub pad0 :            u8,
    pub sequence :        u16,
    pub length :          u32,
    pub number :          u32,
    pub pad1 :            [u8; 20]
}

impl Copy for xcb_xinerama_query_screens_reply_t {}
impl Clone for xcb_xinerama_query_screens_reply_t {
    fn clone(&self) -> xcb_xinerama_query_screens_reply_t { *self }
}
#[link(name="xcb-xinerama")]
extern "C" {

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xinerama_screen_info_t)
///
pub fn xcb_xinerama_screen_info_next (i:*mut xcb_xinerama_screen_info_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xinerama_screen_info_end (i:xcb_xinerama_screen_info_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/// Delivers a request to the X server.
///
pub fn xcb_xinerama_query_version (c : *mut ffi::base::xcb_connection_t,
                                      major :  u8,
                                      minor :  u8) -> xcb_xinerama_query_version_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_xinerama_query_version_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                major :  u8,
                                                minor :  u8) -> xcb_xinerama_query_version_cookie_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_xinerama_query_version_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_xinerama_query_version_reply (c : *mut ffi::base::xcb_connection_t,
                                            cookie : xcb_xinerama_query_version_cookie_t,
                                            e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_xinerama_query_version_reply_t;

/// Delivers a request to the X server.
///
pub fn xcb_xinerama_get_state (c : *mut ffi::base::xcb_connection_t,
                                  window :  ffi::xproto::xcb_window_t) -> xcb_xinerama_get_state_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_xinerama_get_state_unchecked (c : *mut ffi::base::xcb_connection_t,
                                            window :  ffi::xproto::xcb_window_t) -> xcb_xinerama_get_state_cookie_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_xinerama_get_state_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_xinerama_get_state_reply (c : *mut ffi::base::xcb_connection_t,
                                        cookie : xcb_xinerama_get_state_cookie_t,
                                        e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_xinerama_get_state_reply_t;

/// Delivers a request to the X server.
///
pub fn xcb_xinerama_get_screen_count (c : *mut ffi::base::xcb_connection_t,
                                         window :  ffi::xproto::xcb_window_t) -> xcb_xinerama_get_screen_count_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_xinerama_get_screen_count_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                   window :  ffi::xproto::xcb_window_t) -> xcb_xinerama_get_screen_count_cookie_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_xinerama_get_screen_count_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_xinerama_get_screen_count_reply (c : *mut ffi::base::xcb_connection_t,
                                               cookie : xcb_xinerama_get_screen_count_cookie_t,
                                               e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_xinerama_get_screen_count_reply_t;

/// Delivers a request to the X server.
///
pub fn xcb_xinerama_get_screen_size (c : *mut ffi::base::xcb_connection_t,
                                        window :  ffi::xproto::xcb_window_t,
                                        screen :  u32) -> xcb_xinerama_get_screen_size_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_xinerama_get_screen_size_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                  window :  ffi::xproto::xcb_window_t,
                                                  screen :  u32) -> xcb_xinerama_get_screen_size_cookie_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_xinerama_get_screen_size_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_xinerama_get_screen_size_reply (c : *mut ffi::base::xcb_connection_t,
                                              cookie : xcb_xinerama_get_screen_size_cookie_t,
                                              e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_xinerama_get_screen_size_reply_t;

/// Delivers a request to the X server.
///
pub fn xcb_xinerama_is_active (c : *mut ffi::base::xcb_connection_t) -> xcb_xinerama_is_active_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_xinerama_is_active_unchecked (c : *mut ffi::base::xcb_connection_t) -> xcb_xinerama_is_active_cookie_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_xinerama_is_active_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_xinerama_is_active_reply (c : *mut ffi::base::xcb_connection_t,
                                        cookie : xcb_xinerama_is_active_cookie_t,
                                        e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_xinerama_is_active_reply_t;

pub fn xcb_xinerama_query_screens_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
///
pub fn xcb_xinerama_query_screens (c : *mut ffi::base::xcb_connection_t) -> xcb_xinerama_query_screens_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_xinerama_query_screens_unchecked (c : *mut ffi::base::xcb_connection_t) -> xcb_xinerama_query_screens_cookie_t;

pub fn xcb_xinerama_query_screens_screen_info (R : *mut xcb_xinerama_query_screens_reply_t) -> *mut xcb_xinerama_screen_info_t;


pub fn xcb_xinerama_query_screens_screen_info_length (R : *mut xcb_xinerama_query_screens_reply_t) -> c_int;

pub fn xcb_xinerama_query_screens_screen_info_iterator (R : *mut xcb_xinerama_query_screens_reply_t) -> xcb_xinerama_screen_info_iterator_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_xinerama_query_screens_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_xinerama_query_screens_reply (c : *mut ffi::base::xcb_connection_t,
                                            cookie : xcb_xinerama_query_screens_cookie_t,
                                            e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_xinerama_query_screens_reply_t;
}

