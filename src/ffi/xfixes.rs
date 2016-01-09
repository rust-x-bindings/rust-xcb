/*
 * This file generated automatically from xfixes.xml by r_client.py.
 * Edit at your peril.
 */

//Make the compiler quiet
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use std;
use libc::*;
use ffi;
use ffi::xproto;
use ffi::render;
use ffi::shape;

pub const XFIXES_MAJOR_VERSION : c_uint = 5;
pub const XFIXES_MINOR_VERSION : c_uint = 0;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xfixes_query_version_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_xfixes_query_version_request_t {
     pub major_opcode :           u8,
     pub minor_opcode :           u8,
     pub length :                 u16,
     pub client_major_version :   u32,
     pub client_minor_version :   u32
}

impl Copy for xcb_xfixes_query_version_request_t {}
impl Clone for xcb_xfixes_query_version_request_t {
    fn clone(&self) -> xcb_xfixes_query_version_request_t { *self }
}

#[repr(C)]
pub struct xcb_xfixes_query_version_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub major_version :   u32,
     pub minor_version :   u32,
     pub pad1 :            [u8; 16]
}

impl Copy for xcb_xfixes_query_version_reply_t {}
impl Clone for xcb_xfixes_query_version_reply_t {
    fn clone(&self) -> xcb_xfixes_query_version_reply_t { *self }
}


#[repr(C)]
pub struct xcb_xfixes_change_save_set_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub mode :           u8,
     pub target :         u8,
     pub map :            u8,
     pub pad0 :           u8,
     pub window :         ffi::xproto::xcb_window_t
}

impl Copy for xcb_xfixes_change_save_set_request_t {}
impl Clone for xcb_xfixes_change_save_set_request_t {
    fn clone(&self) -> xcb_xfixes_change_save_set_request_t { *self }
}


#[repr(C)]
pub struct xcb_xfixes_selection_notify_event_t {
     pub response_type :         u8,
     pub subtype :               u8,
     pub sequence :              u16,
     pub window :                ffi::xproto::xcb_window_t,
     pub owner :                 ffi::xproto::xcb_window_t,
     pub selection :             ffi::xproto::xcb_atom_t,
     pub timestamp :             ffi::xproto::xcb_timestamp_t,
     pub selection_timestamp :   ffi::xproto::xcb_timestamp_t,
     pub pad0 :                  [u8; 8]
}

impl Copy for xcb_xfixes_selection_notify_event_t {}
impl Clone for xcb_xfixes_selection_notify_event_t {
    fn clone(&self) -> xcb_xfixes_selection_notify_event_t { *self }
}


#[repr(C)]
pub struct xcb_xfixes_select_selection_input_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub window :         ffi::xproto::xcb_window_t,
     pub selection :      ffi::xproto::xcb_atom_t,
     pub event_mask :     u32
}

impl Copy for xcb_xfixes_select_selection_input_request_t {}
impl Clone for xcb_xfixes_select_selection_input_request_t {
    fn clone(&self) -> xcb_xfixes_select_selection_input_request_t { *self }
}


#[repr(C)]
pub struct xcb_xfixes_cursor_notify_event_t {
     pub response_type :   u8,
     pub subtype :         u8,
     pub sequence :        u16,
     pub window :          ffi::xproto::xcb_window_t,
     pub cursor_serial :   u32,
     pub timestamp :       ffi::xproto::xcb_timestamp_t,
     pub name :            ffi::xproto::xcb_atom_t,
     pub pad0 :            [u8; 12]
}

impl Copy for xcb_xfixes_cursor_notify_event_t {}
impl Clone for xcb_xfixes_cursor_notify_event_t {
    fn clone(&self) -> xcb_xfixes_cursor_notify_event_t { *self }
}


#[repr(C)]
pub struct xcb_xfixes_select_cursor_input_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub window :         ffi::xproto::xcb_window_t,
     pub event_mask :     u32
}

impl Copy for xcb_xfixes_select_cursor_input_request_t {}
impl Clone for xcb_xfixes_select_cursor_input_request_t {
    fn clone(&self) -> xcb_xfixes_select_cursor_input_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xfixes_get_cursor_image_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_xfixes_get_cursor_image_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16
}

impl Copy for xcb_xfixes_get_cursor_image_request_t {}
impl Clone for xcb_xfixes_get_cursor_image_request_t {
    fn clone(&self) -> xcb_xfixes_get_cursor_image_request_t { *self }
}

#[repr(C)]
pub struct xcb_xfixes_get_cursor_image_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub x :               i16,
     pub y :               i16,
     pub width :           u16,
     pub height :          u16,
     pub xhot :            u16,
     pub yhot :            u16,
     pub cursor_serial :   u32,
     pub pad1 :            [u8; 8]
}

