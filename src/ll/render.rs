/*
 * This file generated automatically from render.xml by r_client.py.
 * Edit at your peril.
 */

//Make the compiler quiet
#[allow(unused_imports)];
#[allow(non_camel_case_types)];
use core::libc::*;
use ll::base::*;
use ll::xproto;

pub static RENDER_MAJOR_VERSION : c_uint = 0;
pub static RENDER_MINOR_VERSION : c_uint = 11;

pub type pict_type = c_uint;//{
    pub static XCB_RENDER_PICT_TYPE_INDEXED : pict_type = 1;
    pub static XCB_RENDER_PICT_TYPE_DIRECT : pict_type = 2;
//}

pub type picture_enum = c_uint;//{
    pub static XCB_RENDER_PICTURE_NONE : picture_enum = 1;
//}

pub type pict_op = c_uint;//{
    pub static XCB_RENDER_PICT_OP_CLEAR : pict_op = 1;
    pub static XCB_RENDER_PICT_OP_SRC : pict_op = 2;
    pub static XCB_RENDER_PICT_OP_DST : pict_op = 3;
    pub static XCB_RENDER_PICT_OP_OVER : pict_op = 4;
    pub static XCB_RENDER_PICT_OP_OVER_REVERSE : pict_op = 5;
    pub static XCB_RENDER_PICT_OP_IN : pict_op = 6;
    pub static XCB_RENDER_PICT_OP_IN_REVERSE : pict_op = 7;
    pub static XCB_RENDER_PICT_OP_OUT : pict_op = 8;
    pub static XCB_RENDER_PICT_OP_OUT_REVERSE : pict_op = 9;
    pub static XCB_RENDER_PICT_OP_ATOP : pict_op = 10;
    pub static XCB_RENDER_PICT_OP_ATOP_REVERSE : pict_op = 11;
    pub static XCB_RENDER_PICT_OP_XOR : pict_op = 12;
    pub static XCB_RENDER_PICT_OP_ADD : pict_op = 13;
    pub static XCB_RENDER_PICT_OP_SATURATE : pict_op = 14;
    pub static XCB_RENDER_PICT_OP_DISJOINT_CLEAR : pict_op = 16;
    pub static XCB_RENDER_PICT_OP_DISJOINT_SRC : pict_op = 17;
    pub static XCB_RENDER_PICT_OP_DISJOINT_DST : pict_op = 18;
    pub static XCB_RENDER_PICT_OP_DISJOINT_OVER : pict_op = 19;
    pub static XCB_RENDER_PICT_OP_DISJOINT_OVER_REVERSE : pict_op = 20;
    pub static XCB_RENDER_PICT_OP_DISJOINT_IN : pict_op = 21;
    pub static XCB_RENDER_PICT_OP_DISJOINT_IN_REVERSE : pict_op = 22;
    pub static XCB_RENDER_PICT_OP_DISJOINT_OUT : pict_op = 23;
    pub static XCB_RENDER_PICT_OP_DISJOINT_OUT_REVERSE : pict_op = 24;
    pub static XCB_RENDER_PICT_OP_DISJOINT_ATOP : pict_op = 25;
    pub static XCB_RENDER_PICT_OP_DISJOINT_ATOP_REVERSE : pict_op = 26;
    pub static XCB_RENDER_PICT_OP_DISJOINT_XOR : pict_op = 27;
    pub static XCB_RENDER_PICT_OP_CONJOINT_CLEAR : pict_op = 32;
    pub static XCB_RENDER_PICT_OP_CONJOINT_SRC : pict_op = 33;
    pub static XCB_RENDER_PICT_OP_CONJOINT_DST : pict_op = 34;
    pub static XCB_RENDER_PICT_OP_CONJOINT_OVER : pict_op = 35;
    pub static XCB_RENDER_PICT_OP_CONJOINT_OVER_REVERSE : pict_op = 36;
    pub static XCB_RENDER_PICT_OP_CONJOINT_IN : pict_op = 37;
    pub static XCB_RENDER_PICT_OP_CONJOINT_IN_REVERSE : pict_op = 38;
    pub static XCB_RENDER_PICT_OP_CONJOINT_OUT : pict_op = 39;
    pub static XCB_RENDER_PICT_OP_CONJOINT_OUT_REVERSE : pict_op = 40;
    pub static XCB_RENDER_PICT_OP_CONJOINT_ATOP : pict_op = 41;
    pub static XCB_RENDER_PICT_OP_CONJOINT_ATOP_REVERSE : pict_op = 42;
    pub static XCB_RENDER_PICT_OP_CONJOINT_XOR : pict_op = 43;
    pub static XCB_RENDER_PICT_OP_MULTIPLY : pict_op = 48;
    pub static XCB_RENDER_PICT_OP_SCREEN : pict_op = 49;
    pub static XCB_RENDER_PICT_OP_OVERLAY : pict_op = 50;
    pub static XCB_RENDER_PICT_OP_DARKEN : pict_op = 51;
    pub static XCB_RENDER_PICT_OP_LIGHTEN : pict_op = 52;
    pub static XCB_RENDER_PICT_OP_COLOR_DODGE : pict_op = 53;
    pub static XCB_RENDER_PICT_OP_COLOR_BURN : pict_op = 54;
    pub static XCB_RENDER_PICT_OP_HARD_LIGHT : pict_op = 55;
    pub static XCB_RENDER_PICT_OP_SOFT_LIGHT : pict_op = 56;
    pub static XCB_RENDER_PICT_OP_DIFFERENCE : pict_op = 57;
    pub static XCB_RENDER_PICT_OP_EXCLUSION : pict_op = 58;
    pub static XCB_RENDER_PICT_OP_HSL_HUE : pict_op = 59;
    pub static XCB_RENDER_PICT_OP_HSL_SATURATION : pict_op = 60;
    pub static XCB_RENDER_PICT_OP_HSL_COLOR : pict_op = 61;
    pub static XCB_RENDER_PICT_OP_HSL_LUMINOSITY : pict_op = 62;
