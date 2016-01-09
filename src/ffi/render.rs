/*
 * This file generated automatically from render.xml by r_client.py.
 * Edit at your peril.
 */

//Make the compiler quiet
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use std;
use libc::*;
use ffi;
use ffi::xproto;

pub const RENDER_MAJOR_VERSION : c_uint = 0;
pub const RENDER_MINOR_VERSION : c_uint = 11;

pub type xcb_render_glyph_t = u32;
/**
 * @brief xcb_render_glyph_iterator_t
 **/
#[repr(C)]
pub struct xcb_render_glyph_iterator_t {
    pub data : *mut xcb_render_glyph_t,
    pub rem  : c_int,
    pub index: c_int
}


pub type xcb_render_glyphset_t = u32;
/**
 * @brief xcb_render_glyphset_iterator_t
 **/
#[repr(C)]
pub struct xcb_render_glyphset_iterator_t {
    pub data : *mut xcb_render_glyphset_t,
    pub rem  : c_int,
    pub index: c_int
}


pub type xcb_render_picture_t = u32;
/**
 * @brief xcb_render_picture_iterator_t
 **/
#[repr(C)]
pub struct xcb_render_picture_iterator_t {
    pub data : *mut xcb_render_picture_t,
    pub rem  : c_int,
    pub index: c_int
}


pub type xcb_render_pictformat_t = u32;
/**
 * @brief xcb_render_pictformat_iterator_t
 **/
#[repr(C)]
pub struct xcb_render_pictformat_iterator_t {
    pub data : *mut xcb_render_pictformat_t,
    pub rem  : c_int,
    pub index: c_int
}


pub type xcb_render_fixed_t = i32;
/**
 * @brief xcb_render_fixed_iterator_t
 **/
#[repr(C)]
pub struct xcb_render_fixed_iterator_t {
    pub data : *mut xcb_render_fixed_t,
    pub rem  : c_int,
    pub index: c_int
}



#[repr(C)]
pub struct xcb_render_pict_format_error_t {
     pub response_type :   u8,
     pub error_code :      u8,
     pub sequence :        u16
}

impl Copy for xcb_render_pict_format_error_t {}
impl Clone for xcb_render_pict_format_error_t {
    fn clone(&self) -> xcb_render_pict_format_error_t { *self }
}


#[repr(C)]
pub struct xcb_render_picture_error_t {
     pub response_type :   u8,
     pub error_code :      u8,
     pub sequence :        u16
}

impl Copy for xcb_render_picture_error_t {}
impl Clone for xcb_render_picture_error_t {
    fn clone(&self) -> xcb_render_picture_error_t { *self }
}


#[repr(C)]
pub struct xcb_render_pict_op_error_t {
     pub response_type :   u8,
     pub error_code :      u8,
     pub sequence :        u16
}

impl Copy for xcb_render_pict_op_error_t {}
impl Clone for xcb_render_pict_op_error_t {
    fn clone(&self) -> xcb_render_pict_op_error_t { *self }
}


#[repr(C)]
pub struct xcb_render_glyph_set_error_t {
     pub response_type :   u8,
     pub error_code :      u8,
     pub sequence :        u16
}

impl Copy for xcb_render_glyph_set_error_t {}
impl Clone for xcb_render_glyph_set_error_t {
    fn clone(&self) -> xcb_render_glyph_set_error_t { *self }
}


#[repr(C)]
pub struct xcb_render_glyph_error_t {
     pub response_type :   u8,
     pub error_code :      u8,
     pub sequence :        u16
}

impl Copy for xcb_render_glyph_error_t {}
impl Clone for xcb_render_glyph_error_t {
    fn clone(&self) -> xcb_render_glyph_error_t { *self }
}

#[repr(C)]
pub struct xcb_render_directformat_t {
     pub red_shift :     u16,
     pub red_mask :      u16,
     pub green_shift :   u16,
     pub green_mask :    u16,
     pub blue_shift :    u16,
     pub blue_mask :     u16,
     pub alpha_shift :   u16,
     pub alpha_mask :    u16
}

impl Copy for xcb_render_directformat_t {}
impl Clone for xcb_render_directformat_t {
    fn clone(&self) -> xcb_render_directformat_t { *self }
}
/**
 * @brief xcb_render_directformat_iterator_t
 **/
