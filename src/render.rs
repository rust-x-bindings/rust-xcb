/*
 * This file generated automatically from render.xml by r_client.py.
 * Edit at your peril.
 */

//Make the compiler quiet
#![allow(unused_imports)]
#![allow(unused_unsafe)]
use std;
use std::libc::*;
use std::{cast,num,ptr,str,libc};
use std::to_bytes::ToBytes;
use ffi::base::*;
use base;
use base::*;
use ffi;
use ffi::render::*;
use std::option::Option;
use std::iter::Iterator;

use xproto;

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
pub type Glyph = glyph;

pub type GlyphIterator = glyph_iterator;

pub type GlyphsetIterator = glyphset_iterator;

pub type PictureIterator = picture_iterator;

pub type PictformatIterator = pictformat_iterator;

pub type FixedIterator = fixed_iterator;

/** Opcode for xcb_render_pict_format. */
pub static XCB_RENDER_PICT_FORMAT : u8 = 0;
pub type PictFormatError = base::Error<pict_format_error>;
/** Opcode for xcb_render_picture. */
pub static XCB_RENDER_PICTURE : u8 = 1;
pub type PictureError = base::Error<picture_error>;
/** Opcode for xcb_render_pict_op. */
pub static XCB_RENDER_PICT_OP : u8 = 2;
pub type PictOpError = base::Error<pict_op_error>;
/** Opcode for xcb_render_glyph_set. */
pub static XCB_RENDER_GLYPH_SET : u8 = 3;
pub type GlyphSetError = base::Error<glyph_set_error>;
/** Opcode for xcb_render_glyph. */
pub static XCB_RENDER_GLYPH : u8 = 4;
pub type GlyphError = base::Error<glyph_error>;
pub type Directformat = base::Struct<directformat>;

pub type DirectformatIterator = directformat_iterator;

pub type PictforminfoIterator = pictforminfo_iterator;

pub type PictvisualIterator = pictvisual_iterator;

pub type PictdepthIterator = pictdepth_iterator;

pub type PictscreenIterator = pictscreen_iterator;

pub type IndexvalueIterator = indexvalue_iterator;

pub type ColorIterator = color_iterator;

pub type PointfixIterator = pointfix_iterator;

pub type LinefixIterator = linefix_iterator;

pub type TriangleIterator = triangle_iterator;

pub type TrapezoidIterator = trapezoid_iterator;

pub type GlyphinfoIterator = glyphinfo_iterator;

pub type QueryVersionCookie<'s> = base::Cookie<'s, query_version_cookie>;

/** Opcode for xcb_render_query_version. */
pub static XCB_RENDER_QUERY_VERSION : u8 = 0;
pub type QueryVersionReply = base::Reply<query_version_reply>;
pub type QueryPictFormatsCookie<'s> = base::Cookie<'s, query_pict_formats_cookie>;

/** Opcode for xcb_render_query_pict_formats. */
pub static XCB_RENDER_QUERY_PICT_FORMATS : u8 = 1;
pub type QueryPictIndexValuesCookie<'s> = base::Cookie<'s, query_pict_index_values_cookie>;

/** Opcode for xcb_render_query_pict_index_values. */
pub static XCB_RENDER_QUERY_PICT_INDEX_VALUES : u8 = 2;
/** Opcode for xcb_render_create_picture. */
pub static XCB_RENDER_CREATE_PICTURE : u8 = 4;
/** Opcode for xcb_render_change_picture. */
pub static XCB_RENDER_CHANGE_PICTURE : u8 = 5;
/** Opcode for xcb_render_set_picture_clip_rectangles. */
pub static XCB_RENDER_SET_PICTURE_CLIP_RECTANGLES : u8 = 6;
/** Opcode for xcb_render_free_picture. */
pub static XCB_RENDER_FREE_PICTURE : u8 = 7;
/** Opcode for xcb_render_composite. */
pub static XCB_RENDER_COMPOSITE : u8 = 8;
/** Opcode for xcb_render_trapezoids. */
pub static XCB_RENDER_TRAPEZOIDS : u8 = 10;
/** Opcode for xcb_render_triangles. */
pub static XCB_RENDER_TRIANGLES : u8 = 11;
/** Opcode for xcb_render_tri_strip. */
pub static XCB_RENDER_TRI_STRIP : u8 = 12;
/** Opcode for xcb_render_tri_fan. */
pub static XCB_RENDER_TRI_FAN : u8 = 13;
/** Opcode for xcb_render_create_glyph_set. */
pub static XCB_RENDER_CREATE_GLYPH_SET : u8 = 17;
/** Opcode for xcb_render_reference_glyph_set. */
pub static XCB_RENDER_REFERENCE_GLYPH_SET : u8 = 18;
/** Opcode for xcb_render_free_glyph_set. */
pub static XCB_RENDER_FREE_GLYPH_SET : u8 = 19;
/** Opcode for xcb_render_add_glyphs. */
pub static XCB_RENDER_ADD_GLYPHS : u8 = 20;
/** Opcode for xcb_render_free_glyphs. */
pub static XCB_RENDER_FREE_GLYPHS : u8 = 22;
/** Opcode for xcb_render_composite_glyphs_8. */
pub static XCB_RENDER_COMPOSITE_GLYPHS_8 : u8 = 23;
/** Opcode for xcb_render_composite_glyphs_16. */
pub static XCB_RENDER_COMPOSITE_GLYPHS_16 : u8 = 24;
/** Opcode for xcb_render_composite_glyphs_32. */
pub static XCB_RENDER_COMPOSITE_GLYPHS_32 : u8 = 25;
/** Opcode for xcb_render_fill_rectangles. */
pub static XCB_RENDER_FILL_RECTANGLES : u8 = 26;
/** Opcode for xcb_render_create_cursor. */
pub static XCB_RENDER_CREATE_CURSOR : u8 = 27;
pub type TransformIterator = transform_iterator;

/** Opcode for xcb_render_set_picture_transform. */
pub static XCB_RENDER_SET_PICTURE_TRANSFORM : u8 = 28;
pub type QueryFiltersCookie<'s> = base::Cookie<'s, query_filters_cookie>;

/** Opcode for xcb_render_query_filters. */
pub static XCB_RENDER_QUERY_FILTERS : u8 = 29;
/** Opcode for xcb_render_set_picture_filter. */
pub static XCB_RENDER_SET_PICTURE_FILTER : u8 = 30;
pub type AnimcursoreltIterator = animcursorelt_iterator;

