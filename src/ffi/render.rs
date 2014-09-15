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

pub static RENDER_MAJOR_VERSION : c_uint = 0;
pub static RENDER_MINOR_VERSION : c_uint = 11;

pub type glyph = u32;
/**
 * @brief glyph_iterator
 **/
pub struct glyph_iterator {
    pub data : *mut glyph,
    pub rem  : c_int,
    pub index: c_int
}


pub type glyphset = u32;
/**
 * @brief glyphset_iterator
 **/
pub struct glyphset_iterator {
    pub data : *mut glyphset,
    pub rem  : c_int,
    pub index: c_int
}


pub type picture = u32;
/**
 * @brief picture_iterator
 **/
pub struct picture_iterator {
    pub data : *mut picture,
    pub rem  : c_int,
    pub index: c_int
}


pub type pictformat = u32;
/**
 * @brief pictformat_iterator
 **/
pub struct pictformat_iterator {
    pub data : *mut pictformat,
    pub rem  : c_int,
    pub index: c_int
}


pub type fixed = i32;
/**
 * @brief fixed_iterator
 **/
pub struct fixed_iterator {
    pub data : *mut fixed,
    pub rem  : c_int,
    pub index: c_int
}



pub struct pict_format_error {
     pub response_type :   u8,
     pub error_code :      u8,
     pub sequence :        u16
}



pub struct picture_error {
     pub response_type :   u8,
     pub error_code :      u8,
     pub sequence :        u16
}



pub struct pict_op_error {
     pub response_type :   u8,
     pub error_code :      u8,
     pub sequence :        u16
}



pub struct glyph_set_error {
     pub response_type :   u8,
     pub error_code :      u8,
     pub sequence :        u16
}



pub struct glyph_error {
     pub response_type :   u8,
     pub error_code :      u8,
     pub sequence :        u16
}


pub struct directformat {
     pub red_shift :     u16,
     pub red_mask :      u16,
     pub green_shift :   u16,
     pub green_mask :    u16,
     pub blue_shift :    u16,
     pub blue_mask :     u16,
     pub alpha_shift :   u16,
     pub alpha_mask :    u16
}

/**
 * @brief directformat_iterator
 **/
pub struct directformat_iterator {
    pub data : *mut directformat,
    pub rem  : c_int,
    pub index: c_int
}


pub struct pictforminfo {
     pub id :         pictformat,
     pub type_ :      u8,
     pub depth :      u8,
     pub pad0 :       [u8,..2],
     pub direct :     directformat,
     pub colormap :   ffi::xproto::colormap
}

/**
 * @brief pictforminfo_iterator
 **/
pub struct pictforminfo_iterator {
    pub data : *mut pictforminfo,
    pub rem  : c_int,
    pub index: c_int
}


pub struct pictvisual {
     pub visual :   ffi::xproto::visualid,
     pub format :   pictformat
}

/**
 * @brief pictvisual_iterator
 **/
pub struct pictvisual_iterator {
    pub data : *mut pictvisual,
    pub rem  : c_int,
    pub index: c_int
}


pub struct pictdepth {
     pub depth :         u8,
     pub pad0 :          u8,
     pub num_visuals :   u16,
     pub pad1 :          [u8,..4]
}

/**
 * @brief pictdepth_iterator
 **/
pub struct pictdepth_iterator {
    pub data : *mut pictdepth,
    pub rem  : c_int,
    pub index: c_int
}


pub struct pictscreen {
     pub num_depths :   u32,
     pub fallback :     pictformat
}

/**
 * @brief pictscreen_iterator
 **/
pub struct pictscreen_iterator {
    pub data : *mut pictscreen,
    pub rem  : c_int,
    pub index: c_int
}


pub struct indexvalue {
     pub pixel :   u32,
     pub red :     u16,
     pub green :   u16,
     pub blue :    u16,
     pub alpha :   u16
}

/**
 * @brief indexvalue_iterator
 **/
pub struct indexvalue_iterator {
    pub data : *mut indexvalue,
    pub rem  : c_int,
    pub index: c_int
}


pub struct color {
     pub red :     u16,
     pub green :   u16,
     pub blue :    u16,
     pub alpha :   u16
}

/**
 * @brief color_iterator
 **/
pub struct color_iterator {
    pub data : *mut color,
    pub rem  : c_int,
    pub index: c_int
}


pub struct pointfix {
     pub x :   fixed,
     pub y :   fixed
}

/**
 * @brief pointfix_iterator
 **/
pub struct pointfix_iterator {
    pub data : *mut pointfix,
    pub rem  : c_int,
    pub index: c_int
}


pub struct linefix {
     pub p1 :   pointfix,
     pub p2 :   pointfix
}

/**
 * @brief linefix_iterator
 **/
