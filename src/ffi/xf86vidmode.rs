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

pub const XF86VIDMODE_MAJOR_VERSION : c_uint = 2;
pub const XF86VIDMODE_MINOR_VERSION : c_uint = 2;

pub type xcb_xf86vidmode_syncrange_t = u32;
/**
 * @brief xcb_xf86vidmode_syncrange_iterator_t
 **/
#[repr(C)]
pub struct xcb_xf86vidmode_syncrange_iterator_t {
    pub data : *mut xcb_xf86vidmode_syncrange_t,
    pub rem  : c_int,
    pub index: c_int
}


pub type xcb_xf86vidmode_dotclock_t = u32;
/**
 * @brief xcb_xf86vidmode_dotclock_iterator_t
 **/
#[repr(C)]
pub struct xcb_xf86vidmode_dotclock_iterator_t {
    pub data : *mut xcb_xf86vidmode_dotclock_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_xf86vidmode_mode_info_t {
     pub dotclock :     xcb_xf86vidmode_dotclock_t,
     pub hdisplay :     u16,
     pub hsyncstart :   u16,
     pub hsyncend :     u16,
     pub htotal :       u16,
     pub hskew :        u32,
     pub vdisplay :     u16,
     pub vsyncstart :   u16,
     pub vsyncend :     u16,
     pub vtotal :       u16,
     pub pad0 :         [u8; 4],
     pub flags :        u32,
     pub pad1 :         [u8; 12],
     pub privsize :     u32
}

impl Copy for xcb_xf86vidmode_mode_info_t {}
impl Clone for xcb_xf86vidmode_mode_info_t {
    fn clone(&self) -> xcb_xf86vidmode_mode_info_t { *self }
}
/**
 * @brief xcb_xf86vidmode_mode_info_iterator_t
 **/
#[repr(C)]
pub struct xcb_xf86vidmode_mode_info_iterator_t {
    pub data : *mut xcb_xf86vidmode_mode_info_t,
    pub rem  : c_int,
    pub index: c_int
}


#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xf86vidmode_query_version_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_xf86vidmode_query_version_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16
}

impl Copy for xcb_xf86vidmode_query_version_request_t {}
impl Clone for xcb_xf86vidmode_query_version_request_t {
    fn clone(&self) -> xcb_xf86vidmode_query_version_request_t { *self }
}

#[repr(C)]
pub struct xcb_xf86vidmode_query_version_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub major_version :   u16,
     pub minor_version :   u16
}

impl Copy for xcb_xf86vidmode_query_version_reply_t {}
impl Clone for xcb_xf86vidmode_query_version_reply_t {
    fn clone(&self) -> xcb_xf86vidmode_query_version_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xf86vidmode_get_mode_line_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_xf86vidmode_get_mode_line_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub screen :         u16,
     pub pad0 :           [u8; 2]
}

impl Copy for xcb_xf86vidmode_get_mode_line_request_t {}
impl Clone for xcb_xf86vidmode_get_mode_line_request_t {
    fn clone(&self) -> xcb_xf86vidmode_get_mode_line_request_t { *self }
}

#[repr(C)]
pub struct xcb_xf86vidmode_get_mode_line_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub dotclock :        xcb_xf86vidmode_dotclock_t,
     pub hdisplay :        u16,
     pub hsyncstart :      u16,
     pub hsyncend :        u16,
     pub htotal :          u16,
     pub hskew :           u16,
     pub vdisplay :        u16,
     pub vsyncstart :      u16,
     pub vsyncend :        u16,
     pub vtotal :          u16,
     pub pad1 :            [u8; 2],
     pub flags :           u32,
     pub pad2 :            [u8; 12],
     pub privsize :        u32
}

impl Copy for xcb_xf86vidmode_get_mode_line_reply_t {}
impl Clone for xcb_xf86vidmode_get_mode_line_reply_t {
    fn clone(&self) -> xcb_xf86vidmode_get_mode_line_reply_t { *self }
}


#[repr(C)]
pub struct xcb_xf86vidmode_mod_mode_line_request_t {
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
     pub pad0 :           [u8; 2],
     pub flags :          u32,
     pub pad1 :           [u8; 12],
     pub privsize :       u32
}

impl Copy for xcb_xf86vidmode_mod_mode_line_request_t {}
impl Clone for xcb_xf86vidmode_mod_mode_line_request_t {
    fn clone(&self) -> xcb_xf86vidmode_mod_mode_line_request_t { *self }
}


#[repr(C)]
pub struct xcb_xf86vidmode_switch_mode_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub screen :         u16,
     pub zoom :           u16
}

impl Copy for xcb_xf86vidmode_switch_mode_request_t {}
impl Clone for xcb_xf86vidmode_switch_mode_request_t {
    fn clone(&self) -> xcb_xf86vidmode_switch_mode_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xf86vidmode_get_monitor_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_xf86vidmode_get_monitor_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub screen :         u16,
     pub pad0 :           [u8; 2]
}

