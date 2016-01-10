//
// This file generated automatically from record.xml by r_client.py.
// Edit at your peril.
//

//Make the compiler quiet
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use std;
use libc::*;
use ffi;

pub const RECORD_MAJOR_VERSION : c_uint = 1;
pub const RECORD_MINOR_VERSION : c_uint = 13;

pub type xcb_record_context_t = u32;
#[repr(C)]
pub struct xcb_record_context_iterator_t {
    pub data : *mut xcb_record_context_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_record_range_8_t {
     pub first :   u8,
     pub last :    u8
}

impl Copy for xcb_record_range_8_t {}
impl Clone for xcb_record_range_8_t {
    fn clone(&self) -> xcb_record_range_8_t { *self }
}
#[repr(C)]
pub struct xcb_record_range_8_iterator_t {
    pub data : *mut xcb_record_range_8_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_record_range_16_t {
     pub first :   u16,
     pub last :    u16
}

impl Copy for xcb_record_range_16_t {}
impl Clone for xcb_record_range_16_t {
    fn clone(&self) -> xcb_record_range_16_t { *self }
}
#[repr(C)]
pub struct xcb_record_range_16_iterator_t {
    pub data : *mut xcb_record_range_16_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_record_ext_range_t {
     pub major :   xcb_record_range_8_t,
     pub minor :   xcb_record_range_16_t
}

impl Copy for xcb_record_ext_range_t {}
impl Clone for xcb_record_ext_range_t {
    fn clone(&self) -> xcb_record_ext_range_t { *self }
}
#[repr(C)]
pub struct xcb_record_ext_range_iterator_t {
    pub data : *mut xcb_record_ext_range_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_record_range_t {
     pub core_requests :      xcb_record_range_8_t,
     pub core_replies :       xcb_record_range_8_t,
     pub ext_requests :       xcb_record_ext_range_t,
     pub ext_replies :        xcb_record_ext_range_t,
     pub delivered_events :   xcb_record_range_8_t,
     pub device_events :      xcb_record_range_8_t,
     pub errors :             xcb_record_range_8_t,
     pub client_started :     u8,
     pub client_died :        u8
}

impl Copy for xcb_record_range_t {}
impl Clone for xcb_record_range_t {
    fn clone(&self) -> xcb_record_range_t { *self }
}
#[repr(C)]
pub struct xcb_record_range_iterator_t {
    pub data : *mut xcb_record_range_t,
    pub rem  : c_int,
    pub index: c_int
}


pub type xcb_record_element_header_t = u8;
#[repr(C)]
pub struct xcb_record_element_header_iterator_t {
    pub data : *mut xcb_record_element_header_t,
    pub rem  : c_int,
    pub index: c_int
}


pub type xcb_record_client_spec_t = u32;
#[repr(C)]
pub struct xcb_record_client_spec_iterator_t {
    pub data : *mut xcb_record_client_spec_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_record_client_info_t {
     pub client_resource :   xcb_record_client_spec_t,
     pub num_ranges :        u32
}

impl Copy for xcb_record_client_info_t {}
impl Clone for xcb_record_client_info_t {
    fn clone(&self) -> xcb_record_client_info_t { *self }
}
#[repr(C)]
pub struct xcb_record_client_info_iterator_t {
    pub data : *mut xcb_record_client_info_t,
    pub rem  : c_int,
    pub index: c_int
}



#[repr(C)]
pub struct xcb_record_bad_context_error_t {
     pub response_type :    u8,
     pub error_code :       u8,
     pub sequence :         u16,
     pub invalid_record :   u32
}

impl Copy for xcb_record_bad_context_error_t {}
impl Clone for xcb_record_bad_context_error_t {
    fn clone(&self) -> xcb_record_bad_context_error_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_record_query_version_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_record_query_version_request_t {
     pub major_opcode :    u8,
     pub minor_opcode :    u8,
     pub length :          u16,
     pub major_version :   u16,
     pub minor_version :   u16
}

impl Copy for xcb_record_query_version_request_t {}
impl Clone for xcb_record_query_version_request_t {
    fn clone(&self) -> xcb_record_query_version_request_t { *self }
}

#[repr(C)]
pub struct xcb_record_query_version_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub major_version :   u16,
     pub minor_version :   u16
}

impl Copy for xcb_record_query_version_reply_t {}
impl Clone for xcb_record_query_version_reply_t {
    fn clone(&self) -> xcb_record_query_version_reply_t { *self }
}


#[repr(C)]
pub struct xcb_record_create_context_request_t {
     pub major_opcode :       u8,
     pub minor_opcode :       u8,
     pub length :             u16,
     pub context :            xcb_record_context_t,
     pub element_header :     xcb_record_element_header_t,
     pub pad0 :               [u8; 3],
     pub num_client_specs :   u32,
     pub num_ranges :         u32
}

impl Copy for xcb_record_create_context_request_t {}
impl Clone for xcb_record_create_context_request_t {
    fn clone(&self) -> xcb_record_create_context_request_t { *self }
}


#[repr(C)]
pub struct xcb_record_register_clients_request_t {
     pub major_opcode :       u8,
     pub minor_opcode :       u8,
     pub length :             u16,
     pub context :            xcb_record_context_t,
     pub element_header :     xcb_record_element_header_t,
     pub pad0 :               [u8; 3],
     pub num_client_specs :   u32,
     pub num_ranges :         u32
}

impl Copy for xcb_record_register_clients_request_t {}
impl Clone for xcb_record_register_clients_request_t {
    fn clone(&self) -> xcb_record_register_clients_request_t { *self }
}


#[repr(C)]
pub struct xcb_record_unregister_clients_request_t {
     pub major_opcode :       u8,
     pub minor_opcode :       u8,
     pub length :             u16,
     pub context :            xcb_record_context_t,
     pub num_client_specs :   u32
}

impl Copy for xcb_record_unregister_clients_request_t {}
impl Clone for xcb_record_unregister_clients_request_t {
    fn clone(&self) -> xcb_record_unregister_clients_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_record_get_context_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_record_get_context_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub context :        xcb_record_context_t
}

impl Copy for xcb_record_get_context_request_t {}
impl Clone for xcb_record_get_context_request_t {
    fn clone(&self) -> xcb_record_get_context_request_t { *self }
}

#[repr(C)]
pub struct xcb_record_get_context_reply_t {
     pub response_type :             u8,
     pub enabled :                   u8,
     pub sequence :                  u16,
     pub length :                    u32,
     pub element_header :            xcb_record_element_header_t,
     pub pad0 :                      [u8; 3],
     pub num_intercepted_clients :   u32,
     pub pad1 :                      [u8; 16]
}

impl Copy for xcb_record_get_context_reply_t {}
impl Clone for xcb_record_get_context_reply_t {
    fn clone(&self) -> xcb_record_get_context_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_record_enable_context_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_record_enable_context_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub context :        xcb_record_context_t
}

impl Copy for xcb_record_enable_context_request_t {}
impl Clone for xcb_record_enable_context_request_t {
    fn clone(&self) -> xcb_record_enable_context_request_t { *self }
}

#[repr(C)]
pub struct xcb_record_enable_context_reply_t {
     pub response_type :      u8,
     pub category :           u8,
     pub sequence :           u16,
     pub length :             u32,
     pub element_header :     xcb_record_element_header_t,
     pub client_swapped :     u8,
     pub pad0 :               [u8; 2],
     pub xid_base :           u32,
     pub server_time :        u32,
     pub rec_sequence_num :   u32,
     pub pad1 :               [u8; 8]
}

impl Copy for xcb_record_enable_context_reply_t {}
impl Clone for xcb_record_enable_context_reply_t {
    fn clone(&self) -> xcb_record_enable_context_reply_t { *self }
}


#[repr(C)]
pub struct xcb_record_disable_context_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub context :        xcb_record_context_t
}

impl Copy for xcb_record_disable_context_request_t {}
impl Clone for xcb_record_disable_context_request_t {
    fn clone(&self) -> xcb_record_disable_context_request_t { *self }
}


#[repr(C)]
pub struct xcb_record_free_context_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub context :        xcb_record_context_t
}

impl Copy for xcb_record_free_context_request_t {}
impl Clone for xcb_record_free_context_request_t {
    fn clone(&self) -> xcb_record_free_context_request_t { *self }
}
#[link(name="xcb-record")]
extern "C" {

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_record_context_t)
///
pub fn xcb_record_context_next (i:*mut xcb_record_context_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_record_context_end (i:xcb_record_context_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_record_range_8_t)
///
pub fn xcb_record_range_8_next (i:*mut xcb_record_range_8_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_record_range_8_end (i:xcb_record_range_8_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_record_range_16_t)
///
pub fn xcb_record_range_16_next (i:*mut xcb_record_range_16_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_record_range_16_end (i:xcb_record_range_16_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_record_ext_range_t)
///
pub fn xcb_record_ext_range_next (i:*mut xcb_record_ext_range_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_record_ext_range_end (i:xcb_record_ext_range_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_record_range_t)
///
pub fn xcb_record_range_next (i:*mut xcb_record_range_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_record_range_end (i:xcb_record_range_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_record_element_header_t)
///
pub fn xcb_record_element_header_next (i:*mut xcb_record_element_header_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_record_element_header_end (i:xcb_record_element_header_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_record_client_spec_t)
///
pub fn xcb_record_client_spec_next (i:*mut xcb_record_client_spec_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_record_client_spec_end (i:xcb_record_client_spec_iterator_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_record_client_info_sizeof (_buffer :  *mut c_void) -> c_int;

pub fn xcb_record_client_info_ranges (R : *mut xcb_record_client_info_t) -> *mut xcb_record_range_t;


pub fn xcb_record_client_info_ranges_length (R : *mut xcb_record_client_info_t) -> c_int;

pub fn xcb_record_client_info_ranges_iterator (R : *mut xcb_record_client_info_t) -> xcb_record_range_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_record_client_info_t)
///
pub fn xcb_record_client_info_next (i:*mut xcb_record_client_info_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_record_client_info_end (i:xcb_record_client_info_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/// Delivers a request to the X server.
///
pub fn xcb_record_query_version (c : *mut ffi::base::xcb_connection_t,
                                    major_version :  u16,
                                    minor_version :  u16) -> xcb_record_query_version_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_record_query_version_unchecked (c : *mut ffi::base::xcb_connection_t,
                                              major_version :  u16,
                                              minor_version :  u16) -> xcb_record_query_version_cookie_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_record_query_version_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_record_query_version_reply (c : *mut ffi::base::xcb_connection_t,
                                          cookie : xcb_record_query_version_cookie_t,
                                          e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_record_query_version_reply_t;

pub fn xcb_record_create_context_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_record_create_context_checked (c : *mut ffi::base::xcb_connection_t,
                                             context :  xcb_record_context_t,
                                             element_header :  xcb_record_element_header_t,
                                             num_client_specs :  u32,
                                             num_ranges :  u32,
                                             client_specs : *mut xcb_record_client_spec_t,
                                             ranges : *mut xcb_record_range_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_record_create_context (c : *mut ffi::base::xcb_connection_t,
                                     context :  xcb_record_context_t,
                                     element_header :  xcb_record_element_header_t,
                                     num_client_specs :  u32,
                                     num_ranges :  u32,
                                     client_specs : *mut xcb_record_client_spec_t,
                                     ranges : *mut xcb_record_range_t) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_record_register_clients_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_record_register_clients_checked (c : *mut ffi::base::xcb_connection_t,
                                               context :  xcb_record_context_t,
                                               element_header :  xcb_record_element_header_t,
                                               num_client_specs :  u32,
                                               num_ranges :  u32,
                                               client_specs : *mut xcb_record_client_spec_t,
                                               ranges : *mut xcb_record_range_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_record_register_clients (c : *mut ffi::base::xcb_connection_t,
                                       context :  xcb_record_context_t,
                                       element_header :  xcb_record_element_header_t,
                                       num_client_specs :  u32,
                                       num_ranges :  u32,
                                       client_specs : *mut xcb_record_client_spec_t,
                                       ranges : *mut xcb_record_range_t) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_record_unregister_clients_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_record_unregister_clients_checked (c : *mut ffi::base::xcb_connection_t,
                                                 context :  xcb_record_context_t,
                                                 num_client_specs :  u32,
                                                 client_specs : *mut xcb_record_client_spec_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_record_unregister_clients (c : *mut ffi::base::xcb_connection_t,
                                         context :  xcb_record_context_t,
                                         num_client_specs :  u32,
                                         client_specs : *mut xcb_record_client_spec_t) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_record_get_context_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
///
pub fn xcb_record_get_context (c : *mut ffi::base::xcb_connection_t,
                                  context :  xcb_record_context_t) -> xcb_record_get_context_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_record_get_context_unchecked (c : *mut ffi::base::xcb_connection_t,
                                            context :  xcb_record_context_t) -> xcb_record_get_context_cookie_t;


pub fn xcb_record_get_context_intercepted_clients_length (R : *mut xcb_record_get_context_reply_t) -> c_int;

pub fn xcb_record_get_context_intercepted_clients_iterator (R : *mut xcb_record_get_context_reply_t) -> xcb_record_client_info_iterator_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_record_get_context_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_record_get_context_reply (c : *mut ffi::base::xcb_connection_t,
                                        cookie : xcb_record_get_context_cookie_t,
                                        e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_record_get_context_reply_t;

pub fn xcb_record_enable_context_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
///
pub fn xcb_record_enable_context (c : *mut ffi::base::xcb_connection_t,
                                     context :  xcb_record_context_t) -> xcb_record_enable_context_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_record_enable_context_unchecked (c : *mut ffi::base::xcb_connection_t,
                                               context :  xcb_record_context_t) -> xcb_record_enable_context_cookie_t;

pub fn xcb_record_enable_context_data (R : *mut xcb_record_enable_context_reply_t) -> *mut u8;


pub fn xcb_record_enable_context_data_length (R : *mut xcb_record_enable_context_reply_t) -> c_int;


pub fn xcb_record_enable_context_data_end (R : *mut xcb_record_enable_context_reply_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_record_enable_context_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_record_enable_context_reply (c : *mut ffi::base::xcb_connection_t,
                                           cookie : xcb_record_enable_context_cookie_t,
                                           e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_record_enable_context_reply_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_record_disable_context_checked (c : *mut ffi::base::xcb_connection_t,
                                              context :  xcb_record_context_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_record_disable_context (c : *mut ffi::base::xcb_connection_t,
                                      context :  xcb_record_context_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_record_free_context_checked (c : *mut ffi::base::xcb_connection_t,
                                           context :  xcb_record_context_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_record_free_context (c : *mut ffi::base::xcb_connection_t,
                                   context :  xcb_record_context_t) -> ffi::base::xcb_void_cookie_t;
}