pub struct linefix_iterator {
    pub data : *mut linefix,
    pub rem  : c_int,
    pub index: c_int
}


pub struct triangle {
     pub p1 :   pointfix,
     pub p2 :   pointfix,
     pub p3 :   pointfix
}

/**
 * @brief triangle_iterator
 **/
pub struct triangle_iterator {
    pub data : *mut triangle,
    pub rem  : c_int,
    pub index: c_int
}


pub struct trapezoid {
     pub top :      fixed,
     pub bottom :   fixed,
     pub left :     linefix,
     pub right :    linefix
}

/**
 * @brief trapezoid_iterator
 **/
pub struct trapezoid_iterator {
    pub data : *mut trapezoid,
    pub rem  : c_int,
    pub index: c_int
}


pub struct glyphinfo {
     pub width :    u16,
     pub height :   u16,
     pub x :        i16,
     pub y :        i16,
     pub x_off :    i16,
     pub y_off :    i16
}

/**
 * @brief glyphinfo_iterator
 **/
pub struct glyphinfo_iterator {
    pub data : *mut glyphinfo,
    pub rem  : c_int,
    pub index: c_int
}


pub struct query_version_cookie {
    sequence : c_uint
}


pub struct query_version_request {
     pub major_opcode :           u8,
     pub minor_opcode :           u8,
     pub length :                 u16,
     pub client_major_version :   u32,
     pub client_minor_version :   u32
}


pub struct query_version_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub major_version :   u32,
     pub minor_version :   u32,
     pub pad1 :            [u8,..16]
}


pub struct query_pict_formats_cookie {
    sequence : c_uint
}


pub struct query_pict_formats_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16
}


pub struct query_pict_formats_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub num_formats :     u32,
     pub num_screens :     u32,
     pub num_depths :      u32,
     pub num_visuals :     u32,
     pub num_subpixel :    u32,
     pub pad1 :            [u8,..4]
}


pub struct query_pict_index_values_cookie {
    sequence : c_uint
}


pub struct query_pict_index_values_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub format :         pictformat
}


pub struct query_pict_index_values_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub num_values :      u32,
     pub pad1 :            [u8,..20]
}



pub struct create_picture_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub pid :            picture,
     pub drawable :       ffi::xproto::drawable,
     pub format :         pictformat,
     pub value_mask :     u32
}



pub struct change_picture_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub picture :        picture,
     pub value_mask :     u32
}



pub struct set_picture_clip_rectangles_request {
     pub major_opcode :    u8,
     pub minor_opcode :    u8,
     pub length :          u16,
     pub picture :         picture,
     pub clip_x_origin :   i16,
     pub clip_y_origin :   i16
}



pub struct free_picture_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub picture :        picture
}



pub struct composite_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub op :             u8,
     pub pad0 :           [u8,..3],
     pub src :            picture,
     pub mask :           picture,
     pub dst :            picture,
     pub src_x :          i16,
     pub src_y :          i16,
     pub mask_x :         i16,
     pub mask_y :         i16,
     pub dst_x :          i16,
     pub dst_y :          i16,
     pub width :          u16,
     pub height :         u16
}



pub struct trapezoids_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub op :             u8,
     pub pad0 :           [u8,..3],
     pub src :            picture,
     pub dst :            picture,
     pub mask_format :    pictformat,
     pub src_x :          i16,
     pub src_y :          i16
}



pub struct triangles_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub op :             u8,
     pub pad0 :           [u8,..3],
     pub src :            picture,
     pub dst :            picture,
     pub mask_format :    pictformat,
     pub src_x :          i16,
     pub src_y :          i16
}



pub struct tri_strip_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub op :             u8,
     pub pad0 :           [u8,..3],
     pub src :            picture,
     pub dst :            picture,
     pub mask_format :    pictformat,
     pub src_x :          i16,
     pub src_y :          i16
}



pub struct tri_fan_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub op :             u8,
     pub pad0 :           [u8,..3],
     pub src :            picture,
     pub dst :            picture,
     pub mask_format :    pictformat,
     pub src_x :          i16,
     pub src_y :          i16
}



pub struct create_glyph_set_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub gsid :           glyphset,
     pub format :         pictformat
}



pub struct reference_glyph_set_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub gsid :           glyphset,
     pub existing :       glyphset
}



pub struct free_glyph_set_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub glyphset :       glyphset
}



pub struct add_glyphs_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub glyphset :       glyphset,
     pub glyphs_len :     u32
}



pub struct free_glyphs_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub glyphset :       glyphset
}



