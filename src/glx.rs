/*
 * This file generated automatically from glx.xml by r_client.py.
 * Edit at your peril.
 */

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
use ffi::glx::*;
use std::option::Option;
use std::iter::Iterator;

use xproto;
pub type Pixmap = pixmap;

pub type PixmapIterator = pixmap_iterator;

pub type ContextIterator = context_iterator;

pub type PbufferIterator = pbuffer_iterator;

pub type WindowIterator = window_iterator;

pub type FbconfigIterator = fbconfig_iterator;

pub type DrawableIterator = drawable_iterator;

pub type Float32Iterator = float32_iterator;

pub type Float64Iterator = float64_iterator;

pub type Bool32Iterator = bool32_iterator;

pub type ContextTagIterator = context_tag_iterator;

/** Opcode for xcb_glx_generic. */
pub static XCB_GLX_GENERIC : u8 = -1;
pub struct GenericError { pub base : base::Error<generic_error> }
/** Opcode for xcb_glx_bad_context. */
pub static XCB_GLX_BAD_CONTEXT : u8 = 0;
pub struct BadContextError { pub base : base::Error<bad_context_error> }
/** Opcode for xcb_glx_bad_context_state. */
pub static XCB_GLX_BAD_CONTEXT_STATE : u8 = 1;
pub struct BadContextStateError { pub base : base::Error<bad_context_state_error> }
/** Opcode for xcb_glx_bad_drawable. */
pub static XCB_GLX_BAD_DRAWABLE : u8 = 2;
pub struct BadDrawableError { pub base : base::Error<bad_drawable_error> }
/** Opcode for xcb_glx_bad_pixmap. */
pub static XCB_GLX_BAD_PIXMAP : u8 = 3;
pub struct BadPixmapError { pub base : base::Error<bad_pixmap_error> }
/** Opcode for xcb_glx_bad_context_tag. */
pub static XCB_GLX_BAD_CONTEXT_TAG : u8 = 4;
pub struct BadContextTagError { pub base : base::Error<bad_context_tag_error> }
/** Opcode for xcb_glx_bad_current_window. */
pub static XCB_GLX_BAD_CURRENT_WINDOW : u8 = 5;
pub struct BadCurrentWindowError { pub base : base::Error<bad_current_window_error> }
/** Opcode for xcb_glx_bad_render_request. */
pub static XCB_GLX_BAD_RENDER_REQUEST : u8 = 6;
pub struct BadRenderRequestError { pub base : base::Error<bad_render_request_error> }
/** Opcode for xcb_glx_bad_large_request. */
pub static XCB_GLX_BAD_LARGE_REQUEST : u8 = 7;
pub struct BadLargeRequestError { pub base : base::Error<bad_large_request_error> }
/** Opcode for xcb_glx_unsupported_private_request. */
pub static XCB_GLX_UNSUPPORTED_PRIVATE_REQUEST : u8 = 8;
pub struct UnsupportedPrivateRequestError { pub base : base::Error<unsupported_private_request_error> }
/** Opcode for xcb_glx_bad_fb_config. */
pub static XCB_GLX_BAD_FB_CONFIG : u8 = 9;
pub struct BadFbConfigError { pub base : base::Error<bad_fb_config_error> }
/** Opcode for xcb_glx_bad_pbuffer. */
pub static XCB_GLX_BAD_PBUFFER : u8 = 10;
pub struct BadPbufferError { pub base : base::Error<bad_pbuffer_error> }
/** Opcode for xcb_glx_bad_current_drawable. */
pub static XCB_GLX_BAD_CURRENT_DRAWABLE : u8 = 11;
pub struct BadCurrentDrawableError { pub base : base::Error<bad_current_drawable_error> }
/** Opcode for xcb_glx_bad_window. */
pub static XCB_GLX_BAD_WINDOW : u8 = 12;
pub struct BadWindowError { pub base : base::Error<bad_window_error> }
/** Opcode for xcb_glx_glx_bad_profile_arb. */
pub static XCB_GLX_GLX_BAD_PROFILE_ARB : u8 = 13;
pub struct GlxBadProfileArbError { pub base : base::Error<glx_bad_profile_arb_error> }
/** Opcode for xcb_glx_pbuffer_clobber. */
pub static XCB_GLX_PBUFFER_CLOBBER : u8 = 0;
pub struct PbufferClobberEvent {pub base : base::Event<pbuffer_clobber_event>}

pub type pbcet = c_uint;//{
    pub static XCB_GLX_PBCET_DAMAGED : pbcet = 32791;
    pub static XCB_GLX_PBCET_SAVED : pbcet = 32792;
//}

pub type pbcdt = c_uint;//{
    pub static XCB_GLX_PBCDT_WINDOW : pbcdt = 32793;
    pub static XCB_GLX_PBCDT_PBUFFER : pbcdt = 32794;
//}
/** Opcode for xcb_glx_render. */
pub static XCB_GLX_RENDER : u8 = 1;
/** Opcode for xcb_glx_render_large. */
pub static XCB_GLX_RENDER_LARGE : u8 = 2;
/** Opcode for xcb_glx_create_context. */
pub static XCB_GLX_CREATE_CONTEXT : u8 = 3;
/** Opcode for xcb_glx_destroy_context. */
pub static XCB_GLX_DESTROY_CONTEXT : u8 = 4;
pub struct  MakeCurrentCookie<'s> { pub base : base::Cookie<'s, make_current_cookie> }

/** Opcode for xcb_glx_make_current. */
pub static XCB_GLX_MAKE_CURRENT : u8 = 5;
pub struct MakeCurrentReply { base:  base::Reply<make_current_reply> }
fn mk_reply_make_current_reply(reply:*mut make_current_reply) -> MakeCurrentReply { MakeCurrentReply { base : base::mk_reply(reply) } }
pub struct  IsDirectCookie<'s> { pub base : base::Cookie<'s, is_direct_cookie> }

/** Opcode for xcb_glx_is_direct. */
pub static XCB_GLX_IS_DIRECT : u8 = 6;
pub struct IsDirectReply { base:  base::Reply<is_direct_reply> }
fn mk_reply_is_direct_reply(reply:*mut is_direct_reply) -> IsDirectReply { IsDirectReply { base : base::mk_reply(reply) } }
pub struct  QueryVersionCookie<'s> { pub base : base::Cookie<'s, query_version_cookie> }

/** Opcode for xcb_glx_query_version. */
pub static XCB_GLX_QUERY_VERSION : u8 = 7;
pub struct QueryVersionReply { base:  base::Reply<query_version_reply> }
fn mk_reply_query_version_reply(reply:*mut query_version_reply) -> QueryVersionReply { QueryVersionReply { base : base::mk_reply(reply) } }
/** Opcode for xcb_glx_wait_gl. */
pub static XCB_GLX_WAIT_GL : u8 = 8;
/** Opcode for xcb_glx_wait_x. */
pub static XCB_GLX_WAIT_X : u8 = 9;
/** Opcode for xcb_glx_copy_context. */
pub static XCB_GLX_COPY_CONTEXT : u8 = 10;

pub type gc = c_uint;//{
    pub static XCB_GLX_GC_GL_CURRENT_BIT : gc = 1;
    pub static XCB_GLX_GC_GL_POINT_BIT : gc = 2;
    pub static XCB_GLX_GC_GL_LINE_BIT : gc = 4;
    pub static XCB_GLX_GC_GL_POLYGON_BIT : gc = 8;
    pub static XCB_GLX_GC_GL_POLYGON_STIPPLE_BIT : gc = 16;
    pub static XCB_GLX_GC_GL_PIXEL_MODE_BIT : gc = 32;
    pub static XCB_GLX_GC_GL_LIGHTING_BIT : gc = 64;
    pub static XCB_GLX_GC_GL_FOG_BIT : gc = 128;
    pub static XCB_GLX_GC_GL_DEPTH_BUFFER_BIT : gc = 256;
    pub static XCB_GLX_GC_GL_ACCUM_BUFFER_BIT : gc = 512;
    pub static XCB_GLX_GC_GL_STENCIL_BUFFER_BIT : gc = 1024;
    pub static XCB_GLX_GC_GL_VIEWPORT_BIT : gc = 2048;
    pub static XCB_GLX_GC_GL_TRANSFORM_BIT : gc = 4096;
    pub static XCB_GLX_GC_GL_ENABLE_BIT : gc = 8192;
    pub static XCB_GLX_GC_GL_COLOR_BUFFER_BIT : gc = 16384;
    pub static XCB_GLX_GC_GL_HINT_BIT : gc = 32768;
    pub static XCB_GLX_GC_GL_EVAL_BIT : gc = 65536;
    pub static XCB_GLX_GC_GL_LIST_BIT : gc = 131072;
    pub static XCB_GLX_GC_GL_TEXTURE_BIT : gc = 262144;
    pub static XCB_GLX_GC_GL_SCISSOR_BIT : gc = 524288;
    pub static XCB_GLX_GC_GL_ALL_ATTRIB_BITS : gc = 16777215;
//}
/** Opcode for xcb_glx_swap_buffers. */
pub static XCB_GLX_SWAP_BUFFERS : u8 = 11;
/** Opcode for xcb_glx_use_x_font. */
pub static XCB_GLX_USE_X_FONT : u8 = 12;
/** Opcode for xcb_glx_create_glx_pixmap. */
pub static XCB_GLX_CREATE_GLX_PIXMAP : u8 = 13;
pub struct  GetVisualConfigsCookie<'s> { pub base : base::Cookie<'s, get_visual_configs_cookie> }

/** Opcode for xcb_glx_get_visual_configs. */
pub static XCB_GLX_GET_VISUAL_CONFIGS : u8 = 14;
/** Opcode for xcb_glx_destroy_glx_pixmap. */
pub static XCB_GLX_DESTROY_GLX_PIXMAP : u8 = 15;
/** Opcode for xcb_glx_vendor_private. */
pub static XCB_GLX_VENDOR_PRIVATE : u8 = 16;
pub struct  VendorPrivateWithReplyCookie<'s> { pub base : base::Cookie<'s, vendor_private_with_reply_cookie> }

/** Opcode for xcb_glx_vendor_private_with_reply. */
pub static XCB_GLX_VENDOR_PRIVATE_WITH_REPLY : u8 = 17;
pub struct VendorPrivateWithReplyReply { base:  base::Reply<vendor_private_with_reply_reply> }
fn mk_reply_vendor_private_with_reply_reply(reply:*mut vendor_private_with_reply_reply) -> VendorPrivateWithReplyReply { VendorPrivateWithReplyReply { base : base::mk_reply(reply) } }
pub struct  QueryExtensionsStringCookie<'s> { pub base : base::Cookie<'s, query_extensions_string_cookie> }

/** Opcode for xcb_glx_query_extensions_string. */
pub static XCB_GLX_QUERY_EXTENSIONS_STRING : u8 = 18;
pub struct QueryExtensionsStringReply { base:  base::Reply<query_extensions_string_reply> }
fn mk_reply_query_extensions_string_reply(reply:*mut query_extensions_string_reply) -> QueryExtensionsStringReply { QueryExtensionsStringReply { base : base::mk_reply(reply) } }
pub struct  QueryServerStringCookie<'s> { pub base : base::Cookie<'s, query_server_string_cookie> }

/** Opcode for xcb_glx_query_server_string. */
pub static XCB_GLX_QUERY_SERVER_STRING : u8 = 19;
/** Opcode for xcb_glx_client_info. */
pub static XCB_GLX_CLIENT_INFO : u8 = 20;
pub struct  GetFbConfigsCookie<'s> { pub base : base::Cookie<'s, get_fb_configs_cookie> }

/** Opcode for xcb_glx_get_fb_configs. */
pub static XCB_GLX_GET_FB_CONFIGS : u8 = 21;
/** Opcode for xcb_glx_create_pixmap. */
pub static XCB_GLX_CREATE_PIXMAP : u8 = 22;
/** Opcode for xcb_glx_destroy_pixmap. */
pub static XCB_GLX_DESTROY_PIXMAP : u8 = 23;
/** Opcode for xcb_glx_create_new_context. */
pub static XCB_GLX_CREATE_NEW_CONTEXT : u8 = 24;
pub struct  QueryContextCookie<'s> { pub base : base::Cookie<'s, query_context_cookie> }

/** Opcode for xcb_glx_query_context. */
pub static XCB_GLX_QUERY_CONTEXT : u8 = 25;
pub struct  MakeContextCurrentCookie<'s> { pub base : base::Cookie<'s, make_context_current_cookie> }

/** Opcode for xcb_glx_make_context_current. */
pub static XCB_GLX_MAKE_CONTEXT_CURRENT : u8 = 26;
pub struct MakeContextCurrentReply { base:  base::Reply<make_context_current_reply> }
fn mk_reply_make_context_current_reply(reply:*mut make_context_current_reply) -> MakeContextCurrentReply { MakeContextCurrentReply { base : base::mk_reply(reply) } }
/** Opcode for xcb_glx_create_pbuffer. */
pub static XCB_GLX_CREATE_PBUFFER : u8 = 27;
/** Opcode for xcb_glx_destroy_pbuffer. */
pub static XCB_GLX_DESTROY_PBUFFER : u8 = 28;
pub struct  GetDrawableAttributesCookie<'s> { pub base : base::Cookie<'s, get_drawable_attributes_cookie> }

/** Opcode for xcb_glx_get_drawable_attributes. */
pub static XCB_GLX_GET_DRAWABLE_ATTRIBUTES : u8 = 29;
/** Opcode for xcb_glx_change_drawable_attributes. */
pub static XCB_GLX_CHANGE_DRAWABLE_ATTRIBUTES : u8 = 30;
/** Opcode for xcb_glx_create_window. */
pub static XCB_GLX_CREATE_WINDOW : u8 = 31;
/** Opcode for xcb_glx_delete_window. */
pub static XCB_GLX_DELETE_WINDOW : u8 = 32;
/** Opcode for xcb_glx_set_client_info_arb. */
pub static XCB_GLX_SET_CLIENT_INFO_ARB : u8 = 33;
/** Opcode for xcb_glx_create_context_attribs_arb. */
pub static XCB_GLX_CREATE_CONTEXT_ATTRIBS_ARB : u8 = 34;
/** Opcode for xcb_glx_set_client_info_2arb. */
pub static XCB_GLX_SET_CLIENT_INFO_2ARB : u8 = 35;
/** Opcode for xcb_glx_new_list. */
pub static XCB_GLX_NEW_LIST : u8 = 101;
/** Opcode for xcb_glx_end_list. */
pub static XCB_GLX_END_LIST : u8 = 102;
/** Opcode for xcb_glx_delete_lists. */
pub static XCB_GLX_DELETE_LISTS : u8 = 103;
pub struct  GenListsCookie<'s> { pub base : base::Cookie<'s, gen_lists_cookie> }

/** Opcode for xcb_glx_gen_lists. */
pub static XCB_GLX_GEN_LISTS : u8 = 104;
pub struct GenListsReply { base:  base::Reply<gen_lists_reply> }
fn mk_reply_gen_lists_reply(reply:*mut gen_lists_reply) -> GenListsReply { GenListsReply { base : base::mk_reply(reply) } }
/** Opcode for xcb_glx_feedback_buffer. */
pub static XCB_GLX_FEEDBACK_BUFFER : u8 = 105;
/** Opcode for xcb_glx_select_buffer. */
pub static XCB_GLX_SELECT_BUFFER : u8 = 106;
pub struct  RenderModeCookie<'s> { pub base : base::Cookie<'s, render_mode_cookie> }

/** Opcode for xcb_glx_render_mode. */
pub static XCB_GLX_RENDER_MODE : u8 = 107;

pub type rm = c_uint;//{
    pub static XCB_GLX_RM_GL_RENDER : rm = 7168;
    pub static XCB_GLX_RM_GL_FEEDBACK : rm = 7169;
    pub static XCB_GLX_RM_GL_SELECT : rm = 7170;
//}
pub struct  FinishCookie<'s> { pub base : base::Cookie<'s, finish_cookie> }

/** Opcode for xcb_glx_finish. */
pub static XCB_GLX_FINISH : u8 = 108;
pub struct FinishReply { base:  base::Reply<finish_reply> }
fn mk_reply_finish_reply(reply:*mut finish_reply) -> FinishReply { FinishReply { base : base::mk_reply(reply) } }
/** Opcode for xcb_glx_pixel_storef. */
pub static XCB_GLX_PIXEL_STOREF : u8 = 109;
/** Opcode for xcb_glx_pixel_storei. */
pub static XCB_GLX_PIXEL_STOREI : u8 = 110;
pub struct  ReadPixelsCookie<'s> { pub base : base::Cookie<'s, read_pixels_cookie> }

/** Opcode for xcb_glx_read_pixels. */
pub static XCB_GLX_READ_PIXELS : u8 = 111;
pub struct  GetBooleanvCookie<'s> { pub base : base::Cookie<'s, get_booleanv_cookie> }

/** Opcode for xcb_glx_get_booleanv. */
pub static XCB_GLX_GET_BOOLEANV : u8 = 112;
pub struct  GetClipPlaneCookie<'s> { pub base : base::Cookie<'s, get_clip_plane_cookie> }

/** Opcode for xcb_glx_get_clip_plane. */
pub static XCB_GLX_GET_CLIP_PLANE : u8 = 113;
pub struct  GetDoublevCookie<'s> { pub base : base::Cookie<'s, get_doublev_cookie> }

/** Opcode for xcb_glx_get_doublev. */
pub static XCB_GLX_GET_DOUBLEV : u8 = 114;
pub struct  GetErrorCookie<'s> { pub base : base::Cookie<'s, get_error_cookie> }

/** Opcode for xcb_glx_get_error. */
pub static XCB_GLX_GET_ERROR : u8 = 115;
pub struct GetErrorReply { base:  base::Reply<get_error_reply> }
fn mk_reply_get_error_reply(reply:*mut get_error_reply) -> GetErrorReply { GetErrorReply { base : base::mk_reply(reply) } }
pub struct  GetFloatvCookie<'s> { pub base : base::Cookie<'s, get_floatv_cookie> }

/** Opcode for xcb_glx_get_floatv. */
pub static XCB_GLX_GET_FLOATV : u8 = 116;
pub struct  GetIntegervCookie<'s> { pub base : base::Cookie<'s, get_integerv_cookie> }

/** Opcode for xcb_glx_get_integerv. */
pub static XCB_GLX_GET_INTEGERV : u8 = 117;
pub struct  GetLightfvCookie<'s> { pub base : base::Cookie<'s, get_lightfv_cookie> }

/** Opcode for xcb_glx_get_lightfv. */
pub static XCB_GLX_GET_LIGHTFV : u8 = 118;
pub struct  GetLightivCookie<'s> { pub base : base::Cookie<'s, get_lightiv_cookie> }

/** Opcode for xcb_glx_get_lightiv. */
pub static XCB_GLX_GET_LIGHTIV : u8 = 119;
pub struct  GetMapdvCookie<'s> { pub base : base::Cookie<'s, get_mapdv_cookie> }

/** Opcode for xcb_glx_get_mapdv. */
pub static XCB_GLX_GET_MAPDV : u8 = 120;
pub struct  GetMapfvCookie<'s> { pub base : base::Cookie<'s, get_mapfv_cookie> }

/** Opcode for xcb_glx_get_mapfv. */
pub static XCB_GLX_GET_MAPFV : u8 = 121;
pub struct  GetMapivCookie<'s> { pub base : base::Cookie<'s, get_mapiv_cookie> }