impl Copy for xcb_xf86vidmode_get_monitor_request_t {}
impl Clone for xcb_xf86vidmode_get_monitor_request_t {
    fn clone(&self) -> xcb_xf86vidmode_get_monitor_request_t { *self }
}

#[repr(C)]
pub struct xcb_xf86vidmode_get_monitor_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub vendor_length :   u8,
     pub model_length :    u8,
     pub num_hsync :       u8,
     pub num_vsync :       u8,
     pub pad1 :            [u8; 20]
}

impl Copy for xcb_xf86vidmode_get_monitor_reply_t {}
impl Clone for xcb_xf86vidmode_get_monitor_reply_t {
    fn clone(&self) -> xcb_xf86vidmode_get_monitor_reply_t { *self }
}


#[repr(C)]
pub struct xcb_xf86vidmode_lock_mode_switch_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub screen :         u16,
     pub lock :           u16
}

impl Copy for xcb_xf86vidmode_lock_mode_switch_request_t {}
impl Clone for xcb_xf86vidmode_lock_mode_switch_request_t {
    fn clone(&self) -> xcb_xf86vidmode_lock_mode_switch_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xf86vidmode_get_all_mode_lines_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_xf86vidmode_get_all_mode_lines_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub screen :         u16,
     pub pad0 :           [u8; 2]
}

impl Copy for xcb_xf86vidmode_get_all_mode_lines_request_t {}
impl Clone for xcb_xf86vidmode_get_all_mode_lines_request_t {
    fn clone(&self) -> xcb_xf86vidmode_get_all_mode_lines_request_t { *self }
}

#[repr(C)]
pub struct xcb_xf86vidmode_get_all_mode_lines_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub modecount :       u32,
     pub pad1 :            [u8; 20]
}

impl Copy for xcb_xf86vidmode_get_all_mode_lines_reply_t {}
impl Clone for xcb_xf86vidmode_get_all_mode_lines_reply_t {
    fn clone(&self) -> xcb_xf86vidmode_get_all_mode_lines_reply_t { *self }
}


#[repr(C)]
pub struct xcb_xf86vidmode_add_mode_line_request_t {
     pub major_opcode :       u8,
     pub minor_opcode :       u8,
     pub length :             u16,
     pub screen :             u32,
     pub dotclock :           xcb_xf86vidmode_dotclock_t,
     pub hdisplay :           u16,
     pub hsyncstart :         u16,
     pub hsyncend :           u16,
     pub htotal :             u16,
     pub hskew :              u16,
     pub vdisplay :           u16,
     pub vsyncstart :         u16,
     pub vsyncend :           u16,
     pub vtotal :             u16,
     pub pad0 :               [u8; 2],
     pub flags :              u32,
     pub pad1 :               [u8; 12],
     pub privsize :           u32,
     pub after_dotclock :     xcb_xf86vidmode_dotclock_t,
     pub after_hdisplay :     u16,
     pub after_hsyncstart :   u16,
     pub after_hsyncend :     u16,
     pub after_htotal :       u16,
     pub after_hskew :        u16,
     pub after_vdisplay :     u16,
     pub after_vsyncstart :   u16,
     pub after_vsyncend :     u16,
     pub after_vtotal :       u16,
     pub pad2 :               [u8; 2],
     pub after_flags :        u32,
     pub pad3 :               [u8; 12]
}

impl Copy for xcb_xf86vidmode_add_mode_line_request_t {}
impl Clone for xcb_xf86vidmode_add_mode_line_request_t {
    fn clone(&self) -> xcb_xf86vidmode_add_mode_line_request_t { *self }
}


#[repr(C)]
pub struct xcb_xf86vidmode_delete_mode_line_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub screen :         u32,
     pub dotclock :       xcb_xf86vidmode_dotclock_t,
     pub hdisplay :       u16,
     pub hsyncstart :     u16,
     pub hsyncend :       u16,
     pub htotal :         u16,
     pub hskew :          u16,
     pub vdisplay :       u16,
     pub vsyncstart :     u16,
     pub vsyncend :       u16,
     pub vtotal :         u16,
     pub pad0 :           [u8; 2],
     pub flags :          u32,
     pub pad1 :           [u8; 12],
     pub privsize :       u32
}

impl Copy for xcb_xf86vidmode_delete_mode_line_request_t {}
impl Clone for xcb_xf86vidmode_delete_mode_line_request_t {
    fn clone(&self) -> xcb_xf86vidmode_delete_mode_line_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xf86vidmode_validate_mode_line_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_xf86vidmode_validate_mode_line_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub screen :         u32,
     pub dotclock :       xcb_xf86vidmode_dotclock_t,
     pub hdisplay :       u16,
     pub hsyncstart :     u16,
     pub hsyncend :       u16,
     pub htotal :         u16,
     pub hskew :          u16,
     pub vdisplay :       u16,
     pub vsyncstart :     u16,
     pub vsyncend :       u16,
     pub vtotal :         u16,
     pub pad0 :           [u8; 2],
     pub flags :          u32,
     pub pad1 :           [u8; 12],
     pub privsize :       u32
}

