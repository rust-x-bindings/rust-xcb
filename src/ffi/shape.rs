/*
 * This file generated automatically from shape.xml by r_client.py.
 * Edit at your peril.
 */

//Make the compiler quiet
#[allow(unused_imports)];
#[allow(non_camel_case_types)];
use std;
use std::libc::*;
use std::{cast,num,ptr,str,libc};
use std::to_bytes::ToBytes;
use ll::base::*;
use ll;
use ll::xproto;

pub static SHAPE_MAJOR_VERSION : c_uint = 1;
pub static SHAPE_MINOR_VERSION : c_uint = 1;

pub type op = u8;
/**
 * @brief op_iterator
 **/
pub struct op_iterator {
    data : *op,
    rem  : c_int,
    index: c_int
}


pub type kind = u8;
/**
 * @brief kind_iterator
 **/
pub struct kind_iterator {
    data : *kind,
    rem  : c_int,
    index: c_int
}



pub struct notify_event {
    response_type :     u8,
    shape_kind :        kind,
    sequence :          u16,
    affected_window :   ll::xproto::window,
    extents_x :         i16,
    extents_y :         i16,
    extents_width :     u16,
    extents_height :    u16,
    server_time :       ll::xproto::timestamp,
    shaped :            u8,
    pad0 :              [u8,..11]
}


pub struct query_version_cookie {
    sequence : c_uint
}


pub struct query_version_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16
}


pub struct query_version_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    major_version :   u16,
    minor_version :   u16
}



pub struct rectangles_request {
    major_opcode :         u8,
    minor_opcode :         u8,
    length :               u16,
    operation :            op,
    destination_kind :     kind,
    ordering :             u8,
    pad0 :                 u8,
    destination_window :   ll::xproto::window,
    x_offset :             i16,
    y_offset :             i16
}



pub struct mask_request {
    major_opcode :         u8,
    minor_opcode :         u8,
    length :               u16,
    operation :            op,
    destination_kind :     kind,
    pad0 :                 [u8,..2],
    destination_window :   ll::xproto::window,
    x_offset :             i16,
    y_offset :             i16,
    source_bitmap :        ll::xproto::pixmap
}



pub struct combine_request {
    major_opcode :         u8,
    minor_opcode :         u8,
    length :               u16,
    operation :            op,
    destination_kind :     kind,
    source_kind :          kind,
    pad0 :                 u8,
    destination_window :   ll::xproto::window,
    x_offset :             i16,
    y_offset :             i16,
    source_window :        ll::xproto::window
}



pub struct offset_request {
    major_opcode :         u8,
    minor_opcode :         u8,
    length :               u16,
    destination_kind :     kind,
    pad0 :                 [u8,..3],
    destination_window :   ll::xproto::window,
    x_offset :             i16,
    y_offset :             i16
}


pub struct query_extents_cookie {
    sequence : c_uint
}


pub struct query_extents_request {
    major_opcode :         u8,
    minor_opcode :         u8,
    length :               u16,
    destination_window :   ll::xproto::window
}


pub struct query_extents_reply {
    response_type :                   u8,
    pad0 :                            u8,
    sequence :                        u16,
    length :                          u32,
    bounding_shaped :                 u8,
    clip_shaped :                     u8,
    pad1 :                            [u8,..2],
    bounding_shape_extents_x :        i16,
    bounding_shape_extents_y :        i16,
    bounding_shape_extents_width :    u16,
    bounding_shape_extents_height :   u16,
    clip_shape_extents_x :            i16,
    clip_shape_extents_y :            i16,
    clip_shape_extents_width :        u16,
    clip_shape_extents_height :       u16
}



pub struct select_input_request {
    major_opcode :         u8,
    minor_opcode :         u8,
    length :               u16,
    destination_window :   ll::xproto::window,
    enable :               u8,
    pad0 :                 [u8,..3]
}


pub struct input_selected_cookie {
    sequence : c_uint
}


pub struct input_selected_request {
    major_opcode :         u8,
    minor_opcode :         u8,
    length :               u16,
    destination_window :   ll::xproto::window
}


pub struct input_selected_reply {
    response_type :   u8,
    enabled :         u8,
    sequence :        u16,
    length :          u32
}


pub struct get_rectangles_cookie {
    sequence : c_uint
}


pub struct get_rectangles_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    window :         ll::xproto::window,
    source_kind :    kind,
    pad0 :           [u8,..3]
}


pub struct get_rectangles_reply {
    response_type :    u8,
    ordering :         u8,
    sequence :         u16,
    length :           u32,
    rectangles_len :   u32,
    pad0 :             [u8,..20]
}

