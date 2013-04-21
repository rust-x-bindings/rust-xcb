/*
 * This file generated automatically from xf86vidmode.xml by r_client.py.
 * Edit at your peril.
 */

//Make the compiler quiet
#[allow(unused_imports)];
#[allow(non_camel_case_types)];
use core::libc::*;
use ll::base::*;

pub static XF86VIDMODE_MAJOR_VERSION : c_uint = 2;
pub static XF86VIDMODE_MINOR_VERSION : c_uint = 2;

pub type syncrange = u32;

/**
 * @brief syncrange_iterator
 **/
pub struct syncrange_iterator {
    data : *syncrange,
    rem  : c_int,
    index: c_int
}

pub type dotclock = u32;

/**
 * @brief dotclock_iterator
 **/
pub struct dotclock_iterator {
    data : *dotclock,
    rem  : c_int,
    index: c_int
}

pub type mode_flag = c_uint;//{
    pub static XCB_XF86VIDMODE_MODE_FLAG_POSITIVE_H_SYNC : mode_flag = 1;
    pub static XCB_XF86VIDMODE_MODE_FLAG_NEGATIVE_H_SYNC : mode_flag = 2;
    pub static XCB_XF86VIDMODE_MODE_FLAG_POSITIVE_V_SYNC : mode_flag = 4;
    pub static XCB_XF86VIDMODE_MODE_FLAG_NEGATIVE_V_SYNC : mode_flag = 8;
    pub static XCB_XF86VIDMODE_MODE_FLAG_INTERLACE : mode_flag = 16;
    pub static XCB_XF86VIDMODE_MODE_FLAG_COMPOSITE_SYNC : mode_flag = 32;
    pub static XCB_XF86VIDMODE_MODE_FLAG_POSITIVE_C_SYNC : mode_flag = 64;
    pub static XCB_XF86VIDMODE_MODE_FLAG_NEGATIVE_C_SYNC : mode_flag = 128;
    pub static XCB_XF86VIDMODE_MODE_FLAG_H_SKEW : mode_flag = 256;
    pub static XCB_XF86VIDMODE_MODE_FLAG_BROADCAST : mode_flag = 512;
    pub static XCB_XF86VIDMODE_MODE_FLAG_PIXMUX : mode_flag = 1024;
    pub static XCB_XF86VIDMODE_MODE_FLAG_DOUBLE_CLOCK : mode_flag = 2048;
    pub static XCB_XF86VIDMODE_MODE_FLAG_HALF_CLOCK : mode_flag = 4096;
//}

pub type clock_flag = c_uint;//{
    pub static XCB_XF86VIDMODE_CLOCK_FLAG_PROGRAMABLE : clock_flag = 1;
//}

pub type permission = c_uint;//{
    pub static XCB_XF86VIDMODE_PERMISSION_READ : permission = 1;
    pub static XCB_XF86VIDMODE_PERMISSION_WRITE : permission = 2;
//}

pub struct mode_info {
    dotclock :     dotclock,
    hdisplay :     u16,
    hsyncstart :   u16,
    hsyncend :     u16,
    htotal :       u16,
    hskew :        u32,
    vdisplay :     u16,
    vsyncstart :   u16,
    vsyncend :     u16,
    vtotal :       u16,
    pad0 :         [u8,..4],
    flags :        u32,
    pad1 :         [u8,..12],
    privsize :     u32
}

/**
 * @brief mode_info_iterator
 **/
pub struct mode_info_iterator {
    data : *mode_info,
    rem  : c_int,
    index: c_int
}

pub struct query_version_cookie {
    sequence : c_uint
}

/** Opcode for xcb_xf86vidmode_query_version. */
pub static XCB_XF86VIDMODE_QUERY_VERSION : c_int = 0;

pub struct query_version_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16
}

pub struct query_version_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    major_version :   u16,
    minor_version :   u16
}

pub struct get_mode_line_cookie {
    sequence : c_uint
}

/** Opcode for xcb_xf86vidmode_get_mode_line. */
pub static XCB_XF86VIDMODE_GET_MODE_LINE : c_int = 1;

pub struct get_mode_line_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    screen :         u16,
    pad0 :           [u8,..2]
}

pub struct get_mode_line_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    dotclock :        dotclock,
    hdisplay :        u16,
    hsyncstart :      u16,
    hsyncend :        u16,
    htotal :          u16,
    hskew :           u16,
    vdisplay :        u16,
    vsyncstart :      u16,
    vsyncend :        u16,
    vtotal :          u16,
    pad1 :            [u8,..2],
    flags :           u32,
    pad2 :            [u8,..12],
    privsize :        u32
}

