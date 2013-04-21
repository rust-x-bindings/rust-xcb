/*
 * This file generated automatically from glx.xml by r_client.py.
 * Edit at your peril.
 */

//Make the compiler quiet
#[allow(unused_imports)];
#[allow(non_camel_case_types)];
use core::libc::*;
use ll::base::*;
use ll::xproto;

pub static GLX_MAJOR_VERSION : c_uint = 1;
pub static GLX_MINOR_VERSION : c_uint = 3;

pub type pixmap = u32;

/**
 * @brief pixmap_iterator
 **/
pub struct pixmap_iterator {
    data : *pixmap,
    rem  : c_int,
    index: c_int
}

pub type context = u32;

/**
 * @brief context_iterator
 **/
pub struct context_iterator {
    data : *context,
    rem  : c_int,
    index: c_int
}

pub type pbuffer = u32;

/**
 * @brief pbuffer_iterator
 **/
pub struct pbuffer_iterator {
    data : *pbuffer,
    rem  : c_int,
    index: c_int
}

pub type window = u32;

/**
 * @brief window_iterator
 **/
pub struct window_iterator {
    data : *window,
    rem  : c_int,
    index: c_int
}

pub type fbconfig = u32;

/**
 * @brief fbconfig_iterator
 **/
pub struct fbconfig_iterator {
    data : *fbconfig,
    rem  : c_int,
    index: c_int
}

pub type drawable = u32;

/**
 * @brief drawable_iterator
 **/
pub struct drawable_iterator {
    data : *drawable,
    rem  : c_int,
    index: c_int
}

pub type float32 = f32;

/**
 * @brief float32_iterator
 **/
pub struct float32_iterator {
    data : *float32,
    rem  : c_int,
    index: c_int
}

pub type float64 = f64;

/**
 * @brief float64_iterator
 **/
pub struct float64_iterator {
    data : *float64,
    rem  : c_int,
    index: c_int
}

pub type bool32 = u32;

/**
 * @brief bool32_iterator
 **/
pub struct bool32_iterator {
    data : *bool32,
    rem  : c_int,
    index: c_int
}

pub type context_tag = u32;

/**
 * @brief context_tag_iterator
 **/
pub struct context_tag_iterator {
    data : *context_tag,
    rem  : c_int,
    index: c_int
}

/** Opcode for xcb_glx_generic. */
pub static XCB_GLX_GENERIC : c_int = -1;

pub struct generic_error {
    response_type :   u8,
    error_code :      u8,
    sequence :        u16,
    bad_value :       u32,
    minor_opcode :    u16,
    major_opcode :    u8,
    pad0 :            [u8,..21]
}

/** Opcode for xcb_glx_bad_context. */
pub static XCB_GLX_BAD_CONTEXT : c_int = 0;

pub type bad_context_error  = generic_error;

/** Opcode for xcb_glx_bad_context_state. */
pub static XCB_GLX_BAD_CONTEXT_STATE : c_int = 1;

pub type bad_context_state_error  = generic_error;

/** Opcode for xcb_glx_bad_drawable. */
pub static XCB_GLX_BAD_DRAWABLE : c_int = 2;

pub type bad_drawable_error  = generic_error;

/** Opcode for xcb_glx_bad_pixmap. */
pub static XCB_GLX_BAD_PIXMAP : c_int = 3;

pub type bad_pixmap_error  = generic_error;

/** Opcode for xcb_glx_bad_context_tag. */
pub static XCB_GLX_BAD_CONTEXT_TAG : c_int = 4;

pub type bad_context_tag_error  = generic_error;

/** Opcode for xcb_glx_bad_current_window. */
pub static XCB_GLX_BAD_CURRENT_WINDOW : c_int = 5;

pub type bad_current_window_error  = generic_error;

/** Opcode for xcb_glx_bad_render_request. */
pub static XCB_GLX_BAD_RENDER_REQUEST : c_int = 6;

pub type bad_render_request_error  = generic_error;

/** Opcode for xcb_glx_bad_large_request. */
pub static XCB_GLX_BAD_LARGE_REQUEST : c_int = 7;

pub type bad_large_request_error  = generic_error;

/** Opcode for xcb_glx_unsupported_private_request. */
pub static XCB_GLX_UNSUPPORTED_PRIVATE_REQUEST : c_int = 8;

pub type unsupported_private_request_error  = generic_error;

/** Opcode for xcb_glx_bad_fb_config. */
pub static XCB_GLX_BAD_FB_CONFIG : c_int = 9;

pub type bad_fb_config_error  = generic_error;

/** Opcode for xcb_glx_bad_pbuffer. */
pub static XCB_GLX_BAD_PBUFFER : c_int = 10;

pub type bad_pbuffer_error  = generic_error;

/** Opcode for xcb_glx_bad_current_drawable. */
pub static XCB_GLX_BAD_CURRENT_DRAWABLE : c_int = 11;

pub type bad_current_drawable_error  = generic_error;

/** Opcode for xcb_glx_bad_window. */
pub static XCB_GLX_BAD_WINDOW : c_int = 12;

pub type bad_window_error  = generic_error;

/** Opcode for xcb_glx_glx_bad_profile_arb. */
pub static XCB_GLX_GLX_BAD_PROFILE_ARB : c_int = 13;

pub type glx_bad_profile_arb_error  = generic_error;

/** Opcode for xcb_glx_pbuffer_clobber. */
pub static XCB_GLX_PBUFFER_CLOBBER : c_int = 0;

pub struct pbuffer_clobber_event {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    event_type :      u16,
    draw_type :       u16,
    drawable :        drawable,
    b_mask :          u32,
    aux_buffer :      u16,
    x :               u16,
    y :               u16,
    width :           u16,
    height :          u16,
    count :           u16,
    pad1 :            [u8,..4]
}

pub type pbcet = c_uint;//{
    pub static XCB_GLX_PBCET_DAMAGED : pbcet = 32791;
    pub static XCB_GLX_PBCET_SAVED : pbcet = 32792;
//}

pub type pbcdt = c_uint;//{
    pub static XCB_GLX_PBCDT_WINDOW : pbcdt = 32793;
    pub static XCB_GLX_PBCDT_PBUFFER : pbcdt = 32794;
//}

/** Opcode for xcb_glx_render. */
pub static XCB_GLX_RENDER : c_int = 1;

pub struct render_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag
}

/** Opcode for xcb_glx_render_large. */
pub static XCB_GLX_RENDER_LARGE : c_int = 2;

pub struct render_large_request {
    major_opcode :    u8,
    minor_opcode :    u8,
    length :          u16,
    context_tag :     context_tag,
    request_num :     u16,
    request_total :   u16,
    data_len :        u32
}

/** Opcode for xcb_glx_create_context. */
pub static XCB_GLX_CREATE_CONTEXT : c_int = 3;

pub struct create_context_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context :        context,
    visual :         xproto::visualid,
    screen :         u32,
    share_list :     context,
    is_direct :      u8,
    pad0 :           [u8,..3]
}

/** Opcode for xcb_glx_destroy_context. */
pub static XCB_GLX_DESTROY_CONTEXT : c_int = 4;

pub struct destroy_context_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context :        context
}

pub struct make_current_cookie {
    sequence : c_uint
}

/** Opcode for xcb_glx_make_current. */
pub static XCB_GLX_MAKE_CURRENT : c_int = 5;

pub struct make_current_request {
    major_opcode :      u8,
    minor_opcode :      u8,
    length :            u16,
    drawable :          drawable,
    context :           context,
    old_context_tag :   context_tag
}

pub struct make_current_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    context_tag :     context_tag,
    pad1 :            [u8,..20]
}

pub struct is_direct_cookie {
    sequence : c_uint
}

/** Opcode for xcb_glx_is_direct. */
pub static XCB_GLX_IS_DIRECT : c_int = 6;

pub struct is_direct_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context :        context
}

pub struct is_direct_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    is_direct :       u8,
    pad1 :            [u8,..23]
}

pub struct query_version_cookie {
    sequence : c_uint
}

/** Opcode for xcb_glx_query_version. */
pub static XCB_GLX_QUERY_VERSION : c_int = 7;

pub struct query_version_request {
    major_opcode :    u8,
    minor_opcode :    u8,
    length :          u16,
    major_version :   u32,
    minor_version :   u32
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

/** Opcode for xcb_glx_wait_gl. */
pub static XCB_GLX_WAIT_GL : c_int = 8;

pub struct wait_gl_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag
}

/** Opcode for xcb_glx_wait_x. */
pub static XCB_GLX_WAIT_X : c_int = 9;

pub struct wait_x_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag
}

/** Opcode for xcb_glx_copy_context. */
pub static XCB_GLX_COPY_CONTEXT : c_int = 10;

pub struct copy_context_request {
    major_opcode :      u8,
    minor_opcode :      u8,
    length :            u16,
    src :               context,
    dest :              context,
    mask :              u32,
    src_context_tag :   context_tag
}

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
pub static XCB_GLX_SWAP_BUFFERS : c_int = 11;

pub struct swap_buffers_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    drawable :       drawable
}

/** Opcode for xcb_glx_use_x_font. */
pub static XCB_GLX_USE_X_FONT : c_int = 12;

pub struct use_x_font_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    font :           xproto::font,
    first :          u32,
    count :          u32,
    list_base :      u32
}

/** Opcode for xcb_glx_create_glx_pixmap. */
pub static XCB_GLX_CREATE_GLX_PIXMAP : c_int = 13;

pub struct create_glx_pixmap_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    screen :         u32,
    visual :         xproto::visualid,
    pixmap :         xproto::pixmap,
    glx_pixmap :     pixmap
}

pub struct get_visual_configs_cookie {
    sequence : c_uint
}

/** Opcode for xcb_glx_get_visual_configs. */
pub static XCB_GLX_GET_VISUAL_CONFIGS : c_int = 14;

pub struct get_visual_configs_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    screen :         u32
}

pub struct get_visual_configs_reply {
    response_type :    u8,
    pad0 :             u8,
    sequence :         u16,
    length :           u32,
    num_visuals :      u32,
    num_properties :   u32,
    pad1 :             [u8,..16]
}

/** Opcode for xcb_glx_destroy_glx_pixmap. */
pub static XCB_GLX_DESTROY_GLX_PIXMAP : c_int = 15;

pub struct destroy_glx_pixmap_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    glx_pixmap :     pixmap
}

/** Opcode for xcb_glx_vendor_private. */
pub static XCB_GLX_VENDOR_PRIVATE : c_int = 16;

pub struct vendor_private_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    vendor_code :    u32,
    context_tag :    context_tag
}

pub struct vendor_private_with_reply_cookie {
    sequence : c_uint
}

/** Opcode for xcb_glx_vendor_private_with_reply. */
pub static XCB_GLX_VENDOR_PRIVATE_WITH_REPLY : c_int = 17;

pub struct vendor_private_with_reply_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    vendor_code :    u32,
    context_tag :    context_tag
}

pub struct vendor_private_with_reply_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    retval :          u32,
    data1 :           [u8,..24]
}

pub struct query_extensions_string_cookie {
    sequence : c_uint
}

/** Opcode for xcb_glx_query_extensions_string. */
pub static XCB_GLX_QUERY_EXTENSIONS_STRING : c_int = 18;

pub struct query_extensions_string_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    screen :         u32
}

pub struct query_extensions_string_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    n :               u32,
    pad2 :            [u8,..16]
}

pub struct query_server_string_cookie {
    sequence : c_uint
}

/** Opcode for xcb_glx_query_server_string. */
pub static XCB_GLX_QUERY_SERVER_STRING : c_int = 19;

pub struct query_server_string_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    screen :         u32,
    name :           u32
}

pub struct query_server_string_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    str_len :         u32,
    pad2 :            [u8,..16]
}

/** Opcode for xcb_glx_client_info. */
pub static XCB_GLX_CLIENT_INFO : c_int = 20;

pub struct client_info_request {
    major_opcode :    u8,
    minor_opcode :    u8,
    length :          u16,
    major_version :   u32,
    minor_version :   u32,
    str_len :         u32
}

pub struct get_fb_configs_cookie {
    sequence : c_uint
}

/** Opcode for xcb_glx_get_fb_configs. */
pub static XCB_GLX_GET_FB_CONFIGS : c_int = 21;

pub struct get_fb_configs_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    screen :         u32
}

pub struct get_fb_configs_reply {
    response_type :    u8,
    pad0 :             u8,
    sequence :         u16,
    length :           u32,
    num_FB_configs :   u32,
    num_properties :   u32,
    pad1 :             [u8,..16]
}

/** Opcode for xcb_glx_create_pixmap. */
pub static XCB_GLX_CREATE_PIXMAP : c_int = 22;

pub struct create_pixmap_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    screen :         u32,
    fbconfig :       fbconfig,
    pixmap :         xproto::pixmap,
    glx_pixmap :     pixmap,
    num_attribs :    u32
}

/** Opcode for xcb_glx_destroy_pixmap. */
pub static XCB_GLX_DESTROY_PIXMAP : c_int = 23;

pub struct destroy_pixmap_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    glx_pixmap :     pixmap
}

/** Opcode for xcb_glx_create_new_context. */
pub static XCB_GLX_CREATE_NEW_CONTEXT : c_int = 24;

pub struct create_new_context_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context :        context,
    fbconfig :       fbconfig,
    screen :         u32,
    render_type :    u32,
    share_list :     context,
    is_direct :      u8,
    pad0 :           [u8,..3]
}

pub struct query_context_cookie {
    sequence : c_uint
}

/** Opcode for xcb_glx_query_context. */
pub static XCB_GLX_QUERY_CONTEXT : c_int = 25;

pub struct query_context_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context :        context
}

pub struct query_context_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    num_attribs :     u32,
    pad1 :            [u8,..20]
}

pub struct make_context_current_cookie {
    sequence : c_uint
}

/** Opcode for xcb_glx_make_context_current. */
pub static XCB_GLX_MAKE_CONTEXT_CURRENT : c_int = 26;

pub struct make_context_current_request {
    major_opcode :      u8,
    minor_opcode :      u8,
    length :            u16,
    old_context_tag :   context_tag,
    drawable :          drawable,
    read_drawable :     drawable,
    context :           context
}

pub struct make_context_current_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    context_tag :     context_tag,
    pad1 :            [u8,..20]
}