/** Opcode for xcb_render_create_anim_cursor. */
pub static XCB_RENDER_CREATE_ANIM_CURSOR : u8 = 31;
pub type SpanfixIterator = spanfix_iterator;

pub type TrapIterator = trap_iterator;

/** Opcode for xcb_render_add_traps. */
pub static XCB_RENDER_ADD_TRAPS : u8 = 32;
/** Opcode for xcb_render_create_solid_fill. */
pub static XCB_RENDER_CREATE_SOLID_FILL : u8 = 33;
/** Opcode for xcb_render_create_linear_gradient. */
pub static XCB_RENDER_CREATE_LINEAR_GRADIENT : u8 = 34;
/** Opcode for xcb_render_create_radial_gradient. */
pub static XCB_RENDER_CREATE_RADIAL_GRADIENT : u8 = 35;
/** Opcode for xcb_render_create_conical_gradient. */
pub static XCB_RENDER_CREATE_CONICAL_GRADIENT : u8 = 36;

impl<'s, Glyph> Iterator<&'s Glyph> for GlyphIterator {
    pub fn next(&mut self) -> Option<&'s Glyph> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *mut glyph_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_render_glyph_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type Glyphset = glyphset;


impl<'s, Glyphset> Iterator<&'s Glyphset> for GlyphsetIterator {
    pub fn next(&mut self) -> Option<&'s Glyphset> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *mut glyphset_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_render_glyphset_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type Picture = picture;


impl<'s, Picture> Iterator<&'s Picture> for PictureIterator {
    pub fn next(&mut self) -> Option<&'s Picture> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *mut picture_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_render_picture_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type Pictformat = pictformat;


impl<'s, Pictformat> Iterator<&'s Pictformat> for PictformatIterator {
    pub fn next(&mut self) -> Option<&'s Pictformat> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *mut pictformat_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_render_pictformat_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type Fixed = fixed;


impl<'s, Fixed> Iterator<&'s Fixed> for FixedIterator {
    pub fn next(&mut self) -> Option<&'s Fixed> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *mut fixed_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_render_fixed_next(iter);
            Some(cast::transmute(data))
        }
    }
}


impl base::Struct<directformat> {
  pub fn red_shift(&self) -> u16 {
    unsafe { accessor!(red_shift -> u16, s.strct) }
  }

  pub fn red_mask(&self) -> u16 {
    unsafe { accessor!(red_mask -> u16, s.strct) }
  }

  pub fn green_shift(&self) -> u16 {
    unsafe { accessor!(green_shift -> u16, s.strct) }
  }

  pub fn green_mask(&self) -> u16 {
    unsafe { accessor!(green_mask -> u16, s.strct) }
  }

  pub fn blue_shift(&self) -> u16 {
    unsafe { accessor!(blue_shift -> u16, s.strct) }
  }

  pub fn blue_mask(&self) -> u16 {
    unsafe { accessor!(blue_mask -> u16, s.strct) }
  }

  pub fn alpha_shift(&self) -> u16 {
    unsafe { accessor!(alpha_shift -> u16, s.strct) }
  }

  pub fn alpha_mask(&self) -> u16 {
    unsafe { accessor!(alpha_mask -> u16, s.strct) }
  }

}

impl<'s, Directformat> Iterator<&'s Directformat> for DirectformatIterator {
    pub fn next(&mut self) -> Option<&'s Directformat> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *mut directformat_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_render_directformat_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type Pictforminfo = base::Struct<pictforminfo>;


impl base::Struct<pictforminfo> {
  pub fn id(&self) -> Pictformat {
    unsafe { accessor!(id -> Pictformat, s.strct) }
  }

  pub fn type_(&self) -> u8 {
    unsafe { accessor!(type_ -> u8, s.strct) }
  }

  pub fn depth(&self) -> u8 {
    unsafe { accessor!(depth -> u8, s.strct) }
  }

  pub fn direct(&self) -> Directformat {
    unsafe { cast::transmute(s.strct.direct) }
  }
  pub fn colormap(&self) -> xproto::Colormap {
    unsafe { accessor!(colormap -> xproto::Colormap, s.strct) }
  }

}

impl<'s, Pictforminfo> Iterator<&'s Pictforminfo> for PictforminfoIterator {
    pub fn next(&mut self) -> Option<&'s Pictforminfo> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *mut pictforminfo_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_render_pictforminfo_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type Pictvisual = base::Struct<pictvisual>;


impl base::Struct<pictvisual> {
  pub fn visual(&self) -> xproto::Visualid {
    unsafe { accessor!(visual -> xproto::Visualid, s.strct) }
  }

  pub fn format(&self) -> Pictformat {
    unsafe { accessor!(format -> Pictformat, s.strct) }
  }

}

impl<'s, Pictvisual> Iterator<&'s Pictvisual> for PictvisualIterator {
    pub fn next(&mut self) -> Option<&'s Pictvisual> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *mut pictvisual_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_render_pictvisual_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type Pictdepth = base::Struct<pictdepth>;


impl base::Struct<pictdepth> {
  pub fn depth(&self) -> u8 {
    unsafe { accessor!(depth -> u8, s.strct) }
  }

  pub fn visuals(&self) -> PictvisualIterator {
    unsafe { accessor!(PictvisualIterator, xcb_render_pictdepth_visuals_iterator, s.strct) }
  }

}

impl<'s, Pictdepth> Iterator<&'s Pictdepth> for PictdepthIterator {
    pub fn next(&mut self) -> Option<&'s Pictdepth> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *mut pictdepth_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_render_pictdepth_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type Pictscreen = base::Struct<pictscreen>;


impl base::Struct<pictscreen> {
  pub fn fallback(&self) -> Pictformat {
    unsafe { accessor!(fallback -> Pictformat, s.strct) }
  }

  pub fn depths(&self) -> PictdepthIterator {
    unsafe { accessor!(PictdepthIterator, xcb_render_pictscreen_depths_iterator, s.strct) }
  }

}

impl<'s, Pictscreen> Iterator<&'s Pictscreen> for PictscreenIterator {
    pub fn next(&mut self) -> Option<&'s Pictscreen> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *mut pictscreen_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_render_pictscreen_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type Indexvalue = base::Struct<indexvalue>;


impl base::Struct<indexvalue> {
  pub fn pixel(&self) -> u32 {
    unsafe { accessor!(pixel -> u32, s.strct) }
  }

