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
pub struct key_code_iterator {
    data : *mut key_code,
    rem  : c_int,
    index: c_int
}


pub type event_class = u32;
/**
 * @brief event_class_iterator
 **/
pub struct event_class_iterator {
    data : *mut event_class,
    rem  : c_int,
    index: c_int
}


pub struct get_extension_version_cookie {
    sequence : c_uint
}


pub struct get_extension_version_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    name_len :       u16,
    pad0 :           [u8,..2]
}


pub struct get_extension_version_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    server_major :    u16,
    server_minor :    u16,
    present :         u8,
    pad1 :            [u8,..19]
}


pub struct device_info {
    device_type :      ffi::xproto::atom,
    device_id :        u8,
    num_class_info :   u8,
    device_use :       u8,
    pad0 :             u8
}

/**
 * @brief device_info_iterator
 **/
pub struct device_info_iterator {
    data : *mut device_info,
    rem  : c_int,
    index: c_int
}


pub struct list_input_devices_cookie {
    sequence : c_uint
}


pub struct list_input_devices_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16
}


pub struct list_input_devices_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    devices_len :     u8,
    pad1 :            [u8,..23]
}


pub struct input_info {
    class_id :   u8,
    len :        u8
}

/**
 * @brief input_info_iterator
 **/
pub struct input_info_iterator {
    data : *mut input_info,
    rem  : c_int,
    index: c_int
}


pub struct key_info {
    class_id :      u8,
    len :           u8,
    min_keycode :   key_code,
    max_keycode :   key_code,
    num_keys :      u16,
    pad0 :          [u8,..2]
}

/**
 * @brief key_info_iterator
 **/
pub struct key_info_iterator {
    data : *mut key_info,
    rem  : c_int,
    index: c_int
}


pub struct button_info {
    class_id :      u8,
    len :           u8,
    num_buttons :   u16
}

/**
 * @brief button_info_iterator
 **/
pub struct button_info_iterator {
    data : *mut button_info,
    rem  : c_int,
    index: c_int
}


pub struct axis_info {
    resolution :   u32,
    minimum :      i32,
    maximum :      i32
}

/**
 * @brief axis_info_iterator
 **/
pub struct axis_info_iterator {
    data : *mut axis_info,
    rem  : c_int,
    index: c_int
}


pub struct valuator_info {
    class_id :      u8,
    len :           u8,
    axes_len :      u8,
    mode :          u8,
    motion_size :   u32
}

/**
 * @brief valuator_info_iterator
 **/
pub struct valuator_info_iterator {
    data : *mut valuator_info,
    rem  : c_int,
    index: c_int
}


pub struct input_class_info {
    class_id :          u8,
    event_type_base :   u8
}

/**
 * @brief input_class_info_iterator
 **/
pub struct input_class_info_iterator {
    data : *mut input_class_info,
    rem  : c_int,
    index: c_int
}


pub struct open_device_cookie {
    sequence : c_uint
}


pub struct open_device_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    device_id :      u8,
    pad0 :           [u8,..3]
}


pub struct open_device_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    num_classes :     u8,
    pad1 :            [u8,..23]
}



pub struct close_device_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    device_id :      u8,
    pad0 :           [u8,..3]
}


pub struct set_device_mode_cookie {
    sequence : c_uint
}


pub struct set_device_mode_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    device_id :      u8,
    mode :           u8,
    pad0 :           [u8,..2]
}


pub struct set_device_mode_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    status :          u8,
    pad1 :            [u8,..23]
}



pub struct select_extension_event_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    window :         ffi::xproto::window,
    num_classes :    u16,
    pad0 :           [u8,..2]
}


pub struct get_selected_extension_events_cookie {
    sequence : c_uint
}


pub struct get_selected_extension_events_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    window :         ffi::xproto::window
}


pub struct get_selected_extension_events_reply {
    response_type :      u8,
    pad0 :               u8,
    sequence :           u16,
    length :             u32,
    num_this_classes :   u16,
    num_all_classes :    u16,
    pad1 :               [u8,..20]
}



pub struct change_device_dont_propagate_list_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    window :         ffi::xproto::window,
    num_classes :    u16,
    mode :           u8,
    pad0 :           u8
}


pub struct get_device_dont_propagate_list_cookie {
    sequence : c_uint
}


pub struct get_device_dont_propagate_list_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    window :         ffi::xproto::window
}


pub struct get_device_dont_propagate_list_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    num_classes :     u16,
    pad1 :            [u8,..22]
}