/** Opcode for xcb_glx_create_pbuffer. */
pub static XCB_GLX_CREATE_PBUFFER : c_int = 27;

pub struct create_pbuffer_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    screen :         u32,
    fbconfig :       fbconfig,
    pbuffer :        pbuffer,
    num_attribs :    u32
}

/** Opcode for xcb_glx_destroy_pbuffer. */
pub static XCB_GLX_DESTROY_PBUFFER : c_int = 28;

pub struct destroy_pbuffer_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    pbuffer :        pbuffer
}

pub struct get_drawable_attributes_cookie {
    sequence : c_uint
}

/** Opcode for xcb_glx_get_drawable_attributes. */
pub static XCB_GLX_GET_DRAWABLE_ATTRIBUTES : c_int = 29;

pub struct get_drawable_attributes_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    drawable :       drawable
}

pub struct get_drawable_attributes_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    num_attribs :     u32,
    pad1 :            [u8,..20]
}

/** Opcode for xcb_glx_change_drawable_attributes. */
pub static XCB_GLX_CHANGE_DRAWABLE_ATTRIBUTES : c_int = 30;

pub struct change_drawable_attributes_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    drawable :       drawable,
    num_attribs :    u32
}

/** Opcode for xcb_glx_create_window. */
pub static XCB_GLX_CREATE_WINDOW : c_int = 31;

pub struct create_window_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    screen :         u32,
    fbconfig :       fbconfig,
    window :         xproto::window,
    glx_window :     window,
    num_attribs :    u32
}

/** Opcode for xcb_glx_delete_window. */
pub static XCB_GLX_DELETE_WINDOW : c_int = 32;

pub struct delete_window_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    glxwindow :      window
}

/** Opcode for xcb_glx_set_client_info_arb. */
pub static XCB_GLX_SET_CLIENT_INFO_ARB : c_int = 33;

pub struct set_client_info_arb_request {
    major_opcode :    u8,
    minor_opcode :    u8,
    length :          u16,
    major_version :   u32,
    minor_version :   u32,
    num_versions :    u32,
    gl_str_len :      u32,
    glx_str_len :     u32
}

/** Opcode for xcb_glx_create_context_attribs_arb. */
pub static XCB_GLX_CREATE_CONTEXT_ATTRIBS_ARB : c_int = 34;

pub struct create_context_attribs_arb_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context :        context,
    fbconfig :       fbconfig,
    screen :         u32,
    share_list :     context,
    is_direct :      u8,
    pad0 :           [u8,..3],
    num_attribs :    u32
}

/** Opcode for xcb_glx_set_client_info_2arb. */
pub static XCB_GLX_SET_CLIENT_INFO_2ARB : c_int = 35;

pub struct set_client_info_2arb_request {
    major_opcode :    u8,
    minor_opcode :    u8,
    length :          u16,
    major_version :   u32,
    minor_version :   u32,
    num_versions :    u32,
    gl_str_len :      u32,
    glx_str_len :     u32
}

/** Opcode for xcb_glx_new_list. */
pub static XCB_GLX_NEW_LIST : c_int = 101;

pub struct new_list_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    list :           u32,
    mode :           u32
}

/** Opcode for xcb_glx_end_list. */
pub static XCB_GLX_END_LIST : c_int = 102;

pub struct end_list_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag
}

/** Opcode for xcb_glx_delete_lists. */
pub static XCB_GLX_DELETE_LISTS : c_int = 103;

pub struct delete_lists_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    list :           u32,
    range :          i32
}

pub struct gen_lists_cookie {
    sequence : c_uint
}

/** Opcode for xcb_glx_gen_lists. */
pub static XCB_GLX_GEN_LISTS : c_int = 104;

pub struct gen_lists_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    range :          i32
}

pub struct gen_lists_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    ret_val :         u32
}

/** Opcode for xcb_glx_feedback_buffer. */
pub static XCB_GLX_FEEDBACK_BUFFER : c_int = 105;

pub struct feedback_buffer_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    size :           i32,
    type_ :          i32
}

/** Opcode for xcb_glx_select_buffer. */
pub static XCB_GLX_SELECT_BUFFER : c_int = 106;

pub struct select_buffer_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    size :           i32
}

pub struct render_mode_cookie {
    sequence : c_uint
}

/** Opcode for xcb_glx_render_mode. */
pub static XCB_GLX_RENDER_MODE : c_int = 107;

pub struct render_mode_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    mode :           u32
}

pub struct render_mode_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    ret_val :         u32,
    n :               u32,
    new_mode :        u32,
    pad1 :            [u8,..12]
}

pub type rm = c_uint;//{
    pub static XCB_GLX_RM_GL_RENDER : rm = 7168;
    pub static XCB_GLX_RM_GL_FEEDBACK : rm = 7169;
    pub static XCB_GLX_RM_GL_SELECT : rm = 7170;
//}

pub struct finish_cookie {
    sequence : c_uint
}

/** Opcode for xcb_glx_finish. */
pub static XCB_GLX_FINISH : c_int = 108;

pub struct finish_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag
}

pub struct finish_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32
}

/** Opcode for xcb_glx_pixel_storef. */
pub static XCB_GLX_PIXEL_STOREF : c_int = 109;

pub struct pixel_storef_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    pname :          u32,
    datum :          float32
}

/** Opcode for xcb_glx_pixel_storei. */
pub static XCB_GLX_PIXEL_STOREI : c_int = 110;

pub struct pixel_storei_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    pname :          u32,
    datum :          i32
}

pub struct read_pixels_cookie {
    sequence : c_uint
}

/** Opcode for xcb_glx_read_pixels. */
pub static XCB_GLX_READ_PIXELS : c_int = 111;

pub struct read_pixels_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    x :              i32,
    y :              i32,
    width :          i32,
    height :         i32,
    format :         u32,
    type_ :          u32,
    swap_bytes :     u8,
    lsb_first :      u8
}

pub struct read_pixels_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..24]
}

pub struct get_booleanv_cookie {
    sequence : c_uint
}

/** Opcode for xcb_glx_get_booleanv. */
pub static XCB_GLX_GET_BOOLEANV : c_int = 112;

pub struct get_booleanv_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    pname :          i32
}

pub struct get_booleanv_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    n :               u32,
    datum :           u8,
    pad2 :            [u8,..15]
}

pub struct get_clip_plane_cookie {
    sequence : c_uint
}

/** Opcode for xcb_glx_get_clip_plane. */
pub static XCB_GLX_GET_CLIP_PLANE : c_int = 113;

pub struct get_clip_plane_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    plane :          i32
}

pub struct get_clip_plane_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..24]
}

pub struct get_doublev_cookie {
    sequence : c_uint
}

/** Opcode for xcb_glx_get_doublev. */
pub static XCB_GLX_GET_DOUBLEV : c_int = 114;

pub struct get_doublev_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    pname :          u32
}

pub struct get_doublev_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    n :               u32,
    datum :           float64,
    pad2 :            [u8,..8]
}

pub struct get_error_cookie {
    sequence : c_uint
}

/** Opcode for xcb_glx_get_error. */
pub static XCB_GLX_GET_ERROR : c_int = 115;

pub struct get_error_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag
}

pub struct get_error_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    error :           i32
}

pub struct get_floatv_cookie {
    sequence : c_uint
}

/** Opcode for xcb_glx_get_floatv. */
pub static XCB_GLX_GET_FLOATV : c_int = 116;

pub struct get_floatv_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    pname :          u32
}

pub struct get_floatv_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    n :               u32,
    datum :           float32,
    pad2 :            [u8,..12]
}

pub struct get_integerv_cookie {
    sequence : c_uint
}

/** Opcode for xcb_glx_get_integerv. */
pub static XCB_GLX_GET_INTEGERV : c_int = 117;

pub struct get_integerv_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    pname :          u32
}

pub struct get_integerv_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    n :               u32,
    datum :           i32,
    pad2 :            [u8,..12]
}

pub struct get_lightfv_cookie {
    sequence : c_uint
}

/** Opcode for xcb_glx_get_lightfv. */
pub static XCB_GLX_GET_LIGHTFV : c_int = 118;

pub struct get_lightfv_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    light :          u32,
    pname :          u32
}

pub struct get_lightfv_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    n :               u32,
    datum :           float32,
    pad2 :            [u8,..12]
}

pub struct get_lightiv_cookie {
    sequence : c_uint
}

/** Opcode for xcb_glx_get_lightiv. */
pub static XCB_GLX_GET_LIGHTIV : c_int = 119;

pub struct get_lightiv_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    light :          u32,
    pname :          u32
}

pub struct get_lightiv_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    n :               u32,
    datum :           i32,
    pad2 :            [u8,..12]
}

pub struct get_mapdv_cookie {
    sequence : c_uint
}

/** Opcode for xcb_glx_get_mapdv. */
pub static XCB_GLX_GET_MAPDV : c_int = 120;

pub struct get_mapdv_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    target :         u32,
    query :          u32
}

pub struct get_mapdv_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    n :               u32,
    datum :           float64,
    pad2 :            [u8,..8]
}

pub struct get_mapfv_cookie {
    sequence : c_uint
}

/** Opcode for xcb_glx_get_mapfv. */
pub static XCB_GLX_GET_MAPFV : c_int = 121;

pub struct get_mapfv_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    target :         u32,
    query :          u32
}

pub struct get_mapfv_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    n :               u32,
    datum :           float32,
    pad2 :            [u8,..12]
}

pub struct get_mapiv_cookie {
    sequence : c_uint
}

/** Opcode for xcb_glx_get_mapiv. */
pub static XCB_GLX_GET_MAPIV : c_int = 122;

pub struct get_mapiv_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    target :         u32,
    query :          u32
}

pub struct get_mapiv_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    n :               u32,
    datum :           i32,
    pad2 :            [u8,..12]
}

pub struct get_materialfv_cookie {
    sequence : c_uint
}

/** Opcode for xcb_glx_get_materialfv. */
pub static XCB_GLX_GET_MATERIALFV : c_int = 123;

pub struct get_materialfv_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    face :           u32,
    pname :          u32
}

pub struct get_materialfv_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    n :               u32,
    datum :           float32,
    pad2 :            [u8,..12]
}

pub struct get_materialiv_cookie {
    sequence : c_uint
}

/** Opcode for xcb_glx_get_materialiv. */
pub static XCB_GLX_GET_MATERIALIV : c_int = 124;

pub struct get_materialiv_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    face :           u32,
    pname :          u32
}

pub struct get_materialiv_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    n :               u32,
    datum :           i32,
    pad2 :            [u8,..12]
}

pub struct get_pixel_mapfv_cookie {
    sequence : c_uint
}

/** Opcode for xcb_glx_get_pixel_mapfv. */
pub static XCB_GLX_GET_PIXEL_MAPFV : c_int = 125;

pub struct get_pixel_mapfv_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    map :            u32
}

pub struct get_pixel_mapfv_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    n :               u32,
    datum :           float32,
    pad2 :            [u8,..12]
}

pub struct get_pixel_mapuiv_cookie {
    sequence : c_uint
}

/** Opcode for xcb_glx_get_pixel_mapuiv. */
pub static XCB_GLX_GET_PIXEL_MAPUIV : c_int = 126;

pub struct get_pixel_mapuiv_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    map :            u32
}

pub struct get_pixel_mapuiv_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    n :               u32,
    datum :           u32,
    pad2 :            [u8,..12]
}

pub struct get_pixel_mapusv_cookie {
    sequence : c_uint
}

/** Opcode for xcb_glx_get_pixel_mapusv. */
pub static XCB_GLX_GET_PIXEL_MAPUSV : c_int = 127;

pub struct get_pixel_mapusv_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    map :            u32
}

pub struct get_pixel_mapusv_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    n :               u32,
    datum :           u16,
    pad2 :            [u8,..16]
}

pub struct get_polygon_stipple_cookie {
    sequence : c_uint
}

/** Opcode for xcb_glx_get_polygon_stipple. */
pub static XCB_GLX_GET_POLYGON_STIPPLE : c_int = 128;

pub struct get_polygon_stipple_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    lsb_first :      u8
}

pub struct get_polygon_stipple_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..24]
}

pub struct get_string_cookie {
    sequence : c_uint
}

/** Opcode for xcb_glx_get_string. */
pub static XCB_GLX_GET_STRING : c_int = 129;

pub struct get_string_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    name :           u32
}

pub struct get_string_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    n :               u32,
    pad2 :            [u8,..16]
}

pub struct get_tex_envfv_cookie {
    sequence : c_uint
}

/** Opcode for xcb_glx_get_tex_envfv. */
pub static XCB_GLX_GET_TEX_ENVFV : c_int = 130;

pub struct get_tex_envfv_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    target :         u32,
    pname :          u32
}

pub struct get_tex_envfv_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    n :               u32,
    datum :           float32,
    pad2 :            [u8,..12]
}

pub struct get_tex_enviv_cookie {
    sequence : c_uint
}

/** Opcode for xcb_glx_get_tex_enviv. */
pub static XCB_GLX_GET_TEX_ENVIV : c_int = 131;

pub struct get_tex_enviv_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    target :         u32,
    pname :          u32
}

pub struct get_tex_enviv_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    n :               u32,
    datum :           i32,
    pad2 :            [u8,..12]
}

pub struct get_tex_gendv_cookie {
    sequence : c_uint
}

/** Opcode for xcb_glx_get_tex_gendv. */
pub static XCB_GLX_GET_TEX_GENDV : c_int = 132;

pub struct get_tex_gendv_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    coord :          u32,
    pname :          u32
}

pub struct get_tex_gendv_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    n :               u32,
    datum :           float64,
    pad2 :            [u8,..8]
}

pub struct get_tex_genfv_cookie {
    sequence : c_uint
}

/** Opcode for xcb_glx_get_tex_genfv. */
pub static XCB_GLX_GET_TEX_GENFV : c_int = 133;

pub struct get_tex_genfv_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    coord :          u32,
    pname :          u32
}

pub struct get_tex_genfv_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    n :               u32,
    datum :           float32,
    pad2 :            [u8,..12]
}

pub struct get_tex_geniv_cookie {
    sequence : c_uint
}

/** Opcode for xcb_glx_get_tex_geniv. */
pub static XCB_GLX_GET_TEX_GENIV : c_int = 134;

