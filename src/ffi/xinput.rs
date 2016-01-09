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

pub static INPUT_MAJOR_VERSION : c_uint = 1;
pub static INPUT_MINOR_VERSION : c_uint = 4;

pub type key_code = u8;
/**
 * @brief key_code_iterator
 **/
#[repr(C)]
pub struct key_code_iterator {
    pub data : *mut key_code,
    pub rem  : c_int,
    pub index: c_int
}


pub type event_class = u32;
/**
 * @brief event_class_iterator
 **/
#[repr(C)]
pub struct event_class_iterator {
    pub data : *mut event_class,
    pub rem  : c_int,
    pub index: c_int
}


#[derive(Copy, Clone)]
#[repr(C)]
pub struct get_extension_version_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct get_extension_version_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub name_len :       u16,
     pub pad0 :           [u8; 2]
}

impl Copy for get_extension_version_request {}
impl Clone for get_extension_version_request {
    fn clone(&self) -> get_extension_version_request { *self }
}

#[repr(C)]
pub struct get_extension_version_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub server_major :    u16,
     pub server_minor :    u16,
     pub present :         u8,
     pub pad1 :            [u8; 19]
}

impl Copy for get_extension_version_reply {}
impl Clone for get_extension_version_reply {
    fn clone(&self) -> get_extension_version_reply { *self }
}

#[repr(C)]
pub struct device_info {
     pub device_type :      ffi::xproto::atom,
     pub device_id :        u8,
     pub num_class_info :   u8,
     pub device_use :       u8,
     pub pad0 :             u8
}

impl Copy for device_info {}
impl Clone for device_info {
    fn clone(&self) -> device_info { *self }
}
/**
 * @brief device_info_iterator
 **/
#[repr(C)]
pub struct device_info_iterator {
    pub data : *mut device_info,
    pub rem  : c_int,
    pub index: c_int
}


#[derive(Copy, Clone)]
#[repr(C)]
pub struct list_input_devices_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct list_input_devices_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16
}

impl Copy for list_input_devices_request {}
impl Clone for list_input_devices_request {
    fn clone(&self) -> list_input_devices_request { *self }
}

#[repr(C)]
pub struct list_input_devices_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub devices_len :     u8,
     pub pad1 :            [u8; 23]
}

impl Copy for list_input_devices_reply {}
impl Clone for list_input_devices_reply {
    fn clone(&self) -> list_input_devices_reply { *self }
}

#[repr(C)]
pub struct input_info {
     pub class_id :   u8,
     pub len :        u8
}

impl Copy for input_info {}
impl Clone for input_info {
    fn clone(&self) -> input_info { *self }
}
/**
 * @brief input_info_iterator
 **/
#[repr(C)]
pub struct input_info_iterator {
    pub data : *mut input_info,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct key_info {
     pub class_id :      u8,
     pub len :           u8,
     pub min_keycode :   key_code,
     pub max_keycode :   key_code,
     pub num_keys :      u16,
     pub pad0 :          [u8; 2]
}

impl Copy for key_info {}
impl Clone for key_info {
    fn clone(&self) -> key_info { *self }
}
/**
 * @brief key_info_iterator
 **/
#[repr(C)]
pub struct key_info_iterator {
    pub data : *mut key_info,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct button_info {
     pub class_id :      u8,
     pub len :           u8,
     pub num_buttons :   u16
}

impl Copy for button_info {}
impl Clone for button_info {
    fn clone(&self) -> button_info { *self }
}
/**
 * @brief button_info_iterator
 **/
#[repr(C)]
pub struct button_info_iterator {
    pub data : *mut button_info,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct axis_info {
     pub resolution :   u32,
     pub minimum :      i32,
     pub maximum :      i32
}

impl Copy for axis_info {}
impl Clone for axis_info {
    fn clone(&self) -> axis_info { *self }
}
/**
 * @brief axis_info_iterator
 **/
#[repr(C)]
pub struct axis_info_iterator {
    pub data : *mut axis_info,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct valuator_info {
     pub class_id :      u8,
     pub len :           u8,
     pub axes_len :      u8,
     pub mode :          u8,
     pub motion_size :   u32
}

impl Copy for valuator_info {}
impl Clone for valuator_info {
    fn clone(&self) -> valuator_info { *self }
}
/**
 * @brief valuator_info_iterator
 **/
#[repr(C)]
pub struct valuator_info_iterator {
    pub data : *mut valuator_info,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct input_class_info {
     pub class_id :          u8,
     pub event_type_base :   u8
}

impl Copy for input_class_info {}
impl Clone for input_class_info {
    fn clone(&self) -> input_class_info { *self }
}
/**
 * @brief input_class_info_iterator
 **/
#[repr(C)]
pub struct input_class_info_iterator {
    pub data : *mut input_class_info,
    pub rem  : c_int,
    pub index: c_int
}


#[derive(Copy, Clone)]
#[repr(C)]
pub struct open_device_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct open_device_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub device_id :      u8,
     pub pad0 :           [u8; 3]
}

impl Copy for open_device_request {}
impl Clone for open_device_request {
    fn clone(&self) -> open_device_request { *self }
}

#[repr(C)]
pub struct open_device_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub num_classes :     u8,
     pub pad1 :            [u8; 23]
}

impl Copy for open_device_reply {}
impl Clone for open_device_reply {
    fn clone(&self) -> open_device_reply { *self }
}


#[repr(C)]
pub struct close_device_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub device_id :      u8,
     pub pad0 :           [u8; 3]
}

impl Copy for close_device_request {}
impl Clone for close_device_request {
    fn clone(&self) -> close_device_request { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct set_device_mode_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct set_device_mode_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub device_id :      u8,
     pub mode :           u8,
     pub pad0 :           [u8; 2]
}

impl Copy for set_device_mode_request {}
impl Clone for set_device_mode_request {
    fn clone(&self) -> set_device_mode_request { *self }
}

#[repr(C)]
pub struct set_device_mode_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub status :          u8,
     pub pad1 :            [u8; 23]
}

impl Copy for set_device_mode_reply {}
impl Clone for set_device_mode_reply {
    fn clone(&self) -> set_device_mode_reply { *self }
}


#[repr(C)]
pub struct select_extension_event_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub window :         ffi::xproto::window,
     pub num_classes :    u16,
     pub pad0 :           [u8; 2]
}

impl Copy for select_extension_event_request {}
impl Clone for select_extension_event_request {
    fn clone(&self) -> select_extension_event_request { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct get_selected_extension_events_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct get_selected_extension_events_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub window :         ffi::xproto::window
}

impl Copy for get_selected_extension_events_request {}
impl Clone for get_selected_extension_events_request {
    fn clone(&self) -> get_selected_extension_events_request { *self }
}

#[repr(C)]
pub struct get_selected_extension_events_reply {
     pub response_type :      u8,
     pub pad0 :               u8,
     pub sequence :           u16,
     pub length :             u32,
     pub num_this_classes :   u16,
     pub num_all_classes :    u16,
     pub pad1 :               [u8; 20]
}

impl Copy for get_selected_extension_events_reply {}
impl Clone for get_selected_extension_events_reply {
    fn clone(&self) -> get_selected_extension_events_reply { *self }
}


#[repr(C)]
pub struct change_device_dont_propagate_list_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub window :         ffi::xproto::window,
     pub num_classes :    u16,
     pub mode :           u8,
     pub pad0 :           u8
}

impl Copy for change_device_dont_propagate_list_request {}
impl Clone for change_device_dont_propagate_list_request {
    fn clone(&self) -> change_device_dont_propagate_list_request { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct get_device_dont_propagate_list_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct get_device_dont_propagate_list_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub window :         ffi::xproto::window
}

impl Copy for get_device_dont_propagate_list_request {}
impl Clone for get_device_dont_propagate_list_request {
    fn clone(&self) -> get_device_dont_propagate_list_request { *self }
}

#[repr(C)]
pub struct get_device_dont_propagate_list_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub num_classes :     u16,
     pub pad1 :            [u8; 22]
}

impl Copy for get_device_dont_propagate_list_reply {}
impl Clone for get_device_dont_propagate_list_reply {
    fn clone(&self) -> get_device_dont_propagate_list_reply { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct get_device_motion_events_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct get_device_motion_events_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub start :          ffi::xproto::timestamp,
     pub stop :           ffi::xproto::timestamp,
     pub device_id :      u8
}

impl Copy for get_device_motion_events_request {}
impl Clone for get_device_motion_events_request {
    fn clone(&self) -> get_device_motion_events_request { *self }
}

#[repr(C)]
pub struct get_device_motion_events_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub num_coords :      u32,
     pub num_axes :        u8,
     pub device_mode :     u8,
     pub pad1 :            [u8; 18]
}

impl Copy for get_device_motion_events_reply {}
impl Clone for get_device_motion_events_reply {
    fn clone(&self) -> get_device_motion_events_reply { *self }
}

#[repr(C)]
pub struct device_time_coord {
     pub time :   ffi::xproto::timestamp
}

impl Copy for device_time_coord {}
impl Clone for device_time_coord {
    fn clone(&self) -> device_time_coord { *self }
}
/**
 * @brief device_time_coord_iterator
 **/
#[repr(C)]
pub struct device_time_coord_iterator {
    pub data : *mut device_time_coord,
    pub rem  : c_int,
    pub index: c_int
}


#[derive(Copy, Clone)]
#[repr(C)]
pub struct change_keyboard_device_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct change_keyboard_device_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub device_id :      u8,
     pub pad0 :           [u8; 3]
}

impl Copy for change_keyboard_device_request {}
impl Clone for change_keyboard_device_request {
    fn clone(&self) -> change_keyboard_device_request { *self }
}

#[repr(C)]
pub struct change_keyboard_device_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub status :          u8,
     pub pad1 :            [u8; 23]
}

