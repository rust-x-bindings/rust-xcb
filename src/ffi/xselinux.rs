//
// This file generated automatically from xselinux.xml by r_client.py.
// Edit at your peril.
//

//Make the compiler quiet
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use std;
use libc::*;
use ffi;
use ffi::xproto;

pub const SELINUX_MAJOR_VERSION : c_uint = 1;
pub const SELINUX_MINOR_VERSION : c_uint = 0;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_selinux_query_version_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_selinux_query_version_request_t {
    pub major_opcode :   u8,
    pub minor_opcode :   u8,
    pub length :         u16,
    pub client_major :   u8,
    pub client_minor :   u8
}

impl Copy for xcb_selinux_query_version_request_t {}
impl Clone for xcb_selinux_query_version_request_t {
    fn clone(&self) -> xcb_selinux_query_version_request_t { *self }
}

#[repr(C)]
pub struct xcb_selinux_query_version_reply_t {
    pub response_type :   u8,
    pub pad0 :            u8,
    pub sequence :        u16,
    pub length :          u32,
    pub server_major :    u16,
    pub server_minor :    u16
}

impl Copy for xcb_selinux_query_version_reply_t {}
impl Clone for xcb_selinux_query_version_reply_t {
    fn clone(&self) -> xcb_selinux_query_version_reply_t { *self }
}


#[repr(C)]
pub struct xcb_selinux_set_device_create_context_request_t {
    pub major_opcode :   u8,
    pub minor_opcode :   u8,
    pub length :         u16,
    pub context_len :    u32
}

