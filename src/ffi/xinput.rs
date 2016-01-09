/*
 * This file generated automatically from xinput.xml by r_client.py.
 * Edit at your peril.
 */

//Make the compiler quiet
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use std;
use libc::*;
use ffi;
use ffi::xproto;

pub const INPUT_MAJOR_VERSION : c_uint = 1;
pub const INPUT_MINOR_VERSION : c_uint = 4;

pub type xcb_input_key_code_t = u8;
/**
 * @brief xcb_input_key_code_iterator_t
 **/
#[repr(C)]
pub struct xcb_input_key_code_iterator_t {
    pub data : *mut xcb_input_key_code_t,
    pub rem  : c_int,
    pub index: c_int
}


pub type xcb_input_event_class_t = u32;
/**
 * @brief xcb_input_event_class_iterator_t
 **/
#[repr(C)]
pub struct xcb_input_event_class_iterator_t {
    pub data : *mut xcb_input_event_class_t,
    pub rem  : c_int,
    pub index: c_int
}


#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_input_get_extension_version_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_input_get_extension_version_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub name_len :       u16,
     pub pad0 :           [u8; 2]
}

impl Copy for xcb_input_get_extension_version_request_t {}
impl Clone for xcb_input_get_extension_version_request_t {
    fn clone(&self) -> xcb_input_get_extension_version_request_t { *self }
}

#[repr(C)]
pub struct xcb_input_get_extension_version_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub server_major :    u16,
     pub server_minor :    u16,
     pub present :         u8,
     pub pad1 :            [u8; 19]
}

impl Copy for xcb_input_get_extension_version_reply_t {}
impl Clone for xcb_input_get_extension_version_reply_t {
    fn clone(&self) -> xcb_input_get_extension_version_reply_t { *self }
}

#[repr(C)]
pub struct xcb_input_device_info_t {
     pub device_type :      ffi::xproto::xcb_atom_t,
     pub device_id :        u8,
     pub num_class_info :   u8,
     pub device_use :       u8,
     pub pad0 :             u8
}

impl Copy for xcb_input_device_info_t {}
impl Clone for xcb_input_device_info_t {
    fn clone(&self) -> xcb_input_device_info_t { *self }
}
/**
 * @brief xcb_input_device_info_iterator_t
 **/
#[repr(C)]
pub struct xcb_input_device_info_iterator_t {
    pub data : *mut xcb_input_device_info_t,
    pub rem  : c_int,
    pub index: c_int
}


#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_input_list_input_devices_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_input_list_input_devices_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16
}

impl Copy for xcb_input_list_input_devices_request_t {}
impl Clone for xcb_input_list_input_devices_request_t {
    fn clone(&self) -> xcb_input_list_input_devices_request_t { *self }
}

#[repr(C)]
pub struct xcb_input_list_input_devices_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub devices_len :     u8,
     pub pad1 :            [u8; 23]
}

impl Copy for xcb_input_list_input_devices_reply_t {}
impl Clone for xcb_input_list_input_devices_reply_t {
    fn clone(&self) -> xcb_input_list_input_devices_reply_t { *self }
}

#[repr(C)]
pub struct xcb_input_input_info_t {
     pub class_id :   u8,
     pub len :        u8
}

impl Copy for xcb_input_input_info_t {}
impl Clone for xcb_input_input_info_t {
    fn clone(&self) -> xcb_input_input_info_t { *self }
}
/**
 * @brief xcb_input_input_info_iterator_t
 **/
#[repr(C)]
pub struct xcb_input_input_info_iterator_t {
    pub data : *mut xcb_input_input_info_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_input_key_info_t {
     pub class_id :      u8,
     pub len :           u8,
     pub min_keycode :   xcb_input_key_code_t,
     pub max_keycode :   xcb_input_key_code_t,
     pub num_keys :      u16,
     pub pad0 :          [u8; 2]
}

impl Copy for xcb_input_key_info_t {}
impl Clone for xcb_input_key_info_t {
    fn clone(&self) -> xcb_input_key_info_t { *self }
}
/**
 * @brief xcb_input_key_info_iterator_t
 **/
#[repr(C)]
pub struct xcb_input_key_info_iterator_t {
    pub data : *mut xcb_input_key_info_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_input_button_info_t {
     pub class_id :      u8,
     pub len :           u8,
     pub num_buttons :   u16
}

impl Copy for xcb_input_button_info_t {}
impl Clone for xcb_input_button_info_t {
    fn clone(&self) -> xcb_input_button_info_t { *self }
}
/**
 * @brief xcb_input_button_info_iterator_t
 **/
#[repr(C)]
pub struct xcb_input_button_info_iterator_t {
    pub data : *mut xcb_input_button_info_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_input_axis_info_t {
     pub resolution :   u32,
     pub minimum :      i32,
     pub maximum :      i32
}

impl Copy for xcb_input_axis_info_t {}
impl Clone for xcb_input_axis_info_t {
    fn clone(&self) -> xcb_input_axis_info_t { *self }
}
/**
 * @brief xcb_input_axis_info_iterator_t
 **/
#[repr(C)]
pub struct xcb_input_axis_info_iterator_t {
    pub data : *mut xcb_input_axis_info_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_input_valuator_info_t {
     pub class_id :      u8,
     pub len :           u8,
     pub axes_len :      u8,
     pub mode :          u8,
     pub motion_size :   u32
}

impl Copy for xcb_input_valuator_info_t {}
impl Clone for xcb_input_valuator_info_t {
    fn clone(&self) -> xcb_input_valuator_info_t { *self }
}
/**
 * @brief xcb_input_valuator_info_iterator_t
 **/
#[repr(C)]
pub struct xcb_input_valuator_info_iterator_t {
    pub data : *mut xcb_input_valuator_info_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_input_input_class_info_t {
     pub class_id :          u8,
     pub event_type_base :   u8
}

impl Copy for xcb_input_input_class_info_t {}
impl Clone for xcb_input_input_class_info_t {
    fn clone(&self) -> xcb_input_input_class_info_t { *self }
}
/**
 * @brief xcb_input_input_class_info_iterator_t
 **/
#[repr(C)]
pub struct xcb_input_input_class_info_iterator_t {
    pub data : *mut xcb_input_input_class_info_t,
    pub rem  : c_int,
    pub index: c_int
}


#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_input_open_device_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_input_open_device_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub device_id :      u8,
     pub pad0 :           [u8; 3]
}

impl Copy for xcb_input_open_device_request_t {}
impl Clone for xcb_input_open_device_request_t {
    fn clone(&self) -> xcb_input_open_device_request_t { *self }
}

#[repr(C)]
pub struct xcb_input_open_device_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub num_classes :     u8,
     pub pad1 :            [u8; 23]
}

impl Copy for xcb_input_open_device_reply_t {}
impl Clone for xcb_input_open_device_reply_t {
    fn clone(&self) -> xcb_input_open_device_reply_t { *self }
}


#[repr(C)]
pub struct xcb_input_close_device_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub device_id :      u8,
     pub pad0 :           [u8; 3]
}

impl Copy for xcb_input_close_device_request_t {}
impl Clone for xcb_input_close_device_request_t {
    fn clone(&self) -> xcb_input_close_device_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_input_set_device_mode_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_input_set_device_mode_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub device_id :      u8,
     pub mode :           u8,
     pub pad0 :           [u8; 2]
}

impl Copy for xcb_input_set_device_mode_request_t {}
impl Clone for xcb_input_set_device_mode_request_t {
    fn clone(&self) -> xcb_input_set_device_mode_request_t { *self }
}

#[repr(C)]
pub struct xcb_input_set_device_mode_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub status :          u8,
     pub pad1 :            [u8; 23]
}

impl Copy for xcb_input_set_device_mode_reply_t {}
impl Clone for xcb_input_set_device_mode_reply_t {
    fn clone(&self) -> xcb_input_set_device_mode_reply_t { *self }
}


#[repr(C)]
pub struct xcb_input_select_extension_event_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub window :         ffi::xproto::xcb_window_t,
     pub num_classes :    u16,
     pub pad0 :           [u8; 2]
}

impl Copy for xcb_input_select_extension_event_request_t {}
impl Clone for xcb_input_select_extension_event_request_t {
    fn clone(&self) -> xcb_input_select_extension_event_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_input_get_selected_extension_events_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_input_get_selected_extension_events_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub window :         ffi::xproto::xcb_window_t
}

impl Copy for xcb_input_get_selected_extension_events_request_t {}
impl Clone for xcb_input_get_selected_extension_events_request_t {
    fn clone(&self) -> xcb_input_get_selected_extension_events_request_t { *self }
}

#[repr(C)]
pub struct xcb_input_get_selected_extension_events_reply_t {
     pub response_type :      u8,
     pub pad0 :               u8,
     pub sequence :           u16,
     pub length :             u32,
     pub num_this_classes :   u16,
     pub num_all_classes :    u16,
     pub pad1 :               [u8; 20]
}

impl Copy for xcb_input_get_selected_extension_events_reply_t {}
impl Clone for xcb_input_get_selected_extension_events_reply_t {
    fn clone(&self) -> xcb_input_get_selected_extension_events_reply_t { *self }
}


#[repr(C)]
pub struct xcb_input_change_device_dont_propagate_list_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub window :         ffi::xproto::xcb_window_t,
     pub num_classes :    u16,
     pub mode :           u8,
     pub pad0 :           u8
}

impl Copy for xcb_input_change_device_dont_propagate_list_request_t {}
impl Clone for xcb_input_change_device_dont_propagate_list_request_t {
    fn clone(&self) -> xcb_input_change_device_dont_propagate_list_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_input_get_device_dont_propagate_list_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_input_get_device_dont_propagate_list_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub window :         ffi::xproto::xcb_window_t
}

impl Copy for xcb_input_get_device_dont_propagate_list_request_t {}
impl Clone for xcb_input_get_device_dont_propagate_list_request_t {
    fn clone(&self) -> xcb_input_get_device_dont_propagate_list_request_t { *self }
}

#[repr(C)]
pub struct xcb_input_get_device_dont_propagate_list_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub num_classes :     u16,
     pub pad1 :            [u8; 22]
}

impl Copy for xcb_input_get_device_dont_propagate_list_reply_t {}
impl Clone for xcb_input_get_device_dont_propagate_list_reply_t {
    fn clone(&self) -> xcb_input_get_device_dont_propagate_list_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_input_get_device_motion_events_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_input_get_device_motion_events_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub start :          ffi::xproto::xcb_timestamp_t,
     pub stop :           ffi::xproto::xcb_timestamp_t,
     pub device_id :      u8
}

impl Copy for xcb_input_get_device_motion_events_request_t {}
impl Clone for xcb_input_get_device_motion_events_request_t {
    fn clone(&self) -> xcb_input_get_device_motion_events_request_t { *self }
}

#[repr(C)]
pub struct xcb_input_get_device_motion_events_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub num_coords :      u32,
     pub num_axes :        u8,
     pub device_mode :     u8,
     pub pad1 :            [u8; 18]
}

impl Copy for xcb_input_get_device_motion_events_reply_t {}
impl Clone for xcb_input_get_device_motion_events_reply_t {
    fn clone(&self) -> xcb_input_get_device_motion_events_reply_t { *self }
}

#[repr(C)]
pub struct xcb_input_device_time_coord_t {
     pub time :   ffi::xproto::xcb_timestamp_t
}

impl Copy for xcb_input_device_time_coord_t {}
impl Clone for xcb_input_device_time_coord_t {
    fn clone(&self) -> xcb_input_device_time_coord_t { *self }
}
/**
 * @brief xcb_input_device_time_coord_iterator_t
 **/
#[repr(C)]
pub struct xcb_input_device_time_coord_iterator_t {
    pub data : *mut xcb_input_device_time_coord_t,
    pub rem  : c_int,
    pub index: c_int
}