impl Copy for xcb_xf86vidmode_validate_mode_line_request_t {}
impl Clone for xcb_xf86vidmode_validate_mode_line_request_t {
    fn clone(&self) -> xcb_xf86vidmode_validate_mode_line_request_t { *self }
}

#[repr(C)]
pub struct xcb_xf86vidmode_validate_mode_line_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub status :          u32,
     pub pad1 :            [u8; 20]
}

impl Copy for xcb_xf86vidmode_validate_mode_line_reply_t {}
impl Clone for xcb_xf86vidmode_validate_mode_line_reply_t {
    fn clone(&self) -> xcb_xf86vidmode_validate_mode_line_reply_t { *self }
}


#[repr(C)]
pub struct xcb_xf86vidmode_switch_to_mode_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub screen :         u32,
     pub dotclock :       xcb_xf86vidmode_dotclock_t,
     pub hdisplay :       u16,
     pub hsyncstart :     u16,
     pub hsyncend :       u16,
     pub htotal :         u16,
     pub hskew :          u16,
     pub vdisplay :       u16,
     pub vsyncstart :     u16,
     pub vsyncend :       u16,
     pub vtotal :         u16,
     pub pad0 :           [u8; 2],
     pub flags :          u32,
     pub pad1 :           [u8; 12],
     pub privsize :       u32
}

impl Copy for xcb_xf86vidmode_switch_to_mode_request_t {}
impl Clone for xcb_xf86vidmode_switch_to_mode_request_t {
    fn clone(&self) -> xcb_xf86vidmode_switch_to_mode_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xf86vidmode_get_view_port_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_xf86vidmode_get_view_port_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub screen :         u16,
     pub pad0 :           [u8; 2]
}

impl Copy for xcb_xf86vidmode_get_view_port_request_t {}
impl Clone for xcb_xf86vidmode_get_view_port_request_t {
    fn clone(&self) -> xcb_xf86vidmode_get_view_port_request_t { *self }
}

#[repr(C)]
pub struct xcb_xf86vidmode_get_view_port_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub x :               u32,
     pub y :               u32,
     pub pad1 :            [u8; 16]
}

impl Copy for xcb_xf86vidmode_get_view_port_reply_t {}
impl Clone for xcb_xf86vidmode_get_view_port_reply_t {
    fn clone(&self) -> xcb_xf86vidmode_get_view_port_reply_t { *self }
}


#[repr(C)]
pub struct xcb_xf86vidmode_set_view_port_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub screen :         u16,
     pub pad0 :           [u8; 2],
     pub x :              u32,
     pub y :              u32
}

impl Copy for xcb_xf86vidmode_set_view_port_request_t {}
impl Clone for xcb_xf86vidmode_set_view_port_request_t {
    fn clone(&self) -> xcb_xf86vidmode_set_view_port_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xf86vidmode_get_dot_clocks_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_xf86vidmode_get_dot_clocks_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub screen :         u16,
     pub pad0 :           [u8; 2]
}

impl Copy for xcb_xf86vidmode_get_dot_clocks_request_t {}
impl Clone for xcb_xf86vidmode_get_dot_clocks_request_t {
    fn clone(&self) -> xcb_xf86vidmode_get_dot_clocks_request_t { *self }
}

#[repr(C)]
pub struct xcb_xf86vidmode_get_dot_clocks_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub flags :           u32,
     pub clocks :          u32,
     pub maxclocks :       u32,
     pub pad1 :            [u8; 12]
}

impl Copy for xcb_xf86vidmode_get_dot_clocks_reply_t {}
impl Clone for xcb_xf86vidmode_get_dot_clocks_reply_t {
    fn clone(&self) -> xcb_xf86vidmode_get_dot_clocks_reply_t { *self }
}


#[repr(C)]
pub struct xcb_xf86vidmode_set_client_version_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub major :          u16,
     pub minor :          u16
}

impl Copy for xcb_xf86vidmode_set_client_version_request_t {}
impl Clone for xcb_xf86vidmode_set_client_version_request_t {
    fn clone(&self) -> xcb_xf86vidmode_set_client_version_request_t { *self }
}


#[repr(C)]
pub struct xcb_xf86vidmode_set_gamma_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub screen :         u16,
     pub pad0 :           [u8; 2],
     pub red :            u32,
     pub green :          u32,
     pub blue :           u32,
     pub pad1 :           [u8; 12]
}

impl Copy for xcb_xf86vidmode_set_gamma_request_t {}
impl Clone for xcb_xf86vidmode_set_gamma_request_t {
    fn clone(&self) -> xcb_xf86vidmode_set_gamma_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xf86vidmode_get_gamma_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_xf86vidmode_get_gamma_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub screen :         u16,
     pub pad0 :           [u8; 26]
}

impl Copy for xcb_xf86vidmode_get_gamma_request_t {}
impl Clone for xcb_xf86vidmode_get_gamma_request_t {
    fn clone(&self) -> xcb_xf86vidmode_get_gamma_request_t { *self }
}

