//
// This file generated automatically from xevie.xml by r_client.py.
// Edit at your peril.
//

//Make the compiler quiet
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use std;
use libc::*;
use ffi;

pub const XEVIE_MAJOR_VERSION : c_uint = 1;
pub const XEVIE_MINOR_VERSION : c_uint = 0;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xevie_query_version_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_xevie_query_version_request_t {
     pub major_opcode :           u8,
     pub minor_opcode :           u8,
     pub length :                 u16,
     pub client_major_version :   u16,
     pub client_minor_version :   u16
}

impl Copy for xcb_xevie_query_version_request_t {}
impl Clone for xcb_xevie_query_version_request_t {
    fn clone(&self) -> xcb_xevie_query_version_request_t { *self }
}

#[repr(C)]
pub struct xcb_xevie_query_version_reply_t {
     pub response_type :          u8,
     pub pad0 :                   u8,
     pub sequence :               u16,
     pub length :                 u32,
     pub server_major_version :   u16,
     pub server_minor_version :   u16,
     pub pad1 :                   [u8; 20]
}

impl Copy for xcb_xevie_query_version_reply_t {}
impl Clone for xcb_xevie_query_version_reply_t {
    fn clone(&self) -> xcb_xevie_query_version_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xevie_start_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_xevie_start_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub screen :         u32
}

impl Copy for xcb_xevie_start_request_t {}
impl Clone for xcb_xevie_start_request_t {
    fn clone(&self) -> xcb_xevie_start_request_t { *self }
}

#[repr(C)]
pub struct xcb_xevie_start_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub pad1 :            [u8; 24]
}

impl Copy for xcb_xevie_start_reply_t {}
impl Clone for xcb_xevie_start_reply_t {
    fn clone(&self) -> xcb_xevie_start_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xevie_end_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_xevie_end_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub cmap :           u32
}

impl Copy for xcb_xevie_end_request_t {}
impl Clone for xcb_xevie_end_request_t {
    fn clone(&self) -> xcb_xevie_end_request_t { *self }
}

#[repr(C)]
pub struct xcb_xevie_end_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub pad1 :            [u8; 24]
}

impl Copy for xcb_xevie_end_reply_t {}
impl Clone for xcb_xevie_end_reply_t {
    fn clone(&self) -> xcb_xevie_end_reply_t { *self }
}

#[repr(C)]
pub struct xcb_xevie_event_t {
     pub pad0 :   [u8; 32]
}

impl Copy for xcb_xevie_event_t {}
impl Clone for xcb_xevie_event_t {
    fn clone(&self) -> xcb_xevie_event_t { *self }
}
#[repr(C)]
pub struct xcb_xevie_event_iterator_t {
    pub data : *mut xcb_xevie_event_t,
    pub rem  : c_int,
    pub index: c_int
}


#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xevie_send_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_xevie_send_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub event :          xcb_xevie_event_t,
     pub data_type :      u32,
     pub pad0 :           [u8; 64]
}

impl Copy for xcb_xevie_send_request_t {}
impl Clone for xcb_xevie_send_request_t {
    fn clone(&self) -> xcb_xevie_send_request_t { *self }
}

#[repr(C)]
pub struct xcb_xevie_send_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub pad1 :            [u8; 24]
}

impl Copy for xcb_xevie_send_reply_t {}
impl Clone for xcb_xevie_send_reply_t {
    fn clone(&self) -> xcb_xevie_send_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xevie_select_input_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_xevie_select_input_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub event_mask :     u32
}

impl Copy for xcb_xevie_select_input_request_t {}
impl Clone for xcb_xevie_select_input_request_t {
    fn clone(&self) -> xcb_xevie_select_input_request_t { *self }
}

#[repr(C)]
pub struct xcb_xevie_select_input_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub pad1 :            [u8; 24]
}

impl Copy for xcb_xevie_select_input_reply_t {}
impl Clone for xcb_xevie_select_input_reply_t {
    fn clone(&self) -> xcb_xevie_select_input_reply_t { *self }
}
#[link(name="xcb-xevie")]
extern "C" {

/// Delivers a request to the X server.
///
pub fn xcb_xevie_query_version (c : *mut ffi::base::xcb_connection_t,
                                   client_major_version :  u16,
                                   client_minor_version :  u16) -> xcb_xevie_query_version_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_xevie_query_version_unchecked (c : *mut ffi::base::xcb_connection_t,
                                             client_major_version :  u16,
                                             client_minor_version :  u16) -> xcb_xevie_query_version_cookie_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_xevie_query_version_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_xevie_query_version_reply (c : *mut ffi::base::xcb_connection_t,
                                         cookie : xcb_xevie_query_version_cookie_t,
                                         e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_xevie_query_version_reply_t;

/// Delivers a request to the X server.
///
pub fn xcb_xevie_start (c : *mut ffi::base::xcb_connection_t,
                           screen :  u32) -> xcb_xevie_start_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_xevie_start_unchecked (c : *mut ffi::base::xcb_connection_t,
                                     screen :  u32) -> xcb_xevie_start_cookie_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_xevie_start_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_xevie_start_reply (c : *mut ffi::base::xcb_connection_t,
                                 cookie : xcb_xevie_start_cookie_t,
                                 e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_xevie_start_reply_t;

/// Delivers a request to the X server.
///
pub fn xcb_xevie_end (c : *mut ffi::base::xcb_connection_t,
                         cmap :  u32) -> xcb_xevie_end_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_xevie_end_unchecked (c : *mut ffi::base::xcb_connection_t,
                                   cmap :  u32) -> xcb_xevie_end_cookie_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_xevie_end_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_xevie_end_reply (c : *mut ffi::base::xcb_connection_t,
                               cookie : xcb_xevie_end_cookie_t,
                               e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_xevie_end_reply_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xevie_event_t)
///
pub fn xcb_xevie_event_next (i:*mut xcb_xevie_event_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xevie_event_end (i:xcb_xevie_event_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/// Delivers a request to the X server.
///
pub fn xcb_xevie_send (c : *mut ffi::base::xcb_connection_t,
                          event :  xcb_xevie_event_t,
                          data_type :  u32) -> xcb_xevie_send_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_xevie_send_unchecked (c : *mut ffi::base::xcb_connection_t,
                                    event :  xcb_xevie_event_t,
                                    data_type :  u32) -> xcb_xevie_send_cookie_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_xevie_send_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_xevie_send_reply (c : *mut ffi::base::xcb_connection_t,
                                cookie : xcb_xevie_send_cookie_t,
                                e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_xevie_send_reply_t;

/// Delivers a request to the X server.
///
pub fn xcb_xevie_select_input (c : *mut ffi::base::xcb_connection_t,
                                  event_mask :  u32) -> xcb_xevie_select_input_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_xevie_select_input_unchecked (c : *mut ffi::base::xcb_connection_t,
                                            event_mask :  u32) -> xcb_xevie_select_input_cookie_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_xevie_select_input_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_xevie_select_input_reply (c : *mut ffi::base::xcb_connection_t,
                                        cookie : xcb_xevie_select_input_cookie_t,
                                        e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_xevie_select_input_reply_t;
}