#[repr(C)]
pub struct xcb_render_directformat_iterator_t {
    pub data : *mut xcb_render_directformat_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_render_pictforminfo_t {
     pub id :         xcb_render_pictformat_t,
     pub type_ :      u8,
     pub depth :      u8,
     pub pad0 :       [u8; 2],
     pub direct :     xcb_render_directformat_t,
     pub colormap :   ffi::xproto::xcb_colormap_t
}

impl Copy for xcb_render_pictforminfo_t {}
impl Clone for xcb_render_pictforminfo_t {
    fn clone(&self) -> xcb_render_pictforminfo_t { *self }
}
/**
 * @brief xcb_render_pictforminfo_iterator_t
 **/
#[repr(C)]
pub struct xcb_render_pictforminfo_iterator_t {
    pub data : *mut xcb_render_pictforminfo_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_render_pictvisual_t {
     pub visual :   ffi::xproto::xcb_visualid_t,
     pub format :   xcb_render_pictformat_t
}

impl Copy for xcb_render_pictvisual_t {}
impl Clone for xcb_render_pictvisual_t {
    fn clone(&self) -> xcb_render_pictvisual_t { *self }
}
/**
 * @brief xcb_render_pictvisual_iterator_t
 **/
#[repr(C)]
pub struct xcb_render_pictvisual_iterator_t {
    pub data : *mut xcb_render_pictvisual_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_render_pictdepth_t {
     pub depth :         u8,
     pub pad0 :          u8,
     pub num_visuals :   u16,
     pub pad1 :          [u8; 4]
}

impl Copy for xcb_render_pictdepth_t {}
impl Clone for xcb_render_pictdepth_t {
    fn clone(&self) -> xcb_render_pictdepth_t { *self }
}
/**
 * @brief xcb_render_pictdepth_iterator_t
 **/
#[repr(C)]
pub struct xcb_render_pictdepth_iterator_t {
    pub data : *mut xcb_render_pictdepth_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_render_pictscreen_t {
     pub num_depths :   u32,
     pub fallback :     xcb_render_pictformat_t
}

impl Copy for xcb_render_pictscreen_t {}
impl Clone for xcb_render_pictscreen_t {
    fn clone(&self) -> xcb_render_pictscreen_t { *self }
}
/**
 * @brief xcb_render_pictscreen_iterator_t
 **/
#[repr(C)]
pub struct xcb_render_pictscreen_iterator_t {
    pub data : *mut xcb_render_pictscreen_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_render_indexvalue_t {
     pub pixel :   u32,
     pub red :     u16,
     pub green :   u16,
     pub blue :    u16,
     pub alpha :   u16
}

impl Copy for xcb_render_indexvalue_t {}
impl Clone for xcb_render_indexvalue_t {
    fn clone(&self) -> xcb_render_indexvalue_t { *self }
}
/**
 * @brief xcb_render_indexvalue_iterator_t
 **/
#[repr(C)]
pub struct xcb_render_indexvalue_iterator_t {
    pub data : *mut xcb_render_indexvalue_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_render_color_t {
     pub red :     u16,
     pub green :   u16,
     pub blue :    u16,
     pub alpha :   u16
}

impl Copy for xcb_render_color_t {}
impl Clone for xcb_render_color_t {
    fn clone(&self) -> xcb_render_color_t { *self }
}
/**
 * @brief xcb_render_color_iterator_t
 **/
#[repr(C)]
pub struct xcb_render_color_iterator_t {
    pub data : *mut xcb_render_color_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_render_pointfix_t {
     pub x :   xcb_render_fixed_t,
     pub y :   xcb_render_fixed_t
}

impl Copy for xcb_render_pointfix_t {}
impl Clone for xcb_render_pointfix_t {
    fn clone(&self) -> xcb_render_pointfix_t { *self }
}
/**
 * @brief xcb_render_pointfix_iterator_t
 **/
#[repr(C)]
pub struct xcb_render_pointfix_iterator_t {
    pub data : *mut xcb_render_pointfix_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_render_linefix_t {
     pub p1 :   xcb_render_pointfix_t,
     pub p2 :   xcb_render_pointfix_t
}

impl Copy for xcb_render_linefix_t {}
impl Clone for xcb_render_linefix_t {
    fn clone(&self) -> xcb_render_linefix_t { *self }
}
/**
 * @brief xcb_render_linefix_iterator_t
 **/
#[repr(C)]
pub struct xcb_render_linefix_iterator_t {
    pub data : *mut xcb_render_linefix_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_render_triangle_t {
     pub p1 :   xcb_render_pointfix_t,
     pub p2 :   xcb_render_pointfix_t,
     pub p3 :   xcb_render_pointfix_t
}

impl Copy for xcb_render_triangle_t {}
impl Clone for xcb_render_triangle_t {
    fn clone(&self) -> xcb_render_triangle_t { *self }
}
/**
 * @brief xcb_render_triangle_iterator_t
 **/
#[repr(C)]
pub struct xcb_render_triangle_iterator_t {
    pub data : *mut xcb_render_triangle_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_render_trapezoid_t {
     pub top :      xcb_render_fixed_t,
     pub bottom :   xcb_render_fixed_t,
     pub left :     xcb_render_linefix_t,
     pub right :    xcb_render_linefix_t
}

impl Copy for xcb_render_trapezoid_t {}
impl Clone for xcb_render_trapezoid_t {
    fn clone(&self) -> xcb_render_trapezoid_t { *self }
}
/**
 * @brief xcb_render_trapezoid_iterator_t
 **/
#[repr(C)]
pub struct xcb_render_trapezoid_iterator_t {
    pub data : *mut xcb_render_trapezoid_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_render_glyphinfo_t {
     pub width :    u16,
     pub height :   u16,
     pub x :        i16,
     pub y :        i16,
     pub x_off :    i16,
     pub y_off :    i16
}

impl Copy for xcb_render_glyphinfo_t {}
impl Clone for xcb_render_glyphinfo_t {
    fn clone(&self) -> xcb_render_glyphinfo_t { *self }
}
/**
 * @brief xcb_render_glyphinfo_iterator_t
 **/
#[repr(C)]
pub struct xcb_render_glyphinfo_iterator_t {
    pub data : *mut xcb_render_glyphinfo_t,
    pub rem  : c_int,
    pub index: c_int
}


#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_render_query_version_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_render_query_version_request_t {
     pub major_opcode :           u8,
     pub minor_opcode :           u8,
     pub length :                 u16,
     pub client_major_version :   u32,
     pub client_minor_version :   u32
}

impl Copy for xcb_render_query_version_request_t {}
impl Clone for xcb_render_query_version_request_t {
    fn clone(&self) -> xcb_render_query_version_request_t { *self }
}

#[repr(C)]
pub struct xcb_render_query_version_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub major_version :   u32,
     pub minor_version :   u32,
     pub pad1 :            [u8; 16]
}

impl Copy for xcb_render_query_version_reply_t {}
impl Clone for xcb_render_query_version_reply_t {
    fn clone(&self) -> xcb_render_query_version_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_render_query_pict_formats_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_render_query_pict_formats_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16
}

impl Copy for xcb_render_query_pict_formats_request_t {}
impl Clone for xcb_render_query_pict_formats_request_t {
    fn clone(&self) -> xcb_render_query_pict_formats_request_t { *self }
}

#[repr(C)]
pub struct xcb_render_query_pict_formats_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub num_formats :     u32,
     pub num_screens :     u32,
     pub num_depths :      u32,
     pub num_visuals :     u32,
     pub num_subpixel :    u32,
     pub pad1 :            [u8; 4]
}

impl Copy for xcb_render_query_pict_formats_reply_t {}
impl Clone for xcb_render_query_pict_formats_reply_t {
    fn clone(&self) -> xcb_render_query_pict_formats_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_render_query_pict_index_values_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_render_query_pict_index_values_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub format :         xcb_render_pictformat_t
}

impl Copy for xcb_render_query_pict_index_values_request_t {}
impl Clone for xcb_render_query_pict_index_values_request_t {
    fn clone(&self) -> xcb_render_query_pict_index_values_request_t { *self }
}

#[repr(C)]
pub struct xcb_render_query_pict_index_values_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub num_values :      u32,
     pub pad1 :            [u8; 20]
}

impl Copy for xcb_render_query_pict_index_values_reply_t {}
impl Clone for xcb_render_query_pict_index_values_reply_t {
    fn clone(&self) -> xcb_render_query_pict_index_values_reply_t { *self }
}


#[repr(C)]
pub struct xcb_render_create_picture_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub pid :            xcb_render_picture_t,
     pub drawable :       ffi::xproto::xcb_drawable_t,
     pub format :         xcb_render_pictformat_t,
     pub value_mask :     u32
}

impl Copy for xcb_render_create_picture_request_t {}
impl Clone for xcb_render_create_picture_request_t {
    fn clone(&self) -> xcb_render_create_picture_request_t { *self }
}


#[repr(C)]
pub struct xcb_render_change_picture_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub picture :        xcb_render_picture_t,
     pub value_mask :     u32
}

impl Copy for xcb_render_change_picture_request_t {}
impl Clone for xcb_render_change_picture_request_t {
    fn clone(&self) -> xcb_render_change_picture_request_t { *self }
}


#[repr(C)]
pub struct xcb_render_set_picture_clip_rectangles_request_t {
     pub major_opcode :    u8,
     pub minor_opcode :    u8,
     pub length :          u16,
     pub picture :         xcb_render_picture_t,
     pub clip_x_origin :   i16,
     pub clip_y_origin :   i16
}

impl Copy for xcb_render_set_picture_clip_rectangles_request_t {}
impl Clone for xcb_render_set_picture_clip_rectangles_request_t {
    fn clone(&self) -> xcb_render_set_picture_clip_rectangles_request_t { *self }
}


#[repr(C)]
pub struct xcb_render_free_picture_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub picture :        xcb_render_picture_t
}

impl Copy for xcb_render_free_picture_request_t {}
impl Clone for xcb_render_free_picture_request_t {
    fn clone(&self) -> xcb_render_free_picture_request_t { *self }
}


#[repr(C)]
pub struct xcb_render_composite_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub op :             u8,
     pub pad0 :           [u8; 3],
     pub src :            xcb_render_picture_t,
     pub mask :           xcb_render_picture_t,
     pub dst :            xcb_render_picture_t,
     pub src_x :          i16,
     pub src_y :          i16,
     pub mask_x :         i16,
     pub mask_y :         i16,
     pub dst_x :          i16,
     pub dst_y :          i16,
     pub width :          u16,
     pub height :         u16
}

impl Copy for xcb_render_composite_request_t {}
impl Clone for xcb_render_composite_request_t {
    fn clone(&self) -> xcb_render_composite_request_t { *self }
}


#[repr(C)]
pub struct xcb_render_trapezoids_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub op :             u8,
     pub pad0 :           [u8; 3],
     pub src :            xcb_render_picture_t,
     pub dst :            xcb_render_picture_t,
     pub mask_format :    xcb_render_pictformat_t,
     pub src_x :          i16,
     pub src_y :          i16
}

impl Copy for xcb_render_trapezoids_request_t {}
impl Clone for xcb_render_trapezoids_request_t {
    fn clone(&self) -> xcb_render_trapezoids_request_t { *self }
}


#[repr(C)]
pub struct xcb_render_triangles_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub op :             u8,
     pub pad0 :           [u8; 3],
     pub src :            xcb_render_picture_t,
     pub dst :            xcb_render_picture_t,
     pub mask_format :    xcb_render_pictformat_t,
     pub src_x :          i16,
     pub src_y :          i16
}

impl Copy for xcb_render_triangles_request_t {}
impl Clone for xcb_render_triangles_request_t {
    fn clone(&self) -> xcb_render_triangles_request_t { *self }
}


#[repr(C)]
pub struct xcb_render_tri_strip_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub op :             u8,
     pub pad0 :           [u8; 3],
     pub src :            xcb_render_picture_t,
     pub dst :            xcb_render_picture_t,
     pub mask_format :    xcb_render_pictformat_t,
     pub src_x :          i16,
     pub src_y :          i16
}

