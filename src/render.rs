//
// This file generated automatically from render.xml by r_client.py.
// Edit at your peril.
//

//Make the compiler quiet
#![allow(unused_imports)]
#![allow(unused_unsafe)]
use std;
use libc::*;
use std::{mem,num,ptr,str};
use ffi::base::*;
use base;
use base::*;
use ffi;
use ffi::render::*;
use std::option::Option;
use std::iter::Iterator;

use xproto;

pub type xcb_render_pict_type_t = c_uint;//{
    pub const XCB_RENDER_PICT_TYPE_INDEXED : xcb_render_pict_type_t = 1;
    pub const XCB_RENDER_PICT_TYPE_DIRECT : xcb_render_pict_type_t = 2;
//}

pub type xcb_render_picture_enum_t = c_uint;//{
    pub const XCB_RENDER_PICTURE_NONE : xcb_render_picture_enum_t = 1;
//}

pub type xcb_render_pict_op_t = c_uint;//{
    pub const XCB_RENDER_PICT_OP_CLEAR : xcb_render_pict_op_t = 1;
    pub const XCB_RENDER_PICT_OP_SRC : xcb_render_pict_op_t = 2;
    pub const XCB_RENDER_PICT_OP_DST : xcb_render_pict_op_t = 3;
    pub const XCB_RENDER_PICT_OP_OVER : xcb_render_pict_op_t = 4;
    pub const XCB_RENDER_PICT_OP_OVER_REVERSE : xcb_render_pict_op_t = 5;
    pub const XCB_RENDER_PICT_OP_IN : xcb_render_pict_op_t = 6;
    pub const XCB_RENDER_PICT_OP_IN_REVERSE : xcb_render_pict_op_t = 7;
    pub const XCB_RENDER_PICT_OP_OUT : xcb_render_pict_op_t = 8;
    pub const XCB_RENDER_PICT_OP_OUT_REVERSE : xcb_render_pict_op_t = 9;
    pub const XCB_RENDER_PICT_OP_ATOP : xcb_render_pict_op_t = 10;
    pub const XCB_RENDER_PICT_OP_ATOP_REVERSE : xcb_render_pict_op_t = 11;
    pub const XCB_RENDER_PICT_OP_XOR : xcb_render_pict_op_t = 12;
    pub const XCB_RENDER_PICT_OP_ADD : xcb_render_pict_op_t = 13;
    pub const XCB_RENDER_PICT_OP_SATURATE : xcb_render_pict_op_t = 14;
    pub const XCB_RENDER_PICT_OP_DISJOINT_CLEAR : xcb_render_pict_op_t = 16;
    pub const XCB_RENDER_PICT_OP_DISJOINT_SRC : xcb_render_pict_op_t = 17;
    pub const XCB_RENDER_PICT_OP_DISJOINT_DST : xcb_render_pict_op_t = 18;
    pub const XCB_RENDER_PICT_OP_DISJOINT_OVER : xcb_render_pict_op_t = 19;
    pub const XCB_RENDER_PICT_OP_DISJOINT_OVER_REVERSE : xcb_render_pict_op_t = 20;
    pub const XCB_RENDER_PICT_OP_DISJOINT_IN : xcb_render_pict_op_t = 21;
    pub const XCB_RENDER_PICT_OP_DISJOINT_IN_REVERSE : xcb_render_pict_op_t = 22;
    pub const XCB_RENDER_PICT_OP_DISJOINT_OUT : xcb_render_pict_op_t = 23;
    pub const XCB_RENDER_PICT_OP_DISJOINT_OUT_REVERSE : xcb_render_pict_op_t = 24;
    pub const XCB_RENDER_PICT_OP_DISJOINT_ATOP : xcb_render_pict_op_t = 25;
    pub const XCB_RENDER_PICT_OP_DISJOINT_ATOP_REVERSE : xcb_render_pict_op_t = 26;
    pub const XCB_RENDER_PICT_OP_DISJOINT_XOR : xcb_render_pict_op_t = 27;
    pub const XCB_RENDER_PICT_OP_CONJOINT_CLEAR : xcb_render_pict_op_t = 32;
    pub const XCB_RENDER_PICT_OP_CONJOINT_SRC : xcb_render_pict_op_t = 33;
    pub const XCB_RENDER_PICT_OP_CONJOINT_DST : xcb_render_pict_op_t = 34;
    pub const XCB_RENDER_PICT_OP_CONJOINT_OVER : xcb_render_pict_op_t = 35;
    pub const XCB_RENDER_PICT_OP_CONJOINT_OVER_REVERSE : xcb_render_pict_op_t = 36;
    pub const XCB_RENDER_PICT_OP_CONJOINT_IN : xcb_render_pict_op_t = 37;
    pub const XCB_RENDER_PICT_OP_CONJOINT_IN_REVERSE : xcb_render_pict_op_t = 38;
    pub const XCB_RENDER_PICT_OP_CONJOINT_OUT : xcb_render_pict_op_t = 39;
    pub const XCB_RENDER_PICT_OP_CONJOINT_OUT_REVERSE : xcb_render_pict_op_t = 40;
    pub const XCB_RENDER_PICT_OP_CONJOINT_ATOP : xcb_render_pict_op_t = 41;
    pub const XCB_RENDER_PICT_OP_CONJOINT_ATOP_REVERSE : xcb_render_pict_op_t = 42;
    pub const XCB_RENDER_PICT_OP_CONJOINT_XOR : xcb_render_pict_op_t = 43;
    pub const XCB_RENDER_PICT_OP_MULTIPLY : xcb_render_pict_op_t = 48;
    pub const XCB_RENDER_PICT_OP_SCREEN : xcb_render_pict_op_t = 49;
    pub const XCB_RENDER_PICT_OP_OVERLAY : xcb_render_pict_op_t = 50;
    pub const XCB_RENDER_PICT_OP_DARKEN : xcb_render_pict_op_t = 51;
    pub const XCB_RENDER_PICT_OP_LIGHTEN : xcb_render_pict_op_t = 52;
    pub const XCB_RENDER_PICT_OP_COLOR_DODGE : xcb_render_pict_op_t = 53;
    pub const XCB_RENDER_PICT_OP_COLOR_BURN : xcb_render_pict_op_t = 54;
    pub const XCB_RENDER_PICT_OP_HARD_LIGHT : xcb_render_pict_op_t = 55;
    pub const XCB_RENDER_PICT_OP_SOFT_LIGHT : xcb_render_pict_op_t = 56;
    pub const XCB_RENDER_PICT_OP_DIFFERENCE : xcb_render_pict_op_t = 57;
    pub const XCB_RENDER_PICT_OP_EXCLUSION : xcb_render_pict_op_t = 58;
    pub const XCB_RENDER_PICT_OP_HSL_HUE : xcb_render_pict_op_t = 59;
    pub const XCB_RENDER_PICT_OP_HSL_SATURATION : xcb_render_pict_op_t = 60;
    pub const XCB_RENDER_PICT_OP_HSL_COLOR : xcb_render_pict_op_t = 61;
    pub const XCB_RENDER_PICT_OP_HSL_LUMINOSITY : xcb_render_pict_op_t = 62;
//}

pub type xcb_render_poly_edge_t = c_uint;//{
    pub const XCB_RENDER_POLY_EDGE_SHARP : xcb_render_poly_edge_t = 1;
    pub const XCB_RENDER_POLY_EDGE_SMOOTH : xcb_render_poly_edge_t = 2;
//}

pub type xcb_render_poly_mode_t = c_uint;//{
    pub const XCB_RENDER_POLY_MODE_PRECISE : xcb_render_poly_mode_t = 1;
    pub const XCB_RENDER_POLY_MODE_IMPRECISE : xcb_render_poly_mode_t = 2;
//}

pub type xcb_render_cp_t = c_uint;//{
    pub const XCB_RENDER_CP_REPEAT : xcb_render_cp_t = 1;
    pub const XCB_RENDER_CP_ALPHA_MAP : xcb_render_cp_t = 2;
    pub const XCB_RENDER_CP_ALPHA_X_ORIGIN : xcb_render_cp_t = 4;
    pub const XCB_RENDER_CP_ALPHA_Y_ORIGIN : xcb_render_cp_t = 8;
    pub const XCB_RENDER_CP_CLIP_X_ORIGIN : xcb_render_cp_t = 16;
    pub const XCB_RENDER_CP_CLIP_Y_ORIGIN : xcb_render_cp_t = 32;
    pub const XCB_RENDER_CP_CLIP_MASK : xcb_render_cp_t = 64;
    pub const XCB_RENDER_CP_GRAPHICS_EXPOSURE : xcb_render_cp_t = 128;
    pub const XCB_RENDER_CP_SUBWINDOW_MODE : xcb_render_cp_t = 256;
    pub const XCB_RENDER_CP_POLY_EDGE : xcb_render_cp_t = 512;
    pub const XCB_RENDER_CP_POLY_MODE : xcb_render_cp_t = 1024;
    pub const XCB_RENDER_CP_DITHER : xcb_render_cp_t = 2048;
    pub const XCB_RENDER_CP_COMPONENT_ALPHA : xcb_render_cp_t = 4096;
