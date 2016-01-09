/*
 * This file generated automatically from randr.xml by r_client.py.
 * Edit at your peril.
 */

//Make the compiler quiet
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use std;
use libc::*;
use ffi;
use ffi::xproto;
use ffi::render;

pub const RANDR_MAJOR_VERSION : c_uint = 1;
pub const RANDR_MINOR_VERSION : c_uint = 3;

pub type xcb_randr_mode_t = u32;
/**
 * @brief xcb_randr_mode_iterator_t
 **/
#[repr(C)]
pub struct xcb_randr_mode_iterator_t {
    pub data : *mut xcb_randr_mode_t,
    pub rem  : c_int,
    pub index: c_int
}


pub type xcb_randr_crtc_t = u32;
/**
 * @brief xcb_randr_crtc_iterator_t
 **/
#[repr(C)]
pub struct xcb_randr_crtc_iterator_t {
    pub data : *mut xcb_randr_crtc_t,
    pub rem  : c_int,
    pub index: c_int
}


pub type xcb_randr_output_t = u32;
/**
 * @brief xcb_randr_output_iterator_t
 **/
#[repr(C)]
pub struct xcb_randr_output_iterator_t {
    pub data : *mut xcb_randr_output_t,
    pub rem  : c_int,
    pub index: c_int
}



#[repr(C)]
pub struct xcb_randr_bad_output_error_t {
     pub response_type :   u8,
     pub error_code :      u8,
     pub sequence :        u16
}

impl Copy for xcb_randr_bad_output_error_t {}
impl Clone for xcb_randr_bad_output_error_t {
    fn clone(&self) -> xcb_randr_bad_output_error_t { *self }
}


#[repr(C)]
pub struct xcb_randr_bad_crtc_error_t {
     pub response_type :   u8,
     pub error_code :      u8,
     pub sequence :        u16
}

impl Copy for xcb_randr_bad_crtc_error_t {}
impl Clone for xcb_randr_bad_crtc_error_t {
    fn clone(&self) -> xcb_randr_bad_crtc_error_t { *self }
}


#[repr(C)]
pub struct xcb_randr_bad_mode_error_t {
     pub response_type :   u8,
     pub error_code :      u8,
     pub sequence :        u16
}

impl Copy for xcb_randr_bad_mode_error_t {}
impl Clone for xcb_randr_bad_mode_error_t {
    fn clone(&self) -> xcb_randr_bad_mode_error_t { *self }
}

#[repr(C)]
pub struct xcb_randr_screen_size_t {
     pub width :     u16,
     pub height :    u16,
     pub mwidth :    u16,
     pub mheight :   u16
}

impl Copy for xcb_randr_screen_size_t {}
impl Clone for xcb_randr_screen_size_t {
    fn clone(&self) -> xcb_randr_screen_size_t { *self }
}
/**
 * @brief xcb_randr_screen_size_iterator_t
 **/
#[repr(C)]
pub struct xcb_randr_screen_size_iterator_t {
    pub data : *mut xcb_randr_screen_size_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_randr_refresh_rates_t {
     pub nRates :   u16
}

impl Copy for xcb_randr_refresh_rates_t {}
impl Clone for xcb_randr_refresh_rates_t {
    fn clone(&self) -> xcb_randr_refresh_rates_t { *self }
}
/**
 * @brief xcb_randr_refresh_rates_iterator_t
 **/
#[repr(C)]
pub struct xcb_randr_refresh_rates_iterator_t {
    pub data : *mut xcb_randr_refresh_rates_t,
    pub rem  : c_int,
    pub index: c_int
}


#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_randr_query_version_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_randr_query_version_request_t {
     pub major_opcode :    u8,
     pub minor_opcode :    u8,
     pub length :          u16,
     pub major_version :   u32,
     pub minor_version :   u32
}

impl Copy for xcb_randr_query_version_request_t {}
impl Clone for xcb_randr_query_version_request_t {
    fn clone(&self) -> xcb_randr_query_version_request_t { *self }
}

#[repr(C)]
pub struct xcb_randr_query_version_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub major_version :   u32,
     pub minor_version :   u32,
     pub pad1 :            [u8; 16]
}

impl Copy for xcb_randr_query_version_reply_t {}
impl Clone for xcb_randr_query_version_reply_t {
    fn clone(&self) -> xcb_randr_query_version_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_randr_set_screen_config_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_randr_set_screen_config_request_t {
     pub major_opcode :       u8,
     pub minor_opcode :       u8,
     pub length :             u16,
     pub window :             ffi::xproto::xcb_window_t,
     pub timestamp :          ffi::xproto::xcb_timestamp_t,
     pub config_timestamp :   ffi::xproto::xcb_timestamp_t,
     pub sizeID :             u16,
     pub rotation :           u16,
     pub rate :               u16,
     pub pad0 :               [u8; 2]
}

impl Copy for xcb_randr_set_screen_config_request_t {}
impl Clone for xcb_randr_set_screen_config_request_t {
    fn clone(&self) -> xcb_randr_set_screen_config_request_t { *self }
}

#[repr(C)]
pub struct xcb_randr_set_screen_config_reply_t {
     pub response_type :      u8,
     pub status :             u8,
     pub sequence :           u16,
     pub length :             u32,
     pub new_timestamp :      ffi::xproto::xcb_timestamp_t,
     pub config_timestamp :   ffi::xproto::xcb_timestamp_t,
     pub root :               ffi::xproto::xcb_window_t,
     pub subpixel_order :     u16,
     pub pad0 :               [u8; 10]
}

impl Copy for xcb_randr_set_screen_config_reply_t {}
impl Clone for xcb_randr_set_screen_config_reply_t {
    fn clone(&self) -> xcb_randr_set_screen_config_reply_t { *self }
}


#[repr(C)]
pub struct xcb_randr_select_input_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub window :         ffi::xproto::xcb_window_t,
     pub enable :         u16,
     pub pad0 :           [u8; 2]
}

impl Copy for xcb_randr_select_input_request_t {}
impl Clone for xcb_randr_select_input_request_t {
    fn clone(&self) -> xcb_randr_select_input_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_randr_get_screen_info_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_randr_get_screen_info_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub window :         ffi::xproto::xcb_window_t
}

impl Copy for xcb_randr_get_screen_info_request_t {}
impl Clone for xcb_randr_get_screen_info_request_t {
    fn clone(&self) -> xcb_randr_get_screen_info_request_t { *self }
}

#[repr(C)]
pub struct xcb_randr_get_screen_info_reply_t {
     pub response_type :      u8,
     pub rotations :          u8,
     pub sequence :           u16,
     pub length :             u32,
     pub root :               ffi::xproto::xcb_window_t,
     pub timestamp :          ffi::xproto::xcb_timestamp_t,
     pub config_timestamp :   ffi::xproto::xcb_timestamp_t,
     pub nSizes :             u16,
     pub sizeID :             u16,
     pub rotation :           u16,
     pub rate :               u16,
     pub nInfo :              u16,
     pub pad0 :               [u8; 2]
}

impl Copy for xcb_randr_get_screen_info_reply_t {}
impl Clone for xcb_randr_get_screen_info_reply_t {
    fn clone(&self) -> xcb_randr_get_screen_info_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_randr_get_screen_size_range_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_randr_get_screen_size_range_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub window :         ffi::xproto::xcb_window_t
}

impl Copy for xcb_randr_get_screen_size_range_request_t {}
impl Clone for xcb_randr_get_screen_size_range_request_t {
    fn clone(&self) -> xcb_randr_get_screen_size_range_request_t { *self }
}

#[repr(C)]
pub struct xcb_randr_get_screen_size_range_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub min_width :       u16,
     pub min_height :      u16,
     pub max_width :       u16,
     pub max_height :      u16,
     pub pad1 :            [u8; 16]
}

impl Copy for xcb_randr_get_screen_size_range_reply_t {}
impl Clone for xcb_randr_get_screen_size_range_reply_t {
    fn clone(&self) -> xcb_randr_get_screen_size_range_reply_t { *self }
}


#[repr(C)]
pub struct xcb_randr_set_screen_size_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub window :         ffi::xproto::xcb_window_t,
     pub width :          u16,
     pub height :         u16,
     pub mm_width :       u32,
     pub mm_height :      u32
}

impl Copy for xcb_randr_set_screen_size_request_t {}
impl Clone for xcb_randr_set_screen_size_request_t {
    fn clone(&self) -> xcb_randr_set_screen_size_request_t { *self }
}