#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_input_change_keyboard_device_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_input_change_keyboard_device_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub device_id :      u8,
     pub pad0 :           [u8; 3]
}

impl Copy for xcb_input_change_keyboard_device_request_t {}
impl Clone for xcb_input_change_keyboard_device_request_t {
    fn clone(&self) -> xcb_input_change_keyboard_device_request_t { *self }
}

#[repr(C)]
pub struct xcb_input_change_keyboard_device_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub status :          u8,
     pub pad1 :            [u8; 23]
}

impl Copy for xcb_input_change_keyboard_device_reply_t {}
impl Clone for xcb_input_change_keyboard_device_reply_t {
    fn clone(&self) -> xcb_input_change_keyboard_device_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_input_change_pointer_device_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_input_change_pointer_device_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub x_axis :         u8,
     pub y_axis :         u8,
     pub device_id :      u8,
     pub pad0 :           u8
}

impl Copy for xcb_input_change_pointer_device_request_t {}
impl Clone for xcb_input_change_pointer_device_request_t {
    fn clone(&self) -> xcb_input_change_pointer_device_request_t { *self }
}

#[repr(C)]
pub struct xcb_input_change_pointer_device_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub status :          u8,
     pub pad1 :            [u8; 23]
}

impl Copy for xcb_input_change_pointer_device_reply_t {}
impl Clone for xcb_input_change_pointer_device_reply_t {
    fn clone(&self) -> xcb_input_change_pointer_device_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_input_grab_device_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_input_grab_device_request_t {
     pub major_opcode :        u8,
     pub minor_opcode :        u8,
     pub length :              u16,
     pub grab_window :         ffi::xproto::xcb_window_t,
     pub time :                ffi::xproto::xcb_timestamp_t,
     pub num_classes :         u16,
     pub this_device_mode :    u8,
     pub other_device_mode :   u8,
     pub owner_events :        u8,
     pub device_id :           u8,
     pub pad0 :                [u8; 2]
}

impl Copy for xcb_input_grab_device_request_t {}
impl Clone for xcb_input_grab_device_request_t {
    fn clone(&self) -> xcb_input_grab_device_request_t { *self }
}

#[repr(C)]
pub struct xcb_input_grab_device_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub status :          u8,
     pub pad1 :            [u8; 23]
}

impl Copy for xcb_input_grab_device_reply_t {}
impl Clone for xcb_input_grab_device_reply_t {
    fn clone(&self) -> xcb_input_grab_device_reply_t { *self }
}


#[repr(C)]
pub struct xcb_input_ungrab_device_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub time :           ffi::xproto::xcb_timestamp_t,
     pub device_id :      u8
}

impl Copy for xcb_input_ungrab_device_request_t {}
impl Clone for xcb_input_ungrab_device_request_t {
    fn clone(&self) -> xcb_input_ungrab_device_request_t { *self }
}


#[repr(C)]
pub struct xcb_input_grab_device_key_request_t {
     pub major_opcode :        u8,
     pub minor_opcode :        u8,
     pub length :              u16,
     pub grab_window :         ffi::xproto::xcb_window_t,
     pub num_classes :         u16,
     pub modifiers :           u16,
     pub modifier_device :     u8,
     pub grabbed_device :      u8,
     pub key :                 u8,
     pub this_device_mode :    u8,
     pub other_device_mode :   u8,
     pub owner_events :        u8,
     pub pad0 :                [u8; 2]
}

impl Copy for xcb_input_grab_device_key_request_t {}
impl Clone for xcb_input_grab_device_key_request_t {
    fn clone(&self) -> xcb_input_grab_device_key_request_t { *self }
}


#[repr(C)]
pub struct xcb_input_ungrab_device_key_request_t {
     pub major_opcode :      u8,
     pub minor_opcode :      u8,
     pub length :            u16,
     pub grabWindow :        ffi::xproto::xcb_window_t,
     pub modifiers :         u16,
     pub modifier_device :   u8,
     pub key :               u8,
     pub grabbed_device :    u8
}

impl Copy for xcb_input_ungrab_device_key_request_t {}
impl Clone for xcb_input_ungrab_device_key_request_t {
    fn clone(&self) -> xcb_input_ungrab_device_key_request_t { *self }
}


#[repr(C)]
pub struct xcb_input_grab_device_button_request_t {
     pub major_opcode :        u8,
     pub minor_opcode :        u8,
     pub length :              u16,
     pub grab_window :         ffi::xproto::xcb_window_t,
     pub grabbed_device :      u8,
     pub modifier_device :     u8,
     pub num_classes :         u16,
     pub modifiers :           u16,
     pub this_device_mode :    u8,
     pub other_device_mode :   u8,
     pub button :              u8,
     pub owner_events :        u8,
     pub pad0 :                [u8; 2]
}

impl Copy for xcb_input_grab_device_button_request_t {}
impl Clone for xcb_input_grab_device_button_request_t {
    fn clone(&self) -> xcb_input_grab_device_button_request_t { *self }
}


#[repr(C)]
pub struct xcb_input_ungrab_device_button_request_t {
     pub major_opcode :      u8,
     pub minor_opcode :      u8,
     pub length :            u16,
     pub grab_window :       ffi::xproto::xcb_window_t,
     pub modifiers :         u16,
     pub modifier_device :   u8,
     pub button :            u8,
     pub grabbed_device :    u8
}

impl Copy for xcb_input_ungrab_device_button_request_t {}
impl Clone for xcb_input_ungrab_device_button_request_t {
    fn clone(&self) -> xcb_input_ungrab_device_button_request_t { *self }
}


#[repr(C)]
pub struct xcb_input_allow_device_events_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub time :           ffi::xproto::xcb_timestamp_t,
     pub mode :           u8,
     pub device_id :      u8
}

impl Copy for xcb_input_allow_device_events_request_t {}
impl Clone for xcb_input_allow_device_events_request_t {
    fn clone(&self) -> xcb_input_allow_device_events_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_input_get_device_focus_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_input_get_device_focus_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub device_id :      u8,
     pub pad0 :           [u8; 3]
}

impl Copy for xcb_input_get_device_focus_request_t {}
impl Clone for xcb_input_get_device_focus_request_t {
    fn clone(&self) -> xcb_input_get_device_focus_request_t { *self }
}

#[repr(C)]
pub struct xcb_input_get_device_focus_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub focus :           ffi::xproto::xcb_window_t,
     pub time :            ffi::xproto::xcb_timestamp_t,
     pub revert_to :       u8,
     pub pad1 :            [u8; 15]
}

impl Copy for xcb_input_get_device_focus_reply_t {}
impl Clone for xcb_input_get_device_focus_reply_t {
    fn clone(&self) -> xcb_input_get_device_focus_reply_t { *self }
}


#[repr(C)]
pub struct xcb_input_set_device_focus_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub focus :          ffi::xproto::xcb_window_t,
     pub time :           ffi::xproto::xcb_timestamp_t,
     pub revert_to :      u8,
     pub device_id :      u8
}

impl Copy for xcb_input_set_device_focus_request_t {}
impl Clone for xcb_input_set_device_focus_request_t {
    fn clone(&self) -> xcb_input_set_device_focus_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_input_get_feedback_control_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_input_get_feedback_control_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub device_id :      u8,
     pub pad0 :           [u8; 3]
}

impl Copy for xcb_input_get_feedback_control_request_t {}
impl Clone for xcb_input_get_feedback_control_request_t {
    fn clone(&self) -> xcb_input_get_feedback_control_request_t { *self }
}

#[repr(C)]
pub struct xcb_input_get_feedback_control_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub num_feedback :    u16,
     pub pad1 :            [u8; 22]
}

impl Copy for xcb_input_get_feedback_control_reply_t {}
impl Clone for xcb_input_get_feedback_control_reply_t {
    fn clone(&self) -> xcb_input_get_feedback_control_reply_t { *self }
}

#[repr(C)]
pub struct xcb_input_feedback_state_t {
     pub class_id :   u8,
     pub id :         u8,
     pub len :        u16
}

impl Copy for xcb_input_feedback_state_t {}
impl Clone for xcb_input_feedback_state_t {
    fn clone(&self) -> xcb_input_feedback_state_t { *self }
}
/**
 * @brief xcb_input_feedback_state_iterator_t
 **/