  pub fn red(&self) -> u16 {
    unsafe { accessor!(red -> u16, s.strct) }
  }

  pub fn green(&self) -> u16 {
    unsafe { accessor!(green -> u16, s.strct) }
  }

  pub fn blue(&self) -> u16 {
    unsafe { accessor!(blue -> u16, s.strct) }
  }

  pub fn alpha(&self) -> u16 {
    unsafe { accessor!(alpha -> u16, s.strct) }
  }

}

impl<'s, Indexvalue> Iterator<&'s Indexvalue> for IndexvalueIterator {
    pub fn next(&mut self) -> Option<&'s Indexvalue> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *mut indexvalue_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_render_indexvalue_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type Color = base::Struct<color>;


impl base::Struct<color> {
  pub fn red(&self) -> u16 {
    unsafe { accessor!(red -> u16, s.strct) }
  }

  pub fn green(&self) -> u16 {
    unsafe { accessor!(green -> u16, s.strct) }
  }

  pub fn blue(&self) -> u16 {
    unsafe { accessor!(blue -> u16, s.strct) }
  }

  pub fn alpha(&self) -> u16 {
    unsafe { accessor!(alpha -> u16, s.strct) }
  }

}

impl<'s, Color> Iterator<&'s Color> for ColorIterator {
    pub fn next(&mut self) -> Option<&'s Color> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *mut color_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_render_color_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type Pointfix = base::Struct<pointfix>;


impl base::Struct<pointfix> {
  pub fn x(&self) -> Fixed {
    unsafe { accessor!(x -> Fixed, s.strct) }
  }

  pub fn y(&self) -> Fixed {
    unsafe { accessor!(y -> Fixed, s.strct) }
  }

}

impl<'s, Pointfix> Iterator<&'s Pointfix> for PointfixIterator {
    pub fn next(&mut self) -> Option<&'s Pointfix> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *mut pointfix_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_render_pointfix_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type Linefix = base::Struct<linefix>;


impl base::Struct<linefix> {
  pub fn p1(&self) -> Pointfix {
    unsafe { cast::transmute(s.strct.p1) }
  }
  pub fn p2(&self) -> Pointfix {
    unsafe { cast::transmute(s.strct.p2) }
  }
}

impl<'s, Linefix> Iterator<&'s Linefix> for LinefixIterator {
    pub fn next(&mut self) -> Option<&'s Linefix> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *mut linefix_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_render_linefix_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type Triangle = base::Struct<triangle>;


impl base::Struct<triangle> {
  pub fn p1(&self) -> Pointfix {
    unsafe { cast::transmute(s.strct.p1) }
  }
  pub fn p2(&self) -> Pointfix {
    unsafe { cast::transmute(s.strct.p2) }
  }
  pub fn p3(&self) -> Pointfix {
    unsafe { cast::transmute(s.strct.p3) }
  }
}

impl<'s, Triangle> Iterator<&'s Triangle> for TriangleIterator {
    pub fn next(&mut self) -> Option<&'s Triangle> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *mut triangle_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_render_triangle_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type Trapezoid = base::Struct<trapezoid>;


impl base::Struct<trapezoid> {
  pub fn top(&self) -> Fixed {
    unsafe { accessor!(top -> Fixed, s.strct) }
  }

  pub fn bottom(&self) -> Fixed {
    unsafe { accessor!(bottom -> Fixed, s.strct) }
  }

  pub fn left(&self) -> Linefix {
    unsafe { cast::transmute(s.strct.left) }
  }
  pub fn right(&self) -> Linefix {
    unsafe { cast::transmute(s.strct.right) }
  }
}

impl<'s, Trapezoid> Iterator<&'s Trapezoid> for TrapezoidIterator {
    pub fn next(&mut self) -> Option<&'s Trapezoid> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *mut trapezoid_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_render_trapezoid_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type Glyphinfo = base::Struct<glyphinfo>;


impl base::Struct<glyphinfo> {
  pub fn width(&self) -> u16 {
    unsafe { accessor!(width -> u16, s.strct) }
  }

  pub fn height(&self) -> u16 {
    unsafe { accessor!(height -> u16, s.strct) }
  }

  pub fn x(&self) -> i16 {
    unsafe { accessor!(x -> i16, s.strct) }
  }

  pub fn y(&self) -> i16 {
    unsafe { accessor!(y -> i16, s.strct) }
  }

  pub fn x_off(&self) -> i16 {
    unsafe { accessor!(x_off -> i16, s.strct) }
  }

  pub fn y_off(&self) -> i16 {
    unsafe { accessor!(y_off -> i16, s.strct) }
  }

}

impl<'s, Glyphinfo> Iterator<&'s Glyphinfo> for GlyphinfoIterator {
    pub fn next(&mut self) -> Option<&'s Glyphinfo> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *mut glyphinfo_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_render_glyphinfo_next(iter);
            Some(cast::transmute(data))
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
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn QueryVersionUnchecked<'r> (c : &'r Connection,
                              client_major_version : u32,
                              client_minor_version : u32) -> QueryVersionCookie<'r> {
  unsafe {
    let cookie = xcb_render_query_version_unchecked(c.get_raw_conn(),
        client_major_version as u32, //1
        client_minor_version as u32); //2
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}

impl base::Reply<query_version_reply> {
  pub fn major_version(&self) -> u32 {
    unsafe { accessor!(major_version -> u32, (*self.reply)) }
  }

  pub fn minor_version(&self) -> u32 {
    unsafe { accessor!(minor_version -> u32, (*self.reply)) }
  }

}
impl_reply_cookie!(QueryVersionCookie<'s>, query_version_reply, QueryVersionReply, xcb_render_query_version_reply)

pub type QueryPictFormatsReply = base::Reply<query_pict_formats_reply>;
pub fn QueryPictFormats<'r> (c : &'r Connection) -> QueryPictFormatsCookie<'r> {
  unsafe {
    let cookie = xcb_render_query_pict_formats(c.get_raw_conn());
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn QueryPictFormatsUnchecked<'r> (c : &'r Connection) -> QueryPictFormatsCookie<'r> {
  unsafe {
    let cookie = xcb_render_query_pict_formats_unchecked(c.get_raw_conn());
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}

impl base::Reply<query_pict_formats_reply> {
  pub fn num_depths(&self) -> u32 {
    unsafe { accessor!(num_depths -> u32, (*self.reply)) }
  }