pub struct get_device_motion_events_cookie {
    sequence : c_uint
}


pub struct get_device_motion_events_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    start :          ffi::xproto::timestamp,
    stop :           ffi::xproto::timestamp,
    device_id :      u8
}


pub struct get_device_motion_events_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    num_coords :      u32,
    num_axes :        u8,
    device_mode :     u8,
    pad1 :            [u8,..18]
}


pub struct device_time_coord {
    time :   ffi::xproto::timestamp
}

/**
 * @brief device_time_coord_iterator
 **/
pub struct device_time_coord_iterator {
    data : *mut device_time_coord,
    rem  : c_int,
    index: c_int
}


pub struct change_keyboard_device_cookie {
    sequence : c_uint
}


pub struct change_keyboard_device_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    device_id :      u8,
    pad0 :           [u8,..3]
}


pub struct change_keyboard_device_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    status :          u8,
    pad1 :            [u8,..23]
}


pub struct change_pointer_device_cookie {
    sequence : c_uint
}


pub struct change_pointer_device_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    x_axis :         u8,
    y_axis :         u8,
    device_id :      u8,
    pad0 :           u8
}


pub struct change_pointer_device_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    status :          u8,
    pad1 :            [u8,..23]
}


pub struct grab_device_cookie {
    sequence : c_uint
}


pub struct grab_device_request {
    major_opcode :        u8,
    minor_opcode :        u8,
    length :              u16,
    grab_window :         ffi::xproto::window,
    time :                ffi::xproto::timestamp,
    num_classes :         u16,
    this_device_mode :    u8,
    other_device_mode :   u8,
    owner_events :        u8,
    device_id :           u8,
    pad0 :                [u8,..2]
}


pub struct grab_device_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    status :          u8,
    pad1 :            [u8,..23]
}



pub struct ungrab_device_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    time :           ffi::xproto::timestamp,
    device_id :      u8
}



pub struct grab_device_key_request {
    major_opcode :        u8,
    minor_opcode :        u8,
    length :              u16,
    grab_window :         ffi::xproto::window,
    num_classes :         u16,
    modifiers :           u16,
    modifier_device :     u8,
    grabbed_device :      u8,
    key :                 u8,
    this_device_mode :    u8,
    other_device_mode :   u8,
    owner_events :        u8,
    pad0 :                [u8,..2]
}



pub struct ungrab_device_key_request {
    major_opcode :      u8,
    minor_opcode :      u8,
    length :            u16,
    grabWindow :        ffi::xproto::window,
    modifiers :         u16,
    modifier_device :   u8,
    key :               u8,
    grabbed_device :    u8
}



pub struct grab_device_button_request {
    major_opcode :        u8,
    minor_opcode :        u8,
    length :              u16,
    grab_window :         ffi::xproto::window,
    grabbed_device :      u8,
    modifier_device :     u8,
    num_classes :         u16,
    modifiers :           u16,
    this_device_mode :    u8,
    other_device_mode :   u8,
    button :              u8,
    owner_events :        u8,
    pad0 :                [u8,..2]
}



pub struct ungrab_device_button_request {
    major_opcode :      u8,
    minor_opcode :      u8,
    length :            u16,
    grab_window :       ffi::xproto::window,
    modifiers :         u16,
    modifier_device :   u8,
    button :            u8,
    grabbed_device :    u8
}



pub struct allow_device_events_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    time :           ffi::xproto::timestamp,
    mode :           u8,
    device_id :      u8
}


pub struct get_device_focus_cookie {
    sequence : c_uint
}


pub struct get_device_focus_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    device_id :      u8,
    pad0 :           [u8,..3]
}


pub struct get_device_focus_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    focus :           ffi::xproto::window,
    time :            ffi::xproto::timestamp,
    revert_to :       u8,
    pad1 :            [u8,..15]
}



pub struct set_device_focus_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    focus :          ffi::xproto::window,
    time :           ffi::xproto::timestamp,
    revert_to :      u8,
    device_id :      u8
}


pub struct get_feedback_control_cookie {
    sequence : c_uint
}


pub struct get_feedback_control_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    device_id :      u8,
    pad0 :           [u8,..3]
}


pub struct get_feedback_control_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    num_feedback :    u16,
    pad1 :            [u8,..22]
}


pub struct feedback_state {
    class_id :   u8,
    id :         u8,
    len :        u16
}

/**
 * @brief feedback_state_iterator
 **/
