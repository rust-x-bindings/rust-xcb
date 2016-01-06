/*
 * This file generated automatically from xf86dri.xml by r_client.py.
 * Edit at your peril.
 */

//Make the compiler quiet
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use std;
use libc::*;
use ffi;

pub static XF86DRI_MAJOR_VERSION : c_uint = 4;
pub static XF86DRI_MINOR_VERSION : c_uint = 1;

#[repr(C)]
pub struct drm_clip_rect {
     pub x1 :   i16,
     pub y1 :   i16,
     pub x2 :   i16,
     pub x3 :   i16
}

impl Copy for drm_clip_rect {}
impl Clone for drm_clip_rect {
    fn clone(&self) -> drm_clip_rect { *self }
}
/**
 * @brief drm_clip_rect_iterator
 **/
#[repr(C)]
pub struct drm_clip_rect_iterator {
    pub data : *mut drm_clip_rect,
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
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16
}

impl Copy for query_version_request {}
impl Clone for query_version_request {
    fn clone(&self) -> query_version_request { *self }
}

#[repr(C)]
pub struct query_version_reply {
     pub response_type :       u8,
     pub pad0 :                u8,
     pub sequence :            u16,
     pub length :              u32,
     pub dri_major_version :   u16,
     pub dri_minor_version :   u16,
     pub dri_minor_patch :     u32
}

impl Copy for query_version_reply {}
impl Clone for query_version_reply {
    fn clone(&self) -> query_version_reply { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct query_direct_rendering_capable_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct query_direct_rendering_capable_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub screen :         u32
}

impl Copy for query_direct_rendering_capable_request {}
impl Clone for query_direct_rendering_capable_request {
    fn clone(&self) -> query_direct_rendering_capable_request { *self }
}

#[repr(C)]
pub struct query_direct_rendering_capable_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub is_capable :      u8
}

impl Copy for query_direct_rendering_capable_reply {}
impl Clone for query_direct_rendering_capable_reply {
    fn clone(&self) -> query_direct_rendering_capable_reply { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct open_connection_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct open_connection_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub screen :         u32
}

impl Copy for open_connection_request {}
impl Clone for open_connection_request {
    fn clone(&self) -> open_connection_request { *self }
}

#[repr(C)]
pub struct open_connection_reply {
     pub response_type :       u8,
     pub pad0 :                u8,
     pub sequence :            u16,
     pub length :              u32,
     pub sarea_handle_low :    u32,
     pub sarea_handle_high :   u32,
     pub bus_id_len :          u32,
     pub pad1 :                [u8; 12]
}

impl Copy for open_connection_reply {}
impl Clone for open_connection_reply {
    fn clone(&self) -> open_connection_reply { *self }
}


#[repr(C)]
pub struct close_connection_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub screen :         u32
}

impl Copy for close_connection_request {}
impl Clone for close_connection_request {
    fn clone(&self) -> close_connection_request { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct get_client_driver_name_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct get_client_driver_name_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub screen :         u32
}

impl Copy for get_client_driver_name_request {}
impl Clone for get_client_driver_name_request {
    fn clone(&self) -> get_client_driver_name_request { *self }
}

#[repr(C)]
pub struct get_client_driver_name_reply {
     pub response_type :                 u8,
     pub pad0 :                          u8,
     pub sequence :                      u16,
     pub length :                        u32,
     pub client_driver_major_version :   u32,
     pub client_driver_minor_version :   u32,
     pub client_driver_patch_version :   u32,
     pub client_driver_name_len :        u32,
     pub pad1 :                          [u8; 8]
}

impl Copy for get_client_driver_name_reply {}
impl Clone for get_client_driver_name_reply {
    fn clone(&self) -> get_client_driver_name_reply { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct create_context_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct create_context_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub screen :         u32,
     pub visual :         u32,
     pub context :        u32
}

impl Copy for create_context_request {}
impl Clone for create_context_request {
    fn clone(&self) -> create_context_request { *self }
}

#[repr(C)]
pub struct create_context_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub hw_context :      u32
}

impl Copy for create_context_reply {}
impl Clone for create_context_reply {
    fn clone(&self) -> create_context_reply { *self }
}


#[repr(C)]
pub struct destroy_context_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub screen :         u32,
     pub context :        u32
}

