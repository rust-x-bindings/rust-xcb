/*
 * This file generated automatically from shm.xml by r_client.py.
 * Edit at your peril.
 */

//Make the compiler quiet
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use std;
use libc::*;
use ffi;
use ffi::xproto;

pub const SHM_MAJOR_VERSION : c_uint = 1;
pub const SHM_MINOR_VERSION : c_uint = 1;

pub type xcb_shm_seg_t = u32;
/**
 * @brief xcb_shm_seg_iterator_t
 **/
#[repr(C)]
pub struct xcb_shm_seg_iterator_t {
    pub data : *mut xcb_shm_seg_t,
    pub rem  : c_int,
    pub index: c_int
}



#[repr(C)]
pub struct xcb_shm_completion_event_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub drawable :        ffi::xproto::xcb_drawable_t,
     pub minor_event :     u16,
     pub major_event :     u8,
     pub pad1 :            u8,
     pub shmseg :          xcb_shm_seg_t,
     pub offset :          u32
}

impl Copy for xcb_shm_completion_event_t {}
impl Clone for xcb_shm_completion_event_t {
    fn clone(&self) -> xcb_shm_completion_event_t { *self }
}


pub type xcb_shm_bad_seg_error_t  = ffi::xproto::xcb_value_error_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_shm_query_version_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_shm_query_version_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16
}

impl Copy for xcb_shm_query_version_request_t {}
impl Clone for xcb_shm_query_version_request_t {
    fn clone(&self) -> xcb_shm_query_version_request_t { *self }
}

#[repr(C)]
pub struct xcb_shm_query_version_reply_t {
     pub response_type :    u8,
     pub shared_pixmaps :   u8,
     pub sequence :         u16,
     pub length :           u32,
     pub major_version :    u16,
     pub minor_version :    u16,
     pub uid :              u16,
     pub gid :              u16,
     pub pixmap_format :    u8,
     pub pad0 :             [u8; 15]
}

impl Copy for xcb_shm_query_version_reply_t {}
impl Clone for xcb_shm_query_version_reply_t {
    fn clone(&self) -> xcb_shm_query_version_reply_t { *self }
}


#[repr(C)]
pub struct xcb_shm_attach_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub shmseg :         xcb_shm_seg_t,
     pub shmid :          u32,
     pub read_only :      u8,
     pub pad0 :           [u8; 3]
}

impl Copy for xcb_shm_attach_request_t {}
impl Clone for xcb_shm_attach_request_t {
    fn clone(&self) -> xcb_shm_attach_request_t { *self }
}


#[repr(C)]
pub struct xcb_shm_detach_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub shmseg :         xcb_shm_seg_t
}

impl Copy for xcb_shm_detach_request_t {}
impl Clone for xcb_shm_detach_request_t {
    fn clone(&self) -> xcb_shm_detach_request_t { *self }
}


#[repr(C)]
pub struct xcb_shm_put_image_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub drawable :       ffi::xproto::xcb_drawable_t,
     pub gc :             ffi::xproto::xcb_gcontext_t,
     pub total_width :    u16,
     pub total_height :   u16,
     pub src_x :          u16,
     pub src_y :          u16,
     pub src_width :      u16,
     pub src_height :     u16,
     pub dst_x :          i16,
     pub dst_y :          i16,
     pub depth :          u8,
     pub format :         u8,
     pub send_event :     u8,
     pub pad0 :           u8,
     pub shmseg :         xcb_shm_seg_t,
     pub offset :         u32
}

impl Copy for xcb_shm_put_image_request_t {}
impl Clone for xcb_shm_put_image_request_t {
    fn clone(&self) -> xcb_shm_put_image_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_shm_get_image_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_shm_get_image_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub drawable :       ffi::xproto::xcb_drawable_t,
     pub x :              i16,
     pub y :              i16,
     pub width :          u16,
     pub height :         u16,
     pub plane_mask :     u32,
     pub format :         u8,
     pub pad0 :           [u8; 3],
     pub shmseg :         xcb_shm_seg_t,
     pub offset :         u32
}

impl Copy for xcb_shm_get_image_request_t {}
impl Clone for xcb_shm_get_image_request_t {
    fn clone(&self) -> xcb_shm_get_image_request_t { *self }
}

#[repr(C)]
pub struct xcb_shm_get_image_reply_t {
     pub response_type :   u8,
     pub depth :           u8,
     pub sequence :        u16,
     pub length :          u32,
     pub visual :          ffi::xproto::xcb_visualid_t,
     pub size :            u32
}

impl Copy for xcb_shm_get_image_reply_t {}
impl Clone for xcb_shm_get_image_reply_t {
    fn clone(&self) -> xcb_shm_get_image_reply_t { *self }
}


#[repr(C)]
pub struct xcb_shm_create_pixmap_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub pid :            ffi::xproto::xcb_pixmap_t,
     pub drawable :       ffi::xproto::xcb_drawable_t,
     pub width :          u16,
     pub height :         u16,
     pub depth :          u8,
     pub pad0 :           [u8; 3],
     pub shmseg :         xcb_shm_seg_t,
     pub offset :         u32
}