  pub fn num_visuals(&self) -> u32 {
    unsafe { accessor!(num_visuals -> u32, (*self.reply)) }
  }

  pub fn formats(&self) -> PictforminfoIterator {
    unsafe { accessor!(PictforminfoIterator, xcb_render_query_pict_formats_formats_iterator, (*self.reply)) }
  }

  pub fn screens(&self) -> PictscreenIterator {
    unsafe { accessor!(PictscreenIterator, xcb_render_query_pict_formats_screens_iterator, (*self.reply)) }
  }

  pub fn subpixels(&self) -> Box<[u32]> {
    unsafe { accessor!(u32, xcb_render_query_pict_formats_subpixels_length, xcb_render_query_pict_formats_subpixels, (*self.reply)) }
  }

}
impl_reply_cookie!(QueryPictFormatsCookie<'s>, query_pict_formats_reply, QueryPictFormatsReply, xcb_render_query_pict_formats_reply)

pub type QueryPictIndexValuesReply = base::Reply<query_pict_index_values_reply>;
pub fn QueryPictIndexValues<'r> (c : &'r Connection,
                             format : Pictformat) -> QueryPictIndexValuesCookie<'r> {
  unsafe {
    let cookie = xcb_render_query_pict_index_values(c.get_raw_conn(),
        format as pictformat); //1
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn QueryPictIndexValuesUnchecked<'r> (c : &'r Connection,
                                      format : Pictformat) -> QueryPictIndexValuesCookie<'r> {
  unsafe {
    let cookie = xcb_render_query_pict_index_values_unchecked(c.get_raw_conn(),
        format as pictformat); //1
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}

impl base::Reply<query_pict_index_values_reply> {
  pub fn values(&self) -> IndexvalueIterator {
    unsafe { accessor!(IndexvalueIterator, xcb_render_query_pict_index_values_values_iterator, (*self.reply)) }
  }

}
impl_reply_cookie!(QueryPictIndexValuesCookie<'s>, query_pict_index_values_reply, QueryPictIndexValuesReply, xcb_render_query_pict_index_values_reply)

pub fn CreatePictureChecked<'r> (c : &'r Connection,
                             pid : Picture,
                             drawable : xproto::Drawable,
                             format : Pictformat,
                             value_list : &[(u32,u32)]) -> base::VoidCookie<'r> {
  unsafe {
    let (value_list_mask, value_list_vec) = pack_bitfield(value_list);
    let value_list_ptr = std::vec::raw::to_ptr(value_list_vec);
    let cookie = xcb_render_create_picture_checked(c.get_raw_conn(),
        pid as picture, //1
        drawable as ffi::xproto::drawable, //2
        format as pictformat, //3
        value_list_mask as u32, //4
        value_list_ptr as *mut u32); //5
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn CreatePicture<'r> (c : &'r Connection,
                      pid : Picture,
                      drawable : xproto::Drawable,
                      format : Pictformat,
                      value_list : &[(u32,u32)]) -> base::VoidCookie<'r> {
  unsafe {
    let (value_list_mask, value_list_vec) = pack_bitfield(value_list);
    let value_list_ptr = std::vec::raw::to_ptr(value_list_vec);
    let cookie = xcb_render_create_picture(c.get_raw_conn(),
        pid as picture, //1
        drawable as ffi::xproto::drawable, //2
        format as pictformat, //3
        value_list_mask as u32, //4
        value_list_ptr as *mut u32); //5
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn ChangePictureChecked<'r> (c : &'r Connection,
                             picture : Picture,
                             value_list : &[(u32,u32)]) -> base::VoidCookie<'r> {
  unsafe {
    let (value_list_mask, value_list_vec) = pack_bitfield(value_list);
    let value_list_ptr = std::vec::raw::to_ptr(value_list_vec);
    let cookie = xcb_render_change_picture_checked(c.get_raw_conn(),
        picture as picture, //1
        value_list_mask as u32, //2
        value_list_ptr as *mut u32); //3
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn ChangePicture<'r> (c : &'r Connection,
                      picture : Picture,
                      value_list : &[(u32,u32)]) -> base::VoidCookie<'r> {
  unsafe {
    let (value_list_mask, value_list_vec) = pack_bitfield(value_list);
    let value_list_ptr = std::vec::raw::to_ptr(value_list_vec);
    let cookie = xcb_render_change_picture(c.get_raw_conn(),
        picture as picture, //1
        value_list_mask as u32, //2
        value_list_ptr as *mut u32); //3
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn SetPictureClipRectanglesChecked<'r> (c : &'r Connection,
                                        picture : Picture,
                                        clip_x_origin : i16,
                                        clip_y_origin : i16,
                                        rectangles : &[xproto::Rectangle]) -> base::VoidCookie<'r> {
  unsafe {
    let rectangles_len = rectangles.len();
    let rectangles_ptr = std::vec::raw::to_ptr(rectangles);
    let cookie = xcb_render_set_picture_clip_rectangles_checked(c.get_raw_conn(),
        picture as picture, //1
        clip_x_origin as i16, //2
        clip_y_origin as i16, //3
        rectangles_len as u32, //4
        rectangles_ptr as *mut ffi::xproto::rectangle); //5
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn SetPictureClipRectangles<'r> (c : &'r Connection,
                                 picture : Picture,
                                 clip_x_origin : i16,
                                 clip_y_origin : i16,
                                 rectangles : &[xproto::Rectangle]) -> base::VoidCookie<'r> {
  unsafe {
    let rectangles_len = rectangles.len();
    let rectangles_ptr = std::vec::raw::to_ptr(rectangles);
    let cookie = xcb_render_set_picture_clip_rectangles(c.get_raw_conn(),
        picture as picture, //1
        clip_x_origin as i16, //2
        clip_y_origin as i16, //3
        rectangles_len as u32, //4
        rectangles_ptr as *mut ffi::xproto::rectangle); //5
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn FreePictureChecked<'r> (c : &'r Connection,
                           picture : Picture) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_render_free_picture_checked(c.get_raw_conn(),
        picture as picture); //1
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn FreePicture<'r> (c : &'r Connection,
                    picture : Picture) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_render_free_picture(c.get_raw_conn(),
        picture as picture); //1
    Cookie {cookie:cookie,conn:c,checked:false}
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
        src as picture, //2
        mask as picture, //3
        dst as picture, //4
        src_x as i16, //5
        src_y as i16, //6
        mask_x as i16, //7
        mask_y as i16, //8
        dst_x as i16, //9
        dst_y as i16, //10
        width as u16, //11
        height as u16); //12
    Cookie {cookie:cookie,conn:c,checked:true}
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
        src as picture, //2
        mask as picture, //3
        dst as picture, //4
        src_x as i16, //5
        src_y as i16, //6
        mask_x as i16, //7
        mask_y as i16, //8
        dst_x as i16, //9
        dst_y as i16, //10
        width as u16, //11
        height as u16); //12
    Cookie {cookie:cookie,conn:c,checked:false}
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
    let traps_ptr = std::vec::raw::to_ptr(traps);
    let cookie = xcb_render_trapezoids_checked(c.get_raw_conn(),
        op as u8, //1
        src as picture, //2
        dst as picture, //3
        mask_format as pictformat, //4
        src_x as i16, //5
        src_y as i16, //6
        traps_len as u32, //7
        traps_ptr as *mut trapezoid); //8
    Cookie {cookie:cookie,conn:c,checked:true}
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
    let traps_ptr = std::vec::raw::to_ptr(traps);
    let cookie = xcb_render_trapezoids(c.get_raw_conn(),
        op as u8, //1
        src as picture, //2
        dst as picture, //3
        mask_format as pictformat, //4
        src_x as i16, //5
        src_y as i16, //6
        traps_len as u32, //7
        traps_ptr as *mut trapezoid); //8
    Cookie {cookie:cookie,conn:c,checked:false}
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
    let triangles_ptr = std::vec::raw::to_ptr(triangles);
    let cookie = xcb_render_triangles_checked(c.get_raw_conn(),
        op as u8, //1
        src as picture, //2
        dst as picture, //3
        mask_format as pictformat, //4
        src_x as i16, //5
        src_y as i16, //6
        triangles_len as u32, //7
        triangles_ptr as *mut triangle); //8
    Cookie {cookie:cookie,conn:c,checked:true}
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
    let triangles_ptr = std::vec::raw::to_ptr(triangles);
    let cookie = xcb_render_triangles(c.get_raw_conn(),
        op as u8, //1
        src as picture, //2
        dst as picture, //3
        mask_format as pictformat, //4
        src_x as i16, //5
        src_y as i16, //6
        triangles_len as u32, //7
        triangles_ptr as *mut triangle); //8
    Cookie {cookie:cookie,conn:c,checked:false}
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
    let points_ptr = std::vec::raw::to_ptr(points);
    let cookie = xcb_render_tri_strip_checked(c.get_raw_conn(),
        op as u8, //1
        src as picture, //2
        dst as picture, //3
        mask_format as pictformat, //4
        src_x as i16, //5
        src_y as i16, //6
        points_len as u32, //7
        points_ptr as *mut pointfix); //8
    Cookie {cookie:cookie,conn:c,checked:true}
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
    let points_ptr = std::vec::raw::to_ptr(points);
    let cookie = xcb_render_tri_strip(c.get_raw_conn(),
        op as u8, //1
        src as picture, //2
        dst as picture, //3
        mask_format as pictformat, //4
        src_x as i16, //5
        src_y as i16, //6
        points_len as u32, //7
        points_ptr as *mut pointfix); //8
    Cookie {cookie:cookie,conn:c,checked:false}
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
    let points_ptr = std::vec::raw::to_ptr(points);
    let cookie = xcb_render_tri_fan_checked(c.get_raw_conn(),
        op as u8, //1
        src as picture, //2
        dst as picture, //3
        mask_format as pictformat, //4
        src_x as i16, //5
        src_y as i16, //6
        points_len as u32, //7
        points_ptr as *mut pointfix); //8
    Cookie {cookie:cookie,conn:c,checked:true}
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
    let points_ptr = std::vec::raw::to_ptr(points);
    let cookie = xcb_render_tri_fan(c.get_raw_conn(),
        op as u8, //1
        src as picture, //2
        dst as picture, //3
        mask_format as pictformat, //4
        src_x as i16, //5
        src_y as i16, //6
        points_len as u32, //7
        points_ptr as *mut pointfix); //8
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn CreateGlyphSetChecked<'r> (c : &'r Connection,
                              gsid : Glyphset,
                              format : Pictformat) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_render_create_glyph_set_checked(c.get_raw_conn(),
        gsid as glyphset, //1
        format as pictformat); //2
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn CreateGlyphSet<'r> (c : &'r Connection,
                       gsid : Glyphset,
                       format : Pictformat) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_render_create_glyph_set(c.get_raw_conn(),
        gsid as glyphset, //1
        format as pictformat); //2
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn ReferenceGlyphSetChecked<'r> (c : &'r Connection,
                                 gsid : Glyphset,
                                 existing : Glyphset) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_render_reference_glyph_set_checked(c.get_raw_conn(),
        gsid as glyphset, //1
        existing as glyphset); //2
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn ReferenceGlyphSet<'r> (c : &'r Connection,
                          gsid : Glyphset,
                          existing : Glyphset) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_render_reference_glyph_set(c.get_raw_conn(),
        gsid as glyphset, //1
        existing as glyphset); //2
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn FreeGlyphSetChecked<'r> (c : &'r Connection,
                            glyphset : Glyphset) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_render_free_glyph_set_checked(c.get_raw_conn(),
        glyphset as glyphset); //1
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn FreeGlyphSet<'r> (c : &'r Connection,
                     glyphset : Glyphset) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_render_free_glyph_set(c.get_raw_conn(),
        glyphset as glyphset); //1
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn AddGlyphsChecked<'r> (c : &'r Connection,
                         glyphset : Glyphset,
                         glyphids : &[u32],
                         glyphs : &[Glyphinfo],
                         data : &[u8]) -> base::VoidCookie<'r> {
  unsafe {
    let glyphids_len = glyphids.len();
    let glyphids_ptr = std::vec::raw::to_ptr(glyphids);
    let glyphs_ptr = std::vec::raw::to_ptr(glyphs);
    let data_len = data.len();
    let data_ptr = std::vec::raw::to_ptr(data);
    let cookie = xcb_render_add_glyphs_checked(c.get_raw_conn(),
        glyphset as glyphset, //1
        glyphids_len as u32, //2
        glyphids_ptr as *mut u32, //3
        glyphs_ptr as *mut glyphinfo, //4
        data_len as u32, //5
        data_ptr as *mut u8); //6
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn AddGlyphs<'r> (c : &'r Connection,
                  glyphset : Glyphset,
                  glyphids : &[u32],
                  glyphs : &[Glyphinfo],
                  data : &[u8]) -> base::VoidCookie<'r> {
  unsafe {
    let glyphids_len = glyphids.len();
    let glyphids_ptr = std::vec::raw::to_ptr(glyphids);
    let glyphs_ptr = std::vec::raw::to_ptr(glyphs);
    let data_len = data.len();
    let data_ptr = std::vec::raw::to_ptr(data);
    let cookie = xcb_render_add_glyphs(c.get_raw_conn(),
        glyphset as glyphset, //1
        glyphids_len as u32, //2
        glyphids_ptr as *mut u32, //3
        glyphs_ptr as *mut glyphinfo, //4
        data_len as u32, //5
        data_ptr as *mut u8); //6
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn FreeGlyphsChecked<'r> (c : &'r Connection,
                          glyphset : Glyphset,
                          glyphs : &[Glyph]) -> base::VoidCookie<'r> {
  unsafe {
    let glyphs_len = glyphs.len();
    let glyphs_ptr = std::vec::raw::to_ptr(glyphs);
    let cookie = xcb_render_free_glyphs_checked(c.get_raw_conn(),
        glyphset as glyphset, //1
        glyphs_len as u32, //2
        glyphs_ptr as *mut glyph); //3
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn FreeGlyphs<'r> (c : &'r Connection,
                   glyphset : Glyphset,
                   glyphs : &[Glyph]) -> base::VoidCookie<'r> {
  unsafe {
    let glyphs_len = glyphs.len();
    let glyphs_ptr = std::vec::raw::to_ptr(glyphs);
    let cookie = xcb_render_free_glyphs(c.get_raw_conn(),
        glyphset as glyphset, //1
        glyphs_len as u32, //2
        glyphs_ptr as *mut glyph); //3
    Cookie {cookie:cookie,conn:c,checked:false}
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
    let glyphcmds_ptr = std::vec::raw::to_ptr(glyphcmds);
    let cookie = xcb_render_composite_glyphs_8_checked(c.get_raw_conn(),
        op as u8, //1
        src as picture, //2
        dst as picture, //3
        mask_format as pictformat, //4
        glyphset as glyphset, //5
        src_x as i16, //6
        src_y as i16, //7
        glyphcmds_len as u32, //8
        glyphcmds_ptr as *mut u8); //9
    Cookie {cookie:cookie,conn:c,checked:true}
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
    let glyphcmds_ptr = std::vec::raw::to_ptr(glyphcmds);
    let cookie = xcb_render_composite_glyphs_8(c.get_raw_conn(),
        op as u8, //1
        src as picture, //2
        dst as picture, //3
        mask_format as pictformat, //4
        glyphset as glyphset, //5
        src_x as i16, //6
        src_y as i16, //7
        glyphcmds_len as u32, //8
        glyphcmds_ptr as *mut u8); //9
    Cookie {cookie:cookie,conn:c,checked:false}
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
    let glyphcmds_ptr = std::vec::raw::to_ptr(glyphcmds);
    let cookie = xcb_render_composite_glyphs_16_checked(c.get_raw_conn(),
        op as u8, //1
        src as picture, //2
        dst as picture, //3
        mask_format as pictformat, //4
        glyphset as glyphset, //5
        src_x as i16, //6
        src_y as i16, //7
        glyphcmds_len as u32, //8
        glyphcmds_ptr as *mut u8); //9
    Cookie {cookie:cookie,conn:c,checked:true}
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
    let glyphcmds_ptr = std::vec::raw::to_ptr(glyphcmds);
    let cookie = xcb_render_composite_glyphs_16(c.get_raw_conn(),
        op as u8, //1
        src as picture, //2
        dst as picture, //3
        mask_format as pictformat, //4
        glyphset as glyphset, //5
        src_x as i16, //6
        src_y as i16, //7
        glyphcmds_len as u32, //8
        glyphcmds_ptr as *mut u8); //9
    Cookie {cookie:cookie,conn:c,checked:false}
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
    let glyphcmds_ptr = std::vec::raw::to_ptr(glyphcmds);
    let cookie = xcb_render_composite_glyphs_32_checked(c.get_raw_conn(),
        op as u8, //1
        src as picture, //2
        dst as picture, //3
        mask_format as pictformat, //4
        glyphset as glyphset, //5
        src_x as i16, //6
        src_y as i16, //7
        glyphcmds_len as u32, //8
        glyphcmds_ptr as *mut u8); //9
    Cookie {cookie:cookie,conn:c,checked:true}
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
    let glyphcmds_ptr = std::vec::raw::to_ptr(glyphcmds);
    let cookie = xcb_render_composite_glyphs_32(c.get_raw_conn(),
        op as u8, //1
        src as picture, //2
        dst as picture, //3
        mask_format as pictformat, //4
        glyphset as glyphset, //5
        src_x as i16, //6
        src_y as i16, //7
        glyphcmds_len as u32, //8
        glyphcmds_ptr as *mut u8); //9
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn FillRectanglesChecked<'r> (c : &'r Connection,
                              op : u8,
                              dst : Picture,
                              color : Color,
                              rects : &[xproto::Rectangle]) -> base::VoidCookie<'r> {
  unsafe {
    let rects_len = rects.len();
    let rects_ptr = std::vec::raw::to_ptr(rects);
    let cookie = xcb_render_fill_rectangles_checked(c.get_raw_conn(),
        op as u8, //1
        dst as picture, //2
        color.strct, //3
        rects_len as u32, //4
        rects_ptr as *mut ffi::xproto::rectangle); //5
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn FillRectangles<'r> (c : &'r Connection,
                       op : u8,
                       dst : Picture,
                       color : Color,
                       rects : &[xproto::Rectangle]) -> base::VoidCookie<'r> {
  unsafe {
    let rects_len = rects.len();
    let rects_ptr = std::vec::raw::to_ptr(rects);
    let cookie = xcb_render_fill_rectangles(c.get_raw_conn(),
        op as u8, //1
        dst as picture, //2
        color.strct, //3
        rects_len as u32, //4
        rects_ptr as *mut ffi::xproto::rectangle); //5
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn CreateCursorChecked<'r> (c : &'r Connection,
                            cid : xproto::Cursor,
                            source : Picture,
                            x : u16,
                            y : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_render_create_cursor_checked(c.get_raw_conn(),
        cid as ffi::xproto::cursor, //1
        source as picture, //2
        x as u16, //3
        y as u16); //4
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn CreateCursor<'r> (c : &'r Connection,
                     cid : xproto::Cursor,
                     source : Picture,
                     x : u16,
                     y : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_render_create_cursor(c.get_raw_conn(),
        cid as ffi::xproto::cursor, //1
        source as picture, //2
        x as u16, //3
        y as u16); //4
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub type Transform = base::Struct<transform>;


impl base::Struct<transform> {
  pub fn matrix11(&self) -> Fixed {
    unsafe { accessor!(matrix11 -> Fixed, s.strct) }
  }

  pub fn matrix12(&self) -> Fixed {
    unsafe { accessor!(matrix12 -> Fixed, s.strct) }
  }

  pub fn matrix13(&self) -> Fixed {
    unsafe { accessor!(matrix13 -> Fixed, s.strct) }
  }

  pub fn matrix21(&self) -> Fixed {
    unsafe { accessor!(matrix21 -> Fixed, s.strct) }
  }

  pub fn matrix22(&self) -> Fixed {
    unsafe { accessor!(matrix22 -> Fixed, s.strct) }
  }

  pub fn matrix23(&self) -> Fixed {
    unsafe { accessor!(matrix23 -> Fixed, s.strct) }
  }

  pub fn matrix31(&self) -> Fixed {
    unsafe { accessor!(matrix31 -> Fixed, s.strct) }
  }

  pub fn matrix32(&self) -> Fixed {
    unsafe { accessor!(matrix32 -> Fixed, s.strct) }
  }

  pub fn matrix33(&self) -> Fixed {
    unsafe { accessor!(matrix33 -> Fixed, s.strct) }
  }

}

impl<'s, Transform> Iterator<&'s Transform> for TransformIterator {
    pub fn next(&mut self) -> Option<&'s Transform> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *mut transform_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_render_transform_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub fn SetPictureTransformChecked<'r> (c : &'r Connection,
                                   picture : Picture,
                                   transform : Transform) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_render_set_picture_transform_checked(c.get_raw_conn(),
        picture as picture, //1
        transform.strct); //2
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn SetPictureTransform<'r> (c : &'r Connection,
                            picture : Picture,
                            transform : Transform) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_render_set_picture_transform(c.get_raw_conn(),
        picture as picture, //1
        transform.strct); //2
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub type QueryFiltersReply = base::Reply<query_filters_reply>;
pub fn QueryFilters<'r> (c : &'r Connection,
                     drawable : xproto::Drawable) -> QueryFiltersCookie<'r> {
  unsafe {
    let cookie = xcb_render_query_filters(c.get_raw_conn(),
        drawable as ffi::xproto::drawable); //1
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn QueryFiltersUnchecked<'r> (c : &'r Connection,
                              drawable : xproto::Drawable) -> QueryFiltersCookie<'r> {
  unsafe {
    let cookie = xcb_render_query_filters_unchecked(c.get_raw_conn(),
        drawable as ffi::xproto::drawable); //1
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}

impl base::Reply<query_filters_reply> {
  pub fn aliases(&self) -> Box<[u16]> {
    unsafe { accessor!(u16, xcb_render_query_filters_aliases_length, xcb_render_query_filters_aliases, (*self.reply)) }
  }