//}

pub type xcb_render_sub_pixel_t = c_uint;//{
    pub const XCB_RENDER_SUB_PIXEL_UNKNOWN : xcb_render_sub_pixel_t = 1;
    pub const XCB_RENDER_SUB_PIXEL_HORIZONTAL_RGB : xcb_render_sub_pixel_t = 2;
    pub const XCB_RENDER_SUB_PIXEL_HORIZONTAL_BGR : xcb_render_sub_pixel_t = 3;
    pub const XCB_RENDER_SUB_PIXEL_VERTICAL_RGB : xcb_render_sub_pixel_t = 4;
    pub const XCB_RENDER_SUB_PIXEL_VERTICAL_BGR : xcb_render_sub_pixel_t = 5;
    pub const XCB_RENDER_SUB_PIXEL_NONE : xcb_render_sub_pixel_t = 6;
//}

pub type xcb_render_repeat_t = c_uint;//{
    pub const XCB_RENDER_REPEAT_NONE : xcb_render_repeat_t = 1;
    pub const XCB_RENDER_REPEAT_NORMAL : xcb_render_repeat_t = 2;
    pub const XCB_RENDER_REPEAT_PAD : xcb_render_repeat_t = 3;
    pub const XCB_RENDER_REPEAT_REFLECT : xcb_render_repeat_t = 4;
//}
pub type Glyph = xcb_render_glyph_t;

pub type GlyphIterator = xcb_render_glyph_iterator_t;

pub type GlyphsetIterator = xcb_render_glyphset_iterator_t;

pub type PictureIterator = xcb_render_picture_iterator_t;

pub type PictformatIterator = xcb_render_pictformat_iterator_t;

pub type FixedIterator = xcb_render_fixed_iterator_t;

/// Opcode for xcb_render_pict_format.
pub const XCB_RENDER_PICT_FORMAT : u8 = 0;
pub struct PictFormatError { pub base : base::Error<xcb_render_pict_format_error_t> }
/// Opcode for xcb_render_picture.
pub const XCB_RENDER_PICTURE : u8 = 1;
pub struct PictureError { pub base : base::Error<xcb_render_picture_error_t> }
/// Opcode for xcb_render_pict_op.
pub const XCB_RENDER_PICT_OP : u8 = 2;
pub struct PictOpError { pub base : base::Error<xcb_render_pict_op_error_t> }
/// Opcode for xcb_render_glyph_set.
pub const XCB_RENDER_GLYPH_SET : u8 = 3;
pub struct GlyphSetError { pub base : base::Error<xcb_render_glyph_set_error_t> }
/// Opcode for xcb_render_glyph.
pub const XCB_RENDER_GLYPH : u8 = 4;
pub struct GlyphError { pub base : base::Error<xcb_render_glyph_error_t> }
pub struct Directformat {pub base : base::Struct<xcb_render_directformat_t> }

pub type DirectformatIterator = xcb_render_directformat_iterator_t;

pub type PictforminfoIterator = xcb_render_pictforminfo_iterator_t;

pub type PictvisualIterator = xcb_render_pictvisual_iterator_t;

pub type PictdepthIterator = xcb_render_pictdepth_iterator_t;

pub type PictscreenIterator = xcb_render_pictscreen_iterator_t;

pub type IndexvalueIterator = xcb_render_indexvalue_iterator_t;

pub type ColorIterator = xcb_render_color_iterator_t;

pub type PointfixIterator = xcb_render_pointfix_iterator_t;

pub type LinefixIterator = xcb_render_linefix_iterator_t;

pub type TriangleIterator = xcb_render_triangle_iterator_t;

pub type TrapezoidIterator = xcb_render_trapezoid_iterator_t;

pub type GlyphinfoIterator = xcb_render_glyphinfo_iterator_t;

pub struct  QueryVersionCookie<'s> { pub base : base::Cookie<'s, xcb_render_query_version_cookie_t> }

/// Opcode for xcb_render_query_version.
pub const XCB_RENDER_QUERY_VERSION : u8 = 0;
pub struct QueryVersionReply { base:  base::Reply<xcb_render_query_version_reply_t> }
fn mk_reply_xcb_render_query_version_reply_t(reply:*mut xcb_render_query_version_reply_t) -> QueryVersionReply { QueryVersionReply { base : base::mk_reply(reply) } }
pub struct  QueryPictFormatsCookie<'s> { pub base : base::Cookie<'s, xcb_render_query_pict_formats_cookie_t> }

/// Opcode for xcb_render_query_pict_formats.
pub const XCB_RENDER_QUERY_PICT_FORMATS : u8 = 1;
pub struct  QueryPictIndexValuesCookie<'s> { pub base : base::Cookie<'s, xcb_render_query_pict_index_values_cookie_t> }

/// Opcode for xcb_render_query_pict_index_values.
pub const XCB_RENDER_QUERY_PICT_INDEX_VALUES : u8 = 2;
/// Opcode for xcb_render_create_picture.
pub const XCB_RENDER_CREATE_PICTURE : u8 = 4;
/// Opcode for xcb_render_change_picture.
pub const XCB_RENDER_CHANGE_PICTURE : u8 = 5;
/// Opcode for xcb_render_set_picture_clip_rectangles.
pub const XCB_RENDER_SET_PICTURE_CLIP_RECTANGLES : u8 = 6;
/// Opcode for xcb_render_free_picture.
pub const XCB_RENDER_FREE_PICTURE : u8 = 7;
/// Opcode for xcb_render_composite.
pub const XCB_RENDER_COMPOSITE : u8 = 8;
/// Opcode for xcb_render_trapezoids.
pub const XCB_RENDER_TRAPEZOIDS : u8 = 10;
/// Opcode for xcb_render_triangles.
pub const XCB_RENDER_TRIANGLES : u8 = 11;
/// Opcode for xcb_render_tri_strip.
pub const XCB_RENDER_TRI_STRIP : u8 = 12;
/// Opcode for xcb_render_tri_fan.
pub const XCB_RENDER_TRI_FAN : u8 = 13;
/// Opcode for xcb_render_create_glyph_set.
pub const XCB_RENDER_CREATE_GLYPH_SET : u8 = 17;
/// Opcode for xcb_render_reference_glyph_set.
pub const XCB_RENDER_REFERENCE_GLYPH_SET : u8 = 18;
/// Opcode for xcb_render_free_glyph_set.
pub const XCB_RENDER_FREE_GLYPH_SET : u8 = 19;
/// Opcode for xcb_render_add_glyphs.
pub const XCB_RENDER_ADD_GLYPHS : u8 = 20;
/// Opcode for xcb_render_free_glyphs.
pub const XCB_RENDER_FREE_GLYPHS : u8 = 22;
/// Opcode for xcb_render_composite_glyphs_8.
pub const XCB_RENDER_COMPOSITE_GLYPHS_8 : u8 = 23;
/// Opcode for xcb_render_composite_glyphs_16.
pub const XCB_RENDER_COMPOSITE_GLYPHS_16 : u8 = 24;
/// Opcode for xcb_render_composite_glyphs_32.
pub const XCB_RENDER_COMPOSITE_GLYPHS_32 : u8 = 25;
/// Opcode for xcb_render_fill_rectangles.
pub const XCB_RENDER_FILL_RECTANGLES : u8 = 26;
/// Opcode for xcb_render_create_cursor.
pub const XCB_RENDER_CREATE_CURSOR : u8 = 27;
pub type TransformIterator = xcb_render_transform_iterator_t;

/// Opcode for xcb_render_set_picture_transform.
pub const XCB_RENDER_SET_PICTURE_TRANSFORM : u8 = 28;
pub struct  QueryFiltersCookie<'s> { pub base : base::Cookie<'s, xcb_render_query_filters_cookie_t> }

/// Opcode for xcb_render_query_filters.
pub const XCB_RENDER_QUERY_FILTERS : u8 = 29;
/// Opcode for xcb_render_set_picture_filter.
pub const XCB_RENDER_SET_PICTURE_FILTER : u8 = 30;
pub type AnimcursoreltIterator = xcb_render_animcursorelt_iterator_t;

/// Opcode for xcb_render_create_anim_cursor.
pub const XCB_RENDER_CREATE_ANIM_CURSOR : u8 = 31;
pub type SpanfixIterator = xcb_render_spanfix_iterator_t;

pub type TrapIterator = xcb_render_trap_iterator_t;