#[repr(C)]
pub struct xcb_xf86vidmode_get_gamma_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub red :             u32,
     pub green :           u32,
     pub blue :            u32,
     pub pad1 :            [u8; 12]
}

impl Copy for xcb_xf86vidmode_get_gamma_reply_t {}
impl Clone for xcb_xf86vidmode_get_gamma_reply_t {
    fn clone(&self) -> xcb_xf86vidmode_get_gamma_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xf86vidmode_get_gamma_ramp_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_xf86vidmode_get_gamma_ramp_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub screen :         u16,
     pub size :           u16
}

impl Copy for xcb_xf86vidmode_get_gamma_ramp_request_t {}
impl Clone for xcb_xf86vidmode_get_gamma_ramp_request_t {
    fn clone(&self) -> xcb_xf86vidmode_get_gamma_ramp_request_t { *self }
}

#[repr(C)]
pub struct xcb_xf86vidmode_get_gamma_ramp_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub size :            u16,
     pub pad1 :            [u8; 22]
}

impl Copy for xcb_xf86vidmode_get_gamma_ramp_reply_t {}
impl Clone for xcb_xf86vidmode_get_gamma_ramp_reply_t {
    fn clone(&self) -> xcb_xf86vidmode_get_gamma_ramp_reply_t { *self }
}


#[repr(C)]
pub struct xcb_xf86vidmode_set_gamma_ramp_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub screen :         u16,
     pub size :           u16
}

impl Copy for xcb_xf86vidmode_set_gamma_ramp_request_t {}
impl Clone for xcb_xf86vidmode_set_gamma_ramp_request_t {
    fn clone(&self) -> xcb_xf86vidmode_set_gamma_ramp_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xf86vidmode_get_gamma_ramp_size_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_xf86vidmode_get_gamma_ramp_size_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub screen :         u16,
     pub pad0 :           [u8; 2]
}

impl Copy for xcb_xf86vidmode_get_gamma_ramp_size_request_t {}
impl Clone for xcb_xf86vidmode_get_gamma_ramp_size_request_t {
    fn clone(&self) -> xcb_xf86vidmode_get_gamma_ramp_size_request_t { *self }
}

#[repr(C)]
pub struct xcb_xf86vidmode_get_gamma_ramp_size_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub size :            u16,
     pub pad1 :            [u8; 22]
}

impl Copy for xcb_xf86vidmode_get_gamma_ramp_size_reply_t {}
impl Clone for xcb_xf86vidmode_get_gamma_ramp_size_reply_t {
    fn clone(&self) -> xcb_xf86vidmode_get_gamma_ramp_size_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xf86vidmode_get_permissions_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_xf86vidmode_get_permissions_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub screen :         u16,
     pub pad0 :           [u8; 2]
}

impl Copy for xcb_xf86vidmode_get_permissions_request_t {}
impl Clone for xcb_xf86vidmode_get_permissions_request_t {
    fn clone(&self) -> xcb_xf86vidmode_get_permissions_request_t { *self }
}

#[repr(C)]
pub struct xcb_xf86vidmode_get_permissions_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub permissions :     u32,
     pub pad1 :            [u8; 20]
}

impl Copy for xcb_xf86vidmode_get_permissions_reply_t {}
impl Clone for xcb_xf86vidmode_get_permissions_reply_t {
    fn clone(&self) -> xcb_xf86vidmode_get_permissions_reply_t { *self }
}


#[repr(C)]
pub struct xcb_xf86vidmode_bad_clock_error_t {
     pub response_type :   u8,
     pub error_code :      u8,
     pub sequence :        u16
}

impl Copy for xcb_xf86vidmode_bad_clock_error_t {}
impl Clone for xcb_xf86vidmode_bad_clock_error_t {
    fn clone(&self) -> xcb_xf86vidmode_bad_clock_error_t { *self }
}


#[repr(C)]
pub struct xcb_xf86vidmode_bad_h_timings_error_t {
     pub response_type :   u8,
     pub error_code :      u8,
     pub sequence :        u16
}

impl Copy for xcb_xf86vidmode_bad_h_timings_error_t {}
impl Clone for xcb_xf86vidmode_bad_h_timings_error_t {
    fn clone(&self) -> xcb_xf86vidmode_bad_h_timings_error_t { *self }
}


#[repr(C)]
pub struct xcb_xf86vidmode_bad_v_timings_error_t {
     pub response_type :   u8,
     pub error_code :      u8,
     pub sequence :        u16
}

impl Copy for xcb_xf86vidmode_bad_v_timings_error_t {}
impl Clone for xcb_xf86vidmode_bad_v_timings_error_t {
    fn clone(&self) -> xcb_xf86vidmode_bad_v_timings_error_t { *self }
}


#[repr(C)]
pub struct xcb_xf86vidmode_mode_unsuitable_error_t {
     pub response_type :   u8,
     pub error_code :      u8,
     pub sequence :        u16
}

