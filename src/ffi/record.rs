/*
 * This file generated automatically from record.xml by r_client.py.
 * Edit at your peril.
 */

//Make the compiler quiet
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use std;
use libc::*;
use ffi;

pub static RECORD_MAJOR_VERSION : c_uint = 1;
pub static RECORD_MINOR_VERSION : c_uint = 13;

pub type context = u32;
/**
 * @brief context_iterator
 **/
pub struct context_iterator {
    pub data : *mut context,
    pub rem  : c_int,
    pub index: c_int
}


pub struct range_8 {
     pub first :   u8,
     pub last :    u8
}

/**
 * @brief range_8_iterator
 **/
pub struct range_8_iterator {
    pub data : *mut range_8,
    pub rem  : c_int,
    pub index: c_int
}


pub struct range_16 {
     pub first :   u16,
     pub last :    u16
}

/**
 * @brief range_16_iterator
 **/
pub struct range_16_iterator {
    pub data : *mut range_16,
    pub rem  : c_int,
    pub index: c_int
}


pub struct ext_range {
     pub major :   range_8,
     pub minor :   range_16
}

/**
 * @brief ext_range_iterator
 **/
pub struct ext_range_iterator {
    pub data : *mut ext_range,
    pub rem  : c_int,
    pub index: c_int
}


pub struct range {
     pub core_requests :      range_8,
     pub core_replies :       range_8,
     pub ext_requests :       ext_range,
     pub ext_replies :        ext_range,
     pub delivered_events :   range_8,
     pub device_events :      range_8,
     pub errors :             range_8,
     pub client_started :     u8,
     pub client_died :        u8
}

/**
 * @brief range_iterator
 **/
pub struct range_iterator {
    pub data : *mut range,
    pub rem  : c_int,
    pub index: c_int
}


pub type element_header = u8;
/**
 * @brief element_header_iterator
 **/
pub struct element_header_iterator {
    pub data : *mut element_header,
    pub rem  : c_int,
    pub index: c_int
}


pub type client_spec = u32;
/**
 * @brief client_spec_iterator
 **/
pub struct client_spec_iterator {
    pub data : *mut client_spec,
    pub rem  : c_int,
    pub index: c_int
}


pub struct client_info {
     pub client_resource :   client_spec,
     pub num_ranges :        u32
}

/**
 * @brief client_info_iterator
 **/
pub struct client_info_iterator {
    pub data : *mut client_info,
    pub rem  : c_int,
    pub index: c_int
}



pub struct bad_context_error {
     pub response_type :    u8,
     pub error_code :       u8,
     pub sequence :         u16,
     pub invalid_record :   u32
}


pub struct query_version_cookie {
    sequence : c_uint
}


pub struct query_version_request {
     pub major_opcode :    u8,
     pub minor_opcode :    u8,
     pub length :          u16,
     pub major_version :   u16,
     pub minor_version :   u16
}


pub struct query_version_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub major_version :   u16,
     pub minor_version :   u16
}



pub struct create_context_request {
     pub major_opcode :       u8,
     pub minor_opcode :       u8,
     pub length :             u16,
     pub context :            context,
     pub element_header :     element_header,
     pub pad0 :               [u8,..3],
     pub num_client_specs :   u32,
     pub num_ranges :         u32
}



pub struct register_clients_request {
     pub major_opcode :       u8,
     pub minor_opcode :       u8,
     pub length :             u16,
     pub context :            context,
     pub element_header :     element_header,
     pub pad0 :               [u8,..3],
     pub num_client_specs :   u32,
     pub num_ranges :         u32
}



pub struct unregister_clients_request {
     pub major_opcode :       u8,
     pub minor_opcode :       u8,
     pub length :             u16,
     pub context :            context,
     pub num_client_specs :   u32
}


pub struct get_context_cookie {
    sequence : c_uint
}


pub struct get_context_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub context :        context
}


pub struct get_context_reply {
     pub response_type :             u8,
     pub enabled :                   u8,
     pub sequence :                  u16,
     pub length :                    u32,
     pub element_header :            element_header,
     pub pad0 :                      [u8,..3],
     pub num_intercepted_clients :   u32,
     pub pad1 :                      [u8,..16]
}


pub struct enable_context_cookie {
    sequence : c_uint
}


pub struct enable_context_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub context :        context
}


pub struct enable_context_reply {
     pub response_type :      u8,
     pub category :           u8,
     pub sequence :           u16,
     pub length :             u32,
     pub element_header :     element_header,
     pub client_swapped :     u8,
     pub pad0 :               [u8,..2],
     pub xid_base :           u32,
     pub server_time :        u32,
     pub rec_sequence_num :   u32,
     pub pad1 :               [u8,..8]
}



pub struct disable_context_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub context :        context
}



pub struct free_context_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub context :        context
}

