/*
 * This file generated automatically from xselinux.xml by r_client.py.
 * Edit at your peril.
 */

//Make the compiler quiet
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use std;
use libc::*;
use ffi;
use ffi::xproto;

pub static SELINUX_MAJOR_VERSION : c_uint = 1;
pub static SELINUX_MINOR_VERSION : c_uint = 0;

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
     pub client_major :   u8,
     pub client_minor :   u8
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
     pub server_major :    u16,
     pub server_minor :    u16
}

impl Copy for query_version_reply {}
impl Clone for query_version_reply {
    fn clone(&self) -> query_version_reply { *self }
}


#[repr(C)]
pub struct set_device_create_context_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub context_len :    u32
}

impl Copy for set_device_create_context_request {}
impl Clone for set_device_create_context_request {
    fn clone(&self) -> set_device_create_context_request { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct get_device_create_context_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct get_device_create_context_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16
}

impl Copy for get_device_create_context_request {}
impl Clone for get_device_create_context_request {
    fn clone(&self) -> get_device_create_context_request { *self }
}

#[repr(C)]
pub struct get_device_create_context_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub context_len :     u32,
     pub pad1 :            [u8; 20]
}

impl Copy for get_device_create_context_reply {}
impl Clone for get_device_create_context_reply {
    fn clone(&self) -> get_device_create_context_reply { *self }
}


#[repr(C)]
pub struct set_device_context_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub device :         u32,
     pub context_len :    u32
}

impl Copy for set_device_context_request {}
impl Clone for set_device_context_request {
    fn clone(&self) -> set_device_context_request { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct get_device_context_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct get_device_context_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub device :         u32
}

impl Copy for get_device_context_request {}
impl Clone for get_device_context_request {
    fn clone(&self) -> get_device_context_request { *self }
}

#[repr(C)]
pub struct get_device_context_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub context_len :     u32,
     pub pad1 :            [u8; 20]
}

impl Copy for get_device_context_reply {}
impl Clone for get_device_context_reply {
    fn clone(&self) -> get_device_context_reply { *self }
}


#[repr(C)]
pub struct set_window_create_context_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub context_len :    u32
}

impl Copy for set_window_create_context_request {}
impl Clone for set_window_create_context_request {
    fn clone(&self) -> set_window_create_context_request { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct get_window_create_context_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct get_window_create_context_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16
}

impl Copy for get_window_create_context_request {}
impl Clone for get_window_create_context_request {
    fn clone(&self) -> get_window_create_context_request { *self }
}

#[repr(C)]
pub struct get_window_create_context_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub context_len :     u32,
     pub pad1 :            [u8; 20]
}

impl Copy for get_window_create_context_reply {}
impl Clone for get_window_create_context_reply {
    fn clone(&self) -> get_window_create_context_reply { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct get_window_context_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct get_window_context_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub window :         ffi::xproto::window
}

impl Copy for get_window_context_request {}
impl Clone for get_window_context_request {
    fn clone(&self) -> get_window_context_request { *self }
}

#[repr(C)]
pub struct get_window_context_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub context_len :     u32,
     pub pad1 :            [u8; 20]
}

impl Copy for get_window_context_reply {}
impl Clone for get_window_context_reply {
    fn clone(&self) -> get_window_context_reply { *self }
}

#[repr(C)]
pub struct list_item {
     pub name :                 ffi::xproto::atom,
     pub object_context_len :   u32,
     pub data_context_len :     u32
}

impl Copy for list_item {}
impl Clone for list_item {
    fn clone(&self) -> list_item { *self }
}
/**
 * @brief list_item_iterator
 **/
#[repr(C)]
pub struct list_item_iterator {
    pub data : *mut list_item,
    pub rem  : c_int,
    pub index: c_int
}



#[repr(C)]
pub struct set_property_create_context_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub context_len :    u32
}

impl Copy for set_property_create_context_request {}
impl Clone for set_property_create_context_request {
    fn clone(&self) -> set_property_create_context_request { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct get_property_create_context_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct get_property_create_context_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16
}

impl Copy for get_property_create_context_request {}
impl Clone for get_property_create_context_request {
    fn clone(&self) -> get_property_create_context_request { *self }
}

#[repr(C)]
pub struct get_property_create_context_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub context_len :     u32,
     pub pad1 :            [u8; 20]
}

