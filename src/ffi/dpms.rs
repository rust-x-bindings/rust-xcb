/*
 * This file generated automatically from dpms.xml by r_client.py.
 * Edit at your peril.
 */

//Make the compiler quiet
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use std;
use libc::*;
use ffi;

pub static DPMS_MAJOR_VERSION : c_uint = 0;
pub static DPMS_MINOR_VERSION : c_uint = 0;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct get_version_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct get_version_request {
     pub major_opcode :           u8,
     pub minor_opcode :           u8,
     pub length :                 u16,
     pub client_major_version :   u16,
     pub client_minor_version :   u16
}

impl Copy for get_version_request {}
impl Clone for get_version_request {
    fn clone(&self) -> get_version_request { *self }
}

#[repr(C)]
pub struct get_version_reply {
     pub response_type :          u8,
     pub pad0 :                   u8,
     pub sequence :               u16,
     pub length :                 u32,
     pub server_major_version :   u16,
     pub server_minor_version :   u16
}

impl Copy for get_version_reply {}
impl Clone for get_version_reply {
    fn clone(&self) -> get_version_reply { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct capable_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct capable_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16
}

impl Copy for capable_request {}
impl Clone for capable_request {
    fn clone(&self) -> capable_request { *self }
}

#[repr(C)]
pub struct capable_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub capable :         u8,
     pub pad1 :            [u8; 23]
}

impl Copy for capable_reply {}
impl Clone for capable_reply {
    fn clone(&self) -> capable_reply { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct get_timeouts_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct get_timeouts_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16
}

impl Copy for get_timeouts_request {}
impl Clone for get_timeouts_request {
    fn clone(&self) -> get_timeouts_request { *self }
}

#[repr(C)]
pub struct get_timeouts_reply {
     pub response_type :     u8,
     pub pad0 :              u8,
     pub sequence :          u16,
     pub length :            u32,
     pub standby_timeout :   u16,
     pub suspend_timeout :   u16,
     pub off_timeout :       u16,
     pub pad1 :              [u8; 18]
}

impl Copy for get_timeouts_reply {}
impl Clone for get_timeouts_reply {
    fn clone(&self) -> get_timeouts_reply { *self }
}


#[repr(C)]
pub struct set_timeouts_request {
     pub major_opcode :      u8,
     pub minor_opcode :      u8,
     pub length :            u16,
     pub standby_timeout :   u16,
     pub suspend_timeout :   u16,
     pub off_timeout :       u16
}

impl Copy for set_timeouts_request {}
impl Clone for set_timeouts_request {
    fn clone(&self) -> set_timeouts_request { *self }
}


#[repr(C)]
pub struct enable_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16
}

impl Copy for enable_request {}
impl Clone for enable_request {
    fn clone(&self) -> enable_request { *self }
}


#[repr(C)]
pub struct disable_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16
}

impl Copy for disable_request {}
impl Clone for disable_request {
    fn clone(&self) -> disable_request { *self }
}


#[repr(C)]
pub struct force_level_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub power_level :    u16
}

impl Copy for force_level_request {}
impl Clone for force_level_request {
    fn clone(&self) -> force_level_request { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct info_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct info_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16
}

impl Copy for info_request {}
impl Clone for info_request {
    fn clone(&self) -> info_request { *self }
}

#[repr(C)]
pub struct info_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub power_level :     u16,
     pub state :           u8,
     pub pad1 :            [u8; 21]
}

impl Copy for info_reply {}
impl Clone for info_reply {
    fn clone(&self) -> info_reply { *self }
}
#[link(name="xcb-dpms")]
extern "C" {

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_dpms_get_version (c : *mut ffi::base::connection,
                                client_major_version :  u16,
                                client_minor_version :  u16) -> get_version_cookie;

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
pub fn xcb_dpms_get_version_unchecked (c : *mut ffi::base::connection,
                                          client_major_version :  u16,
                                          client_minor_version :  u16) -> get_version_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_dpms_get_version_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_dpms_get_version_reply (c : *mut ffi::base::connection,
                                      cookie : get_version_cookie,
                                      e : *mut *mut ffi::base::generic_error) -> *mut get_version_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_dpms_capable (c : *mut ffi::base::connection) -> capable_cookie;

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
pub fn xcb_dpms_capable_unchecked (c : *mut ffi::base::connection) -> capable_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_dpms_capable_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_dpms_capable_reply (c : *mut ffi::base::connection,
                                  cookie : capable_cookie,
                                  e : *mut *mut ffi::base::generic_error) -> *mut capable_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_dpms_get_timeouts (c : *mut ffi::base::connection) -> get_timeouts_cookie;

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
pub fn xcb_dpms_get_timeouts_unchecked (c : *mut ffi::base::connection) -> get_timeouts_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_dpms_get_timeouts_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_dpms_get_timeouts_reply (c : *mut ffi::base::connection,
                                       cookie : get_timeouts_cookie,
                                       e : *mut *mut ffi::base::generic_error) -> *mut get_timeouts_reply;

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
pub fn xcb_dpms_set_timeouts_checked (c : *mut ffi::base::connection,
                                         standby_timeout :  u16,
                                         suspend_timeout :  u16,
                                         off_timeout :  u16) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_dpms_set_timeouts (c : *mut ffi::base::connection,
                                 standby_timeout :  u16,
                                 suspend_timeout :  u16,
                                 off_timeout :  u16) -> ffi::base::void_cookie;

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
pub fn xcb_dpms_enable_checked (c : *mut ffi::base::connection) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_dpms_enable (c : *mut ffi::base::connection) -> ffi::base::void_cookie;

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
pub fn xcb_dpms_disable_checked (c : *mut ffi::base::connection) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_dpms_disable (c : *mut ffi::base::connection) -> ffi::base::void_cookie;

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
pub fn xcb_dpms_force_level_checked (c : *mut ffi::base::connection,
                                        power_level :  u16) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_dpms_force_level (c : *mut ffi::base::connection,
                                power_level :  u16) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_dpms_info (c : *mut ffi::base::connection) -> info_cookie;

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
pub fn xcb_dpms_info_unchecked (c : *mut ffi::base::connection) -> info_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_dpms_info_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_dpms_info_reply (c : *mut ffi::base::connection,
                               cookie : info_cookie,
                               e : *mut *mut ffi::base::generic_error) -> *mut info_reply;
}