impl Copy for xcb_xfixes_get_cursor_image_reply_t {}
impl Clone for xcb_xfixes_get_cursor_image_reply_t {
    fn clone(&self) -> xcb_xfixes_get_cursor_image_reply_t { *self }
}

pub type xcb_xfixes_region_t = u32;
/**
 * @brief xcb_xfixes_region_iterator_t
 **/
#[repr(C)]
pub struct xcb_xfixes_region_iterator_t {
    pub data : *mut xcb_xfixes_region_t,
    pub rem  : c_int,
    pub index: c_int
}



#[repr(C)]
pub struct xcb_xfixes_bad_region_error_t {
     pub response_type :   u8,
     pub error_code :      u8,
     pub sequence :        u16
}

impl Copy for xcb_xfixes_bad_region_error_t {}
impl Clone for xcb_xfixes_bad_region_error_t {
    fn clone(&self) -> xcb_xfixes_bad_region_error_t { *self }
}


#[repr(C)]
pub struct xcb_xfixes_create_region_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub region :         xcb_xfixes_region_t
}

impl Copy for xcb_xfixes_create_region_request_t {}
impl Clone for xcb_xfixes_create_region_request_t {
    fn clone(&self) -> xcb_xfixes_create_region_request_t { *self }
}


#[repr(C)]
pub struct xcb_xfixes_create_region_from_bitmap_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub region :         xcb_xfixes_region_t,
     pub bitmap :         ffi::xproto::xcb_pixmap_t
}

impl Copy for xcb_xfixes_create_region_from_bitmap_request_t {}
impl Clone for xcb_xfixes_create_region_from_bitmap_request_t {
    fn clone(&self) -> xcb_xfixes_create_region_from_bitmap_request_t { *self }
}


#[repr(C)]
pub struct xcb_xfixes_create_region_from_window_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub region :         xcb_xfixes_region_t,
     pub window :         ffi::xproto::xcb_window_t,
     pub kind :           ffi::shape::xcb_shape_kind_t,
     pub pad0 :           [u8; 3]
}

impl Copy for xcb_xfixes_create_region_from_window_request_t {}
impl Clone for xcb_xfixes_create_region_from_window_request_t {
    fn clone(&self) -> xcb_xfixes_create_region_from_window_request_t { *self }
}


#[repr(C)]
pub struct xcb_xfixes_create_region_from_gc_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub region :         xcb_xfixes_region_t,
     pub gc :             ffi::xproto::xcb_gcontext_t
}

impl Copy for xcb_xfixes_create_region_from_gc_request_t {}
impl Clone for xcb_xfixes_create_region_from_gc_request_t {
    fn clone(&self) -> xcb_xfixes_create_region_from_gc_request_t { *self }
}


#[repr(C)]
pub struct xcb_xfixes_create_region_from_picture_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub region :         xcb_xfixes_region_t,
     pub picture :        ffi::render::xcb_render_picture_t
}

impl Copy for xcb_xfixes_create_region_from_picture_request_t {}
impl Clone for xcb_xfixes_create_region_from_picture_request_t {
    fn clone(&self) -> xcb_xfixes_create_region_from_picture_request_t { *self }
}


#[repr(C)]
pub struct xcb_xfixes_destroy_region_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub region :         xcb_xfixes_region_t
}

impl Copy for xcb_xfixes_destroy_region_request_t {}
impl Clone for xcb_xfixes_destroy_region_request_t {
    fn clone(&self) -> xcb_xfixes_destroy_region_request_t { *self }
}


#[repr(C)]
pub struct xcb_xfixes_set_region_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub region :         xcb_xfixes_region_t
}

impl Copy for xcb_xfixes_set_region_request_t {}
impl Clone for xcb_xfixes_set_region_request_t {
    fn clone(&self) -> xcb_xfixes_set_region_request_t { *self }
}


#[repr(C)]
pub struct xcb_xfixes_copy_region_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub source :         xcb_xfixes_region_t,
     pub destination :    xcb_xfixes_region_t
}

impl Copy for xcb_xfixes_copy_region_request_t {}
impl Clone for xcb_xfixes_copy_region_request_t {
    fn clone(&self) -> xcb_xfixes_copy_region_request_t { *self }
}


#[repr(C)]
pub struct xcb_xfixes_union_region_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub source1 :        xcb_xfixes_region_t,
     pub source2 :        xcb_xfixes_region_t,
     pub destination :    xcb_xfixes_region_t
}

impl Copy for xcb_xfixes_union_region_request_t {}
impl Clone for xcb_xfixes_union_region_request_t {
    fn clone(&self) -> xcb_xfixes_union_region_request_t { *self }
}


#[repr(C)]
pub struct xcb_xfixes_intersect_region_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub source1 :        xcb_xfixes_region_t,
     pub source2 :        xcb_xfixes_region_t,
     pub destination :    xcb_xfixes_region_t
}

