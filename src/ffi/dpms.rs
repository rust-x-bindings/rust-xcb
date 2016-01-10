//
// This file generated automatically from dpms.xml by r_client.py.
// Edit at your peril.
//

//Make the compiler quiet
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use std;
use libc::*;
use ffi;

pub const DPMS_MAJOR_VERSION : c_uint = 0;
pub const DPMS_MINOR_VERSION : c_uint = 0;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_dpms_get_version_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_dpms_get_version_request_t {
     pub major_opcode :           u8,
     pub minor_opcode :           u8,
     pub length :                 u16,
     pub client_major_version :   u16,
     pub client_minor_version :   u16
}

impl Copy for xcb_dpms_get_version_request_t {}
impl Clone for xcb_dpms_get_version_request_t {
    fn clone(&self) -> xcb_dpms_get_version_request_t { *self }
}

#[repr(C)]
pub struct xcb_dpms_get_version_reply_t {
     pub response_type :          u8,
     pub pad0 :                   u8,
     pub sequence :               u16,
     pub length :                 u32,
     pub server_major_version :   u16,
     pub server_minor_version :   u16
}

impl Copy for xcb_dpms_get_version_reply_t {}
impl Clone for xcb_dpms_get_version_reply_t {
    fn clone(&self) -> xcb_dpms_get_version_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_dpms_capable_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_dpms_capable_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16
}

impl Copy for xcb_dpms_capable_request_t {}
impl Clone for xcb_dpms_capable_request_t {
    fn clone(&self) -> xcb_dpms_capable_request_t { *self }
}

#[repr(C)]
pub struct xcb_dpms_capable_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub capable :         u8,
     pub pad1 :            [u8; 23]
}

impl Copy for xcb_dpms_capable_reply_t {}
impl Clone for xcb_dpms_capable_reply_t {
    fn clone(&self) -> xcb_dpms_capable_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_dpms_get_timeouts_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_dpms_get_timeouts_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16
}

impl Copy for xcb_dpms_get_timeouts_request_t {}
impl Clone for xcb_dpms_get_timeouts_request_t {
    fn clone(&self) -> xcb_dpms_get_timeouts_request_t { *self }
}

#[repr(C)]
pub struct xcb_dpms_get_timeouts_reply_t {
     pub response_type :     u8,
     pub pad0 :              u8,
     pub sequence :          u16,
     pub length :            u32,
     pub standby_timeout :   u16,
     pub suspend_timeout :   u16,
     pub off_timeout :       u16,
     pub pad1 :              [u8; 18]
}

impl Copy for xcb_dpms_get_timeouts_reply_t {}
impl Clone for xcb_dpms_get_timeouts_reply_t {
    fn clone(&self) -> xcb_dpms_get_timeouts_reply_t { *self }
}


#[repr(C)]
pub struct xcb_dpms_set_timeouts_request_t {
     pub major_opcode :      u8,
     pub minor_opcode :      u8,
     pub length :            u16,
     pub standby_timeout :   u16,
     pub suspend_timeout :   u16,
     pub off_timeout :       u16
}

impl Copy for xcb_dpms_set_timeouts_request_t {}
impl Clone for xcb_dpms_set_timeouts_request_t {
    fn clone(&self) -> xcb_dpms_set_timeouts_request_t { *self }
}


#[repr(C)]
pub struct xcb_dpms_enable_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16
}

impl Copy for xcb_dpms_enable_request_t {}
impl Clone for xcb_dpms_enable_request_t {
    fn clone(&self) -> xcb_dpms_enable_request_t { *self }
}


#[repr(C)]
pub struct xcb_dpms_disable_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16
}

impl Copy for xcb_dpms_disable_request_t {}
impl Clone for xcb_dpms_disable_request_t {
    fn clone(&self) -> xcb_dpms_disable_request_t { *self }
}


#[repr(C)]
pub struct xcb_dpms_force_level_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub power_level :    u16
}