#[repr(C)]
pub struct xcb_randr_mode_info_t {
     pub id :            u32,
     pub width :         u16,
     pub height :        u16,
     pub dot_clock :     u32,
     pub hsync_start :   u16,
     pub hsync_end :     u16,
     pub htotal :        u16,
     pub hskew :         u16,
     pub vsync_start :   u16,
     pub vsync_end :     u16,
     pub vtotal :        u16,
     pub name_len :      u16,
     pub mode_flags :    u32
}

impl Copy for xcb_randr_mode_info_t {}
impl Clone for xcb_randr_mode_info_t {
    fn clone(&self) -> xcb_randr_mode_info_t { *self }
}
/**
 * @brief xcb_randr_mode_info_iterator_t
 **/
#[repr(C)]
pub struct xcb_randr_mode_info_iterator_t {
    pub data : *mut xcb_randr_mode_info_t,
    pub rem  : c_int,
    pub index: c_int
}


#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_randr_get_screen_resources_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_randr_get_screen_resources_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub window :         ffi::xproto::xcb_window_t
}

impl Copy for xcb_randr_get_screen_resources_request_t {}
impl Clone for xcb_randr_get_screen_resources_request_t {
    fn clone(&self) -> xcb_randr_get_screen_resources_request_t { *self }
}

#[repr(C)]
pub struct xcb_randr_get_screen_resources_reply_t {
     pub response_type :      u8,
     pub pad0 :               u8,
     pub sequence :           u16,
     pub length :             u32,
     pub timestamp :          ffi::xproto::xcb_timestamp_t,
     pub config_timestamp :   ffi::xproto::xcb_timestamp_t,
     pub num_crtcs :          u16,
     pub num_outputs :        u16,
     pub num_modes :          u16,
     pub names_len :          u16,
     pub pad1 :               [u8; 8]
}

impl Copy for xcb_randr_get_screen_resources_reply_t {}
impl Clone for xcb_randr_get_screen_resources_reply_t {
    fn clone(&self) -> xcb_randr_get_screen_resources_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_randr_get_output_info_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_randr_get_output_info_request_t {
     pub major_opcode :       u8,
     pub minor_opcode :       u8,
     pub length :             u16,
     pub output :             xcb_randr_output_t,
     pub config_timestamp :   ffi::xproto::xcb_timestamp_t
}

impl Copy for xcb_randr_get_output_info_request_t {}
impl Clone for xcb_randr_get_output_info_request_t {
    fn clone(&self) -> xcb_randr_get_output_info_request_t { *self }
}

#[repr(C)]
pub struct xcb_randr_get_output_info_reply_t {
     pub response_type :    u8,
     pub status :           u8,
     pub sequence :         u16,
     pub length :           u32,
     pub timestamp :        ffi::xproto::xcb_timestamp_t,
     pub crtc :             xcb_randr_crtc_t,
     pub mm_width :         u32,
     pub mm_height :        u32,
     pub connection :       u8,
     pub subpixel_order :   u8,
     pub num_crtcs :        u16,
     pub num_modes :        u16,
     pub num_preferred :    u16,
     pub num_clones :       u16,
     pub name_len :         u16
}

impl Copy for xcb_randr_get_output_info_reply_t {}
impl Clone for xcb_randr_get_output_info_reply_t {
    fn clone(&self) -> xcb_randr_get_output_info_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_randr_list_output_properties_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_randr_list_output_properties_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub output :         xcb_randr_output_t
}

impl Copy for xcb_randr_list_output_properties_request_t {}
impl Clone for xcb_randr_list_output_properties_request_t {
    fn clone(&self) -> xcb_randr_list_output_properties_request_t { *self }
}

#[repr(C)]
pub struct xcb_randr_list_output_properties_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub num_atoms :       u16,
     pub pad1 :            [u8; 22]
}

impl Copy for xcb_randr_list_output_properties_reply_t {}
impl Clone for xcb_randr_list_output_properties_reply_t {
    fn clone(&self) -> xcb_randr_list_output_properties_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_randr_query_output_property_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_randr_query_output_property_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub output :         xcb_randr_output_t,
     pub property :       ffi::xproto::xcb_atom_t
}

impl Copy for xcb_randr_query_output_property_request_t {}
impl Clone for xcb_randr_query_output_property_request_t {
    fn clone(&self) -> xcb_randr_query_output_property_request_t { *self }
}

#[repr(C)]
pub struct xcb_randr_query_output_property_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub pending :         u8,
     pub range :           u8,
     pub immutable :       u8,
     pub pad1 :            [u8; 21]
}

impl Copy for xcb_randr_query_output_property_reply_t {}
impl Clone for xcb_randr_query_output_property_reply_t {
    fn clone(&self) -> xcb_randr_query_output_property_reply_t { *self }
}


#[repr(C)]
pub struct xcb_randr_configure_output_property_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub output :         xcb_randr_output_t,
     pub property :       ffi::xproto::xcb_atom_t,
     pub pending :        u8,
     pub range :          u8,
     pub pad0 :           [u8; 2]
}

impl Copy for xcb_randr_configure_output_property_request_t {}
impl Clone for xcb_randr_configure_output_property_request_t {
    fn clone(&self) -> xcb_randr_configure_output_property_request_t { *self }
}


#[repr(C)]
pub struct xcb_randr_change_output_property_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub output :         xcb_randr_output_t,
     pub property :       ffi::xproto::xcb_atom_t,
     pub type_ :          ffi::xproto::xcb_atom_t,
     pub format :         u8,
     pub mode :           u8,
     pub pad0 :           [u8; 2],
     pub num_units :      u32
}

impl Copy for xcb_randr_change_output_property_request_t {}
impl Clone for xcb_randr_change_output_property_request_t {
    fn clone(&self) -> xcb_randr_change_output_property_request_t { *self }
}


#[repr(C)]
pub struct xcb_randr_delete_output_property_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub output :         xcb_randr_output_t,
     pub property :       ffi::xproto::xcb_atom_t
}

impl Copy for xcb_randr_delete_output_property_request_t {}
impl Clone for xcb_randr_delete_output_property_request_t {
    fn clone(&self) -> xcb_randr_delete_output_property_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_randr_get_output_property_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_randr_get_output_property_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub output :         xcb_randr_output_t,
     pub property :       ffi::xproto::xcb_atom_t,
     pub type_ :          ffi::xproto::xcb_atom_t,
     pub long_offset :    u32,
     pub long_length :    u32,
     pub delete :         u8,
     pub pending :        u8,
     pub pad0 :           [u8; 2]
}

impl Copy for xcb_randr_get_output_property_request_t {}
impl Clone for xcb_randr_get_output_property_request_t {
    fn clone(&self) -> xcb_randr_get_output_property_request_t { *self }
}

#[repr(C)]
pub struct xcb_randr_get_output_property_reply_t {
     pub response_type :   u8,
     pub format :          u8,
     pub sequence :        u16,
     pub length :          u32,
     pub type_ :           ffi::xproto::xcb_atom_t,
     pub bytes_after :     u32,
     pub num_items :       u32,
     pub pad0 :            [u8; 12]
}

impl Copy for xcb_randr_get_output_property_reply_t {}
impl Clone for xcb_randr_get_output_property_reply_t {
    fn clone(&self) -> xcb_randr_get_output_property_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_randr_create_mode_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_randr_create_mode_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub window :         ffi::xproto::xcb_window_t,
     pub mode_info :      xcb_randr_mode_info_t
}

impl Copy for xcb_randr_create_mode_request_t {}
impl Clone for xcb_randr_create_mode_request_t {
    fn clone(&self) -> xcb_randr_create_mode_request_t { *self }
}

#[repr(C)]
pub struct xcb_randr_create_mode_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub mode :            xcb_randr_mode_t,
     pub pad1 :            [u8; 20]
}

impl Copy for xcb_randr_create_mode_reply_t {}
impl Clone for xcb_randr_create_mode_reply_t {
    fn clone(&self) -> xcb_randr_create_mode_reply_t { *self }
}


#[repr(C)]
pub struct xcb_randr_destroy_mode_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub mode :           xcb_randr_mode_t
}

impl Copy for xcb_randr_destroy_mode_request_t {}
impl Clone for xcb_randr_destroy_mode_request_t {
    fn clone(&self) -> xcb_randr_destroy_mode_request_t { *self }
}


#[repr(C)]
pub struct xcb_randr_add_output_mode_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub output :         xcb_randr_output_t,
     pub mode :           xcb_randr_mode_t
}