#[repr(C)]
pub struct xcb_input_feedback_state_iterator_t {
    pub data : *mut xcb_input_feedback_state_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_input_kbd_feedback_state_t {
     pub class_id :             u8,
     pub id :                   u8,
     pub len :                  u16,
     pub pitch :                u16,
     pub duration :             u16,
     pub led_mask :             u32,
     pub led_values :           u32,
     pub global_auto_repeat :   u8,
     pub click :                u8,
     pub percent :              u8,
     pub pad0 :                 u8,
     pub auto_repeats :         [u8; 32]
}

impl Copy for xcb_input_kbd_feedback_state_t {}
impl Clone for xcb_input_kbd_feedback_state_t {
    fn clone(&self) -> xcb_input_kbd_feedback_state_t { *self }
}
/**
 * @brief xcb_input_kbd_feedback_state_iterator_t
 **/
#[repr(C)]
pub struct xcb_input_kbd_feedback_state_iterator_t {
    pub data : *mut xcb_input_kbd_feedback_state_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_input_ptr_feedback_state_t {
     pub class_id :      u8,
     pub id :            u8,
     pub len :           u16,
     pub pad0 :          [u8; 2],
     pub accel_num :     u16,
     pub accel_denom :   u16,
     pub threshold :     u16
}

impl Copy for xcb_input_ptr_feedback_state_t {}
impl Clone for xcb_input_ptr_feedback_state_t {
    fn clone(&self) -> xcb_input_ptr_feedback_state_t { *self }
}
/**
 * @brief xcb_input_ptr_feedback_state_iterator_t
 **/
#[repr(C)]
pub struct xcb_input_ptr_feedback_state_iterator_t {
    pub data : *mut xcb_input_ptr_feedback_state_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_input_integer_feedback_state_t {
     pub class_id :     u8,
     pub id :           u8,
     pub len :          u16,
     pub resolution :   u32,
     pub min_value :    i32,
     pub max_value :    i32
}

impl Copy for xcb_input_integer_feedback_state_t {}
impl Clone for xcb_input_integer_feedback_state_t {
    fn clone(&self) -> xcb_input_integer_feedback_state_t { *self }
}
/**
 * @brief xcb_input_integer_feedback_state_iterator_t
 **/
#[repr(C)]
pub struct xcb_input_integer_feedback_state_iterator_t {
    pub data : *mut xcb_input_integer_feedback_state_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_input_string_feedback_state_t {
     pub class_id :      u8,
     pub id :            u8,
     pub len :           u16,
     pub max_symbols :   u16,
     pub num_keysyms :   u16
}

impl Copy for xcb_input_string_feedback_state_t {}
impl Clone for xcb_input_string_feedback_state_t {
    fn clone(&self) -> xcb_input_string_feedback_state_t { *self }
}
/**
 * @brief xcb_input_string_feedback_state_iterator_t
 **/
#[repr(C)]
pub struct xcb_input_string_feedback_state_iterator_t {
    pub data : *mut xcb_input_string_feedback_state_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_input_bell_feedback_state_t {
     pub class_id :   u8,
     pub id :         u8,
     pub len :        u16,
     pub percent :    u8,
     pub pad0 :       [u8; 3],
     pub pitch :      u16,
     pub duration :   u16
}

impl Copy for xcb_input_bell_feedback_state_t {}
impl Clone for xcb_input_bell_feedback_state_t {
    fn clone(&self) -> xcb_input_bell_feedback_state_t { *self }
}
/**
 * @brief xcb_input_bell_feedback_state_iterator_t
 **/
#[repr(C)]
pub struct xcb_input_bell_feedback_state_iterator_t {
    pub data : *mut xcb_input_bell_feedback_state_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_input_led_feedback_state_t {
     pub class_id :     u8,
     pub id :           u8,
     pub len :          u16,
     pub led_mask :     u32,
     pub led_values :   u32
}

impl Copy for xcb_input_led_feedback_state_t {}
impl Clone for xcb_input_led_feedback_state_t {
    fn clone(&self) -> xcb_input_led_feedback_state_t { *self }
}
/**
 * @brief xcb_input_led_feedback_state_iterator_t
 **/
#[repr(C)]
pub struct xcb_input_led_feedback_state_iterator_t {
    pub data : *mut xcb_input_led_feedback_state_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_input_feedback_ctl_t {
     pub class_id :   u8,
     pub id :         u8,
     pub len :        u16
}

impl Copy for xcb_input_feedback_ctl_t {}
impl Clone for xcb_input_feedback_ctl_t {
    fn clone(&self) -> xcb_input_feedback_ctl_t { *self }
}
/**
 * @brief xcb_input_feedback_ctl_iterator_t
 **/
#[repr(C)]
pub struct xcb_input_feedback_ctl_iterator_t {
    pub data : *mut xcb_input_feedback_ctl_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_input_kbd_feedback_ctl_t {
     pub class_id :            u8,
     pub id :                  u8,
     pub len :                 u16,
     pub key :                 xcb_input_key_code_t,
     pub auto_repeat_mode :    u8,
     pub key_click_percent :   i8,
     pub bell_percent :        i8,
     pub bell_pitch :          i16,
     pub bell_duration :       i16,
     pub led_mask :            u32,
     pub led_values :          u32
}

impl Copy for xcb_input_kbd_feedback_ctl_t {}
impl Clone for xcb_input_kbd_feedback_ctl_t {
    fn clone(&self) -> xcb_input_kbd_feedback_ctl_t { *self }
}
/**
 * @brief xcb_input_kbd_feedback_ctl_iterator_t
 **/
#[repr(C)]
pub struct xcb_input_kbd_feedback_ctl_iterator_t {
    pub data : *mut xcb_input_kbd_feedback_ctl_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_input_ptr_feedback_ctl_t {
     pub class_id :    u8,
     pub id :          u8,
     pub len :         u16,
     pub pad0 :        [u8; 2],
     pub num :         i16,
     pub denom :       i16,
     pub threshold :   i16
}

impl Copy for xcb_input_ptr_feedback_ctl_t {}
impl Clone for xcb_input_ptr_feedback_ctl_t {
    fn clone(&self) -> xcb_input_ptr_feedback_ctl_t { *self }
}
/**
 * @brief xcb_input_ptr_feedback_ctl_iterator_t
 **/
#[repr(C)]
pub struct xcb_input_ptr_feedback_ctl_iterator_t {
    pub data : *mut xcb_input_ptr_feedback_ctl_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_input_integer_feedback_ctl_t {
     pub class_id :         u8,
     pub id :               u8,
     pub len :              u16,
     pub int_to_display :   i32
}

impl Copy for xcb_input_integer_feedback_ctl_t {}
impl Clone for xcb_input_integer_feedback_ctl_t {
    fn clone(&self) -> xcb_input_integer_feedback_ctl_t { *self }
}
/**
 * @brief xcb_input_integer_feedback_ctl_iterator_t
 **/
#[repr(C)]
pub struct xcb_input_integer_feedback_ctl_iterator_t {
    pub data : *mut xcb_input_integer_feedback_ctl_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_input_string_feedback_ctl_t {
     pub class_id :      u8,
     pub id :            u8,
     pub len :           u16,
     pub pad0 :          [u8; 2],
     pub num_keysyms :   u16
}

impl Copy for xcb_input_string_feedback_ctl_t {}
impl Clone for xcb_input_string_feedback_ctl_t {
    fn clone(&self) -> xcb_input_string_feedback_ctl_t { *self }
}
/**
 * @brief xcb_input_string_feedback_ctl_iterator_t
 **/
#[repr(C)]
pub struct xcb_input_string_feedback_ctl_iterator_t {
    pub data : *mut xcb_input_string_feedback_ctl_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_input_bell_feedback_ctl_t {
     pub class_id :   u8,
     pub id :         u8,
     pub len :        u16,
     pub percent :    i8,
     pub pad0 :       [u8; 3],
     pub pitch :      i16,
     pub duration :   i16
}

impl Copy for xcb_input_bell_feedback_ctl_t {}
impl Clone for xcb_input_bell_feedback_ctl_t {
    fn clone(&self) -> xcb_input_bell_feedback_ctl_t { *self }
}
/**
 * @brief xcb_input_bell_feedback_ctl_iterator_t
 **/
#[repr(C)]
pub struct xcb_input_bell_feedback_ctl_iterator_t {
    pub data : *mut xcb_input_bell_feedback_ctl_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_input_led_feedback_ctl_t {
     pub class_id :     u8,
     pub id :           u8,
     pub len :          u16,
     pub led_mask :     u32,
     pub led_values :   u32
}

impl Copy for xcb_input_led_feedback_ctl_t {}
impl Clone for xcb_input_led_feedback_ctl_t {
    fn clone(&self) -> xcb_input_led_feedback_ctl_t { *self }
}
/**
 * @brief xcb_input_led_feedback_ctl_iterator_t
 **/
#[repr(C)]
pub struct xcb_input_led_feedback_ctl_iterator_t {
    pub data : *mut xcb_input_led_feedback_ctl_t,
    pub rem  : c_int,
    pub index: c_int
}


#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_input_get_device_key_mapping_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_input_get_device_key_mapping_request_t {
     pub major_opcode :    u8,
     pub minor_opcode :    u8,
     pub length :          u16,
     pub device_id :       u8,
     pub first_keycode :   xcb_input_key_code_t,
     pub count :           u8
}

impl Copy for xcb_input_get_device_key_mapping_request_t {}
impl Clone for xcb_input_get_device_key_mapping_request_t {
    fn clone(&self) -> xcb_input_get_device_key_mapping_request_t { *self }
}

#[repr(C)]
pub struct xcb_input_get_device_key_mapping_reply_t {
     pub response_type :         u8,
     pub pad0 :                  u8,
     pub sequence :              u16,
     pub length :                u32,
     pub keysyms_per_keycode :   u8,
     pub pad1 :                  [u8; 23]
}

impl Copy for xcb_input_get_device_key_mapping_reply_t {}
impl Clone for xcb_input_get_device_key_mapping_reply_t {
    fn clone(&self) -> xcb_input_get_device_key_mapping_reply_t { *self }
}


#[repr(C)]
pub struct xcb_input_change_device_key_mapping_request_t {
     pub major_opcode :          u8,
     pub minor_opcode :          u8,
     pub length :                u16,
     pub device_id :             u8,
     pub first_keycode :         xcb_input_key_code_t,
     pub keysyms_per_keycode :   u8,
     pub keycode_count :         u8
}

impl Copy for xcb_input_change_device_key_mapping_request_t {}
impl Clone for xcb_input_change_device_key_mapping_request_t {
    fn clone(&self) -> xcb_input_change_device_key_mapping_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_input_get_device_modifier_mapping_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_input_get_device_modifier_mapping_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub device_id :      u8,
     pub pad0 :           [u8; 3]
}

impl Copy for xcb_input_get_device_modifier_mapping_request_t {}
impl Clone for xcb_input_get_device_modifier_mapping_request_t {
    fn clone(&self) -> xcb_input_get_device_modifier_mapping_request_t { *self }
}

#[repr(C)]
pub struct xcb_input_get_device_modifier_mapping_reply_t {
     pub response_type :           u8,
     pub pad0 :                    u8,
     pub sequence :                u16,
     pub length :                  u32,
     pub keycodes_per_modifier :   u8,
     pub pad1 :                    [u8; 23]
}

impl Copy for xcb_input_get_device_modifier_mapping_reply_t {}
impl Clone for xcb_input_get_device_modifier_mapping_reply_t {
    fn clone(&self) -> xcb_input_get_device_modifier_mapping_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_input_set_device_modifier_mapping_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_input_set_device_modifier_mapping_request_t {
     pub major_opcode :            u8,
     pub minor_opcode :            u8,
     pub length :                  u16,
     pub device_id :               u8,
     pub keycodes_per_modifier :   u8,
     pub pad0 :                    u8
}

impl Copy for xcb_input_set_device_modifier_mapping_request_t {}
impl Clone for xcb_input_set_device_modifier_mapping_request_t {
    fn clone(&self) -> xcb_input_set_device_modifier_mapping_request_t { *self }
}

#[repr(C)]
pub struct xcb_input_set_device_modifier_mapping_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub status :          u8,
     pub pad1 :            [u8; 23]
}

impl Copy for xcb_input_set_device_modifier_mapping_reply_t {}
impl Clone for xcb_input_set_device_modifier_mapping_reply_t {
    fn clone(&self) -> xcb_input_set_device_modifier_mapping_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_input_get_device_button_mapping_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_input_get_device_button_mapping_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub device_id :      u8,
     pub pad0 :           [u8; 3]
}

impl Copy for xcb_input_get_device_button_mapping_request_t {}
impl Clone for xcb_input_get_device_button_mapping_request_t {
    fn clone(&self) -> xcb_input_get_device_button_mapping_request_t { *self }
}

#[repr(C)]
pub struct xcb_input_get_device_button_mapping_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub map_size :        u8,
     pub pad1 :            [u8; 23]
}

impl Copy for xcb_input_get_device_button_mapping_reply_t {}
impl Clone for xcb_input_get_device_button_mapping_reply_t {
    fn clone(&self) -> xcb_input_get_device_button_mapping_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_input_set_device_button_mapping_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_input_set_device_button_mapping_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub device_id :      u8,
     pub map_size :       u8,
     pub pad0 :           [u8; 2]
}

impl Copy for xcb_input_set_device_button_mapping_request_t {}
impl Clone for xcb_input_set_device_button_mapping_request_t {
    fn clone(&self) -> xcb_input_set_device_button_mapping_request_t { *self }
}

#[repr(C)]
pub struct xcb_input_set_device_button_mapping_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub status :          u8,
     pub pad1 :            [u8; 23]
}

impl Copy for xcb_input_set_device_button_mapping_reply_t {}
impl Clone for xcb_input_set_device_button_mapping_reply_t {
    fn clone(&self) -> xcb_input_set_device_button_mapping_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_input_query_device_state_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_input_query_device_state_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub device_id :      u8,
     pub pad0 :           [u8; 3]
}

impl Copy for xcb_input_query_device_state_request_t {}
impl Clone for xcb_input_query_device_state_request_t {
    fn clone(&self) -> xcb_input_query_device_state_request_t { *self }
}

#[repr(C)]
pub struct xcb_input_query_device_state_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub num_classes :     u8,
     pub pad1 :            [u8; 23]
}

impl Copy for xcb_input_query_device_state_reply_t {}
impl Clone for xcb_input_query_device_state_reply_t {
    fn clone(&self) -> xcb_input_query_device_state_reply_t { *self }
}

#[repr(C)]
pub struct xcb_input_input_state_t {
     pub class_id :    u8,
     pub len :         u8,
     pub num_items :   u8
}

impl Copy for xcb_input_input_state_t {}
impl Clone for xcb_input_input_state_t {
    fn clone(&self) -> xcb_input_input_state_t { *self }
}
/**
 * @brief xcb_input_input_state_iterator_t
 **/
#[repr(C)]
pub struct xcb_input_input_state_iterator_t {
    pub data : *mut xcb_input_input_state_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_input_key_state_t {
     pub class_id :   u8,
     pub len :        u8,
     pub num_keys :   u8,
     pub pad0 :       u8,
     pub keys :       [u8; 32]
}

impl Copy for xcb_input_key_state_t {}
impl Clone for xcb_input_key_state_t {
    fn clone(&self) -> xcb_input_key_state_t { *self }
}
/**
 * @brief xcb_input_key_state_iterator_t
 **/
#[repr(C)]
pub struct xcb_input_key_state_iterator_t {
    pub data : *mut xcb_input_key_state_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_input_button_state_t {
     pub class_id :      u8,
     pub len :           u8,
     pub num_buttons :   u8,
     pub pad0 :          u8,
     pub buttons :       [u8; 32]
}

impl Copy for xcb_input_button_state_t {}
impl Clone for xcb_input_button_state_t {
    fn clone(&self) -> xcb_input_button_state_t { *self }
}
/**
 * @brief xcb_input_button_state_iterator_t
 **/
#[repr(C)]
pub struct xcb_input_button_state_iterator_t {
    pub data : *mut xcb_input_button_state_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_input_valuator_state_t {
     pub class_id :        u8,
     pub len :             u8,
     pub num_valuators :   u8,
     pub mode :            u8
}

impl Copy for xcb_input_valuator_state_t {}
impl Clone for xcb_input_valuator_state_t {
    fn clone(&self) -> xcb_input_valuator_state_t { *self }
}
/**
 * @brief xcb_input_valuator_state_iterator_t
 **/
#[repr(C)]
pub struct xcb_input_valuator_state_iterator_t {
    pub data : *mut xcb_input_valuator_state_t,
    pub rem  : c_int,
    pub index: c_int
}



#[repr(C)]
pub struct xcb_input_send_extension_event_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub destination :    ffi::xproto::xcb_window_t,
     pub device_id :      u8,
     pub propagate :      u8,
     pub num_classes :    u16,
     pub num_events :     u8,
     pub pad0 :           [u8; 3]
}

impl Copy for xcb_input_send_extension_event_request_t {}
impl Clone for xcb_input_send_extension_event_request_t {
    fn clone(&self) -> xcb_input_send_extension_event_request_t { *self }
}


#[repr(C)]
pub struct xcb_input_device_bell_request_t {
     pub major_opcode :     u8,
     pub minor_opcode :     u8,
     pub length :           u16,
     pub device_id :        u8,
     pub feedback_id :      u8,
     pub feedback_class :   u8,
     pub percent :          i8
}

impl Copy for xcb_input_device_bell_request_t {}
impl Clone for xcb_input_device_bell_request_t {
    fn clone(&self) -> xcb_input_device_bell_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_input_set_device_valuators_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_input_set_device_valuators_request_t {
     pub major_opcode :     u8,
     pub minor_opcode :     u8,
     pub length :           u16,
     pub device_id :        u8,
     pub first_valuator :   u8,
     pub num_valuators :    u8,
     pub pad0 :             u8
}

impl Copy for xcb_input_set_device_valuators_request_t {}
impl Clone for xcb_input_set_device_valuators_request_t {
    fn clone(&self) -> xcb_input_set_device_valuators_request_t { *self }
}

#[repr(C)]
pub struct xcb_input_set_device_valuators_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub status :          u8,
     pub pad1 :            [u8; 23]
}

impl Copy for xcb_input_set_device_valuators_reply_t {}
impl Clone for xcb_input_set_device_valuators_reply_t {
    fn clone(&self) -> xcb_input_set_device_valuators_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_input_get_device_control_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_input_get_device_control_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub control_id :     u16,
     pub device_id :      u8,
     pub pad0 :           u8
}

impl Copy for xcb_input_get_device_control_request_t {}
impl Clone for xcb_input_get_device_control_request_t {
    fn clone(&self) -> xcb_input_get_device_control_request_t { *self }
}

#[repr(C)]
pub struct xcb_input_get_device_control_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub status :          u8,
     pub pad1 :            [u8; 23]
}

impl Copy for xcb_input_get_device_control_reply_t {}
impl Clone for xcb_input_get_device_control_reply_t {
    fn clone(&self) -> xcb_input_get_device_control_reply_t { *self }
}

#[repr(C)]
pub struct xcb_input_device_state_t {
     pub control_id :   u16,
     pub len :          u16
}

impl Copy for xcb_input_device_state_t {}
impl Clone for xcb_input_device_state_t {
    fn clone(&self) -> xcb_input_device_state_t { *self }
}
/**
 * @brief xcb_input_device_state_iterator_t
 **/
#[repr(C)]
pub struct xcb_input_device_state_iterator_t {
    pub data : *mut xcb_input_device_state_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_input_device_resolution_state_t {
     pub control_id :      u16,
     pub len :             u16,
     pub num_valuators :   u32
}

impl Copy for xcb_input_device_resolution_state_t {}
impl Clone for xcb_input_device_resolution_state_t {
    fn clone(&self) -> xcb_input_device_resolution_state_t { *self }
}
/**
 * @brief xcb_input_device_resolution_state_iterator_t
 **/
#[repr(C)]
pub struct xcb_input_device_resolution_state_iterator_t {
    pub data : *mut xcb_input_device_resolution_state_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_input_device_abs_calib_state_t {
     pub control_id :         u16,
     pub len :                u16,
     pub min_x :              i32,
     pub max_x :              i32,
     pub min_y :              i32,
     pub max_y :              i32,
     pub flip_x :             u32,
     pub flip_y :             u32,
     pub rotation :           u32,
     pub button_threshold :   u32
}

impl Copy for xcb_input_device_abs_calib_state_t {}
impl Clone for xcb_input_device_abs_calib_state_t {
    fn clone(&self) -> xcb_input_device_abs_calib_state_t { *self }
}
/**
 * @brief xcb_input_device_abs_calib_state_iterator_t
 **/
#[repr(C)]
pub struct xcb_input_device_abs_calib_state_iterator_t {
    pub data : *mut xcb_input_device_abs_calib_state_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_input_device_abs_area_state_t {
     pub control_id :   u16,
     pub len :          u16,
     pub offset_x :     u32,
     pub offset_y :     u32,
     pub width :        u32,
     pub height :       u32,
     pub screen :       u32,
     pub following :    u32
}

impl Copy for xcb_input_device_abs_area_state_t {}
impl Clone for xcb_input_device_abs_area_state_t {
    fn clone(&self) -> xcb_input_device_abs_area_state_t { *self }
}
/**
 * @brief xcb_input_device_abs_area_state_iterator_t
 **/
#[repr(C)]
pub struct xcb_input_device_abs_area_state_iterator_t {
    pub data : *mut xcb_input_device_abs_area_state_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_input_device_core_state_t {
     pub control_id :   u16,
     pub len :          u16,
     pub status :       u8,
     pub iscore :       u8,
     pub pad0 :         [u8; 2]
}

impl Copy for xcb_input_device_core_state_t {}
impl Clone for xcb_input_device_core_state_t {
    fn clone(&self) -> xcb_input_device_core_state_t { *self }
}
/**
 * @brief xcb_input_device_core_state_iterator_t
 **/
#[repr(C)]
pub struct xcb_input_device_core_state_iterator_t {
    pub data : *mut xcb_input_device_core_state_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_input_device_enable_state_t {
     pub control_id :   u16,
     pub len :          u16,
     pub enable :       u8,
     pub pad0 :         [u8; 3]
}

impl Copy for xcb_input_device_enable_state_t {}
impl Clone for xcb_input_device_enable_state_t {
    fn clone(&self) -> xcb_input_device_enable_state_t { *self }
}
/**
 * @brief xcb_input_device_enable_state_iterator_t
 **/
#[repr(C)]
pub struct xcb_input_device_enable_state_iterator_t {
    pub data : *mut xcb_input_device_enable_state_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_input_device_ctl_t {
     pub control_id :   u16,
     pub len :          u16
}

impl Copy for xcb_input_device_ctl_t {}
impl Clone for xcb_input_device_ctl_t {
    fn clone(&self) -> xcb_input_device_ctl_t { *self }
}
/**
 * @brief xcb_input_device_ctl_iterator_t
 **/
#[repr(C)]
pub struct xcb_input_device_ctl_iterator_t {
    pub data : *mut xcb_input_device_ctl_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_input_device_resolution_ctl_t {
     pub control_id :       u16,
     pub len :              u16,
     pub first_valuator :   u8,
     pub num_valuators :    u8
}

impl Copy for xcb_input_device_resolution_ctl_t {}
impl Clone for xcb_input_device_resolution_ctl_t {
    fn clone(&self) -> xcb_input_device_resolution_ctl_t { *self }
}
/**
 * @brief xcb_input_device_resolution_ctl_iterator_t
 **/
#[repr(C)]
pub struct xcb_input_device_resolution_ctl_iterator_t {
    pub data : *mut xcb_input_device_resolution_ctl_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_input_device_abs_calib_ctl_t {
     pub control_id :         u16,
     pub len :                u16,
     pub min_x :              i32,
     pub max_x :              i32,
     pub min_y :              i32,
     pub max_y :              i32,
     pub flip_x :             u32,
     pub flip_y :             u32,
     pub rotation :           u32,
     pub button_threshold :   u32
}

impl Copy for xcb_input_device_abs_calib_ctl_t {}
impl Clone for xcb_input_device_abs_calib_ctl_t {
    fn clone(&self) -> xcb_input_device_abs_calib_ctl_t { *self }
}
/**
 * @brief xcb_input_device_abs_calib_ctl_iterator_t
 **/
#[repr(C)]
pub struct xcb_input_device_abs_calib_ctl_iterator_t {
    pub data : *mut xcb_input_device_abs_calib_ctl_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_input_device_abs_area_ctrl_t {
     pub control_id :   u16,
     pub len :          u16,
     pub offset_x :     u32,
     pub offset_y :     u32,
     pub width :        i32,
     pub height :       i32,
     pub screen :       i32,
     pub following :    u32
}

impl Copy for xcb_input_device_abs_area_ctrl_t {}
impl Clone for xcb_input_device_abs_area_ctrl_t {
    fn clone(&self) -> xcb_input_device_abs_area_ctrl_t { *self }
}
/**
 * @brief xcb_input_device_abs_area_ctrl_iterator_t
 **/
#[repr(C)]
pub struct xcb_input_device_abs_area_ctrl_iterator_t {
    pub data : *mut xcb_input_device_abs_area_ctrl_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_input_device_core_ctrl_t {
     pub control_id :   u16,
     pub len :          u16,
     pub status :       u8,
     pub pad0 :         [u8; 3]
}

impl Copy for xcb_input_device_core_ctrl_t {}
impl Clone for xcb_input_device_core_ctrl_t {
    fn clone(&self) -> xcb_input_device_core_ctrl_t { *self }
}
/**
 * @brief xcb_input_device_core_ctrl_iterator_t
 **/
#[repr(C)]
pub struct xcb_input_device_core_ctrl_iterator_t {
    pub data : *mut xcb_input_device_core_ctrl_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_input_device_enable_ctrl_t {
     pub control_id :   u16,
     pub len :          u16,
     pub enable :       u8,
     pub pad0 :         [u8; 3]
}

impl Copy for xcb_input_device_enable_ctrl_t {}
impl Clone for xcb_input_device_enable_ctrl_t {
    fn clone(&self) -> xcb_input_device_enable_ctrl_t { *self }
}
/**
 * @brief xcb_input_device_enable_ctrl_iterator_t
 **/
#[repr(C)]
pub struct xcb_input_device_enable_ctrl_iterator_t {
    pub data : *mut xcb_input_device_enable_ctrl_t,
    pub rem  : c_int,
    pub index: c_int
}



#[repr(C)]
pub struct xcb_input_device_valuator_event_t {
     pub response_type :    u8,
     pub device_id :        u8,
     pub sequence :         u16,
     pub device_state :     u16,
     pub num_valuators :    u8,
     pub first_valuator :   u8,
     pub valuators :        [i32; 6]
}

impl Copy for xcb_input_device_valuator_event_t {}
impl Clone for xcb_input_device_valuator_event_t {
    fn clone(&self) -> xcb_input_device_valuator_event_t { *self }
}


#[repr(C)]
pub struct xcb_input_device_key_press_event_t {
     pub response_type :   u8,
     pub detail :          u8,
     pub sequence :        u16,
     pub time :            ffi::xproto::xcb_timestamp_t,
     pub root :            ffi::xproto::xcb_window_t,
     pub event :           ffi::xproto::xcb_window_t,
     pub child :           ffi::xproto::xcb_window_t,
     pub root_x :          i16,
     pub root_y :          i16,
     pub event_x :         i16,
     pub event_y :         i16,
     pub state :           u16,
     pub same_screen :     u8,
     pub device_id :       u8
}

impl Copy for xcb_input_device_key_press_event_t {}
impl Clone for xcb_input_device_key_press_event_t {
    fn clone(&self) -> xcb_input_device_key_press_event_t { *self }
}


pub type xcb_input_device_key_release_event_t = xcb_input_device_key_press_event_t;


pub type xcb_input_device_button_press_event_t = xcb_input_device_key_press_event_t;


pub type xcb_input_device_button_release_event_t = xcb_input_device_key_press_event_t;


pub type xcb_input_device_motion_notify_event_t = xcb_input_device_key_press_event_t;


pub type xcb_input_proximity_in_event_t = xcb_input_device_key_press_event_t;


pub type xcb_input_proximity_out_event_t = xcb_input_device_key_press_event_t;


#[repr(C)]
pub struct xcb_input_focus_in_event_t {
     pub response_type :   u8,
     pub detail :          u8,
     pub sequence :        u16,
     pub time :            ffi::xproto::xcb_timestamp_t,
     pub window :          ffi::xproto::xcb_window_t,
     pub mode :            u8,
     pub device_id :       u8,
     pub pad0 :            [u8; 18]
}

impl Copy for xcb_input_focus_in_event_t {}
impl Clone for xcb_input_focus_in_event_t {
    fn clone(&self) -> xcb_input_focus_in_event_t { *self }
}


pub type xcb_input_focus_out_event_t = xcb_input_focus_in_event_t;


#[repr(C)]
pub struct xcb_input_device_state_notify_event_t {
     pub response_type :      u8,
     pub device_id :          u8,
     pub sequence :           u16,
     pub time :               ffi::xproto::xcb_timestamp_t,
     pub num_keys :           u8,
     pub num_buttons :        u8,
     pub num_valuators :      u8,
     pub classes_reported :   u8,
     pub buttons :            [u8; 4],
     pub keys :               [u8; 4],
     pub valuators :          [u32; 3]
}

impl Copy for xcb_input_device_state_notify_event_t {}
impl Clone for xcb_input_device_state_notify_event_t {
    fn clone(&self) -> xcb_input_device_state_notify_event_t { *self }
}


#[repr(C)]
pub struct xcb_input_device_mapping_notify_event_t {
     pub response_type :   u8,
     pub device_id :       u8,
     pub sequence :        u16,
     pub request :         u8,
     pub first_keycode :   xcb_input_key_code_t,
     pub count :           u8,
     pub pad0 :            u8,
     pub time :            ffi::xproto::xcb_timestamp_t,
     pub pad1 :            [u8; 20]
}

impl Copy for xcb_input_device_mapping_notify_event_t {}
impl Clone for xcb_input_device_mapping_notify_event_t {
    fn clone(&self) -> xcb_input_device_mapping_notify_event_t { *self }
}


#[repr(C)]
pub struct xcb_input_change_device_notify_event_t {
     pub response_type :   u8,
     pub device_id :       u8,
     pub sequence :        u16,
     pub time :            ffi::xproto::xcb_timestamp_t,
     pub request :         u8,
     pub pad0 :            [u8; 23]
}

impl Copy for xcb_input_change_device_notify_event_t {}
impl Clone for xcb_input_change_device_notify_event_t {
    fn clone(&self) -> xcb_input_change_device_notify_event_t { *self }
}


#[repr(C)]
pub struct xcb_input_device_key_state_notify_event_t {
     pub response_type :   u8,
     pub device_id :       u8,
     pub sequence :        u16,
     pub keys :            [u8; 28]
}

impl Copy for xcb_input_device_key_state_notify_event_t {}
impl Clone for xcb_input_device_key_state_notify_event_t {
    fn clone(&self) -> xcb_input_device_key_state_notify_event_t { *self }
}


#[repr(C)]
pub struct xcb_input_device_button_state_notify_event_t {
     pub response_type :   u8,
     pub device_id :       u8,
     pub sequence :        u16,
     pub buttons :         [u8; 28]
}

impl Copy for xcb_input_device_button_state_notify_event_t {}
impl Clone for xcb_input_device_button_state_notify_event_t {
    fn clone(&self) -> xcb_input_device_button_state_notify_event_t { *self }
}


#[repr(C)]
pub struct xcb_input_device_presence_notify_event_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub time :            ffi::xproto::xcb_timestamp_t,
     pub devchange :       u8,
     pub device_id :       u8,
     pub control :         u16,
     pub pad1 :            [u8; 20]
}

impl Copy for xcb_input_device_presence_notify_event_t {}
impl Clone for xcb_input_device_presence_notify_event_t {
    fn clone(&self) -> xcb_input_device_presence_notify_event_t { *self }
}


#[repr(C)]
pub struct xcb_input_device_error_t {
     pub response_type :   u8,
     pub error_code :      u8,
     pub sequence :        u16
}

impl Copy for xcb_input_device_error_t {}
impl Clone for xcb_input_device_error_t {
    fn clone(&self) -> xcb_input_device_error_t { *self }
}


#[repr(C)]
pub struct xcb_input_event_error_t {
     pub response_type :   u8,
     pub error_code :      u8,
     pub sequence :        u16
}

impl Copy for xcb_input_event_error_t {}
impl Clone for xcb_input_event_error_t {
    fn clone(&self) -> xcb_input_event_error_t { *self }
}


#[repr(C)]
pub struct xcb_input_mode_error_t {
     pub response_type :   u8,
     pub error_code :      u8,
     pub sequence :        u16
}

impl Copy for xcb_input_mode_error_t {}
impl Clone for xcb_input_mode_error_t {
    fn clone(&self) -> xcb_input_mode_error_t { *self }
}


#[repr(C)]
pub struct xcb_input_device_busy_error_t {
     pub response_type :   u8,
     pub error_code :      u8,
     pub sequence :        u16
}

impl Copy for xcb_input_device_busy_error_t {}
impl Clone for xcb_input_device_busy_error_t {
    fn clone(&self) -> xcb_input_device_busy_error_t { *self }
}


#[repr(C)]
pub struct xcb_input_class_error_t {
     pub response_type :   u8,
     pub error_code :      u8,
     pub sequence :        u16
}

impl Copy for xcb_input_class_error_t {}
impl Clone for xcb_input_class_error_t {
    fn clone(&self) -> xcb_input_class_error_t { *self }
}
#[link(name="xcb-xinput")]
extern "C" {

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_input_key_code_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_input_key_code_t)
 *
 *
 */
pub fn xcb_input_key_code_next (i:*mut xcb_input_key_code_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_input_key_code_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_key_code_end (i:xcb_input_key_code_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_input_event_class_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_input_event_class_t)
 *
 *
 */
pub fn xcb_input_event_class_next (i:*mut xcb_input_event_class_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_input_event_class_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_event_class_end (i:xcb_input_event_class_iterator_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_input_get_extension_version_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_input_get_extension_version (c : *mut ffi::base::xcb_connection_t,
                                           name_len :  u16,
                                           name : *mut c_char) -> xcb_input_get_extension_version_cookie_t;

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
pub fn xcb_input_get_extension_version_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                     name_len :  u16,
                                                     name : *mut c_char) -> xcb_input_get_extension_version_cookie_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_input_get_extension_version_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_input_get_extension_version_reply (c : *mut ffi::base::xcb_connection_t,
                                                 cookie : xcb_input_get_extension_version_cookie_t,
                                                 e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_input_get_extension_version_reply_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_input_device_info_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_input_device_info_t)
 *
 *
 */
pub fn xcb_input_device_info_next (i:*mut xcb_input_device_info_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_input_device_info_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_device_info_end (i:xcb_input_device_info_iterator_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_input_list_input_devices_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_input_list_input_devices (c : *mut ffi::base::xcb_connection_t) -> xcb_input_list_input_devices_cookie_t;

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
pub fn xcb_input_list_input_devices_unchecked (c : *mut ffi::base::xcb_connection_t) -> xcb_input_list_input_devices_cookie_t;

pub fn xcb_input_list_input_devices_devices (R : *mut xcb_input_list_input_devices_reply_t) -> *mut xcb_input_device_info_t;


pub fn xcb_input_list_input_devices_devices_length (R : *mut xcb_input_list_input_devices_reply_t) -> c_int;

pub fn xcb_input_list_input_devices_devices_iterator (R : *mut xcb_input_list_input_devices_reply_t) -> xcb_input_device_info_iterator_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_input_list_input_devices_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_input_list_input_devices_reply (c : *mut ffi::base::xcb_connection_t,
                                              cookie : xcb_input_list_input_devices_cookie_t,
                                              e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_input_list_input_devices_reply_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_input_input_info_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_input_input_info_t)
 *
 *
 */
pub fn xcb_input_input_info_next (i:*mut xcb_input_input_info_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_input_input_info_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_input_info_end (i:xcb_input_input_info_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_input_key_info_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_input_key_info_t)
 *
 *
 */
pub fn xcb_input_key_info_next (i:*mut xcb_input_key_info_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_input_key_info_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_key_info_end (i:xcb_input_key_info_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_input_button_info_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_input_button_info_t)
 *
 *
 */
pub fn xcb_input_button_info_next (i:*mut xcb_input_button_info_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_input_button_info_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_button_info_end (i:xcb_input_button_info_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_input_axis_info_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_input_axis_info_t)
 *
 *
 */
pub fn xcb_input_axis_info_next (i:*mut xcb_input_axis_info_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_input_axis_info_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_axis_info_end (i:xcb_input_axis_info_iterator_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_input_valuator_info_sizeof (_buffer :  *mut c_void) -> c_int;

pub fn xcb_input_valuator_info_axes (R : *mut xcb_input_valuator_info_t) -> *mut xcb_input_axis_info_t;


pub fn xcb_input_valuator_info_axes_length (R : *mut xcb_input_valuator_info_t) -> c_int;

pub fn xcb_input_valuator_info_axes_iterator (R : *mut xcb_input_valuator_info_t) -> xcb_input_axis_info_iterator_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_input_valuator_info_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_input_valuator_info_t)
 *
 *
 */
pub fn xcb_input_valuator_info_next (i:*mut xcb_input_valuator_info_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_input_valuator_info_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_valuator_info_end (i:xcb_input_valuator_info_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_input_input_class_info_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_input_input_class_info_t)
 *
 *
 */
pub fn xcb_input_input_class_info_next (i:*mut xcb_input_input_class_info_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_input_input_class_info_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_input_class_info_end (i:xcb_input_input_class_info_iterator_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_input_open_device_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_input_open_device (c : *mut ffi::base::xcb_connection_t,
                                 device_id :  u8) -> xcb_input_open_device_cookie_t;

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
pub fn xcb_input_open_device_unchecked (c : *mut ffi::base::xcb_connection_t,
                                           device_id :  u8) -> xcb_input_open_device_cookie_t;

pub fn xcb_input_open_device_class_info (R : *mut xcb_input_open_device_reply_t) -> *mut xcb_input_input_class_info_t;


pub fn xcb_input_open_device_class_info_length (R : *mut xcb_input_open_device_reply_t) -> c_int;

pub fn xcb_input_open_device_class_info_iterator (R : *mut xcb_input_open_device_reply_t) -> xcb_input_input_class_info_iterator_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_input_open_device_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_input_open_device_reply (c : *mut ffi::base::xcb_connection_t,
                                       cookie : xcb_input_open_device_cookie_t,
                                       e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_input_open_device_reply_t;

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
pub fn xcb_input_close_device_checked (c : *mut ffi::base::xcb_connection_t,
                                          device_id :  u8) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_input_close_device (c : *mut ffi::base::xcb_connection_t,
                                  device_id :  u8) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_input_set_device_mode (c : *mut ffi::base::xcb_connection_t,
                                     device_id :  u8,
                                     mode :  u8) -> xcb_input_set_device_mode_cookie_t;

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
pub fn xcb_input_set_device_mode_unchecked (c : *mut ffi::base::xcb_connection_t,
                                               device_id :  u8,
                                               mode :  u8) -> xcb_input_set_device_mode_cookie_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_input_set_device_mode_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_input_set_device_mode_reply (c : *mut ffi::base::xcb_connection_t,
                                           cookie : xcb_input_set_device_mode_cookie_t,
                                           e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_input_set_device_mode_reply_t;

pub fn xcb_input_select_extension_event_sizeof (_buffer :  *mut c_void) -> c_int;

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
pub fn xcb_input_select_extension_event_checked (c : *mut ffi::base::xcb_connection_t,
                                                    window :  ffi::xproto::xcb_window_t,
                                                    num_classes :  u16,
                                                    classes : *mut xcb_input_event_class_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_input_select_extension_event (c : *mut ffi::base::xcb_connection_t,
                                            window :  ffi::xproto::xcb_window_t,
                                            num_classes :  u16,
                                            classes : *mut xcb_input_event_class_t) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_input_get_selected_extension_events_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_input_get_selected_extension_events (c : *mut ffi::base::xcb_connection_t,
                                                   window :  ffi::xproto::xcb_window_t) -> xcb_input_get_selected_extension_events_cookie_t;

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
pub fn xcb_input_get_selected_extension_events_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                             window :  ffi::xproto::xcb_window_t) -> xcb_input_get_selected_extension_events_cookie_t;

pub fn xcb_input_get_selected_extension_events_this_classes (R : *mut xcb_input_get_selected_extension_events_reply_t) -> *mut xcb_input_event_class_t;


pub fn xcb_input_get_selected_extension_events_this_classes_length (R : *mut xcb_input_get_selected_extension_events_reply_t) -> c_int;


pub fn xcb_input_get_selected_extension_events_this_classes_end (R : *mut xcb_input_get_selected_extension_events_reply_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_input_get_selected_extension_events_all_classes (R : *mut xcb_input_get_selected_extension_events_reply_t) -> *mut xcb_input_event_class_t;


pub fn xcb_input_get_selected_extension_events_all_classes_length (R : *mut xcb_input_get_selected_extension_events_reply_t) -> c_int;


pub fn xcb_input_get_selected_extension_events_all_classes_end (R : *mut xcb_input_get_selected_extension_events_reply_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_input_get_selected_extension_events_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_input_get_selected_extension_events_reply (c : *mut ffi::base::xcb_connection_t,
                                                         cookie : xcb_input_get_selected_extension_events_cookie_t,
                                                         e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_input_get_selected_extension_events_reply_t;

pub fn xcb_input_change_device_dont_propagate_list_sizeof (_buffer :  *mut c_void) -> c_int;

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
pub fn xcb_input_change_device_dont_propagate_list_checked (c : *mut ffi::base::xcb_connection_t,
                                                               window :  ffi::xproto::xcb_window_t,
                                                               num_classes :  u16,
                                                               mode :  u8,
                                                               classes : *mut xcb_input_event_class_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_input_change_device_dont_propagate_list (c : *mut ffi::base::xcb_connection_t,
                                                       window :  ffi::xproto::xcb_window_t,
                                                       num_classes :  u16,
                                                       mode :  u8,
                                                       classes : *mut xcb_input_event_class_t) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_input_get_device_dont_propagate_list_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_input_get_device_dont_propagate_list (c : *mut ffi::base::xcb_connection_t,
                                                    window :  ffi::xproto::xcb_window_t) -> xcb_input_get_device_dont_propagate_list_cookie_t;

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
pub fn xcb_input_get_device_dont_propagate_list_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                              window :  ffi::xproto::xcb_window_t) -> xcb_input_get_device_dont_propagate_list_cookie_t;

pub fn xcb_input_get_device_dont_propagate_list_classes (R : *mut xcb_input_get_device_dont_propagate_list_reply_t) -> *mut xcb_input_event_class_t;


pub fn xcb_input_get_device_dont_propagate_list_classes_length (R : *mut xcb_input_get_device_dont_propagate_list_reply_t) -> c_int;


pub fn xcb_input_get_device_dont_propagate_list_classes_end (R : *mut xcb_input_get_device_dont_propagate_list_reply_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_input_get_device_dont_propagate_list_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_input_get_device_dont_propagate_list_reply (c : *mut ffi::base::xcb_connection_t,
                                                          cookie : xcb_input_get_device_dont_propagate_list_cookie_t,
                                                          e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_input_get_device_dont_propagate_list_reply_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_input_get_device_motion_events (c : *mut ffi::base::xcb_connection_t,
                                              start :  ffi::xproto::xcb_timestamp_t,
                                              stop :  ffi::xproto::xcb_timestamp_t,
                                              device_id :  u8) -> xcb_input_get_device_motion_events_cookie_t;

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
pub fn xcb_input_get_device_motion_events_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                        start :  ffi::xproto::xcb_timestamp_t,
                                                        stop :  ffi::xproto::xcb_timestamp_t,
                                                        device_id :  u8) -> xcb_input_get_device_motion_events_cookie_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_input_get_device_motion_events_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_input_get_device_motion_events_reply (c : *mut ffi::base::xcb_connection_t,
                                                    cookie : xcb_input_get_device_motion_events_cookie_t,
                                                    e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_input_get_device_motion_events_reply_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_input_device_time_coord_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_input_device_time_coord_t)
 *
 *
 */
pub fn xcb_input_device_time_coord_next (i:*mut xcb_input_device_time_coord_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_input_device_time_coord_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_device_time_coord_end (i:xcb_input_device_time_coord_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_input_change_keyboard_device (c : *mut ffi::base::xcb_connection_t,
                                            device_id :  u8) -> xcb_input_change_keyboard_device_cookie_t;

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
pub fn xcb_input_change_keyboard_device_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                      device_id :  u8) -> xcb_input_change_keyboard_device_cookie_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_input_change_keyboard_device_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_input_change_keyboard_device_reply (c : *mut ffi::base::xcb_connection_t,
                                                  cookie : xcb_input_change_keyboard_device_cookie_t,
                                                  e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_input_change_keyboard_device_reply_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_input_change_pointer_device (c : *mut ffi::base::xcb_connection_t,
                                           x_axis :  u8,
                                           y_axis :  u8,
                                           device_id :  u8) -> xcb_input_change_pointer_device_cookie_t;

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
pub fn xcb_input_change_pointer_device_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                     x_axis :  u8,
                                                     y_axis :  u8,
                                                     device_id :  u8) -> xcb_input_change_pointer_device_cookie_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_input_change_pointer_device_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_input_change_pointer_device_reply (c : *mut ffi::base::xcb_connection_t,
                                                 cookie : xcb_input_change_pointer_device_cookie_t,
                                                 e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_input_change_pointer_device_reply_t;

pub fn xcb_input_grab_device_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_input_grab_device (c : *mut ffi::base::xcb_connection_t,
                                 grab_window :  ffi::xproto::xcb_window_t,
                                 time :  ffi::xproto::xcb_timestamp_t,
                                 num_classes :  u16,
                                 this_device_mode :  u8,
                                 other_device_mode :  u8,
                                 owner_events :  u8,
                                 device_id :  u8,
                                 classes : *mut xcb_input_event_class_t) -> xcb_input_grab_device_cookie_t;

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
pub fn xcb_input_grab_device_unchecked (c : *mut ffi::base::xcb_connection_t,
                                           grab_window :  ffi::xproto::xcb_window_t,
                                           time :  ffi::xproto::xcb_timestamp_t,
                                           num_classes :  u16,
                                           this_device_mode :  u8,
                                           other_device_mode :  u8,
                                           owner_events :  u8,
                                           device_id :  u8,
                                           classes : *mut xcb_input_event_class_t) -> xcb_input_grab_device_cookie_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_input_grab_device_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_input_grab_device_reply (c : *mut ffi::base::xcb_connection_t,
                                       cookie : xcb_input_grab_device_cookie_t,
                                       e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_input_grab_device_reply_t;

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
pub fn xcb_input_ungrab_device_checked (c : *mut ffi::base::xcb_connection_t,
                                           time :  ffi::xproto::xcb_timestamp_t,
                                           device_id :  u8) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_input_ungrab_device (c : *mut ffi::base::xcb_connection_t,
                                   time :  ffi::xproto::xcb_timestamp_t,
                                   device_id :  u8) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_input_grab_device_key_sizeof (_buffer :  *mut c_void) -> c_int;

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
pub fn xcb_input_grab_device_key_checked (c : *mut ffi::base::xcb_connection_t,
                                             grab_window :  ffi::xproto::xcb_window_t,
                                             num_classes :  u16,
                                             modifiers :  u16,
                                             modifier_device :  u8,
                                             grabbed_device :  u8,
                                             key :  u8,
                                             this_device_mode :  u8,
                                             other_device_mode :  u8,
                                             owner_events :  u8,
                                             classes : *mut xcb_input_event_class_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_input_grab_device_key (c : *mut ffi::base::xcb_connection_t,
                                     grab_window :  ffi::xproto::xcb_window_t,
                                     num_classes :  u16,
                                     modifiers :  u16,
                                     modifier_device :  u8,
                                     grabbed_device :  u8,
                                     key :  u8,
                                     this_device_mode :  u8,
                                     other_device_mode :  u8,
                                     owner_events :  u8,
                                     classes : *mut xcb_input_event_class_t) -> ffi::base::xcb_void_cookie_t;

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
pub fn xcb_input_ungrab_device_key_checked (c : *mut ffi::base::xcb_connection_t,
                                               grabWindow :  ffi::xproto::xcb_window_t,
                                               modifiers :  u16,
                                               modifier_device :  u8,
                                               key :  u8,
                                               grabbed_device :  u8) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_input_ungrab_device_key (c : *mut ffi::base::xcb_connection_t,
                                       grabWindow :  ffi::xproto::xcb_window_t,
                                       modifiers :  u16,
                                       modifier_device :  u8,
                                       key :  u8,
                                       grabbed_device :  u8) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_input_grab_device_button_sizeof (_buffer :  *mut c_void) -> c_int;

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
pub fn xcb_input_grab_device_button_checked (c : *mut ffi::base::xcb_connection_t,
                                                grab_window :  ffi::xproto::xcb_window_t,
                                                grabbed_device :  u8,
                                                modifier_device :  u8,
                                                num_classes :  u16,
                                                modifiers :  u16,
                                                this_device_mode :  u8,
                                                other_device_mode :  u8,
                                                button :  u8,
                                                owner_events :  u8,
                                                classes : *mut xcb_input_event_class_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_input_grab_device_button (c : *mut ffi::base::xcb_connection_t,
                                        grab_window :  ffi::xproto::xcb_window_t,
                                        grabbed_device :  u8,
                                        modifier_device :  u8,
                                        num_classes :  u16,
                                        modifiers :  u16,
                                        this_device_mode :  u8,
                                        other_device_mode :  u8,
                                        button :  u8,
                                        owner_events :  u8,
                                        classes : *mut xcb_input_event_class_t) -> ffi::base::xcb_void_cookie_t;

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
pub fn xcb_input_ungrab_device_button_checked (c : *mut ffi::base::xcb_connection_t,
                                                  grab_window :  ffi::xproto::xcb_window_t,
                                                  modifiers :  u16,
                                                  modifier_device :  u8,
                                                  button :  u8,
                                                  grabbed_device :  u8) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_input_ungrab_device_button (c : *mut ffi::base::xcb_connection_t,
                                          grab_window :  ffi::xproto::xcb_window_t,
                                          modifiers :  u16,
                                          modifier_device :  u8,
                                          button :  u8,
                                          grabbed_device :  u8) -> ffi::base::xcb_void_cookie_t;

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
pub fn xcb_input_allow_device_events_checked (c : *mut ffi::base::xcb_connection_t,
                                                 time :  ffi::xproto::xcb_timestamp_t,
                                                 mode :  u8,
                                                 device_id :  u8) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_input_allow_device_events (c : *mut ffi::base::xcb_connection_t,
                                         time :  ffi::xproto::xcb_timestamp_t,
                                         mode :  u8,
                                         device_id :  u8) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_input_get_device_focus (c : *mut ffi::base::xcb_connection_t,
                                      device_id :  u8) -> xcb_input_get_device_focus_cookie_t;

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
pub fn xcb_input_get_device_focus_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                device_id :  u8) -> xcb_input_get_device_focus_cookie_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_input_get_device_focus_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_input_get_device_focus_reply (c : *mut ffi::base::xcb_connection_t,
                                            cookie : xcb_input_get_device_focus_cookie_t,
                                            e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_input_get_device_focus_reply_t;

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
pub fn xcb_input_set_device_focus_checked (c : *mut ffi::base::xcb_connection_t,
                                              focus :  ffi::xproto::xcb_window_t,
                                              time :  ffi::xproto::xcb_timestamp_t,
                                              revert_to :  u8,
                                              device_id :  u8) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_input_set_device_focus (c : *mut ffi::base::xcb_connection_t,
                                      focus :  ffi::xproto::xcb_window_t,
                                      time :  ffi::xproto::xcb_timestamp_t,
                                      revert_to :  u8,
                                      device_id :  u8) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_input_get_feedback_control (c : *mut ffi::base::xcb_connection_t,
                                          device_id :  u8) -> xcb_input_get_feedback_control_cookie_t;

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
pub fn xcb_input_get_feedback_control_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                    device_id :  u8) -> xcb_input_get_feedback_control_cookie_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_input_get_feedback_control_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_input_get_feedback_control_reply (c : *mut ffi::base::xcb_connection_t,
                                                cookie : xcb_input_get_feedback_control_cookie_t,
                                                e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_input_get_feedback_control_reply_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_input_feedback_state_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_input_feedback_state_t)
 *
 *
 */
pub fn xcb_input_feedback_state_next (i:*mut xcb_input_feedback_state_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_input_feedback_state_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_feedback_state_end (i:xcb_input_feedback_state_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_input_kbd_feedback_state_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_input_kbd_feedback_state_t)
 *
 *
 */
pub fn xcb_input_kbd_feedback_state_next (i:*mut xcb_input_kbd_feedback_state_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_input_kbd_feedback_state_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_kbd_feedback_state_end (i:xcb_input_kbd_feedback_state_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_input_ptr_feedback_state_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_input_ptr_feedback_state_t)
 *
 *
 */
pub fn xcb_input_ptr_feedback_state_next (i:*mut xcb_input_ptr_feedback_state_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_input_ptr_feedback_state_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_ptr_feedback_state_end (i:xcb_input_ptr_feedback_state_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_input_integer_feedback_state_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_input_integer_feedback_state_t)
 *
 *
 */
pub fn xcb_input_integer_feedback_state_next (i:*mut xcb_input_integer_feedback_state_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_input_integer_feedback_state_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_integer_feedback_state_end (i:xcb_input_integer_feedback_state_iterator_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_input_string_feedback_state_sizeof (_buffer :  *mut c_void) -> c_int;

pub fn xcb_input_string_feedback_state_keysyms (R : *mut xcb_input_string_feedback_state_t) -> *mut ffi::xproto::xcb_keysym_t;


pub fn xcb_input_string_feedback_state_keysyms_length (R : *mut xcb_input_string_feedback_state_t) -> c_int;


pub fn xcb_input_string_feedback_state_keysyms_end (R : *mut xcb_input_string_feedback_state_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_input_string_feedback_state_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_input_string_feedback_state_t)
 *
 *
 */
pub fn xcb_input_string_feedback_state_next (i:*mut xcb_input_string_feedback_state_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_input_string_feedback_state_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_string_feedback_state_end (i:xcb_input_string_feedback_state_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_input_bell_feedback_state_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_input_bell_feedback_state_t)
 *
 *
 */
pub fn xcb_input_bell_feedback_state_next (i:*mut xcb_input_bell_feedback_state_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_input_bell_feedback_state_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_bell_feedback_state_end (i:xcb_input_bell_feedback_state_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_input_led_feedback_state_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_input_led_feedback_state_t)
 *
 *
 */
pub fn xcb_input_led_feedback_state_next (i:*mut xcb_input_led_feedback_state_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_input_led_feedback_state_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_led_feedback_state_end (i:xcb_input_led_feedback_state_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_input_feedback_ctl_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_input_feedback_ctl_t)
 *
 *
 */
pub fn xcb_input_feedback_ctl_next (i:*mut xcb_input_feedback_ctl_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_input_feedback_ctl_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_feedback_ctl_end (i:xcb_input_feedback_ctl_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_input_kbd_feedback_ctl_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_input_kbd_feedback_ctl_t)
 *
 *
 */
pub fn xcb_input_kbd_feedback_ctl_next (i:*mut xcb_input_kbd_feedback_ctl_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_input_kbd_feedback_ctl_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_kbd_feedback_ctl_end (i:xcb_input_kbd_feedback_ctl_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_input_ptr_feedback_ctl_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_input_ptr_feedback_ctl_t)
 *
 *
 */
pub fn xcb_input_ptr_feedback_ctl_next (i:*mut xcb_input_ptr_feedback_ctl_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_input_ptr_feedback_ctl_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_ptr_feedback_ctl_end (i:xcb_input_ptr_feedback_ctl_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_input_integer_feedback_ctl_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_input_integer_feedback_ctl_t)
 *
 *
 */
pub fn xcb_input_integer_feedback_ctl_next (i:*mut xcb_input_integer_feedback_ctl_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_input_integer_feedback_ctl_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_integer_feedback_ctl_end (i:xcb_input_integer_feedback_ctl_iterator_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_input_string_feedback_ctl_sizeof (_buffer :  *mut c_void) -> c_int;

pub fn xcb_input_string_feedback_ctl_keysyms (R : *mut xcb_input_string_feedback_ctl_t) -> *mut ffi::xproto::xcb_keysym_t;


pub fn xcb_input_string_feedback_ctl_keysyms_length (R : *mut xcb_input_string_feedback_ctl_t) -> c_int;


pub fn xcb_input_string_feedback_ctl_keysyms_end (R : *mut xcb_input_string_feedback_ctl_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_input_string_feedback_ctl_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_input_string_feedback_ctl_t)
 *
 *
 */
pub fn xcb_input_string_feedback_ctl_next (i:*mut xcb_input_string_feedback_ctl_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_input_string_feedback_ctl_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_string_feedback_ctl_end (i:xcb_input_string_feedback_ctl_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_input_bell_feedback_ctl_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_input_bell_feedback_ctl_t)
 *
 *
 */
pub fn xcb_input_bell_feedback_ctl_next (i:*mut xcb_input_bell_feedback_ctl_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_input_bell_feedback_ctl_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_bell_feedback_ctl_end (i:xcb_input_bell_feedback_ctl_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_input_led_feedback_ctl_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_input_led_feedback_ctl_t)
 *
 *
 */
pub fn xcb_input_led_feedback_ctl_next (i:*mut xcb_input_led_feedback_ctl_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_input_led_feedback_ctl_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_led_feedback_ctl_end (i:xcb_input_led_feedback_ctl_iterator_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_input_get_device_key_mapping_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_input_get_device_key_mapping (c : *mut ffi::base::xcb_connection_t,
                                            device_id :  u8,
                                            first_keycode :  xcb_input_key_code_t,
                                            count :  u8) -> xcb_input_get_device_key_mapping_cookie_t;

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
pub fn xcb_input_get_device_key_mapping_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                      device_id :  u8,
                                                      first_keycode :  xcb_input_key_code_t,
                                                      count :  u8) -> xcb_input_get_device_key_mapping_cookie_t;

pub fn xcb_input_get_device_key_mapping_keysyms (R : *mut xcb_input_get_device_key_mapping_reply_t) -> *mut ffi::xproto::xcb_keysym_t;


pub fn xcb_input_get_device_key_mapping_keysyms_length (R : *mut xcb_input_get_device_key_mapping_reply_t) -> c_int;


pub fn xcb_input_get_device_key_mapping_keysyms_end (R : *mut xcb_input_get_device_key_mapping_reply_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_input_get_device_key_mapping_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_input_get_device_key_mapping_reply (c : *mut ffi::base::xcb_connection_t,
                                                  cookie : xcb_input_get_device_key_mapping_cookie_t,
                                                  e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_input_get_device_key_mapping_reply_t;

pub fn xcb_input_change_device_key_mapping_sizeof (_buffer :  *mut c_void) -> c_int;

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
pub fn xcb_input_change_device_key_mapping_checked (c : *mut ffi::base::xcb_connection_t,
                                                       device_id :  u8,
                                                       first_keycode :  xcb_input_key_code_t,
                                                       keysyms_per_keycode :  u8,
                                                       keycode_count :  u8,
                                                       keysyms : *mut ffi::xproto::xcb_keysym_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_input_change_device_key_mapping (c : *mut ffi::base::xcb_connection_t,
                                               device_id :  u8,
                                               first_keycode :  xcb_input_key_code_t,
                                               keysyms_per_keycode :  u8,
                                               keycode_count :  u8,
                                               keysyms : *mut ffi::xproto::xcb_keysym_t) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_input_get_device_modifier_mapping_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_input_get_device_modifier_mapping (c : *mut ffi::base::xcb_connection_t,
                                                 device_id :  u8) -> xcb_input_get_device_modifier_mapping_cookie_t;

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
pub fn xcb_input_get_device_modifier_mapping_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                           device_id :  u8) -> xcb_input_get_device_modifier_mapping_cookie_t;

pub fn xcb_input_get_device_modifier_mapping_keymaps (R : *mut xcb_input_get_device_modifier_mapping_reply_t) -> *mut u8;


pub fn xcb_input_get_device_modifier_mapping_keymaps_length (R : *mut xcb_input_get_device_modifier_mapping_reply_t) -> c_int;


pub fn xcb_input_get_device_modifier_mapping_keymaps_end (R : *mut xcb_input_get_device_modifier_mapping_reply_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_input_get_device_modifier_mapping_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_input_get_device_modifier_mapping_reply (c : *mut ffi::base::xcb_connection_t,
                                                       cookie : xcb_input_get_device_modifier_mapping_cookie_t,
                                                       e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_input_get_device_modifier_mapping_reply_t;

pub fn xcb_input_set_device_modifier_mapping_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_input_set_device_modifier_mapping (c : *mut ffi::base::xcb_connection_t,
                                                 device_id :  u8,
                                                 keycodes_per_modifier :  u8,
                                                 keymaps : *mut u8) -> xcb_input_set_device_modifier_mapping_cookie_t;

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
pub fn xcb_input_set_device_modifier_mapping_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                           device_id :  u8,
                                                           keycodes_per_modifier :  u8,
                                                           keymaps : *mut u8) -> xcb_input_set_device_modifier_mapping_cookie_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_input_set_device_modifier_mapping_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_input_set_device_modifier_mapping_reply (c : *mut ffi::base::xcb_connection_t,
                                                       cookie : xcb_input_set_device_modifier_mapping_cookie_t,
                                                       e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_input_set_device_modifier_mapping_reply_t;

pub fn xcb_input_get_device_button_mapping_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_input_get_device_button_mapping (c : *mut ffi::base::xcb_connection_t,
                                               device_id :  u8) -> xcb_input_get_device_button_mapping_cookie_t;

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
pub fn xcb_input_get_device_button_mapping_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                         device_id :  u8) -> xcb_input_get_device_button_mapping_cookie_t;

pub fn xcb_input_get_device_button_mapping_map (R : *mut xcb_input_get_device_button_mapping_reply_t) -> *mut u8;


pub fn xcb_input_get_device_button_mapping_map_length (R : *mut xcb_input_get_device_button_mapping_reply_t) -> c_int;


pub fn xcb_input_get_device_button_mapping_map_end (R : *mut xcb_input_get_device_button_mapping_reply_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_input_get_device_button_mapping_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_input_get_device_button_mapping_reply (c : *mut ffi::base::xcb_connection_t,
                                                     cookie : xcb_input_get_device_button_mapping_cookie_t,
                                                     e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_input_get_device_button_mapping_reply_t;

pub fn xcb_input_set_device_button_mapping_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_input_set_device_button_mapping (c : *mut ffi::base::xcb_connection_t,
                                               device_id :  u8,
                                               map_size :  u8,
                                               map : *mut u8) -> xcb_input_set_device_button_mapping_cookie_t;

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
pub fn xcb_input_set_device_button_mapping_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                         device_id :  u8,
                                                         map_size :  u8,
                                                         map : *mut u8) -> xcb_input_set_device_button_mapping_cookie_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_input_set_device_button_mapping_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_input_set_device_button_mapping_reply (c : *mut ffi::base::xcb_connection_t,
                                                     cookie : xcb_input_set_device_button_mapping_cookie_t,
                                                     e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_input_set_device_button_mapping_reply_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_input_query_device_state (c : *mut ffi::base::xcb_connection_t,
                                        device_id :  u8) -> xcb_input_query_device_state_cookie_t;

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
pub fn xcb_input_query_device_state_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                  device_id :  u8) -> xcb_input_query_device_state_cookie_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_input_query_device_state_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_input_query_device_state_reply (c : *mut ffi::base::xcb_connection_t,
                                              cookie : xcb_input_query_device_state_cookie_t,
                                              e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_input_query_device_state_reply_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_input_input_state_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_input_input_state_t)
 *
 *
 */
pub fn xcb_input_input_state_next (i:*mut xcb_input_input_state_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_input_input_state_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_input_state_end (i:xcb_input_input_state_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_input_key_state_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_input_key_state_t)
 *
 *
 */
pub fn xcb_input_key_state_next (i:*mut xcb_input_key_state_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_input_key_state_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_key_state_end (i:xcb_input_key_state_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_input_button_state_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_input_button_state_t)
 *
 *
 */
pub fn xcb_input_button_state_next (i:*mut xcb_input_button_state_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_input_button_state_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_button_state_end (i:xcb_input_button_state_iterator_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_input_valuator_state_sizeof (_buffer :  *mut c_void) -> c_int;

pub fn xcb_input_valuator_state_valuators (R : *mut xcb_input_valuator_state_t) -> *mut u32;


pub fn xcb_input_valuator_state_valuators_length (R : *mut xcb_input_valuator_state_t) -> c_int;


pub fn xcb_input_valuator_state_valuators_end (R : *mut xcb_input_valuator_state_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_input_valuator_state_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_input_valuator_state_t)
 *
 *
 */
pub fn xcb_input_valuator_state_next (i:*mut xcb_input_valuator_state_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_input_valuator_state_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_valuator_state_end (i:xcb_input_valuator_state_iterator_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_input_send_extension_event_sizeof (_buffer :  *mut c_void) -> c_int;

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
pub fn xcb_input_send_extension_event_checked (c : *mut ffi::base::xcb_connection_t,
                                                  destination :  ffi::xproto::xcb_window_t,
                                                  device_id :  u8,
                                                  propagate :  u8,
                                                  num_classes :  u16,
                                                  num_events :  u8,
                                                  events : *mut c_char,
                                                  classes : *mut xcb_input_event_class_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_input_send_extension_event (c : *mut ffi::base::xcb_connection_t,
                                          destination :  ffi::xproto::xcb_window_t,
                                          device_id :  u8,
                                          propagate :  u8,
                                          num_classes :  u16,
                                          num_events :  u8,
                                          events : *mut c_char,
                                          classes : *mut xcb_input_event_class_t) -> ffi::base::xcb_void_cookie_t;

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
pub fn xcb_input_device_bell_checked (c : *mut ffi::base::xcb_connection_t,
                                         device_id :  u8,
                                         feedback_id :  u8,
                                         feedback_class :  u8,
                                         percent :  i8) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_input_device_bell (c : *mut ffi::base::xcb_connection_t,
                                 device_id :  u8,
                                 feedback_id :  u8,
                                 feedback_class :  u8,
                                 percent :  i8) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_input_set_device_valuators_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_input_set_device_valuators (c : *mut ffi::base::xcb_connection_t,
                                          device_id :  u8,
                                          first_valuator :  u8,
                                          num_valuators :  u8,
                                          valuators : *mut i32) -> xcb_input_set_device_valuators_cookie_t;

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
pub fn xcb_input_set_device_valuators_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                    device_id :  u8,
                                                    first_valuator :  u8,
                                                    num_valuators :  u8,
                                                    valuators : *mut i32) -> xcb_input_set_device_valuators_cookie_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_input_set_device_valuators_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_input_set_device_valuators_reply (c : *mut ffi::base::xcb_connection_t,
                                                cookie : xcb_input_set_device_valuators_cookie_t,
                                                e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_input_set_device_valuators_reply_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_input_get_device_control (c : *mut ffi::base::xcb_connection_t,
                                        control_id :  u16,
                                        device_id :  u8) -> xcb_input_get_device_control_cookie_t;

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
pub fn xcb_input_get_device_control_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                  control_id :  u16,
                                                  device_id :  u8) -> xcb_input_get_device_control_cookie_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_input_get_device_control_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_input_get_device_control_reply (c : *mut ffi::base::xcb_connection_t,
                                              cookie : xcb_input_get_device_control_cookie_t,
                                              e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_input_get_device_control_reply_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_input_device_state_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_input_device_state_t)
 *
 *
 */
pub fn xcb_input_device_state_next (i:*mut xcb_input_device_state_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_input_device_state_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_device_state_end (i:xcb_input_device_state_iterator_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_input_device_resolution_state_sizeof (_buffer :  *mut c_void) -> c_int;

pub fn xcb_input_device_resolution_state_resolution_values (R : *mut xcb_input_device_resolution_state_t) -> *mut u32;


pub fn xcb_input_device_resolution_state_resolution_values_length (R : *mut xcb_input_device_resolution_state_t) -> c_int;


pub fn xcb_input_device_resolution_state_resolution_values_end (R : *mut xcb_input_device_resolution_state_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_input_device_resolution_state_resolution_min (R : *mut xcb_input_device_resolution_state_t) -> *mut u32;


pub fn xcb_input_device_resolution_state_resolution_min_length (R : *mut xcb_input_device_resolution_state_t) -> c_int;


pub fn xcb_input_device_resolution_state_resolution_min_end (R : *mut xcb_input_device_resolution_state_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_input_device_resolution_state_resolution_max (R : *mut xcb_input_device_resolution_state_t) -> *mut u32;


pub fn xcb_input_device_resolution_state_resolution_max_length (R : *mut xcb_input_device_resolution_state_t) -> c_int;


pub fn xcb_input_device_resolution_state_resolution_max_end (R : *mut xcb_input_device_resolution_state_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_input_device_resolution_state_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_input_device_resolution_state_t)
 *
 *
 */
pub fn xcb_input_device_resolution_state_next (i:*mut xcb_input_device_resolution_state_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_input_device_resolution_state_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_device_resolution_state_end (i:xcb_input_device_resolution_state_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_input_device_abs_calib_state_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_input_device_abs_calib_state_t)
 *
 *
 */
pub fn xcb_input_device_abs_calib_state_next (i:*mut xcb_input_device_abs_calib_state_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_input_device_abs_calib_state_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_device_abs_calib_state_end (i:xcb_input_device_abs_calib_state_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_input_device_abs_area_state_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_input_device_abs_area_state_t)
 *
 *
 */
pub fn xcb_input_device_abs_area_state_next (i:*mut xcb_input_device_abs_area_state_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_input_device_abs_area_state_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_device_abs_area_state_end (i:xcb_input_device_abs_area_state_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_input_device_core_state_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_input_device_core_state_t)
 *
 *
 */
pub fn xcb_input_device_core_state_next (i:*mut xcb_input_device_core_state_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_input_device_core_state_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_device_core_state_end (i:xcb_input_device_core_state_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_input_device_enable_state_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_input_device_enable_state_t)
 *
 *
 */
pub fn xcb_input_device_enable_state_next (i:*mut xcb_input_device_enable_state_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_input_device_enable_state_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_device_enable_state_end (i:xcb_input_device_enable_state_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_input_device_ctl_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_input_device_ctl_t)
 *
 *
 */
pub fn xcb_input_device_ctl_next (i:*mut xcb_input_device_ctl_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_input_device_ctl_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_device_ctl_end (i:xcb_input_device_ctl_iterator_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_input_device_resolution_ctl_sizeof (_buffer :  *mut c_void) -> c_int;

pub fn xcb_input_device_resolution_ctl_resolution_values (R : *mut xcb_input_device_resolution_ctl_t) -> *mut u32;


pub fn xcb_input_device_resolution_ctl_resolution_values_length (R : *mut xcb_input_device_resolution_ctl_t) -> c_int;


pub fn xcb_input_device_resolution_ctl_resolution_values_end (R : *mut xcb_input_device_resolution_ctl_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_input_device_resolution_ctl_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_input_device_resolution_ctl_t)
 *
 *
 */
pub fn xcb_input_device_resolution_ctl_next (i:*mut xcb_input_device_resolution_ctl_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_input_device_resolution_ctl_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_device_resolution_ctl_end (i:xcb_input_device_resolution_ctl_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_input_device_abs_calib_ctl_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_input_device_abs_calib_ctl_t)
 *
 *
 */
pub fn xcb_input_device_abs_calib_ctl_next (i:*mut xcb_input_device_abs_calib_ctl_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_input_device_abs_calib_ctl_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_device_abs_calib_ctl_end (i:xcb_input_device_abs_calib_ctl_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_input_device_abs_area_ctrl_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_input_device_abs_area_ctrl_t)
 *
 *
 */
pub fn xcb_input_device_abs_area_ctrl_next (i:*mut xcb_input_device_abs_area_ctrl_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_input_device_abs_area_ctrl_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_device_abs_area_ctrl_end (i:xcb_input_device_abs_area_ctrl_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_input_device_core_ctrl_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_input_device_core_ctrl_t)
 *
 *
 */
pub fn xcb_input_device_core_ctrl_next (i:*mut xcb_input_device_core_ctrl_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_input_device_core_ctrl_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_device_core_ctrl_end (i:xcb_input_device_core_ctrl_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_input_device_enable_ctrl_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_input_device_enable_ctrl_t)
 *
 *
 */
pub fn xcb_input_device_enable_ctrl_next (i:*mut xcb_input_device_enable_ctrl_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_input_device_enable_ctrl_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_device_enable_ctrl_end (i:xcb_input_device_enable_ctrl_iterator_t) -> ffi::base::xcb_generic_iterator_t;
}

