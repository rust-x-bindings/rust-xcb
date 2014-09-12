/*
 * This file generated automatically from render.xml by r_client.py.
 * Edit at your peril.
 */

//Make the compiler quiet
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use std;
use std::libc::*;
use std::{cast,num,ptr,str,libc};
use std::to_bytes::ToBytes;
use ffi::base::*;
use ffi;
use ffi::xproto;

pub static RENDER_MAJOR_VERSION : c_uint = 0;
pub static RENDER_MINOR_VERSION : c_uint = 11;

pub type glyph = u32;
/**
 * @brief glyph_iterator
 **/
pub struct glyph_iterator {
    data : *mut glyph,
    rem  : c_int,
    index: c_int
}


pub type glyphset = u32;
/**
 * @brief glyphset_iterator
 **/
pub struct glyphset_iterator {
    data : *mut glyphset,
    rem  : c_int,
    index: c_int
}


pub type picture = u32;
/**
 * @brief picture_iterator
 **/
pub struct picture_iterator {
    data : *mut picture,
    rem  : c_int,
    index: c_int
}


pub type pictformat = u32;
/**
 * @brief pictformat_iterator
 **/
pub struct pictformat_iterator {
    data : *mut pictformat,
    rem  : c_int,
    index: c_int
}


pub type fixed = i32;
/**
 * @brief fixed_iterator
 **/
pub struct fixed_iterator {
    data : *mut fixed,
    rem  : c_int,
    index: c_int
}



pub struct pict_format_error {
    response_type :   u8,
    error_code :      u8,
    sequence :        u16
}



pub struct picture_error {
    response_type :   u8,
    error_code :      u8,
    sequence :        u16
}



pub struct pict_op_error {
    response_type :   u8,
    error_code :      u8,
    sequence :        u16
}



pub struct glyph_set_error {
    response_type :   u8,
    error_code :      u8,
    sequence :        u16
}



pub struct glyph_error {
    response_type :   u8,
    error_code :      u8,
    sequence :        u16
}


pub struct directformat {
    red_shift :     u16,
    red_mask :      u16,
    green_shift :   u16,
    green_mask :    u16,
    blue_shift :    u16,
    blue_mask :     u16,
    alpha_shift :   u16,
    alpha_mask :    u16
}

/**
 * @brief directformat_iterator
 **/
pub struct directformat_iterator {
    data : *mut directformat,
    rem  : c_int,
    index: c_int
}


pub struct pictforminfo {
    id :         pictformat,
    type_ :      u8,
    depth :      u8,
    pad0 :       [u8,..2],
    direct :     directformat,
    colormap :   ffi::xproto::colormap
}

/**
 * @brief pictforminfo_iterator
 **/
pub struct pictforminfo_iterator {
    data : *mut pictforminfo,
    rem  : c_int,
    index: c_int
}


pub struct pictvisual {
    visual :   ffi::xproto::visualid,
    format :   pictformat
}

/**
 * @brief pictvisual_iterator
 **/
pub struct pictvisual_iterator {
    data : *mut pictvisual,
    rem  : c_int,
    index: c_int
}


pub struct pictdepth {
    depth :         u8,
    pad0 :          u8,
    num_visuals :   u16,
    pad1 :          [u8,..4]
}

/**
 * @brief pictdepth_iterator
 **/
pub struct pictdepth_iterator {
    data : *mut pictdepth,
    rem  : c_int,
    index: c_int
}


pub struct pictscreen {
    num_depths :   u32,
    fallback :     pictformat
}

/**
 * @brief pictscreen_iterator
 **/
pub struct pictscreen_iterator {
    data : *mut pictscreen,
    rem  : c_int,
    index: c_int
}


pub struct indexvalue {
    pixel :   u32,
    red :     u16,
    green :   u16,
    blue :    u16,
    alpha :   u16
}

/**
 * @brief indexvalue_iterator
 **/
pub struct indexvalue_iterator {
    data : *mut indexvalue,
    rem  : c_int,
    index: c_int
}


pub struct color {
    red :     u16,
    green :   u16,
    blue :    u16,
    alpha :   u16
}

/**
 * @brief color_iterator
 **/
pub struct color_iterator {
    data : *mut color,
    rem  : c_int,
    index: c_int
}


pub struct pointfix {
    x :   fixed,
    y :   fixed
}