impl Copy for change_keyboard_device_reply {}
impl Clone for change_keyboard_device_reply {
    fn clone(&self) -> change_keyboard_device_reply { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct change_pointer_device_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct change_pointer_device_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub x_axis :         u8,
     pub y_axis :         u8,
     pub device_id :      u8,
     pub pad0 :           u8
}

impl Copy for change_pointer_device_request {}
impl Clone for change_pointer_device_request {
    fn clone(&self) -> change_pointer_device_request { *self }
}

#[repr(C)]
pub struct change_pointer_device_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub status :          u8,
     pub pad1 :            [u8; 23]
}

impl Copy for change_pointer_device_reply {}
impl Clone for change_pointer_device_reply {
    fn clone(&self) -> change_pointer_device_reply { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct grab_device_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct grab_device_request {
     pub major_opcode :        u8,
     pub minor_opcode :        u8,
     pub length :              u16,
     pub grab_window :         ffi::xproto::window,
     pub time :                ffi::xproto::timestamp,
     pub num_classes :         u16,
     pub this_device_mode :    u8,
     pub other_device_mode :   u8,
     pub owner_events :        u8,
     pub device_id :           u8,
     pub pad0 :                [u8; 2]
}

impl Copy for grab_device_request {}
impl Clone for grab_device_request {
    fn clone(&self) -> grab_device_request { *self }
}

#[repr(C)]
pub struct grab_device_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub status :          u8,
     pub pad1 :            [u8; 23]
}

impl Copy for grab_device_reply {}
impl Clone for grab_device_reply {
    fn clone(&self) -> grab_device_reply { *self }
}


#[repr(C)]
pub struct ungrab_device_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub time :           ffi::xproto::timestamp,
     pub device_id :      u8
}

impl Copy for ungrab_device_request {}
impl Clone for ungrab_device_request {
    fn clone(&self) -> ungrab_device_request { *self }
}


#[repr(C)]
pub struct grab_device_key_request {
     pub major_opcode :        u8,
     pub minor_opcode :        u8,
     pub length :              u16,
     pub grab_window :         ffi::xproto::window,
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

impl Copy for grab_device_key_request {}
impl Clone for grab_device_key_request {
    fn clone(&self) -> grab_device_key_request { *self }
}


#[repr(C)]
pub struct ungrab_device_key_request {
     pub major_opcode :      u8,
     pub minor_opcode :      u8,
     pub length :            u16,
     pub grabWindow :        ffi::xproto::window,
     pub modifiers :         u16,
     pub modifier_device :   u8,
     pub key :               u8,
     pub grabbed_device :    u8
}

impl Copy for ungrab_device_key_request {}
impl Clone for ungrab_device_key_request {
    fn clone(&self) -> ungrab_device_key_request { *self }
}


#[repr(C)]
pub struct grab_device_button_request {
     pub major_opcode :        u8,
     pub minor_opcode :        u8,
     pub length :              u16,
     pub grab_window :         ffi::xproto::window,
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

impl Copy for grab_device_button_request {}
impl Clone for grab_device_button_request {
    fn clone(&self) -> grab_device_button_request { *self }
}


#[repr(C)]
pub struct ungrab_device_button_request {
     pub major_opcode :      u8,
     pub minor_opcode :      u8,
     pub length :            u16,
     pub grab_window :       ffi::xproto::window,
     pub modifiers :         u16,
     pub modifier_device :   u8,
     pub button :            u8,
     pub grabbed_device :    u8
}

impl Copy for ungrab_device_button_request {}
impl Clone for ungrab_device_button_request {
    fn clone(&self) -> ungrab_device_button_request { *self }
}


#[repr(C)]
pub struct allow_device_events_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub time :           ffi::xproto::timestamp,
     pub mode :           u8,
     pub device_id :      u8
}

impl Copy for allow_device_events_request {}
impl Clone for allow_device_events_request {
    fn clone(&self) -> allow_device_events_request { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct get_device_focus_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct get_device_focus_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub device_id :      u8,
     pub pad0 :           [u8; 3]
}

impl Copy for get_device_focus_request {}
impl Clone for get_device_focus_request {
    fn clone(&self) -> get_device_focus_request { *self }
}

#[repr(C)]
pub struct get_device_focus_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub focus :           ffi::xproto::window,
     pub time :            ffi::xproto::timestamp,
     pub revert_to :       u8,
     pub pad1 :            [u8; 15]
}

impl Copy for get_device_focus_reply {}
impl Clone for get_device_focus_reply {
    fn clone(&self) -> get_device_focus_reply { *self }
}


#[repr(C)]
pub struct set_device_focus_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub focus :          ffi::xproto::window,
     pub time :           ffi::xproto::timestamp,
     pub revert_to :      u8,
     pub device_id :      u8
}

impl Copy for set_device_focus_request {}
impl Clone for set_device_focus_request {
    fn clone(&self) -> set_device_focus_request { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct get_feedback_control_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct get_feedback_control_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub device_id :      u8,
     pub pad0 :           [u8; 3]
}

impl Copy for get_feedback_control_request {}
impl Clone for get_feedback_control_request {
    fn clone(&self) -> get_feedback_control_request { *self }
}

#[repr(C)]
pub struct get_feedback_control_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub num_feedback :    u16,
     pub pad1 :            [u8; 22]
}

impl Copy for get_feedback_control_reply {}
impl Clone for get_feedback_control_reply {
    fn clone(&self) -> get_feedback_control_reply { *self }
}

#[repr(C)]
pub struct feedback_state {
     pub class_id :   u8,
     pub id :         u8,
     pub len :        u16
}

impl Copy for feedback_state {}
impl Clone for feedback_state {
    fn clone(&self) -> feedback_state { *self }
}
/**
 * @brief feedback_state_iterator
 **/