impl Copy for get_property_create_context_reply {}
impl Clone for get_property_create_context_reply {
    fn clone(&self) -> get_property_create_context_reply { *self }
}


#[repr(C)]
pub struct set_property_use_context_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub context_len :    u32
}

impl Copy for set_property_use_context_request {}
impl Clone for set_property_use_context_request {
    fn clone(&self) -> set_property_use_context_request { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct get_property_use_context_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct get_property_use_context_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16
}

impl Copy for get_property_use_context_request {}
impl Clone for get_property_use_context_request {
    fn clone(&self) -> get_property_use_context_request { *self }
}

#[repr(C)]
pub struct get_property_use_context_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub context_len :     u32,
     pub pad1 :            [u8; 20]
}

impl Copy for get_property_use_context_reply {}
impl Clone for get_property_use_context_reply {
    fn clone(&self) -> get_property_use_context_reply { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct get_property_context_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct get_property_context_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub window :         ffi::xproto::window,
     pub property :       ffi::xproto::atom
}

impl Copy for get_property_context_request {}
impl Clone for get_property_context_request {
    fn clone(&self) -> get_property_context_request { *self }
}

#[repr(C)]
pub struct get_property_context_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub context_len :     u32,
     pub pad1 :            [u8; 20]
}

impl Copy for get_property_context_reply {}
impl Clone for get_property_context_reply {
    fn clone(&self) -> get_property_context_reply { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct get_property_data_context_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct get_property_data_context_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub window :         ffi::xproto::window,
     pub property :       ffi::xproto::atom
}

impl Copy for get_property_data_context_request {}
impl Clone for get_property_data_context_request {
    fn clone(&self) -> get_property_data_context_request { *self }
}

#[repr(C)]
pub struct get_property_data_context_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub context_len :     u32,
     pub pad1 :            [u8; 20]
}

impl Copy for get_property_data_context_reply {}
impl Clone for get_property_data_context_reply {
    fn clone(&self) -> get_property_data_context_reply { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct list_properties_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct list_properties_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub window :         ffi::xproto::window
}

impl Copy for list_properties_request {}
impl Clone for list_properties_request {
    fn clone(&self) -> list_properties_request { *self }
}

#[repr(C)]
pub struct list_properties_reply {
     pub response_type :    u8,
     pub pad0 :             u8,
     pub sequence :         u16,
     pub length :           u32,
     pub properties_len :   u32,
     pub pad1 :             [u8; 20]
}

impl Copy for list_properties_reply {}
impl Clone for list_properties_reply {
    fn clone(&self) -> list_properties_reply { *self }
}


#[repr(C)]
pub struct set_selection_create_context_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub context_len :    u32
}

impl Copy for set_selection_create_context_request {}
impl Clone for set_selection_create_context_request {
    fn clone(&self) -> set_selection_create_context_request { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct get_selection_create_context_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct get_selection_create_context_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16
}

impl Copy for get_selection_create_context_request {}
impl Clone for get_selection_create_context_request {
    fn clone(&self) -> get_selection_create_context_request { *self }
}

#[repr(C)]
pub struct get_selection_create_context_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub context_len :     u32,
     pub pad1 :            [u8; 20]
}

impl Copy for get_selection_create_context_reply {}
impl Clone for get_selection_create_context_reply {
    fn clone(&self) -> get_selection_create_context_reply { *self }
}


#[repr(C)]
pub struct set_selection_use_context_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub context_len :    u32
}

impl Copy for set_selection_use_context_request {}
impl Clone for set_selection_use_context_request {
    fn clone(&self) -> set_selection_use_context_request { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct get_selection_use_context_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct get_selection_use_context_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16
}

impl Copy for get_selection_use_context_request {}
impl Clone for get_selection_use_context_request {
    fn clone(&self) -> get_selection_use_context_request { *self }
}

#[repr(C)]
pub struct get_selection_use_context_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub context_len :     u32,
     pub pad1 :            [u8; 20]
}