//}

pub type poly_edge = c_uint;//{
    pub static XCB_RENDER_POLY_EDGE_SHARP : poly_edge = 1;
    pub static XCB_RENDER_POLY_EDGE_SMOOTH : poly_edge = 2;
//}

pub type poly_mode = c_uint;//{
    pub static XCB_RENDER_POLY_MODE_PRECISE : poly_mode = 1;
    pub static XCB_RENDER_POLY_MODE_IMPRECISE : poly_mode = 2;
//}

pub type cp = c_uint;//{
    pub static XCB_RENDER_CP_REPEAT : cp = 1;
    pub static XCB_RENDER_CP_ALPHA_MAP : cp = 2;
    pub static XCB_RENDER_CP_ALPHA_X_ORIGIN : cp = 4;
    pub static XCB_RENDER_CP_ALPHA_Y_ORIGIN : cp = 8;
    pub static XCB_RENDER_CP_CLIP_X_ORIGIN : cp = 16;
    pub static XCB_RENDER_CP_CLIP_Y_ORIGIN : cp = 32;
    pub static XCB_RENDER_CP_CLIP_MASK : cp = 64;
    pub static XCB_RENDER_CP_GRAPHICS_EXPOSURE : cp = 128;
    pub static XCB_RENDER_CP_SUBWINDOW_MODE : cp = 256;
    pub static XCB_RENDER_CP_POLY_EDGE : cp = 512;
    pub static XCB_RENDER_CP_POLY_MODE : cp = 1024;
    pub static XCB_RENDER_CP_DITHER : cp = 2048;
    pub static XCB_RENDER_CP_COMPONENT_ALPHA : cp = 4096;
//}

pub type sub_pixel = c_uint;//{
    pub static XCB_RENDER_SUB_PIXEL_UNKNOWN : sub_pixel = 1;
    pub static XCB_RENDER_SUB_PIXEL_HORIZONTAL_RGB : sub_pixel = 2;
    pub static XCB_RENDER_SUB_PIXEL_HORIZONTAL_BGR : sub_pixel = 3;
    pub static XCB_RENDER_SUB_PIXEL_VERTICAL_RGB : sub_pixel = 4;
    pub static XCB_RENDER_SUB_PIXEL_VERTICAL_BGR : sub_pixel = 5;
    pub static XCB_RENDER_SUB_PIXEL_NONE : sub_pixel = 6;
//}

pub type repeat = c_uint;//{
    pub static XCB_RENDER_REPEAT_NONE : repeat = 1;
    pub static XCB_RENDER_REPEAT_NORMAL : repeat = 2;
    pub static XCB_RENDER_REPEAT_PAD : repeat = 3;
    pub static XCB_RENDER_REPEAT_REFLECT : repeat = 4;
//}

pub type glyph = u32;

/**
 * @brief glyph_iterator
 **/
pub struct glyph_iterator {
    data : *glyph,
    rem  : c_int,
    index: c_int
}

pub type glyphset = u32;

/**
 * @brief glyphset_iterator
 **/
pub struct glyphset_iterator {
    data : *glyphset,
    rem  : c_int,
    index: c_int
}

pub type picture = u32;

/**
 * @brief picture_iterator
 **/
pub struct picture_iterator {
    data : *picture,
    rem  : c_int,
    index: c_int
}

pub type pictformat = u32;

/**
 * @brief pictformat_iterator
 **/
pub struct pictformat_iterator {
    data : *pictformat,
    rem  : c_int,
    index: c_int
}

pub type fixed = i32;

/**
 * @brief fixed_iterator
 **/
pub struct fixed_iterator {
    data : *fixed,
    rem  : c_int,
    index: c_int
}

/** Opcode for xcb_render_pict_format. */
pub static XCB_RENDER_PICT_FORMAT : c_int = 0;

pub struct pict_format_error {
    response_type :   u8,
    error_code :      u8,
    sequence :        u16
}

/** Opcode for xcb_render_picture. */
pub static XCB_RENDER_PICTURE : c_int = 1;

pub struct picture_error {
    response_type :   u8,
    error_code :      u8,
    sequence :        u16
}

