//
// This file generated automatically from res.xml by r_client.py.
// Edit at your peril.
//

//Make the compiler quiet
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use std;
use libc::*;
use ffi;
use ffi::xproto;

pub const RES_MAJOR_VERSION : c_uint = 1;
pub const RES_MINOR_VERSION : c_uint = 0;

#[repr(C)]
pub struct xcb_res_client_t {
    pub resource_base :   u32,
    pub resource_mask :   u32
}

impl Copy for xcb_res_client_t {}
impl Clone for xcb_res_client_t {
    fn clone(&self) -> xcb_res_client_t { *self }
}
#[repr(C)]
pub struct xcb_res_client_iterator_t {
    pub data : *mut xcb_res_client_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_res_type_t {
    pub resource_type :   ffi::xproto::xcb_atom_t,
    pub count :           u32
}

impl Copy for xcb_res_type_t {}
impl Clone for xcb_res_type_t {
    fn clone(&self) -> xcb_res_type_t { *self }
}
#[repr(C)]
pub struct xcb_res_type_iterator_t {
    pub data : *mut xcb_res_type_t,
    pub rem  : c_int,
    pub index: c_int
}


#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_res_query_version_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_res_query_version_request_t {
    pub major_opcode :   u8,
    pub minor_opcode :   u8,
    pub length :         u16,
    pub client_major :   u8,
    pub client_minor :   u8
}

impl Copy for xcb_res_query_version_request_t {}
impl Clone for xcb_res_query_version_request_t {
    fn clone(&self) -> xcb_res_query_version_request_t { *self }
}

#[repr(C)]
pub struct xcb_res_query_version_reply_t {
    pub response_type :   u8,
    pub pad0 :            u8,
    pub sequence :        u16,
    pub length :          u32,
    pub server_major :    u16,
    pub server_minor :    u16
}