impl Copy for xcb_dpms_force_level_request_t {}
impl Clone for xcb_dpms_force_level_request_t {
    fn clone(&self) -> xcb_dpms_force_level_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_dpms_info_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_dpms_info_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16
}

impl Copy for xcb_dpms_info_request_t {}
impl Clone for xcb_dpms_info_request_t {
    fn clone(&self) -> xcb_dpms_info_request_t { *self }
}

#[repr(C)]
pub struct xcb_dpms_info_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub power_level :     u16,
     pub state :           u8,
     pub pad1 :            [u8; 21]
}

impl Copy for xcb_dpms_info_reply_t {}
impl Clone for xcb_dpms_info_reply_t {
    fn clone(&self) -> xcb_dpms_info_reply_t { *self }
}
#[link(name="xcb-dpms")]
extern "C" {

/// Delivers a request to the X server.
///
pub fn xcb_dpms_get_version (c : *mut ffi::base::xcb_connection_t,
                                client_major_version :  u16,
                                client_minor_version :  u16) -> xcb_dpms_get_version_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_dpms_get_version_unchecked (c : *mut ffi::base::xcb_connection_t,
                                          client_major_version :  u16,
                                          client_minor_version :  u16) -> xcb_dpms_get_version_cookie_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_dpms_get_version_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_dpms_get_version_reply (c : *mut ffi::base::xcb_connection_t,
                                      cookie : xcb_dpms_get_version_cookie_t,
                                      e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_dpms_get_version_reply_t;

/// Delivers a request to the X server.
///
pub fn xcb_dpms_capable (c : *mut ffi::base::xcb_connection_t) -> xcb_dpms_capable_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_dpms_capable_unchecked (c : *mut ffi::base::xcb_connection_t) -> xcb_dpms_capable_cookie_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_dpms_capable_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_dpms_capable_reply (c : *mut ffi::base::xcb_connection_t,
                                  cookie : xcb_dpms_capable_cookie_t,
                                  e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_dpms_capable_reply_t;

/// Delivers a request to the X server.
///
pub fn xcb_dpms_get_timeouts (c : *mut ffi::base::xcb_connection_t) -> xcb_dpms_get_timeouts_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_dpms_get_timeouts_unchecked (c : *mut ffi::base::xcb_connection_t) -> xcb_dpms_get_timeouts_cookie_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_dpms_get_timeouts_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_dpms_get_timeouts_reply (c : *mut ffi::base::xcb_connection_t,
                                       cookie : xcb_dpms_get_timeouts_cookie_t,
                                       e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_dpms_get_timeouts_reply_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_dpms_set_timeouts_checked (c : *mut ffi::base::xcb_connection_t,
                                         standby_timeout :  u16,
                                         suspend_timeout :  u16,
                                         off_timeout :  u16) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_dpms_set_timeouts (c : *mut ffi::base::xcb_connection_t,
                                 standby_timeout :  u16,
                                 suspend_timeout :  u16,
                                 off_timeout :  u16) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_dpms_enable_checked (c : *mut ffi::base::xcb_connection_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_dpms_enable (c : *mut ffi::base::xcb_connection_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_dpms_disable_checked (c : *mut ffi::base::xcb_connection_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_dpms_disable (c : *mut ffi::base::xcb_connection_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_dpms_force_level_checked (c : *mut ffi::base::xcb_connection_t,
                                        power_level :  u16) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_dpms_force_level (c : *mut ffi::base::xcb_connection_t,
                                power_level :  u16) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_dpms_info (c : *mut ffi::base::xcb_connection_t) -> xcb_dpms_info_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_dpms_info_unchecked (c : *mut ffi::base::xcb_connection_t) -> xcb_dpms_info_cookie_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_dpms_info_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_dpms_info_reply (c : *mut ffi::base::xcb_connection_t,
                               cookie : xcb_dpms_info_cookie_t,
                               e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_dpms_info_reply_t;
}