impl Copy for xcb_xf86vidmode_mode_unsuitable_error_t {}
impl Clone for xcb_xf86vidmode_mode_unsuitable_error_t {
    fn clone(&self) -> xcb_xf86vidmode_mode_unsuitable_error_t { *self }
}


#[repr(C)]
pub struct xcb_xf86vidmode_extension_disabled_error_t {
     pub response_type :   u8,
     pub error_code :      u8,
     pub sequence :        u16
}

impl Copy for xcb_xf86vidmode_extension_disabled_error_t {}
impl Clone for xcb_xf86vidmode_extension_disabled_error_t {
    fn clone(&self) -> xcb_xf86vidmode_extension_disabled_error_t { *self }
}


#[repr(C)]
pub struct xcb_xf86vidmode_client_not_local_error_t {
     pub response_type :   u8,
     pub error_code :      u8,
     pub sequence :        u16
}

impl Copy for xcb_xf86vidmode_client_not_local_error_t {}
impl Clone for xcb_xf86vidmode_client_not_local_error_t {
    fn clone(&self) -> xcb_xf86vidmode_client_not_local_error_t { *self }
}


#[repr(C)]
pub struct xcb_xf86vidmode_zoom_locked_error_t {
     pub response_type :   u8,
     pub error_code :      u8,
     pub sequence :        u16
}

impl Copy for xcb_xf86vidmode_zoom_locked_error_t {}
impl Clone for xcb_xf86vidmode_zoom_locked_error_t {
    fn clone(&self) -> xcb_xf86vidmode_zoom_locked_error_t { *self }
}
extern "C" {

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_xf86vidmode_syncrange_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_xf86vidmode_syncrange_t)
 *
 *
 */