/** Opcode for xcb_xf86vidmode_mod_mode_line. */
pub static XCB_XF86VIDMODE_MOD_MODE_LINE : c_int = 2;

pub struct mod_mode_line_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    screen :         u32,
    hdisplay :       u16,
    hsyncstart :     u16,
    hsyncend :       u16,
    htotal :         u16,
    hskew :          u16,
    vdisplay :       u16,
    vsyncstart :     u16,
    vsyncend :       u16,
    vtotal :         u16,
    pad0 :           [u8,..2],
    flags :          u32,
    pad1 :           [u8,..12],
    privsize :       u32
}

/** Opcode for xcb_xf86vidmode_switch_mode. */
pub static XCB_XF86VIDMODE_SWITCH_MODE : c_int = 3;

pub struct switch_mode_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    screen :         u16,
    zoom :           u16
}

pub struct get_monitor_cookie {
    sequence : c_uint
}

/** Opcode for xcb_xf86vidmode_get_monitor. */
pub static XCB_XF86VIDMODE_GET_MONITOR : c_int = 4;

pub struct get_monitor_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    screen :         u16,
    pad0 :           [u8,..2]
}

pub struct get_monitor_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    vendor_length :   u8,
    model_length :    u8,
    num_hsync :       u8,
    num_vsync :       u8,
    pad1 :            [u8,..20]
}

/** Opcode for xcb_xf86vidmode_lock_mode_switch. */
pub static XCB_XF86VIDMODE_LOCK_MODE_SWITCH : c_int = 5;

pub struct lock_mode_switch_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    screen :         u16,
    lock :           u16
}

pub struct get_all_mode_lines_cookie {
    sequence : c_uint
}

/** Opcode for xcb_xf86vidmode_get_all_mode_lines. */
pub static XCB_XF86VIDMODE_GET_ALL_MODE_LINES : c_int = 6;

pub struct get_all_mode_lines_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    screen :         u16,
    pad0 :           [u8,..2]
}

pub struct get_all_mode_lines_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    modecount :       u32,
    pad1 :            [u8,..20]
}

/** Opcode for xcb_xf86vidmode_add_mode_line. */
pub static XCB_XF86VIDMODE_ADD_MODE_LINE : c_int = 7;

pub struct add_mode_line_request {
    major_opcode :       u8,
    minor_opcode :       u8,
    length :             u16,
    screen :             u32,
    dotclock :           dotclock,
    hdisplay :           u16,
    hsyncstart :         u16,
    hsyncend :           u16,
    htotal :             u16,
    hskew :              u16,
    vdisplay :           u16,
    vsyncstart :         u16,
    vsyncend :           u16,
    vtotal :             u16,
    pad0 :               [u8,..2],
    flags :              u32,
    pad1 :               [u8,..12],
    privsize :           u32,
    after_dotclock :     dotclock,
    after_hdisplay :     u16,
    after_hsyncstart :   u16,
    after_hsyncend :     u16,
    after_htotal :       u16,
    after_hskew :        u16,
    after_vdisplay :     u16,
    after_vsyncstart :   u16,
    after_vsyncend :     u16,
    after_vtotal :       u16,
    pad2 :               [u8,..2],
    after_flags :        u32,
    pad3 :               [u8,..12]
}

/** Opcode for xcb_xf86vidmode_delete_mode_line. */
pub static XCB_XF86VIDMODE_DELETE_MODE_LINE : c_int = 8;

pub struct delete_mode_line_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    screen :         u32,
    dotclock :       dotclock,
    hdisplay :       u16,
    hsyncstart :     u16,
    hsyncend :       u16,
    htotal :         u16,
    hskew :          u16,
    vdisplay :       u16,
    vsyncstart :     u16,
    vsyncend :       u16,
    vtotal :         u16,
    pad0 :           [u8,..2],
    flags :          u32,
    pad1 :           [u8,..12],
    privsize :       u32
}

pub struct validate_mode_line_cookie {
    sequence : c_uint
}

/** Opcode for xcb_xf86vidmode_validate_mode_line. */
pub static XCB_XF86VIDMODE_VALIDATE_MODE_LINE : c_int = 9;