pub struct feedback_state_iterator {
    data : *mut feedback_state,
    rem  : c_int,
    index: c_int
}


pub struct kbd_feedback_state {
    class_id :             u8,
    id :                   u8,
    len :                  u16,
    pitch :                u16,
    duration :             u16,
    led_mask :             u32,
    led_values :           u32,
    global_auto_repeat :   u8,
    click :                u8,
    percent :              u8,
    pad0 :                 u8,
    auto_repeats :         [u8,..32]
}

/**
 * @brief kbd_feedback_state_iterator
 **/
pub struct kbd_feedback_state_iterator {
    data : *mut kbd_feedback_state,
    rem  : c_int,
    index: c_int
}


pub struct ptr_feedback_state {
    class_id :      u8,
    id :            u8,
    len :           u16,
    pad0 :          [u8,..2],
    accel_num :     u16,
    accel_denom :   u16,
    threshold :     u16
}

/**
 * @brief ptr_feedback_state_iterator
 **/
pub struct ptr_feedback_state_iterator {
    data : *mut ptr_feedback_state,
    rem  : c_int,
    index: c_int
}


pub struct integer_feedback_state {
    class_id :     u8,
    id :           u8,
    len :          u16,
    resolution :   u32,
    min_value :    i32,
    max_value :    i32
}

/**
 * @brief integer_feedback_state_iterator
 **/
pub struct integer_feedback_state_iterator {
    data : *mut integer_feedback_state,
    rem  : c_int,
    index: c_int
}


pub struct string_feedback_state {
    class_id :      u8,
    id :            u8,
    len :           u16,
    max_symbols :   u16,
    num_keysyms :   u16
}

/**
 * @brief string_feedback_state_iterator
 **/
pub struct string_feedback_state_iterator {
    data : *mut string_feedback_state,
    rem  : c_int,
    index: c_int
}


pub struct bell_feedback_state {
    class_id :   u8,
    id :         u8,
    len :        u16,
    percent :    u8,
    pad0 :       [u8,..3],
    pitch :      u16,
    duration :   u16
}

/**
 * @brief bell_feedback_state_iterator
 **/
pub struct bell_feedback_state_iterator {
    data : *mut bell_feedback_state,
    rem  : c_int,
    index: c_int
}


pub struct led_feedback_state {
    class_id :     u8,
    id :           u8,
    len :          u16,
    led_mask :     u32,
    led_values :   u32
}

/**
 * @brief led_feedback_state_iterator
 **/
pub struct led_feedback_state_iterator {
    data : *mut led_feedback_state,
    rem  : c_int,
    index: c_int
}


pub struct feedback_ctl {
    class_id :   u8,
    id :         u8,
    len :        u16
}

/**
 * @brief feedback_ctl_iterator
 **/
pub struct feedback_ctl_iterator {
    data : *mut feedback_ctl,
    rem  : c_int,
    index: c_int
}


pub struct kbd_feedback_ctl {
    class_id :            u8,
    id :                  u8,
    len :                 u16,
    key :                 key_code,
    auto_repeat_mode :    u8,
    key_click_percent :   i8,
    bell_percent :        i8,
    bell_pitch :          i16,
    bell_duration :       i16,
    led_mask :            u32,
    led_values :          u32
}

/**
 * @brief kbd_feedback_ctl_iterator
 **/
pub struct kbd_feedback_ctl_iterator {
    data : *mut kbd_feedback_ctl,
    rem  : c_int,
    index: c_int
}


pub struct ptr_feedback_ctl {
    class_id :    u8,
    id :          u8,
    len :         u16,
    pad0 :        [u8,..2],
    num :         i16,
    denom :       i16,
    threshold :   i16
}

/**
 * @brief ptr_feedback_ctl_iterator
 **/
pub struct ptr_feedback_ctl_iterator {
    data : *mut ptr_feedback_ctl,
    rem  : c_int,
    index: c_int
}


pub struct integer_feedback_ctl {
    class_id :         u8,
    id :               u8,
    len :              u16,
    int_to_display :   i32
}

/**
 * @brief integer_feedback_ctl_iterator
 **/
pub struct integer_feedback_ctl_iterator {
    data : *mut integer_feedback_ctl,
    rem  : c_int,
    index: c_int
}


pub struct string_feedback_ctl {
    class_id :      u8,
    id :            u8,
    len :           u16,
    pad0 :          [u8,..2],
    num_keysyms :   u16
}

/**
 * @brief string_feedback_ctl_iterator
 **/