pub struct composite_glyphs_8_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub op :             u8,
     pub pad0 :           [u8,..3],
     pub src :            picture,
     pub dst :            picture,
     pub mask_format :    pictformat,
     pub glyphset :       glyphset,
     pub src_x :          i16,
     pub src_y :          i16
}



pub struct composite_glyphs_16_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub op :             u8,
     pub pad0 :           [u8,..3],
     pub src :            picture,
     pub dst :            picture,
     pub mask_format :    pictformat,
     pub glyphset :       glyphset,
     pub src_x :          i16,
     pub src_y :          i16
}



pub struct composite_glyphs_32_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub op :             u8,
     pub pad0 :           [u8,..3],
     pub src :            picture,
     pub dst :            picture,
     pub mask_format :    pictformat,
     pub glyphset :       glyphset,
     pub src_x :          i16,
     pub src_y :          i16
}



pub struct fill_rectangles_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub op :             u8,
     pub pad0 :           [u8,..3],
     pub dst :            picture,
     pub color :          color
}



pub struct create_cursor_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub cid :            ffi::xproto::cursor,
     pub source :         picture,
     pub x :              u16,
     pub y :              u16
}


pub struct transform {
     pub matrix11 :   fixed,
     pub matrix12 :   fixed,
     pub matrix13 :   fixed,
     pub matrix21 :   fixed,
     pub matrix22 :   fixed,
     pub matrix23 :   fixed,
     pub matrix31 :   fixed,
     pub matrix32 :   fixed,
     pub matrix33 :   fixed
}

/**
 * @brief transform_iterator
 **/
pub struct transform_iterator {
    pub data : *mut transform,
    pub rem  : c_int,
    pub index: c_int
}



pub struct set_picture_transform_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub picture :        picture,
     pub transform :      transform
}


pub struct query_filters_cookie {
    sequence : c_uint
}


pub struct query_filters_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub drawable :       ffi::xproto::drawable
}


pub struct query_filters_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub num_aliases :     u32,
     pub num_filters :     u32,
     pub pad1 :            [u8,..16]
}



pub struct set_picture_filter_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub picture :        picture,
     pub filter_len :     u16,
     pub pad0 :           [u8,..2]
}


pub struct animcursorelt {
     pub cursor :   ffi::xproto::cursor,
     pub delay :    u32
}

/**
 * @brief animcursorelt_iterator
 **/
pub struct animcursorelt_iterator {
    pub data : *mut animcursorelt,
    pub rem  : c_int,
    pub index: c_int
}



pub struct create_anim_cursor_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub cid :            ffi::xproto::cursor
}


pub struct spanfix {
     pub l :   fixed,
     pub r :   fixed,
     pub y :   fixed
}

/**
 * @brief spanfix_iterator
 **/
pub struct spanfix_iterator {
    pub data : *mut spanfix,
    pub rem  : c_int,
    pub index: c_int
}


pub struct trap {
     pub top :   spanfix,
     pub bot :   spanfix
}

/**
 * @brief trap_iterator
 **/
pub struct trap_iterator {
    pub data : *mut trap,
    pub rem  : c_int,
    pub index: c_int
}



pub struct add_traps_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub picture :        picture,
     pub x_off :          i16,
     pub y_off :          i16
}



pub struct create_solid_fill_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub picture :        picture,
     pub color :          color
}



pub struct create_linear_gradient_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub picture :        picture,
     pub p1 :             pointfix,
     pub p2 :             pointfix,
     pub num_stops :      u32
}



pub struct create_radial_gradient_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub picture :        picture,
     pub inner :          pointfix,
     pub outer :          pointfix,
     pub inner_radius :   fixed,
     pub outer_radius :   fixed,
     pub num_stops :      u32
}



pub struct create_conical_gradient_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub picture :        picture,
     pub center :         pointfix,
     pub angle :          fixed,
     pub num_stops :      u32
}