impl Copy for xcb_randr_add_output_mode_request_t {}
impl Clone for xcb_randr_add_output_mode_request_t {
    fn clone(&self) -> xcb_randr_add_output_mode_request_t { *self }
}


#[repr(C)]
pub struct xcb_randr_delete_output_mode_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub output :         xcb_randr_output_t,
     pub mode :           xcb_randr_mode_t
}

impl Copy for xcb_randr_delete_output_mode_request_t {}
impl Clone for xcb_randr_delete_output_mode_request_t {
    fn clone(&self) -> xcb_randr_delete_output_mode_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_randr_get_crtc_info_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_randr_get_crtc_info_request_t {
     pub major_opcode :       u8,
     pub minor_opcode :       u8,
     pub length :             u16,
     pub crtc :               xcb_randr_crtc_t,
     pub config_timestamp :   ffi::xproto::xcb_timestamp_t
}

impl Copy for xcb_randr_get_crtc_info_request_t {}
impl Clone for xcb_randr_get_crtc_info_request_t {
    fn clone(&self) -> xcb_randr_get_crtc_info_request_t { *self }
}

#[repr(C)]
pub struct xcb_randr_get_crtc_info_reply_t {
     pub response_type :          u8,
     pub status :                 u8,
     pub sequence :               u16,
     pub length :                 u32,
     pub timestamp :              ffi::xproto::xcb_timestamp_t,
     pub x :                      i16,
     pub y :                      i16,
     pub width :                  u16,
     pub height :                 u16,
     pub mode :                   xcb_randr_mode_t,
     pub rotation :               u16,
     pub rotations :              u16,
     pub num_outputs :            u16,
     pub num_possible_outputs :   u16
}

impl Copy for xcb_randr_get_crtc_info_reply_t {}
impl Clone for xcb_randr_get_crtc_info_reply_t {
    fn clone(&self) -> xcb_randr_get_crtc_info_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_randr_set_crtc_config_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_randr_set_crtc_config_request_t {
     pub major_opcode :       u8,
     pub minor_opcode :       u8,
     pub length :             u16,
     pub crtc :               xcb_randr_crtc_t,
     pub timestamp :          ffi::xproto::xcb_timestamp_t,
     pub config_timestamp :   ffi::xproto::xcb_timestamp_t,
     pub x :                  i16,
     pub y :                  i16,
     pub mode :               xcb_randr_mode_t,
     pub rotation :           u16,
     pub pad0 :               [u8; 2]
}

impl Copy for xcb_randr_set_crtc_config_request_t {}
impl Clone for xcb_randr_set_crtc_config_request_t {
    fn clone(&self) -> xcb_randr_set_crtc_config_request_t { *self }
}

#[repr(C)]
pub struct xcb_randr_set_crtc_config_reply_t {
     pub response_type :   u8,
     pub status :          u8,
     pub sequence :        u16,
     pub length :          u32,
     pub timestamp :       ffi::xproto::xcb_timestamp_t,
     pub pad0 :            [u8; 20]
}

impl Copy for xcb_randr_set_crtc_config_reply_t {}
impl Clone for xcb_randr_set_crtc_config_reply_t {
    fn clone(&self) -> xcb_randr_set_crtc_config_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_randr_get_crtc_gamma_size_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_randr_get_crtc_gamma_size_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub crtc :           xcb_randr_crtc_t
}

impl Copy for xcb_randr_get_crtc_gamma_size_request_t {}
impl Clone for xcb_randr_get_crtc_gamma_size_request_t {
    fn clone(&self) -> xcb_randr_get_crtc_gamma_size_request_t { *self }
}

#[repr(C)]
pub struct xcb_randr_get_crtc_gamma_size_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub size :            u16,
     pub pad1 :            [u8; 22]
}

impl Copy for xcb_randr_get_crtc_gamma_size_reply_t {}
impl Clone for xcb_randr_get_crtc_gamma_size_reply_t {
    fn clone(&self) -> xcb_randr_get_crtc_gamma_size_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_randr_get_crtc_gamma_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_randr_get_crtc_gamma_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub crtc :           xcb_randr_crtc_t
}

impl Copy for xcb_randr_get_crtc_gamma_request_t {}
impl Clone for xcb_randr_get_crtc_gamma_request_t {
    fn clone(&self) -> xcb_randr_get_crtc_gamma_request_t { *self }
}

#[repr(C)]
pub struct xcb_randr_get_crtc_gamma_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub size :            u16,
     pub pad1 :            [u8; 22]
}

impl Copy for xcb_randr_get_crtc_gamma_reply_t {}
impl Clone for xcb_randr_get_crtc_gamma_reply_t {
    fn clone(&self) -> xcb_randr_get_crtc_gamma_reply_t { *self }
}


#[repr(C)]
pub struct xcb_randr_set_crtc_gamma_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub crtc :           xcb_randr_crtc_t,
     pub size :           u16,
     pub pad0 :           [u8; 2]
}

impl Copy for xcb_randr_set_crtc_gamma_request_t {}
impl Clone for xcb_randr_set_crtc_gamma_request_t {
    fn clone(&self) -> xcb_randr_set_crtc_gamma_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_randr_get_screen_resources_current_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_randr_get_screen_resources_current_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub window :         ffi::xproto::xcb_window_t
}

impl Copy for xcb_randr_get_screen_resources_current_request_t {}
impl Clone for xcb_randr_get_screen_resources_current_request_t {
    fn clone(&self) -> xcb_randr_get_screen_resources_current_request_t { *self }
}

#[repr(C)]
pub struct xcb_randr_get_screen_resources_current_reply_t {
     pub response_type :      u8,
     pub pad0 :               u8,
     pub sequence :           u16,
     pub length :             u32,
     pub timestamp :          ffi::xproto::xcb_timestamp_t,
     pub config_timestamp :   ffi::xproto::xcb_timestamp_t,
     pub num_crtcs :          u16,
     pub num_outputs :        u16,
     pub num_modes :          u16,
     pub names_len :          u16,
     pub pad1 :               [u8; 8]
}

impl Copy for xcb_randr_get_screen_resources_current_reply_t {}
impl Clone for xcb_randr_get_screen_resources_current_reply_t {
    fn clone(&self) -> xcb_randr_get_screen_resources_current_reply_t { *self }
}


#[repr(C)]
pub struct xcb_randr_set_crtc_transform_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub crtc :           xcb_randr_crtc_t,
     pub transform :      ffi::render::xcb_render_transform_t,
     pub filter_len :     u16,
     pub pad0 :           [u8; 2]
}

impl Copy for xcb_randr_set_crtc_transform_request_t {}
impl Clone for xcb_randr_set_crtc_transform_request_t {
    fn clone(&self) -> xcb_randr_set_crtc_transform_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_randr_get_crtc_transform_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_randr_get_crtc_transform_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub crtc :           xcb_randr_crtc_t
}

impl Copy for xcb_randr_get_crtc_transform_request_t {}
impl Clone for xcb_randr_get_crtc_transform_request_t {
    fn clone(&self) -> xcb_randr_get_crtc_transform_request_t { *self }
}

#[repr(C)]
pub struct xcb_randr_get_crtc_transform_reply_t {
     pub response_type :       u8,
     pub pad0 :                u8,
     pub sequence :            u16,
     pub length :              u32,
     pub pending_transform :   ffi::render::xcb_render_transform_t,
     pub has_transforms :      u8,
     pub pad1 :                [u8; 3],
     pub current_transform :   ffi::render::xcb_render_transform_t,
     pub pad2 :                [u8; 4],
     pub pending_len :         u16,
     pub pending_nparams :     u16,
     pub current_len :         u16,
     pub current_nparams :     u16
}

impl Copy for xcb_randr_get_crtc_transform_reply_t {}
impl Clone for xcb_randr_get_crtc_transform_reply_t {
    fn clone(&self) -> xcb_randr_get_crtc_transform_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_randr_get_panning_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_randr_get_panning_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub crtc :           xcb_randr_crtc_t
}

impl Copy for xcb_randr_get_panning_request_t {}
impl Clone for xcb_randr_get_panning_request_t {
    fn clone(&self) -> xcb_randr_get_panning_request_t { *self }
}