pub struct string_feedback_ctl_iterator {
    data : *mut string_feedback_ctl,
    rem  : c_int,
    index: c_int
}


pub struct bell_feedback_ctl {
    class_id :   u8,
    id :         u8,
    len :        u16,
    percent :    i8,
    pad0 :       [u8,..3],
    pitch :      i16,
    duration :   i16
}

/**
 * @brief bell_feedback_ctl_iterator
 **/
pub struct bell_feedback_ctl_iterator {
    data : *mut bell_feedback_ctl,
    rem  : c_int,
    index: c_int
}


pub struct led_feedback_ctl {
    class_id :     u8,
    id :           u8,
    len :          u16,
    led_mask :     u32,
    led_values :   u32
}

/**
 * @brief led_feedback_ctl_iterator
 **/
pub struct led_feedback_ctl_iterator {
    data : *mut led_feedback_ctl,
    rem  : c_int,
    index: c_int
}


pub struct get_device_key_mapping_cookie {
    sequence : c_uint
}


pub struct get_device_key_mapping_request {
    major_opcode :    u8,
    minor_opcode :    u8,
    length :          u16,
    device_id :       u8,
    first_keycode :   key_code,
    count :           u8
}


pub struct get_device_key_mapping_reply {
    response_type :         u8,
    pad0 :                  u8,
    sequence :              u16,
    length :                u32,
    keysyms_per_keycode :   u8,
    pad1 :                  [u8,..23]
}



pub struct change_device_key_mapping_request {
    major_opcode :          u8,
    minor_opcode :          u8,
    length :                u16,
    device_id :             u8,
    first_keycode :         key_code,
    keysyms_per_keycode :   u8,
    keycode_count :         u8
}


pub struct get_device_modifier_mapping_cookie {
    sequence : c_uint
}


pub struct get_device_modifier_mapping_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    device_id :      u8,
    pad0 :           [u8,..3]
}


pub struct get_device_modifier_mapping_reply {
    response_type :           u8,
    pad0 :                    u8,
    sequence :                u16,
    length :                  u32,
    keycodes_per_modifier :   u8,
    pad1 :                    [u8,..23]
}


pub struct set_device_modifier_mapping_cookie {
    sequence : c_uint
}


pub struct set_device_modifier_mapping_request {
    major_opcode :            u8,
    minor_opcode :            u8,
    length :                  u16,
    device_id :               u8,
    keycodes_per_modifier :   u8,
    pad0 :                    u8
}


pub struct set_device_modifier_mapping_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    status :          u8,
    pad1 :            [u8,..23]
}


pub struct get_device_button_mapping_cookie {
    sequence : c_uint
}


pub struct get_device_button_mapping_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    device_id :      u8,
    pad0 :           [u8,..3]
}


pub struct get_device_button_mapping_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    map_size :        u8,
    pad1 :            [u8,..23]
}


pub struct set_device_button_mapping_cookie {
    sequence : c_uint
}


pub struct set_device_button_mapping_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    device_id :      u8,
    map_size :       u8,
    pad0 :           [u8,..2]
}


pub struct set_device_button_mapping_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    status :          u8,
    pad1 :            [u8,..23]
}


pub struct query_device_state_cookie {
    sequence : c_uint
}


pub struct query_device_state_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    device_id :      u8,
    pad0 :           [u8,..3]
}


pub struct query_device_state_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    num_classes :     u8,
    pad1 :            [u8,..23]
}


pub struct input_state {
    class_id :    u8,
    len :         u8,
    num_items :   u8
}

/**
 * @brief input_state_iterator
 **/
pub struct input_state_iterator {
    data : *mut input_state,
    rem  : c_int,
    index: c_int
}


pub struct key_state {
    class_id :   u8,
    len :        u8,
    num_keys :   u8,
    pad0 :       u8,
    keys :       [u8,..32]
}

/**
 * @brief key_state_iterator
 **/
pub struct key_state_iterator {
    data : *mut key_state,
    rem  : c_int,
    index: c_int
}


pub struct button_state {
    class_id :      u8,
    len :           u8,
    num_buttons :   u8,
    pad0 :          u8,
    buttons :       [u8,..32]
}

/**
 * @brief button_state_iterator
 **/
pub struct button_state_iterator {
    data : *mut button_state,
    rem  : c_int,
    index: c_int
}


pub struct valuator_state {
    class_id :        u8,
    len :             u8,
    num_valuators :   u8,
    mode :            u8
}