impl Copy for xcb_render_tri_strip_request_t {}
impl Clone for xcb_render_tri_strip_request_t {
    fn clone(&self) -> xcb_render_tri_strip_request_t { *self }
}


#[repr(C)]
pub struct xcb_render_tri_fan_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub op :             u8,
     pub pad0 :           [u8; 3],
     pub src :            xcb_render_picture_t,
     pub dst :            xcb_render_picture_t,
     pub mask_format :    xcb_render_pictformat_t,
     pub src_x :          i16,
     pub src_y :          i16
}

impl Copy for xcb_render_tri_fan_request_t {}
impl Clone for xcb_render_tri_fan_request_t {
    fn clone(&self) -> xcb_render_tri_fan_request_t { *self }
}


#[repr(C)]
pub struct xcb_render_create_glyph_set_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub gsid :           xcb_render_glyphset_t,
     pub format :         xcb_render_pictformat_t
}

impl Copy for xcb_render_create_glyph_set_request_t {}
impl Clone for xcb_render_create_glyph_set_request_t {
    fn clone(&self) -> xcb_render_create_glyph_set_request_t { *self }
}


#[repr(C)]
pub struct xcb_render_reference_glyph_set_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub gsid :           xcb_render_glyphset_t,
     pub existing :       xcb_render_glyphset_t
}

impl Copy for xcb_render_reference_glyph_set_request_t {}
impl Clone for xcb_render_reference_glyph_set_request_t {
    fn clone(&self) -> xcb_render_reference_glyph_set_request_t { *self }
}


#[repr(C)]
pub struct xcb_render_free_glyph_set_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub glyphset :       xcb_render_glyphset_t
}

impl Copy for xcb_render_free_glyph_set_request_t {}
impl Clone for xcb_render_free_glyph_set_request_t {
    fn clone(&self) -> xcb_render_free_glyph_set_request_t { *self }
}


#[repr(C)]
pub struct xcb_render_add_glyphs_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub glyphset :       xcb_render_glyphset_t,
     pub glyphs_len :     u32
}

impl Copy for xcb_render_add_glyphs_request_t {}
impl Clone for xcb_render_add_glyphs_request_t {
    fn clone(&self) -> xcb_render_add_glyphs_request_t { *self }
}


#[repr(C)]
pub struct xcb_render_free_glyphs_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub glyphset :       xcb_render_glyphset_t
}

impl Copy for xcb_render_free_glyphs_request_t {}
impl Clone for xcb_render_free_glyphs_request_t {
    fn clone(&self) -> xcb_render_free_glyphs_request_t { *self }
}


#[repr(C)]
pub struct xcb_render_composite_glyphs_8_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub op :             u8,
     pub pad0 :           [u8; 3],
     pub src :            xcb_render_picture_t,
     pub dst :            xcb_render_picture_t,
     pub mask_format :    xcb_render_pictformat_t,
     pub glyphset :       xcb_render_glyphset_t,
     pub src_x :          i16,
     pub src_y :          i16
}

impl Copy for xcb_render_composite_glyphs_8_request_t {}
impl Clone for xcb_render_composite_glyphs_8_request_t {
    fn clone(&self) -> xcb_render_composite_glyphs_8_request_t { *self }
}


#[repr(C)]
pub struct xcb_render_composite_glyphs_16_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub op :             u8,
     pub pad0 :           [u8; 3],
     pub src :            xcb_render_picture_t,
     pub dst :            xcb_render_picture_t,
     pub mask_format :    xcb_render_pictformat_t,
     pub glyphset :       xcb_render_glyphset_t,
     pub src_x :          i16,
     pub src_y :          i16
}

impl Copy for xcb_render_composite_glyphs_16_request_t {}
impl Clone for xcb_render_composite_glyphs_16_request_t {
    fn clone(&self) -> xcb_render_composite_glyphs_16_request_t { *self }
}


#[repr(C)]
pub struct xcb_render_composite_glyphs_32_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub op :             u8,
     pub pad0 :           [u8; 3],
     pub src :            xcb_render_picture_t,
     pub dst :            xcb_render_picture_t,
     pub mask_format :    xcb_render_pictformat_t,
     pub glyphset :       xcb_render_glyphset_t,
     pub src_x :          i16,
     pub src_y :          i16
}

impl Copy for xcb_render_composite_glyphs_32_request_t {}
impl Clone for xcb_render_composite_glyphs_32_request_t {
    fn clone(&self) -> xcb_render_composite_glyphs_32_request_t { *self }
}


#[repr(C)]
pub struct xcb_render_fill_rectangles_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub op :             u8,
     pub pad0 :           [u8; 3],
     pub dst :            xcb_render_picture_t,
     pub color :          xcb_render_color_t
}

impl Copy for xcb_render_fill_rectangles_request_t {}
impl Clone for xcb_render_fill_rectangles_request_t {
    fn clone(&self) -> xcb_render_fill_rectangles_request_t { *self }
}


#[repr(C)]
pub struct xcb_render_create_cursor_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub cid :            ffi::xproto::xcb_cursor_t,
     pub source :         xcb_render_picture_t,
     pub x :              u16,
     pub y :              u16
}

impl Copy for xcb_render_create_cursor_request_t {}
impl Clone for xcb_render_create_cursor_request_t {
    fn clone(&self) -> xcb_render_create_cursor_request_t { *self }
}

#[repr(C)]
pub struct xcb_render_transform_t {
     pub matrix11 :   xcb_render_fixed_t,
     pub matrix12 :   xcb_render_fixed_t,
     pub matrix13 :   xcb_render_fixed_t,
     pub matrix21 :   xcb_render_fixed_t,
     pub matrix22 :   xcb_render_fixed_t,
     pub matrix23 :   xcb_render_fixed_t,
     pub matrix31 :   xcb_render_fixed_t,
     pub matrix32 :   xcb_render_fixed_t,
     pub matrix33 :   xcb_render_fixed_t
}