/**
 * @brief pointfix_iterator
 **/
pub struct pointfix_iterator {
    data : *mut pointfix,
    rem  : c_int,
    index: c_int
}


pub struct linefix {
    p1 :   pointfix,
    p2 :   pointfix
}

/**
 * @brief linefix_iterator
 **/
pub struct linefix_iterator {
    data : *mut linefix,
    rem  : c_int,
    index: c_int
}


pub struct triangle {
    p1 :   pointfix,
    p2 :   pointfix,
    p3 :   pointfix
}

/**
 * @brief triangle_iterator
 **/
pub struct triangle_iterator {
    data : *mut triangle,
    rem  : c_int,
    index: c_int
}


pub struct trapezoid {
    top :      fixed,
    bottom :   fixed,
    left :     linefix,
    right :    linefix
}

/**
 * @brief trapezoid_iterator
 **/
pub struct trapezoid_iterator {
    data : *mut trapezoid,
    rem  : c_int,
    index: c_int
}


pub struct glyphinfo {
    width :    u16,
    height :   u16,
    x :        i16,
    y :        i16,
    x_off :    i16,
    y_off :    i16
}

/**
 * @brief glyphinfo_iterator
 **/
pub struct glyphinfo_iterator {
    data : *mut glyphinfo,
    rem  : c_int,
    index: c_int
}


pub struct query_version_cookie {
    sequence : c_uint
}


pub struct query_version_request {
    major_opcode :           u8,
    minor_opcode :           u8,
    length :                 u16,
    client_major_version :   u32,
    client_minor_version :   u32
}


pub struct query_version_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    major_version :   u32,
    minor_version :   u32,
    pad1 :            [u8,..16]
}


pub struct query_pict_formats_cookie {
    sequence : c_uint
}


pub struct query_pict_formats_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16
}


pub struct query_pict_formats_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    num_formats :     u32,
    num_screens :     u32,
    num_depths :      u32,
    num_visuals :     u32,
    num_subpixel :    u32,
    pad1 :            [u8,..4]
}


pub struct query_pict_index_values_cookie {
    sequence : c_uint
}


pub struct query_pict_index_values_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    format :         pictformat
}


pub struct query_pict_index_values_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    num_values :      u32,
    pad1 :            [u8,..20]
}



pub struct create_picture_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    pid :            picture,
    drawable :       ffi::xproto::drawable,
    format :         pictformat,
    value_mask :     u32
}



pub struct change_picture_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    picture :        picture,
    value_mask :     u32
}



pub struct set_picture_clip_rectangles_request {
    major_opcode :    u8,
    minor_opcode :    u8,
    length :          u16,
    picture :         picture,
    clip_x_origin :   i16,
    clip_y_origin :   i16
}



pub struct free_picture_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    picture :        picture
}



pub struct composite_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    op :             u8,
    pad0 :           [u8,..3],
    src :            picture,
    mask :           picture,
    dst :            picture,
    src_x :          i16,
    src_y :          i16,
    mask_x :         i16,
    mask_y :         i16,
    dst_x :          i16,
    dst_y :          i16,
    width :          u16,
    height :         u16
}



pub struct trapezoids_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    op :             u8,
    pad0 :           [u8,..3],
    src :            picture,
    dst :            picture,
    mask_format :    pictformat,
    src_x :          i16,
    src_y :          i16
}



pub struct triangles_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    op :             u8,
    pad0 :           [u8,..3],
    src :            picture,
    dst :            picture,
    mask_format :    pictformat,
    src_x :          i16,
    src_y :          i16
}



pub struct tri_strip_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    op :             u8,
    pad0 :           [u8,..3],
    src :            picture,
    dst :            picture,
    mask_format :    pictformat,
    src_x :          i16,
    src_y :          i16
}



pub struct tri_fan_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    op :             u8,
    pad0 :           [u8,..3],
    src :            picture,
    dst :            picture,
    mask_format :    pictformat,
    src_x :          i16,
    src_y :          i16
}



pub struct create_glyph_set_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    gsid :           glyphset,
    format :         pictformat
}



pub struct reference_glyph_set_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    gsid :           glyphset,
    existing :       glyphset
}



pub struct free_glyph_set_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    glyphset :       glyphset
}



pub struct add_glyphs_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    glyphset :       glyphset,
    glyphs_len :     u32
}