/**
 * @brief valuator_state_iterator
 **/
pub struct valuator_state_iterator {
    data : *mut valuator_state,
    rem  : c_int,
    index: c_int
}



pub struct send_extension_event_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    destination :    ffi::xproto::window,
    device_id :      u8,
    propagate :      u8,
    num_classes :    u16,
    num_events :     u8,
    pad0 :           [u8,..3]
}



pub struct device_bell_request {
    major_opcode :     u8,
    minor_opcode :     u8,
    length :           u16,
    device_id :        u8,
    feedback_id :      u8,
    feedback_class :   u8,
    percent :          i8
}


pub struct set_device_valuators_cookie {
    sequence : c_uint
}


pub struct set_device_valuators_request {
    major_opcode :     u8,
    minor_opcode :     u8,
    length :           u16,
    device_id :        u8,
    first_valuator :   u8,
    num_valuators :    u8,
    pad0 :             u8
}


pub struct set_device_valuators_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    status :          u8,
    pad1 :            [u8,..23]
}


pub struct get_device_control_cookie {
    sequence : c_uint
}


pub struct get_device_control_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    control_id :     u16,
    device_id :      u8,
    pad0 :           u8
}


pub struct get_device_control_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    status :          u8,
    pad1 :            [u8,..23]
}


pub struct device_state {
    control_id :   u16,
    len :          u16
}

/**
 * @brief device_state_iterator
 **/
pub struct device_state_iterator {
    data : *mut device_state,
    rem  : c_int,
    index: c_int
}


pub struct device_resolution_state {
    control_id :      u16,
    len :             u16,
    num_valuators :   u32
}

/**
 * @brief device_resolution_state_iterator
 **/
pub struct device_resolution_state_iterator {
    data : *mut device_resolution_state,
    rem  : c_int,
    index: c_int
}


pub struct device_abs_calib_state {
    control_id :         u16,
    len :                u16,
    min_x :              i32,
    max_x :              i32,
    min_y :              i32,
    max_y :              i32,
    flip_x :             u32,
    flip_y :             u32,
    rotation :           u32,
    button_threshold :   u32
}

/**
 * @brief device_abs_calib_state_iterator
 **/
pub struct device_abs_calib_state_iterator {
    data : *mut device_abs_calib_state,
    rem  : c_int,
    index: c_int
}


pub struct device_abs_area_state {
    control_id :   u16,
    len :          u16,
    offset_x :     u32,
    offset_y :     u32,
    width :        u32,
    height :       u32,
    screen :       u32,
    following :    u32
}

/**
 * @brief device_abs_area_state_iterator
 **/
pub struct device_abs_area_state_iterator {
    data : *mut device_abs_area_state,
    rem  : c_int,
    index: c_int
}


pub struct device_core_state {
    control_id :   u16,
    len :          u16,
    status :       u8,
    iscore :       u8,
    pad0 :         [u8,..2]
}

/**
 * @brief device_core_state_iterator
 **/
pub struct device_core_state_iterator {
    data : *mut device_core_state,
    rem  : c_int,
    index: c_int
}


pub struct device_enable_state {
    control_id :   u16,
    len :          u16,
    enable :       u8,
    pad0 :         [u8,..3]
}

/**
 * @brief device_enable_state_iterator
 **/
pub struct device_enable_state_iterator {
    data : *mut device_enable_state,
    rem  : c_int,
    index: c_int
}


pub struct device_ctl {
    control_id :   u16,
    len :          u16
}

/**
 * @brief device_ctl_iterator
 **/
pub struct device_ctl_iterator {
    data : *mut device_ctl,
    rem  : c_int,
    index: c_int
}


pub struct device_resolution_ctl {
    control_id :       u16,
    len :              u16,
    first_valuator :   u8,
    num_valuators :    u8
}

/**
 * @brief device_resolution_ctl_iterator
 **/
pub struct device_resolution_ctl_iterator {
    data : *mut device_resolution_ctl,
    rem  : c_int,
    index: c_int
}


pub struct device_abs_calib_ctl {
    control_id :         u16,
    len :                u16,
    min_x :              i32,
    max_x :              i32,
    min_y :              i32,
    max_y :              i32,
    flip_x :             u32,
    flip_y :             u32,
    rotation :           u32,
    button_threshold :   u32
}

/**
 * @brief device_abs_calib_ctl_iterator
 **/
pub struct device_abs_calib_ctl_iterator {
    data : *mut device_abs_calib_ctl,
    rem  : c_int,
    index: c_int
}