impl Copy for xcb_xfixes_intersect_region_request_t {}
impl Clone for xcb_xfixes_intersect_region_request_t {
    fn clone(&self) -> xcb_xfixes_intersect_region_request_t { *self }
}


#[repr(C)]
pub struct xcb_xfixes_subtract_region_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub source1 :        xcb_xfixes_region_t,
     pub source2 :        xcb_xfixes_region_t,
     pub destination :    xcb_xfixes_region_t
}

impl Copy for xcb_xfixes_subtract_region_request_t {}
impl Clone for xcb_xfixes_subtract_region_request_t {
    fn clone(&self) -> xcb_xfixes_subtract_region_request_t { *self }
}


#[repr(C)]
pub struct xcb_xfixes_invert_region_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub source :         xcb_xfixes_region_t,
     pub bounds :         ffi::xproto::xcb_rectangle_t,
     pub destination :    xcb_xfixes_region_t
}

impl Copy for xcb_xfixes_invert_region_request_t {}
impl Clone for xcb_xfixes_invert_region_request_t {
    fn clone(&self) -> xcb_xfixes_invert_region_request_t { *self }
}


#[repr(C)]
pub struct xcb_xfixes_translate_region_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub region :         xcb_xfixes_region_t,
     pub dx :             i16,
     pub dy :             i16
}

impl Copy for xcb_xfixes_translate_region_request_t {}
impl Clone for xcb_xfixes_translate_region_request_t {
    fn clone(&self) -> xcb_xfixes_translate_region_request_t { *self }
}


#[repr(C)]
pub struct xcb_xfixes_region_extents_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub source :         xcb_xfixes_region_t,
     pub destination :    xcb_xfixes_region_t
}

impl Copy for xcb_xfixes_region_extents_request_t {}
impl Clone for xcb_xfixes_region_extents_request_t {
    fn clone(&self) -> xcb_xfixes_region_extents_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xfixes_fetch_region_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_xfixes_fetch_region_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub region :         xcb_xfixes_region_t
}

impl Copy for xcb_xfixes_fetch_region_request_t {}
impl Clone for xcb_xfixes_fetch_region_request_t {
    fn clone(&self) -> xcb_xfixes_fetch_region_request_t { *self }
}

#[repr(C)]
pub struct xcb_xfixes_fetch_region_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub extents :         ffi::xproto::xcb_rectangle_t,
     pub pad1 :            [u8; 16]
}

impl Copy for xcb_xfixes_fetch_region_reply_t {}
impl Clone for xcb_xfixes_fetch_region_reply_t {
    fn clone(&self) -> xcb_xfixes_fetch_region_reply_t { *self }
}


#[repr(C)]
pub struct xcb_xfixes_set_gc_clip_region_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub gc :             ffi::xproto::xcb_gcontext_t,
     pub region :         xcb_xfixes_region_t,
     pub x_origin :       i16,
     pub y_origin :       i16
}

impl Copy for xcb_xfixes_set_gc_clip_region_request_t {}
impl Clone for xcb_xfixes_set_gc_clip_region_request_t {
    fn clone(&self) -> xcb_xfixes_set_gc_clip_region_request_t { *self }
}


#[repr(C)]
pub struct xcb_xfixes_set_window_shape_region_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub dest :           ffi::xproto::xcb_window_t,
     pub dest_kind :      ffi::shape::xcb_shape_kind_t,
     pub pad0 :           [u8; 3],
     pub x_offset :       i16,
     pub y_offset :       i16,
     pub region :         xcb_xfixes_region_t
}

impl Copy for xcb_xfixes_set_window_shape_region_request_t {}
impl Clone for xcb_xfixes_set_window_shape_region_request_t {
    fn clone(&self) -> xcb_xfixes_set_window_shape_region_request_t { *self }
}


#[repr(C)]
pub struct xcb_xfixes_set_picture_clip_region_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub picture :        ffi::render::xcb_render_picture_t,
     pub region :         xcb_xfixes_region_t,
     pub x_origin :       i16,
     pub y_origin :       i16
}

impl Copy for xcb_xfixes_set_picture_clip_region_request_t {}
impl Clone for xcb_xfixes_set_picture_clip_region_request_t {
    fn clone(&self) -> xcb_xfixes_set_picture_clip_region_request_t { *self }
}


#[repr(C)]
pub struct xcb_xfixes_set_cursor_name_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub cursor :         ffi::xproto::xcb_cursor_t,
     pub nbytes :         u16,
     pub pad0 :           [u8; 2]
}

impl Copy for xcb_xfixes_set_cursor_name_request_t {}
impl Clone for xcb_xfixes_set_cursor_name_request_t {
    fn clone(&self) -> xcb_xfixes_set_cursor_name_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xfixes_get_cursor_name_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_xfixes_get_cursor_name_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub cursor :         ffi::xproto::xcb_cursor_t
}

