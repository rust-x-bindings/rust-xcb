//
// This file generated automatically from xv.xml by r_client.py.
// Edit at your peril.
//

//Make the compiler quiet
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use std;
use libc::*;
use ffi;
use ffi::xproto;
use ffi::shm;

pub const XV_MAJOR_VERSION : c_uint = 2;
pub const XV_MINOR_VERSION : c_uint = 2;

pub type xcb_xv_port_t = u32;
#[repr(C)]
pub struct xcb_xv_port_iterator_t {
    pub data : *mut xcb_xv_port_t,
    pub rem  : c_int,
    pub index: c_int
}


pub type xcb_xv_encoding_t = u32;
#[repr(C)]
pub struct xcb_xv_encoding_iterator_t {
    pub data : *mut xcb_xv_encoding_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_xv_rational_t {
    pub numerator :     i32,
    pub denominator :   i32
}

impl Copy for xcb_xv_rational_t {}
impl Clone for xcb_xv_rational_t {
    fn clone(&self) -> xcb_xv_rational_t { *self }
}
#[repr(C)]
pub struct xcb_xv_rational_iterator_t {
    pub data : *mut xcb_xv_rational_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_xv_format_t {
    pub visual :   ffi::xproto::xcb_visualid_t,
    pub depth :    u8,
    pub pad0 :     [u8; 3]
}

impl Copy for xcb_xv_format_t {}
impl Clone for xcb_xv_format_t {
    fn clone(&self) -> xcb_xv_format_t { *self }
}
#[repr(C)]
pub struct xcb_xv_format_iterator_t {
    pub data : *mut xcb_xv_format_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_xv_adaptor_info_t {
    pub base_id :       xcb_xv_port_t,
    pub name_size :     u16,
    pub num_ports :     u16,
    pub num_formats :   u16,
    pub type_ :         u8,
    pub pad0 :          u8
}

impl Copy for xcb_xv_adaptor_info_t {}
impl Clone for xcb_xv_adaptor_info_t {
    fn clone(&self) -> xcb_xv_adaptor_info_t { *self }
}
#[repr(C)]
pub struct xcb_xv_adaptor_info_iterator_t {
    pub data : *mut xcb_xv_adaptor_info_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_xv_encoding_info_t {
    pub encoding :    xcb_xv_encoding_t,
    pub name_size :   u16,
    pub width :       u16,
    pub height :      u16,
    pub pad0 :        [u8; 2],
    pub rate :        xcb_xv_rational_t
}

impl Copy for xcb_xv_encoding_info_t {}
impl Clone for xcb_xv_encoding_info_t {
    fn clone(&self) -> xcb_xv_encoding_info_t { *self }
}
#[repr(C)]
pub struct xcb_xv_encoding_info_iterator_t {
    pub data : *mut xcb_xv_encoding_info_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_xv_image_t {
    pub id :           u32,
    pub width :        u16,
    pub height :       u16,
    pub data_size :    u32,
    pub num_planes :   u32
}

impl Copy for xcb_xv_image_t {}
impl Clone for xcb_xv_image_t {
    fn clone(&self) -> xcb_xv_image_t { *self }
}
#[repr(C)]
pub struct xcb_xv_image_iterator_t {
    pub data : *mut xcb_xv_image_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_xv_attribute_info_t {
    pub flags :   u32,
    pub min :     i32,
    pub max :     i32,
    pub size :    u32
}

impl Copy for xcb_xv_attribute_info_t {}
impl Clone for xcb_xv_attribute_info_t {
    fn clone(&self) -> xcb_xv_attribute_info_t { *self }
}
#[repr(C)]
pub struct xcb_xv_attribute_info_iterator_t {
    pub data : *mut xcb_xv_attribute_info_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_xv_image_format_info_t {
    pub id :                u32,
    pub type_ :             u8,
    pub byte_order :        u8,
    pub pad0 :              [u8; 2],
    pub guid :              [u8; 16],
    pub bpp :               u8,
    pub num_planes :        u8,
    pub pad1 :              [u8; 2],
    pub depth :             u8,
    pub pad2 :              [u8; 3],
    pub red_mask :          u32,
    pub green_mask :        u32,
    pub blue_mask :         u32,
    pub format :            u8,
    pub pad3 :              [u8; 3],
    pub y_sample_bits :     u32,
    pub u_sample_bits :     u32,
    pub v_sample_bits :     u32,
    pub vhorz_y_period :    u32,
    pub vhorz_u_period :    u32,
    pub vhorz_v_period :    u32,
    pub vvert_y_period :    u32,
    pub vvert_u_period :    u32,
    pub vvert_v_period :    u32,
    pub vcomp_order :       [u8; 32],
    pub vscanline_order :   u8,
    pub pad4 :              [u8; 11]
}

impl Copy for xcb_xv_image_format_info_t {}
impl Clone for xcb_xv_image_format_info_t {
    fn clone(&self) -> xcb_xv_image_format_info_t { *self }
}
#[repr(C)]
pub struct xcb_xv_image_format_info_iterator_t {
    pub data : *mut xcb_xv_image_format_info_t,
    pub rem  : c_int,
    pub index: c_int
}



#[repr(C)]
pub struct xcb_xv_bad_port_error_t {
    pub response_type :   u8,
    pub error_code :      u8,
    pub sequence :        u16
}

impl Copy for xcb_xv_bad_port_error_t {}
impl Clone for xcb_xv_bad_port_error_t {
    fn clone(&self) -> xcb_xv_bad_port_error_t { *self }
}


#[repr(C)]
pub struct xcb_xv_bad_encoding_error_t {
    pub response_type :   u8,
    pub error_code :      u8,
    pub sequence :        u16
}

impl Copy for xcb_xv_bad_encoding_error_t {}
impl Clone for xcb_xv_bad_encoding_error_t {
    fn clone(&self) -> xcb_xv_bad_encoding_error_t { *self }
}


#[repr(C)]
pub struct xcb_xv_bad_control_error_t {
    pub response_type :   u8,
    pub error_code :      u8,
    pub sequence :        u16
}

impl Copy for xcb_xv_bad_control_error_t {}
impl Clone for xcb_xv_bad_control_error_t {
    fn clone(&self) -> xcb_xv_bad_control_error_t { *self }
}


#[repr(C)]
pub struct xcb_xv_video_notify_event_t {
    pub response_type :   u8,
    pub reason :          u8,
    pub sequence :        u16,
    pub time :            ffi::xproto::xcb_timestamp_t,
    pub drawable :        ffi::xproto::xcb_drawable_t,
    pub port :            xcb_xv_port_t
}

impl Copy for xcb_xv_video_notify_event_t {}
impl Clone for xcb_xv_video_notify_event_t {
    fn clone(&self) -> xcb_xv_video_notify_event_t { *self }
}


#[repr(C)]
pub struct xcb_xv_port_notify_event_t {
    pub response_type :   u8,
    pub pad0 :            u8,
    pub sequence :        u16,
    pub time :            ffi::xproto::xcb_timestamp_t,
    pub port :            xcb_xv_port_t,
    pub attribute :       ffi::xproto::xcb_atom_t,
    pub value :           i32
}

impl Copy for xcb_xv_port_notify_event_t {}
impl Clone for xcb_xv_port_notify_event_t {
    fn clone(&self) -> xcb_xv_port_notify_event_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xv_query_extension_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_xv_query_extension_request_t {
    pub major_opcode :   u8,
    pub minor_opcode :   u8,
    pub length :         u16
}

impl Copy for xcb_xv_query_extension_request_t {}
impl Clone for xcb_xv_query_extension_request_t {
    fn clone(&self) -> xcb_xv_query_extension_request_t { *self }
}

#[repr(C)]
pub struct xcb_xv_query_extension_reply_t {
    pub response_type :   u8,
    pub pad0 :            u8,
    pub sequence :        u16,
    pub length :          u32,
    pub major :           u16,
    pub minor :           u16
}

impl Copy for xcb_xv_query_extension_reply_t {}
impl Clone for xcb_xv_query_extension_reply_t {
    fn clone(&self) -> xcb_xv_query_extension_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xv_query_adaptors_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_xv_query_adaptors_request_t {
    pub major_opcode :   u8,
    pub minor_opcode :   u8,
    pub length :         u16,
    pub window :         ffi::xproto::xcb_window_t
}

impl Copy for xcb_xv_query_adaptors_request_t {}
impl Clone for xcb_xv_query_adaptors_request_t {
    fn clone(&self) -> xcb_xv_query_adaptors_request_t { *self }
}

#[repr(C)]
pub struct xcb_xv_query_adaptors_reply_t {
    pub response_type :   u8,
    pub pad0 :            u8,
    pub sequence :        u16,
    pub length :          u32,
    pub num_adaptors :    u16,
    pub pad1 :            [u8; 22]
}

impl Copy for xcb_xv_query_adaptors_reply_t {}
impl Clone for xcb_xv_query_adaptors_reply_t {
    fn clone(&self) -> xcb_xv_query_adaptors_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xv_query_encodings_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_xv_query_encodings_request_t {
    pub major_opcode :   u8,
    pub minor_opcode :   u8,
    pub length :         u16,
    pub port :           xcb_xv_port_t
}

impl Copy for xcb_xv_query_encodings_request_t {}
impl Clone for xcb_xv_query_encodings_request_t {
    fn clone(&self) -> xcb_xv_query_encodings_request_t { *self }
}

#[repr(C)]
pub struct xcb_xv_query_encodings_reply_t {
    pub response_type :   u8,
    pub pad0 :            u8,
    pub sequence :        u16,
    pub length :          u32,
    pub num_encodings :   u16,
    pub pad1 :            [u8; 22]
}

impl Copy for xcb_xv_query_encodings_reply_t {}
impl Clone for xcb_xv_query_encodings_reply_t {
    fn clone(&self) -> xcb_xv_query_encodings_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xv_grab_port_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_xv_grab_port_request_t {
    pub major_opcode :   u8,
    pub minor_opcode :   u8,
    pub length :         u16,
    pub port :           xcb_xv_port_t,
    pub time :           ffi::xproto::xcb_timestamp_t
}

impl Copy for xcb_xv_grab_port_request_t {}
impl Clone for xcb_xv_grab_port_request_t {
    fn clone(&self) -> xcb_xv_grab_port_request_t { *self }
}

#[repr(C)]
pub struct xcb_xv_grab_port_reply_t {
    pub response_type :   u8,
    pub result :          u8,
    pub sequence :        u16,
    pub length :          u32
}

impl Copy for xcb_xv_grab_port_reply_t {}
impl Clone for xcb_xv_grab_port_reply_t {
    fn clone(&self) -> xcb_xv_grab_port_reply_t { *self }
}


#[repr(C)]
pub struct xcb_xv_ungrab_port_request_t {
    pub major_opcode :   u8,
    pub minor_opcode :   u8,
    pub length :         u16,
    pub port :           xcb_xv_port_t,
    pub time :           ffi::xproto::xcb_timestamp_t
}

impl Copy for xcb_xv_ungrab_port_request_t {}
impl Clone for xcb_xv_ungrab_port_request_t {
    fn clone(&self) -> xcb_xv_ungrab_port_request_t { *self }
}


#[repr(C)]
pub struct xcb_xv_put_video_request_t {
    pub major_opcode :   u8,
    pub minor_opcode :   u8,
    pub length :         u16,
    pub port :           xcb_xv_port_t,
    pub drawable :       ffi::xproto::xcb_drawable_t,
    pub gc :             ffi::xproto::xcb_gcontext_t,
    pub vid_x :          i16,
    pub vid_y :          i16,
    pub vid_w :          u16,
    pub vid_h :          u16,
    pub drw_x :          i16,
    pub drw_y :          i16,
    pub drw_w :          u16,
    pub drw_h :          u16
}

impl Copy for xcb_xv_put_video_request_t {}
impl Clone for xcb_xv_put_video_request_t {
    fn clone(&self) -> xcb_xv_put_video_request_t { *self }
}


#[repr(C)]
pub struct xcb_xv_put_still_request_t {
    pub major_opcode :   u8,
    pub minor_opcode :   u8,
    pub length :         u16,
    pub port :           xcb_xv_port_t,
    pub drawable :       ffi::xproto::xcb_drawable_t,
    pub gc :             ffi::xproto::xcb_gcontext_t,
    pub vid_x :          i16,
    pub vid_y :          i16,
    pub vid_w :          u16,
    pub vid_h :          u16,
    pub drw_x :          i16,
    pub drw_y :          i16,
    pub drw_w :          u16,
    pub drw_h :          u16
}

impl Copy for xcb_xv_put_still_request_t {}
impl Clone for xcb_xv_put_still_request_t {
    fn clone(&self) -> xcb_xv_put_still_request_t { *self }
}


#[repr(C)]
pub struct xcb_xv_get_video_request_t {
    pub major_opcode :   u8,
    pub minor_opcode :   u8,
    pub length :         u16,
    pub port :           xcb_xv_port_t,
    pub drawable :       ffi::xproto::xcb_drawable_t,
    pub gc :             ffi::xproto::xcb_gcontext_t,
    pub vid_x :          i16,
    pub vid_y :          i16,
    pub vid_w :          u16,
    pub vid_h :          u16,
    pub drw_x :          i16,
    pub drw_y :          i16,
    pub drw_w :          u16,
    pub drw_h :          u16
}

impl Copy for xcb_xv_get_video_request_t {}
impl Clone for xcb_xv_get_video_request_t {
    fn clone(&self) -> xcb_xv_get_video_request_t { *self }
}


#[repr(C)]
pub struct xcb_xv_get_still_request_t {
    pub major_opcode :   u8,
    pub minor_opcode :   u8,
    pub length :         u16,
    pub port :           xcb_xv_port_t,
    pub drawable :       ffi::xproto::xcb_drawable_t,
    pub gc :             ffi::xproto::xcb_gcontext_t,
    pub vid_x :          i16,
    pub vid_y :          i16,
    pub vid_w :          u16,
    pub vid_h :          u16,
    pub drw_x :          i16,
    pub drw_y :          i16,
    pub drw_w :          u16,
    pub drw_h :          u16
}

impl Copy for xcb_xv_get_still_request_t {}
impl Clone for xcb_xv_get_still_request_t {
    fn clone(&self) -> xcb_xv_get_still_request_t { *self }
}


#[repr(C)]
pub struct xcb_xv_stop_video_request_t {
    pub major_opcode :   u8,
    pub minor_opcode :   u8,
    pub length :         u16,
    pub port :           xcb_xv_port_t,
    pub drawable :       ffi::xproto::xcb_drawable_t
}

impl Copy for xcb_xv_stop_video_request_t {}
impl Clone for xcb_xv_stop_video_request_t {
    fn clone(&self) -> xcb_xv_stop_video_request_t { *self }
}


#[repr(C)]
pub struct xcb_xv_select_video_notify_request_t {
    pub major_opcode :   u8,
    pub minor_opcode :   u8,
    pub length :         u16,
    pub drawable :       ffi::xproto::xcb_drawable_t,
    pub onoff :          u8,
    pub pad0 :           [u8; 3]
}

impl Copy for xcb_xv_select_video_notify_request_t {}
impl Clone for xcb_xv_select_video_notify_request_t {
    fn clone(&self) -> xcb_xv_select_video_notify_request_t { *self }
}


#[repr(C)]
pub struct xcb_xv_select_port_notify_request_t {
    pub major_opcode :   u8,
    pub minor_opcode :   u8,
    pub length :         u16,
    pub port :           xcb_xv_port_t,
    pub onoff :          u8,
    pub pad0 :           [u8; 3]
}

impl Copy for xcb_xv_select_port_notify_request_t {}
impl Clone for xcb_xv_select_port_notify_request_t {
    fn clone(&self) -> xcb_xv_select_port_notify_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xv_query_best_size_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_xv_query_best_size_request_t {
    pub major_opcode :   u8,
    pub minor_opcode :   u8,
    pub length :         u16,
    pub port :           xcb_xv_port_t,
    pub vid_w :          u16,
    pub vid_h :          u16,
    pub drw_w :          u16,
    pub drw_h :          u16,
    pub motion :         u8,
    pub pad0 :           [u8; 3]
}

impl Copy for xcb_xv_query_best_size_request_t {}
impl Clone for xcb_xv_query_best_size_request_t {
    fn clone(&self) -> xcb_xv_query_best_size_request_t { *self }
}

#[repr(C)]
pub struct xcb_xv_query_best_size_reply_t {
    pub response_type :   u8,
    pub pad0 :            u8,
    pub sequence :        u16,
    pub length :          u32,
    pub actual_width :    u16,
    pub actual_height :   u16
}

impl Copy for xcb_xv_query_best_size_reply_t {}
impl Clone for xcb_xv_query_best_size_reply_t {
    fn clone(&self) -> xcb_xv_query_best_size_reply_t { *self }
}


#[repr(C)]
pub struct xcb_xv_set_port_attribute_request_t {
    pub major_opcode :   u8,
    pub minor_opcode :   u8,
    pub length :         u16,
    pub port :           xcb_xv_port_t,
    pub attribute :      ffi::xproto::xcb_atom_t,
    pub value :          i32
}

impl Copy for xcb_xv_set_port_attribute_request_t {}
impl Clone for xcb_xv_set_port_attribute_request_t {
    fn clone(&self) -> xcb_xv_set_port_attribute_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xv_get_port_attribute_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_xv_get_port_attribute_request_t {
    pub major_opcode :   u8,
    pub minor_opcode :   u8,
    pub length :         u16,
    pub port :           xcb_xv_port_t,
    pub attribute :      ffi::xproto::xcb_atom_t
}

impl Copy for xcb_xv_get_port_attribute_request_t {}
impl Clone for xcb_xv_get_port_attribute_request_t {
    fn clone(&self) -> xcb_xv_get_port_attribute_request_t { *self }
}

#[repr(C)]
pub struct xcb_xv_get_port_attribute_reply_t {
    pub response_type :   u8,
    pub pad0 :            u8,
    pub sequence :        u16,
    pub length :          u32,
    pub value :           i32
}

impl Copy for xcb_xv_get_port_attribute_reply_t {}
impl Clone for xcb_xv_get_port_attribute_reply_t {
    fn clone(&self) -> xcb_xv_get_port_attribute_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xv_query_port_attributes_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_xv_query_port_attributes_request_t {
    pub major_opcode :   u8,
    pub minor_opcode :   u8,
    pub length :         u16,
    pub port :           xcb_xv_port_t
}

impl Copy for xcb_xv_query_port_attributes_request_t {}
impl Clone for xcb_xv_query_port_attributes_request_t {
    fn clone(&self) -> xcb_xv_query_port_attributes_request_t { *self }
}

#[repr(C)]
pub struct xcb_xv_query_port_attributes_reply_t {
    pub response_type :    u8,
    pub pad0 :             u8,
    pub sequence :         u16,
    pub length :           u32,
    pub num_attributes :   u32,
    pub text_size :        u32,
    pub pad1 :             [u8; 16]
}

impl Copy for xcb_xv_query_port_attributes_reply_t {}
impl Clone for xcb_xv_query_port_attributes_reply_t {
    fn clone(&self) -> xcb_xv_query_port_attributes_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xv_list_image_formats_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_xv_list_image_formats_request_t {
    pub major_opcode :   u8,
    pub minor_opcode :   u8,
    pub length :         u16,
    pub port :           xcb_xv_port_t
}

impl Copy for xcb_xv_list_image_formats_request_t {}
impl Clone for xcb_xv_list_image_formats_request_t {
    fn clone(&self) -> xcb_xv_list_image_formats_request_t { *self }
}

#[repr(C)]
pub struct xcb_xv_list_image_formats_reply_t {
    pub response_type :   u8,
    pub pad0 :            u8,
    pub sequence :        u16,
    pub length :          u32,
    pub num_formats :     u32,
    pub pad1 :            [u8; 20]
}

impl Copy for xcb_xv_list_image_formats_reply_t {}
impl Clone for xcb_xv_list_image_formats_reply_t {
    fn clone(&self) -> xcb_xv_list_image_formats_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xv_query_image_attributes_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_xv_query_image_attributes_request_t {
    pub major_opcode :   u8,
    pub minor_opcode :   u8,
    pub length :         u16,
    pub port :           xcb_xv_port_t,
    pub id :             u32,
    pub width :          u16,
    pub height :         u16
}

impl Copy for xcb_xv_query_image_attributes_request_t {}
impl Clone for xcb_xv_query_image_attributes_request_t {
    fn clone(&self) -> xcb_xv_query_image_attributes_request_t { *self }
}

#[repr(C)]
pub struct xcb_xv_query_image_attributes_reply_t {
    pub response_type :   u8,
    pub pad0 :            u8,
    pub sequence :        u16,
    pub length :          u32,
    pub num_planes :      u32,
    pub data_size :       u32,
    pub width :           u16,
    pub height :          u16,
    pub pad1 :            [u8; 12]
}

impl Copy for xcb_xv_query_image_attributes_reply_t {}
impl Clone for xcb_xv_query_image_attributes_reply_t {
    fn clone(&self) -> xcb_xv_query_image_attributes_reply_t { *self }
}


#[repr(C)]
pub struct xcb_xv_put_image_request_t {
    pub major_opcode :   u8,
    pub minor_opcode :   u8,
    pub length :         u16,
    pub port :           xcb_xv_port_t,
    pub drawable :       ffi::xproto::xcb_drawable_t,
    pub gc :             ffi::xproto::xcb_gcontext_t,
    pub id :             u32,
    pub src_x :          i16,
    pub src_y :          i16,
    pub src_w :          u16,
    pub src_h :          u16,
    pub drw_x :          i16,
    pub drw_y :          i16,
    pub drw_w :          u16,
    pub drw_h :          u16,
    pub width :          u16,
    pub height :         u16
}

impl Copy for xcb_xv_put_image_request_t {}
impl Clone for xcb_xv_put_image_request_t {
    fn clone(&self) -> xcb_xv_put_image_request_t { *self }
}


#[repr(C)]
pub struct xcb_xv_shm_put_image_request_t {
    pub major_opcode :   u8,
    pub minor_opcode :   u8,
    pub length :         u16,
    pub port :           xcb_xv_port_t,
    pub drawable :       ffi::xproto::xcb_drawable_t,
    pub gc :             ffi::xproto::xcb_gcontext_t,
    pub shmseg :         ffi::shm::xcb_shm_seg_t,
    pub id :             u32,
    pub offset :         u32,
    pub src_x :          i16,
    pub src_y :          i16,
    pub src_w :          u16,
    pub src_h :          u16,
    pub drw_x :          i16,
    pub drw_y :          i16,
    pub drw_w :          u16,
    pub drw_h :          u16,
    pub width :          u16,
    pub height :         u16,
    pub send_event :     u8,
    pub pad0 :           [u8; 3]
}

impl Copy for xcb_xv_shm_put_image_request_t {}
impl Clone for xcb_xv_shm_put_image_request_t {
    fn clone(&self) -> xcb_xv_shm_put_image_request_t { *self }
}
#[link(name="xcb-xv")]
extern "C" {

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xv_port_t)
///
pub fn xcb_xv_port_next (i:*mut xcb_xv_port_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xv_port_end (i:xcb_xv_port_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xv_encoding_t)
///
pub fn xcb_xv_encoding_next (i:*mut xcb_xv_encoding_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xv_encoding_end (i:xcb_xv_encoding_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xv_rational_t)
///
pub fn xcb_xv_rational_next (i:*mut xcb_xv_rational_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xv_rational_end (i:xcb_xv_rational_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xv_format_t)
///
pub fn xcb_xv_format_next (i:*mut xcb_xv_format_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xv_format_end (i:xcb_xv_format_iterator_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_xv_adaptor_info_sizeof (_buffer :  *mut c_void) -> c_int;

pub fn xcb_xv_adaptor_info_name (R : *mut xcb_xv_adaptor_info_t) -> *mut c_char;


pub fn xcb_xv_adaptor_info_name_length (R : *mut xcb_xv_adaptor_info_t) -> c_int;


pub fn xcb_xv_adaptor_info_name_end (R : *mut xcb_xv_adaptor_info_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_xv_adaptor_info_formats (R : *mut xcb_xv_adaptor_info_t) -> *mut xcb_xv_format_t;


pub fn xcb_xv_adaptor_info_formats_length (R : *mut xcb_xv_adaptor_info_t) -> c_int;

pub fn xcb_xv_adaptor_info_formats_iterator (R : *mut xcb_xv_adaptor_info_t) -> xcb_xv_format_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xv_adaptor_info_t)
///
pub fn xcb_xv_adaptor_info_next (i:*mut xcb_xv_adaptor_info_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xv_adaptor_info_end (i:xcb_xv_adaptor_info_iterator_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_xv_encoding_info_sizeof (_buffer :  *mut c_void) -> c_int;

pub fn xcb_xv_encoding_info_name (R : *mut xcb_xv_encoding_info_t) -> *mut c_char;


pub fn xcb_xv_encoding_info_name_length (R : *mut xcb_xv_encoding_info_t) -> c_int;


pub fn xcb_xv_encoding_info_name_end (R : *mut xcb_xv_encoding_info_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xv_encoding_info_t)
///
pub fn xcb_xv_encoding_info_next (i:*mut xcb_xv_encoding_info_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xv_encoding_info_end (i:xcb_xv_encoding_info_iterator_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_xv_image_sizeof (_buffer :  *mut c_void) -> c_int;

pub fn xcb_xv_image_pitches (R : *mut xcb_xv_image_t) -> *mut u32;


pub fn xcb_xv_image_pitches_length (R : *mut xcb_xv_image_t) -> c_int;


pub fn xcb_xv_image_pitches_end (R : *mut xcb_xv_image_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_xv_image_offsets (R : *mut xcb_xv_image_t) -> *mut u32;


pub fn xcb_xv_image_offsets_length (R : *mut xcb_xv_image_t) -> c_int;


pub fn xcb_xv_image_offsets_end (R : *mut xcb_xv_image_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_xv_image_data (R : *mut xcb_xv_image_t) -> *mut u8;


pub fn xcb_xv_image_data_length (R : *mut xcb_xv_image_t) -> c_int;


pub fn xcb_xv_image_data_end (R : *mut xcb_xv_image_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xv_image_t)
///
pub fn xcb_xv_image_next (i:*mut xcb_xv_image_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xv_image_end (i:xcb_xv_image_iterator_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_xv_attribute_info_sizeof (_buffer :  *mut c_void) -> c_int;

pub fn xcb_xv_attribute_info_name (R : *mut xcb_xv_attribute_info_t) -> *mut c_char;


pub fn xcb_xv_attribute_info_name_length (R : *mut xcb_xv_attribute_info_t) -> c_int;


pub fn xcb_xv_attribute_info_name_end (R : *mut xcb_xv_attribute_info_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xv_attribute_info_t)
///
pub fn xcb_xv_attribute_info_next (i:*mut xcb_xv_attribute_info_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xv_attribute_info_end (i:xcb_xv_attribute_info_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xv_image_format_info_t)
///
pub fn xcb_xv_image_format_info_next (i:*mut xcb_xv_image_format_info_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xv_image_format_info_end (i:xcb_xv_image_format_info_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/// Delivers a request to the X server.
///
pub fn xcb_xv_query_extension (c : *mut ffi::base::xcb_connection_t) -> xcb_xv_query_extension_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_xv_query_extension_unchecked (c : *mut ffi::base::xcb_connection_t) -> xcb_xv_query_extension_cookie_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_xv_query_extension_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_xv_query_extension_reply (c : *mut ffi::base::xcb_connection_t,
                                        cookie : xcb_xv_query_extension_cookie_t,
                                        e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_xv_query_extension_reply_t;

pub fn xcb_xv_query_adaptors_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
///
pub fn xcb_xv_query_adaptors (c : *mut ffi::base::xcb_connection_t,
                                 window :  ffi::xproto::xcb_window_t) -> xcb_xv_query_adaptors_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_xv_query_adaptors_unchecked (c : *mut ffi::base::xcb_connection_t,
                                           window :  ffi::xproto::xcb_window_t) -> xcb_xv_query_adaptors_cookie_t;


pub fn xcb_xv_query_adaptors_info_length (R : *mut xcb_xv_query_adaptors_reply_t) -> c_int;

pub fn xcb_xv_query_adaptors_info_iterator (R : *mut xcb_xv_query_adaptors_reply_t) -> xcb_xv_adaptor_info_iterator_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_xv_query_adaptors_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_xv_query_adaptors_reply (c : *mut ffi::base::xcb_connection_t,
                                       cookie : xcb_xv_query_adaptors_cookie_t,
                                       e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_xv_query_adaptors_reply_t;

pub fn xcb_xv_query_encodings_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
///
pub fn xcb_xv_query_encodings (c : *mut ffi::base::xcb_connection_t,
                                  port :  xcb_xv_port_t) -> xcb_xv_query_encodings_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_xv_query_encodings_unchecked (c : *mut ffi::base::xcb_connection_t,
                                            port :  xcb_xv_port_t) -> xcb_xv_query_encodings_cookie_t;


pub fn xcb_xv_query_encodings_info_length (R : *mut xcb_xv_query_encodings_reply_t) -> c_int;

pub fn xcb_xv_query_encodings_info_iterator (R : *mut xcb_xv_query_encodings_reply_t) -> xcb_xv_encoding_info_iterator_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_xv_query_encodings_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_xv_query_encodings_reply (c : *mut ffi::base::xcb_connection_t,
                                        cookie : xcb_xv_query_encodings_cookie_t,
                                        e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_xv_query_encodings_reply_t;

/// Delivers a request to the X server.
///
pub fn xcb_xv_grab_port (c : *mut ffi::base::xcb_connection_t,
                            port :  xcb_xv_port_t,
                            time :  ffi::xproto::xcb_timestamp_t) -> xcb_xv_grab_port_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_xv_grab_port_unchecked (c : *mut ffi::base::xcb_connection_t,
                                      port :  xcb_xv_port_t,
                                      time :  ffi::xproto::xcb_timestamp_t) -> xcb_xv_grab_port_cookie_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_xv_grab_port_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_xv_grab_port_reply (c : *mut ffi::base::xcb_connection_t,
                                  cookie : xcb_xv_grab_port_cookie_t,
                                  e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_xv_grab_port_reply_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_xv_ungrab_port_checked (c : *mut ffi::base::xcb_connection_t,
                                      port :  xcb_xv_port_t,
                                      time :  ffi::xproto::xcb_timestamp_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_xv_ungrab_port (c : *mut ffi::base::xcb_connection_t,
                              port :  xcb_xv_port_t,
                              time :  ffi::xproto::xcb_timestamp_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_xv_put_video_checked (c : *mut ffi::base::xcb_connection_t,
                                    port :  xcb_xv_port_t,
                                    drawable :  ffi::xproto::xcb_drawable_t,
                                    gc :  ffi::xproto::xcb_gcontext_t,
                                    vid_x :  i16,
                                    vid_y :  i16,
                                    vid_w :  u16,
                                    vid_h :  u16,
                                    drw_x :  i16,
                                    drw_y :  i16,
                                    drw_w :  u16,
                                    drw_h :  u16) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_xv_put_video (c : *mut ffi::base::xcb_connection_t,
                            port :  xcb_xv_port_t,
                            drawable :  ffi::xproto::xcb_drawable_t,
                            gc :  ffi::xproto::xcb_gcontext_t,
                            vid_x :  i16,
                            vid_y :  i16,
                            vid_w :  u16,
                            vid_h :  u16,
                            drw_x :  i16,
                            drw_y :  i16,
                            drw_w :  u16,
                            drw_h :  u16) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_xv_put_still_checked (c : *mut ffi::base::xcb_connection_t,
                                    port :  xcb_xv_port_t,
                                    drawable :  ffi::xproto::xcb_drawable_t,
                                    gc :  ffi::xproto::xcb_gcontext_t,
                                    vid_x :  i16,
                                    vid_y :  i16,
                                    vid_w :  u16,
                                    vid_h :  u16,
                                    drw_x :  i16,
                                    drw_y :  i16,
                                    drw_w :  u16,
                                    drw_h :  u16) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_xv_put_still (c : *mut ffi::base::xcb_connection_t,
                            port :  xcb_xv_port_t,
                            drawable :  ffi::xproto::xcb_drawable_t,
                            gc :  ffi::xproto::xcb_gcontext_t,
                            vid_x :  i16,
                            vid_y :  i16,
                            vid_w :  u16,
                            vid_h :  u16,
                            drw_x :  i16,
                            drw_y :  i16,
                            drw_w :  u16,
                            drw_h :  u16) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_xv_get_video_checked (c : *mut ffi::base::xcb_connection_t,
                                    port :  xcb_xv_port_t,
                                    drawable :  ffi::xproto::xcb_drawable_t,
                                    gc :  ffi::xproto::xcb_gcontext_t,
                                    vid_x :  i16,
                                    vid_y :  i16,
                                    vid_w :  u16,
                                    vid_h :  u16,
                                    drw_x :  i16,
                                    drw_y :  i16,
                                    drw_w :  u16,
                                    drw_h :  u16) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_xv_get_video (c : *mut ffi::base::xcb_connection_t,
                            port :  xcb_xv_port_t,
                            drawable :  ffi::xproto::xcb_drawable_t,
                            gc :  ffi::xproto::xcb_gcontext_t,
                            vid_x :  i16,
                            vid_y :  i16,
                            vid_w :  u16,
                            vid_h :  u16,
                            drw_x :  i16,
                            drw_y :  i16,
                            drw_w :  u16,
                            drw_h :  u16) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_xv_get_still_checked (c : *mut ffi::base::xcb_connection_t,
                                    port :  xcb_xv_port_t,
                                    drawable :  ffi::xproto::xcb_drawable_t,
                                    gc :  ffi::xproto::xcb_gcontext_t,
                                    vid_x :  i16,
                                    vid_y :  i16,
                                    vid_w :  u16,
                                    vid_h :  u16,
                                    drw_x :  i16,
                                    drw_y :  i16,
                                    drw_w :  u16,
                                    drw_h :  u16) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_xv_get_still (c : *mut ffi::base::xcb_connection_t,
                            port :  xcb_xv_port_t,
                            drawable :  ffi::xproto::xcb_drawable_t,
                            gc :  ffi::xproto::xcb_gcontext_t,
                            vid_x :  i16,
                            vid_y :  i16,
                            vid_w :  u16,
                            vid_h :  u16,
                            drw_x :  i16,
                            drw_y :  i16,
                            drw_w :  u16,
                            drw_h :  u16) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_xv_stop_video_checked (c : *mut ffi::base::xcb_connection_t,
                                     port :  xcb_xv_port_t,
                                     drawable :  ffi::xproto::xcb_drawable_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_xv_stop_video (c : *mut ffi::base::xcb_connection_t,
                             port :  xcb_xv_port_t,
                             drawable :  ffi::xproto::xcb_drawable_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_xv_select_video_notify_checked (c : *mut ffi::base::xcb_connection_t,
                                              drawable :  ffi::xproto::xcb_drawable_t,
                                              onoff :  u8) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_xv_select_video_notify (c : *mut ffi::base::xcb_connection_t,
                                      drawable :  ffi::xproto::xcb_drawable_t,
                                      onoff :  u8) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_xv_select_port_notify_checked (c : *mut ffi::base::xcb_connection_t,
                                             port :  xcb_xv_port_t,
                                             onoff :  u8) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_xv_select_port_notify (c : *mut ffi::base::xcb_connection_t,
                                     port :  xcb_xv_port_t,
                                     onoff :  u8) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_xv_query_best_size (c : *mut ffi::base::xcb_connection_t,
                                  port :  xcb_xv_port_t,
                                  vid_w :  u16,
                                  vid_h :  u16,
                                  drw_w :  u16,
                                  drw_h :  u16,
                                  motion :  u8) -> xcb_xv_query_best_size_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_xv_query_best_size_unchecked (c : *mut ffi::base::xcb_connection_t,
                                            port :  xcb_xv_port_t,
                                            vid_w :  u16,
                                            vid_h :  u16,
                                            drw_w :  u16,
                                            drw_h :  u16,
                                            motion :  u8) -> xcb_xv_query_best_size_cookie_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_xv_query_best_size_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_xv_query_best_size_reply (c : *mut ffi::base::xcb_connection_t,
                                        cookie : xcb_xv_query_best_size_cookie_t,
                                        e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_xv_query_best_size_reply_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_xv_set_port_attribute_checked (c : *mut ffi::base::xcb_connection_t,
                                             port :  xcb_xv_port_t,
                                             attribute :  ffi::xproto::xcb_atom_t,
                                             value :  i32) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_xv_set_port_attribute (c : *mut ffi::base::xcb_connection_t,
                                     port :  xcb_xv_port_t,
                                     attribute :  ffi::xproto::xcb_atom_t,
                                     value :  i32) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_xv_get_port_attribute (c : *mut ffi::base::xcb_connection_t,
                                     port :  xcb_xv_port_t,
                                     attribute :  ffi::xproto::xcb_atom_t) -> xcb_xv_get_port_attribute_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_xv_get_port_attribute_unchecked (c : *mut ffi::base::xcb_connection_t,
                                               port :  xcb_xv_port_t,
                                               attribute :  ffi::xproto::xcb_atom_t) -> xcb_xv_get_port_attribute_cookie_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_xv_get_port_attribute_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_xv_get_port_attribute_reply (c : *mut ffi::base::xcb_connection_t,
                                           cookie : xcb_xv_get_port_attribute_cookie_t,
                                           e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_xv_get_port_attribute_reply_t;

pub fn xcb_xv_query_port_attributes_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
///
pub fn xcb_xv_query_port_attributes (c : *mut ffi::base::xcb_connection_t,
                                        port :  xcb_xv_port_t) -> xcb_xv_query_port_attributes_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_xv_query_port_attributes_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                  port :  xcb_xv_port_t) -> xcb_xv_query_port_attributes_cookie_t;


pub fn xcb_xv_query_port_attributes_attributes_length (R : *mut xcb_xv_query_port_attributes_reply_t) -> c_int;

pub fn xcb_xv_query_port_attributes_attributes_iterator (R : *mut xcb_xv_query_port_attributes_reply_t) -> xcb_xv_attribute_info_iterator_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_xv_query_port_attributes_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_xv_query_port_attributes_reply (c : *mut ffi::base::xcb_connection_t,
                                              cookie : xcb_xv_query_port_attributes_cookie_t,
                                              e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_xv_query_port_attributes_reply_t;

pub fn xcb_xv_list_image_formats_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
///
pub fn xcb_xv_list_image_formats (c : *mut ffi::base::xcb_connection_t,
                                     port :  xcb_xv_port_t) -> xcb_xv_list_image_formats_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_xv_list_image_formats_unchecked (c : *mut ffi::base::xcb_connection_t,
                                               port :  xcb_xv_port_t) -> xcb_xv_list_image_formats_cookie_t;

pub fn xcb_xv_list_image_formats_format (R : *mut xcb_xv_list_image_formats_reply_t) -> *mut xcb_xv_image_format_info_t;


pub fn xcb_xv_list_image_formats_format_length (R : *mut xcb_xv_list_image_formats_reply_t) -> c_int;

pub fn xcb_xv_list_image_formats_format_iterator (R : *mut xcb_xv_list_image_formats_reply_t) -> xcb_xv_image_format_info_iterator_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_xv_list_image_formats_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_xv_list_image_formats_reply (c : *mut ffi::base::xcb_connection_t,
                                           cookie : xcb_xv_list_image_formats_cookie_t,
                                           e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_xv_list_image_formats_reply_t;

pub fn xcb_xv_query_image_attributes_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
///
pub fn xcb_xv_query_image_attributes (c : *mut ffi::base::xcb_connection_t,
                                         port :  xcb_xv_port_t,
                                         id :  u32,
                                         width :  u16,
                                         height :  u16) -> xcb_xv_query_image_attributes_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_xv_query_image_attributes_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                   port :  xcb_xv_port_t,
                                                   id :  u32,
                                                   width :  u16,
                                                   height :  u16) -> xcb_xv_query_image_attributes_cookie_t;

pub fn xcb_xv_query_image_attributes_pitches (R : *mut xcb_xv_query_image_attributes_reply_t) -> *mut u32;


pub fn xcb_xv_query_image_attributes_pitches_length (R : *mut xcb_xv_query_image_attributes_reply_t) -> c_int;


pub fn xcb_xv_query_image_attributes_pitches_end (R : *mut xcb_xv_query_image_attributes_reply_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_xv_query_image_attributes_offsets (R : *mut xcb_xv_query_image_attributes_reply_t) -> *mut u32;


pub fn xcb_xv_query_image_attributes_offsets_length (R : *mut xcb_xv_query_image_attributes_reply_t) -> c_int;


pub fn xcb_xv_query_image_attributes_offsets_end (R : *mut xcb_xv_query_image_attributes_reply_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_xv_query_image_attributes_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_xv_query_image_attributes_reply (c : *mut ffi::base::xcb_connection_t,
                                               cookie : xcb_xv_query_image_attributes_cookie_t,
                                               e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_xv_query_image_attributes_reply_t;

pub fn xcb_xv_put_image_sizeof (_buffer :  *mut c_void,
                         data_len :     u32) -> c_int;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_xv_put_image_checked (c : *mut ffi::base::xcb_connection_t,
                                    port :  xcb_xv_port_t,
                                    drawable :  ffi::xproto::xcb_drawable_t,
                                    gc :  ffi::xproto::xcb_gcontext_t,
                                    id :  u32,
                                    src_x :  i16,
                                    src_y :  i16,
                                    src_w :  u16,
                                    src_h :  u16,
                                    drw_x :  i16,
                                    drw_y :  i16,
                                    drw_w :  u16,
                                    drw_h :  u16,
                                    width :  u16,
                                    height :  u16,
                                    data_len :  u32,
                                    data : *mut u8) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_xv_put_image (c : *mut ffi::base::xcb_connection_t,
                            port :  xcb_xv_port_t,
                            drawable :  ffi::xproto::xcb_drawable_t,
                            gc :  ffi::xproto::xcb_gcontext_t,
                            id :  u32,
                            src_x :  i16,
                            src_y :  i16,
                            src_w :  u16,
                            src_h :  u16,
                            drw_x :  i16,
                            drw_y :  i16,
                            drw_w :  u16,
                            drw_h :  u16,
                            width :  u16,
                            height :  u16,
                            data_len :  u32,
                            data : *mut u8) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_xv_shm_put_image_checked (c : *mut ffi::base::xcb_connection_t,
                                        port :  xcb_xv_port_t,
                                        drawable :  ffi::xproto::xcb_drawable_t,
                                        gc :  ffi::xproto::xcb_gcontext_t,
                                        shmseg :  ffi::shm::xcb_shm_seg_t,
                                        id :  u32,
                                        offset :  u32,
                                        src_x :  i16,
                                        src_y :  i16,
                                        src_w :  u16,
                                        src_h :  u16,
                                        drw_x :  i16,
                                        drw_y :  i16,
                                        drw_w :  u16,
                                        drw_h :  u16,
                                        width :  u16,
                                        height :  u16,
                                        send_event :  u8) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_xv_shm_put_image (c : *mut ffi::base::xcb_connection_t,
                                port :  xcb_xv_port_t,
                                drawable :  ffi::xproto::xcb_drawable_t,
                                gc :  ffi::xproto::xcb_gcontext_t,
                                shmseg :  ffi::shm::xcb_shm_seg_t,
                                id :  u32,
                                offset :  u32,
                                src_x :  i16,
                                src_y :  i16,
                                src_w :  u16,
                                src_h :  u16,
                                drw_x :  i16,
                                drw_y :  i16,
                                drw_w :  u16,
                                drw_h :  u16,
                                width :  u16,
                                height :  u16,
                                send_event :  u8) -> ffi::base::xcb_void_cookie_t;
}