pub struct device_abs_area_ctrl {
    control_id :   u16,
    len :          u16,
    offset_x :     u32,
    offset_y :     u32,
    width :        i32,
    height :       i32,
    screen :       i32,
    following :    u32
}

/**
 * @brief device_abs_area_ctrl_iterator
 **/
pub struct device_abs_area_ctrl_iterator {
    data : *mut device_abs_area_ctrl,
    rem  : c_int,
    index: c_int
}


pub struct device_core_ctrl {
    control_id :   u16,
    len :          u16,
    status :       u8,
    pad0 :         [u8,..3]
}

/**
 * @brief device_core_ctrl_iterator
 **/
pub struct device_core_ctrl_iterator {
    data : *mut device_core_ctrl,
    rem  : c_int,
    index: c_int
}


pub struct device_enable_ctrl {
    control_id :   u16,
    len :          u16,
    enable :       u8,
    pad0 :         [u8,..3]
}

/**
 * @brief device_enable_ctrl_iterator
 **/
pub struct device_enable_ctrl_iterator {
    data : *mut device_enable_ctrl,
    rem  : c_int,
    index: c_int
}



pub struct device_valuator_event {
    response_type :    u8,
    device_id :        u8,
    sequence :         u16,
    device_state :     u16,
    num_valuators :    u8,
    first_valuator :   u8,
    valuators :        [i32,..6]
}



pub struct device_key_press_event {
    response_type :   u8,
    detail :          u8,
    sequence :        u16,
    time :            ffi::xproto::timestamp,
    root :            ffi::xproto::window,
    event :           ffi::xproto::window,
    child :           ffi::xproto::window,
    root_x :          i16,
    root_y :          i16,
    event_x :         i16,
    event_y :         i16,
    state :           u16,
    same_screen :     u8,
    device_id :       u8
}



pub type device_key_release_event = device_key_press_event;


pub type device_button_press_event = device_key_press_event;


pub type device_button_release_event = device_key_press_event;


pub type device_motion_notify_event = device_key_press_event;


pub type proximity_in_event = device_key_press_event;


pub type proximity_out_event = device_key_press_event;


pub struct focus_in_event {
    response_type :   u8,
    detail :          u8,
    sequence :        u16,
    time :            ffi::xproto::timestamp,
    window :          ffi::xproto::window,
    mode :            u8,
    device_id :       u8,
    pad0 :            [u8,..18]
}



pub type focus_out_event = focus_in_event;


pub struct device_state_notify_event {
    response_type :      u8,
    device_id :          u8,
    sequence :           u16,
    time :               ffi::xproto::timestamp,
    num_keys :           u8,
    num_buttons :        u8,
    num_valuators :      u8,
    classes_reported :   u8,
    buttons :            [u8,..4],
    keys :               [u8,..4],
    valuators :          [u32,..3]
}



pub struct device_mapping_notify_event {
    response_type :   u8,
    device_id :       u8,
    sequence :        u16,
    request :         u8,
    first_keycode :   key_code,
    count :           u8,
    pad0 :            u8,
    time :            ffi::xproto::timestamp,
    pad1 :            [u8,..20]
}



pub struct change_device_notify_event {
    response_type :   u8,
    device_id :       u8,
    sequence :        u16,
    time :            ffi::xproto::timestamp,
    request :         u8,
    pad0 :            [u8,..23]
}



pub struct device_key_state_notify_event {
    response_type :   u8,
    device_id :       u8,
    sequence :        u16,
    keys :            [u8,..28]
}



pub struct device_button_state_notify_event {
    response_type :   u8,
    device_id :       u8,
    sequence :        u16,
    buttons :         [u8,..28]
}



pub struct device_presence_notify_event {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    time :            ffi::xproto::timestamp,
    devchange :       u8,
    device_id :       u8,
    control :         u16,
    pad1 :            [u8,..20]
}



pub struct device_error {
    response_type :   u8,
    error_code :      u8,
    sequence :        u16
}



pub struct event_error {
    response_type :   u8,
    error_code :      u8,
    sequence :        u16
}



pub struct mode_error {
    response_type :   u8,
    error_code :      u8,
    sequence :        u16
}



pub struct device_busy_error {
    response_type :   u8,
    error_code :      u8,
    sequence :        u16
}



pub struct class_error {
    response_type :   u8,
    error_code :      u8,
    sequence :        u16
}

#[link(name="lxcb-xinput")]
pub extern "C" {

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

