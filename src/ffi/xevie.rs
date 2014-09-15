/*
 * This file generated automatically from xevie.xml by r_client.py.
 * Edit at your peril.
 */

//Make the compiler quiet
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use std;
use libc::*;
use ffi;

pub static XEVIE_MAJOR_VERSION : c_uint = 1;
pub static XEVIE_MINOR_VERSION : c_uint = 0;

pub struct query_version_cookie {
    sequence : c_uint
}


pub struct query_version_request {
     pub major_opcode :           u8,
     pub minor_opcode :           u8,
     pub length :                 u16,
     pub client_major_version :   u16,
     pub client_minor_version :   u16
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


pub struct start_cookie {
    sequence : c_uint
}


pub struct start_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub screen :         u32
}


pub struct start_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub pad1 :            [u8,..24]
}


pub struct end_cookie {
    sequence : c_uint
}


pub struct end_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub cmap :           u32
}


pub struct end_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub pad1 :            [u8,..24]
}


pub struct event {
     pub pad0 :   [u8,..32]
}

/**
 * @brief event_iterator
 **/
pub struct event_iterator {
    pub data : *mut event,
    pub rem  : c_int,
    pub index: c_int
}


pub struct send_cookie {
    sequence : c_uint
}


pub struct send_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub event :          event,
     pub data_type :      u32,
     pub pad0 :           [u8,..64]
}


pub struct send_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub pad1 :            [u8,..24]
}


pub struct select_input_cookie {
    sequence : c_uint
}


pub struct select_input_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub event_mask :     u32
}


pub struct select_input_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub pad1 :            [u8,..24]
}

#[link(name="lxcb-xevie")]
extern "C" {

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xevie_query_version (c : *mut ffi::base::connection,
                                   client_major_version :  u16,
                                   client_minor_version :  u16) -> query_version_cookie;

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
pub fn xcb_xevie_query_version_unchecked (c : *mut ffi::base::connection,
                                             client_major_version :  u16,
                                             client_minor_version :  u16) -> query_version_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xevie_query_version_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_xevie_query_version_reply (c : *mut ffi::base::connection,
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
pub fn xcb_xevie_start (c : *mut ffi::base::connection,
                           screen :  u32) -> start_cookie;

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
pub fn xcb_xevie_start_unchecked (c : *mut ffi::base::connection,
                                     screen :  u32) -> start_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xevie_start_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_xevie_start_reply (c : *mut ffi::base::connection,
                                 cookie : start_cookie,
                                 e : *mut *mut ffi::base::generic_error) -> *mut start_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xevie_end (c : *mut ffi::base::connection,
                         cmap :  u32) -> end_cookie;

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
pub fn xcb_xevie_end_unchecked (c : *mut ffi::base::connection,
                                   cmap :  u32) -> end_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xevie_end_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_xevie_end_reply (c : *mut ffi::base::connection,
                               cookie : end_cookie,
                               e : *mut *mut ffi::base::generic_error) -> *mut end_reply;

/**
 * Get the next element of the iterator
 * @param i Pointer to a event_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(event)
 *
 *
 */
pub fn xcb_xevie_event_next (i:*mut event_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An event_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_xevie_event_end (i:event_iterator) -> ffi::base::generic_iterator;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xevie_send (c : *mut ffi::base::connection,
                          event :  event,
                          data_type :  u32) -> send_cookie;

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
pub fn xcb_xevie_send_unchecked (c : *mut ffi::base::connection,
                                    event :  event,
                                    data_type :  u32) -> send_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xevie_send_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_xevie_send_reply (c : *mut ffi::base::connection,
                                cookie : send_cookie,
                                e : *mut *mut ffi::base::generic_error) -> *mut send_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xevie_select_input (c : *mut ffi::base::connection,
                                  event_mask :  u32) -> select_input_cookie;

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
pub fn xcb_xevie_select_input_unchecked (c : *mut ffi::base::connection,
                                            event_mask :  u32) -> select_input_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xevie_select_input_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_xevie_select_input_reply (c : *mut ffi::base::connection,
                                        cookie : select_input_cookie,
                                        e : *mut *mut ffi::base::generic_error) -> *mut select_input_reply;
}