#[repr(C)]
pub struct feedback_state_iterator {
    pub data : *mut feedback_state,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct kbd_feedback_state {
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

impl Copy for kbd_feedback_state {}
impl Clone for kbd_feedback_state {
    fn clone(&self) -> kbd_feedback_state { *self }
}
/**
 * @brief kbd_feedback_state_iterator
 **/
#[repr(C)]
pub struct kbd_feedback_state_iterator {
    pub data : *mut kbd_feedback_state,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct ptr_feedback_state {
     pub class_id :      u8,
     pub id :            u8,
     pub len :           u16,
     pub pad0 :          [u8; 2],
     pub accel_num :     u16,
     pub accel_denom :   u16,
     pub threshold :     u16
}

impl Copy for ptr_feedback_state {}
impl Clone for ptr_feedback_state {
    fn clone(&self) -> ptr_feedback_state { *self }
}
/**
 * @brief ptr_feedback_state_iterator
 **/
#[repr(C)]
pub struct ptr_feedback_state_iterator {
    pub data : *mut ptr_feedback_state,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct integer_feedback_state {
     pub class_id :     u8,
     pub id :           u8,
     pub len :          u16,
     pub resolution :   u32,
     pub min_value :    i32,
     pub max_value :    i32
}

impl Copy for integer_feedback_state {}
impl Clone for integer_feedback_state {
    fn clone(&self) -> integer_feedback_state { *self }
}
/**
 * @brief integer_feedback_state_iterator
 **/
#[repr(C)]
pub struct integer_feedback_state_iterator {
    pub data : *mut integer_feedback_state,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct string_feedback_state {
     pub class_id :      u8,
     pub id :            u8,
     pub len :           u16,
     pub max_symbols :   u16,
     pub num_keysyms :   u16
}

impl Copy for string_feedback_state {}
impl Clone for string_feedback_state {
    fn clone(&self) -> string_feedback_state { *self }
}
/**
 * @brief string_feedback_state_iterator
 **/
#[repr(C)]
pub struct string_feedback_state_iterator {
    pub data : *mut string_feedback_state,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct bell_feedback_state {
     pub class_id :   u8,
     pub id :         u8,
     pub len :        u16,
     pub percent :    u8,
     pub pad0 :       [u8; 3],
     pub pitch :      u16,
     pub duration :   u16
}

impl Copy for bell_feedback_state {}
impl Clone for bell_feedback_state {
    fn clone(&self) -> bell_feedback_state { *self }
}
/**
 * @brief bell_feedback_state_iterator
 **/
#[repr(C)]
pub struct bell_feedback_state_iterator {
    pub data : *mut bell_feedback_state,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct led_feedback_state {
     pub class_id :     u8,
     pub id :           u8,
     pub len :          u16,
     pub led_mask :     u32,
     pub led_values :   u32
}

impl Copy for led_feedback_state {}
impl Clone for led_feedback_state {
    fn clone(&self) -> led_feedback_state { *self }
}
/**
 * @brief led_feedback_state_iterator
 **/
#[repr(C)]
pub struct led_feedback_state_iterator {
    pub data : *mut led_feedback_state,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct feedback_ctl {
     pub class_id :   u8,
     pub id :         u8,
     pub len :        u16
}

impl Copy for feedback_ctl {}
impl Clone for feedback_ctl {
    fn clone(&self) -> feedback_ctl { *self }
}
/**
 * @brief feedback_ctl_iterator
 **/
#[repr(C)]
pub struct feedback_ctl_iterator {
    pub data : *mut feedback_ctl,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct kbd_feedback_ctl {
     pub class_id :            u8,
     pub id :                  u8,
     pub len :                 u16,
     pub key :                 key_code,
     pub auto_repeat_mode :    u8,
     pub key_click_percent :   i8,
     pub bell_percent :        i8,
     pub bell_pitch :          i16,
     pub bell_duration :       i16,
     pub led_mask :            u32,
     pub led_values :          u32
}

impl Copy for kbd_feedback_ctl {}
impl Clone for kbd_feedback_ctl {
    fn clone(&self) -> kbd_feedback_ctl { *self }
}
/**
 * @brief kbd_feedback_ctl_iterator
 **/
#[repr(C)]
pub struct kbd_feedback_ctl_iterator {
    pub data : *mut kbd_feedback_ctl,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct ptr_feedback_ctl {
     pub class_id :    u8,
     pub id :          u8,
     pub len :         u16,
     pub pad0 :        [u8; 2],
     pub num :         i16,
     pub denom :       i16,
     pub threshold :   i16
}

impl Copy for ptr_feedback_ctl {}
impl Clone for ptr_feedback_ctl {
    fn clone(&self) -> ptr_feedback_ctl { *self }
}
/**
 * @brief ptr_feedback_ctl_iterator
 **/
#[repr(C)]
pub struct ptr_feedback_ctl_iterator {
    pub data : *mut ptr_feedback_ctl,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct integer_feedback_ctl {
     pub class_id :         u8,
     pub id :               u8,
     pub len :              u16,
     pub int_to_display :   i32
}

impl Copy for integer_feedback_ctl {}
impl Clone for integer_feedback_ctl {
    fn clone(&self) -> integer_feedback_ctl { *self }
}
/**
 * @brief integer_feedback_ctl_iterator
 **/
#[repr(C)]
pub struct integer_feedback_ctl_iterator {
    pub data : *mut integer_feedback_ctl,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct string_feedback_ctl {
     pub class_id :      u8,
     pub id :            u8,
     pub len :           u16,
     pub pad0 :          [u8; 2],
     pub num_keysyms :   u16
}

impl Copy for string_feedback_ctl {}
impl Clone for string_feedback_ctl {
    fn clone(&self) -> string_feedback_ctl { *self }
}
/**
 * @brief string_feedback_ctl_iterator
 **/
#[repr(C)]
pub struct string_feedback_ctl_iterator {
    pub data : *mut string_feedback_ctl,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct bell_feedback_ctl {
     pub class_id :   u8,
     pub id :         u8,
     pub len :        u16,
     pub percent :    i8,
     pub pad0 :       [u8; 3],
     pub pitch :      i16,
     pub duration :   i16
}

impl Copy for bell_feedback_ctl {}
impl Clone for bell_feedback_ctl {
    fn clone(&self) -> bell_feedback_ctl { *self }
}
/**
 * @brief bell_feedback_ctl_iterator
 **/
#[repr(C)]
pub struct bell_feedback_ctl_iterator {
    pub data : *mut bell_feedback_ctl,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct led_feedback_ctl {
     pub class_id :     u8,
     pub id :           u8,
     pub len :          u16,
     pub led_mask :     u32,
     pub led_values :   u32
}

impl Copy for led_feedback_ctl {}
impl Clone for led_feedback_ctl {
    fn clone(&self) -> led_feedback_ctl { *self }
}
/**
 * @brief led_feedback_ctl_iterator
 **/
#[repr(C)]
pub struct led_feedback_ctl_iterator {
    pub data : *mut led_feedback_ctl,
    pub rem  : c_int,
    pub index: c_int
}


#[derive(Copy, Clone)]
#[repr(C)]
pub struct get_device_key_mapping_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct get_device_key_mapping_request {
     pub major_opcode :    u8,
     pub minor_opcode :    u8,
     pub length :          u16,
     pub device_id :       u8,
     pub first_keycode :   key_code,
     pub count :           u8
}

impl Copy for get_device_key_mapping_request {}
impl Clone for get_device_key_mapping_request {
    fn clone(&self) -> get_device_key_mapping_request { *self }
}

#[repr(C)]
pub struct get_device_key_mapping_reply {
     pub response_type :         u8,
     pub pad0 :                  u8,
     pub sequence :              u16,
     pub length :                u32,
     pub keysyms_per_keycode :   u8,
     pub pad1 :                  [u8; 23]
}

impl Copy for get_device_key_mapping_reply {}
impl Clone for get_device_key_mapping_reply {
    fn clone(&self) -> get_device_key_mapping_reply { *self }
}


#[repr(C)]
pub struct change_device_key_mapping_request {
     pub major_opcode :          u8,
     pub minor_opcode :          u8,
     pub length :                u16,
     pub device_id :             u8,
     pub first_keycode :         key_code,
     pub keysyms_per_keycode :   u8,
     pub keycode_count :         u8
}

impl Copy for change_device_key_mapping_request {}
impl Clone for change_device_key_mapping_request {
    fn clone(&self) -> change_device_key_mapping_request { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct get_device_modifier_mapping_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct get_device_modifier_mapping_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub device_id :      u8,
     pub pad0 :           [u8; 3]
}

impl Copy for get_device_modifier_mapping_request {}
impl Clone for get_device_modifier_mapping_request {
    fn clone(&self) -> get_device_modifier_mapping_request { *self }
}

#[repr(C)]
pub struct get_device_modifier_mapping_reply {
     pub response_type :           u8,
     pub pad0 :                    u8,
     pub sequence :                u16,
     pub length :                  u32,
     pub keycodes_per_modifier :   u8,
     pub pad1 :                    [u8; 23]
}

impl Copy for get_device_modifier_mapping_reply {}
impl Clone for get_device_modifier_mapping_reply {
    fn clone(&self) -> get_device_modifier_mapping_reply { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct set_device_modifier_mapping_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct set_device_modifier_mapping_request {
     pub major_opcode :            u8,
     pub minor_opcode :            u8,
     pub length :                  u16,
     pub device_id :               u8,
     pub keycodes_per_modifier :   u8,
     pub pad0 :                    u8
}

impl Copy for set_device_modifier_mapping_request {}
impl Clone for set_device_modifier_mapping_request {
    fn clone(&self) -> set_device_modifier_mapping_request { *self }
}

#[repr(C)]
pub struct set_device_modifier_mapping_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub status :          u8,
     pub pad1 :            [u8; 23]
}

impl Copy for set_device_modifier_mapping_reply {}
impl Clone for set_device_modifier_mapping_reply {
    fn clone(&self) -> set_device_modifier_mapping_reply { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct get_device_button_mapping_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct get_device_button_mapping_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub device_id :      u8,
     pub pad0 :           [u8; 3]
}

impl Copy for get_device_button_mapping_request {}
impl Clone for get_device_button_mapping_request {
    fn clone(&self) -> get_device_button_mapping_request { *self }
}

#[repr(C)]
pub struct get_device_button_mapping_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub map_size :        u8,
     pub pad1 :            [u8; 23]
}

impl Copy for get_device_button_mapping_reply {}
impl Clone for get_device_button_mapping_reply {
    fn clone(&self) -> get_device_button_mapping_reply { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct set_device_button_mapping_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct set_device_button_mapping_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub device_id :      u8,
     pub map_size :       u8,
     pub pad0 :           [u8; 2]
}

impl Copy for set_device_button_mapping_request {}
impl Clone for set_device_button_mapping_request {
    fn clone(&self) -> set_device_button_mapping_request { *self }
}

#[repr(C)]
pub struct set_device_button_mapping_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub status :          u8,
     pub pad1 :            [u8; 23]
}

impl Copy for set_device_button_mapping_reply {}
impl Clone for set_device_button_mapping_reply {
    fn clone(&self) -> set_device_button_mapping_reply { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct query_device_state_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct query_device_state_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub device_id :      u8,
     pub pad0 :           [u8; 3]
}

impl Copy for query_device_state_request {}
impl Clone for query_device_state_request {
    fn clone(&self) -> query_device_state_request { *self }
}

#[repr(C)]
pub struct query_device_state_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub num_classes :     u8,
     pub pad1 :            [u8; 23]
}

impl Copy for query_device_state_reply {}
impl Clone for query_device_state_reply {
    fn clone(&self) -> query_device_state_reply { *self }
}

#[repr(C)]
pub struct input_state {
     pub class_id :    u8,
     pub len :         u8,
     pub num_items :   u8
}

impl Copy for input_state {}
impl Clone for input_state {
    fn clone(&self) -> input_state { *self }
}
/**
 * @brief input_state_iterator
 **/
#[repr(C)]
pub struct input_state_iterator {
    pub data : *mut input_state,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct key_state {
     pub class_id :   u8,
     pub len :        u8,
     pub num_keys :   u8,
     pub pad0 :       u8,
     pub keys :       [u8; 32]
}

impl Copy for key_state {}
impl Clone for key_state {
    fn clone(&self) -> key_state { *self }
}
/**
 * @brief key_state_iterator
 **/
#[repr(C)]
pub struct key_state_iterator {
    pub data : *mut key_state,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct button_state {
     pub class_id :      u8,
     pub len :           u8,
     pub num_buttons :   u8,
     pub pad0 :          u8,
     pub buttons :       [u8; 32]
}

impl Copy for button_state {}
impl Clone for button_state {
    fn clone(&self) -> button_state { *self }
}
/**
 * @brief button_state_iterator
 **/
#[repr(C)]
pub struct button_state_iterator {
    pub data : *mut button_state,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct valuator_state {
     pub class_id :        u8,
     pub len :             u8,
     pub num_valuators :   u8,
     pub mode :            u8
}

impl Copy for valuator_state {}
impl Clone for valuator_state {
    fn clone(&self) -> valuator_state { *self }
}
/**
 * @brief valuator_state_iterator
 **/
#[repr(C)]
pub struct valuator_state_iterator {
    pub data : *mut valuator_state,
    pub rem  : c_int,
    pub index: c_int
}



#[repr(C)]
pub struct send_extension_event_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub destination :    ffi::xproto::window,
     pub device_id :      u8,
     pub propagate :      u8,
     pub num_classes :    u16,
     pub num_events :     u8,
     pub pad0 :           [u8; 3]
}

impl Copy for send_extension_event_request {}
impl Clone for send_extension_event_request {
    fn clone(&self) -> send_extension_event_request { *self }
}


#[repr(C)]
pub struct device_bell_request {
     pub major_opcode :     u8,
     pub minor_opcode :     u8,
     pub length :           u16,
     pub device_id :        u8,
     pub feedback_id :      u8,
     pub feedback_class :   u8,
     pub percent :          i8
}

impl Copy for device_bell_request {}
impl Clone for device_bell_request {
    fn clone(&self) -> device_bell_request { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct set_device_valuators_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct set_device_valuators_request {
     pub major_opcode :     u8,
     pub minor_opcode :     u8,
     pub length :           u16,
     pub device_id :        u8,
     pub first_valuator :   u8,
     pub num_valuators :    u8,
     pub pad0 :             u8
}

impl Copy for set_device_valuators_request {}
impl Clone for set_device_valuators_request {
    fn clone(&self) -> set_device_valuators_request { *self }
}

#[repr(C)]
pub struct set_device_valuators_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub status :          u8,
     pub pad1 :            [u8; 23]
}

impl Copy for set_device_valuators_reply {}
impl Clone for set_device_valuators_reply {
    fn clone(&self) -> set_device_valuators_reply { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct get_device_control_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct get_device_control_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub control_id :     u16,
     pub device_id :      u8,
     pub pad0 :           u8
}

impl Copy for get_device_control_request {}
impl Clone for get_device_control_request {
    fn clone(&self) -> get_device_control_request { *self }
}

#[repr(C)]
pub struct get_device_control_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub status :          u8,
     pub pad1 :            [u8; 23]
}

impl Copy for get_device_control_reply {}
impl Clone for get_device_control_reply {
    fn clone(&self) -> get_device_control_reply { *self }
}

#[repr(C)]
pub struct device_state {
     pub control_id :   u16,
     pub len :          u16
}

impl Copy for device_state {}
impl Clone for device_state {
    fn clone(&self) -> device_state { *self }
}
/**
 * @brief device_state_iterator
 **/
#[repr(C)]
pub struct device_state_iterator {
    pub data : *mut device_state,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct device_resolution_state {
     pub control_id :      u16,
     pub len :             u16,
     pub num_valuators :   u32
}

impl Copy for device_resolution_state {}
impl Clone for device_resolution_state {
    fn clone(&self) -> device_resolution_state { *self }
}
/**
 * @brief device_resolution_state_iterator
 **/
#[repr(C)]
pub struct device_resolution_state_iterator {
    pub data : *mut device_resolution_state,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct device_abs_calib_state {
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

impl Copy for device_abs_calib_state {}
impl Clone for device_abs_calib_state {
    fn clone(&self) -> device_abs_calib_state { *self }
}
/**
 * @brief device_abs_calib_state_iterator
 **/
#[repr(C)]
pub struct device_abs_calib_state_iterator {
    pub data : *mut device_abs_calib_state,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct device_abs_area_state {
     pub control_id :   u16,
     pub len :          u16,
     pub offset_x :     u32,
     pub offset_y :     u32,
     pub width :        u32,
     pub height :       u32,
     pub screen :       u32,
     pub following :    u32
}

impl Copy for device_abs_area_state {}
impl Clone for device_abs_area_state {
    fn clone(&self) -> device_abs_area_state { *self }
}
/**
 * @brief device_abs_area_state_iterator
 **/
#[repr(C)]
pub struct device_abs_area_state_iterator {
    pub data : *mut device_abs_area_state,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct device_core_state {
     pub control_id :   u16,
     pub len :          u16,
     pub status :       u8,
     pub iscore :       u8,
     pub pad0 :         [u8; 2]
}

impl Copy for device_core_state {}
impl Clone for device_core_state {
    fn clone(&self) -> device_core_state { *self }
}
/**
 * @brief device_core_state_iterator
 **/
#[repr(C)]
pub struct device_core_state_iterator {
    pub data : *mut device_core_state,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct device_enable_state {
     pub control_id :   u16,
     pub len :          u16,
     pub enable :       u8,
     pub pad0 :         [u8; 3]
}

impl Copy for device_enable_state {}
impl Clone for device_enable_state {
    fn clone(&self) -> device_enable_state { *self }
}
/**
 * @brief device_enable_state_iterator
 **/
#[repr(C)]
pub struct device_enable_state_iterator {
    pub data : *mut device_enable_state,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct device_ctl {
     pub control_id :   u16,
     pub len :          u16
}

impl Copy for device_ctl {}
impl Clone for device_ctl {
    fn clone(&self) -> device_ctl { *self }
}
/**
 * @brief device_ctl_iterator
 **/
#[repr(C)]
pub struct device_ctl_iterator {
    pub data : *mut device_ctl,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct device_resolution_ctl {
     pub control_id :       u16,
     pub len :              u16,
     pub first_valuator :   u8,
     pub num_valuators :    u8
}

impl Copy for device_resolution_ctl {}
impl Clone for device_resolution_ctl {
    fn clone(&self) -> device_resolution_ctl { *self }
}
/**
 * @brief device_resolution_ctl_iterator
 **/
#[repr(C)]
pub struct device_resolution_ctl_iterator {
    pub data : *mut device_resolution_ctl,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct device_abs_calib_ctl {
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

impl Copy for device_abs_calib_ctl {}
impl Clone for device_abs_calib_ctl {
    fn clone(&self) -> device_abs_calib_ctl { *self }
}
/**
 * @brief device_abs_calib_ctl_iterator
 **/
#[repr(C)]
pub struct device_abs_calib_ctl_iterator {
    pub data : *mut device_abs_calib_ctl,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct device_abs_area_ctrl {
     pub control_id :   u16,
     pub len :          u16,
     pub offset_x :     u32,
     pub offset_y :     u32,
     pub width :        i32,
     pub height :       i32,
     pub screen :       i32,
     pub following :    u32
}

impl Copy for device_abs_area_ctrl {}
impl Clone for device_abs_area_ctrl {
    fn clone(&self) -> device_abs_area_ctrl { *self }
}
/**
 * @brief device_abs_area_ctrl_iterator
 **/
#[repr(C)]
pub struct device_abs_area_ctrl_iterator {
    pub data : *mut device_abs_area_ctrl,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct device_core_ctrl {
     pub control_id :   u16,
     pub len :          u16,
     pub status :       u8,
     pub pad0 :         [u8; 3]
}

impl Copy for device_core_ctrl {}
impl Clone for device_core_ctrl {
    fn clone(&self) -> device_core_ctrl { *self }
}
/**
 * @brief device_core_ctrl_iterator
 **/
#[repr(C)]
pub struct device_core_ctrl_iterator {
    pub data : *mut device_core_ctrl,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct device_enable_ctrl {
     pub control_id :   u16,
     pub len :          u16,
     pub enable :       u8,
     pub pad0 :         [u8; 3]
}

impl Copy for device_enable_ctrl {}
impl Clone for device_enable_ctrl {
    fn clone(&self) -> device_enable_ctrl { *self }
}
/**
 * @brief device_enable_ctrl_iterator
 **/
#[repr(C)]
pub struct device_enable_ctrl_iterator {
    pub data : *mut device_enable_ctrl,
    pub rem  : c_int,
    pub index: c_int
}



#[repr(C)]
pub struct device_valuator_event {
     pub response_type :    u8,
     pub device_id :        u8,
     pub sequence :         u16,
     pub device_state :     u16,
     pub num_valuators :    u8,
     pub first_valuator :   u8,
     pub valuators :        [i32; 6]
}

impl Copy for device_valuator_event {}
impl Clone for device_valuator_event {
    fn clone(&self) -> device_valuator_event { *self }
}


#[repr(C)]
pub struct device_key_press_event {
     pub response_type :   u8,
     pub detail :          u8,
     pub sequence :        u16,
     pub time :            ffi::xproto::timestamp,
     pub root :            ffi::xproto::window,
     pub event :           ffi::xproto::window,
     pub child :           ffi::xproto::window,
     pub root_x :          i16,
     pub root_y :          i16,
     pub event_x :         i16,
     pub event_y :         i16,
     pub state :           u16,
     pub same_screen :     u8,
     pub device_id :       u8
}

impl Copy for device_key_press_event {}
impl Clone for device_key_press_event {
    fn clone(&self) -> device_key_press_event { *self }
}


pub type device_key_release_event = device_key_press_event;


pub type device_button_press_event = device_key_press_event;


pub type device_button_release_event = device_key_press_event;


pub type device_motion_notify_event = device_key_press_event;


pub type proximity_in_event = device_key_press_event;


pub type proximity_out_event = device_key_press_event;


#[repr(C)]
pub struct focus_in_event {
     pub response_type :   u8,
     pub detail :          u8,
     pub sequence :        u16,
     pub time :            ffi::xproto::timestamp,
     pub window :          ffi::xproto::window,
     pub mode :            u8,
     pub device_id :       u8,
     pub pad0 :            [u8; 18]
}

impl Copy for focus_in_event {}
impl Clone for focus_in_event {
    fn clone(&self) -> focus_in_event { *self }
}


pub type focus_out_event = focus_in_event;


#[repr(C)]
pub struct device_state_notify_event {
     pub response_type :      u8,
     pub device_id :          u8,
     pub sequence :           u16,
     pub time :               ffi::xproto::timestamp,
     pub num_keys :           u8,
     pub num_buttons :        u8,
     pub num_valuators :      u8,
     pub classes_reported :   u8,
     pub buttons :            [u8; 4],
     pub keys :               [u8; 4],
     pub valuators :          [u32; 3]
}

impl Copy for device_state_notify_event {}
impl Clone for device_state_notify_event {
    fn clone(&self) -> device_state_notify_event { *self }
}


#[repr(C)]
pub struct device_mapping_notify_event {
     pub response_type :   u8,
     pub device_id :       u8,
     pub sequence :        u16,
     pub request :         u8,
     pub first_keycode :   key_code,
     pub count :           u8,
     pub pad0 :            u8,
     pub time :            ffi::xproto::timestamp,
     pub pad1 :            [u8; 20]
}

impl Copy for device_mapping_notify_event {}
impl Clone for device_mapping_notify_event {
    fn clone(&self) -> device_mapping_notify_event { *self }
}


#[repr(C)]
pub struct change_device_notify_event {
     pub response_type :   u8,
     pub device_id :       u8,
     pub sequence :        u16,
     pub time :            ffi::xproto::timestamp,
     pub request :         u8,
     pub pad0 :            [u8; 23]
}

impl Copy for change_device_notify_event {}
impl Clone for change_device_notify_event {
    fn clone(&self) -> change_device_notify_event { *self }
}


#[repr(C)]
pub struct device_key_state_notify_event {
     pub response_type :   u8,
     pub device_id :       u8,
     pub sequence :        u16,
     pub keys :            [u8; 28]
}

impl Copy for device_key_state_notify_event {}
impl Clone for device_key_state_notify_event {
    fn clone(&self) -> device_key_state_notify_event { *self }
}


#[repr(C)]
pub struct device_button_state_notify_event {
     pub response_type :   u8,
     pub device_id :       u8,
     pub sequence :        u16,
     pub buttons :         [u8; 28]
}

impl Copy for device_button_state_notify_event {}
impl Clone for device_button_state_notify_event {
    fn clone(&self) -> device_button_state_notify_event { *self }
}


#[repr(C)]
pub struct device_presence_notify_event {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub time :            ffi::xproto::timestamp,
     pub devchange :       u8,
     pub device_id :       u8,
     pub control :         u16,
     pub pad1 :            [u8; 20]
}

impl Copy for device_presence_notify_event {}
impl Clone for device_presence_notify_event {
    fn clone(&self) -> device_presence_notify_event { *self }
}


#[repr(C)]
pub struct device_error {
     pub response_type :   u8,
     pub error_code :      u8,
     pub sequence :        u16
}

impl Copy for device_error {}
impl Clone for device_error {
    fn clone(&self) -> device_error { *self }
}


#[repr(C)]
pub struct event_error {
     pub response_type :   u8,
     pub error_code :      u8,
     pub sequence :        u16
}

impl Copy for event_error {}
impl Clone for event_error {
    fn clone(&self) -> event_error { *self }
}


#[repr(C)]
pub struct mode_error {
     pub response_type :   u8,
     pub error_code :      u8,
     pub sequence :        u16
}

impl Copy for mode_error {}
impl Clone for mode_error {
    fn clone(&self) -> mode_error { *self }
}


#[repr(C)]
pub struct device_busy_error {
     pub response_type :   u8,
     pub error_code :      u8,
     pub sequence :        u16
}

impl Copy for device_busy_error {}
impl Clone for device_busy_error {
    fn clone(&self) -> device_busy_error { *self }
}


#[repr(C)]
pub struct class_error {
     pub response_type :   u8,
     pub error_code :      u8,
     pub sequence :        u16
}

impl Copy for class_error {}
impl Clone for class_error {
    fn clone(&self) -> class_error { *self }
}
#[link(name="xcb-xinput")]
extern "C" {

/**
 * Get the next element of the iterator
 * @param i Pointer to a key_code_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(key_code)
 *
 *
 */
pub fn xcb_input_key_code_next (i:*mut key_code_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An key_code_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_key_code_end (i:key_code_iterator) -> ffi::base::generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a event_class_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(event_class)
 *
 *
 */
pub fn xcb_input_event_class_next (i:*mut event_class_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An event_class_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_event_class_end (i:event_class_iterator) -> ffi::base::generic_iterator;

pub fn xcb_input_get_extension_version_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_input_get_extension_version (c : *mut ffi::base::connection,
                                           name_len :  u16,
                                           name : *mut c_char) -> get_extension_version_cookie;

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
pub fn xcb_input_get_extension_version_unchecked (c : *mut ffi::base::connection,
                                                     name_len :  u16,
                                                     name : *mut c_char) -> get_extension_version_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_input_get_extension_version_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_input_get_extension_version_reply (c : *mut ffi::base::connection,
                                                 cookie : get_extension_version_cookie,
                                                 e : *mut *mut ffi::base::generic_error) -> *mut get_extension_version_reply;

/**
 * Get the next element of the iterator
 * @param i Pointer to a device_info_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(device_info)
 *
 *
 */
pub fn xcb_input_device_info_next (i:*mut device_info_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An device_info_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_device_info_end (i:device_info_iterator) -> ffi::base::generic_iterator;

pub fn xcb_input_list_input_devices_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_input_list_input_devices (c : *mut ffi::base::connection) -> list_input_devices_cookie;

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
pub fn xcb_input_list_input_devices_unchecked (c : *mut ffi::base::connection) -> list_input_devices_cookie;

pub fn xcb_input_list_input_devices_devices (R : *mut list_input_devices_reply) -> *mut device_info;


pub fn xcb_input_list_input_devices_devices_length (R : *mut list_input_devices_reply) -> c_int;

pub fn xcb_input_list_input_devices_devices_iterator (R : *mut list_input_devices_reply) -> device_info_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_input_list_input_devices_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_input_list_input_devices_reply (c : *mut ffi::base::connection,
                                              cookie : list_input_devices_cookie,
                                              e : *mut *mut ffi::base::generic_error) -> *mut list_input_devices_reply;

/**
 * Get the next element of the iterator
 * @param i Pointer to a input_info_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(input_info)
 *
 *
 */
pub fn xcb_input_input_info_next (i:*mut input_info_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An input_info_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_input_info_end (i:input_info_iterator) -> ffi::base::generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a key_info_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(key_info)
 *
 *
 */
pub fn xcb_input_key_info_next (i:*mut key_info_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An key_info_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_key_info_end (i:key_info_iterator) -> ffi::base::generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a button_info_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(button_info)
 *
 *
 */
pub fn xcb_input_button_info_next (i:*mut button_info_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An button_info_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_button_info_end (i:button_info_iterator) -> ffi::base::generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a axis_info_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(axis_info)
 *
 *
 */
pub fn xcb_input_axis_info_next (i:*mut axis_info_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An axis_info_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_axis_info_end (i:axis_info_iterator) -> ffi::base::generic_iterator;

pub fn xcb_input_valuator_info_sizeof (_buffer :  *mut c_void) -> c_int;

pub fn xcb_input_valuator_info_axes (R : *mut valuator_info) -> *mut axis_info;


pub fn xcb_input_valuator_info_axes_length (R : *mut valuator_info) -> c_int;

pub fn xcb_input_valuator_info_axes_iterator (R : *mut valuator_info) -> axis_info_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a valuator_info_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(valuator_info)
 *
 *
 */
pub fn xcb_input_valuator_info_next (i:*mut valuator_info_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An valuator_info_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_valuator_info_end (i:valuator_info_iterator) -> ffi::base::generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a input_class_info_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(input_class_info)
 *
 *
 */
pub fn xcb_input_input_class_info_next (i:*mut input_class_info_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An input_class_info_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_input_class_info_end (i:input_class_info_iterator) -> ffi::base::generic_iterator;

pub fn xcb_input_open_device_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_input_open_device (c : *mut ffi::base::connection,
                                 device_id :  u8) -> open_device_cookie;

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
pub fn xcb_input_open_device_unchecked (c : *mut ffi::base::connection,
                                           device_id :  u8) -> open_device_cookie;

pub fn xcb_input_open_device_class_info (R : *mut open_device_reply) -> *mut input_class_info;


pub fn xcb_input_open_device_class_info_length (R : *mut open_device_reply) -> c_int;

pub fn xcb_input_open_device_class_info_iterator (R : *mut open_device_reply) -> input_class_info_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_input_open_device_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_input_open_device_reply (c : *mut ffi::base::connection,
                                       cookie : open_device_cookie,
                                       e : *mut *mut ffi::base::generic_error) -> *mut open_device_reply;

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
pub fn xcb_input_close_device_checked (c : *mut ffi::base::connection,
                                          device_id :  u8) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_input_close_device (c : *mut ffi::base::connection,
                                  device_id :  u8) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_input_set_device_mode (c : *mut ffi::base::connection,
                                     device_id :  u8,
                                     mode :  u8) -> set_device_mode_cookie;

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
pub fn xcb_input_set_device_mode_unchecked (c : *mut ffi::base::connection,
                                               device_id :  u8,
                                               mode :  u8) -> set_device_mode_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_input_set_device_mode_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_input_set_device_mode_reply (c : *mut ffi::base::connection,
                                           cookie : set_device_mode_cookie,
                                           e : *mut *mut ffi::base::generic_error) -> *mut set_device_mode_reply;

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
pub fn xcb_input_select_extension_event_checked (c : *mut ffi::base::connection,
                                                    window :  ffi::xproto::window,
                                                    num_classes :  u16,
                                                    classes : *mut event_class) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_input_select_extension_event (c : *mut ffi::base::connection,
                                            window :  ffi::xproto::window,
                                            num_classes :  u16,
                                            classes : *mut event_class) -> ffi::base::void_cookie;

pub fn xcb_input_get_selected_extension_events_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_input_get_selected_extension_events (c : *mut ffi::base::connection,
                                                   window :  ffi::xproto::window) -> get_selected_extension_events_cookie;

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
pub fn xcb_input_get_selected_extension_events_unchecked (c : *mut ffi::base::connection,
                                                             window :  ffi::xproto::window) -> get_selected_extension_events_cookie;

pub fn xcb_input_get_selected_extension_events_this_classes (R : *mut get_selected_extension_events_reply) -> *mut event_class;


pub fn xcb_input_get_selected_extension_events_this_classes_length (R : *mut get_selected_extension_events_reply) -> c_int;


pub fn xcb_input_get_selected_extension_events_this_classes_end (R : *mut get_selected_extension_events_reply) -> ffi::base::generic_iterator;

pub fn xcb_input_get_selected_extension_events_all_classes (R : *mut get_selected_extension_events_reply) -> *mut event_class;


pub fn xcb_input_get_selected_extension_events_all_classes_length (R : *mut get_selected_extension_events_reply) -> c_int;


pub fn xcb_input_get_selected_extension_events_all_classes_end (R : *mut get_selected_extension_events_reply) -> ffi::base::generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_input_get_selected_extension_events_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_input_get_selected_extension_events_reply (c : *mut ffi::base::connection,
                                                         cookie : get_selected_extension_events_cookie,
                                                         e : *mut *mut ffi::base::generic_error) -> *mut get_selected_extension_events_reply;

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
pub fn xcb_input_change_device_dont_propagate_list_checked (c : *mut ffi::base::connection,
                                                               window :  ffi::xproto::window,
                                                               num_classes :  u16,
                                                               mode :  u8,
                                                               classes : *mut event_class) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_input_change_device_dont_propagate_list (c : *mut ffi::base::connection,
                                                       window :  ffi::xproto::window,
                                                       num_classes :  u16,
                                                       mode :  u8,
                                                       classes : *mut event_class) -> ffi::base::void_cookie;

pub fn xcb_input_get_device_dont_propagate_list_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_input_get_device_dont_propagate_list (c : *mut ffi::base::connection,
                                                    window :  ffi::xproto::window) -> get_device_dont_propagate_list_cookie;

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
pub fn xcb_input_get_device_dont_propagate_list_unchecked (c : *mut ffi::base::connection,
                                                              window :  ffi::xproto::window) -> get_device_dont_propagate_list_cookie;

pub fn xcb_input_get_device_dont_propagate_list_classes (R : *mut get_device_dont_propagate_list_reply) -> *mut event_class;


pub fn xcb_input_get_device_dont_propagate_list_classes_length (R : *mut get_device_dont_propagate_list_reply) -> c_int;


pub fn xcb_input_get_device_dont_propagate_list_classes_end (R : *mut get_device_dont_propagate_list_reply) -> ffi::base::generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_input_get_device_dont_propagate_list_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_input_get_device_dont_propagate_list_reply (c : *mut ffi::base::connection,
                                                          cookie : get_device_dont_propagate_list_cookie,
                                                          e : *mut *mut ffi::base::generic_error) -> *mut get_device_dont_propagate_list_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_input_get_device_motion_events (c : *mut ffi::base::connection,
                                              start :  ffi::xproto::timestamp,
                                              stop :  ffi::xproto::timestamp,
                                              device_id :  u8) -> get_device_motion_events_cookie;

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
pub fn xcb_input_get_device_motion_events_unchecked (c : *mut ffi::base::connection,
                                                        start :  ffi::xproto::timestamp,
                                                        stop :  ffi::xproto::timestamp,
                                                        device_id :  u8) -> get_device_motion_events_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_input_get_device_motion_events_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_input_get_device_motion_events_reply (c : *mut ffi::base::connection,
                                                    cookie : get_device_motion_events_cookie,
                                                    e : *mut *mut ffi::base::generic_error) -> *mut get_device_motion_events_reply;

/**
 * Get the next element of the iterator
 * @param i Pointer to a device_time_coord_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(device_time_coord)
 *
 *
 */
pub fn xcb_input_device_time_coord_next (i:*mut device_time_coord_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An device_time_coord_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_device_time_coord_end (i:device_time_coord_iterator) -> ffi::base::generic_iterator;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_input_change_keyboard_device (c : *mut ffi::base::connection,
                                            device_id :  u8) -> change_keyboard_device_cookie;

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
pub fn xcb_input_change_keyboard_device_unchecked (c : *mut ffi::base::connection,
                                                      device_id :  u8) -> change_keyboard_device_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_input_change_keyboard_device_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_input_change_keyboard_device_reply (c : *mut ffi::base::connection,
                                                  cookie : change_keyboard_device_cookie,
                                                  e : *mut *mut ffi::base::generic_error) -> *mut change_keyboard_device_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_input_change_pointer_device (c : *mut ffi::base::connection,
                                           x_axis :  u8,
                                           y_axis :  u8,
                                           device_id :  u8) -> change_pointer_device_cookie;

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
pub fn xcb_input_change_pointer_device_unchecked (c : *mut ffi::base::connection,
                                                     x_axis :  u8,
                                                     y_axis :  u8,
                                                     device_id :  u8) -> change_pointer_device_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_input_change_pointer_device_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_input_change_pointer_device_reply (c : *mut ffi::base::connection,
                                                 cookie : change_pointer_device_cookie,
                                                 e : *mut *mut ffi::base::generic_error) -> *mut change_pointer_device_reply;

pub fn xcb_input_grab_device_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_input_grab_device (c : *mut ffi::base::connection,
                                 grab_window :  ffi::xproto::window,
                                 time :  ffi::xproto::timestamp,
                                 num_classes :  u16,
                                 this_device_mode :  u8,
                                 other_device_mode :  u8,
                                 owner_events :  u8,
                                 device_id :  u8,
                                 classes : *mut event_class) -> grab_device_cookie;

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
pub fn xcb_input_grab_device_unchecked (c : *mut ffi::base::connection,
                                           grab_window :  ffi::xproto::window,
                                           time :  ffi::xproto::timestamp,
                                           num_classes :  u16,
                                           this_device_mode :  u8,
                                           other_device_mode :  u8,
                                           owner_events :  u8,
                                           device_id :  u8,
                                           classes : *mut event_class) -> grab_device_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_input_grab_device_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_input_grab_device_reply (c : *mut ffi::base::connection,
                                       cookie : grab_device_cookie,
                                       e : *mut *mut ffi::base::generic_error) -> *mut grab_device_reply;

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
pub fn xcb_input_ungrab_device_checked (c : *mut ffi::base::connection,
                                           time :  ffi::xproto::timestamp,
                                           device_id :  u8) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_input_ungrab_device (c : *mut ffi::base::connection,
                                   time :  ffi::xproto::timestamp,
                                   device_id :  u8) -> ffi::base::void_cookie;

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
pub fn xcb_input_grab_device_key_checked (c : *mut ffi::base::connection,
                                             grab_window :  ffi::xproto::window,
                                             num_classes :  u16,
                                             modifiers :  u16,
                                             modifier_device :  u8,
                                             grabbed_device :  u8,
                                             key :  u8,
                                             this_device_mode :  u8,
                                             other_device_mode :  u8,
                                             owner_events :  u8,
                                             classes : *mut event_class) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_input_grab_device_key (c : *mut ffi::base::connection,
                                     grab_window :  ffi::xproto::window,
                                     num_classes :  u16,
                                     modifiers :  u16,
                                     modifier_device :  u8,
                                     grabbed_device :  u8,
                                     key :  u8,
                                     this_device_mode :  u8,
                                     other_device_mode :  u8,
                                     owner_events :  u8,
                                     classes : *mut event_class) -> ffi::base::void_cookie;

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
pub fn xcb_input_ungrab_device_key_checked (c : *mut ffi::base::connection,
                                               grabWindow :  ffi::xproto::window,
                                               modifiers :  u16,
                                               modifier_device :  u8,
                                               key :  u8,
                                               grabbed_device :  u8) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_input_ungrab_device_key (c : *mut ffi::base::connection,
                                       grabWindow :  ffi::xproto::window,
                                       modifiers :  u16,
                                       modifier_device :  u8,
                                       key :  u8,
                                       grabbed_device :  u8) -> ffi::base::void_cookie;

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
pub fn xcb_input_grab_device_button_checked (c : *mut ffi::base::connection,
                                                grab_window :  ffi::xproto::window,
                                                grabbed_device :  u8,
                                                modifier_device :  u8,
                                                num_classes :  u16,
                                                modifiers :  u16,
                                                this_device_mode :  u8,
                                                other_device_mode :  u8,
                                                button :  u8,
                                                owner_events :  u8,
                                                classes : *mut event_class) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_input_grab_device_button (c : *mut ffi::base::connection,
                                        grab_window :  ffi::xproto::window,
                                        grabbed_device :  u8,
                                        modifier_device :  u8,
                                        num_classes :  u16,
                                        modifiers :  u16,
                                        this_device_mode :  u8,
                                        other_device_mode :  u8,
                                        button :  u8,
                                        owner_events :  u8,
                                        classes : *mut event_class) -> ffi::base::void_cookie;

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
pub fn xcb_input_ungrab_device_button_checked (c : *mut ffi::base::connection,
                                                  grab_window :  ffi::xproto::window,
                                                  modifiers :  u16,
                                                  modifier_device :  u8,
                                                  button :  u8,
                                                  grabbed_device :  u8) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_input_ungrab_device_button (c : *mut ffi::base::connection,
                                          grab_window :  ffi::xproto::window,
                                          modifiers :  u16,
                                          modifier_device :  u8,
                                          button :  u8,
                                          grabbed_device :  u8) -> ffi::base::void_cookie;

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
pub fn xcb_input_allow_device_events_checked (c : *mut ffi::base::connection,
                                                 time :  ffi::xproto::timestamp,
                                                 mode :  u8,
                                                 device_id :  u8) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_input_allow_device_events (c : *mut ffi::base::connection,
                                         time :  ffi::xproto::timestamp,
                                         mode :  u8,
                                         device_id :  u8) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_input_get_device_focus (c : *mut ffi::base::connection,
                                      device_id :  u8) -> get_device_focus_cookie;

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
pub fn xcb_input_get_device_focus_unchecked (c : *mut ffi::base::connection,
                                                device_id :  u8) -> get_device_focus_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_input_get_device_focus_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_input_get_device_focus_reply (c : *mut ffi::base::connection,
                                            cookie : get_device_focus_cookie,
                                            e : *mut *mut ffi::base::generic_error) -> *mut get_device_focus_reply;

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
pub fn xcb_input_set_device_focus_checked (c : *mut ffi::base::connection,
                                              focus :  ffi::xproto::window,
                                              time :  ffi::xproto::timestamp,
                                              revert_to :  u8,
                                              device_id :  u8) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_input_set_device_focus (c : *mut ffi::base::connection,
                                      focus :  ffi::xproto::window,
                                      time :  ffi::xproto::timestamp,
                                      revert_to :  u8,
                                      device_id :  u8) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_input_get_feedback_control (c : *mut ffi::base::connection,
                                          device_id :  u8) -> get_feedback_control_cookie;

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
pub fn xcb_input_get_feedback_control_unchecked (c : *mut ffi::base::connection,
                                                    device_id :  u8) -> get_feedback_control_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_input_get_feedback_control_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_input_get_feedback_control_reply (c : *mut ffi::base::connection,
                                                cookie : get_feedback_control_cookie,
                                                e : *mut *mut ffi::base::generic_error) -> *mut get_feedback_control_reply;

/**
 * Get the next element of the iterator
 * @param i Pointer to a feedback_state_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(feedback_state)
 *
 *
 */
pub fn xcb_input_feedback_state_next (i:*mut feedback_state_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An feedback_state_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_feedback_state_end (i:feedback_state_iterator) -> ffi::base::generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a kbd_feedback_state_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(kbd_feedback_state)
 *
 *
 */
pub fn xcb_input_kbd_feedback_state_next (i:*mut kbd_feedback_state_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An kbd_feedback_state_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_kbd_feedback_state_end (i:kbd_feedback_state_iterator) -> ffi::base::generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a ptr_feedback_state_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(ptr_feedback_state)
 *
 *
 */
pub fn xcb_input_ptr_feedback_state_next (i:*mut ptr_feedback_state_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An ptr_feedback_state_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_ptr_feedback_state_end (i:ptr_feedback_state_iterator) -> ffi::base::generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a integer_feedback_state_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(integer_feedback_state)
 *
 *
 */
pub fn xcb_input_integer_feedback_state_next (i:*mut integer_feedback_state_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An integer_feedback_state_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_integer_feedback_state_end (i:integer_feedback_state_iterator) -> ffi::base::generic_iterator;

pub fn xcb_input_string_feedback_state_sizeof (_buffer :  *mut c_void) -> c_int;

pub fn xcb_input_string_feedback_state_keysyms (R : *mut string_feedback_state) -> *mut ffi::xproto::keysym;


pub fn xcb_input_string_feedback_state_keysyms_length (R : *mut string_feedback_state) -> c_int;


pub fn xcb_input_string_feedback_state_keysyms_end (R : *mut string_feedback_state) -> ffi::base::generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a string_feedback_state_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(string_feedback_state)
 *
 *
 */
pub fn xcb_input_string_feedback_state_next (i:*mut string_feedback_state_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An string_feedback_state_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_string_feedback_state_end (i:string_feedback_state_iterator) -> ffi::base::generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a bell_feedback_state_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(bell_feedback_state)
 *
 *
 */
pub fn xcb_input_bell_feedback_state_next (i:*mut bell_feedback_state_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An bell_feedback_state_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_bell_feedback_state_end (i:bell_feedback_state_iterator) -> ffi::base::generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a led_feedback_state_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(led_feedback_state)
 *
 *
 */
pub fn xcb_input_led_feedback_state_next (i:*mut led_feedback_state_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An led_feedback_state_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_led_feedback_state_end (i:led_feedback_state_iterator) -> ffi::base::generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a feedback_ctl_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(feedback_ctl)
 *
 *
 */
pub fn xcb_input_feedback_ctl_next (i:*mut feedback_ctl_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An feedback_ctl_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_feedback_ctl_end (i:feedback_ctl_iterator) -> ffi::base::generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a kbd_feedback_ctl_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(kbd_feedback_ctl)
 *
 *
 */
pub fn xcb_input_kbd_feedback_ctl_next (i:*mut kbd_feedback_ctl_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An kbd_feedback_ctl_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_kbd_feedback_ctl_end (i:kbd_feedback_ctl_iterator) -> ffi::base::generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a ptr_feedback_ctl_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(ptr_feedback_ctl)
 *
 *
 */
pub fn xcb_input_ptr_feedback_ctl_next (i:*mut ptr_feedback_ctl_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An ptr_feedback_ctl_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_ptr_feedback_ctl_end (i:ptr_feedback_ctl_iterator) -> ffi::base::generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a integer_feedback_ctl_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(integer_feedback_ctl)
 *
 *
 */
pub fn xcb_input_integer_feedback_ctl_next (i:*mut integer_feedback_ctl_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An integer_feedback_ctl_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_integer_feedback_ctl_end (i:integer_feedback_ctl_iterator) -> ffi::base::generic_iterator;

pub fn xcb_input_string_feedback_ctl_sizeof (_buffer :  *mut c_void) -> c_int;

pub fn xcb_input_string_feedback_ctl_keysyms (R : *mut string_feedback_ctl) -> *mut ffi::xproto::keysym;


pub fn xcb_input_string_feedback_ctl_keysyms_length (R : *mut string_feedback_ctl) -> c_int;


pub fn xcb_input_string_feedback_ctl_keysyms_end (R : *mut string_feedback_ctl) -> ffi::base::generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a string_feedback_ctl_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(string_feedback_ctl)
 *
 *
 */
pub fn xcb_input_string_feedback_ctl_next (i:*mut string_feedback_ctl_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An string_feedback_ctl_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_string_feedback_ctl_end (i:string_feedback_ctl_iterator) -> ffi::base::generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a bell_feedback_ctl_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(bell_feedback_ctl)
 *
 *
 */
pub fn xcb_input_bell_feedback_ctl_next (i:*mut bell_feedback_ctl_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An bell_feedback_ctl_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_bell_feedback_ctl_end (i:bell_feedback_ctl_iterator) -> ffi::base::generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a led_feedback_ctl_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(led_feedback_ctl)
 *
 *
 */
pub fn xcb_input_led_feedback_ctl_next (i:*mut led_feedback_ctl_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An led_feedback_ctl_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_led_feedback_ctl_end (i:led_feedback_ctl_iterator) -> ffi::base::generic_iterator;

pub fn xcb_input_get_device_key_mapping_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_input_get_device_key_mapping (c : *mut ffi::base::connection,
                                            device_id :  u8,
                                            first_keycode :  key_code,
                                            count :  u8) -> get_device_key_mapping_cookie;

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
pub fn xcb_input_get_device_key_mapping_unchecked (c : *mut ffi::base::connection,
                                                      device_id :  u8,
                                                      first_keycode :  key_code,
                                                      count :  u8) -> get_device_key_mapping_cookie;

pub fn xcb_input_get_device_key_mapping_keysyms (R : *mut get_device_key_mapping_reply) -> *mut ffi::xproto::keysym;


pub fn xcb_input_get_device_key_mapping_keysyms_length (R : *mut get_device_key_mapping_reply) -> c_int;


pub fn xcb_input_get_device_key_mapping_keysyms_end (R : *mut get_device_key_mapping_reply) -> ffi::base::generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_input_get_device_key_mapping_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_input_get_device_key_mapping_reply (c : *mut ffi::base::connection,
                                                  cookie : get_device_key_mapping_cookie,
                                                  e : *mut *mut ffi::base::generic_error) -> *mut get_device_key_mapping_reply;

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
pub fn xcb_input_change_device_key_mapping_checked (c : *mut ffi::base::connection,
                                                       device_id :  u8,
                                                       first_keycode :  key_code,
                                                       keysyms_per_keycode :  u8,
                                                       keycode_count :  u8,
                                                       keysyms : *mut ffi::xproto::keysym) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_input_change_device_key_mapping (c : *mut ffi::base::connection,
                                               device_id :  u8,
                                               first_keycode :  key_code,
                                               keysyms_per_keycode :  u8,
                                               keycode_count :  u8,
                                               keysyms : *mut ffi::xproto::keysym) -> ffi::base::void_cookie;

pub fn xcb_input_get_device_modifier_mapping_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_input_get_device_modifier_mapping (c : *mut ffi::base::connection,
                                                 device_id :  u8) -> get_device_modifier_mapping_cookie;

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
pub fn xcb_input_get_device_modifier_mapping_unchecked (c : *mut ffi::base::connection,
                                                           device_id :  u8) -> get_device_modifier_mapping_cookie;

pub fn xcb_input_get_device_modifier_mapping_keymaps (R : *mut get_device_modifier_mapping_reply) -> *mut u8;


pub fn xcb_input_get_device_modifier_mapping_keymaps_length (R : *mut get_device_modifier_mapping_reply) -> c_int;


pub fn xcb_input_get_device_modifier_mapping_keymaps_end (R : *mut get_device_modifier_mapping_reply) -> ffi::base::generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_input_get_device_modifier_mapping_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_input_get_device_modifier_mapping_reply (c : *mut ffi::base::connection,
                                                       cookie : get_device_modifier_mapping_cookie,
                                                       e : *mut *mut ffi::base::generic_error) -> *mut get_device_modifier_mapping_reply;

pub fn xcb_input_set_device_modifier_mapping_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_input_set_device_modifier_mapping (c : *mut ffi::base::connection,
                                                 device_id :  u8,
                                                 keycodes_per_modifier :  u8,
                                                 keymaps : *mut u8) -> set_device_modifier_mapping_cookie;

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
pub fn xcb_input_set_device_modifier_mapping_unchecked (c : *mut ffi::base::connection,
                                                           device_id :  u8,
                                                           keycodes_per_modifier :  u8,
                                                           keymaps : *mut u8) -> set_device_modifier_mapping_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_input_set_device_modifier_mapping_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_input_set_device_modifier_mapping_reply (c : *mut ffi::base::connection,
                                                       cookie : set_device_modifier_mapping_cookie,
                                                       e : *mut *mut ffi::base::generic_error) -> *mut set_device_modifier_mapping_reply;

pub fn xcb_input_get_device_button_mapping_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_input_get_device_button_mapping (c : *mut ffi::base::connection,
                                               device_id :  u8) -> get_device_button_mapping_cookie;

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
pub fn xcb_input_get_device_button_mapping_unchecked (c : *mut ffi::base::connection,
                                                         device_id :  u8) -> get_device_button_mapping_cookie;

pub fn xcb_input_get_device_button_mapping_map (R : *mut get_device_button_mapping_reply) -> *mut u8;


pub fn xcb_input_get_device_button_mapping_map_length (R : *mut get_device_button_mapping_reply) -> c_int;


pub fn xcb_input_get_device_button_mapping_map_end (R : *mut get_device_button_mapping_reply) -> ffi::base::generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_input_get_device_button_mapping_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_input_get_device_button_mapping_reply (c : *mut ffi::base::connection,
                                                     cookie : get_device_button_mapping_cookie,
                                                     e : *mut *mut ffi::base::generic_error) -> *mut get_device_button_mapping_reply;

pub fn xcb_input_set_device_button_mapping_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_input_set_device_button_mapping (c : *mut ffi::base::connection,
                                               device_id :  u8,
                                               map_size :  u8,
                                               map : *mut u8) -> set_device_button_mapping_cookie;

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
pub fn xcb_input_set_device_button_mapping_unchecked (c : *mut ffi::base::connection,
                                                         device_id :  u8,
                                                         map_size :  u8,
                                                         map : *mut u8) -> set_device_button_mapping_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_input_set_device_button_mapping_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_input_set_device_button_mapping_reply (c : *mut ffi::base::connection,
                                                     cookie : set_device_button_mapping_cookie,
                                                     e : *mut *mut ffi::base::generic_error) -> *mut set_device_button_mapping_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_input_query_device_state (c : *mut ffi::base::connection,
                                        device_id :  u8) -> query_device_state_cookie;

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
pub fn xcb_input_query_device_state_unchecked (c : *mut ffi::base::connection,
                                                  device_id :  u8) -> query_device_state_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_input_query_device_state_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_input_query_device_state_reply (c : *mut ffi::base::connection,
                                              cookie : query_device_state_cookie,
                                              e : *mut *mut ffi::base::generic_error) -> *mut query_device_state_reply;

/**
 * Get the next element of the iterator
 * @param i Pointer to a input_state_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(input_state)
 *
 *
 */
pub fn xcb_input_input_state_next (i:*mut input_state_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An input_state_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_input_state_end (i:input_state_iterator) -> ffi::base::generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a key_state_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(key_state)
 *
 *
 */
pub fn xcb_input_key_state_next (i:*mut key_state_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An key_state_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_key_state_end (i:key_state_iterator) -> ffi::base::generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a button_state_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(button_state)
 *
 *
 */
pub fn xcb_input_button_state_next (i:*mut button_state_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An button_state_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_button_state_end (i:button_state_iterator) -> ffi::base::generic_iterator;

pub fn xcb_input_valuator_state_sizeof (_buffer :  *mut c_void) -> c_int;

pub fn xcb_input_valuator_state_valuators (R : *mut valuator_state) -> *mut u32;


pub fn xcb_input_valuator_state_valuators_length (R : *mut valuator_state) -> c_int;


pub fn xcb_input_valuator_state_valuators_end (R : *mut valuator_state) -> ffi::base::generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a valuator_state_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(valuator_state)
 *
 *
 */
pub fn xcb_input_valuator_state_next (i:*mut valuator_state_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An valuator_state_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_valuator_state_end (i:valuator_state_iterator) -> ffi::base::generic_iterator;

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
pub fn xcb_input_send_extension_event_checked (c : *mut ffi::base::connection,
                                                  destination :  ffi::xproto::window,
                                                  device_id :  u8,
                                                  propagate :  u8,
                                                  num_classes :  u16,
                                                  num_events :  u8,
                                                  events : *mut c_char,
                                                  classes : *mut event_class) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_input_send_extension_event (c : *mut ffi::base::connection,
                                          destination :  ffi::xproto::window,
                                          device_id :  u8,
                                          propagate :  u8,
                                          num_classes :  u16,
                                          num_events :  u8,
                                          events : *mut c_char,
                                          classes : *mut event_class) -> ffi::base::void_cookie;

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
pub fn xcb_input_device_bell_checked (c : *mut ffi::base::connection,
                                         device_id :  u8,
                                         feedback_id :  u8,
                                         feedback_class :  u8,
                                         percent :  i8) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_input_device_bell (c : *mut ffi::base::connection,
                                 device_id :  u8,
                                 feedback_id :  u8,
                                 feedback_class :  u8,
                                 percent :  i8) -> ffi::base::void_cookie;

pub fn xcb_input_set_device_valuators_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_input_set_device_valuators (c : *mut ffi::base::connection,
                                          device_id :  u8,
                                          first_valuator :  u8,
                                          num_valuators :  u8,
                                          valuators : *mut i32) -> set_device_valuators_cookie;

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
pub fn xcb_input_set_device_valuators_unchecked (c : *mut ffi::base::connection,
                                                    device_id :  u8,
                                                    first_valuator :  u8,
                                                    num_valuators :  u8,
                                                    valuators : *mut i32) -> set_device_valuators_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_input_set_device_valuators_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_input_set_device_valuators_reply (c : *mut ffi::base::connection,
                                                cookie : set_device_valuators_cookie,
                                                e : *mut *mut ffi::base::generic_error) -> *mut set_device_valuators_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_input_get_device_control (c : *mut ffi::base::connection,
                                        control_id :  u16,
                                        device_id :  u8) -> get_device_control_cookie;

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
pub fn xcb_input_get_device_control_unchecked (c : *mut ffi::base::connection,
                                                  control_id :  u16,
                                                  device_id :  u8) -> get_device_control_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_input_get_device_control_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_input_get_device_control_reply (c : *mut ffi::base::connection,
                                              cookie : get_device_control_cookie,
                                              e : *mut *mut ffi::base::generic_error) -> *mut get_device_control_reply;

/**
 * Get the next element of the iterator
 * @param i Pointer to a device_state_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(device_state)
 *
 *
 */
pub fn xcb_input_device_state_next (i:*mut device_state_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An device_state_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_device_state_end (i:device_state_iterator) -> ffi::base::generic_iterator;

pub fn xcb_input_device_resolution_state_sizeof (_buffer :  *mut c_void) -> c_int;

pub fn xcb_input_device_resolution_state_resolution_values (R : *mut device_resolution_state) -> *mut u32;


pub fn xcb_input_device_resolution_state_resolution_values_length (R : *mut device_resolution_state) -> c_int;


pub fn xcb_input_device_resolution_state_resolution_values_end (R : *mut device_resolution_state) -> ffi::base::generic_iterator;

pub fn xcb_input_device_resolution_state_resolution_min (R : *mut device_resolution_state) -> *mut u32;


pub fn xcb_input_device_resolution_state_resolution_min_length (R : *mut device_resolution_state) -> c_int;


pub fn xcb_input_device_resolution_state_resolution_min_end (R : *mut device_resolution_state) -> ffi::base::generic_iterator;

pub fn xcb_input_device_resolution_state_resolution_max (R : *mut device_resolution_state) -> *mut u32;


pub fn xcb_input_device_resolution_state_resolution_max_length (R : *mut device_resolution_state) -> c_int;


pub fn xcb_input_device_resolution_state_resolution_max_end (R : *mut device_resolution_state) -> ffi::base::generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a device_resolution_state_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(device_resolution_state)
 *
 *
 */
pub fn xcb_input_device_resolution_state_next (i:*mut device_resolution_state_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An device_resolution_state_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_device_resolution_state_end (i:device_resolution_state_iterator) -> ffi::base::generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a device_abs_calib_state_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(device_abs_calib_state)
 *
 *
 */
pub fn xcb_input_device_abs_calib_state_next (i:*mut device_abs_calib_state_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An device_abs_calib_state_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_device_abs_calib_state_end (i:device_abs_calib_state_iterator) -> ffi::base::generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a device_abs_area_state_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(device_abs_area_state)
 *
 *
 */
pub fn xcb_input_device_abs_area_state_next (i:*mut device_abs_area_state_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An device_abs_area_state_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_device_abs_area_state_end (i:device_abs_area_state_iterator) -> ffi::base::generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a device_core_state_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(device_core_state)
 *
 *
 */
pub fn xcb_input_device_core_state_next (i:*mut device_core_state_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An device_core_state_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_device_core_state_end (i:device_core_state_iterator) -> ffi::base::generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a device_enable_state_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(device_enable_state)
 *
 *
 */
pub fn xcb_input_device_enable_state_next (i:*mut device_enable_state_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An device_enable_state_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_device_enable_state_end (i:device_enable_state_iterator) -> ffi::base::generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a device_ctl_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(device_ctl)
 *
 *
 */
pub fn xcb_input_device_ctl_next (i:*mut device_ctl_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An device_ctl_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_device_ctl_end (i:device_ctl_iterator) -> ffi::base::generic_iterator;

pub fn xcb_input_device_resolution_ctl_sizeof (_buffer :  *mut c_void) -> c_int;

pub fn xcb_input_device_resolution_ctl_resolution_values (R : *mut device_resolution_ctl) -> *mut u32;


pub fn xcb_input_device_resolution_ctl_resolution_values_length (R : *mut device_resolution_ctl) -> c_int;


pub fn xcb_input_device_resolution_ctl_resolution_values_end (R : *mut device_resolution_ctl) -> ffi::base::generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a device_resolution_ctl_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(device_resolution_ctl)
 *
 *
 */
pub fn xcb_input_device_resolution_ctl_next (i:*mut device_resolution_ctl_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An device_resolution_ctl_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_device_resolution_ctl_end (i:device_resolution_ctl_iterator) -> ffi::base::generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a device_abs_calib_ctl_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(device_abs_calib_ctl)
 *
 *
 */
pub fn xcb_input_device_abs_calib_ctl_next (i:*mut device_abs_calib_ctl_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An device_abs_calib_ctl_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_device_abs_calib_ctl_end (i:device_abs_calib_ctl_iterator) -> ffi::base::generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a device_abs_area_ctrl_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(device_abs_area_ctrl)
 *
 *
 */
pub fn xcb_input_device_abs_area_ctrl_next (i:*mut device_abs_area_ctrl_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An device_abs_area_ctrl_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_device_abs_area_ctrl_end (i:device_abs_area_ctrl_iterator) -> ffi::base::generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a device_core_ctrl_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(device_core_ctrl)
 *
 *
 */
pub fn xcb_input_device_core_ctrl_next (i:*mut device_core_ctrl_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An device_core_ctrl_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_device_core_ctrl_end (i:device_core_ctrl_iterator) -> ffi::base::generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a device_enable_ctrl_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(device_enable_ctrl)
 *
 *
 */
pub fn xcb_input_device_enable_ctrl_next (i:*mut device_enable_ctrl_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An device_enable_ctrl_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_input_device_enable_ctrl_end (i:device_enable_ctrl_iterator) -> ffi::base::generic_iterator;
}

