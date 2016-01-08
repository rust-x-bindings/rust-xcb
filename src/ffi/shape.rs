/*
 * This file generated automatically from shape.xml by r_client.py.
 * Edit at your peril.
 */

//Make the compiler quiet
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use std;
use libc::*;
use ffi;
use ffi::xproto;

pub static SHAPE_MAJOR_VERSION : c_uint = 1;
pub static SHAPE_MINOR_VERSION : c_uint = 1;

pub type op = u8;
/**
 * @brief op_iterator
 **/
#[repr(C)]
pub struct op_iterator {
    pub data : *mut op,
    pub rem  : c_int,
    pub index: c_int
}


pub type kind = u8;
/**
 * @brief kind_iterator
 **/
#[repr(C)]
pub struct kind_iterator {
    pub data : *mut kind,
    pub rem  : c_int,
    pub index: c_int
}



#[repr(C)]
pub struct notify_event {
     pub response_type :     u8,
     pub shape_kind :        kind,
     pub sequence :          u16,
     pub affected_window :   ffi::xproto::window,
     pub extents_x :         i16,
     pub extents_y :         i16,
     pub extents_width :     u16,
     pub extents_height :    u16,
     pub server_time :       ffi::xproto::timestamp,
     pub shaped :            u8,
     pub pad0 :              [u8; 11]
}

impl Copy for notify_event {}
impl Clone for notify_event {
    fn clone(&self) -> notify_event { *self }
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
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub major_version :   u16,
     pub minor_version :   u16
}

impl Copy for query_version_reply {}
impl Clone for query_version_reply {
    fn clone(&self) -> query_version_reply { *self }
}


#[repr(C)]
pub struct rectangles_request {
     pub major_opcode :         u8,
     pub minor_opcode :         u8,
     pub length :               u16,
     pub operation :            op,
     pub destination_kind :     kind,
     pub ordering :             u8,
     pub pad0 :                 u8,
     pub destination_window :   ffi::xproto::window,
     pub x_offset :             i16,
     pub y_offset :             i16
}

impl Copy for rectangles_request {}
impl Clone for rectangles_request {
    fn clone(&self) -> rectangles_request { *self }
}


#[repr(C)]
pub struct mask_request {
     pub major_opcode :         u8,
     pub minor_opcode :         u8,
     pub length :               u16,
     pub operation :            op,
     pub destination_kind :     kind,
     pub pad0 :                 [u8; 2],
     pub destination_window :   ffi::xproto::window,
     pub x_offset :             i16,
     pub y_offset :             i16,
     pub source_bitmap :        ffi::xproto::pixmap
}

impl Copy for mask_request {}
impl Clone for mask_request {
    fn clone(&self) -> mask_request { *self }
}


#[repr(C)]
pub struct combine_request {
     pub major_opcode :         u8,
     pub minor_opcode :         u8,
     pub length :               u16,
     pub operation :            op,
     pub destination_kind :     kind,
     pub source_kind :          kind,
     pub pad0 :                 u8,
     pub destination_window :   ffi::xproto::window,
     pub x_offset :             i16,
     pub y_offset :             i16,
     pub source_window :        ffi::xproto::window
}

impl Copy for combine_request {}
impl Clone for combine_request {
    fn clone(&self) -> combine_request { *self }
}


#[repr(C)]
pub struct offset_request {
     pub major_opcode :         u8,
     pub minor_opcode :         u8,
     pub length :               u16,
     pub destination_kind :     kind,
     pub pad0 :                 [u8; 3],
     pub destination_window :   ffi::xproto::window,
     pub x_offset :             i16,
     pub y_offset :             i16
}