#[repr(C)]
pub struct xcb_randr_get_panning_reply_t {
     pub response_type :   u8,
     pub status :          u8,
     pub sequence :        u16,
     pub length :          u32,
     pub timestamp :       ffi::xproto::xcb_timestamp_t,
     pub left :            u16,
     pub top :             u16,
     pub width :           u16,
     pub height :          u16,
     pub track_left :      u16,
     pub track_top :       u16,
     pub track_width :     u16,
     pub track_height :    u16,
     pub border_left :     i16,
     pub border_top :      i16,
     pub border_right :    i16,
     pub border_bottom :   i16
}

impl Copy for xcb_randr_get_panning_reply_t {}
impl Clone for xcb_randr_get_panning_reply_t {
    fn clone(&self) -> xcb_randr_get_panning_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_randr_set_panning_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_randr_set_panning_request_t {
     pub major_opcode :    u8,
     pub minor_opcode :    u8,
     pub length :          u16,
     pub crtc :            xcb_randr_crtc_t,
     pub timestamp :       ffi::xproto::xcb_timestamp_t,
     pub left :            u16,
     pub top :             u16,
     pub width :           u16,
     pub height :          u16,
     pub track_left :      u16,
     pub track_top :       u16,
     pub track_width :     u16,
     pub track_height :    u16,
     pub border_left :     i16,
     pub border_top :      i16,
     pub border_right :    i16,
     pub border_bottom :   i16
}

impl Copy for xcb_randr_set_panning_request_t {}
impl Clone for xcb_randr_set_panning_request_t {
    fn clone(&self) -> xcb_randr_set_panning_request_t { *self }
}

#[repr(C)]
pub struct xcb_randr_set_panning_reply_t {
     pub response_type :   u8,
     pub status :          u8,
     pub sequence :        u16,
     pub length :          u32,
     pub timestamp :       ffi::xproto::xcb_timestamp_t
}

impl Copy for xcb_randr_set_panning_reply_t {}
impl Clone for xcb_randr_set_panning_reply_t {
    fn clone(&self) -> xcb_randr_set_panning_reply_t { *self }
}


#[repr(C)]
pub struct xcb_randr_set_output_primary_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub window :         ffi::xproto::xcb_window_t,
     pub output :         xcb_randr_output_t
}

impl Copy for xcb_randr_set_output_primary_request_t {}
impl Clone for xcb_randr_set_output_primary_request_t {
    fn clone(&self) -> xcb_randr_set_output_primary_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_randr_get_output_primary_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_randr_get_output_primary_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub window :         ffi::xproto::xcb_window_t
}

impl Copy for xcb_randr_get_output_primary_request_t {}
impl Clone for xcb_randr_get_output_primary_request_t {
    fn clone(&self) -> xcb_randr_get_output_primary_request_t { *self }
}

#[repr(C)]
pub struct xcb_randr_get_output_primary_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub output :          xcb_randr_output_t
}

impl Copy for xcb_randr_get_output_primary_reply_t {}
impl Clone for xcb_randr_get_output_primary_reply_t {
    fn clone(&self) -> xcb_randr_get_output_primary_reply_t { *self }
}


#[repr(C)]
pub struct xcb_randr_screen_change_notify_event_t {
     pub response_type :      u8,
     pub rotation :           u8,
     pub sequence :           u16,
     pub timestamp :          ffi::xproto::xcb_timestamp_t,
     pub config_timestamp :   ffi::xproto::xcb_timestamp_t,
     pub root :               ffi::xproto::xcb_window_t,
     pub request_window :     ffi::xproto::xcb_window_t,
     pub sizeID :             u16,
     pub subpixel_order :     u16,
     pub width :              u16,
     pub height :             u16,
     pub mwidth :             u16,
     pub mheight :            u16
}

impl Copy for xcb_randr_screen_change_notify_event_t {}
impl Clone for xcb_randr_screen_change_notify_event_t {
    fn clone(&self) -> xcb_randr_screen_change_notify_event_t { *self }
}

#[repr(C)]
pub struct xcb_randr_crtc_change_t {
     pub timestamp :   ffi::xproto::xcb_timestamp_t,
     pub window :      ffi::xproto::xcb_window_t,
     pub crtc :        xcb_randr_crtc_t,
     pub mode :        xcb_randr_mode_t,
     pub rotation :    u16,
     pub pad0 :        [u8; 2],
     pub x :           i16,
     pub y :           i16,
     pub width :       u16,
     pub height :      u16
}

impl Copy for xcb_randr_crtc_change_t {}
impl Clone for xcb_randr_crtc_change_t {
    fn clone(&self) -> xcb_randr_crtc_change_t { *self }
}
/**
 * @brief xcb_randr_crtc_change_iterator_t
 **/