/** Opcode for xcb_glx_get_mapiv. */
pub static XCB_GLX_GET_MAPIV : u8 = 122;
pub struct  GetMaterialfvCookie<'s> { pub base : base::Cookie<'s, get_materialfv_cookie> }

/** Opcode for xcb_glx_get_materialfv. */
pub static XCB_GLX_GET_MATERIALFV : u8 = 123;
pub struct  GetMaterialivCookie<'s> { pub base : base::Cookie<'s, get_materialiv_cookie> }

/** Opcode for xcb_glx_get_materialiv. */
pub static XCB_GLX_GET_MATERIALIV : u8 = 124;
pub struct  GetPixelMapfvCookie<'s> { pub base : base::Cookie<'s, get_pixel_mapfv_cookie> }

/** Opcode for xcb_glx_get_pixel_mapfv. */
pub static XCB_GLX_GET_PIXEL_MAPFV : u8 = 125;
pub struct  GetPixelMapuivCookie<'s> { pub base : base::Cookie<'s, get_pixel_mapuiv_cookie> }

/** Opcode for xcb_glx_get_pixel_mapuiv. */
pub static XCB_GLX_GET_PIXEL_MAPUIV : u8 = 126;
pub struct  GetPixelMapusvCookie<'s> { pub base : base::Cookie<'s, get_pixel_mapusv_cookie> }

/** Opcode for xcb_glx_get_pixel_mapusv. */
pub static XCB_GLX_GET_PIXEL_MAPUSV : u8 = 127;
pub struct  GetPolygonStippleCookie<'s> { pub base : base::Cookie<'s, get_polygon_stipple_cookie> }

/** Opcode for xcb_glx_get_polygon_stipple. */
pub static XCB_GLX_GET_POLYGON_STIPPLE : u8 = 128;
pub struct  GetStringCookie<'s> { pub base : base::Cookie<'s, get_string_cookie> }

/** Opcode for xcb_glx_get_string. */
pub static XCB_GLX_GET_STRING : u8 = 129;
pub struct  GetTexEnvfvCookie<'s> { pub base : base::Cookie<'s, get_tex_envfv_cookie> }

/** Opcode for xcb_glx_get_tex_envfv. */
pub static XCB_GLX_GET_TEX_ENVFV : u8 = 130;
pub struct  GetTexEnvivCookie<'s> { pub base : base::Cookie<'s, get_tex_enviv_cookie> }

/** Opcode for xcb_glx_get_tex_enviv. */
pub static XCB_GLX_GET_TEX_ENVIV : u8 = 131;
pub struct  GetTexGendvCookie<'s> { pub base : base::Cookie<'s, get_tex_gendv_cookie> }

/** Opcode for xcb_glx_get_tex_gendv. */
pub static XCB_GLX_GET_TEX_GENDV : u8 = 132;
pub struct  GetTexGenfvCookie<'s> { pub base : base::Cookie<'s, get_tex_genfv_cookie> }

/** Opcode for xcb_glx_get_tex_genfv. */
pub static XCB_GLX_GET_TEX_GENFV : u8 = 133;
pub struct  GetTexGenivCookie<'s> { pub base : base::Cookie<'s, get_tex_geniv_cookie> }

/** Opcode for xcb_glx_get_tex_geniv. */
pub static XCB_GLX_GET_TEX_GENIV : u8 = 134;
pub struct  GetTexImageCookie<'s> { pub base : base::Cookie<'s, get_tex_image_cookie> }

/** Opcode for xcb_glx_get_tex_image. */
pub static XCB_GLX_GET_TEX_IMAGE : u8 = 135;
pub struct  GetTexParameterfvCookie<'s> { pub base : base::Cookie<'s, get_tex_parameterfv_cookie> }

/** Opcode for xcb_glx_get_tex_parameterfv. */
pub static XCB_GLX_GET_TEX_PARAMETERFV : u8 = 136;
pub struct  GetTexParameterivCookie<'s> { pub base : base::Cookie<'s, get_tex_parameteriv_cookie> }

/** Opcode for xcb_glx_get_tex_parameteriv. */
pub static XCB_GLX_GET_TEX_PARAMETERIV : u8 = 137;
pub struct  GetTexLevelParameterfvCookie<'s> { pub base : base::Cookie<'s, get_tex_level_parameterfv_cookie> }

/** Opcode for xcb_glx_get_tex_level_parameterfv. */
pub static XCB_GLX_GET_TEX_LEVEL_PARAMETERFV : u8 = 138;
pub struct  GetTexLevelParameterivCookie<'s> { pub base : base::Cookie<'s, get_tex_level_parameteriv_cookie> }

/** Opcode for xcb_glx_get_tex_level_parameteriv. */
pub static XCB_GLX_GET_TEX_LEVEL_PARAMETERIV : u8 = 139;
pub struct  IsListCookie<'s> { pub base : base::Cookie<'s, is_list_cookie> }

/** Opcode for xcb_glx_is_list. */
pub static XCB_GLX_IS_LIST : u8 = 141;
pub struct IsListReply { base:  base::Reply<is_list_reply> }
fn mk_reply_is_list_reply(reply:*mut is_list_reply) -> IsListReply { IsListReply { base : base::mk_reply(reply) } }
/** Opcode for xcb_glx_flush. */
pub static XCB_GLX_FLUSH : u8 = 142;
pub struct  AreTexturesResidentCookie<'s> { pub base : base::Cookie<'s, are_textures_resident_cookie> }

/** Opcode for xcb_glx_are_textures_resident. */
pub static XCB_GLX_ARE_TEXTURES_RESIDENT : u8 = 143;
pub struct AreTexturesResidentReply { base:  base::Reply<are_textures_resident_reply> }
fn mk_reply_are_textures_resident_reply(reply:*mut are_textures_resident_reply) -> AreTexturesResidentReply { AreTexturesResidentReply { base : base::mk_reply(reply) } }
/** Opcode for xcb_glx_delete_textures. */
pub static XCB_GLX_DELETE_TEXTURES : u8 = 144;
pub struct  GenTexturesCookie<'s> { pub base : base::Cookie<'s, gen_textures_cookie> }

/** Opcode for xcb_glx_gen_textures. */
pub static XCB_GLX_GEN_TEXTURES : u8 = 145;
pub struct  IsTextureCookie<'s> { pub base : base::Cookie<'s, is_texture_cookie> }

/** Opcode for xcb_glx_is_texture. */
pub static XCB_GLX_IS_TEXTURE : u8 = 146;
pub struct IsTextureReply { base:  base::Reply<is_texture_reply> }
fn mk_reply_is_texture_reply(reply:*mut is_texture_reply) -> IsTextureReply { IsTextureReply { base : base::mk_reply(reply) } }
pub struct  GetColorTableCookie<'s> { pub base : base::Cookie<'s, get_color_table_cookie> }

/** Opcode for xcb_glx_get_color_table. */
pub static XCB_GLX_GET_COLOR_TABLE : u8 = 147;
pub struct  GetColorTableParameterfvCookie<'s> { pub base : base::Cookie<'s, get_color_table_parameterfv_cookie> }

/** Opcode for xcb_glx_get_color_table_parameterfv. */
pub static XCB_GLX_GET_COLOR_TABLE_PARAMETERFV : u8 = 148;
pub struct  GetColorTableParameterivCookie<'s> { pub base : base::Cookie<'s, get_color_table_parameteriv_cookie> }

/** Opcode for xcb_glx_get_color_table_parameteriv. */
pub static XCB_GLX_GET_COLOR_TABLE_PARAMETERIV : u8 = 149;
pub struct  GetConvolutionFilterCookie<'s> { pub base : base::Cookie<'s, get_convolution_filter_cookie> }

/** Opcode for xcb_glx_get_convolution_filter. */
pub static XCB_GLX_GET_CONVOLUTION_FILTER : u8 = 150;
pub struct  GetConvolutionParameterfvCookie<'s> { pub base : base::Cookie<'s, get_convolution_parameterfv_cookie> }

/** Opcode for xcb_glx_get_convolution_parameterfv. */
pub static XCB_GLX_GET_CONVOLUTION_PARAMETERFV : u8 = 151;
pub struct  GetConvolutionParameterivCookie<'s> { pub base : base::Cookie<'s, get_convolution_parameteriv_cookie> }

/** Opcode for xcb_glx_get_convolution_parameteriv. */
pub static XCB_GLX_GET_CONVOLUTION_PARAMETERIV : u8 = 152;
pub struct  GetSeparableFilterCookie<'s> { pub base : base::Cookie<'s, get_separable_filter_cookie> }

/** Opcode for xcb_glx_get_separable_filter. */
pub static XCB_GLX_GET_SEPARABLE_FILTER : u8 = 153;
pub struct  GetHistogramCookie<'s> { pub base : base::Cookie<'s, get_histogram_cookie> }

/** Opcode for xcb_glx_get_histogram. */
pub static XCB_GLX_GET_HISTOGRAM : u8 = 154;
pub struct  GetHistogramParameterfvCookie<'s> { pub base : base::Cookie<'s, get_histogram_parameterfv_cookie> }

/** Opcode for xcb_glx_get_histogram_parameterfv. */
pub static XCB_GLX_GET_HISTOGRAM_PARAMETERFV : u8 = 155;
pub struct  GetHistogramParameterivCookie<'s> { pub base : base::Cookie<'s, get_histogram_parameteriv_cookie> }

/** Opcode for xcb_glx_get_histogram_parameteriv. */
pub static XCB_GLX_GET_HISTOGRAM_PARAMETERIV : u8 = 156;
pub struct  GetMinmaxCookie<'s> { pub base : base::Cookie<'s, get_minmax_cookie> }

/** Opcode for xcb_glx_get_minmax. */
pub static XCB_GLX_GET_MINMAX : u8 = 157;
pub struct  GetMinmaxParameterfvCookie<'s> { pub base : base::Cookie<'s, get_minmax_parameterfv_cookie> }

/** Opcode for xcb_glx_get_minmax_parameterfv. */
pub static XCB_GLX_GET_MINMAX_PARAMETERFV : u8 = 158;
pub struct  GetMinmaxParameterivCookie<'s> { pub base : base::Cookie<'s, get_minmax_parameteriv_cookie> }

/** Opcode for xcb_glx_get_minmax_parameteriv. */
pub static XCB_GLX_GET_MINMAX_PARAMETERIV : u8 = 159;
pub struct  GetCompressedTexImageArbCookie<'s> { pub base : base::Cookie<'s, get_compressed_tex_image_arb_cookie> }

/** Opcode for xcb_glx_get_compressed_tex_image_arb. */
pub static XCB_GLX_GET_COMPRESSED_TEX_IMAGE_ARB : u8 = 160;
/** Opcode for xcb_glx_delete_queries_arb. */
pub static XCB_GLX_DELETE_QUERIES_ARB : u8 = 161;
pub struct  GenQueriesArbCookie<'s> { pub base : base::Cookie<'s, gen_queries_arb_cookie> }

/** Opcode for xcb_glx_gen_queries_arb. */
pub static XCB_GLX_GEN_QUERIES_ARB : u8 = 162;
pub struct  IsQueryArbCookie<'s> { pub base : base::Cookie<'s, is_query_arb_cookie> }

/** Opcode for xcb_glx_is_query_arb. */
pub static XCB_GLX_IS_QUERY_ARB : u8 = 163;
pub struct IsQueryArbReply { base:  base::Reply<is_query_arb_reply> }
fn mk_reply_is_query_arb_reply(reply:*mut is_query_arb_reply) -> IsQueryArbReply { IsQueryArbReply { base : base::mk_reply(reply) } }
pub struct  GetQueryivArbCookie<'s> { pub base : base::Cookie<'s, get_queryiv_arb_cookie> }

/** Opcode for xcb_glx_get_queryiv_arb. */
pub static XCB_GLX_GET_QUERYIV_ARB : u8 = 164;
pub struct  GetQueryObjectivArbCookie<'s> { pub base : base::Cookie<'s, get_query_objectiv_arb_cookie> }

/** Opcode for xcb_glx_get_query_objectiv_arb. */
pub static XCB_GLX_GET_QUERY_OBJECTIV_ARB : u8 = 165;
pub struct  GetQueryObjectuivArbCookie<'s> { pub base : base::Cookie<'s, get_query_objectuiv_arb_cookie> }

/** Opcode for xcb_glx_get_query_objectuiv_arb. */
pub static XCB_GLX_GET_QUERY_OBJECTUIV_ARB : u8 = 166;

impl Iterator for PixmapIterator {
    type Item = Pixmap;
    fn next(&mut self) -> Option<Pixmap> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut pixmap_iterator = mem::transmute(self);
            let data = (*iter).data;
            xcb_glx_pixmap_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub type Context = context;


impl Iterator for ContextIterator {
    type Item = Context;
    fn next(&mut self) -> Option<Context> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut context_iterator = mem::transmute(self);
            let data = (*iter).data;
            xcb_glx_context_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub type Pbuffer = pbuffer;


impl Iterator for PbufferIterator {
    type Item = Pbuffer;
    fn next(&mut self) -> Option<Pbuffer> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut pbuffer_iterator = mem::transmute(self);
            let data = (*iter).data;
            xcb_glx_pbuffer_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub type Window = window;


impl Iterator for WindowIterator {
    type Item = Window;
    fn next(&mut self) -> Option<Window> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut window_iterator = mem::transmute(self);
            let data = (*iter).data;
            xcb_glx_window_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub type Fbconfig = fbconfig;


impl Iterator for FbconfigIterator {
    type Item = Fbconfig;
    fn next(&mut self) -> Option<Fbconfig> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut fbconfig_iterator = mem::transmute(self);
            let data = (*iter).data;
            xcb_glx_fbconfig_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub type Drawable = drawable;


impl Iterator for DrawableIterator {
    type Item = Drawable;
    fn next(&mut self) -> Option<Drawable> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut drawable_iterator = mem::transmute(self);
            let data = (*iter).data;
            xcb_glx_drawable_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub type Float32 = float32;


impl Iterator for Float32Iterator {
    type Item = Float32;
    fn next(&mut self) -> Option<Float32> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut float32_iterator = mem::transmute(self);
            let data = (*iter).data;
            xcb_glx_float32_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub type Float64 = float64;


impl Iterator for Float64Iterator {
    type Item = Float64;
    fn next(&mut self) -> Option<Float64> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut float64_iterator = mem::transmute(self);
            let data = (*iter).data;
            xcb_glx_float64_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub type Bool32 = bool32;


impl Iterator for Bool32Iterator {
    type Item = Bool32;
    fn next(&mut self) -> Option<Bool32> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut bool32_iterator = mem::transmute(self);
            let data = (*iter).data;
            xcb_glx_bool32_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub type ContextTag = context_tag;


impl Iterator for ContextTagIterator {
    type Item = ContextTag;
    fn next(&mut self) -> Option<ContextTag> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut context_tag_iterator = mem::transmute(self);
            let data = (*iter).data;
            xcb_glx_context_tag_next(iter);
            Some(mem::transmute(*data))
        }
    }
}


impl PbufferClobberEvent {
  pub fn event_type(&mut self) -> u16 {
    unsafe { accessor!(event_type -> u16, (*self.base.event)) }
  }

  pub fn draw_type(&mut self) -> u16 {
    unsafe { accessor!(draw_type -> u16, (*self.base.event)) }
  }

  pub fn drawable(&mut self) -> Drawable {
    unsafe { accessor!(drawable -> Drawable, (*self.base.event)) }
  }

  pub fn b_mask(&mut self) -> u32 {
    unsafe { accessor!(b_mask -> u32, (*self.base.event)) }
  }

  pub fn aux_buffer(&mut self) -> u16 {
    unsafe { accessor!(aux_buffer -> u16, (*self.base.event)) }
  }

  pub fn x(&mut self) -> u16 {
    unsafe { accessor!(x -> u16, (*self.base.event)) }
  }

  pub fn y(&mut self) -> u16 {
    unsafe { accessor!(y -> u16, (*self.base.event)) }
  }

  pub fn width(&mut self) -> u16 {
    unsafe { accessor!(width -> u16, (*self.base.event)) }
  }

  pub fn height(&mut self) -> u16 {
    unsafe { accessor!(height -> u16, (*self.base.event)) }
  }

  pub fn count(&mut self) -> u16 {
    unsafe { accessor!(count -> u16, (*self.base.event)) }
  }

