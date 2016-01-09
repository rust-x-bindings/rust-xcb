/*
 * This file generated automatically from dri2.xml by r_client.py.
 * Edit at your peril.
 */

//Make the compiler quiet
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use std;
use libc::*;
use ffi;
use ffi::xproto;

pub static DRI2_MAJOR_VERSION : c_uint = 1;
pub static DRI2_MINOR_VERSION : c_uint = 4;

#[repr(C)]
pub struct xcb_dri2_dri2_buffer_t {
     pub attachment :   u32,
     pub name :         u32,
     pub pitch :        u32,
     pub cpp :          u32,
     pub flags :        u32
}

impl Copy for xcb_dri2_dri2_buffer_t {}
impl Clone for xcb_dri2_dri2_buffer_t {
    fn clone(&self) -> xcb_dri2_dri2_buffer_t { *self }
}
/**
 * @brief xcb_dri2_dri2_buffer_iterator_t
 **/
#[repr(C)]
pub struct xcb_dri2_dri2_buffer_iterator_t {
    pub data : *mut xcb_dri2_dri2_buffer_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_dri2_attach_format_t {
     pub attachment :   u32,
     pub format :       u32
}

impl Copy for xcb_dri2_attach_format_t {}
impl Clone for xcb_dri2_attach_format_t {
    fn clone(&self) -> xcb_dri2_attach_format_t { *self }
}
/**
 * @brief xcb_dri2_attach_format_iterator_t
 **/
#[repr(C)]
pub struct xcb_dri2_attach_format_iterator_t {
    pub data : *mut xcb_dri2_attach_format_t,
    pub rem  : c_int,
    pub index: c_int
}


#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_dri2_query_version_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_dri2_query_version_request_t {
     pub major_opcode :    u8,
     pub minor_opcode :    u8,
     pub length :          u16,
     pub major_version :   u32,
     pub minor_version :   u32
}

impl Copy for xcb_dri2_query_version_request_t {}
impl Clone for xcb_dri2_query_version_request_t {
    fn clone(&self) -> xcb_dri2_query_version_request_t { *self }
}

#[repr(C)]
pub struct xcb_dri2_query_version_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub major_version :   u32,
     pub minor_version :   u32
}

impl Copy for xcb_dri2_query_version_reply_t {}
impl Clone for xcb_dri2_query_version_reply_t {
    fn clone(&self) -> xcb_dri2_query_version_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_dri2_connect_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_dri2_connect_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub window :         ffi::xproto::xcb_window_t,
     pub driver_type :    u32
}

impl Copy for xcb_dri2_connect_request_t {}
impl Clone for xcb_dri2_connect_request_t {
    fn clone(&self) -> xcb_dri2_connect_request_t { *self }
}

#[repr(C)]
pub struct xcb_dri2_connect_reply_t {
     pub response_type :        u8,
     pub pad0 :                 u8,
     pub sequence :             u16,
     pub length :               u32,
     pub driver_name_length :   u32,
     pub device_name_length :   u32,
     pub pad1 :                 [u8; 16]
}

impl Copy for xcb_dri2_connect_reply_t {}
impl Clone for xcb_dri2_connect_reply_t {
    fn clone(&self) -> xcb_dri2_connect_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_dri2_authenticate_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_dri2_authenticate_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub window :         ffi::xproto::xcb_window_t,
     pub magic :          u32
}

impl Copy for xcb_dri2_authenticate_request_t {}
impl Clone for xcb_dri2_authenticate_request_t {
    fn clone(&self) -> xcb_dri2_authenticate_request_t { *self }
}

#[repr(C)]
pub struct xcb_dri2_authenticate_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub authenticated :   u32
}

impl Copy for xcb_dri2_authenticate_reply_t {}
impl Clone for xcb_dri2_authenticate_reply_t {
    fn clone(&self) -> xcb_dri2_authenticate_reply_t { *self }
}


#[repr(C)]
pub struct xcb_dri2_create_drawable_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub drawable :       ffi::xproto::xcb_drawable_t
}

impl Copy for xcb_dri2_create_drawable_request_t {}
impl Clone for xcb_dri2_create_drawable_request_t {
    fn clone(&self) -> xcb_dri2_create_drawable_request_t { *self }
}


#[repr(C)]
pub struct xcb_dri2_destroy_drawable_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub drawable :       ffi::xproto::xcb_drawable_t
}