/** Opcode for xcb_render_pict_op. */
pub static XCB_RENDER_PICT_OP : c_int = 2;

pub struct pict_op_error {
    response_type :   u8,
    error_code :      u8,
    sequence :        u16
}

/** Opcode for xcb_render_glyph_set. */
pub static XCB_RENDER_GLYPH_SET : c_int = 3;

pub struct glyph_set_error {
    response_type :   u8,
    error_code :      u8,
    sequence :        u16
}

/** Opcode for xcb_render_glyph. */
pub static XCB_RENDER_GLYPH : c_int = 4;

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
    data : *directformat,
    rem  : c_int,
    index: c_int
}

pub struct pictforminfo {
    id :         pictformat,
    type_ :      u8,
    depth :      u8,
    pad0 :       [u8,..2],
    direct :     directformat,
    colormap :   xproto::colormap
}

/**
 * @brief pictforminfo_iterator
 **/
pub struct pictforminfo_iterator {
    data : *pictforminfo,
    rem  : c_int,
    index: c_int
}

pub struct pictvisual {
    visual :   xproto::visualid,
    format :   pictformat
}

/**
 * @brief pictvisual_iterator
 **/
pub struct pictvisual_iterator {
    data : *pictvisual,
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
    data : *pictdepth,
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
    data : *pictscreen,
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
    data : *indexvalue,
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
    data : *color,
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
    data : *pointfix,
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
    data : *linefix,
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
    data : *triangle,
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
    data : *trapezoid,
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
    data : *glyphinfo,
    rem  : c_int,
    index: c_int
}

pub struct query_version_cookie {
    sequence : c_uint
}

/** Opcode for xcb_render_query_version. */
pub static XCB_RENDER_QUERY_VERSION : c_int = 0;

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

/** Opcode for xcb_render_query_pict_formats. */
pub static XCB_RENDER_QUERY_PICT_FORMATS : c_int = 1;

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

/** Opcode for xcb_render_query_pict_index_values. */
pub static XCB_RENDER_QUERY_PICT_INDEX_VALUES : c_int = 2;

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

/** Opcode for xcb_render_create_picture. */
pub static XCB_RENDER_CREATE_PICTURE : c_int = 4;

pub struct create_picture_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    pid :            picture,
    drawable :       xproto::drawable,
    format :         pictformat,
    value_mask :     u32
}

/** Opcode for xcb_render_change_picture. */
pub static XCB_RENDER_CHANGE_PICTURE : c_int = 5;

pub struct change_picture_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    picture :        picture,
    value_mask :     u32
}

/** Opcode for xcb_render_set_picture_clip_rectangles. */
pub static XCB_RENDER_SET_PICTURE_CLIP_RECTANGLES : c_int = 6;

pub struct set_picture_clip_rectangles_request {
    major_opcode :    u8,
    minor_opcode :    u8,
    length :          u16,
    picture :         picture,
    clip_x_origin :   i16,
    clip_y_origin :   i16
}

/** Opcode for xcb_render_free_picture. */
pub static XCB_RENDER_FREE_PICTURE : c_int = 7;

pub struct free_picture_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    picture :        picture
}

/** Opcode for xcb_render_composite. */
pub static XCB_RENDER_COMPOSITE : c_int = 8;

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

/** Opcode for xcb_render_trapezoids. */
pub static XCB_RENDER_TRAPEZOIDS : c_int = 10;

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

/** Opcode for xcb_render_triangles. */
pub static XCB_RENDER_TRIANGLES : c_int = 11;

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

/** Opcode for xcb_render_tri_strip. */
pub static XCB_RENDER_TRI_STRIP : c_int = 12;

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

/** Opcode for xcb_render_tri_fan. */
pub static XCB_RENDER_TRI_FAN : c_int = 13;

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

/** Opcode for xcb_render_create_glyph_set. */
pub static XCB_RENDER_CREATE_GLYPH_SET : c_int = 17;

pub struct create_glyph_set_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    gsid :           glyphset,
    format :         pictformat
}

/** Opcode for xcb_render_reference_glyph_set. */
pub static XCB_RENDER_REFERENCE_GLYPH_SET : c_int = 18;

pub struct reference_glyph_set_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    gsid :           glyphset,
    existing :       glyphset
}

/** Opcode for xcb_render_free_glyph_set. */
pub static XCB_RENDER_FREE_GLYPH_SET : c_int = 19;

pub struct free_glyph_set_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    glyphset :       glyphset
}

/** Opcode for xcb_render_add_glyphs. */
pub static XCB_RENDER_ADD_GLYPHS : c_int = 20;

pub struct add_glyphs_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    glyphset :       glyphset,
    glyphs_len :     u32
}

/** Opcode for xcb_render_free_glyphs. */
pub static XCB_RENDER_FREE_GLYPHS : c_int = 22;

pub struct free_glyphs_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    glyphset :       glyphset
}

/** Opcode for xcb_render_composite_glyphs_8. */
pub static XCB_RENDER_COMPOSITE_GLYPHS_8 : c_int = 23;

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