pub fn xcb_xf86vidmode_syncrange_next (i:*mut xcb_xf86vidmode_syncrange_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_xf86vidmode_syncrange_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_xf86vidmode_syncrange_end (i:xcb_xf86vidmode_syncrange_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_xf86vidmode_dotclock_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_xf86vidmode_dotclock_t)
 *
 *
 */
pub fn xcb_xf86vidmode_dotclock_next (i:*mut xcb_xf86vidmode_dotclock_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_xf86vidmode_dotclock_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_xf86vidmode_dotclock_end (i:xcb_xf86vidmode_dotclock_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_xf86vidmode_mode_info_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_xf86vidmode_mode_info_t)
 *
 *
 */
pub fn xcb_xf86vidmode_mode_info_next (i:*mut xcb_xf86vidmode_mode_info_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_xf86vidmode_mode_info_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_xf86vidmode_mode_info_end (i:xcb_xf86vidmode_mode_info_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_xf86vidmode_query_version (c : *mut ffi::base::xcb_connection_t) -> xcb_xf86vidmode_query_version_cookie_t;

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
pub fn xcb_xf86vidmode_query_version_unchecked (c : *mut ffi::base::xcb_connection_t) -> xcb_xf86vidmode_query_version_cookie_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xf86vidmode_query_version_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_xf86vidmode_query_version_reply (c : *mut ffi::base::xcb_connection_t,
                                               cookie : xcb_xf86vidmode_query_version_cookie_t,
                                               e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_xf86vidmode_query_version_reply_t;

pub fn xcb_xf86vidmode_get_mode_line_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_xf86vidmode_get_mode_line (c : *mut ffi::base::xcb_connection_t,
                                         screen :  u16) -> xcb_xf86vidmode_get_mode_line_cookie_t;

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
pub fn xcb_xf86vidmode_get_mode_line_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                   screen :  u16) -> xcb_xf86vidmode_get_mode_line_cookie_t;

pub fn xcb_xf86vidmode_get_mode_line_private (R : *mut xcb_xf86vidmode_get_mode_line_reply_t) -> *mut u8;


pub fn xcb_xf86vidmode_get_mode_line_private_length (R : *mut xcb_xf86vidmode_get_mode_line_reply_t) -> c_int;


pub fn xcb_xf86vidmode_get_mode_line_private_end (R : *mut xcb_xf86vidmode_get_mode_line_reply_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xf86vidmode_get_mode_line_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_xf86vidmode_get_mode_line_reply (c : *mut ffi::base::xcb_connection_t,
                                               cookie : xcb_xf86vidmode_get_mode_line_cookie_t,
                                               e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_xf86vidmode_get_mode_line_reply_t;

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
pub fn xcb_xf86vidmode_mod_mode_line_checked (c : *mut ffi::base::xcb_connection_t,
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
                                                 private : *mut u8) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_xf86vidmode_mod_mode_line (c : *mut ffi::base::xcb_connection_t,
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
                                         private : *mut u8) -> ffi::base::xcb_void_cookie_t;

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
pub fn xcb_xf86vidmode_switch_mode_checked (c : *mut ffi::base::xcb_connection_t,
                                               screen :  u16,
                                               zoom :  u16) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_xf86vidmode_switch_mode (c : *mut ffi::base::xcb_connection_t,
                                       screen :  u16,
                                       zoom :  u16) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_xf86vidmode_get_monitor_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_xf86vidmode_get_monitor (c : *mut ffi::base::xcb_connection_t,
                                       screen :  u16) -> xcb_xf86vidmode_get_monitor_cookie_t;

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
pub fn xcb_xf86vidmode_get_monitor_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                 screen :  u16) -> xcb_xf86vidmode_get_monitor_cookie_t;

pub fn xcb_xf86vidmode_get_monitor_hsync (R : *mut xcb_xf86vidmode_get_monitor_reply_t) -> *mut xcb_xf86vidmode_syncrange_t;


pub fn xcb_xf86vidmode_get_monitor_hsync_length (R : *mut xcb_xf86vidmode_get_monitor_reply_t) -> c_int;


pub fn xcb_xf86vidmode_get_monitor_hsync_end (R : *mut xcb_xf86vidmode_get_monitor_reply_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_xf86vidmode_get_monitor_vsync (R : *mut xcb_xf86vidmode_get_monitor_reply_t) -> *mut xcb_xf86vidmode_syncrange_t;


pub fn xcb_xf86vidmode_get_monitor_vsync_length (R : *mut xcb_xf86vidmode_get_monitor_reply_t) -> c_int;


pub fn xcb_xf86vidmode_get_monitor_vsync_end (R : *mut xcb_xf86vidmode_get_monitor_reply_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_xf86vidmode_get_monitor_vendor (R : *mut xcb_xf86vidmode_get_monitor_reply_t) -> *mut c_char;


pub fn xcb_xf86vidmode_get_monitor_vendor_length (R : *mut xcb_xf86vidmode_get_monitor_reply_t) -> c_int;


pub fn xcb_xf86vidmode_get_monitor_vendor_end (R : *mut xcb_xf86vidmode_get_monitor_reply_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_xf86vidmode_get_monitor_alignment_pad (R : *mut xcb_xf86vidmode_get_monitor_reply_t) -> *mut c_void;


pub fn xcb_xf86vidmode_get_monitor_alignment_pad_length (R : *mut xcb_xf86vidmode_get_monitor_reply_t) -> c_int;


pub fn xcb_xf86vidmode_get_monitor_alignment_pad_end (R : *mut xcb_xf86vidmode_get_monitor_reply_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_xf86vidmode_get_monitor_model (R : *mut xcb_xf86vidmode_get_monitor_reply_t) -> *mut c_char;


pub fn xcb_xf86vidmode_get_monitor_model_length (R : *mut xcb_xf86vidmode_get_monitor_reply_t) -> c_int;


pub fn xcb_xf86vidmode_get_monitor_model_end (R : *mut xcb_xf86vidmode_get_monitor_reply_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xf86vidmode_get_monitor_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_xf86vidmode_get_monitor_reply (c : *mut ffi::base::xcb_connection_t,
                                             cookie : xcb_xf86vidmode_get_monitor_cookie_t,
                                             e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_xf86vidmode_get_monitor_reply_t;

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
pub fn xcb_xf86vidmode_lock_mode_switch_checked (c : *mut ffi::base::xcb_connection_t,
                                                    screen :  u16,
                                                    lock :  u16) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_xf86vidmode_lock_mode_switch (c : *mut ffi::base::xcb_connection_t,
                                            screen :  u16,
                                            lock :  u16) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_xf86vidmode_get_all_mode_lines_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_xf86vidmode_get_all_mode_lines (c : *mut ffi::base::xcb_connection_t,
                                              screen :  u16) -> xcb_xf86vidmode_get_all_mode_lines_cookie_t;

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
pub fn xcb_xf86vidmode_get_all_mode_lines_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                        screen :  u16) -> xcb_xf86vidmode_get_all_mode_lines_cookie_t;

pub fn xcb_xf86vidmode_get_all_mode_lines_modeinfo (R : *mut xcb_xf86vidmode_get_all_mode_lines_reply_t) -> *mut xcb_xf86vidmode_mode_info_t;


pub fn xcb_xf86vidmode_get_all_mode_lines_modeinfo_length (R : *mut xcb_xf86vidmode_get_all_mode_lines_reply_t) -> c_int;

pub fn xcb_xf86vidmode_get_all_mode_lines_modeinfo_iterator (R : *mut xcb_xf86vidmode_get_all_mode_lines_reply_t) -> xcb_xf86vidmode_mode_info_iterator_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xf86vidmode_get_all_mode_lines_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_xf86vidmode_get_all_mode_lines_reply (c : *mut ffi::base::xcb_connection_t,
                                                    cookie : xcb_xf86vidmode_get_all_mode_lines_cookie_t,
                                                    e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_xf86vidmode_get_all_mode_lines_reply_t;

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
pub fn xcb_xf86vidmode_add_mode_line_checked (c : *mut ffi::base::xcb_connection_t,
                                                 screen :  u32,
                                                 dotclock :  xcb_xf86vidmode_dotclock_t,
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
                                                 after_dotclock :  xcb_xf86vidmode_dotclock_t,
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
                                                 private : *mut u8) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_xf86vidmode_add_mode_line (c : *mut ffi::base::xcb_connection_t,
                                         screen :  u32,
                                         dotclock :  xcb_xf86vidmode_dotclock_t,
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
                                         after_dotclock :  xcb_xf86vidmode_dotclock_t,
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
                                         private : *mut u8) -> ffi::base::xcb_void_cookie_t;

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
pub fn xcb_xf86vidmode_delete_mode_line_checked (c : *mut ffi::base::xcb_connection_t,
                                                    screen :  u32,
                                                    dotclock :  xcb_xf86vidmode_dotclock_t,
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
                                                    private : *mut u8) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_xf86vidmode_delete_mode_line (c : *mut ffi::base::xcb_connection_t,
                                            screen :  u32,
                                            dotclock :  xcb_xf86vidmode_dotclock_t,
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
                                            private : *mut u8) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_xf86vidmode_validate_mode_line_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_xf86vidmode_validate_mode_line (c : *mut ffi::base::xcb_connection_t,
                                              screen :  u32,
                                              dotclock :  xcb_xf86vidmode_dotclock_t,
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
                                              private : *mut u8) -> xcb_xf86vidmode_validate_mode_line_cookie_t;

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
pub fn xcb_xf86vidmode_validate_mode_line_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                        screen :  u32,
                                                        dotclock :  xcb_xf86vidmode_dotclock_t,
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
                                                        private : *mut u8) -> xcb_xf86vidmode_validate_mode_line_cookie_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xf86vidmode_validate_mode_line_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_xf86vidmode_validate_mode_line_reply (c : *mut ffi::base::xcb_connection_t,
                                                    cookie : xcb_xf86vidmode_validate_mode_line_cookie_t,
                                                    e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_xf86vidmode_validate_mode_line_reply_t;

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
pub fn xcb_xf86vidmode_switch_to_mode_checked (c : *mut ffi::base::xcb_connection_t,
                                                  screen :  u32,
                                                  dotclock :  xcb_xf86vidmode_dotclock_t,
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
                                                  private : *mut u8) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_xf86vidmode_switch_to_mode (c : *mut ffi::base::xcb_connection_t,
                                          screen :  u32,
                                          dotclock :  xcb_xf86vidmode_dotclock_t,
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
                                          private : *mut u8) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_xf86vidmode_get_view_port (c : *mut ffi::base::xcb_connection_t,
                                         screen :  u16) -> xcb_xf86vidmode_get_view_port_cookie_t;

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
pub fn xcb_xf86vidmode_get_view_port_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                   screen :  u16) -> xcb_xf86vidmode_get_view_port_cookie_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xf86vidmode_get_view_port_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_xf86vidmode_get_view_port_reply (c : *mut ffi::base::xcb_connection_t,
                                               cookie : xcb_xf86vidmode_get_view_port_cookie_t,
                                               e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_xf86vidmode_get_view_port_reply_t;

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
pub fn xcb_xf86vidmode_set_view_port_checked (c : *mut ffi::base::xcb_connection_t,
                                                 screen :  u16,
                                                 x :  u32,
                                                 y :  u32) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_xf86vidmode_set_view_port (c : *mut ffi::base::xcb_connection_t,
                                         screen :  u16,
                                         x :  u32,
                                         y :  u32) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_xf86vidmode_get_dot_clocks_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_xf86vidmode_get_dot_clocks (c : *mut ffi::base::xcb_connection_t,
                                          screen :  u16) -> xcb_xf86vidmode_get_dot_clocks_cookie_t;

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
pub fn xcb_xf86vidmode_get_dot_clocks_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                    screen :  u16) -> xcb_xf86vidmode_get_dot_clocks_cookie_t;

pub fn xcb_xf86vidmode_get_dot_clocks_clock (R : *mut xcb_xf86vidmode_get_dot_clocks_reply_t) -> *mut u32;


pub fn xcb_xf86vidmode_get_dot_clocks_clock_length (R : *mut xcb_xf86vidmode_get_dot_clocks_reply_t) -> c_int;


pub fn xcb_xf86vidmode_get_dot_clocks_clock_end (R : *mut xcb_xf86vidmode_get_dot_clocks_reply_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xf86vidmode_get_dot_clocks_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_xf86vidmode_get_dot_clocks_reply (c : *mut ffi::base::xcb_connection_t,
                                                cookie : xcb_xf86vidmode_get_dot_clocks_cookie_t,
                                                e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_xf86vidmode_get_dot_clocks_reply_t;

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
pub fn xcb_xf86vidmode_set_client_version_checked (c : *mut ffi::base::xcb_connection_t,
                                                      major :  u16,
                                                      minor :  u16) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_xf86vidmode_set_client_version (c : *mut ffi::base::xcb_connection_t,
                                              major :  u16,
                                              minor :  u16) -> ffi::base::xcb_void_cookie_t;

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
pub fn xcb_xf86vidmode_set_gamma_checked (c : *mut ffi::base::xcb_connection_t,
                                             screen :  u16,
                                             red :  u32,
                                             green :  u32,
                                             blue :  u32) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_xf86vidmode_set_gamma (c : *mut ffi::base::xcb_connection_t,
                                     screen :  u16,
                                     red :  u32,
                                     green :  u32,
                                     blue :  u32) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_xf86vidmode_get_gamma (c : *mut ffi::base::xcb_connection_t,
                                     screen :  u16) -> xcb_xf86vidmode_get_gamma_cookie_t;

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
pub fn xcb_xf86vidmode_get_gamma_unchecked (c : *mut ffi::base::xcb_connection_t,
                                               screen :  u16) -> xcb_xf86vidmode_get_gamma_cookie_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xf86vidmode_get_gamma_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_xf86vidmode_get_gamma_reply (c : *mut ffi::base::xcb_connection_t,
                                           cookie : xcb_xf86vidmode_get_gamma_cookie_t,
                                           e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_xf86vidmode_get_gamma_reply_t;

pub fn xcb_xf86vidmode_get_gamma_ramp_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_xf86vidmode_get_gamma_ramp (c : *mut ffi::base::xcb_connection_t,
                                          screen :  u16,
                                          size :  u16) -> xcb_xf86vidmode_get_gamma_ramp_cookie_t;

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
pub fn xcb_xf86vidmode_get_gamma_ramp_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                    screen :  u16,
                                                    size :  u16) -> xcb_xf86vidmode_get_gamma_ramp_cookie_t;

pub fn xcb_xf86vidmode_get_gamma_ramp_red (R : *mut xcb_xf86vidmode_get_gamma_ramp_reply_t) -> *mut u16;


pub fn xcb_xf86vidmode_get_gamma_ramp_red_length (R : *mut xcb_xf86vidmode_get_gamma_ramp_reply_t) -> c_int;


pub fn xcb_xf86vidmode_get_gamma_ramp_red_end (R : *mut xcb_xf86vidmode_get_gamma_ramp_reply_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_xf86vidmode_get_gamma_ramp_green (R : *mut xcb_xf86vidmode_get_gamma_ramp_reply_t) -> *mut u16;


pub fn xcb_xf86vidmode_get_gamma_ramp_green_length (R : *mut xcb_xf86vidmode_get_gamma_ramp_reply_t) -> c_int;


pub fn xcb_xf86vidmode_get_gamma_ramp_green_end (R : *mut xcb_xf86vidmode_get_gamma_ramp_reply_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_xf86vidmode_get_gamma_ramp_blue (R : *mut xcb_xf86vidmode_get_gamma_ramp_reply_t) -> *mut u16;


pub fn xcb_xf86vidmode_get_gamma_ramp_blue_length (R : *mut xcb_xf86vidmode_get_gamma_ramp_reply_t) -> c_int;


pub fn xcb_xf86vidmode_get_gamma_ramp_blue_end (R : *mut xcb_xf86vidmode_get_gamma_ramp_reply_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xf86vidmode_get_gamma_ramp_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_xf86vidmode_get_gamma_ramp_reply (c : *mut ffi::base::xcb_connection_t,
                                                cookie : xcb_xf86vidmode_get_gamma_ramp_cookie_t,
                                                e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_xf86vidmode_get_gamma_ramp_reply_t;

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
pub fn xcb_xf86vidmode_set_gamma_ramp_checked (c : *mut ffi::base::xcb_connection_t,
                                                  screen :  u16,
                                                  size :  u16,
                                                  red : *mut u16,
                                                  green : *mut u16,
                                                  blue : *mut u16) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_xf86vidmode_set_gamma_ramp (c : *mut ffi::base::xcb_connection_t,
                                          screen :  u16,
                                          size :  u16,
                                          red : *mut u16,
                                          green : *mut u16,
                                          blue : *mut u16) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_xf86vidmode_get_gamma_ramp_size (c : *mut ffi::base::xcb_connection_t,
                                               screen :  u16) -> xcb_xf86vidmode_get_gamma_ramp_size_cookie_t;

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
pub fn xcb_xf86vidmode_get_gamma_ramp_size_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                         screen :  u16) -> xcb_xf86vidmode_get_gamma_ramp_size_cookie_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xf86vidmode_get_gamma_ramp_size_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_xf86vidmode_get_gamma_ramp_size_reply (c : *mut ffi::base::xcb_connection_t,
                                                     cookie : xcb_xf86vidmode_get_gamma_ramp_size_cookie_t,
                                                     e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_xf86vidmode_get_gamma_ramp_size_reply_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_xf86vidmode_get_permissions (c : *mut ffi::base::xcb_connection_t,
                                           screen :  u16) -> xcb_xf86vidmode_get_permissions_cookie_t;

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
pub fn xcb_xf86vidmode_get_permissions_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                     screen :  u16) -> xcb_xf86vidmode_get_permissions_cookie_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xf86vidmode_get_permissions_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_xf86vidmode_get_permissions_reply (c : *mut ffi::base::xcb_connection_t,
                                                 cookie : xcb_xf86vidmode_get_permissions_cookie_t,
                                                 e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_xf86vidmode_get_permissions_reply_t;
}

