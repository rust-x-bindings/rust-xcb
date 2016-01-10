//
// This file generated automatically from shape.xml by r_client.py.
// Edit at your peril.
//

//Make the compiler quiet
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use std;
use libc::*;
use ffi;
use ffi::xproto;

pub const SHAPE_MAJOR_VERSION : c_uint = 1;
pub const SHAPE_MINOR_VERSION : c_uint = 1;

pub type xcb_shape_op_t = u8;
#[repr(C)]
pub struct xcb_shape_op_iterator_t {
    pub data : *mut xcb_shape_op_t,
    pub rem  : c_int,
    pub index: c_int
}


pub type xcb_shape_kind_t = u8;
#[repr(C)]
pub struct xcb_shape_kind_iterator_t {
    pub data : *mut xcb_shape_kind_t,
    pub rem  : c_int,
    pub index: c_int
}



#[repr(C)]
pub struct xcb_shape_notify_event_t {
     pub response_type :     u8,
     pub shape_kind :        xcb_shape_kind_t,
     pub sequence :          u16,
     pub affected_window :   ffi::xproto::xcb_window_t,
     pub extents_x :         i16,
     pub extents_y :         i16,
     pub extents_width :     u16,
     pub extents_height :    u16,
     pub server_time :       ffi::xproto::xcb_timestamp_t,
     pub shaped :            u8,
     pub pad0 :              [u8; 11]
}

impl Copy for xcb_shape_notify_event_t {}
impl Clone for xcb_shape_notify_event_t {
    fn clone(&self) -> xcb_shape_notify_event_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_shape_query_version_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_shape_query_version_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16
}

impl Copy for xcb_shape_query_version_request_t {}
impl Clone for xcb_shape_query_version_request_t {
    fn clone(&self) -> xcb_shape_query_version_request_t { *self }
}

#[repr(C)]
pub struct xcb_shape_query_version_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub major_version :   u16,
     pub minor_version :   u16
}

impl Copy for xcb_shape_query_version_reply_t {}
impl Clone for xcb_shape_query_version_reply_t {
    fn clone(&self) -> xcb_shape_query_version_reply_t { *self }
}


#[repr(C)]
pub struct xcb_shape_rectangles_request_t {
     pub major_opcode :         u8,
     pub minor_opcode :         u8,
     pub length :               u16,
     pub operation :            xcb_shape_op_t,
     pub destination_kind :     xcb_shape_kind_t,
     pub ordering :             u8,
     pub pad0 :                 u8,
     pub destination_window :   ffi::xproto::xcb_window_t,
     pub x_offset :             i16,
     pub y_offset :             i16
}

impl Copy for xcb_shape_rectangles_request_t {}
impl Clone for xcb_shape_rectangles_request_t {
    fn clone(&self) -> xcb_shape_rectangles_request_t { *self }
}


#[repr(C)]
pub struct xcb_shape_mask_request_t {
     pub major_opcode :         u8,
     pub minor_opcode :         u8,
     pub length :               u16,
     pub operation :            xcb_shape_op_t,
     pub destination_kind :     xcb_shape_kind_t,
     pub pad0 :                 [u8; 2],
     pub destination_window :   ffi::xproto::xcb_window_t,
     pub x_offset :             i16,
     pub y_offset :             i16,
     pub source_bitmap :        ffi::xproto::xcb_pixmap_t
}

impl Copy for xcb_shape_mask_request_t {}
impl Clone for xcb_shape_mask_request_t {
    fn clone(&self) -> xcb_shape_mask_request_t { *self }
}


#[repr(C)]
pub struct xcb_shape_combine_request_t {
     pub major_opcode :         u8,
     pub minor_opcode :         u8,
     pub length :               u16,
     pub operation :            xcb_shape_op_t,
     pub destination_kind :     xcb_shape_kind_t,
     pub source_kind :          xcb_shape_kind_t,
     pub pad0 :                 u8,
     pub destination_window :   ffi::xproto::xcb_window_t,
     pub x_offset :             i16,
     pub y_offset :             i16,
     pub source_window :        ffi::xproto::xcb_window_t
}

impl Copy for xcb_shape_combine_request_t {}
impl Clone for xcb_shape_combine_request_t {
    fn clone(&self) -> xcb_shape_combine_request_t { *self }
}


#[repr(C)]
pub struct xcb_shape_offset_request_t {
     pub major_opcode :         u8,
     pub minor_opcode :         u8,
     pub length :               u16,
     pub destination_kind :     xcb_shape_kind_t,
     pub pad0 :                 [u8; 3],
     pub destination_window :   ffi::xproto::xcb_window_t,
     pub x_offset :             i16,
     pub y_offset :             i16
}