#[repr(C)]
pub struct xcb_randr_crtc_change_iterator_t {
    pub data : *mut xcb_randr_crtc_change_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_randr_output_change_t {
     pub timestamp :          ffi::xproto::xcb_timestamp_t,
     pub config_timestamp :   ffi::xproto::xcb_timestamp_t,
     pub window :             ffi::xproto::xcb_window_t,
     pub output :             xcb_randr_output_t,
     pub crtc :               xcb_randr_crtc_t,
     pub mode :               xcb_randr_mode_t,
     pub rotation :           u16,
     pub connection :         u8,
     pub subpixel_order :     u8
}

impl Copy for xcb_randr_output_change_t {}
impl Clone for xcb_randr_output_change_t {
    fn clone(&self) -> xcb_randr_output_change_t { *self }
}
/**
 * @brief xcb_randr_output_change_iterator_t
 **/
#[repr(C)]
pub struct xcb_randr_output_change_iterator_t {
    pub data : *mut xcb_randr_output_change_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_randr_output_property_t {
     pub window :      ffi::xproto::xcb_window_t,
     pub output :      xcb_randr_output_t,
     pub atom :        ffi::xproto::xcb_atom_t,
     pub timestamp :   ffi::xproto::xcb_timestamp_t,
     pub status :      u8,
     pub pad0 :        [u8; 11]
}

impl Copy for xcb_randr_output_property_t {}
impl Clone for xcb_randr_output_property_t {
    fn clone(&self) -> xcb_randr_output_property_t { *self }
}
/**
 * @brief xcb_randr_output_property_iterator_t
 **/
#[repr(C)]
pub struct xcb_randr_output_property_iterator_t {
    pub data : *mut xcb_randr_output_property_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_randr_notify_data_t {
    data : [u8; 28]
}
impl Copy for xcb_randr_notify_data_t {}
impl Clone for xcb_randr_notify_data_t {
    fn clone(&self) -> xcb_randr_notify_data_t { *self }
}
/**
 * @brief xcb_randr_notify_data_iterator_t
 **/
#[repr(C)]
pub struct xcb_randr_notify_data_iterator_t {
    pub data : *mut xcb_randr_notify_data_t,
    pub rem  : c_int,
    pub index: c_int
}



#[repr(C)]
pub struct xcb_randr_notify_event_t {
     pub response_type :   u8,
     pub subCode :         u8,
     pub sequence :        u16,
     pub u :               xcb_randr_notify_data_t
}

impl Copy for xcb_randr_notify_event_t {}
impl Clone for xcb_randr_notify_event_t {
    fn clone(&self) -> xcb_randr_notify_event_t { *self }
}
#[link(name="xcb-randr")]
extern "C" {

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_randr_mode_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_randr_mode_t)
 *
 *
 */
pub fn xcb_randr_mode_next (i:*mut xcb_randr_mode_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_randr_mode_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_randr_mode_end (i:xcb_randr_mode_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_randr_crtc_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_randr_crtc_t)
 *
 *
 */
pub fn xcb_randr_crtc_next (i:*mut xcb_randr_crtc_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_randr_crtc_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_randr_crtc_end (i:xcb_randr_crtc_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_randr_output_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_randr_output_t)
 *
 *
 */
pub fn xcb_randr_output_next (i:*mut xcb_randr_output_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_randr_output_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_randr_output_end (i:xcb_randr_output_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_randr_screen_size_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_randr_screen_size_t)
 *
 *
 */
pub fn xcb_randr_screen_size_next (i:*mut xcb_randr_screen_size_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_randr_screen_size_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_randr_screen_size_end (i:xcb_randr_screen_size_iterator_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_randr_refresh_rates_sizeof (_buffer :  *mut c_void) -> c_int;

pub fn xcb_randr_refresh_rates_rates (R : *mut xcb_randr_refresh_rates_t) -> *mut u16;


pub fn xcb_randr_refresh_rates_rates_length (R : *mut xcb_randr_refresh_rates_t) -> c_int;


pub fn xcb_randr_refresh_rates_rates_end (R : *mut xcb_randr_refresh_rates_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_randr_refresh_rates_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_randr_refresh_rates_t)
 *
 *
 */
pub fn xcb_randr_refresh_rates_next (i:*mut xcb_randr_refresh_rates_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_randr_refresh_rates_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_randr_refresh_rates_end (i:xcb_randr_refresh_rates_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_randr_query_version (c : *mut ffi::base::xcb_connection_t,
                                   major_version :  u32,
                                   minor_version :  u32) -> xcb_randr_query_version_cookie_t;

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
pub fn xcb_randr_query_version_unchecked (c : *mut ffi::base::xcb_connection_t,
                                             major_version :  u32,
                                             minor_version :  u32) -> xcb_randr_query_version_cookie_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_randr_query_version_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_randr_query_version_reply (c : *mut ffi::base::xcb_connection_t,
                                         cookie : xcb_randr_query_version_cookie_t,
                                         e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_randr_query_version_reply_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_randr_set_screen_config (c : *mut ffi::base::xcb_connection_t,
                                       window :  ffi::xproto::xcb_window_t,
                                       timestamp :  ffi::xproto::xcb_timestamp_t,
                                       config_timestamp :  ffi::xproto::xcb_timestamp_t,
                                       sizeID :  u16,
                                       rotation :  u16,
                                       rate :  u16) -> xcb_randr_set_screen_config_cookie_t;

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
pub fn xcb_randr_set_screen_config_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                 window :  ffi::xproto::xcb_window_t,
                                                 timestamp :  ffi::xproto::xcb_timestamp_t,
                                                 config_timestamp :  ffi::xproto::xcb_timestamp_t,
                                                 sizeID :  u16,
                                                 rotation :  u16,
                                                 rate :  u16) -> xcb_randr_set_screen_config_cookie_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_randr_set_screen_config_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_randr_set_screen_config_reply (c : *mut ffi::base::xcb_connection_t,
                                             cookie : xcb_randr_set_screen_config_cookie_t,
                                             e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_randr_set_screen_config_reply_t;

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
pub fn xcb_randr_select_input_checked (c : *mut ffi::base::xcb_connection_t,
                                          window :  ffi::xproto::xcb_window_t,
                                          enable :  u16) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_randr_select_input (c : *mut ffi::base::xcb_connection_t,
                                  window :  ffi::xproto::xcb_window_t,
                                  enable :  u16) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_randr_get_screen_info_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_randr_get_screen_info (c : *mut ffi::base::xcb_connection_t,
                                     window :  ffi::xproto::xcb_window_t) -> xcb_randr_get_screen_info_cookie_t;

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
pub fn xcb_randr_get_screen_info_unchecked (c : *mut ffi::base::xcb_connection_t,
                                               window :  ffi::xproto::xcb_window_t) -> xcb_randr_get_screen_info_cookie_t;

pub fn xcb_randr_get_screen_info_sizes (R : *mut xcb_randr_get_screen_info_reply_t) -> *mut xcb_randr_screen_size_t;


pub fn xcb_randr_get_screen_info_sizes_length (R : *mut xcb_randr_get_screen_info_reply_t) -> c_int;

pub fn xcb_randr_get_screen_info_sizes_iterator (R : *mut xcb_randr_get_screen_info_reply_t) -> xcb_randr_screen_size_iterator_t;


pub fn xcb_randr_get_screen_info_rates_length (R : *mut xcb_randr_get_screen_info_reply_t) -> c_int;

pub fn xcb_randr_get_screen_info_rates_iterator (R : *mut xcb_randr_get_screen_info_reply_t) -> xcb_randr_refresh_rates_iterator_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_randr_get_screen_info_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_randr_get_screen_info_reply (c : *mut ffi::base::xcb_connection_t,
                                           cookie : xcb_randr_get_screen_info_cookie_t,
                                           e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_randr_get_screen_info_reply_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_randr_get_screen_size_range (c : *mut ffi::base::xcb_connection_t,
                                           window :  ffi::xproto::xcb_window_t) -> xcb_randr_get_screen_size_range_cookie_t;

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
pub fn xcb_randr_get_screen_size_range_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                     window :  ffi::xproto::xcb_window_t) -> xcb_randr_get_screen_size_range_cookie_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_randr_get_screen_size_range_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_randr_get_screen_size_range_reply (c : *mut ffi::base::xcb_connection_t,
                                                 cookie : xcb_randr_get_screen_size_range_cookie_t,
                                                 e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_randr_get_screen_size_range_reply_t;

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
pub fn xcb_randr_set_screen_size_checked (c : *mut ffi::base::xcb_connection_t,
                                             window :  ffi::xproto::xcb_window_t,
                                             width :  u16,
                                             height :  u16,
                                             mm_width :  u32,
                                             mm_height :  u32) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_randr_set_screen_size (c : *mut ffi::base::xcb_connection_t,
                                     window :  ffi::xproto::xcb_window_t,
                                     width :  u16,
                                     height :  u16,
                                     mm_width :  u32,
                                     mm_height :  u32) -> ffi::base::xcb_void_cookie_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_randr_mode_info_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_randr_mode_info_t)
 *
 *
 */
pub fn xcb_randr_mode_info_next (i:*mut xcb_randr_mode_info_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_randr_mode_info_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_randr_mode_info_end (i:xcb_randr_mode_info_iterator_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_randr_get_screen_resources_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_randr_get_screen_resources (c : *mut ffi::base::xcb_connection_t,
                                          window :  ffi::xproto::xcb_window_t) -> xcb_randr_get_screen_resources_cookie_t;

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
pub fn xcb_randr_get_screen_resources_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                    window :  ffi::xproto::xcb_window_t) -> xcb_randr_get_screen_resources_cookie_t;

pub fn xcb_randr_get_screen_resources_crtcs (R : *mut xcb_randr_get_screen_resources_reply_t) -> *mut xcb_randr_crtc_t;


pub fn xcb_randr_get_screen_resources_crtcs_length (R : *mut xcb_randr_get_screen_resources_reply_t) -> c_int;


pub fn xcb_randr_get_screen_resources_crtcs_end (R : *mut xcb_randr_get_screen_resources_reply_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_randr_get_screen_resources_outputs (R : *mut xcb_randr_get_screen_resources_reply_t) -> *mut xcb_randr_output_t;


pub fn xcb_randr_get_screen_resources_outputs_length (R : *mut xcb_randr_get_screen_resources_reply_t) -> c_int;


pub fn xcb_randr_get_screen_resources_outputs_end (R : *mut xcb_randr_get_screen_resources_reply_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_randr_get_screen_resources_modes (R : *mut xcb_randr_get_screen_resources_reply_t) -> *mut xcb_randr_mode_info_t;


pub fn xcb_randr_get_screen_resources_modes_length (R : *mut xcb_randr_get_screen_resources_reply_t) -> c_int;

pub fn xcb_randr_get_screen_resources_modes_iterator (R : *mut xcb_randr_get_screen_resources_reply_t) -> xcb_randr_mode_info_iterator_t;

pub fn xcb_randr_get_screen_resources_names (R : *mut xcb_randr_get_screen_resources_reply_t) -> *mut u8;


pub fn xcb_randr_get_screen_resources_names_length (R : *mut xcb_randr_get_screen_resources_reply_t) -> c_int;


pub fn xcb_randr_get_screen_resources_names_end (R : *mut xcb_randr_get_screen_resources_reply_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_randr_get_screen_resources_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_randr_get_screen_resources_reply (c : *mut ffi::base::xcb_connection_t,
                                                cookie : xcb_randr_get_screen_resources_cookie_t,
                                                e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_randr_get_screen_resources_reply_t;

pub fn xcb_randr_get_output_info_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_randr_get_output_info (c : *mut ffi::base::xcb_connection_t,
                                     output :  xcb_randr_output_t,
                                     config_timestamp :  ffi::xproto::xcb_timestamp_t) -> xcb_randr_get_output_info_cookie_t;

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
pub fn xcb_randr_get_output_info_unchecked (c : *mut ffi::base::xcb_connection_t,
                                               output :  xcb_randr_output_t,
                                               config_timestamp :  ffi::xproto::xcb_timestamp_t) -> xcb_randr_get_output_info_cookie_t;

pub fn xcb_randr_get_output_info_crtcs (R : *mut xcb_randr_get_output_info_reply_t) -> *mut xcb_randr_crtc_t;


pub fn xcb_randr_get_output_info_crtcs_length (R : *mut xcb_randr_get_output_info_reply_t) -> c_int;


pub fn xcb_randr_get_output_info_crtcs_end (R : *mut xcb_randr_get_output_info_reply_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_randr_get_output_info_modes (R : *mut xcb_randr_get_output_info_reply_t) -> *mut xcb_randr_mode_t;


pub fn xcb_randr_get_output_info_modes_length (R : *mut xcb_randr_get_output_info_reply_t) -> c_int;


pub fn xcb_randr_get_output_info_modes_end (R : *mut xcb_randr_get_output_info_reply_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_randr_get_output_info_clones (R : *mut xcb_randr_get_output_info_reply_t) -> *mut xcb_randr_output_t;


pub fn xcb_randr_get_output_info_clones_length (R : *mut xcb_randr_get_output_info_reply_t) -> c_int;


pub fn xcb_randr_get_output_info_clones_end (R : *mut xcb_randr_get_output_info_reply_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_randr_get_output_info_name (R : *mut xcb_randr_get_output_info_reply_t) -> *mut u8;


pub fn xcb_randr_get_output_info_name_length (R : *mut xcb_randr_get_output_info_reply_t) -> c_int;


pub fn xcb_randr_get_output_info_name_end (R : *mut xcb_randr_get_output_info_reply_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_randr_get_output_info_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_randr_get_output_info_reply (c : *mut ffi::base::xcb_connection_t,
                                           cookie : xcb_randr_get_output_info_cookie_t,
                                           e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_randr_get_output_info_reply_t;

pub fn xcb_randr_list_output_properties_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_randr_list_output_properties (c : *mut ffi::base::xcb_connection_t,
                                            output :  xcb_randr_output_t) -> xcb_randr_list_output_properties_cookie_t;

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
pub fn xcb_randr_list_output_properties_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                      output :  xcb_randr_output_t) -> xcb_randr_list_output_properties_cookie_t;

pub fn xcb_randr_list_output_properties_atoms (R : *mut xcb_randr_list_output_properties_reply_t) -> *mut ffi::xproto::xcb_atom_t;


pub fn xcb_randr_list_output_properties_atoms_length (R : *mut xcb_randr_list_output_properties_reply_t) -> c_int;


pub fn xcb_randr_list_output_properties_atoms_end (R : *mut xcb_randr_list_output_properties_reply_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_randr_list_output_properties_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_randr_list_output_properties_reply (c : *mut ffi::base::xcb_connection_t,
                                                  cookie : xcb_randr_list_output_properties_cookie_t,
                                                  e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_randr_list_output_properties_reply_t;

pub fn xcb_randr_query_output_property_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_randr_query_output_property (c : *mut ffi::base::xcb_connection_t,
                                           output :  xcb_randr_output_t,
                                           property :  ffi::xproto::xcb_atom_t) -> xcb_randr_query_output_property_cookie_t;

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
pub fn xcb_randr_query_output_property_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                     output :  xcb_randr_output_t,
                                                     property :  ffi::xproto::xcb_atom_t) -> xcb_randr_query_output_property_cookie_t;

pub fn xcb_randr_query_output_property_valid_values (R : *mut xcb_randr_query_output_property_reply_t) -> *mut i32;


pub fn xcb_randr_query_output_property_valid_values_length (R : *mut xcb_randr_query_output_property_reply_t) -> c_int;


pub fn xcb_randr_query_output_property_valid_values_end (R : *mut xcb_randr_query_output_property_reply_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_randr_query_output_property_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_randr_query_output_property_reply (c : *mut ffi::base::xcb_connection_t,
                                                 cookie : xcb_randr_query_output_property_cookie_t,
                                                 e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_randr_query_output_property_reply_t;

pub fn xcb_randr_configure_output_property_sizeof (_buffer :  *mut c_void,
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
pub fn xcb_randr_configure_output_property_checked (c : *mut ffi::base::xcb_connection_t,
                                                       output :  xcb_randr_output_t,
                                                       property :  ffi::xproto::xcb_atom_t,
                                                       pending :  u8,
                                                       range :  u8,
                                                       values_len :  u32,
                                                       values : *mut i32) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_randr_configure_output_property (c : *mut ffi::base::xcb_connection_t,
                                               output :  xcb_randr_output_t,
                                               property :  ffi::xproto::xcb_atom_t,
                                               pending :  u8,
                                               range :  u8,
                                               values_len :  u32,
                                               values : *mut i32) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_randr_change_output_property_sizeof (_buffer :  *mut c_void) -> c_int;

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
pub fn xcb_randr_change_output_property_checked (c : *mut ffi::base::xcb_connection_t,
                                                    output :  xcb_randr_output_t,
                                                    property :  ffi::xproto::xcb_atom_t,
                                                    type_ :  ffi::xproto::xcb_atom_t,
                                                    format :  u8,
                                                    mode :  u8,
                                                    num_units :  u32,
                                                    data : *mut c_void) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_randr_change_output_property (c : *mut ffi::base::xcb_connection_t,
                                            output :  xcb_randr_output_t,
                                            property :  ffi::xproto::xcb_atom_t,
                                            type_ :  ffi::xproto::xcb_atom_t,
                                            format :  u8,
                                            mode :  u8,
                                            num_units :  u32,
                                            data : *mut c_void) -> ffi::base::xcb_void_cookie_t;

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
pub fn xcb_randr_delete_output_property_checked (c : *mut ffi::base::xcb_connection_t,
                                                    output :  xcb_randr_output_t,
                                                    property :  ffi::xproto::xcb_atom_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_randr_delete_output_property (c : *mut ffi::base::xcb_connection_t,
                                            output :  xcb_randr_output_t,
                                            property :  ffi::xproto::xcb_atom_t) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_randr_get_output_property_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_randr_get_output_property (c : *mut ffi::base::xcb_connection_t,
                                         output :  xcb_randr_output_t,
                                         property :  ffi::xproto::xcb_atom_t,
                                         type_ :  ffi::xproto::xcb_atom_t,
                                         long_offset :  u32,
                                         long_length :  u32,
                                         delete :  u8,
                                         pending :  u8) -> xcb_randr_get_output_property_cookie_t;

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
pub fn xcb_randr_get_output_property_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                   output :  xcb_randr_output_t,
                                                   property :  ffi::xproto::xcb_atom_t,
                                                   type_ :  ffi::xproto::xcb_atom_t,
                                                   long_offset :  u32,
                                                   long_length :  u32,
                                                   delete :  u8,
                                                   pending :  u8) -> xcb_randr_get_output_property_cookie_t;

pub fn xcb_randr_get_output_property_data (R : *mut xcb_randr_get_output_property_reply_t) -> *mut u8;


pub fn xcb_randr_get_output_property_data_length (R : *mut xcb_randr_get_output_property_reply_t) -> c_int;


pub fn xcb_randr_get_output_property_data_end (R : *mut xcb_randr_get_output_property_reply_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_randr_get_output_property_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_randr_get_output_property_reply (c : *mut ffi::base::xcb_connection_t,
                                               cookie : xcb_randr_get_output_property_cookie_t,
                                               e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_randr_get_output_property_reply_t;

pub fn xcb_randr_create_mode_sizeof (_buffer :  *mut c_void,
                              name_len :     u32) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_randr_create_mode (c : *mut ffi::base::xcb_connection_t,
                                 window :  ffi::xproto::xcb_window_t,
                                 mode_info :  xcb_randr_mode_info_t,
                                 name_len :  u32,
                                 name : *mut c_char) -> xcb_randr_create_mode_cookie_t;

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
pub fn xcb_randr_create_mode_unchecked (c : *mut ffi::base::xcb_connection_t,
                                           window :  ffi::xproto::xcb_window_t,
                                           mode_info :  xcb_randr_mode_info_t,
                                           name_len :  u32,
                                           name : *mut c_char) -> xcb_randr_create_mode_cookie_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_randr_create_mode_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_randr_create_mode_reply (c : *mut ffi::base::xcb_connection_t,
                                       cookie : xcb_randr_create_mode_cookie_t,
                                       e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_randr_create_mode_reply_t;

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
pub fn xcb_randr_destroy_mode_checked (c : *mut ffi::base::xcb_connection_t,
                                          mode :  xcb_randr_mode_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_randr_destroy_mode (c : *mut ffi::base::xcb_connection_t,
                                  mode :  xcb_randr_mode_t) -> ffi::base::xcb_void_cookie_t;

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
pub fn xcb_randr_add_output_mode_checked (c : *mut ffi::base::xcb_connection_t,
                                             output :  xcb_randr_output_t,
                                             mode :  xcb_randr_mode_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_randr_add_output_mode (c : *mut ffi::base::xcb_connection_t,
                                     output :  xcb_randr_output_t,
                                     mode :  xcb_randr_mode_t) -> ffi::base::xcb_void_cookie_t;

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
pub fn xcb_randr_delete_output_mode_checked (c : *mut ffi::base::xcb_connection_t,
                                                output :  xcb_randr_output_t,
                                                mode :  xcb_randr_mode_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_randr_delete_output_mode (c : *mut ffi::base::xcb_connection_t,
                                        output :  xcb_randr_output_t,
                                        mode :  xcb_randr_mode_t) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_randr_get_crtc_info_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_randr_get_crtc_info (c : *mut ffi::base::xcb_connection_t,
                                   crtc :  xcb_randr_crtc_t,
                                   config_timestamp :  ffi::xproto::xcb_timestamp_t) -> xcb_randr_get_crtc_info_cookie_t;

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
pub fn xcb_randr_get_crtc_info_unchecked (c : *mut ffi::base::xcb_connection_t,
                                             crtc :  xcb_randr_crtc_t,
                                             config_timestamp :  ffi::xproto::xcb_timestamp_t) -> xcb_randr_get_crtc_info_cookie_t;

pub fn xcb_randr_get_crtc_info_outputs (R : *mut xcb_randr_get_crtc_info_reply_t) -> *mut xcb_randr_output_t;


pub fn xcb_randr_get_crtc_info_outputs_length (R : *mut xcb_randr_get_crtc_info_reply_t) -> c_int;


pub fn xcb_randr_get_crtc_info_outputs_end (R : *mut xcb_randr_get_crtc_info_reply_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_randr_get_crtc_info_possible (R : *mut xcb_randr_get_crtc_info_reply_t) -> *mut xcb_randr_output_t;


pub fn xcb_randr_get_crtc_info_possible_length (R : *mut xcb_randr_get_crtc_info_reply_t) -> c_int;


pub fn xcb_randr_get_crtc_info_possible_end (R : *mut xcb_randr_get_crtc_info_reply_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_randr_get_crtc_info_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_randr_get_crtc_info_reply (c : *mut ffi::base::xcb_connection_t,
                                         cookie : xcb_randr_get_crtc_info_cookie_t,
                                         e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_randr_get_crtc_info_reply_t;

pub fn xcb_randr_set_crtc_config_sizeof (_buffer :  *mut c_void,
                                  outputs_len :  u32) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_randr_set_crtc_config (c : *mut ffi::base::xcb_connection_t,
                                     crtc :  xcb_randr_crtc_t,
                                     timestamp :  ffi::xproto::xcb_timestamp_t,
                                     config_timestamp :  ffi::xproto::xcb_timestamp_t,
                                     x :  i16,
                                     y :  i16,
                                     mode :  xcb_randr_mode_t,
                                     rotation :  u16,
                                     outputs_len :  u32,
                                     outputs : *mut xcb_randr_output_t) -> xcb_randr_set_crtc_config_cookie_t;

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
pub fn xcb_randr_set_crtc_config_unchecked (c : *mut ffi::base::xcb_connection_t,
                                               crtc :  xcb_randr_crtc_t,
                                               timestamp :  ffi::xproto::xcb_timestamp_t,
                                               config_timestamp :  ffi::xproto::xcb_timestamp_t,
                                               x :  i16,
                                               y :  i16,
                                               mode :  xcb_randr_mode_t,
                                               rotation :  u16,
                                               outputs_len :  u32,
                                               outputs : *mut xcb_randr_output_t) -> xcb_randr_set_crtc_config_cookie_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_randr_set_crtc_config_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_randr_set_crtc_config_reply (c : *mut ffi::base::xcb_connection_t,
                                           cookie : xcb_randr_set_crtc_config_cookie_t,
                                           e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_randr_set_crtc_config_reply_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_randr_get_crtc_gamma_size (c : *mut ffi::base::xcb_connection_t,
                                         crtc :  xcb_randr_crtc_t) -> xcb_randr_get_crtc_gamma_size_cookie_t;

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
pub fn xcb_randr_get_crtc_gamma_size_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                   crtc :  xcb_randr_crtc_t) -> xcb_randr_get_crtc_gamma_size_cookie_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_randr_get_crtc_gamma_size_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_randr_get_crtc_gamma_size_reply (c : *mut ffi::base::xcb_connection_t,
                                               cookie : xcb_randr_get_crtc_gamma_size_cookie_t,
                                               e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_randr_get_crtc_gamma_size_reply_t;

pub fn xcb_randr_get_crtc_gamma_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_randr_get_crtc_gamma (c : *mut ffi::base::xcb_connection_t,
                                    crtc :  xcb_randr_crtc_t) -> xcb_randr_get_crtc_gamma_cookie_t;

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
pub fn xcb_randr_get_crtc_gamma_unchecked (c : *mut ffi::base::xcb_connection_t,
                                              crtc :  xcb_randr_crtc_t) -> xcb_randr_get_crtc_gamma_cookie_t;

pub fn xcb_randr_get_crtc_gamma_red (R : *mut xcb_randr_get_crtc_gamma_reply_t) -> *mut u16;


pub fn xcb_randr_get_crtc_gamma_red_length (R : *mut xcb_randr_get_crtc_gamma_reply_t) -> c_int;


pub fn xcb_randr_get_crtc_gamma_red_end (R : *mut xcb_randr_get_crtc_gamma_reply_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_randr_get_crtc_gamma_green (R : *mut xcb_randr_get_crtc_gamma_reply_t) -> *mut u16;


pub fn xcb_randr_get_crtc_gamma_green_length (R : *mut xcb_randr_get_crtc_gamma_reply_t) -> c_int;


pub fn xcb_randr_get_crtc_gamma_green_end (R : *mut xcb_randr_get_crtc_gamma_reply_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_randr_get_crtc_gamma_blue (R : *mut xcb_randr_get_crtc_gamma_reply_t) -> *mut u16;


pub fn xcb_randr_get_crtc_gamma_blue_length (R : *mut xcb_randr_get_crtc_gamma_reply_t) -> c_int;


pub fn xcb_randr_get_crtc_gamma_blue_end (R : *mut xcb_randr_get_crtc_gamma_reply_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_randr_get_crtc_gamma_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_randr_get_crtc_gamma_reply (c : *mut ffi::base::xcb_connection_t,
                                          cookie : xcb_randr_get_crtc_gamma_cookie_t,
                                          e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_randr_get_crtc_gamma_reply_t;

pub fn xcb_randr_set_crtc_gamma_sizeof (_buffer :  *mut c_void) -> c_int;

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
pub fn xcb_randr_set_crtc_gamma_checked (c : *mut ffi::base::xcb_connection_t,
                                            crtc :  xcb_randr_crtc_t,
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
pub fn xcb_randr_set_crtc_gamma (c : *mut ffi::base::xcb_connection_t,
                                    crtc :  xcb_randr_crtc_t,
                                    size :  u16,
                                    red : *mut u16,
                                    green : *mut u16,
                                    blue : *mut u16) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_randr_get_screen_resources_current_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_randr_get_screen_resources_current (c : *mut ffi::base::xcb_connection_t,
                                                  window :  ffi::xproto::xcb_window_t) -> xcb_randr_get_screen_resources_current_cookie_t;

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
pub fn xcb_randr_get_screen_resources_current_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                            window :  ffi::xproto::xcb_window_t) -> xcb_randr_get_screen_resources_current_cookie_t;

pub fn xcb_randr_get_screen_resources_current_crtcs (R : *mut xcb_randr_get_screen_resources_current_reply_t) -> *mut xcb_randr_crtc_t;


pub fn xcb_randr_get_screen_resources_current_crtcs_length (R : *mut xcb_randr_get_screen_resources_current_reply_t) -> c_int;


pub fn xcb_randr_get_screen_resources_current_crtcs_end (R : *mut xcb_randr_get_screen_resources_current_reply_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_randr_get_screen_resources_current_outputs (R : *mut xcb_randr_get_screen_resources_current_reply_t) -> *mut xcb_randr_output_t;


pub fn xcb_randr_get_screen_resources_current_outputs_length (R : *mut xcb_randr_get_screen_resources_current_reply_t) -> c_int;


pub fn xcb_randr_get_screen_resources_current_outputs_end (R : *mut xcb_randr_get_screen_resources_current_reply_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_randr_get_screen_resources_current_modes (R : *mut xcb_randr_get_screen_resources_current_reply_t) -> *mut xcb_randr_mode_info_t;


pub fn xcb_randr_get_screen_resources_current_modes_length (R : *mut xcb_randr_get_screen_resources_current_reply_t) -> c_int;

pub fn xcb_randr_get_screen_resources_current_modes_iterator (R : *mut xcb_randr_get_screen_resources_current_reply_t) -> xcb_randr_mode_info_iterator_t;

pub fn xcb_randr_get_screen_resources_current_names (R : *mut xcb_randr_get_screen_resources_current_reply_t) -> *mut u8;


pub fn xcb_randr_get_screen_resources_current_names_length (R : *mut xcb_randr_get_screen_resources_current_reply_t) -> c_int;


pub fn xcb_randr_get_screen_resources_current_names_end (R : *mut xcb_randr_get_screen_resources_current_reply_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_randr_get_screen_resources_current_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_randr_get_screen_resources_current_reply (c : *mut ffi::base::xcb_connection_t,
                                                        cookie : xcb_randr_get_screen_resources_current_cookie_t,
                                                        e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_randr_get_screen_resources_current_reply_t;

pub fn xcb_randr_set_crtc_transform_sizeof (_buffer :  *mut c_void,
                                     filter_params_len :  u32) -> c_int;

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
pub fn xcb_randr_set_crtc_transform_checked (c : *mut ffi::base::xcb_connection_t,
                                                crtc :  xcb_randr_crtc_t,
                                                transform :  ffi::render::xcb_render_transform_t,
                                                filter_len :  u16,
                                                filter_name : *mut c_char,
                                                filter_params_len :  u32,
                                                filter_params : *mut ffi::render::xcb_render_fixed_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_randr_set_crtc_transform (c : *mut ffi::base::xcb_connection_t,
                                        crtc :  xcb_randr_crtc_t,
                                        transform :  ffi::render::xcb_render_transform_t,
                                        filter_len :  u16,
                                        filter_name : *mut c_char,
                                        filter_params_len :  u32,
                                        filter_params : *mut ffi::render::xcb_render_fixed_t) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_randr_get_crtc_transform_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_randr_get_crtc_transform (c : *mut ffi::base::xcb_connection_t,
                                        crtc :  xcb_randr_crtc_t) -> xcb_randr_get_crtc_transform_cookie_t;

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
pub fn xcb_randr_get_crtc_transform_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                  crtc :  xcb_randr_crtc_t) -> xcb_randr_get_crtc_transform_cookie_t;

pub fn xcb_randr_get_crtc_transform_pending_filter_name (R : *mut xcb_randr_get_crtc_transform_reply_t) -> *mut c_char;


pub fn xcb_randr_get_crtc_transform_pending_filter_name_length (R : *mut xcb_randr_get_crtc_transform_reply_t) -> c_int;


pub fn xcb_randr_get_crtc_transform_pending_filter_name_end (R : *mut xcb_randr_get_crtc_transform_reply_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_randr_get_crtc_transform_pending_params (R : *mut xcb_randr_get_crtc_transform_reply_t) -> *mut ffi::render::xcb_render_fixed_t;


pub fn xcb_randr_get_crtc_transform_pending_params_length (R : *mut xcb_randr_get_crtc_transform_reply_t) -> c_int;


pub fn xcb_randr_get_crtc_transform_pending_params_end (R : *mut xcb_randr_get_crtc_transform_reply_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_randr_get_crtc_transform_current_filter_name (R : *mut xcb_randr_get_crtc_transform_reply_t) -> *mut c_char;


pub fn xcb_randr_get_crtc_transform_current_filter_name_length (R : *mut xcb_randr_get_crtc_transform_reply_t) -> c_int;


pub fn xcb_randr_get_crtc_transform_current_filter_name_end (R : *mut xcb_randr_get_crtc_transform_reply_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_randr_get_crtc_transform_current_params (R : *mut xcb_randr_get_crtc_transform_reply_t) -> *mut ffi::render::xcb_render_fixed_t;


pub fn xcb_randr_get_crtc_transform_current_params_length (R : *mut xcb_randr_get_crtc_transform_reply_t) -> c_int;


pub fn xcb_randr_get_crtc_transform_current_params_end (R : *mut xcb_randr_get_crtc_transform_reply_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_randr_get_crtc_transform_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_randr_get_crtc_transform_reply (c : *mut ffi::base::xcb_connection_t,
                                              cookie : xcb_randr_get_crtc_transform_cookie_t,
                                              e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_randr_get_crtc_transform_reply_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_randr_get_panning (c : *mut ffi::base::xcb_connection_t,
                                 crtc :  xcb_randr_crtc_t) -> xcb_randr_get_panning_cookie_t;

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
pub fn xcb_randr_get_panning_unchecked (c : *mut ffi::base::xcb_connection_t,
                                           crtc :  xcb_randr_crtc_t) -> xcb_randr_get_panning_cookie_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_randr_get_panning_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_randr_get_panning_reply (c : *mut ffi::base::xcb_connection_t,
                                       cookie : xcb_randr_get_panning_cookie_t,
                                       e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_randr_get_panning_reply_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_randr_set_panning (c : *mut ffi::base::xcb_connection_t,
                                 crtc :  xcb_randr_crtc_t,
                                 timestamp :  ffi::xproto::xcb_timestamp_t,
                                 left :  u16,
                                 top :  u16,
                                 width :  u16,
                                 height :  u16,
                                 track_left :  u16,
                                 track_top :  u16,
                                 track_width :  u16,
                                 track_height :  u16,
                                 border_left :  i16,
                                 border_top :  i16,
                                 border_right :  i16,
                                 border_bottom :  i16) -> xcb_randr_set_panning_cookie_t;

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
pub fn xcb_randr_set_panning_unchecked (c : *mut ffi::base::xcb_connection_t,
                                           crtc :  xcb_randr_crtc_t,
                                           timestamp :  ffi::xproto::xcb_timestamp_t,
                                           left :  u16,
                                           top :  u16,
                                           width :  u16,
                                           height :  u16,
                                           track_left :  u16,
                                           track_top :  u16,
                                           track_width :  u16,
                                           track_height :  u16,
                                           border_left :  i16,
                                           border_top :  i16,
                                           border_right :  i16,
                                           border_bottom :  i16) -> xcb_randr_set_panning_cookie_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_randr_set_panning_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_randr_set_panning_reply (c : *mut ffi::base::xcb_connection_t,
                                       cookie : xcb_randr_set_panning_cookie_t,
                                       e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_randr_set_panning_reply_t;

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
pub fn xcb_randr_set_output_primary_checked (c : *mut ffi::base::xcb_connection_t,
                                                window :  ffi::xproto::xcb_window_t,
                                                output :  xcb_randr_output_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_randr_set_output_primary (c : *mut ffi::base::xcb_connection_t,
                                        window :  ffi::xproto::xcb_window_t,
                                        output :  xcb_randr_output_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_randr_get_output_primary (c : *mut ffi::base::xcb_connection_t,
                                        window :  ffi::xproto::xcb_window_t) -> xcb_randr_get_output_primary_cookie_t;

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
pub fn xcb_randr_get_output_primary_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                  window :  ffi::xproto::xcb_window_t) -> xcb_randr_get_output_primary_cookie_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_randr_get_output_primary_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_randr_get_output_primary_reply (c : *mut ffi::base::xcb_connection_t,
                                              cookie : xcb_randr_get_output_primary_cookie_t,
                                              e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_randr_get_output_primary_reply_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_randr_crtc_change_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_randr_crtc_change_t)
 *
 *
 */
pub fn xcb_randr_crtc_change_next (i:*mut xcb_randr_crtc_change_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_randr_crtc_change_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_randr_crtc_change_end (i:xcb_randr_crtc_change_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_randr_output_change_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_randr_output_change_t)
 *
 *
 */
pub fn xcb_randr_output_change_next (i:*mut xcb_randr_output_change_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_randr_output_change_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_randr_output_change_end (i:xcb_randr_output_change_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_randr_output_property_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_randr_output_property_t)
 *
 *
 */
pub fn xcb_randr_output_property_next (i:*mut xcb_randr_output_property_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_randr_output_property_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_randr_output_property_end (i:xcb_randr_output_property_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_randr_notify_data_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_randr_notify_data_t)
 *
 *
 */
pub fn xcb_randr_notify_data_next (i:*mut xcb_randr_notify_data_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_randr_notify_data_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_randr_notify_data_end (i:xcb_randr_notify_data_iterator_t) -> ffi::base::xcb_generic_iterator_t;
}