/// Opcode for xcb_render_add_traps.
pub const XCB_RENDER_ADD_TRAPS : u8 = 32;
/// Opcode for xcb_render_create_solid_fill.
pub const XCB_RENDER_CREATE_SOLID_FILL : u8 = 33;
/// Opcode for xcb_render_create_linear_gradient.
pub const XCB_RENDER_CREATE_LINEAR_GRADIENT : u8 = 34;
/// Opcode for xcb_render_create_radial_gradient.
pub const XCB_RENDER_CREATE_RADIAL_GRADIENT : u8 = 35;
/// Opcode for xcb_render_create_conical_gradient.
pub const XCB_RENDER_CREATE_CONICAL_GRADIENT : u8 = 36;

impl Iterator for GlyphIterator {
    type Item = Glyph;
    fn next(&mut self) -> Option<Glyph> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_render_glyph_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_render_glyph_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub type Glyphset = xcb_render_glyphset_t;


impl Iterator for GlyphsetIterator {
    type Item = Glyphset;
    fn next(&mut self) -> Option<Glyphset> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_render_glyphset_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_render_glyphset_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub type Picture = xcb_render_picture_t;


impl Iterator for PictureIterator {
    type Item = Picture;
    fn next(&mut self) -> Option<Picture> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_render_picture_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_render_picture_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub type Pictformat = xcb_render_pictformat_t;


impl Iterator for PictformatIterator {
    type Item = Pictformat;
    fn next(&mut self) -> Option<Pictformat> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_render_pictformat_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_render_pictformat_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub type Fixed = xcb_render_fixed_t;


impl Iterator for FixedIterator {
    type Item = Fixed;
    fn next(&mut self) -> Option<Fixed> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_render_fixed_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_render_fixed_next(iter);
            Some(mem::transmute(*data))
        }
    }
}


impl Directformat {
  pub fn red_shift(&mut self) -> u16 {
    unsafe { accessor!(red_shift -> u16, self.base.strct) }
  }

  pub fn red_mask(&mut self) -> u16 {
    unsafe { accessor!(red_mask -> u16, self.base.strct) }
  }

  pub fn green_shift(&mut self) -> u16 {
    unsafe { accessor!(green_shift -> u16, self.base.strct) }
  }

  pub fn green_mask(&mut self) -> u16 {
    unsafe { accessor!(green_mask -> u16, self.base.strct) }
  }

  pub fn blue_shift(&mut self) -> u16 {
    unsafe { accessor!(blue_shift -> u16, self.base.strct) }
  }

  pub fn blue_mask(&mut self) -> u16 {
    unsafe { accessor!(blue_mask -> u16, self.base.strct) }
  }

  pub fn alpha_shift(&mut self) -> u16 {
    unsafe { accessor!(alpha_shift -> u16, self.base.strct) }
  }

  pub fn alpha_mask(&mut self) -> u16 {
    unsafe { accessor!(alpha_mask -> u16, self.base.strct) }
  }

}

impl Iterator for DirectformatIterator {
    type Item = Directformat;
    fn next(&mut self) -> Option<Directformat> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_render_directformat_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_render_directformat_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct Pictforminfo {pub base : base::Struct<xcb_render_pictforminfo_t> }


impl Pictforminfo {
  pub fn id(&mut self) -> Pictformat {
    unsafe { accessor!(id -> Pictformat, self.base.strct) }
  }

  pub fn type_(&mut self) -> u8 {
    unsafe { accessor!(type_ -> u8, self.base.strct) }
  }

  pub fn depth(&mut self) -> u8 {
    unsafe { accessor!(depth -> u8, self.base.strct) }
  }

  pub fn direct(&self) -> Directformat {
    unsafe { mem::transmute(self.base.strct.direct) }
  }
  pub fn colormap(&mut self) -> xproto::Colormap {
    unsafe { accessor!(colormap -> xproto::Colormap, self.base.strct) }
  }

}

impl Iterator for PictforminfoIterator {
    type Item = Pictforminfo;
    fn next(&mut self) -> Option<Pictforminfo> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_render_pictforminfo_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_render_pictforminfo_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct Pictvisual {pub base : base::Struct<xcb_render_pictvisual_t> }


impl Pictvisual {
  pub fn visual(&mut self) -> xproto::Visualid {
    unsafe { accessor!(visual -> xproto::Visualid, self.base.strct) }
  }

  pub fn format(&mut self) -> Pictformat {
    unsafe { accessor!(format -> Pictformat, self.base.strct) }
  }

}

impl Iterator for PictvisualIterator {
    type Item = Pictvisual;
    fn next(&mut self) -> Option<Pictvisual> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_render_pictvisual_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_render_pictvisual_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct Pictdepth {pub base : base::Struct<xcb_render_pictdepth_t> }


impl Pictdepth {
  pub fn depth(&mut self) -> u8 {
    unsafe { accessor!(depth -> u8, self.base.strct) }
  }

  pub fn visuals(&mut self) -> PictvisualIterator {
    unsafe { accessor!(PictvisualIterator, xcb_render_pictdepth_visuals_iterator, self.base.strct) }
  }

}

impl Iterator for PictdepthIterator {
    type Item = Pictdepth;
    fn next(&mut self) -> Option<Pictdepth> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_render_pictdepth_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_render_pictdepth_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct Pictscreen {pub base : base::Struct<xcb_render_pictscreen_t> }


impl Pictscreen {
  pub fn fallback(&mut self) -> Pictformat {
    unsafe { accessor!(fallback -> Pictformat, self.base.strct) }
  }

  pub fn depths(&mut self) -> PictdepthIterator {
    unsafe { accessor!(PictdepthIterator, xcb_render_pictscreen_depths_iterator, self.base.strct) }
  }

}

impl Iterator for PictscreenIterator {
    type Item = Pictscreen;
    fn next(&mut self) -> Option<Pictscreen> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_render_pictscreen_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_render_pictscreen_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct Indexvalue {pub base : base::Struct<xcb_render_indexvalue_t> }


impl Indexvalue {
  pub fn pixel(&mut self) -> u32 {
    unsafe { accessor!(pixel -> u32, self.base.strct) }
  }

  pub fn red(&mut self) -> u16 {
    unsafe { accessor!(red -> u16, self.base.strct) }
  }

  pub fn green(&mut self) -> u16 {
    unsafe { accessor!(green -> u16, self.base.strct) }
  }

  pub fn blue(&mut self) -> u16 {
    unsafe { accessor!(blue -> u16, self.base.strct) }
  }

  pub fn alpha(&mut self) -> u16 {
    unsafe { accessor!(alpha -> u16, self.base.strct) }
  }

}

impl Iterator for IndexvalueIterator {
    type Item = Indexvalue;
    fn next(&mut self) -> Option<Indexvalue> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_render_indexvalue_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_render_indexvalue_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct Color {pub base : base::Struct<xcb_render_color_t> }


impl Color {
  pub fn red(&mut self) -> u16 {
    unsafe { accessor!(red -> u16, self.base.strct) }
  }

  pub fn green(&mut self) -> u16 {
    unsafe { accessor!(green -> u16, self.base.strct) }
  }

  pub fn blue(&mut self) -> u16 {
    unsafe { accessor!(blue -> u16, self.base.strct) }
  }

  pub fn alpha(&mut self) -> u16 {
    unsafe { accessor!(alpha -> u16, self.base.strct) }
  }

}

impl Iterator for ColorIterator {
    type Item = Color;
    fn next(&mut self) -> Option<Color> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_render_color_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_render_color_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct Pointfix {pub base : base::Struct<xcb_render_pointfix_t> }


impl Pointfix {
  pub fn x(&mut self) -> Fixed {
    unsafe { accessor!(x -> Fixed, self.base.strct) }
  }

  pub fn y(&mut self) -> Fixed {
    unsafe { accessor!(y -> Fixed, self.base.strct) }
  }

}

impl Iterator for PointfixIterator {
    type Item = Pointfix;
    fn next(&mut self) -> Option<Pointfix> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_render_pointfix_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_render_pointfix_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct Linefix {pub base : base::Struct<xcb_render_linefix_t> }


impl Linefix {
  pub fn p1(&self) -> Pointfix {
    unsafe { mem::transmute(self.base.strct.p1) }
  }
  pub fn p2(&self) -> Pointfix {
    unsafe { mem::transmute(self.base.strct.p2) }
  }
}

impl Iterator for LinefixIterator {
    type Item = Linefix;
    fn next(&mut self) -> Option<Linefix> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_render_linefix_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_render_linefix_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct Triangle {pub base : base::Struct<xcb_render_triangle_t> }


impl Triangle {
  pub fn p1(&self) -> Pointfix {
    unsafe { mem::transmute(self.base.strct.p1) }
  }
  pub fn p2(&self) -> Pointfix {
    unsafe { mem::transmute(self.base.strct.p2) }
  }
  pub fn p3(&self) -> Pointfix {
    unsafe { mem::transmute(self.base.strct.p3) }
  }
}

impl Iterator for TriangleIterator {
    type Item = Triangle;
    fn next(&mut self) -> Option<Triangle> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_render_triangle_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_render_triangle_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct Trapezoid {pub base : base::Struct<xcb_render_trapezoid_t> }


impl Trapezoid {
  pub fn top(&mut self) -> Fixed {
    unsafe { accessor!(top -> Fixed, self.base.strct) }
  }