#[link_args="-lxcb-shape"]
pub extern "C" {

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
pub unsafe fn xcb_shape_op_next (i:*op_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An op_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub unsafe fn xcb_shape_op_end (i:op_iterator) -> generic_iterator;

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
pub unsafe fn xcb_shape_kind_next (i:*kind_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An kind_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub unsafe fn xcb_shape_kind_end (i:kind_iterator) -> generic_iterator;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_shape_query_version (c : *connection) -> query_version_cookie;

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
pub unsafe fn xcb_shape_query_version_unchecked (c : *connection) -> query_version_cookie;

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
pub unsafe fn xcb_shape_query_version_reply (c : *connection,
                                         cookie : query_version_cookie,
                                         e : **generic_error) -> *query_version_reply;

pub unsafe fn xcb_shape_rectangles_sizeof (_buffer :  *c_void,
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
pub unsafe fn xcb_shape_rectangles_checked (c : *connection,
                                        operation :  op,
                                        destination_kind :  kind,
                                        ordering :  u8,
                                        destination_window :  ll::xproto::window,
                                        x_offset :  i16,
                                        y_offset :  i16,
                                        rectangles_len :  u32,
                                        rectangles : *ll::xproto::rectangle) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_shape_rectangles (c : *connection,
                                operation :  op,
                                destination_kind :  kind,
                                ordering :  u8,
                                destination_window :  ll::xproto::window,
                                x_offset :  i16,
                                y_offset :  i16,
                                rectangles_len :  u32,
                                rectangles : *ll::xproto::rectangle) -> void_cookie;

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
pub unsafe fn xcb_shape_mask_checked (c : *connection,
                                  operation :  op,
                                  destination_kind :  kind,
                                  destination_window :  ll::xproto::window,
                                  x_offset :  i16,
                                  y_offset :  i16,
                                  source_bitmap :  ll::xproto::pixmap) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_shape_mask (c : *connection,
                          operation :  op,
                          destination_kind :  kind,
                          destination_window :  ll::xproto::window,
                          x_offset :  i16,
                          y_offset :  i16,
                          source_bitmap :  ll::xproto::pixmap) -> void_cookie;

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
pub unsafe fn xcb_shape_combine_checked (c : *connection,
                                     operation :  op,
                                     destination_kind :  kind,
                                     source_kind :  kind,
                                     destination_window :  ll::xproto::window,
                                     x_offset :  i16,
                                     y_offset :  i16,
                                     source_window :  ll::xproto::window) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_shape_combine (c : *connection,
                             operation :  op,
                             destination_kind :  kind,
                             source_kind :  kind,
                             destination_window :  ll::xproto::window,
                             x_offset :  i16,
                             y_offset :  i16,
                             source_window :  ll::xproto::window) -> void_cookie;

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
pub unsafe fn xcb_shape_offset_checked (c : *connection,
                                    destination_kind :  kind,
                                    destination_window :  ll::xproto::window,
                                    x_offset :  i16,
                                    y_offset :  i16) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_shape_offset (c : *connection,
                            destination_kind :  kind,
                            destination_window :  ll::xproto::window,
                            x_offset :  i16,
                            y_offset :  i16) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_shape_query_extents (c : *connection,
                                   destination_window :  ll::xproto::window) -> query_extents_cookie;

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
pub unsafe fn xcb_shape_query_extents_unchecked (c : *connection,
                                             destination_window :  ll::xproto::window) -> query_extents_cookie;

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
pub unsafe fn xcb_shape_query_extents_reply (c : *connection,
                                         cookie : query_extents_cookie,
                                         e : **generic_error) -> *query_extents_reply;

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
pub unsafe fn xcb_shape_select_input_checked (c : *connection,
                                          destination_window :  ll::xproto::window,
                                          enable :  u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_shape_select_input (c : *connection,
                                  destination_window :  ll::xproto::window,
                                  enable :  u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_shape_input_selected (c : *connection,
                                    destination_window :  ll::xproto::window) -> input_selected_cookie;

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
pub unsafe fn xcb_shape_input_selected_unchecked (c : *connection,
                                              destination_window :  ll::xproto::window) -> input_selected_cookie;

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
pub unsafe fn xcb_shape_input_selected_reply (c : *connection,
                                          cookie : input_selected_cookie,
                                          e : **generic_error) -> *input_selected_reply;

pub unsafe fn xcb_shape_get_rectangles_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_shape_get_rectangles (c : *connection,
                                    window :  ll::xproto::window,
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
pub unsafe fn xcb_shape_get_rectangles_unchecked (c : *connection,
                                              window :  ll::xproto::window,
                                              source_kind :  kind) -> get_rectangles_cookie;

pub unsafe fn xcb_shape_get_rectangles_rectangles (R : *get_rectangles_reply) -> *ll::xproto::rectangle;


pub unsafe fn xcb_shape_get_rectangles_rectangles_length (R : *get_rectangles_reply) -> c_int;

pub unsafe fn xcb_shape_get_rectangles_rectangles_iterator (R : *get_rectangles_reply) -> ll::xproto::rectangle_iterator;

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
pub unsafe fn xcb_shape_get_rectangles_reply (c : *connection,
                                          cookie : get_rectangles_cookie,
                                          e : **generic_error) -> *get_rectangles_reply;
}

