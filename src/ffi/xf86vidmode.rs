/*
 * This file generated automatically from xf86vidmode.xml by r_client.py.
 * Edit at your peril.
 */

//Make the compiler quiet
#[allow(unused_imports)];
#[allow(non_camel_case_types)];
use std;
use std::libc::*;
use std::{cast,num,ptr,str,libc};
use std::to_bytes::ToBytes;
use ffi::base::*;
use ffi;

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



pub struct set_client_version_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    major :          u16,
    minor :          u16
}



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



pub struct bad_clock_error {
    response_type :   u8,
    error_code :      u8,
    sequence :        u16
}



pub struct bad_h_timings_error {
    response_type :   u8,
    error_code :      u8,
    sequence :        u16
}



pub struct bad_v_timings_error {
    response_type :   u8,
    error_code :      u8,
    sequence :        u16
}



pub struct mode_unsuitable_error {
    response_type :   u8,
    error_code :      u8,
    sequence :        u16
}



pub struct extension_disabled_error {
    response_type :   u8,
    error_code :      u8,
    sequence :        u16
}



pub struct client_not_local_error {
    response_type :   u8,
    error_code :      u8,
    sequence :        u16
}



pub struct zoom_locked_error {
    response_type :   u8,
    error_code :      u8,
    sequence :        u16
}