  pub fn bottom(&mut self) -> Fixed {
    unsafe { accessor!(bottom -> Fixed, self.base.strct) }
  }

  pub fn left(&self) -> Linefix {
    unsafe { mem::transmute(self.base.strct.left) }
  }
  pub fn right(&self) -> Linefix {
    unsafe { mem::transmute(self.base.strct.right) }
  }
}

impl Iterator for TrapezoidIterator {
    type Item = Trapezoid;
    fn next(&mut self) -> Option<Trapezoid> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_render_trapezoid_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_render_trapezoid_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct Glyphinfo {pub base : base::Struct<xcb_render_glyphinfo_t> }


impl Glyphinfo {
  pub fn width(&mut self) -> u16 {
    unsafe { accessor!(width -> u16, self.base.strct) }
  }

  pub fn height(&mut self) -> u16 {
    unsafe { accessor!(height -> u16, self.base.strct) }
  }

  pub fn x(&mut self) -> i16 {
    unsafe { accessor!(x -> i16, self.base.strct) }
  }

  pub fn y(&mut self) -> i16 {
    unsafe { accessor!(y -> i16, self.base.strct) }
  }

  pub fn x_off(&mut self) -> i16 {
    unsafe { accessor!(x_off -> i16, self.base.strct) }
  }