/** Opcode for xcb_render_composite_glyphs_16. */
pub static XCB_RENDER_COMPOSITE_GLYPHS_16 : c_int = 24;

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

/** Opcode for xcb_render_composite_glyphs_32. */
pub static XCB_RENDER_COMPOSITE_GLYPHS_32 : c_int = 25;

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

/** Opcode for xcb_render_fill_rectangles. */
pub static XCB_RENDER_FILL_RECTANGLES : c_int = 26;

pub struct fill_rectangles_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    op :             u8,
    pad0 :           [u8,..3],
    dst :            picture,
    color :          color
}

/** Opcode for xcb_render_create_cursor. */
pub static XCB_RENDER_CREATE_CURSOR : c_int = 27;

pub struct create_cursor_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    cid :            xproto::cursor,
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
    data : *transform,
    rem  : c_int,
    index: c_int
}

/** Opcode for xcb_render_set_picture_transform. */
pub static XCB_RENDER_SET_PICTURE_TRANSFORM : c_int = 28;

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

/** Opcode for xcb_render_query_filters. */
pub static XCB_RENDER_QUERY_FILTERS : c_int = 29;

pub struct query_filters_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    drawable :       xproto::drawable
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

/** Opcode for xcb_render_set_picture_filter. */
pub static XCB_RENDER_SET_PICTURE_FILTER : c_int = 30;

pub struct set_picture_filter_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    picture :        picture,
    filter_len :     u16,
    pad0 :           [u8,..2]
}

pub struct animcursorelt {
    cursor :   xproto::cursor,
    delay :    u32
}

/**
 * @brief animcursorelt_iterator
 **/
pub struct animcursorelt_iterator {
    data : *animcursorelt,
    rem  : c_int,
    index: c_int
}

/** Opcode for xcb_render_create_anim_cursor. */
pub static XCB_RENDER_CREATE_ANIM_CURSOR : c_int = 31;

pub struct create_anim_cursor_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    cid :            xproto::cursor
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
    data : *spanfix,
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
    data : *trap,
    rem  : c_int,
    index: c_int
}

/** Opcode for xcb_render_add_traps. */
pub static XCB_RENDER_ADD_TRAPS : c_int = 32;

pub struct add_traps_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    picture :        picture,
    x_off :          i16,
    y_off :          i16
}

/** Opcode for xcb_render_create_solid_fill. */
pub static XCB_RENDER_CREATE_SOLID_FILL : c_int = 33;

pub struct create_solid_fill_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    picture :        picture,
    color :          color
}

/** Opcode for xcb_render_create_linear_gradient. */
pub static XCB_RENDER_CREATE_LINEAR_GRADIENT : c_int = 34;

pub struct create_linear_gradient_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    picture :        picture,
    p1 :             pointfix,
    p2 :             pointfix,
    num_stops :      u32
}

/** Opcode for xcb_render_create_radial_gradient. */
pub static XCB_RENDER_CREATE_RADIAL_GRADIENT : c_int = 35;

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

/** Opcode for xcb_render_create_conical_gradient. */
pub static XCB_RENDER_CREATE_CONICAL_GRADIENT : c_int = 36;