pub struct free_glyphs_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    glyphset :       glyphset
}



pub struct composite_glyphs_8_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    op :             u8,
    pad0 :           [u8,..3],
    src :            picture,
    dst :            picture,
    mask_format :    pictformat,
    glyphset :       glyphset,
    src_x :          i16,
    src_y :          i16
}



pub struct composite_glyphs_16_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    op :             u8,
    pad0 :           [u8,..3],
    src :            picture,
    dst :            picture,
    mask_format :    pictformat,
    glyphset :       glyphset,
    src_x :          i16,
    src_y :          i16
}



pub struct composite_glyphs_32_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    op :             u8,
    pad0 :           [u8,..3],
    src :            picture,
    dst :            picture,
    mask_format :    pictformat,
    glyphset :       glyphset,
    src_x :          i16,
    src_y :          i16
}



pub struct fill_rectangles_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    op :             u8,
    pad0 :           [u8,..3],
    dst :            picture,
    color :          color
}



pub struct create_cursor_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    cid :            ffi::xproto::cursor,
    source :         picture,
    x :              u16,
    y :              u16
}


pub struct transform {
    matrix11 :   fixed,
    matrix12 :   fixed,
    matrix13 :   fixed,
    matrix21 :   fixed,
    matrix22 :   fixed,
    matrix23 :   fixed,
    matrix31 :   fixed,
    matrix32 :   fixed,
    matrix33 :   fixed
}

/**
 * @brief transform_iterator
 **/
pub struct transform_iterator {
    data : *mut transform,
    rem  : c_int,
    index: c_int
}



pub struct set_picture_transform_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    picture :        picture,
    transform :      transform
}


pub struct query_filters_cookie {
    sequence : c_uint
}


pub struct query_filters_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    drawable :       ffi::xproto::drawable
}


pub struct query_filters_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    num_aliases :     u32,
    num_filters :     u32,
    pad1 :            [u8,..16]
}



pub struct set_picture_filter_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    picture :        picture,
    filter_len :     u16,
    pad0 :           [u8,..2]
}


pub struct animcursorelt {
    cursor :   ffi::xproto::cursor,
    delay :    u32
}

/**
 * @brief animcursorelt_iterator
 **/
pub struct animcursorelt_iterator {
    data : *mut animcursorelt,
    rem  : c_int,
    index: c_int
}



pub struct create_anim_cursor_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    cid :            ffi::xproto::cursor
}


pub struct spanfix {
    l :   fixed,
    r :   fixed,
    y :   fixed
}

/**
 * @brief spanfix_iterator
 **/
pub struct spanfix_iterator {
    data : *mut spanfix,
    rem  : c_int,
    index: c_int
}


pub struct trap {
    top :   spanfix,
    bot :   spanfix
}

/**
 * @brief trap_iterator
 **/
pub struct trap_iterator {
    data : *mut trap,
    rem  : c_int,
    index: c_int
}



pub struct add_traps_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    picture :        picture,
    x_off :          i16,
    y_off :          i16
}



pub struct create_solid_fill_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    picture :        picture,
    color :          color
}



pub struct create_linear_gradient_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    picture :        picture,
    p1 :             pointfix,
    p2 :             pointfix,
    num_stops :      u32
}



pub struct create_radial_gradient_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    picture :        picture,
    inner :          pointfix,
    outer :          pointfix,
    inner_radius :   fixed,
    outer_radius :   fixed,
    num_stops :      u32
}



pub struct create_conical_gradient_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    picture :        picture,
    center :         pointfix,
    angle :          fixed,
    num_stops :      u32
}