impl Copy for offset_request {}
impl Clone for offset_request {
    fn clone(&self) -> offset_request { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct query_extents_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct query_extents_request {
     pub major_opcode :         u8,
     pub minor_opcode :         u8,
     pub length :               u16,
     pub destination_window :   ffi::xproto::window
}

impl Copy for query_extents_request {}
impl Clone for query_extents_request {
    fn clone(&self) -> query_extents_request { *self }
}

#[repr(C)]
pub struct query_extents_reply {
     pub response_type :                   u8,
     pub pad0 :                            u8,
     pub sequence :                        u16,
     pub length :                          u32,
     pub bounding_shaped :                 u8,
     pub clip_shaped :                     u8,
     pub pad1 :                            [u8; 2],
     pub bounding_shape_extents_x :        i16,
     pub bounding_shape_extents_y :        i16,
     pub bounding_shape_extents_width :    u16,
     pub bounding_shape_extents_height :   u16,
     pub clip_shape_extents_x :            i16,
     pub clip_shape_extents_y :            i16,
     pub clip_shape_extents_width :        u16,
     pub clip_shape_extents_height :       u16
}

impl Copy for query_extents_reply {}
impl Clone for query_extents_reply {
    fn clone(&self) -> query_extents_reply { *self }
}


#[repr(C)]
pub struct select_input_request {
     pub major_opcode :         u8,
     pub minor_opcode :         u8,
     pub length :               u16,
     pub destination_window :   ffi::xproto::window,
     pub enable :               u8,
     pub pad0 :                 [u8; 3]
}

impl Copy for select_input_request {}
impl Clone for select_input_request {
    fn clone(&self) -> select_input_request { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct input_selected_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct input_selected_request {
     pub major_opcode :         u8,
     pub minor_opcode :         u8,
     pub length :               u16,
     pub destination_window :   ffi::xproto::window
}

impl Copy for input_selected_request {}
impl Clone for input_selected_request {
    fn clone(&self) -> input_selected_request { *self }
}

#[repr(C)]
pub struct input_selected_reply {
     pub response_type :   u8,
     pub enabled :         u8,
     pub sequence :        u16,
     pub length :          u32
}

impl Copy for input_selected_reply {}
impl Clone for input_selected_reply {
    fn clone(&self) -> input_selected_reply { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct get_rectangles_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct get_rectangles_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub window :         ffi::xproto::window,
     pub source_kind :    kind,
     pub pad0 :           [u8; 3]
}

impl Copy for get_rectangles_request {}
impl Clone for get_rectangles_request {
    fn clone(&self) -> get_rectangles_request { *self }
}

#[repr(C)]
pub struct get_rectangles_reply {
     pub response_type :    u8,
     pub ordering :         u8,
     pub sequence :         u16,
     pub length :           u32,
     pub rectangles_len :   u32,
     pub pad0 :             [u8; 20]
}

impl Copy for get_rectangles_reply {}
impl Clone for get_rectangles_reply {
    fn clone(&self) -> get_rectangles_reply { *self }
}
#[link(name="xcb-shape")]
extern "C" {

/**
 * Get the next element of the iterator
 * @param i Pointer to a op_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(op)
 *
 *
 */
pub fn xcb_shape_op_next (i:*mut op_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An op_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_shape_op_end (i:op_iterator) -> ffi::base::generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a kind_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(kind)
 *
 *
 */
pub fn xcb_shape_kind_next (i:*mut kind_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An kind_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_shape_kind_end (i:kind_iterator) -> ffi::base::generic_iterator;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_shape_query_version (c : *mut ffi::base::connection) -> query_version_cookie;

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
pub fn xcb_shape_query_version_unchecked (c : *mut ffi::base::connection) -> query_version_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_shape_query_version_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_shape_query_version_reply (c : *mut ffi::base::connection,
                                         cookie : query_version_cookie,
                                         e : *mut *mut ffi::base::generic_error) -> *mut query_version_reply;

pub fn xcb_shape_rectangles_sizeof (_buffer :  *mut c_void,
                             rectangles_len :  u32) -> c_int;

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
pub fn xcb_shape_rectangles_checked (c : *mut ffi::base::connection,
                                        operation :  op,
                                        destination_kind :  kind,
                                        ordering :  u8,
                                        destination_window :  ffi::xproto::window,
                                        x_offset :  i16,
                                        y_offset :  i16,
                                        rectangles_len :  u32,
                                        rectangles : *mut ffi::xproto::rectangle) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_shape_rectangles (c : *mut ffi::base::connection,
                                operation :  op,
                                destination_kind :  kind,
                                ordering :  u8,
                                destination_window :  ffi::xproto::window,
                                x_offset :  i16,
                                y_offset :  i16,
                                rectangles_len :  u32,
                                rectangles : *mut ffi::xproto::rectangle) -> ffi::base::void_cookie;

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
pub fn xcb_shape_mask_checked (c : *mut ffi::base::connection,
                                  operation :  op,
                                  destination_kind :  kind,
                                  destination_window :  ffi::xproto::window,
                                  x_offset :  i16,
                                  y_offset :  i16,
                                  source_bitmap :  ffi::xproto::pixmap) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_shape_mask (c : *mut ffi::base::connection,
                          operation :  op,
                          destination_kind :  kind,
                          destination_window :  ffi::xproto::window,
                          x_offset :  i16,
                          y_offset :  i16,
                          source_bitmap :  ffi::xproto::pixmap) -> ffi::base::void_cookie;

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
pub fn xcb_shape_combine_checked (c : *mut ffi::base::connection,
                                     operation :  op,
                                     destination_kind :  kind,
                                     source_kind :  kind,
                                     destination_window :  ffi::xproto::window,
                                     x_offset :  i16,
                                     y_offset :  i16,
                                     source_window :  ffi::xproto::window) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_shape_combine (c : *mut ffi::base::connection,
                             operation :  op,
                             destination_kind :  kind,
                             source_kind :  kind,
                             destination_window :  ffi::xproto::window,
                             x_offset :  i16,
                             y_offset :  i16,
                             source_window :  ffi::xproto::window) -> ffi::base::void_cookie;

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
pub fn xcb_shape_offset_checked (c : *mut ffi::base::connection,
                                    destination_kind :  kind,
                                    destination_window :  ffi::xproto::window,
                                    x_offset :  i16,
                                    y_offset :  i16) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_shape_offset (c : *mut ffi::base::connection,
                            destination_kind :  kind,
                            destination_window :  ffi::xproto::window,
                            x_offset :  i16,
                            y_offset :  i16) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_shape_query_extents (c : *mut ffi::base::connection,
                                   destination_window :  ffi::xproto::window) -> query_extents_cookie;

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
pub fn xcb_shape_query_extents_unchecked (c : *mut ffi::base::connection,
                                             destination_window :  ffi::xproto::window) -> query_extents_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_shape_query_extents_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_shape_query_extents_reply (c : *mut ffi::base::connection,
                                         cookie : query_extents_cookie,
                                         e : *mut *mut ffi::base::generic_error) -> *mut query_extents_reply;

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
pub fn xcb_shape_select_input_checked (c : *mut ffi::base::connection,
                                          destination_window :  ffi::xproto::window,
                                          enable :  u8) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_shape_select_input (c : *mut ffi::base::connection,
                                  destination_window :  ffi::xproto::window,
                                  enable :  u8) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_shape_input_selected (c : *mut ffi::base::connection,
                                    destination_window :  ffi::xproto::window) -> input_selected_cookie;

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
pub fn xcb_shape_input_selected_unchecked (c : *mut ffi::base::connection,
                                              destination_window :  ffi::xproto::window) -> input_selected_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_shape_input_selected_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_shape_input_selected_reply (c : *mut ffi::base::connection,
                                          cookie : input_selected_cookie,
                                          e : *mut *mut ffi::base::generic_error) -> *mut input_selected_reply;

pub fn xcb_shape_get_rectangles_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_shape_get_rectangles (c : *mut ffi::base::connection,
                                    window :  ffi::xproto::window,
                                    source_kind :  kind) -> get_rectangles_cookie;

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
pub fn xcb_shape_get_rectangles_unchecked (c : *mut ffi::base::connection,
                                              window :  ffi::xproto::window,
                                              source_kind :  kind) -> get_rectangles_cookie;

pub fn xcb_shape_get_rectangles_rectangles (R : *mut get_rectangles_reply) -> *mut ffi::xproto::rectangle;


pub fn xcb_shape_get_rectangles_rectangles_length (R : *mut get_rectangles_reply) -> c_int;

pub fn xcb_shape_get_rectangles_rectangles_iterator (R : *mut get_rectangles_reply) -> ffi::xproto::rectangle_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_shape_get_rectangles_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_shape_get_rectangles_reply (c : *mut ffi::base::connection,
                                          cookie : get_rectangles_cookie,
                                          e : *mut *mut ffi::base::generic_error) -> *mut get_rectangles_reply;
}