pub struct create_conical_gradient_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    picture :        picture,
    center :         pointfix,
    angle :          fixed,
    num_stops :      u32
}
#[link_args="-lxcb-render"]
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
unsafe fn xcb_render_glyph_next (i:*glyph_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An glyph_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_render_glyph_end (i:glyph_iterator) -> generic_iterator;

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
unsafe fn xcb_render_glyphset_next (i:*glyphset_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An glyphset_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_render_glyphset_end (i:glyphset_iterator) -> generic_iterator;

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
unsafe fn xcb_render_picture_next (i:*picture_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An picture_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_render_picture_end (i:picture_iterator) -> generic_iterator;

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
unsafe fn xcb_render_pictformat_next (i:*pictformat_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An pictformat_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_render_pictformat_end (i:pictformat_iterator) -> generic_iterator;

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
unsafe fn xcb_render_fixed_next (i:*fixed_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An fixed_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_render_fixed_end (i:fixed_iterator) -> generic_iterator;

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
unsafe fn xcb_render_directformat_next (i:*directformat_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An directformat_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_render_directformat_end (i:directformat_iterator) -> generic_iterator;

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
unsafe fn xcb_render_pictforminfo_next (i:*pictforminfo_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An pictforminfo_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_render_pictforminfo_end (i:pictforminfo_iterator) -> generic_iterator;

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
unsafe fn xcb_render_pictvisual_next (i:*pictvisual_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An pictvisual_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_render_pictvisual_end (i:pictvisual_iterator) -> generic_iterator;

unsafe fn xcb_render_pictdepth_sizeof (_buffer :  *c_void) -> c_int;

unsafe fn xcb_render_pictdepth_visuals (R : *pictdepth) -> *pictvisual;


unsafe fn xcb_render_pictdepth_visuals_length (R : *pictdepth) -> c_int;

unsafe fn xcb_render_pictdepth_visuals_iterator (R : *pictdepth) -> pictvisual_iterator;

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
unsafe fn xcb_render_pictdepth_next (i:*pictdepth_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An pictdepth_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_render_pictdepth_end (i:pictdepth_iterator) -> generic_iterator;

unsafe fn xcb_render_pictscreen_sizeof (_buffer :  *c_void) -> c_int;


unsafe fn xcb_render_pictscreen_depths_length (R : *pictscreen) -> c_int;

unsafe fn xcb_render_pictscreen_depths_iterator (R : *pictscreen) -> pictdepth_iterator;

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
unsafe fn xcb_render_pictscreen_next (i:*pictscreen_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An pictscreen_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_render_pictscreen_end (i:pictscreen_iterator) -> generic_iterator;

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
unsafe fn xcb_render_indexvalue_next (i:*indexvalue_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An indexvalue_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_render_indexvalue_end (i:indexvalue_iterator) -> generic_iterator;

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
unsafe fn xcb_render_color_next (i:*color_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An color_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_render_color_end (i:color_iterator) -> generic_iterator;

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
unsafe fn xcb_render_pointfix_next (i:*pointfix_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An pointfix_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_render_pointfix_end (i:pointfix_iterator) -> generic_iterator;

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
unsafe fn xcb_render_linefix_next (i:*linefix_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An linefix_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_render_linefix_end (i:linefix_iterator) -> generic_iterator;

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
unsafe fn xcb_render_triangle_next (i:*triangle_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An triangle_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_render_triangle_end (i:triangle_iterator) -> generic_iterator;

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
unsafe fn xcb_render_trapezoid_next (i:*trapezoid_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An trapezoid_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_render_trapezoid_end (i:trapezoid_iterator) -> generic_iterator;

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
unsafe fn xcb_render_glyphinfo_next (i:*glyphinfo_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An glyphinfo_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_render_glyphinfo_end (i:glyphinfo_iterator) -> generic_iterator;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_render_query_version (c : *connection,
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
unsafe fn xcb_render_query_version_unchecked (c : *connection,
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
unsafe fn xcb_render_query_version_reply (c : *connection,
                                          cookie : query_version_cookie,
                                          e : **generic_error) -> *query_version_reply;

unsafe fn xcb_render_query_pict_formats_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_render_query_pict_formats (c : *connection) -> query_pict_formats_cookie;

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
unsafe fn xcb_render_query_pict_formats_unchecked (c : *connection) -> query_pict_formats_cookie;

unsafe fn xcb_render_query_pict_formats_formats (R : *query_pict_formats_reply) -> *pictforminfo;


unsafe fn xcb_render_query_pict_formats_formats_length (R : *query_pict_formats_reply) -> c_int;

unsafe fn xcb_render_query_pict_formats_formats_iterator (R : *query_pict_formats_reply) -> pictforminfo_iterator;


unsafe fn xcb_render_query_pict_formats_screens_length (R : *query_pict_formats_reply) -> c_int;

unsafe fn xcb_render_query_pict_formats_screens_iterator (R : *query_pict_formats_reply) -> pictscreen_iterator;

unsafe fn xcb_render_query_pict_formats_subpixels (R : *query_pict_formats_reply) -> *u32;


unsafe fn xcb_render_query_pict_formats_subpixels_length (R : *query_pict_formats_reply) -> c_int;


unsafe fn xcb_render_query_pict_formats_subpixels_end (R : *query_pict_formats_reply) -> generic_iterator;

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
unsafe fn xcb_render_query_pict_formats_reply (c : *connection,
                                               cookie : query_pict_formats_cookie,
                                               e : **generic_error) -> *query_pict_formats_reply;

unsafe fn xcb_render_query_pict_index_values_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_render_query_pict_index_values (c : *connection,
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
unsafe fn xcb_render_query_pict_index_values_unchecked (c : *connection,
                                                        format :  pictformat) -> query_pict_index_values_cookie;

unsafe fn xcb_render_query_pict_index_values_values (R : *query_pict_index_values_reply) -> *indexvalue;


unsafe fn xcb_render_query_pict_index_values_values_length (R : *query_pict_index_values_reply) -> c_int;

unsafe fn xcb_render_query_pict_index_values_values_iterator (R : *query_pict_index_values_reply) -> indexvalue_iterator;

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
unsafe fn xcb_render_query_pict_index_values_reply (c : *connection,
                                                    cookie : query_pict_index_values_cookie,
                                                    e : **generic_error) -> *query_pict_index_values_reply;

unsafe fn xcb_render_create_picture_sizeof (_buffer :  *c_void) -> c_int;

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
unsafe fn xcb_render_create_picture_checked (c : *connection,
                                             pid :  picture,
                                             drawable :  xproto::drawable,
                                             format :  pictformat,
                                             value_mask :  u32,
                                             value_list : *u32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_render_create_picture (c : *connection,
                                     pid :  picture,
                                     drawable :  xproto::drawable,
                                     format :  pictformat,
                                     value_mask :  u32,
                                     value_list : *u32) -> void_cookie;

unsafe fn xcb_render_change_picture_sizeof (_buffer :  *c_void) -> c_int;

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
unsafe fn xcb_render_change_picture_checked (c : *connection,
                                             picture :  picture,
                                             value_mask :  u32,
                                             value_list : *u32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_render_change_picture (c : *connection,
                                     picture :  picture,
                                     value_mask :  u32,
                                     value_list : *u32) -> void_cookie;

unsafe fn xcb_render_set_picture_clip_rectangles_sizeof (_buffer :  *c_void,
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
unsafe fn xcb_render_set_picture_clip_rectangles_checked (c : *connection,
                                                          picture :  picture,
                                                          clip_x_origin :  i16,
                                                          clip_y_origin :  i16,
                                                          rectangles_len :  u32,
                                                          rectangles : *xproto::rectangle) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_render_set_picture_clip_rectangles (c : *connection,
                                                  picture :  picture,
                                                  clip_x_origin :  i16,
                                                  clip_y_origin :  i16,
                                                  rectangles_len :  u32,
                                                  rectangles : *xproto::rectangle) -> void_cookie;

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
unsafe fn xcb_render_free_picture_checked (c : *connection,
                                           picture :  picture) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_render_free_picture (c : *connection,
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
unsafe fn xcb_render_composite_checked (c : *connection,
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
unsafe fn xcb_render_composite (c : *connection,
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

unsafe fn xcb_render_trapezoids_sizeof (_buffer :  *c_void,
                              traps_len :  u32) -> c_int;

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
unsafe fn xcb_render_trapezoids_checked (c : *connection,
                                         op :  u8,
                                         src :  picture,
                                         dst :  picture,
                                         mask_format :  pictformat,
                                         src_x :  i16,
                                         src_y :  i16,
                                         traps_len :  u32,
                                         traps : *trapezoid) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_render_trapezoids (c : *connection,
                                 op :  u8,
                                 src :  picture,
                                 dst :  picture,
                                 mask_format :  pictformat,
                                 src_x :  i16,
                                 src_y :  i16,
                                 traps_len :  u32,
                                 traps : *trapezoid) -> void_cookie;

unsafe fn xcb_render_triangles_sizeof (_buffer :  *c_void,
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
unsafe fn xcb_render_triangles_checked (c : *connection,
                                        op :  u8,
                                        src :  picture,
                                        dst :  picture,
                                        mask_format :  pictformat,
                                        src_x :  i16,
                                        src_y :  i16,
                                        triangles_len :  u32,
                                        triangles : *triangle) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_render_triangles (c : *connection,
                                op :  u8,
                                src :  picture,
                                dst :  picture,
                                mask_format :  pictformat,
                                src_x :  i16,
                                src_y :  i16,
                                triangles_len :  u32,
                                triangles : *triangle) -> void_cookie;

unsafe fn xcb_render_tri_strip_sizeof (_buffer :  *c_void,
                             points_len :  u32) -> c_int;

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
unsafe fn xcb_render_tri_strip_checked (c : *connection,
                                        op :  u8,
                                        src :  picture,
                                        dst :  picture,
                                        mask_format :  pictformat,
                                        src_x :  i16,
                                        src_y :  i16,
                                        points_len :  u32,
                                        points : *pointfix) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_render_tri_strip (c : *connection,
                                op :  u8,
                                src :  picture,
                                dst :  picture,
                                mask_format :  pictformat,
                                src_x :  i16,
                                src_y :  i16,
                                points_len :  u32,
                                points : *pointfix) -> void_cookie;

unsafe fn xcb_render_tri_fan_sizeof (_buffer :  *c_void,
                           points_len :  u32) -> c_int;

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
unsafe fn xcb_render_tri_fan_checked (c : *connection,
                                      op :  u8,
                                      src :  picture,
                                      dst :  picture,
                                      mask_format :  pictformat,
                                      src_x :  i16,
                                      src_y :  i16,
                                      points_len :  u32,
                                      points : *pointfix) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_render_tri_fan (c : *connection,
                              op :  u8,
                              src :  picture,
                              dst :  picture,
                              mask_format :  pictformat,
                              src_x :  i16,
                              src_y :  i16,
                              points_len :  u32,
                              points : *pointfix) -> void_cookie;

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
unsafe fn xcb_render_create_glyph_set_checked (c : *connection,
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
unsafe fn xcb_render_create_glyph_set (c : *connection,
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
unsafe fn xcb_render_reference_glyph_set_checked (c : *connection,
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
unsafe fn xcb_render_reference_glyph_set (c : *connection,
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
unsafe fn xcb_render_free_glyph_set_checked (c : *connection,
                                             glyphset :  glyphset) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_render_free_glyph_set (c : *connection,
                                     glyphset :  glyphset) -> void_cookie;

unsafe fn xcb_render_add_glyphs_sizeof (_buffer :  *c_void,
                              data_len :  u32) -> c_int;

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
unsafe fn xcb_render_add_glyphs_checked (c : *connection,
                                         glyphset :  glyphset,
                                         glyphs_len :  u32,
                                         glyphids : *u32,
                                         glyphs : *glyphinfo,
                                         data_len :  u32,
                                         data : *u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_render_add_glyphs (c : *connection,
                                 glyphset :  glyphset,
                                 glyphs_len :  u32,
                                 glyphids : *u32,
                                 glyphs : *glyphinfo,
                                 data_len :  u32,
                                 data : *u8) -> void_cookie;

unsafe fn xcb_render_free_glyphs_sizeof (_buffer :  *c_void,
                               glyphs_len :  u32) -> c_int;

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
unsafe fn xcb_render_free_glyphs_checked (c : *connection,
                                          glyphset :  glyphset,
                                          glyphs_len :  u32,
                                          glyphs : *glyph) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_render_free_glyphs (c : *connection,
                                  glyphset :  glyphset,
                                  glyphs_len :  u32,
                                  glyphs : *glyph) -> void_cookie;

unsafe fn xcb_render_composite_glyphs_8_sizeof (_buffer :  *c_void,
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
unsafe fn xcb_render_composite_glyphs_8_checked (c : *connection,
                                                 op :  u8,
                                                 src :  picture,
                                                 dst :  picture,
                                                 mask_format :  pictformat,
                                                 glyphset :  glyphset,
                                                 src_x :  i16,
                                                 src_y :  i16,
                                                 glyphcmds_len :  u32,
                                                 glyphcmds : *u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_render_composite_glyphs_8 (c : *connection,
                                         op :  u8,
                                         src :  picture,
                                         dst :  picture,
                                         mask_format :  pictformat,
                                         glyphset :  glyphset,
                                         src_x :  i16,
                                         src_y :  i16,
                                         glyphcmds_len :  u32,
                                         glyphcmds : *u8) -> void_cookie;

unsafe fn xcb_render_composite_glyphs_16_sizeof (_buffer :  *c_void,
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
unsafe fn xcb_render_composite_glyphs_16_checked (c : *connection,
                                                  op :  u8,
                                                  src :  picture,
                                                  dst :  picture,
                                                  mask_format :  pictformat,
                                                  glyphset :  glyphset,
                                                  src_x :  i16,
                                                  src_y :  i16,
                                                  glyphcmds_len :  u32,
                                                  glyphcmds : *u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_render_composite_glyphs_16 (c : *connection,
                                          op :  u8,
                                          src :  picture,
                                          dst :  picture,
                                          mask_format :  pictformat,
                                          glyphset :  glyphset,
                                          src_x :  i16,
                                          src_y :  i16,
                                          glyphcmds_len :  u32,
                                          glyphcmds : *u8) -> void_cookie;

unsafe fn xcb_render_composite_glyphs_32_sizeof (_buffer :  *c_void,
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
unsafe fn xcb_render_composite_glyphs_32_checked (c : *connection,
                                                  op :  u8,
                                                  src :  picture,
                                                  dst :  picture,
                                                  mask_format :  pictformat,
                                                  glyphset :  glyphset,
                                                  src_x :  i16,
                                                  src_y :  i16,
                                                  glyphcmds_len :  u32,
                                                  glyphcmds : *u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_render_composite_glyphs_32 (c : *connection,
                                          op :  u8,
                                          src :  picture,
                                          dst :  picture,
                                          mask_format :  pictformat,
                                          glyphset :  glyphset,
                                          src_x :  i16,
                                          src_y :  i16,
                                          glyphcmds_len :  u32,
                                          glyphcmds : *u8) -> void_cookie;

unsafe fn xcb_render_fill_rectangles_sizeof (_buffer :  *c_void,
                                   rects_len :  u32) -> c_int;

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
unsafe fn xcb_render_fill_rectangles_checked (c : *connection,
                                              op :  u8,
                                              dst :  picture,
                                              color :  color,
                                              rects_len :  u32,
                                              rects : *xproto::rectangle) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_render_fill_rectangles (c : *connection,
                                      op :  u8,
                                      dst :  picture,
                                      color :  color,
                                      rects_len :  u32,
                                      rects : *xproto::rectangle) -> void_cookie;

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
unsafe fn xcb_render_create_cursor_checked (c : *connection,
                                            cid :  xproto::cursor,
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
unsafe fn xcb_render_create_cursor (c : *connection,
                                    cid :  xproto::cursor,
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
unsafe fn xcb_render_transform_next (i:*transform_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An transform_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_render_transform_end (i:transform_iterator) -> generic_iterator;

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
unsafe fn xcb_render_set_picture_transform_checked (c : *connection,
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
unsafe fn xcb_render_set_picture_transform (c : *connection,
                                            picture :  picture,
                                            transform :  transform) -> void_cookie;

unsafe fn xcb_render_query_filters_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_render_query_filters (c : *connection,
                                    drawable :  xproto::drawable) -> query_filters_cookie;

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
unsafe fn xcb_render_query_filters_unchecked (c : *connection,
                                              drawable :  xproto::drawable) -> query_filters_cookie;

unsafe fn xcb_render_query_filters_aliases (R : *query_filters_reply) -> *u16;


unsafe fn xcb_render_query_filters_aliases_length (R : *query_filters_reply) -> c_int;


unsafe fn xcb_render_query_filters_aliases_end (R : *query_filters_reply) -> generic_iterator;


unsafe fn xcb_render_query_filters_filters_length (R : *query_filters_reply) -> c_int;

unsafe fn xcb_render_query_filters_filters_iterator (R : *query_filters_reply) -> xproto::str_iterator;

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
unsafe fn xcb_render_query_filters_reply (c : *connection,
                                          cookie : query_filters_cookie,
                                          e : **generic_error) -> *query_filters_reply;

unsafe fn xcb_render_set_picture_filter_sizeof (_buffer :  *c_void,
                                      values_len :  u32) -> c_int;

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
unsafe fn xcb_render_set_picture_filter_checked (c : *connection,
                                                 picture :  picture,
                                                 filter_len :  u16,
                                                 filter : *u8,
                                                 values_len :  u32,
                                                 values : *fixed) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_render_set_picture_filter (c : *connection,
                                         picture :  picture,
                                         filter_len :  u16,
                                         filter : *u8,
                                         values_len :  u32,
                                         values : *fixed) -> void_cookie;

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
unsafe fn xcb_render_animcursorelt_next (i:*animcursorelt_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An animcursorelt_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_render_animcursorelt_end (i:animcursorelt_iterator) -> generic_iterator;

unsafe fn xcb_render_create_anim_cursor_sizeof (_buffer :  *c_void,
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
unsafe fn xcb_render_create_anim_cursor_checked (c : *connection,
                                                 cid :  xproto::cursor,
                                                 cursors_len :  u32,
                                                 cursors : *animcursorelt) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_render_create_anim_cursor (c : *connection,
                                         cid :  xproto::cursor,
                                         cursors_len :  u32,
                                         cursors : *animcursorelt) -> void_cookie;

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
unsafe fn xcb_render_spanfix_next (i:*spanfix_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An spanfix_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_render_spanfix_end (i:spanfix_iterator) -> generic_iterator;

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
unsafe fn xcb_render_trap_next (i:*trap_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An trap_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_render_trap_end (i:trap_iterator) -> generic_iterator;

unsafe fn xcb_render_add_traps_sizeof (_buffer :  *c_void,
                             traps_len :  u32) -> c_int;

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
unsafe fn xcb_render_add_traps_checked (c : *connection,
                                        picture :  picture,
                                        x_off :  i16,
                                        y_off :  i16,
                                        traps_len :  u32,
                                        traps : *trap) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_render_add_traps (c : *connection,
                                picture :  picture,
                                x_off :  i16,
                                y_off :  i16,
                                traps_len :  u32,
                                traps : *trap) -> void_cookie;

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
unsafe fn xcb_render_create_solid_fill_checked (c : *connection,
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
unsafe fn xcb_render_create_solid_fill (c : *connection,
                                        picture :  picture,
                                        color :  color) -> void_cookie;

unsafe fn xcb_render_create_linear_gradient_sizeof (_buffer :  *c_void) -> c_int;

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
unsafe fn xcb_render_create_linear_gradient_checked (c : *connection,
                                                     picture :  picture,
                                                     p1 :  pointfix,
                                                     p2 :  pointfix,
                                                     num_stops :  u32,
                                                     stops : *fixed,
                                                     colors : *color) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_render_create_linear_gradient (c : *connection,
                                             picture :  picture,
                                             p1 :  pointfix,
                                             p2 :  pointfix,
                                             num_stops :  u32,
                                             stops : *fixed,
                                             colors : *color) -> void_cookie;

unsafe fn xcb_render_create_radial_gradient_sizeof (_buffer :  *c_void) -> c_int;

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
unsafe fn xcb_render_create_radial_gradient_checked (c : *connection,
                                                     picture :  picture,
                                                     inner :  pointfix,
                                                     outer :  pointfix,
                                                     inner_radius :  fixed,
                                                     outer_radius :  fixed,
                                                     num_stops :  u32,
                                                     stops : *fixed,
                                                     colors : *color) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_render_create_radial_gradient (c : *connection,
                                             picture :  picture,
                                             inner :  pointfix,
                                             outer :  pointfix,
                                             inner_radius :  fixed,
                                             outer_radius :  fixed,
                                             num_stops :  u32,
                                             stops : *fixed,
                                             colors : *color) -> void_cookie;

unsafe fn xcb_render_create_conical_gradient_sizeof (_buffer :  *c_void) -> c_int;

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
unsafe fn xcb_render_create_conical_gradient_checked (c : *connection,
                                                      picture :  picture,
                                                      center :  pointfix,
                                                      angle :  fixed,
                                                      num_stops :  u32,
                                                      stops : *fixed,
                                                      colors : *color) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_render_create_conical_gradient (c : *connection,
                                              picture :  picture,
                                              center :  pointfix,
                                              angle :  fixed,
                                              num_stops :  u32,
                                              stops : *fixed,
                                              colors : *color) -> void_cookie;
}