pub struct validate_mode_line_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    screen :         u32,
    dotclock :       dotclock,
    hdisplay :       u16,
    hsyncstart :     u16,
    hsyncend :       u16,
    htotal :         u16,
    hskew :          u16,
    vdisplay :       u16,
    vsyncstart :     u16,
    vsyncend :       u16,
    vtotal :         u16,
    pad0 :           [u8,..2],
    flags :          u32,
    pad1 :           [u8,..12],
    privsize :       u32
}

pub struct validate_mode_line_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    status :          u32,
    pad1 :            [u8,..20]
}

/** Opcode for xcb_xf86vidmode_switch_to_mode. */
pub static XCB_XF86VIDMODE_SWITCH_TO_MODE : c_int = 10;

pub struct switch_to_mode_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    screen :         u32,
    dotclock :       dotclock,
    hdisplay :       u16,
    hsyncstart :     u16,
    hsyncend :       u16,
    htotal :         u16,
    hskew :          u16,
    vdisplay :       u16,
    vsyncstart :     u16,
    vsyncend :       u16,
    vtotal :         u16,
    pad0 :           [u8,..2],
    flags :          u32,
    pad1 :           [u8,..12],
    privsize :       u32
}

pub struct get_view_port_cookie {
    sequence : c_uint
}

/** Opcode for xcb_xf86vidmode_get_view_port. */
pub static XCB_XF86VIDMODE_GET_VIEW_PORT : c_int = 11;

pub struct get_view_port_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    screen :         u16,
    pad0 :           [u8,..2]
}

pub struct get_view_port_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    x :               u32,
    y :               u32,
    pad1 :            [u8,..16]
}

/** Opcode for xcb_xf86vidmode_set_view_port. */
pub static XCB_XF86VIDMODE_SET_VIEW_PORT : c_int = 12;

pub struct set_view_port_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    screen :         u16,
    pad0 :           [u8,..2],
    x :              u32,
    y :              u32
}

pub struct get_dot_clocks_cookie {
    sequence : c_uint
}

/** Opcode for xcb_xf86vidmode_get_dot_clocks. */
pub static XCB_XF86VIDMODE_GET_DOT_CLOCKS : c_int = 13;

pub struct get_dot_clocks_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    screen :         u16,
    pad0 :           [u8,..2]
}

pub struct get_dot_clocks_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    flags :           u32,
    clocks :          u32,
    maxclocks :       u32,
    pad1 :            [u8,..12]
}

/** Opcode for xcb_xf86vidmode_set_client_version. */
pub static XCB_XF86VIDMODE_SET_CLIENT_VERSION : c_int = 14;

pub struct set_client_version_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    major :          u16,
    minor :          u16
}

/** Opcode for xcb_xf86vidmode_set_gamma. */
pub static XCB_XF86VIDMODE_SET_GAMMA : c_int = 15;

pub struct set_gamma_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    screen :         u16,
    pad0 :           [u8,..2],
    red :            u32,
    green :          u32,
    blue :           u32,
    pad1 :           [u8,..12]
}

pub struct get_gamma_cookie {
    sequence : c_uint
}

/** Opcode for xcb_xf86vidmode_get_gamma. */
pub static XCB_XF86VIDMODE_GET_GAMMA : c_int = 16;

pub struct get_gamma_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    screen :         u16,
    pad0 :           [u8,..26]
}

pub struct get_gamma_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    red :             u32,
    green :           u32,
    blue :            u32,
    pad1 :            [u8,..12]
}

pub struct get_gamma_ramp_cookie {
    sequence : c_uint
}

/** Opcode for xcb_xf86vidmode_get_gamma_ramp. */
pub static XCB_XF86VIDMODE_GET_GAMMA_RAMP : c_int = 17;

pub struct get_gamma_ramp_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    screen :         u16,
    size :           u16
}

pub struct get_gamma_ramp_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    size :            u16,
    pad1 :            [u8,..22]
}

/** Opcode for xcb_xf86vidmode_set_gamma_ramp. */
pub static XCB_XF86VIDMODE_SET_GAMMA_RAMP : c_int = 18;

pub struct set_gamma_ramp_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    screen :         u16,
    size :           u16
}

pub struct get_gamma_ramp_size_cookie {
    sequence : c_uint
}

/** Opcode for xcb_xf86vidmode_get_gamma_ramp_size. */
pub static XCB_XF86VIDMODE_GET_GAMMA_RAMP_SIZE : c_int = 19;

pub struct get_gamma_ramp_size_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    screen :         u16,
    pad0 :           [u8,..2]
}

