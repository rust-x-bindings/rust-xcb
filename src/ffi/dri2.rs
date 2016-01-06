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
pub struct dri2_buffer {
     pub attachment :   u32,
     pub name :         u32,
     pub pitch :        u32,
     pub cpp :          u32,
     pub flags :        u32
}

impl Copy for dri2_buffer {}
impl Clone for dri2_buffer {
    fn clone(&self) -> dri2_buffer { *self }
}
/**
 * @brief dri2_buffer_iterator
 **/
#[repr(C)]
pub struct dri2_buffer_iterator {
    pub data : *mut dri2_buffer,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct attach_format {
     pub attachment :   u32,
     pub format :       u32
}

impl Copy for attach_format {}
impl Clone for attach_format {
    fn clone(&self) -> attach_format { *self }
}
/**
 * @brief attach_format_iterator
 **/
#[repr(C)]
pub struct attach_format_iterator {
    pub data : *mut attach_format,
    pub rem  : c_int,
    pub index: c_int
}


#[derive(Copy, Clone)]
#[repr(C)]
pub struct query_version_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct query_version_request {
     pub major_opcode :    u8,
     pub minor_opcode :    u8,
     pub length :          u16,
     pub major_version :   u32,
     pub minor_version :   u32
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
     pub major_version :   u32,
     pub minor_version :   u32
}

impl Copy for query_version_reply {}
impl Clone for query_version_reply {
    fn clone(&self) -> query_version_reply { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct connect_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct connect_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub window :         ffi::xproto::window,
     pub driver_type :    u32
}

impl Copy for connect_request {}
impl Clone for connect_request {
    fn clone(&self) -> connect_request { *self }
}

#[repr(C)]
pub struct connect_reply {
     pub response_type :        u8,
     pub pad0 :                 u8,
     pub sequence :             u16,
     pub length :               u32,
     pub driver_name_length :   u32,
     pub device_name_length :   u32,
     pub pad1 :                 [u8; 16]
}

impl Copy for connect_reply {}
impl Clone for connect_reply {
    fn clone(&self) -> connect_reply { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct authenticate_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct authenticate_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub window :         ffi::xproto::window,
     pub magic :          u32
}

impl Copy for authenticate_request {}
impl Clone for authenticate_request {
    fn clone(&self) -> authenticate_request { *self }
}

#[repr(C)]
pub struct authenticate_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub authenticated :   u32
}

impl Copy for authenticate_reply {}
impl Clone for authenticate_reply {
    fn clone(&self) -> authenticate_reply { *self }
}


#[repr(C)]
pub struct create_drawable_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub drawable :       ffi::xproto::drawable
}

impl Copy for create_drawable_request {}
impl Clone for create_drawable_request {
    fn clone(&self) -> create_drawable_request { *self }
}


#[repr(C)]
pub struct destroy_drawable_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub drawable :       ffi::xproto::drawable
}

impl Copy for destroy_drawable_request {}
impl Clone for destroy_drawable_request {
    fn clone(&self) -> destroy_drawable_request { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct get_buffers_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct get_buffers_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub drawable :       ffi::xproto::drawable,
     pub count :          u32
}

impl Copy for get_buffers_request {}
impl Clone for get_buffers_request {
    fn clone(&self) -> get_buffers_request { *self }
}

#[repr(C)]
pub struct get_buffers_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub width :           u32,
     pub height :          u32,
     pub count :           u32,
     pub pad1 :            [u8; 12]
}

impl Copy for get_buffers_reply {}
impl Clone for get_buffers_reply {
    fn clone(&self) -> get_buffers_reply { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct copy_region_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct copy_region_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub drawable :       ffi::xproto::drawable,
     pub region :         u32,
     pub dest :           u32,
     pub src :            u32
}

impl Copy for copy_region_request {}
impl Clone for copy_region_request {
    fn clone(&self) -> copy_region_request { *self }
}

#[repr(C)]
pub struct copy_region_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32
}