impl Copy for xcb_shm_create_pixmap_request_t {}
impl Clone for xcb_shm_create_pixmap_request_t {
    fn clone(&self) -> xcb_shm_create_pixmap_request_t { *self }
}
#[link(name="xcb-shm")]
extern "C" {

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_shm_seg_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_shm_seg_t)
 *
 *
 */
pub fn xcb_shm_seg_next (i:*mut xcb_shm_seg_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_shm_seg_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_shm_seg_end (i:xcb_shm_seg_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_shm_query_version (c : *mut ffi::base::xcb_connection_t) -> xcb_shm_query_version_cookie_t;

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
pub fn xcb_shm_query_version_unchecked (c : *mut ffi::base::xcb_connection_t) -> xcb_shm_query_version_cookie_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_shm_query_version_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_shm_query_version_reply (c : *mut ffi::base::xcb_connection_t,
                                       cookie : xcb_shm_query_version_cookie_t,
                                       e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_shm_query_version_reply_t;

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
pub fn xcb_shm_attach_checked (c : *mut ffi::base::xcb_connection_t,
                                  shmseg :  xcb_shm_seg_t,
                                  shmid :  u32,
                                  read_only :  u8) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_shm_attach (c : *mut ffi::base::xcb_connection_t,
                          shmseg :  xcb_shm_seg_t,
                          shmid :  u32,
                          read_only :  u8) -> ffi::base::xcb_void_cookie_t;

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
pub fn xcb_shm_detach_checked (c : *mut ffi::base::xcb_connection_t,
                                  shmseg :  xcb_shm_seg_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_shm_detach (c : *mut ffi::base::xcb_connection_t,
                          shmseg :  xcb_shm_seg_t) -> ffi::base::xcb_void_cookie_t;

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
pub fn xcb_shm_put_image_checked (c : *mut ffi::base::xcb_connection_t,
                                     drawable :  ffi::xproto::xcb_drawable_t,
                                     gc :  ffi::xproto::xcb_gcontext_t,
                                     total_width :  u16,
                                     total_height :  u16,
                                     src_x :  u16,
                                     src_y :  u16,
                                     src_width :  u16,
                                     src_height :  u16,
                                     dst_x :  i16,
                                     dst_y :  i16,
                                     depth :  u8,
                                     format :  u8,
                                     send_event :  u8,
                                     shmseg :  xcb_shm_seg_t,
                                     offset :  u32) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_shm_put_image (c : *mut ffi::base::xcb_connection_t,
                             drawable :  ffi::xproto::xcb_drawable_t,
                             gc :  ffi::xproto::xcb_gcontext_t,
                             total_width :  u16,
                             total_height :  u16,
                             src_x :  u16,
                             src_y :  u16,
                             src_width :  u16,
                             src_height :  u16,
                             dst_x :  i16,
                             dst_y :  i16,
                             depth :  u8,
                             format :  u8,
                             send_event :  u8,
                             shmseg :  xcb_shm_seg_t,
                             offset :  u32) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_shm_get_image (c : *mut ffi::base::xcb_connection_t,
                             drawable :  ffi::xproto::xcb_drawable_t,
                             x :  i16,
                             y :  i16,
                             width :  u16,
                             height :  u16,
                             plane_mask :  u32,
                             format :  u8,
                             shmseg :  xcb_shm_seg_t,
                             offset :  u32) -> xcb_shm_get_image_cookie_t;

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
pub fn xcb_shm_get_image_unchecked (c : *mut ffi::base::xcb_connection_t,
                                       drawable :  ffi::xproto::xcb_drawable_t,
                                       x :  i16,
                                       y :  i16,
                                       width :  u16,
                                       height :  u16,
                                       plane_mask :  u32,
                                       format :  u8,
                                       shmseg :  xcb_shm_seg_t,
                                       offset :  u32) -> xcb_shm_get_image_cookie_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_shm_get_image_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_shm_get_image_reply (c : *mut ffi::base::xcb_connection_t,
                                   cookie : xcb_shm_get_image_cookie_t,
                                   e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_shm_get_image_reply_t;

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
pub fn xcb_shm_create_pixmap_checked (c : *mut ffi::base::xcb_connection_t,
                                         pid :  ffi::xproto::xcb_pixmap_t,
                                         drawable :  ffi::xproto::xcb_drawable_t,
                                         width :  u16,
                                         height :  u16,
                                         depth :  u8,
                                         shmseg :  xcb_shm_seg_t,
                                         offset :  u32) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_shm_create_pixmap (c : *mut ffi::base::xcb_connection_t,
                                 pid :  ffi::xproto::xcb_pixmap_t,
                                 drawable :  ffi::xproto::xcb_drawable_t,
                                 width :  u16,
                                 height :  u16,
                                 depth :  u8,
                                 shmseg :  xcb_shm_seg_t,
                                 offset :  u32) -> ffi::base::xcb_void_cookie_t;
}