pub struct get_tex_geniv_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    coord :          u32,
    pname :          u32
}

pub struct get_tex_geniv_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    n :               u32,
    datum :           i32,
    pad2 :            [u8,..12]
}

pub struct get_tex_image_cookie {
    sequence : c_uint
}

/** Opcode for xcb_glx_get_tex_image. */
pub static XCB_GLX_GET_TEX_IMAGE : c_int = 135;

pub struct get_tex_image_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    target :         u32,
    level :          i32,
    format :         u32,
    type_ :          u32,
    swap_bytes :     u8
}

pub struct get_tex_image_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..8],
    width :           i32,
    height :          i32,
    depth :           i32,
    pad2 :            [u8,..4]
}

pub struct get_tex_parameterfv_cookie {
    sequence : c_uint
}

/** Opcode for xcb_glx_get_tex_parameterfv. */
pub static XCB_GLX_GET_TEX_PARAMETERFV : c_int = 136;

pub struct get_tex_parameterfv_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    target :         u32,
    pname :          u32
}

pub struct get_tex_parameterfv_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    n :               u32,
    datum :           float32,
    pad2 :            [u8,..12]
}

pub struct get_tex_parameteriv_cookie {
    sequence : c_uint
}

/** Opcode for xcb_glx_get_tex_parameteriv. */
pub static XCB_GLX_GET_TEX_PARAMETERIV : c_int = 137;

pub struct get_tex_parameteriv_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    target :         u32,
    pname :          u32
}

pub struct get_tex_parameteriv_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    n :               u32,
    datum :           i32,
    pad2 :            [u8,..12]
}

pub struct get_tex_level_parameterfv_cookie {
    sequence : c_uint
}

/** Opcode for xcb_glx_get_tex_level_parameterfv. */
pub static XCB_GLX_GET_TEX_LEVEL_PARAMETERFV : c_int = 138;

pub struct get_tex_level_parameterfv_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    target :         u32,
    level :          i32,
    pname :          u32
}

pub struct get_tex_level_parameterfv_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    n :               u32,
    datum :           float32,
    pad2 :            [u8,..12]
}

pub struct get_tex_level_parameteriv_cookie {
    sequence : c_uint
}

/** Opcode for xcb_glx_get_tex_level_parameteriv. */
pub static XCB_GLX_GET_TEX_LEVEL_PARAMETERIV : c_int = 139;

pub struct get_tex_level_parameteriv_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    target :         u32,
    level :          i32,
    pname :          u32
}

pub struct get_tex_level_parameteriv_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    n :               u32,
    datum :           i32,
    pad2 :            [u8,..12]
}

pub struct is_list_cookie {
    sequence : c_uint
}

/** Opcode for xcb_glx_is_list. */
pub static XCB_GLX_IS_LIST : c_int = 141;

pub struct is_list_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    list :           u32
}

pub struct is_list_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    ret_val :         bool32
}

/** Opcode for xcb_glx_flush. */
pub static XCB_GLX_FLUSH : c_int = 142;

pub struct flush_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag
}

pub struct are_textures_resident_cookie {
    sequence : c_uint
}

/** Opcode for xcb_glx_are_textures_resident. */
pub static XCB_GLX_ARE_TEXTURES_RESIDENT : c_int = 143;

pub struct are_textures_resident_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    n :              i32
}

pub struct are_textures_resident_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    ret_val :         bool32,
    pad1 :            [u8,..20]
}

/** Opcode for xcb_glx_delete_textures. */
pub static XCB_GLX_DELETE_TEXTURES : c_int = 144;

pub struct delete_textures_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    n :              i32
}

pub struct gen_textures_cookie {
    sequence : c_uint
}

/** Opcode for xcb_glx_gen_textures. */
pub static XCB_GLX_GEN_TEXTURES : c_int = 145;

pub struct gen_textures_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    n :              i32
}

pub struct gen_textures_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..24]
}

pub struct is_texture_cookie {
    sequence : c_uint
}

/** Opcode for xcb_glx_is_texture. */
pub static XCB_GLX_IS_TEXTURE : c_int = 146;

pub struct is_texture_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    texture :        u32
}

pub struct is_texture_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    ret_val :         bool32
}

pub struct get_color_table_cookie {
    sequence : c_uint
}

/** Opcode for xcb_glx_get_color_table. */
pub static XCB_GLX_GET_COLOR_TABLE : c_int = 147;

pub struct get_color_table_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    target :         u32,
    format :         u32,
    type_ :          u32,
    swap_bytes :     u8
}

pub struct get_color_table_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..8],
    width :           i32,
    pad2 :            [u8,..12]
}

pub struct get_color_table_parameterfv_cookie {
    sequence : c_uint
}

/** Opcode for xcb_glx_get_color_table_parameterfv. */
pub static XCB_GLX_GET_COLOR_TABLE_PARAMETERFV : c_int = 148;

pub struct get_color_table_parameterfv_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    target :         u32,
    pname :          u32
}

pub struct get_color_table_parameterfv_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    n :               u32,
    datum :           float32,
    pad2 :            [u8,..12]
}

pub struct get_color_table_parameteriv_cookie {
    sequence : c_uint
}

/** Opcode for xcb_glx_get_color_table_parameteriv. */
pub static XCB_GLX_GET_COLOR_TABLE_PARAMETERIV : c_int = 149;

pub struct get_color_table_parameteriv_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    target :         u32,
    pname :          u32
}

pub struct get_color_table_parameteriv_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    n :               u32,
    datum :           i32,
    pad2 :            [u8,..12]
}

pub struct get_convolution_filter_cookie {
    sequence : c_uint
}

/** Opcode for xcb_glx_get_convolution_filter. */
pub static XCB_GLX_GET_CONVOLUTION_FILTER : c_int = 150;

pub struct get_convolution_filter_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    target :         u32,
    format :         u32,
    type_ :          u32,
    swap_bytes :     u8
}

pub struct get_convolution_filter_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..8],
    width :           i32,
    height :          i32,
    pad2 :            [u8,..8]
}

pub struct get_convolution_parameterfv_cookie {
    sequence : c_uint
}

/** Opcode for xcb_glx_get_convolution_parameterfv. */
pub static XCB_GLX_GET_CONVOLUTION_PARAMETERFV : c_int = 151;

pub struct get_convolution_parameterfv_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    target :         u32,
    pname :          u32
}

pub struct get_convolution_parameterfv_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    n :               u32,
    datum :           float32,
    pad2 :            [u8,..12]
}

pub struct get_convolution_parameteriv_cookie {
    sequence : c_uint
}

/** Opcode for xcb_glx_get_convolution_parameteriv. */
pub static XCB_GLX_GET_CONVOLUTION_PARAMETERIV : c_int = 152;

pub struct get_convolution_parameteriv_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    target :         u32,
    pname :          u32
}

pub struct get_convolution_parameteriv_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    n :               u32,
    datum :           i32,
    pad2 :            [u8,..12]
}

pub struct get_separable_filter_cookie {
    sequence : c_uint
}

/** Opcode for xcb_glx_get_separable_filter. */
pub static XCB_GLX_GET_SEPARABLE_FILTER : c_int = 153;

pub struct get_separable_filter_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    target :         u32,
    format :         u32,
    type_ :          u32,
    swap_bytes :     u8
}

pub struct get_separable_filter_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..8],
    row_w :           i32,
    col_h :           i32,
    pad2 :            [u8,..8]
}

pub struct get_histogram_cookie {
    sequence : c_uint
}

/** Opcode for xcb_glx_get_histogram. */
pub static XCB_GLX_GET_HISTOGRAM : c_int = 154;

pub struct get_histogram_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    target :         u32,
    format :         u32,
    type_ :          u32,
    swap_bytes :     u8,
    reset :          u8
}

pub struct get_histogram_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..8],
    width :           i32,
    pad2 :            [u8,..12]
}

pub struct get_histogram_parameterfv_cookie {
    sequence : c_uint
}

/** Opcode for xcb_glx_get_histogram_parameterfv. */
pub static XCB_GLX_GET_HISTOGRAM_PARAMETERFV : c_int = 155;

pub struct get_histogram_parameterfv_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    target :         u32,
    pname :          u32
}

pub struct get_histogram_parameterfv_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    n :               u32,
    datum :           float32,
    pad2 :            [u8,..12]
}

pub struct get_histogram_parameteriv_cookie {
    sequence : c_uint
}

/** Opcode for xcb_glx_get_histogram_parameteriv. */
pub static XCB_GLX_GET_HISTOGRAM_PARAMETERIV : c_int = 156;

pub struct get_histogram_parameteriv_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    target :         u32,
    pname :          u32
}

pub struct get_histogram_parameteriv_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    n :               u32,
    datum :           i32,
    pad2 :            [u8,..12]
}

pub struct get_minmax_cookie {
    sequence : c_uint
}

/** Opcode for xcb_glx_get_minmax. */
pub static XCB_GLX_GET_MINMAX : c_int = 157;

pub struct get_minmax_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    target :         u32,
    format :         u32,
    type_ :          u32,
    swap_bytes :     u8,
    reset :          u8
}

pub struct get_minmax_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..24]
}

pub struct get_minmax_parameterfv_cookie {
    sequence : c_uint
}

/** Opcode for xcb_glx_get_minmax_parameterfv. */
pub static XCB_GLX_GET_MINMAX_PARAMETERFV : c_int = 158;

pub struct get_minmax_parameterfv_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    target :         u32,
    pname :          u32
}

pub struct get_minmax_parameterfv_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    n :               u32,
    datum :           float32,
    pad2 :            [u8,..12]
}

pub struct get_minmax_parameteriv_cookie {
    sequence : c_uint
}

/** Opcode for xcb_glx_get_minmax_parameteriv. */
pub static XCB_GLX_GET_MINMAX_PARAMETERIV : c_int = 159;

pub struct get_minmax_parameteriv_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    target :         u32,
    pname :          u32
}

pub struct get_minmax_parameteriv_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    n :               u32,
    datum :           i32,
    pad2 :            [u8,..12]
}

pub struct get_compressed_tex_image_arb_cookie {
    sequence : c_uint
}

/** Opcode for xcb_glx_get_compressed_tex_image_arb. */
pub static XCB_GLX_GET_COMPRESSED_TEX_IMAGE_ARB : c_int = 160;

pub struct get_compressed_tex_image_arb_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    target :         u32,
    level :          i32
}

pub struct get_compressed_tex_image_arb_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..8],
    size :            i32,
    pad2 :            [u8,..12]
}

/** Opcode for xcb_glx_delete_queries_arb. */
pub static XCB_GLX_DELETE_QUERIES_ARB : c_int = 161;

pub struct delete_queries_arb_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    n :              i32
}

pub struct gen_queries_arb_cookie {
    sequence : c_uint
}

/** Opcode for xcb_glx_gen_queries_arb. */
pub static XCB_GLX_GEN_QUERIES_ARB : c_int = 162;

pub struct gen_queries_arb_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    n :              i32
}

pub struct gen_queries_arb_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..24]
}

pub struct is_query_arb_cookie {
    sequence : c_uint
}

/** Opcode for xcb_glx_is_query_arb. */
pub static XCB_GLX_IS_QUERY_ARB : c_int = 163;

pub struct is_query_arb_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    id :             u32
}

pub struct is_query_arb_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    ret_val :         bool32
}

pub struct get_queryiv_arb_cookie {
    sequence : c_uint
}

/** Opcode for xcb_glx_get_queryiv_arb. */
pub static XCB_GLX_GET_QUERYIV_ARB : c_int = 164;

pub struct get_queryiv_arb_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    target :         u32,
    pname :          u32
}

pub struct get_queryiv_arb_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    n :               u32,
    datum :           i32,
    pad2 :            [u8,..12]
}

pub struct get_query_objectiv_arb_cookie {
    sequence : c_uint
}

/** Opcode for xcb_glx_get_query_objectiv_arb. */
pub static XCB_GLX_GET_QUERY_OBJECTIV_ARB : c_int = 165;

pub struct get_query_objectiv_arb_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    id :             u32,
    pname :          u32
}

pub struct get_query_objectiv_arb_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    n :               u32,
    datum :           i32,
    pad2 :            [u8,..12]
}

pub struct get_query_objectuiv_arb_cookie {
    sequence : c_uint
}

/** Opcode for xcb_glx_get_query_objectuiv_arb. */
pub static XCB_GLX_GET_QUERY_OBJECTUIV_ARB : c_int = 166;

pub struct get_query_objectuiv_arb_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    id :             u32,
    pname :          u32
}