impl Copy for destroy_context_request {}
impl Clone for destroy_context_request {
    fn clone(&self) -> destroy_context_request { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct create_drawable_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct create_drawable_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub screen :         u32,
     pub drawable :       u32
}

impl Copy for create_drawable_request {}
impl Clone for create_drawable_request {
    fn clone(&self) -> create_drawable_request { *self }
}

#[repr(C)]
pub struct create_drawable_reply {
     pub response_type :        u8,
     pub pad0 :                 u8,
     pub sequence :             u16,
     pub length :               u32,
     pub hw_drawable_handle :   u32
}

impl Copy for create_drawable_reply {}
impl Clone for create_drawable_reply {
    fn clone(&self) -> create_drawable_reply { *self }
}


#[repr(C)]
pub struct destroy_drawable_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub screen :         u32,
     pub drawable :       u32
}

impl Copy for destroy_drawable_request {}
impl Clone for destroy_drawable_request {
    fn clone(&self) -> destroy_drawable_request { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct get_drawable_info_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct get_drawable_info_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub screen :         u32,
     pub drawable :       u32
}

impl Copy for get_drawable_info_request {}
impl Clone for get_drawable_info_request {
    fn clone(&self) -> get_drawable_info_request { *self }
}

#[repr(C)]
pub struct get_drawable_info_reply {
     pub response_type :          u8,
     pub pad0 :                   u8,
     pub sequence :               u16,
     pub length :                 u32,
     pub drawable_table_index :   u32,
     pub drawable_table_stamp :   u32,
     pub drawable_origin_X :      i16,
     pub drawable_origin_Y :      i16,
     pub drawable_size_W :        i16,
     pub drawable_size_H :        i16,
     pub num_clip_rects :         u32,
     pub back_x :                 i16,
     pub back_y :                 i16,
     pub num_back_clip_rects :    u32
}

impl Copy for get_drawable_info_reply {}
impl Clone for get_drawable_info_reply {
    fn clone(&self) -> get_drawable_info_reply { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct get_device_info_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct get_device_info_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub screen :         u32
}

impl Copy for get_device_info_request {}
impl Clone for get_device_info_request {
    fn clone(&self) -> get_device_info_request { *self }
}

#[repr(C)]
pub struct get_device_info_reply {
     pub response_type :               u8,
     pub pad0 :                        u8,
     pub sequence :                    u16,
     pub length :                      u32,
     pub framebuffer_handle_low :      u32,
     pub framebuffer_handle_high :     u32,
     pub framebuffer_origin_offset :   u32,
     pub framebuffer_size :            u32,
     pub framebuffer_stride :          u32,
     pub device_private_size :         u32
}

impl Copy for get_device_info_reply {}
impl Clone for get_device_info_reply {
    fn clone(&self) -> get_device_info_reply { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct auth_connection_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct auth_connection_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub screen :         u32,
     pub magic :          u32
}

impl Copy for auth_connection_request {}
impl Clone for auth_connection_request {
    fn clone(&self) -> auth_connection_request { *self }
}

#[repr(C)]
pub struct auth_connection_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub authenticated :   u32
}

impl Copy for auth_connection_reply {}
impl Clone for auth_connection_reply {
    fn clone(&self) -> auth_connection_reply { *self }
}
#[link(name="xcb-xf86dri")]
extern "C" {

/**
 * Get the next element of the iterator
 * @param i Pointer to a drm_clip_rect_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(drm_clip_rect)
 *
 *
 */
pub fn xcb_xf86dri_drm_clip_rect_next (i:*mut drm_clip_rect_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An drm_clip_rect_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_xf86dri_drm_clip_rect_end (i:drm_clip_rect_iterator) -> ffi::base::generic_iterator;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xf86dri_query_version (c : *mut ffi::base::connection) -> query_version_cookie;

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
pub fn xcb_xf86dri_query_version_unchecked (c : *mut ffi::base::connection) -> query_version_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xf86dri_query_version_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_xf86dri_query_version_reply (c : *mut ffi::base::connection,
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
pub fn xcb_xf86dri_query_direct_rendering_capable (c : *mut ffi::base::connection,
                                                      screen :  u32) -> query_direct_rendering_capable_cookie;

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
pub fn xcb_xf86dri_query_direct_rendering_capable_unchecked (c : *mut ffi::base::connection,
                                                                screen :  u32) -> query_direct_rendering_capable_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xf86dri_query_direct_rendering_capable_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_xf86dri_query_direct_rendering_capable_reply (c : *mut ffi::base::connection,
                                                            cookie : query_direct_rendering_capable_cookie,
                                                            e : *mut *mut ffi::base::generic_error) -> *mut query_direct_rendering_capable_reply;

pub fn xcb_xf86dri_open_connection_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xf86dri_open_connection (c : *mut ffi::base::connection,
                                       screen :  u32) -> open_connection_cookie;

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
pub fn xcb_xf86dri_open_connection_unchecked (c : *mut ffi::base::connection,
                                                 screen :  u32) -> open_connection_cookie;

pub fn xcb_xf86dri_open_connection_bus_id (R : *mut open_connection_reply) -> *mut c_char;


pub fn xcb_xf86dri_open_connection_bus_id_length (R : *mut open_connection_reply) -> c_int;


pub fn xcb_xf86dri_open_connection_bus_id_end (R : *mut open_connection_reply) -> ffi::base::generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xf86dri_open_connection_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_xf86dri_open_connection_reply (c : *mut ffi::base::connection,
                                             cookie : open_connection_cookie,
                                             e : *mut *mut ffi::base::generic_error) -> *mut open_connection_reply;

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
pub fn xcb_xf86dri_close_connection_checked (c : *mut ffi::base::connection,
                                                screen :  u32) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xf86dri_close_connection (c : *mut ffi::base::connection,
                                        screen :  u32) -> ffi::base::void_cookie;

pub fn xcb_xf86dri_get_client_driver_name_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xf86dri_get_client_driver_name (c : *mut ffi::base::connection,
                                              screen :  u32) -> get_client_driver_name_cookie;

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
pub fn xcb_xf86dri_get_client_driver_name_unchecked (c : *mut ffi::base::connection,
                                                        screen :  u32) -> get_client_driver_name_cookie;

pub fn xcb_xf86dri_get_client_driver_name_client_driver_name (R : *mut get_client_driver_name_reply) -> *mut c_char;


pub fn xcb_xf86dri_get_client_driver_name_client_driver_name_length (R : *mut get_client_driver_name_reply) -> c_int;


pub fn xcb_xf86dri_get_client_driver_name_client_driver_name_end (R : *mut get_client_driver_name_reply) -> ffi::base::generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xf86dri_get_client_driver_name_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_xf86dri_get_client_driver_name_reply (c : *mut ffi::base::connection,
                                                    cookie : get_client_driver_name_cookie,
                                                    e : *mut *mut ffi::base::generic_error) -> *mut get_client_driver_name_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xf86dri_create_context (c : *mut ffi::base::connection,
                                      screen :  u32,
                                      visual :  u32,
                                      context :  u32) -> create_context_cookie;

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
pub fn xcb_xf86dri_create_context_unchecked (c : *mut ffi::base::connection,
                                                screen :  u32,
                                                visual :  u32,
                                                context :  u32) -> create_context_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xf86dri_create_context_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_xf86dri_create_context_reply (c : *mut ffi::base::connection,
                                            cookie : create_context_cookie,
                                            e : *mut *mut ffi::base::generic_error) -> *mut create_context_reply;

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
pub fn xcb_xf86dri_destroy_context_checked (c : *mut ffi::base::connection,
                                               screen :  u32,
                                               context :  u32) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xf86dri_destroy_context (c : *mut ffi::base::connection,
                                       screen :  u32,
                                       context :  u32) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xf86dri_create_drawable (c : *mut ffi::base::connection,
                                       screen :  u32,
                                       drawable :  u32) -> create_drawable_cookie;

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
pub fn xcb_xf86dri_create_drawable_unchecked (c : *mut ffi::base::connection,
                                                 screen :  u32,
                                                 drawable :  u32) -> create_drawable_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xf86dri_create_drawable_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_xf86dri_create_drawable_reply (c : *mut ffi::base::connection,
                                             cookie : create_drawable_cookie,
                                             e : *mut *mut ffi::base::generic_error) -> *mut create_drawable_reply;

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
pub fn xcb_xf86dri_destroy_drawable_checked (c : *mut ffi::base::connection,
                                                screen :  u32,
                                                drawable :  u32) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xf86dri_destroy_drawable (c : *mut ffi::base::connection,
                                        screen :  u32,
                                        drawable :  u32) -> ffi::base::void_cookie;

pub fn xcb_xf86dri_get_drawable_info_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xf86dri_get_drawable_info (c : *mut ffi::base::connection,
                                         screen :  u32,
                                         drawable :  u32) -> get_drawable_info_cookie;

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
pub fn xcb_xf86dri_get_drawable_info_unchecked (c : *mut ffi::base::connection,
                                                   screen :  u32,
                                                   drawable :  u32) -> get_drawable_info_cookie;

pub fn xcb_xf86dri_get_drawable_info_clip_rects (R : *mut get_drawable_info_reply) -> *mut drm_clip_rect;


pub fn xcb_xf86dri_get_drawable_info_clip_rects_length (R : *mut get_drawable_info_reply) -> c_int;

pub fn xcb_xf86dri_get_drawable_info_clip_rects_iterator (R : *mut get_drawable_info_reply) -> drm_clip_rect_iterator;

pub fn xcb_xf86dri_get_drawable_info_back_clip_rects (R : *mut get_drawable_info_reply) -> *mut drm_clip_rect;


pub fn xcb_xf86dri_get_drawable_info_back_clip_rects_length (R : *mut get_drawable_info_reply) -> c_int;

pub fn xcb_xf86dri_get_drawable_info_back_clip_rects_iterator (R : *mut get_drawable_info_reply) -> drm_clip_rect_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xf86dri_get_drawable_info_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_xf86dri_get_drawable_info_reply (c : *mut ffi::base::connection,
                                               cookie : get_drawable_info_cookie,
                                               e : *mut *mut ffi::base::generic_error) -> *mut get_drawable_info_reply;

pub fn xcb_xf86dri_get_device_info_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xf86dri_get_device_info (c : *mut ffi::base::connection,
                                       screen :  u32) -> get_device_info_cookie;

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
pub fn xcb_xf86dri_get_device_info_unchecked (c : *mut ffi::base::connection,
                                                 screen :  u32) -> get_device_info_cookie;

pub fn xcb_xf86dri_get_device_info_device_private (R : *mut get_device_info_reply) -> *mut u32;


pub fn xcb_xf86dri_get_device_info_device_private_length (R : *mut get_device_info_reply) -> c_int;


pub fn xcb_xf86dri_get_device_info_device_private_end (R : *mut get_device_info_reply) -> ffi::base::generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xf86dri_get_device_info_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_xf86dri_get_device_info_reply (c : *mut ffi::base::connection,
                                             cookie : get_device_info_cookie,
                                             e : *mut *mut ffi::base::generic_error) -> *mut get_device_info_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xf86dri_auth_connection (c : *mut ffi::base::connection,
                                       screen :  u32,
                                       magic :  u32) -> auth_connection_cookie;

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
pub fn xcb_xf86dri_auth_connection_unchecked (c : *mut ffi::base::connection,
                                                 screen :  u32,
                                                 magic :  u32) -> auth_connection_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xf86dri_auth_connection_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_xf86dri_auth_connection_reply (c : *mut ffi::base::connection,
                                             cookie : auth_connection_cookie,
                                             e : *mut *mut ffi::base::generic_error) -> *mut auth_connection_reply;
}