  pub fn y_off(&mut self) -> i16 {
    unsafe { accessor!(y_off -> i16, self.base.strct) }
  }

}

impl Iterator for GlyphinfoIterator {
    type Item = Glyphinfo;
    fn next(&mut self) -> Option<Glyphinfo> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_render_glyphinfo_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_render_glyphinfo_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub fn QueryVersion<'r> (c : &'r Connection,
                     client_major_version : u32,
                     client_minor_version : u32) -> QueryVersionCookie<'r> {
  unsafe {
    let cookie = xcb_render_query_version(c.get_raw_conn(),
        client_major_version as u32, //1
        client_minor_version as u32); //2
    QueryVersionCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn QueryVersionUnchecked<'r> (c : &'r Connection,
                              client_major_version : u32,
                              client_minor_version : u32) -> QueryVersionCookie<'r> {
  unsafe {
    let cookie = xcb_render_query_version_unchecked(c.get_raw_conn(),
        client_major_version as u32, //1
        client_minor_version as u32); //2
    QueryVersionCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl QueryVersionReply {
  pub fn major_version(&mut self) -> u32 {
    unsafe { accessor!(major_version -> u32, (*self.base.reply)) }
  }

  pub fn minor_version(&mut self) -> u32 {
    unsafe { accessor!(minor_version -> u32, (*self.base.reply)) }
  }

}
impl_reply_cookie!(QueryVersionCookie<'s>, mk_reply_xcb_render_query_version_reply_t, QueryVersionReply, xcb_render_query_version_reply);

pub struct QueryPictFormatsReply { base:  base::Reply<xcb_render_query_pict_formats_reply_t> }
fn mk_reply_xcb_render_query_pict_formats_reply_t(reply:*mut xcb_render_query_pict_formats_reply_t) -> QueryPictFormatsReply { QueryPictFormatsReply { base : base::mk_reply(reply) } }
pub fn QueryPictFormats<'r> (c : &'r Connection) -> QueryPictFormatsCookie<'r> {
  unsafe {
    let cookie = xcb_render_query_pict_formats(c.get_raw_conn());
    QueryPictFormatsCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn QueryPictFormatsUnchecked<'r> (c : &'r Connection) -> QueryPictFormatsCookie<'r> {
  unsafe {
    let cookie = xcb_render_query_pict_formats_unchecked(c.get_raw_conn());
    QueryPictFormatsCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl QueryPictFormatsReply {
  pub fn num_depths(&mut self) -> u32 {
    unsafe { accessor!(num_depths -> u32, (*self.base.reply)) }
  }

  pub fn num_visuals(&mut self) -> u32 {
    unsafe { accessor!(num_visuals -> u32, (*self.base.reply)) }
  }

  pub fn formats(&mut self) -> PictforminfoIterator {
    unsafe { accessor!(PictforminfoIterator, xcb_render_query_pict_formats_formats_iterator, (*self.base.reply)) }
  }

  pub fn screens(&mut self) -> PictscreenIterator {
    unsafe { accessor!(PictscreenIterator, xcb_render_query_pict_formats_screens_iterator, (*self.base.reply)) }
  }

  pub fn subpixels(&mut self) -> Vec<u32> {
    unsafe { accessor!(u32, xcb_render_query_pict_formats_subpixels_length, xcb_render_query_pict_formats_subpixels, (*self.base.reply)) }
  }

}
impl_reply_cookie!(QueryPictFormatsCookie<'s>, mk_reply_xcb_render_query_pict_formats_reply_t, QueryPictFormatsReply, xcb_render_query_pict_formats_reply);

pub struct QueryPictIndexValuesReply { base:  base::Reply<xcb_render_query_pict_index_values_reply_t> }
fn mk_reply_xcb_render_query_pict_index_values_reply_t(reply:*mut xcb_render_query_pict_index_values_reply_t) -> QueryPictIndexValuesReply { QueryPictIndexValuesReply { base : base::mk_reply(reply) } }
pub fn QueryPictIndexValues<'r> (c : &'r Connection,
                             format : Pictformat) -> QueryPictIndexValuesCookie<'r> {
  unsafe {
    let cookie = xcb_render_query_pict_index_values(c.get_raw_conn(),
        format as xcb_render_pictformat_t); //1
    QueryPictIndexValuesCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn QueryPictIndexValuesUnchecked<'r> (c : &'r Connection,
                                      format : Pictformat) -> QueryPictIndexValuesCookie<'r> {
  unsafe {
    let cookie = xcb_render_query_pict_index_values_unchecked(c.get_raw_conn(),
        format as xcb_render_pictformat_t); //1
    QueryPictIndexValuesCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl QueryPictIndexValuesReply {
  pub fn values(&mut self) -> IndexvalueIterator {
    unsafe { accessor!(IndexvalueIterator, xcb_render_query_pict_index_values_values_iterator, (*self.base.reply)) }
  }

}
impl_reply_cookie!(QueryPictIndexValuesCookie<'s>, mk_reply_xcb_render_query_pict_index_values_reply_t, QueryPictIndexValuesReply, xcb_render_query_pict_index_values_reply);

pub fn CreatePictureChecked<'r> (c : &'r Connection,
                             pid : Picture,
                             drawable : xproto::Drawable,
                             format : Pictformat,
                             value_list : &[(u32,u32)]) -> base::VoidCookie<'r> {
  unsafe {
    let mut value_list_copy = value_list.to_vec();
    let (value_list_mask, value_list_vec) = pack_bitfield(&mut value_list_copy);
    let value_list_ptr = value_list_vec.as_ptr();
    let cookie = xcb_render_create_picture_checked(c.get_raw_conn(),
        pid as xcb_render_picture_t, //1
        drawable as ffi::xproto::xcb_drawable_t, //2
        format as xcb_render_pictformat_t, //3
        value_list_mask as u32, //4
        value_list_ptr as *mut u32); //5
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn CreatePicture<'r> (c : &'r Connection,
                      pid : Picture,
                      drawable : xproto::Drawable,
                      format : Pictformat,
                      value_list : &[(u32,u32)]) -> base::VoidCookie<'r> {
  unsafe {
    let mut value_list_copy = value_list.to_vec();
    let (value_list_mask, value_list_vec) = pack_bitfield(&mut value_list_copy);
    let value_list_ptr = value_list_vec.as_ptr();
    let cookie = xcb_render_create_picture(c.get_raw_conn(),
        pid as xcb_render_picture_t, //1
        drawable as ffi::xproto::xcb_drawable_t, //2
        format as xcb_render_pictformat_t, //3
        value_list_mask as u32, //4
        value_list_ptr as *mut u32); //5
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn ChangePictureChecked<'r> (c : &'r Connection,
                             picture : Picture,
                             value_list : &[(u32,u32)]) -> base::VoidCookie<'r> {
  unsafe {
    let mut value_list_copy = value_list.to_vec();
    let (value_list_mask, value_list_vec) = pack_bitfield(&mut value_list_copy);
    let value_list_ptr = value_list_vec.as_ptr();
    let cookie = xcb_render_change_picture_checked(c.get_raw_conn(),
        picture as xcb_render_picture_t, //1
        value_list_mask as u32, //2
        value_list_ptr as *mut u32); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn ChangePicture<'r> (c : &'r Connection,
                      picture : Picture,
                      value_list : &[(u32,u32)]) -> base::VoidCookie<'r> {
  unsafe {
    let mut value_list_copy = value_list.to_vec();
    let (value_list_mask, value_list_vec) = pack_bitfield(&mut value_list_copy);
    let value_list_ptr = value_list_vec.as_ptr();
    let cookie = xcb_render_change_picture(c.get_raw_conn(),
        picture as xcb_render_picture_t, //1
        value_list_mask as u32, //2
        value_list_ptr as *mut u32); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn SetPictureClipRectanglesChecked<'r> (c : &'r Connection,
                                        picture : Picture,
                                        clip_x_origin : i16,
                                        clip_y_origin : i16,
                                        rectangles : &[xproto::Rectangle]) -> base::VoidCookie<'r> {
  unsafe {
    let rectangles_len = rectangles.len();
    let rectangles_ptr = rectangles.as_ptr();
    let cookie = xcb_render_set_picture_clip_rectangles_checked(c.get_raw_conn(),
        picture as xcb_render_picture_t, //1
        clip_x_origin as i16, //2
        clip_y_origin as i16, //3
        rectangles_len as u32, //4
        rectangles_ptr as *mut ffi::xproto::xcb_rectangle_t); //5
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn SetPictureClipRectangles<'r> (c : &'r Connection,
                                 picture : Picture,
                                 clip_x_origin : i16,
                                 clip_y_origin : i16,
                                 rectangles : &[xproto::Rectangle]) -> base::VoidCookie<'r> {
  unsafe {
    let rectangles_len = rectangles.len();
    let rectangles_ptr = rectangles.as_ptr();
    let cookie = xcb_render_set_picture_clip_rectangles(c.get_raw_conn(),
        picture as xcb_render_picture_t, //1
        clip_x_origin as i16, //2
        clip_y_origin as i16, //3
        rectangles_len as u32, //4
        rectangles_ptr as *mut ffi::xproto::xcb_rectangle_t); //5
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn FreePictureChecked<'r> (c : &'r Connection,
                           picture : Picture) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_render_free_picture_checked(c.get_raw_conn(),
        picture as xcb_render_picture_t); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn FreePicture<'r> (c : &'r Connection,
                    picture : Picture) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_render_free_picture(c.get_raw_conn(),
        picture as xcb_render_picture_t); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn CompositeChecked<'r> (c : &'r Connection,
                         op : u8,
                         src : Picture,
                         mask : Picture,
                         dst : Picture,
                         src_x : i16,
                         src_y : i16,
                         mask_x : i16,
                         mask_y : i16,
                         dst_x : i16,
                         dst_y : i16,
                         width : u16,
                         height : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_render_composite_checked(c.get_raw_conn(),
        op as u8, //1
        src as xcb_render_picture_t, //2
        mask as xcb_render_picture_t, //3
        dst as xcb_render_picture_t, //4
        src_x as i16, //5
        src_y as i16, //6
        mask_x as i16, //7
        mask_y as i16, //8
        dst_x as i16, //9
        dst_y as i16, //10
        width as u16, //11
        height as u16); //12
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn Composite<'r> (c : &'r Connection,
                  op : u8,
                  src : Picture,
                  mask : Picture,
                  dst : Picture,
                  src_x : i16,
                  src_y : i16,
                  mask_x : i16,
                  mask_y : i16,
                  dst_x : i16,
                  dst_y : i16,
                  width : u16,
                  height : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_render_composite(c.get_raw_conn(),
        op as u8, //1
        src as xcb_render_picture_t, //2
        mask as xcb_render_picture_t, //3
        dst as xcb_render_picture_t, //4
        src_x as i16, //5
        src_y as i16, //6
        mask_x as i16, //7
        mask_y as i16, //8
        dst_x as i16, //9
        dst_y as i16, //10
        width as u16, //11
        height as u16); //12
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn TrapezoidsChecked<'r> (c : &'r Connection,
                          op : u8,
                          src : Picture,
                          dst : Picture,
                          mask_format : Pictformat,
                          src_x : i16,
                          src_y : i16,
                          traps : &[Trapezoid]) -> base::VoidCookie<'r> {
  unsafe {
    let traps_len = traps.len();
    let traps_ptr = traps.as_ptr();
    let cookie = xcb_render_trapezoids_checked(c.get_raw_conn(),
        op as u8, //1
        src as xcb_render_picture_t, //2
        dst as xcb_render_picture_t, //3
        mask_format as xcb_render_pictformat_t, //4
        src_x as i16, //5
        src_y as i16, //6
        traps_len as u32, //7
        traps_ptr as *mut xcb_render_trapezoid_t); //8
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn Trapezoids<'r> (c : &'r Connection,
                   op : u8,
                   src : Picture,
                   dst : Picture,
                   mask_format : Pictformat,
                   src_x : i16,
                   src_y : i16,
                   traps : &[Trapezoid]) -> base::VoidCookie<'r> {
  unsafe {
    let traps_len = traps.len();
    let traps_ptr = traps.as_ptr();
    let cookie = xcb_render_trapezoids(c.get_raw_conn(),
        op as u8, //1
        src as xcb_render_picture_t, //2
        dst as xcb_render_picture_t, //3
        mask_format as xcb_render_pictformat_t, //4
        src_x as i16, //5
        src_y as i16, //6
        traps_len as u32, //7
        traps_ptr as *mut xcb_render_trapezoid_t); //8
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn TrianglesChecked<'r> (c : &'r Connection,
                         op : u8,
                         src : Picture,
                         dst : Picture,
                         mask_format : Pictformat,
                         src_x : i16,
                         src_y : i16,
                         triangles : &[Triangle]) -> base::VoidCookie<'r> {
  unsafe {
    let triangles_len = triangles.len();
    let triangles_ptr = triangles.as_ptr();
    let cookie = xcb_render_triangles_checked(c.get_raw_conn(),
        op as u8, //1
        src as xcb_render_picture_t, //2
        dst as xcb_render_picture_t, //3
        mask_format as xcb_render_pictformat_t, //4
        src_x as i16, //5
        src_y as i16, //6
        triangles_len as u32, //7
        triangles_ptr as *mut xcb_render_triangle_t); //8
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn Triangles<'r> (c : &'r Connection,
                  op : u8,
                  src : Picture,
                  dst : Picture,
                  mask_format : Pictformat,
                  src_x : i16,
                  src_y : i16,
                  triangles : &[Triangle]) -> base::VoidCookie<'r> {
  unsafe {
    let triangles_len = triangles.len();
    let triangles_ptr = triangles.as_ptr();
    let cookie = xcb_render_triangles(c.get_raw_conn(),
        op as u8, //1
        src as xcb_render_picture_t, //2
        dst as xcb_render_picture_t, //3
        mask_format as xcb_render_pictformat_t, //4
        src_x as i16, //5
        src_y as i16, //6
        triangles_len as u32, //7
        triangles_ptr as *mut xcb_render_triangle_t); //8
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn TriStripChecked<'r> (c : &'r Connection,
                        op : u8,
                        src : Picture,
                        dst : Picture,
                        mask_format : Pictformat,
                        src_x : i16,
                        src_y : i16,
                        points : &[Pointfix]) -> base::VoidCookie<'r> {
  unsafe {
    let points_len = points.len();
    let points_ptr = points.as_ptr();
    let cookie = xcb_render_tri_strip_checked(c.get_raw_conn(),
        op as u8, //1
        src as xcb_render_picture_t, //2
        dst as xcb_render_picture_t, //3
        mask_format as xcb_render_pictformat_t, //4
        src_x as i16, //5
        src_y as i16, //6
        points_len as u32, //7
        points_ptr as *mut xcb_render_pointfix_t); //8
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn TriStrip<'r> (c : &'r Connection,
                 op : u8,
                 src : Picture,
                 dst : Picture,
                 mask_format : Pictformat,
                 src_x : i16,
                 src_y : i16,
                 points : &[Pointfix]) -> base::VoidCookie<'r> {
  unsafe {
    let points_len = points.len();
    let points_ptr = points.as_ptr();
    let cookie = xcb_render_tri_strip(c.get_raw_conn(),
        op as u8, //1
        src as xcb_render_picture_t, //2
        dst as xcb_render_picture_t, //3
        mask_format as xcb_render_pictformat_t, //4
        src_x as i16, //5
        src_y as i16, //6
        points_len as u32, //7
        points_ptr as *mut xcb_render_pointfix_t); //8
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn TriFanChecked<'r> (c : &'r Connection,
                      op : u8,
                      src : Picture,
                      dst : Picture,
                      mask_format : Pictformat,
                      src_x : i16,
                      src_y : i16,
                      points : &[Pointfix]) -> base::VoidCookie<'r> {
  unsafe {
    let points_len = points.len();
    let points_ptr = points.as_ptr();
    let cookie = xcb_render_tri_fan_checked(c.get_raw_conn(),
        op as u8, //1
        src as xcb_render_picture_t, //2
        dst as xcb_render_picture_t, //3
        mask_format as xcb_render_pictformat_t, //4
        src_x as i16, //5
        src_y as i16, //6
        points_len as u32, //7
        points_ptr as *mut xcb_render_pointfix_t); //8
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn TriFan<'r> (c : &'r Connection,
               op : u8,
               src : Picture,
               dst : Picture,
               mask_format : Pictformat,
               src_x : i16,
               src_y : i16,
               points : &[Pointfix]) -> base::VoidCookie<'r> {
  unsafe {
    let points_len = points.len();
    let points_ptr = points.as_ptr();
    let cookie = xcb_render_tri_fan(c.get_raw_conn(),
        op as u8, //1
        src as xcb_render_picture_t, //2
        dst as xcb_render_picture_t, //3
        mask_format as xcb_render_pictformat_t, //4
        src_x as i16, //5
        src_y as i16, //6
        points_len as u32, //7
        points_ptr as *mut xcb_render_pointfix_t); //8
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn CreateGlyphSetChecked<'r> (c : &'r Connection,
                              gsid : Glyphset,
                              format : Pictformat) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_render_create_glyph_set_checked(c.get_raw_conn(),
        gsid as xcb_render_glyphset_t, //1
        format as xcb_render_pictformat_t); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn CreateGlyphSet<'r> (c : &'r Connection,
                       gsid : Glyphset,
                       format : Pictformat) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_render_create_glyph_set(c.get_raw_conn(),
        gsid as xcb_render_glyphset_t, //1
        format as xcb_render_pictformat_t); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn ReferenceGlyphSetChecked<'r> (c : &'r Connection,
                                 gsid : Glyphset,
                                 existing : Glyphset) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_render_reference_glyph_set_checked(c.get_raw_conn(),
        gsid as xcb_render_glyphset_t, //1
        existing as xcb_render_glyphset_t); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn ReferenceGlyphSet<'r> (c : &'r Connection,
                          gsid : Glyphset,
                          existing : Glyphset) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_render_reference_glyph_set(c.get_raw_conn(),
        gsid as xcb_render_glyphset_t, //1
        existing as xcb_render_glyphset_t); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn FreeGlyphSetChecked<'r> (c : &'r Connection,
                            glyphset : Glyphset) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_render_free_glyph_set_checked(c.get_raw_conn(),
        glyphset as xcb_render_glyphset_t); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn FreeGlyphSet<'r> (c : &'r Connection,
                     glyphset : Glyphset) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_render_free_glyph_set(c.get_raw_conn(),
        glyphset as xcb_render_glyphset_t); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn AddGlyphsChecked<'r> (c : &'r Connection,
                         glyphset : Glyphset,
                         glyphids : &[u32],
                         glyphs : &[Glyphinfo],
                         data : &[u8]) -> base::VoidCookie<'r> {
  unsafe {
    let glyphids_len = glyphids.len();
    let glyphids_ptr = glyphids.as_ptr();
    let glyphs_ptr = glyphs.as_ptr();
    let data_len = data.len();
    let data_ptr = data.as_ptr();
    let cookie = xcb_render_add_glyphs_checked(c.get_raw_conn(),
        glyphset as xcb_render_glyphset_t, //1
        glyphids_len as u32, //2
        glyphids_ptr as *mut u32, //3
        glyphs_ptr as *mut xcb_render_glyphinfo_t, //4
        data_len as u32, //5
        data_ptr as *mut u8); //6
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn AddGlyphs<'r> (c : &'r Connection,
                  glyphset : Glyphset,
                  glyphids : &[u32],
                  glyphs : &[Glyphinfo],
                  data : &[u8]) -> base::VoidCookie<'r> {
  unsafe {
    let glyphids_len = glyphids.len();
    let glyphids_ptr = glyphids.as_ptr();
    let glyphs_ptr = glyphs.as_ptr();
    let data_len = data.len();
    let data_ptr = data.as_ptr();
    let cookie = xcb_render_add_glyphs(c.get_raw_conn(),
        glyphset as xcb_render_glyphset_t, //1
        glyphids_len as u32, //2
        glyphids_ptr as *mut u32, //3
        glyphs_ptr as *mut xcb_render_glyphinfo_t, //4
        data_len as u32, //5
        data_ptr as *mut u8); //6
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn FreeGlyphsChecked<'r> (c : &'r Connection,
                          glyphset : Glyphset,
                          glyphs : &[Glyph]) -> base::VoidCookie<'r> {
  unsafe {
    let glyphs_len = glyphs.len();
    let glyphs_ptr = glyphs.as_ptr();
    let cookie = xcb_render_free_glyphs_checked(c.get_raw_conn(),
        glyphset as xcb_render_glyphset_t, //1
        glyphs_len as u32, //2
        glyphs_ptr as *mut xcb_render_glyph_t); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn FreeGlyphs<'r> (c : &'r Connection,
                   glyphset : Glyphset,
                   glyphs : &[Glyph]) -> base::VoidCookie<'r> {
  unsafe {
    let glyphs_len = glyphs.len();
    let glyphs_ptr = glyphs.as_ptr();
    let cookie = xcb_render_free_glyphs(c.get_raw_conn(),
        glyphset as xcb_render_glyphset_t, //1
        glyphs_len as u32, //2
        glyphs_ptr as *mut xcb_render_glyph_t); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn CompositeGlyphs8Checked<'r> (c : &'r Connection,
                                op : u8,
                                src : Picture,
                                dst : Picture,
                                mask_format : Pictformat,
                                glyphset : Glyphset,
                                src_x : i16,
                                src_y : i16,
                                glyphcmds : &[u8]) -> base::VoidCookie<'r> {
  unsafe {
    let glyphcmds_len = glyphcmds.len();
    let glyphcmds_ptr = glyphcmds.as_ptr();
    let cookie = xcb_render_composite_glyphs_8_checked(c.get_raw_conn(),
        op as u8, //1
        src as xcb_render_picture_t, //2
        dst as xcb_render_picture_t, //3
        mask_format as xcb_render_pictformat_t, //4
        glyphset as xcb_render_glyphset_t, //5
        src_x as i16, //6
        src_y as i16, //7
        glyphcmds_len as u32, //8
        glyphcmds_ptr as *mut u8); //9
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn CompositeGlyphs8<'r> (c : &'r Connection,
                         op : u8,
                         src : Picture,
                         dst : Picture,
                         mask_format : Pictformat,
                         glyphset : Glyphset,
                         src_x : i16,
                         src_y : i16,
                         glyphcmds : &[u8]) -> base::VoidCookie<'r> {
  unsafe {
    let glyphcmds_len = glyphcmds.len();
    let glyphcmds_ptr = glyphcmds.as_ptr();
    let cookie = xcb_render_composite_glyphs_8(c.get_raw_conn(),
        op as u8, //1
        src as xcb_render_picture_t, //2
        dst as xcb_render_picture_t, //3
        mask_format as xcb_render_pictformat_t, //4
        glyphset as xcb_render_glyphset_t, //5
        src_x as i16, //6
        src_y as i16, //7
        glyphcmds_len as u32, //8
        glyphcmds_ptr as *mut u8); //9
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn CompositeGlyphs16Checked<'r> (c : &'r Connection,
                                 op : u8,
                                 src : Picture,
                                 dst : Picture,
                                 mask_format : Pictformat,
                                 glyphset : Glyphset,
                                 src_x : i16,
                                 src_y : i16,
                                 glyphcmds : &[u8]) -> base::VoidCookie<'r> {
  unsafe {
    let glyphcmds_len = glyphcmds.len();
    let glyphcmds_ptr = glyphcmds.as_ptr();
    let cookie = xcb_render_composite_glyphs_16_checked(c.get_raw_conn(),
        op as u8, //1
        src as xcb_render_picture_t, //2
        dst as xcb_render_picture_t, //3
        mask_format as xcb_render_pictformat_t, //4
        glyphset as xcb_render_glyphset_t, //5
        src_x as i16, //6
        src_y as i16, //7
        glyphcmds_len as u32, //8
        glyphcmds_ptr as *mut u8); //9
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn CompositeGlyphs16<'r> (c : &'r Connection,
                          op : u8,
                          src : Picture,
                          dst : Picture,
                          mask_format : Pictformat,
                          glyphset : Glyphset,
                          src_x : i16,
                          src_y : i16,
                          glyphcmds : &[u8]) -> base::VoidCookie<'r> {
  unsafe {
    let glyphcmds_len = glyphcmds.len();
    let glyphcmds_ptr = glyphcmds.as_ptr();
    let cookie = xcb_render_composite_glyphs_16(c.get_raw_conn(),
        op as u8, //1
        src as xcb_render_picture_t, //2
        dst as xcb_render_picture_t, //3
        mask_format as xcb_render_pictformat_t, //4
        glyphset as xcb_render_glyphset_t, //5
        src_x as i16, //6
        src_y as i16, //7
        glyphcmds_len as u32, //8
        glyphcmds_ptr as *mut u8); //9
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn CompositeGlyphs32Checked<'r> (c : &'r Connection,
                                 op : u8,
                                 src : Picture,
                                 dst : Picture,
                                 mask_format : Pictformat,
                                 glyphset : Glyphset,
                                 src_x : i16,
                                 src_y : i16,
                                 glyphcmds : &[u8]) -> base::VoidCookie<'r> {
  unsafe {
    let glyphcmds_len = glyphcmds.len();
    let glyphcmds_ptr = glyphcmds.as_ptr();
    let cookie = xcb_render_composite_glyphs_32_checked(c.get_raw_conn(),
        op as u8, //1
        src as xcb_render_picture_t, //2
        dst as xcb_render_picture_t, //3
        mask_format as xcb_render_pictformat_t, //4
        glyphset as xcb_render_glyphset_t, //5
        src_x as i16, //6
        src_y as i16, //7
        glyphcmds_len as u32, //8
        glyphcmds_ptr as *mut u8); //9
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn CompositeGlyphs32<'r> (c : &'r Connection,
                          op : u8,
                          src : Picture,
                          dst : Picture,
                          mask_format : Pictformat,
                          glyphset : Glyphset,
                          src_x : i16,
                          src_y : i16,
                          glyphcmds : &[u8]) -> base::VoidCookie<'r> {
  unsafe {
    let glyphcmds_len = glyphcmds.len();
    let glyphcmds_ptr = glyphcmds.as_ptr();
    let cookie = xcb_render_composite_glyphs_32(c.get_raw_conn(),
        op as u8, //1
        src as xcb_render_picture_t, //2
        dst as xcb_render_picture_t, //3
        mask_format as xcb_render_pictformat_t, //4
        glyphset as xcb_render_glyphset_t, //5
        src_x as i16, //6
        src_y as i16, //7
        glyphcmds_len as u32, //8
        glyphcmds_ptr as *mut u8); //9
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn FillRectanglesChecked<'r> (c : &'r Connection,
                              op : u8,
                              dst : Picture,
                              color : Color,
                              rects : &[xproto::Rectangle]) -> base::VoidCookie<'r> {
  unsafe {
    let rects_len = rects.len();
    let rects_ptr = rects.as_ptr();
    let cookie = xcb_render_fill_rectangles_checked(c.get_raw_conn(),
        op as u8, //1
        dst as xcb_render_picture_t, //2
        color.base.strct, //3
        rects_len as u32, //4
        rects_ptr as *mut ffi::xproto::xcb_rectangle_t); //5
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn FillRectangles<'r> (c : &'r Connection,
                       op : u8,
                       dst : Picture,
                       color : Color,
                       rects : &[xproto::Rectangle]) -> base::VoidCookie<'r> {
  unsafe {
    let rects_len = rects.len();
    let rects_ptr = rects.as_ptr();
    let cookie = xcb_render_fill_rectangles(c.get_raw_conn(),
        op as u8, //1
        dst as xcb_render_picture_t, //2
        color.base.strct, //3
        rects_len as u32, //4
        rects_ptr as *mut ffi::xproto::xcb_rectangle_t); //5
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn CreateCursorChecked<'r> (c : &'r Connection,
                            cid : xproto::Cursor,
                            source : Picture,
                            x : u16,
                            y : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_render_create_cursor_checked(c.get_raw_conn(),
        cid as ffi::xproto::xcb_cursor_t, //1
        source as xcb_render_picture_t, //2
        x as u16, //3
        y as u16); //4
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn CreateCursor<'r> (c : &'r Connection,
                     cid : xproto::Cursor,
                     source : Picture,
                     x : u16,
                     y : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_render_create_cursor(c.get_raw_conn(),
        cid as ffi::xproto::xcb_cursor_t, //1
        source as xcb_render_picture_t, //2
        x as u16, //3
        y as u16); //4
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub struct Transform {pub base : base::Struct<xcb_render_transform_t> }


impl Transform {
  pub fn matrix11(&mut self) -> Fixed {
    unsafe { accessor!(matrix11 -> Fixed, self.base.strct) }
  }

  pub fn matrix12(&mut self) -> Fixed {
    unsafe { accessor!(matrix12 -> Fixed, self.base.strct) }
  }

  pub fn matrix13(&mut self) -> Fixed {
    unsafe { accessor!(matrix13 -> Fixed, self.base.strct) }
  }

  pub fn matrix21(&mut self) -> Fixed {
    unsafe { accessor!(matrix21 -> Fixed, self.base.strct) }
  }

  pub fn matrix22(&mut self) -> Fixed {
    unsafe { accessor!(matrix22 -> Fixed, self.base.strct) }
  }

  pub fn matrix23(&mut self) -> Fixed {
    unsafe { accessor!(matrix23 -> Fixed, self.base.strct) }
  }

  pub fn matrix31(&mut self) -> Fixed {
    unsafe { accessor!(matrix31 -> Fixed, self.base.strct) }
  }

  pub fn matrix32(&mut self) -> Fixed {
    unsafe { accessor!(matrix32 -> Fixed, self.base.strct) }
  }

  pub fn matrix33(&mut self) -> Fixed {
    unsafe { accessor!(matrix33 -> Fixed, self.base.strct) }
  }

}

impl Iterator for TransformIterator {
    type Item = Transform;
    fn next(&mut self) -> Option<Transform> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_render_transform_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_render_transform_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub fn SetPictureTransformChecked<'r> (c : &'r Connection,
                                   picture : Picture,
                                   transform : Transform) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_render_set_picture_transform_checked(c.get_raw_conn(),
        picture as xcb_render_picture_t, //1
        transform.base.strct); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn SetPictureTransform<'r> (c : &'r Connection,
                            picture : Picture,
                            transform : Transform) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_render_set_picture_transform(c.get_raw_conn(),
        picture as xcb_render_picture_t, //1
        transform.base.strct); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub struct QueryFiltersReply { base:  base::Reply<xcb_render_query_filters_reply_t> }
fn mk_reply_xcb_render_query_filters_reply_t(reply:*mut xcb_render_query_filters_reply_t) -> QueryFiltersReply { QueryFiltersReply { base : base::mk_reply(reply) } }
pub fn QueryFilters<'r> (c : &'r Connection,
                     drawable : xproto::Drawable) -> QueryFiltersCookie<'r> {
  unsafe {
    let cookie = xcb_render_query_filters(c.get_raw_conn(),
        drawable as ffi::xproto::xcb_drawable_t); //1
    QueryFiltersCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn QueryFiltersUnchecked<'r> (c : &'r Connection,
                              drawable : xproto::Drawable) -> QueryFiltersCookie<'r> {
  unsafe {
    let cookie = xcb_render_query_filters_unchecked(c.get_raw_conn(),
        drawable as ffi::xproto::xcb_drawable_t); //1
    QueryFiltersCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl QueryFiltersReply {
  pub fn aliases(&mut self) -> Vec<u16> {
    unsafe { accessor!(u16, xcb_render_query_filters_aliases_length, xcb_render_query_filters_aliases, (*self.base.reply)) }
  }

  pub fn filters(&mut self) -> xproto::StrIterator {
    unsafe { accessor!(xproto::StrIterator, xcb_render_query_filters_filters_iterator, (*self.base.reply)) }
  }

}
impl_reply_cookie!(QueryFiltersCookie<'s>, mk_reply_xcb_render_query_filters_reply_t, QueryFiltersReply, xcb_render_query_filters_reply);

pub fn SetPictureFilterChecked<'r> (c : &'r Connection,
                                picture : Picture,
                                filter : &str,
                                values : &[Fixed]) -> base::VoidCookie<'r> {
  unsafe {
    let filter = (filter).as_bytes();
    let filter_len = filter.len();
    let filter_ptr = filter.as_ptr();
    let values_len = values.len();
    let values_ptr = values.as_ptr();
    let cookie = xcb_render_set_picture_filter_checked(c.get_raw_conn(),
        picture as xcb_render_picture_t, //1
        filter_len as u16, //2
        filter_ptr as *mut c_char, //3
        values_len as u32, //4
        values_ptr as *mut xcb_render_fixed_t); //5
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn SetPictureFilter<'r> (c : &'r Connection,
                         picture : Picture,
                         filter : &str,
                         values : &[Fixed]) -> base::VoidCookie<'r> {
  unsafe {
    let filter = (filter).as_bytes();
    let filter_len = filter.len();
    let filter_ptr = filter.as_ptr();
    let values_len = values.len();
    let values_ptr = values.as_ptr();
    let cookie = xcb_render_set_picture_filter(c.get_raw_conn(),
        picture as xcb_render_picture_t, //1
        filter_len as u16, //2
        filter_ptr as *mut c_char, //3
        values_len as u32, //4
        values_ptr as *mut xcb_render_fixed_t); //5
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub struct Animcursorelt {pub base : base::Struct<xcb_render_animcursorelt_t> }


impl Animcursorelt {
  pub fn cursor(&mut self) -> xproto::Cursor {
    unsafe { accessor!(cursor -> xproto::Cursor, self.base.strct) }
  }

  pub fn delay(&mut self) -> u32 {
    unsafe { accessor!(delay -> u32, self.base.strct) }
  }

}

impl Iterator for AnimcursoreltIterator {
    type Item = Animcursorelt;
    fn next(&mut self) -> Option<Animcursorelt> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_render_animcursorelt_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_render_animcursorelt_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub fn CreateAnimCursorChecked<'r> (c : &'r Connection,
                                cid : xproto::Cursor,
                                cursors : &[Animcursorelt]) -> base::VoidCookie<'r> {
  unsafe {
    let cursors_len = cursors.len();
    let cursors_ptr = cursors.as_ptr();
    let cookie = xcb_render_create_anim_cursor_checked(c.get_raw_conn(),
        cid as ffi::xproto::xcb_cursor_t, //1
        cursors_len as u32, //2
        cursors_ptr as *mut xcb_render_animcursorelt_t); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn CreateAnimCursor<'r> (c : &'r Connection,
                         cid : xproto::Cursor,
                         cursors : &[Animcursorelt]) -> base::VoidCookie<'r> {
  unsafe {
    let cursors_len = cursors.len();
    let cursors_ptr = cursors.as_ptr();
    let cookie = xcb_render_create_anim_cursor(c.get_raw_conn(),
        cid as ffi::xproto::xcb_cursor_t, //1
        cursors_len as u32, //2
        cursors_ptr as *mut xcb_render_animcursorelt_t); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub struct Spanfix {pub base : base::Struct<xcb_render_spanfix_t> }


impl Spanfix {
  pub fn l(&mut self) -> Fixed {
    unsafe { accessor!(l -> Fixed, self.base.strct) }
  }

  pub fn r(&mut self) -> Fixed {
    unsafe { accessor!(r -> Fixed, self.base.strct) }
  }

  pub fn y(&mut self) -> Fixed {
    unsafe { accessor!(y -> Fixed, self.base.strct) }
  }

}

impl Iterator for SpanfixIterator {
    type Item = Spanfix;
    fn next(&mut self) -> Option<Spanfix> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_render_spanfix_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_render_spanfix_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct Trap {pub base : base::Struct<xcb_render_trap_t> }


impl Trap {
  pub fn top(&self) -> Spanfix {
    unsafe { mem::transmute(self.base.strct.top) }
  }
  pub fn bot(&self) -> Spanfix {
    unsafe { mem::transmute(self.base.strct.bot) }
  }
}

impl Iterator for TrapIterator {
    type Item = Trap;
    fn next(&mut self) -> Option<Trap> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_render_trap_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_render_trap_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub fn AddTrapsChecked<'r> (c : &'r Connection,
                        picture : Picture,
                        x_off : i16,
                        y_off : i16,
                        traps : &[Trap]) -> base::VoidCookie<'r> {
  unsafe {
    let traps_len = traps.len();
    let traps_ptr = traps.as_ptr();
    let cookie = xcb_render_add_traps_checked(c.get_raw_conn(),
        picture as xcb_render_picture_t, //1
        x_off as i16, //2
        y_off as i16, //3
        traps_len as u32, //4
        traps_ptr as *mut xcb_render_trap_t); //5
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn AddTraps<'r> (c : &'r Connection,
                 picture : Picture,
                 x_off : i16,
                 y_off : i16,
                 traps : &[Trap]) -> base::VoidCookie<'r> {
  unsafe {
    let traps_len = traps.len();
    let traps_ptr = traps.as_ptr();
    let cookie = xcb_render_add_traps(c.get_raw_conn(),
        picture as xcb_render_picture_t, //1
        x_off as i16, //2
        y_off as i16, //3
        traps_len as u32, //4
        traps_ptr as *mut xcb_render_trap_t); //5
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn CreateSolidFillChecked<'r> (c : &'r Connection,
                               picture : Picture,
                               color : Color) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_render_create_solid_fill_checked(c.get_raw_conn(),
        picture as xcb_render_picture_t, //1
        color.base.strct); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn CreateSolidFill<'r> (c : &'r Connection,
                        picture : Picture,
                        color : Color) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_render_create_solid_fill(c.get_raw_conn(),
        picture as xcb_render_picture_t, //1
        color.base.strct); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn CreateLinearGradientChecked<'r> (c : &'r Connection,
                                    picture : Picture,
                                    p1 : Pointfix,
                                    p2 : Pointfix,
                                    stops : &[Fixed],
                                    colors : &[Color]) -> base::VoidCookie<'r> {
  unsafe {
    let stops_len = stops.len();
    let stops_ptr = stops.as_ptr();
    let colors_ptr = colors.as_ptr();
    let cookie = xcb_render_create_linear_gradient_checked(c.get_raw_conn(),
        picture as xcb_render_picture_t, //1
        p1.base.strct, //2
        p2.base.strct, //3
        stops_len as u32, //4
        stops_ptr as *mut xcb_render_fixed_t, //5
        colors_ptr as *mut xcb_render_color_t); //6
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn CreateLinearGradient<'r> (c : &'r Connection,
                             picture : Picture,
                             p1 : Pointfix,
                             p2 : Pointfix,
                             stops : &[Fixed],
                             colors : &[Color]) -> base::VoidCookie<'r> {
  unsafe {
    let stops_len = stops.len();
    let stops_ptr = stops.as_ptr();
    let colors_ptr = colors.as_ptr();
    let cookie = xcb_render_create_linear_gradient(c.get_raw_conn(),
        picture as xcb_render_picture_t, //1
        p1.base.strct, //2
        p2.base.strct, //3
        stops_len as u32, //4
        stops_ptr as *mut xcb_render_fixed_t, //5
        colors_ptr as *mut xcb_render_color_t); //6
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn CreateRadialGradientChecked<'r> (c : &'r Connection,
                                    picture : Picture,
                                    inner : Pointfix,
                                    outer : Pointfix,
                                    inner_radius : Fixed,
                                    outer_radius : Fixed,
                                    stops : &[Fixed],
                                    colors : &[Color]) -> base::VoidCookie<'r> {
  unsafe {
    let stops_len = stops.len();
    let stops_ptr = stops.as_ptr();
    let colors_ptr = colors.as_ptr();
    let cookie = xcb_render_create_radial_gradient_checked(c.get_raw_conn(),
        picture as xcb_render_picture_t, //1
        inner.base.strct, //2
        outer.base.strct, //3
        inner_radius as xcb_render_fixed_t, //4
        outer_radius as xcb_render_fixed_t, //5
        stops_len as u32, //6
        stops_ptr as *mut xcb_render_fixed_t, //7
        colors_ptr as *mut xcb_render_color_t); //8
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn CreateRadialGradient<'r> (c : &'r Connection,
                             picture : Picture,
                             inner : Pointfix,
                             outer : Pointfix,
                             inner_radius : Fixed,
                             outer_radius : Fixed,
                             stops : &[Fixed],
                             colors : &[Color]) -> base::VoidCookie<'r> {
  unsafe {
    let stops_len = stops.len();
    let stops_ptr = stops.as_ptr();
    let colors_ptr = colors.as_ptr();
    let cookie = xcb_render_create_radial_gradient(c.get_raw_conn(),
        picture as xcb_render_picture_t, //1
        inner.base.strct, //2
        outer.base.strct, //3
        inner_radius as xcb_render_fixed_t, //4
        outer_radius as xcb_render_fixed_t, //5
        stops_len as u32, //6
        stops_ptr as *mut xcb_render_fixed_t, //7
        colors_ptr as *mut xcb_render_color_t); //8
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn CreateConicalGradientChecked<'r> (c : &'r Connection,
                                     picture : Picture,
                                     center : Pointfix,
                                     angle : Fixed,
                                     stops : &[Fixed],
                                     colors : &[Color]) -> base::VoidCookie<'r> {
  unsafe {
    let stops_len = stops.len();
    let stops_ptr = stops.as_ptr();
    let colors_ptr = colors.as_ptr();
    let cookie = xcb_render_create_conical_gradient_checked(c.get_raw_conn(),
        picture as xcb_render_picture_t, //1
        center.base.strct, //2
        angle as xcb_render_fixed_t, //3
        stops_len as u32, //4
        stops_ptr as *mut xcb_render_fixed_t, //5
        colors_ptr as *mut xcb_render_color_t); //6
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn CreateConicalGradient<'r> (c : &'r Connection,
                              picture : Picture,
                              center : Pointfix,
                              angle : Fixed,
                              stops : &[Fixed],
                              colors : &[Color]) -> base::VoidCookie<'r> {
  unsafe {
    let stops_len = stops.len();
    let stops_ptr = stops.as_ptr();
    let colors_ptr = colors.as_ptr();
    let cookie = xcb_render_create_conical_gradient(c.get_raw_conn(),
        picture as xcb_render_picture_t, //1
        center.base.strct, //2
        angle as xcb_render_fixed_t, //3
        stops_len as u32, //4
        stops_ptr as *mut xcb_render_fixed_t, //5
        colors_ptr as *mut xcb_render_color_t); //6
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