impl Copy for xcb_dri2_destroy_drawable_request_t {}
impl Clone for xcb_dri2_destroy_drawable_request_t {
    fn clone(&self) -> xcb_dri2_destroy_drawable_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_dri2_get_buffers_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_dri2_get_buffers_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub drawable :       ffi::xproto::xcb_drawable_t,
     pub count :          u32
}

impl Copy for xcb_dri2_get_buffers_request_t {}
impl Clone for xcb_dri2_get_buffers_request_t {
    fn clone(&self) -> xcb_dri2_get_buffers_request_t { *self }
}

#[repr(C)]
pub struct xcb_dri2_get_buffers_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub width :           u32,
     pub height :          u32,
     pub count :           u32,
     pub pad1 :            [u8; 12]
}

impl Copy for xcb_dri2_get_buffers_reply_t {}
impl Clone for xcb_dri2_get_buffers_reply_t {
    fn clone(&self) -> xcb_dri2_get_buffers_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_dri2_copy_region_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_dri2_copy_region_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub drawable :       ffi::xproto::xcb_drawable_t,
     pub region :         u32,
     pub dest :           u32,
     pub src :            u32
}

impl Copy for xcb_dri2_copy_region_request_t {}
impl Clone for xcb_dri2_copy_region_request_t {
    fn clone(&self) -> xcb_dri2_copy_region_request_t { *self }
}

#[repr(C)]
pub struct xcb_dri2_copy_region_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32
}

impl Copy for xcb_dri2_copy_region_reply_t {}
impl Clone for xcb_dri2_copy_region_reply_t {
    fn clone(&self) -> xcb_dri2_copy_region_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_dri2_get_buffers_with_format_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_dri2_get_buffers_with_format_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub drawable :       ffi::xproto::xcb_drawable_t,
     pub count :          u32
}

impl Copy for xcb_dri2_get_buffers_with_format_request_t {}
impl Clone for xcb_dri2_get_buffers_with_format_request_t {
    fn clone(&self) -> xcb_dri2_get_buffers_with_format_request_t { *self }
}

#[repr(C)]
pub struct xcb_dri2_get_buffers_with_format_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub width :           u32,
     pub height :          u32,
     pub count :           u32,
     pub pad1 :            [u8; 12]
}

impl Copy for xcb_dri2_get_buffers_with_format_reply_t {}
impl Clone for xcb_dri2_get_buffers_with_format_reply_t {
    fn clone(&self) -> xcb_dri2_get_buffers_with_format_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_dri2_swap_buffers_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_dri2_swap_buffers_request_t {
     pub major_opcode :    u8,
     pub minor_opcode :    u8,
     pub length :          u16,
     pub drawable :        ffi::xproto::xcb_drawable_t,
     pub target_msc_hi :   u32,
     pub target_msc_lo :   u32,
     pub divisor_hi :      u32,
     pub divisor_lo :      u32,
     pub remainder_hi :    u32,
     pub remainder_lo :    u32
}

impl Copy for xcb_dri2_swap_buffers_request_t {}
impl Clone for xcb_dri2_swap_buffers_request_t {
    fn clone(&self) -> xcb_dri2_swap_buffers_request_t { *self }
}

#[repr(C)]
pub struct xcb_dri2_swap_buffers_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub swap_hi :         u32,
     pub swap_lo :         u32
}

impl Copy for xcb_dri2_swap_buffers_reply_t {}
impl Clone for xcb_dri2_swap_buffers_reply_t {
    fn clone(&self) -> xcb_dri2_swap_buffers_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_dri2_get_msc_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_dri2_get_msc_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub drawable :       ffi::xproto::xcb_drawable_t
}

impl Copy for xcb_dri2_get_msc_request_t {}
impl Clone for xcb_dri2_get_msc_request_t {
    fn clone(&self) -> xcb_dri2_get_msc_request_t { *self }
}

#[repr(C)]
pub struct xcb_dri2_get_msc_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub ust_hi :          u32,
     pub ust_lo :          u32,
     pub msc_hi :          u32,
     pub msc_lo :          u32,
     pub sbc_hi :          u32,
     pub sbc_lo :          u32
}