impl Copy for xcb_res_query_version_reply_t {}
impl Clone for xcb_res_query_version_reply_t {
    fn clone(&self) -> xcb_res_query_version_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_res_query_clients_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_res_query_clients_request_t {
    pub major_opcode :   u8,
    pub minor_opcode :   u8,
    pub length :         u16
}

impl Copy for xcb_res_query_clients_request_t {}
impl Clone for xcb_res_query_clients_request_t {
    fn clone(&self) -> xcb_res_query_clients_request_t { *self }
}

#[repr(C)]
pub struct xcb_res_query_clients_reply_t {
    pub response_type :   u8,
    pub pad0 :            u8,
    pub sequence :        u16,
    pub length :          u32,
    pub num_clients :     u32,
    pub pad1 :            [u8; 20]
}

impl Copy for xcb_res_query_clients_reply_t {}
impl Clone for xcb_res_query_clients_reply_t {
    fn clone(&self) -> xcb_res_query_clients_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_res_query_client_resources_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_res_query_client_resources_request_t {
    pub major_opcode :   u8,
    pub minor_opcode :   u8,
    pub length :         u16,
    pub xid :            u32
}

impl Copy for xcb_res_query_client_resources_request_t {}
impl Clone for xcb_res_query_client_resources_request_t {
    fn clone(&self) -> xcb_res_query_client_resources_request_t { *self }
}

#[repr(C)]
pub struct xcb_res_query_client_resources_reply_t {
    pub response_type :   u8,
    pub pad0 :            u8,
    pub sequence :        u16,
    pub length :          u32,
    pub num_types :       u32,
    pub pad1 :            [u8; 20]
}

impl Copy for xcb_res_query_client_resources_reply_t {}
impl Clone for xcb_res_query_client_resources_reply_t {
    fn clone(&self) -> xcb_res_query_client_resources_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_res_query_client_pixmap_bytes_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_res_query_client_pixmap_bytes_request_t {
    pub major_opcode :   u8,
    pub minor_opcode :   u8,
    pub length :         u16,
    pub xid :            u32
}

impl Copy for xcb_res_query_client_pixmap_bytes_request_t {}
impl Clone for xcb_res_query_client_pixmap_bytes_request_t {
    fn clone(&self) -> xcb_res_query_client_pixmap_bytes_request_t { *self }
}

#[repr(C)]
pub struct xcb_res_query_client_pixmap_bytes_reply_t {
    pub response_type :    u8,
    pub pad0 :             u8,
    pub sequence :         u16,
    pub length :           u32,
    pub bytes :            u32,
    pub bytes_overflow :   u32
}

impl Copy for xcb_res_query_client_pixmap_bytes_reply_t {}
impl Clone for xcb_res_query_client_pixmap_bytes_reply_t {
    fn clone(&self) -> xcb_res_query_client_pixmap_bytes_reply_t { *self }
}
#[link(name="xcb-res")]
extern "C" {

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_res_client_t)
///
pub fn xcb_res_client_next (i:*mut xcb_res_client_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_res_client_end (i:xcb_res_client_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_res_type_t)
///
pub fn xcb_res_type_next (i:*mut xcb_res_type_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_res_type_end (i:xcb_res_type_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/// Delivers a request to the X server.
///
pub fn xcb_res_query_version (c : *mut ffi::base::xcb_connection_t,
                                 client_major :  u8,
                                 client_minor :  u8) -> xcb_res_query_version_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_res_query_version_unchecked (c : *mut ffi::base::xcb_connection_t,
                                           client_major :  u8,
                                           client_minor :  u8) -> xcb_res_query_version_cookie_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_res_query_version_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_res_query_version_reply (c : *mut ffi::base::xcb_connection_t,
                                       cookie : xcb_res_query_version_cookie_t,
                                       e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_res_query_version_reply_t;

pub fn xcb_res_query_clients_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
///
pub fn xcb_res_query_clients (c : *mut ffi::base::xcb_connection_t) -> xcb_res_query_clients_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_res_query_clients_unchecked (c : *mut ffi::base::xcb_connection_t) -> xcb_res_query_clients_cookie_t;

pub fn xcb_res_query_clients_clients (R : *mut xcb_res_query_clients_reply_t) -> *mut xcb_res_client_t;


pub fn xcb_res_query_clients_clients_length (R : *mut xcb_res_query_clients_reply_t) -> c_int;

pub fn xcb_res_query_clients_clients_iterator (R : *mut xcb_res_query_clients_reply_t) -> xcb_res_client_iterator_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_res_query_clients_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_res_query_clients_reply (c : *mut ffi::base::xcb_connection_t,
                                       cookie : xcb_res_query_clients_cookie_t,
                                       e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_res_query_clients_reply_t;

pub fn xcb_res_query_client_resources_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
///
pub fn xcb_res_query_client_resources (c : *mut ffi::base::xcb_connection_t,
                                          xid :  u32) -> xcb_res_query_client_resources_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_res_query_client_resources_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                    xid :  u32) -> xcb_res_query_client_resources_cookie_t;

pub fn xcb_res_query_client_resources_types (R : *mut xcb_res_query_client_resources_reply_t) -> *mut xcb_res_type_t;


pub fn xcb_res_query_client_resources_types_length (R : *mut xcb_res_query_client_resources_reply_t) -> c_int;

pub fn xcb_res_query_client_resources_types_iterator (R : *mut xcb_res_query_client_resources_reply_t) -> xcb_res_type_iterator_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_res_query_client_resources_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_res_query_client_resources_reply (c : *mut ffi::base::xcb_connection_t,
                                                cookie : xcb_res_query_client_resources_cookie_t,
                                                e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_res_query_client_resources_reply_t;

/// Delivers a request to the X server.
///
pub fn xcb_res_query_client_pixmap_bytes (c : *mut ffi::base::xcb_connection_t,
                                             xid :  u32) -> xcb_res_query_client_pixmap_bytes_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_res_query_client_pixmap_bytes_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                       xid :  u32) -> xcb_res_query_client_pixmap_bytes_cookie_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_res_query_client_pixmap_bytes_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_res_query_client_pixmap_bytes_reply (c : *mut ffi::base::xcb_connection_t,
                                                   cookie : xcb_res_query_client_pixmap_bytes_cookie_t,
                                                   e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_res_query_client_pixmap_bytes_reply_t;
}