#[link(name="xcb-record")]
extern "C" {

/**
 * Get the next element of the iterator
 * @param i Pointer to a context_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(context)
 *
 *
 */
pub fn xcb_record_context_next (i:*mut context_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An context_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_record_context_end (i:context_iterator) -> ffi::base::generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a range_8_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(range_8)
 *
 *
 */
pub fn xcb_record_range_8_next (i:*mut range_8_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An range_8_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_record_range_8_end (i:range_8_iterator) -> ffi::base::generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a range_16_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(range_16)
 *
 *
 */
pub fn xcb_record_range_16_next (i:*mut range_16_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An range_16_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_record_range_16_end (i:range_16_iterator) -> ffi::base::generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a ext_range_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(ext_range)
 *
 *
 */
pub fn xcb_record_ext_range_next (i:*mut ext_range_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An ext_range_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_record_ext_range_end (i:ext_range_iterator) -> ffi::base::generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a range_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(range)
 *
 *
 */
pub fn xcb_record_range_next (i:*mut range_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An range_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_record_range_end (i:range_iterator) -> ffi::base::generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a element_header_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(element_header)
 *
 *
 */
pub fn xcb_record_element_header_next (i:*mut element_header_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An element_header_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_record_element_header_end (i:element_header_iterator) -> ffi::base::generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a client_spec_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(client_spec)
 *
 *
 */
pub fn xcb_record_client_spec_next (i:*mut client_spec_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An client_spec_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_record_client_spec_end (i:client_spec_iterator) -> ffi::base::generic_iterator;

pub fn xcb_record_client_info_sizeof (_buffer :  *mut c_void) -> c_int;

pub fn xcb_record_client_info_ranges (R : *mut client_info) -> *mut range;


pub fn xcb_record_client_info_ranges_length (R : *mut client_info) -> c_int;

pub fn xcb_record_client_info_ranges_iterator (R : *mut client_info) -> range_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a client_info_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(client_info)
 *
 *
 */
pub fn xcb_record_client_info_next (i:*mut client_info_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An client_info_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_record_client_info_end (i:client_info_iterator) -> ffi::base::generic_iterator;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_record_query_version (c : *mut ffi::base::connection,
                                    major_version :  u16,
                                    minor_version :  u16) -> query_version_cookie;

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
pub fn xcb_record_query_version_unchecked (c : *mut ffi::base::connection,
                                              major_version :  u16,
                                              minor_version :  u16) -> query_version_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_record_query_version_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_record_query_version_reply (c : *mut ffi::base::connection,
                                          cookie : query_version_cookie,
                                          e : *mut *mut ffi::base::generic_error) -> *mut query_version_reply;

pub fn xcb_record_create_context_sizeof (_buffer :  *mut c_void) -> c_int;

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
pub fn xcb_record_create_context_checked (c : *mut ffi::base::connection,
                                             context :  context,
                                             element_header :  element_header,
                                             num_client_specs :  u32,
                                             num_ranges :  u32,
                                             client_specs : *mut client_spec,
                                             ranges : *mut range) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_record_create_context (c : *mut ffi::base::connection,
                                     context :  context,
                                     element_header :  element_header,
                                     num_client_specs :  u32,
                                     num_ranges :  u32,
                                     client_specs : *mut client_spec,
                                     ranges : *mut range) -> ffi::base::void_cookie;

pub fn xcb_record_register_clients_sizeof (_buffer :  *mut c_void) -> c_int;

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
pub fn xcb_record_register_clients_checked (c : *mut ffi::base::connection,
                                               context :  context,
                                               element_header :  element_header,
                                               num_client_specs :  u32,
                                               num_ranges :  u32,
                                               client_specs : *mut client_spec,
                                               ranges : *mut range) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_record_register_clients (c : *mut ffi::base::connection,
                                       context :  context,
                                       element_header :  element_header,
                                       num_client_specs :  u32,
                                       num_ranges :  u32,
                                       client_specs : *mut client_spec,
                                       ranges : *mut range) -> ffi::base::void_cookie;

pub fn xcb_record_unregister_clients_sizeof (_buffer :  *mut c_void) -> c_int;

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
pub fn xcb_record_unregister_clients_checked (c : *mut ffi::base::connection,
                                                 context :  context,
                                                 num_client_specs :  u32,
                                                 client_specs : *mut client_spec) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_record_unregister_clients (c : *mut ffi::base::connection,
                                         context :  context,
                                         num_client_specs :  u32,
                                         client_specs : *mut client_spec) -> ffi::base::void_cookie;

pub fn xcb_record_get_context_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_record_get_context (c : *mut ffi::base::connection,
                                  context :  context) -> get_context_cookie;

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
pub fn xcb_record_get_context_unchecked (c : *mut ffi::base::connection,
                                            context :  context) -> get_context_cookie;


pub fn xcb_record_get_context_intercepted_clients_length (R : *mut get_context_reply) -> c_int;

pub fn xcb_record_get_context_intercepted_clients_iterator (R : *mut get_context_reply) -> client_info_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_record_get_context_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_record_get_context_reply (c : *mut ffi::base::connection,
                                        cookie : get_context_cookie,
                                        e : *mut *mut ffi::base::generic_error) -> *mut get_context_reply;

pub fn xcb_record_enable_context_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_record_enable_context (c : *mut ffi::base::connection,
                                     context :  context) -> enable_context_cookie;

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
pub fn xcb_record_enable_context_unchecked (c : *mut ffi::base::connection,
                                               context :  context) -> enable_context_cookie;

pub fn xcb_record_enable_context_data (R : *mut enable_context_reply) -> *mut u8;


pub fn xcb_record_enable_context_data_length (R : *mut enable_context_reply) -> c_int;


pub fn xcb_record_enable_context_data_end (R : *mut enable_context_reply) -> ffi::base::generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_record_enable_context_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_record_enable_context_reply (c : *mut ffi::base::connection,
                                           cookie : enable_context_cookie,
                                           e : *mut *mut ffi::base::generic_error) -> *mut enable_context_reply;

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
pub fn xcb_record_disable_context_checked (c : *mut ffi::base::connection,
                                              context :  context) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_record_disable_context (c : *mut ffi::base::connection,
                                      context :  context) -> ffi::base::void_cookie;

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
pub fn xcb_record_free_context_checked (c : *mut ffi::base::connection,
                                           context :  context) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_record_free_context (c : *mut ffi::base::connection,
                                   context :  context) -> ffi::base::void_cookie;
}