#[link(name="lxcb-render")]
pub extern "C" {

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
pub fn xcb_render_glyph_end (i:glyph_iterator) -> generic_iterator;

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
pub fn xcb_render_glyphset_end (i:glyphset_iterator) -> generic_iterator;

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
pub fn xcb_render_picture_end (i:picture_iterator) -> generic_iterator;

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
pub fn xcb_render_pictformat_end (i:pictformat_iterator) -> generic_iterator;

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
pub fn xcb_render_fixed_end (i:fixed_iterator) -> generic_iterator;

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
pub fn xcb_render_directformat_end (i:directformat_iterator) -> generic_iterator;

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
pub fn xcb_render_pictforminfo_end (i:pictforminfo_iterator) -> generic_iterator;

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
pub fn xcb_render_pictvisual_end (i:pictvisual_iterator) -> generic_iterator;

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
pub fn xcb_render_pictdepth_end (i:pictdepth_iterator) -> generic_iterator;

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
pub fn xcb_render_pictscreen_end (i:pictscreen_iterator) -> generic_iterator;

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
pub fn xcb_render_indexvalue_end (i:indexvalue_iterator) -> generic_iterator;

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
pub fn xcb_render_color_end (i:color_iterator) -> generic_iterator;

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
pub fn xcb_render_pointfix_end (i:pointfix_iterator) -> generic_iterator;

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
pub fn xcb_render_linefix_end (i:linefix_iterator) -> generic_iterator;

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
pub fn xcb_render_triangle_end (i:triangle_iterator) -> generic_iterator;

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
pub fn xcb_render_trapezoid_end (i:trapezoid_iterator) -> generic_iterator;

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
pub fn xcb_render_glyphinfo_end (i:glyphinfo_iterator) -> generic_iterator;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_render_query_version (c : *mut connection,
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
pub fn xcb_render_query_version_unchecked (c : *mut connection,
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
pub fn xcb_render_query_version_reply (c : *mut connection,
                                          cookie : query_version_cookie,
                                          e : *mut *mut generic_error) -> *mut query_version_reply;

pub fn xcb_render_query_pict_formats_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_render_query_pict_formats (c : *mut connection) -> query_pict_formats_cookie;

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
pub fn xcb_render_query_pict_formats_unchecked (c : *mut connection) -> query_pict_formats_cookie;

pub fn xcb_render_query_pict_formats_formats (R : *mut query_pict_formats_reply) -> *mut pictforminfo;


pub fn xcb_render_query_pict_formats_formats_length (R : *mut query_pict_formats_reply) -> c_int;

pub fn xcb_render_query_pict_formats_formats_iterator (R : *mut query_pict_formats_reply) -> pictforminfo_iterator;


pub fn xcb_render_query_pict_formats_screens_length (R : *mut query_pict_formats_reply) -> c_int;

pub fn xcb_render_query_pict_formats_screens_iterator (R : *mut query_pict_formats_reply) -> pictscreen_iterator;

pub fn xcb_render_query_pict_formats_subpixels (R : *mut query_pict_formats_reply) -> *mut u32;


pub fn xcb_render_query_pict_formats_subpixels_length (R : *mut query_pict_formats_reply) -> c_int;


pub fn xcb_render_query_pict_formats_subpixels_end (R : *mut query_pict_formats_reply) -> generic_iterator;

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
pub fn xcb_render_query_pict_formats_reply (c : *mut connection,
                                               cookie : query_pict_formats_cookie,
                                               e : *mut *mut generic_error) -> *mut query_pict_formats_reply;

pub fn xcb_render_query_pict_index_values_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_render_query_pict_index_values (c : *mut connection,
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
pub fn xcb_render_query_pict_index_values_unchecked (c : *mut connection,
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
pub fn xcb_render_query_pict_index_values_reply (c : *mut connection,
                                                    cookie : query_pict_index_values_cookie,
                                                    e : *mut *mut generic_error) -> *mut query_pict_index_values_reply;

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
pub fn xcb_render_create_picture_checked (c : *mut connection,
                                             pid :  picture,
                                             drawable :  ffi::xproto::drawable,
                                             format :  pictformat,
                                             value_mask :  u32,
                                             value_list : *mut u32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_render_create_picture (c : *mut connection,
                                     pid :  picture,
                                     drawable :  ffi::xproto::drawable,
                                     format :  pictformat,
                                     value_mask :  u32,
                                     value_list : *mut u32) -> void_cookie;

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
pub fn xcb_render_change_picture_checked (c : *mut connection,
                                             picture :  picture,
                                             value_mask :  u32,
                                             value_list : *mut u32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_render_change_picture (c : *mut connection,
                                     picture :  picture,
                                     value_mask :  u32,
                                     value_list : *mut u32) -> void_cookie;

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
pub fn xcb_render_set_picture_clip_rectangles_checked (c : *mut connection,
                                                          picture :  picture,
                                                          clip_x_origin :  i16,
                                                          clip_y_origin :  i16,
                                                          rectangles_len :  u32,
                                                          rectangles : *mut ffi::xproto::rectangle) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_render_set_picture_clip_rectangles (c : *mut connection,
                                                  picture :  picture,
                                                  clip_x_origin :  i16,
                                                  clip_y_origin :  i16,
                                                  rectangles_len :  u32,
                                                  rectangles : *mut ffi::xproto::rectangle) -> void_cookie;

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
pub fn xcb_render_free_picture_checked (c : *mut connection,
                                           picture :  picture) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_render_free_picture (c : *mut connection,
                                   picture :  picture) -> void_cookie;

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
pub fn xcb_render_composite_checked (c : *mut connection,
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
                                        height :  u16) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_render_composite (c : *mut connection,
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
                                height :  u16) -> void_cookie;

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
pub fn xcb_render_trapezoids_checked (c : *mut connection,
                                         op :  u8,
                                         src :  picture,
                                         dst :  picture,
                                         mask_format :  pictformat,
                                         src_x :  i16,
                                         src_y :  i16,
                                         traps_len :  u32,
                                         traps : *mut trapezoid) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_render_trapezoids (c : *mut connection,
                                 op :  u8,
                                 src :  picture,
                                 dst :  picture,
                                 mask_format :  pictformat,
                                 src_x :  i16,
                                 src_y :  i16,
                                 traps_len :  u32,
                                 traps : *mut trapezoid) -> void_cookie;

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
pub fn xcb_render_triangles_checked (c : *mut connection,
                                        op :  u8,
                                        src :  picture,
                                        dst :  picture,
                                        mask_format :  pictformat,
                                        src_x :  i16,
                                        src_y :  i16,
                                        triangles_len :  u32,
                                        triangles : *mut triangle) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_render_triangles (c : *mut connection,
                                op :  u8,
                                src :  picture,
                                dst :  picture,
                                mask_format :  pictformat,
                                src_x :  i16,
                                src_y :  i16,
                                triangles_len :  u32,
                                triangles : *mut triangle) -> void_cookie;

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
pub fn xcb_render_tri_strip_checked (c : *mut connection,
                                        op :  u8,
                                        src :  picture,
                                        dst :  picture,
                                        mask_format :  pictformat,
                                        src_x :  i16,
                                        src_y :  i16,
                                        points_len :  u32,
                                        points : *mut pointfix) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_render_tri_strip (c : *mut connection,
                                op :  u8,
                                src :  picture,
                                dst :  picture,
                                mask_format :  pictformat,
                                src_x :  i16,
                                src_y :  i16,
                                points_len :  u32,
                                points : *mut pointfix) -> void_cookie;

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
pub fn xcb_render_tri_fan_checked (c : *mut connection,
                                      op :  u8,
                                      src :  picture,
                                      dst :  picture,
                                      mask_format :  pictformat,
                                      src_x :  i16,
                                      src_y :  i16,
                                      points_len :  u32,
                                      points : *mut pointfix) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_render_tri_fan (c : *mut connection,
                              op :  u8,
                              src :  picture,
                              dst :  picture,
                              mask_format :  pictformat,
                              src_x :  i16,
                              src_y :  i16,
                              points_len :  u32,
                              points : *mut pointfix) -> void_cookie;

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
pub fn xcb_render_create_glyph_set_checked (c : *mut connection,
                                               gsid :  glyphset,
                                               format :  pictformat) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_render_create_glyph_set (c : *mut connection,
                                       gsid :  glyphset,
                                       format :  pictformat) -> void_cookie;

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
pub fn xcb_render_reference_glyph_set_checked (c : *mut connection,
                                                  gsid :  glyphset,
                                                  existing :  glyphset) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_render_reference_glyph_set (c : *mut connection,
                                          gsid :  glyphset,
                                          existing :  glyphset) -> void_cookie;

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
pub fn xcb_render_free_glyph_set_checked (c : *mut connection,
                                             glyphset :  glyphset) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_render_free_glyph_set (c : *mut connection,
                                     glyphset :  glyphset) -> void_cookie;

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
pub fn xcb_render_add_glyphs_checked (c : *mut connection,
                                         glyphset :  glyphset,
                                         glyphs_len :  u32,
                                         glyphids : *mut u32,
                                         glyphs : *mut glyphinfo,
                                         data_len :  u32,
                                         data : *mut u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_render_add_glyphs (c : *mut connection,
                                 glyphset :  glyphset,
                                 glyphs_len :  u32,
                                 glyphids : *mut u32,
                                 glyphs : *mut glyphinfo,
                                 data_len :  u32,
                                 data : *mut u8) -> void_cookie;

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
pub fn xcb_render_free_glyphs_checked (c : *mut connection,
                                          glyphset :  glyphset,
                                          glyphs_len :  u32,
                                          glyphs : *mut glyph) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_render_free_glyphs (c : *mut connection,
                                  glyphset :  glyphset,
                                  glyphs_len :  u32,
                                  glyphs : *mut glyph) -> void_cookie;

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
pub fn xcb_render_composite_glyphs_8_checked (c : *mut connection,
                                                 op :  u8,
                                                 src :  picture,
                                                 dst :  picture,
                                                 mask_format :  pictformat,
                                                 glyphset :  glyphset,
                                                 src_x :  i16,
                                                 src_y :  i16,
                                                 glyphcmds_len :  u32,
                                                 glyphcmds : *mut u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_render_composite_glyphs_8 (c : *mut connection,
                                         op :  u8,
                                         src :  picture,
                                         dst :  picture,
                                         mask_format :  pictformat,
                                         glyphset :  glyphset,
                                         src_x :  i16,
                                         src_y :  i16,
                                         glyphcmds_len :  u32,
                                         glyphcmds : *mut u8) -> void_cookie;

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
pub fn xcb_render_composite_glyphs_16_checked (c : *mut connection,
                                                  op :  u8,
                                                  src :  picture,
                                                  dst :  picture,
                                                  mask_format :  pictformat,
                                                  glyphset :  glyphset,
                                                  src_x :  i16,
                                                  src_y :  i16,
                                                  glyphcmds_len :  u32,
                                                  glyphcmds : *mut u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_render_composite_glyphs_16 (c : *mut connection,
                                          op :  u8,
                                          src :  picture,
                                          dst :  picture,
                                          mask_format :  pictformat,
                                          glyphset :  glyphset,
                                          src_x :  i16,
                                          src_y :  i16,
                                          glyphcmds_len :  u32,
                                          glyphcmds : *mut u8) -> void_cookie;

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
pub fn xcb_render_composite_glyphs_32_checked (c : *mut connection,
                                                  op :  u8,
                                                  src :  picture,
                                                  dst :  picture,
                                                  mask_format :  pictformat,
                                                  glyphset :  glyphset,
                                                  src_x :  i16,
                                                  src_y :  i16,
                                                  glyphcmds_len :  u32,
                                                  glyphcmds : *mut u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_render_composite_glyphs_32 (c : *mut connection,
                                          op :  u8,
                                          src :  picture,
                                          dst :  picture,
                                          mask_format :  pictformat,
                                          glyphset :  glyphset,
                                          src_x :  i16,
                                          src_y :  i16,
                                          glyphcmds_len :  u32,
                                          glyphcmds : *mut u8) -> void_cookie;

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
pub fn xcb_render_fill_rectangles_checked (c : *mut connection,
                                              op :  u8,
                                              dst :  picture,
                                              color :  color,
                                              rects_len :  u32,
                                              rects : *mut ffi::xproto::rectangle) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_render_fill_rectangles (c : *mut connection,
                                      op :  u8,
                                      dst :  picture,
                                      color :  color,
                                      rects_len :  u32,
                                      rects : *mut ffi::xproto::rectangle) -> void_cookie;

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
pub fn xcb_render_create_cursor_checked (c : *mut connection,
                                            cid :  ffi::xproto::cursor,
                                            source :  picture,
                                            x :  u16,
                                            y :  u16) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_render_create_cursor (c : *mut connection,
                                    cid :  ffi::xproto::cursor,
                                    source :  picture,
                                    x :  u16,
                                    y :  u16) -> void_cookie;

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
pub fn xcb_render_transform_end (i:transform_iterator) -> generic_iterator;

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
pub fn xcb_render_set_picture_transform_checked (c : *mut connection,
                                                    picture :  picture,
                                                    transform :  transform) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_render_set_picture_transform (c : *mut connection,
                                            picture :  picture,
                                            transform :  transform) -> void_cookie;

pub fn xcb_render_query_filters_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_render_query_filters (c : *mut connection,
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
pub fn xcb_render_query_filters_unchecked (c : *mut connection,
                                              drawable :  ffi::xproto::drawable) -> query_filters_cookie;

pub fn xcb_render_query_filters_aliases (R : *mut query_filters_reply) -> *mut u16;


pub fn xcb_render_query_filters_aliases_length (R : *mut query_filters_reply) -> c_int;


pub fn xcb_render_query_filters_aliases_end (R : *mut query_filters_reply) -> generic_iterator;


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
pub fn xcb_render_query_filters_reply (c : *mut connection,
                                          cookie : query_filters_cookie,
                                          e : *mut *mut generic_error) -> *mut query_filters_reply;

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
pub fn xcb_render_set_picture_filter_checked (c : *mut connection,
                                                 picture :  picture,
                                                 filter_len :  u16,
                                                 filter : *mut c_char,
                                                 values_len :  u32,
                                                 values : *mut fixed) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_render_set_picture_filter (c : *mut connection,
                                         picture :  picture,
                                         filter_len :  u16,
                                         filter : *mut c_char,
                                         values_len :  u32,
                                         values : *mut fixed) -> void_cookie;

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
pub fn xcb_render_animcursorelt_end (i:animcursorelt_iterator) -> generic_iterator;

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
pub fn xcb_render_create_anim_cursor_checked (c : *mut connection,
                                                 cid :  ffi::xproto::cursor,
                                                 cursors_len :  u32,
                                                 cursors : *mut animcursorelt) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_render_create_anim_cursor (c : *mut connection,
                                         cid :  ffi::xproto::cursor,
                                         cursors_len :  u32,
                                         cursors : *mut animcursorelt) -> void_cookie;

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
pub fn xcb_render_spanfix_end (i:spanfix_iterator) -> generic_iterator;

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
pub fn xcb_render_trap_end (i:trap_iterator) -> generic_iterator;

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
pub fn xcb_render_add_traps_checked (c : *mut connection,
                                        picture :  picture,
                                        x_off :  i16,
                                        y_off :  i16,
                                        traps_len :  u32,
                                        traps : *mut trap) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_render_add_traps (c : *mut connection,
                                picture :  picture,
                                x_off :  i16,
                                y_off :  i16,
                                traps_len :  u32,
                                traps : *mut trap) -> void_cookie;

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
pub fn xcb_render_create_solid_fill_checked (c : *mut connection,
                                                picture :  picture,
                                                color :  color) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_render_create_solid_fill (c : *mut connection,
                                        picture :  picture,
                                        color :  color) -> void_cookie;

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
pub fn xcb_render_create_linear_gradient_checked (c : *mut connection,
                                                     picture :  picture,
                                                     p1 :  pointfix,
                                                     p2 :  pointfix,
                                                     num_stops :  u32,
                                                     stops : *mut fixed,
                                                     colors : *mut color) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_render_create_linear_gradient (c : *mut connection,
                                             picture :  picture,
                                             p1 :  pointfix,
                                             p2 :  pointfix,
                                             num_stops :  u32,
                                             stops : *mut fixed,
                                             colors : *mut color) -> void_cookie;

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
pub fn xcb_render_create_radial_gradient_checked (c : *mut connection,
                                                     picture :  picture,
                                                     inner :  pointfix,
                                                     outer :  pointfix,
                                                     inner_radius :  fixed,
                                                     outer_radius :  fixed,
                                                     num_stops :  u32,
                                                     stops : *mut fixed,
                                                     colors : *mut color) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_render_create_radial_gradient (c : *mut connection,
                                             picture :  picture,
                                             inner :  pointfix,
                                             outer :  pointfix,
                                             inner_radius :  fixed,
                                             outer_radius :  fixed,
                                             num_stops :  u32,
                                             stops : *mut fixed,
                                             colors : *mut color) -> void_cookie;

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
pub fn xcb_render_create_conical_gradient_checked (c : *mut connection,
                                                      picture :  picture,
                                                      center :  pointfix,
                                                      angle :  fixed,
                                                      num_stops :  u32,
                                                      stops : *mut fixed,
                                                      colors : *mut color) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_render_create_conical_gradient (c : *mut connection,
                                              picture :  picture,
                                              center :  pointfix,
                                              angle :  fixed,
                                              num_stops :  u32,
                                              stops : *mut fixed,
                                              colors : *mut color) -> void_cookie;
}