impl Copy for xcb_render_transform_t {}
impl Clone for xcb_render_transform_t {
    fn clone(&self) -> xcb_render_transform_t { *self }
}
/**
 * @brief xcb_render_transform_iterator_t
 **/
#[repr(C)]
pub struct xcb_render_transform_iterator_t {
    pub data : *mut xcb_render_transform_t,
    pub rem  : c_int,
    pub index: c_int
}



#[repr(C)]
pub struct xcb_render_set_picture_transform_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub picture :        xcb_render_picture_t,
     pub transform :      xcb_render_transform_t
}

impl Copy for xcb_render_set_picture_transform_request_t {}
impl Clone for xcb_render_set_picture_transform_request_t {
    fn clone(&self) -> xcb_render_set_picture_transform_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_render_query_filters_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_render_query_filters_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub drawable :       ffi::xproto::xcb_drawable_t
}

impl Copy for xcb_render_query_filters_request_t {}
impl Clone for xcb_render_query_filters_request_t {
    fn clone(&self) -> xcb_render_query_filters_request_t { *self }
}

#[repr(C)]
pub struct xcb_render_query_filters_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub num_aliases :     u32,
     pub num_filters :     u32,
     pub pad1 :            [u8; 16]
}

impl Copy for xcb_render_query_filters_reply_t {}
impl Clone for xcb_render_query_filters_reply_t {
    fn clone(&self) -> xcb_render_query_filters_reply_t { *self }
}


#[repr(C)]
pub struct xcb_render_set_picture_filter_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub picture :        xcb_render_picture_t,
     pub filter_len :     u16,
     pub pad0 :           [u8; 2]
}

impl Copy for xcb_render_set_picture_filter_request_t {}
impl Clone for xcb_render_set_picture_filter_request_t {
    fn clone(&self) -> xcb_render_set_picture_filter_request_t { *self }
}

#[repr(C)]
pub struct xcb_render_animcursorelt_t {
     pub cursor :   ffi::xproto::xcb_cursor_t,
     pub delay :    u32
}

impl Copy for xcb_render_animcursorelt_t {}
impl Clone for xcb_render_animcursorelt_t {
    fn clone(&self) -> xcb_render_animcursorelt_t { *self }
}
/**
 * @brief xcb_render_animcursorelt_iterator_t
 **/
#[repr(C)]
pub struct xcb_render_animcursorelt_iterator_t {
    pub data : *mut xcb_render_animcursorelt_t,
    pub rem  : c_int,
    pub index: c_int
}



#[repr(C)]
pub struct xcb_render_create_anim_cursor_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub cid :            ffi::xproto::xcb_cursor_t
}

impl Copy for xcb_render_create_anim_cursor_request_t {}
impl Clone for xcb_render_create_anim_cursor_request_t {
    fn clone(&self) -> xcb_render_create_anim_cursor_request_t { *self }
}

#[repr(C)]
pub struct xcb_render_spanfix_t {
     pub l :   xcb_render_fixed_t,
     pub r :   xcb_render_fixed_t,
     pub y :   xcb_render_fixed_t
}

impl Copy for xcb_render_spanfix_t {}
impl Clone for xcb_render_spanfix_t {
    fn clone(&self) -> xcb_render_spanfix_t { *self }
}
/**
 * @brief xcb_render_spanfix_iterator_t
 **/
#[repr(C)]
pub struct xcb_render_spanfix_iterator_t {
    pub data : *mut xcb_render_spanfix_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_render_trap_t {
     pub top :   xcb_render_spanfix_t,
     pub bot :   xcb_render_spanfix_t
}

impl Copy for xcb_render_trap_t {}
impl Clone for xcb_render_trap_t {
    fn clone(&self) -> xcb_render_trap_t { *self }
}
/**
 * @brief xcb_render_trap_iterator_t
 **/
#[repr(C)]
pub struct xcb_render_trap_iterator_t {
    pub data : *mut xcb_render_trap_t,
    pub rem  : c_int,
    pub index: c_int
}



#[repr(C)]
pub struct xcb_render_add_traps_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub picture :        xcb_render_picture_t,
     pub x_off :          i16,
     pub y_off :          i16
}

impl Copy for xcb_render_add_traps_request_t {}
impl Clone for xcb_render_add_traps_request_t {
    fn clone(&self) -> xcb_render_add_traps_request_t { *self }
}


#[repr(C)]
pub struct xcb_render_create_solid_fill_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub picture :        xcb_render_picture_t,
     pub color :          xcb_render_color_t
}

impl Copy for xcb_render_create_solid_fill_request_t {}
impl Clone for xcb_render_create_solid_fill_request_t {
    fn clone(&self) -> xcb_render_create_solid_fill_request_t { *self }
}


#[repr(C)]
pub struct xcb_render_create_linear_gradient_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub picture :        xcb_render_picture_t,
     pub p1 :             xcb_render_pointfix_t,
     pub p2 :             xcb_render_pointfix_t,
     pub num_stops :      u32
}

impl Copy for xcb_render_create_linear_gradient_request_t {}
impl Clone for xcb_render_create_linear_gradient_request_t {
    fn clone(&self) -> xcb_render_create_linear_gradient_request_t { *self }
}


#[repr(C)]
pub struct xcb_render_create_radial_gradient_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub picture :        xcb_render_picture_t,
     pub inner :          xcb_render_pointfix_t,
     pub outer :          xcb_render_pointfix_t,
     pub inner_radius :   xcb_render_fixed_t,
     pub outer_radius :   xcb_render_fixed_t,
     pub num_stops :      u32
}

impl Copy for xcb_render_create_radial_gradient_request_t {}
impl Clone for xcb_render_create_radial_gradient_request_t {
    fn clone(&self) -> xcb_render_create_radial_gradient_request_t { *self }
}


#[repr(C)]
pub struct xcb_render_create_conical_gradient_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub picture :        xcb_render_picture_t,
     pub center :         xcb_render_pointfix_t,
     pub angle :          xcb_render_fixed_t,
     pub num_stops :      u32
}

impl Copy for xcb_render_create_conical_gradient_request_t {}
impl Clone for xcb_render_create_conical_gradient_request_t {
    fn clone(&self) -> xcb_render_create_conical_gradient_request_t { *self }
}
#[link(name="xcb-render")]
extern "C" {

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_render_glyph_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_render_glyph_t)
 *
 *
 */