impl Copy for xcb_xfixes_get_cursor_name_request_t {}
impl Clone for xcb_xfixes_get_cursor_name_request_t {
    fn clone(&self) -> xcb_xfixes_get_cursor_name_request_t { *self }
}

#[repr(C)]
pub struct xcb_xfixes_get_cursor_name_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub atom :            ffi::xproto::xcb_atom_t,
     pub nbytes :          u16,
     pub pad1 :            [u8; 18]
}

impl Copy for xcb_xfixes_get_cursor_name_reply_t {}
impl Clone for xcb_xfixes_get_cursor_name_reply_t {
    fn clone(&self) -> xcb_xfixes_get_cursor_name_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xfixes_get_cursor_image_and_name_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_xfixes_get_cursor_image_and_name_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16
}

impl Copy for xcb_xfixes_get_cursor_image_and_name_request_t {}
impl Clone for xcb_xfixes_get_cursor_image_and_name_request_t {
    fn clone(&self) -> xcb_xfixes_get_cursor_image_and_name_request_t { *self }
}

#[repr(C)]
pub struct xcb_xfixes_get_cursor_image_and_name_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub x :               i16,
     pub y :               i16,
     pub width :           u16,
     pub height :          u16,
     pub xhot :            u16,
     pub yhot :            u16,
     pub cursor_serial :   u32,
     pub cursor_atom :     ffi::xproto::xcb_atom_t,
     pub nbytes :          u16,
     pub pad1 :            [u8; 2]
}

impl Copy for xcb_xfixes_get_cursor_image_and_name_reply_t {}
impl Clone for xcb_xfixes_get_cursor_image_and_name_reply_t {
    fn clone(&self) -> xcb_xfixes_get_cursor_image_and_name_reply_t { *self }
}


#[repr(C)]
pub struct xcb_xfixes_change_cursor_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub source :         ffi::xproto::xcb_cursor_t,
     pub destination :    ffi::xproto::xcb_cursor_t
}

impl Copy for xcb_xfixes_change_cursor_request_t {}
impl Clone for xcb_xfixes_change_cursor_request_t {
    fn clone(&self) -> xcb_xfixes_change_cursor_request_t { *self }
}


#[repr(C)]
pub struct xcb_xfixes_change_cursor_by_name_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub src :            ffi::xproto::xcb_cursor_t,
     pub nbytes :         u16,
     pub pad0 :           [u8; 2]
}

impl Copy for xcb_xfixes_change_cursor_by_name_request_t {}
impl Clone for xcb_xfixes_change_cursor_by_name_request_t {
    fn clone(&self) -> xcb_xfixes_change_cursor_by_name_request_t { *self }
}


#[repr(C)]
pub struct xcb_xfixes_expand_region_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub source :         xcb_xfixes_region_t,
     pub destination :    xcb_xfixes_region_t,
     pub left :           u16,
     pub right :          u16,
     pub top :            u16,
     pub bottom :         u16
}

impl Copy for xcb_xfixes_expand_region_request_t {}
impl Clone for xcb_xfixes_expand_region_request_t {
    fn clone(&self) -> xcb_xfixes_expand_region_request_t { *self }
}


#[repr(C)]
pub struct xcb_xfixes_hide_cursor_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub window :         ffi::xproto::xcb_window_t
}

impl Copy for xcb_xfixes_hide_cursor_request_t {}
impl Clone for xcb_xfixes_hide_cursor_request_t {
    fn clone(&self) -> xcb_xfixes_hide_cursor_request_t { *self }
}


#[repr(C)]
pub struct xcb_xfixes_show_cursor_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub window :         ffi::xproto::xcb_window_t
}

