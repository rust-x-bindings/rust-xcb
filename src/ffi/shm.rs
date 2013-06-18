/*
 * This file generated automatically from shm.xml by r_client.py.
 * Edit at your peril.
 */

//Make the compiler quiet
#[allow(unused_imports)];
#[allow(non_camel_case_types)];
use std;
use std::libc::*;
use std::{cast,num,ptr,str,libc};
use std::to_bytes::ToBytes;
use ffi::base::*;
use ffi;
use ffi::xproto;

pub static SHM_MAJOR_VERSION : c_uint = 1;
pub static SHM_MINOR_VERSION : c_uint = 1;

pub type seg = u32;
/**
 * @brief seg_iterator
 **/
pub struct seg_iterator {
    data : *seg,
    rem  : c_int,
    index: c_int
}



pub struct completion_event {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    drawable :        ffi::xproto::drawable,
    minor_event :     u16,
    major_event :     u8,
    pad1 :            u8,
    shmseg :          seg,
    offset :          u32
}



pub type bad_seg_error  = ffi::xproto::value_error;

pub struct query_version_cookie {
    sequence : c_uint
}


pub struct query_version_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16
}


pub struct query_version_reply {
    response_type :    u8,
    shared_pixmaps :   u8,
    sequence :         u16,
    length :           u32,
    major_version :    u16,
    minor_version :    u16,
    uid :              u16,
    gid :              u16,
    pixmap_format :    u8,
    pad0 :             [u8,..15]
}



pub struct attach_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    shmseg :         seg,
    shmid :          u32,
    read_only :      u8,
    pad0 :           [u8,..3]
}



pub struct detach_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    shmseg :         seg
}



pub struct put_image_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    drawable :       ffi::xproto::drawable,
    gc :             ffi::xproto::gcontext,
    total_width :    u16,
    total_height :   u16,
    src_x :          u16,
    src_y :          u16,
    src_width :      u16,
    src_height :     u16,
    dst_x :          i16,
    dst_y :          i16,
    depth :          u8,
    format :         u8,
    send_event :     u8,
    pad0 :           u8,
    shmseg :         seg,
    offset :         u32
}


pub struct get_image_cookie {
    sequence : c_uint
}


pub struct get_image_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    drawable :       ffi::xproto::drawable,
    x :              i16,
    y :              i16,
    width :          u16,
    height :         u16,
    plane_mask :     u32,
    format :         u8,
    pad0 :           [u8,..3],
    shmseg :         seg,
    offset :         u32
}


pub struct get_image_reply {
    response_type :   u8,
    depth :           u8,
    sequence :        u16,
    length :          u32,
    visual :          ffi::xproto::visualid,
    size :            u32
}



pub struct create_pixmap_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    pid :            ffi::xproto::pixmap,
    drawable :       ffi::xproto::drawable,
    width :          u16,
    height :         u16,
    depth :          u8,
    pad0 :           [u8,..3],
    shmseg :         seg,
    offset :         u32
}

#[link_args="-lxcb-shm"]
pub extern "C" {

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
pub unsafe fn xcb_shm_seg_next (i:*seg_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An seg_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub unsafe fn xcb_shm_seg_end (i:seg_iterator) -> generic_iterator;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_shm_query_version (c : *connection) -> query_version_cookie;

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
pub unsafe fn xcb_shm_query_version_unchecked (c : *connection) -> query_version_cookie;

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
pub unsafe fn xcb_shm_query_version_reply (c : *connection,
                                       cookie : query_version_cookie,
                                       e : **generic_error) -> *query_version_reply;

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
pub unsafe fn xcb_shm_attach_checked (c : *connection,
                                  shmseg :  seg,
                                  shmid :  u32,
                                  read_only :  u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_shm_attach (c : *connection,
                          shmseg :  seg,
                          shmid :  u32,
                          read_only :  u8) -> void_cookie;

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
pub unsafe fn xcb_shm_detach_checked (c : *connection,
                                  shmseg :  seg) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_shm_detach (c : *connection,
                          shmseg :  seg) -> void_cookie;

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
pub unsafe fn xcb_shm_put_image_checked (c : *connection,
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
                                     offset :  u32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_shm_put_image (c : *connection,
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
                             offset :  u32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_shm_get_image (c : *connection,
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
pub unsafe fn xcb_shm_get_image_unchecked (c : *connection,
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
pub unsafe fn xcb_shm_get_image_reply (c : *connection,
                                   cookie : get_image_cookie,
                                   e : **generic_error) -> *get_image_reply;

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
pub unsafe fn xcb_shm_create_pixmap_checked (c : *connection,
                                         pid :  ffi::xproto::pixmap,
                                         drawable :  ffi::xproto::drawable,
                                         width :  u16,
                                         height :  u16,
                                         depth :  u8,
                                         shmseg :  seg,
                                         offset :  u32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_shm_create_pixmap (c : *connection,
                                 pid :  ffi::xproto::pixmap,
                                 drawable :  ffi::xproto::drawable,
                                 width :  u16,
                                 height :  u16,
                                 depth :  u8,
                                 shmseg :  seg,
                                 offset :  u32) -> void_cookie;
}

