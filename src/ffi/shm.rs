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

pub static SHM_MAJOR_VERSION : c_uint = 1;
pub static SHM_MINOR_VERSION : c_uint = 1;

pub type seg = u32;
/**
 * @brief seg_iterator
 **/
pub struct seg_iterator {
    pub data : *mut seg,
    pub rem  : c_int,
    pub index: c_int
}



pub struct completion_event {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub drawable :        ffi::xproto::drawable,
     pub minor_event :     u16,
     pub major_event :     u8,
     pub pad1 :            u8,
     pub shmseg :          seg,
     pub offset :          u32
}



pub type bad_seg_error  = ffi::xproto::value_error;

pub struct query_version_cookie {
    sequence : c_uint
}


pub struct query_version_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16
}


pub struct query_version_reply {
     pub response_type :    u8,
     pub shared_pixmaps :   u8,
     pub sequence :         u16,
     pub length :           u32,
     pub major_version :    u16,
     pub minor_version :    u16,
     pub uid :              u16,
     pub gid :              u16,
     pub pixmap_format :    u8,
     pub pad0 :             [u8,..15]
}



pub struct attach_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub shmseg :         seg,
     pub shmid :          u32,
     pub read_only :      u8,
     pub pad0 :           [u8,..3]
}



pub struct detach_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub shmseg :         seg
}



pub struct put_image_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub drawable :       ffi::xproto::drawable,
     pub gc :             ffi::xproto::gcontext,
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
     pub shmseg :         seg,
     pub offset :         u32
}


pub struct get_image_cookie {
    sequence : c_uint
}


pub struct get_image_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub drawable :       ffi::xproto::drawable,
     pub x :              i16,
     pub y :              i16,
     pub width :          u16,
     pub height :         u16,
     pub plane_mask :     u32,
     pub format :         u8,
     pub pad0 :           [u8,..3],
     pub shmseg :         seg,
     pub offset :         u32
}


pub struct get_image_reply {
     pub response_type :   u8,
     pub depth :           u8,
     pub sequence :        u16,
     pub length :          u32,
     pub visual :          ffi::xproto::visualid,
     pub size :            u32
}



pub struct create_pixmap_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub pid :            ffi::xproto::pixmap,
     pub drawable :       ffi::xproto::drawable,
     pub width :          u16,
     pub height :         u16,
     pub depth :          u8,
     pub pad0 :           [u8,..3],
     pub shmseg :         seg,
     pub offset :         u32
}

#[link(name="lxcb-shm")]
extern "C" {

/**
 * Get the next element of the iterator
 * @param i Pointer to a seg_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(seg)
 *
 *
 */
pub fn xcb_shm_seg_next (i:*mut seg_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An seg_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_shm_seg_end (i:seg_iterator) -> ffi::base::generic_iterator;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_shm_query_version (c : *mut ffi::base::connection) -> query_version_cookie;

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
pub fn xcb_shm_query_version_unchecked (c : *mut ffi::base::connection) -> query_version_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_shm_query_version_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_shm_query_version_reply (c : *mut ffi::base::connection,
                                       cookie : query_version_cookie,
                                       e : *mut *mut ffi::base::generic_error) -> *mut query_version_reply;

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
pub fn xcb_shm_attach_checked (c : *mut ffi::base::connection,
                                  shmseg :  seg,
                                  shmid :  u32,
                                  read_only :  u8) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_shm_attach (c : *mut ffi::base::connection,
                          shmseg :  seg,
                          shmid :  u32,
                          read_only :  u8) -> ffi::base::void_cookie;

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
pub fn xcb_shm_detach_checked (c : *mut ffi::base::connection,
                                  shmseg :  seg) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_shm_detach (c : *mut ffi::base::connection,
                          shmseg :  seg) -> ffi::base::void_cookie;

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
pub fn xcb_shm_put_image_checked (c : *mut ffi::base::connection,
                                     drawable :  ffi::xproto::drawable,
                                     gc :  ffi::xproto::gcontext,
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
                                     shmseg :  seg,
                                     offset :  u32) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_shm_put_image (c : *mut ffi::base::connection,
                             drawable :  ffi::xproto::drawable,
                             gc :  ffi::xproto::gcontext,
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
                             shmseg :  seg,
                             offset :  u32) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_shm_get_image (c : *mut ffi::base::connection,
                             drawable :  ffi::xproto::drawable,
                             x :  i16,
                             y :  i16,
                             width :  u16,
                             height :  u16,
                             plane_mask :  u32,
                             format :  u8,
                             shmseg :  seg,
                             offset :  u32) -> get_image_cookie;

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
pub fn xcb_shm_get_image_unchecked (c : *mut ffi::base::connection,
                                       drawable :  ffi::xproto::drawable,
                                       x :  i16,
                                       y :  i16,
                                       width :  u16,
                                       height :  u16,
                                       plane_mask :  u32,
                                       format :  u8,
                                       shmseg :  seg,
                                       offset :  u32) -> get_image_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_shm_get_image_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_shm_get_image_reply (c : *mut ffi::base::connection,
                                   cookie : get_image_cookie,
                                   e : *mut *mut ffi::base::generic_error) -> *mut get_image_reply;

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
pub fn xcb_shm_create_pixmap_checked (c : *mut ffi::base::connection,
                                         pid :  ffi::xproto::pixmap,
                                         drawable :  ffi::xproto::drawable,
                                         width :  u16,
                                         height :  u16,
                                         depth :  u8,
                                         shmseg :  seg,
                                         offset :  u32) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_shm_create_pixmap (c : *mut ffi::base::connection,
                                 pid :  ffi::xproto::pixmap,
                                 drawable :  ffi::xproto::drawable,
                                 width :  u16,
                                 height :  u16,
                                 depth :  u8,
                                 shmseg :  seg,
                                 offset :  u32) -> ffi::base::void_cookie;
}