  pub fn filters(&self) -> xproto::StrIterator {
    unsafe { accessor!(xproto::StrIterator, xcb_render_query_filters_filters_iterator, (*self.reply)) }
  }

}
impl_reply_cookie!(QueryFiltersCookie<'s>, query_filters_reply, QueryFiltersReply, xcb_render_query_filters_reply)

pub fn SetPictureFilterChecked<'r> (c : &'r Connection,
                                picture : Picture,
                                filter : &str,
                                values : &[Fixed]) -> base::VoidCookie<'r> {
  unsafe {
    let filter = (filter).to_bytes(false);
    let filter_len = filter.len();
    let filter_ptr = std::vec::raw::to_ptr(filter);
    let values_len = values.len();
    let values_ptr = std::vec::raw::to_ptr(values);
    let cookie = xcb_render_set_picture_filter_checked(c.get_raw_conn(),
        picture as picture, //1
        filter_len as u16, //2
        filter_ptr as *mut c_char, //3
        values_len as u32, //4
        values_ptr as *mut fixed); //5
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn SetPictureFilter<'r> (c : &'r Connection,
                         picture : Picture,
                         filter : &str,
                         values : &[Fixed]) -> base::VoidCookie<'r> {
  unsafe {
    let filter = (filter).to_bytes(false);
    let filter_len = filter.len();
    let filter_ptr = std::vec::raw::to_ptr(filter);
    let values_len = values.len();
    let values_ptr = std::vec::raw::to_ptr(values);
    let cookie = xcb_render_set_picture_filter(c.get_raw_conn(),
        picture as picture, //1
        filter_len as u16, //2
        filter_ptr as *mut c_char, //3
        values_len as u32, //4
        values_ptr as *mut fixed); //5
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub type Animcursorelt = base::Struct<animcursorelt>;


impl base::Struct<animcursorelt> {
  pub fn cursor(&self) -> xproto::Cursor {
    unsafe { accessor!(cursor -> xproto::Cursor, s.strct) }
  }

  pub fn delay(&self) -> u32 {
    unsafe { accessor!(delay -> u32, s.strct) }
  }

}

impl<'s, Animcursorelt> Iterator<&'s Animcursorelt> for AnimcursoreltIterator {
    pub fn next(&mut self) -> Option<&'s Animcursorelt> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *mut animcursorelt_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_render_animcursorelt_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub fn CreateAnimCursorChecked<'r> (c : &'r Connection,
                                cid : xproto::Cursor,
                                cursors : &[Animcursorelt]) -> base::VoidCookie<'r> {
  unsafe {
    let cursors_len = cursors.len();
    let cursors_ptr = std::vec::raw::to_ptr(cursors);
    let cookie = xcb_render_create_anim_cursor_checked(c.get_raw_conn(),
        cid as ffi::xproto::cursor, //1
        cursors_len as u32, //2
        cursors_ptr as *mut animcursorelt); //3
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn CreateAnimCursor<'r> (c : &'r Connection,
                         cid : xproto::Cursor,
                         cursors : &[Animcursorelt]) -> base::VoidCookie<'r> {
  unsafe {
    let cursors_len = cursors.len();
    let cursors_ptr = std::vec::raw::to_ptr(cursors);
    let cookie = xcb_render_create_anim_cursor(c.get_raw_conn(),
        cid as ffi::xproto::cursor, //1
        cursors_len as u32, //2
        cursors_ptr as *mut animcursorelt); //3
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub type Spanfix = base::Struct<spanfix>;


impl base::Struct<spanfix> {
  pub fn l(&self) -> Fixed {
    unsafe { accessor!(l -> Fixed, s.strct) }
  }

  pub fn r(&self) -> Fixed {
    unsafe { accessor!(r -> Fixed, s.strct) }
  }

  pub fn y(&self) -> Fixed {
    unsafe { accessor!(y -> Fixed, s.strct) }
  }

}

impl<'s, Spanfix> Iterator<&'s Spanfix> for SpanfixIterator {
    pub fn next(&mut self) -> Option<&'s Spanfix> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *mut spanfix_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_render_spanfix_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type Trap = base::Struct<trap>;


impl base::Struct<trap> {
  pub fn top(&self) -> Spanfix {
    unsafe { cast::transmute(s.strct.top) }
  }
  pub fn bot(&self) -> Spanfix {
    unsafe { cast::transmute(s.strct.bot) }
  }
}

impl<'s, Trap> Iterator<&'s Trap> for TrapIterator {
    pub fn next(&mut self) -> Option<&'s Trap> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *mut trap_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_render_trap_next(iter);
            Some(cast::transmute(data))
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
    let traps_ptr = std::vec::raw::to_ptr(traps);
    let cookie = xcb_render_add_traps_checked(c.get_raw_conn(),
        picture as picture, //1
        x_off as i16, //2
        y_off as i16, //3
        traps_len as u32, //4
        traps_ptr as *mut trap); //5
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn AddTraps<'r> (c : &'r Connection,
                 picture : Picture,
                 x_off : i16,
                 y_off : i16,
                 traps : &[Trap]) -> base::VoidCookie<'r> {
  unsafe {
    let traps_len = traps.len();
    let traps_ptr = std::vec::raw::to_ptr(traps);
    let cookie = xcb_render_add_traps(c.get_raw_conn(),
        picture as picture, //1
        x_off as i16, //2
        y_off as i16, //3
        traps_len as u32, //4
        traps_ptr as *mut trap); //5
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn CreateSolidFillChecked<'r> (c : &'r Connection,
                               picture : Picture,
                               color : Color) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_render_create_solid_fill_checked(c.get_raw_conn(),
        picture as picture, //1
        color.strct); //2
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn CreateSolidFill<'r> (c : &'r Connection,
                        picture : Picture,
                        color : Color) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_render_create_solid_fill(c.get_raw_conn(),
        picture as picture, //1
        color.strct); //2
    Cookie {cookie:cookie,conn:c,checked:false}
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
    let stops_ptr = std::vec::raw::to_ptr(stops);
    let colors_ptr = std::vec::raw::to_ptr(colors);
    let cookie = xcb_render_create_linear_gradient_checked(c.get_raw_conn(),
        picture as picture, //1
        p1.strct, //2
        p2.strct, //3
        stops_len as u32, //4
        stops_ptr as *mut fixed, //5
        colors_ptr as *mut color); //6
    Cookie {cookie:cookie,conn:c,checked:true}
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
    let stops_ptr = std::vec::raw::to_ptr(stops);
    let colors_ptr = std::vec::raw::to_ptr(colors);
    let cookie = xcb_render_create_linear_gradient(c.get_raw_conn(),
        picture as picture, //1
        p1.strct, //2
        p2.strct, //3
        stops_len as u32, //4
        stops_ptr as *mut fixed, //5
        colors_ptr as *mut color); //6
    Cookie {cookie:cookie,conn:c,checked:false}
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
    let stops_ptr = std::vec::raw::to_ptr(stops);
    let colors_ptr = std::vec::raw::to_ptr(colors);
    let cookie = xcb_render_create_radial_gradient_checked(c.get_raw_conn(),
        picture as picture, //1
        inner.strct, //2
        outer.strct, //3
        inner_radius as fixed, //4
        outer_radius as fixed, //5
        stops_len as u32, //6
        stops_ptr as *mut fixed, //7
        colors_ptr as *mut color); //8
    Cookie {cookie:cookie,conn:c,checked:true}
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
    let stops_ptr = std::vec::raw::to_ptr(stops);
    let colors_ptr = std::vec::raw::to_ptr(colors);
    let cookie = xcb_render_create_radial_gradient(c.get_raw_conn(),
        picture as picture, //1
        inner.strct, //2
        outer.strct, //3
        inner_radius as fixed, //4
        outer_radius as fixed, //5
        stops_len as u32, //6
        stops_ptr as *mut fixed, //7
        colors_ptr as *mut color); //8
    Cookie {cookie:cookie,conn:c,checked:false}
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
    let stops_ptr = std::vec::raw::to_ptr(stops);
    let colors_ptr = std::vec::raw::to_ptr(colors);
    let cookie = xcb_render_create_conical_gradient_checked(c.get_raw_conn(),
        picture as picture, //1
        center.strct, //2
        angle as fixed, //3
        stops_len as u32, //4
        stops_ptr as *mut fixed, //5
        colors_ptr as *mut color); //6
    Cookie {cookie:cookie,conn:c,checked:true}
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
    let stops_ptr = std::vec::raw::to_ptr(stops);
    let colors_ptr = std::vec::raw::to_ptr(colors);
    let cookie = xcb_render_create_conical_gradient(c.get_raw_conn(),
        picture as picture, //1
        center.strct, //2
        angle as fixed, //3
        stops_len as u32, //4
        stops_ptr as *mut fixed, //5
        colors_ptr as *mut color); //6
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}