impl Copy for xcb_shape_offset_request_t {}
impl Clone for xcb_shape_offset_request_t {
    fn clone(&self) -> xcb_shape_offset_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_shape_query_extents_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_shape_query_extents_request_t {
     pub major_opcode :         u8,
     pub minor_opcode :         u8,
     pub length :               u16,
     pub destination_window :   ffi::xproto::xcb_window_t
}

impl Copy for xcb_shape_query_extents_request_t {}
impl Clone for xcb_shape_query_extents_request_t {
    fn clone(&self) -> xcb_shape_query_extents_request_t { *self }
}

#[repr(C)]
pub struct xcb_shape_query_extents_reply_t {
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

impl Copy for xcb_shape_query_extents_reply_t {}
impl Clone for xcb_shape_query_extents_reply_t {
    fn clone(&self) -> xcb_shape_query_extents_reply_t { *self }
}


#[repr(C)]
pub struct xcb_shape_select_input_request_t {
     pub major_opcode :         u8,
     pub minor_opcode :         u8,
     pub length :               u16,
     pub destination_window :   ffi::xproto::xcb_window_t,
     pub enable :               u8,
     pub pad0 :                 [u8; 3]
}

impl Copy for xcb_shape_select_input_request_t {}
impl Clone for xcb_shape_select_input_request_t {
    fn clone(&self) -> xcb_shape_select_input_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_shape_input_selected_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_shape_input_selected_request_t {
     pub major_opcode :         u8,
     pub minor_opcode :         u8,
     pub length :               u16,
     pub destination_window :   ffi::xproto::xcb_window_t
}

impl Copy for xcb_shape_input_selected_request_t {}
impl Clone for xcb_shape_input_selected_request_t {
    fn clone(&self) -> xcb_shape_input_selected_request_t { *self }
}

#[repr(C)]
pub struct xcb_shape_input_selected_reply_t {
     pub response_type :   u8,
     pub enabled :         u8,
     pub sequence :        u16,
     pub length :          u32
}

impl Copy for xcb_shape_input_selected_reply_t {}
impl Clone for xcb_shape_input_selected_reply_t {
    fn clone(&self) -> xcb_shape_input_selected_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_shape_get_rectangles_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_shape_get_rectangles_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub window :         ffi::xproto::xcb_window_t,
     pub source_kind :    xcb_shape_kind_t,
     pub pad0 :           [u8; 3]
}

impl Copy for xcb_shape_get_rectangles_request_t {}
impl Clone for xcb_shape_get_rectangles_request_t {
    fn clone(&self) -> xcb_shape_get_rectangles_request_t { *self }
}

#[repr(C)]
pub struct xcb_shape_get_rectangles_reply_t {
     pub response_type :    u8,
     pub ordering :         u8,
     pub sequence :         u16,
     pub length :           u32,
     pub rectangles_len :   u32,
     pub pad0 :             [u8; 20]
}

impl Copy for xcb_shape_get_rectangles_reply_t {}
impl Clone for xcb_shape_get_rectangles_reply_t {
    fn clone(&self) -> xcb_shape_get_rectangles_reply_t { *self }
}
#[link(name="xcb-shape")]
extern "C" {

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_shape_op_t)
///
pub fn xcb_shape_op_next (i:*mut xcb_shape_op_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_shape_op_end (i:xcb_shape_op_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_shape_kind_t)
///
pub fn xcb_shape_kind_next (i:*mut xcb_shape_kind_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_shape_kind_end (i:xcb_shape_kind_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/// Delivers a request to the X server.
///
pub fn xcb_shape_query_version (c : *mut ffi::base::xcb_connection_t) -> xcb_shape_query_version_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_shape_query_version_unchecked (c : *mut ffi::base::xcb_connection_t) -> xcb_shape_query_version_cookie_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_shape_query_version_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_shape_query_version_reply (c : *mut ffi::base::xcb_connection_t,
                                         cookie : xcb_shape_query_version_cookie_t,
                                         e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_shape_query_version_reply_t;

pub fn xcb_shape_rectangles_sizeof (_buffer :  *mut c_void,
                             rectangles_len :  u32) -> c_int;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_shape_rectangles_checked (c : *mut ffi::base::xcb_connection_t,
                                        operation :  xcb_shape_op_t,
                                        destination_kind :  xcb_shape_kind_t,
                                        ordering :  u8,
                                        destination_window :  ffi::xproto::xcb_window_t,
                                        x_offset :  i16,
                                        y_offset :  i16,
                                        rectangles_len :  u32,
                                        rectangles : *mut ffi::xproto::xcb_rectangle_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_shape_rectangles (c : *mut ffi::base::xcb_connection_t,
                                operation :  xcb_shape_op_t,
                                destination_kind :  xcb_shape_kind_t,
                                ordering :  u8,
                                destination_window :  ffi::xproto::xcb_window_t,
                                x_offset :  i16,
                                y_offset :  i16,
                                rectangles_len :  u32,
                                rectangles : *mut ffi::xproto::xcb_rectangle_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_shape_mask_checked (c : *mut ffi::base::xcb_connection_t,
                                  operation :  xcb_shape_op_t,
                                  destination_kind :  xcb_shape_kind_t,
                                  destination_window :  ffi::xproto::xcb_window_t,
                                  x_offset :  i16,
                                  y_offset :  i16,
                                  source_bitmap :  ffi::xproto::xcb_pixmap_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_shape_mask (c : *mut ffi::base::xcb_connection_t,
                          operation :  xcb_shape_op_t,
                          destination_kind :  xcb_shape_kind_t,
                          destination_window :  ffi::xproto::xcb_window_t,
                          x_offset :  i16,
                          y_offset :  i16,
                          source_bitmap :  ffi::xproto::xcb_pixmap_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_shape_combine_checked (c : *mut ffi::base::xcb_connection_t,
                                     operation :  xcb_shape_op_t,
                                     destination_kind :  xcb_shape_kind_t,
                                     source_kind :  xcb_shape_kind_t,
                                     destination_window :  ffi::xproto::xcb_window_t,
                                     x_offset :  i16,
                                     y_offset :  i16,
                                     source_window :  ffi::xproto::xcb_window_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_shape_combine (c : *mut ffi::base::xcb_connection_t,
                             operation :  xcb_shape_op_t,
                             destination_kind :  xcb_shape_kind_t,
                             source_kind :  xcb_shape_kind_t,
                             destination_window :  ffi::xproto::xcb_window_t,
                             x_offset :  i16,
                             y_offset :  i16,
                             source_window :  ffi::xproto::xcb_window_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_shape_offset_checked (c : *mut ffi::base::xcb_connection_t,
                                    destination_kind :  xcb_shape_kind_t,
                                    destination_window :  ffi::xproto::xcb_window_t,
                                    x_offset :  i16,
                                    y_offset :  i16) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_shape_offset (c : *mut ffi::base::xcb_connection_t,
                            destination_kind :  xcb_shape_kind_t,
                            destination_window :  ffi::xproto::xcb_window_t,
                            x_offset :  i16,
                            y_offset :  i16) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_shape_query_extents (c : *mut ffi::base::xcb_connection_t,
                                   destination_window :  ffi::xproto::xcb_window_t) -> xcb_shape_query_extents_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_shape_query_extents_unchecked (c : *mut ffi::base::xcb_connection_t,
                                             destination_window :  ffi::xproto::xcb_window_t) -> xcb_shape_query_extents_cookie_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_shape_query_extents_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_shape_query_extents_reply (c : *mut ffi::base::xcb_connection_t,
                                         cookie : xcb_shape_query_extents_cookie_t,
                                         e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_shape_query_extents_reply_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_shape_select_input_checked (c : *mut ffi::base::xcb_connection_t,
                                          destination_window :  ffi::xproto::xcb_window_t,
                                          enable :  u8) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_shape_select_input (c : *mut ffi::base::xcb_connection_t,
                                  destination_window :  ffi::xproto::xcb_window_t,
                                  enable :  u8) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_shape_input_selected (c : *mut ffi::base::xcb_connection_t,
                                    destination_window :  ffi::xproto::xcb_window_t) -> xcb_shape_input_selected_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_shape_input_selected_unchecked (c : *mut ffi::base::xcb_connection_t,
                                              destination_window :  ffi::xproto::xcb_window_t) -> xcb_shape_input_selected_cookie_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_shape_input_selected_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_shape_input_selected_reply (c : *mut ffi::base::xcb_connection_t,
                                          cookie : xcb_shape_input_selected_cookie_t,
                                          e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_shape_input_selected_reply_t;

pub fn xcb_shape_get_rectangles_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
///
pub fn xcb_shape_get_rectangles (c : *mut ffi::base::xcb_connection_t,
                                    window :  ffi::xproto::xcb_window_t,
                                    source_kind :  xcb_shape_kind_t) -> xcb_shape_get_rectangles_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_shape_get_rectangles_unchecked (c : *mut ffi::base::xcb_connection_t,
                                              window :  ffi::xproto::xcb_window_t,
                                              source_kind :  xcb_shape_kind_t) -> xcb_shape_get_rectangles_cookie_t;

pub fn xcb_shape_get_rectangles_rectangles (R : *mut xcb_shape_get_rectangles_reply_t) -> *mut ffi::xproto::xcb_rectangle_t;


pub fn xcb_shape_get_rectangles_rectangles_length (R : *mut xcb_shape_get_rectangles_reply_t) -> c_int;

pub fn xcb_shape_get_rectangles_rectangles_iterator (R : *mut xcb_shape_get_rectangles_reply_t) -> ffi::xproto::xcb_rectangle_iterator_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_shape_get_rectangles_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_shape_get_rectangles_reply (c : *mut ffi::base::xcb_connection_t,
                                          cookie : xcb_shape_get_rectangles_cookie_t,
                                          e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_shape_get_rectangles_reply_t;
}