impl Copy for xcb_selinux_set_device_create_context_request_t {}
impl Clone for xcb_selinux_set_device_create_context_request_t {
    fn clone(&self) -> xcb_selinux_set_device_create_context_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_selinux_get_device_create_context_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_selinux_get_device_create_context_request_t {
    pub major_opcode :   u8,
    pub minor_opcode :   u8,
    pub length :         u16
}

impl Copy for xcb_selinux_get_device_create_context_request_t {}
impl Clone for xcb_selinux_get_device_create_context_request_t {
    fn clone(&self) -> xcb_selinux_get_device_create_context_request_t { *self }
}

#[repr(C)]
pub struct xcb_selinux_get_device_create_context_reply_t {
    pub response_type :   u8,
    pub pad0 :            u8,
    pub sequence :        u16,
    pub length :          u32,
    pub context_len :     u32,
    pub pad1 :            [u8; 20]
}

impl Copy for xcb_selinux_get_device_create_context_reply_t {}
impl Clone for xcb_selinux_get_device_create_context_reply_t {
    fn clone(&self) -> xcb_selinux_get_device_create_context_reply_t { *self }
}


#[repr(C)]
pub struct xcb_selinux_set_device_context_request_t {
    pub major_opcode :   u8,
    pub minor_opcode :   u8,
    pub length :         u16,
    pub device :         u32,
    pub context_len :    u32
}

impl Copy for xcb_selinux_set_device_context_request_t {}
impl Clone for xcb_selinux_set_device_context_request_t {
    fn clone(&self) -> xcb_selinux_set_device_context_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_selinux_get_device_context_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_selinux_get_device_context_request_t {
    pub major_opcode :   u8,
    pub minor_opcode :   u8,
    pub length :         u16,
    pub device :         u32
}

impl Copy for xcb_selinux_get_device_context_request_t {}
impl Clone for xcb_selinux_get_device_context_request_t {
    fn clone(&self) -> xcb_selinux_get_device_context_request_t { *self }
}

#[repr(C)]
pub struct xcb_selinux_get_device_context_reply_t {
    pub response_type :   u8,
    pub pad0 :            u8,
    pub sequence :        u16,
    pub length :          u32,
    pub context_len :     u32,
    pub pad1 :            [u8; 20]
}

impl Copy for xcb_selinux_get_device_context_reply_t {}
impl Clone for xcb_selinux_get_device_context_reply_t {
    fn clone(&self) -> xcb_selinux_get_device_context_reply_t { *self }
}


#[repr(C)]
pub struct xcb_selinux_set_window_create_context_request_t {
    pub major_opcode :   u8,
    pub minor_opcode :   u8,
    pub length :         u16,
    pub context_len :    u32
}

impl Copy for xcb_selinux_set_window_create_context_request_t {}
impl Clone for xcb_selinux_set_window_create_context_request_t {
    fn clone(&self) -> xcb_selinux_set_window_create_context_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_selinux_get_window_create_context_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_selinux_get_window_create_context_request_t {
    pub major_opcode :   u8,
    pub minor_opcode :   u8,
    pub length :         u16
}

impl Copy for xcb_selinux_get_window_create_context_request_t {}
impl Clone for xcb_selinux_get_window_create_context_request_t {
    fn clone(&self) -> xcb_selinux_get_window_create_context_request_t { *self }
}

#[repr(C)]
pub struct xcb_selinux_get_window_create_context_reply_t {
    pub response_type :   u8,
    pub pad0 :            u8,
    pub sequence :        u16,
    pub length :          u32,
    pub context_len :     u32,
    pub pad1 :            [u8; 20]
}

impl Copy for xcb_selinux_get_window_create_context_reply_t {}
impl Clone for xcb_selinux_get_window_create_context_reply_t {
    fn clone(&self) -> xcb_selinux_get_window_create_context_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_selinux_get_window_context_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_selinux_get_window_context_request_t {
    pub major_opcode :   u8,
    pub minor_opcode :   u8,
    pub length :         u16,
    pub window :         ffi::xproto::xcb_window_t
}

impl Copy for xcb_selinux_get_window_context_request_t {}
impl Clone for xcb_selinux_get_window_context_request_t {
    fn clone(&self) -> xcb_selinux_get_window_context_request_t { *self }
}

#[repr(C)]
pub struct xcb_selinux_get_window_context_reply_t {
    pub response_type :   u8,
    pub pad0 :            u8,
    pub sequence :        u16,
    pub length :          u32,
    pub context_len :     u32,
    pub pad1 :            [u8; 20]
}

impl Copy for xcb_selinux_get_window_context_reply_t {}
impl Clone for xcb_selinux_get_window_context_reply_t {
    fn clone(&self) -> xcb_selinux_get_window_context_reply_t { *self }
}

#[repr(C)]
pub struct xcb_selinux_list_item_t {
    pub name :                 ffi::xproto::xcb_atom_t,
    pub object_context_len :   u32,
    pub data_context_len :     u32
}

impl Copy for xcb_selinux_list_item_t {}
impl Clone for xcb_selinux_list_item_t {
    fn clone(&self) -> xcb_selinux_list_item_t { *self }
}
#[repr(C)]
pub struct xcb_selinux_list_item_iterator_t {
    pub data : *mut xcb_selinux_list_item_t,
    pub rem  : c_int,
    pub index: c_int
}



#[repr(C)]
pub struct xcb_selinux_set_property_create_context_request_t {
    pub major_opcode :   u8,
    pub minor_opcode :   u8,
    pub length :         u16,
    pub context_len :    u32
}

impl Copy for xcb_selinux_set_property_create_context_request_t {}
impl Clone for xcb_selinux_set_property_create_context_request_t {
    fn clone(&self) -> xcb_selinux_set_property_create_context_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_selinux_get_property_create_context_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_selinux_get_property_create_context_request_t {
    pub major_opcode :   u8,
    pub minor_opcode :   u8,
    pub length :         u16
}

impl Copy for xcb_selinux_get_property_create_context_request_t {}
impl Clone for xcb_selinux_get_property_create_context_request_t {
    fn clone(&self) -> xcb_selinux_get_property_create_context_request_t { *self }
}

#[repr(C)]
pub struct xcb_selinux_get_property_create_context_reply_t {
    pub response_type :   u8,
    pub pad0 :            u8,
    pub sequence :        u16,
    pub length :          u32,
    pub context_len :     u32,
    pub pad1 :            [u8; 20]
}

impl Copy for xcb_selinux_get_property_create_context_reply_t {}
impl Clone for xcb_selinux_get_property_create_context_reply_t {
    fn clone(&self) -> xcb_selinux_get_property_create_context_reply_t { *self }
}


#[repr(C)]
pub struct xcb_selinux_set_property_use_context_request_t {
    pub major_opcode :   u8,
    pub minor_opcode :   u8,
    pub length :         u16,
    pub context_len :    u32
}

impl Copy for xcb_selinux_set_property_use_context_request_t {}
impl Clone for xcb_selinux_set_property_use_context_request_t {
    fn clone(&self) -> xcb_selinux_set_property_use_context_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_selinux_get_property_use_context_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_selinux_get_property_use_context_request_t {
    pub major_opcode :   u8,
    pub minor_opcode :   u8,
    pub length :         u16
}

impl Copy for xcb_selinux_get_property_use_context_request_t {}
impl Clone for xcb_selinux_get_property_use_context_request_t {
    fn clone(&self) -> xcb_selinux_get_property_use_context_request_t { *self }
}

#[repr(C)]
pub struct xcb_selinux_get_property_use_context_reply_t {
    pub response_type :   u8,
    pub pad0 :            u8,
    pub sequence :        u16,
    pub length :          u32,
    pub context_len :     u32,
    pub pad1 :            [u8; 20]
}

impl Copy for xcb_selinux_get_property_use_context_reply_t {}
impl Clone for xcb_selinux_get_property_use_context_reply_t {
    fn clone(&self) -> xcb_selinux_get_property_use_context_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_selinux_get_property_context_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_selinux_get_property_context_request_t {
    pub major_opcode :   u8,
    pub minor_opcode :   u8,
    pub length :         u16,
    pub window :         ffi::xproto::xcb_window_t,
    pub property :       ffi::xproto::xcb_atom_t
}

impl Copy for xcb_selinux_get_property_context_request_t {}
impl Clone for xcb_selinux_get_property_context_request_t {
    fn clone(&self) -> xcb_selinux_get_property_context_request_t { *self }
}

#[repr(C)]
pub struct xcb_selinux_get_property_context_reply_t {
    pub response_type :   u8,
    pub pad0 :            u8,
    pub sequence :        u16,
    pub length :          u32,
    pub context_len :     u32,
    pub pad1 :            [u8; 20]
}

impl Copy for xcb_selinux_get_property_context_reply_t {}
impl Clone for xcb_selinux_get_property_context_reply_t {
    fn clone(&self) -> xcb_selinux_get_property_context_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_selinux_get_property_data_context_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_selinux_get_property_data_context_request_t {
    pub major_opcode :   u8,
    pub minor_opcode :   u8,
    pub length :         u16,
    pub window :         ffi::xproto::xcb_window_t,
    pub property :       ffi::xproto::xcb_atom_t
}

impl Copy for xcb_selinux_get_property_data_context_request_t {}
impl Clone for xcb_selinux_get_property_data_context_request_t {
    fn clone(&self) -> xcb_selinux_get_property_data_context_request_t { *self }
}

#[repr(C)]
pub struct xcb_selinux_get_property_data_context_reply_t {
    pub response_type :   u8,
    pub pad0 :            u8,
    pub sequence :        u16,
    pub length :          u32,
    pub context_len :     u32,
    pub pad1 :            [u8; 20]
}

impl Copy for xcb_selinux_get_property_data_context_reply_t {}
impl Clone for xcb_selinux_get_property_data_context_reply_t {
    fn clone(&self) -> xcb_selinux_get_property_data_context_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_selinux_list_properties_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_selinux_list_properties_request_t {
    pub major_opcode :   u8,
    pub minor_opcode :   u8,
    pub length :         u16,
    pub window :         ffi::xproto::xcb_window_t
}

impl Copy for xcb_selinux_list_properties_request_t {}
impl Clone for xcb_selinux_list_properties_request_t {
    fn clone(&self) -> xcb_selinux_list_properties_request_t { *self }
}

#[repr(C)]
pub struct xcb_selinux_list_properties_reply_t {
    pub response_type :    u8,
    pub pad0 :             u8,
    pub sequence :         u16,
    pub length :           u32,
    pub properties_len :   u32,
    pub pad1 :             [u8; 20]
}

impl Copy for xcb_selinux_list_properties_reply_t {}
impl Clone for xcb_selinux_list_properties_reply_t {
    fn clone(&self) -> xcb_selinux_list_properties_reply_t { *self }
}


#[repr(C)]
pub struct xcb_selinux_set_selection_create_context_request_t {
    pub major_opcode :   u8,
    pub minor_opcode :   u8,
    pub length :         u16,
    pub context_len :    u32
}

impl Copy for xcb_selinux_set_selection_create_context_request_t {}
impl Clone for xcb_selinux_set_selection_create_context_request_t {
    fn clone(&self) -> xcb_selinux_set_selection_create_context_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_selinux_get_selection_create_context_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_selinux_get_selection_create_context_request_t {
    pub major_opcode :   u8,
    pub minor_opcode :   u8,
    pub length :         u16
}

impl Copy for xcb_selinux_get_selection_create_context_request_t {}
impl Clone for xcb_selinux_get_selection_create_context_request_t {
    fn clone(&self) -> xcb_selinux_get_selection_create_context_request_t { *self }
}

#[repr(C)]
pub struct xcb_selinux_get_selection_create_context_reply_t {
    pub response_type :   u8,
    pub pad0 :            u8,
    pub sequence :        u16,
    pub length :          u32,
    pub context_len :     u32,
    pub pad1 :            [u8; 20]
}

impl Copy for xcb_selinux_get_selection_create_context_reply_t {}
impl Clone for xcb_selinux_get_selection_create_context_reply_t {
    fn clone(&self) -> xcb_selinux_get_selection_create_context_reply_t { *self }
}


#[repr(C)]
pub struct xcb_selinux_set_selection_use_context_request_t {
    pub major_opcode :   u8,
    pub minor_opcode :   u8,
    pub length :         u16,
    pub context_len :    u32
}

impl Copy for xcb_selinux_set_selection_use_context_request_t {}
impl Clone for xcb_selinux_set_selection_use_context_request_t {
    fn clone(&self) -> xcb_selinux_set_selection_use_context_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_selinux_get_selection_use_context_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_selinux_get_selection_use_context_request_t {
    pub major_opcode :   u8,
    pub minor_opcode :   u8,
    pub length :         u16
}

impl Copy for xcb_selinux_get_selection_use_context_request_t {}
impl Clone for xcb_selinux_get_selection_use_context_request_t {
    fn clone(&self) -> xcb_selinux_get_selection_use_context_request_t { *self }
}

#[repr(C)]
pub struct xcb_selinux_get_selection_use_context_reply_t {
    pub response_type :   u8,
    pub pad0 :            u8,
    pub sequence :        u16,
    pub length :          u32,
    pub context_len :     u32,
    pub pad1 :            [u8; 20]
}

impl Copy for xcb_selinux_get_selection_use_context_reply_t {}
impl Clone for xcb_selinux_get_selection_use_context_reply_t {
    fn clone(&self) -> xcb_selinux_get_selection_use_context_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_selinux_get_selection_context_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_selinux_get_selection_context_request_t {
    pub major_opcode :   u8,
    pub minor_opcode :   u8,
    pub length :         u16,
    pub selection :      ffi::xproto::xcb_atom_t
}

impl Copy for xcb_selinux_get_selection_context_request_t {}
impl Clone for xcb_selinux_get_selection_context_request_t {
    fn clone(&self) -> xcb_selinux_get_selection_context_request_t { *self }
}

#[repr(C)]
pub struct xcb_selinux_get_selection_context_reply_t {
    pub response_type :   u8,
    pub pad0 :            u8,
    pub sequence :        u16,
    pub length :          u32,
    pub context_len :     u32,
    pub pad1 :            [u8; 20]
}

impl Copy for xcb_selinux_get_selection_context_reply_t {}
impl Clone for xcb_selinux_get_selection_context_reply_t {
    fn clone(&self) -> xcb_selinux_get_selection_context_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_selinux_get_selection_data_context_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_selinux_get_selection_data_context_request_t {
    pub major_opcode :   u8,
    pub minor_opcode :   u8,
    pub length :         u16,
    pub selection :      ffi::xproto::xcb_atom_t
}

impl Copy for xcb_selinux_get_selection_data_context_request_t {}
impl Clone for xcb_selinux_get_selection_data_context_request_t {
    fn clone(&self) -> xcb_selinux_get_selection_data_context_request_t { *self }
}

#[repr(C)]
pub struct xcb_selinux_get_selection_data_context_reply_t {
    pub response_type :   u8,
    pub pad0 :            u8,
    pub sequence :        u16,
    pub length :          u32,
    pub context_len :     u32,
    pub pad1 :            [u8; 20]
}

impl Copy for xcb_selinux_get_selection_data_context_reply_t {}
impl Clone for xcb_selinux_get_selection_data_context_reply_t {
    fn clone(&self) -> xcb_selinux_get_selection_data_context_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_selinux_list_selections_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_selinux_list_selections_request_t {
    pub major_opcode :   u8,
    pub minor_opcode :   u8,
    pub length :         u16
}

impl Copy for xcb_selinux_list_selections_request_t {}
impl Clone for xcb_selinux_list_selections_request_t {
    fn clone(&self) -> xcb_selinux_list_selections_request_t { *self }
}

#[repr(C)]
pub struct xcb_selinux_list_selections_reply_t {
    pub response_type :    u8,
    pub pad0 :             u8,
    pub sequence :         u16,
    pub length :           u32,
    pub selections_len :   u32,
    pub pad1 :             [u8; 20]
}

impl Copy for xcb_selinux_list_selections_reply_t {}
impl Clone for xcb_selinux_list_selections_reply_t {
    fn clone(&self) -> xcb_selinux_list_selections_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_selinux_get_client_context_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_selinux_get_client_context_request_t {
    pub major_opcode :   u8,
    pub minor_opcode :   u8,
    pub length :         u16,
    pub resource :       u32
}

impl Copy for xcb_selinux_get_client_context_request_t {}
impl Clone for xcb_selinux_get_client_context_request_t {
    fn clone(&self) -> xcb_selinux_get_client_context_request_t { *self }
}

#[repr(C)]
pub struct xcb_selinux_get_client_context_reply_t {
    pub response_type :   u8,
    pub pad0 :            u8,
    pub sequence :        u16,
    pub length :          u32,
    pub context_len :     u32,
    pub pad1 :            [u8; 20]
}

impl Copy for xcb_selinux_get_client_context_reply_t {}
impl Clone for xcb_selinux_get_client_context_reply_t {
    fn clone(&self) -> xcb_selinux_get_client_context_reply_t { *self }
}
#[link(name="xcb-xselinux")]
extern "C" {

/// Delivers a request to the X server.
///
pub fn xcb_selinux_query_version (c : *mut ffi::base::xcb_connection_t,
                                     client_major :  u8,
                                     client_minor :  u8) -> xcb_selinux_query_version_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_selinux_query_version_unchecked (c : *mut ffi::base::xcb_connection_t,
                                               client_major :  u8,
                                               client_minor :  u8) -> xcb_selinux_query_version_cookie_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_selinux_query_version_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_selinux_query_version_reply (c : *mut ffi::base::xcb_connection_t,
                                           cookie : xcb_selinux_query_version_cookie_t,
                                           e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_selinux_query_version_reply_t;

pub fn xcb_selinux_set_device_create_context_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_selinux_set_device_create_context_checked (c : *mut ffi::base::xcb_connection_t,
                                                         context_len :  u32,
                                                         context : *mut c_char) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_selinux_set_device_create_context (c : *mut ffi::base::xcb_connection_t,
                                                 context_len :  u32,
                                                 context : *mut c_char) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_selinux_get_device_create_context_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
///
pub fn xcb_selinux_get_device_create_context (c : *mut ffi::base::xcb_connection_t) -> xcb_selinux_get_device_create_context_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_selinux_get_device_create_context_unchecked (c : *mut ffi::base::xcb_connection_t) -> xcb_selinux_get_device_create_context_cookie_t;

pub fn xcb_selinux_get_device_create_context_context (R : *mut xcb_selinux_get_device_create_context_reply_t) -> *mut c_char;


pub fn xcb_selinux_get_device_create_context_context_length (R : *mut xcb_selinux_get_device_create_context_reply_t) -> c_int;


pub fn xcb_selinux_get_device_create_context_context_end (R : *mut xcb_selinux_get_device_create_context_reply_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_selinux_get_device_create_context_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_selinux_get_device_create_context_reply (c : *mut ffi::base::xcb_connection_t,
                                                       cookie : xcb_selinux_get_device_create_context_cookie_t,
                                                       e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_selinux_get_device_create_context_reply_t;

pub fn xcb_selinux_set_device_context_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_selinux_set_device_context_checked (c : *mut ffi::base::xcb_connection_t,
                                                  device :  u32,
                                                  context_len :  u32,
                                                  context : *mut c_char) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_selinux_set_device_context (c : *mut ffi::base::xcb_connection_t,
                                          device :  u32,
                                          context_len :  u32,
                                          context : *mut c_char) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_selinux_get_device_context_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
///
pub fn xcb_selinux_get_device_context (c : *mut ffi::base::xcb_connection_t,
                                          device :  u32) -> xcb_selinux_get_device_context_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_selinux_get_device_context_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                    device :  u32) -> xcb_selinux_get_device_context_cookie_t;

pub fn xcb_selinux_get_device_context_context (R : *mut xcb_selinux_get_device_context_reply_t) -> *mut c_char;


pub fn xcb_selinux_get_device_context_context_length (R : *mut xcb_selinux_get_device_context_reply_t) -> c_int;


pub fn xcb_selinux_get_device_context_context_end (R : *mut xcb_selinux_get_device_context_reply_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_selinux_get_device_context_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_selinux_get_device_context_reply (c : *mut ffi::base::xcb_connection_t,
                                                cookie : xcb_selinux_get_device_context_cookie_t,
                                                e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_selinux_get_device_context_reply_t;

pub fn xcb_selinux_set_window_create_context_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_selinux_set_window_create_context_checked (c : *mut ffi::base::xcb_connection_t,
                                                         context_len :  u32,
                                                         context : *mut c_char) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_selinux_set_window_create_context (c : *mut ffi::base::xcb_connection_t,
                                                 context_len :  u32,
                                                 context : *mut c_char) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_selinux_get_window_create_context_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
///
pub fn xcb_selinux_get_window_create_context (c : *mut ffi::base::xcb_connection_t) -> xcb_selinux_get_window_create_context_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_selinux_get_window_create_context_unchecked (c : *mut ffi::base::xcb_connection_t) -> xcb_selinux_get_window_create_context_cookie_t;

pub fn xcb_selinux_get_window_create_context_context (R : *mut xcb_selinux_get_window_create_context_reply_t) -> *mut c_char;


pub fn xcb_selinux_get_window_create_context_context_length (R : *mut xcb_selinux_get_window_create_context_reply_t) -> c_int;


pub fn xcb_selinux_get_window_create_context_context_end (R : *mut xcb_selinux_get_window_create_context_reply_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_selinux_get_window_create_context_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_selinux_get_window_create_context_reply (c : *mut ffi::base::xcb_connection_t,
                                                       cookie : xcb_selinux_get_window_create_context_cookie_t,
                                                       e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_selinux_get_window_create_context_reply_t;

pub fn xcb_selinux_get_window_context_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
///
pub fn xcb_selinux_get_window_context (c : *mut ffi::base::xcb_connection_t,
                                          window :  ffi::xproto::xcb_window_t) -> xcb_selinux_get_window_context_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_selinux_get_window_context_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                    window :  ffi::xproto::xcb_window_t) -> xcb_selinux_get_window_context_cookie_t;

pub fn xcb_selinux_get_window_context_context (R : *mut xcb_selinux_get_window_context_reply_t) -> *mut c_char;


pub fn xcb_selinux_get_window_context_context_length (R : *mut xcb_selinux_get_window_context_reply_t) -> c_int;


pub fn xcb_selinux_get_window_context_context_end (R : *mut xcb_selinux_get_window_context_reply_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_selinux_get_window_context_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_selinux_get_window_context_reply (c : *mut ffi::base::xcb_connection_t,
                                                cookie : xcb_selinux_get_window_context_cookie_t,
                                                e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_selinux_get_window_context_reply_t;

pub fn xcb_selinux_list_item_sizeof (_buffer :  *mut c_void) -> c_int;

pub fn xcb_selinux_list_item_object_context (R : *mut xcb_selinux_list_item_t) -> *mut c_char;


pub fn xcb_selinux_list_item_object_context_length (R : *mut xcb_selinux_list_item_t) -> c_int;


pub fn xcb_selinux_list_item_object_context_end (R : *mut xcb_selinux_list_item_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_selinux_list_item_data_context (R : *mut xcb_selinux_list_item_t) -> *mut c_char;


pub fn xcb_selinux_list_item_data_context_length (R : *mut xcb_selinux_list_item_t) -> c_int;


pub fn xcb_selinux_list_item_data_context_end (R : *mut xcb_selinux_list_item_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_selinux_list_item_t)
///
pub fn xcb_selinux_list_item_next (i:*mut xcb_selinux_list_item_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_selinux_list_item_end (i:xcb_selinux_list_item_iterator_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_selinux_set_property_create_context_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_selinux_set_property_create_context_checked (c : *mut ffi::base::xcb_connection_t,
                                                           context_len :  u32,
                                                           context : *mut c_char) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_selinux_set_property_create_context (c : *mut ffi::base::xcb_connection_t,
                                                   context_len :  u32,
                                                   context : *mut c_char) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_selinux_get_property_create_context_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
///
pub fn xcb_selinux_get_property_create_context (c : *mut ffi::base::xcb_connection_t) -> xcb_selinux_get_property_create_context_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_selinux_get_property_create_context_unchecked (c : *mut ffi::base::xcb_connection_t) -> xcb_selinux_get_property_create_context_cookie_t;

pub fn xcb_selinux_get_property_create_context_context (R : *mut xcb_selinux_get_property_create_context_reply_t) -> *mut c_char;


pub fn xcb_selinux_get_property_create_context_context_length (R : *mut xcb_selinux_get_property_create_context_reply_t) -> c_int;


pub fn xcb_selinux_get_property_create_context_context_end (R : *mut xcb_selinux_get_property_create_context_reply_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_selinux_get_property_create_context_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_selinux_get_property_create_context_reply (c : *mut ffi::base::xcb_connection_t,
                                                         cookie : xcb_selinux_get_property_create_context_cookie_t,
                                                         e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_selinux_get_property_create_context_reply_t;

pub fn xcb_selinux_set_property_use_context_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_selinux_set_property_use_context_checked (c : *mut ffi::base::xcb_connection_t,
                                                        context_len :  u32,
                                                        context : *mut c_char) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_selinux_set_property_use_context (c : *mut ffi::base::xcb_connection_t,
                                                context_len :  u32,
                                                context : *mut c_char) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_selinux_get_property_use_context_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
///
pub fn xcb_selinux_get_property_use_context (c : *mut ffi::base::xcb_connection_t) -> xcb_selinux_get_property_use_context_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_selinux_get_property_use_context_unchecked (c : *mut ffi::base::xcb_connection_t) -> xcb_selinux_get_property_use_context_cookie_t;

pub fn xcb_selinux_get_property_use_context_context (R : *mut xcb_selinux_get_property_use_context_reply_t) -> *mut c_char;


pub fn xcb_selinux_get_property_use_context_context_length (R : *mut xcb_selinux_get_property_use_context_reply_t) -> c_int;


pub fn xcb_selinux_get_property_use_context_context_end (R : *mut xcb_selinux_get_property_use_context_reply_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_selinux_get_property_use_context_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_selinux_get_property_use_context_reply (c : *mut ffi::base::xcb_connection_t,
                                                      cookie : xcb_selinux_get_property_use_context_cookie_t,
                                                      e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_selinux_get_property_use_context_reply_t;

pub fn xcb_selinux_get_property_context_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
///
pub fn xcb_selinux_get_property_context (c : *mut ffi::base::xcb_connection_t,
                                            window :  ffi::xproto::xcb_window_t,
                                            property :  ffi::xproto::xcb_atom_t) -> xcb_selinux_get_property_context_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_selinux_get_property_context_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                      window :  ffi::xproto::xcb_window_t,
                                                      property :  ffi::xproto::xcb_atom_t) -> xcb_selinux_get_property_context_cookie_t;

pub fn xcb_selinux_get_property_context_context (R : *mut xcb_selinux_get_property_context_reply_t) -> *mut c_char;


pub fn xcb_selinux_get_property_context_context_length (R : *mut xcb_selinux_get_property_context_reply_t) -> c_int;


pub fn xcb_selinux_get_property_context_context_end (R : *mut xcb_selinux_get_property_context_reply_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_selinux_get_property_context_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_selinux_get_property_context_reply (c : *mut ffi::base::xcb_connection_t,
                                                  cookie : xcb_selinux_get_property_context_cookie_t,
                                                  e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_selinux_get_property_context_reply_t;

pub fn xcb_selinux_get_property_data_context_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
///
pub fn xcb_selinux_get_property_data_context (c : *mut ffi::base::xcb_connection_t,
                                                 window :  ffi::xproto::xcb_window_t,
                                                 property :  ffi::xproto::xcb_atom_t) -> xcb_selinux_get_property_data_context_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_selinux_get_property_data_context_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                           window :  ffi::xproto::xcb_window_t,
                                                           property :  ffi::xproto::xcb_atom_t) -> xcb_selinux_get_property_data_context_cookie_t;

pub fn xcb_selinux_get_property_data_context_context (R : *mut xcb_selinux_get_property_data_context_reply_t) -> *mut c_char;


pub fn xcb_selinux_get_property_data_context_context_length (R : *mut xcb_selinux_get_property_data_context_reply_t) -> c_int;


pub fn xcb_selinux_get_property_data_context_context_end (R : *mut xcb_selinux_get_property_data_context_reply_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_selinux_get_property_data_context_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_selinux_get_property_data_context_reply (c : *mut ffi::base::xcb_connection_t,
                                                       cookie : xcb_selinux_get_property_data_context_cookie_t,
                                                       e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_selinux_get_property_data_context_reply_t;

pub fn xcb_selinux_list_properties_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
///
pub fn xcb_selinux_list_properties (c : *mut ffi::base::xcb_connection_t,
                                       window :  ffi::xproto::xcb_window_t) -> xcb_selinux_list_properties_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_selinux_list_properties_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                 window :  ffi::xproto::xcb_window_t) -> xcb_selinux_list_properties_cookie_t;


pub fn xcb_selinux_list_properties_properties_length (R : *mut xcb_selinux_list_properties_reply_t) -> c_int;

pub fn xcb_selinux_list_properties_properties_iterator (R : *mut xcb_selinux_list_properties_reply_t) -> xcb_selinux_list_item_iterator_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_selinux_list_properties_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_selinux_list_properties_reply (c : *mut ffi::base::xcb_connection_t,
                                             cookie : xcb_selinux_list_properties_cookie_t,
                                             e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_selinux_list_properties_reply_t;

pub fn xcb_selinux_set_selection_create_context_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_selinux_set_selection_create_context_checked (c : *mut ffi::base::xcb_connection_t,
                                                            context_len :  u32,
                                                            context : *mut c_char) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_selinux_set_selection_create_context (c : *mut ffi::base::xcb_connection_t,
                                                    context_len :  u32,
                                                    context : *mut c_char) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_selinux_get_selection_create_context_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
///
pub fn xcb_selinux_get_selection_create_context (c : *mut ffi::base::xcb_connection_t) -> xcb_selinux_get_selection_create_context_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_selinux_get_selection_create_context_unchecked (c : *mut ffi::base::xcb_connection_t) -> xcb_selinux_get_selection_create_context_cookie_t;

pub fn xcb_selinux_get_selection_create_context_context (R : *mut xcb_selinux_get_selection_create_context_reply_t) -> *mut c_char;


pub fn xcb_selinux_get_selection_create_context_context_length (R : *mut xcb_selinux_get_selection_create_context_reply_t) -> c_int;


pub fn xcb_selinux_get_selection_create_context_context_end (R : *mut xcb_selinux_get_selection_create_context_reply_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_selinux_get_selection_create_context_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_selinux_get_selection_create_context_reply (c : *mut ffi::base::xcb_connection_t,
                                                          cookie : xcb_selinux_get_selection_create_context_cookie_t,
                                                          e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_selinux_get_selection_create_context_reply_t;

pub fn xcb_selinux_set_selection_use_context_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_selinux_set_selection_use_context_checked (c : *mut ffi::base::xcb_connection_t,
                                                         context_len :  u32,
                                                         context : *mut c_char) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_selinux_set_selection_use_context (c : *mut ffi::base::xcb_connection_t,
                                                 context_len :  u32,
                                                 context : *mut c_char) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_selinux_get_selection_use_context_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
///
pub fn xcb_selinux_get_selection_use_context (c : *mut ffi::base::xcb_connection_t) -> xcb_selinux_get_selection_use_context_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_selinux_get_selection_use_context_unchecked (c : *mut ffi::base::xcb_connection_t) -> xcb_selinux_get_selection_use_context_cookie_t;

pub fn xcb_selinux_get_selection_use_context_context (R : *mut xcb_selinux_get_selection_use_context_reply_t) -> *mut c_char;


pub fn xcb_selinux_get_selection_use_context_context_length (R : *mut xcb_selinux_get_selection_use_context_reply_t) -> c_int;


pub fn xcb_selinux_get_selection_use_context_context_end (R : *mut xcb_selinux_get_selection_use_context_reply_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_selinux_get_selection_use_context_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_selinux_get_selection_use_context_reply (c : *mut ffi::base::xcb_connection_t,
                                                       cookie : xcb_selinux_get_selection_use_context_cookie_t,
                                                       e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_selinux_get_selection_use_context_reply_t;

pub fn xcb_selinux_get_selection_context_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
///
pub fn xcb_selinux_get_selection_context (c : *mut ffi::base::xcb_connection_t,
                                             selection :  ffi::xproto::xcb_atom_t) -> xcb_selinux_get_selection_context_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_selinux_get_selection_context_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                       selection :  ffi::xproto::xcb_atom_t) -> xcb_selinux_get_selection_context_cookie_t;

pub fn xcb_selinux_get_selection_context_context (R : *mut xcb_selinux_get_selection_context_reply_t) -> *mut c_char;


pub fn xcb_selinux_get_selection_context_context_length (R : *mut xcb_selinux_get_selection_context_reply_t) -> c_int;


pub fn xcb_selinux_get_selection_context_context_end (R : *mut xcb_selinux_get_selection_context_reply_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_selinux_get_selection_context_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_selinux_get_selection_context_reply (c : *mut ffi::base::xcb_connection_t,
                                                   cookie : xcb_selinux_get_selection_context_cookie_t,
                                                   e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_selinux_get_selection_context_reply_t;

pub fn xcb_selinux_get_selection_data_context_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
///
pub fn xcb_selinux_get_selection_data_context (c : *mut ffi::base::xcb_connection_t,
                                                  selection :  ffi::xproto::xcb_atom_t) -> xcb_selinux_get_selection_data_context_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_selinux_get_selection_data_context_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                            selection :  ffi::xproto::xcb_atom_t) -> xcb_selinux_get_selection_data_context_cookie_t;

pub fn xcb_selinux_get_selection_data_context_context (R : *mut xcb_selinux_get_selection_data_context_reply_t) -> *mut c_char;


pub fn xcb_selinux_get_selection_data_context_context_length (R : *mut xcb_selinux_get_selection_data_context_reply_t) -> c_int;


pub fn xcb_selinux_get_selection_data_context_context_end (R : *mut xcb_selinux_get_selection_data_context_reply_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_selinux_get_selection_data_context_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_selinux_get_selection_data_context_reply (c : *mut ffi::base::xcb_connection_t,
                                                        cookie : xcb_selinux_get_selection_data_context_cookie_t,
                                                        e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_selinux_get_selection_data_context_reply_t;

pub fn xcb_selinux_list_selections_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
///
pub fn xcb_selinux_list_selections (c : *mut ffi::base::xcb_connection_t) -> xcb_selinux_list_selections_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_selinux_list_selections_unchecked (c : *mut ffi::base::xcb_connection_t) -> xcb_selinux_list_selections_cookie_t;


pub fn xcb_selinux_list_selections_selections_length (R : *mut xcb_selinux_list_selections_reply_t) -> c_int;

pub fn xcb_selinux_list_selections_selections_iterator (R : *mut xcb_selinux_list_selections_reply_t) -> xcb_selinux_list_item_iterator_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_selinux_list_selections_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_selinux_list_selections_reply (c : *mut ffi::base::xcb_connection_t,
                                             cookie : xcb_selinux_list_selections_cookie_t,
                                             e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_selinux_list_selections_reply_t;

pub fn xcb_selinux_get_client_context_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
///
pub fn xcb_selinux_get_client_context (c : *mut ffi::base::xcb_connection_t,
                                          resource :  u32) -> xcb_selinux_get_client_context_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_selinux_get_client_context_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                    resource :  u32) -> xcb_selinux_get_client_context_cookie_t;

pub fn xcb_selinux_get_client_context_context (R : *mut xcb_selinux_get_client_context_reply_t) -> *mut c_char;


pub fn xcb_selinux_get_client_context_context_length (R : *mut xcb_selinux_get_client_context_reply_t) -> c_int;


pub fn xcb_selinux_get_client_context_context_end (R : *mut xcb_selinux_get_client_context_reply_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_selinux_get_client_context_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_selinux_get_client_context_reply (c : *mut ffi::base::xcb_connection_t,
                                                cookie : xcb_selinux_get_client_context_cookie_t,
                                                e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_selinux_get_client_context_reply_t;
}