  pub fn new(event_type : u16,
         draw_type : u16,
         drawable : Drawable,
         b_mask : u32,
         aux_buffer : u16,
         x : u16,
         y : u16,
         width : u16,
         height : u16,
         count : u16) -> PbufferClobberEvent {
    unsafe {
      let raw = malloc(32 as size_t) as *mut pbuffer_clobber_event;
      (*raw).event_type = event_type;
      (*raw).draw_type = draw_type;
      (*raw).drawable = drawable;
      (*raw).b_mask = b_mask;
      (*raw).aux_buffer = aux_buffer;
      (*raw).x = x;
      (*raw).y = y;
      (*raw).width = width;
      (*raw).height = height;
      (*raw).count = count;
      PbufferClobberEvent { base : Event { event : raw as *mut pbuffer_clobber_event }}
    }
  }
}
pub fn RenderChecked<'r> (c : &'r Connection,
                      context_tag : ContextTag,
                      data : &[u8]) -> base::VoidCookie<'r> {
  unsafe {
    let data_len = data.len();
    let data_ptr = data.as_ptr();
    let cookie = xcb_glx_render_checked(c.get_raw_conn(),
        context_tag as context_tag, //1
        data_len as u32, //2
        data_ptr as *mut u8); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn Render<'r> (c : &'r Connection,
               context_tag : ContextTag,
               data : &[u8]) -> base::VoidCookie<'r> {
  unsafe {
    let data_len = data.len();
    let data_ptr = data.as_ptr();
    let cookie = xcb_glx_render(c.get_raw_conn(),
        context_tag as context_tag, //1
        data_len as u32, //2
        data_ptr as *mut u8); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn RenderLargeChecked<'r> (c : &'r Connection,
                           context_tag : ContextTag,
                           request_num : u16,
                           request_total : u16,
                           data : &[u8]) -> base::VoidCookie<'r> {
  unsafe {
    let data_len = data.len();
    let data_ptr = data.as_ptr();
    let cookie = xcb_glx_render_large_checked(c.get_raw_conn(),
        context_tag as context_tag, //1
        request_num as u16, //2
        request_total as u16, //3
        data_len as u32, //4
        data_ptr as *mut u8); //5
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn RenderLarge<'r> (c : &'r Connection,
                    context_tag : ContextTag,
                    request_num : u16,
                    request_total : u16,
                    data : &[u8]) -> base::VoidCookie<'r> {
  unsafe {
    let data_len = data.len();
    let data_ptr = data.as_ptr();
    let cookie = xcb_glx_render_large(c.get_raw_conn(),
        context_tag as context_tag, //1
        request_num as u16, //2
        request_total as u16, //3
        data_len as u32, //4
        data_ptr as *mut u8); //5
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn CreateContextChecked<'r> (c : &'r Connection,
                             context : Context,
                             visual : xproto::Visualid,
                             screen : u32,
                             share_list : Context,
                             is_direct : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_glx_create_context_checked(c.get_raw_conn(),
        context as context, //1
        visual as ffi::xproto::visualid, //2
        screen as u32, //3
        share_list as context, //4
        is_direct as u8); //5
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn CreateContext<'r> (c : &'r Connection,
                      context : Context,
                      visual : xproto::Visualid,
                      screen : u32,
                      share_list : Context,
                      is_direct : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_glx_create_context(c.get_raw_conn(),
        context as context, //1
        visual as ffi::xproto::visualid, //2
        screen as u32, //3
        share_list as context, //4
        is_direct as u8); //5
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn DestroyContextChecked<'r> (c : &'r Connection,
                              context : Context) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_glx_destroy_context_checked(c.get_raw_conn(),
        context as context); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn DestroyContext<'r> (c : &'r Connection,
                       context : Context) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_glx_destroy_context(c.get_raw_conn(),
        context as context); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn MakeCurrent<'r> (c : &'r Connection,
                    drawable : Drawable,
                    context : Context,
                    old_context_tag : ContextTag) -> MakeCurrentCookie<'r> {
  unsafe {
    let cookie = xcb_glx_make_current(c.get_raw_conn(),
        drawable as drawable, //1
        context as context, //2
        old_context_tag as context_tag); //3
    MakeCurrentCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn MakeCurrentUnchecked<'r> (c : &'r Connection,
                             drawable : Drawable,
                             context : Context,
                             old_context_tag : ContextTag) -> MakeCurrentCookie<'r> {
  unsafe {
    let cookie = xcb_glx_make_current_unchecked(c.get_raw_conn(),
        drawable as drawable, //1
        context as context, //2
        old_context_tag as context_tag); //3
    MakeCurrentCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl MakeCurrentReply {
  pub fn context_tag(&mut self) -> ContextTag {
    unsafe { accessor!(context_tag -> ContextTag, (*self.base.reply)) }
  }

}
impl_reply_cookie!(MakeCurrentCookie<'s>, mk_reply_make_current_reply, MakeCurrentReply, xcb_glx_make_current_reply);

pub fn IsDirect<'r> (c : &'r Connection,
                 context : Context) -> IsDirectCookie<'r> {
  unsafe {
    let cookie = xcb_glx_is_direct(c.get_raw_conn(),
        context as context); //1
    IsDirectCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn IsDirectUnchecked<'r> (c : &'r Connection,
                          context : Context) -> IsDirectCookie<'r> {
  unsafe {
    let cookie = xcb_glx_is_direct_unchecked(c.get_raw_conn(),
        context as context); //1
    IsDirectCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl IsDirectReply {
  pub fn is_direct(&mut self) -> u8 {
    unsafe { accessor!(is_direct -> u8, (*self.base.reply)) }
  }

}
impl_reply_cookie!(IsDirectCookie<'s>, mk_reply_is_direct_reply, IsDirectReply, xcb_glx_is_direct_reply);

pub fn QueryVersion<'r> (c : &'r Connection,
                     major_version : u32,
                     minor_version : u32) -> QueryVersionCookie<'r> {
  unsafe {
    let cookie = xcb_glx_query_version(c.get_raw_conn(),
        major_version as u32, //1
        minor_version as u32); //2
    QueryVersionCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn QueryVersionUnchecked<'r> (c : &'r Connection,
                              major_version : u32,
                              minor_version : u32) -> QueryVersionCookie<'r> {
  unsafe {
    let cookie = xcb_glx_query_version_unchecked(c.get_raw_conn(),
        major_version as u32, //1
        minor_version as u32); //2
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
impl_reply_cookie!(QueryVersionCookie<'s>, mk_reply_query_version_reply, QueryVersionReply, xcb_glx_query_version_reply);

pub fn WaitGlChecked<'r> (c : &'r Connection,
                      context_tag : ContextTag) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_glx_wait_gl_checked(c.get_raw_conn(),
        context_tag as context_tag); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn WaitGl<'r> (c : &'r Connection,
               context_tag : ContextTag) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_glx_wait_gl(c.get_raw_conn(),
        context_tag as context_tag); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn WaitXChecked<'r> (c : &'r Connection,
                     context_tag : ContextTag) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_glx_wait_x_checked(c.get_raw_conn(),
        context_tag as context_tag); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn WaitX<'r> (c : &'r Connection,
              context_tag : ContextTag) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_glx_wait_x(c.get_raw_conn(),
        context_tag as context_tag); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn CopyContextChecked<'r> (c : &'r Connection,
                           src : Context,
                           dest : Context,
                           mask : u32,
                           src_context_tag : ContextTag) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_glx_copy_context_checked(c.get_raw_conn(),
        src as context, //1
        dest as context, //2
        mask as u32, //3
        src_context_tag as context_tag); //4
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn CopyContext<'r> (c : &'r Connection,
                    src : Context,
                    dest : Context,
                    mask : u32,
                    src_context_tag : ContextTag) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_glx_copy_context(c.get_raw_conn(),
        src as context, //1
        dest as context, //2
        mask as u32, //3
        src_context_tag as context_tag); //4
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn SwapBuffersChecked<'r> (c : &'r Connection,
                           context_tag : ContextTag,
                           drawable : Drawable) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_glx_swap_buffers_checked(c.get_raw_conn(),
        context_tag as context_tag, //1
        drawable as drawable); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn SwapBuffers<'r> (c : &'r Connection,
                    context_tag : ContextTag,
                    drawable : Drawable) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_glx_swap_buffers(c.get_raw_conn(),
        context_tag as context_tag, //1
        drawable as drawable); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn UseXFontChecked<'r> (c : &'r Connection,
                        context_tag : ContextTag,
                        font : xproto::Font,
                        first : u32,
                        count : u32,
                        list_base : u32) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_glx_use_x_font_checked(c.get_raw_conn(),
        context_tag as context_tag, //1
        font as ffi::xproto::font, //2
        first as u32, //3
        count as u32, //4
        list_base as u32); //5
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn UseXFont<'r> (c : &'r Connection,
                 context_tag : ContextTag,
                 font : xproto::Font,
                 first : u32,
                 count : u32,
                 list_base : u32) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_glx_use_x_font(c.get_raw_conn(),
        context_tag as context_tag, //1
        font as ffi::xproto::font, //2
        first as u32, //3
        count as u32, //4
        list_base as u32); //5
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn CreateGlxPixmapChecked<'r> (c : &'r Connection,
                               screen : u32,
                               visual : xproto::Visualid,
                               pixmap : xproto::Pixmap,
                               glx_pixmap : Pixmap) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_glx_create_glx_pixmap_checked(c.get_raw_conn(),
        screen as u32, //1
        visual as ffi::xproto::visualid, //2
        pixmap as ffi::xproto::pixmap, //3
        glx_pixmap as pixmap); //4
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn CreateGlxPixmap<'r> (c : &'r Connection,
                        screen : u32,
                        visual : xproto::Visualid,
                        pixmap : xproto::Pixmap,
                        glx_pixmap : Pixmap) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_glx_create_glx_pixmap(c.get_raw_conn(),
        screen as u32, //1
        visual as ffi::xproto::visualid, //2
        pixmap as ffi::xproto::pixmap, //3
        glx_pixmap as pixmap); //4
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub struct GetVisualConfigsReply { base:  base::Reply<get_visual_configs_reply> }
fn mk_reply_get_visual_configs_reply(reply:*mut get_visual_configs_reply) -> GetVisualConfigsReply { GetVisualConfigsReply { base : base::mk_reply(reply) } }
pub fn GetVisualConfigs<'r> (c : &'r Connection,
                         screen : u32) -> GetVisualConfigsCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_visual_configs(c.get_raw_conn(),
        screen as u32); //1
    GetVisualConfigsCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetVisualConfigsUnchecked<'r> (c : &'r Connection,
                                  screen : u32) -> GetVisualConfigsCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_visual_configs_unchecked(c.get_raw_conn(),
        screen as u32); //1
    GetVisualConfigsCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetVisualConfigsReply {
  pub fn num_visuals(&mut self) -> u32 {
    unsafe { accessor!(num_visuals -> u32, (*self.base.reply)) }
  }

  pub fn num_properties(&mut self) -> u32 {
    unsafe { accessor!(num_properties -> u32, (*self.base.reply)) }
  }

  pub fn property_list(&mut self) -> Vec<u32> {
    unsafe { accessor!(u32, xcb_glx_get_visual_configs_property_list_length, xcb_glx_get_visual_configs_property_list, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetVisualConfigsCookie<'s>, mk_reply_get_visual_configs_reply, GetVisualConfigsReply, xcb_glx_get_visual_configs_reply);

pub fn DestroyGlxPixmapChecked<'r> (c : &'r Connection,
                                glx_pixmap : Pixmap) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_glx_destroy_glx_pixmap_checked(c.get_raw_conn(),
        glx_pixmap as pixmap); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn DestroyGlxPixmap<'r> (c : &'r Connection,
                         glx_pixmap : Pixmap) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_glx_destroy_glx_pixmap(c.get_raw_conn(),
        glx_pixmap as pixmap); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn VendorPrivateChecked<'r> (c : &'r Connection,
                             vendor_code : u32,
                             context_tag : ContextTag,
                             data : &[u8]) -> base::VoidCookie<'r> {
  unsafe {
    let data_len = data.len();
    let data_ptr = data.as_ptr();
    let cookie = xcb_glx_vendor_private_checked(c.get_raw_conn(),
        vendor_code as u32, //1
        context_tag as context_tag, //2
        data_len as u32, //3
        data_ptr as *mut u8); //4
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn VendorPrivate<'r> (c : &'r Connection,
                      vendor_code : u32,
                      context_tag : ContextTag,
                      data : &[u8]) -> base::VoidCookie<'r> {
  unsafe {
    let data_len = data.len();
    let data_ptr = data.as_ptr();
    let cookie = xcb_glx_vendor_private(c.get_raw_conn(),
        vendor_code as u32, //1
        context_tag as context_tag, //2
        data_len as u32, //3
        data_ptr as *mut u8); //4
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn VendorPrivateWithReply<'r> (c : &'r Connection,
                               vendor_code : u32,
                               context_tag : ContextTag,
                               data : &[u8]) -> VendorPrivateWithReplyCookie<'r> {
  unsafe {
    let data_len = data.len();
    let data_ptr = data.as_ptr();
    let cookie = xcb_glx_vendor_private_with_reply(c.get_raw_conn(),
        vendor_code as u32, //1
        context_tag as context_tag, //2
        data_len as u32, //3
        data_ptr as *mut u8); //4
    VendorPrivateWithReplyCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn VendorPrivateWithReplyUnchecked<'r> (c : &'r Connection,
                                        vendor_code : u32,
                                        context_tag : ContextTag,
                                        data : &[u8]) -> VendorPrivateWithReplyCookie<'r> {
  unsafe {
    let data_len = data.len();
    let data_ptr = data.as_ptr();
    let cookie = xcb_glx_vendor_private_with_reply_unchecked(c.get_raw_conn(),
        vendor_code as u32, //1
        context_tag as context_tag, //2
        data_len as u32, //3
        data_ptr as *mut u8); //4
    VendorPrivateWithReplyCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl VendorPrivateWithReplyReply {
  pub fn retval(&mut self) -> u32 {
    unsafe { accessor!(retval -> u32, (*self.base.reply)) }
  }

  pub fn data1(&self) -> Vec<u8> {
    unsafe { ((*self.base.reply).data1).to_vec() }
  }

  pub fn data2(&mut self) -> Vec<u8> {
    unsafe { accessor!(u8, xcb_glx_vendor_private_with_reply_data_2_length, xcb_glx_vendor_private_with_reply_data_2, (*self.base.reply)) }
  }

}
impl_reply_cookie!(VendorPrivateWithReplyCookie<'s>, mk_reply_vendor_private_with_reply_reply, VendorPrivateWithReplyReply, xcb_glx_vendor_private_with_reply_reply);

pub fn QueryExtensionsString<'r> (c : &'r Connection,
                              screen : u32) -> QueryExtensionsStringCookie<'r> {
  unsafe {
    let cookie = xcb_glx_query_extensions_string(c.get_raw_conn(),
        screen as u32); //1
    QueryExtensionsStringCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn QueryExtensionsStringUnchecked<'r> (c : &'r Connection,
                                       screen : u32) -> QueryExtensionsStringCookie<'r> {
  unsafe {
    let cookie = xcb_glx_query_extensions_string_unchecked(c.get_raw_conn(),
        screen as u32); //1
    QueryExtensionsStringCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl QueryExtensionsStringReply {
  pub fn n(&mut self) -> u32 {
    unsafe { accessor!(n -> u32, (*self.base.reply)) }
  }

}
impl_reply_cookie!(QueryExtensionsStringCookie<'s>, mk_reply_query_extensions_string_reply, QueryExtensionsStringReply, xcb_glx_query_extensions_string_reply);

pub struct QueryServerStringReply { base:  base::Reply<query_server_string_reply> }
fn mk_reply_query_server_string_reply(reply:*mut query_server_string_reply) -> QueryServerStringReply { QueryServerStringReply { base : base::mk_reply(reply) } }
pub fn QueryServerString<'r> (c : &'r Connection,
                          screen : u32,
                          name : u32) -> QueryServerStringCookie<'r> {
  unsafe {
    let cookie = xcb_glx_query_server_string(c.get_raw_conn(),
        screen as u32, //1
        name as u32); //2
    QueryServerStringCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn QueryServerStringUnchecked<'r> (c : &'r Connection,
                                   screen : u32,
                                   name : u32) -> QueryServerStringCookie<'r> {
  unsafe {
    let cookie = xcb_glx_query_server_string_unchecked(c.get_raw_conn(),
        screen as u32, //1
        name as u32); //2
    QueryServerStringCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl QueryServerStringReply {
  pub fn string(&mut self) -> String {
    unsafe { accessor!(str, xcb_glx_query_server_string_string_length, xcb_glx_query_server_string_string, (*self.base.reply)) }
  }

}
impl_reply_cookie!(QueryServerStringCookie<'s>, mk_reply_query_server_string_reply, QueryServerStringReply, xcb_glx_query_server_string_reply);

pub fn ClientInfoChecked<'r> (c : &'r Connection,
                          major_version : u32,
                          minor_version : u32,
                          string : &str) -> base::VoidCookie<'r> {
  unsafe {
    let string = (string).as_bytes();
    let string_len = string.len();
    let string_ptr = string.as_ptr();
    let cookie = xcb_glx_client_info_checked(c.get_raw_conn(),
        major_version as u32, //1
        minor_version as u32, //2
        string_len as u32, //3
        string_ptr as *mut c_char); //4
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn ClientInfo<'r> (c : &'r Connection,
                   major_version : u32,
                   minor_version : u32,
                   string : &str) -> base::VoidCookie<'r> {
  unsafe {
    let string = (string).as_bytes();
    let string_len = string.len();
    let string_ptr = string.as_ptr();
    let cookie = xcb_glx_client_info(c.get_raw_conn(),
        major_version as u32, //1
        minor_version as u32, //2
        string_len as u32, //3
        string_ptr as *mut c_char); //4
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub struct GetFbConfigsReply { base:  base::Reply<get_fb_configs_reply> }
fn mk_reply_get_fb_configs_reply(reply:*mut get_fb_configs_reply) -> GetFbConfigsReply { GetFbConfigsReply { base : base::mk_reply(reply) } }
pub fn GetFbConfigs<'r> (c : &'r Connection,
                     screen : u32) -> GetFbConfigsCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_fb_configs(c.get_raw_conn(),
        screen as u32); //1
    GetFbConfigsCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetFbConfigsUnchecked<'r> (c : &'r Connection,
                              screen : u32) -> GetFbConfigsCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_fb_configs_unchecked(c.get_raw_conn(),
        screen as u32); //1
    GetFbConfigsCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetFbConfigsReply {
  pub fn num_FB_configs(&mut self) -> u32 {
    unsafe { accessor!(num_FB_configs -> u32, (*self.base.reply)) }
  }

  pub fn num_properties(&mut self) -> u32 {
    unsafe { accessor!(num_properties -> u32, (*self.base.reply)) }
  }

  pub fn property_list(&mut self) -> Vec<u32> {
    unsafe { accessor!(u32, xcb_glx_get_fb_configs_property_list_length, xcb_glx_get_fb_configs_property_list, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetFbConfigsCookie<'s>, mk_reply_get_fb_configs_reply, GetFbConfigsReply, xcb_glx_get_fb_configs_reply);

pub fn CreatePixmapChecked<'r> (c : &'r Connection,
                            screen : u32,
                            fbconfig : Fbconfig,
                            pixmap : xproto::Pixmap,
                            glx_pixmap : Pixmap,
                            attribs : &[u32]) -> base::VoidCookie<'r> {
  unsafe {
    let attribs_len = attribs.len();
    let attribs_ptr = attribs.as_ptr();
    let cookie = xcb_glx_create_pixmap_checked(c.get_raw_conn(),
        screen as u32, //1
        fbconfig as fbconfig, //2
        pixmap as ffi::xproto::pixmap, //3
        glx_pixmap as pixmap, //4
        attribs_len as u32, //5
        attribs_ptr as *mut u32); //6
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn CreatePixmap<'r> (c : &'r Connection,
                     screen : u32,
                     fbconfig : Fbconfig,
                     pixmap : xproto::Pixmap,
                     glx_pixmap : Pixmap,
                     attribs : &[u32]) -> base::VoidCookie<'r> {
  unsafe {
    let attribs_len = attribs.len();
    let attribs_ptr = attribs.as_ptr();
    let cookie = xcb_glx_create_pixmap(c.get_raw_conn(),
        screen as u32, //1
        fbconfig as fbconfig, //2
        pixmap as ffi::xproto::pixmap, //3
        glx_pixmap as pixmap, //4
        attribs_len as u32, //5
        attribs_ptr as *mut u32); //6
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn DestroyPixmapChecked<'r> (c : &'r Connection,
                             glx_pixmap : Pixmap) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_glx_destroy_pixmap_checked(c.get_raw_conn(),
        glx_pixmap as pixmap); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn DestroyPixmap<'r> (c : &'r Connection,
                      glx_pixmap : Pixmap) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_glx_destroy_pixmap(c.get_raw_conn(),
        glx_pixmap as pixmap); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn CreateNewContextChecked<'r> (c : &'r Connection,
                                context : Context,
                                fbconfig : Fbconfig,
                                screen : u32,
                                render_type : u32,
                                share_list : Context,
                                is_direct : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_glx_create_new_context_checked(c.get_raw_conn(),
        context as context, //1
        fbconfig as fbconfig, //2
        screen as u32, //3
        render_type as u32, //4
        share_list as context, //5
        is_direct as u8); //6
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn CreateNewContext<'r> (c : &'r Connection,
                         context : Context,
                         fbconfig : Fbconfig,
                         screen : u32,
                         render_type : u32,
                         share_list : Context,
                         is_direct : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_glx_create_new_context(c.get_raw_conn(),
        context as context, //1
        fbconfig as fbconfig, //2
        screen as u32, //3
        render_type as u32, //4
        share_list as context, //5
        is_direct as u8); //6
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub struct QueryContextReply { base:  base::Reply<query_context_reply> }
fn mk_reply_query_context_reply(reply:*mut query_context_reply) -> QueryContextReply { QueryContextReply { base : base::mk_reply(reply) } }
pub fn QueryContext<'r> (c : &'r Connection,
                     context : Context) -> QueryContextCookie<'r> {
  unsafe {
    let cookie = xcb_glx_query_context(c.get_raw_conn(),
        context as context); //1
    QueryContextCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn QueryContextUnchecked<'r> (c : &'r Connection,
                              context : Context) -> QueryContextCookie<'r> {
  unsafe {
    let cookie = xcb_glx_query_context_unchecked(c.get_raw_conn(),
        context as context); //1
    QueryContextCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl QueryContextReply {
  pub fn attribs(&mut self) -> Vec<u32> {
    unsafe { accessor!(u32, xcb_glx_query_context_attribs_length, xcb_glx_query_context_attribs, (*self.base.reply)) }
  }

}
impl_reply_cookie!(QueryContextCookie<'s>, mk_reply_query_context_reply, QueryContextReply, xcb_glx_query_context_reply);

pub fn MakeContextCurrent<'r> (c : &'r Connection,
                           old_context_tag : ContextTag,
                           drawable : Drawable,
                           read_drawable : Drawable,
                           context : Context) -> MakeContextCurrentCookie<'r> {
  unsafe {
    let cookie = xcb_glx_make_context_current(c.get_raw_conn(),
        old_context_tag as context_tag, //1
        drawable as drawable, //2
        read_drawable as drawable, //3
        context as context); //4
    MakeContextCurrentCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn MakeContextCurrentUnchecked<'r> (c : &'r Connection,
                                    old_context_tag : ContextTag,
                                    drawable : Drawable,
                                    read_drawable : Drawable,
                                    context : Context) -> MakeContextCurrentCookie<'r> {
  unsafe {
    let cookie = xcb_glx_make_context_current_unchecked(c.get_raw_conn(),
        old_context_tag as context_tag, //1
        drawable as drawable, //2
        read_drawable as drawable, //3
        context as context); //4
    MakeContextCurrentCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl MakeContextCurrentReply {
  pub fn context_tag(&mut self) -> ContextTag {
    unsafe { accessor!(context_tag -> ContextTag, (*self.base.reply)) }
  }

}
impl_reply_cookie!(MakeContextCurrentCookie<'s>, mk_reply_make_context_current_reply, MakeContextCurrentReply, xcb_glx_make_context_current_reply);

pub fn CreatePbufferChecked<'r> (c : &'r Connection,
                             screen : u32,
                             fbconfig : Fbconfig,
                             pbuffer : Pbuffer,
                             attribs : &[u32]) -> base::VoidCookie<'r> {
  unsafe {
    let attribs_len = attribs.len();
    let attribs_ptr = attribs.as_ptr();
    let cookie = xcb_glx_create_pbuffer_checked(c.get_raw_conn(),
        screen as u32, //1
        fbconfig as fbconfig, //2
        pbuffer as pbuffer, //3
        attribs_len as u32, //4
        attribs_ptr as *mut u32); //5
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn CreatePbuffer<'r> (c : &'r Connection,
                      screen : u32,
                      fbconfig : Fbconfig,
                      pbuffer : Pbuffer,
                      attribs : &[u32]) -> base::VoidCookie<'r> {
  unsafe {
    let attribs_len = attribs.len();
    let attribs_ptr = attribs.as_ptr();
    let cookie = xcb_glx_create_pbuffer(c.get_raw_conn(),
        screen as u32, //1
        fbconfig as fbconfig, //2
        pbuffer as pbuffer, //3
        attribs_len as u32, //4
        attribs_ptr as *mut u32); //5
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn DestroyPbufferChecked<'r> (c : &'r Connection,
                              pbuffer : Pbuffer) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_glx_destroy_pbuffer_checked(c.get_raw_conn(),
        pbuffer as pbuffer); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn DestroyPbuffer<'r> (c : &'r Connection,
                       pbuffer : Pbuffer) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_glx_destroy_pbuffer(c.get_raw_conn(),
        pbuffer as pbuffer); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub struct GetDrawableAttributesReply { base:  base::Reply<get_drawable_attributes_reply> }
fn mk_reply_get_drawable_attributes_reply(reply:*mut get_drawable_attributes_reply) -> GetDrawableAttributesReply { GetDrawableAttributesReply { base : base::mk_reply(reply) } }
pub fn GetDrawableAttributes<'r> (c : &'r Connection,
                              drawable : Drawable) -> GetDrawableAttributesCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_drawable_attributes(c.get_raw_conn(),
        drawable as drawable); //1
    GetDrawableAttributesCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetDrawableAttributesUnchecked<'r> (c : &'r Connection,
                                       drawable : Drawable) -> GetDrawableAttributesCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_drawable_attributes_unchecked(c.get_raw_conn(),
        drawable as drawable); //1
    GetDrawableAttributesCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetDrawableAttributesReply {
  pub fn attribs(&mut self) -> Vec<u32> {
    unsafe { accessor!(u32, xcb_glx_get_drawable_attributes_attribs_length, xcb_glx_get_drawable_attributes_attribs, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetDrawableAttributesCookie<'s>, mk_reply_get_drawable_attributes_reply, GetDrawableAttributesReply, xcb_glx_get_drawable_attributes_reply);

pub fn ChangeDrawableAttributesChecked<'r> (c : &'r Connection,
                                        drawable : Drawable,
                                        attribs : &[u32]) -> base::VoidCookie<'r> {
  unsafe {
    let attribs_len = attribs.len();
    let attribs_ptr = attribs.as_ptr();
    let cookie = xcb_glx_change_drawable_attributes_checked(c.get_raw_conn(),
        drawable as drawable, //1
        attribs_len as u32, //2
        attribs_ptr as *mut u32); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn ChangeDrawableAttributes<'r> (c : &'r Connection,
                                 drawable : Drawable,
                                 attribs : &[u32]) -> base::VoidCookie<'r> {
  unsafe {
    let attribs_len = attribs.len();
    let attribs_ptr = attribs.as_ptr();
    let cookie = xcb_glx_change_drawable_attributes(c.get_raw_conn(),
        drawable as drawable, //1
        attribs_len as u32, //2
        attribs_ptr as *mut u32); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn CreateWindowChecked<'r> (c : &'r Connection,
                            screen : u32,
                            fbconfig : Fbconfig,
                            window : xproto::Window,
                            glx_window : Window,
                            attribs : &[u32]) -> base::VoidCookie<'r> {
  unsafe {
    let attribs_len = attribs.len();
    let attribs_ptr = attribs.as_ptr();
    let cookie = xcb_glx_create_window_checked(c.get_raw_conn(),
        screen as u32, //1
        fbconfig as fbconfig, //2
        window as ffi::xproto::window, //3
        glx_window as window, //4
        attribs_len as u32, //5
        attribs_ptr as *mut u32); //6
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn CreateWindow<'r> (c : &'r Connection,
                     screen : u32,
                     fbconfig : Fbconfig,
                     window : xproto::Window,
                     glx_window : Window,
                     attribs : &[u32]) -> base::VoidCookie<'r> {
  unsafe {
    let attribs_len = attribs.len();
    let attribs_ptr = attribs.as_ptr();
    let cookie = xcb_glx_create_window(c.get_raw_conn(),
        screen as u32, //1
        fbconfig as fbconfig, //2
        window as ffi::xproto::window, //3
        glx_window as window, //4
        attribs_len as u32, //5
        attribs_ptr as *mut u32); //6
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn DeleteWindowChecked<'r> (c : &'r Connection,
                            glxwindow : Window) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_glx_delete_window_checked(c.get_raw_conn(),
        glxwindow as window); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn DeleteWindow<'r> (c : &'r Connection,
                     glxwindow : Window) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_glx_delete_window(c.get_raw_conn(),
        glxwindow as window); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn SetClientInfoArbChecked<'r> (c : &'r Connection,
                                major_version : u32,
                                minor_version : u32,
                                gl_versions : &[u32],
                                gl_extension_string : &str,
                                glx_extension_string : &str) -> base::VoidCookie<'r> {
  unsafe {
    let gl_versions_len = gl_versions.len();
    let gl_versions_ptr = gl_versions.as_ptr();
    let gl_extension_string = (gl_extension_string).as_bytes();
    let gl_extension_string_len = gl_extension_string.len();
    let gl_extension_string_ptr = gl_extension_string.as_ptr();
    let glx_extension_string = (glx_extension_string).as_bytes();
    let glx_extension_string_len = glx_extension_string.len();
    let glx_extension_string_ptr = glx_extension_string.as_ptr();
    let cookie = xcb_glx_set_client_info_arb_checked(c.get_raw_conn(),
        major_version as u32, //1
        minor_version as u32, //2
        gl_versions_len as u32, //3
        gl_extension_string_len as u32, //4
        glx_extension_string_len as u32, //5
        gl_versions_ptr as *mut u32, //6
        gl_extension_string_ptr as *mut c_char, //7
        glx_extension_string_ptr as *mut c_char); //8
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn SetClientInfoArb<'r> (c : &'r Connection,
                         major_version : u32,
                         minor_version : u32,
                         gl_versions : &[u32],
                         gl_extension_string : &str,
                         glx_extension_string : &str) -> base::VoidCookie<'r> {
  unsafe {
    let gl_versions_len = gl_versions.len();
    let gl_versions_ptr = gl_versions.as_ptr();
    let gl_extension_string = (gl_extension_string).as_bytes();
    let gl_extension_string_len = gl_extension_string.len();
    let gl_extension_string_ptr = gl_extension_string.as_ptr();
    let glx_extension_string = (glx_extension_string).as_bytes();
    let glx_extension_string_len = glx_extension_string.len();
    let glx_extension_string_ptr = glx_extension_string.as_ptr();
    let cookie = xcb_glx_set_client_info_arb(c.get_raw_conn(),
        major_version as u32, //1
        minor_version as u32, //2
        gl_versions_len as u32, //3
        gl_extension_string_len as u32, //4
        glx_extension_string_len as u32, //5
        gl_versions_ptr as *mut u32, //6
        gl_extension_string_ptr as *mut c_char, //7
        glx_extension_string_ptr as *mut c_char); //8
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn CreateContextAttribsArbChecked<'r> (c : &'r Connection,
                                       context : Context,
                                       fbconfig : Fbconfig,
                                       screen : u32,
                                       share_list : Context,
                                       is_direct : u8,
                                       attribs : &[u32]) -> base::VoidCookie<'r> {
  unsafe {
    let attribs_len = attribs.len();
    let attribs_ptr = attribs.as_ptr();
    let cookie = xcb_glx_create_context_attribs_arb_checked(c.get_raw_conn(),
        context as context, //1
        fbconfig as fbconfig, //2
        screen as u32, //3
        share_list as context, //4
        is_direct as u8, //5
        attribs_len as u32, //6
        attribs_ptr as *mut u32); //7
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn CreateContextAttribsArb<'r> (c : &'r Connection,
                                context : Context,
                                fbconfig : Fbconfig,
                                screen : u32,
                                share_list : Context,
                                is_direct : u8,
                                attribs : &[u32]) -> base::VoidCookie<'r> {
  unsafe {
    let attribs_len = attribs.len();
    let attribs_ptr = attribs.as_ptr();
    let cookie = xcb_glx_create_context_attribs_arb(c.get_raw_conn(),
        context as context, //1
        fbconfig as fbconfig, //2
        screen as u32, //3
        share_list as context, //4
        is_direct as u8, //5
        attribs_len as u32, //6
        attribs_ptr as *mut u32); //7
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn SetClientInfo2arbChecked<'r> (c : &'r Connection,
                                 major_version : u32,
                                 minor_version : u32,
                                 gl_versions : &[u32],
                                 gl_extension_string : &str,
                                 glx_extension_string : &str) -> base::VoidCookie<'r> {
  unsafe {
    let gl_versions_len = gl_versions.len();
    let gl_versions_ptr = gl_versions.as_ptr();
    let gl_extension_string = (gl_extension_string).as_bytes();
    let gl_extension_string_len = gl_extension_string.len();
    let gl_extension_string_ptr = gl_extension_string.as_ptr();
    let glx_extension_string = (glx_extension_string).as_bytes();
    let glx_extension_string_len = glx_extension_string.len();
    let glx_extension_string_ptr = glx_extension_string.as_ptr();
    let cookie = xcb_glx_set_client_info_2arb_checked(c.get_raw_conn(),
        major_version as u32, //1
        minor_version as u32, //2
        gl_versions_len as u32, //3
        gl_extension_string_len as u32, //4
        glx_extension_string_len as u32, //5
        gl_versions_ptr as *mut u32, //6
        gl_extension_string_ptr as *mut c_char, //7
        glx_extension_string_ptr as *mut c_char); //8
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn SetClientInfo2arb<'r> (c : &'r Connection,
                          major_version : u32,
                          minor_version : u32,
                          gl_versions : &[u32],
                          gl_extension_string : &str,
                          glx_extension_string : &str) -> base::VoidCookie<'r> {
  unsafe {
    let gl_versions_len = gl_versions.len();
    let gl_versions_ptr = gl_versions.as_ptr();
    let gl_extension_string = (gl_extension_string).as_bytes();
    let gl_extension_string_len = gl_extension_string.len();
    let gl_extension_string_ptr = gl_extension_string.as_ptr();
    let glx_extension_string = (glx_extension_string).as_bytes();
    let glx_extension_string_len = glx_extension_string.len();
    let glx_extension_string_ptr = glx_extension_string.as_ptr();
    let cookie = xcb_glx_set_client_info_2arb(c.get_raw_conn(),
        major_version as u32, //1
        minor_version as u32, //2
        gl_versions_len as u32, //3
        gl_extension_string_len as u32, //4
        glx_extension_string_len as u32, //5
        gl_versions_ptr as *mut u32, //6
        gl_extension_string_ptr as *mut c_char, //7
        glx_extension_string_ptr as *mut c_char); //8
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn NewListChecked<'r> (c : &'r Connection,
                       context_tag : ContextTag,
                       list : u32,
                       mode : u32) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_glx_new_list_checked(c.get_raw_conn(),
        context_tag as context_tag, //1
        list as u32, //2
        mode as u32); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn NewList<'r> (c : &'r Connection,
                context_tag : ContextTag,
                list : u32,
                mode : u32) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_glx_new_list(c.get_raw_conn(),
        context_tag as context_tag, //1
        list as u32, //2
        mode as u32); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn EndListChecked<'r> (c : &'r Connection,
                       context_tag : ContextTag) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_glx_end_list_checked(c.get_raw_conn(),
        context_tag as context_tag); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn EndList<'r> (c : &'r Connection,
                context_tag : ContextTag) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_glx_end_list(c.get_raw_conn(),
        context_tag as context_tag); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn DeleteListsChecked<'r> (c : &'r Connection,
                           context_tag : ContextTag,
                           list : u32,
                           range : i32) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_glx_delete_lists_checked(c.get_raw_conn(),
        context_tag as context_tag, //1
        list as u32, //2
        range as i32); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn DeleteLists<'r> (c : &'r Connection,
                    context_tag : ContextTag,
                    list : u32,
                    range : i32) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_glx_delete_lists(c.get_raw_conn(),
        context_tag as context_tag, //1
        list as u32, //2
        range as i32); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GenLists<'r> (c : &'r Connection,
                 context_tag : ContextTag,
                 range : i32) -> GenListsCookie<'r> {
  unsafe {
    let cookie = xcb_glx_gen_lists(c.get_raw_conn(),
        context_tag as context_tag, //1
        range as i32); //2
    GenListsCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GenListsUnchecked<'r> (c : &'r Connection,
                          context_tag : ContextTag,
                          range : i32) -> GenListsCookie<'r> {
  unsafe {
    let cookie = xcb_glx_gen_lists_unchecked(c.get_raw_conn(),
        context_tag as context_tag, //1
        range as i32); //2
    GenListsCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GenListsReply {
  pub fn ret_val(&mut self) -> u32 {
    unsafe { accessor!(ret_val -> u32, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GenListsCookie<'s>, mk_reply_gen_lists_reply, GenListsReply, xcb_glx_gen_lists_reply);

pub fn FeedbackBufferChecked<'r> (c : &'r Connection,
                              context_tag : ContextTag,
                              size : i32,
                              type_ : i32) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_glx_feedback_buffer_checked(c.get_raw_conn(),
        context_tag as context_tag, //1
        size as i32, //2
        type_ as i32); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn FeedbackBuffer<'r> (c : &'r Connection,
                       context_tag : ContextTag,
                       size : i32,
                       type_ : i32) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_glx_feedback_buffer(c.get_raw_conn(),
        context_tag as context_tag, //1
        size as i32, //2
        type_ as i32); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn SelectBufferChecked<'r> (c : &'r Connection,
                            context_tag : ContextTag,
                            size : i32) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_glx_select_buffer_checked(c.get_raw_conn(),
        context_tag as context_tag, //1
        size as i32); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn SelectBuffer<'r> (c : &'r Connection,
                     context_tag : ContextTag,
                     size : i32) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_glx_select_buffer(c.get_raw_conn(),
        context_tag as context_tag, //1
        size as i32); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub struct RenderModeReply { base:  base::Reply<render_mode_reply> }
fn mk_reply_render_mode_reply(reply:*mut render_mode_reply) -> RenderModeReply { RenderModeReply { base : base::mk_reply(reply) } }
pub fn RenderMode<'r> (c : &'r Connection,
                   context_tag : ContextTag,
                   mode : u32) -> RenderModeCookie<'r> {
  unsafe {
    let cookie = xcb_glx_render_mode(c.get_raw_conn(),
        context_tag as context_tag, //1
        mode as u32); //2
    RenderModeCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn RenderModeUnchecked<'r> (c : &'r Connection,
                            context_tag : ContextTag,
                            mode : u32) -> RenderModeCookie<'r> {
  unsafe {
    let cookie = xcb_glx_render_mode_unchecked(c.get_raw_conn(),
        context_tag as context_tag, //1
        mode as u32); //2
    RenderModeCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl RenderModeReply {
  pub fn ret_val(&mut self) -> u32 {
    unsafe { accessor!(ret_val -> u32, (*self.base.reply)) }
  }

  pub fn new_mode(&mut self) -> u32 {
    unsafe { accessor!(new_mode -> u32, (*self.base.reply)) }
  }

  pub fn data(&mut self) -> Vec<u32> {
    unsafe { accessor!(u32, xcb_glx_render_mode_data_length, xcb_glx_render_mode_data, (*self.base.reply)) }
  }

}
impl_reply_cookie!(RenderModeCookie<'s>, mk_reply_render_mode_reply, RenderModeReply, xcb_glx_render_mode_reply);

pub fn Finish<'r> (c : &'r Connection,
               context_tag : ContextTag) -> FinishCookie<'r> {
  unsafe {
    let cookie = xcb_glx_finish(c.get_raw_conn(),
        context_tag as context_tag); //1
    FinishCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn FinishUnchecked<'r> (c : &'r Connection,
                        context_tag : ContextTag) -> FinishCookie<'r> {
  unsafe {
    let cookie = xcb_glx_finish_unchecked(c.get_raw_conn(),
        context_tag as context_tag); //1
    FinishCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl FinishReply {
}
impl_reply_cookie!(FinishCookie<'s>, mk_reply_finish_reply, FinishReply, xcb_glx_finish_reply);

pub fn PixelStorefChecked<'r> (c : &'r Connection,
                           context_tag : ContextTag,
                           pname : u32,
                           datum : Float32) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_glx_pixel_storef_checked(c.get_raw_conn(),
        context_tag as context_tag, //1
        pname as u32, //2
        datum as float32); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn PixelStoref<'r> (c : &'r Connection,
                    context_tag : ContextTag,
                    pname : u32,
                    datum : Float32) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_glx_pixel_storef(c.get_raw_conn(),
        context_tag as context_tag, //1
        pname as u32, //2
        datum as float32); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn PixelStoreiChecked<'r> (c : &'r Connection,
                           context_tag : ContextTag,
                           pname : u32,
                           datum : i32) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_glx_pixel_storei_checked(c.get_raw_conn(),
        context_tag as context_tag, //1
        pname as u32, //2
        datum as i32); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn PixelStorei<'r> (c : &'r Connection,
                    context_tag : ContextTag,
                    pname : u32,
                    datum : i32) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_glx_pixel_storei(c.get_raw_conn(),
        context_tag as context_tag, //1
        pname as u32, //2
        datum as i32); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub struct ReadPixelsReply { base:  base::Reply<read_pixels_reply> }