pub struct get_gamma_ramp_size_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    size :            u16,
    pad1 :            [u8,..22]
}

pub struct get_permissions_cookie {
    sequence : c_uint
}

/** Opcode for xcb_xf86vidmode_get_permissions. */
pub static XCB_XF86VIDMODE_GET_PERMISSIONS : c_int = 20;

pub struct get_permissions_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    screen :         u16,
    pad0 :           [u8,..2]
}

pub struct get_permissions_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    permissions :     u32,
    pad1 :            [u8,..20]
}

/** Opcode for xcb_xf86vidmode_bad_clock. */
pub static XCB_XF86VIDMODE_BAD_CLOCK : c_int = 0;

pub struct bad_clock_error {
    response_type :   u8,
    error_code :      u8,
    sequence :        u16
}

/** Opcode for xcb_xf86vidmode_bad_h_timings. */
pub static XCB_XF86VIDMODE_BAD_H_TIMINGS : c_int = 1;

pub struct bad_h_timings_error {
    response_type :   u8,
    error_code :      u8,
    sequence :        u16
}

/** Opcode for xcb_xf86vidmode_bad_v_timings. */
pub static XCB_XF86VIDMODE_BAD_V_TIMINGS : c_int = 2;

pub struct bad_v_timings_error {
    response_type :   u8,
    error_code :      u8,
    sequence :        u16
}

/** Opcode for xcb_xf86vidmode_mode_unsuitable. */
pub static XCB_XF86VIDMODE_MODE_UNSUITABLE : c_int = 3;

pub struct mode_unsuitable_error {
    response_type :   u8,
    error_code :      u8,
    sequence :        u16
}

/** Opcode for xcb_xf86vidmode_extension_disabled. */
pub static XCB_XF86VIDMODE_EXTENSION_DISABLED : c_int = 4;

pub struct extension_disabled_error {
    response_type :   u8,
    error_code :      u8,
    sequence :        u16
}

/** Opcode for xcb_xf86vidmode_client_not_local. */
pub static XCB_XF86VIDMODE_CLIENT_NOT_LOCAL : c_int = 5;

pub struct client_not_local_error {
    response_type :   u8,
    error_code :      u8,
    sequence :        u16
}

/** Opcode for xcb_xf86vidmode_zoom_locked. */
pub static XCB_XF86VIDMODE_ZOOM_LOCKED : c_int = 6;