pub struct get_query_objectuiv_arb_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    n :               u32,
    datum :           u32,
    pad2 :            [u8,..12]
}
#[link_args="-lxcb-glx"]
extern "C" {

/**
 * Get the next element of the iterator
 * @param i Pointer to a pixmap_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(pixmap)
 *
 *
 */
unsafe fn xcb_glx_pixmap_next (i:*pixmap_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An pixmap_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_glx_pixmap_end (i:pixmap_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a context_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(context)
 *
 *
 */
unsafe fn xcb_glx_context_next (i:*context_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An context_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_glx_context_end (i:context_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a pbuffer_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(pbuffer)
 *
 *
 */
unsafe fn xcb_glx_pbuffer_next (i:*pbuffer_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An pbuffer_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_glx_pbuffer_end (i:pbuffer_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a window_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(window)
 *
 *
 */
unsafe fn xcb_glx_window_next (i:*window_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An window_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_glx_window_end (i:window_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a fbconfig_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(fbconfig)
 *
 *
 */
unsafe fn xcb_glx_fbconfig_next (i:*fbconfig_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An fbconfig_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_glx_fbconfig_end (i:fbconfig_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a drawable_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(drawable)
 *
 *
 */
unsafe fn xcb_glx_drawable_next (i:*drawable_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An drawable_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_glx_drawable_end (i:drawable_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a float32_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(float32)
 *
 *
 */
unsafe fn xcb_glx_float32_next (i:*float32_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An float32_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_glx_float32_end (i:float32_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a float64_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(float64)
 *
 *
 */
unsafe fn xcb_glx_float64_next (i:*float64_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An float64_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_glx_float64_end (i:float64_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a bool32_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(bool32)
 *
 *
 */
unsafe fn xcb_glx_bool32_next (i:*bool32_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An bool32_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_glx_bool32_end (i:bool32_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a context_tag_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(context_tag)
 *
 *
 */
unsafe fn xcb_glx_context_tag_next (i:*context_tag_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An context_tag_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_glx_context_tag_end (i:context_tag_iterator) -> generic_iterator;

unsafe fn xcb_glx_render_sizeof (_buffer :  *c_void,
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
unsafe fn xcb_glx_render_checked (c : *connection,
                                  context_tag :  context_tag,
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
unsafe fn xcb_glx_render (c : *connection,
                          context_tag :  context_tag,
                          data_len :  u32,
                          data : *u8) -> void_cookie;

unsafe fn xcb_glx_render_large_sizeof (_buffer :  *c_void) -> c_int;

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
unsafe fn xcb_glx_render_large_checked (c : *connection,
                                        context_tag :  context_tag,
                                        request_num :  u16,
                                        request_total :  u16,
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
unsafe fn xcb_glx_render_large (c : *connection,
                                context_tag :  context_tag,
                                request_num :  u16,
                                request_total :  u16,
                                data_len :  u32,
                                data : *u8) -> void_cookie;

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
unsafe fn xcb_glx_create_context_checked (c : *connection,
                                          context :  context,
                                          visual :  xproto::visualid,
                                          screen :  u32,
                                          share_list :  context,
                                          is_direct :  u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_create_context (c : *connection,
                                  context :  context,
                                  visual :  xproto::visualid,
                                  screen :  u32,
                                  share_list :  context,
                                  is_direct :  u8) -> void_cookie;

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
unsafe fn xcb_glx_destroy_context_checked (c : *connection,
                                           context :  context) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_destroy_context (c : *connection,
                                   context :  context) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_make_current (c : *connection,
                                drawable :  drawable,
                                context :  context,
                                old_context_tag :  context_tag) -> make_current_cookie;

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
unsafe fn xcb_glx_make_current_unchecked (c : *connection,
                                          drawable :  drawable,
                                          context :  context,
                                          old_context_tag :  context_tag) -> make_current_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_make_current_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_glx_make_current_reply (c : *connection,
                                      cookie : make_current_cookie,
                                      e : **generic_error) -> *make_current_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_is_direct (c : *connection,
                             context :  context) -> is_direct_cookie;

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
unsafe fn xcb_glx_is_direct_unchecked (c : *connection,
                                       context :  context) -> is_direct_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_is_direct_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_glx_is_direct_reply (c : *connection,
                                   cookie : is_direct_cookie,
                                   e : **generic_error) -> *is_direct_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_query_version (c : *connection,
                                 major_version :  u32,
                                 minor_version :  u32) -> query_version_cookie;

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
unsafe fn xcb_glx_query_version_unchecked (c : *connection,
                                           major_version :  u32,
                                           minor_version :  u32) -> query_version_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_query_version_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_glx_query_version_reply (c : *connection,
                                       cookie : query_version_cookie,
                                       e : **generic_error) -> *query_version_reply;

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
unsafe fn xcb_glx_wait_gl_checked (c : *connection,
                                   context_tag :  context_tag) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_wait_gl (c : *connection,
                           context_tag :  context_tag) -> void_cookie;

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
unsafe fn xcb_glx_wait_x_checked (c : *connection,
                                  context_tag :  context_tag) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_wait_x (c : *connection,
                          context_tag :  context_tag) -> void_cookie;

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
unsafe fn xcb_glx_copy_context_checked (c : *connection,
                                        src :  context,
                                        dest :  context,
                                        mask :  u32,
                                        src_context_tag :  context_tag) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_copy_context (c : *connection,
                                src :  context,
                                dest :  context,
                                mask :  u32,
                                src_context_tag :  context_tag) -> void_cookie;

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
unsafe fn xcb_glx_swap_buffers_checked (c : *connection,
                                        context_tag :  context_tag,
                                        drawable :  drawable) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_swap_buffers (c : *connection,
                                context_tag :  context_tag,
                                drawable :  drawable) -> void_cookie;

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
unsafe fn xcb_glx_use_x_font_checked (c : *connection,
                                      context_tag :  context_tag,
                                      font :  xproto::font,
                                      first :  u32,
                                      count :  u32,
                                      list_base :  u32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_use_x_font (c : *connection,
                              context_tag :  context_tag,
                              font :  xproto::font,
                              first :  u32,
                              count :  u32,
                              list_base :  u32) -> void_cookie;

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
unsafe fn xcb_glx_create_glx_pixmap_checked (c : *connection,
                                             screen :  u32,
                                             visual :  xproto::visualid,
                                             pixmap :  xproto::pixmap,
                                             glx_pixmap :  pixmap) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_create_glx_pixmap (c : *connection,
                                     screen :  u32,
                                     visual :  xproto::visualid,
                                     pixmap :  xproto::pixmap,
                                     glx_pixmap :  pixmap) -> void_cookie;

unsafe fn xcb_glx_get_visual_configs_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_get_visual_configs (c : *connection,
                                      screen :  u32) -> get_visual_configs_cookie;

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
unsafe fn xcb_glx_get_visual_configs_unchecked (c : *connection,
                                                screen :  u32) -> get_visual_configs_cookie;

unsafe fn xcb_glx_get_visual_configs_property_list (R : *get_visual_configs_reply) -> *u32;


unsafe fn xcb_glx_get_visual_configs_property_list_length (R : *get_visual_configs_reply) -> c_int;


unsafe fn xcb_glx_get_visual_configs_property_list_end (R : *get_visual_configs_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_visual_configs_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_glx_get_visual_configs_reply (c : *connection,
                                            cookie : get_visual_configs_cookie,
                                            e : **generic_error) -> *get_visual_configs_reply;

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
unsafe fn xcb_glx_destroy_glx_pixmap_checked (c : *connection,
                                              glx_pixmap :  pixmap) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_destroy_glx_pixmap (c : *connection,
                                      glx_pixmap :  pixmap) -> void_cookie;

unsafe fn xcb_glx_vendor_private_sizeof (_buffer :  *c_void,
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
unsafe fn xcb_glx_vendor_private_checked (c : *connection,
                                          vendor_code :  u32,
                                          context_tag :  context_tag,
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
unsafe fn xcb_glx_vendor_private (c : *connection,
                                  vendor_code :  u32,
                                  context_tag :  context_tag,
                                  data_len :  u32,
                                  data : *u8) -> void_cookie;

unsafe fn xcb_glx_vendor_private_with_reply_sizeof (_buffer :  *c_void,
                                          data_len :  u32) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_vendor_private_with_reply (c : *connection,
                                             vendor_code :  u32,
                                             context_tag :  context_tag,
                                             data_len :  u32,
                                             data : *u8) -> vendor_private_with_reply_cookie;

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
unsafe fn xcb_glx_vendor_private_with_reply_unchecked (c : *connection,
                                                       vendor_code :  u32,
                                                       context_tag :  context_tag,
                                                       data_len :  u32,
                                                       data : *u8) -> vendor_private_with_reply_cookie;

unsafe fn xcb_glx_vendor_private_with_reply_data_2 (R : *vendor_private_with_reply_reply) -> *u8;


unsafe fn xcb_glx_vendor_private_with_reply_data_2_length (R : *vendor_private_with_reply_reply) -> c_int;


unsafe fn xcb_glx_vendor_private_with_reply_data_2_end (R : *vendor_private_with_reply_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_vendor_private_with_reply_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_glx_vendor_private_with_reply_reply (c : *connection,
                                                   cookie : vendor_private_with_reply_cookie,
                                                   e : **generic_error) -> *vendor_private_with_reply_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_query_extensions_string (c : *connection,
                                           screen :  u32) -> query_extensions_string_cookie;

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
unsafe fn xcb_glx_query_extensions_string_unchecked (c : *connection,
                                                     screen :  u32) -> query_extensions_string_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_query_extensions_string_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_glx_query_extensions_string_reply (c : *connection,
                                                 cookie : query_extensions_string_cookie,
                                                 e : **generic_error) -> *query_extensions_string_reply;

unsafe fn xcb_glx_query_server_string_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_query_server_string (c : *connection,
                                       screen :  u32,
                                       name :  u32) -> query_server_string_cookie;

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
unsafe fn xcb_glx_query_server_string_unchecked (c : *connection,
                                                 screen :  u32,
                                                 name :  u32) -> query_server_string_cookie;

unsafe fn xcb_glx_query_server_string_string (R : *query_server_string_reply) -> *u8;


unsafe fn xcb_glx_query_server_string_string_length (R : *query_server_string_reply) -> c_int;


unsafe fn xcb_glx_query_server_string_string_end (R : *query_server_string_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_query_server_string_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_glx_query_server_string_reply (c : *connection,
                                             cookie : query_server_string_cookie,
                                             e : **generic_error) -> *query_server_string_reply;

unsafe fn xcb_glx_client_info_sizeof (_buffer :  *c_void) -> c_int;

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
unsafe fn xcb_glx_client_info_checked (c : *connection,
                                       major_version :  u32,
                                       minor_version :  u32,
                                       str_len :  u32,
                                       string : *u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_client_info (c : *connection,
                               major_version :  u32,
                               minor_version :  u32,
                               str_len :  u32,
                               string : *u8) -> void_cookie;

unsafe fn xcb_glx_get_fb_configs_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_get_fb_configs (c : *connection,
                                  screen :  u32) -> get_fb_configs_cookie;

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
unsafe fn xcb_glx_get_fb_configs_unchecked (c : *connection,
                                            screen :  u32) -> get_fb_configs_cookie;

unsafe fn xcb_glx_get_fb_configs_property_list (R : *get_fb_configs_reply) -> *u32;


unsafe fn xcb_glx_get_fb_configs_property_list_length (R : *get_fb_configs_reply) -> c_int;


unsafe fn xcb_glx_get_fb_configs_property_list_end (R : *get_fb_configs_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_fb_configs_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_glx_get_fb_configs_reply (c : *connection,
                                        cookie : get_fb_configs_cookie,
                                        e : **generic_error) -> *get_fb_configs_reply;

unsafe fn xcb_glx_create_pixmap_sizeof (_buffer :  *c_void) -> c_int;

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
unsafe fn xcb_glx_create_pixmap_checked (c : *connection,
                                         screen :  u32,
                                         fbconfig :  fbconfig,
                                         pixmap :  xproto::pixmap,
                                         glx_pixmap :  pixmap,
                                         num_attribs :  u32,
                                         attribs : *u32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_create_pixmap (c : *connection,
                                 screen :  u32,
                                 fbconfig :  fbconfig,
                                 pixmap :  xproto::pixmap,
                                 glx_pixmap :  pixmap,
                                 num_attribs :  u32,
                                 attribs : *u32) -> void_cookie;

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
unsafe fn xcb_glx_destroy_pixmap_checked (c : *connection,
                                          glx_pixmap :  pixmap) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_destroy_pixmap (c : *connection,
                                  glx_pixmap :  pixmap) -> void_cookie;

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
unsafe fn xcb_glx_create_new_context_checked (c : *connection,
                                              context :  context,
                                              fbconfig :  fbconfig,
                                              screen :  u32,
                                              render_type :  u32,
                                              share_list :  context,
                                              is_direct :  u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_create_new_context (c : *connection,
                                      context :  context,
                                      fbconfig :  fbconfig,
                                      screen :  u32,
                                      render_type :  u32,
                                      share_list :  context,
                                      is_direct :  u8) -> void_cookie;

unsafe fn xcb_glx_query_context_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_query_context (c : *connection,
                                 context :  context) -> query_context_cookie;

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
unsafe fn xcb_glx_query_context_unchecked (c : *connection,
                                           context :  context) -> query_context_cookie;

unsafe fn xcb_glx_query_context_attribs (R : *query_context_reply) -> *u32;


unsafe fn xcb_glx_query_context_attribs_length (R : *query_context_reply) -> c_int;


unsafe fn xcb_glx_query_context_attribs_end (R : *query_context_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_query_context_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_glx_query_context_reply (c : *connection,
                                       cookie : query_context_cookie,
                                       e : **generic_error) -> *query_context_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_make_context_current (c : *connection,
                                        old_context_tag :  context_tag,
                                        drawable :  drawable,
                                        read_drawable :  drawable,
                                        context :  context) -> make_context_current_cookie;

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
unsafe fn xcb_glx_make_context_current_unchecked (c : *connection,
                                                  old_context_tag :  context_tag,
                                                  drawable :  drawable,
                                                  read_drawable :  drawable,
                                                  context :  context) -> make_context_current_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_make_context_current_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_glx_make_context_current_reply (c : *connection,
                                              cookie : make_context_current_cookie,
                                              e : **generic_error) -> *make_context_current_reply;

unsafe fn xcb_glx_create_pbuffer_sizeof (_buffer :  *c_void) -> c_int;

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
unsafe fn xcb_glx_create_pbuffer_checked (c : *connection,
                                          screen :  u32,
                                          fbconfig :  fbconfig,
                                          pbuffer :  pbuffer,
                                          num_attribs :  u32,
                                          attribs : *u32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_create_pbuffer (c : *connection,
                                  screen :  u32,
                                  fbconfig :  fbconfig,
                                  pbuffer :  pbuffer,
                                  num_attribs :  u32,
                                  attribs : *u32) -> void_cookie;

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
unsafe fn xcb_glx_destroy_pbuffer_checked (c : *connection,
                                           pbuffer :  pbuffer) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_destroy_pbuffer (c : *connection,
                                   pbuffer :  pbuffer) -> void_cookie;

unsafe fn xcb_glx_get_drawable_attributes_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_get_drawable_attributes (c : *connection,
                                           drawable :  drawable) -> get_drawable_attributes_cookie;

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
unsafe fn xcb_glx_get_drawable_attributes_unchecked (c : *connection,
                                                     drawable :  drawable) -> get_drawable_attributes_cookie;

unsafe fn xcb_glx_get_drawable_attributes_attribs (R : *get_drawable_attributes_reply) -> *u32;


unsafe fn xcb_glx_get_drawable_attributes_attribs_length (R : *get_drawable_attributes_reply) -> c_int;


unsafe fn xcb_glx_get_drawable_attributes_attribs_end (R : *get_drawable_attributes_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_drawable_attributes_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_glx_get_drawable_attributes_reply (c : *connection,
                                                 cookie : get_drawable_attributes_cookie,
                                                 e : **generic_error) -> *get_drawable_attributes_reply;

unsafe fn xcb_glx_change_drawable_attributes_sizeof (_buffer :  *c_void) -> c_int;

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
unsafe fn xcb_glx_change_drawable_attributes_checked (c : *connection,
                                                      drawable :  drawable,
                                                      num_attribs :  u32,
                                                      attribs : *u32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_change_drawable_attributes (c : *connection,
                                              drawable :  drawable,
                                              num_attribs :  u32,
                                              attribs : *u32) -> void_cookie;

unsafe fn xcb_glx_create_window_sizeof (_buffer :  *c_void) -> c_int;

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
unsafe fn xcb_glx_create_window_checked (c : *connection,
                                         screen :  u32,
                                         fbconfig :  fbconfig,
                                         window :  xproto::window,
                                         glx_window :  window,
                                         num_attribs :  u32,
                                         attribs : *u32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_create_window (c : *connection,
                                 screen :  u32,
                                 fbconfig :  fbconfig,
                                 window :  xproto::window,
                                 glx_window :  window,
                                 num_attribs :  u32,
                                 attribs : *u32) -> void_cookie;

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
unsafe fn xcb_glx_delete_window_checked (c : *connection,
                                         glxwindow :  window) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_delete_window (c : *connection,
                                 glxwindow :  window) -> void_cookie;

unsafe fn xcb_glx_set_client_info_arb_sizeof (_buffer :  *c_void) -> c_int;

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
unsafe fn xcb_glx_set_client_info_arb_checked (c : *connection,
                                               major_version :  u32,
                                               minor_version :  u32,
                                               num_versions :  u32,
                                               gl_str_len :  u32,
                                               glx_str_len :  u32,
                                               gl_versions : *u32,
                                               gl_extension_string : *u8,
                                               glx_extension_string : *u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_set_client_info_arb (c : *connection,
                                       major_version :  u32,
                                       minor_version :  u32,
                                       num_versions :  u32,
                                       gl_str_len :  u32,
                                       glx_str_len :  u32,
                                       gl_versions : *u32,
                                       gl_extension_string : *u8,
                                       glx_extension_string : *u8) -> void_cookie;

unsafe fn xcb_glx_create_context_attribs_arb_sizeof (_buffer :  *c_void) -> c_int;

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
unsafe fn xcb_glx_create_context_attribs_arb_checked (c : *connection,
                                                      context :  context,
                                                      fbconfig :  fbconfig,
                                                      screen :  u32,
                                                      share_list :  context,
                                                      is_direct :  u8,
                                                      num_attribs :  u32,
                                                      attribs : *u32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_create_context_attribs_arb (c : *connection,
                                              context :  context,
                                              fbconfig :  fbconfig,
                                              screen :  u32,
                                              share_list :  context,
                                              is_direct :  u8,
                                              num_attribs :  u32,
                                              attribs : *u32) -> void_cookie;

unsafe fn xcb_glx_set_client_info_2arb_sizeof (_buffer :  *c_void) -> c_int;

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
unsafe fn xcb_glx_set_client_info_2arb_checked (c : *connection,
                                                major_version :  u32,
                                                minor_version :  u32,
                                                num_versions :  u32,
                                                gl_str_len :  u32,
                                                glx_str_len :  u32,
                                                gl_versions : *u32,
                                                gl_extension_string : *u8,
                                                glx_extension_string : *u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_set_client_info_2arb (c : *connection,
                                        major_version :  u32,
                                        minor_version :  u32,
                                        num_versions :  u32,
                                        gl_str_len :  u32,
                                        glx_str_len :  u32,
                                        gl_versions : *u32,
                                        gl_extension_string : *u8,
                                        glx_extension_string : *u8) -> void_cookie;

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
unsafe fn xcb_glx_new_list_checked (c : *connection,
                                    context_tag :  context_tag,
                                    list :  u32,
                                    mode :  u32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_new_list (c : *connection,
                            context_tag :  context_tag,
                            list :  u32,
                            mode :  u32) -> void_cookie;

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
unsafe fn xcb_glx_end_list_checked (c : *connection,
                                    context_tag :  context_tag) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_end_list (c : *connection,
                            context_tag :  context_tag) -> void_cookie;

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
unsafe fn xcb_glx_delete_lists_checked (c : *connection,
                                        context_tag :  context_tag,
                                        list :  u32,
                                        range :  i32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_delete_lists (c : *connection,
                                context_tag :  context_tag,
                                list :  u32,
                                range :  i32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_gen_lists (c : *connection,
                             context_tag :  context_tag,
                             range :  i32) -> gen_lists_cookie;

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
unsafe fn xcb_glx_gen_lists_unchecked (c : *connection,
                                       context_tag :  context_tag,
                                       range :  i32) -> gen_lists_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_gen_lists_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_glx_gen_lists_reply (c : *connection,
                                   cookie : gen_lists_cookie,
                                   e : **generic_error) -> *gen_lists_reply;

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
unsafe fn xcb_glx_feedback_buffer_checked (c : *connection,
                                           context_tag :  context_tag,
                                           size :  i32,
                                           type_ :  i32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_feedback_buffer (c : *connection,
                                   context_tag :  context_tag,
                                   size :  i32,
                                   type_ :  i32) -> void_cookie;

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
unsafe fn xcb_glx_select_buffer_checked (c : *connection,
                                         context_tag :  context_tag,
                                         size :  i32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_select_buffer (c : *connection,
                                 context_tag :  context_tag,
                                 size :  i32) -> void_cookie;

unsafe fn xcb_glx_render_mode_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_render_mode (c : *connection,
                               context_tag :  context_tag,
                               mode :  u32) -> render_mode_cookie;

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
unsafe fn xcb_glx_render_mode_unchecked (c : *connection,
                                         context_tag :  context_tag,
                                         mode :  u32) -> render_mode_cookie;

unsafe fn xcb_glx_render_mode_data (R : *render_mode_reply) -> *u32;


unsafe fn xcb_glx_render_mode_data_length (R : *render_mode_reply) -> c_int;


unsafe fn xcb_glx_render_mode_data_end (R : *render_mode_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_render_mode_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_glx_render_mode_reply (c : *connection,
                                     cookie : render_mode_cookie,
                                     e : **generic_error) -> *render_mode_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_finish (c : *connection,
                          context_tag :  context_tag) -> finish_cookie;

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
unsafe fn xcb_glx_finish_unchecked (c : *connection,
                                    context_tag :  context_tag) -> finish_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_finish_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_glx_finish_reply (c : *connection,
                                cookie : finish_cookie,
                                e : **generic_error) -> *finish_reply;

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
unsafe fn xcb_glx_pixel_storef_checked (c : *connection,
                                        context_tag :  context_tag,
                                        pname :  u32,
                                        datum :  float32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_pixel_storef (c : *connection,
                                context_tag :  context_tag,
                                pname :  u32,
                                datum :  float32) -> void_cookie;

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
unsafe fn xcb_glx_pixel_storei_checked (c : *connection,
                                        context_tag :  context_tag,
                                        pname :  u32,
                                        datum :  i32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_pixel_storei (c : *connection,
                                context_tag :  context_tag,
                                pname :  u32,
                                datum :  i32) -> void_cookie;

unsafe fn xcb_glx_read_pixels_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_read_pixels (c : *connection,
                               context_tag :  context_tag,
                               x :  i32,
                               y :  i32,
                               width :  i32,
                               height :  i32,
                               format :  u32,
                               type_ :  u32,
                               swap_bytes :  u8,
                               lsb_first :  u8) -> read_pixels_cookie;

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
unsafe fn xcb_glx_read_pixels_unchecked (c : *connection,
                                         context_tag :  context_tag,
                                         x :  i32,
                                         y :  i32,
                                         width :  i32,
                                         height :  i32,
                                         format :  u32,
                                         type_ :  u32,
                                         swap_bytes :  u8,
                                         lsb_first :  u8) -> read_pixels_cookie;

unsafe fn xcb_glx_read_pixels_data (R : *read_pixels_reply) -> *u8;


unsafe fn xcb_glx_read_pixels_data_length (R : *read_pixels_reply) -> c_int;


unsafe fn xcb_glx_read_pixels_data_end (R : *read_pixels_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_read_pixels_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_glx_read_pixels_reply (c : *connection,
                                     cookie : read_pixels_cookie,
                                     e : **generic_error) -> *read_pixels_reply;

unsafe fn xcb_glx_get_booleanv_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_get_booleanv (c : *connection,
                                context_tag :  context_tag,
                                pname :  i32) -> get_booleanv_cookie;

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
unsafe fn xcb_glx_get_booleanv_unchecked (c : *connection,
                                          context_tag :  context_tag,
                                          pname :  i32) -> get_booleanv_cookie;

unsafe fn xcb_glx_get_booleanv_data (R : *get_booleanv_reply) -> *u8;


unsafe fn xcb_glx_get_booleanv_data_length (R : *get_booleanv_reply) -> c_int;


unsafe fn xcb_glx_get_booleanv_data_end (R : *get_booleanv_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_booleanv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_glx_get_booleanv_reply (c : *connection,
                                      cookie : get_booleanv_cookie,
                                      e : **generic_error) -> *get_booleanv_reply;

unsafe fn xcb_glx_get_clip_plane_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_get_clip_plane (c : *connection,
                                  context_tag :  context_tag,
                                  plane :  i32) -> get_clip_plane_cookie;

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
unsafe fn xcb_glx_get_clip_plane_unchecked (c : *connection,
                                            context_tag :  context_tag,
                                            plane :  i32) -> get_clip_plane_cookie;

unsafe fn xcb_glx_get_clip_plane_data (R : *get_clip_plane_reply) -> *float64;


unsafe fn xcb_glx_get_clip_plane_data_length (R : *get_clip_plane_reply) -> c_int;


unsafe fn xcb_glx_get_clip_plane_data_end (R : *get_clip_plane_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_clip_plane_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_glx_get_clip_plane_reply (c : *connection,
                                        cookie : get_clip_plane_cookie,
                                        e : **generic_error) -> *get_clip_plane_reply;

unsafe fn xcb_glx_get_doublev_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_get_doublev (c : *connection,
                               context_tag :  context_tag,
                               pname :  u32) -> get_doublev_cookie;

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
unsafe fn xcb_glx_get_doublev_unchecked (c : *connection,
                                         context_tag :  context_tag,
                                         pname :  u32) -> get_doublev_cookie;

unsafe fn xcb_glx_get_doublev_data (R : *get_doublev_reply) -> *float64;


unsafe fn xcb_glx_get_doublev_data_length (R : *get_doublev_reply) -> c_int;


unsafe fn xcb_glx_get_doublev_data_end (R : *get_doublev_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_doublev_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_glx_get_doublev_reply (c : *connection,
                                     cookie : get_doublev_cookie,
                                     e : **generic_error) -> *get_doublev_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_get_error (c : *connection,
                             context_tag :  context_tag) -> get_error_cookie;

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
unsafe fn xcb_glx_get_error_unchecked (c : *connection,
                                       context_tag :  context_tag) -> get_error_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_error_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_glx_get_error_reply (c : *connection,
                                   cookie : get_error_cookie,
                                   e : **generic_error) -> *get_error_reply;

unsafe fn xcb_glx_get_floatv_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_get_floatv (c : *connection,
                              context_tag :  context_tag,
                              pname :  u32) -> get_floatv_cookie;

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
unsafe fn xcb_glx_get_floatv_unchecked (c : *connection,
                                        context_tag :  context_tag,
                                        pname :  u32) -> get_floatv_cookie;

unsafe fn xcb_glx_get_floatv_data (R : *get_floatv_reply) -> *float32;


unsafe fn xcb_glx_get_floatv_data_length (R : *get_floatv_reply) -> c_int;


unsafe fn xcb_glx_get_floatv_data_end (R : *get_floatv_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_floatv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_glx_get_floatv_reply (c : *connection,
                                    cookie : get_floatv_cookie,
                                    e : **generic_error) -> *get_floatv_reply;

unsafe fn xcb_glx_get_integerv_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_get_integerv (c : *connection,
                                context_tag :  context_tag,
                                pname :  u32) -> get_integerv_cookie;

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
unsafe fn xcb_glx_get_integerv_unchecked (c : *connection,
                                          context_tag :  context_tag,
                                          pname :  u32) -> get_integerv_cookie;

unsafe fn xcb_glx_get_integerv_data (R : *get_integerv_reply) -> *i32;


unsafe fn xcb_glx_get_integerv_data_length (R : *get_integerv_reply) -> c_int;


unsafe fn xcb_glx_get_integerv_data_end (R : *get_integerv_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_integerv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_glx_get_integerv_reply (c : *connection,
                                      cookie : get_integerv_cookie,
                                      e : **generic_error) -> *get_integerv_reply;

unsafe fn xcb_glx_get_lightfv_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_get_lightfv (c : *connection,
                               context_tag :  context_tag,
                               light :  u32,
                               pname :  u32) -> get_lightfv_cookie;

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
unsafe fn xcb_glx_get_lightfv_unchecked (c : *connection,
                                         context_tag :  context_tag,
                                         light :  u32,
                                         pname :  u32) -> get_lightfv_cookie;

unsafe fn xcb_glx_get_lightfv_data (R : *get_lightfv_reply) -> *float32;


unsafe fn xcb_glx_get_lightfv_data_length (R : *get_lightfv_reply) -> c_int;


unsafe fn xcb_glx_get_lightfv_data_end (R : *get_lightfv_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_lightfv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_glx_get_lightfv_reply (c : *connection,
                                     cookie : get_lightfv_cookie,
                                     e : **generic_error) -> *get_lightfv_reply;

unsafe fn xcb_glx_get_lightiv_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_get_lightiv (c : *connection,
                               context_tag :  context_tag,
                               light :  u32,
                               pname :  u32) -> get_lightiv_cookie;

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
unsafe fn xcb_glx_get_lightiv_unchecked (c : *connection,
                                         context_tag :  context_tag,
                                         light :  u32,
                                         pname :  u32) -> get_lightiv_cookie;

unsafe fn xcb_glx_get_lightiv_data (R : *get_lightiv_reply) -> *i32;


unsafe fn xcb_glx_get_lightiv_data_length (R : *get_lightiv_reply) -> c_int;


unsafe fn xcb_glx_get_lightiv_data_end (R : *get_lightiv_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_lightiv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_glx_get_lightiv_reply (c : *connection,
                                     cookie : get_lightiv_cookie,
                                     e : **generic_error) -> *get_lightiv_reply;

unsafe fn xcb_glx_get_mapdv_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_get_mapdv (c : *connection,
                             context_tag :  context_tag,
                             target :  u32,
                             query :  u32) -> get_mapdv_cookie;

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
unsafe fn xcb_glx_get_mapdv_unchecked (c : *connection,
                                       context_tag :  context_tag,
                                       target :  u32,
                                       query :  u32) -> get_mapdv_cookie;

unsafe fn xcb_glx_get_mapdv_data (R : *get_mapdv_reply) -> *float64;


unsafe fn xcb_glx_get_mapdv_data_length (R : *get_mapdv_reply) -> c_int;


unsafe fn xcb_glx_get_mapdv_data_end (R : *get_mapdv_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_mapdv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_glx_get_mapdv_reply (c : *connection,
                                   cookie : get_mapdv_cookie,
                                   e : **generic_error) -> *get_mapdv_reply;

unsafe fn xcb_glx_get_mapfv_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_get_mapfv (c : *connection,
                             context_tag :  context_tag,
                             target :  u32,
                             query :  u32) -> get_mapfv_cookie;

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
unsafe fn xcb_glx_get_mapfv_unchecked (c : *connection,
                                       context_tag :  context_tag,
                                       target :  u32,
                                       query :  u32) -> get_mapfv_cookie;

unsafe fn xcb_glx_get_mapfv_data (R : *get_mapfv_reply) -> *float32;


unsafe fn xcb_glx_get_mapfv_data_length (R : *get_mapfv_reply) -> c_int;


unsafe fn xcb_glx_get_mapfv_data_end (R : *get_mapfv_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_mapfv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_glx_get_mapfv_reply (c : *connection,
                                   cookie : get_mapfv_cookie,
                                   e : **generic_error) -> *get_mapfv_reply;

unsafe fn xcb_glx_get_mapiv_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_get_mapiv (c : *connection,
                             context_tag :  context_tag,
                             target :  u32,
                             query :  u32) -> get_mapiv_cookie;

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
unsafe fn xcb_glx_get_mapiv_unchecked (c : *connection,
                                       context_tag :  context_tag,
                                       target :  u32,
                                       query :  u32) -> get_mapiv_cookie;

unsafe fn xcb_glx_get_mapiv_data (R : *get_mapiv_reply) -> *i32;


unsafe fn xcb_glx_get_mapiv_data_length (R : *get_mapiv_reply) -> c_int;


unsafe fn xcb_glx_get_mapiv_data_end (R : *get_mapiv_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_mapiv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_glx_get_mapiv_reply (c : *connection,
                                   cookie : get_mapiv_cookie,
                                   e : **generic_error) -> *get_mapiv_reply;

unsafe fn xcb_glx_get_materialfv_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_get_materialfv (c : *connection,
                                  context_tag :  context_tag,
                                  face :  u32,
                                  pname :  u32) -> get_materialfv_cookie;

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
unsafe fn xcb_glx_get_materialfv_unchecked (c : *connection,
                                            context_tag :  context_tag,
                                            face :  u32,
                                            pname :  u32) -> get_materialfv_cookie;

unsafe fn xcb_glx_get_materialfv_data (R : *get_materialfv_reply) -> *float32;


unsafe fn xcb_glx_get_materialfv_data_length (R : *get_materialfv_reply) -> c_int;


unsafe fn xcb_glx_get_materialfv_data_end (R : *get_materialfv_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_materialfv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_glx_get_materialfv_reply (c : *connection,
                                        cookie : get_materialfv_cookie,
                                        e : **generic_error) -> *get_materialfv_reply;

unsafe fn xcb_glx_get_materialiv_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_get_materialiv (c : *connection,
                                  context_tag :  context_tag,
                                  face :  u32,
                                  pname :  u32) -> get_materialiv_cookie;

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
unsafe fn xcb_glx_get_materialiv_unchecked (c : *connection,
                                            context_tag :  context_tag,
                                            face :  u32,
                                            pname :  u32) -> get_materialiv_cookie;

unsafe fn xcb_glx_get_materialiv_data (R : *get_materialiv_reply) -> *i32;


unsafe fn xcb_glx_get_materialiv_data_length (R : *get_materialiv_reply) -> c_int;


unsafe fn xcb_glx_get_materialiv_data_end (R : *get_materialiv_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_materialiv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_glx_get_materialiv_reply (c : *connection,
                                        cookie : get_materialiv_cookie,
                                        e : **generic_error) -> *get_materialiv_reply;

unsafe fn xcb_glx_get_pixel_mapfv_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_get_pixel_mapfv (c : *connection,
                                   context_tag :  context_tag,
                                   map :  u32) -> get_pixel_mapfv_cookie;

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
unsafe fn xcb_glx_get_pixel_mapfv_unchecked (c : *connection,
                                             context_tag :  context_tag,
                                             map :  u32) -> get_pixel_mapfv_cookie;

unsafe fn xcb_glx_get_pixel_mapfv_data (R : *get_pixel_mapfv_reply) -> *float32;


unsafe fn xcb_glx_get_pixel_mapfv_data_length (R : *get_pixel_mapfv_reply) -> c_int;


unsafe fn xcb_glx_get_pixel_mapfv_data_end (R : *get_pixel_mapfv_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_pixel_mapfv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_glx_get_pixel_mapfv_reply (c : *connection,
                                         cookie : get_pixel_mapfv_cookie,
                                         e : **generic_error) -> *get_pixel_mapfv_reply;

unsafe fn xcb_glx_get_pixel_mapuiv_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_get_pixel_mapuiv (c : *connection,
                                    context_tag :  context_tag,
                                    map :  u32) -> get_pixel_mapuiv_cookie;

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
unsafe fn xcb_glx_get_pixel_mapuiv_unchecked (c : *connection,
                                              context_tag :  context_tag,
                                              map :  u32) -> get_pixel_mapuiv_cookie;

unsafe fn xcb_glx_get_pixel_mapuiv_data (R : *get_pixel_mapuiv_reply) -> *u32;


unsafe fn xcb_glx_get_pixel_mapuiv_data_length (R : *get_pixel_mapuiv_reply) -> c_int;


unsafe fn xcb_glx_get_pixel_mapuiv_data_end (R : *get_pixel_mapuiv_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_pixel_mapuiv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_glx_get_pixel_mapuiv_reply (c : *connection,
                                          cookie : get_pixel_mapuiv_cookie,
                                          e : **generic_error) -> *get_pixel_mapuiv_reply;

unsafe fn xcb_glx_get_pixel_mapusv_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_get_pixel_mapusv (c : *connection,
                                    context_tag :  context_tag,
                                    map :  u32) -> get_pixel_mapusv_cookie;

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
unsafe fn xcb_glx_get_pixel_mapusv_unchecked (c : *connection,
                                              context_tag :  context_tag,
                                              map :  u32) -> get_pixel_mapusv_cookie;

unsafe fn xcb_glx_get_pixel_mapusv_data (R : *get_pixel_mapusv_reply) -> *u16;


unsafe fn xcb_glx_get_pixel_mapusv_data_length (R : *get_pixel_mapusv_reply) -> c_int;


unsafe fn xcb_glx_get_pixel_mapusv_data_end (R : *get_pixel_mapusv_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_pixel_mapusv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_glx_get_pixel_mapusv_reply (c : *connection,
                                          cookie : get_pixel_mapusv_cookie,
                                          e : **generic_error) -> *get_pixel_mapusv_reply;

unsafe fn xcb_glx_get_polygon_stipple_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_get_polygon_stipple (c : *connection,
                                       context_tag :  context_tag,
                                       lsb_first :  u8) -> get_polygon_stipple_cookie;

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
unsafe fn xcb_glx_get_polygon_stipple_unchecked (c : *connection,
                                                 context_tag :  context_tag,
                                                 lsb_first :  u8) -> get_polygon_stipple_cookie;

unsafe fn xcb_glx_get_polygon_stipple_data (R : *get_polygon_stipple_reply) -> *u8;


unsafe fn xcb_glx_get_polygon_stipple_data_length (R : *get_polygon_stipple_reply) -> c_int;


unsafe fn xcb_glx_get_polygon_stipple_data_end (R : *get_polygon_stipple_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_polygon_stipple_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_glx_get_polygon_stipple_reply (c : *connection,
                                             cookie : get_polygon_stipple_cookie,
                                             e : **generic_error) -> *get_polygon_stipple_reply;

unsafe fn xcb_glx_get_string_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_get_string (c : *connection,
                              context_tag :  context_tag,
                              name :  u32) -> get_string_cookie;

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
unsafe fn xcb_glx_get_string_unchecked (c : *connection,
                                        context_tag :  context_tag,
                                        name :  u32) -> get_string_cookie;

unsafe fn xcb_glx_get_string_string (R : *get_string_reply) -> *u8;


unsafe fn xcb_glx_get_string_string_length (R : *get_string_reply) -> c_int;


unsafe fn xcb_glx_get_string_string_end (R : *get_string_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_string_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_glx_get_string_reply (c : *connection,
                                    cookie : get_string_cookie,
                                    e : **generic_error) -> *get_string_reply;

unsafe fn xcb_glx_get_tex_envfv_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_get_tex_envfv (c : *connection,
                                 context_tag :  context_tag,
                                 target :  u32,
                                 pname :  u32) -> get_tex_envfv_cookie;

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
unsafe fn xcb_glx_get_tex_envfv_unchecked (c : *connection,
                                           context_tag :  context_tag,
                                           target :  u32,
                                           pname :  u32) -> get_tex_envfv_cookie;

unsafe fn xcb_glx_get_tex_envfv_data (R : *get_tex_envfv_reply) -> *float32;


unsafe fn xcb_glx_get_tex_envfv_data_length (R : *get_tex_envfv_reply) -> c_int;


unsafe fn xcb_glx_get_tex_envfv_data_end (R : *get_tex_envfv_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_tex_envfv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_glx_get_tex_envfv_reply (c : *connection,
                                       cookie : get_tex_envfv_cookie,
                                       e : **generic_error) -> *get_tex_envfv_reply;

unsafe fn xcb_glx_get_tex_enviv_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_get_tex_enviv (c : *connection,
                                 context_tag :  context_tag,
                                 target :  u32,
                                 pname :  u32) -> get_tex_enviv_cookie;

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
unsafe fn xcb_glx_get_tex_enviv_unchecked (c : *connection,
                                           context_tag :  context_tag,
                                           target :  u32,
                                           pname :  u32) -> get_tex_enviv_cookie;

unsafe fn xcb_glx_get_tex_enviv_data (R : *get_tex_enviv_reply) -> *i32;


unsafe fn xcb_glx_get_tex_enviv_data_length (R : *get_tex_enviv_reply) -> c_int;


unsafe fn xcb_glx_get_tex_enviv_data_end (R : *get_tex_enviv_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_tex_enviv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_glx_get_tex_enviv_reply (c : *connection,
                                       cookie : get_tex_enviv_cookie,
                                       e : **generic_error) -> *get_tex_enviv_reply;

unsafe fn xcb_glx_get_tex_gendv_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_get_tex_gendv (c : *connection,
                                 context_tag :  context_tag,
                                 coord :  u32,
                                 pname :  u32) -> get_tex_gendv_cookie;

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
unsafe fn xcb_glx_get_tex_gendv_unchecked (c : *connection,
                                           context_tag :  context_tag,
                                           coord :  u32,
                                           pname :  u32) -> get_tex_gendv_cookie;

unsafe fn xcb_glx_get_tex_gendv_data (R : *get_tex_gendv_reply) -> *float64;


unsafe fn xcb_glx_get_tex_gendv_data_length (R : *get_tex_gendv_reply) -> c_int;


unsafe fn xcb_glx_get_tex_gendv_data_end (R : *get_tex_gendv_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_tex_gendv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_glx_get_tex_gendv_reply (c : *connection,
                                       cookie : get_tex_gendv_cookie,
                                       e : **generic_error) -> *get_tex_gendv_reply;

unsafe fn xcb_glx_get_tex_genfv_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_get_tex_genfv (c : *connection,
                                 context_tag :  context_tag,
                                 coord :  u32,
                                 pname :  u32) -> get_tex_genfv_cookie;

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
unsafe fn xcb_glx_get_tex_genfv_unchecked (c : *connection,
                                           context_tag :  context_tag,
                                           coord :  u32,
                                           pname :  u32) -> get_tex_genfv_cookie;

unsafe fn xcb_glx_get_tex_genfv_data (R : *get_tex_genfv_reply) -> *float32;


unsafe fn xcb_glx_get_tex_genfv_data_length (R : *get_tex_genfv_reply) -> c_int;


unsafe fn xcb_glx_get_tex_genfv_data_end (R : *get_tex_genfv_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_tex_genfv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_glx_get_tex_genfv_reply (c : *connection,
                                       cookie : get_tex_genfv_cookie,
                                       e : **generic_error) -> *get_tex_genfv_reply;

unsafe fn xcb_glx_get_tex_geniv_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_get_tex_geniv (c : *connection,
                                 context_tag :  context_tag,
                                 coord :  u32,
                                 pname :  u32) -> get_tex_geniv_cookie;

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
unsafe fn xcb_glx_get_tex_geniv_unchecked (c : *connection,
                                           context_tag :  context_tag,
                                           coord :  u32,
                                           pname :  u32) -> get_tex_geniv_cookie;

unsafe fn xcb_glx_get_tex_geniv_data (R : *get_tex_geniv_reply) -> *i32;


unsafe fn xcb_glx_get_tex_geniv_data_length (R : *get_tex_geniv_reply) -> c_int;


unsafe fn xcb_glx_get_tex_geniv_data_end (R : *get_tex_geniv_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_tex_geniv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_glx_get_tex_geniv_reply (c : *connection,
                                       cookie : get_tex_geniv_cookie,
                                       e : **generic_error) -> *get_tex_geniv_reply;

unsafe fn xcb_glx_get_tex_image_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_get_tex_image (c : *connection,
                                 context_tag :  context_tag,
                                 target :  u32,
                                 level :  i32,
                                 format :  u32,
                                 type_ :  u32,
                                 swap_bytes :  u8) -> get_tex_image_cookie;

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
unsafe fn xcb_glx_get_tex_image_unchecked (c : *connection,
                                           context_tag :  context_tag,
                                           target :  u32,
                                           level :  i32,
                                           format :  u32,
                                           type_ :  u32,
                                           swap_bytes :  u8) -> get_tex_image_cookie;

unsafe fn xcb_glx_get_tex_image_data (R : *get_tex_image_reply) -> *u8;


unsafe fn xcb_glx_get_tex_image_data_length (R : *get_tex_image_reply) -> c_int;


unsafe fn xcb_glx_get_tex_image_data_end (R : *get_tex_image_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_tex_image_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_glx_get_tex_image_reply (c : *connection,
                                       cookie : get_tex_image_cookie,
                                       e : **generic_error) -> *get_tex_image_reply;

unsafe fn xcb_glx_get_tex_parameterfv_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_get_tex_parameterfv (c : *connection,
                                       context_tag :  context_tag,
                                       target :  u32,
                                       pname :  u32) -> get_tex_parameterfv_cookie;

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
unsafe fn xcb_glx_get_tex_parameterfv_unchecked (c : *connection,
                                                 context_tag :  context_tag,
                                                 target :  u32,
                                                 pname :  u32) -> get_tex_parameterfv_cookie;

unsafe fn xcb_glx_get_tex_parameterfv_data (R : *get_tex_parameterfv_reply) -> *float32;


unsafe fn xcb_glx_get_tex_parameterfv_data_length (R : *get_tex_parameterfv_reply) -> c_int;


unsafe fn xcb_glx_get_tex_parameterfv_data_end (R : *get_tex_parameterfv_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_tex_parameterfv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_glx_get_tex_parameterfv_reply (c : *connection,
                                             cookie : get_tex_parameterfv_cookie,
                                             e : **generic_error) -> *get_tex_parameterfv_reply;

unsafe fn xcb_glx_get_tex_parameteriv_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_get_tex_parameteriv (c : *connection,
                                       context_tag :  context_tag,
                                       target :  u32,
                                       pname :  u32) -> get_tex_parameteriv_cookie;

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
unsafe fn xcb_glx_get_tex_parameteriv_unchecked (c : *connection,
                                                 context_tag :  context_tag,
                                                 target :  u32,
                                                 pname :  u32) -> get_tex_parameteriv_cookie;

unsafe fn xcb_glx_get_tex_parameteriv_data (R : *get_tex_parameteriv_reply) -> *i32;


unsafe fn xcb_glx_get_tex_parameteriv_data_length (R : *get_tex_parameteriv_reply) -> c_int;


unsafe fn xcb_glx_get_tex_parameteriv_data_end (R : *get_tex_parameteriv_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_tex_parameteriv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_glx_get_tex_parameteriv_reply (c : *connection,
                                             cookie : get_tex_parameteriv_cookie,
                                             e : **generic_error) -> *get_tex_parameteriv_reply;

unsafe fn xcb_glx_get_tex_level_parameterfv_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_get_tex_level_parameterfv (c : *connection,
                                             context_tag :  context_tag,
                                             target :  u32,
                                             level :  i32,
                                             pname :  u32) -> get_tex_level_parameterfv_cookie;

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
unsafe fn xcb_glx_get_tex_level_parameterfv_unchecked (c : *connection,
                                                       context_tag :  context_tag,
                                                       target :  u32,
                                                       level :  i32,
                                                       pname :  u32) -> get_tex_level_parameterfv_cookie;

unsafe fn xcb_glx_get_tex_level_parameterfv_data (R : *get_tex_level_parameterfv_reply) -> *float32;


unsafe fn xcb_glx_get_tex_level_parameterfv_data_length (R : *get_tex_level_parameterfv_reply) -> c_int;


unsafe fn xcb_glx_get_tex_level_parameterfv_data_end (R : *get_tex_level_parameterfv_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_tex_level_parameterfv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_glx_get_tex_level_parameterfv_reply (c : *connection,
                                                   cookie : get_tex_level_parameterfv_cookie,
                                                   e : **generic_error) -> *get_tex_level_parameterfv_reply;

unsafe fn xcb_glx_get_tex_level_parameteriv_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_get_tex_level_parameteriv (c : *connection,
                                             context_tag :  context_tag,
                                             target :  u32,
                                             level :  i32,
                                             pname :  u32) -> get_tex_level_parameteriv_cookie;

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
unsafe fn xcb_glx_get_tex_level_parameteriv_unchecked (c : *connection,
                                                       context_tag :  context_tag,
                                                       target :  u32,
                                                       level :  i32,
                                                       pname :  u32) -> get_tex_level_parameteriv_cookie;

unsafe fn xcb_glx_get_tex_level_parameteriv_data (R : *get_tex_level_parameteriv_reply) -> *i32;


unsafe fn xcb_glx_get_tex_level_parameteriv_data_length (R : *get_tex_level_parameteriv_reply) -> c_int;


unsafe fn xcb_glx_get_tex_level_parameteriv_data_end (R : *get_tex_level_parameteriv_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_tex_level_parameteriv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_glx_get_tex_level_parameteriv_reply (c : *connection,
                                                   cookie : get_tex_level_parameteriv_cookie,
                                                   e : **generic_error) -> *get_tex_level_parameteriv_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_is_list (c : *connection,
                           context_tag :  context_tag,
                           list :  u32) -> is_list_cookie;

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
unsafe fn xcb_glx_is_list_unchecked (c : *connection,
                                     context_tag :  context_tag,
                                     list :  u32) -> is_list_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_is_list_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_glx_is_list_reply (c : *connection,
                                 cookie : is_list_cookie,
                                 e : **generic_error) -> *is_list_reply;

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
unsafe fn xcb_glx_flush_checked (c : *connection,
                                 context_tag :  context_tag) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_flush (c : *connection,
                         context_tag :  context_tag) -> void_cookie;

unsafe fn xcb_glx_are_textures_resident_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_are_textures_resident (c : *connection,
                                         context_tag :  context_tag,
                                         n :  i32,
                                         textures : *u32) -> are_textures_resident_cookie;

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
unsafe fn xcb_glx_are_textures_resident_unchecked (c : *connection,
                                                   context_tag :  context_tag,
                                                   n :  i32,
                                                   textures : *u32) -> are_textures_resident_cookie;

unsafe fn xcb_glx_are_textures_resident_data (R : *are_textures_resident_reply) -> *u8;


unsafe fn xcb_glx_are_textures_resident_data_length (R : *are_textures_resident_reply) -> c_int;


unsafe fn xcb_glx_are_textures_resident_data_end (R : *are_textures_resident_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_are_textures_resident_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_glx_are_textures_resident_reply (c : *connection,
                                               cookie : are_textures_resident_cookie,
                                               e : **generic_error) -> *are_textures_resident_reply;

unsafe fn xcb_glx_delete_textures_sizeof (_buffer :  *c_void) -> c_int;

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
unsafe fn xcb_glx_delete_textures_checked (c : *connection,
                                           context_tag :  context_tag,
                                           n :  i32,
                                           textures : *u32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_delete_textures (c : *connection,
                                   context_tag :  context_tag,
                                   n :  i32,
                                   textures : *u32) -> void_cookie;

unsafe fn xcb_glx_gen_textures_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_gen_textures (c : *connection,
                                context_tag :  context_tag,
                                n :  i32) -> gen_textures_cookie;

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
unsafe fn xcb_glx_gen_textures_unchecked (c : *connection,
                                          context_tag :  context_tag,
                                          n :  i32) -> gen_textures_cookie;

unsafe fn xcb_glx_gen_textures_data (R : *gen_textures_reply) -> *u32;


unsafe fn xcb_glx_gen_textures_data_length (R : *gen_textures_reply) -> c_int;


unsafe fn xcb_glx_gen_textures_data_end (R : *gen_textures_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_gen_textures_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_glx_gen_textures_reply (c : *connection,
                                      cookie : gen_textures_cookie,
                                      e : **generic_error) -> *gen_textures_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_is_texture (c : *connection,
                              context_tag :  context_tag,
                              texture :  u32) -> is_texture_cookie;

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
unsafe fn xcb_glx_is_texture_unchecked (c : *connection,
                                        context_tag :  context_tag,
                                        texture :  u32) -> is_texture_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_is_texture_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_glx_is_texture_reply (c : *connection,
                                    cookie : is_texture_cookie,
                                    e : **generic_error) -> *is_texture_reply;

unsafe fn xcb_glx_get_color_table_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_get_color_table (c : *connection,
                                   context_tag :  context_tag,
                                   target :  u32,
                                   format :  u32,
                                   type_ :  u32,
                                   swap_bytes :  u8) -> get_color_table_cookie;

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
unsafe fn xcb_glx_get_color_table_unchecked (c : *connection,
                                             context_tag :  context_tag,
                                             target :  u32,
                                             format :  u32,
                                             type_ :  u32,
                                             swap_bytes :  u8) -> get_color_table_cookie;

unsafe fn xcb_glx_get_color_table_data (R : *get_color_table_reply) -> *u8;


unsafe fn xcb_glx_get_color_table_data_length (R : *get_color_table_reply) -> c_int;


unsafe fn xcb_glx_get_color_table_data_end (R : *get_color_table_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_color_table_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_glx_get_color_table_reply (c : *connection,
                                         cookie : get_color_table_cookie,
                                         e : **generic_error) -> *get_color_table_reply;

unsafe fn xcb_glx_get_color_table_parameterfv_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_get_color_table_parameterfv (c : *connection,
                                               context_tag :  context_tag,
                                               target :  u32,
                                               pname :  u32) -> get_color_table_parameterfv_cookie;

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
unsafe fn xcb_glx_get_color_table_parameterfv_unchecked (c : *connection,
                                                         context_tag :  context_tag,
                                                         target :  u32,
                                                         pname :  u32) -> get_color_table_parameterfv_cookie;

unsafe fn xcb_glx_get_color_table_parameterfv_data (R : *get_color_table_parameterfv_reply) -> *float32;


unsafe fn xcb_glx_get_color_table_parameterfv_data_length (R : *get_color_table_parameterfv_reply) -> c_int;


unsafe fn xcb_glx_get_color_table_parameterfv_data_end (R : *get_color_table_parameterfv_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_color_table_parameterfv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_glx_get_color_table_parameterfv_reply (c : *connection,
                                                     cookie : get_color_table_parameterfv_cookie,
                                                     e : **generic_error) -> *get_color_table_parameterfv_reply;

unsafe fn xcb_glx_get_color_table_parameteriv_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_get_color_table_parameteriv (c : *connection,
                                               context_tag :  context_tag,
                                               target :  u32,
                                               pname :  u32) -> get_color_table_parameteriv_cookie;

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
unsafe fn xcb_glx_get_color_table_parameteriv_unchecked (c : *connection,
                                                         context_tag :  context_tag,
                                                         target :  u32,
                                                         pname :  u32) -> get_color_table_parameteriv_cookie;

unsafe fn xcb_glx_get_color_table_parameteriv_data (R : *get_color_table_parameteriv_reply) -> *i32;


unsafe fn xcb_glx_get_color_table_parameteriv_data_length (R : *get_color_table_parameteriv_reply) -> c_int;


unsafe fn xcb_glx_get_color_table_parameteriv_data_end (R : *get_color_table_parameteriv_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_color_table_parameteriv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_glx_get_color_table_parameteriv_reply (c : *connection,
                                                     cookie : get_color_table_parameteriv_cookie,
                                                     e : **generic_error) -> *get_color_table_parameteriv_reply;

unsafe fn xcb_glx_get_convolution_filter_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_get_convolution_filter (c : *connection,
                                          context_tag :  context_tag,
                                          target :  u32,
                                          format :  u32,
                                          type_ :  u32,
                                          swap_bytes :  u8) -> get_convolution_filter_cookie;

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
unsafe fn xcb_glx_get_convolution_filter_unchecked (c : *connection,
                                                    context_tag :  context_tag,
                                                    target :  u32,
                                                    format :  u32,
                                                    type_ :  u32,
                                                    swap_bytes :  u8) -> get_convolution_filter_cookie;

unsafe fn xcb_glx_get_convolution_filter_data (R : *get_convolution_filter_reply) -> *u8;


unsafe fn xcb_glx_get_convolution_filter_data_length (R : *get_convolution_filter_reply) -> c_int;


unsafe fn xcb_glx_get_convolution_filter_data_end (R : *get_convolution_filter_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_convolution_filter_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_glx_get_convolution_filter_reply (c : *connection,
                                                cookie : get_convolution_filter_cookie,
                                                e : **generic_error) -> *get_convolution_filter_reply;

unsafe fn xcb_glx_get_convolution_parameterfv_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_get_convolution_parameterfv (c : *connection,
                                               context_tag :  context_tag,
                                               target :  u32,
                                               pname :  u32) -> get_convolution_parameterfv_cookie;

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
unsafe fn xcb_glx_get_convolution_parameterfv_unchecked (c : *connection,
                                                         context_tag :  context_tag,
                                                         target :  u32,
                                                         pname :  u32) -> get_convolution_parameterfv_cookie;

unsafe fn xcb_glx_get_convolution_parameterfv_data (R : *get_convolution_parameterfv_reply) -> *float32;


unsafe fn xcb_glx_get_convolution_parameterfv_data_length (R : *get_convolution_parameterfv_reply) -> c_int;


unsafe fn xcb_glx_get_convolution_parameterfv_data_end (R : *get_convolution_parameterfv_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_convolution_parameterfv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_glx_get_convolution_parameterfv_reply (c : *connection,
                                                     cookie : get_convolution_parameterfv_cookie,
                                                     e : **generic_error) -> *get_convolution_parameterfv_reply;

unsafe fn xcb_glx_get_convolution_parameteriv_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_get_convolution_parameteriv (c : *connection,
                                               context_tag :  context_tag,
                                               target :  u32,
                                               pname :  u32) -> get_convolution_parameteriv_cookie;

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
unsafe fn xcb_glx_get_convolution_parameteriv_unchecked (c : *connection,
                                                         context_tag :  context_tag,
                                                         target :  u32,
                                                         pname :  u32) -> get_convolution_parameteriv_cookie;

unsafe fn xcb_glx_get_convolution_parameteriv_data (R : *get_convolution_parameteriv_reply) -> *i32;


unsafe fn xcb_glx_get_convolution_parameteriv_data_length (R : *get_convolution_parameteriv_reply) -> c_int;


unsafe fn xcb_glx_get_convolution_parameteriv_data_end (R : *get_convolution_parameteriv_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_convolution_parameteriv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_glx_get_convolution_parameteriv_reply (c : *connection,
                                                     cookie : get_convolution_parameteriv_cookie,
                                                     e : **generic_error) -> *get_convolution_parameteriv_reply;

unsafe fn xcb_glx_get_separable_filter_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_get_separable_filter (c : *connection,
                                        context_tag :  context_tag,
                                        target :  u32,
                                        format :  u32,
                                        type_ :  u32,
                                        swap_bytes :  u8) -> get_separable_filter_cookie;

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
unsafe fn xcb_glx_get_separable_filter_unchecked (c : *connection,
                                                  context_tag :  context_tag,
                                                  target :  u32,
                                                  format :  u32,
                                                  type_ :  u32,
                                                  swap_bytes :  u8) -> get_separable_filter_cookie;

unsafe fn xcb_glx_get_separable_filter_rows_and_cols (R : *get_separable_filter_reply) -> *u8;


unsafe fn xcb_glx_get_separable_filter_rows_and_cols_length (R : *get_separable_filter_reply) -> c_int;


unsafe fn xcb_glx_get_separable_filter_rows_and_cols_end (R : *get_separable_filter_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_separable_filter_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_glx_get_separable_filter_reply (c : *connection,
                                              cookie : get_separable_filter_cookie,
                                              e : **generic_error) -> *get_separable_filter_reply;

unsafe fn xcb_glx_get_histogram_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_get_histogram (c : *connection,
                                 context_tag :  context_tag,
                                 target :  u32,
                                 format :  u32,
                                 type_ :  u32,
                                 swap_bytes :  u8,
                                 reset :  u8) -> get_histogram_cookie;

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
unsafe fn xcb_glx_get_histogram_unchecked (c : *connection,
                                           context_tag :  context_tag,
                                           target :  u32,
                                           format :  u32,
                                           type_ :  u32,
                                           swap_bytes :  u8,
                                           reset :  u8) -> get_histogram_cookie;

unsafe fn xcb_glx_get_histogram_data (R : *get_histogram_reply) -> *u8;


unsafe fn xcb_glx_get_histogram_data_length (R : *get_histogram_reply) -> c_int;


unsafe fn xcb_glx_get_histogram_data_end (R : *get_histogram_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_histogram_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_glx_get_histogram_reply (c : *connection,
                                       cookie : get_histogram_cookie,
                                       e : **generic_error) -> *get_histogram_reply;

unsafe fn xcb_glx_get_histogram_parameterfv_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_get_histogram_parameterfv (c : *connection,
                                             context_tag :  context_tag,
                                             target :  u32,
                                             pname :  u32) -> get_histogram_parameterfv_cookie;

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
unsafe fn xcb_glx_get_histogram_parameterfv_unchecked (c : *connection,
                                                       context_tag :  context_tag,
                                                       target :  u32,
                                                       pname :  u32) -> get_histogram_parameterfv_cookie;

unsafe fn xcb_glx_get_histogram_parameterfv_data (R : *get_histogram_parameterfv_reply) -> *float32;


unsafe fn xcb_glx_get_histogram_parameterfv_data_length (R : *get_histogram_parameterfv_reply) -> c_int;


unsafe fn xcb_glx_get_histogram_parameterfv_data_end (R : *get_histogram_parameterfv_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_histogram_parameterfv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_glx_get_histogram_parameterfv_reply (c : *connection,
                                                   cookie : get_histogram_parameterfv_cookie,
                                                   e : **generic_error) -> *get_histogram_parameterfv_reply;

unsafe fn xcb_glx_get_histogram_parameteriv_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_get_histogram_parameteriv (c : *connection,
                                             context_tag :  context_tag,
                                             target :  u32,
                                             pname :  u32) -> get_histogram_parameteriv_cookie;

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
unsafe fn xcb_glx_get_histogram_parameteriv_unchecked (c : *connection,
                                                       context_tag :  context_tag,
                                                       target :  u32,
                                                       pname :  u32) -> get_histogram_parameteriv_cookie;

unsafe fn xcb_glx_get_histogram_parameteriv_data (R : *get_histogram_parameteriv_reply) -> *i32;


unsafe fn xcb_glx_get_histogram_parameteriv_data_length (R : *get_histogram_parameteriv_reply) -> c_int;


unsafe fn xcb_glx_get_histogram_parameteriv_data_end (R : *get_histogram_parameteriv_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_histogram_parameteriv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_glx_get_histogram_parameteriv_reply (c : *connection,
                                                   cookie : get_histogram_parameteriv_cookie,
                                                   e : **generic_error) -> *get_histogram_parameteriv_reply;

unsafe fn xcb_glx_get_minmax_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_get_minmax (c : *connection,
                              context_tag :  context_tag,
                              target :  u32,
                              format :  u32,
                              type_ :  u32,
                              swap_bytes :  u8,
                              reset :  u8) -> get_minmax_cookie;

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
unsafe fn xcb_glx_get_minmax_unchecked (c : *connection,
                                        context_tag :  context_tag,
                                        target :  u32,
                                        format :  u32,
                                        type_ :  u32,
                                        swap_bytes :  u8,
                                        reset :  u8) -> get_minmax_cookie;

unsafe fn xcb_glx_get_minmax_data (R : *get_minmax_reply) -> *u8;


unsafe fn xcb_glx_get_minmax_data_length (R : *get_minmax_reply) -> c_int;


unsafe fn xcb_glx_get_minmax_data_end (R : *get_minmax_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_minmax_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_glx_get_minmax_reply (c : *connection,
                                    cookie : get_minmax_cookie,
                                    e : **generic_error) -> *get_minmax_reply;

unsafe fn xcb_glx_get_minmax_parameterfv_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_get_minmax_parameterfv (c : *connection,
                                          context_tag :  context_tag,
                                          target :  u32,
                                          pname :  u32) -> get_minmax_parameterfv_cookie;

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
unsafe fn xcb_glx_get_minmax_parameterfv_unchecked (c : *connection,
                                                    context_tag :  context_tag,
                                                    target :  u32,
                                                    pname :  u32) -> get_minmax_parameterfv_cookie;

unsafe fn xcb_glx_get_minmax_parameterfv_data (R : *get_minmax_parameterfv_reply) -> *float32;


unsafe fn xcb_glx_get_minmax_parameterfv_data_length (R : *get_minmax_parameterfv_reply) -> c_int;


unsafe fn xcb_glx_get_minmax_parameterfv_data_end (R : *get_minmax_parameterfv_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_minmax_parameterfv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_glx_get_minmax_parameterfv_reply (c : *connection,
                                                cookie : get_minmax_parameterfv_cookie,
                                                e : **generic_error) -> *get_minmax_parameterfv_reply;

unsafe fn xcb_glx_get_minmax_parameteriv_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_get_minmax_parameteriv (c : *connection,
                                          context_tag :  context_tag,
                                          target :  u32,
                                          pname :  u32) -> get_minmax_parameteriv_cookie;

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
unsafe fn xcb_glx_get_minmax_parameteriv_unchecked (c : *connection,
                                                    context_tag :  context_tag,
                                                    target :  u32,
                                                    pname :  u32) -> get_minmax_parameteriv_cookie;

unsafe fn xcb_glx_get_minmax_parameteriv_data (R : *get_minmax_parameteriv_reply) -> *i32;


unsafe fn xcb_glx_get_minmax_parameteriv_data_length (R : *get_minmax_parameteriv_reply) -> c_int;


unsafe fn xcb_glx_get_minmax_parameteriv_data_end (R : *get_minmax_parameteriv_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_minmax_parameteriv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_glx_get_minmax_parameteriv_reply (c : *connection,
                                                cookie : get_minmax_parameteriv_cookie,
                                                e : **generic_error) -> *get_minmax_parameteriv_reply;

unsafe fn xcb_glx_get_compressed_tex_image_arb_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_get_compressed_tex_image_arb (c : *connection,
                                                context_tag :  context_tag,
                                                target :  u32,
                                                level :  i32) -> get_compressed_tex_image_arb_cookie;

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
unsafe fn xcb_glx_get_compressed_tex_image_arb_unchecked (c : *connection,
                                                          context_tag :  context_tag,
                                                          target :  u32,
                                                          level :  i32) -> get_compressed_tex_image_arb_cookie;

unsafe fn xcb_glx_get_compressed_tex_image_arb_data (R : *get_compressed_tex_image_arb_reply) -> *u8;


unsafe fn xcb_glx_get_compressed_tex_image_arb_data_length (R : *get_compressed_tex_image_arb_reply) -> c_int;


unsafe fn xcb_glx_get_compressed_tex_image_arb_data_end (R : *get_compressed_tex_image_arb_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_compressed_tex_image_arb_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_glx_get_compressed_tex_image_arb_reply (c : *connection,
                                                      cookie : get_compressed_tex_image_arb_cookie,
                                                      e : **generic_error) -> *get_compressed_tex_image_arb_reply;

unsafe fn xcb_glx_delete_queries_arb_sizeof (_buffer :  *c_void) -> c_int;

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
unsafe fn xcb_glx_delete_queries_arb_checked (c : *connection,
                                              context_tag :  context_tag,
                                              n :  i32,
                                              ids : *u32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_delete_queries_arb (c : *connection,
                                      context_tag :  context_tag,
                                      n :  i32,
                                      ids : *u32) -> void_cookie;

unsafe fn xcb_glx_gen_queries_arb_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_gen_queries_arb (c : *connection,
                                   context_tag :  context_tag,
                                   n :  i32) -> gen_queries_arb_cookie;

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
unsafe fn xcb_glx_gen_queries_arb_unchecked (c : *connection,
                                             context_tag :  context_tag,
                                             n :  i32) -> gen_queries_arb_cookie;

unsafe fn xcb_glx_gen_queries_arb_data (R : *gen_queries_arb_reply) -> *u32;


unsafe fn xcb_glx_gen_queries_arb_data_length (R : *gen_queries_arb_reply) -> c_int;


unsafe fn xcb_glx_gen_queries_arb_data_end (R : *gen_queries_arb_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_gen_queries_arb_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_glx_gen_queries_arb_reply (c : *connection,
                                         cookie : gen_queries_arb_cookie,
                                         e : **generic_error) -> *gen_queries_arb_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_is_query_arb (c : *connection,
                                context_tag :  context_tag,
                                id :  u32) -> is_query_arb_cookie;

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
unsafe fn xcb_glx_is_query_arb_unchecked (c : *connection,
                                          context_tag :  context_tag,
                                          id :  u32) -> is_query_arb_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_is_query_arb_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_glx_is_query_arb_reply (c : *connection,
                                      cookie : is_query_arb_cookie,
                                      e : **generic_error) -> *is_query_arb_reply;

unsafe fn xcb_glx_get_queryiv_arb_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_get_queryiv_arb (c : *connection,
                                   context_tag :  context_tag,
                                   target :  u32,
                                   pname :  u32) -> get_queryiv_arb_cookie;

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
unsafe fn xcb_glx_get_queryiv_arb_unchecked (c : *connection,
                                             context_tag :  context_tag,
                                             target :  u32,
                                             pname :  u32) -> get_queryiv_arb_cookie;

unsafe fn xcb_glx_get_queryiv_arb_data (R : *get_queryiv_arb_reply) -> *i32;


unsafe fn xcb_glx_get_queryiv_arb_data_length (R : *get_queryiv_arb_reply) -> c_int;


unsafe fn xcb_glx_get_queryiv_arb_data_end (R : *get_queryiv_arb_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_queryiv_arb_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_glx_get_queryiv_arb_reply (c : *connection,
                                         cookie : get_queryiv_arb_cookie,
                                         e : **generic_error) -> *get_queryiv_arb_reply;

unsafe fn xcb_glx_get_query_objectiv_arb_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_get_query_objectiv_arb (c : *connection,
                                          context_tag :  context_tag,
                                          id :  u32,
                                          pname :  u32) -> get_query_objectiv_arb_cookie;

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
unsafe fn xcb_glx_get_query_objectiv_arb_unchecked (c : *connection,
                                                    context_tag :  context_tag,
                                                    id :  u32,
                                                    pname :  u32) -> get_query_objectiv_arb_cookie;

unsafe fn xcb_glx_get_query_objectiv_arb_data (R : *get_query_objectiv_arb_reply) -> *i32;


unsafe fn xcb_glx_get_query_objectiv_arb_data_length (R : *get_query_objectiv_arb_reply) -> c_int;


unsafe fn xcb_glx_get_query_objectiv_arb_data_end (R : *get_query_objectiv_arb_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_query_objectiv_arb_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_glx_get_query_objectiv_arb_reply (c : *connection,
                                                cookie : get_query_objectiv_arb_cookie,
                                                e : **generic_error) -> *get_query_objectiv_arb_reply;

unsafe fn xcb_glx_get_query_objectuiv_arb_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_glx_get_query_objectuiv_arb (c : *connection,
                                           context_tag :  context_tag,
                                           id :  u32,
                                           pname :  u32) -> get_query_objectuiv_arb_cookie;

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
unsafe fn xcb_glx_get_query_objectuiv_arb_unchecked (c : *connection,
                                                     context_tag :  context_tag,
                                                     id :  u32,
                                                     pname :  u32) -> get_query_objectuiv_arb_cookie;

unsafe fn xcb_glx_get_query_objectuiv_arb_data (R : *get_query_objectuiv_arb_reply) -> *u32;


unsafe fn xcb_glx_get_query_objectuiv_arb_data_length (R : *get_query_objectuiv_arb_reply) -> c_int;


unsafe fn xcb_glx_get_query_objectuiv_arb_data_end (R : *get_query_objectuiv_arb_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_query_objectuiv_arb_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_glx_get_query_objectuiv_arb_reply (c : *connection,
                                                 cookie : get_query_objectuiv_arb_cookie,
                                                 e : **generic_error) -> *get_query_objectuiv_arb_reply;
}