#[link(name="lxcb-render")]
extern "C" {

/**
 * Get the next element of the iterator
 * @param i Pointer to a glyph_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(glyph)
 *
 *
 */
pub fn xcb_render_glyph_next (i:*mut glyph_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An glyph_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_render_glyph_end (i:glyph_iterator) -> ffi::base::generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a glyphset_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(glyphset)
 *
 *
 */
pub fn xcb_render_glyphset_next (i:*mut glyphset_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An glyphset_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_render_glyphset_end (i:glyphset_iterator) -> ffi::base::generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a picture_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(picture)
 *
 *
 */
pub fn xcb_render_picture_next (i:*mut picture_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An picture_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_render_picture_end (i:picture_iterator) -> ffi::base::generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a pictformat_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(pictformat)
 *
 *
 */
pub fn xcb_render_pictformat_next (i:*mut pictformat_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An pictformat_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_render_pictformat_end (i:pictformat_iterator) -> ffi::base::generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a fixed_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(fixed)
 *
 *
 */
pub fn xcb_render_fixed_next (i:*mut fixed_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An fixed_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_render_fixed_end (i:fixed_iterator) -> ffi::base::generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a directformat_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(directformat)
 *
 *
 */
pub fn xcb_render_directformat_next (i:*mut directformat_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An directformat_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_render_directformat_end (i:directformat_iterator) -> ffi::base::generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a pictforminfo_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(pictforminfo)
 *
 *
 */
pub fn xcb_render_pictforminfo_next (i:*mut pictforminfo_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An pictforminfo_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_render_pictforminfo_end (i:pictforminfo_iterator) -> ffi::base::generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a pictvisual_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(pictvisual)
 *
 *
 */
pub fn xcb_render_pictvisual_next (i:*mut pictvisual_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An pictvisual_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_render_pictvisual_end (i:pictvisual_iterator) -> ffi::base::generic_iterator;

pub fn xcb_render_pictdepth_sizeof (_buffer :  *mut c_void) -> c_int;

pub fn xcb_render_pictdepth_visuals (R : *mut pictdepth) -> *mut pictvisual;


pub fn xcb_render_pictdepth_visuals_length (R : *mut pictdepth) -> c_int;

pub fn xcb_render_pictdepth_visuals_iterator (R : *mut pictdepth) -> pictvisual_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a pictdepth_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(pictdepth)
 *
 *
 */
pub fn xcb_render_pictdepth_next (i:*mut pictdepth_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An pictdepth_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_render_pictdepth_end (i:pictdepth_iterator) -> ffi::base::generic_iterator;

pub fn xcb_render_pictscreen_sizeof (_buffer :  *mut c_void) -> c_int;


pub fn xcb_render_pictscreen_depths_length (R : *mut pictscreen) -> c_int;

pub fn xcb_render_pictscreen_depths_iterator (R : *mut pictscreen) -> pictdepth_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a pictscreen_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(pictscreen)
 *
 *
 */
pub fn xcb_render_pictscreen_next (i:*mut pictscreen_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An pictscreen_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_render_pictscreen_end (i:pictscreen_iterator) -> ffi::base::generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a indexvalue_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(indexvalue)
 *
 *
 */
pub fn xcb_render_indexvalue_next (i:*mut indexvalue_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An indexvalue_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_render_indexvalue_end (i:indexvalue_iterator) -> ffi::base::generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a color_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(color)
 *
 *
 */
pub fn xcb_render_color_next (i:*mut color_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An color_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_render_color_end (i:color_iterator) -> ffi::base::generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a pointfix_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(pointfix)
 *
 *
 */
pub fn xcb_render_pointfix_next (i:*mut pointfix_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An pointfix_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_render_pointfix_end (i:pointfix_iterator) -> ffi::base::generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a linefix_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(linefix)
 *
 *
 */
pub fn xcb_render_linefix_next (i:*mut linefix_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An linefix_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_render_linefix_end (i:linefix_iterator) -> ffi::base::generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a triangle_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(triangle)
 *
 *
 */
pub fn xcb_render_triangle_next (i:*mut triangle_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An triangle_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_render_triangle_end (i:triangle_iterator) -> ffi::base::generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a trapezoid_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(trapezoid)
 *
 *
 */
pub fn xcb_render_trapezoid_next (i:*mut trapezoid_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An trapezoid_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_render_trapezoid_end (i:trapezoid_iterator) -> ffi::base::generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a glyphinfo_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(glyphinfo)
 *
 *
 */
pub fn xcb_render_glyphinfo_next (i:*mut glyphinfo_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An glyphinfo_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_render_glyphinfo_end (i:glyphinfo_iterator) -> ffi::base::generic_iterator;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_render_query_version (c : *mut ffi::base::connection,
                                    client_major_version :  u32,
                                    client_minor_version :  u32) -> query_version_cookie;

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
pub fn xcb_render_query_version_unchecked (c : *mut ffi::base::connection,
                                              client_major_version :  u32,
                                              client_minor_version :  u32) -> query_version_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_render_query_version_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_render_query_version_reply (c : *mut ffi::base::connection,
                                          cookie : query_version_cookie,
                                          e : *mut *mut ffi::base::generic_error) -> *mut query_version_reply;

pub fn xcb_render_query_pict_formats_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_render_query_pict_formats (c : *mut ffi::base::connection) -> query_pict_formats_cookie;

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
pub fn xcb_render_query_pict_formats_unchecked (c : *mut ffi::base::connection) -> query_pict_formats_cookie;

pub fn xcb_render_query_pict_formats_formats (R : *mut query_pict_formats_reply) -> *mut pictforminfo;


pub fn xcb_render_query_pict_formats_formats_length (R : *mut query_pict_formats_reply) -> c_int;

pub fn xcb_render_query_pict_formats_formats_iterator (R : *mut query_pict_formats_reply) -> pictforminfo_iterator;


pub fn xcb_render_query_pict_formats_screens_length (R : *mut query_pict_formats_reply) -> c_int;

pub fn xcb_render_query_pict_formats_screens_iterator (R : *mut query_pict_formats_reply) -> pictscreen_iterator;

pub fn xcb_render_query_pict_formats_subpixels (R : *mut query_pict_formats_reply) -> *mut u32;


pub fn xcb_render_query_pict_formats_subpixels_length (R : *mut query_pict_formats_reply) -> c_int;


pub fn xcb_render_query_pict_formats_subpixels_end (R : *mut query_pict_formats_reply) -> ffi::base::generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_render_query_pict_formats_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_render_query_pict_formats_reply (c : *mut ffi::base::connection,
                                               cookie : query_pict_formats_cookie,
                                               e : *mut *mut ffi::base::generic_error) -> *mut query_pict_formats_reply;

pub fn xcb_render_query_pict_index_values_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_render_query_pict_index_values (c : *mut ffi::base::connection,
                                              format :  pictformat) -> query_pict_index_values_cookie;

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
pub fn xcb_render_query_pict_index_values_unchecked (c : *mut ffi::base::connection,
                                                        format :  pictformat) -> query_pict_index_values_cookie;

pub fn xcb_render_query_pict_index_values_values (R : *mut query_pict_index_values_reply) -> *mut indexvalue;


pub fn xcb_render_query_pict_index_values_values_length (R : *mut query_pict_index_values_reply) -> c_int;

pub fn xcb_render_query_pict_index_values_values_iterator (R : *mut query_pict_index_values_reply) -> indexvalue_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_render_query_pict_index_values_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_render_query_pict_index_values_reply (c : *mut ffi::base::connection,
                                                    cookie : query_pict_index_values_cookie,
                                                    e : *mut *mut ffi::base::generic_error) -> *mut query_pict_index_values_reply;

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
pub fn xcb_render_create_picture_checked (c : *mut ffi::base::connection,
                                             pid :  picture,
                                             drawable :  ffi::xproto::drawable,
                                             format :  pictformat,
                                             value_mask :  u32,
                                             value_list : *mut u32) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_render_create_picture (c : *mut ffi::base::connection,
                                     pid :  picture,
                                     drawable :  ffi::xproto::drawable,
                                     format :  pictformat,
                                     value_mask :  u32,
                                     value_list : *mut u32) -> ffi::base::void_cookie;

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
pub fn xcb_render_change_picture_checked (c : *mut ffi::base::connection,
                                             picture :  picture,
                                             value_mask :  u32,
                                             value_list : *mut u32) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_render_change_picture (c : *mut ffi::base::connection,
                                     picture :  picture,
                                     value_mask :  u32,
                                     value_list : *mut u32) -> ffi::base::void_cookie;

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
pub fn xcb_render_set_picture_clip_rectangles_checked (c : *mut ffi::base::connection,
                                                          picture :  picture,
                                                          clip_x_origin :  i16,
                                                          clip_y_origin :  i16,
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
pub fn xcb_render_set_picture_clip_rectangles (c : *mut ffi::base::connection,
                                                  picture :  picture,
                                                  clip_x_origin :  i16,
                                                  clip_y_origin :  i16,
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
pub fn xcb_render_free_picture_checked (c : *mut ffi::base::connection,
                                           picture :  picture) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_render_free_picture (c : *mut ffi::base::connection,
                                   picture :  picture) -> ffi::base::void_cookie;

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
pub fn xcb_render_composite_checked (c : *mut ffi::base::connection,
                                        op :  u8,
                                        src :  picture,
                                        mask :  picture,
                                        dst :  picture,
                                        src_x :  i16,
                                        src_y :  i16,
                                        mask_x :  i16,
                                        mask_y :  i16,
                                        dst_x :  i16,
                                        dst_y :  i16,
                                        width :  u16,
                                        height :  u16) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_render_composite (c : *mut ffi::base::connection,
                                op :  u8,
                                src :  picture,
                                mask :  picture,
                                dst :  picture,
                                src_x :  i16,
                                src_y :  i16,
                                mask_x :  i16,
                                mask_y :  i16,
                                dst_x :  i16,
                                dst_y :  i16,
                                width :  u16,
                                height :  u16) -> ffi::base::void_cookie;

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
pub fn xcb_render_trapezoids_checked (c : *mut ffi::base::connection,
                                         op :  u8,
                                         src :  picture,
                                         dst :  picture,
                                         mask_format :  pictformat,
                                         src_x :  i16,
                                         src_y :  i16,
                                         traps_len :  u32,
                                         traps : *mut trapezoid) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_render_trapezoids (c : *mut ffi::base::connection,
                                 op :  u8,
                                 src :  picture,
                                 dst :  picture,
                                 mask_format :  pictformat,
                                 src_x :  i16,
                                 src_y :  i16,
                                 traps_len :  u32,
                                 traps : *mut trapezoid) -> ffi::base::void_cookie;

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
pub fn xcb_render_triangles_checked (c : *mut ffi::base::connection,
                                        op :  u8,
                                        src :  picture,
                                        dst :  picture,
                                        mask_format :  pictformat,
                                        src_x :  i16,
                                        src_y :  i16,
                                        triangles_len :  u32,
                                        triangles : *mut triangle) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_render_triangles (c : *mut ffi::base::connection,
                                op :  u8,
                                src :  picture,
                                dst :  picture,
                                mask_format :  pictformat,
                                src_x :  i16,
                                src_y :  i16,
                                triangles_len :  u32,
                                triangles : *mut triangle) -> ffi::base::void_cookie;

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
pub fn xcb_render_tri_strip_checked (c : *mut ffi::base::connection,
                                        op :  u8,
                                        src :  picture,
                                        dst :  picture,
                                        mask_format :  pictformat,
                                        src_x :  i16,
                                        src_y :  i16,
                                        points_len :  u32,
                                        points : *mut pointfix) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_render_tri_strip (c : *mut ffi::base::connection,
                                op :  u8,
                                src :  picture,
                                dst :  picture,
                                mask_format :  pictformat,
                                src_x :  i16,
                                src_y :  i16,
                                points_len :  u32,
                                points : *mut pointfix) -> ffi::base::void_cookie;

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
pub fn xcb_render_tri_fan_checked (c : *mut ffi::base::connection,
                                      op :  u8,
                                      src :  picture,
                                      dst :  picture,
                                      mask_format :  pictformat,
                                      src_x :  i16,
                                      src_y :  i16,
                                      points_len :  u32,
                                      points : *mut pointfix) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_render_tri_fan (c : *mut ffi::base::connection,
                              op :  u8,
                              src :  picture,
                              dst :  picture,
                              mask_format :  pictformat,
                              src_x :  i16,
                              src_y :  i16,
                              points_len :  u32,
                              points : *mut pointfix) -> ffi::base::void_cookie;

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
pub fn xcb_render_create_glyph_set_checked (c : *mut ffi::base::connection,
                                               gsid :  glyphset,
                                               format :  pictformat) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_render_create_glyph_set (c : *mut ffi::base::connection,
                                       gsid :  glyphset,
                                       format :  pictformat) -> ffi::base::void_cookie;

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
pub fn xcb_render_reference_glyph_set_checked (c : *mut ffi::base::connection,
                                                  gsid :  glyphset,
                                                  existing :  glyphset) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_render_reference_glyph_set (c : *mut ffi::base::connection,
                                          gsid :  glyphset,
                                          existing :  glyphset) -> ffi::base::void_cookie;

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
pub fn xcb_render_free_glyph_set_checked (c : *mut ffi::base::connection,
                                             glyphset :  glyphset) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_render_free_glyph_set (c : *mut ffi::base::connection,
                                     glyphset :  glyphset) -> ffi::base::void_cookie;

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
pub fn xcb_render_add_glyphs_checked (c : *mut ffi::base::connection,
                                         glyphset :  glyphset,
                                         glyphs_len :  u32,
                                         glyphids : *mut u32,
                                         glyphs : *mut glyphinfo,
                                         data_len :  u32,
                                         data : *mut u8) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_render_add_glyphs (c : *mut ffi::base::connection,
                                 glyphset :  glyphset,
                                 glyphs_len :  u32,
                                 glyphids : *mut u32,
                                 glyphs : *mut glyphinfo,
                                 data_len :  u32,
                                 data : *mut u8) -> ffi::base::void_cookie;

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
pub fn xcb_render_free_glyphs_checked (c : *mut ffi::base::connection,
                                          glyphset :  glyphset,
                                          glyphs_len :  u32,
                                          glyphs : *mut glyph) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_render_free_glyphs (c : *mut ffi::base::connection,
                                  glyphset :  glyphset,
                                  glyphs_len :  u32,
                                  glyphs : *mut glyph) -> ffi::base::void_cookie;

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
pub fn xcb_render_composite_glyphs_8_checked (c : *mut ffi::base::connection,
                                                 op :  u8,
                                                 src :  picture,
                                                 dst :  picture,
                                                 mask_format :  pictformat,
                                                 glyphset :  glyphset,
                                                 src_x :  i16,
                                                 src_y :  i16,
                                                 glyphcmds_len :  u32,
                                                 glyphcmds : *mut u8) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_render_composite_glyphs_8 (c : *mut ffi::base::connection,
                                         op :  u8,
                                         src :  picture,
                                         dst :  picture,
                                         mask_format :  pictformat,
                                         glyphset :  glyphset,
                                         src_x :  i16,
                                         src_y :  i16,
                                         glyphcmds_len :  u32,
                                         glyphcmds : *mut u8) -> ffi::base::void_cookie;

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
pub fn xcb_render_composite_glyphs_16_checked (c : *mut ffi::base::connection,
                                                  op :  u8,
                                                  src :  picture,
                                                  dst :  picture,
                                                  mask_format :  pictformat,
                                                  glyphset :  glyphset,
                                                  src_x :  i16,
                                                  src_y :  i16,
                                                  glyphcmds_len :  u32,
                                                  glyphcmds : *mut u8) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_render_composite_glyphs_16 (c : *mut ffi::base::connection,
                                          op :  u8,
                                          src :  picture,
                                          dst :  picture,
                                          mask_format :  pictformat,
                                          glyphset :  glyphset,
                                          src_x :  i16,
                                          src_y :  i16,
                                          glyphcmds_len :  u32,
                                          glyphcmds : *mut u8) -> ffi::base::void_cookie;

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
pub fn xcb_render_composite_glyphs_32_checked (c : *mut ffi::base::connection,
                                                  op :  u8,
                                                  src :  picture,
                                                  dst :  picture,
                                                  mask_format :  pictformat,
                                                  glyphset :  glyphset,
                                                  src_x :  i16,
                                                  src_y :  i16,
                                                  glyphcmds_len :  u32,
                                                  glyphcmds : *mut u8) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_render_composite_glyphs_32 (c : *mut ffi::base::connection,
                                          op :  u8,
                                          src :  picture,
                                          dst :  picture,
                                          mask_format :  pictformat,
                                          glyphset :  glyphset,
                                          src_x :  i16,
                                          src_y :  i16,
                                          glyphcmds_len :  u32,
                                          glyphcmds : *mut u8) -> ffi::base::void_cookie;

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
pub fn xcb_render_fill_rectangles_checked (c : *mut ffi::base::connection,
                                              op :  u8,
                                              dst :  picture,
                                              color :  color,
                                              rects_len :  u32,
                                              rects : *mut ffi::xproto::rectangle) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_render_fill_rectangles (c : *mut ffi::base::connection,
                                      op :  u8,
                                      dst :  picture,
                                      color :  color,
                                      rects_len :  u32,
                                      rects : *mut ffi::xproto::rectangle) -> ffi::base::void_cookie;

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
pub fn xcb_render_create_cursor_checked (c : *mut ffi::base::connection,
                                            cid :  ffi::xproto::cursor,
                                            source :  picture,
                                            x :  u16,
                                            y :  u16) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_render_create_cursor (c : *mut ffi::base::connection,
                                    cid :  ffi::xproto::cursor,
                                    source :  picture,
                                    x :  u16,
                                    y :  u16) -> ffi::base::void_cookie;

/**
 * Get the next element of the iterator
 * @param i Pointer to a transform_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(transform)
 *
 *
 */
pub fn xcb_render_transform_next (i:*mut transform_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An transform_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_render_transform_end (i:transform_iterator) -> ffi::base::generic_iterator;

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
pub fn xcb_render_set_picture_transform_checked (c : *mut ffi::base::connection,
                                                    picture :  picture,
                                                    transform :  transform) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_render_set_picture_transform (c : *mut ffi::base::connection,
                                            picture :  picture,
                                            transform :  transform) -> ffi::base::void_cookie;

pub fn xcb_render_query_filters_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_render_query_filters (c : *mut ffi::base::connection,
                                    drawable :  ffi::xproto::drawable) -> query_filters_cookie;

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
pub fn xcb_render_query_filters_unchecked (c : *mut ffi::base::connection,
                                              drawable :  ffi::xproto::drawable) -> query_filters_cookie;

pub fn xcb_render_query_filters_aliases (R : *mut query_filters_reply) -> *mut u16;


pub fn xcb_render_query_filters_aliases_length (R : *mut query_filters_reply) -> c_int;


pub fn xcb_render_query_filters_aliases_end (R : *mut query_filters_reply) -> ffi::base::generic_iterator;


pub fn xcb_render_query_filters_filters_length (R : *mut query_filters_reply) -> c_int;

pub fn xcb_render_query_filters_filters_iterator (R : *mut query_filters_reply) -> ffi::xproto::str_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_render_query_filters_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_render_query_filters_reply (c : *mut ffi::base::connection,
                                          cookie : query_filters_cookie,
                                          e : *mut *mut ffi::base::generic_error) -> *mut query_filters_reply;

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
pub fn xcb_render_set_picture_filter_checked (c : *mut ffi::base::connection,
                                                 picture :  picture,
                                                 filter_len :  u16,
                                                 filter : *mut c_char,
                                                 values_len :  u32,
                                                 values : *mut fixed) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_render_set_picture_filter (c : *mut ffi::base::connection,
                                         picture :  picture,
                                         filter_len :  u16,
                                         filter : *mut c_char,
                                         values_len :  u32,
                                         values : *mut fixed) -> ffi::base::void_cookie;

/**
 * Get the next element of the iterator
 * @param i Pointer to a animcursorelt_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(animcursorelt)
 *
 *
 */
pub fn xcb_render_animcursorelt_next (i:*mut animcursorelt_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An animcursorelt_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_render_animcursorelt_end (i:animcursorelt_iterator) -> ffi::base::generic_iterator;

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
pub fn xcb_render_create_anim_cursor_checked (c : *mut ffi::base::connection,
                                                 cid :  ffi::xproto::cursor,
                                                 cursors_len :  u32,
                                                 cursors : *mut animcursorelt) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_render_create_anim_cursor (c : *mut ffi::base::connection,
                                         cid :  ffi::xproto::cursor,
                                         cursors_len :  u32,
                                         cursors : *mut animcursorelt) -> ffi::base::void_cookie;

/**
 * Get the next element of the iterator
 * @param i Pointer to a spanfix_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(spanfix)
 *
 *
 */
pub fn xcb_render_spanfix_next (i:*mut spanfix_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An spanfix_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_render_spanfix_end (i:spanfix_iterator) -> ffi::base::generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a trap_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(trap)
 *
 *
 */
pub fn xcb_render_trap_next (i:*mut trap_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An trap_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_render_trap_end (i:trap_iterator) -> ffi::base::generic_iterator;

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
pub fn xcb_render_add_traps_checked (c : *mut ffi::base::connection,
                                        picture :  picture,
                                        x_off :  i16,
                                        y_off :  i16,
                                        traps_len :  u32,
                                        traps : *mut trap) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_render_add_traps (c : *mut ffi::base::connection,
                                picture :  picture,
                                x_off :  i16,
                                y_off :  i16,
                                traps_len :  u32,
                                traps : *mut trap) -> ffi::base::void_cookie;

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
pub fn xcb_render_create_solid_fill_checked (c : *mut ffi::base::connection,
                                                picture :  picture,
                                                color :  color) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_render_create_solid_fill (c : *mut ffi::base::connection,
                                        picture :  picture,
                                        color :  color) -> ffi::base::void_cookie;

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
pub fn xcb_render_create_linear_gradient_checked (c : *mut ffi::base::connection,
                                                     picture :  picture,
                                                     p1 :  pointfix,
                                                     p2 :  pointfix,
                                                     num_stops :  u32,
                                                     stops : *mut fixed,
                                                     colors : *mut color) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_render_create_linear_gradient (c : *mut ffi::base::connection,
                                             picture :  picture,
                                             p1 :  pointfix,
                                             p2 :  pointfix,
                                             num_stops :  u32,
                                             stops : *mut fixed,
                                             colors : *mut color) -> ffi::base::void_cookie;

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
pub fn xcb_render_create_radial_gradient_checked (c : *mut ffi::base::connection,
                                                     picture :  picture,
                                                     inner :  pointfix,
                                                     outer :  pointfix,
                                                     inner_radius :  fixed,
                                                     outer_radius :  fixed,
                                                     num_stops :  u32,
                                                     stops : *mut fixed,
                                                     colors : *mut color) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_render_create_radial_gradient (c : *mut ffi::base::connection,
                                             picture :  picture,
                                             inner :  pointfix,
                                             outer :  pointfix,
                                             inner_radius :  fixed,
                                             outer_radius :  fixed,
                                             num_stops :  u32,
                                             stops : *mut fixed,
                                             colors : *mut color) -> ffi::base::void_cookie;

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
pub fn xcb_render_create_conical_gradient_checked (c : *mut ffi::base::connection,
                                                      picture :  picture,
                                                      center :  pointfix,
                                                      angle :  fixed,
                                                      num_stops :  u32,
                                                      stops : *mut fixed,
                                                      colors : *mut color) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_render_create_conical_gradient (c : *mut ffi::base::connection,
                                              picture :  picture,
                                              center :  pointfix,
                                              angle :  fixed,
                                              num_stops :  u32,
                                              stops : *mut fixed,
                                              colors : *mut color) -> ffi::base::void_cookie;
}