pub struct zoom_locked_error {
    response_type :   u8,
    error_code :      u8,
    sequence :        u16
}
extern "C" {

/**
 * Get the next element of the iterator
 * @param i Pointer to a syncrange_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(syncrange)
 *
 *
 */
unsafe fn xcb_xf86vidmode_syncrange_next (i:*syncrange_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An syncrange_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xf86vidmode_syncrange_end (i:syncrange_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a dotclock_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(dotclock)
 *
 *
 */
unsafe fn xcb_xf86vidmode_dotclock_next (i:*dotclock_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An dotclock_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xf86vidmode_dotclock_end (i:dotclock_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a mode_info_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(mode_info)
 *
 *
 */
unsafe fn xcb_xf86vidmode_mode_info_next (i:*mode_info_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An mode_info_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xf86vidmode_mode_info_end (i:mode_info_iterator) -> generic_iterator;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xf86vidmode_query_version (c : *connection) -> query_version_cookie;

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
unsafe fn xcb_xf86vidmode_query_version_unchecked (c : *connection) -> query_version_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xf86vidmode_query_version_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_xf86vidmode_query_version_reply (c : *connection,
                                               cookie : query_version_cookie,
                                               e : **generic_error) -> *query_version_reply;

unsafe fn xcb_xf86vidmode_get_mode_line_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xf86vidmode_get_mode_line (c : *connection,
                                         screen :  u16) -> get_mode_line_cookie;

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
unsafe fn xcb_xf86vidmode_get_mode_line_unchecked (c : *connection,
                                                   screen :  u16) -> get_mode_line_cookie;

unsafe fn xcb_xf86vidmode_get_mode_line_private (R : *get_mode_line_reply) -> *u8;


unsafe fn xcb_xf86vidmode_get_mode_line_private_length (R : *get_mode_line_reply) -> c_int;


unsafe fn xcb_xf86vidmode_get_mode_line_private_end (R : *get_mode_line_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xf86vidmode_get_mode_line_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_xf86vidmode_get_mode_line_reply (c : *connection,
                                               cookie : get_mode_line_cookie,
                                               e : **generic_error) -> *get_mode_line_reply;

unsafe fn xcb_xf86vidmode_mod_mode_line_sizeof (_buffer :  *c_void) -> c_int;

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
unsafe fn xcb_xf86vidmode_mod_mode_line_checked (c : *connection,
                                                 screen :  u32,
                                                 hdisplay :  u16,
                                                 hsyncstart :  u16,
                                                 hsyncend :  u16,
                                                 htotal :  u16,
                                                 hskew :  u16,
                                                 vdisplay :  u16,
                                                 vsyncstart :  u16,
                                                 vsyncend :  u16,
                                                 vtotal :  u16,
                                                 flags :  u32,
                                                 privsize :  u32,
                                                 private : *u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xf86vidmode_mod_mode_line (c : *connection,
                                         screen :  u32,
                                         hdisplay :  u16,
                                         hsyncstart :  u16,
                                         hsyncend :  u16,
                                         htotal :  u16,
                                         hskew :  u16,
                                         vdisplay :  u16,
                                         vsyncstart :  u16,
                                         vsyncend :  u16,
                                         vtotal :  u16,
                                         flags :  u32,
                                         privsize :  u32,
                                         private : *u8) -> void_cookie;

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
unsafe fn xcb_xf86vidmode_switch_mode_checked (c : *connection,
                                               screen :  u16,
                                               zoom :  u16) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xf86vidmode_switch_mode (c : *connection,
                                       screen :  u16,
                                       zoom :  u16) -> void_cookie;

unsafe fn xcb_xf86vidmode_get_monitor_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xf86vidmode_get_monitor (c : *connection,
                                       screen :  u16) -> get_monitor_cookie;

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
unsafe fn xcb_xf86vidmode_get_monitor_unchecked (c : *connection,
                                                 screen :  u16) -> get_monitor_cookie;

unsafe fn xcb_xf86vidmode_get_monitor_hsync (R : *get_monitor_reply) -> *syncrange;


unsafe fn xcb_xf86vidmode_get_monitor_hsync_length (R : *get_monitor_reply) -> c_int;


unsafe fn xcb_xf86vidmode_get_monitor_hsync_end (R : *get_monitor_reply) -> generic_iterator;

unsafe fn xcb_xf86vidmode_get_monitor_vsync (R : *get_monitor_reply) -> *syncrange;


unsafe fn xcb_xf86vidmode_get_monitor_vsync_length (R : *get_monitor_reply) -> c_int;


unsafe fn xcb_xf86vidmode_get_monitor_vsync_end (R : *get_monitor_reply) -> generic_iterator;

unsafe fn xcb_xf86vidmode_get_monitor_vendor (R : *get_monitor_reply) -> *u8;


unsafe fn xcb_xf86vidmode_get_monitor_vendor_length (R : *get_monitor_reply) -> c_int;


unsafe fn xcb_xf86vidmode_get_monitor_vendor_end (R : *get_monitor_reply) -> generic_iterator;

unsafe fn xcb_xf86vidmode_get_monitor_alignment_pad (R : *get_monitor_reply) -> *c_void;


unsafe fn xcb_xf86vidmode_get_monitor_alignment_pad_length (R : *get_monitor_reply) -> c_int;


unsafe fn xcb_xf86vidmode_get_monitor_alignment_pad_end (R : *get_monitor_reply) -> generic_iterator;

unsafe fn xcb_xf86vidmode_get_monitor_model (R : *get_monitor_reply) -> *u8;


unsafe fn xcb_xf86vidmode_get_monitor_model_length (R : *get_monitor_reply) -> c_int;


unsafe fn xcb_xf86vidmode_get_monitor_model_end (R : *get_monitor_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xf86vidmode_get_monitor_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_xf86vidmode_get_monitor_reply (c : *connection,
                                             cookie : get_monitor_cookie,
                                             e : **generic_error) -> *get_monitor_reply;

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
unsafe fn xcb_xf86vidmode_lock_mode_switch_checked (c : *connection,
                                                    screen :  u16,
                                                    lock :  u16) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xf86vidmode_lock_mode_switch (c : *connection,
                                            screen :  u16,
                                            lock :  u16) -> void_cookie;

unsafe fn xcb_xf86vidmode_get_all_mode_lines_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xf86vidmode_get_all_mode_lines (c : *connection,
                                              screen :  u16) -> get_all_mode_lines_cookie;

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
unsafe fn xcb_xf86vidmode_get_all_mode_lines_unchecked (c : *connection,
                                                        screen :  u16) -> get_all_mode_lines_cookie;

unsafe fn xcb_xf86vidmode_get_all_mode_lines_modeinfo (R : *get_all_mode_lines_reply) -> *mode_info;


unsafe fn xcb_xf86vidmode_get_all_mode_lines_modeinfo_length (R : *get_all_mode_lines_reply) -> c_int;

unsafe fn xcb_xf86vidmode_get_all_mode_lines_modeinfo_iterator (R : *get_all_mode_lines_reply) -> mode_info_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xf86vidmode_get_all_mode_lines_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_xf86vidmode_get_all_mode_lines_reply (c : *connection,
                                                    cookie : get_all_mode_lines_cookie,
                                                    e : **generic_error) -> *get_all_mode_lines_reply;

unsafe fn xcb_xf86vidmode_add_mode_line_sizeof (_buffer :  *c_void) -> c_int;

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
unsafe fn xcb_xf86vidmode_add_mode_line_checked (c : *connection,
                                                 screen :  u32,
                                                 dotclock :  dotclock,
                                                 hdisplay :  u16,
                                                 hsyncstart :  u16,
                                                 hsyncend :  u16,
                                                 htotal :  u16,
                                                 hskew :  u16,
                                                 vdisplay :  u16,
                                                 vsyncstart :  u16,
                                                 vsyncend :  u16,
                                                 vtotal :  u16,
                                                 flags :  u32,
                                                 privsize :  u32,
                                                 after_dotclock :  dotclock,
                                                 after_hdisplay :  u16,
                                                 after_hsyncstart :  u16,
                                                 after_hsyncend :  u16,
                                                 after_htotal :  u16,
                                                 after_hskew :  u16,
                                                 after_vdisplay :  u16,
                                                 after_vsyncstart :  u16,
                                                 after_vsyncend :  u16,
                                                 after_vtotal :  u16,
                                                 after_flags :  u32,
                                                 private : *u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xf86vidmode_add_mode_line (c : *connection,
                                         screen :  u32,
                                         dotclock :  dotclock,
                                         hdisplay :  u16,
                                         hsyncstart :  u16,
                                         hsyncend :  u16,
                                         htotal :  u16,
                                         hskew :  u16,
                                         vdisplay :  u16,
                                         vsyncstart :  u16,
                                         vsyncend :  u16,
                                         vtotal :  u16,
                                         flags :  u32,
                                         privsize :  u32,
                                         after_dotclock :  dotclock,
                                         after_hdisplay :  u16,
                                         after_hsyncstart :  u16,
                                         after_hsyncend :  u16,
                                         after_htotal :  u16,
                                         after_hskew :  u16,
                                         after_vdisplay :  u16,
                                         after_vsyncstart :  u16,
                                         after_vsyncend :  u16,
                                         after_vtotal :  u16,
                                         after_flags :  u32,
                                         private : *u8) -> void_cookie;

unsafe fn xcb_xf86vidmode_delete_mode_line_sizeof (_buffer :  *c_void) -> c_int;

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
unsafe fn xcb_xf86vidmode_delete_mode_line_checked (c : *connection,
                                                    screen :  u32,
                                                    dotclock :  dotclock,
                                                    hdisplay :  u16,
                                                    hsyncstart :  u16,
                                                    hsyncend :  u16,
                                                    htotal :  u16,
                                                    hskew :  u16,
                                                    vdisplay :  u16,
                                                    vsyncstart :  u16,
                                                    vsyncend :  u16,
                                                    vtotal :  u16,
                                                    flags :  u32,
                                                    privsize :  u32,
                                                    private : *u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xf86vidmode_delete_mode_line (c : *connection,
                                            screen :  u32,
                                            dotclock :  dotclock,
                                            hdisplay :  u16,
                                            hsyncstart :  u16,
                                            hsyncend :  u16,
                                            htotal :  u16,
                                            hskew :  u16,
                                            vdisplay :  u16,
                                            vsyncstart :  u16,
                                            vsyncend :  u16,
                                            vtotal :  u16,
                                            flags :  u32,
                                            privsize :  u32,
                                            private : *u8) -> void_cookie;

unsafe fn xcb_xf86vidmode_validate_mode_line_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xf86vidmode_validate_mode_line (c : *connection,
                                              screen :  u32,
                                              dotclock :  dotclock,
                                              hdisplay :  u16,
                                              hsyncstart :  u16,
                                              hsyncend :  u16,
                                              htotal :  u16,
                                              hskew :  u16,
                                              vdisplay :  u16,
                                              vsyncstart :  u16,
                                              vsyncend :  u16,
                                              vtotal :  u16,
                                              flags :  u32,
                                              privsize :  u32,
                                              private : *u8) -> validate_mode_line_cookie;

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
unsafe fn xcb_xf86vidmode_validate_mode_line_unchecked (c : *connection,
                                                        screen :  u32,
                                                        dotclock :  dotclock,
                                                        hdisplay :  u16,
                                                        hsyncstart :  u16,
                                                        hsyncend :  u16,
                                                        htotal :  u16,
                                                        hskew :  u16,
                                                        vdisplay :  u16,
                                                        vsyncstart :  u16,
                                                        vsyncend :  u16,
                                                        vtotal :  u16,
                                                        flags :  u32,
                                                        privsize :  u32,
                                                        private : *u8) -> validate_mode_line_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xf86vidmode_validate_mode_line_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_xf86vidmode_validate_mode_line_reply (c : *connection,
                                                    cookie : validate_mode_line_cookie,
                                                    e : **generic_error) -> *validate_mode_line_reply;

unsafe fn xcb_xf86vidmode_switch_to_mode_sizeof (_buffer :  *c_void) -> c_int;

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
unsafe fn xcb_xf86vidmode_switch_to_mode_checked (c : *connection,
                                                  screen :  u32,
                                                  dotclock :  dotclock,
                                                  hdisplay :  u16,
                                                  hsyncstart :  u16,
                                                  hsyncend :  u16,
                                                  htotal :  u16,
                                                  hskew :  u16,
                                                  vdisplay :  u16,
                                                  vsyncstart :  u16,
                                                  vsyncend :  u16,
                                                  vtotal :  u16,
                                                  flags :  u32,
                                                  privsize :  u32,
                                                  private : *u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xf86vidmode_switch_to_mode (c : *connection,
                                          screen :  u32,
                                          dotclock :  dotclock,
                                          hdisplay :  u16,
                                          hsyncstart :  u16,
                                          hsyncend :  u16,
                                          htotal :  u16,
                                          hskew :  u16,
                                          vdisplay :  u16,
                                          vsyncstart :  u16,
                                          vsyncend :  u16,
                                          vtotal :  u16,
                                          flags :  u32,
                                          privsize :  u32,
                                          private : *u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xf86vidmode_get_view_port (c : *connection,
                                         screen :  u16) -> get_view_port_cookie;

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
unsafe fn xcb_xf86vidmode_get_view_port_unchecked (c : *connection,
                                                   screen :  u16) -> get_view_port_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xf86vidmode_get_view_port_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_xf86vidmode_get_view_port_reply (c : *connection,
                                               cookie : get_view_port_cookie,
                                               e : **generic_error) -> *get_view_port_reply;

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
unsafe fn xcb_xf86vidmode_set_view_port_checked (c : *connection,
                                                 screen :  u16,
                                                 x :  u32,
                                                 y :  u32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xf86vidmode_set_view_port (c : *connection,
                                         screen :  u16,
                                         x :  u32,
                                         y :  u32) -> void_cookie;

unsafe fn xcb_xf86vidmode_get_dot_clocks_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xf86vidmode_get_dot_clocks (c : *connection,
                                          screen :  u16) -> get_dot_clocks_cookie;

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
unsafe fn xcb_xf86vidmode_get_dot_clocks_unchecked (c : *connection,
                                                    screen :  u16) -> get_dot_clocks_cookie;

unsafe fn xcb_xf86vidmode_get_dot_clocks_clock (R : *get_dot_clocks_reply) -> *u32;


unsafe fn xcb_xf86vidmode_get_dot_clocks_clock_length (R : *get_dot_clocks_reply) -> c_int;


unsafe fn xcb_xf86vidmode_get_dot_clocks_clock_end (R : *get_dot_clocks_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xf86vidmode_get_dot_clocks_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_xf86vidmode_get_dot_clocks_reply (c : *connection,
                                                cookie : get_dot_clocks_cookie,
                                                e : **generic_error) -> *get_dot_clocks_reply;

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
unsafe fn xcb_xf86vidmode_set_client_version_checked (c : *connection,
                                                      major :  u16,
                                                      minor :  u16) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xf86vidmode_set_client_version (c : *connection,
                                              major :  u16,
                                              minor :  u16) -> void_cookie;

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
unsafe fn xcb_xf86vidmode_set_gamma_checked (c : *connection,
                                             screen :  u16,
                                             red :  u32,
                                             green :  u32,
                                             blue :  u32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xf86vidmode_set_gamma (c : *connection,
                                     screen :  u16,
                                     red :  u32,
                                     green :  u32,
                                     blue :  u32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xf86vidmode_get_gamma (c : *connection,
                                     screen :  u16) -> get_gamma_cookie;

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
unsafe fn xcb_xf86vidmode_get_gamma_unchecked (c : *connection,
                                               screen :  u16) -> get_gamma_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xf86vidmode_get_gamma_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_xf86vidmode_get_gamma_reply (c : *connection,
                                           cookie : get_gamma_cookie,
                                           e : **generic_error) -> *get_gamma_reply;

unsafe fn xcb_xf86vidmode_get_gamma_ramp_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xf86vidmode_get_gamma_ramp (c : *connection,
                                          screen :  u16,
                                          size :  u16) -> get_gamma_ramp_cookie;

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
unsafe fn xcb_xf86vidmode_get_gamma_ramp_unchecked (c : *connection,
                                                    screen :  u16,
                                                    size :  u16) -> get_gamma_ramp_cookie;

unsafe fn xcb_xf86vidmode_get_gamma_ramp_red (R : *get_gamma_ramp_reply) -> *u16;


unsafe fn xcb_xf86vidmode_get_gamma_ramp_red_length (R : *get_gamma_ramp_reply) -> c_int;


unsafe fn xcb_xf86vidmode_get_gamma_ramp_red_end (R : *get_gamma_ramp_reply) -> generic_iterator;

unsafe fn xcb_xf86vidmode_get_gamma_ramp_green (R : *get_gamma_ramp_reply) -> *u16;


unsafe fn xcb_xf86vidmode_get_gamma_ramp_green_length (R : *get_gamma_ramp_reply) -> c_int;


unsafe fn xcb_xf86vidmode_get_gamma_ramp_green_end (R : *get_gamma_ramp_reply) -> generic_iterator;

unsafe fn xcb_xf86vidmode_get_gamma_ramp_blue (R : *get_gamma_ramp_reply) -> *u16;


unsafe fn xcb_xf86vidmode_get_gamma_ramp_blue_length (R : *get_gamma_ramp_reply) -> c_int;


unsafe fn xcb_xf86vidmode_get_gamma_ramp_blue_end (R : *get_gamma_ramp_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xf86vidmode_get_gamma_ramp_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_xf86vidmode_get_gamma_ramp_reply (c : *connection,
                                                cookie : get_gamma_ramp_cookie,
                                                e : **generic_error) -> *get_gamma_ramp_reply;

unsafe fn xcb_xf86vidmode_set_gamma_ramp_sizeof (_buffer :  *c_void) -> c_int;

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
unsafe fn xcb_xf86vidmode_set_gamma_ramp_checked (c : *connection,
                                                  screen :  u16,
                                                  size :  u16,
                                                  red : *u16,
                                                  green : *u16,
                                                  blue : *u16) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xf86vidmode_set_gamma_ramp (c : *connection,
                                          screen :  u16,
                                          size :  u16,
                                          red : *u16,
                                          green : *u16,
                                          blue : *u16) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xf86vidmode_get_gamma_ramp_size (c : *connection,
                                               screen :  u16) -> get_gamma_ramp_size_cookie;

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
unsafe fn xcb_xf86vidmode_get_gamma_ramp_size_unchecked (c : *connection,
                                                         screen :  u16) -> get_gamma_ramp_size_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xf86vidmode_get_gamma_ramp_size_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_xf86vidmode_get_gamma_ramp_size_reply (c : *connection,
                                                     cookie : get_gamma_ramp_size_cookie,
                                                     e : **generic_error) -> *get_gamma_ramp_size_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xf86vidmode_get_permissions (c : *connection,
                                           screen :  u16) -> get_permissions_cookie;

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
unsafe fn xcb_xf86vidmode_get_permissions_unchecked (c : *connection,
                                                     screen :  u16) -> get_permissions_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xf86vidmode_get_permissions_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_xf86vidmode_get_permissions_reply (c : *connection,
                                                 cookie : get_permissions_cookie,
                                                 e : **generic_error) -> *get_permissions_reply;
}