pub extern "C" {

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
pub unsafe fn xcb_xf86vidmode_syncrange_next (i:*syncrange_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An syncrange_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub unsafe fn xcb_xf86vidmode_syncrange_end (i:syncrange_iterator) -> generic_iterator;

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
pub unsafe fn xcb_xf86vidmode_dotclock_next (i:*dotclock_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An dotclock_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub unsafe fn xcb_xf86vidmode_dotclock_end (i:dotclock_iterator) -> generic_iterator;

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
pub unsafe fn xcb_xf86vidmode_mode_info_next (i:*mode_info_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An mode_info_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub unsafe fn xcb_xf86vidmode_mode_info_end (i:mode_info_iterator) -> generic_iterator;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_xf86vidmode_query_version (c : *connection) -> query_version_cookie;

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
pub unsafe fn xcb_xf86vidmode_query_version_unchecked (c : *connection) -> query_version_cookie;

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
pub unsafe fn xcb_xf86vidmode_query_version_reply (c : *connection,
                                               cookie : query_version_cookie,
                                               e : **generic_error) -> *query_version_reply;

pub unsafe fn xcb_xf86vidmode_get_mode_line_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_xf86vidmode_get_mode_line (c : *connection,
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
pub unsafe fn xcb_xf86vidmode_get_mode_line_unchecked (c : *connection,
                                                   screen :  u16) -> get_mode_line_cookie;

pub unsafe fn xcb_xf86vidmode_get_mode_line_private (R : *get_mode_line_reply) -> *u8;


pub unsafe fn xcb_xf86vidmode_get_mode_line_private_length (R : *get_mode_line_reply) -> c_int;


pub unsafe fn xcb_xf86vidmode_get_mode_line_private_end (R : *get_mode_line_reply) -> generic_iterator;

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
pub unsafe fn xcb_xf86vidmode_get_mode_line_reply (c : *connection,
                                               cookie : get_mode_line_cookie,
                                               e : **generic_error) -> *get_mode_line_reply;

pub unsafe fn xcb_xf86vidmode_mod_mode_line_sizeof (_buffer :  *c_void) -> c_int;

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
pub unsafe fn xcb_xf86vidmode_mod_mode_line_checked (c : *connection,
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
pub unsafe fn xcb_xf86vidmode_mod_mode_line (c : *connection,
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
pub unsafe fn xcb_xf86vidmode_switch_mode_checked (c : *connection,
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
pub unsafe fn xcb_xf86vidmode_switch_mode (c : *connection,
                                       screen :  u16,
                                       zoom :  u16) -> void_cookie;

pub unsafe fn xcb_xf86vidmode_get_monitor_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_xf86vidmode_get_monitor (c : *connection,
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
pub unsafe fn xcb_xf86vidmode_get_monitor_unchecked (c : *connection,
                                                 screen :  u16) -> get_monitor_cookie;

pub unsafe fn xcb_xf86vidmode_get_monitor_hsync (R : *get_monitor_reply) -> *syncrange;


pub unsafe fn xcb_xf86vidmode_get_monitor_hsync_length (R : *get_monitor_reply) -> c_int;


pub unsafe fn xcb_xf86vidmode_get_monitor_hsync_end (R : *get_monitor_reply) -> generic_iterator;

pub unsafe fn xcb_xf86vidmode_get_monitor_vsync (R : *get_monitor_reply) -> *syncrange;


pub unsafe fn xcb_xf86vidmode_get_monitor_vsync_length (R : *get_monitor_reply) -> c_int;


pub unsafe fn xcb_xf86vidmode_get_monitor_vsync_end (R : *get_monitor_reply) -> generic_iterator;

pub unsafe fn xcb_xf86vidmode_get_monitor_vendor (R : *get_monitor_reply) -> *c_char;


pub unsafe fn xcb_xf86vidmode_get_monitor_vendor_length (R : *get_monitor_reply) -> c_int;


pub unsafe fn xcb_xf86vidmode_get_monitor_vendor_end (R : *get_monitor_reply) -> generic_iterator;

pub unsafe fn xcb_xf86vidmode_get_monitor_alignment_pad (R : *get_monitor_reply) -> *c_void;


pub unsafe fn xcb_xf86vidmode_get_monitor_alignment_pad_length (R : *get_monitor_reply) -> c_int;


pub unsafe fn xcb_xf86vidmode_get_monitor_alignment_pad_end (R : *get_monitor_reply) -> generic_iterator;

pub unsafe fn xcb_xf86vidmode_get_monitor_model (R : *get_monitor_reply) -> *c_char;


pub unsafe fn xcb_xf86vidmode_get_monitor_model_length (R : *get_monitor_reply) -> c_int;


pub unsafe fn xcb_xf86vidmode_get_monitor_model_end (R : *get_monitor_reply) -> generic_iterator;

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
pub unsafe fn xcb_xf86vidmode_get_monitor_reply (c : *connection,
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
pub unsafe fn xcb_xf86vidmode_lock_mode_switch_checked (c : *connection,
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
pub unsafe fn xcb_xf86vidmode_lock_mode_switch (c : *connection,
                                            screen :  u16,
                                            lock :  u16) -> void_cookie;

pub unsafe fn xcb_xf86vidmode_get_all_mode_lines_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_xf86vidmode_get_all_mode_lines (c : *connection,
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
pub unsafe fn xcb_xf86vidmode_get_all_mode_lines_unchecked (c : *connection,
                                                        screen :  u16) -> get_all_mode_lines_cookie;

pub unsafe fn xcb_xf86vidmode_get_all_mode_lines_modeinfo (R : *get_all_mode_lines_reply) -> *mode_info;


pub unsafe fn xcb_xf86vidmode_get_all_mode_lines_modeinfo_length (R : *get_all_mode_lines_reply) -> c_int;

pub unsafe fn xcb_xf86vidmode_get_all_mode_lines_modeinfo_iterator (R : *get_all_mode_lines_reply) -> mode_info_iterator;

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
pub unsafe fn xcb_xf86vidmode_get_all_mode_lines_reply (c : *connection,
                                                    cookie : get_all_mode_lines_cookie,
                                                    e : **generic_error) -> *get_all_mode_lines_reply;

pub unsafe fn xcb_xf86vidmode_add_mode_line_sizeof (_buffer :  *c_void) -> c_int;

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
pub unsafe fn xcb_xf86vidmode_add_mode_line_checked (c : *connection,
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
pub unsafe fn xcb_xf86vidmode_add_mode_line (c : *connection,
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

pub unsafe fn xcb_xf86vidmode_delete_mode_line_sizeof (_buffer :  *c_void) -> c_int;

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
pub unsafe fn xcb_xf86vidmode_delete_mode_line_checked (c : *connection,
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
pub unsafe fn xcb_xf86vidmode_delete_mode_line (c : *connection,
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

pub unsafe fn xcb_xf86vidmode_validate_mode_line_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_xf86vidmode_validate_mode_line (c : *connection,
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
pub unsafe fn xcb_xf86vidmode_validate_mode_line_unchecked (c : *connection,
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
pub unsafe fn xcb_xf86vidmode_validate_mode_line_reply (c : *connection,
                                                    cookie : validate_mode_line_cookie,
                                                    e : **generic_error) -> *validate_mode_line_reply;

pub unsafe fn xcb_xf86vidmode_switch_to_mode_sizeof (_buffer :  *c_void) -> c_int;

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
pub unsafe fn xcb_xf86vidmode_switch_to_mode_checked (c : *connection,
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
pub unsafe fn xcb_xf86vidmode_switch_to_mode (c : *connection,
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
pub unsafe fn xcb_xf86vidmode_get_view_port (c : *connection,
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
pub unsafe fn xcb_xf86vidmode_get_view_port_unchecked (c : *connection,
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
pub unsafe fn xcb_xf86vidmode_get_view_port_reply (c : *connection,
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
pub unsafe fn xcb_xf86vidmode_set_view_port_checked (c : *connection,
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
pub unsafe fn xcb_xf86vidmode_set_view_port (c : *connection,
                                         screen :  u16,
                                         x :  u32,
                                         y :  u32) -> void_cookie;

pub unsafe fn xcb_xf86vidmode_get_dot_clocks_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_xf86vidmode_get_dot_clocks (c : *connection,
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
pub unsafe fn xcb_xf86vidmode_get_dot_clocks_unchecked (c : *connection,
                                                    screen :  u16) -> get_dot_clocks_cookie;

pub unsafe fn xcb_xf86vidmode_get_dot_clocks_clock (R : *get_dot_clocks_reply) -> *u32;


pub unsafe fn xcb_xf86vidmode_get_dot_clocks_clock_length (R : *get_dot_clocks_reply) -> c_int;


pub unsafe fn xcb_xf86vidmode_get_dot_clocks_clock_end (R : *get_dot_clocks_reply) -> generic_iterator;

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
pub unsafe fn xcb_xf86vidmode_get_dot_clocks_reply (c : *connection,
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
pub unsafe fn xcb_xf86vidmode_set_client_version_checked (c : *connection,
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
pub unsafe fn xcb_xf86vidmode_set_client_version (c : *connection,
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
pub unsafe fn xcb_xf86vidmode_set_gamma_checked (c : *connection,
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
pub unsafe fn xcb_xf86vidmode_set_gamma (c : *connection,
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
pub unsafe fn xcb_xf86vidmode_get_gamma (c : *connection,
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
pub unsafe fn xcb_xf86vidmode_get_gamma_unchecked (c : *connection,
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
pub unsafe fn xcb_xf86vidmode_get_gamma_reply (c : *connection,
                                           cookie : get_gamma_cookie,
                                           e : **generic_error) -> *get_gamma_reply;

pub unsafe fn xcb_xf86vidmode_get_gamma_ramp_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_xf86vidmode_get_gamma_ramp (c : *connection,
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
pub unsafe fn xcb_xf86vidmode_get_gamma_ramp_unchecked (c : *connection,
                                                    screen :  u16,
                                                    size :  u16) -> get_gamma_ramp_cookie;

pub unsafe fn xcb_xf86vidmode_get_gamma_ramp_red (R : *get_gamma_ramp_reply) -> *u16;


pub unsafe fn xcb_xf86vidmode_get_gamma_ramp_red_length (R : *get_gamma_ramp_reply) -> c_int;


pub unsafe fn xcb_xf86vidmode_get_gamma_ramp_red_end (R : *get_gamma_ramp_reply) -> generic_iterator;

pub unsafe fn xcb_xf86vidmode_get_gamma_ramp_green (R : *get_gamma_ramp_reply) -> *u16;


pub unsafe fn xcb_xf86vidmode_get_gamma_ramp_green_length (R : *get_gamma_ramp_reply) -> c_int;


pub unsafe fn xcb_xf86vidmode_get_gamma_ramp_green_end (R : *get_gamma_ramp_reply) -> generic_iterator;

pub unsafe fn xcb_xf86vidmode_get_gamma_ramp_blue (R : *get_gamma_ramp_reply) -> *u16;


pub unsafe fn xcb_xf86vidmode_get_gamma_ramp_blue_length (R : *get_gamma_ramp_reply) -> c_int;


pub unsafe fn xcb_xf86vidmode_get_gamma_ramp_blue_end (R : *get_gamma_ramp_reply) -> generic_iterator;

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
pub unsafe fn xcb_xf86vidmode_get_gamma_ramp_reply (c : *connection,
                                                cookie : get_gamma_ramp_cookie,
                                                e : **generic_error) -> *get_gamma_ramp_reply;

pub unsafe fn xcb_xf86vidmode_set_gamma_ramp_sizeof (_buffer :  *c_void) -> c_int;

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
pub unsafe fn xcb_xf86vidmode_set_gamma_ramp_checked (c : *connection,
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
pub unsafe fn xcb_xf86vidmode_set_gamma_ramp (c : *connection,
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
pub unsafe fn xcb_xf86vidmode_get_gamma_ramp_size (c : *connection,
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
pub unsafe fn xcb_xf86vidmode_get_gamma_ramp_size_unchecked (c : *connection,
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
pub unsafe fn xcb_xf86vidmode_get_gamma_ramp_size_reply (c : *connection,
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
pub unsafe fn xcb_xf86vidmode_get_permissions (c : *connection,
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
pub unsafe fn xcb_xf86vidmode_get_permissions_unchecked (c : *connection,
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
pub unsafe fn xcb_xf86vidmode_get_permissions_reply (c : *connection,
                                                 cookie : get_permissions_cookie,
                                                 e : **generic_error) -> *get_permissions_reply;
}

