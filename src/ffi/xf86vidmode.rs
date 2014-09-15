/*
 * This file generated automatically from xf86vidmode.xml by r_client.py.
 * Edit at your peril.
 */

//Make the compiler quiet
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use std;
use libc::*;
use ffi;

pub static XF86VIDMODE_MAJOR_VERSION : c_uint = 2;
pub static XF86VIDMODE_MINOR_VERSION : c_uint = 2;

pub type syncrange = u32;
/**
 * @brief syncrange_iterator
 **/
pub struct syncrange_iterator {
    pub data : *mut syncrange,
    pub rem  : c_int,
    pub index: c_int
}


pub type dotclock = u32;
/**
 * @brief dotclock_iterator
 **/
pub struct dotclock_iterator {
    pub data : *mut dotclock,
    pub rem  : c_int,
    pub index: c_int
}


pub struct mode_info {
     pub dotclock :     dotclock,
     pub hdisplay :     u16,
     pub hsyncstart :   u16,
     pub hsyncend :     u16,
     pub htotal :       u16,
     pub hskew :        u32,
     pub vdisplay :     u16,
     pub vsyncstart :   u16,
     pub vsyncend :     u16,
     pub vtotal :       u16,
     pub pad0 :         [u8,..4],
     pub flags :        u32,
     pub pad1 :         [u8,..12],
     pub privsize :     u32
}

/**
 * @brief mode_info_iterator
 **/
pub struct mode_info_iterator {
    pub data : *mut mode_info,
    pub rem  : c_int,
    pub index: c_int
}


pub struct query_version_cookie {
    sequence : c_uint
}


pub struct query_version_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16
}


pub struct query_version_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub major_version :   u16,
     pub minor_version :   u16
}


pub struct get_mode_line_cookie {
    sequence : c_uint
}


pub struct get_mode_line_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub screen :         u16,
     pub pad0 :           [u8,..2]
}


pub struct get_mode_line_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub dotclock :        dotclock,
     pub hdisplay :        u16,
     pub hsyncstart :      u16,
     pub hsyncend :        u16,
     pub htotal :          u16,
     pub hskew :           u16,
     pub vdisplay :        u16,
     pub vsyncstart :      u16,
     pub vsyncend :        u16,
     pub vtotal :          u16,
     pub pad1 :            [u8,..2],
     pub flags :           u32,
     pub pad2 :            [u8,..12],
     pub privsize :        u32
}



pub struct mod_mode_line_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub screen :         u32,
     pub hdisplay :       u16,
     pub hsyncstart :     u16,
     pub hsyncend :       u16,
     pub htotal :         u16,
     pub hskew :          u16,
     pub vdisplay :       u16,
     pub vsyncstart :     u16,
     pub vsyncend :       u16,
     pub vtotal :         u16,
     pub pad0 :           [u8,..2],
     pub flags :          u32,
     pub pad1 :           [u8,..12],
     pub privsize :       u32
}



pub struct switch_mode_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub screen :         u16,
     pub zoom :           u16
}


pub struct get_monitor_cookie {
    sequence : c_uint
}


pub struct get_monitor_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub screen :         u16,
     pub pad0 :           [u8,..2]
}


pub struct get_monitor_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub vendor_length :   u8,
     pub model_length :    u8,
     pub num_hsync :       u8,
     pub num_vsync :       u8,
     pub pad1 :            [u8,..20]
}



pub struct lock_mode_switch_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub screen :         u16,
     pub lock :           u16
}


pub struct get_all_mode_lines_cookie {
    sequence : c_uint
}


pub struct get_all_mode_lines_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub screen :         u16,
     pub pad0 :           [u8,..2]
}


pub struct get_all_mode_lines_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub modecount :       u32,
     pub pad1 :            [u8,..20]
}