impl Copy for xcb_xfixes_show_cursor_request_t {}
impl Clone for xcb_xfixes_show_cursor_request_t {
    fn clone(&self) -> xcb_xfixes_show_cursor_request_t { *self }
}
#[link(name="xcb-xfixes")]
extern "C" {

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_xfixes_query_version (c : *mut ffi::base::xcb_connection_t,
                                    client_major_version :  u32,
                                    client_minor_version :  u32) -> xcb_xfixes_query_version_cookie_t;

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
pub fn xcb_xfixes_query_version_unchecked (c : *mut ffi::base::xcb_connection_t,
                                              client_major_version :  u32,
                                              client_minor_version :  u32) -> xcb_xfixes_query_version_cookie_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xfixes_query_version_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_xfixes_query_version_reply (c : *mut ffi::base::xcb_connection_t,
                                          cookie : xcb_xfixes_query_version_cookie_t,
                                          e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_xfixes_query_version_reply_t;

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
pub fn xcb_xfixes_change_save_set_checked (c : *mut ffi::base::xcb_connection_t,
                                              mode :  u8,
                                              target :  u8,
                                              map :  u8,
                                              window :  ffi::xproto::xcb_window_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_xfixes_change_save_set (c : *mut ffi::base::xcb_connection_t,
                                      mode :  u8,
                                      target :  u8,
                                      map :  u8,
                                      window :  ffi::xproto::xcb_window_t) -> ffi::base::xcb_void_cookie_t;

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
pub fn xcb_xfixes_select_selection_input_checked (c : *mut ffi::base::xcb_connection_t,
                                                     window :  ffi::xproto::xcb_window_t,
                                                     selection :  ffi::xproto::xcb_atom_t,
                                                     event_mask :  u32) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_xfixes_select_selection_input (c : *mut ffi::base::xcb_connection_t,
                                             window :  ffi::xproto::xcb_window_t,
                                             selection :  ffi::xproto::xcb_atom_t,
                                             event_mask :  u32) -> ffi::base::xcb_void_cookie_t;

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
pub fn xcb_xfixes_select_cursor_input_checked (c : *mut ffi::base::xcb_connection_t,
                                                  window :  ffi::xproto::xcb_window_t,
                                                  event_mask :  u32) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_xfixes_select_cursor_input (c : *mut ffi::base::xcb_connection_t,
                                          window :  ffi::xproto::xcb_window_t,
                                          event_mask :  u32) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_xfixes_get_cursor_image_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_xfixes_get_cursor_image (c : *mut ffi::base::xcb_connection_t) -> xcb_xfixes_get_cursor_image_cookie_t;

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
pub fn xcb_xfixes_get_cursor_image_unchecked (c : *mut ffi::base::xcb_connection_t) -> xcb_xfixes_get_cursor_image_cookie_t;

pub fn xcb_xfixes_get_cursor_image_cursor_image (R : *mut xcb_xfixes_get_cursor_image_reply_t) -> *mut u32;


pub fn xcb_xfixes_get_cursor_image_cursor_image_length (R : *mut xcb_xfixes_get_cursor_image_reply_t) -> c_int;


pub fn xcb_xfixes_get_cursor_image_cursor_image_end (R : *mut xcb_xfixes_get_cursor_image_reply_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xfixes_get_cursor_image_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_xfixes_get_cursor_image_reply (c : *mut ffi::base::xcb_connection_t,
                                             cookie : xcb_xfixes_get_cursor_image_cookie_t,
                                             e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_xfixes_get_cursor_image_reply_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_xfixes_region_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_xfixes_region_t)
 *
 *
 */
pub fn xcb_xfixes_region_next (i:*mut xcb_xfixes_region_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_xfixes_region_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_xfixes_region_end (i:xcb_xfixes_region_iterator_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_xfixes_create_region_sizeof (_buffer :  *mut c_void,
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
pub fn xcb_xfixes_create_region_checked (c : *mut ffi::base::xcb_connection_t,
                                            region :  xcb_xfixes_region_t,
                                            rectangles_len :  u32,
                                            rectangles : *mut ffi::xproto::xcb_rectangle_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_xfixes_create_region (c : *mut ffi::base::xcb_connection_t,
                                    region :  xcb_xfixes_region_t,
                                    rectangles_len :  u32,
                                    rectangles : *mut ffi::xproto::xcb_rectangle_t) -> ffi::base::xcb_void_cookie_t;

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
pub fn xcb_xfixes_create_region_from_bitmap_checked (c : *mut ffi::base::xcb_connection_t,
                                                        region :  xcb_xfixes_region_t,
                                                        bitmap :  ffi::xproto::xcb_pixmap_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_xfixes_create_region_from_bitmap (c : *mut ffi::base::xcb_connection_t,
                                                region :  xcb_xfixes_region_t,
                                                bitmap :  ffi::xproto::xcb_pixmap_t) -> ffi::base::xcb_void_cookie_t;

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
pub fn xcb_xfixes_create_region_from_window_checked (c : *mut ffi::base::xcb_connection_t,
                                                        region :  xcb_xfixes_region_t,
                                                        window :  ffi::xproto::xcb_window_t,
                                                        kind :  ffi::shape::xcb_shape_kind_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_xfixes_create_region_from_window (c : *mut ffi::base::xcb_connection_t,
                                                region :  xcb_xfixes_region_t,
                                                window :  ffi::xproto::xcb_window_t,
                                                kind :  ffi::shape::xcb_shape_kind_t) -> ffi::base::xcb_void_cookie_t;

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
pub fn xcb_xfixes_create_region_from_gc_checked (c : *mut ffi::base::xcb_connection_t,
                                                    region :  xcb_xfixes_region_t,
                                                    gc :  ffi::xproto::xcb_gcontext_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_xfixes_create_region_from_gc (c : *mut ffi::base::xcb_connection_t,
                                            region :  xcb_xfixes_region_t,
                                            gc :  ffi::xproto::xcb_gcontext_t) -> ffi::base::xcb_void_cookie_t;

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
pub fn xcb_xfixes_create_region_from_picture_checked (c : *mut ffi::base::xcb_connection_t,
                                                         region :  xcb_xfixes_region_t,
                                                         picture :  ffi::render::xcb_render_picture_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_xfixes_create_region_from_picture (c : *mut ffi::base::xcb_connection_t,
                                                 region :  xcb_xfixes_region_t,
                                                 picture :  ffi::render::xcb_render_picture_t) -> ffi::base::xcb_void_cookie_t;

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
pub fn xcb_xfixes_destroy_region_checked (c : *mut ffi::base::xcb_connection_t,
                                             region :  xcb_xfixes_region_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_xfixes_destroy_region (c : *mut ffi::base::xcb_connection_t,
                                     region :  xcb_xfixes_region_t) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_xfixes_set_region_sizeof (_buffer :  *mut c_void,
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
pub fn xcb_xfixes_set_region_checked (c : *mut ffi::base::xcb_connection_t,
                                         region :  xcb_xfixes_region_t,
                                         rectangles_len :  u32,
                                         rectangles : *mut ffi::xproto::xcb_rectangle_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_xfixes_set_region (c : *mut ffi::base::xcb_connection_t,
                                 region :  xcb_xfixes_region_t,
                                 rectangles_len :  u32,
                                 rectangles : *mut ffi::xproto::xcb_rectangle_t) -> ffi::base::xcb_void_cookie_t;

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
pub fn xcb_xfixes_copy_region_checked (c : *mut ffi::base::xcb_connection_t,
                                          source :  xcb_xfixes_region_t,
                                          destination :  xcb_xfixes_region_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_xfixes_copy_region (c : *mut ffi::base::xcb_connection_t,
                                  source :  xcb_xfixes_region_t,
                                  destination :  xcb_xfixes_region_t) -> ffi::base::xcb_void_cookie_t;

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
pub fn xcb_xfixes_union_region_checked (c : *mut ffi::base::xcb_connection_t,
                                           source1 :  xcb_xfixes_region_t,
                                           source2 :  xcb_xfixes_region_t,
                                           destination :  xcb_xfixes_region_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_xfixes_union_region (c : *mut ffi::base::xcb_connection_t,
                                   source1 :  xcb_xfixes_region_t,
                                   source2 :  xcb_xfixes_region_t,
                                   destination :  xcb_xfixes_region_t) -> ffi::base::xcb_void_cookie_t;

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
pub fn xcb_xfixes_intersect_region_checked (c : *mut ffi::base::xcb_connection_t,
                                               source1 :  xcb_xfixes_region_t,
                                               source2 :  xcb_xfixes_region_t,
                                               destination :  xcb_xfixes_region_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_xfixes_intersect_region (c : *mut ffi::base::xcb_connection_t,
                                       source1 :  xcb_xfixes_region_t,
                                       source2 :  xcb_xfixes_region_t,
                                       destination :  xcb_xfixes_region_t) -> ffi::base::xcb_void_cookie_t;

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
pub fn xcb_xfixes_subtract_region_checked (c : *mut ffi::base::xcb_connection_t,
                                              source1 :  xcb_xfixes_region_t,
                                              source2 :  xcb_xfixes_region_t,
                                              destination :  xcb_xfixes_region_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_xfixes_subtract_region (c : *mut ffi::base::xcb_connection_t,
                                      source1 :  xcb_xfixes_region_t,
                                      source2 :  xcb_xfixes_region_t,
                                      destination :  xcb_xfixes_region_t) -> ffi::base::xcb_void_cookie_t;

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
pub fn xcb_xfixes_invert_region_checked (c : *mut ffi::base::xcb_connection_t,
                                            source :  xcb_xfixes_region_t,
                                            bounds :  ffi::xproto::xcb_rectangle_t,
                                            destination :  xcb_xfixes_region_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_xfixes_invert_region (c : *mut ffi::base::xcb_connection_t,
                                    source :  xcb_xfixes_region_t,
                                    bounds :  ffi::xproto::xcb_rectangle_t,
                                    destination :  xcb_xfixes_region_t) -> ffi::base::xcb_void_cookie_t;

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
pub fn xcb_xfixes_translate_region_checked (c : *mut ffi::base::xcb_connection_t,
                                               region :  xcb_xfixes_region_t,
                                               dx :  i16,
                                               dy :  i16) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_xfixes_translate_region (c : *mut ffi::base::xcb_connection_t,
                                       region :  xcb_xfixes_region_t,
                                       dx :  i16,
                                       dy :  i16) -> ffi::base::xcb_void_cookie_t;

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
pub fn xcb_xfixes_region_extents_checked (c : *mut ffi::base::xcb_connection_t,
                                             source :  xcb_xfixes_region_t,
                                             destination :  xcb_xfixes_region_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_xfixes_region_extents (c : *mut ffi::base::xcb_connection_t,
                                     source :  xcb_xfixes_region_t,
                                     destination :  xcb_xfixes_region_t) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_xfixes_fetch_region_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_xfixes_fetch_region (c : *mut ffi::base::xcb_connection_t,
                                   region :  xcb_xfixes_region_t) -> xcb_xfixes_fetch_region_cookie_t;

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
pub fn xcb_xfixes_fetch_region_unchecked (c : *mut ffi::base::xcb_connection_t,
                                             region :  xcb_xfixes_region_t) -> xcb_xfixes_fetch_region_cookie_t;

pub fn xcb_xfixes_fetch_region_rectangles (R : *mut xcb_xfixes_fetch_region_reply_t) -> *mut ffi::xproto::xcb_rectangle_t;


pub fn xcb_xfixes_fetch_region_rectangles_length (R : *mut xcb_xfixes_fetch_region_reply_t) -> c_int;

pub fn xcb_xfixes_fetch_region_rectangles_iterator (R : *mut xcb_xfixes_fetch_region_reply_t) -> ffi::xproto::xcb_rectangle_iterator_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xfixes_fetch_region_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_xfixes_fetch_region_reply (c : *mut ffi::base::xcb_connection_t,
                                         cookie : xcb_xfixes_fetch_region_cookie_t,
                                         e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_xfixes_fetch_region_reply_t;

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
pub fn xcb_xfixes_set_gc_clip_region_checked (c : *mut ffi::base::xcb_connection_t,
                                                 gc :  ffi::xproto::xcb_gcontext_t,
                                                 region :  xcb_xfixes_region_t,
                                                 x_origin :  i16,
                                                 y_origin :  i16) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_xfixes_set_gc_clip_region (c : *mut ffi::base::xcb_connection_t,
                                         gc :  ffi::xproto::xcb_gcontext_t,
                                         region :  xcb_xfixes_region_t,
                                         x_origin :  i16,
                                         y_origin :  i16) -> ffi::base::xcb_void_cookie_t;

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
pub fn xcb_xfixes_set_window_shape_region_checked (c : *mut ffi::base::xcb_connection_t,
                                                      dest :  ffi::xproto::xcb_window_t,
                                                      dest_kind :  ffi::shape::xcb_shape_kind_t,
                                                      x_offset :  i16,
                                                      y_offset :  i16,
                                                      region :  xcb_xfixes_region_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_xfixes_set_window_shape_region (c : *mut ffi::base::xcb_connection_t,
                                              dest :  ffi::xproto::xcb_window_t,
                                              dest_kind :  ffi::shape::xcb_shape_kind_t,
                                              x_offset :  i16,
                                              y_offset :  i16,
                                              region :  xcb_xfixes_region_t) -> ffi::base::xcb_void_cookie_t;

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
pub fn xcb_xfixes_set_picture_clip_region_checked (c : *mut ffi::base::xcb_connection_t,
                                                      picture :  ffi::render::xcb_render_picture_t,
                                                      region :  xcb_xfixes_region_t,
                                                      x_origin :  i16,
                                                      y_origin :  i16) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_xfixes_set_picture_clip_region (c : *mut ffi::base::xcb_connection_t,
                                              picture :  ffi::render::xcb_render_picture_t,
                                              region :  xcb_xfixes_region_t,
                                              x_origin :  i16,
                                              y_origin :  i16) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_xfixes_set_cursor_name_sizeof (_buffer :  *mut c_void) -> c_int;

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
pub fn xcb_xfixes_set_cursor_name_checked (c : *mut ffi::base::xcb_connection_t,
                                              cursor :  ffi::xproto::xcb_cursor_t,
                                              nbytes :  u16,
                                              name : *mut c_char) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_xfixes_set_cursor_name (c : *mut ffi::base::xcb_connection_t,
                                      cursor :  ffi::xproto::xcb_cursor_t,
                                      nbytes :  u16,
                                      name : *mut c_char) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_xfixes_get_cursor_name_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_xfixes_get_cursor_name (c : *mut ffi::base::xcb_connection_t,
                                      cursor :  ffi::xproto::xcb_cursor_t) -> xcb_xfixes_get_cursor_name_cookie_t;

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
pub fn xcb_xfixes_get_cursor_name_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                cursor :  ffi::xproto::xcb_cursor_t) -> xcb_xfixes_get_cursor_name_cookie_t;

pub fn xcb_xfixes_get_cursor_name_name (R : *mut xcb_xfixes_get_cursor_name_reply_t) -> *mut c_char;


pub fn xcb_xfixes_get_cursor_name_name_length (R : *mut xcb_xfixes_get_cursor_name_reply_t) -> c_int;


pub fn xcb_xfixes_get_cursor_name_name_end (R : *mut xcb_xfixes_get_cursor_name_reply_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xfixes_get_cursor_name_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_xfixes_get_cursor_name_reply (c : *mut ffi::base::xcb_connection_t,
                                            cookie : xcb_xfixes_get_cursor_name_cookie_t,
                                            e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_xfixes_get_cursor_name_reply_t;

pub fn xcb_xfixes_get_cursor_image_and_name_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_xfixes_get_cursor_image_and_name (c : *mut ffi::base::xcb_connection_t) -> xcb_xfixes_get_cursor_image_and_name_cookie_t;

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
pub fn xcb_xfixes_get_cursor_image_and_name_unchecked (c : *mut ffi::base::xcb_connection_t) -> xcb_xfixes_get_cursor_image_and_name_cookie_t;

pub fn xcb_xfixes_get_cursor_image_and_name_name (R : *mut xcb_xfixes_get_cursor_image_and_name_reply_t) -> *mut c_char;


pub fn xcb_xfixes_get_cursor_image_and_name_name_length (R : *mut xcb_xfixes_get_cursor_image_and_name_reply_t) -> c_int;


pub fn xcb_xfixes_get_cursor_image_and_name_name_end (R : *mut xcb_xfixes_get_cursor_image_and_name_reply_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_xfixes_get_cursor_image_and_name_cursor_image (R : *mut xcb_xfixes_get_cursor_image_and_name_reply_t) -> *mut u32;


pub fn xcb_xfixes_get_cursor_image_and_name_cursor_image_length (R : *mut xcb_xfixes_get_cursor_image_and_name_reply_t) -> c_int;


pub fn xcb_xfixes_get_cursor_image_and_name_cursor_image_end (R : *mut xcb_xfixes_get_cursor_image_and_name_reply_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xfixes_get_cursor_image_and_name_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_xfixes_get_cursor_image_and_name_reply (c : *mut ffi::base::xcb_connection_t,
                                                      cookie : xcb_xfixes_get_cursor_image_and_name_cookie_t,
                                                      e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_xfixes_get_cursor_image_and_name_reply_t;

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
pub fn xcb_xfixes_change_cursor_checked (c : *mut ffi::base::xcb_connection_t,
                                            source :  ffi::xproto::xcb_cursor_t,
                                            destination :  ffi::xproto::xcb_cursor_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_xfixes_change_cursor (c : *mut ffi::base::xcb_connection_t,
                                    source :  ffi::xproto::xcb_cursor_t,
                                    destination :  ffi::xproto::xcb_cursor_t) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_xfixes_change_cursor_by_name_sizeof (_buffer :  *mut c_void) -> c_int;

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
pub fn xcb_xfixes_change_cursor_by_name_checked (c : *mut ffi::base::xcb_connection_t,
                                                    src :  ffi::xproto::xcb_cursor_t,
                                                    nbytes :  u16,
                                                    name : *mut c_char) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_xfixes_change_cursor_by_name (c : *mut ffi::base::xcb_connection_t,
                                            src :  ffi::xproto::xcb_cursor_t,
                                            nbytes :  u16,
                                            name : *mut c_char) -> ffi::base::xcb_void_cookie_t;

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
pub fn xcb_xfixes_expand_region_checked (c : *mut ffi::base::xcb_connection_t,
                                            source :  xcb_xfixes_region_t,
                                            destination :  xcb_xfixes_region_t,
                                            left :  u16,
                                            right :  u16,
                                            top :  u16,
                                            bottom :  u16) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_xfixes_expand_region (c : *mut ffi::base::xcb_connection_t,
                                    source :  xcb_xfixes_region_t,
                                    destination :  xcb_xfixes_region_t,
                                    left :  u16,
                                    right :  u16,
                                    top :  u16,
                                    bottom :  u16) -> ffi::base::xcb_void_cookie_t;

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
pub fn xcb_xfixes_hide_cursor_checked (c : *mut ffi::base::xcb_connection_t,
                                          window :  ffi::xproto::xcb_window_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_xfixes_hide_cursor (c : *mut ffi::base::xcb_connection_t,
                                  window :  ffi::xproto::xcb_window_t) -> ffi::base::xcb_void_cookie_t;

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
pub fn xcb_xfixes_show_cursor_checked (c : *mut ffi::base::xcb_connection_t,
                                          window :  ffi::xproto::xcb_window_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_xfixes_show_cursor (c : *mut ffi::base::xcb_connection_t,
                                  window :  ffi::xproto::xcb_window_t) -> ffi::base::xcb_void_cookie_t;
}