fn mk_reply_read_pixels_reply(reply:*mut read_pixels_reply) -> ReadPixelsReply { ReadPixelsReply { base : base::mk_reply(reply) } }
pub fn ReadPixels<'r> (c : &'r Connection,
                   context_tag : ContextTag,
                   x : i32,
                   y : i32,
                   width : i32,
                   height : i32,
                   format : u32,
                   type_ : u32,
                   swap_bytes : u8,
                   lsb_first : u8) -> ReadPixelsCookie<'r> {
  unsafe {
    let cookie = xcb_glx_read_pixels(c.get_raw_conn(),
        context_tag as context_tag, //1
        x as i32, //2
        y as i32, //3
        width as i32, //4
        height as i32, //5
        format as u32, //6
        type_ as u32, //7
        swap_bytes as u8, //8
        lsb_first as u8); //9
    ReadPixelsCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn ReadPixelsUnchecked<'r> (c : &'r Connection,
                            context_tag : ContextTag,
                            x : i32,
                            y : i32,
                            width : i32,
                            height : i32,
                            format : u32,
                            type_ : u32,
                            swap_bytes : u8,
                            lsb_first : u8) -> ReadPixelsCookie<'r> {
  unsafe {
    let cookie = xcb_glx_read_pixels_unchecked(c.get_raw_conn(),
        context_tag as context_tag, //1
        x as i32, //2
        y as i32, //3
        width as i32, //4
        height as i32, //5
        format as u32, //6
        type_ as u32, //7
        swap_bytes as u8, //8
        lsb_first as u8); //9
    ReadPixelsCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl ReadPixelsReply {
  pub fn data(&mut self) -> Vec<u8> {
    unsafe { accessor!(u8, xcb_glx_read_pixels_data_length, xcb_glx_read_pixels_data, (*self.base.reply)) }
  }

}
impl_reply_cookie!(ReadPixelsCookie<'s>, mk_reply_read_pixels_reply, ReadPixelsReply, xcb_glx_read_pixels_reply);

pub struct GetBooleanvReply { base:  base::Reply<get_booleanv_reply> }
fn mk_reply_get_booleanv_reply(reply:*mut get_booleanv_reply) -> GetBooleanvReply { GetBooleanvReply { base : base::mk_reply(reply) } }
pub fn GetBooleanv<'r> (c : &'r Connection,
                    context_tag : ContextTag,
                    pname : i32) -> GetBooleanvCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_booleanv(c.get_raw_conn(),
        context_tag as context_tag, //1
        pname as i32); //2
    GetBooleanvCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetBooleanvUnchecked<'r> (c : &'r Connection,
                             context_tag : ContextTag,
                             pname : i32) -> GetBooleanvCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_booleanv_unchecked(c.get_raw_conn(),
        context_tag as context_tag, //1
        pname as i32); //2
    GetBooleanvCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetBooleanvReply {
  pub fn datum(&mut self) -> u8 {
    unsafe { accessor!(datum -> u8, (*self.base.reply)) }
  }

  pub fn data(&mut self) -> Vec<u8> {
    unsafe { accessor!(u8, xcb_glx_get_booleanv_data_length, xcb_glx_get_booleanv_data, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetBooleanvCookie<'s>, mk_reply_get_booleanv_reply, GetBooleanvReply, xcb_glx_get_booleanv_reply);

pub struct GetClipPlaneReply { base:  base::Reply<get_clip_plane_reply> }
fn mk_reply_get_clip_plane_reply(reply:*mut get_clip_plane_reply) -> GetClipPlaneReply { GetClipPlaneReply { base : base::mk_reply(reply) } }
pub fn GetClipPlane<'r> (c : &'r Connection,
                     context_tag : ContextTag,
                     plane : i32) -> GetClipPlaneCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_clip_plane(c.get_raw_conn(),
        context_tag as context_tag, //1
        plane as i32); //2
    GetClipPlaneCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetClipPlaneUnchecked<'r> (c : &'r Connection,
                              context_tag : ContextTag,
                              plane : i32) -> GetClipPlaneCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_clip_plane_unchecked(c.get_raw_conn(),
        context_tag as context_tag, //1
        plane as i32); //2
    GetClipPlaneCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetClipPlaneReply {
  pub fn data(&mut self) -> Vec<Float64> {
    unsafe { accessor!(Float64, xcb_glx_get_clip_plane_data_length, xcb_glx_get_clip_plane_data, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetClipPlaneCookie<'s>, mk_reply_get_clip_plane_reply, GetClipPlaneReply, xcb_glx_get_clip_plane_reply);

pub struct GetDoublevReply { base:  base::Reply<get_doublev_reply> }
fn mk_reply_get_doublev_reply(reply:*mut get_doublev_reply) -> GetDoublevReply { GetDoublevReply { base : base::mk_reply(reply) } }
pub fn GetDoublev<'r> (c : &'r Connection,
                   context_tag : ContextTag,
                   pname : u32) -> GetDoublevCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_doublev(c.get_raw_conn(),
        context_tag as context_tag, //1
        pname as u32); //2
    GetDoublevCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetDoublevUnchecked<'r> (c : &'r Connection,
                            context_tag : ContextTag,
                            pname : u32) -> GetDoublevCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_doublev_unchecked(c.get_raw_conn(),
        context_tag as context_tag, //1
        pname as u32); //2
    GetDoublevCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetDoublevReply {
  pub fn datum(&mut self) -> Float64 {
    unsafe { accessor!(datum -> Float64, (*self.base.reply)) }
  }

  pub fn data(&mut self) -> Vec<Float64> {
    unsafe { accessor!(Float64, xcb_glx_get_doublev_data_length, xcb_glx_get_doublev_data, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetDoublevCookie<'s>, mk_reply_get_doublev_reply, GetDoublevReply, xcb_glx_get_doublev_reply);

pub fn GetError<'r> (c : &'r Connection,
                 context_tag : ContextTag) -> GetErrorCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_error(c.get_raw_conn(),
        context_tag as context_tag); //1
    GetErrorCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetErrorUnchecked<'r> (c : &'r Connection,
                          context_tag : ContextTag) -> GetErrorCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_error_unchecked(c.get_raw_conn(),
        context_tag as context_tag); //1
    GetErrorCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetErrorReply {
  pub fn error(&mut self) -> i32 {
    unsafe { accessor!(error -> i32, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetErrorCookie<'s>, mk_reply_get_error_reply, GetErrorReply, xcb_glx_get_error_reply);

pub struct GetFloatvReply { base:  base::Reply<get_floatv_reply> }
fn mk_reply_get_floatv_reply(reply:*mut get_floatv_reply) -> GetFloatvReply { GetFloatvReply { base : base::mk_reply(reply) } }
pub fn GetFloatv<'r> (c : &'r Connection,
                  context_tag : ContextTag,
                  pname : u32) -> GetFloatvCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_floatv(c.get_raw_conn(),
        context_tag as context_tag, //1
        pname as u32); //2
    GetFloatvCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetFloatvUnchecked<'r> (c : &'r Connection,
                           context_tag : ContextTag,
                           pname : u32) -> GetFloatvCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_floatv_unchecked(c.get_raw_conn(),
        context_tag as context_tag, //1
        pname as u32); //2
    GetFloatvCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetFloatvReply {
  pub fn datum(&mut self) -> Float32 {
    unsafe { accessor!(datum -> Float32, (*self.base.reply)) }
  }

  pub fn data(&mut self) -> Vec<Float32> {
    unsafe { accessor!(Float32, xcb_glx_get_floatv_data_length, xcb_glx_get_floatv_data, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetFloatvCookie<'s>, mk_reply_get_floatv_reply, GetFloatvReply, xcb_glx_get_floatv_reply);

pub struct GetIntegervReply { base:  base::Reply<get_integerv_reply> }
fn mk_reply_get_integerv_reply(reply:*mut get_integerv_reply) -> GetIntegervReply { GetIntegervReply { base : base::mk_reply(reply) } }
pub fn GetIntegerv<'r> (c : &'r Connection,
                    context_tag : ContextTag,
                    pname : u32) -> GetIntegervCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_integerv(c.get_raw_conn(),
        context_tag as context_tag, //1
        pname as u32); //2
    GetIntegervCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetIntegervUnchecked<'r> (c : &'r Connection,
                             context_tag : ContextTag,
                             pname : u32) -> GetIntegervCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_integerv_unchecked(c.get_raw_conn(),
        context_tag as context_tag, //1
        pname as u32); //2
    GetIntegervCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetIntegervReply {
  pub fn datum(&mut self) -> i32 {
    unsafe { accessor!(datum -> i32, (*self.base.reply)) }
  }

  pub fn data(&mut self) -> Vec<i32> {
    unsafe { accessor!(i32, xcb_glx_get_integerv_data_length, xcb_glx_get_integerv_data, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetIntegervCookie<'s>, mk_reply_get_integerv_reply, GetIntegervReply, xcb_glx_get_integerv_reply);

pub struct GetLightfvReply { base:  base::Reply<get_lightfv_reply> }
fn mk_reply_get_lightfv_reply(reply:*mut get_lightfv_reply) -> GetLightfvReply { GetLightfvReply { base : base::mk_reply(reply) } }
pub fn GetLightfv<'r> (c : &'r Connection,
                   context_tag : ContextTag,
                   light : u32,
                   pname : u32) -> GetLightfvCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_lightfv(c.get_raw_conn(),
        context_tag as context_tag, //1
        light as u32, //2
        pname as u32); //3
    GetLightfvCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetLightfvUnchecked<'r> (c : &'r Connection,
                            context_tag : ContextTag,
                            light : u32,
                            pname : u32) -> GetLightfvCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_lightfv_unchecked(c.get_raw_conn(),
        context_tag as context_tag, //1
        light as u32, //2
        pname as u32); //3
    GetLightfvCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetLightfvReply {
  pub fn datum(&mut self) -> Float32 {
    unsafe { accessor!(datum -> Float32, (*self.base.reply)) }
  }

  pub fn data(&mut self) -> Vec<Float32> {
    unsafe { accessor!(Float32, xcb_glx_get_lightfv_data_length, xcb_glx_get_lightfv_data, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetLightfvCookie<'s>, mk_reply_get_lightfv_reply, GetLightfvReply, xcb_glx_get_lightfv_reply);

pub struct GetLightivReply { base:  base::Reply<get_lightiv_reply> }
fn mk_reply_get_lightiv_reply(reply:*mut get_lightiv_reply) -> GetLightivReply { GetLightivReply { base : base::mk_reply(reply) } }
pub fn GetLightiv<'r> (c : &'r Connection,
                   context_tag : ContextTag,
                   light : u32,
                   pname : u32) -> GetLightivCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_lightiv(c.get_raw_conn(),
        context_tag as context_tag, //1
        light as u32, //2
        pname as u32); //3
    GetLightivCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetLightivUnchecked<'r> (c : &'r Connection,
                            context_tag : ContextTag,
                            light : u32,
                            pname : u32) -> GetLightivCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_lightiv_unchecked(c.get_raw_conn(),
        context_tag as context_tag, //1
        light as u32, //2
        pname as u32); //3
    GetLightivCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetLightivReply {
  pub fn datum(&mut self) -> i32 {
    unsafe { accessor!(datum -> i32, (*self.base.reply)) }
  }

  pub fn data(&mut self) -> Vec<i32> {
    unsafe { accessor!(i32, xcb_glx_get_lightiv_data_length, xcb_glx_get_lightiv_data, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetLightivCookie<'s>, mk_reply_get_lightiv_reply, GetLightivReply, xcb_glx_get_lightiv_reply);

pub struct GetMapdvReply { base:  base::Reply<get_mapdv_reply> }
fn mk_reply_get_mapdv_reply(reply:*mut get_mapdv_reply) -> GetMapdvReply { GetMapdvReply { base : base::mk_reply(reply) } }
pub fn GetMapdv<'r> (c : &'r Connection,
                 context_tag : ContextTag,
                 target : u32,
                 query : u32) -> GetMapdvCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_mapdv(c.get_raw_conn(),
        context_tag as context_tag, //1
        target as u32, //2
        query as u32); //3
    GetMapdvCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetMapdvUnchecked<'r> (c : &'r Connection,
                          context_tag : ContextTag,
                          target : u32,
                          query : u32) -> GetMapdvCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_mapdv_unchecked(c.get_raw_conn(),
        context_tag as context_tag, //1
        target as u32, //2
        query as u32); //3
    GetMapdvCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetMapdvReply {
  pub fn datum(&mut self) -> Float64 {
    unsafe { accessor!(datum -> Float64, (*self.base.reply)) }
  }

  pub fn data(&mut self) -> Vec<Float64> {
    unsafe { accessor!(Float64, xcb_glx_get_mapdv_data_length, xcb_glx_get_mapdv_data, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetMapdvCookie<'s>, mk_reply_get_mapdv_reply, GetMapdvReply, xcb_glx_get_mapdv_reply);

pub struct GetMapfvReply { base:  base::Reply<get_mapfv_reply> }
fn mk_reply_get_mapfv_reply(reply:*mut get_mapfv_reply) -> GetMapfvReply { GetMapfvReply { base : base::mk_reply(reply) } }
pub fn GetMapfv<'r> (c : &'r Connection,
                 context_tag : ContextTag,
                 target : u32,
                 query : u32) -> GetMapfvCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_mapfv(c.get_raw_conn(),
        context_tag as context_tag, //1
        target as u32, //2
        query as u32); //3
    GetMapfvCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetMapfvUnchecked<'r> (c : &'r Connection,
                          context_tag : ContextTag,
                          target : u32,
                          query : u32) -> GetMapfvCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_mapfv_unchecked(c.get_raw_conn(),
        context_tag as context_tag, //1
        target as u32, //2
        query as u32); //3
    GetMapfvCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetMapfvReply {
  pub fn datum(&mut self) -> Float32 {
    unsafe { accessor!(datum -> Float32, (*self.base.reply)) }
  }

  pub fn data(&mut self) -> Vec<Float32> {
    unsafe { accessor!(Float32, xcb_glx_get_mapfv_data_length, xcb_glx_get_mapfv_data, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetMapfvCookie<'s>, mk_reply_get_mapfv_reply, GetMapfvReply, xcb_glx_get_mapfv_reply);

pub struct GetMapivReply { base:  base::Reply<get_mapiv_reply> }
fn mk_reply_get_mapiv_reply(reply:*mut get_mapiv_reply) -> GetMapivReply { GetMapivReply { base : base::mk_reply(reply) } }
pub fn GetMapiv<'r> (c : &'r Connection,
                 context_tag : ContextTag,
                 target : u32,
                 query : u32) -> GetMapivCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_mapiv(c.get_raw_conn(),
        context_tag as context_tag, //1
        target as u32, //2
        query as u32); //3
    GetMapivCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetMapivUnchecked<'r> (c : &'r Connection,
                          context_tag : ContextTag,
                          target : u32,
                          query : u32) -> GetMapivCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_mapiv_unchecked(c.get_raw_conn(),
        context_tag as context_tag, //1
        target as u32, //2
        query as u32); //3
    GetMapivCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetMapivReply {
  pub fn datum(&mut self) -> i32 {
    unsafe { accessor!(datum -> i32, (*self.base.reply)) }
  }

  pub fn data(&mut self) -> Vec<i32> {
    unsafe { accessor!(i32, xcb_glx_get_mapiv_data_length, xcb_glx_get_mapiv_data, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetMapivCookie<'s>, mk_reply_get_mapiv_reply, GetMapivReply, xcb_glx_get_mapiv_reply);

pub struct GetMaterialfvReply { base:  base::Reply<get_materialfv_reply> }
fn mk_reply_get_materialfv_reply(reply:*mut get_materialfv_reply) -> GetMaterialfvReply { GetMaterialfvReply { base : base::mk_reply(reply) } }
pub fn GetMaterialfv<'r> (c : &'r Connection,
                      context_tag : ContextTag,
                      face : u32,
                      pname : u32) -> GetMaterialfvCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_materialfv(c.get_raw_conn(),
        context_tag as context_tag, //1
        face as u32, //2
        pname as u32); //3
    GetMaterialfvCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetMaterialfvUnchecked<'r> (c : &'r Connection,
                               context_tag : ContextTag,
                               face : u32,
                               pname : u32) -> GetMaterialfvCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_materialfv_unchecked(c.get_raw_conn(),
        context_tag as context_tag, //1
        face as u32, //2
        pname as u32); //3
    GetMaterialfvCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetMaterialfvReply {
  pub fn datum(&mut self) -> Float32 {
    unsafe { accessor!(datum -> Float32, (*self.base.reply)) }
  }

  pub fn data(&mut self) -> Vec<Float32> {
    unsafe { accessor!(Float32, xcb_glx_get_materialfv_data_length, xcb_glx_get_materialfv_data, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetMaterialfvCookie<'s>, mk_reply_get_materialfv_reply, GetMaterialfvReply, xcb_glx_get_materialfv_reply);

pub struct GetMaterialivReply { base:  base::Reply<get_materialiv_reply> }
fn mk_reply_get_materialiv_reply(reply:*mut get_materialiv_reply) -> GetMaterialivReply { GetMaterialivReply { base : base::mk_reply(reply) } }
pub fn GetMaterialiv<'r> (c : &'r Connection,
                      context_tag : ContextTag,
                      face : u32,
                      pname : u32) -> GetMaterialivCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_materialiv(c.get_raw_conn(),
        context_tag as context_tag, //1
        face as u32, //2
        pname as u32); //3
    GetMaterialivCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetMaterialivUnchecked<'r> (c : &'r Connection,
                               context_tag : ContextTag,
                               face : u32,
                               pname : u32) -> GetMaterialivCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_materialiv_unchecked(c.get_raw_conn(),
        context_tag as context_tag, //1
        face as u32, //2
        pname as u32); //3
    GetMaterialivCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetMaterialivReply {
  pub fn datum(&mut self) -> i32 {
    unsafe { accessor!(datum -> i32, (*self.base.reply)) }
  }

  pub fn data(&mut self) -> Vec<i32> {
    unsafe { accessor!(i32, xcb_glx_get_materialiv_data_length, xcb_glx_get_materialiv_data, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetMaterialivCookie<'s>, mk_reply_get_materialiv_reply, GetMaterialivReply, xcb_glx_get_materialiv_reply);

pub struct GetPixelMapfvReply { base:  base::Reply<get_pixel_mapfv_reply> }
fn mk_reply_get_pixel_mapfv_reply(reply:*mut get_pixel_mapfv_reply) -> GetPixelMapfvReply { GetPixelMapfvReply { base : base::mk_reply(reply) } }
pub fn GetPixelMapfv<'r> (c : &'r Connection,
                      context_tag : ContextTag,
                      map : u32) -> GetPixelMapfvCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_pixel_mapfv(c.get_raw_conn(),
        context_tag as context_tag, //1
        map as u32); //2
    GetPixelMapfvCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetPixelMapfvUnchecked<'r> (c : &'r Connection,
                               context_tag : ContextTag,
                               map : u32) -> GetPixelMapfvCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_pixel_mapfv_unchecked(c.get_raw_conn(),
        context_tag as context_tag, //1
        map as u32); //2
    GetPixelMapfvCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetPixelMapfvReply {
  pub fn datum(&mut self) -> Float32 {
    unsafe { accessor!(datum -> Float32, (*self.base.reply)) }
  }

  pub fn data(&mut self) -> Vec<Float32> {
    unsafe { accessor!(Float32, xcb_glx_get_pixel_mapfv_data_length, xcb_glx_get_pixel_mapfv_data, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetPixelMapfvCookie<'s>, mk_reply_get_pixel_mapfv_reply, GetPixelMapfvReply, xcb_glx_get_pixel_mapfv_reply);

pub struct GetPixelMapuivReply { base:  base::Reply<get_pixel_mapuiv_reply> }
fn mk_reply_get_pixel_mapuiv_reply(reply:*mut get_pixel_mapuiv_reply) -> GetPixelMapuivReply { GetPixelMapuivReply { base : base::mk_reply(reply) } }
pub fn GetPixelMapuiv<'r> (c : &'r Connection,
                       context_tag : ContextTag,
                       map : u32) -> GetPixelMapuivCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_pixel_mapuiv(c.get_raw_conn(),
        context_tag as context_tag, //1
        map as u32); //2
    GetPixelMapuivCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetPixelMapuivUnchecked<'r> (c : &'r Connection,
                                context_tag : ContextTag,
                                map : u32) -> GetPixelMapuivCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_pixel_mapuiv_unchecked(c.get_raw_conn(),
        context_tag as context_tag, //1
        map as u32); //2
    GetPixelMapuivCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetPixelMapuivReply {
  pub fn datum(&mut self) -> u32 {
    unsafe { accessor!(datum -> u32, (*self.base.reply)) }
  }

  pub fn data(&mut self) -> Vec<u32> {
    unsafe { accessor!(u32, xcb_glx_get_pixel_mapuiv_data_length, xcb_glx_get_pixel_mapuiv_data, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetPixelMapuivCookie<'s>, mk_reply_get_pixel_mapuiv_reply, GetPixelMapuivReply, xcb_glx_get_pixel_mapuiv_reply);

pub struct GetPixelMapusvReply { base:  base::Reply<get_pixel_mapusv_reply> }
fn mk_reply_get_pixel_mapusv_reply(reply:*mut get_pixel_mapusv_reply) -> GetPixelMapusvReply { GetPixelMapusvReply { base : base::mk_reply(reply) } }
pub fn GetPixelMapusv<'r> (c : &'r Connection,
                       context_tag : ContextTag,
                       map : u32) -> GetPixelMapusvCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_pixel_mapusv(c.get_raw_conn(),
        context_tag as context_tag, //1
        map as u32); //2
    GetPixelMapusvCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetPixelMapusvUnchecked<'r> (c : &'r Connection,
                                context_tag : ContextTag,
                                map : u32) -> GetPixelMapusvCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_pixel_mapusv_unchecked(c.get_raw_conn(),
        context_tag as context_tag, //1
        map as u32); //2
    GetPixelMapusvCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetPixelMapusvReply {
  pub fn datum(&mut self) -> u16 {
    unsafe { accessor!(datum -> u16, (*self.base.reply)) }
  }

  pub fn data(&mut self) -> Vec<u16> {
    unsafe { accessor!(u16, xcb_glx_get_pixel_mapusv_data_length, xcb_glx_get_pixel_mapusv_data, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetPixelMapusvCookie<'s>, mk_reply_get_pixel_mapusv_reply, GetPixelMapusvReply, xcb_glx_get_pixel_mapusv_reply);

pub struct GetPolygonStippleReply { base:  base::Reply<get_polygon_stipple_reply> }
fn mk_reply_get_polygon_stipple_reply(reply:*mut get_polygon_stipple_reply) -> GetPolygonStippleReply { GetPolygonStippleReply { base : base::mk_reply(reply) } }
pub fn GetPolygonStipple<'r> (c : &'r Connection,
                          context_tag : ContextTag,
                          lsb_first : u8) -> GetPolygonStippleCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_polygon_stipple(c.get_raw_conn(),
        context_tag as context_tag, //1
        lsb_first as u8); //2
    GetPolygonStippleCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetPolygonStippleUnchecked<'r> (c : &'r Connection,
                                   context_tag : ContextTag,
                                   lsb_first : u8) -> GetPolygonStippleCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_polygon_stipple_unchecked(c.get_raw_conn(),
        context_tag as context_tag, //1
        lsb_first as u8); //2
    GetPolygonStippleCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetPolygonStippleReply {
  pub fn data(&mut self) -> Vec<u8> {
    unsafe { accessor!(u8, xcb_glx_get_polygon_stipple_data_length, xcb_glx_get_polygon_stipple_data, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetPolygonStippleCookie<'s>, mk_reply_get_polygon_stipple_reply, GetPolygonStippleReply, xcb_glx_get_polygon_stipple_reply);

pub struct GetStringReply { base:  base::Reply<get_string_reply> }
fn mk_reply_get_string_reply(reply:*mut get_string_reply) -> GetStringReply { GetStringReply { base : base::mk_reply(reply) } }
pub fn GetString<'r> (c : &'r Connection,
                  context_tag : ContextTag,
                  name : u32) -> GetStringCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_string(c.get_raw_conn(),
        context_tag as context_tag, //1
        name as u32); //2
    GetStringCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetStringUnchecked<'r> (c : &'r Connection,
                           context_tag : ContextTag,
                           name : u32) -> GetStringCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_string_unchecked(c.get_raw_conn(),
        context_tag as context_tag, //1
        name as u32); //2
    GetStringCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetStringReply {
  pub fn string(&mut self) -> String {
    unsafe { accessor!(str, xcb_glx_get_string_string_length, xcb_glx_get_string_string, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetStringCookie<'s>, mk_reply_get_string_reply, GetStringReply, xcb_glx_get_string_reply);

pub struct GetTexEnvfvReply { base:  base::Reply<get_tex_envfv_reply> }
fn mk_reply_get_tex_envfv_reply(reply:*mut get_tex_envfv_reply) -> GetTexEnvfvReply { GetTexEnvfvReply { base : base::mk_reply(reply) } }
pub fn GetTexEnvfv<'r> (c : &'r Connection,
                    context_tag : ContextTag,
                    target : u32,
                    pname : u32) -> GetTexEnvfvCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_tex_envfv(c.get_raw_conn(),
        context_tag as context_tag, //1
        target as u32, //2
        pname as u32); //3
    GetTexEnvfvCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetTexEnvfvUnchecked<'r> (c : &'r Connection,
                             context_tag : ContextTag,
                             target : u32,
                             pname : u32) -> GetTexEnvfvCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_tex_envfv_unchecked(c.get_raw_conn(),
        context_tag as context_tag, //1
        target as u32, //2
        pname as u32); //3
    GetTexEnvfvCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetTexEnvfvReply {
  pub fn datum(&mut self) -> Float32 {
    unsafe { accessor!(datum -> Float32, (*self.base.reply)) }
  }

  pub fn data(&mut self) -> Vec<Float32> {
    unsafe { accessor!(Float32, xcb_glx_get_tex_envfv_data_length, xcb_glx_get_tex_envfv_data, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetTexEnvfvCookie<'s>, mk_reply_get_tex_envfv_reply, GetTexEnvfvReply, xcb_glx_get_tex_envfv_reply);

pub struct GetTexEnvivReply { base:  base::Reply<get_tex_enviv_reply> }
fn mk_reply_get_tex_enviv_reply(reply:*mut get_tex_enviv_reply) -> GetTexEnvivReply { GetTexEnvivReply { base : base::mk_reply(reply) } }
pub fn GetTexEnviv<'r> (c : &'r Connection,
                    context_tag : ContextTag,
                    target : u32,
                    pname : u32) -> GetTexEnvivCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_tex_enviv(c.get_raw_conn(),
        context_tag as context_tag, //1
        target as u32, //2
        pname as u32); //3
    GetTexEnvivCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetTexEnvivUnchecked<'r> (c : &'r Connection,
                             context_tag : ContextTag,
                             target : u32,
                             pname : u32) -> GetTexEnvivCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_tex_enviv_unchecked(c.get_raw_conn(),
        context_tag as context_tag, //1
        target as u32, //2
        pname as u32); //3
    GetTexEnvivCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetTexEnvivReply {
  pub fn datum(&mut self) -> i32 {
    unsafe { accessor!(datum -> i32, (*self.base.reply)) }
  }

  pub fn data(&mut self) -> Vec<i32> {
    unsafe { accessor!(i32, xcb_glx_get_tex_enviv_data_length, xcb_glx_get_tex_enviv_data, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetTexEnvivCookie<'s>, mk_reply_get_tex_enviv_reply, GetTexEnvivReply, xcb_glx_get_tex_enviv_reply);

pub struct GetTexGendvReply { base:  base::Reply<get_tex_gendv_reply> }
fn mk_reply_get_tex_gendv_reply(reply:*mut get_tex_gendv_reply) -> GetTexGendvReply { GetTexGendvReply { base : base::mk_reply(reply) } }
pub fn GetTexGendv<'r> (c : &'r Connection,
                    context_tag : ContextTag,
                    coord : u32,
                    pname : u32) -> GetTexGendvCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_tex_gendv(c.get_raw_conn(),
        context_tag as context_tag, //1
        coord as u32, //2
        pname as u32); //3
    GetTexGendvCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetTexGendvUnchecked<'r> (c : &'r Connection,
                             context_tag : ContextTag,
                             coord : u32,
                             pname : u32) -> GetTexGendvCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_tex_gendv_unchecked(c.get_raw_conn(),
        context_tag as context_tag, //1
        coord as u32, //2
        pname as u32); //3
    GetTexGendvCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetTexGendvReply {
  pub fn datum(&mut self) -> Float64 {
    unsafe { accessor!(datum -> Float64, (*self.base.reply)) }
  }

  pub fn data(&mut self) -> Vec<Float64> {
    unsafe { accessor!(Float64, xcb_glx_get_tex_gendv_data_length, xcb_glx_get_tex_gendv_data, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetTexGendvCookie<'s>, mk_reply_get_tex_gendv_reply, GetTexGendvReply, xcb_glx_get_tex_gendv_reply);

pub struct GetTexGenfvReply { base:  base::Reply<get_tex_genfv_reply> }
fn mk_reply_get_tex_genfv_reply(reply:*mut get_tex_genfv_reply) -> GetTexGenfvReply { GetTexGenfvReply { base : base::mk_reply(reply) } }
pub fn GetTexGenfv<'r> (c : &'r Connection,
                    context_tag : ContextTag,
                    coord : u32,
                    pname : u32) -> GetTexGenfvCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_tex_genfv(c.get_raw_conn(),
        context_tag as context_tag, //1
        coord as u32, //2
        pname as u32); //3
    GetTexGenfvCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetTexGenfvUnchecked<'r> (c : &'r Connection,
                             context_tag : ContextTag,
                             coord : u32,
                             pname : u32) -> GetTexGenfvCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_tex_genfv_unchecked(c.get_raw_conn(),
        context_tag as context_tag, //1
        coord as u32, //2
        pname as u32); //3
    GetTexGenfvCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetTexGenfvReply {
  pub fn datum(&mut self) -> Float32 {
    unsafe { accessor!(datum -> Float32, (*self.base.reply)) }
  }

  pub fn data(&mut self) -> Vec<Float32> {
    unsafe { accessor!(Float32, xcb_glx_get_tex_genfv_data_length, xcb_glx_get_tex_genfv_data, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetTexGenfvCookie<'s>, mk_reply_get_tex_genfv_reply, GetTexGenfvReply, xcb_glx_get_tex_genfv_reply);

pub struct GetTexGenivReply { base:  base::Reply<get_tex_geniv_reply> }
fn mk_reply_get_tex_geniv_reply(reply:*mut get_tex_geniv_reply) -> GetTexGenivReply { GetTexGenivReply { base : base::mk_reply(reply) } }
pub fn GetTexGeniv<'r> (c : &'r Connection,
                    context_tag : ContextTag,
                    coord : u32,
                    pname : u32) -> GetTexGenivCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_tex_geniv(c.get_raw_conn(),
        context_tag as context_tag, //1
        coord as u32, //2
        pname as u32); //3
    GetTexGenivCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetTexGenivUnchecked<'r> (c : &'r Connection,
                             context_tag : ContextTag,
                             coord : u32,
                             pname : u32) -> GetTexGenivCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_tex_geniv_unchecked(c.get_raw_conn(),
        context_tag as context_tag, //1
        coord as u32, //2
        pname as u32); //3
    GetTexGenivCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetTexGenivReply {
  pub fn datum(&mut self) -> i32 {
    unsafe { accessor!(datum -> i32, (*self.base.reply)) }
  }

  pub fn data(&mut self) -> Vec<i32> {
    unsafe { accessor!(i32, xcb_glx_get_tex_geniv_data_length, xcb_glx_get_tex_geniv_data, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetTexGenivCookie<'s>, mk_reply_get_tex_geniv_reply, GetTexGenivReply, xcb_glx_get_tex_geniv_reply);

pub struct GetTexImageReply { base:  base::Reply<get_tex_image_reply> }
fn mk_reply_get_tex_image_reply(reply:*mut get_tex_image_reply) -> GetTexImageReply { GetTexImageReply { base : base::mk_reply(reply) } }
pub fn GetTexImage<'r> (c : &'r Connection,
                    context_tag : ContextTag,
                    target : u32,
                    level : i32,
                    format : u32,
                    type_ : u32,
                    swap_bytes : u8) -> GetTexImageCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_tex_image(c.get_raw_conn(),
        context_tag as context_tag, //1
        target as u32, //2
        level as i32, //3
        format as u32, //4
        type_ as u32, //5
        swap_bytes as u8); //6
    GetTexImageCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetTexImageUnchecked<'r> (c : &'r Connection,
                             context_tag : ContextTag,
                             target : u32,
                             level : i32,
                             format : u32,
                             type_ : u32,
                             swap_bytes : u8) -> GetTexImageCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_tex_image_unchecked(c.get_raw_conn(),
        context_tag as context_tag, //1
        target as u32, //2
        level as i32, //3
        format as u32, //4
        type_ as u32, //5
        swap_bytes as u8); //6
    GetTexImageCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetTexImageReply {
  pub fn width(&mut self) -> i32 {
    unsafe { accessor!(width -> i32, (*self.base.reply)) }
  }

  pub fn height(&mut self) -> i32 {
    unsafe { accessor!(height -> i32, (*self.base.reply)) }
  }

  pub fn depth(&mut self) -> i32 {
    unsafe { accessor!(depth -> i32, (*self.base.reply)) }
  }

  pub fn data(&mut self) -> Vec<u8> {
    unsafe { accessor!(u8, xcb_glx_get_tex_image_data_length, xcb_glx_get_tex_image_data, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetTexImageCookie<'s>, mk_reply_get_tex_image_reply, GetTexImageReply, xcb_glx_get_tex_image_reply);

pub struct GetTexParameterfvReply { base:  base::Reply<get_tex_parameterfv_reply> }
fn mk_reply_get_tex_parameterfv_reply(reply:*mut get_tex_parameterfv_reply) -> GetTexParameterfvReply { GetTexParameterfvReply { base : base::mk_reply(reply) } }
pub fn GetTexParameterfv<'r> (c : &'r Connection,
                          context_tag : ContextTag,
                          target : u32,
                          pname : u32) -> GetTexParameterfvCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_tex_parameterfv(c.get_raw_conn(),
        context_tag as context_tag, //1
        target as u32, //2
        pname as u32); //3
    GetTexParameterfvCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetTexParameterfvUnchecked<'r> (c : &'r Connection,
                                   context_tag : ContextTag,
                                   target : u32,
                                   pname : u32) -> GetTexParameterfvCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_tex_parameterfv_unchecked(c.get_raw_conn(),
        context_tag as context_tag, //1
        target as u32, //2
        pname as u32); //3
    GetTexParameterfvCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetTexParameterfvReply {
  pub fn datum(&mut self) -> Float32 {
    unsafe { accessor!(datum -> Float32, (*self.base.reply)) }
  }

  pub fn data(&mut self) -> Vec<Float32> {
    unsafe { accessor!(Float32, xcb_glx_get_tex_parameterfv_data_length, xcb_glx_get_tex_parameterfv_data, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetTexParameterfvCookie<'s>, mk_reply_get_tex_parameterfv_reply, GetTexParameterfvReply, xcb_glx_get_tex_parameterfv_reply);

pub struct GetTexParameterivReply { base:  base::Reply<get_tex_parameteriv_reply> }
fn mk_reply_get_tex_parameteriv_reply(reply:*mut get_tex_parameteriv_reply) -> GetTexParameterivReply { GetTexParameterivReply { base : base::mk_reply(reply) } }
pub fn GetTexParameteriv<'r> (c : &'r Connection,
                          context_tag : ContextTag,
                          target : u32,
                          pname : u32) -> GetTexParameterivCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_tex_parameteriv(c.get_raw_conn(),
        context_tag as context_tag, //1
        target as u32, //2
        pname as u32); //3
    GetTexParameterivCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetTexParameterivUnchecked<'r> (c : &'r Connection,
                                   context_tag : ContextTag,
                                   target : u32,
                                   pname : u32) -> GetTexParameterivCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_tex_parameteriv_unchecked(c.get_raw_conn(),
        context_tag as context_tag, //1
        target as u32, //2
        pname as u32); //3
    GetTexParameterivCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetTexParameterivReply {
  pub fn datum(&mut self) -> i32 {
    unsafe { accessor!(datum -> i32, (*self.base.reply)) }
  }

  pub fn data(&mut self) -> Vec<i32> {
    unsafe { accessor!(i32, xcb_glx_get_tex_parameteriv_data_length, xcb_glx_get_tex_parameteriv_data, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetTexParameterivCookie<'s>, mk_reply_get_tex_parameteriv_reply, GetTexParameterivReply, xcb_glx_get_tex_parameteriv_reply);

pub struct GetTexLevelParameterfvReply { base:  base::Reply<get_tex_level_parameterfv_reply> }
fn mk_reply_get_tex_level_parameterfv_reply(reply:*mut get_tex_level_parameterfv_reply) -> GetTexLevelParameterfvReply { GetTexLevelParameterfvReply { base : base::mk_reply(reply) } }
pub fn GetTexLevelParameterfv<'r> (c : &'r Connection,
                               context_tag : ContextTag,
                               target : u32,
                               level : i32,
                               pname : u32) -> GetTexLevelParameterfvCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_tex_level_parameterfv(c.get_raw_conn(),
        context_tag as context_tag, //1
        target as u32, //2
        level as i32, //3
        pname as u32); //4
    GetTexLevelParameterfvCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetTexLevelParameterfvUnchecked<'r> (c : &'r Connection,
                                        context_tag : ContextTag,
                                        target : u32,
                                        level : i32,
                                        pname : u32) -> GetTexLevelParameterfvCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_tex_level_parameterfv_unchecked(c.get_raw_conn(),
        context_tag as context_tag, //1
        target as u32, //2
        level as i32, //3
        pname as u32); //4
    GetTexLevelParameterfvCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetTexLevelParameterfvReply {
  pub fn datum(&mut self) -> Float32 {
    unsafe { accessor!(datum -> Float32, (*self.base.reply)) }
  }

  pub fn data(&mut self) -> Vec<Float32> {
    unsafe { accessor!(Float32, xcb_glx_get_tex_level_parameterfv_data_length, xcb_glx_get_tex_level_parameterfv_data, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetTexLevelParameterfvCookie<'s>, mk_reply_get_tex_level_parameterfv_reply, GetTexLevelParameterfvReply, xcb_glx_get_tex_level_parameterfv_reply);

pub struct GetTexLevelParameterivReply { base:  base::Reply<get_tex_level_parameteriv_reply> }
fn mk_reply_get_tex_level_parameteriv_reply(reply:*mut get_tex_level_parameteriv_reply) -> GetTexLevelParameterivReply { GetTexLevelParameterivReply { base : base::mk_reply(reply) } }
pub fn GetTexLevelParameteriv<'r> (c : &'r Connection,
                               context_tag : ContextTag,
                               target : u32,
                               level : i32,
                               pname : u32) -> GetTexLevelParameterivCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_tex_level_parameteriv(c.get_raw_conn(),
        context_tag as context_tag, //1
        target as u32, //2
        level as i32, //3
        pname as u32); //4
    GetTexLevelParameterivCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetTexLevelParameterivUnchecked<'r> (c : &'r Connection,
                                        context_tag : ContextTag,
                                        target : u32,
                                        level : i32,
                                        pname : u32) -> GetTexLevelParameterivCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_tex_level_parameteriv_unchecked(c.get_raw_conn(),
        context_tag as context_tag, //1
        target as u32, //2
        level as i32, //3
        pname as u32); //4
    GetTexLevelParameterivCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetTexLevelParameterivReply {
  pub fn datum(&mut self) -> i32 {
    unsafe { accessor!(datum -> i32, (*self.base.reply)) }
  }

  pub fn data(&mut self) -> Vec<i32> {
    unsafe { accessor!(i32, xcb_glx_get_tex_level_parameteriv_data_length, xcb_glx_get_tex_level_parameteriv_data, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetTexLevelParameterivCookie<'s>, mk_reply_get_tex_level_parameteriv_reply, GetTexLevelParameterivReply, xcb_glx_get_tex_level_parameteriv_reply);

pub fn IsList<'r> (c : &'r Connection,
               context_tag : ContextTag,
               list : u32) -> IsListCookie<'r> {
  unsafe {
    let cookie = xcb_glx_is_list(c.get_raw_conn(),
        context_tag as context_tag, //1
        list as u32); //2
    IsListCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn IsListUnchecked<'r> (c : &'r Connection,
                        context_tag : ContextTag,
                        list : u32) -> IsListCookie<'r> {
  unsafe {
    let cookie = xcb_glx_is_list_unchecked(c.get_raw_conn(),
        context_tag as context_tag, //1
        list as u32); //2
    IsListCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl IsListReply {
  pub fn ret_val(&mut self) -> Bool32 {
    unsafe { accessor!(ret_val -> Bool32, (*self.base.reply)) }
  }

}
impl_reply_cookie!(IsListCookie<'s>, mk_reply_is_list_reply, IsListReply, xcb_glx_is_list_reply);

pub fn FlushChecked<'r> (c : &'r Connection,
                     context_tag : ContextTag) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_glx_flush_checked(c.get_raw_conn(),
        context_tag as context_tag); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn Flush<'r> (c : &'r Connection,
              context_tag : ContextTag) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_glx_flush(c.get_raw_conn(),
        context_tag as context_tag); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn AreTexturesResident<'r> (c : &'r Connection,
                            context_tag : ContextTag,
                            textures : &[u32]) -> AreTexturesResidentCookie<'r> {
  unsafe {
    let textures_len = textures.len();
    let textures_ptr = textures.as_ptr();
    let cookie = xcb_glx_are_textures_resident(c.get_raw_conn(),
        context_tag as context_tag, //1
        textures_len as i32, //2
        textures_ptr as *mut u32); //3
    AreTexturesResidentCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn AreTexturesResidentUnchecked<'r> (c : &'r Connection,
                                     context_tag : ContextTag,
                                     textures : &[u32]) -> AreTexturesResidentCookie<'r> {
  unsafe {
    let textures_len = textures.len();
    let textures_ptr = textures.as_ptr();
    let cookie = xcb_glx_are_textures_resident_unchecked(c.get_raw_conn(),
        context_tag as context_tag, //1
        textures_len as i32, //2
        textures_ptr as *mut u32); //3
    AreTexturesResidentCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl AreTexturesResidentReply {
  pub fn ret_val(&mut self) -> Bool32 {
    unsafe { accessor!(ret_val -> Bool32, (*self.base.reply)) }
  }

  pub fn data(&mut self) -> Vec<u8> {
    unsafe { accessor!(u8, xcb_glx_are_textures_resident_data_length, xcb_glx_are_textures_resident_data, (*self.base.reply)) }
  }

}
impl_reply_cookie!(AreTexturesResidentCookie<'s>, mk_reply_are_textures_resident_reply, AreTexturesResidentReply, xcb_glx_are_textures_resident_reply);

pub fn DeleteTexturesChecked<'r> (c : &'r Connection,
                              context_tag : ContextTag,
                              textures : &[u32]) -> base::VoidCookie<'r> {
  unsafe {
    let textures_len = textures.len();
    let textures_ptr = textures.as_ptr();
    let cookie = xcb_glx_delete_textures_checked(c.get_raw_conn(),
        context_tag as context_tag, //1
        textures_len as i32, //2
        textures_ptr as *mut u32); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn DeleteTextures<'r> (c : &'r Connection,
                       context_tag : ContextTag,
                       textures : &[u32]) -> base::VoidCookie<'r> {
  unsafe {
    let textures_len = textures.len();
    let textures_ptr = textures.as_ptr();
    let cookie = xcb_glx_delete_textures(c.get_raw_conn(),
        context_tag as context_tag, //1
        textures_len as i32, //2
        textures_ptr as *mut u32); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub struct GenTexturesReply { base:  base::Reply<gen_textures_reply> }
fn mk_reply_gen_textures_reply(reply:*mut gen_textures_reply) -> GenTexturesReply { GenTexturesReply { base : base::mk_reply(reply) } }
pub fn GenTextures<'r> (c : &'r Connection,
                    context_tag : ContextTag,
                    n : i32) -> GenTexturesCookie<'r> {
  unsafe {
    let cookie = xcb_glx_gen_textures(c.get_raw_conn(),
        context_tag as context_tag, //1
        n as i32); //2
    GenTexturesCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GenTexturesUnchecked<'r> (c : &'r Connection,
                             context_tag : ContextTag,
                             n : i32) -> GenTexturesCookie<'r> {
  unsafe {
    let cookie = xcb_glx_gen_textures_unchecked(c.get_raw_conn(),
        context_tag as context_tag, //1
        n as i32); //2
    GenTexturesCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GenTexturesReply {
  pub fn data(&mut self) -> Vec<u32> {
    unsafe { accessor!(u32, xcb_glx_gen_textures_data_length, xcb_glx_gen_textures_data, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GenTexturesCookie<'s>, mk_reply_gen_textures_reply, GenTexturesReply, xcb_glx_gen_textures_reply);

pub fn IsTexture<'r> (c : &'r Connection,
                  context_tag : ContextTag,
                  texture : u32) -> IsTextureCookie<'r> {
  unsafe {
    let cookie = xcb_glx_is_texture(c.get_raw_conn(),
        context_tag as context_tag, //1
        texture as u32); //2
    IsTextureCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn IsTextureUnchecked<'r> (c : &'r Connection,
                           context_tag : ContextTag,
                           texture : u32) -> IsTextureCookie<'r> {
  unsafe {
    let cookie = xcb_glx_is_texture_unchecked(c.get_raw_conn(),
        context_tag as context_tag, //1
        texture as u32); //2
    IsTextureCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl IsTextureReply {
  pub fn ret_val(&mut self) -> Bool32 {
    unsafe { accessor!(ret_val -> Bool32, (*self.base.reply)) }
  }

}
impl_reply_cookie!(IsTextureCookie<'s>, mk_reply_is_texture_reply, IsTextureReply, xcb_glx_is_texture_reply);

pub struct GetColorTableReply { base:  base::Reply<get_color_table_reply> }
fn mk_reply_get_color_table_reply(reply:*mut get_color_table_reply) -> GetColorTableReply { GetColorTableReply { base : base::mk_reply(reply) } }
pub fn GetColorTable<'r> (c : &'r Connection,
                      context_tag : ContextTag,
                      target : u32,
                      format : u32,
                      type_ : u32,
                      swap_bytes : u8) -> GetColorTableCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_color_table(c.get_raw_conn(),
        context_tag as context_tag, //1
        target as u32, //2
        format as u32, //3
        type_ as u32, //4
        swap_bytes as u8); //5
    GetColorTableCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetColorTableUnchecked<'r> (c : &'r Connection,
                               context_tag : ContextTag,
                               target : u32,
                               format : u32,
                               type_ : u32,
                               swap_bytes : u8) -> GetColorTableCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_color_table_unchecked(c.get_raw_conn(),
        context_tag as context_tag, //1
        target as u32, //2
        format as u32, //3
        type_ as u32, //4
        swap_bytes as u8); //5
    GetColorTableCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetColorTableReply {
  pub fn width(&mut self) -> i32 {
    unsafe { accessor!(width -> i32, (*self.base.reply)) }
  }

  pub fn data(&mut self) -> Vec<u8> {
    unsafe { accessor!(u8, xcb_glx_get_color_table_data_length, xcb_glx_get_color_table_data, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetColorTableCookie<'s>, mk_reply_get_color_table_reply, GetColorTableReply, xcb_glx_get_color_table_reply);

pub struct GetColorTableParameterfvReply { base:  base::Reply<get_color_table_parameterfv_reply> }
fn mk_reply_get_color_table_parameterfv_reply(reply:*mut get_color_table_parameterfv_reply) -> GetColorTableParameterfvReply { GetColorTableParameterfvReply { base : base::mk_reply(reply) } }
pub fn GetColorTableParameterfv<'r> (c : &'r Connection,
                                 context_tag : ContextTag,
                                 target : u32,
                                 pname : u32) -> GetColorTableParameterfvCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_color_table_parameterfv(c.get_raw_conn(),
        context_tag as context_tag, //1
        target as u32, //2
        pname as u32); //3
    GetColorTableParameterfvCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetColorTableParameterfvUnchecked<'r> (c : &'r Connection,
                                          context_tag : ContextTag,
                                          target : u32,
                                          pname : u32) -> GetColorTableParameterfvCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_color_table_parameterfv_unchecked(c.get_raw_conn(),
        context_tag as context_tag, //1
        target as u32, //2
        pname as u32); //3
    GetColorTableParameterfvCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetColorTableParameterfvReply {
  pub fn datum(&mut self) -> Float32 {
    unsafe { accessor!(datum -> Float32, (*self.base.reply)) }
  }

  pub fn data(&mut self) -> Vec<Float32> {
    unsafe { accessor!(Float32, xcb_glx_get_color_table_parameterfv_data_length, xcb_glx_get_color_table_parameterfv_data, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetColorTableParameterfvCookie<'s>, mk_reply_get_color_table_parameterfv_reply, GetColorTableParameterfvReply, xcb_glx_get_color_table_parameterfv_reply);

pub struct GetColorTableParameterivReply { base:  base::Reply<get_color_table_parameteriv_reply> }
fn mk_reply_get_color_table_parameteriv_reply(reply:*mut get_color_table_parameteriv_reply) -> GetColorTableParameterivReply { GetColorTableParameterivReply { base : base::mk_reply(reply) } }
pub fn GetColorTableParameteriv<'r> (c : &'r Connection,
                                 context_tag : ContextTag,
                                 target : u32,
                                 pname : u32) -> GetColorTableParameterivCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_color_table_parameteriv(c.get_raw_conn(),
        context_tag as context_tag, //1
        target as u32, //2
        pname as u32); //3
    GetColorTableParameterivCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetColorTableParameterivUnchecked<'r> (c : &'r Connection,
                                          context_tag : ContextTag,
                                          target : u32,
                                          pname : u32) -> GetColorTableParameterivCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_color_table_parameteriv_unchecked(c.get_raw_conn(),
        context_tag as context_tag, //1
        target as u32, //2
        pname as u32); //3
    GetColorTableParameterivCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetColorTableParameterivReply {
  pub fn datum(&mut self) -> i32 {
    unsafe { accessor!(datum -> i32, (*self.base.reply)) }
  }

  pub fn data(&mut self) -> Vec<i32> {
    unsafe { accessor!(i32, xcb_glx_get_color_table_parameteriv_data_length, xcb_glx_get_color_table_parameteriv_data, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetColorTableParameterivCookie<'s>, mk_reply_get_color_table_parameteriv_reply, GetColorTableParameterivReply, xcb_glx_get_color_table_parameteriv_reply);

pub struct GetConvolutionFilterReply { base:  base::Reply<get_convolution_filter_reply> }
fn mk_reply_get_convolution_filter_reply(reply:*mut get_convolution_filter_reply) -> GetConvolutionFilterReply { GetConvolutionFilterReply { base : base::mk_reply(reply) } }
pub fn GetConvolutionFilter<'r> (c : &'r Connection,
                             context_tag : ContextTag,
                             target : u32,
                             format : u32,
                             type_ : u32,
                             swap_bytes : u8) -> GetConvolutionFilterCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_convolution_filter(c.get_raw_conn(),
        context_tag as context_tag, //1
        target as u32, //2
        format as u32, //3
        type_ as u32, //4
        swap_bytes as u8); //5
    GetConvolutionFilterCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetConvolutionFilterUnchecked<'r> (c : &'r Connection,
                                      context_tag : ContextTag,
                                      target : u32,
                                      format : u32,
                                      type_ : u32,
                                      swap_bytes : u8) -> GetConvolutionFilterCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_convolution_filter_unchecked(c.get_raw_conn(),
        context_tag as context_tag, //1
        target as u32, //2
        format as u32, //3
        type_ as u32, //4
        swap_bytes as u8); //5
    GetConvolutionFilterCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetConvolutionFilterReply {
  pub fn width(&mut self) -> i32 {
    unsafe { accessor!(width -> i32, (*self.base.reply)) }
  }

  pub fn height(&mut self) -> i32 {
    unsafe { accessor!(height -> i32, (*self.base.reply)) }
  }

  pub fn data(&mut self) -> Vec<u8> {
    unsafe { accessor!(u8, xcb_glx_get_convolution_filter_data_length, xcb_glx_get_convolution_filter_data, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetConvolutionFilterCookie<'s>, mk_reply_get_convolution_filter_reply, GetConvolutionFilterReply, xcb_glx_get_convolution_filter_reply);

pub struct GetConvolutionParameterfvReply { base:  base::Reply<get_convolution_parameterfv_reply> }
fn mk_reply_get_convolution_parameterfv_reply(reply:*mut get_convolution_parameterfv_reply) -> GetConvolutionParameterfvReply { GetConvolutionParameterfvReply { base : base::mk_reply(reply) } }
pub fn GetConvolutionParameterfv<'r> (c : &'r Connection,
                                  context_tag : ContextTag,
                                  target : u32,
                                  pname : u32) -> GetConvolutionParameterfvCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_convolution_parameterfv(c.get_raw_conn(),
        context_tag as context_tag, //1
        target as u32, //2
        pname as u32); //3
    GetConvolutionParameterfvCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetConvolutionParameterfvUnchecked<'r> (c : &'r Connection,
                                           context_tag : ContextTag,
                                           target : u32,
                                           pname : u32) -> GetConvolutionParameterfvCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_convolution_parameterfv_unchecked(c.get_raw_conn(),
        context_tag as context_tag, //1
        target as u32, //2
        pname as u32); //3
    GetConvolutionParameterfvCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetConvolutionParameterfvReply {
  pub fn datum(&mut self) -> Float32 {
    unsafe { accessor!(datum -> Float32, (*self.base.reply)) }
  }

  pub fn data(&mut self) -> Vec<Float32> {
    unsafe { accessor!(Float32, xcb_glx_get_convolution_parameterfv_data_length, xcb_glx_get_convolution_parameterfv_data, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetConvolutionParameterfvCookie<'s>, mk_reply_get_convolution_parameterfv_reply, GetConvolutionParameterfvReply, xcb_glx_get_convolution_parameterfv_reply);

pub struct GetConvolutionParameterivReply { base:  base::Reply<get_convolution_parameteriv_reply> }
fn mk_reply_get_convolution_parameteriv_reply(reply:*mut get_convolution_parameteriv_reply) -> GetConvolutionParameterivReply { GetConvolutionParameterivReply { base : base::mk_reply(reply) } }
pub fn GetConvolutionParameteriv<'r> (c : &'r Connection,
                                  context_tag : ContextTag,
                                  target : u32,
                                  pname : u32) -> GetConvolutionParameterivCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_convolution_parameteriv(c.get_raw_conn(),
        context_tag as context_tag, //1
        target as u32, //2
        pname as u32); //3
    GetConvolutionParameterivCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetConvolutionParameterivUnchecked<'r> (c : &'r Connection,
                                           context_tag : ContextTag,
                                           target : u32,
                                           pname : u32) -> GetConvolutionParameterivCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_convolution_parameteriv_unchecked(c.get_raw_conn(),
        context_tag as context_tag, //1
        target as u32, //2
        pname as u32); //3
    GetConvolutionParameterivCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetConvolutionParameterivReply {
  pub fn datum(&mut self) -> i32 {
    unsafe { accessor!(datum -> i32, (*self.base.reply)) }
  }

  pub fn data(&mut self) -> Vec<i32> {
    unsafe { accessor!(i32, xcb_glx_get_convolution_parameteriv_data_length, xcb_glx_get_convolution_parameteriv_data, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetConvolutionParameterivCookie<'s>, mk_reply_get_convolution_parameteriv_reply, GetConvolutionParameterivReply, xcb_glx_get_convolution_parameteriv_reply);

pub struct GetSeparableFilterReply { base:  base::Reply<get_separable_filter_reply> }
fn mk_reply_get_separable_filter_reply(reply:*mut get_separable_filter_reply) -> GetSeparableFilterReply { GetSeparableFilterReply { base : base::mk_reply(reply) } }
pub fn GetSeparableFilter<'r> (c : &'r Connection,
                           context_tag : ContextTag,
                           target : u32,
                           format : u32,
                           type_ : u32,
                           swap_bytes : u8) -> GetSeparableFilterCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_separable_filter(c.get_raw_conn(),
        context_tag as context_tag, //1
        target as u32, //2
        format as u32, //3
        type_ as u32, //4
        swap_bytes as u8); //5
    GetSeparableFilterCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetSeparableFilterUnchecked<'r> (c : &'r Connection,
                                    context_tag : ContextTag,
                                    target : u32,
                                    format : u32,
                                    type_ : u32,
                                    swap_bytes : u8) -> GetSeparableFilterCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_separable_filter_unchecked(c.get_raw_conn(),
        context_tag as context_tag, //1
        target as u32, //2
        format as u32, //3
        type_ as u32, //4
        swap_bytes as u8); //5
    GetSeparableFilterCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetSeparableFilterReply {
  pub fn row_w(&mut self) -> i32 {
    unsafe { accessor!(row_w -> i32, (*self.base.reply)) }
  }

  pub fn col_h(&mut self) -> i32 {
    unsafe { accessor!(col_h -> i32, (*self.base.reply)) }
  }

  pub fn rows_and_cols(&mut self) -> Vec<u8> {
    unsafe { accessor!(u8, xcb_glx_get_separable_filter_rows_and_cols_length, xcb_glx_get_separable_filter_rows_and_cols, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetSeparableFilterCookie<'s>, mk_reply_get_separable_filter_reply, GetSeparableFilterReply, xcb_glx_get_separable_filter_reply);

pub struct GetHistogramReply { base:  base::Reply<get_histogram_reply> }
fn mk_reply_get_histogram_reply(reply:*mut get_histogram_reply) -> GetHistogramReply { GetHistogramReply { base : base::mk_reply(reply) } }
pub fn GetHistogram<'r> (c : &'r Connection,
                     context_tag : ContextTag,
                     target : u32,
                     format : u32,
                     type_ : u32,
                     swap_bytes : u8,
                     reset : u8) -> GetHistogramCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_histogram(c.get_raw_conn(),
        context_tag as context_tag, //1
        target as u32, //2
        format as u32, //3
        type_ as u32, //4
        swap_bytes as u8, //5
        reset as u8); //6
    GetHistogramCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetHistogramUnchecked<'r> (c : &'r Connection,
                              context_tag : ContextTag,
                              target : u32,
                              format : u32,
                              type_ : u32,
                              swap_bytes : u8,
                              reset : u8) -> GetHistogramCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_histogram_unchecked(c.get_raw_conn(),
        context_tag as context_tag, //1
        target as u32, //2
        format as u32, //3
        type_ as u32, //4
        swap_bytes as u8, //5
        reset as u8); //6
    GetHistogramCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetHistogramReply {
  pub fn width(&mut self) -> i32 {
    unsafe { accessor!(width -> i32, (*self.base.reply)) }
  }

  pub fn data(&mut self) -> Vec<u8> {
    unsafe { accessor!(u8, xcb_glx_get_histogram_data_length, xcb_glx_get_histogram_data, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetHistogramCookie<'s>, mk_reply_get_histogram_reply, GetHistogramReply, xcb_glx_get_histogram_reply);

pub struct GetHistogramParameterfvReply { base:  base::Reply<get_histogram_parameterfv_reply> }
fn mk_reply_get_histogram_parameterfv_reply(reply:*mut get_histogram_parameterfv_reply) -> GetHistogramParameterfvReply { GetHistogramParameterfvReply { base : base::mk_reply(reply) } }
pub fn GetHistogramParameterfv<'r> (c : &'r Connection,
                                context_tag : ContextTag,
                                target : u32,
                                pname : u32) -> GetHistogramParameterfvCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_histogram_parameterfv(c.get_raw_conn(),
        context_tag as context_tag, //1
        target as u32, //2
        pname as u32); //3
    GetHistogramParameterfvCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetHistogramParameterfvUnchecked<'r> (c : &'r Connection,
                                         context_tag : ContextTag,
                                         target : u32,
                                         pname : u32) -> GetHistogramParameterfvCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_histogram_parameterfv_unchecked(c.get_raw_conn(),
        context_tag as context_tag, //1
        target as u32, //2
        pname as u32); //3
    GetHistogramParameterfvCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetHistogramParameterfvReply {
  pub fn datum(&mut self) -> Float32 {
    unsafe { accessor!(datum -> Float32, (*self.base.reply)) }
  }

  pub fn data(&mut self) -> Vec<Float32> {
    unsafe { accessor!(Float32, xcb_glx_get_histogram_parameterfv_data_length, xcb_glx_get_histogram_parameterfv_data, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetHistogramParameterfvCookie<'s>, mk_reply_get_histogram_parameterfv_reply, GetHistogramParameterfvReply, xcb_glx_get_histogram_parameterfv_reply);

pub struct GetHistogramParameterivReply { base:  base::Reply<get_histogram_parameteriv_reply> }
fn mk_reply_get_histogram_parameteriv_reply(reply:*mut get_histogram_parameteriv_reply) -> GetHistogramParameterivReply { GetHistogramParameterivReply { base : base::mk_reply(reply) } }
pub fn GetHistogramParameteriv<'r> (c : &'r Connection,
                                context_tag : ContextTag,
                                target : u32,
                                pname : u32) -> GetHistogramParameterivCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_histogram_parameteriv(c.get_raw_conn(),
        context_tag as context_tag, //1
        target as u32, //2
        pname as u32); //3
    GetHistogramParameterivCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetHistogramParameterivUnchecked<'r> (c : &'r Connection,
                                         context_tag : ContextTag,
                                         target : u32,
                                         pname : u32) -> GetHistogramParameterivCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_histogram_parameteriv_unchecked(c.get_raw_conn(),
        context_tag as context_tag, //1
        target as u32, //2
        pname as u32); //3
    GetHistogramParameterivCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetHistogramParameterivReply {
  pub fn datum(&mut self) -> i32 {
    unsafe { accessor!(datum -> i32, (*self.base.reply)) }
  }

  pub fn data(&mut self) -> Vec<i32> {
    unsafe { accessor!(i32, xcb_glx_get_histogram_parameteriv_data_length, xcb_glx_get_histogram_parameteriv_data, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetHistogramParameterivCookie<'s>, mk_reply_get_histogram_parameteriv_reply, GetHistogramParameterivReply, xcb_glx_get_histogram_parameteriv_reply);

pub struct GetMinmaxReply { base:  base::Reply<get_minmax_reply> }
fn mk_reply_get_minmax_reply(reply:*mut get_minmax_reply) -> GetMinmaxReply { GetMinmaxReply { base : base::mk_reply(reply) } }
pub fn GetMinmax<'r> (c : &'r Connection,
                  context_tag : ContextTag,
                  target : u32,
                  format : u32,
                  type_ : u32,
                  swap_bytes : u8,
                  reset : u8) -> GetMinmaxCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_minmax(c.get_raw_conn(),
        context_tag as context_tag, //1
        target as u32, //2
        format as u32, //3
        type_ as u32, //4
        swap_bytes as u8, //5
        reset as u8); //6
    GetMinmaxCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetMinmaxUnchecked<'r> (c : &'r Connection,
                           context_tag : ContextTag,
                           target : u32,
                           format : u32,
                           type_ : u32,
                           swap_bytes : u8,
                           reset : u8) -> GetMinmaxCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_minmax_unchecked(c.get_raw_conn(),
        context_tag as context_tag, //1
        target as u32, //2
        format as u32, //3
        type_ as u32, //4
        swap_bytes as u8, //5
        reset as u8); //6
    GetMinmaxCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetMinmaxReply {
  pub fn data(&mut self) -> Vec<u8> {
    unsafe { accessor!(u8, xcb_glx_get_minmax_data_length, xcb_glx_get_minmax_data, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetMinmaxCookie<'s>, mk_reply_get_minmax_reply, GetMinmaxReply, xcb_glx_get_minmax_reply);

pub struct GetMinmaxParameterfvReply { base:  base::Reply<get_minmax_parameterfv_reply> }
fn mk_reply_get_minmax_parameterfv_reply(reply:*mut get_minmax_parameterfv_reply) -> GetMinmaxParameterfvReply { GetMinmaxParameterfvReply { base : base::mk_reply(reply) } }
pub fn GetMinmaxParameterfv<'r> (c : &'r Connection,
                             context_tag : ContextTag,
                             target : u32,
                             pname : u32) -> GetMinmaxParameterfvCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_minmax_parameterfv(c.get_raw_conn(),
        context_tag as context_tag, //1
        target as u32, //2
        pname as u32); //3
    GetMinmaxParameterfvCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetMinmaxParameterfvUnchecked<'r> (c : &'r Connection,
                                      context_tag : ContextTag,
                                      target : u32,
                                      pname : u32) -> GetMinmaxParameterfvCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_minmax_parameterfv_unchecked(c.get_raw_conn(),
        context_tag as context_tag, //1
        target as u32, //2
        pname as u32); //3
    GetMinmaxParameterfvCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetMinmaxParameterfvReply {
  pub fn datum(&mut self) -> Float32 {
    unsafe { accessor!(datum -> Float32, (*self.base.reply)) }
  }

  pub fn data(&mut self) -> Vec<Float32> {
    unsafe { accessor!(Float32, xcb_glx_get_minmax_parameterfv_data_length, xcb_glx_get_minmax_parameterfv_data, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetMinmaxParameterfvCookie<'s>, mk_reply_get_minmax_parameterfv_reply, GetMinmaxParameterfvReply, xcb_glx_get_minmax_parameterfv_reply);

pub struct GetMinmaxParameterivReply { base:  base::Reply<get_minmax_parameteriv_reply> }
fn mk_reply_get_minmax_parameteriv_reply(reply:*mut get_minmax_parameteriv_reply) -> GetMinmaxParameterivReply { GetMinmaxParameterivReply { base : base::mk_reply(reply) } }
pub fn GetMinmaxParameteriv<'r> (c : &'r Connection,
                             context_tag : ContextTag,
                             target : u32,
                             pname : u32) -> GetMinmaxParameterivCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_minmax_parameteriv(c.get_raw_conn(),
        context_tag as context_tag, //1
        target as u32, //2
        pname as u32); //3
    GetMinmaxParameterivCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetMinmaxParameterivUnchecked<'r> (c : &'r Connection,
                                      context_tag : ContextTag,
                                      target : u32,
                                      pname : u32) -> GetMinmaxParameterivCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_minmax_parameteriv_unchecked(c.get_raw_conn(),
        context_tag as context_tag, //1
        target as u32, //2
        pname as u32); //3
    GetMinmaxParameterivCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetMinmaxParameterivReply {
  pub fn datum(&mut self) -> i32 {
    unsafe { accessor!(datum -> i32, (*self.base.reply)) }
  }

  pub fn data(&mut self) -> Vec<i32> {
    unsafe { accessor!(i32, xcb_glx_get_minmax_parameteriv_data_length, xcb_glx_get_minmax_parameteriv_data, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetMinmaxParameterivCookie<'s>, mk_reply_get_minmax_parameteriv_reply, GetMinmaxParameterivReply, xcb_glx_get_minmax_parameteriv_reply);

pub struct GetCompressedTexImageArbReply { base:  base::Reply<get_compressed_tex_image_arb_reply> }
fn mk_reply_get_compressed_tex_image_arb_reply(reply:*mut get_compressed_tex_image_arb_reply) -> GetCompressedTexImageArbReply { GetCompressedTexImageArbReply { base : base::mk_reply(reply) } }
pub fn GetCompressedTexImageArb<'r> (c : &'r Connection,
                                 context_tag : ContextTag,
                                 target : u32,
                                 level : i32) -> GetCompressedTexImageArbCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_compressed_tex_image_arb(c.get_raw_conn(),
        context_tag as context_tag, //1
        target as u32, //2
        level as i32); //3
    GetCompressedTexImageArbCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetCompressedTexImageArbUnchecked<'r> (c : &'r Connection,
                                          context_tag : ContextTag,
                                          target : u32,
                                          level : i32) -> GetCompressedTexImageArbCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_compressed_tex_image_arb_unchecked(c.get_raw_conn(),
        context_tag as context_tag, //1
        target as u32, //2
        level as i32); //3
    GetCompressedTexImageArbCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetCompressedTexImageArbReply {
  pub fn size(&mut self) -> i32 {
    unsafe { accessor!(size -> i32, (*self.base.reply)) }
  }

  pub fn data(&mut self) -> Vec<u8> {
    unsafe { accessor!(u8, xcb_glx_get_compressed_tex_image_arb_data_length, xcb_glx_get_compressed_tex_image_arb_data, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetCompressedTexImageArbCookie<'s>, mk_reply_get_compressed_tex_image_arb_reply, GetCompressedTexImageArbReply, xcb_glx_get_compressed_tex_image_arb_reply);

pub fn DeleteQueriesArbChecked<'r> (c : &'r Connection,
                                context_tag : ContextTag,
                                ids : &[u32]) -> base::VoidCookie<'r> {
  unsafe {
    let ids_len = ids.len();
    let ids_ptr = ids.as_ptr();
    let cookie = xcb_glx_delete_queries_arb_checked(c.get_raw_conn(),
        context_tag as context_tag, //1
        ids_len as i32, //2
        ids_ptr as *mut u32); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn DeleteQueriesArb<'r> (c : &'r Connection,
                         context_tag : ContextTag,
                         ids : &[u32]) -> base::VoidCookie<'r> {
  unsafe {
    let ids_len = ids.len();
    let ids_ptr = ids.as_ptr();
    let cookie = xcb_glx_delete_queries_arb(c.get_raw_conn(),
        context_tag as context_tag, //1
        ids_len as i32, //2
        ids_ptr as *mut u32); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub struct GenQueriesArbReply { base:  base::Reply<gen_queries_arb_reply> }
fn mk_reply_gen_queries_arb_reply(reply:*mut gen_queries_arb_reply) -> GenQueriesArbReply { GenQueriesArbReply { base : base::mk_reply(reply) } }
pub fn GenQueriesArb<'r> (c : &'r Connection,
                      context_tag : ContextTag,
                      n : i32) -> GenQueriesArbCookie<'r> {
  unsafe {
    let cookie = xcb_glx_gen_queries_arb(c.get_raw_conn(),
        context_tag as context_tag, //1
        n as i32); //2
    GenQueriesArbCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GenQueriesArbUnchecked<'r> (c : &'r Connection,
                               context_tag : ContextTag,
                               n : i32) -> GenQueriesArbCookie<'r> {
  unsafe {
    let cookie = xcb_glx_gen_queries_arb_unchecked(c.get_raw_conn(),
        context_tag as context_tag, //1
        n as i32); //2
    GenQueriesArbCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GenQueriesArbReply {
  pub fn data(&mut self) -> Vec<u32> {
    unsafe { accessor!(u32, xcb_glx_gen_queries_arb_data_length, xcb_glx_gen_queries_arb_data, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GenQueriesArbCookie<'s>, mk_reply_gen_queries_arb_reply, GenQueriesArbReply, xcb_glx_gen_queries_arb_reply);

pub fn IsQueryArb<'r> (c : &'r Connection,
                   context_tag : ContextTag,
                   id : u32) -> IsQueryArbCookie<'r> {
  unsafe {
    let cookie = xcb_glx_is_query_arb(c.get_raw_conn(),
        context_tag as context_tag, //1
        id as u32); //2
    IsQueryArbCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn IsQueryArbUnchecked<'r> (c : &'r Connection,
                            context_tag : ContextTag,
                            id : u32) -> IsQueryArbCookie<'r> {
  unsafe {
    let cookie = xcb_glx_is_query_arb_unchecked(c.get_raw_conn(),
        context_tag as context_tag, //1
        id as u32); //2
    IsQueryArbCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl IsQueryArbReply {
  pub fn ret_val(&mut self) -> Bool32 {
    unsafe { accessor!(ret_val -> Bool32, (*self.base.reply)) }
  }

}
impl_reply_cookie!(IsQueryArbCookie<'s>, mk_reply_is_query_arb_reply, IsQueryArbReply, xcb_glx_is_query_arb_reply);

pub struct GetQueryivArbReply { base:  base::Reply<get_queryiv_arb_reply> }
fn mk_reply_get_queryiv_arb_reply(reply:*mut get_queryiv_arb_reply) -> GetQueryivArbReply { GetQueryivArbReply { base : base::mk_reply(reply) } }
pub fn GetQueryivArb<'r> (c : &'r Connection,
                      context_tag : ContextTag,
                      target : u32,
                      pname : u32) -> GetQueryivArbCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_queryiv_arb(c.get_raw_conn(),
        context_tag as context_tag, //1
        target as u32, //2
        pname as u32); //3
    GetQueryivArbCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetQueryivArbUnchecked<'r> (c : &'r Connection,
                               context_tag : ContextTag,
                               target : u32,
                               pname : u32) -> GetQueryivArbCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_queryiv_arb_unchecked(c.get_raw_conn(),
        context_tag as context_tag, //1
        target as u32, //2
        pname as u32); //3
    GetQueryivArbCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetQueryivArbReply {
  pub fn datum(&mut self) -> i32 {
    unsafe { accessor!(datum -> i32, (*self.base.reply)) }
  }

  pub fn data(&mut self) -> Vec<i32> {
    unsafe { accessor!(i32, xcb_glx_get_queryiv_arb_data_length, xcb_glx_get_queryiv_arb_data, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetQueryivArbCookie<'s>, mk_reply_get_queryiv_arb_reply, GetQueryivArbReply, xcb_glx_get_queryiv_arb_reply);

pub struct GetQueryObjectivArbReply { base:  base::Reply<get_query_objectiv_arb_reply> }
fn mk_reply_get_query_objectiv_arb_reply(reply:*mut get_query_objectiv_arb_reply) -> GetQueryObjectivArbReply { GetQueryObjectivArbReply { base : base::mk_reply(reply) } }
pub fn GetQueryObjectivArb<'r> (c : &'r Connection,
                            context_tag : ContextTag,
                            id : u32,
                            pname : u32) -> GetQueryObjectivArbCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_query_objectiv_arb(c.get_raw_conn(),
        context_tag as context_tag, //1
        id as u32, //2
        pname as u32); //3
    GetQueryObjectivArbCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetQueryObjectivArbUnchecked<'r> (c : &'r Connection,
                                     context_tag : ContextTag,
                                     id : u32,
                                     pname : u32) -> GetQueryObjectivArbCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_query_objectiv_arb_unchecked(c.get_raw_conn(),
        context_tag as context_tag, //1
        id as u32, //2
        pname as u32); //3
    GetQueryObjectivArbCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetQueryObjectivArbReply {
  pub fn datum(&mut self) -> i32 {
    unsafe { accessor!(datum -> i32, (*self.base.reply)) }
  }

  pub fn data(&mut self) -> Vec<i32> {
    unsafe { accessor!(i32, xcb_glx_get_query_objectiv_arb_data_length, xcb_glx_get_query_objectiv_arb_data, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetQueryObjectivArbCookie<'s>, mk_reply_get_query_objectiv_arb_reply, GetQueryObjectivArbReply, xcb_glx_get_query_objectiv_arb_reply);

pub struct GetQueryObjectuivArbReply { base:  base::Reply<get_query_objectuiv_arb_reply> }
fn mk_reply_get_query_objectuiv_arb_reply(reply:*mut get_query_objectuiv_arb_reply) -> GetQueryObjectuivArbReply { GetQueryObjectuivArbReply { base : base::mk_reply(reply) } }
pub fn GetQueryObjectuivArb<'r> (c : &'r Connection,
                             context_tag : ContextTag,
                             id : u32,
                             pname : u32) -> GetQueryObjectuivArbCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_query_objectuiv_arb(c.get_raw_conn(),
        context_tag as context_tag, //1
        id as u32, //2
        pname as u32); //3
    GetQueryObjectuivArbCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetQueryObjectuivArbUnchecked<'r> (c : &'r Connection,
                                      context_tag : ContextTag,
                                      id : u32,
                                      pname : u32) -> GetQueryObjectuivArbCookie<'r> {
  unsafe {
    let cookie = xcb_glx_get_query_objectuiv_arb_unchecked(c.get_raw_conn(),
        context_tag as context_tag, //1
        id as u32, //2
        pname as u32); //3
    GetQueryObjectuivArbCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetQueryObjectuivArbReply {
  pub fn datum(&mut self) -> u32 {
    unsafe { accessor!(datum -> u32, (*self.base.reply)) }
  }

  pub fn data(&mut self) -> Vec<u32> {
    unsafe { accessor!(u32, xcb_glx_get_query_objectuiv_arb_data_length, xcb_glx_get_query_objectuiv_arb_data, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetQueryObjectuivArbCookie<'s>, mk_reply_get_query_objectuiv_arb_reply, GetQueryObjectuivArbReply, xcb_glx_get_query_objectuiv_arb_reply);