pub struct add_mode_line_request {
     pub major_opcode :       u8,
     pub minor_opcode :       u8,
     pub length :             u16,
     pub screen :             u32,
     pub dotclock :           dotclock,
     pub hdisplay :           u16,
     pub hsyncstart :         u16,
     pub hsyncend :           u16,
     pub htotal :             u16,
     pub hskew :              u16,
     pub vdisplay :           u16,
     pub vsyncstart :         u16,
     pub vsyncend :           u16,
     pub vtotal :             u16,
     pub pad0 :               [u8,..2],
     pub flags :              u32,
     pub pad1 :               [u8,..12],
     pub privsize :           u32,
     pub after_dotclock :     dotclock,
     pub after_hdisplay :     u16,
     pub after_hsyncstart :   u16,
     pub after_hsyncend :     u16,
     pub after_htotal :       u16,
     pub after_hskew :        u16,
     pub after_vdisplay :     u16,
     pub after_vsyncstart :   u16,
     pub after_vsyncend :     u16,
     pub after_vtotal :       u16,
     pub pad2 :               [u8,..2],
     pub after_flags :        u32,
     pub pad3 :               [u8,..12]
}



pub struct delete_mode_line_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub screen :         u32,
     pub dotclock :       dotclock,
     pub hdisplay :       u16,
     pub hsyncstart :     u16,
     pub hsyncend :       u16,
     pub htotal :         u16,
     pub hskew :          u16,
     pub vdisplay :       u16,
     pub vsyncstart :     u16,
     pub vsyncend :       u16,
     pub vtotal :         u16,
     pub pad0 :           [u8,..2],
     pub flags :          u32,
     pub pad1 :           [u8,..12],
     pub privsize :       u32
}


pub struct validate_mode_line_cookie {
    sequence : c_uint
}


pub struct validate_mode_line_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub screen :         u32,
     pub dotclock :       dotclock,
     pub hdisplay :       u16,
     pub hsyncstart :     u16,
     pub hsyncend :       u16,
     pub htotal :         u16,
     pub hskew :          u16,
     pub vdisplay :       u16,
     pub vsyncstart :     u16,
     pub vsyncend :       u16,
     pub vtotal :         u16,
     pub pad0 :           [u8,..2],
     pub flags :          u32,
     pub pad1 :           [u8,..12],
     pub privsize :       u32
}


pub struct validate_mode_line_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub status :          u32,
     pub pad1 :            [u8,..20]
}



pub struct switch_to_mode_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub screen :         u32,
     pub dotclock :       dotclock,
     pub hdisplay :       u16,
     pub hsyncstart :     u16,
     pub hsyncend :       u16,
     pub htotal :         u16,
     pub hskew :          u16,
     pub vdisplay :       u16,
     pub vsyncstart :     u16,
     pub vsyncend :       u16,
     pub vtotal :         u16,
     pub pad0 :           [u8,..2],
     pub flags :          u32,
     pub pad1 :           [u8,..12],
     pub privsize :       u32
}


pub struct get_view_port_cookie {
    sequence : c_uint
}


pub struct get_view_port_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub screen :         u16,
     pub pad0 :           [u8,..2]
}


pub struct get_view_port_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub x :               u32,
     pub y :               u32,
     pub pad1 :            [u8,..16]
}



pub struct set_view_port_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub screen :         u16,
     pub pad0 :           [u8,..2],
     pub x :              u32,
     pub y :              u32
}


pub struct get_dot_clocks_cookie {
    sequence : c_uint
}


pub struct get_dot_clocks_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub screen :         u16,
     pub pad0 :           [u8,..2]
}


pub struct get_dot_clocks_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub flags :           u32,
     pub clocks :          u32,
     pub maxclocks :       u32,
     pub pad1 :            [u8,..12]
}



pub struct set_client_version_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub major :          u16,
     pub minor :          u16
}



pub struct set_gamma_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub screen :         u16,
     pub pad0 :           [u8,..2],
     pub red :            u32,
     pub green :          u32,
     pub blue :           u32,
     pub pad1 :           [u8,..12]
}


pub struct get_gamma_cookie {
    sequence : c_uint
}


pub struct get_gamma_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub screen :         u16,
     pub pad0 :           [u8,..26]
}


pub struct get_gamma_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub red :             u32,
     pub green :           u32,
     pub blue :            u32,
     pub pad1 :            [u8,..12]
}


pub struct get_gamma_ramp_cookie {
    sequence : c_uint
}


pub struct get_gamma_ramp_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub screen :         u16,
     pub size :           u16
}