impl Copy for xcb_dri2_get_msc_reply_t {}
impl Clone for xcb_dri2_get_msc_reply_t {
    fn clone(&self) -> xcb_dri2_get_msc_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_dri2_wait_msc_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_dri2_wait_msc_request_t {
     pub major_opcode :    u8,
     pub minor_opcode :    u8,
     pub length :          u16,
     pub drawable :        ffi::xproto::xcb_drawable_t,
     pub target_msc_hi :   u32,
     pub target_msc_lo :   u32,
     pub divisor_hi :      u32,
     pub divisor_lo :      u32,
     pub remainder_hi :    u32,
     pub remainder_lo :    u32
}

impl Copy for xcb_dri2_wait_msc_request_t {}
impl Clone for xcb_dri2_wait_msc_request_t {
    fn clone(&self) -> xcb_dri2_wait_msc_request_t { *self }
}

#[repr(C)]
pub struct xcb_dri2_wait_msc_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub ust_hi :          u32,
     pub ust_lo :          u32,
     pub msc_hi :          u32,
     pub msc_lo :          u32,
     pub sbc_hi :          u32,
     pub sbc_lo :          u32
}

impl Copy for xcb_dri2_wait_msc_reply_t {}
impl Clone for xcb_dri2_wait_msc_reply_t {
    fn clone(&self) -> xcb_dri2_wait_msc_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_dri2_wait_sbc_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_dri2_wait_sbc_request_t {
     pub major_opcode :    u8,
     pub minor_opcode :    u8,
     pub length :          u16,
     pub drawable :        ffi::xproto::xcb_drawable_t,
     pub target_sbc_hi :   u32,
     pub target_sbc_lo :   u32
}

impl Copy for xcb_dri2_wait_sbc_request_t {}
impl Clone for xcb_dri2_wait_sbc_request_t {
    fn clone(&self) -> xcb_dri2_wait_sbc_request_t { *self }
}

#[repr(C)]
pub struct xcb_dri2_wait_sbc_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub ust_hi :          u32,
     pub ust_lo :          u32,
     pub msc_hi :          u32,
     pub msc_lo :          u32,
     pub sbc_hi :          u32,
     pub sbc_lo :          u32
}

impl Copy for xcb_dri2_wait_sbc_reply_t {}
impl Clone for xcb_dri2_wait_sbc_reply_t {
    fn clone(&self) -> xcb_dri2_wait_sbc_reply_t { *self }
}


#[repr(C)]
pub struct xcb_dri2_swap_interval_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub drawable :       ffi::xproto::xcb_drawable_t,
     pub interval :       u32
}

impl Copy for xcb_dri2_swap_interval_request_t {}
impl Clone for xcb_dri2_swap_interval_request_t {
    fn clone(&self) -> xcb_dri2_swap_interval_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_dri2_get_param_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_dri2_get_param_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub drawable :       ffi::xproto::xcb_drawable_t,
     pub param :          u32
}

impl Copy for xcb_dri2_get_param_request_t {}
impl Clone for xcb_dri2_get_param_request_t {
    fn clone(&self) -> xcb_dri2_get_param_request_t { *self }
}

#[repr(C)]
pub struct xcb_dri2_get_param_reply_t {
     pub response_type :         u8,
     pub is_param_recognized :   u8,
     pub sequence :              u16,
     pub length :                u32,
     pub value_hi :              u32,
     pub value_lo :              u32
}

impl Copy for xcb_dri2_get_param_reply_t {}
impl Clone for xcb_dri2_get_param_reply_t {
    fn clone(&self) -> xcb_dri2_get_param_reply_t { *self }
}


#[repr(C)]
pub struct xcb_dri2_buffer_swap_complete_event_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub event_type :      u16,
     pub pad1 :            [u8; 2],
     pub drawable :        ffi::xproto::xcb_drawable_t,
     pub ust_hi :          u32,
     pub ust_lo :          u32,
     pub msc_hi :          u32,
     pub msc_lo :          u32,
     pub sbc :             u32
}

impl Copy for xcb_dri2_buffer_swap_complete_event_t {}
impl Clone for xcb_dri2_buffer_swap_complete_event_t {
    fn clone(&self) -> xcb_dri2_buffer_swap_complete_event_t { *self }
}


#[repr(C)]
pub struct xcb_dri2_invalidate_buffers_event_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub drawable :        ffi::xproto::xcb_drawable_t
}

impl Copy for xcb_dri2_invalidate_buffers_event_t {}
impl Clone for xcb_dri2_invalidate_buffers_event_t {
    fn clone(&self) -> xcb_dri2_invalidate_buffers_event_t { *self }
}
#[link(name="xcb-dri2")]
extern "C" {

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_dri2_dri2_buffer_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_dri2_dri2_buffer_t)
 *
 *
 */