impl Copy for get_selection_use_context_reply {}
impl Clone for get_selection_use_context_reply {
    fn clone(&self) -> get_selection_use_context_reply { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct get_selection_context_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct get_selection_context_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub selection :      ffi::xproto::atom
}

impl Copy for get_selection_context_request {}
impl Clone for get_selection_context_request {
    fn clone(&self) -> get_selection_context_request { *self }
}

#[repr(C)]
pub struct get_selection_context_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub context_len :     u32,
     pub pad1 :            [u8; 20]
}

impl Copy for get_selection_context_reply {}
impl Clone for get_selection_context_reply {
    fn clone(&self) -> get_selection_context_reply { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct get_selection_data_context_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct get_selection_data_context_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub selection :      ffi::xproto::atom
}

impl Copy for get_selection_data_context_request {}
impl Clone for get_selection_data_context_request {
    fn clone(&self) -> get_selection_data_context_request { *self }
}

#[repr(C)]
pub struct get_selection_data_context_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub context_len :     u32,
     pub pad1 :            [u8; 20]
}

impl Copy for get_selection_data_context_reply {}
impl Clone for get_selection_data_context_reply {
    fn clone(&self) -> get_selection_data_context_reply { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct list_selections_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct list_selections_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16
}

impl Copy for list_selections_request {}
impl Clone for list_selections_request {
    fn clone(&self) -> list_selections_request { *self }
}

#[repr(C)]
pub struct list_selections_reply {
     pub response_type :    u8,
     pub pad0 :             u8,
     pub sequence :         u16,
     pub length :           u32,
     pub selections_len :   u32,
     pub pad1 :             [u8; 20]
}

impl Copy for list_selections_reply {}
impl Clone for list_selections_reply {
    fn clone(&self) -> list_selections_reply { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct get_client_context_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct get_client_context_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub resource :       u32
}

impl Copy for get_client_context_request {}
impl Clone for get_client_context_request {
    fn clone(&self) -> get_client_context_request { *self }
}

#[repr(C)]
pub struct get_client_context_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub context_len :     u32,
     pub pad1 :            [u8; 20]
}

impl Copy for get_client_context_reply {}
impl Clone for get_client_context_reply {
    fn clone(&self) -> get_client_context_reply { *self }
}
#[link(name="xcb-xselinux")]
extern "C" {

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_selinux_query_version (c : *mut ffi::base::connection,
                                     client_major :  u8,
                                     client_minor :  u8) -> query_version_cookie;

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
pub fn xcb_selinux_query_version_unchecked (c : *mut ffi::base::connection,
                                               client_major :  u8,
                                               client_minor :  u8) -> query_version_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_selinux_query_version_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_selinux_query_version_reply (c : *mut ffi::base::connection,
                                           cookie : query_version_cookie,
                                           e : *mut *mut ffi::base::generic_error) -> *mut query_version_reply;

pub fn xcb_selinux_set_device_create_context_sizeof (_buffer :  *mut c_void) -> c_int;

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
pub fn xcb_selinux_set_device_create_context_checked (c : *mut ffi::base::connection,
                                                         context_len :  u32,
                                                         context : *mut c_char) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_selinux_set_device_create_context (c : *mut ffi::base::connection,
                                                 context_len :  u32,
                                                 context : *mut c_char) -> ffi::base::void_cookie;

pub fn xcb_selinux_get_device_create_context_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_selinux_get_device_create_context (c : *mut ffi::base::connection) -> get_device_create_context_cookie;

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
pub fn xcb_selinux_get_device_create_context_unchecked (c : *mut ffi::base::connection) -> get_device_create_context_cookie;

pub fn xcb_selinux_get_device_create_context_context (R : *mut get_device_create_context_reply) -> *mut c_char;


pub fn xcb_selinux_get_device_create_context_context_length (R : *mut get_device_create_context_reply) -> c_int;


pub fn xcb_selinux_get_device_create_context_context_end (R : *mut get_device_create_context_reply) -> ffi::base::generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_selinux_get_device_create_context_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_selinux_get_device_create_context_reply (c : *mut ffi::base::connection,
                                                       cookie : get_device_create_context_cookie,
                                                       e : *mut *mut ffi::base::generic_error) -> *mut get_device_create_context_reply;

pub fn xcb_selinux_set_device_context_sizeof (_buffer :  *mut c_void) -> c_int;

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
pub fn xcb_selinux_set_device_context_checked (c : *mut ffi::base::connection,
                                                  device :  u32,
                                                  context_len :  u32,
                                                  context : *mut c_char) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_selinux_set_device_context (c : *mut ffi::base::connection,
                                          device :  u32,
                                          context_len :  u32,
                                          context : *mut c_char) -> ffi::base::void_cookie;

pub fn xcb_selinux_get_device_context_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_selinux_get_device_context (c : *mut ffi::base::connection,
                                          device :  u32) -> get_device_context_cookie;

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
pub fn xcb_selinux_get_device_context_unchecked (c : *mut ffi::base::connection,
                                                    device :  u32) -> get_device_context_cookie;

pub fn xcb_selinux_get_device_context_context (R : *mut get_device_context_reply) -> *mut c_char;


pub fn xcb_selinux_get_device_context_context_length (R : *mut get_device_context_reply) -> c_int;


pub fn xcb_selinux_get_device_context_context_end (R : *mut get_device_context_reply) -> ffi::base::generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_selinux_get_device_context_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_selinux_get_device_context_reply (c : *mut ffi::base::connection,
                                                cookie : get_device_context_cookie,
                                                e : *mut *mut ffi::base::generic_error) -> *mut get_device_context_reply;

pub fn xcb_selinux_set_window_create_context_sizeof (_buffer :  *mut c_void) -> c_int;

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
pub fn xcb_selinux_set_window_create_context_checked (c : *mut ffi::base::connection,
                                                         context_len :  u32,
                                                         context : *mut c_char) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_selinux_set_window_create_context (c : *mut ffi::base::connection,
                                                 context_len :  u32,
                                                 context : *mut c_char) -> ffi::base::void_cookie;

pub fn xcb_selinux_get_window_create_context_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_selinux_get_window_create_context (c : *mut ffi::base::connection) -> get_window_create_context_cookie;

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
pub fn xcb_selinux_get_window_create_context_unchecked (c : *mut ffi::base::connection) -> get_window_create_context_cookie;

pub fn xcb_selinux_get_window_create_context_context (R : *mut get_window_create_context_reply) -> *mut c_char;


pub fn xcb_selinux_get_window_create_context_context_length (R : *mut get_window_create_context_reply) -> c_int;


pub fn xcb_selinux_get_window_create_context_context_end (R : *mut get_window_create_context_reply) -> ffi::base::generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_selinux_get_window_create_context_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_selinux_get_window_create_context_reply (c : *mut ffi::base::connection,
                                                       cookie : get_window_create_context_cookie,
                                                       e : *mut *mut ffi::base::generic_error) -> *mut get_window_create_context_reply;

pub fn xcb_selinux_get_window_context_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_selinux_get_window_context (c : *mut ffi::base::connection,
                                          window :  ffi::xproto::window) -> get_window_context_cookie;

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
pub fn xcb_selinux_get_window_context_unchecked (c : *mut ffi::base::connection,
                                                    window :  ffi::xproto::window) -> get_window_context_cookie;

pub fn xcb_selinux_get_window_context_context (R : *mut get_window_context_reply) -> *mut c_char;


pub fn xcb_selinux_get_window_context_context_length (R : *mut get_window_context_reply) -> c_int;


pub fn xcb_selinux_get_window_context_context_end (R : *mut get_window_context_reply) -> ffi::base::generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_selinux_get_window_context_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_selinux_get_window_context_reply (c : *mut ffi::base::connection,
                                                cookie : get_window_context_cookie,
                                                e : *mut *mut ffi::base::generic_error) -> *mut get_window_context_reply;

pub fn xcb_selinux_list_item_sizeof (_buffer :  *mut c_void) -> c_int;

pub fn xcb_selinux_list_item_object_context (R : *mut list_item) -> *mut c_char;


pub fn xcb_selinux_list_item_object_context_length (R : *mut list_item) -> c_int;


pub fn xcb_selinux_list_item_object_context_end (R : *mut list_item) -> ffi::base::generic_iterator;

pub fn xcb_selinux_list_item_data_context (R : *mut list_item) -> *mut c_char;


pub fn xcb_selinux_list_item_data_context_length (R : *mut list_item) -> c_int;


pub fn xcb_selinux_list_item_data_context_end (R : *mut list_item) -> ffi::base::generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a list_item_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(list_item)
 *
 *
 */
pub fn xcb_selinux_list_item_next (i:*mut list_item_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An list_item_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_selinux_list_item_end (i:list_item_iterator) -> ffi::base::generic_iterator;

pub fn xcb_selinux_set_property_create_context_sizeof (_buffer :  *mut c_void) -> c_int;

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
pub fn xcb_selinux_set_property_create_context_checked (c : *mut ffi::base::connection,
                                                           context_len :  u32,
                                                           context : *mut c_char) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_selinux_set_property_create_context (c : *mut ffi::base::connection,
                                                   context_len :  u32,
                                                   context : *mut c_char) -> ffi::base::void_cookie;

pub fn xcb_selinux_get_property_create_context_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_selinux_get_property_create_context (c : *mut ffi::base::connection) -> get_property_create_context_cookie;

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
pub fn xcb_selinux_get_property_create_context_unchecked (c : *mut ffi::base::connection) -> get_property_create_context_cookie;

pub fn xcb_selinux_get_property_create_context_context (R : *mut get_property_create_context_reply) -> *mut c_char;


pub fn xcb_selinux_get_property_create_context_context_length (R : *mut get_property_create_context_reply) -> c_int;


pub fn xcb_selinux_get_property_create_context_context_end (R : *mut get_property_create_context_reply) -> ffi::base::generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_selinux_get_property_create_context_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_selinux_get_property_create_context_reply (c : *mut ffi::base::connection,
                                                         cookie : get_property_create_context_cookie,
                                                         e : *mut *mut ffi::base::generic_error) -> *mut get_property_create_context_reply;

pub fn xcb_selinux_set_property_use_context_sizeof (_buffer :  *mut c_void) -> c_int;

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
pub fn xcb_selinux_set_property_use_context_checked (c : *mut ffi::base::connection,
                                                        context_len :  u32,
                                                        context : *mut c_char) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_selinux_set_property_use_context (c : *mut ffi::base::connection,
                                                context_len :  u32,
                                                context : *mut c_char) -> ffi::base::void_cookie;

pub fn xcb_selinux_get_property_use_context_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_selinux_get_property_use_context (c : *mut ffi::base::connection) -> get_property_use_context_cookie;

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
pub fn xcb_selinux_get_property_use_context_unchecked (c : *mut ffi::base::connection) -> get_property_use_context_cookie;

pub fn xcb_selinux_get_property_use_context_context (R : *mut get_property_use_context_reply) -> *mut c_char;


pub fn xcb_selinux_get_property_use_context_context_length (R : *mut get_property_use_context_reply) -> c_int;


pub fn xcb_selinux_get_property_use_context_context_end (R : *mut get_property_use_context_reply) -> ffi::base::generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_selinux_get_property_use_context_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_selinux_get_property_use_context_reply (c : *mut ffi::base::connection,
                                                      cookie : get_property_use_context_cookie,
                                                      e : *mut *mut ffi::base::generic_error) -> *mut get_property_use_context_reply;

pub fn xcb_selinux_get_property_context_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_selinux_get_property_context (c : *mut ffi::base::connection,
                                            window :  ffi::xproto::window,
                                            property :  ffi::xproto::atom) -> get_property_context_cookie;

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
pub fn xcb_selinux_get_property_context_unchecked (c : *mut ffi::base::connection,
                                                      window :  ffi::xproto::window,
                                                      property :  ffi::xproto::atom) -> get_property_context_cookie;

pub fn xcb_selinux_get_property_context_context (R : *mut get_property_context_reply) -> *mut c_char;


pub fn xcb_selinux_get_property_context_context_length (R : *mut get_property_context_reply) -> c_int;


pub fn xcb_selinux_get_property_context_context_end (R : *mut get_property_context_reply) -> ffi::base::generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_selinux_get_property_context_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_selinux_get_property_context_reply (c : *mut ffi::base::connection,
                                                  cookie : get_property_context_cookie,
                                                  e : *mut *mut ffi::base::generic_error) -> *mut get_property_context_reply;

pub fn xcb_selinux_get_property_data_context_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_selinux_get_property_data_context (c : *mut ffi::base::connection,
                                                 window :  ffi::xproto::window,
                                                 property :  ffi::xproto::atom) -> get_property_data_context_cookie;

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
pub fn xcb_selinux_get_property_data_context_unchecked (c : *mut ffi::base::connection,
                                                           window :  ffi::xproto::window,
                                                           property :  ffi::xproto::atom) -> get_property_data_context_cookie;

pub fn xcb_selinux_get_property_data_context_context (R : *mut get_property_data_context_reply) -> *mut c_char;


pub fn xcb_selinux_get_property_data_context_context_length (R : *mut get_property_data_context_reply) -> c_int;


pub fn xcb_selinux_get_property_data_context_context_end (R : *mut get_property_data_context_reply) -> ffi::base::generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_selinux_get_property_data_context_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_selinux_get_property_data_context_reply (c : *mut ffi::base::connection,
                                                       cookie : get_property_data_context_cookie,
                                                       e : *mut *mut ffi::base::generic_error) -> *mut get_property_data_context_reply;

pub fn xcb_selinux_list_properties_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_selinux_list_properties (c : *mut ffi::base::connection,
                                       window :  ffi::xproto::window) -> list_properties_cookie;

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
pub fn xcb_selinux_list_properties_unchecked (c : *mut ffi::base::connection,
                                                 window :  ffi::xproto::window) -> list_properties_cookie;


pub fn xcb_selinux_list_properties_properties_length (R : *mut list_properties_reply) -> c_int;

pub fn xcb_selinux_list_properties_properties_iterator (R : *mut list_properties_reply) -> list_item_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_selinux_list_properties_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_selinux_list_properties_reply (c : *mut ffi::base::connection,
                                             cookie : list_properties_cookie,
                                             e : *mut *mut ffi::base::generic_error) -> *mut list_properties_reply;

pub fn xcb_selinux_set_selection_create_context_sizeof (_buffer :  *mut c_void) -> c_int;

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
pub fn xcb_selinux_set_selection_create_context_checked (c : *mut ffi::base::connection,
                                                            context_len :  u32,
                                                            context : *mut c_char) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_selinux_set_selection_create_context (c : *mut ffi::base::connection,
                                                    context_len :  u32,
                                                    context : *mut c_char) -> ffi::base::void_cookie;

pub fn xcb_selinux_get_selection_create_context_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_selinux_get_selection_create_context (c : *mut ffi::base::connection) -> get_selection_create_context_cookie;

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
pub fn xcb_selinux_get_selection_create_context_unchecked (c : *mut ffi::base::connection) -> get_selection_create_context_cookie;

pub fn xcb_selinux_get_selection_create_context_context (R : *mut get_selection_create_context_reply) -> *mut c_char;


pub fn xcb_selinux_get_selection_create_context_context_length (R : *mut get_selection_create_context_reply) -> c_int;


pub fn xcb_selinux_get_selection_create_context_context_end (R : *mut get_selection_create_context_reply) -> ffi::base::generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_selinux_get_selection_create_context_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_selinux_get_selection_create_context_reply (c : *mut ffi::base::connection,
                                                          cookie : get_selection_create_context_cookie,
                                                          e : *mut *mut ffi::base::generic_error) -> *mut get_selection_create_context_reply;

pub fn xcb_selinux_set_selection_use_context_sizeof (_buffer :  *mut c_void) -> c_int;

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
pub fn xcb_selinux_set_selection_use_context_checked (c : *mut ffi::base::connection,
                                                         context_len :  u32,
                                                         context : *mut c_char) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_selinux_set_selection_use_context (c : *mut ffi::base::connection,
                                                 context_len :  u32,
                                                 context : *mut c_char) -> ffi::base::void_cookie;

pub fn xcb_selinux_get_selection_use_context_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_selinux_get_selection_use_context (c : *mut ffi::base::connection) -> get_selection_use_context_cookie;

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
pub fn xcb_selinux_get_selection_use_context_unchecked (c : *mut ffi::base::connection) -> get_selection_use_context_cookie;

pub fn xcb_selinux_get_selection_use_context_context (R : *mut get_selection_use_context_reply) -> *mut c_char;


pub fn xcb_selinux_get_selection_use_context_context_length (R : *mut get_selection_use_context_reply) -> c_int;


pub fn xcb_selinux_get_selection_use_context_context_end (R : *mut get_selection_use_context_reply) -> ffi::base::generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_selinux_get_selection_use_context_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_selinux_get_selection_use_context_reply (c : *mut ffi::base::connection,
                                                       cookie : get_selection_use_context_cookie,
                                                       e : *mut *mut ffi::base::generic_error) -> *mut get_selection_use_context_reply;

pub fn xcb_selinux_get_selection_context_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_selinux_get_selection_context (c : *mut ffi::base::connection,
                                             selection :  ffi::xproto::atom) -> get_selection_context_cookie;

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
pub fn xcb_selinux_get_selection_context_unchecked (c : *mut ffi::base::connection,
                                                       selection :  ffi::xproto::atom) -> get_selection_context_cookie;

pub fn xcb_selinux_get_selection_context_context (R : *mut get_selection_context_reply) -> *mut c_char;


pub fn xcb_selinux_get_selection_context_context_length (R : *mut get_selection_context_reply) -> c_int;


pub fn xcb_selinux_get_selection_context_context_end (R : *mut get_selection_context_reply) -> ffi::base::generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_selinux_get_selection_context_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_selinux_get_selection_context_reply (c : *mut ffi::base::connection,
                                                   cookie : get_selection_context_cookie,
                                                   e : *mut *mut ffi::base::generic_error) -> *mut get_selection_context_reply;

pub fn xcb_selinux_get_selection_data_context_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_selinux_get_selection_data_context (c : *mut ffi::base::connection,
                                                  selection :  ffi::xproto::atom) -> get_selection_data_context_cookie;

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
pub fn xcb_selinux_get_selection_data_context_unchecked (c : *mut ffi::base::connection,
                                                            selection :  ffi::xproto::atom) -> get_selection_data_context_cookie;

pub fn xcb_selinux_get_selection_data_context_context (R : *mut get_selection_data_context_reply) -> *mut c_char;


pub fn xcb_selinux_get_selection_data_context_context_length (R : *mut get_selection_data_context_reply) -> c_int;


pub fn xcb_selinux_get_selection_data_context_context_end (R : *mut get_selection_data_context_reply) -> ffi::base::generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_selinux_get_selection_data_context_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_selinux_get_selection_data_context_reply (c : *mut ffi::base::connection,
                                                        cookie : get_selection_data_context_cookie,
                                                        e : *mut *mut ffi::base::generic_error) -> *mut get_selection_data_context_reply;

pub fn xcb_selinux_list_selections_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_selinux_list_selections (c : *mut ffi::base::connection) -> list_selections_cookie;

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
pub fn xcb_selinux_list_selections_unchecked (c : *mut ffi::base::connection) -> list_selections_cookie;


pub fn xcb_selinux_list_selections_selections_length (R : *mut list_selections_reply) -> c_int;

pub fn xcb_selinux_list_selections_selections_iterator (R : *mut list_selections_reply) -> list_item_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_selinux_list_selections_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_selinux_list_selections_reply (c : *mut ffi::base::connection,
                                             cookie : list_selections_cookie,
                                             e : *mut *mut ffi::base::generic_error) -> *mut list_selections_reply;

pub fn xcb_selinux_get_client_context_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_selinux_get_client_context (c : *mut ffi::base::connection,
                                          resource :  u32) -> get_client_context_cookie;

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
pub fn xcb_selinux_get_client_context_unchecked (c : *mut ffi::base::connection,
                                                    resource :  u32) -> get_client_context_cookie;

pub fn xcb_selinux_get_client_context_context (R : *mut get_client_context_reply) -> *mut c_char;


pub fn xcb_selinux_get_client_context_context_length (R : *mut get_client_context_reply) -> c_int;


pub fn xcb_selinux_get_client_context_context_end (R : *mut get_client_context_reply) -> ffi::base::generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_selinux_get_client_context_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_selinux_get_client_context_reply (c : *mut ffi::base::connection,
                                                cookie : get_client_context_cookie,
                                                e : *mut *mut ffi::base::generic_error) -> *mut get_client_context_reply;
}