pub struct get_gamma_ramp_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub size :            u16,
     pub pad1 :            [u8,..22]
}



pub struct set_gamma_ramp_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub screen :         u16,
     pub size :           u16
}


pub struct get_gamma_ramp_size_cookie {
    sequence : c_uint
}


pub struct get_gamma_ramp_size_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub screen :         u16,
     pub pad0 :           [u8,..2]
}


pub struct get_gamma_ramp_size_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub size :            u16,
     pub pad1 :            [u8,..22]
}


pub struct get_permissions_cookie {
    sequence : c_uint
}


pub struct get_permissions_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub screen :         u16,
     pub pad0 :           [u8,..2]
}


pub struct get_permissions_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub permissions :     u32,
     pub pad1 :            [u8,..20]
}



pub struct bad_clock_error {
     pub response_type :   u8,
     pub error_code :      u8,
     pub sequence :        u16
}



pub struct bad_h_timings_error {
     pub response_type :   u8,
     pub error_code :      u8,
     pub sequence :        u16
}



pub struct bad_v_timings_error {
     pub response_type :   u8,
     pub error_code :      u8,
     pub sequence :        u16
}



pub struct mode_unsuitable_error {
     pub response_type :   u8,
     pub error_code :      u8,
     pub sequence :        u16
}



pub struct extension_disabled_error {
     pub response_type :   u8,
     pub error_code :      u8,
     pub sequence :        u16
}



pub struct client_not_local_error {
     pub response_type :   u8,
     pub error_code :      u8,
     pub sequence :        u16
}