impl Copy for copy_region_reply {}
impl Clone for copy_region_reply {
    fn clone(&self) -> copy_region_reply { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct get_buffers_with_format_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct get_buffers_with_format_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub drawable :       ffi::xproto::drawable,
     pub count :          u32
}

impl Copy for get_buffers_with_format_request {}
impl Clone for get_buffers_with_format_request {
    fn clone(&self) -> get_buffers_with_format_request { *self }
}

#[repr(C)]
pub struct get_buffers_with_format_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub width :           u32,
     pub height :          u32,
     pub count :           u32,
     pub pad1 :            [u8; 12]
}

impl Copy for get_buffers_with_format_reply {}
impl Clone for get_buffers_with_format_reply {
    fn clone(&self) -> get_buffers_with_format_reply { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct swap_buffers_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct swap_buffers_request {
     pub major_opcode :    u8,
     pub minor_opcode :    u8,
     pub length :          u16,
     pub drawable :        ffi::xproto::drawable,
     pub target_msc_hi :   u32,
     pub target_msc_lo :   u32,
     pub divisor_hi :      u32,
     pub divisor_lo :      u32,
     pub remainder_hi :    u32,
     pub remainder_lo :    u32
}

impl Copy for swap_buffers_request {}
impl Clone for swap_buffers_request {
    fn clone(&self) -> swap_buffers_request { *self }
}

#[repr(C)]
pub struct swap_buffers_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub swap_hi :         u32,
     pub swap_lo :         u32
}

impl Copy for swap_buffers_reply {}
impl Clone for swap_buffers_reply {
    fn clone(&self) -> swap_buffers_reply { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct get_msc_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct get_msc_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub drawable :       ffi::xproto::drawable
}

impl Copy for get_msc_request {}
impl Clone for get_msc_request {
    fn clone(&self) -> get_msc_request { *self }
}

#[repr(C)]
pub struct get_msc_reply {
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

impl Copy for get_msc_reply {}
impl Clone for get_msc_reply {
    fn clone(&self) -> get_msc_reply { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct wait_msc_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct wait_msc_request {
     pub major_opcode :    u8,
     pub minor_opcode :    u8,
     pub length :          u16,
     pub drawable :        ffi::xproto::drawable,
     pub target_msc_hi :   u32,
     pub target_msc_lo :   u32,
     pub divisor_hi :      u32,
     pub divisor_lo :      u32,
     pub remainder_hi :    u32,
     pub remainder_lo :    u32
}

impl Copy for wait_msc_request {}
impl Clone for wait_msc_request {
    fn clone(&self) -> wait_msc_request { *self }
}

#[repr(C)]
pub struct wait_msc_reply {
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

impl Copy for wait_msc_reply {}
impl Clone for wait_msc_reply {
    fn clone(&self) -> wait_msc_reply { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct wait_sbc_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct wait_sbc_request {
     pub major_opcode :    u8,
     pub minor_opcode :    u8,
     pub length :          u16,
     pub drawable :        ffi::xproto::drawable,
     pub target_sbc_hi :   u32,
     pub target_sbc_lo :   u32
}

impl Copy for wait_sbc_request {}
impl Clone for wait_sbc_request {
    fn clone(&self) -> wait_sbc_request { *self }
}

#[repr(C)]
pub struct wait_sbc_reply {
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

impl Copy for wait_sbc_reply {}
impl Clone for wait_sbc_reply {
    fn clone(&self) -> wait_sbc_reply { *self }
}


#[repr(C)]
pub struct swap_interval_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub drawable :       ffi::xproto::drawable,
     pub interval :       u32
}

impl Copy for swap_interval_request {}
impl Clone for swap_interval_request {
    fn clone(&self) -> swap_interval_request { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct get_param_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct get_param_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub drawable :       ffi::xproto::drawable,
     pub param :          u32
}

impl Copy for get_param_request {}
impl Clone for get_param_request {
    fn clone(&self) -> get_param_request { *self }
}

#[repr(C)]
pub struct get_param_reply {
     pub response_type :         u8,
     pub is_param_recognized :   u8,
     pub sequence :              u16,
     pub length :                u32,
     pub value_hi :              u32,
     pub value_lo :              u32
}

impl Copy for get_param_reply {}
impl Clone for get_param_reply {
    fn clone(&self) -> get_param_reply { *self }
}


#[repr(C)]
pub struct buffer_swap_complete_event {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub event_type :      u16,
     pub pad1 :            [u8; 2],
     pub drawable :        ffi::xproto::drawable,
     pub ust_hi :          u32,
     pub ust_lo :          u32,
     pub msc_hi :          u32,
     pub msc_lo :          u32,
     pub sbc :             u32
}

impl Copy for buffer_swap_complete_event {}
impl Clone for buffer_swap_complete_event {
    fn clone(&self) -> buffer_swap_complete_event { *self }
}


#[repr(C)]
pub struct invalidate_buffers_event {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub drawable :        ffi::xproto::drawable
}

impl Copy for invalidate_buffers_event {}
impl Clone for invalidate_buffers_event {
    fn clone(&self) -> invalidate_buffers_event { *self }
}
#[link(name="xcb-dri2")]
extern "C" {

/**
 * Get the next element of the iterator
 * @param i Pointer to a dri2_buffer_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(dri2_buffer)
 *
 *
 */
pub fn xcb_dri2_dri2_buffer_next (i:*mut dri2_buffer_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An dri2_buffer_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_dri2_dri2_buffer_end (i:dri2_buffer_iterator) -> ffi::base::generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a attach_format_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(attach_format)
 *
 *
 */
pub fn xcb_dri2_attach_format_next (i:*mut attach_format_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An attach_format_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_dri2_attach_format_end (i:attach_format_iterator) -> ffi::base::generic_iterator;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_dri2_query_version (c : *mut ffi::base::connection,
                                  major_version :  u32,
                                  minor_version :  u32) -> query_version_cookie;

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
pub fn xcb_dri2_query_version_unchecked (c : *mut ffi::base::connection,
                                            major_version :  u32,
                                            minor_version :  u32) -> query_version_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_dri2_query_version_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_dri2_query_version_reply (c : *mut ffi::base::connection,
                                        cookie : query_version_cookie,
                                        e : *mut *mut ffi::base::generic_error) -> *mut query_version_reply;

pub fn xcb_dri2_connect_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_dri2_connect (c : *mut ffi::base::connection,
                            window :  ffi::xproto::window,
                            driver_type :  u32) -> connect_cookie;

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
pub fn xcb_dri2_connect_unchecked (c : *mut ffi::base::connection,
                                      window :  ffi::xproto::window,
                                      driver_type :  u32) -> connect_cookie;

pub fn xcb_dri2_connect_driver_name (R : *mut connect_reply) -> *mut c_char;


pub fn xcb_dri2_connect_driver_name_length (R : *mut connect_reply) -> c_int;


pub fn xcb_dri2_connect_driver_name_end (R : *mut connect_reply) -> ffi::base::generic_iterator;

pub fn xcb_dri2_connect_alignment_pad (R : *mut connect_reply) -> *mut c_void;


pub fn xcb_dri2_connect_alignment_pad_length (R : *mut connect_reply) -> c_int;


pub fn xcb_dri2_connect_alignment_pad_end (R : *mut connect_reply) -> ffi::base::generic_iterator;

pub fn xcb_dri2_connect_device_name (R : *mut connect_reply) -> *mut c_char;


pub fn xcb_dri2_connect_device_name_length (R : *mut connect_reply) -> c_int;


pub fn xcb_dri2_connect_device_name_end (R : *mut connect_reply) -> ffi::base::generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_dri2_connect_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_dri2_connect_reply (c : *mut ffi::base::connection,
                                  cookie : connect_cookie,
                                  e : *mut *mut ffi::base::generic_error) -> *mut connect_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_dri2_authenticate (c : *mut ffi::base::connection,
                                 window :  ffi::xproto::window,
                                 magic :  u32) -> authenticate_cookie;

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
pub fn xcb_dri2_authenticate_unchecked (c : *mut ffi::base::connection,
                                           window :  ffi::xproto::window,
                                           magic :  u32) -> authenticate_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_dri2_authenticate_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_dri2_authenticate_reply (c : *mut ffi::base::connection,
                                       cookie : authenticate_cookie,
                                       e : *mut *mut ffi::base::generic_error) -> *mut authenticate_reply;

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
pub fn xcb_dri2_create_drawable_checked (c : *mut ffi::base::connection,
                                            drawable :  ffi::xproto::drawable) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_dri2_create_drawable (c : *mut ffi::base::connection,
                                    drawable :  ffi::xproto::drawable) -> ffi::base::void_cookie;

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
pub fn xcb_dri2_destroy_drawable_checked (c : *mut ffi::base::connection,
                                             drawable :  ffi::xproto::drawable) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_dri2_destroy_drawable (c : *mut ffi::base::connection,
                                     drawable :  ffi::xproto::drawable) -> ffi::base::void_cookie;

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
pub fn xcb_dri2_get_buffers (c : *mut ffi::base::connection,
                                drawable :  ffi::xproto::drawable,
                                count :  u32,
                                attachments_len :  u32,
                                attachments : *mut u32) -> get_buffers_cookie;

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
pub fn xcb_dri2_get_buffers_unchecked (c : *mut ffi::base::connection,
                                          drawable :  ffi::xproto::drawable,
                                          count :  u32,
                                          attachments_len :  u32,
                                          attachments : *mut u32) -> get_buffers_cookie;

pub fn xcb_dri2_get_buffers_buffers (R : *mut get_buffers_reply) -> *mut dri2_buffer;


pub fn xcb_dri2_get_buffers_buffers_length (R : *mut get_buffers_reply) -> c_int;

pub fn xcb_dri2_get_buffers_buffers_iterator (R : *mut get_buffers_reply) -> dri2_buffer_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_dri2_get_buffers_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_dri2_get_buffers_reply (c : *mut ffi::base::connection,
                                      cookie : get_buffers_cookie,
                                      e : *mut *mut ffi::base::generic_error) -> *mut get_buffers_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_dri2_copy_region (c : *mut ffi::base::connection,
                                drawable :  ffi::xproto::drawable,
                                region :  u32,
                                dest :  u32,
                                src :  u32) -> copy_region_cookie;

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
pub fn xcb_dri2_copy_region_unchecked (c : *mut ffi::base::connection,
                                          drawable :  ffi::xproto::drawable,
                                          region :  u32,
                                          dest :  u32,
                                          src :  u32) -> copy_region_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_dri2_copy_region_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_dri2_copy_region_reply (c : *mut ffi::base::connection,
                                      cookie : copy_region_cookie,
                                      e : *mut *mut ffi::base::generic_error) -> *mut copy_region_reply;

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
pub fn xcb_dri2_get_buffers_with_format (c : *mut ffi::base::connection,
                                            drawable :  ffi::xproto::drawable,
                                            count :  u32,
                                            attachments_len :  u32,
                                            attachments : *mut attach_format) -> get_buffers_with_format_cookie;

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
pub fn xcb_dri2_get_buffers_with_format_unchecked (c : *mut ffi::base::connection,
                                                      drawable :  ffi::xproto::drawable,
                                                      count :  u32,
                                                      attachments_len :  u32,
                                                      attachments : *mut attach_format) -> get_buffers_with_format_cookie;

pub fn xcb_dri2_get_buffers_with_format_buffers (R : *mut get_buffers_with_format_reply) -> *mut dri2_buffer;


pub fn xcb_dri2_get_buffers_with_format_buffers_length (R : *mut get_buffers_with_format_reply) -> c_int;

pub fn xcb_dri2_get_buffers_with_format_buffers_iterator (R : *mut get_buffers_with_format_reply) -> dri2_buffer_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_dri2_get_buffers_with_format_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_dri2_get_buffers_with_format_reply (c : *mut ffi::base::connection,
                                                  cookie : get_buffers_with_format_cookie,
                                                  e : *mut *mut ffi::base::generic_error) -> *mut get_buffers_with_format_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_dri2_swap_buffers (c : *mut ffi::base::connection,
                                 drawable :  ffi::xproto::drawable,
                                 target_msc_hi :  u32,
                                 target_msc_lo :  u32,
                                 divisor_hi :  u32,
                                 divisor_lo :  u32,
                                 remainder_hi :  u32,
                                 remainder_lo :  u32) -> swap_buffers_cookie;

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
pub fn xcb_dri2_swap_buffers_unchecked (c : *mut ffi::base::connection,
                                           drawable :  ffi::xproto::drawable,
                                           target_msc_hi :  u32,
                                           target_msc_lo :  u32,
                                           divisor_hi :  u32,
                                           divisor_lo :  u32,
                                           remainder_hi :  u32,
                                           remainder_lo :  u32) -> swap_buffers_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_dri2_swap_buffers_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_dri2_swap_buffers_reply (c : *mut ffi::base::connection,
                                       cookie : swap_buffers_cookie,
                                       e : *mut *mut ffi::base::generic_error) -> *mut swap_buffers_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_dri2_get_msc (c : *mut ffi::base::connection,
                            drawable :  ffi::xproto::drawable) -> get_msc_cookie;

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
pub fn xcb_dri2_get_msc_unchecked (c : *mut ffi::base::connection,
                                      drawable :  ffi::xproto::drawable) -> get_msc_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_dri2_get_msc_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_dri2_get_msc_reply (c : *mut ffi::base::connection,
                                  cookie : get_msc_cookie,
                                  e : *mut *mut ffi::base::generic_error) -> *mut get_msc_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_dri2_wait_msc (c : *mut ffi::base::connection,
                             drawable :  ffi::xproto::drawable,
                             target_msc_hi :  u32,
                             target_msc_lo :  u32,
                             divisor_hi :  u32,
                             divisor_lo :  u32,
                             remainder_hi :  u32,
                             remainder_lo :  u32) -> wait_msc_cookie;

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
pub fn xcb_dri2_wait_msc_unchecked (c : *mut ffi::base::connection,
                                       drawable :  ffi::xproto::drawable,
                                       target_msc_hi :  u32,
                                       target_msc_lo :  u32,
                                       divisor_hi :  u32,
                                       divisor_lo :  u32,
                                       remainder_hi :  u32,
                                       remainder_lo :  u32) -> wait_msc_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_dri2_wait_msc_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_dri2_wait_msc_reply (c : *mut ffi::base::connection,
                                   cookie : wait_msc_cookie,
                                   e : *mut *mut ffi::base::generic_error) -> *mut wait_msc_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_dri2_wait_sbc (c : *mut ffi::base::connection,
                             drawable :  ffi::xproto::drawable,
                             target_sbc_hi :  u32,
                             target_sbc_lo :  u32) -> wait_sbc_cookie;

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
pub fn xcb_dri2_wait_sbc_unchecked (c : *mut ffi::base::connection,
                                       drawable :  ffi::xproto::drawable,
                                       target_sbc_hi :  u32,
                                       target_sbc_lo :  u32) -> wait_sbc_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_dri2_wait_sbc_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_dri2_wait_sbc_reply (c : *mut ffi::base::connection,
                                   cookie : wait_sbc_cookie,
                                   e : *mut *mut ffi::base::generic_error) -> *mut wait_sbc_reply;

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
pub fn xcb_dri2_swap_interval_checked (c : *mut ffi::base::connection,
                                          drawable :  ffi::xproto::drawable,
                                          interval :  u32) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_dri2_swap_interval (c : *mut ffi::base::connection,
                                  drawable :  ffi::xproto::drawable,
                                  interval :  u32) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_dri2_get_param (c : *mut ffi::base::connection,
                              drawable :  ffi::xproto::drawable,
                              param :  u32) -> get_param_cookie;

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
pub fn xcb_dri2_get_param_unchecked (c : *mut ffi::base::connection,
                                        drawable :  ffi::xproto::drawable,
                                        param :  u32) -> get_param_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_dri2_get_param_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_dri2_get_param_reply (c : *mut ffi::base::connection,
                                    cookie : get_param_cookie,
                                    e : *mut *mut ffi::base::generic_error) -> *mut get_param_reply;
}