pub fn xcb_render_glyph_next (i:*mut xcb_render_glyph_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_render_glyph_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_render_glyph_end (i:xcb_render_glyph_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_render_glyphset_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_render_glyphset_t)
 *
 *
 */
pub fn xcb_render_glyphset_next (i:*mut xcb_render_glyphset_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_render_glyphset_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_render_glyphset_end (i:xcb_render_glyphset_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_render_picture_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_render_picture_t)
 *
 *
 */
pub fn xcb_render_picture_next (i:*mut xcb_render_picture_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_render_picture_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_render_picture_end (i:xcb_render_picture_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_render_pictformat_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_render_pictformat_t)
 *
 *
 */
pub fn xcb_render_pictformat_next (i:*mut xcb_render_pictformat_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_render_pictformat_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_render_pictformat_end (i:xcb_render_pictformat_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_render_fixed_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_render_fixed_t)
 *
 *
 */
pub fn xcb_render_fixed_next (i:*mut xcb_render_fixed_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_render_fixed_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_render_fixed_end (i:xcb_render_fixed_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_render_directformat_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_render_directformat_t)
 *
 *
 */
pub fn xcb_render_directformat_next (i:*mut xcb_render_directformat_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_render_directformat_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_render_directformat_end (i:xcb_render_directformat_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_render_pictforminfo_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_render_pictforminfo_t)
 *
 *
 */
pub fn xcb_render_pictforminfo_next (i:*mut xcb_render_pictforminfo_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_render_pictforminfo_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_render_pictforminfo_end (i:xcb_render_pictforminfo_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_render_pictvisual_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_render_pictvisual_t)
 *
 *
 */
pub fn xcb_render_pictvisual_next (i:*mut xcb_render_pictvisual_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_render_pictvisual_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_render_pictvisual_end (i:xcb_render_pictvisual_iterator_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_render_pictdepth_sizeof (_buffer :  *mut c_void) -> c_int;

pub fn xcb_render_pictdepth_visuals (R : *mut xcb_render_pictdepth_t) -> *mut xcb_render_pictvisual_t;


pub fn xcb_render_pictdepth_visuals_length (R : *mut xcb_render_pictdepth_t) -> c_int;

pub fn xcb_render_pictdepth_visuals_iterator (R : *mut xcb_render_pictdepth_t) -> xcb_render_pictvisual_iterator_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_render_pictdepth_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_render_pictdepth_t)
 *
 *
 */
pub fn xcb_render_pictdepth_next (i:*mut xcb_render_pictdepth_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_render_pictdepth_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_render_pictdepth_end (i:xcb_render_pictdepth_iterator_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_render_pictscreen_sizeof (_buffer :  *mut c_void) -> c_int;


pub fn xcb_render_pictscreen_depths_length (R : *mut xcb_render_pictscreen_t) -> c_int;

pub fn xcb_render_pictscreen_depths_iterator (R : *mut xcb_render_pictscreen_t) -> xcb_render_pictdepth_iterator_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_render_pictscreen_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_render_pictscreen_t)
 *
 *
 */
pub fn xcb_render_pictscreen_next (i:*mut xcb_render_pictscreen_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_render_pictscreen_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_render_pictscreen_end (i:xcb_render_pictscreen_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_render_indexvalue_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_render_indexvalue_t)
 *
 *
 */
pub fn xcb_render_indexvalue_next (i:*mut xcb_render_indexvalue_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_render_indexvalue_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_render_indexvalue_end (i:xcb_render_indexvalue_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_render_color_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_render_color_t)
 *
 *
 */
pub fn xcb_render_color_next (i:*mut xcb_render_color_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_render_color_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_render_color_end (i:xcb_render_color_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_render_pointfix_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_render_pointfix_t)
 *
 *
 */
pub fn xcb_render_pointfix_next (i:*mut xcb_render_pointfix_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_render_pointfix_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_render_pointfix_end (i:xcb_render_pointfix_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_render_linefix_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_render_linefix_t)
 *
 *
 */
pub fn xcb_render_linefix_next (i:*mut xcb_render_linefix_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_render_linefix_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_render_linefix_end (i:xcb_render_linefix_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_render_triangle_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_render_triangle_t)
 *
 *
 */
pub fn xcb_render_triangle_next (i:*mut xcb_render_triangle_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_render_triangle_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_render_triangle_end (i:xcb_render_triangle_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_render_trapezoid_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_render_trapezoid_t)
 *
 *
 */
pub fn xcb_render_trapezoid_next (i:*mut xcb_render_trapezoid_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_render_trapezoid_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_render_trapezoid_end (i:xcb_render_trapezoid_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_render_glyphinfo_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_render_glyphinfo_t)
 *
 *
 */
pub fn xcb_render_glyphinfo_next (i:*mut xcb_render_glyphinfo_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_render_glyphinfo_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_render_glyphinfo_end (i:xcb_render_glyphinfo_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_render_query_version (c : *mut ffi::base::xcb_connection_t,
                                    client_major_version :  u32,
                                    client_minor_version :  u32) -> xcb_render_query_version_cookie_t;

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
pub fn xcb_render_query_version_unchecked (c : *mut ffi::base::xcb_connection_t,
                                              client_major_version :  u32,
                                              client_minor_version :  u32) -> xcb_render_query_version_cookie_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_render_query_version_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_render_query_version_reply (c : *mut ffi::base::xcb_connection_t,
                                          cookie : xcb_render_query_version_cookie_t,
                                          e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_render_query_version_reply_t;

pub fn xcb_render_query_pict_formats_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_render_query_pict_formats (c : *mut ffi::base::xcb_connection_t) -> xcb_render_query_pict_formats_cookie_t;

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
pub fn xcb_render_query_pict_formats_unchecked (c : *mut ffi::base::xcb_connection_t) -> xcb_render_query_pict_formats_cookie_t;

pub fn xcb_render_query_pict_formats_formats (R : *mut xcb_render_query_pict_formats_reply_t) -> *mut xcb_render_pictforminfo_t;


pub fn xcb_render_query_pict_formats_formats_length (R : *mut xcb_render_query_pict_formats_reply_t) -> c_int;

pub fn xcb_render_query_pict_formats_formats_iterator (R : *mut xcb_render_query_pict_formats_reply_t) -> xcb_render_pictforminfo_iterator_t;


pub fn xcb_render_query_pict_formats_screens_length (R : *mut xcb_render_query_pict_formats_reply_t) -> c_int;

pub fn xcb_render_query_pict_formats_screens_iterator (R : *mut xcb_render_query_pict_formats_reply_t) -> xcb_render_pictscreen_iterator_t;

pub fn xcb_render_query_pict_formats_subpixels (R : *mut xcb_render_query_pict_formats_reply_t) -> *mut u32;


pub fn xcb_render_query_pict_formats_subpixels_length (R : *mut xcb_render_query_pict_formats_reply_t) -> c_int;


pub fn xcb_render_query_pict_formats_subpixels_end (R : *mut xcb_render_query_pict_formats_reply_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_render_query_pict_formats_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_render_query_pict_formats_reply (c : *mut ffi::base::xcb_connection_t,
                                               cookie : xcb_render_query_pict_formats_cookie_t,
                                               e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_render_query_pict_formats_reply_t;

pub fn xcb_render_query_pict_index_values_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_render_query_pict_index_values (c : *mut ffi::base::xcb_connection_t,
                                              format :  xcb_render_pictformat_t) -> xcb_render_query_pict_index_values_cookie_t;

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
pub fn xcb_render_query_pict_index_values_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                        format :  xcb_render_pictformat_t) -> xcb_render_query_pict_index_values_cookie_t;

pub fn xcb_render_query_pict_index_values_values (R : *mut xcb_render_query_pict_index_values_reply_t) -> *mut xcb_render_indexvalue_t;


pub fn xcb_render_query_pict_index_values_values_length (R : *mut xcb_render_query_pict_index_values_reply_t) -> c_int;

pub fn xcb_render_query_pict_index_values_values_iterator (R : *mut xcb_render_query_pict_index_values_reply_t) -> xcb_render_indexvalue_iterator_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_render_query_pict_index_values_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_render_query_pict_index_values_reply (c : *mut ffi::base::xcb_connection_t,
                                                    cookie : xcb_render_query_pict_index_values_cookie_t,
                                                    e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_render_query_pict_index_values_reply_t;

pub fn xcb_render_create_picture_sizeof (_buffer :  *mut c_void) -> c_int;

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
pub fn xcb_render_create_picture_checked (c : *mut ffi::base::xcb_connection_t,
                                             pid :  xcb_render_picture_t,
                                             drawable :  ffi::xproto::xcb_drawable_t,
                                             format :  xcb_render_pictformat_t,
                                             value_mask :  u32,
                                             value_list : *mut u32) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_render_create_picture (c : *mut ffi::base::xcb_connection_t,
                                     pid :  xcb_render_picture_t,
                                     drawable :  ffi::xproto::xcb_drawable_t,
                                     format :  xcb_render_pictformat_t,
                                     value_mask :  u32,
                                     value_list : *mut u32) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_render_change_picture_sizeof (_buffer :  *mut c_void) -> c_int;

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
pub fn xcb_render_change_picture_checked (c : *mut ffi::base::xcb_connection_t,
                                             picture :  xcb_render_picture_t,
                                             value_mask :  u32,
                                             value_list : *mut u32) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_render_change_picture (c : *mut ffi::base::xcb_connection_t,
                                     picture :  xcb_render_picture_t,
                                     value_mask :  u32,
                                     value_list : *mut u32) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_render_set_picture_clip_rectangles_sizeof (_buffer :  *mut c_void,
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
pub fn xcb_render_set_picture_clip_rectangles_checked (c : *mut ffi::base::xcb_connection_t,
                                                          picture :  xcb_render_picture_t,
                                                          clip_x_origin :  i16,
                                                          clip_y_origin :  i16,
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
pub fn xcb_render_set_picture_clip_rectangles (c : *mut ffi::base::xcb_connection_t,
                                                  picture :  xcb_render_picture_t,
                                                  clip_x_origin :  i16,
                                                  clip_y_origin :  i16,
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
pub fn xcb_render_free_picture_checked (c : *mut ffi::base::xcb_connection_t,
                                           picture :  xcb_render_picture_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_render_free_picture (c : *mut ffi::base::xcb_connection_t,
                                   picture :  xcb_render_picture_t) -> ffi::base::xcb_void_cookie_t;

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
pub fn xcb_render_composite_checked (c : *mut ffi::base::xcb_connection_t,
                                        op :  u8,
                                        src :  xcb_render_picture_t,
                                        mask :  xcb_render_picture_t,
                                        dst :  xcb_render_picture_t,
                                        src_x :  i16,
                                        src_y :  i16,
                                        mask_x :  i16,
                                        mask_y :  i16,
                                        dst_x :  i16,
                                        dst_y :  i16,
                                        width :  u16,
                                        height :  u16) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_render_composite (c : *mut ffi::base::xcb_connection_t,
                                op :  u8,
                                src :  xcb_render_picture_t,
                                mask :  xcb_render_picture_t,
                                dst :  xcb_render_picture_t,
                                src_x :  i16,
                                src_y :  i16,
                                mask_x :  i16,
                                mask_y :  i16,
                                dst_x :  i16,
                                dst_y :  i16,
                                width :  u16,
                                height :  u16) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_render_trapezoids_sizeof (_buffer :  *mut c_void,
                              traps_len :    u32) -> c_int;

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
pub fn xcb_render_trapezoids_checked (c : *mut ffi::base::xcb_connection_t,
                                         op :  u8,
                                         src :  xcb_render_picture_t,
                                         dst :  xcb_render_picture_t,
                                         mask_format :  xcb_render_pictformat_t,
                                         src_x :  i16,
                                         src_y :  i16,
                                         traps_len :  u32,
                                         traps : *mut xcb_render_trapezoid_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_render_trapezoids (c : *mut ffi::base::xcb_connection_t,
                                 op :  u8,
                                 src :  xcb_render_picture_t,
                                 dst :  xcb_render_picture_t,
                                 mask_format :  xcb_render_pictformat_t,
                                 src_x :  i16,
                                 src_y :  i16,
                                 traps_len :  u32,
                                 traps : *mut xcb_render_trapezoid_t) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_render_triangles_sizeof (_buffer :  *mut c_void,
                             triangles_len :  u32) -> c_int;

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
pub fn xcb_render_triangles_checked (c : *mut ffi::base::xcb_connection_t,
                                        op :  u8,
                                        src :  xcb_render_picture_t,
                                        dst :  xcb_render_picture_t,
                                        mask_format :  xcb_render_pictformat_t,
                                        src_x :  i16,
                                        src_y :  i16,
                                        triangles_len :  u32,
                                        triangles : *mut xcb_render_triangle_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_render_triangles (c : *mut ffi::base::xcb_connection_t,
                                op :  u8,
                                src :  xcb_render_picture_t,
                                dst :  xcb_render_picture_t,
                                mask_format :  xcb_render_pictformat_t,
                                src_x :  i16,
                                src_y :  i16,
                                triangles_len :  u32,
                                triangles : *mut xcb_render_triangle_t) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_render_tri_strip_sizeof (_buffer :  *mut c_void,
                             points_len :   u32) -> c_int;

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
pub fn xcb_render_tri_strip_checked (c : *mut ffi::base::xcb_connection_t,
                                        op :  u8,
                                        src :  xcb_render_picture_t,
                                        dst :  xcb_render_picture_t,
                                        mask_format :  xcb_render_pictformat_t,
                                        src_x :  i16,
                                        src_y :  i16,
                                        points_len :  u32,
                                        points : *mut xcb_render_pointfix_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_render_tri_strip (c : *mut ffi::base::xcb_connection_t,
                                op :  u8,
                                src :  xcb_render_picture_t,
                                dst :  xcb_render_picture_t,
                                mask_format :  xcb_render_pictformat_t,
                                src_x :  i16,
                                src_y :  i16,
                                points_len :  u32,
                                points : *mut xcb_render_pointfix_t) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_render_tri_fan_sizeof (_buffer :  *mut c_void,
                           points_len :   u32) -> c_int;

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
pub fn xcb_render_tri_fan_checked (c : *mut ffi::base::xcb_connection_t,
                                      op :  u8,
                                      src :  xcb_render_picture_t,
                                      dst :  xcb_render_picture_t,
                                      mask_format :  xcb_render_pictformat_t,
                                      src_x :  i16,
                                      src_y :  i16,
                                      points_len :  u32,
                                      points : *mut xcb_render_pointfix_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_render_tri_fan (c : *mut ffi::base::xcb_connection_t,
                              op :  u8,
                              src :  xcb_render_picture_t,
                              dst :  xcb_render_picture_t,
                              mask_format :  xcb_render_pictformat_t,
                              src_x :  i16,
                              src_y :  i16,
                              points_len :  u32,
                              points : *mut xcb_render_pointfix_t) -> ffi::base::xcb_void_cookie_t;

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
pub fn xcb_render_create_glyph_set_checked (c : *mut ffi::base::xcb_connection_t,
                                               gsid :  xcb_render_glyphset_t,
                                               format :  xcb_render_pictformat_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_render_create_glyph_set (c : *mut ffi::base::xcb_connection_t,
                                       gsid :  xcb_render_glyphset_t,
                                       format :  xcb_render_pictformat_t) -> ffi::base::xcb_void_cookie_t;

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
pub fn xcb_render_reference_glyph_set_checked (c : *mut ffi::base::xcb_connection_t,
                                                  gsid :  xcb_render_glyphset_t,
                                                  existing :  xcb_render_glyphset_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_render_reference_glyph_set (c : *mut ffi::base::xcb_connection_t,
                                          gsid :  xcb_render_glyphset_t,
                                          existing :  xcb_render_glyphset_t) -> ffi::base::xcb_void_cookie_t;

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
pub fn xcb_render_free_glyph_set_checked (c : *mut ffi::base::xcb_connection_t,
                                             glyphset :  xcb_render_glyphset_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_render_free_glyph_set (c : *mut ffi::base::xcb_connection_t,
                                     glyphset :  xcb_render_glyphset_t) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_render_add_glyphs_sizeof (_buffer :  *mut c_void,
                              data_len :     u32) -> c_int;

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
pub fn xcb_render_add_glyphs_checked (c : *mut ffi::base::xcb_connection_t,
                                         glyphset :  xcb_render_glyphset_t,
                                         glyphs_len :  u32,
                                         glyphids : *mut u32,
                                         glyphs : *mut xcb_render_glyphinfo_t,
                                         data_len :  u32,
                                         data : *mut u8) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_render_add_glyphs (c : *mut ffi::base::xcb_connection_t,
                                 glyphset :  xcb_render_glyphset_t,
                                 glyphs_len :  u32,
                                 glyphids : *mut u32,
                                 glyphs : *mut xcb_render_glyphinfo_t,
                                 data_len :  u32,
                                 data : *mut u8) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_render_free_glyphs_sizeof (_buffer :  *mut c_void,
                               glyphs_len :   u32) -> c_int;

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
pub fn xcb_render_free_glyphs_checked (c : *mut ffi::base::xcb_connection_t,
                                          glyphset :  xcb_render_glyphset_t,
                                          glyphs_len :  u32,
                                          glyphs : *mut xcb_render_glyph_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_render_free_glyphs (c : *mut ffi::base::xcb_connection_t,
                                  glyphset :  xcb_render_glyphset_t,
                                  glyphs_len :  u32,
                                  glyphs : *mut xcb_render_glyph_t) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_render_composite_glyphs_8_sizeof (_buffer :  *mut c_void,
                                      glyphcmds_len :  u32) -> c_int;

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
pub fn xcb_render_composite_glyphs_8_checked (c : *mut ffi::base::xcb_connection_t,
                                                 op :  u8,
                                                 src :  xcb_render_picture_t,
                                                 dst :  xcb_render_picture_t,
                                                 mask_format :  xcb_render_pictformat_t,
                                                 glyphset :  xcb_render_glyphset_t,
                                                 src_x :  i16,
                                                 src_y :  i16,
                                                 glyphcmds_len :  u32,
                                                 glyphcmds : *mut u8) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_render_composite_glyphs_8 (c : *mut ffi::base::xcb_connection_t,
                                         op :  u8,
                                         src :  xcb_render_picture_t,
                                         dst :  xcb_render_picture_t,
                                         mask_format :  xcb_render_pictformat_t,
                                         glyphset :  xcb_render_glyphset_t,
                                         src_x :  i16,
                                         src_y :  i16,
                                         glyphcmds_len :  u32,
                                         glyphcmds : *mut u8) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_render_composite_glyphs_16_sizeof (_buffer :  *mut c_void,
                                       glyphcmds_len :  u32) -> c_int;

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
pub fn xcb_render_composite_glyphs_16_checked (c : *mut ffi::base::xcb_connection_t,
                                                  op :  u8,
                                                  src :  xcb_render_picture_t,
                                                  dst :  xcb_render_picture_t,
                                                  mask_format :  xcb_render_pictformat_t,
                                                  glyphset :  xcb_render_glyphset_t,
                                                  src_x :  i16,
                                                  src_y :  i16,
                                                  glyphcmds_len :  u32,
                                                  glyphcmds : *mut u8) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_render_composite_glyphs_16 (c : *mut ffi::base::xcb_connection_t,
                                          op :  u8,
                                          src :  xcb_render_picture_t,
                                          dst :  xcb_render_picture_t,
                                          mask_format :  xcb_render_pictformat_t,
                                          glyphset :  xcb_render_glyphset_t,
                                          src_x :  i16,
                                          src_y :  i16,
                                          glyphcmds_len :  u32,
                                          glyphcmds : *mut u8) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_render_composite_glyphs_32_sizeof (_buffer :  *mut c_void,
                                       glyphcmds_len :  u32) -> c_int;

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
pub fn xcb_render_composite_glyphs_32_checked (c : *mut ffi::base::xcb_connection_t,
                                                  op :  u8,
                                                  src :  xcb_render_picture_t,
                                                  dst :  xcb_render_picture_t,
                                                  mask_format :  xcb_render_pictformat_t,
                                                  glyphset :  xcb_render_glyphset_t,
                                                  src_x :  i16,
                                                  src_y :  i16,
                                                  glyphcmds_len :  u32,
                                                  glyphcmds : *mut u8) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_render_composite_glyphs_32 (c : *mut ffi::base::xcb_connection_t,
                                          op :  u8,
                                          src :  xcb_render_picture_t,
                                          dst :  xcb_render_picture_t,
                                          mask_format :  xcb_render_pictformat_t,
                                          glyphset :  xcb_render_glyphset_t,
                                          src_x :  i16,
                                          src_y :  i16,
                                          glyphcmds_len :  u32,
                                          glyphcmds : *mut u8) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_render_fill_rectangles_sizeof (_buffer :  *mut c_void,
                                   rects_len :    u32) -> c_int;

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
pub fn xcb_render_fill_rectangles_checked (c : *mut ffi::base::xcb_connection_t,
                                              op :  u8,
                                              dst :  xcb_render_picture_t,
                                              color :  xcb_render_color_t,
                                              rects_len :  u32,
                                              rects : *mut ffi::xproto::xcb_rectangle_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_render_fill_rectangles (c : *mut ffi::base::xcb_connection_t,
                                      op :  u8,
                                      dst :  xcb_render_picture_t,
                                      color :  xcb_render_color_t,
                                      rects_len :  u32,
                                      rects : *mut ffi::xproto::xcb_rectangle_t) -> ffi::base::xcb_void_cookie_t;

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
pub fn xcb_render_create_cursor_checked (c : *mut ffi::base::xcb_connection_t,
                                            cid :  ffi::xproto::xcb_cursor_t,
                                            source :  xcb_render_picture_t,
                                            x :  u16,
                                            y :  u16) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_render_create_cursor (c : *mut ffi::base::xcb_connection_t,
                                    cid :  ffi::xproto::xcb_cursor_t,
                                    source :  xcb_render_picture_t,
                                    x :  u16,
                                    y :  u16) -> ffi::base::xcb_void_cookie_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_render_transform_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_render_transform_t)
 *
 *
 */
pub fn xcb_render_transform_next (i:*mut xcb_render_transform_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_render_transform_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_render_transform_end (i:xcb_render_transform_iterator_t) -> ffi::base::xcb_generic_iterator_t;

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
pub fn xcb_render_set_picture_transform_checked (c : *mut ffi::base::xcb_connection_t,
                                                    picture :  xcb_render_picture_t,
                                                    transform :  xcb_render_transform_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_render_set_picture_transform (c : *mut ffi::base::xcb_connection_t,
                                            picture :  xcb_render_picture_t,
                                            transform :  xcb_render_transform_t) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_render_query_filters_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_render_query_filters (c : *mut ffi::base::xcb_connection_t,
                                    drawable :  ffi::xproto::xcb_drawable_t) -> xcb_render_query_filters_cookie_t;

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
pub fn xcb_render_query_filters_unchecked (c : *mut ffi::base::xcb_connection_t,
                                              drawable :  ffi::xproto::xcb_drawable_t) -> xcb_render_query_filters_cookie_t;

pub fn xcb_render_query_filters_aliases (R : *mut xcb_render_query_filters_reply_t) -> *mut u16;


pub fn xcb_render_query_filters_aliases_length (R : *mut xcb_render_query_filters_reply_t) -> c_int;


pub fn xcb_render_query_filters_aliases_end (R : *mut xcb_render_query_filters_reply_t) -> ffi::base::xcb_generic_iterator_t;


pub fn xcb_render_query_filters_filters_length (R : *mut xcb_render_query_filters_reply_t) -> c_int;

pub fn xcb_render_query_filters_filters_iterator (R : *mut xcb_render_query_filters_reply_t) -> ffi::xproto::xcb_str_iterator_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_render_query_filters_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_render_query_filters_reply (c : *mut ffi::base::xcb_connection_t,
                                          cookie : xcb_render_query_filters_cookie_t,
                                          e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_render_query_filters_reply_t;

pub fn xcb_render_set_picture_filter_sizeof (_buffer :  *mut c_void,
                                      values_len :   u32) -> c_int;

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
pub fn xcb_render_set_picture_filter_checked (c : *mut ffi::base::xcb_connection_t,
                                                 picture :  xcb_render_picture_t,
                                                 filter_len :  u16,
                                                 filter : *mut c_char,
                                                 values_len :  u32,
                                                 values : *mut xcb_render_fixed_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_render_set_picture_filter (c : *mut ffi::base::xcb_connection_t,
                                         picture :  xcb_render_picture_t,
                                         filter_len :  u16,
                                         filter : *mut c_char,
                                         values_len :  u32,
                                         values : *mut xcb_render_fixed_t) -> ffi::base::xcb_void_cookie_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_render_animcursorelt_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_render_animcursorelt_t)
 *
 *
 */
pub fn xcb_render_animcursorelt_next (i:*mut xcb_render_animcursorelt_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_render_animcursorelt_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_render_animcursorelt_end (i:xcb_render_animcursorelt_iterator_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_render_create_anim_cursor_sizeof (_buffer :  *mut c_void,
                                      cursors_len :  u32) -> c_int;

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
pub fn xcb_render_create_anim_cursor_checked (c : *mut ffi::base::xcb_connection_t,
                                                 cid :  ffi::xproto::xcb_cursor_t,
                                                 cursors_len :  u32,
                                                 cursors : *mut xcb_render_animcursorelt_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_render_create_anim_cursor (c : *mut ffi::base::xcb_connection_t,
                                         cid :  ffi::xproto::xcb_cursor_t,
                                         cursors_len :  u32,
                                         cursors : *mut xcb_render_animcursorelt_t) -> ffi::base::xcb_void_cookie_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_render_spanfix_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_render_spanfix_t)
 *
 *
 */
pub fn xcb_render_spanfix_next (i:*mut xcb_render_spanfix_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_render_spanfix_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_render_spanfix_end (i:xcb_render_spanfix_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_render_trap_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_render_trap_t)
 *
 *
 */
pub fn xcb_render_trap_next (i:*mut xcb_render_trap_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_render_trap_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_render_trap_end (i:xcb_render_trap_iterator_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_render_add_traps_sizeof (_buffer :  *mut c_void,
                             traps_len :    u32) -> c_int;

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
pub fn xcb_render_add_traps_checked (c : *mut ffi::base::xcb_connection_t,
                                        picture :  xcb_render_picture_t,
                                        x_off :  i16,
                                        y_off :  i16,
                                        traps_len :  u32,
                                        traps : *mut xcb_render_trap_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_render_add_traps (c : *mut ffi::base::xcb_connection_t,
                                picture :  xcb_render_picture_t,
                                x_off :  i16,
                                y_off :  i16,
                                traps_len :  u32,
                                traps : *mut xcb_render_trap_t) -> ffi::base::xcb_void_cookie_t;

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
pub fn xcb_render_create_solid_fill_checked (c : *mut ffi::base::xcb_connection_t,
                                                picture :  xcb_render_picture_t,
                                                color :  xcb_render_color_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_render_create_solid_fill (c : *mut ffi::base::xcb_connection_t,
                                        picture :  xcb_render_picture_t,
                                        color :  xcb_render_color_t) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_render_create_linear_gradient_sizeof (_buffer :  *mut c_void) -> c_int;

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
pub fn xcb_render_create_linear_gradient_checked (c : *mut ffi::base::xcb_connection_t,
                                                     picture :  xcb_render_picture_t,
                                                     p1 :  xcb_render_pointfix_t,
                                                     p2 :  xcb_render_pointfix_t,
                                                     num_stops :  u32,
                                                     stops : *mut xcb_render_fixed_t,
                                                     colors : *mut xcb_render_color_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_render_create_linear_gradient (c : *mut ffi::base::xcb_connection_t,
                                             picture :  xcb_render_picture_t,
                                             p1 :  xcb_render_pointfix_t,
                                             p2 :  xcb_render_pointfix_t,
                                             num_stops :  u32,
                                             stops : *mut xcb_render_fixed_t,
                                             colors : *mut xcb_render_color_t) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_render_create_radial_gradient_sizeof (_buffer :  *mut c_void) -> c_int;

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
pub fn xcb_render_create_radial_gradient_checked (c : *mut ffi::base::xcb_connection_t,
                                                     picture :  xcb_render_picture_t,
                                                     inner :  xcb_render_pointfix_t,
                                                     outer :  xcb_render_pointfix_t,
                                                     inner_radius :  xcb_render_fixed_t,
                                                     outer_radius :  xcb_render_fixed_t,
                                                     num_stops :  u32,
                                                     stops : *mut xcb_render_fixed_t,
                                                     colors : *mut xcb_render_color_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_render_create_radial_gradient (c : *mut ffi::base::xcb_connection_t,
                                             picture :  xcb_render_picture_t,
                                             inner :  xcb_render_pointfix_t,
                                             outer :  xcb_render_pointfix_t,
                                             inner_radius :  xcb_render_fixed_t,
                                             outer_radius :  xcb_render_fixed_t,
                                             num_stops :  u32,
                                             stops : *mut xcb_render_fixed_t,
                                             colors : *mut xcb_render_color_t) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_render_create_conical_gradient_sizeof (_buffer :  *mut c_void) -> c_int;

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
pub fn xcb_render_create_conical_gradient_checked (c : *mut ffi::base::xcb_connection_t,
                                                      picture :  xcb_render_picture_t,
                                                      center :  xcb_render_pointfix_t,
                                                      angle :  xcb_render_fixed_t,
                                                      num_stops :  u32,
                                                      stops : *mut xcb_render_fixed_t,
                                                      colors : *mut xcb_render_color_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_render_create_conical_gradient (c : *mut ffi::base::xcb_connection_t,
                                              picture :  xcb_render_picture_t,
                                              center :  xcb_render_pointfix_t,
                                              angle :  xcb_render_fixed_t,
                                              num_stops :  u32,
                                              stops : *mut xcb_render_fixed_t,
                                              colors : *mut xcb_render_color_t) -> ffi::base::xcb_void_cookie_t;
}