pub fn xcb_dri2_dri2_buffer_next (i:*mut xcb_dri2_dri2_buffer_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_dri2_dri2_buffer_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_dri2_dri2_buffer_end (i:xcb_dri2_dri2_buffer_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_dri2_attach_format_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_dri2_attach_format_t)
 *
 *
 */
pub fn xcb_dri2_attach_format_next (i:*mut xcb_dri2_attach_format_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_dri2_attach_format_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_dri2_attach_format_end (i:xcb_dri2_attach_format_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_dri2_query_version (c : *mut ffi::base::xcb_connection_t,
                                  major_version :  u32,
                                  minor_version :  u32) -> xcb_dri2_query_version_cookie_t;

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
pub fn xcb_dri2_query_version_unchecked (c : *mut ffi::base::xcb_connection_t,
                                            major_version :  u32,
                                            minor_version :  u32) -> xcb_dri2_query_version_cookie_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_dri2_query_version_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_dri2_query_version_reply (c : *mut ffi::base::xcb_connection_t,
                                        cookie : xcb_dri2_query_version_cookie_t,
                                        e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_dri2_query_version_reply_t;

pub fn xcb_dri2_connect_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_dri2_connect (c : *mut ffi::base::xcb_connection_t,
                            window :  ffi::xproto::xcb_window_t,
                            driver_type :  u32) -> xcb_dri2_connect_cookie_t;

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
pub fn xcb_dri2_connect_unchecked (c : *mut ffi::base::xcb_connection_t,
                                      window :  ffi::xproto::xcb_window_t,
                                      driver_type :  u32) -> xcb_dri2_connect_cookie_t;

pub fn xcb_dri2_connect_driver_name (R : *mut xcb_dri2_connect_reply_t) -> *mut c_char;


pub fn xcb_dri2_connect_driver_name_length (R : *mut xcb_dri2_connect_reply_t) -> c_int;


pub fn xcb_dri2_connect_driver_name_end (R : *mut xcb_dri2_connect_reply_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_dri2_connect_alignment_pad (R : *mut xcb_dri2_connect_reply_t) -> *mut c_void;


pub fn xcb_dri2_connect_alignment_pad_length (R : *mut xcb_dri2_connect_reply_t) -> c_int;


pub fn xcb_dri2_connect_alignment_pad_end (R : *mut xcb_dri2_connect_reply_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_dri2_connect_device_name (R : *mut xcb_dri2_connect_reply_t) -> *mut c_char;


pub fn xcb_dri2_connect_device_name_length (R : *mut xcb_dri2_connect_reply_t) -> c_int;


pub fn xcb_dri2_connect_device_name_end (R : *mut xcb_dri2_connect_reply_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_dri2_connect_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_dri2_connect_reply (c : *mut ffi::base::xcb_connection_t,
                                  cookie : xcb_dri2_connect_cookie_t,
                                  e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_dri2_connect_reply_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_dri2_authenticate (c : *mut ffi::base::xcb_connection_t,
                                 window :  ffi::xproto::xcb_window_t,
                                 magic :  u32) -> xcb_dri2_authenticate_cookie_t;

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
pub fn xcb_dri2_authenticate_unchecked (c : *mut ffi::base::xcb_connection_t,
                                           window :  ffi::xproto::xcb_window_t,
                                           magic :  u32) -> xcb_dri2_authenticate_cookie_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_dri2_authenticate_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_dri2_authenticate_reply (c : *mut ffi::base::xcb_connection_t,
                                       cookie : xcb_dri2_authenticate_cookie_t,
                                       e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_dri2_authenticate_reply_t;

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
pub fn xcb_dri2_create_drawable_checked (c : *mut ffi::base::xcb_connection_t,
                                            drawable :  ffi::xproto::xcb_drawable_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_dri2_create_drawable (c : *mut ffi::base::xcb_connection_t,
                                    drawable :  ffi::xproto::xcb_drawable_t) -> ffi::base::xcb_void_cookie_t;

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
pub fn xcb_dri2_destroy_drawable_checked (c : *mut ffi::base::xcb_connection_t,
                                             drawable :  ffi::xproto::xcb_drawable_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_dri2_destroy_drawable (c : *mut ffi::base::xcb_connection_t,
                                     drawable :  ffi::xproto::xcb_drawable_t) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_dri2_get_buffers_sizeof (_buffer :  *mut c_void,
                             attachments_len :  u32) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_dri2_get_buffers (c : *mut ffi::base::xcb_connection_t,
                                drawable :  ffi::xproto::xcb_drawable_t,
                                count :  u32,
                                attachments_len :  u32,
                                attachments : *mut u32) -> xcb_dri2_get_buffers_cookie_t;

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
pub fn xcb_dri2_get_buffers_unchecked (c : *mut ffi::base::xcb_connection_t,
                                          drawable :  ffi::xproto::xcb_drawable_t,
                                          count :  u32,
                                          attachments_len :  u32,
                                          attachments : *mut u32) -> xcb_dri2_get_buffers_cookie_t;

pub fn xcb_dri2_get_buffers_buffers (R : *mut xcb_dri2_get_buffers_reply_t) -> *mut xcb_dri2_dri2_buffer_t;


pub fn xcb_dri2_get_buffers_buffers_length (R : *mut xcb_dri2_get_buffers_reply_t) -> c_int;

pub fn xcb_dri2_get_buffers_buffers_iterator (R : *mut xcb_dri2_get_buffers_reply_t) -> xcb_dri2_dri2_buffer_iterator_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_dri2_get_buffers_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_dri2_get_buffers_reply (c : *mut ffi::base::xcb_connection_t,
                                      cookie : xcb_dri2_get_buffers_cookie_t,
                                      e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_dri2_get_buffers_reply_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_dri2_copy_region (c : *mut ffi::base::xcb_connection_t,
                                drawable :  ffi::xproto::xcb_drawable_t,
                                region :  u32,
                                dest :  u32,
                                src :  u32) -> xcb_dri2_copy_region_cookie_t;

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
pub fn xcb_dri2_copy_region_unchecked (c : *mut ffi::base::xcb_connection_t,
                                          drawable :  ffi::xproto::xcb_drawable_t,
                                          region :  u32,
                                          dest :  u32,
                                          src :  u32) -> xcb_dri2_copy_region_cookie_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_dri2_copy_region_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_dri2_copy_region_reply (c : *mut ffi::base::xcb_connection_t,
                                      cookie : xcb_dri2_copy_region_cookie_t,
                                      e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_dri2_copy_region_reply_t;

pub fn xcb_dri2_get_buffers_with_format_sizeof (_buffer :  *mut c_void,
                                         attachments_len :  u32) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_dri2_get_buffers_with_format (c : *mut ffi::base::xcb_connection_t,
                                            drawable :  ffi::xproto::xcb_drawable_t,
                                            count :  u32,
                                            attachments_len :  u32,
                                            attachments : *mut xcb_dri2_attach_format_t) -> xcb_dri2_get_buffers_with_format_cookie_t;

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
pub fn xcb_dri2_get_buffers_with_format_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                      drawable :  ffi::xproto::xcb_drawable_t,
                                                      count :  u32,
                                                      attachments_len :  u32,
                                                      attachments : *mut xcb_dri2_attach_format_t) -> xcb_dri2_get_buffers_with_format_cookie_t;

pub fn xcb_dri2_get_buffers_with_format_buffers (R : *mut xcb_dri2_get_buffers_with_format_reply_t) -> *mut xcb_dri2_dri2_buffer_t;


pub fn xcb_dri2_get_buffers_with_format_buffers_length (R : *mut xcb_dri2_get_buffers_with_format_reply_t) -> c_int;

pub fn xcb_dri2_get_buffers_with_format_buffers_iterator (R : *mut xcb_dri2_get_buffers_with_format_reply_t) -> xcb_dri2_dri2_buffer_iterator_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_dri2_get_buffers_with_format_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_dri2_get_buffers_with_format_reply (c : *mut ffi::base::xcb_connection_t,
                                                  cookie : xcb_dri2_get_buffers_with_format_cookie_t,
                                                  e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_dri2_get_buffers_with_format_reply_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_dri2_swap_buffers (c : *mut ffi::base::xcb_connection_t,
                                 drawable :  ffi::xproto::xcb_drawable_t,
                                 target_msc_hi :  u32,
                                 target_msc_lo :  u32,
                                 divisor_hi :  u32,
                                 divisor_lo :  u32,
                                 remainder_hi :  u32,
                                 remainder_lo :  u32) -> xcb_dri2_swap_buffers_cookie_t;

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
pub fn xcb_dri2_swap_buffers_unchecked (c : *mut ffi::base::xcb_connection_t,
                                           drawable :  ffi::xproto::xcb_drawable_t,
                                           target_msc_hi :  u32,
                                           target_msc_lo :  u32,
                                           divisor_hi :  u32,
                                           divisor_lo :  u32,
                                           remainder_hi :  u32,
                                           remainder_lo :  u32) -> xcb_dri2_swap_buffers_cookie_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_dri2_swap_buffers_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_dri2_swap_buffers_reply (c : *mut ffi::base::xcb_connection_t,
                                       cookie : xcb_dri2_swap_buffers_cookie_t,
                                       e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_dri2_swap_buffers_reply_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_dri2_get_msc (c : *mut ffi::base::xcb_connection_t,
                            drawable :  ffi::xproto::xcb_drawable_t) -> xcb_dri2_get_msc_cookie_t;

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
pub fn xcb_dri2_get_msc_unchecked (c : *mut ffi::base::xcb_connection_t,
                                      drawable :  ffi::xproto::xcb_drawable_t) -> xcb_dri2_get_msc_cookie_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_dri2_get_msc_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_dri2_get_msc_reply (c : *mut ffi::base::xcb_connection_t,
                                  cookie : xcb_dri2_get_msc_cookie_t,
                                  e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_dri2_get_msc_reply_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_dri2_wait_msc (c : *mut ffi::base::xcb_connection_t,
                             drawable :  ffi::xproto::xcb_drawable_t,
                             target_msc_hi :  u32,
                             target_msc_lo :  u32,
                             divisor_hi :  u32,
                             divisor_lo :  u32,
                             remainder_hi :  u32,
                             remainder_lo :  u32) -> xcb_dri2_wait_msc_cookie_t;

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
pub fn xcb_dri2_wait_msc_unchecked (c : *mut ffi::base::xcb_connection_t,
                                       drawable :  ffi::xproto::xcb_drawable_t,
                                       target_msc_hi :  u32,
                                       target_msc_lo :  u32,
                                       divisor_hi :  u32,
                                       divisor_lo :  u32,
                                       remainder_hi :  u32,
                                       remainder_lo :  u32) -> xcb_dri2_wait_msc_cookie_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_dri2_wait_msc_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_dri2_wait_msc_reply (c : *mut ffi::base::xcb_connection_t,
                                   cookie : xcb_dri2_wait_msc_cookie_t,
                                   e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_dri2_wait_msc_reply_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_dri2_wait_sbc (c : *mut ffi::base::xcb_connection_t,
                             drawable :  ffi::xproto::xcb_drawable_t,
                             target_sbc_hi :  u32,
                             target_sbc_lo :  u32) -> xcb_dri2_wait_sbc_cookie_t;

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
pub fn xcb_dri2_wait_sbc_unchecked (c : *mut ffi::base::xcb_connection_t,
                                       drawable :  ffi::xproto::xcb_drawable_t,
                                       target_sbc_hi :  u32,
                                       target_sbc_lo :  u32) -> xcb_dri2_wait_sbc_cookie_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_dri2_wait_sbc_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_dri2_wait_sbc_reply (c : *mut ffi::base::xcb_connection_t,
                                   cookie : xcb_dri2_wait_sbc_cookie_t,
                                   e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_dri2_wait_sbc_reply_t;

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
pub fn xcb_dri2_swap_interval_checked (c : *mut ffi::base::xcb_connection_t,
                                          drawable :  ffi::xproto::xcb_drawable_t,
                                          interval :  u32) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_dri2_swap_interval (c : *mut ffi::base::xcb_connection_t,
                                  drawable :  ffi::xproto::xcb_drawable_t,
                                  interval :  u32) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_dri2_get_param (c : *mut ffi::base::xcb_connection_t,
                              drawable :  ffi::xproto::xcb_drawable_t,
                              param :  u32) -> xcb_dri2_get_param_cookie_t;

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
pub fn xcb_dri2_get_param_unchecked (c : *mut ffi::base::xcb_connection_t,
                                        drawable :  ffi::xproto::xcb_drawable_t,
                                        param :  u32) -> xcb_dri2_get_param_cookie_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_dri2_get_param_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_dri2_get_param_reply (c : *mut ffi::base::xcb_connection_t,
                                    cookie : xcb_dri2_get_param_cookie_t,
                                    e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_dri2_get_param_reply_t;
}