pub struct zoom_locked_error {
     pub response_type :   u8,
     pub error_code :      u8,
     pub sequence :        u16
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
pub fn xcb_xf86vidmode_syncrange_next (i:*mut syncrange_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An syncrange_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_xf86vidmode_syncrange_end (i:syncrange_iterator) -> ffi::base::generic_iterator;

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
pub fn xcb_xf86vidmode_dotclock_next (i:*mut dotclock_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An dotclock_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_xf86vidmode_dotclock_end (i:dotclock_iterator) -> ffi::base::generic_iterator;

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
pub fn xcb_xf86vidmode_mode_info_next (i:*mut mode_info_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An mode_info_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_xf86vidmode_mode_info_end (i:mode_info_iterator) -> ffi::base::generic_iterator;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xf86vidmode_query_version (c : *mut ffi::base::connection) -> query_version_cookie;

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
pub fn xcb_xf86vidmode_query_version_unchecked (c : *mut ffi::base::connection) -> query_version_cookie;

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
pub fn xcb_xf86vidmode_query_version_reply (c : *mut ffi::base::connection,
                                               cookie : query_version_cookie,
                                               e : *mut *mut ffi::base::generic_error) -> *mut query_version_reply;

pub fn xcb_xf86vidmode_get_mode_line_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xf86vidmode_get_mode_line (c : *mut ffi::base::connection,
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
pub fn xcb_xf86vidmode_get_mode_line_unchecked (c : *mut ffi::base::connection,
                                                   screen :  u16) -> get_mode_line_cookie;

pub fn xcb_xf86vidmode_get_mode_line_private (R : *mut get_mode_line_reply) -> *mut u8;


pub fn xcb_xf86vidmode_get_mode_line_private_length (R : *mut get_mode_line_reply) -> c_int;


pub fn xcb_xf86vidmode_get_mode_line_private_end (R : *mut get_mode_line_reply) -> ffi::base::generic_iterator;

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
pub fn xcb_xf86vidmode_get_mode_line_reply (c : *mut ffi::base::connection,
                                               cookie : get_mode_line_cookie,
                                               e : *mut *mut ffi::base::generic_error) -> *mut get_mode_line_reply;

pub fn xcb_xf86vidmode_mod_mode_line_sizeof (_buffer :  *mut c_void) -> c_int;

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
pub fn xcb_xf86vidmode_mod_mode_line_checked (c : *mut ffi::base::connection,
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
                                                 private : *mut u8) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xf86vidmode_mod_mode_line (c : *mut ffi::base::connection,
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
                                         private : *mut u8) -> ffi::base::void_cookie;

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
pub fn xcb_xf86vidmode_switch_mode_checked (c : *mut ffi::base::connection,
                                               screen :  u16,
                                               zoom :  u16) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xf86vidmode_switch_mode (c : *mut ffi::base::connection,
                                       screen :  u16,
                                       zoom :  u16) -> ffi::base::void_cookie;

pub fn xcb_xf86vidmode_get_monitor_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xf86vidmode_get_monitor (c : *mut ffi::base::connection,
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
pub fn xcb_xf86vidmode_get_monitor_unchecked (c : *mut ffi::base::connection,
                                                 screen :  u16) -> get_monitor_cookie;

pub fn xcb_xf86vidmode_get_monitor_hsync (R : *mut get_monitor_reply) -> *mut syncrange;


pub fn xcb_xf86vidmode_get_monitor_hsync_length (R : *mut get_monitor_reply) -> c_int;


pub fn xcb_xf86vidmode_get_monitor_hsync_end (R : *mut get_monitor_reply) -> ffi::base::generic_iterator;

pub fn xcb_xf86vidmode_get_monitor_vsync (R : *mut get_monitor_reply) -> *mut syncrange;


pub fn xcb_xf86vidmode_get_monitor_vsync_length (R : *mut get_monitor_reply) -> c_int;


pub fn xcb_xf86vidmode_get_monitor_vsync_end (R : *mut get_monitor_reply) -> ffi::base::generic_iterator;

pub fn xcb_xf86vidmode_get_monitor_vendor (R : *mut get_monitor_reply) -> *mut c_char;


pub fn xcb_xf86vidmode_get_monitor_vendor_length (R : *mut get_monitor_reply) -> c_int;


pub fn xcb_xf86vidmode_get_monitor_vendor_end (R : *mut get_monitor_reply) -> ffi::base::generic_iterator;

pub fn xcb_xf86vidmode_get_monitor_alignment_pad (R : *mut get_monitor_reply) -> *mut c_void;


pub fn xcb_xf86vidmode_get_monitor_alignment_pad_length (R : *mut get_monitor_reply) -> c_int;


pub fn xcb_xf86vidmode_get_monitor_alignment_pad_end (R : *mut get_monitor_reply) -> ffi::base::generic_iterator;

pub fn xcb_xf86vidmode_get_monitor_model (R : *mut get_monitor_reply) -> *mut c_char;


pub fn xcb_xf86vidmode_get_monitor_model_length (R : *mut get_monitor_reply) -> c_int;


pub fn xcb_xf86vidmode_get_monitor_model_end (R : *mut get_monitor_reply) -> ffi::base::generic_iterator;

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
pub fn xcb_xf86vidmode_get_monitor_reply (c : *mut ffi::base::connection,
                                             cookie : get_monitor_cookie,
                                             e : *mut *mut ffi::base::generic_error) -> *mut get_monitor_reply;

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
pub fn xcb_xf86vidmode_lock_mode_switch_checked (c : *mut ffi::base::connection,
                                                    screen :  u16,
                                                    lock :  u16) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xf86vidmode_lock_mode_switch (c : *mut ffi::base::connection,
                                            screen :  u16,
                                            lock :  u16) -> ffi::base::void_cookie;

pub fn xcb_xf86vidmode_get_all_mode_lines_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xf86vidmode_get_all_mode_lines (c : *mut ffi::base::connection,
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
pub fn xcb_xf86vidmode_get_all_mode_lines_unchecked (c : *mut ffi::base::connection,
                                                        screen :  u16) -> get_all_mode_lines_cookie;

pub fn xcb_xf86vidmode_get_all_mode_lines_modeinfo (R : *mut get_all_mode_lines_reply) -> *mut mode_info;


pub fn xcb_xf86vidmode_get_all_mode_lines_modeinfo_length (R : *mut get_all_mode_lines_reply) -> c_int;

pub fn xcb_xf86vidmode_get_all_mode_lines_modeinfo_iterator (R : *mut get_all_mode_lines_reply) -> mode_info_iterator;

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
pub fn xcb_xf86vidmode_get_all_mode_lines_reply (c : *mut ffi::base::connection,
                                                    cookie : get_all_mode_lines_cookie,
                                                    e : *mut *mut ffi::base::generic_error) -> *mut get_all_mode_lines_reply;

pub fn xcb_xf86vidmode_add_mode_line_sizeof (_buffer :  *mut c_void) -> c_int;

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
pub fn xcb_xf86vidmode_add_mode_line_checked (c : *mut ffi::base::connection,
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
                                                 private : *mut u8) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xf86vidmode_add_mode_line (c : *mut ffi::base::connection,
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
                                         private : *mut u8) -> ffi::base::void_cookie;

pub fn xcb_xf86vidmode_delete_mode_line_sizeof (_buffer :  *mut c_void) -> c_int;

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
pub fn xcb_xf86vidmode_delete_mode_line_checked (c : *mut ffi::base::connection,
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
                                                    private : *mut u8) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xf86vidmode_delete_mode_line (c : *mut ffi::base::connection,
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
                                            private : *mut u8) -> ffi::base::void_cookie;

pub fn xcb_xf86vidmode_validate_mode_line_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xf86vidmode_validate_mode_line (c : *mut ffi::base::connection,
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
                                              private : *mut u8) -> validate_mode_line_cookie;

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
pub fn xcb_xf86vidmode_validate_mode_line_unchecked (c : *mut ffi::base::connection,
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
                                                        private : *mut u8) -> validate_mode_line_cookie;

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
pub fn xcb_xf86vidmode_validate_mode_line_reply (c : *mut ffi::base::connection,
                                                    cookie : validate_mode_line_cookie,
                                                    e : *mut *mut ffi::base::generic_error) -> *mut validate_mode_line_reply;

pub fn xcb_xf86vidmode_switch_to_mode_sizeof (_buffer :  *mut c_void) -> c_int;

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
pub fn xcb_xf86vidmode_switch_to_mode_checked (c : *mut ffi::base::connection,
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
                                                  private : *mut u8) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xf86vidmode_switch_to_mode (c : *mut ffi::base::connection,
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
                                          private : *mut u8) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xf86vidmode_get_view_port (c : *mut ffi::base::connection,
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
pub fn xcb_xf86vidmode_get_view_port_unchecked (c : *mut ffi::base::connection,
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
pub fn xcb_xf86vidmode_get_view_port_reply (c : *mut ffi::base::connection,
                                               cookie : get_view_port_cookie,
                                               e : *mut *mut ffi::base::generic_error) -> *mut get_view_port_reply;

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
pub fn xcb_xf86vidmode_set_view_port_checked (c : *mut ffi::base::connection,
                                                 screen :  u16,
                                                 x :  u32,
                                                 y :  u32) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xf86vidmode_set_view_port (c : *mut ffi::base::connection,
                                         screen :  u16,
                                         x :  u32,
                                         y :  u32) -> ffi::base::void_cookie;

pub fn xcb_xf86vidmode_get_dot_clocks_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xf86vidmode_get_dot_clocks (c : *mut ffi::base::connection,
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
pub fn xcb_xf86vidmode_get_dot_clocks_unchecked (c : *mut ffi::base::connection,
                                                    screen :  u16) -> get_dot_clocks_cookie;

pub fn xcb_xf86vidmode_get_dot_clocks_clock (R : *mut get_dot_clocks_reply) -> *mut u32;


pub fn xcb_xf86vidmode_get_dot_clocks_clock_length (R : *mut get_dot_clocks_reply) -> c_int;


pub fn xcb_xf86vidmode_get_dot_clocks_clock_end (R : *mut get_dot_clocks_reply) -> ffi::base::generic_iterator;

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
pub fn xcb_xf86vidmode_get_dot_clocks_reply (c : *mut ffi::base::connection,
                                                cookie : get_dot_clocks_cookie,
                                                e : *mut *mut ffi::base::generic_error) -> *mut get_dot_clocks_reply;

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
pub fn xcb_xf86vidmode_set_client_version_checked (c : *mut ffi::base::connection,
                                                      major :  u16,
                                                      minor :  u16) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xf86vidmode_set_client_version (c : *mut ffi::base::connection,
                                              major :  u16,
                                              minor :  u16) -> ffi::base::void_cookie;

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
pub fn xcb_xf86vidmode_set_gamma_checked (c : *mut ffi::base::connection,
                                             screen :  u16,
                                             red :  u32,
                                             green :  u32,
                                             blue :  u32) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xf86vidmode_set_gamma (c : *mut ffi::base::connection,
                                     screen :  u16,
                                     red :  u32,
                                     green :  u32,
                                     blue :  u32) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xf86vidmode_get_gamma (c : *mut ffi::base::connection,
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
pub fn xcb_xf86vidmode_get_gamma_unchecked (c : *mut ffi::base::connection,
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
pub fn xcb_xf86vidmode_get_gamma_reply (c : *mut ffi::base::connection,
                                           cookie : get_gamma_cookie,
                                           e : *mut *mut ffi::base::generic_error) -> *mut get_gamma_reply;

pub fn xcb_xf86vidmode_get_gamma_ramp_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xf86vidmode_get_gamma_ramp (c : *mut ffi::base::connection,
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
pub fn xcb_xf86vidmode_get_gamma_ramp_unchecked (c : *mut ffi::base::connection,
                                                    screen :  u16,
                                                    size :  u16) -> get_gamma_ramp_cookie;

pub fn xcb_xf86vidmode_get_gamma_ramp_red (R : *mut get_gamma_ramp_reply) -> *mut u16;


pub fn xcb_xf86vidmode_get_gamma_ramp_red_length (R : *mut get_gamma_ramp_reply) -> c_int;


pub fn xcb_xf86vidmode_get_gamma_ramp_red_end (R : *mut get_gamma_ramp_reply) -> ffi::base::generic_iterator;

pub fn xcb_xf86vidmode_get_gamma_ramp_green (R : *mut get_gamma_ramp_reply) -> *mut u16;


pub fn xcb_xf86vidmode_get_gamma_ramp_green_length (R : *mut get_gamma_ramp_reply) -> c_int;


pub fn xcb_xf86vidmode_get_gamma_ramp_green_end (R : *mut get_gamma_ramp_reply) -> ffi::base::generic_iterator;

pub fn xcb_xf86vidmode_get_gamma_ramp_blue (R : *mut get_gamma_ramp_reply) -> *mut u16;


pub fn xcb_xf86vidmode_get_gamma_ramp_blue_length (R : *mut get_gamma_ramp_reply) -> c_int;


pub fn xcb_xf86vidmode_get_gamma_ramp_blue_end (R : *mut get_gamma_ramp_reply) -> ffi::base::generic_iterator;

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
pub fn xcb_xf86vidmode_get_gamma_ramp_reply (c : *mut ffi::base::connection,
                                                cookie : get_gamma_ramp_cookie,
                                                e : *mut *mut ffi::base::generic_error) -> *mut get_gamma_ramp_reply;

pub fn xcb_xf86vidmode_set_gamma_ramp_sizeof (_buffer :  *mut c_void) -> c_int;

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
pub fn xcb_xf86vidmode_set_gamma_ramp_checked (c : *mut ffi::base::connection,
                                                  screen :  u16,
                                                  size :  u16,
                                                  red : *mut u16,
                                                  green : *mut u16,
                                                  blue : *mut u16) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xf86vidmode_set_gamma_ramp (c : *mut ffi::base::connection,
                                          screen :  u16,
                                          size :  u16,
                                          red : *mut u16,
                                          green : *mut u16,
                                          blue : *mut u16) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xf86vidmode_get_gamma_ramp_size (c : *mut ffi::base::connection,
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
pub fn xcb_xf86vidmode_get_gamma_ramp_size_unchecked (c : *mut ffi::base::connection,
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
pub fn xcb_xf86vidmode_get_gamma_ramp_size_reply (c : *mut ffi::base::connection,
                                                     cookie : get_gamma_ramp_size_cookie,
                                                     e : *mut *mut ffi::base::generic_error) -> *mut get_gamma_ramp_size_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xf86vidmode_get_permissions (c : *mut ffi::base::connection,
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
pub fn xcb_xf86vidmode_get_permissions_unchecked (c : *mut ffi::base::connection,
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
pub fn xcb_xf86vidmode_get_permissions_reply (c : *mut ffi::base::connection,
                                                 cookie : get_permissions_cookie,
                                                 e : *mut *mut ffi::base::generic_error) -> *mut get_permissions_reply;
}

