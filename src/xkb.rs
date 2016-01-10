//
// This file generated automatically from xkb.xml by r_client.py.
// Edit at your peril.
//

//Make the compiler quiet
#![allow(unused_imports)]
#![allow(unused_unsafe)]
use std;
use libc::*;
use std::{mem,num,ptr,str};
use ffi::base::*;
use base;
use base::*;
use ffi;
use ffi::xkb::*;
use std::option::Option;
use std::iter::Iterator;

use xproto;

pub type xcb_xkb_const_t = c_uint;//{
    pub const XCB_XKB_CONST_MAX_LEGAL_KEY_CODE : xcb_xkb_const_t = 255;
    pub const XCB_XKB_CONST_PER_KEY_BIT_ARRAY_SIZE : xcb_xkb_const_t = 32;
    pub const XCB_XKB_CONST_KEY_NAME_LENGTH : xcb_xkb_const_t = 4;
//}

pub type xcb_xkb_event_type_t = c_uint;//{
    pub const XCB_XKB_EVENT_TYPE_NEW_KEYBOARD_NOTIFY : xcb_xkb_event_type_t = 1;
    pub const XCB_XKB_EVENT_TYPE_MAP_NOTIFY : xcb_xkb_event_type_t = 2;
    pub const XCB_XKB_EVENT_TYPE_STATE_NOTIFY : xcb_xkb_event_type_t = 4;
    pub const XCB_XKB_EVENT_TYPE_CONTROLS_NOTIFY : xcb_xkb_event_type_t = 8;
    pub const XCB_XKB_EVENT_TYPE_INDICATOR_STATE_NOTIFY : xcb_xkb_event_type_t = 16;
    pub const XCB_XKB_EVENT_TYPE_INDICATOR_MAP_NOTIFY : xcb_xkb_event_type_t = 32;
    pub const XCB_XKB_EVENT_TYPE_NAMES_NOTIFY : xcb_xkb_event_type_t = 64;
    pub const XCB_XKB_EVENT_TYPE_COMPAT_MAP_NOTIFY : xcb_xkb_event_type_t = 128;
    pub const XCB_XKB_EVENT_TYPE_BELL_NOTIFY : xcb_xkb_event_type_t = 256;
    pub const XCB_XKB_EVENT_TYPE_ACTION_MESSAGE : xcb_xkb_event_type_t = 512;
    pub const XCB_XKB_EVENT_TYPE_ACCESS_X_NOTIFY : xcb_xkb_event_type_t = 1024;
    pub const XCB_XKB_EVENT_TYPE_EXTENSION_DEVICE_NOTIFY : xcb_xkb_event_type_t = 2048;
//}

pub type xcb_xkb_nkn_detail_t = c_uint;//{
    pub const XCB_XKB_NKN_DETAIL_KEYCODES : xcb_xkb_nkn_detail_t = 1;
    pub const XCB_XKB_NKN_DETAIL_GEOMETRY : xcb_xkb_nkn_detail_t = 2;
    pub const XCB_XKB_NKN_DETAIL_DEVICE_ID : xcb_xkb_nkn_detail_t = 4;
//}

pub type xcb_xkb_axn_detail_t = c_uint;//{
    pub const XCB_XKB_AXN_DETAIL_SK_PRESS : xcb_xkb_axn_detail_t = 1;
    pub const XCB_XKB_AXN_DETAIL_SK_ACCEPT : xcb_xkb_axn_detail_t = 2;
    pub const XCB_XKB_AXN_DETAIL_SK_REJECT : xcb_xkb_axn_detail_t = 4;
    pub const XCB_XKB_AXN_DETAIL_SK_RELEASE : xcb_xkb_axn_detail_t = 8;
    pub const XCB_XKB_AXN_DETAIL_BK_ACCEPT : xcb_xkb_axn_detail_t = 16;
    pub const XCB_XKB_AXN_DETAIL_BK_REJECT : xcb_xkb_axn_detail_t = 32;
    pub const XCB_XKB_AXN_DETAIL_AXK_WARNING : xcb_xkb_axn_detail_t = 64;
//}

pub type xcb_xkb_map_part_t = c_uint;//{
    pub const XCB_XKB_MAP_PART_KEY_TYPES : xcb_xkb_map_part_t = 1;
    pub const XCB_XKB_MAP_PART_KEY_SYMS : xcb_xkb_map_part_t = 2;
    pub const XCB_XKB_MAP_PART_MODIFIER_MAP : xcb_xkb_map_part_t = 4;
    pub const XCB_XKB_MAP_PART_EXPLICIT_COMPONENTS : xcb_xkb_map_part_t = 8;
    pub const XCB_XKB_MAP_PART_KEY_ACTIONS : xcb_xkb_map_part_t = 16;
    pub const XCB_XKB_MAP_PART_KEY_BEHAVIORS : xcb_xkb_map_part_t = 32;
    pub const XCB_XKB_MAP_PART_VIRTUAL_MODS : xcb_xkb_map_part_t = 64;
    pub const XCB_XKB_MAP_PART_VIRTUAL_MOD_MAP : xcb_xkb_map_part_t = 128;
//}

pub type xcb_xkb_set_map_flags_t = c_uint;//{
    pub const XCB_XKB_SET_MAP_FLAGS_RESIZE_TYPES : xcb_xkb_set_map_flags_t = 1;
    pub const XCB_XKB_SET_MAP_FLAGS_RECOMPUTE_ACTIONS : xcb_xkb_set_map_flags_t = 2;
//}

pub type xcb_xkb_state_part_t = c_uint;//{
    pub const XCB_XKB_STATE_PART_MODIFIER_STATE : xcb_xkb_state_part_t = 1;
    pub const XCB_XKB_STATE_PART_MODIFIER_BASE : xcb_xkb_state_part_t = 2;
    pub const XCB_XKB_STATE_PART_MODIFIER_LATCH : xcb_xkb_state_part_t = 4;
    pub const XCB_XKB_STATE_PART_MODIFIER_LOCK : xcb_xkb_state_part_t = 8;
    pub const XCB_XKB_STATE_PART_GROUP_STATE : xcb_xkb_state_part_t = 16;
    pub const XCB_XKB_STATE_PART_GROUP_BASE : xcb_xkb_state_part_t = 32;
    pub const XCB_XKB_STATE_PART_GROUP_LATCH : xcb_xkb_state_part_t = 64;
    pub const XCB_XKB_STATE_PART_GROUP_LOCK : xcb_xkb_state_part_t = 128;
    pub const XCB_XKB_STATE_PART_COMPAT_STATE : xcb_xkb_state_part_t = 256;
    pub const XCB_XKB_STATE_PART_GRAB_MODS : xcb_xkb_state_part_t = 512;
    pub const XCB_XKB_STATE_PART_COMPAT_GRAB_MODS : xcb_xkb_state_part_t = 1024;
    pub const XCB_XKB_STATE_PART_LOOKUP_MODS : xcb_xkb_state_part_t = 2048;
    pub const XCB_XKB_STATE_PART_COMPAT_LOOKUP_MODS : xcb_xkb_state_part_t = 4096;
    pub const XCB_XKB_STATE_PART_POINTER_BUTTONS : xcb_xkb_state_part_t = 8192;
//}

pub type xcb_xkb_bool_ctrl_t = c_uint;//{
    pub const XCB_XKB_BOOL_CTRL_REPEAT_KEYS : xcb_xkb_bool_ctrl_t = 1;
    pub const XCB_XKB_BOOL_CTRL_SLOW_KEYS : xcb_xkb_bool_ctrl_t = 2;
    pub const XCB_XKB_BOOL_CTRL_BOUNCE_KEYS : xcb_xkb_bool_ctrl_t = 4;
    pub const XCB_XKB_BOOL_CTRL_STICKY_KEYS : xcb_xkb_bool_ctrl_t = 8;
    pub const XCB_XKB_BOOL_CTRL_MOUSE_KEYS : xcb_xkb_bool_ctrl_t = 16;
    pub const XCB_XKB_BOOL_CTRL_MOUSE_KEYS_ACCEL : xcb_xkb_bool_ctrl_t = 32;
    pub const XCB_XKB_BOOL_CTRL_ACCESS_X_KEYS : xcb_xkb_bool_ctrl_t = 64;
    pub const XCB_XKB_BOOL_CTRL_ACCESS_X_TIMEOUT_MASK : xcb_xkb_bool_ctrl_t = 128;
    pub const XCB_XKB_BOOL_CTRL_ACCESS_X_FEEDBACK_MASK : xcb_xkb_bool_ctrl_t = 256;
    pub const XCB_XKB_BOOL_CTRL_AUDIBLE_BELL_MASK : xcb_xkb_bool_ctrl_t = 512;
    pub const XCB_XKB_BOOL_CTRL_OVERLAY_1_MASK : xcb_xkb_bool_ctrl_t = 1024;
    pub const XCB_XKB_BOOL_CTRL_OVERLAY_2_MASK : xcb_xkb_bool_ctrl_t = 2048;
    pub const XCB_XKB_BOOL_CTRL_IGNORE_GROUP_LOCK_MASK : xcb_xkb_bool_ctrl_t = 4096;
//}

pub type xcb_xkb_control_t = c_uint;//{
    pub const XCB_XKB_CONTROL_GROUPS_WRAP : xcb_xkb_control_t = 134217728;
    pub const XCB_XKB_CONTROL_INTERNAL_MODS : xcb_xkb_control_t = 268435456;
    pub const XCB_XKB_CONTROL_IGNORE_LOCK_MODS : xcb_xkb_control_t = 536870912;
    pub const XCB_XKB_CONTROL_PER_KEY_REPEAT : xcb_xkb_control_t = 1073741824;
    pub const XCB_XKB_CONTROL_CONTROLS_ENABLED : xcb_xkb_control_t = 2147483648;
//}

pub type xcb_xkb_axfb_opt_t = c_uint;//{
    pub const XCB_XKB_AXFB_OPT_SK_PRESS_FB : xcb_xkb_axfb_opt_t = 1;
    pub const XCB_XKB_AXFB_OPT_SK_ACCEPT_FB : xcb_xkb_axfb_opt_t = 2;
    pub const XCB_XKB_AXFB_OPT_FEATURE_FB : xcb_xkb_axfb_opt_t = 4;
    pub const XCB_XKB_AXFB_OPT_SLOW_WARN_FB : xcb_xkb_axfb_opt_t = 8;
    pub const XCB_XKB_AXFB_OPT_INDICATOR_FB : xcb_xkb_axfb_opt_t = 16;
    pub const XCB_XKB_AXFB_OPT_STICKY_KEYS_FB : xcb_xkb_axfb_opt_t = 32;
    pub const XCB_XKB_AXFB_OPT_SK_RELEASE_FB : xcb_xkb_axfb_opt_t = 64;
    pub const XCB_XKB_AXFB_OPT_SK_REJECT_FB : xcb_xkb_axfb_opt_t = 128;
    pub const XCB_XKB_AXFB_OPT_BK_REJECT_FB : xcb_xkb_axfb_opt_t = 256;
    pub const XCB_XKB_AXFB_OPT_DUMB_BELL : xcb_xkb_axfb_opt_t = 512;
//}

pub type xcb_xkb_axsk_opt_t = c_uint;//{
    pub const XCB_XKB_AXSK_OPT_TWO_KEYS : xcb_xkb_axsk_opt_t = 64;
    pub const XCB_XKB_AXSK_OPT_LATCH_TO_LOCK : xcb_xkb_axsk_opt_t = 128;
//}
pub struct AxOption {pub base : base::Struct<xcb_xkb_ax_option_t>}
pub type AxOptionIterator = xcb_xkb_ax_option_iterator_t;

pub type DeviceSpecIterator = xcb_xkb_device_spec_iterator_t;


pub type xcb_xkb_led_class_result_t = c_uint;//{
    pub const XCB_XKB_LED_CLASS_RESULT_KBD_FEEDBACK_CLASS : xcb_xkb_led_class_result_t = 0;
    pub const XCB_XKB_LED_CLASS_RESULT_LED_FEEDBACK_CLASS : xcb_xkb_led_class_result_t = 4;
//}

pub type xcb_xkb_led_class_t = c_uint;//{
    pub const XCB_XKB_LED_CLASS_DFLT_XI_CLASS : xcb_xkb_led_class_t = 768;
    pub const XCB_XKB_LED_CLASS_ALL_XI_CLASSES : xcb_xkb_led_class_t = 1280;
//}
pub type LedClassSpec = xcb_xkb_led_class_spec_t;

pub type LedClassSpecIterator = xcb_xkb_led_class_spec_iterator_t;


pub type xcb_xkb_bell_class_result_t = c_uint;//{
    pub const XCB_XKB_BELL_CLASS_RESULT_KBD_FEEDBACK_CLASS : xcb_xkb_bell_class_result_t = 0;
    pub const XCB_XKB_BELL_CLASS_RESULT_BELL_FEEDBACK_CLASS : xcb_xkb_bell_class_result_t = 5;
//}

pub type xcb_xkb_bell_class_t = c_uint;//{
    pub const XCB_XKB_BELL_CLASS_DFLT_XI_CLASS : xcb_xkb_bell_class_t = 768;
//}
pub type BellClassSpec = xcb_xkb_bell_class_spec_t;

pub type BellClassSpecIterator = xcb_xkb_bell_class_spec_iterator_t;


pub type xcb_xkb_id_t = c_uint;//{
    pub const XCB_XKB_ID_USE_CORE_KBD : xcb_xkb_id_t = 256;
    pub const XCB_XKB_ID_USE_CORE_PTR : xcb_xkb_id_t = 512;
    pub const XCB_XKB_ID_DFLT_XI_CLASS : xcb_xkb_id_t = 768;
    pub const XCB_XKB_ID_DFLT_XI_ID : xcb_xkb_id_t = 1024;
    pub const XCB_XKB_ID_ALL_XI_CLASS : xcb_xkb_id_t = 1280;
    pub const XCB_XKB_ID_ALL_XI_ID : xcb_xkb_id_t = 1536;
    pub const XCB_XKB_ID_XI_NONE : xcb_xkb_id_t = 65280;
//}
pub type IdSpec = xcb_xkb_id_spec_t;

pub type IdSpecIterator = xcb_xkb_id_spec_iterator_t;


pub type xcb_xkb_group_t = c_uint;//{
    pub const XCB_XKB_GROUP_1 : xcb_xkb_group_t = 0;
    pub const XCB_XKB_GROUP_2 : xcb_xkb_group_t = 1;
    pub const XCB_XKB_GROUP_3 : xcb_xkb_group_t = 2;
    pub const XCB_XKB_GROUP_4 : xcb_xkb_group_t = 3;
//}

pub type xcb_xkb_groups_t = c_uint;//{
    pub const XCB_XKB_GROUPS_ANY : xcb_xkb_groups_t = 254;
    pub const XCB_XKB_GROUPS_ALL : xcb_xkb_groups_t = 255;
//}

pub type xcb_xkb_set_of_group_t = c_uint;//{
    pub const XCB_XKB_SET_OF_GROUP_GROUP_1 : xcb_xkb_set_of_group_t = 1;
    pub const XCB_XKB_SET_OF_GROUP_GROUP_2 : xcb_xkb_set_of_group_t = 2;
    pub const XCB_XKB_SET_OF_GROUP_GROUP_3 : xcb_xkb_set_of_group_t = 4;
    pub const XCB_XKB_SET_OF_GROUP_GROUP_4 : xcb_xkb_set_of_group_t = 8;
//}

pub type xcb_xkb_set_of_groups_t = c_uint;//{
    pub const XCB_XKB_SET_OF_GROUPS_ANY : xcb_xkb_set_of_groups_t = 128;
//}

pub type xcb_xkb_groups_wrap_t = c_uint;//{
    pub const XCB_XKB_GROUPS_WRAP_WRAP_INTO_RANGE : xcb_xkb_groups_wrap_t = 0;
    pub const XCB_XKB_GROUPS_WRAP_CLAMP_INTO_RANGE : xcb_xkb_groups_wrap_t = 64;
    pub const XCB_XKB_GROUPS_WRAP_REDIRECT_INTO_RANGE : xcb_xkb_groups_wrap_t = 128;
//}

pub type xcb_xkb_v_mods_high_t = c_uint;//{
    pub const XCB_XKB_V_MODS_HIGH_15 : xcb_xkb_v_mods_high_t = 128;
    pub const XCB_XKB_V_MODS_HIGH_14 : xcb_xkb_v_mods_high_t = 64;
    pub const XCB_XKB_V_MODS_HIGH_13 : xcb_xkb_v_mods_high_t = 32;
    pub const XCB_XKB_V_MODS_HIGH_12 : xcb_xkb_v_mods_high_t = 16;
    pub const XCB_XKB_V_MODS_HIGH_11 : xcb_xkb_v_mods_high_t = 8;
    pub const XCB_XKB_V_MODS_HIGH_10 : xcb_xkb_v_mods_high_t = 4;
    pub const XCB_XKB_V_MODS_HIGH_9 : xcb_xkb_v_mods_high_t = 2;
    pub const XCB_XKB_V_MODS_HIGH_8 : xcb_xkb_v_mods_high_t = 1;
//}

pub type xcb_xkb_v_mods_low_t = c_uint;//{
    pub const XCB_XKB_V_MODS_LOW_7 : xcb_xkb_v_mods_low_t = 128;
    pub const XCB_XKB_V_MODS_LOW_6 : xcb_xkb_v_mods_low_t = 64;
    pub const XCB_XKB_V_MODS_LOW_5 : xcb_xkb_v_mods_low_t = 32;
    pub const XCB_XKB_V_MODS_LOW_4 : xcb_xkb_v_mods_low_t = 16;
    pub const XCB_XKB_V_MODS_LOW_3 : xcb_xkb_v_mods_low_t = 8;
    pub const XCB_XKB_V_MODS_LOW_2 : xcb_xkb_v_mods_low_t = 4;
    pub const XCB_XKB_V_MODS_LOW_1 : xcb_xkb_v_mods_low_t = 2;
    pub const XCB_XKB_V_MODS_LOW_0 : xcb_xkb_v_mods_low_t = 1;
//}

pub type xcb_xkb_v_mod_t = c_uint;//{
    pub const XCB_XKB_V_MOD_15 : xcb_xkb_v_mod_t = 32768;
    pub const XCB_XKB_V_MOD_14 : xcb_xkb_v_mod_t = 16384;
    pub const XCB_XKB_V_MOD_13 : xcb_xkb_v_mod_t = 8192;
    pub const XCB_XKB_V_MOD_12 : xcb_xkb_v_mod_t = 4096;
    pub const XCB_XKB_V_MOD_11 : xcb_xkb_v_mod_t = 2048;
    pub const XCB_XKB_V_MOD_10 : xcb_xkb_v_mod_t = 1024;
    pub const XCB_XKB_V_MOD_9 : xcb_xkb_v_mod_t = 512;
    pub const XCB_XKB_V_MOD_8 : xcb_xkb_v_mod_t = 256;
    pub const XCB_XKB_V_MOD_7 : xcb_xkb_v_mod_t = 128;
    pub const XCB_XKB_V_MOD_6 : xcb_xkb_v_mod_t = 64;
    pub const XCB_XKB_V_MOD_5 : xcb_xkb_v_mod_t = 32;
    pub const XCB_XKB_V_MOD_4 : xcb_xkb_v_mod_t = 16;
    pub const XCB_XKB_V_MOD_3 : xcb_xkb_v_mod_t = 8;
    pub const XCB_XKB_V_MOD_2 : xcb_xkb_v_mod_t = 4;
    pub const XCB_XKB_V_MOD_1 : xcb_xkb_v_mod_t = 2;
    pub const XCB_XKB_V_MOD_0 : xcb_xkb_v_mod_t = 1;
//}

pub type xcb_xkb_explicit_t = c_uint;//{
    pub const XCB_XKB_EXPLICIT_V_MOD_MAP : xcb_xkb_explicit_t = 128;
    pub const XCB_XKB_EXPLICIT_BEHAVIOR : xcb_xkb_explicit_t = 64;
    pub const XCB_XKB_EXPLICIT_AUTO_REPEAT : xcb_xkb_explicit_t = 32;
    pub const XCB_XKB_EXPLICIT_INTERPRET : xcb_xkb_explicit_t = 16;
    pub const XCB_XKB_EXPLICIT_KEY_TYPE_4 : xcb_xkb_explicit_t = 8;
    pub const XCB_XKB_EXPLICIT_KEY_TYPE_3 : xcb_xkb_explicit_t = 4;
    pub const XCB_XKB_EXPLICIT_KEY_TYPE_2 : xcb_xkb_explicit_t = 2;
    pub const XCB_XKB_EXPLICIT_KEY_TYPE_1 : xcb_xkb_explicit_t = 1;
//}

pub type xcb_xkb_sym_interpret_t = c_uint;//{
    pub const XCB_XKB_SYM_INTERPRET_NONE_OF : xcb_xkb_sym_interpret_t = 0;
    pub const XCB_XKB_SYM_INTERPRET_ANY_OF_OR_NONE : xcb_xkb_sym_interpret_t = 1;
    pub const XCB_XKB_SYM_INTERPRET_ANY_OF : xcb_xkb_sym_interpret_t = 2;
    pub const XCB_XKB_SYM_INTERPRET_ALL_OF : xcb_xkb_sym_interpret_t = 3;
    pub const XCB_XKB_SYM_INTERPRET_EXACTLY : xcb_xkb_sym_interpret_t = 4;
//}

pub type xcb_xkb_sym_interp_match_t = c_uint;//{
    pub const XCB_XKB_SYM_INTERP_MATCH_LEVEL_ONE_ONLY : xcb_xkb_sym_interp_match_t = 128;
    pub const XCB_XKB_SYM_INTERP_MATCH_OP_MASK : xcb_xkb_sym_interp_match_t = 127;
//}

pub type xcb_xkb_im_flag_t = c_uint;//{
    pub const XCB_XKB_IM_FLAG_NO_EXPLICIT : xcb_xkb_im_flag_t = 128;
    pub const XCB_XKB_IM_FLAG_NO_AUTOMATIC : xcb_xkb_im_flag_t = 64;
    pub const XCB_XKB_IM_FLAG_LED_DRIVES_KB : xcb_xkb_im_flag_t = 32;
//}

pub type xcb_xkb_im_mods_which_t = c_uint;//{
    pub const XCB_XKB_IM_MODS_WHICH_USE_COMPAT : xcb_xkb_im_mods_which_t = 16;
    pub const XCB_XKB_IM_MODS_WHICH_USE_EFFECTIVE : xcb_xkb_im_mods_which_t = 8;
    pub const XCB_XKB_IM_MODS_WHICH_USE_LOCKED : xcb_xkb_im_mods_which_t = 4;
    pub const XCB_XKB_IM_MODS_WHICH_USE_LATCHED : xcb_xkb_im_mods_which_t = 2;
    pub const XCB_XKB_IM_MODS_WHICH_USE_BASE : xcb_xkb_im_mods_which_t = 1;
//}

pub type xcb_xkb_im_groups_which_t = c_uint;//{
    pub const XCB_XKB_IM_GROUPS_WHICH_USE_COMPAT : xcb_xkb_im_groups_which_t = 16;
    pub const XCB_XKB_IM_GROUPS_WHICH_USE_EFFECTIVE : xcb_xkb_im_groups_which_t = 8;
    pub const XCB_XKB_IM_GROUPS_WHICH_USE_LOCKED : xcb_xkb_im_groups_which_t = 4;
    pub const XCB_XKB_IM_GROUPS_WHICH_USE_LATCHED : xcb_xkb_im_groups_which_t = 2;
    pub const XCB_XKB_IM_GROUPS_WHICH_USE_BASE : xcb_xkb_im_groups_which_t = 1;
//}
pub struct IndicatorMap {pub base : base::Struct<xcb_xkb_indicator_map_t> }

pub type IndicatorMapIterator = xcb_xkb_indicator_map_iterator_t;


pub type xcb_xkb_cm_detail_t = c_uint;//{
    pub const XCB_XKB_CM_DETAIL_SYM_INTERP : xcb_xkb_cm_detail_t = 1;
    pub const XCB_XKB_CM_DETAIL_GROUP_COMPAT : xcb_xkb_cm_detail_t = 2;
//}

pub type xcb_xkb_name_detail_t = c_uint;//{
    pub const XCB_XKB_NAME_DETAIL_KEYCODES : xcb_xkb_name_detail_t = 1;
    pub const XCB_XKB_NAME_DETAIL_GEOMETRY : xcb_xkb_name_detail_t = 2;
    pub const XCB_XKB_NAME_DETAIL_SYMBOLS : xcb_xkb_name_detail_t = 4;
    pub const XCB_XKB_NAME_DETAIL_PHYS_SYMBOLS : xcb_xkb_name_detail_t = 8;
    pub const XCB_XKB_NAME_DETAIL_TYPES : xcb_xkb_name_detail_t = 16;
    pub const XCB_XKB_NAME_DETAIL_COMPAT : xcb_xkb_name_detail_t = 32;
    pub const XCB_XKB_NAME_DETAIL_KEY_TYPE_NAMES : xcb_xkb_name_detail_t = 64;
    pub const XCB_XKB_NAME_DETAIL_KT_LEVEL_NAMES : xcb_xkb_name_detail_t = 128;
    pub const XCB_XKB_NAME_DETAIL_INDICATOR_NAMES : xcb_xkb_name_detail_t = 256;
    pub const XCB_XKB_NAME_DETAIL_KEY_NAMES : xcb_xkb_name_detail_t = 512;
    pub const XCB_XKB_NAME_DETAIL_KEY_ALIASES : xcb_xkb_name_detail_t = 1024;
    pub const XCB_XKB_NAME_DETAIL_VIRTUAL_MOD_NAMES : xcb_xkb_name_detail_t = 2048;
    pub const XCB_XKB_NAME_DETAIL_GROUP_NAMES : xcb_xkb_name_detail_t = 4096;
    pub const XCB_XKB_NAME_DETAIL_RG_NAMES : xcb_xkb_name_detail_t = 8192;
//}

pub type xcb_xkb_gbn_detail_t = c_uint;//{
    pub const XCB_XKB_GBN_DETAIL_TYPES : xcb_xkb_gbn_detail_t = 1;
    pub const XCB_XKB_GBN_DETAIL_COMPAT_MAP : xcb_xkb_gbn_detail_t = 2;
    pub const XCB_XKB_GBN_DETAIL_CLIENT_SYMBOLS : xcb_xkb_gbn_detail_t = 4;
    pub const XCB_XKB_GBN_DETAIL_SERVER_SYMBOLS : xcb_xkb_gbn_detail_t = 8;
    pub const XCB_XKB_GBN_DETAIL_INDICATOR_MAPS : xcb_xkb_gbn_detail_t = 16;
    pub const XCB_XKB_GBN_DETAIL_KEY_NAMES : xcb_xkb_gbn_detail_t = 32;
    pub const XCB_XKB_GBN_DETAIL_GEOMETRY : xcb_xkb_gbn_detail_t = 64;
    pub const XCB_XKB_GBN_DETAIL_OTHER_NAMES : xcb_xkb_gbn_detail_t = 128;
//}

pub type xcb_xkb_xi_feature_t = c_uint;//{
    pub const XCB_XKB_XI_FEATURE_KEYBOARDS : xcb_xkb_xi_feature_t = 1;
    pub const XCB_XKB_XI_FEATURE_BUTTON_ACTIONS : xcb_xkb_xi_feature_t = 2;
    pub const XCB_XKB_XI_FEATURE_INDICATOR_NAMES : xcb_xkb_xi_feature_t = 4;
    pub const XCB_XKB_XI_FEATURE_INDICATOR_MAPS : xcb_xkb_xi_feature_t = 8;
    pub const XCB_XKB_XI_FEATURE_INDICATOR_STATE : xcb_xkb_xi_feature_t = 16;
//}

pub type xcb_xkb_per_client_flag_t = c_uint;//{
    pub const XCB_XKB_PER_CLIENT_FLAG_DETECTABLE_AUTO_REPEAT : xcb_xkb_per_client_flag_t = 1;
    pub const XCB_XKB_PER_CLIENT_FLAG_GRABS_USE_XKB_STATE : xcb_xkb_per_client_flag_t = 2;
    pub const XCB_XKB_PER_CLIENT_FLAG_AUTO_RESET_CONTROLS : xcb_xkb_per_client_flag_t = 4;
    pub const XCB_XKB_PER_CLIENT_FLAG_LOOKUP_STATE_WHEN_GRABBED : xcb_xkb_per_client_flag_t = 8;
    pub const XCB_XKB_PER_CLIENT_FLAG_SEND_EVENT_USES_XKB_STATE : xcb_xkb_per_client_flag_t = 16;
//}
pub struct ModDef {pub base : base::Struct<xcb_xkb_mod_def_t> }

pub type ModDefIterator = xcb_xkb_mod_def_iterator_t;

pub type KeyNameIterator = xcb_xkb_key_name_iterator_t;

pub type KeyAliasIterator = xcb_xkb_key_alias_iterator_t;

pub type CountedString8Iterator = xcb_xkb_counted_string_8_iterator_t;

pub type CountedString16Iterator = xcb_xkb_counted_string_16_iterator_t;

pub type KtMapEntryIterator = xcb_xkb_kt_map_entry_iterator_t;

pub type KeyTypeIterator = xcb_xkb_key_type_iterator_t;

pub type KeySymMapIterator = xcb_xkb_key_sym_map_iterator_t;

pub type CommonBehaviorIterator = xcb_xkb_common_behavior_iterator_t;

pub type DefaultBehaviorIterator = xcb_xkb_default_behavior_iterator_t;

pub type LockBehaviorIterator = xcb_xkb_lock_behavior_iterator_t;

pub type RadioGroupBehaviorIterator = xcb_xkb_radio_group_behavior_iterator_t;

pub type Overlay1BehaviorIterator = xcb_xkb_overlay_1_behavior_iterator_t;

pub type Overlay2BehaviorIterator = xcb_xkb_overlay_2_behavior_iterator_t;

pub type PermamentLockBehaviorIterator = xcb_xkb_permament_lock_behavior_iterator_t;

pub type PermamentRadioGroupBehaviorIterator = xcb_xkb_permament_radio_group_behavior_iterator_t;

pub type PermamentOverlay1BehaviorIterator = xcb_xkb_permament_overlay_1_behavior_iterator_t;

pub type PermamentOverlay2BehaviorIterator = xcb_xkb_permament_overlay_2_behavior_iterator_t;

pub type BehaviorIterator = xcb_xkb_behavior_iterator_t;


pub type xcb_xkb_behavior_type_t = c_uint;//{
    pub const XCB_XKB_BEHAVIOR_TYPE_DEFAULT : xcb_xkb_behavior_type_t = 0;
    pub const XCB_XKB_BEHAVIOR_TYPE_LOCK : xcb_xkb_behavior_type_t = 1;
    pub const XCB_XKB_BEHAVIOR_TYPE_RADIO_GROUP : xcb_xkb_behavior_type_t = 2;
    pub const XCB_XKB_BEHAVIOR_TYPE_OVERLAY_1 : xcb_xkb_behavior_type_t = 3;
    pub const XCB_XKB_BEHAVIOR_TYPE_OVERLAY_2 : xcb_xkb_behavior_type_t = 4;
    pub const XCB_XKB_BEHAVIOR_TYPE_PERMAMENT_LOCK : xcb_xkb_behavior_type_t = 129;
    pub const XCB_XKB_BEHAVIOR_TYPE_PERMAMENT_RADIO_GROUP : xcb_xkb_behavior_type_t = 130;
    pub const XCB_XKB_BEHAVIOR_TYPE_PERMAMENT_OVERLAY_1 : xcb_xkb_behavior_type_t = 131;
    pub const XCB_XKB_BEHAVIOR_TYPE_PERMAMENT_OVERLAY_2 : xcb_xkb_behavior_type_t = 132;
//}
pub struct SetBehavior {pub base : base::Struct<xcb_xkb_set_behavior_t> }

pub type SetBehaviorIterator = xcb_xkb_set_behavior_iterator_t;

pub type SetExplicitIterator = xcb_xkb_set_explicit_iterator_t;

pub type KeyModMapIterator = xcb_xkb_key_mod_map_iterator_t;

pub type KeyVModMapIterator = xcb_xkb_key_v_mod_map_iterator_t;

pub type KtSetMapEntryIterator = xcb_xkb_kt_set_map_entry_iterator_t;

pub type SetKeyTypeIterator = xcb_xkb_set_key_type_iterator_t;

pub type String8Iterator = xcb_xkb_string8_iterator_t;

pub type PropertyIterator = xcb_xkb_property_iterator_t;

pub type OutlineIterator = xcb_xkb_outline_iterator_t;

pub type ShapeIterator = xcb_xkb_shape_iterator_t;

pub type KeyIterator = xcb_xkb_key_iterator_t;

pub type OverlayKeyIterator = xcb_xkb_overlay_key_iterator_t;

pub type OverlayRowIterator = xcb_xkb_overlay_row_iterator_t;

pub type OverlayIterator = xcb_xkb_overlay_iterator_t;

pub type RowIterator = xcb_xkb_row_iterator_t;


pub type xcb_xkb_doodad_type_t = c_uint;//{
    pub const XCB_XKB_DOODAD_TYPE_OUTLINE : xcb_xkb_doodad_type_t = 1;
    pub const XCB_XKB_DOODAD_TYPE_SOLID : xcb_xkb_doodad_type_t = 2;
    pub const XCB_XKB_DOODAD_TYPE_TEXT : xcb_xkb_doodad_type_t = 3;
    pub const XCB_XKB_DOODAD_TYPE_INDICATOR : xcb_xkb_doodad_type_t = 4;
    pub const XCB_XKB_DOODAD_TYPE_LOGO : xcb_xkb_doodad_type_t = 5;
//}
pub struct CommonDoodad {pub base : base::Struct<xcb_xkb_common_doodad_t> }

pub type CommonDoodadIterator = xcb_xkb_common_doodad_iterator_t;

pub type ShapeDoodadIterator = xcb_xkb_shape_doodad_iterator_t;

pub type TextDoodadIterator = xcb_xkb_text_doodad_iterator_t;

pub type IndicatorDoodadIterator = xcb_xkb_indicator_doodad_iterator_t;

pub type LogoDoodadIterator = xcb_xkb_logo_doodad_iterator_t;

pub type DoodadIterator = xcb_xkb_doodad_iterator_t;

pub type SectionIterator = xcb_xkb_section_iterator_t;

pub type ListingIterator = xcb_xkb_listing_iterator_t;

pub type DeviceLedInfoIterator = xcb_xkb_device_led_info_iterator_t;


pub type xcb_xkb_error_t = c_uint;//{
    pub const XCB_XKB_ERROR_BAD_DEVICE : xcb_xkb_error_t = 255;
    pub const XCB_XKB_ERROR_BAD_CLASS : xcb_xkb_error_t = 254;
    pub const XCB_XKB_ERROR_BAD_ID : xcb_xkb_error_t = 253;
//}
/// Opcode for xcb_xkb_keyboard.
pub const XCB_XKB_KEYBOARD : u8 = 0;
pub struct KeyboardError { pub base : base::Error<xcb_xkb_keyboard_error_t> }

pub type xcb_xkb_sa_t = c_uint;//{
    pub const XCB_XKB_SA_CLEAR_LOCKS : xcb_xkb_sa_t = 1;
    pub const XCB_XKB_SA_LATCH_TO_LOCK : xcb_xkb_sa_t = 2;
    pub const XCB_XKB_SA_USE_MOD_MAP_MODS : xcb_xkb_sa_t = 4;
    pub const XCB_XKB_SA_GROUP_ABSOLUTE : xcb_xkb_sa_t = 4;
//}

pub type xcb_xkb_sa_type_t = c_uint;//{
    pub const XCB_XKB_SA_TYPE_NO_ACTION : xcb_xkb_sa_type_t = 0;
    pub const XCB_XKB_SA_TYPE_SET_MODS : xcb_xkb_sa_type_t = 1;
    pub const XCB_XKB_SA_TYPE_LATCH_MODS : xcb_xkb_sa_type_t = 2;
    pub const XCB_XKB_SA_TYPE_LOCK_MODS : xcb_xkb_sa_type_t = 3;
    pub const XCB_XKB_SA_TYPE_SET_GROUP : xcb_xkb_sa_type_t = 4;
    pub const XCB_XKB_SA_TYPE_LATCH_GROUP : xcb_xkb_sa_type_t = 5;
    pub const XCB_XKB_SA_TYPE_LOCK_GROUP : xcb_xkb_sa_type_t = 6;
    pub const XCB_XKB_SA_TYPE_MOVE_PTR : xcb_xkb_sa_type_t = 7;
    pub const XCB_XKB_SA_TYPE_PTR_BTN : xcb_xkb_sa_type_t = 8;
    pub const XCB_XKB_SA_TYPE_LOCK_PTR_BTN : xcb_xkb_sa_type_t = 9;
    pub const XCB_XKB_SA_TYPE_SET_PTR_DFLT : xcb_xkb_sa_type_t = 10;
    pub const XCB_XKB_SA_TYPE_ISO_LOCK : xcb_xkb_sa_type_t = 11;
    pub const XCB_XKB_SA_TYPE_TERMINATE : xcb_xkb_sa_type_t = 12;
    pub const XCB_XKB_SA_TYPE_SWITCH_SCREEN : xcb_xkb_sa_type_t = 13;
    pub const XCB_XKB_SA_TYPE_SET_CONTROLS : xcb_xkb_sa_type_t = 14;
    pub const XCB_XKB_SA_TYPE_LOCK_CONTROLS : xcb_xkb_sa_type_t = 15;
    pub const XCB_XKB_SA_TYPE_ACTION_MESSAGE : xcb_xkb_sa_type_t = 16;
    pub const XCB_XKB_SA_TYPE_REDIRECT_KEY : xcb_xkb_sa_type_t = 17;
    pub const XCB_XKB_SA_TYPE_DEVICE_BTN : xcb_xkb_sa_type_t = 18;
    pub const XCB_XKB_SA_TYPE_LOCK_DEVICE_BTN : xcb_xkb_sa_type_t = 19;
    pub const XCB_XKB_SA_TYPE_DEVICE_VALUATOR : xcb_xkb_sa_type_t = 20;
//}
pub struct SaNoAction {pub base : base::Struct<xcb_xkb_sa_no_action_t> }

pub type SaNoActionIterator = xcb_xkb_sa_no_action_iterator_t;

pub type SaSetModsIterator = xcb_xkb_sa_set_mods_iterator_t;

pub type SaLatchModsIterator = xcb_xkb_sa_latch_mods_iterator_t;

pub type SaLockModsIterator = xcb_xkb_sa_lock_mods_iterator_t;

pub type SaSetGroupIterator = xcb_xkb_sa_set_group_iterator_t;

pub type SaLatchGroupIterator = xcb_xkb_sa_latch_group_iterator_t;

pub type SaLockGroupIterator = xcb_xkb_sa_lock_group_iterator_t;


pub type xcb_xkb_sa_move_ptr_flag_t = c_uint;//{
    pub const XCB_XKB_SA_MOVE_PTR_FLAG_NO_ACCELERATION : xcb_xkb_sa_move_ptr_flag_t = 1;
    pub const XCB_XKB_SA_MOVE_PTR_FLAG_MOVE_ABSOLUTE_X : xcb_xkb_sa_move_ptr_flag_t = 2;
    pub const XCB_XKB_SA_MOVE_PTR_FLAG_MOVE_ABSOLUTE_Y : xcb_xkb_sa_move_ptr_flag_t = 4;
//}
pub struct SaMovePtr {pub base : base::Struct<xcb_xkb_sa_move_ptr_t> }

pub type SaMovePtrIterator = xcb_xkb_sa_move_ptr_iterator_t;

pub type SaPtrBtnIterator = xcb_xkb_sa_ptr_btn_iterator_t;

pub type SaLockPtrBtnIterator = xcb_xkb_sa_lock_ptr_btn_iterator_t;


pub type xcb_xkb_sa_set_ptr_dflt_flag_t = c_uint;//{
    pub const XCB_XKB_SA_SET_PTR_DFLT_FLAG_DFLT_BTN_ABSOLUTE : xcb_xkb_sa_set_ptr_dflt_flag_t = 2;
    pub const XCB_XKB_SA_SET_PTR_DFLT_FLAG_AFFECT_DFLT_BUTTON : xcb_xkb_sa_set_ptr_dflt_flag_t = 1;
//}
pub struct SaSetPtrDflt {pub base : base::Struct<xcb_xkb_sa_set_ptr_dflt_t> }

pub type SaSetPtrDfltIterator = xcb_xkb_sa_set_ptr_dflt_iterator_t;


pub type xcb_xkb_sa_iso_lock_flag_t = c_uint;//{
    pub const XCB_XKB_SA_ISO_LOCK_FLAG_NO_LOCK : xcb_xkb_sa_iso_lock_flag_t = 1;
    pub const XCB_XKB_SA_ISO_LOCK_FLAG_NO_UNLOCK : xcb_xkb_sa_iso_lock_flag_t = 2;
    pub const XCB_XKB_SA_ISO_LOCK_FLAG_USE_MOD_MAP_MODS : xcb_xkb_sa_iso_lock_flag_t = 4;
    pub const XCB_XKB_SA_ISO_LOCK_FLAG_GROUP_ABSOLUTE : xcb_xkb_sa_iso_lock_flag_t = 4;
    pub const XCB_XKB_SA_ISO_LOCK_FLAG_ISO_DFLT_IS_GROUP : xcb_xkb_sa_iso_lock_flag_t = 8;
//}

pub type xcb_xkb_sa_iso_lock_no_affect_t = c_uint;//{
    pub const XCB_XKB_SA_ISO_LOCK_NO_AFFECT_CTRLS : xcb_xkb_sa_iso_lock_no_affect_t = 8;
    pub const XCB_XKB_SA_ISO_LOCK_NO_AFFECT_PTR : xcb_xkb_sa_iso_lock_no_affect_t = 16;
    pub const XCB_XKB_SA_ISO_LOCK_NO_AFFECT_GROUP : xcb_xkb_sa_iso_lock_no_affect_t = 32;
    pub const XCB_XKB_SA_ISO_LOCK_NO_AFFECT_MODS : xcb_xkb_sa_iso_lock_no_affect_t = 64;
//}
pub struct SaIsoLock {pub base : base::Struct<xcb_xkb_sa_iso_lock_t> }

pub type SaIsoLockIterator = xcb_xkb_sa_iso_lock_iterator_t;

pub type SaTerminateIterator = xcb_xkb_sa_terminate_iterator_t;


pub type xcb_xkb_switch_screen_flag_t = c_uint;//{
    pub const XCB_XKB_SWITCH_SCREEN_FLAG_APPLICATION : xcb_xkb_switch_screen_flag_t = 1;
    pub const XCB_XKB_SWITCH_SCREEN_FLAG_ABSOLUTE : xcb_xkb_switch_screen_flag_t = 4;
//}
pub struct SaSwitchScreen {pub base : base::Struct<xcb_xkb_sa_switch_screen_t> }

pub type SaSwitchScreenIterator = xcb_xkb_sa_switch_screen_iterator_t;


pub type xcb_xkb_bool_ctrls_high_t = c_uint;//{
    pub const XCB_XKB_BOOL_CTRLS_HIGH_ACCESS_X_FEEDBACK : xcb_xkb_bool_ctrls_high_t = 1;
    pub const XCB_XKB_BOOL_CTRLS_HIGH_AUDIBLE_BELL : xcb_xkb_bool_ctrls_high_t = 2;
    pub const XCB_XKB_BOOL_CTRLS_HIGH_OVERLAY_1 : xcb_xkb_bool_ctrls_high_t = 4;
    pub const XCB_XKB_BOOL_CTRLS_HIGH_OVERLAY_2 : xcb_xkb_bool_ctrls_high_t = 8;
    pub const XCB_XKB_BOOL_CTRLS_HIGH_IGNORE_GROUP_LOCK : xcb_xkb_bool_ctrls_high_t = 16;
//}

pub type xcb_xkb_bool_ctrls_low_t = c_uint;//{
    pub const XCB_XKB_BOOL_CTRLS_LOW_REPEAT_KEYS : xcb_xkb_bool_ctrls_low_t = 1;
    pub const XCB_XKB_BOOL_CTRLS_LOW_SLOW_KEYS : xcb_xkb_bool_ctrls_low_t = 2;
    pub const XCB_XKB_BOOL_CTRLS_LOW_BOUNCE_KEYS : xcb_xkb_bool_ctrls_low_t = 4;
    pub const XCB_XKB_BOOL_CTRLS_LOW_STICKY_KEYS : xcb_xkb_bool_ctrls_low_t = 8;
    pub const XCB_XKB_BOOL_CTRLS_LOW_MOUSE_KEYS : xcb_xkb_bool_ctrls_low_t = 16;
    pub const XCB_XKB_BOOL_CTRLS_LOW_MOUSE_KEYS_ACCEL : xcb_xkb_bool_ctrls_low_t = 32;
    pub const XCB_XKB_BOOL_CTRLS_LOW_ACCESS_X_KEYS : xcb_xkb_bool_ctrls_low_t = 64;
    pub const XCB_XKB_BOOL_CTRLS_LOW_ACCESS_X_TIMEOUT : xcb_xkb_bool_ctrls_low_t = 128;
//}
pub struct SaSetControls {pub base : base::Struct<xcb_xkb_sa_set_controls_t> }

pub type SaSetControlsIterator = xcb_xkb_sa_set_controls_iterator_t;

pub type SaLockControlsIterator = xcb_xkb_sa_lock_controls_iterator_t;


pub type xcb_xkb_action_message_flag_t = c_uint;//{
    pub const XCB_XKB_ACTION_MESSAGE_FLAG_ON_PRESS : xcb_xkb_action_message_flag_t = 1;
    pub const XCB_XKB_ACTION_MESSAGE_FLAG_ON_RELEASE : xcb_xkb_action_message_flag_t = 2;
    pub const XCB_XKB_ACTION_MESSAGE_FLAG_GEN_KEY_EVENT : xcb_xkb_action_message_flag_t = 4;
//}
pub struct SaActionMessage {pub base : base::Struct<xcb_xkb_sa_action_message_t> }

pub type SaActionMessageIterator = xcb_xkb_sa_action_message_iterator_t;

pub type SaRedirectKeyIterator = xcb_xkb_sa_redirect_key_iterator_t;

pub type SaDeviceBtnIterator = xcb_xkb_sa_device_btn_iterator_t;


pub type xcb_xkb_lock_device_flags_t = c_uint;//{
    pub const XCB_XKB_LOCK_DEVICE_FLAGS_NO_LOCK : xcb_xkb_lock_device_flags_t = 1;
    pub const XCB_XKB_LOCK_DEVICE_FLAGS_NO_UNLOCK : xcb_xkb_lock_device_flags_t = 2;
//}
pub struct SaLockDeviceBtn {pub base : base::Struct<xcb_xkb_sa_lock_device_btn_t> }

pub type SaLockDeviceBtnIterator = xcb_xkb_sa_lock_device_btn_iterator_t;


pub type xcb_xkb_sa_val_what_t = c_uint;//{
    pub const XCB_XKB_SA_VAL_WHAT_IGNORE_VAL : xcb_xkb_sa_val_what_t = 0;
    pub const XCB_XKB_SA_VAL_WHAT_SET_VAL_MIN : xcb_xkb_sa_val_what_t = 1;
    pub const XCB_XKB_SA_VAL_WHAT_SET_VAL_CENTER : xcb_xkb_sa_val_what_t = 2;
    pub const XCB_XKB_SA_VAL_WHAT_SET_VAL_MAX : xcb_xkb_sa_val_what_t = 3;
    pub const XCB_XKB_SA_VAL_WHAT_SET_VAL_RELATIVE : xcb_xkb_sa_val_what_t = 4;
    pub const XCB_XKB_SA_VAL_WHAT_SET_VAL_ABSOLUTE : xcb_xkb_sa_val_what_t = 5;
//}
pub struct SaDeviceValuator {pub base : base::Struct<xcb_xkb_sa_device_valuator_t> }

pub type SaDeviceValuatorIterator = xcb_xkb_sa_device_valuator_iterator_t;

pub type ActionIterator = xcb_xkb_action_iterator_t;

pub struct  UseExtensionCookie<'s> { pub base : base::Cookie<'s, xcb_xkb_use_extension_cookie_t> }

/// Opcode for xcb_xkb_use_extension.
pub const XCB_XKB_USE_EXTENSION : u8 = 0;
pub struct UseExtensionReply { base:  base::Reply<xcb_xkb_use_extension_reply_t> }
fn mk_reply_xcb_xkb_use_extension_reply_t(reply:*mut xcb_xkb_use_extension_reply_t) -> UseExtensionReply { UseExtensionReply { base : base::mk_reply(reply) } }
/// Opcode for xcb_xkb_select_events.
pub const XCB_XKB_SELECT_EVENTS : u8 = 1;
/// Opcode for xcb_xkb_bell.
pub const XCB_XKB_BELL : u8 = 3;
pub struct  GetStateCookie<'s> { pub base : base::Cookie<'s, xcb_xkb_get_state_cookie_t> }

/// Opcode for xcb_xkb_get_state.
pub const XCB_XKB_GET_STATE : u8 = 4;
pub struct GetStateReply { base:  base::Reply<xcb_xkb_get_state_reply_t> }
fn mk_reply_xcb_xkb_get_state_reply_t(reply:*mut xcb_xkb_get_state_reply_t) -> GetStateReply { GetStateReply { base : base::mk_reply(reply) } }
/// Opcode for xcb_xkb_latch_lock_state.
pub const XCB_XKB_LATCH_LOCK_STATE : u8 = 5;
pub struct  GetControlsCookie<'s> { pub base : base::Cookie<'s, xcb_xkb_get_controls_cookie_t> }

/// Opcode for xcb_xkb_get_controls.
pub const XCB_XKB_GET_CONTROLS : u8 = 6;
pub struct GetControlsReply { base:  base::Reply<xcb_xkb_get_controls_reply_t> }
fn mk_reply_xcb_xkb_get_controls_reply_t(reply:*mut xcb_xkb_get_controls_reply_t) -> GetControlsReply { GetControlsReply { base : base::mk_reply(reply) } }
/// Opcode for xcb_xkb_set_controls.
pub const XCB_XKB_SET_CONTROLS : u8 = 7;
pub struct  GetMapCookie<'s> { pub base : base::Cookie<'s, xcb_xkb_get_map_cookie_t> }

/// Opcode for xcb_xkb_get_map.
pub const XCB_XKB_GET_MAP : u8 = 8;
pub struct GetMapMap {pub base : base::Struct<xcb_xkb_get_map_map_t> }

/// Opcode for xcb_xkb_set_map.
pub const XCB_XKB_SET_MAP : u8 = 9;
pub struct  GetCompatMapCookie<'s> { pub base : base::Cookie<'s, xcb_xkb_get_compat_map_cookie_t> }

/// Opcode for xcb_xkb_get_compat_map.
pub const XCB_XKB_GET_COMPAT_MAP : u8 = 10;
/// Opcode for xcb_xkb_set_compat_map.
pub const XCB_XKB_SET_COMPAT_MAP : u8 = 11;
pub struct  GetIndicatorStateCookie<'s> { pub base : base::Cookie<'s, xcb_xkb_get_indicator_state_cookie_t> }

/// Opcode for xcb_xkb_get_indicator_state.
pub const XCB_XKB_GET_INDICATOR_STATE : u8 = 12;
pub struct GetIndicatorStateReply { base:  base::Reply<xcb_xkb_get_indicator_state_reply_t> }
fn mk_reply_xcb_xkb_get_indicator_state_reply_t(reply:*mut xcb_xkb_get_indicator_state_reply_t) -> GetIndicatorStateReply { GetIndicatorStateReply { base : base::mk_reply(reply) } }
pub struct  GetIndicatorMapCookie<'s> { pub base : base::Cookie<'s, xcb_xkb_get_indicator_map_cookie_t> }

/// Opcode for xcb_xkb_get_indicator_map.
pub const XCB_XKB_GET_INDICATOR_MAP : u8 = 13;
/// Opcode for xcb_xkb_set_indicator_map.
pub const XCB_XKB_SET_INDICATOR_MAP : u8 = 14;
pub struct  GetNamedIndicatorCookie<'s> { pub base : base::Cookie<'s, xcb_xkb_get_named_indicator_cookie_t> }

/// Opcode for xcb_xkb_get_named_indicator.
pub const XCB_XKB_GET_NAMED_INDICATOR : u8 = 15;
pub struct GetNamedIndicatorReply { base:  base::Reply<xcb_xkb_get_named_indicator_reply_t> }
fn mk_reply_xcb_xkb_get_named_indicator_reply_t(reply:*mut xcb_xkb_get_named_indicator_reply_t) -> GetNamedIndicatorReply { GetNamedIndicatorReply { base : base::mk_reply(reply) } }
/// Opcode for xcb_xkb_set_named_indicator.
pub const XCB_XKB_SET_NAMED_INDICATOR : u8 = 16;
pub struct  GetNamesCookie<'s> { pub base : base::Cookie<'s, xcb_xkb_get_names_cookie_t> }

/// Opcode for xcb_xkb_get_names.
pub const XCB_XKB_GET_NAMES : u8 = 17;
pub struct GetNamesValueList {pub base : base::Struct<xcb_xkb_get_names_value_list_t> }

/// Opcode for xcb_xkb_set_names.
pub const XCB_XKB_SET_NAMES : u8 = 18;
pub struct  GetGeometryCookie<'s> { pub base : base::Cookie<'s, xcb_xkb_get_geometry_cookie_t> }

/// Opcode for xcb_xkb_get_geometry.
pub const XCB_XKB_GET_GEOMETRY : u8 = 19;
/// Opcode for xcb_xkb_set_geometry.
pub const XCB_XKB_SET_GEOMETRY : u8 = 20;
pub struct  PerClientFlagsCookie<'s> { pub base : base::Cookie<'s, xcb_xkb_per_client_flags_cookie_t> }

/// Opcode for xcb_xkb_per_client_flags.
pub const XCB_XKB_PER_CLIENT_FLAGS : u8 = 21;
pub struct PerClientFlagsReply { base:  base::Reply<xcb_xkb_per_client_flags_reply_t> }
fn mk_reply_xcb_xkb_per_client_flags_reply_t(reply:*mut xcb_xkb_per_client_flags_reply_t) -> PerClientFlagsReply { PerClientFlagsReply { base : base::mk_reply(reply) } }
pub struct  ListComponentsCookie<'s> { pub base : base::Cookie<'s, xcb_xkb_list_components_cookie_t> }

/// Opcode for xcb_xkb_list_components.
pub const XCB_XKB_LIST_COMPONENTS : u8 = 22;
pub struct ListComponentsReply { base:  base::Reply<xcb_xkb_list_components_reply_t> }
fn mk_reply_xcb_xkb_list_components_reply_t(reply:*mut xcb_xkb_list_components_reply_t) -> ListComponentsReply { ListComponentsReply { base : base::mk_reply(reply) } }
pub struct  GetKbdByNameCookie<'s> { pub base : base::Cookie<'s, xcb_xkb_get_kbd_by_name_cookie_t> }

/// Opcode for xcb_xkb_get_kbd_by_name.
pub const XCB_XKB_GET_KBD_BY_NAME : u8 = 23;
pub struct GetKbdByNameRepliesTypesMap {pub base : base::Struct<xcb_xkb_get_kbd_by_name_replies_types_map_t> }

pub struct  GetDeviceInfoCookie<'s> { pub base : base::Cookie<'s, xcb_xkb_get_device_info_cookie_t> }

/// Opcode for xcb_xkb_get_device_info.
pub const XCB_XKB_GET_DEVICE_INFO : u8 = 24;
/// Opcode for xcb_xkb_set_device_info.
pub const XCB_XKB_SET_DEVICE_INFO : u8 = 25;
pub struct  SetDebuggingFlagsCookie<'s> { pub base : base::Cookie<'s, xcb_xkb_set_debugging_flags_cookie_t> }

/// Opcode for xcb_xkb_set_debugging_flags.
pub const XCB_XKB_SET_DEBUGGING_FLAGS : u8 = 101;
pub struct SetDebuggingFlagsReply { base:  base::Reply<xcb_xkb_set_debugging_flags_reply_t> }
fn mk_reply_xcb_xkb_set_debugging_flags_reply_t(reply:*mut xcb_xkb_set_debugging_flags_reply_t) -> SetDebuggingFlagsReply { SetDebuggingFlagsReply { base : base::mk_reply(reply) } }
/// Opcode for xcb_xkb_new_keyboard_notify.
pub const XCB_XKB_NEW_KEYBOARD_NOTIFY : u8 = 0;
pub struct NewKeyboardNotifyEvent {pub base : base::Event<xcb_xkb_new_keyboard_notify_event_t>}
/// Opcode for xcb_xkb_map_notify.
pub const XCB_XKB_MAP_NOTIFY : u8 = 1;
pub struct MapNotifyEvent {pub base : base::Event<xcb_xkb_map_notify_event_t>}
/// Opcode for xcb_xkb_state_notify.
pub const XCB_XKB_STATE_NOTIFY : u8 = 2;
pub struct StateNotifyEvent {pub base : base::Event<xcb_xkb_state_notify_event_t>}
/// Opcode for xcb_xkb_controls_notify.
pub const XCB_XKB_CONTROLS_NOTIFY : u8 = 3;
pub struct ControlsNotifyEvent {pub base : base::Event<xcb_xkb_controls_notify_event_t>}
/// Opcode for xcb_xkb_indicator_state_notify.
pub const XCB_XKB_INDICATOR_STATE_NOTIFY : u8 = 4;
pub struct IndicatorStateNotifyEvent {pub base : base::Event<xcb_xkb_indicator_state_notify_event_t>}
/// Opcode for xcb_xkb_indicator_map_notify.
pub const XCB_XKB_INDICATOR_MAP_NOTIFY : u8 = 5;
pub struct IndicatorMapNotifyEvent {pub base : base::Event<xcb_xkb_indicator_map_notify_event_t>}
/// Opcode for xcb_xkb_names_notify.
pub const XCB_XKB_NAMES_NOTIFY : u8 = 6;
pub struct NamesNotifyEvent {pub base : base::Event<xcb_xkb_names_notify_event_t>}
/// Opcode for xcb_xkb_compat_map_notify.
pub const XCB_XKB_COMPAT_MAP_NOTIFY : u8 = 7;
pub struct CompatMapNotifyEvent {pub base : base::Event<xcb_xkb_compat_map_notify_event_t>}
/// Opcode for xcb_xkb_bell_notify.
pub const XCB_XKB_BELL_NOTIFY : u8 = 8;
pub struct BellNotifyEvent {pub base : base::Event<xcb_xkb_bell_notify_event_t>}
/// Opcode for xcb_xkb_action_message.
pub const XCB_XKB_ACTION_MESSAGE : u8 = 9;
pub struct ActionMessageEvent {pub base : base::Event<xcb_xkb_action_message_event_t>}
/// Opcode for xcb_xkb_access_x_notify.
pub const XCB_XKB_ACCESS_X_NOTIFY : u8 = 10;
pub struct AccessXNotifyEvent {pub base : base::Event<xcb_xkb_access_x_notify_event_t>}
/// Opcode for xcb_xkb_extension_device_notify.
pub const XCB_XKB_EXTENSION_DEVICE_NOTIFY : u8 = 11;
pub struct ExtensionDeviceNotifyEvent {pub base : base::Event<xcb_xkb_extension_device_notify_event_t>}

impl Iterator for AxOptionIterator {
    type Item = AxOption;
    fn next(&mut self) -> Option<AxOption> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xkb_ax_option_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xkb_ax_option_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub type DeviceSpec = xcb_xkb_device_spec_t;


impl Iterator for DeviceSpecIterator {
    type Item = DeviceSpec;
    fn next(&mut self) -> Option<DeviceSpec> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xkb_device_spec_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xkb_device_spec_next(iter);
            Some(mem::transmute(*data))
        }
    }
}


impl Iterator for LedClassSpecIterator {
    type Item = LedClassSpec;
    fn next(&mut self) -> Option<LedClassSpec> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xkb_led_class_spec_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xkb_led_class_spec_next(iter);
            Some(mem::transmute(*data))
        }
    }
}


impl Iterator for BellClassSpecIterator {
    type Item = BellClassSpec;
    fn next(&mut self) -> Option<BellClassSpec> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xkb_bell_class_spec_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xkb_bell_class_spec_next(iter);
            Some(mem::transmute(*data))
        }
    }
}


impl Iterator for IdSpecIterator {
    type Item = IdSpec;
    fn next(&mut self) -> Option<IdSpec> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xkb_id_spec_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xkb_id_spec_next(iter);
            Some(mem::transmute(*data))
        }
    }
}


impl IndicatorMap {
  pub fn flags(&mut self) -> u8 {
    unsafe { accessor!(flags -> u8, self.base.strct) }
  }

  pub fn whichGroups(&mut self) -> u8 {
    unsafe { accessor!(whichGroups -> u8, self.base.strct) }
  }

  pub fn groups(&mut self) -> u8 {
    unsafe { accessor!(groups -> u8, self.base.strct) }
  }

  pub fn whichMods(&mut self) -> u8 {
    unsafe { accessor!(whichMods -> u8, self.base.strct) }
  }

  pub fn mods(&mut self) -> u8 {
    unsafe { accessor!(mods -> u8, self.base.strct) }
  }

  pub fn realMods(&mut self) -> u8 {
    unsafe { accessor!(realMods -> u8, self.base.strct) }
  }

  pub fn vmods(&mut self) -> u16 {
    unsafe { accessor!(vmods -> u16, self.base.strct) }
  }

  pub fn ctrls(&mut self) -> u32 {
    unsafe { accessor!(ctrls -> u32, self.base.strct) }
  }

}

impl Iterator for IndicatorMapIterator {
    type Item = IndicatorMap;
    fn next(&mut self) -> Option<IndicatorMap> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xkb_indicator_map_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xkb_indicator_map_next(iter);
            Some(mem::transmute(*data))
        }
    }
}


impl ModDef {
  pub fn mask(&mut self) -> u8 {
    unsafe { accessor!(mask -> u8, self.base.strct) }
  }

  pub fn realMods(&mut self) -> u8 {
    unsafe { accessor!(realMods -> u8, self.base.strct) }
  }

  pub fn vmods(&mut self) -> u16 {
    unsafe { accessor!(vmods -> u16, self.base.strct) }
  }

}

impl Iterator for ModDefIterator {
    type Item = ModDef;
    fn next(&mut self) -> Option<ModDef> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xkb_mod_def_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xkb_mod_def_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct KeyName {pub base : base::Struct<xcb_xkb_key_name_t> }


impl KeyName {
  pub fn name(&self) -> Vec<u8> {
    unsafe { (self.base.strct.name).to_vec() }
  }

}

impl Iterator for KeyNameIterator {
    type Item = KeyName;
    fn next(&mut self) -> Option<KeyName> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xkb_key_name_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xkb_key_name_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct KeyAlias {pub base : base::Struct<xcb_xkb_key_alias_t> }


impl KeyAlias {
  pub fn real(&self) -> Vec<u8> {
    unsafe { (self.base.strct.real).to_vec() }
  }

  pub fn alias(&self) -> Vec<u8> {
    unsafe { (self.base.strct.alias).to_vec() }
  }

}

impl Iterator for KeyAliasIterator {
    type Item = KeyAlias;
    fn next(&mut self) -> Option<KeyAlias> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xkb_key_alias_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xkb_key_alias_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct CountedString8 {pub base : base::Struct<xcb_xkb_counted_string_8_t> }


impl CountedString8 {
  pub fn string(&mut self) -> Vec<u8> {
    unsafe { accessor!(u8, xcb_xkb_counted_string_8_string_length, xcb_xkb_counted_string_8_string, self.base.strct) }
  }

}

impl Iterator for CountedString8Iterator {
    type Item = CountedString8;
    fn next(&mut self) -> Option<CountedString8> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xkb_counted_string_8_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xkb_counted_string_8_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct CountedString16 {pub base : base::Struct<xcb_xkb_counted_string_16_t> }


impl CountedString16 {
  pub fn string(&mut self) -> Vec<u8> {
    unsafe { accessor!(u8, xcb_xkb_counted_string_16_string_length, xcb_xkb_counted_string_16_string, self.base.strct) }
  }

}

impl Iterator for CountedString16Iterator {
    type Item = CountedString16;
    fn next(&mut self) -> Option<CountedString16> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xkb_counted_string_16_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xkb_counted_string_16_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct KtMapEntry {pub base : base::Struct<xcb_xkb_kt_map_entry_t> }


impl KtMapEntry {
  pub fn active(&mut self) -> u8 {
    unsafe { accessor!(active -> u8, self.base.strct) }
  }

  pub fn level(&mut self) -> u8 {
    unsafe { accessor!(level -> u8, self.base.strct) }
  }

  pub fn mods_mask(&mut self) -> u8 {
    unsafe { accessor!(mods_mask -> u8, self.base.strct) }
  }

  pub fn mods_mods(&mut self) -> u8 {
    unsafe { accessor!(mods_mods -> u8, self.base.strct) }
  }

  pub fn mods_vmods(&mut self) -> u16 {
    unsafe { accessor!(mods_vmods -> u16, self.base.strct) }
  }

}

impl Iterator for KtMapEntryIterator {
    type Item = KtMapEntry;
    fn next(&mut self) -> Option<KtMapEntry> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xkb_kt_map_entry_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xkb_kt_map_entry_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct KeyType {pub base : base::Struct<xcb_xkb_key_type_t> }


impl KeyType {
  pub fn mods_mask(&mut self) -> u8 {
    unsafe { accessor!(mods_mask -> u8, self.base.strct) }
  }

  pub fn mods_mods(&mut self) -> u8 {
    unsafe { accessor!(mods_mods -> u8, self.base.strct) }
  }

  pub fn mods_vmods(&mut self) -> u16 {
    unsafe { accessor!(mods_vmods -> u16, self.base.strct) }
  }

  pub fn numLevels(&mut self) -> u8 {
    unsafe { accessor!(numLevels -> u8, self.base.strct) }
  }

  pub fn map(&mut self) -> KtMapEntryIterator {
    unsafe { accessor!(KtMapEntryIterator, xcb_xkb_key_type_map_iterator, self.base.strct) }
  }

  pub fn preserve(&mut self) -> ModDefIterator {
    unsafe { accessor!(ModDefIterator, xcb_xkb_key_type_preserve_iterator, self.base.strct) }
  }

}

impl Iterator for KeyTypeIterator {
    type Item = KeyType;
    fn next(&mut self) -> Option<KeyType> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xkb_key_type_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xkb_key_type_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct KeySymMap {pub base : base::Struct<xcb_xkb_key_sym_map_t> }


impl KeySymMap {
  pub fn kt_index(&self) -> Vec<u8> {
    unsafe { (self.base.strct.kt_index).to_vec() }
  }

  pub fn groupInfo(&mut self) -> u8 {
    unsafe { accessor!(groupInfo -> u8, self.base.strct) }
  }

  pub fn width(&mut self) -> u8 {
    unsafe { accessor!(width -> u8, self.base.strct) }
  }

  pub fn syms(&mut self) -> Vec<xproto::Keysym> {
    unsafe { accessor!(xproto::Keysym, xcb_xkb_key_sym_map_syms_length, xcb_xkb_key_sym_map_syms, self.base.strct) }
  }

}

impl Iterator for KeySymMapIterator {
    type Item = KeySymMap;
    fn next(&mut self) -> Option<KeySymMap> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xkb_key_sym_map_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xkb_key_sym_map_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct CommonBehavior {pub base : base::Struct<xcb_xkb_common_behavior_t> }


impl CommonBehavior {
  pub fn type_(&mut self) -> u8 {
    unsafe { accessor!(type_ -> u8, self.base.strct) }
  }

  pub fn data(&mut self) -> u8 {
    unsafe { accessor!(data -> u8, self.base.strct) }
  }

}

impl Iterator for CommonBehaviorIterator {
    type Item = CommonBehavior;
    fn next(&mut self) -> Option<CommonBehavior> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xkb_common_behavior_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xkb_common_behavior_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct DefaultBehavior {pub base : base::Struct<xcb_xkb_default_behavior_t> }


impl DefaultBehavior {
  pub fn type_(&mut self) -> u8 {
    unsafe { accessor!(type_ -> u8, self.base.strct) }
  }

}

impl Iterator for DefaultBehaviorIterator {
    type Item = DefaultBehavior;
    fn next(&mut self) -> Option<DefaultBehavior> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xkb_default_behavior_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xkb_default_behavior_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct LockBehavior {pub base : base::Struct<xcb_xkb_lock_behavior_t> }


impl LockBehavior {
  pub fn type_(&mut self) -> u8 {
    unsafe { accessor!(type_ -> u8, self.base.strct) }
  }

}

impl Iterator for LockBehaviorIterator {
    type Item = LockBehavior;
    fn next(&mut self) -> Option<LockBehavior> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xkb_lock_behavior_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xkb_lock_behavior_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct RadioGroupBehavior {pub base : base::Struct<xcb_xkb_radio_group_behavior_t> }


impl RadioGroupBehavior {
  pub fn type_(&mut self) -> u8 {
    unsafe { accessor!(type_ -> u8, self.base.strct) }
  }

  pub fn group(&mut self) -> u8 {
    unsafe { accessor!(group -> u8, self.base.strct) }
  }

}

impl Iterator for RadioGroupBehaviorIterator {
    type Item = RadioGroupBehavior;
    fn next(&mut self) -> Option<RadioGroupBehavior> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xkb_radio_group_behavior_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xkb_radio_group_behavior_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct Overlay1Behavior {pub base : base::Struct<xcb_xkb_overlay_1_behavior_t> }


impl Overlay1Behavior {
  pub fn type_(&mut self) -> u8 {
    unsafe { accessor!(type_ -> u8, self.base.strct) }
  }

  pub fn key(&mut self) -> xproto::Keycode {
    unsafe { accessor!(key -> xproto::Keycode, self.base.strct) }
  }

}

impl Iterator for Overlay1BehaviorIterator {
    type Item = Overlay1Behavior;
    fn next(&mut self) -> Option<Overlay1Behavior> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xkb_overlay_1_behavior_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xkb_overlay_1_behavior_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct Overlay2Behavior {pub base : base::Struct<xcb_xkb_overlay_2_behavior_t> }


impl Overlay2Behavior {
  pub fn type_(&mut self) -> u8 {
    unsafe { accessor!(type_ -> u8, self.base.strct) }
  }

  pub fn key(&mut self) -> u8 {
    unsafe { accessor!(key -> u8, self.base.strct) }
  }

}

impl Iterator for Overlay2BehaviorIterator {
    type Item = Overlay2Behavior;
    fn next(&mut self) -> Option<Overlay2Behavior> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xkb_overlay_2_behavior_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xkb_overlay_2_behavior_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct PermamentLockBehavior {pub base : base::Struct<xcb_xkb_permament_lock_behavior_t> }


impl PermamentLockBehavior {
  pub fn type_(&mut self) -> u8 {
    unsafe { accessor!(type_ -> u8, self.base.strct) }
  }

}

impl Iterator for PermamentLockBehaviorIterator {
    type Item = PermamentLockBehavior;
    fn next(&mut self) -> Option<PermamentLockBehavior> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xkb_permament_lock_behavior_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xkb_permament_lock_behavior_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct PermamentRadioGroupBehavior {pub base : base::Struct<xcb_xkb_permament_radio_group_behavior_t> }


impl PermamentRadioGroupBehavior {
  pub fn type_(&mut self) -> u8 {
    unsafe { accessor!(type_ -> u8, self.base.strct) }
  }

  pub fn group(&mut self) -> u8 {
    unsafe { accessor!(group -> u8, self.base.strct) }
  }

}

impl Iterator for PermamentRadioGroupBehaviorIterator {
    type Item = PermamentRadioGroupBehavior;
    fn next(&mut self) -> Option<PermamentRadioGroupBehavior> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xkb_permament_radio_group_behavior_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xkb_permament_radio_group_behavior_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct PermamentOverlay1Behavior {pub base : base::Struct<xcb_xkb_permament_overlay_1_behavior_t> }


impl PermamentOverlay1Behavior {
  pub fn type_(&mut self) -> u8 {
    unsafe { accessor!(type_ -> u8, self.base.strct) }
  }

  pub fn key(&mut self) -> xproto::Keycode {
    unsafe { accessor!(key -> xproto::Keycode, self.base.strct) }
  }

}

impl Iterator for PermamentOverlay1BehaviorIterator {
    type Item = PermamentOverlay1Behavior;
    fn next(&mut self) -> Option<PermamentOverlay1Behavior> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xkb_permament_overlay_1_behavior_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xkb_permament_overlay_1_behavior_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct PermamentOverlay2Behavior {pub base : base::Struct<xcb_xkb_permament_overlay_2_behavior_t> }


impl PermamentOverlay2Behavior {
  pub fn type_(&mut self) -> u8 {
    unsafe { accessor!(type_ -> u8, self.base.strct) }
  }

  pub fn key(&mut self) -> u8 {
    unsafe { accessor!(key -> u8, self.base.strct) }
  }

}

impl Iterator for PermamentOverlay2BehaviorIterator {
    type Item = PermamentOverlay2Behavior;
    fn next(&mut self) -> Option<PermamentOverlay2Behavior> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xkb_permament_overlay_2_behavior_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xkb_permament_overlay_2_behavior_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct Behavior {pub base : base::Struct<xcb_xkb_behavior_t>}

impl Iterator for BehaviorIterator {
    type Item = Behavior;
    fn next(&mut self) -> Option<Behavior> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xkb_behavior_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xkb_behavior_next(iter);
            Some(mem::transmute(*data))
        }
    }
}


impl SetBehavior {
  pub fn keycode(&mut self) -> xproto::Keycode {
    unsafe { accessor!(keycode -> xproto::Keycode, self.base.strct) }
  }

  pub fn behavior(&self) -> Behavior {
    unsafe { mem::transmute(self.base.strct.behavior) }
  }
}

impl Iterator for SetBehaviorIterator {
    type Item = SetBehavior;
    fn next(&mut self) -> Option<SetBehavior> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xkb_set_behavior_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xkb_set_behavior_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct SetExplicit {pub base : base::Struct<xcb_xkb_set_explicit_t> }


impl SetExplicit {
  pub fn keycode(&mut self) -> xproto::Keycode {
    unsafe { accessor!(keycode -> xproto::Keycode, self.base.strct) }
  }

  pub fn explicit(&mut self) -> u8 {
    unsafe { accessor!(explicit -> u8, self.base.strct) }
  }

}

impl Iterator for SetExplicitIterator {
    type Item = SetExplicit;
    fn next(&mut self) -> Option<SetExplicit> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xkb_set_explicit_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xkb_set_explicit_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct KeyModMap {pub base : base::Struct<xcb_xkb_key_mod_map_t> }


impl KeyModMap {
  pub fn keycode(&mut self) -> xproto::Keycode {
    unsafe { accessor!(keycode -> xproto::Keycode, self.base.strct) }
  }

  pub fn mods(&mut self) -> u8 {
    unsafe { accessor!(mods -> u8, self.base.strct) }
  }

}

impl Iterator for KeyModMapIterator {
    type Item = KeyModMap;
    fn next(&mut self) -> Option<KeyModMap> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xkb_key_mod_map_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xkb_key_mod_map_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct KeyVModMap {pub base : base::Struct<xcb_xkb_key_v_mod_map_t> }


impl KeyVModMap {
  pub fn keycode(&mut self) -> xproto::Keycode {
    unsafe { accessor!(keycode -> xproto::Keycode, self.base.strct) }
  }

  pub fn vmods(&mut self) -> u16 {
    unsafe { accessor!(vmods -> u16, self.base.strct) }
  }

}

impl Iterator for KeyVModMapIterator {
    type Item = KeyVModMap;
    fn next(&mut self) -> Option<KeyVModMap> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xkb_key_v_mod_map_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xkb_key_v_mod_map_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct KtSetMapEntry {pub base : base::Struct<xcb_xkb_kt_set_map_entry_t> }


impl KtSetMapEntry {
  pub fn level(&mut self) -> u8 {
    unsafe { accessor!(level -> u8, self.base.strct) }
  }

  pub fn realMods(&mut self) -> u8 {
    unsafe { accessor!(realMods -> u8, self.base.strct) }
  }

  pub fn virtualMods(&mut self) -> u16 {
    unsafe { accessor!(virtualMods -> u16, self.base.strct) }
  }

}

impl Iterator for KtSetMapEntryIterator {
    type Item = KtSetMapEntry;
    fn next(&mut self) -> Option<KtSetMapEntry> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xkb_kt_set_map_entry_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xkb_kt_set_map_entry_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct SetKeyType {pub base : base::Struct<xcb_xkb_set_key_type_t> }


impl SetKeyType {
  pub fn mask(&mut self) -> u8 {
    unsafe { accessor!(mask -> u8, self.base.strct) }
  }

  pub fn realMods(&mut self) -> u8 {
    unsafe { accessor!(realMods -> u8, self.base.strct) }
  }

  pub fn virtualMods(&mut self) -> u16 {
    unsafe { accessor!(virtualMods -> u16, self.base.strct) }
  }

  pub fn numLevels(&mut self) -> u8 {
    unsafe { accessor!(numLevels -> u8, self.base.strct) }
  }

  pub fn entries(&mut self) -> KtSetMapEntryIterator {
    unsafe { accessor!(KtSetMapEntryIterator, xcb_xkb_set_key_type_entries_iterator, self.base.strct) }
  }

  pub fn preserve_entries(&mut self) -> KtSetMapEntryIterator {
    unsafe { accessor!(KtSetMapEntryIterator, xcb_xkb_set_key_type_preserve_entries_iterator, self.base.strct) }
  }

}

impl Iterator for SetKeyTypeIterator {
    type Item = SetKeyType;
    fn next(&mut self) -> Option<SetKeyType> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xkb_set_key_type_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xkb_set_key_type_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub type String8 = xcb_xkb_string8_t;


impl Iterator for String8Iterator {
    type Item = String8;
    fn next(&mut self) -> Option<String8> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xkb_string8_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xkb_string8_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct Property {pub base : base::Struct<xcb_xkb_property_t> }


impl Property {
  pub fn name(&mut self) -> Vec<String8> {
    unsafe { accessor!(String8, xcb_xkb_property_name_length, xcb_xkb_property_name, self.base.strct) }
  }

  pub fn value(&mut self) -> Vec<String8> {
    unsafe { accessor!(String8, xcb_xkb_property_value_length, xcb_xkb_property_value, self.base.strct) }
  }

}

impl Iterator for PropertyIterator {
    type Item = Property;
    fn next(&mut self) -> Option<Property> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xkb_property_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xkb_property_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct Outline {pub base : base::Struct<xcb_xkb_outline_t> }


impl Outline {
  pub fn cornerRadius(&mut self) -> u8 {
    unsafe { accessor!(cornerRadius -> u8, self.base.strct) }
  }

  pub fn points(&mut self) -> xproto::PointIterator {
    unsafe { accessor!(xproto::PointIterator, xcb_xkb_outline_points_iterator, self.base.strct) }
  }

}

impl Iterator for OutlineIterator {
    type Item = Outline;
    fn next(&mut self) -> Option<Outline> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xkb_outline_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xkb_outline_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct Shape {pub base : base::Struct<xcb_xkb_shape_t> }


impl Shape {
  pub fn name(&mut self) -> xproto::Atom {
    unsafe { accessor!(name -> xproto::Atom, self.base.strct) }
  }

  pub fn primaryNdx(&mut self) -> u8 {
    unsafe { accessor!(primaryNdx -> u8, self.base.strct) }
  }

  pub fn approxNdx(&mut self) -> u8 {
    unsafe { accessor!(approxNdx -> u8, self.base.strct) }
  }

  pub fn outlines(&mut self) -> OutlineIterator {
    unsafe { accessor!(OutlineIterator, xcb_xkb_shape_outlines_iterator, self.base.strct) }
  }

}

impl Iterator for ShapeIterator {
    type Item = Shape;
    fn next(&mut self) -> Option<Shape> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xkb_shape_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xkb_shape_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct Key {pub base : base::Struct<xcb_xkb_key_t> }


impl Key {
  pub fn name(&self) -> Vec<String8> {
    unsafe { (self.base.strct.name).to_vec() }
  }

  pub fn gap(&mut self) -> i16 {
    unsafe { accessor!(gap -> i16, self.base.strct) }
  }

  pub fn shapeNdx(&mut self) -> u8 {
    unsafe { accessor!(shapeNdx -> u8, self.base.strct) }
  }

  pub fn colorNdx(&mut self) -> u8 {
    unsafe { accessor!(colorNdx -> u8, self.base.strct) }
  }

}

impl Iterator for KeyIterator {
    type Item = Key;
    fn next(&mut self) -> Option<Key> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xkb_key_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xkb_key_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct OverlayKey {pub base : base::Struct<xcb_xkb_overlay_key_t> }


impl OverlayKey {
  pub fn over(&self) -> Vec<String8> {
    unsafe { (self.base.strct.over).to_vec() }
  }

  pub fn under(&self) -> Vec<String8> {
    unsafe { (self.base.strct.under).to_vec() }
  }

}

impl Iterator for OverlayKeyIterator {
    type Item = OverlayKey;
    fn next(&mut self) -> Option<OverlayKey> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xkb_overlay_key_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xkb_overlay_key_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct OverlayRow {pub base : base::Struct<xcb_xkb_overlay_row_t> }


impl OverlayRow {
  pub fn rowUnder(&mut self) -> u8 {
    unsafe { accessor!(rowUnder -> u8, self.base.strct) }
  }

  pub fn keys(&mut self) -> OverlayKeyIterator {
    unsafe { accessor!(OverlayKeyIterator, xcb_xkb_overlay_row_keys_iterator, self.base.strct) }
  }

}

impl Iterator for OverlayRowIterator {
    type Item = OverlayRow;
    fn next(&mut self) -> Option<OverlayRow> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xkb_overlay_row_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xkb_overlay_row_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct Overlay {pub base : base::Struct<xcb_xkb_overlay_t> }


impl Overlay {
  pub fn name(&mut self) -> xproto::Atom {
    unsafe { accessor!(name -> xproto::Atom, self.base.strct) }
  }

  pub fn rows(&mut self) -> OverlayRowIterator {
    unsafe { accessor!(OverlayRowIterator, xcb_xkb_overlay_rows_iterator, self.base.strct) }
  }

}

impl Iterator for OverlayIterator {
    type Item = Overlay;
    fn next(&mut self) -> Option<Overlay> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xkb_overlay_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xkb_overlay_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct Row {pub base : base::Struct<xcb_xkb_row_t> }


impl Row {
  pub fn top(&mut self) -> i16 {
    unsafe { accessor!(top -> i16, self.base.strct) }
  }

  pub fn left(&mut self) -> i16 {
    unsafe { accessor!(left -> i16, self.base.strct) }
  }

  pub fn vertical(&mut self) -> u8 {
    unsafe { accessor!(vertical -> u8, self.base.strct) }
  }

  pub fn keys(&mut self) -> KeyIterator {
    unsafe { accessor!(KeyIterator, xcb_xkb_row_keys_iterator, self.base.strct) }
  }

}

impl Iterator for RowIterator {
    type Item = Row;
    fn next(&mut self) -> Option<Row> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xkb_row_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xkb_row_next(iter);
            Some(mem::transmute(*data))
        }
    }
}


impl CommonDoodad {
  pub fn name(&mut self) -> xproto::Atom {
    unsafe { accessor!(name -> xproto::Atom, self.base.strct) }
  }

  pub fn type_(&mut self) -> u8 {
    unsafe { accessor!(type_ -> u8, self.base.strct) }
  }

  pub fn priority(&mut self) -> u8 {
    unsafe { accessor!(priority -> u8, self.base.strct) }
  }

  pub fn top(&mut self) -> i16 {
    unsafe { accessor!(top -> i16, self.base.strct) }
  }

  pub fn left(&mut self) -> i16 {
    unsafe { accessor!(left -> i16, self.base.strct) }
  }

  pub fn angle(&mut self) -> i16 {
    unsafe { accessor!(angle -> i16, self.base.strct) }
  }

}

impl Iterator for CommonDoodadIterator {
    type Item = CommonDoodad;
    fn next(&mut self) -> Option<CommonDoodad> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xkb_common_doodad_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xkb_common_doodad_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct ShapeDoodad {pub base : base::Struct<xcb_xkb_shape_doodad_t> }


impl ShapeDoodad {
  pub fn name(&mut self) -> xproto::Atom {
    unsafe { accessor!(name -> xproto::Atom, self.base.strct) }
  }

  pub fn type_(&mut self) -> u8 {
    unsafe { accessor!(type_ -> u8, self.base.strct) }
  }

  pub fn priority(&mut self) -> u8 {
    unsafe { accessor!(priority -> u8, self.base.strct) }
  }

  pub fn top(&mut self) -> i16 {
    unsafe { accessor!(top -> i16, self.base.strct) }
  }

  pub fn left(&mut self) -> i16 {
    unsafe { accessor!(left -> i16, self.base.strct) }
  }

  pub fn angle(&mut self) -> i16 {
    unsafe { accessor!(angle -> i16, self.base.strct) }
  }

  pub fn colorNdx(&mut self) -> u8 {
    unsafe { accessor!(colorNdx -> u8, self.base.strct) }
  }

  pub fn shapeNdx(&mut self) -> u8 {
    unsafe { accessor!(shapeNdx -> u8, self.base.strct) }
  }

}

impl Iterator for ShapeDoodadIterator {
    type Item = ShapeDoodad;
    fn next(&mut self) -> Option<ShapeDoodad> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xkb_shape_doodad_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xkb_shape_doodad_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct TextDoodad {pub base : base::Struct<xcb_xkb_text_doodad_t> }


impl TextDoodad {
  pub fn name(&mut self) -> xproto::Atom {
    unsafe { accessor!(name -> xproto::Atom, self.base.strct) }
  }

  pub fn type_(&mut self) -> u8 {
    unsafe { accessor!(type_ -> u8, self.base.strct) }
  }

  pub fn priority(&mut self) -> u8 {
    unsafe { accessor!(priority -> u8, self.base.strct) }
  }

  pub fn top(&mut self) -> i16 {
    unsafe { accessor!(top -> i16, self.base.strct) }
  }

  pub fn left(&mut self) -> i16 {
    unsafe { accessor!(left -> i16, self.base.strct) }
  }

  pub fn angle(&mut self) -> i16 {
    unsafe { accessor!(angle -> i16, self.base.strct) }
  }

  pub fn width(&mut self) -> u16 {
    unsafe { accessor!(width -> u16, self.base.strct) }
  }

  pub fn height(&mut self) -> u16 {
    unsafe { accessor!(height -> u16, self.base.strct) }
  }

  pub fn colorNdx(&mut self) -> u8 {
    unsafe { accessor!(colorNdx -> u8, self.base.strct) }
  }

  pub fn text(&self) -> CountedString16 {
    unsafe { mem::transmute(self.base.strct.text) }
  }
  pub fn font(&self) -> CountedString16 {
    unsafe { mem::transmute(self.base.strct.font) }
  }
}

impl Iterator for TextDoodadIterator {
    type Item = TextDoodad;
    fn next(&mut self) -> Option<TextDoodad> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xkb_text_doodad_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xkb_text_doodad_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct IndicatorDoodad {pub base : base::Struct<xcb_xkb_indicator_doodad_t> }


impl IndicatorDoodad {
  pub fn name(&mut self) -> xproto::Atom {
    unsafe { accessor!(name -> xproto::Atom, self.base.strct) }
  }

  pub fn type_(&mut self) -> u8 {
    unsafe { accessor!(type_ -> u8, self.base.strct) }
  }

  pub fn priority(&mut self) -> u8 {
    unsafe { accessor!(priority -> u8, self.base.strct) }
  }

  pub fn top(&mut self) -> i16 {
    unsafe { accessor!(top -> i16, self.base.strct) }
  }

  pub fn left(&mut self) -> i16 {
    unsafe { accessor!(left -> i16, self.base.strct) }
  }

  pub fn angle(&mut self) -> i16 {
    unsafe { accessor!(angle -> i16, self.base.strct) }
  }

  pub fn shapeNdx(&mut self) -> u8 {
    unsafe { accessor!(shapeNdx -> u8, self.base.strct) }
  }

  pub fn onColorNdx(&mut self) -> u8 {
    unsafe { accessor!(onColorNdx -> u8, self.base.strct) }
  }

  pub fn offColorNdx(&mut self) -> u8 {
    unsafe { accessor!(offColorNdx -> u8, self.base.strct) }
  }

}

impl Iterator for IndicatorDoodadIterator {
    type Item = IndicatorDoodad;
    fn next(&mut self) -> Option<IndicatorDoodad> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xkb_indicator_doodad_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xkb_indicator_doodad_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct LogoDoodad {pub base : base::Struct<xcb_xkb_logo_doodad_t> }


impl LogoDoodad {
  pub fn name(&mut self) -> xproto::Atom {
    unsafe { accessor!(name -> xproto::Atom, self.base.strct) }
  }

  pub fn type_(&mut self) -> u8 {
    unsafe { accessor!(type_ -> u8, self.base.strct) }
  }

  pub fn priority(&mut self) -> u8 {
    unsafe { accessor!(priority -> u8, self.base.strct) }
  }

  pub fn top(&mut self) -> i16 {
    unsafe { accessor!(top -> i16, self.base.strct) }
  }

  pub fn left(&mut self) -> i16 {
    unsafe { accessor!(left -> i16, self.base.strct) }
  }

  pub fn angle(&mut self) -> i16 {
    unsafe { accessor!(angle -> i16, self.base.strct) }
  }

  pub fn colorNdx(&mut self) -> u8 {
    unsafe { accessor!(colorNdx -> u8, self.base.strct) }
  }

  pub fn shapeNdx(&mut self) -> u8 {
    unsafe { accessor!(shapeNdx -> u8, self.base.strct) }
  }

  pub fn logoName(&self) -> CountedString16 {
    unsafe { mem::transmute(self.base.strct.logoName) }
  }
}

impl Iterator for LogoDoodadIterator {
    type Item = LogoDoodad;
    fn next(&mut self) -> Option<LogoDoodad> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xkb_logo_doodad_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xkb_logo_doodad_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct Doodad {pub base : base::Struct<xcb_xkb_doodad_t>}

impl Iterator for DoodadIterator {
    type Item = Doodad;
    fn next(&mut self) -> Option<Doodad> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xkb_doodad_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xkb_doodad_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct Section {pub base : base::Struct<xcb_xkb_section_t> }


impl Section {
  pub fn name(&mut self) -> xproto::Atom {
    unsafe { accessor!(name -> xproto::Atom, self.base.strct) }
  }

  pub fn top(&mut self) -> i16 {
    unsafe { accessor!(top -> i16, self.base.strct) }
  }

  pub fn left(&mut self) -> i16 {
    unsafe { accessor!(left -> i16, self.base.strct) }
  }

  pub fn width(&mut self) -> u16 {
    unsafe { accessor!(width -> u16, self.base.strct) }
  }

  pub fn height(&mut self) -> u16 {
    unsafe { accessor!(height -> u16, self.base.strct) }
  }

  pub fn angle(&mut self) -> i16 {
    unsafe { accessor!(angle -> i16, self.base.strct) }
  }

  pub fn priority(&mut self) -> u8 {
    unsafe { accessor!(priority -> u8, self.base.strct) }
  }

  pub fn rows(&mut self) -> RowIterator {
    unsafe { accessor!(RowIterator, xcb_xkb_section_rows_iterator, self.base.strct) }
  }

  pub fn doodads(&mut self) -> DoodadIterator {
    unsafe { accessor!(DoodadIterator, xcb_xkb_section_doodads_iterator, self.base.strct) }
  }

  pub fn overlays(&mut self) -> OverlayIterator {
    unsafe { accessor!(OverlayIterator, xcb_xkb_section_overlays_iterator, self.base.strct) }
  }

}

impl Iterator for SectionIterator {
    type Item = Section;
    fn next(&mut self) -> Option<Section> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xkb_section_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xkb_section_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct Listing {pub base : base::Struct<xcb_xkb_listing_t> }


impl Listing {
  pub fn flags(&mut self) -> u16 {
    unsafe { accessor!(flags -> u16, self.base.strct) }
  }

  pub fn string(&mut self) -> Vec<String8> {
    unsafe { accessor!(String8, xcb_xkb_listing_string_length, xcb_xkb_listing_string, self.base.strct) }
  }

}

impl Iterator for ListingIterator {
    type Item = Listing;
    fn next(&mut self) -> Option<Listing> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xkb_listing_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xkb_listing_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct DeviceLedInfo {pub base : base::Struct<xcb_xkb_device_led_info_t> }


impl DeviceLedInfo {
  pub fn ledClass(&mut self) -> LedClassSpec {
    unsafe { accessor!(ledClass -> LedClassSpec, self.base.strct) }
  }

  pub fn ledID(&mut self) -> IdSpec {
    unsafe { accessor!(ledID -> IdSpec, self.base.strct) }
  }

  pub fn physIndicators(&mut self) -> u32 {
    unsafe { accessor!(physIndicators -> u32, self.base.strct) }
  }

  pub fn state(&mut self) -> u32 {
    unsafe { accessor!(state -> u32, self.base.strct) }
  }

  pub fn names(&mut self) -> Vec<u32> {
    unsafe { accessor!(u32, xcb_xkb_device_led_info_names_length, xcb_xkb_device_led_info_names, self.base.strct) }
  }

  pub fn maps(&mut self) -> IndicatorMapIterator {
    unsafe { accessor!(IndicatorMapIterator, xcb_xkb_device_led_info_maps_iterator, self.base.strct) }
  }

}

impl Iterator for DeviceLedInfoIterator {
    type Item = DeviceLedInfo;
    fn next(&mut self) -> Option<DeviceLedInfo> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xkb_device_led_info_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xkb_device_led_info_next(iter);
            Some(mem::transmute(*data))
        }
    }
}


impl SaNoAction {
  pub fn type_(&mut self) -> u8 {
    unsafe { accessor!(type_ -> u8, self.base.strct) }
  }

}

impl Iterator for SaNoActionIterator {
    type Item = SaNoAction;
    fn next(&mut self) -> Option<SaNoAction> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xkb_sa_no_action_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xkb_sa_no_action_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct SaSetMods {pub base : base::Struct<xcb_xkb_sa_set_mods_t> }


impl SaSetMods {
  pub fn type_(&mut self) -> u8 {
    unsafe { accessor!(type_ -> u8, self.base.strct) }
  }

  pub fn flags(&mut self) -> u8 {
    unsafe { accessor!(flags -> u8, self.base.strct) }
  }

  pub fn mask(&mut self) -> u8 {
    unsafe { accessor!(mask -> u8, self.base.strct) }
  }

  pub fn realMods(&mut self) -> u8 {
    unsafe { accessor!(realMods -> u8, self.base.strct) }
  }

  pub fn vmodsHigh(&mut self) -> u8 {
    unsafe { accessor!(vmodsHigh -> u8, self.base.strct) }
  }

  pub fn vmodsLow(&mut self) -> u8 {
    unsafe { accessor!(vmodsLow -> u8, self.base.strct) }
  }

}

impl Iterator for SaSetModsIterator {
    type Item = SaSetMods;
    fn next(&mut self) -> Option<SaSetMods> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xkb_sa_set_mods_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xkb_sa_set_mods_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct SaLatchMods {pub base : base::Struct<xcb_xkb_sa_latch_mods_t> }


impl SaLatchMods {
  pub fn type_(&mut self) -> u8 {
    unsafe { accessor!(type_ -> u8, self.base.strct) }
  }

  pub fn flags(&mut self) -> u8 {
    unsafe { accessor!(flags -> u8, self.base.strct) }
  }

  pub fn mask(&mut self) -> u8 {
    unsafe { accessor!(mask -> u8, self.base.strct) }
  }

  pub fn realMods(&mut self) -> u8 {
    unsafe { accessor!(realMods -> u8, self.base.strct) }
  }

  pub fn vmodsHigh(&mut self) -> u8 {
    unsafe { accessor!(vmodsHigh -> u8, self.base.strct) }
  }

  pub fn vmodsLow(&mut self) -> u8 {
    unsafe { accessor!(vmodsLow -> u8, self.base.strct) }
  }

}

impl Iterator for SaLatchModsIterator {
    type Item = SaLatchMods;
    fn next(&mut self) -> Option<SaLatchMods> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xkb_sa_latch_mods_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xkb_sa_latch_mods_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct SaLockMods {pub base : base::Struct<xcb_xkb_sa_lock_mods_t> }


impl SaLockMods {
  pub fn type_(&mut self) -> u8 {
    unsafe { accessor!(type_ -> u8, self.base.strct) }
  }

  pub fn flags(&mut self) -> u8 {
    unsafe { accessor!(flags -> u8, self.base.strct) }
  }

  pub fn mask(&mut self) -> u8 {
    unsafe { accessor!(mask -> u8, self.base.strct) }
  }

  pub fn realMods(&mut self) -> u8 {
    unsafe { accessor!(realMods -> u8, self.base.strct) }
  }

  pub fn vmodsHigh(&mut self) -> u8 {
    unsafe { accessor!(vmodsHigh -> u8, self.base.strct) }
  }

  pub fn vmodsLow(&mut self) -> u8 {
    unsafe { accessor!(vmodsLow -> u8, self.base.strct) }
  }

}

impl Iterator for SaLockModsIterator {
    type Item = SaLockMods;
    fn next(&mut self) -> Option<SaLockMods> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xkb_sa_lock_mods_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xkb_sa_lock_mods_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct SaSetGroup {pub base : base::Struct<xcb_xkb_sa_set_group_t> }


impl SaSetGroup {
  pub fn type_(&mut self) -> u8 {
    unsafe { accessor!(type_ -> u8, self.base.strct) }
  }

  pub fn flags(&mut self) -> u8 {
    unsafe { accessor!(flags -> u8, self.base.strct) }
  }

  pub fn group(&mut self) -> i8 {
    unsafe { accessor!(group -> i8, self.base.strct) }
  }

}

impl Iterator for SaSetGroupIterator {
    type Item = SaSetGroup;
    fn next(&mut self) -> Option<SaSetGroup> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xkb_sa_set_group_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xkb_sa_set_group_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct SaLatchGroup {pub base : base::Struct<xcb_xkb_sa_latch_group_t> }


impl SaLatchGroup {
  pub fn type_(&mut self) -> u8 {
    unsafe { accessor!(type_ -> u8, self.base.strct) }
  }

  pub fn flags(&mut self) -> u8 {
    unsafe { accessor!(flags -> u8, self.base.strct) }
  }

  pub fn group(&mut self) -> i8 {
    unsafe { accessor!(group -> i8, self.base.strct) }
  }

}

impl Iterator for SaLatchGroupIterator {
    type Item = SaLatchGroup;
    fn next(&mut self) -> Option<SaLatchGroup> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xkb_sa_latch_group_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xkb_sa_latch_group_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct SaLockGroup {pub base : base::Struct<xcb_xkb_sa_lock_group_t> }


impl SaLockGroup {
  pub fn type_(&mut self) -> u8 {
    unsafe { accessor!(type_ -> u8, self.base.strct) }
  }

  pub fn flags(&mut self) -> u8 {
    unsafe { accessor!(flags -> u8, self.base.strct) }
  }

  pub fn group(&mut self) -> i8 {
    unsafe { accessor!(group -> i8, self.base.strct) }
  }

}

impl Iterator for SaLockGroupIterator {
    type Item = SaLockGroup;
    fn next(&mut self) -> Option<SaLockGroup> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xkb_sa_lock_group_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xkb_sa_lock_group_next(iter);
            Some(mem::transmute(*data))
        }
    }
}


impl SaMovePtr {
  pub fn type_(&mut self) -> u8 {
    unsafe { accessor!(type_ -> u8, self.base.strct) }
  }

  pub fn flags(&mut self) -> u8 {
    unsafe { accessor!(flags -> u8, self.base.strct) }
  }

  pub fn xHigh(&mut self) -> i8 {
    unsafe { accessor!(xHigh -> i8, self.base.strct) }
  }

  pub fn xLow(&mut self) -> u8 {
    unsafe { accessor!(xLow -> u8, self.base.strct) }
  }

  pub fn yHigh(&mut self) -> i8 {
    unsafe { accessor!(yHigh -> i8, self.base.strct) }
  }

  pub fn yLow(&mut self) -> u8 {
    unsafe { accessor!(yLow -> u8, self.base.strct) }
  }

}

impl Iterator for SaMovePtrIterator {
    type Item = SaMovePtr;
    fn next(&mut self) -> Option<SaMovePtr> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xkb_sa_move_ptr_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xkb_sa_move_ptr_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct SaPtrBtn {pub base : base::Struct<xcb_xkb_sa_ptr_btn_t> }


impl SaPtrBtn {
  pub fn type_(&mut self) -> u8 {
    unsafe { accessor!(type_ -> u8, self.base.strct) }
  }

  pub fn flags(&mut self) -> u8 {
    unsafe { accessor!(flags -> u8, self.base.strct) }
  }

  pub fn count(&mut self) -> u8 {
    unsafe { accessor!(count -> u8, self.base.strct) }
  }

  pub fn button(&mut self) -> u8 {
    unsafe { accessor!(button -> u8, self.base.strct) }
  }

}

impl Iterator for SaPtrBtnIterator {
    type Item = SaPtrBtn;
    fn next(&mut self) -> Option<SaPtrBtn> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xkb_sa_ptr_btn_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xkb_sa_ptr_btn_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct SaLockPtrBtn {pub base : base::Struct<xcb_xkb_sa_lock_ptr_btn_t> }


impl SaLockPtrBtn {
  pub fn type_(&mut self) -> u8 {
    unsafe { accessor!(type_ -> u8, self.base.strct) }
  }

  pub fn flags(&mut self) -> u8 {
    unsafe { accessor!(flags -> u8, self.base.strct) }
  }

  pub fn button(&mut self) -> u8 {
    unsafe { accessor!(button -> u8, self.base.strct) }
  }

}

impl Iterator for SaLockPtrBtnIterator {
    type Item = SaLockPtrBtn;
    fn next(&mut self) -> Option<SaLockPtrBtn> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xkb_sa_lock_ptr_btn_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xkb_sa_lock_ptr_btn_next(iter);
            Some(mem::transmute(*data))
        }
    }
}


impl SaSetPtrDflt {
  pub fn type_(&mut self) -> u8 {
    unsafe { accessor!(type_ -> u8, self.base.strct) }
  }

  pub fn flags(&mut self) -> u8 {
    unsafe { accessor!(flags -> u8, self.base.strct) }
  }

  pub fn affect(&mut self) -> u8 {
    unsafe { accessor!(affect -> u8, self.base.strct) }
  }

  pub fn value(&mut self) -> i8 {
    unsafe { accessor!(value -> i8, self.base.strct) }
  }

}

impl Iterator for SaSetPtrDfltIterator {
    type Item = SaSetPtrDflt;
    fn next(&mut self) -> Option<SaSetPtrDflt> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xkb_sa_set_ptr_dflt_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xkb_sa_set_ptr_dflt_next(iter);
            Some(mem::transmute(*data))
        }
    }
}


impl SaIsoLock {
  pub fn type_(&mut self) -> u8 {
    unsafe { accessor!(type_ -> u8, self.base.strct) }
  }

  pub fn flags(&mut self) -> u8 {
    unsafe { accessor!(flags -> u8, self.base.strct) }
  }

  pub fn mask(&mut self) -> u8 {
    unsafe { accessor!(mask -> u8, self.base.strct) }
  }

  pub fn realMods(&mut self) -> u8 {
    unsafe { accessor!(realMods -> u8, self.base.strct) }
  }

  pub fn group(&mut self) -> i8 {
    unsafe { accessor!(group -> i8, self.base.strct) }
  }

  pub fn affect(&mut self) -> u8 {
    unsafe { accessor!(affect -> u8, self.base.strct) }
  }

  pub fn vmodsHigh(&mut self) -> u8 {
    unsafe { accessor!(vmodsHigh -> u8, self.base.strct) }
  }

  pub fn vmodsLow(&mut self) -> u8 {
    unsafe { accessor!(vmodsLow -> u8, self.base.strct) }
  }

}

impl Iterator for SaIsoLockIterator {
    type Item = SaIsoLock;
    fn next(&mut self) -> Option<SaIsoLock> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xkb_sa_iso_lock_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xkb_sa_iso_lock_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct SaTerminate {pub base : base::Struct<xcb_xkb_sa_terminate_t> }


impl SaTerminate {
  pub fn type_(&mut self) -> u8 {
    unsafe { accessor!(type_ -> u8, self.base.strct) }
  }

}

impl Iterator for SaTerminateIterator {
    type Item = SaTerminate;
    fn next(&mut self) -> Option<SaTerminate> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xkb_sa_terminate_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xkb_sa_terminate_next(iter);
            Some(mem::transmute(*data))
        }
    }
}


impl SaSwitchScreen {
  pub fn type_(&mut self) -> u8 {
    unsafe { accessor!(type_ -> u8, self.base.strct) }
  }

  pub fn flags(&mut self) -> u8 {
    unsafe { accessor!(flags -> u8, self.base.strct) }
  }

  pub fn newScreen(&mut self) -> i8 {
    unsafe { accessor!(newScreen -> i8, self.base.strct) }
  }

}

impl Iterator for SaSwitchScreenIterator {
    type Item = SaSwitchScreen;
    fn next(&mut self) -> Option<SaSwitchScreen> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xkb_sa_switch_screen_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xkb_sa_switch_screen_next(iter);
            Some(mem::transmute(*data))
        }
    }
}


impl SaSetControls {
  pub fn type_(&mut self) -> u8 {
    unsafe { accessor!(type_ -> u8, self.base.strct) }
  }

  pub fn boolCtrlsHigh(&mut self) -> u8 {
    unsafe { accessor!(boolCtrlsHigh -> u8, self.base.strct) }
  }

  pub fn boolCtrlsLow(&mut self) -> u8 {
    unsafe { accessor!(boolCtrlsLow -> u8, self.base.strct) }
  }

}

impl Iterator for SaSetControlsIterator {
    type Item = SaSetControls;
    fn next(&mut self) -> Option<SaSetControls> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xkb_sa_set_controls_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xkb_sa_set_controls_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct SaLockControls {pub base : base::Struct<xcb_xkb_sa_lock_controls_t> }


impl SaLockControls {
  pub fn type_(&mut self) -> u8 {
    unsafe { accessor!(type_ -> u8, self.base.strct) }
  }

  pub fn boolCtrlsHigh(&mut self) -> u8 {
    unsafe { accessor!(boolCtrlsHigh -> u8, self.base.strct) }
  }

  pub fn boolCtrlsLow(&mut self) -> u8 {
    unsafe { accessor!(boolCtrlsLow -> u8, self.base.strct) }
  }

}

impl Iterator for SaLockControlsIterator {
    type Item = SaLockControls;
    fn next(&mut self) -> Option<SaLockControls> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xkb_sa_lock_controls_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xkb_sa_lock_controls_next(iter);
            Some(mem::transmute(*data))
        }
    }
}


impl SaActionMessage {
  pub fn type_(&mut self) -> u8 {
    unsafe { accessor!(type_ -> u8, self.base.strct) }
  }

  pub fn flags(&mut self) -> u8 {
    unsafe { accessor!(flags -> u8, self.base.strct) }
  }

  pub fn message(&self) -> Vec<u8> {
    unsafe { (self.base.strct.message).to_vec() }
  }

}

impl Iterator for SaActionMessageIterator {
    type Item = SaActionMessage;
    fn next(&mut self) -> Option<SaActionMessage> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xkb_sa_action_message_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xkb_sa_action_message_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct SaRedirectKey {pub base : base::Struct<xcb_xkb_sa_redirect_key_t> }


impl SaRedirectKey {
  pub fn type_(&mut self) -> u8 {
    unsafe { accessor!(type_ -> u8, self.base.strct) }
  }

  pub fn newkey(&mut self) -> xproto::Keycode {
    unsafe { accessor!(newkey -> xproto::Keycode, self.base.strct) }
  }

  pub fn mask(&mut self) -> u8 {
    unsafe { accessor!(mask -> u8, self.base.strct) }
  }

  pub fn realModifiers(&mut self) -> u8 {
    unsafe { accessor!(realModifiers -> u8, self.base.strct) }
  }

  pub fn vmodsMaskHigh(&mut self) -> u8 {
    unsafe { accessor!(vmodsMaskHigh -> u8, self.base.strct) }
  }

  pub fn vmodsMaskLow(&mut self) -> u8 {
    unsafe { accessor!(vmodsMaskLow -> u8, self.base.strct) }
  }

  pub fn vmodsHigh(&mut self) -> u8 {
    unsafe { accessor!(vmodsHigh -> u8, self.base.strct) }
  }

  pub fn vmodsLow(&mut self) -> u8 {
    unsafe { accessor!(vmodsLow -> u8, self.base.strct) }
  }

}

impl Iterator for SaRedirectKeyIterator {
    type Item = SaRedirectKey;
    fn next(&mut self) -> Option<SaRedirectKey> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xkb_sa_redirect_key_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xkb_sa_redirect_key_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct SaDeviceBtn {pub base : base::Struct<xcb_xkb_sa_device_btn_t> }


impl SaDeviceBtn {
  pub fn type_(&mut self) -> u8 {
    unsafe { accessor!(type_ -> u8, self.base.strct) }
  }

  pub fn flags(&mut self) -> u8 {
    unsafe { accessor!(flags -> u8, self.base.strct) }
  }

  pub fn count(&mut self) -> u8 {
    unsafe { accessor!(count -> u8, self.base.strct) }
  }

  pub fn button(&mut self) -> u8 {
    unsafe { accessor!(button -> u8, self.base.strct) }
  }

  pub fn device(&mut self) -> u8 {
    unsafe { accessor!(device -> u8, self.base.strct) }
  }

}

impl Iterator for SaDeviceBtnIterator {
    type Item = SaDeviceBtn;
    fn next(&mut self) -> Option<SaDeviceBtn> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xkb_sa_device_btn_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xkb_sa_device_btn_next(iter);
            Some(mem::transmute(*data))
        }
    }
}


impl SaLockDeviceBtn {
  pub fn type_(&mut self) -> u8 {
    unsafe { accessor!(type_ -> u8, self.base.strct) }
  }

  pub fn flags(&mut self) -> u8 {
    unsafe { accessor!(flags -> u8, self.base.strct) }
  }

  pub fn button(&mut self) -> u8 {
    unsafe { accessor!(button -> u8, self.base.strct) }
  }

  pub fn device(&mut self) -> u8 {
    unsafe { accessor!(device -> u8, self.base.strct) }
  }

}

impl Iterator for SaLockDeviceBtnIterator {
    type Item = SaLockDeviceBtn;
    fn next(&mut self) -> Option<SaLockDeviceBtn> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xkb_sa_lock_device_btn_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xkb_sa_lock_device_btn_next(iter);
            Some(mem::transmute(*data))
        }
    }
}


impl SaDeviceValuator {
  pub fn type_(&mut self) -> u8 {
    unsafe { accessor!(type_ -> u8, self.base.strct) }
  }

  pub fn device(&mut self) -> u8 {
    unsafe { accessor!(device -> u8, self.base.strct) }
  }

  pub fn val1what(&mut self) -> u8 {
    unsafe { accessor!(val1what -> u8, self.base.strct) }
  }

  pub fn val1index(&mut self) -> u8 {
    unsafe { accessor!(val1index -> u8, self.base.strct) }
  }

  pub fn val1value(&mut self) -> u8 {
    unsafe { accessor!(val1value -> u8, self.base.strct) }
  }

  pub fn val2what(&mut self) -> u8 {
    unsafe { accessor!(val2what -> u8, self.base.strct) }
  }

  pub fn val2index(&mut self) -> u8 {
    unsafe { accessor!(val2index -> u8, self.base.strct) }
  }

  pub fn val2value(&mut self) -> u8 {
    unsafe { accessor!(val2value -> u8, self.base.strct) }
  }

}

impl Iterator for SaDeviceValuatorIterator {
    type Item = SaDeviceValuator;
    fn next(&mut self) -> Option<SaDeviceValuator> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xkb_sa_device_valuator_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xkb_sa_device_valuator_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct Action {pub base : base::Struct<xcb_xkb_action_t>}

impl Iterator for ActionIterator {
    type Item = Action;
    fn next(&mut self) -> Option<Action> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xkb_action_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xkb_action_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub fn UseExtension<'r> (c : &'r Connection,
                     wantedMajor : u16,
                     wantedMinor : u16) -> UseExtensionCookie<'r> {
  unsafe {
    let cookie = xcb_xkb_use_extension(c.get_raw_conn(),
        wantedMajor as u16, //1
        wantedMinor as u16); //2
    UseExtensionCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn UseExtensionUnchecked<'r> (c : &'r Connection,
                              wantedMajor : u16,
                              wantedMinor : u16) -> UseExtensionCookie<'r> {
  unsafe {
    let cookie = xcb_xkb_use_extension_unchecked(c.get_raw_conn(),
        wantedMajor as u16, //1
        wantedMinor as u16); //2
    UseExtensionCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl UseExtensionReply {
  pub fn supported(&mut self) -> u8 {
    unsafe { accessor!(supported -> u8, (*self.base.reply)) }
  }

  pub fn serverMajor(&mut self) -> u16 {
    unsafe { accessor!(serverMajor -> u16, (*self.base.reply)) }
  }

  pub fn serverMinor(&mut self) -> u16 {
    unsafe { accessor!(serverMinor -> u16, (*self.base.reply)) }
  }

}
impl_reply_cookie!(UseExtensionCookie<'s>, mk_reply_xcb_xkb_use_extension_reply_t, UseExtensionReply, xcb_xkb_use_extension_reply);

pub struct SelectEventsDetails {pub base : base::Struct<xcb_xkb_select_events_details_t> }


impl SelectEventsDetails {
  pub fn affectNewKeyboard(&mut self) -> u16 {
    unsafe { accessor!(affectNewKeyboard -> u16, self.base.strct) }
  }

  pub fn newKeyboardDetails(&mut self) -> u16 {
    unsafe { accessor!(newKeyboardDetails -> u16, self.base.strct) }
  }

  pub fn affectState(&mut self) -> u16 {
    unsafe { accessor!(affectState -> u16, self.base.strct) }
  }

  pub fn stateDetails(&mut self) -> u16 {
    unsafe { accessor!(stateDetails -> u16, self.base.strct) }
  }

  pub fn affectCtrls(&mut self) -> u32 {
    unsafe { accessor!(affectCtrls -> u32, self.base.strct) }
  }

  pub fn ctrlDetails(&mut self) -> u32 {
    unsafe { accessor!(ctrlDetails -> u32, self.base.strct) }
  }

  pub fn affectIndicatorState(&mut self) -> u32 {
    unsafe { accessor!(affectIndicatorState -> u32, self.base.strct) }
  }

  pub fn indicatorStateDetails(&mut self) -> u32 {
    unsafe { accessor!(indicatorStateDetails -> u32, self.base.strct) }
  }

  pub fn affectIndicatorMap(&mut self) -> u32 {
    unsafe { accessor!(affectIndicatorMap -> u32, self.base.strct) }
  }

  pub fn indicatorMapDetails(&mut self) -> u32 {
    unsafe { accessor!(indicatorMapDetails -> u32, self.base.strct) }
  }

  pub fn affectNames(&mut self) -> u16 {
    unsafe { accessor!(affectNames -> u16, self.base.strct) }
  }

  pub fn namesDetails(&mut self) -> u16 {
    unsafe { accessor!(namesDetails -> u16, self.base.strct) }
  }

  pub fn affectCompat(&mut self) -> u8 {
    unsafe { accessor!(affectCompat -> u8, self.base.strct) }
  }

  pub fn compatDetails(&mut self) -> u8 {
    unsafe { accessor!(compatDetails -> u8, self.base.strct) }
  }

  pub fn affectBell(&mut self) -> u8 {
    unsafe { accessor!(affectBell -> u8, self.base.strct) }
  }

  pub fn bellDetails(&mut self) -> u8 {
    unsafe { accessor!(bellDetails -> u8, self.base.strct) }
  }

  pub fn affectMsgDetails(&mut self) -> u8 {
    unsafe { accessor!(affectMsgDetails -> u8, self.base.strct) }
  }

  pub fn msgDetails(&mut self) -> u8 {
    unsafe { accessor!(msgDetails -> u8, self.base.strct) }
  }

  pub fn affectAccessX(&mut self) -> u16 {
    unsafe { accessor!(affectAccessX -> u16, self.base.strct) }
  }

  pub fn accessXDetails(&mut self) -> u16 {
    unsafe { accessor!(accessXDetails -> u16, self.base.strct) }
  }

  pub fn affectExtDev(&mut self) -> u16 {
    unsafe { accessor!(affectExtDev -> u16, self.base.strct) }
  }

  pub fn extdevDetails(&mut self) -> u16 {
    unsafe { accessor!(extdevDetails -> u16, self.base.strct) }
  }

}
pub fn SelectEventsChecked<'r> (c : &'r Connection,
                            deviceSpec : DeviceSpec,
                            affectWhich : u16,
                            clear : u16,
                            selectAll : u16,
                            affectMap : u16,
                            map : u16,
                            details : SelectEventsDetails) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xkb_select_events_checked(c.get_raw_conn(),
        deviceSpec as xcb_xkb_device_spec_t, //1
        affectWhich as u16, //2
        clear as u16, //3
        selectAll as u16, //4
        affectMap as u16, //5
        map as u16, //6
        details.base.strct); //7
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn SelectEvents<'r> (c : &'r Connection,
                     deviceSpec : DeviceSpec,
                     affectWhich : u16,
                     clear : u16,
                     selectAll : u16,
                     affectMap : u16,
                     map : u16,
                     details : SelectEventsDetails) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xkb_select_events(c.get_raw_conn(),
        deviceSpec as xcb_xkb_device_spec_t, //1
        affectWhich as u16, //2
        clear as u16, //3
        selectAll as u16, //4
        affectMap as u16, //5
        map as u16, //6
        details.base.strct); //7
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn SelectEventsAuxChecked<'r> (c : &'r Connection,
                               deviceSpec : DeviceSpec,
                               affectWhich : u16,
                               clear : u16,
                               selectAll : u16,
                               affectMap : u16,
                               map : u16,
                               details : SelectEventsDetails) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xkb_select_events_aux_checked(c.get_raw_conn(),
        deviceSpec as xcb_xkb_device_spec_t, //1
        affectWhich as u16, //2
        clear as u16, //3
        selectAll as u16, //4
        affectMap as u16, //5
        map as u16, //6
        details.base.strct); //7
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn SelectEventsAux<'r> (c : &'r Connection,
                        deviceSpec : DeviceSpec,
                        affectWhich : u16,
                        clear : u16,
                        selectAll : u16,
                        affectMap : u16,
                        map : u16,
                        details : SelectEventsDetails) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xkb_select_events_aux(c.get_raw_conn(),
        deviceSpec as xcb_xkb_device_spec_t, //1
        affectWhich as u16, //2
        clear as u16, //3
        selectAll as u16, //4
        affectMap as u16, //5
        map as u16, //6
        details.base.strct); //7
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn BellChecked<'r> (c : &'r Connection,
                    deviceSpec : DeviceSpec,
                    bellClass : BellClassSpec,
                    bellID : IdSpec,
                    percent : i8,
                    forceSound : u8,
                    eventOnly : u8,
                    pitch : i16,
                    duration : i16,
                    name : xproto::Atom,
                    window : xproto::Window) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xkb_bell_checked(c.get_raw_conn(),
        deviceSpec as xcb_xkb_device_spec_t, //1
        bellClass as xcb_xkb_bell_class_spec_t, //2
        bellID as xcb_xkb_id_spec_t, //3
        percent as i8, //4
        forceSound as u8, //5
        eventOnly as u8, //6
        pitch as i16, //7
        duration as i16, //8
        name as ffi::xproto::xcb_atom_t, //9
        window as ffi::xproto::xcb_window_t); //10
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn Bell<'r> (c : &'r Connection,
             deviceSpec : DeviceSpec,
             bellClass : BellClassSpec,
             bellID : IdSpec,
             percent : i8,
             forceSound : u8,
             eventOnly : u8,
             pitch : i16,
             duration : i16,
             name : xproto::Atom,
             window : xproto::Window) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xkb_bell(c.get_raw_conn(),
        deviceSpec as xcb_xkb_device_spec_t, //1
        bellClass as xcb_xkb_bell_class_spec_t, //2
        bellID as xcb_xkb_id_spec_t, //3
        percent as i8, //4
        forceSound as u8, //5
        eventOnly as u8, //6
        pitch as i16, //7
        duration as i16, //8
        name as ffi::xproto::xcb_atom_t, //9
        window as ffi::xproto::xcb_window_t); //10
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetState<'r> (c : &'r Connection,
                 deviceSpec : DeviceSpec) -> GetStateCookie<'r> {
  unsafe {
    let cookie = xcb_xkb_get_state(c.get_raw_conn(),
        deviceSpec as xcb_xkb_device_spec_t); //1
    GetStateCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetStateUnchecked<'r> (c : &'r Connection,
                          deviceSpec : DeviceSpec) -> GetStateCookie<'r> {
  unsafe {
    let cookie = xcb_xkb_get_state_unchecked(c.get_raw_conn(),
        deviceSpec as xcb_xkb_device_spec_t); //1
    GetStateCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetStateReply {
  pub fn deviceID(&mut self) -> u8 {
    unsafe { accessor!(deviceID -> u8, (*self.base.reply)) }
  }

  pub fn mods(&mut self) -> u8 {
    unsafe { accessor!(mods -> u8, (*self.base.reply)) }
  }

  pub fn baseMods(&mut self) -> u8 {
    unsafe { accessor!(baseMods -> u8, (*self.base.reply)) }
  }

  pub fn latchedMods(&mut self) -> u8 {
    unsafe { accessor!(latchedMods -> u8, (*self.base.reply)) }
  }

  pub fn lockedMods(&mut self) -> u8 {
    unsafe { accessor!(lockedMods -> u8, (*self.base.reply)) }
  }

  pub fn group(&mut self) -> u8 {
    unsafe { accessor!(group -> u8, (*self.base.reply)) }
  }

  pub fn lockedGroup(&mut self) -> u8 {
    unsafe { accessor!(lockedGroup -> u8, (*self.base.reply)) }
  }

  pub fn baseGroup(&mut self) -> i16 {
    unsafe { accessor!(baseGroup -> i16, (*self.base.reply)) }
  }

  pub fn latchedGroup(&mut self) -> i16 {
    unsafe { accessor!(latchedGroup -> i16, (*self.base.reply)) }
  }

  pub fn compatState(&mut self) -> u8 {
    unsafe { accessor!(compatState -> u8, (*self.base.reply)) }
  }

  pub fn grabMods(&mut self) -> u8 {
    unsafe { accessor!(grabMods -> u8, (*self.base.reply)) }
  }

  pub fn compatGrabMods(&mut self) -> u8 {
    unsafe { accessor!(compatGrabMods -> u8, (*self.base.reply)) }
  }

  pub fn lookupMods(&mut self) -> u8 {
    unsafe { accessor!(lookupMods -> u8, (*self.base.reply)) }
  }

  pub fn compatLookupMods(&mut self) -> u8 {
    unsafe { accessor!(compatLookupMods -> u8, (*self.base.reply)) }
  }

  pub fn ptrBtnState(&mut self) -> u16 {
    unsafe { accessor!(ptrBtnState -> u16, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetStateCookie<'s>, mk_reply_xcb_xkb_get_state_reply_t, GetStateReply, xcb_xkb_get_state_reply);

pub fn LatchLockStateChecked<'r> (c : &'r Connection,
                              deviceSpec : DeviceSpec,
                              affectModLocks : u8,
                              modLocks : u8,
                              lockGroup : u8,
                              groupLock : u8,
                              affectModLatches : u8,
                              latchGroup : u8,
                              groupLatch : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xkb_latch_lock_state_checked(c.get_raw_conn(),
        deviceSpec as xcb_xkb_device_spec_t, //1
        affectModLocks as u8, //2
        modLocks as u8, //3
        lockGroup as u8, //4
        groupLock as u8, //5
        affectModLatches as u8, //6
        latchGroup as u8, //7
        groupLatch as u16); //8
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn LatchLockState<'r> (c : &'r Connection,
                       deviceSpec : DeviceSpec,
                       affectModLocks : u8,
                       modLocks : u8,
                       lockGroup : u8,
                       groupLock : u8,
                       affectModLatches : u8,
                       latchGroup : u8,
                       groupLatch : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xkb_latch_lock_state(c.get_raw_conn(),
        deviceSpec as xcb_xkb_device_spec_t, //1
        affectModLocks as u8, //2
        modLocks as u8, //3
        lockGroup as u8, //4
        groupLock as u8, //5
        affectModLatches as u8, //6
        latchGroup as u8, //7
        groupLatch as u16); //8
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetControls<'r> (c : &'r Connection,
                    deviceSpec : DeviceSpec) -> GetControlsCookie<'r> {
  unsafe {
    let cookie = xcb_xkb_get_controls(c.get_raw_conn(),
        deviceSpec as xcb_xkb_device_spec_t); //1
    GetControlsCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetControlsUnchecked<'r> (c : &'r Connection,
                             deviceSpec : DeviceSpec) -> GetControlsCookie<'r> {
  unsafe {
    let cookie = xcb_xkb_get_controls_unchecked(c.get_raw_conn(),
        deviceSpec as xcb_xkb_device_spec_t); //1
    GetControlsCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetControlsReply {
  pub fn deviceID(&mut self) -> u8 {
    unsafe { accessor!(deviceID -> u8, (*self.base.reply)) }
  }

  pub fn mouseKeysDfltBtn(&mut self) -> u8 {
    unsafe { accessor!(mouseKeysDfltBtn -> u8, (*self.base.reply)) }
  }

  pub fn numGroups(&mut self) -> u8 {
    unsafe { accessor!(numGroups -> u8, (*self.base.reply)) }
  }

  pub fn groupsWrap(&mut self) -> u8 {
    unsafe { accessor!(groupsWrap -> u8, (*self.base.reply)) }
  }

  pub fn internalModsMask(&mut self) -> u8 {
    unsafe { accessor!(internalModsMask -> u8, (*self.base.reply)) }
  }

  pub fn ignoreLockModsMask(&mut self) -> u8 {
    unsafe { accessor!(ignoreLockModsMask -> u8, (*self.base.reply)) }
  }

  pub fn internalModsRealMods(&mut self) -> u8 {
    unsafe { accessor!(internalModsRealMods -> u8, (*self.base.reply)) }
  }

  pub fn ignoreLockModsRealMods(&mut self) -> u8 {
    unsafe { accessor!(ignoreLockModsRealMods -> u8, (*self.base.reply)) }
  }

  pub fn internalModsVmods(&mut self) -> u16 {
    unsafe { accessor!(internalModsVmods -> u16, (*self.base.reply)) }
  }

  pub fn ignoreLockModsVmods(&mut self) -> u16 {
    unsafe { accessor!(ignoreLockModsVmods -> u16, (*self.base.reply)) }
  }

  pub fn repeatDelay(&mut self) -> u16 {
    unsafe { accessor!(repeatDelay -> u16, (*self.base.reply)) }
  }

  pub fn repeatInterval(&mut self) -> u16 {
    unsafe { accessor!(repeatInterval -> u16, (*self.base.reply)) }
  }

  pub fn slowKeysDelay(&mut self) -> u16 {
    unsafe { accessor!(slowKeysDelay -> u16, (*self.base.reply)) }
  }

  pub fn debounceDelay(&mut self) -> u16 {
    unsafe { accessor!(debounceDelay -> u16, (*self.base.reply)) }
  }

  pub fn mouseKeysDelay(&mut self) -> u16 {
    unsafe { accessor!(mouseKeysDelay -> u16, (*self.base.reply)) }
  }

  pub fn mouseKeysInterval(&mut self) -> u16 {
    unsafe { accessor!(mouseKeysInterval -> u16, (*self.base.reply)) }
  }

  pub fn mouseKeysTimeToMax(&mut self) -> u16 {
    unsafe { accessor!(mouseKeysTimeToMax -> u16, (*self.base.reply)) }
  }

  pub fn mouseKeysMaxSpeed(&mut self) -> u16 {
    unsafe { accessor!(mouseKeysMaxSpeed -> u16, (*self.base.reply)) }
  }

  pub fn mouseKeysCurve(&mut self) -> i16 {
    unsafe { accessor!(mouseKeysCurve -> i16, (*self.base.reply)) }
  }

  pub fn accessXOption(&self) -> AxOption {
    unsafe { mem::transmute((*self.base.reply).accessXOption) }
  }
  pub fn accessXTimeout(&mut self) -> u16 {
    unsafe { accessor!(accessXTimeout -> u16, (*self.base.reply)) }
  }

  pub fn accessXTimeoutOptionsMask(&self) -> AxOption {
    unsafe { mem::transmute((*self.base.reply).accessXTimeoutOptionsMask) }
  }
  pub fn accessXTimeoutOptionsValues(&self) -> AxOption {
    unsafe { mem::transmute((*self.base.reply).accessXTimeoutOptionsValues) }
  }
  pub fn accessXTimeoutMask(&mut self) -> u32 {
    unsafe { accessor!(accessXTimeoutMask -> u32, (*self.base.reply)) }
  }

  pub fn accessXTimeoutValues(&mut self) -> u32 {
    unsafe { accessor!(accessXTimeoutValues -> u32, (*self.base.reply)) }
  }

  pub fn enabledControls(&mut self) -> u32 {
    unsafe { accessor!(enabledControls -> u32, (*self.base.reply)) }
  }

  pub fn perKeyRepeat(&self) -> Vec<u8> {
    unsafe { ((*self.base.reply).perKeyRepeat).to_vec() }
  }

}
impl_reply_cookie!(GetControlsCookie<'s>, mk_reply_xcb_xkb_get_controls_reply_t, GetControlsReply, xcb_xkb_get_controls_reply);

pub fn SetControlsChecked<'r> (c : &'r Connection,
                           deviceSpec : DeviceSpec,
                           affectInternalRealMods : u8,
                           internalRealMods : u8,
                           affectIgnoreLockRealMods : u8,
                           ignoreLockRealMods : u8,
                           affectInternalVirtualMods : u16,
                           internalVirtualMods : u16,
                           affectIgnoreLockVirtualMods : u16,
                           ignoreLockVirtualMods : u16,
                           mouseKeysDfltBtn : u8,
                           groupsWrap : u8,
                           accessXOptions : AxOption,
                           affectEnabledControls : u32,
                           enabledControls : u32,
                           changeControls : u32,
                           repeatDelay : u16,
                           repeatInterval : u16,
                           slowKeysDelay : u16,
                           debounceDelay : u16,
                           mouseKeysDelay : u16,
                           mouseKeysInterval : u16,
                           mouseKeysTimeToMax : u16,
                           mouseKeysMaxSpeed : u16,
                           mouseKeysCurve : i16,
                           accessXTimeout : u16,
                           accessXTimeoutMask : u32,
                           accessXTimeoutValues : u32,
                           accessXTimeoutOptionsMask : AxOption,
                           accessXTimeoutOptionsValues : AxOption,
                           perKeyRepeat : &[u8]) -> base::VoidCookie<'r> {
  unsafe {
    let perKeyRepeat_ptr = perKeyRepeat.as_ptr();
    let cookie = xcb_xkb_set_controls_checked(c.get_raw_conn(),
        deviceSpec as xcb_xkb_device_spec_t, //1
        affectInternalRealMods as u8, //2
        internalRealMods as u8, //3
        affectIgnoreLockRealMods as u8, //4
        ignoreLockRealMods as u8, //5
        affectInternalVirtualMods as u16, //6
        internalVirtualMods as u16, //7
        affectIgnoreLockVirtualMods as u16, //8
        ignoreLockVirtualMods as u16, //9
        mouseKeysDfltBtn as u8, //10
        groupsWrap as u8, //11
        accessXOptions.base.strct, //12
        affectEnabledControls as u32, //13
        enabledControls as u32, //14
        changeControls as u32, //15
        repeatDelay as u16, //16
        repeatInterval as u16, //17
        slowKeysDelay as u16, //18
        debounceDelay as u16, //19
        mouseKeysDelay as u16, //20
        mouseKeysInterval as u16, //21
        mouseKeysTimeToMax as u16, //22
        mouseKeysMaxSpeed as u16, //23
        mouseKeysCurve as i16, //24
        accessXTimeout as u16, //25
        accessXTimeoutMask as u32, //26
        accessXTimeoutValues as u32, //27
        accessXTimeoutOptionsMask.base.strct, //28
        accessXTimeoutOptionsValues.base.strct, //29
        perKeyRepeat_ptr as *mut u8); //30
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn SetControls<'r> (c : &'r Connection,
                    deviceSpec : DeviceSpec,
                    affectInternalRealMods : u8,
                    internalRealMods : u8,
                    affectIgnoreLockRealMods : u8,
                    ignoreLockRealMods : u8,
                    affectInternalVirtualMods : u16,
                    internalVirtualMods : u16,
                    affectIgnoreLockVirtualMods : u16,
                    ignoreLockVirtualMods : u16,
                    mouseKeysDfltBtn : u8,
                    groupsWrap : u8,
                    accessXOptions : AxOption,
                    affectEnabledControls : u32,
                    enabledControls : u32,
                    changeControls : u32,
                    repeatDelay : u16,
                    repeatInterval : u16,
                    slowKeysDelay : u16,
                    debounceDelay : u16,
                    mouseKeysDelay : u16,
                    mouseKeysInterval : u16,
                    mouseKeysTimeToMax : u16,
                    mouseKeysMaxSpeed : u16,
                    mouseKeysCurve : i16,
                    accessXTimeout : u16,
                    accessXTimeoutMask : u32,
                    accessXTimeoutValues : u32,
                    accessXTimeoutOptionsMask : AxOption,
                    accessXTimeoutOptionsValues : AxOption,
                    perKeyRepeat : &[u8]) -> base::VoidCookie<'r> {
  unsafe {
    let perKeyRepeat_ptr = perKeyRepeat.as_ptr();
    let cookie = xcb_xkb_set_controls(c.get_raw_conn(),
        deviceSpec as xcb_xkb_device_spec_t, //1
        affectInternalRealMods as u8, //2
        internalRealMods as u8, //3
        affectIgnoreLockRealMods as u8, //4
        ignoreLockRealMods as u8, //5
        affectInternalVirtualMods as u16, //6
        internalVirtualMods as u16, //7
        affectIgnoreLockVirtualMods as u16, //8
        ignoreLockVirtualMods as u16, //9
        mouseKeysDfltBtn as u8, //10
        groupsWrap as u8, //11
        accessXOptions.base.strct, //12
        affectEnabledControls as u32, //13
        enabledControls as u32, //14
        changeControls as u32, //15
        repeatDelay as u16, //16
        repeatInterval as u16, //17
        slowKeysDelay as u16, //18
        debounceDelay as u16, //19
        mouseKeysDelay as u16, //20
        mouseKeysInterval as u16, //21
        mouseKeysTimeToMax as u16, //22
        mouseKeysMaxSpeed as u16, //23
        mouseKeysCurve as i16, //24
        accessXTimeout as u16, //25
        accessXTimeoutMask as u32, //26
        accessXTimeoutValues as u32, //27
        accessXTimeoutOptionsMask.base.strct, //28
        accessXTimeoutOptionsValues.base.strct, //29
        perKeyRepeat_ptr as *mut u8); //30
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetMapMap {
  pub fn types_rtrn(&mut self) -> KeyTypeIterator {
    unsafe { accessor!(KeyTypeIterator, xcb_xkb_get_map_map_types_rtrn_iterator, self.base.strct) }
  }

  pub fn syms_rtrn(&mut self) -> KeySymMapIterator {
    unsafe { accessor!(KeySymMapIterator, xcb_xkb_get_map_map_syms_rtrn_iterator, self.base.strct) }
  }

  pub fn acts_rtrn_count(&mut self) -> Vec<xproto::Keycode> {
    unsafe { accessor!(xproto::Keycode, xcb_xkb_get_map_map_acts_rtrn_count_length, xcb_xkb_get_map_map_acts_rtrn_count, self.base.strct) }
  }

  pub fn acts_rtrn_acts(&mut self) -> ActionIterator {
    unsafe { accessor!(ActionIterator, xcb_xkb_get_map_map_acts_rtrn_acts_iterator, self.base.strct) }
  }

  pub fn behaviors_rtrn(&mut self) -> SetBehaviorIterator {
    unsafe { accessor!(SetBehaviorIterator, xcb_xkb_get_map_map_behaviors_rtrn_iterator, self.base.strct) }
  }

  pub fn vmods_rtrn(&mut self) -> Vec<xproto::Keycode> {
    unsafe { accessor!(xproto::Keycode, xcb_xkb_get_map_map_vmods_rtrn_length, xcb_xkb_get_map_map_vmods_rtrn, self.base.strct) }
  }

  pub fn explicit_rtrn(&mut self) -> SetExplicitIterator {
    unsafe { accessor!(SetExplicitIterator, xcb_xkb_get_map_map_explicit_rtrn_iterator, self.base.strct) }
  }

  pub fn modmap_rtrn(&mut self) -> KeyModMapIterator {
    unsafe { accessor!(KeyModMapIterator, xcb_xkb_get_map_map_modmap_rtrn_iterator, self.base.strct) }
  }

  pub fn vmodmap_rtrn(&mut self) -> KeyVModMapIterator {
    unsafe { accessor!(KeyVModMapIterator, xcb_xkb_get_map_map_vmodmap_rtrn_iterator, self.base.strct) }
  }

}
pub struct GetMapReply { base:  base::Reply<xcb_xkb_get_map_reply_t> }
fn mk_reply_xcb_xkb_get_map_reply_t(reply:*mut xcb_xkb_get_map_reply_t) -> GetMapReply { GetMapReply { base : base::mk_reply(reply) } }
pub fn GetMap<'r> (c : &'r Connection,
               deviceSpec : DeviceSpec,
               full : u16,
               partial : u16,
               firstType : u8,
               nTypes : u8,
               firstKeySym : xproto::Keycode,
               nKeySyms : u8,
               firstKeyAction : xproto::Keycode,
               nKeyActions : u8,
               firstKeyBehavior : xproto::Keycode,
               nKeyBehaviors : u8,
               virtualMods : u16,
               firstKeyExplicit : xproto::Keycode,
               nKeyExplicit : u8,
               firstModMapKey : xproto::Keycode,
               nModMapKeys : u8,
               firstVModMapKey : xproto::Keycode,
               nVModMapKeys : u8) -> GetMapCookie<'r> {
  unsafe {
    let cookie = xcb_xkb_get_map(c.get_raw_conn(),
        deviceSpec as xcb_xkb_device_spec_t, //1
        full as u16, //2
        partial as u16, //3
        firstType as u8, //4
        nTypes as u8, //5
        firstKeySym as ffi::xproto::xcb_keycode_t, //6
        nKeySyms as u8, //7
        firstKeyAction as ffi::xproto::xcb_keycode_t, //8
        nKeyActions as u8, //9
        firstKeyBehavior as ffi::xproto::xcb_keycode_t, //10
        nKeyBehaviors as u8, //11
        virtualMods as u16, //12
        firstKeyExplicit as ffi::xproto::xcb_keycode_t, //13
        nKeyExplicit as u8, //14
        firstModMapKey as ffi::xproto::xcb_keycode_t, //15
        nModMapKeys as u8, //16
        firstVModMapKey as ffi::xproto::xcb_keycode_t, //17
        nVModMapKeys as u8); //18
    GetMapCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetMapUnchecked<'r> (c : &'r Connection,
                        deviceSpec : DeviceSpec,
                        full : u16,
                        partial : u16,
                        firstType : u8,
                        nTypes : u8,
                        firstKeySym : xproto::Keycode,
                        nKeySyms : u8,
                        firstKeyAction : xproto::Keycode,
                        nKeyActions : u8,
                        firstKeyBehavior : xproto::Keycode,
                        nKeyBehaviors : u8,
                        virtualMods : u16,
                        firstKeyExplicit : xproto::Keycode,
                        nKeyExplicit : u8,
                        firstModMapKey : xproto::Keycode,
                        nModMapKeys : u8,
                        firstVModMapKey : xproto::Keycode,
                        nVModMapKeys : u8) -> GetMapCookie<'r> {
  unsafe {
    let cookie = xcb_xkb_get_map_unchecked(c.get_raw_conn(),
        deviceSpec as xcb_xkb_device_spec_t, //1
        full as u16, //2
        partial as u16, //3
        firstType as u8, //4
        nTypes as u8, //5
        firstKeySym as ffi::xproto::xcb_keycode_t, //6
        nKeySyms as u8, //7
        firstKeyAction as ffi::xproto::xcb_keycode_t, //8
        nKeyActions as u8, //9
        firstKeyBehavior as ffi::xproto::xcb_keycode_t, //10
        nKeyBehaviors as u8, //11
        virtualMods as u16, //12
        firstKeyExplicit as ffi::xproto::xcb_keycode_t, //13
        nKeyExplicit as u8, //14
        firstModMapKey as ffi::xproto::xcb_keycode_t, //15
        nModMapKeys as u8, //16
        firstVModMapKey as ffi::xproto::xcb_keycode_t, //17
        nVModMapKeys as u8); //18
    GetMapCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetMapReply {
  pub fn deviceID(&mut self) -> u8 {
    unsafe { accessor!(deviceID -> u8, (*self.base.reply)) }
  }

  pub fn minKeyCode(&mut self) -> xproto::Keycode {
    unsafe { accessor!(minKeyCode -> xproto::Keycode, (*self.base.reply)) }
  }

  pub fn maxKeyCode(&mut self) -> xproto::Keycode {
    unsafe { accessor!(maxKeyCode -> xproto::Keycode, (*self.base.reply)) }
  }

  pub fn present(&mut self) -> u16 {
    unsafe { accessor!(present -> u16, (*self.base.reply)) }
  }

  pub fn firstType(&mut self) -> u8 {
    unsafe { accessor!(firstType -> u8, (*self.base.reply)) }
  }

  pub fn nTypes(&mut self) -> u8 {
    unsafe { accessor!(nTypes -> u8, (*self.base.reply)) }
  }

  pub fn totalTypes(&mut self) -> u8 {
    unsafe { accessor!(totalTypes -> u8, (*self.base.reply)) }
  }

  pub fn firstKeySym(&mut self) -> xproto::Keycode {
    unsafe { accessor!(firstKeySym -> xproto::Keycode, (*self.base.reply)) }
  }

  pub fn totalSyms(&mut self) -> u16 {
    unsafe { accessor!(totalSyms -> u16, (*self.base.reply)) }
  }

  pub fn nKeySyms(&mut self) -> u8 {
    unsafe { accessor!(nKeySyms -> u8, (*self.base.reply)) }
  }

  pub fn firstKeyAction(&mut self) -> xproto::Keycode {
    unsafe { accessor!(firstKeyAction -> xproto::Keycode, (*self.base.reply)) }
  }

  pub fn totalActions(&mut self) -> u16 {
    unsafe { accessor!(totalActions -> u16, (*self.base.reply)) }
  }

  pub fn nKeyActions(&mut self) -> u8 {
    unsafe { accessor!(nKeyActions -> u8, (*self.base.reply)) }
  }

  pub fn firstKeyBehavior(&mut self) -> xproto::Keycode {
    unsafe { accessor!(firstKeyBehavior -> xproto::Keycode, (*self.base.reply)) }
  }

  pub fn nKeyBehaviors(&mut self) -> u8 {
    unsafe { accessor!(nKeyBehaviors -> u8, (*self.base.reply)) }
  }

  pub fn totalKeyBehaviors(&mut self) -> u8 {
    unsafe { accessor!(totalKeyBehaviors -> u8, (*self.base.reply)) }
  }

  pub fn firstKeyExplicit(&mut self) -> xproto::Keycode {
    unsafe { accessor!(firstKeyExplicit -> xproto::Keycode, (*self.base.reply)) }
  }

  pub fn nKeyExplicit(&mut self) -> u8 {
    unsafe { accessor!(nKeyExplicit -> u8, (*self.base.reply)) }
  }

  pub fn totalKeyExplicit(&mut self) -> u8 {
    unsafe { accessor!(totalKeyExplicit -> u8, (*self.base.reply)) }
  }

  pub fn firstModMapKey(&mut self) -> xproto::Keycode {
    unsafe { accessor!(firstModMapKey -> xproto::Keycode, (*self.base.reply)) }
  }

  pub fn nModMapKeys(&mut self) -> u8 {
    unsafe { accessor!(nModMapKeys -> u8, (*self.base.reply)) }
  }

  pub fn totalModMapKeys(&mut self) -> u8 {
    unsafe { accessor!(totalModMapKeys -> u8, (*self.base.reply)) }
  }

  pub fn firstVModMapKey(&mut self) -> xproto::Keycode {
    unsafe { accessor!(firstVModMapKey -> xproto::Keycode, (*self.base.reply)) }
  }

  pub fn nVModMapKeys(&mut self) -> u8 {
    unsafe { accessor!(nVModMapKeys -> u8, (*self.base.reply)) }
  }

  pub fn totalVModMapKeys(&mut self) -> u8 {
    unsafe { accessor!(totalVModMapKeys -> u8, (*self.base.reply)) }
  }

  pub fn virtualMods(&mut self) -> u16 {
    unsafe { accessor!(virtualMods -> u16, (*self.base.reply)) }
  }

  pub fn map(&self) -> GetMapMap {
    unsafe { mem::transmute((*self.base.reply).map) }
  }
}
impl_reply_cookie!(GetMapCookie<'s>, mk_reply_xcb_xkb_get_map_reply_t, GetMapReply, xcb_xkb_get_map_reply);

pub struct SetMapValues {pub base : base::Struct<xcb_xkb_set_map_values_t> }


impl SetMapValues {
  pub fn types(&mut self) -> SetKeyTypeIterator {
    unsafe { accessor!(SetKeyTypeIterator, xcb_xkb_set_map_values_types_iterator, self.base.strct) }
  }

  pub fn syms(&mut self) -> KeySymMapIterator {
    unsafe { accessor!(KeySymMapIterator, xcb_xkb_set_map_values_syms_iterator, self.base.strct) }
  }

  pub fn actionsCount(&mut self) -> Vec<xproto::Keycode> {
    unsafe { accessor!(xproto::Keycode, xcb_xkb_set_map_values_actions_count_length, xcb_xkb_set_map_values_actions_count, self.base.strct) }
  }

  pub fn actions(&mut self) -> ActionIterator {
    unsafe { accessor!(ActionIterator, xcb_xkb_set_map_values_actions_iterator, self.base.strct) }
  }

  pub fn behaviors(&mut self) -> SetBehaviorIterator {
    unsafe { accessor!(SetBehaviorIterator, xcb_xkb_set_map_values_behaviors_iterator, self.base.strct) }
  }

  pub fn vmods(&mut self) -> Vec<xproto::Keycode> {
    unsafe { accessor!(xproto::Keycode, xcb_xkb_set_map_values_vmods_length, xcb_xkb_set_map_values_vmods, self.base.strct) }
  }

  pub fn explicit(&mut self) -> SetExplicitIterator {
    unsafe { accessor!(SetExplicitIterator, xcb_xkb_set_map_values_explicit_iterator, self.base.strct) }
  }

  pub fn modmap(&mut self) -> KeyModMapIterator {
    unsafe { accessor!(KeyModMapIterator, xcb_xkb_set_map_values_modmap_iterator, self.base.strct) }
  }

  pub fn vmodmap(&mut self) -> KeyVModMapIterator {
    unsafe { accessor!(KeyVModMapIterator, xcb_xkb_set_map_values_vmodmap_iterator, self.base.strct) }
  }

}
pub fn SetMapChecked<'r> (c : &'r Connection,
                      deviceSpec : DeviceSpec,
                      present : u16,
                      flags : u16,
                      minKeyCode : xproto::Keycode,
                      maxKeyCode : xproto::Keycode,
                      firstType : u8,
                      nTypes : u8,
                      firstKeySym : xproto::Keycode,
                      nKeySyms : u8,
                      totalSyms : u16,
                      firstKeyAction : xproto::Keycode,
                      nKeyActions : u8,
                      totalActions : u16,
                      firstKeyBehavior : xproto::Keycode,
                      nKeyBehaviors : u8,
                      totalKeyBehaviors : u8,
                      firstKeyExplicit : xproto::Keycode,
                      nKeyExplicit : u8,
                      totalKeyExplicit : u8,
                      firstModMapKey : xproto::Keycode,
                      nModMapKeys : u8,
                      totalModMapKeys : u8,
                      firstVModMapKey : xproto::Keycode,
                      nVModMapKeys : u8,
                      totalVModMapKeys : u8,
                      virtualMods : u16,
                      values : SetMapValues) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xkb_set_map_checked(c.get_raw_conn(),
        deviceSpec as xcb_xkb_device_spec_t, //1
        present as u16, //2
        flags as u16, //3
        minKeyCode as ffi::xproto::xcb_keycode_t, //4
        maxKeyCode as ffi::xproto::xcb_keycode_t, //5
        firstType as u8, //6
        nTypes as u8, //7
        firstKeySym as ffi::xproto::xcb_keycode_t, //8
        nKeySyms as u8, //9
        totalSyms as u16, //10
        firstKeyAction as ffi::xproto::xcb_keycode_t, //11
        nKeyActions as u8, //12
        totalActions as u16, //13
        firstKeyBehavior as ffi::xproto::xcb_keycode_t, //14
        nKeyBehaviors as u8, //15
        totalKeyBehaviors as u8, //16
        firstKeyExplicit as ffi::xproto::xcb_keycode_t, //17
        nKeyExplicit as u8, //18
        totalKeyExplicit as u8, //19
        firstModMapKey as ffi::xproto::xcb_keycode_t, //20
        nModMapKeys as u8, //21
        totalModMapKeys as u8, //22
        firstVModMapKey as ffi::xproto::xcb_keycode_t, //23
        nVModMapKeys as u8, //24
        totalVModMapKeys as u8, //25
        virtualMods as u16, //26
        values.base.strct); //27
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn SetMap<'r> (c : &'r Connection,
               deviceSpec : DeviceSpec,
               present : u16,
               flags : u16,
               minKeyCode : xproto::Keycode,
               maxKeyCode : xproto::Keycode,
               firstType : u8,
               nTypes : u8,
               firstKeySym : xproto::Keycode,
               nKeySyms : u8,
               totalSyms : u16,
               firstKeyAction : xproto::Keycode,
               nKeyActions : u8,
               totalActions : u16,
               firstKeyBehavior : xproto::Keycode,
               nKeyBehaviors : u8,
               totalKeyBehaviors : u8,
               firstKeyExplicit : xproto::Keycode,
               nKeyExplicit : u8,
               totalKeyExplicit : u8,
               firstModMapKey : xproto::Keycode,
               nModMapKeys : u8,
               totalModMapKeys : u8,
               firstVModMapKey : xproto::Keycode,
               nVModMapKeys : u8,
               totalVModMapKeys : u8,
               virtualMods : u16,
               values : SetMapValues) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xkb_set_map(c.get_raw_conn(),
        deviceSpec as xcb_xkb_device_spec_t, //1
        present as u16, //2
        flags as u16, //3
        minKeyCode as ffi::xproto::xcb_keycode_t, //4
        maxKeyCode as ffi::xproto::xcb_keycode_t, //5
        firstType as u8, //6
        nTypes as u8, //7
        firstKeySym as ffi::xproto::xcb_keycode_t, //8
        nKeySyms as u8, //9
        totalSyms as u16, //10
        firstKeyAction as ffi::xproto::xcb_keycode_t, //11
        nKeyActions as u8, //12
        totalActions as u16, //13
        firstKeyBehavior as ffi::xproto::xcb_keycode_t, //14
        nKeyBehaviors as u8, //15
        totalKeyBehaviors as u8, //16
        firstKeyExplicit as ffi::xproto::xcb_keycode_t, //17
        nKeyExplicit as u8, //18
        totalKeyExplicit as u8, //19
        firstModMapKey as ffi::xproto::xcb_keycode_t, //20
        nModMapKeys as u8, //21
        totalModMapKeys as u8, //22
        firstVModMapKey as ffi::xproto::xcb_keycode_t, //23
        nVModMapKeys as u8, //24
        totalVModMapKeys as u8, //25
        virtualMods as u16, //26
        values.base.strct); //27
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn SetMapAuxChecked<'r> (c : &'r Connection,
                         deviceSpec : DeviceSpec,
                         present : u16,
                         flags : u16,
                         minKeyCode : xproto::Keycode,
                         maxKeyCode : xproto::Keycode,
                         firstType : u8,
                         nTypes : u8,
                         firstKeySym : xproto::Keycode,
                         nKeySyms : u8,
                         totalSyms : u16,
                         firstKeyAction : xproto::Keycode,
                         nKeyActions : u8,
                         totalActions : u16,
                         firstKeyBehavior : xproto::Keycode,
                         nKeyBehaviors : u8,
                         totalKeyBehaviors : u8,
                         firstKeyExplicit : xproto::Keycode,
                         nKeyExplicit : u8,
                         totalKeyExplicit : u8,
                         firstModMapKey : xproto::Keycode,
                         nModMapKeys : u8,
                         totalModMapKeys : u8,
                         firstVModMapKey : xproto::Keycode,
                         nVModMapKeys : u8,
                         totalVModMapKeys : u8,
                         virtualMods : u16,
                         values : SetMapValues) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xkb_set_map_aux_checked(c.get_raw_conn(),
        deviceSpec as xcb_xkb_device_spec_t, //1
        present as u16, //2
        flags as u16, //3
        minKeyCode as ffi::xproto::xcb_keycode_t, //4
        maxKeyCode as ffi::xproto::xcb_keycode_t, //5
        firstType as u8, //6
        nTypes as u8, //7
        firstKeySym as ffi::xproto::xcb_keycode_t, //8
        nKeySyms as u8, //9
        totalSyms as u16, //10
        firstKeyAction as ffi::xproto::xcb_keycode_t, //11
        nKeyActions as u8, //12
        totalActions as u16, //13
        firstKeyBehavior as ffi::xproto::xcb_keycode_t, //14
        nKeyBehaviors as u8, //15
        totalKeyBehaviors as u8, //16
        firstKeyExplicit as ffi::xproto::xcb_keycode_t, //17
        nKeyExplicit as u8, //18
        totalKeyExplicit as u8, //19
        firstModMapKey as ffi::xproto::xcb_keycode_t, //20
        nModMapKeys as u8, //21
        totalModMapKeys as u8, //22
        firstVModMapKey as ffi::xproto::xcb_keycode_t, //23
        nVModMapKeys as u8, //24
        totalVModMapKeys as u8, //25
        virtualMods as u16, //26
        values.base.strct); //27
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn SetMapAux<'r> (c : &'r Connection,
                  deviceSpec : DeviceSpec,
                  present : u16,
                  flags : u16,
                  minKeyCode : xproto::Keycode,
                  maxKeyCode : xproto::Keycode,
                  firstType : u8,
                  nTypes : u8,
                  firstKeySym : xproto::Keycode,
                  nKeySyms : u8,
                  totalSyms : u16,
                  firstKeyAction : xproto::Keycode,
                  nKeyActions : u8,
                  totalActions : u16,
                  firstKeyBehavior : xproto::Keycode,
                  nKeyBehaviors : u8,
                  totalKeyBehaviors : u8,
                  firstKeyExplicit : xproto::Keycode,
                  nKeyExplicit : u8,
                  totalKeyExplicit : u8,
                  firstModMapKey : xproto::Keycode,
                  nModMapKeys : u8,
                  totalModMapKeys : u8,
                  firstVModMapKey : xproto::Keycode,
                  nVModMapKeys : u8,
                  totalVModMapKeys : u8,
                  virtualMods : u16,
                  values : SetMapValues) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xkb_set_map_aux(c.get_raw_conn(),
        deviceSpec as xcb_xkb_device_spec_t, //1
        present as u16, //2
        flags as u16, //3
        minKeyCode as ffi::xproto::xcb_keycode_t, //4
        maxKeyCode as ffi::xproto::xcb_keycode_t, //5
        firstType as u8, //6
        nTypes as u8, //7
        firstKeySym as ffi::xproto::xcb_keycode_t, //8
        nKeySyms as u8, //9
        totalSyms as u16, //10
        firstKeyAction as ffi::xproto::xcb_keycode_t, //11
        nKeyActions as u8, //12
        totalActions as u16, //13
        firstKeyBehavior as ffi::xproto::xcb_keycode_t, //14
        nKeyBehaviors as u8, //15
        totalKeyBehaviors as u8, //16
        firstKeyExplicit as ffi::xproto::xcb_keycode_t, //17
        nKeyExplicit as u8, //18
        totalKeyExplicit as u8, //19
        firstModMapKey as ffi::xproto::xcb_keycode_t, //20
        nModMapKeys as u8, //21
        totalModMapKeys as u8, //22
        firstVModMapKey as ffi::xproto::xcb_keycode_t, //23
        nVModMapKeys as u8, //24
        totalVModMapKeys as u8, //25
        virtualMods as u16, //26
        values.base.strct); //27
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub struct GetCompatMapReply { base:  base::Reply<xcb_xkb_get_compat_map_reply_t> }
fn mk_reply_xcb_xkb_get_compat_map_reply_t(reply:*mut xcb_xkb_get_compat_map_reply_t) -> GetCompatMapReply { GetCompatMapReply { base : base::mk_reply(reply) } }
pub fn GetCompatMap<'r> (c : &'r Connection,
                     deviceSpec : DeviceSpec,
                     groups : u8,
                     getAllSI : u8,
                     firstSI : u16,
                     nSI : u16) -> GetCompatMapCookie<'r> {
  unsafe {
    let cookie = xcb_xkb_get_compat_map(c.get_raw_conn(),
        deviceSpec as xcb_xkb_device_spec_t, //1
        groups as u8, //2
        getAllSI as u8, //3
        firstSI as u16, //4
        nSI as u16); //5
    GetCompatMapCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetCompatMapUnchecked<'r> (c : &'r Connection,
                              deviceSpec : DeviceSpec,
                              groups : u8,
                              getAllSI : u8,
                              firstSI : u16,
                              nSI : u16) -> GetCompatMapCookie<'r> {
  unsafe {
    let cookie = xcb_xkb_get_compat_map_unchecked(c.get_raw_conn(),
        deviceSpec as xcb_xkb_device_spec_t, //1
        groups as u8, //2
        getAllSI as u8, //3
        firstSI as u16, //4
        nSI as u16); //5
    GetCompatMapCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetCompatMapReply {
  pub fn deviceID(&mut self) -> u8 {
    unsafe { accessor!(deviceID -> u8, (*self.base.reply)) }
  }

  pub fn firstSIRtrn(&mut self) -> u16 {
    unsafe { accessor!(firstSIRtrn -> u16, (*self.base.reply)) }
  }

  pub fn nTotalSI(&mut self) -> u16 {
    unsafe { accessor!(nTotalSI -> u16, (*self.base.reply)) }
  }

  pub fn si_rtrn(&mut self) -> Vec<u8> {
    unsafe { accessor!(u8, xcb_xkb_get_compat_map_si_rtrn_length, xcb_xkb_get_compat_map_si_rtrn, (*self.base.reply)) }
  }

  pub fn group_rtrn(&mut self) -> ModDefIterator {
    unsafe { accessor!(ModDefIterator, xcb_xkb_get_compat_map_group_rtrn_iterator, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetCompatMapCookie<'s>, mk_reply_xcb_xkb_get_compat_map_reply_t, GetCompatMapReply, xcb_xkb_get_compat_map_reply);

pub fn SetCompatMapChecked<'r> (c : &'r Connection,
                            deviceSpec : DeviceSpec,
                            recomputeActions : u8,
                            truncateSI : u8,
                            firstSI : u16,
                            si : &[u8],
                            groupMaps : &[ModDef]) -> base::VoidCookie<'r> {
  unsafe {
    let si_len = si.len();
    let si_ptr = si.as_ptr();
    let groupMaps_len = groupMaps.len();
    let groupMaps_ptr = groupMaps.as_ptr();
    let cookie = xcb_xkb_set_compat_map_checked(c.get_raw_conn(),
        deviceSpec as xcb_xkb_device_spec_t, //1
        recomputeActions as u8, //2
        truncateSI as u8, //3
        groupMaps_len as u8, //4
        firstSI as u16, //5
        si_len as u16, //6
        si_ptr as *mut u8, //7
        groupMaps_ptr as *mut xcb_xkb_mod_def_t); //8
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn SetCompatMap<'r> (c : &'r Connection,
                     deviceSpec : DeviceSpec,
                     recomputeActions : u8,
                     truncateSI : u8,
                     firstSI : u16,
                     si : &[u8],
                     groupMaps : &[ModDef]) -> base::VoidCookie<'r> {
  unsafe {
    let si_len = si.len();
    let si_ptr = si.as_ptr();
    let groupMaps_len = groupMaps.len();
    let groupMaps_ptr = groupMaps.as_ptr();
    let cookie = xcb_xkb_set_compat_map(c.get_raw_conn(),
        deviceSpec as xcb_xkb_device_spec_t, //1
        recomputeActions as u8, //2
        truncateSI as u8, //3
        groupMaps_len as u8, //4
        firstSI as u16, //5
        si_len as u16, //6
        si_ptr as *mut u8, //7
        groupMaps_ptr as *mut xcb_xkb_mod_def_t); //8
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetIndicatorState<'r> (c : &'r Connection,
                          deviceSpec : DeviceSpec) -> GetIndicatorStateCookie<'r> {
  unsafe {
    let cookie = xcb_xkb_get_indicator_state(c.get_raw_conn(),
        deviceSpec as xcb_xkb_device_spec_t); //1
    GetIndicatorStateCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetIndicatorStateUnchecked<'r> (c : &'r Connection,
                                   deviceSpec : DeviceSpec) -> GetIndicatorStateCookie<'r> {
  unsafe {
    let cookie = xcb_xkb_get_indicator_state_unchecked(c.get_raw_conn(),
        deviceSpec as xcb_xkb_device_spec_t); //1
    GetIndicatorStateCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetIndicatorStateReply {
  pub fn deviceID(&mut self) -> u8 {
    unsafe { accessor!(deviceID -> u8, (*self.base.reply)) }
  }

  pub fn state(&mut self) -> u32 {
    unsafe { accessor!(state -> u32, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetIndicatorStateCookie<'s>, mk_reply_xcb_xkb_get_indicator_state_reply_t, GetIndicatorStateReply, xcb_xkb_get_indicator_state_reply);

pub struct GetIndicatorMapReply { base:  base::Reply<xcb_xkb_get_indicator_map_reply_t> }
fn mk_reply_xcb_xkb_get_indicator_map_reply_t(reply:*mut xcb_xkb_get_indicator_map_reply_t) -> GetIndicatorMapReply { GetIndicatorMapReply { base : base::mk_reply(reply) } }
pub fn GetIndicatorMap<'r> (c : &'r Connection,
                        deviceSpec : DeviceSpec,
                        which : u32) -> GetIndicatorMapCookie<'r> {
  unsafe {
    let cookie = xcb_xkb_get_indicator_map(c.get_raw_conn(),
        deviceSpec as xcb_xkb_device_spec_t, //1
        which as u32); //2
    GetIndicatorMapCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetIndicatorMapUnchecked<'r> (c : &'r Connection,
                                 deviceSpec : DeviceSpec,
                                 which : u32) -> GetIndicatorMapCookie<'r> {
  unsafe {
    let cookie = xcb_xkb_get_indicator_map_unchecked(c.get_raw_conn(),
        deviceSpec as xcb_xkb_device_spec_t, //1
        which as u32); //2
    GetIndicatorMapCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetIndicatorMapReply {
  pub fn deviceID(&mut self) -> u8 {
    unsafe { accessor!(deviceID -> u8, (*self.base.reply)) }
  }

  pub fn which(&mut self) -> u32 {
    unsafe { accessor!(which -> u32, (*self.base.reply)) }
  }

  pub fn realIndicators(&mut self) -> u32 {
    unsafe { accessor!(realIndicators -> u32, (*self.base.reply)) }
  }

  pub fn maps(&mut self) -> IndicatorMapIterator {
    unsafe { accessor!(IndicatorMapIterator, xcb_xkb_get_indicator_map_maps_iterator, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetIndicatorMapCookie<'s>, mk_reply_xcb_xkb_get_indicator_map_reply_t, GetIndicatorMapReply, xcb_xkb_get_indicator_map_reply);

pub fn SetIndicatorMapChecked<'r> (c : &'r Connection,
                               deviceSpec : DeviceSpec,
                               maps : &[IndicatorMap]) -> base::VoidCookie<'r> {
  unsafe {
    let maps_len = maps.len();
    let maps_ptr = maps.as_ptr();
    let cookie = xcb_xkb_set_indicator_map_checked(c.get_raw_conn(),
        deviceSpec as xcb_xkb_device_spec_t, //1
        maps_len as u32, //2
        maps_ptr as *mut xcb_xkb_indicator_map_t); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn SetIndicatorMap<'r> (c : &'r Connection,
                        deviceSpec : DeviceSpec,
                        maps : &[IndicatorMap]) -> base::VoidCookie<'r> {
  unsafe {
    let maps_len = maps.len();
    let maps_ptr = maps.as_ptr();
    let cookie = xcb_xkb_set_indicator_map(c.get_raw_conn(),
        deviceSpec as xcb_xkb_device_spec_t, //1
        maps_len as u32, //2
        maps_ptr as *mut xcb_xkb_indicator_map_t); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetNamedIndicator<'r> (c : &'r Connection,
                          deviceSpec : DeviceSpec,
                          ledClass : LedClassSpec,
                          ledID : IdSpec,
                          indicator : xproto::Atom) -> GetNamedIndicatorCookie<'r> {
  unsafe {
    let cookie = xcb_xkb_get_named_indicator(c.get_raw_conn(),
        deviceSpec as xcb_xkb_device_spec_t, //1
        ledClass as xcb_xkb_led_class_spec_t, //2
        ledID as xcb_xkb_id_spec_t, //3
        indicator as ffi::xproto::xcb_atom_t); //4
    GetNamedIndicatorCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetNamedIndicatorUnchecked<'r> (c : &'r Connection,
                                   deviceSpec : DeviceSpec,
                                   ledClass : LedClassSpec,
                                   ledID : IdSpec,
                                   indicator : xproto::Atom) -> GetNamedIndicatorCookie<'r> {
  unsafe {
    let cookie = xcb_xkb_get_named_indicator_unchecked(c.get_raw_conn(),
        deviceSpec as xcb_xkb_device_spec_t, //1
        ledClass as xcb_xkb_led_class_spec_t, //2
        ledID as xcb_xkb_id_spec_t, //3
        indicator as ffi::xproto::xcb_atom_t); //4
    GetNamedIndicatorCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetNamedIndicatorReply {
  pub fn deviceID(&mut self) -> u8 {
    unsafe { accessor!(deviceID -> u8, (*self.base.reply)) }
  }

  pub fn indicator(&mut self) -> xproto::Atom {
    unsafe { accessor!(indicator -> xproto::Atom, (*self.base.reply)) }
  }

  pub fn found(&mut self) -> u8 {
    unsafe { accessor!(found -> u8, (*self.base.reply)) }
  }

  pub fn on(&mut self) -> u8 {
    unsafe { accessor!(on -> u8, (*self.base.reply)) }
  }

  pub fn realIndicator(&mut self) -> u8 {
    unsafe { accessor!(realIndicator -> u8, (*self.base.reply)) }
  }

  pub fn ndx(&mut self) -> u8 {
    unsafe { accessor!(ndx -> u8, (*self.base.reply)) }
  }

  pub fn map_flags(&mut self) -> u8 {
    unsafe { accessor!(map_flags -> u8, (*self.base.reply)) }
  }

  pub fn map_whichGroups(&mut self) -> u8 {
    unsafe { accessor!(map_whichGroups -> u8, (*self.base.reply)) }
  }

  pub fn map_groups(&mut self) -> u8 {
    unsafe { accessor!(map_groups -> u8, (*self.base.reply)) }
  }

  pub fn map_whichMods(&mut self) -> u8 {
    unsafe { accessor!(map_whichMods -> u8, (*self.base.reply)) }
  }

  pub fn map_mods(&mut self) -> u8 {
    unsafe { accessor!(map_mods -> u8, (*self.base.reply)) }
  }

  pub fn map_realMods(&mut self) -> u8 {
    unsafe { accessor!(map_realMods -> u8, (*self.base.reply)) }
  }

  pub fn map_vmod(&mut self) -> u16 {
    unsafe { accessor!(map_vmod -> u16, (*self.base.reply)) }
  }

  pub fn map_ctrls(&mut self) -> u32 {
    unsafe { accessor!(map_ctrls -> u32, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetNamedIndicatorCookie<'s>, mk_reply_xcb_xkb_get_named_indicator_reply_t, GetNamedIndicatorReply, xcb_xkb_get_named_indicator_reply);

pub fn SetNamedIndicatorChecked<'r> (c : &'r Connection,
                                 deviceSpec : DeviceSpec,
                                 ledClass : LedClassSpec,
                                 ledID : IdSpec,
                                 indicator : xproto::Atom,
                                 setState : u8,
                                 on : u8,
                                 setMap : u8,
                                 createMap : u8,
                                 map_flags : u8,
                                 map_whichGroups : u8,
                                 map_groups : u8,
                                 map_whichMods : u8,
                                 map_realMods : u8,
                                 map_vmods : u16,
                                 map_ctrls : u32) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xkb_set_named_indicator_checked(c.get_raw_conn(),
        deviceSpec as xcb_xkb_device_spec_t, //1
        ledClass as xcb_xkb_led_class_spec_t, //2
        ledID as xcb_xkb_id_spec_t, //3
        indicator as ffi::xproto::xcb_atom_t, //4
        setState as u8, //5
        on as u8, //6
        setMap as u8, //7
        createMap as u8, //8
        map_flags as u8, //9
        map_whichGroups as u8, //10
        map_groups as u8, //11
        map_whichMods as u8, //12
        map_realMods as u8, //13
        map_vmods as u16, //14
        map_ctrls as u32); //15
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn SetNamedIndicator<'r> (c : &'r Connection,
                          deviceSpec : DeviceSpec,
                          ledClass : LedClassSpec,
                          ledID : IdSpec,
                          indicator : xproto::Atom,
                          setState : u8,
                          on : u8,
                          setMap : u8,
                          createMap : u8,
                          map_flags : u8,
                          map_whichGroups : u8,
                          map_groups : u8,
                          map_whichMods : u8,
                          map_realMods : u8,
                          map_vmods : u16,
                          map_ctrls : u32) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xkb_set_named_indicator(c.get_raw_conn(),
        deviceSpec as xcb_xkb_device_spec_t, //1
        ledClass as xcb_xkb_led_class_spec_t, //2
        ledID as xcb_xkb_id_spec_t, //3
        indicator as ffi::xproto::xcb_atom_t, //4
        setState as u8, //5
        on as u8, //6
        setMap as u8, //7
        createMap as u8, //8
        map_flags as u8, //9
        map_whichGroups as u8, //10
        map_groups as u8, //11
        map_whichMods as u8, //12
        map_realMods as u8, //13
        map_vmods as u16, //14
        map_ctrls as u32); //15
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetNamesValueList {
  pub fn keycodesName(&mut self) -> xproto::Atom {
    unsafe { accessor!(keycodesName -> xproto::Atom, self.base.strct) }
  }

  pub fn geometryName(&mut self) -> xproto::Atom {
    unsafe { accessor!(geometryName -> xproto::Atom, self.base.strct) }
  }

  pub fn symbolsName(&mut self) -> xproto::Atom {
    unsafe { accessor!(symbolsName -> xproto::Atom, self.base.strct) }
  }

  pub fn physSymbolsName(&mut self) -> xproto::Atom {
    unsafe { accessor!(physSymbolsName -> xproto::Atom, self.base.strct) }
  }

  pub fn typesName(&mut self) -> xproto::Atom {
    unsafe { accessor!(typesName -> xproto::Atom, self.base.strct) }
  }

  pub fn compatName(&mut self) -> xproto::Atom {
    unsafe { accessor!(compatName -> xproto::Atom, self.base.strct) }
  }

  pub fn typeNames(&mut self) -> Vec<xproto::Atom> {
    unsafe { accessor!(xproto::Atom, xcb_xkb_get_names_value_list_type_names_length, xcb_xkb_get_names_value_list_type_names, self.base.strct) }
  }

  pub fn ktLevelNames(&mut self) -> Vec<xproto::Atom> {
    unsafe { accessor!(xproto::Atom, xcb_xkb_get_names_value_list_kt_level_names_length, xcb_xkb_get_names_value_list_kt_level_names, self.base.strct) }
  }

  pub fn indicatorNames(&mut self) -> Vec<xproto::Atom> {
    unsafe { accessor!(xproto::Atom, xcb_xkb_get_names_value_list_indicator_names_length, xcb_xkb_get_names_value_list_indicator_names, self.base.strct) }
  }

  pub fn virtualModNames(&mut self) -> Vec<xproto::Atom> {
    unsafe { accessor!(xproto::Atom, xcb_xkb_get_names_value_list_virtual_mod_names_length, xcb_xkb_get_names_value_list_virtual_mod_names, self.base.strct) }
  }

  pub fn groups(&mut self) -> Vec<xproto::Atom> {
    unsafe { accessor!(xproto::Atom, xcb_xkb_get_names_value_list_groups_length, xcb_xkb_get_names_value_list_groups, self.base.strct) }
  }

  pub fn keyNames(&mut self) -> KeyNameIterator {
    unsafe { accessor!(KeyNameIterator, xcb_xkb_get_names_value_list_key_names_iterator, self.base.strct) }
  }

  pub fn keyAliases(&mut self) -> KeyAliasIterator {
    unsafe { accessor!(KeyAliasIterator, xcb_xkb_get_names_value_list_key_aliases_iterator, self.base.strct) }
  }

  pub fn radioGroupNames(&mut self) -> Vec<xproto::Atom> {
    unsafe { accessor!(xproto::Atom, xcb_xkb_get_names_value_list_radio_group_names_length, xcb_xkb_get_names_value_list_radio_group_names, self.base.strct) }
  }

}
pub struct GetNamesReply { base:  base::Reply<xcb_xkb_get_names_reply_t> }
fn mk_reply_xcb_xkb_get_names_reply_t(reply:*mut xcb_xkb_get_names_reply_t) -> GetNamesReply { GetNamesReply { base : base::mk_reply(reply) } }
pub fn GetNames<'r> (c : &'r Connection,
                 deviceSpec : DeviceSpec,
                 which : u32) -> GetNamesCookie<'r> {
  unsafe {
    let cookie = xcb_xkb_get_names(c.get_raw_conn(),
        deviceSpec as xcb_xkb_device_spec_t, //1
        which as u32); //2
    GetNamesCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetNamesUnchecked<'r> (c : &'r Connection,
                          deviceSpec : DeviceSpec,
                          which : u32) -> GetNamesCookie<'r> {
  unsafe {
    let cookie = xcb_xkb_get_names_unchecked(c.get_raw_conn(),
        deviceSpec as xcb_xkb_device_spec_t, //1
        which as u32); //2
    GetNamesCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetNamesReply {
  pub fn deviceID(&mut self) -> u8 {
    unsafe { accessor!(deviceID -> u8, (*self.base.reply)) }
  }

  pub fn which(&mut self) -> u32 {
    unsafe { accessor!(which -> u32, (*self.base.reply)) }
  }

  pub fn minKeyCode(&mut self) -> xproto::Keycode {
    unsafe { accessor!(minKeyCode -> xproto::Keycode, (*self.base.reply)) }
  }

  pub fn maxKeyCode(&mut self) -> xproto::Keycode {
    unsafe { accessor!(maxKeyCode -> xproto::Keycode, (*self.base.reply)) }
  }

  pub fn nTypes(&mut self) -> u8 {
    unsafe { accessor!(nTypes -> u8, (*self.base.reply)) }
  }

  pub fn groupNames(&mut self) -> u8 {
    unsafe { accessor!(groupNames -> u8, (*self.base.reply)) }
  }

  pub fn virtualMods(&mut self) -> u16 {
    unsafe { accessor!(virtualMods -> u16, (*self.base.reply)) }
  }

  pub fn firstKey(&mut self) -> xproto::Keycode {
    unsafe { accessor!(firstKey -> xproto::Keycode, (*self.base.reply)) }
  }

  pub fn nKeys(&mut self) -> u8 {
    unsafe { accessor!(nKeys -> u8, (*self.base.reply)) }
  }

  pub fn indicators(&mut self) -> u32 {
    unsafe { accessor!(indicators -> u32, (*self.base.reply)) }
  }

  pub fn nRadioGroups(&mut self) -> u8 {
    unsafe { accessor!(nRadioGroups -> u8, (*self.base.reply)) }
  }

  pub fn nKeyAliases(&mut self) -> u8 {
    unsafe { accessor!(nKeyAliases -> u8, (*self.base.reply)) }
  }

  pub fn nKTLevels(&mut self) -> u16 {
    unsafe { accessor!(nKTLevels -> u16, (*self.base.reply)) }
  }

  pub fn valueList(&self) -> GetNamesValueList {
    unsafe { mem::transmute((*self.base.reply).valueList) }
  }
}
impl_reply_cookie!(GetNamesCookie<'s>, mk_reply_xcb_xkb_get_names_reply_t, GetNamesReply, xcb_xkb_get_names_reply);

pub struct SetNamesValues {pub base : base::Struct<xcb_xkb_set_names_values_t> }


impl SetNamesValues {
  pub fn keycodesName(&mut self) -> xproto::Atom {
    unsafe { accessor!(keycodesName -> xproto::Atom, self.base.strct) }
  }

  pub fn geometryName(&mut self) -> xproto::Atom {
    unsafe { accessor!(geometryName -> xproto::Atom, self.base.strct) }
  }

  pub fn symbolsName(&mut self) -> xproto::Atom {
    unsafe { accessor!(symbolsName -> xproto::Atom, self.base.strct) }
  }

  pub fn physSymbolsName(&mut self) -> xproto::Atom {
    unsafe { accessor!(physSymbolsName -> xproto::Atom, self.base.strct) }
  }

  pub fn typesName(&mut self) -> xproto::Atom {
    unsafe { accessor!(typesName -> xproto::Atom, self.base.strct) }
  }

  pub fn compatName(&mut self) -> xproto::Atom {
    unsafe { accessor!(compatName -> xproto::Atom, self.base.strct) }
  }

  pub fn typeNames(&mut self) -> Vec<xproto::Atom> {
    unsafe { accessor!(xproto::Atom, xcb_xkb_set_names_values_type_names_length, xcb_xkb_set_names_values_type_names, self.base.strct) }
  }

  pub fn ktLevelNames(&mut self) -> Vec<xproto::Atom> {
    unsafe { accessor!(xproto::Atom, xcb_xkb_set_names_values_kt_level_names_length, xcb_xkb_set_names_values_kt_level_names, self.base.strct) }
  }

  pub fn indicatorNames(&mut self) -> Vec<xproto::Atom> {
    unsafe { accessor!(xproto::Atom, xcb_xkb_set_names_values_indicator_names_length, xcb_xkb_set_names_values_indicator_names, self.base.strct) }
  }

  pub fn virtualModNames(&mut self) -> Vec<xproto::Atom> {
    unsafe { accessor!(xproto::Atom, xcb_xkb_set_names_values_virtual_mod_names_length, xcb_xkb_set_names_values_virtual_mod_names, self.base.strct) }
  }

  pub fn groups(&mut self) -> Vec<xproto::Atom> {
    unsafe { accessor!(xproto::Atom, xcb_xkb_set_names_values_groups_length, xcb_xkb_set_names_values_groups, self.base.strct) }
  }

  pub fn keyNames(&mut self) -> KeyNameIterator {
    unsafe { accessor!(KeyNameIterator, xcb_xkb_set_names_values_key_names_iterator, self.base.strct) }
  }

  pub fn keyAliases(&mut self) -> KeyAliasIterator {
    unsafe { accessor!(KeyAliasIterator, xcb_xkb_set_names_values_key_aliases_iterator, self.base.strct) }
  }

  pub fn radioGroupNames(&mut self) -> Vec<xproto::Atom> {
    unsafe { accessor!(xproto::Atom, xcb_xkb_set_names_values_radio_group_names_length, xcb_xkb_set_names_values_radio_group_names, self.base.strct) }
  }

}
pub fn SetNamesChecked<'r> (c : &'r Connection,
                        deviceSpec : DeviceSpec,
                        virtualMods : u16,
                        which : u32,
                        firstType : u8,
                        nTypes : u8,
                        firstKTLevelt : u8,
                        nKTLevels : u8,
                        indicators : u32,
                        groupNames : u8,
                        nRadioGroups : u8,
                        firstKey : xproto::Keycode,
                        nKeys : u8,
                        nKeyAliases : u8,
                        totalKTLevelNames : u16,
                        values : SetNamesValues) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xkb_set_names_checked(c.get_raw_conn(),
        deviceSpec as xcb_xkb_device_spec_t, //1
        virtualMods as u16, //2
        which as u32, //3
        firstType as u8, //4
        nTypes as u8, //5
        firstKTLevelt as u8, //6
        nKTLevels as u8, //7
        indicators as u32, //8
        groupNames as u8, //9
        nRadioGroups as u8, //10
        firstKey as ffi::xproto::xcb_keycode_t, //11
        nKeys as u8, //12
        nKeyAliases as u8, //13
        totalKTLevelNames as u16, //14
        values.base.strct); //15
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn SetNames<'r> (c : &'r Connection,
                 deviceSpec : DeviceSpec,
                 virtualMods : u16,
                 which : u32,
                 firstType : u8,
                 nTypes : u8,
                 firstKTLevelt : u8,
                 nKTLevels : u8,
                 indicators : u32,
                 groupNames : u8,
                 nRadioGroups : u8,
                 firstKey : xproto::Keycode,
                 nKeys : u8,
                 nKeyAliases : u8,
                 totalKTLevelNames : u16,
                 values : SetNamesValues) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xkb_set_names(c.get_raw_conn(),
        deviceSpec as xcb_xkb_device_spec_t, //1
        virtualMods as u16, //2
        which as u32, //3
        firstType as u8, //4
        nTypes as u8, //5
        firstKTLevelt as u8, //6
        nKTLevels as u8, //7
        indicators as u32, //8
        groupNames as u8, //9
        nRadioGroups as u8, //10
        firstKey as ffi::xproto::xcb_keycode_t, //11
        nKeys as u8, //12
        nKeyAliases as u8, //13
        totalKTLevelNames as u16, //14
        values.base.strct); //15
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn SetNamesAuxChecked<'r> (c : &'r Connection,
                           deviceSpec : DeviceSpec,
                           virtualMods : u16,
                           which : u32,
                           firstType : u8,
                           nTypes : u8,
                           firstKTLevelt : u8,
                           nKTLevels : u8,
                           indicators : u32,
                           groupNames : u8,
                           nRadioGroups : u8,
                           firstKey : xproto::Keycode,
                           nKeys : u8,
                           nKeyAliases : u8,
                           totalKTLevelNames : u16,
                           values : SetNamesValues) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xkb_set_names_aux_checked(c.get_raw_conn(),
        deviceSpec as xcb_xkb_device_spec_t, //1
        virtualMods as u16, //2
        which as u32, //3
        firstType as u8, //4
        nTypes as u8, //5
        firstKTLevelt as u8, //6
        nKTLevels as u8, //7
        indicators as u32, //8
        groupNames as u8, //9
        nRadioGroups as u8, //10
        firstKey as ffi::xproto::xcb_keycode_t, //11
        nKeys as u8, //12
        nKeyAliases as u8, //13
        totalKTLevelNames as u16, //14
        values.base.strct); //15
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn SetNamesAux<'r> (c : &'r Connection,
                    deviceSpec : DeviceSpec,
                    virtualMods : u16,
                    which : u32,
                    firstType : u8,
                    nTypes : u8,
                    firstKTLevelt : u8,
                    nKTLevels : u8,
                    indicators : u32,
                    groupNames : u8,
                    nRadioGroups : u8,
                    firstKey : xproto::Keycode,
                    nKeys : u8,
                    nKeyAliases : u8,
                    totalKTLevelNames : u16,
                    values : SetNamesValues) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xkb_set_names_aux(c.get_raw_conn(),
        deviceSpec as xcb_xkb_device_spec_t, //1
        virtualMods as u16, //2
        which as u32, //3
        firstType as u8, //4
        nTypes as u8, //5
        firstKTLevelt as u8, //6
        nKTLevels as u8, //7
        indicators as u32, //8
        groupNames as u8, //9
        nRadioGroups as u8, //10
        firstKey as ffi::xproto::xcb_keycode_t, //11
        nKeys as u8, //12
        nKeyAliases as u8, //13
        totalKTLevelNames as u16, //14
        values.base.strct); //15
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub struct GetGeometryReply { base:  base::Reply<xcb_xkb_get_geometry_reply_t> }
fn mk_reply_xcb_xkb_get_geometry_reply_t(reply:*mut xcb_xkb_get_geometry_reply_t) -> GetGeometryReply { GetGeometryReply { base : base::mk_reply(reply) } }
pub fn GetGeometry<'r> (c : &'r Connection,
                    deviceSpec : DeviceSpec,
                    name : xproto::Atom) -> GetGeometryCookie<'r> {
  unsafe {
    let cookie = xcb_xkb_get_geometry(c.get_raw_conn(),
        deviceSpec as xcb_xkb_device_spec_t, //1
        name as ffi::xproto::xcb_atom_t); //2
    GetGeometryCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetGeometryUnchecked<'r> (c : &'r Connection,
                             deviceSpec : DeviceSpec,
                             name : xproto::Atom) -> GetGeometryCookie<'r> {
  unsafe {
    let cookie = xcb_xkb_get_geometry_unchecked(c.get_raw_conn(),
        deviceSpec as xcb_xkb_device_spec_t, //1
        name as ffi::xproto::xcb_atom_t); //2
    GetGeometryCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetGeometryReply {
  pub fn deviceID(&mut self) -> u8 {
    unsafe { accessor!(deviceID -> u8, (*self.base.reply)) }
  }

  pub fn name(&mut self) -> xproto::Atom {
    unsafe { accessor!(name -> xproto::Atom, (*self.base.reply)) }
  }

  pub fn found(&mut self) -> u8 {
    unsafe { accessor!(found -> u8, (*self.base.reply)) }
  }

  pub fn widthMM(&mut self) -> u16 {
    unsafe { accessor!(widthMM -> u16, (*self.base.reply)) }
  }

  pub fn heightMM(&mut self) -> u16 {
    unsafe { accessor!(heightMM -> u16, (*self.base.reply)) }
  }

  pub fn baseColorNdx(&mut self) -> u8 {
    unsafe { accessor!(baseColorNdx -> u8, (*self.base.reply)) }
  }

  pub fn labelColorNdx(&mut self) -> u8 {
    unsafe { accessor!(labelColorNdx -> u8, (*self.base.reply)) }
  }

  pub fn labelFont(&self) -> CountedString16 {
    unsafe { mem::transmute((*self.base.reply).labelFont) }
  }
  pub fn properties(&mut self) -> PropertyIterator {
    unsafe { accessor!(PropertyIterator, xcb_xkb_get_geometry_properties_iterator, (*self.base.reply)) }
  }

  pub fn colors(&mut self) -> CountedString16Iterator {
    unsafe { accessor!(CountedString16Iterator, xcb_xkb_get_geometry_colors_iterator, (*self.base.reply)) }
  }

  pub fn shapes(&mut self) -> ShapeIterator {
    unsafe { accessor!(ShapeIterator, xcb_xkb_get_geometry_shapes_iterator, (*self.base.reply)) }
  }

  pub fn sections(&mut self) -> SectionIterator {
    unsafe { accessor!(SectionIterator, xcb_xkb_get_geometry_sections_iterator, (*self.base.reply)) }
  }

  pub fn doodads(&mut self) -> DoodadIterator {
    unsafe { accessor!(DoodadIterator, xcb_xkb_get_geometry_doodads_iterator, (*self.base.reply)) }
  }

  pub fn keyAliases(&mut self) -> KeyAliasIterator {
    unsafe { accessor!(KeyAliasIterator, xcb_xkb_get_geometry_key_aliases_iterator, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetGeometryCookie<'s>, mk_reply_xcb_xkb_get_geometry_reply_t, GetGeometryReply, xcb_xkb_get_geometry_reply);

pub fn SetGeometryChecked<'r> (c : &'r Connection,
                           deviceSpec : DeviceSpec,
                           name : xproto::Atom,
                           widthMM : u16,
                           heightMM : u16,
                           baseColorNdx : u8,
                           labelColorNdx : u8,
                           labelFont : CountedString16,
                           properties : &[Property],
                           colors : &[CountedString16],
                           shapes : &[Shape],
                           sections : &[Section],
                           doodads : &[Doodad],
                           keyAliases : &[KeyAlias]) -> base::VoidCookie<'r> {
  unsafe {
    let properties_len = properties.len();
    let properties_ptr = properties.as_ptr();
    let colors_len = colors.len();
    let colors_ptr = colors.as_ptr();
    let shapes_len = shapes.len();
    let shapes_ptr = shapes.as_ptr();
    let sections_len = sections.len();
    let sections_ptr = sections.as_ptr();
    let doodads_len = doodads.len();
    let doodads_ptr = doodads.as_ptr();
    let keyAliases_len = keyAliases.len();
    let keyAliases_ptr = keyAliases.as_ptr();
    let cookie = xcb_xkb_set_geometry_checked(c.get_raw_conn(),
        deviceSpec as xcb_xkb_device_spec_t, //1
        shapes_len as u8, //2
        sections_len as u8, //3
        name as ffi::xproto::xcb_atom_t, //4
        widthMM as u16, //5
        heightMM as u16, //6
        properties_len as u16, //7
        colors_len as u16, //8
        doodads_len as u16, //9
        keyAliases_len as u16, //10
        baseColorNdx as u8, //11
        labelColorNdx as u8, //12
        labelFont.base.strct, //13
        properties_ptr as *mut xcb_xkb_property_t, //14
        colors_ptr as *mut xcb_xkb_counted_string_16_t, //15
        shapes_ptr as *mut xcb_xkb_shape_t, //16
        sections_ptr as *mut xcb_xkb_section_t, //17
        doodads_ptr as *mut xcb_xkb_doodad_t, //18
        keyAliases_ptr as *mut xcb_xkb_key_alias_t); //19
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn SetGeometry<'r> (c : &'r Connection,
                    deviceSpec : DeviceSpec,
                    name : xproto::Atom,
                    widthMM : u16,
                    heightMM : u16,
                    baseColorNdx : u8,
                    labelColorNdx : u8,
                    labelFont : CountedString16,
                    properties : &[Property],
                    colors : &[CountedString16],
                    shapes : &[Shape],
                    sections : &[Section],
                    doodads : &[Doodad],
                    keyAliases : &[KeyAlias]) -> base::VoidCookie<'r> {
  unsafe {
    let properties_len = properties.len();
    let properties_ptr = properties.as_ptr();
    let colors_len = colors.len();
    let colors_ptr = colors.as_ptr();
    let shapes_len = shapes.len();
    let shapes_ptr = shapes.as_ptr();
    let sections_len = sections.len();
    let sections_ptr = sections.as_ptr();
    let doodads_len = doodads.len();
    let doodads_ptr = doodads.as_ptr();
    let keyAliases_len = keyAliases.len();
    let keyAliases_ptr = keyAliases.as_ptr();
    let cookie = xcb_xkb_set_geometry(c.get_raw_conn(),
        deviceSpec as xcb_xkb_device_spec_t, //1
        shapes_len as u8, //2
        sections_len as u8, //3
        name as ffi::xproto::xcb_atom_t, //4
        widthMM as u16, //5
        heightMM as u16, //6
        properties_len as u16, //7
        colors_len as u16, //8
        doodads_len as u16, //9
        keyAliases_len as u16, //10
        baseColorNdx as u8, //11
        labelColorNdx as u8, //12
        labelFont.base.strct, //13
        properties_ptr as *mut xcb_xkb_property_t, //14
        colors_ptr as *mut xcb_xkb_counted_string_16_t, //15
        shapes_ptr as *mut xcb_xkb_shape_t, //16
        sections_ptr as *mut xcb_xkb_section_t, //17
        doodads_ptr as *mut xcb_xkb_doodad_t, //18
        keyAliases_ptr as *mut xcb_xkb_key_alias_t); //19
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn PerClientFlags<'r> (c : &'r Connection,
                       deviceSpec : DeviceSpec,
                       change : u32,
                       value : u32,
                       ctrlsToChange : u32,
                       autoCtrls : u32,
                       autoCtrlsValues : u32) -> PerClientFlagsCookie<'r> {
  unsafe {
    let cookie = xcb_xkb_per_client_flags(c.get_raw_conn(),
        deviceSpec as xcb_xkb_device_spec_t, //1
        change as u32, //2
        value as u32, //3
        ctrlsToChange as u32, //4
        autoCtrls as u32, //5
        autoCtrlsValues as u32); //6
    PerClientFlagsCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn PerClientFlagsUnchecked<'r> (c : &'r Connection,
                                deviceSpec : DeviceSpec,
                                change : u32,
                                value : u32,
                                ctrlsToChange : u32,
                                autoCtrls : u32,
                                autoCtrlsValues : u32) -> PerClientFlagsCookie<'r> {
  unsafe {
    let cookie = xcb_xkb_per_client_flags_unchecked(c.get_raw_conn(),
        deviceSpec as xcb_xkb_device_spec_t, //1
        change as u32, //2
        value as u32, //3
        ctrlsToChange as u32, //4
        autoCtrls as u32, //5
        autoCtrlsValues as u32); //6
    PerClientFlagsCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl PerClientFlagsReply {
  pub fn deviceID(&mut self) -> u8 {
    unsafe { accessor!(deviceID -> u8, (*self.base.reply)) }
  }

  pub fn supported(&mut self) -> u32 {
    unsafe { accessor!(supported -> u32, (*self.base.reply)) }
  }

  pub fn value(&mut self) -> u32 {
    unsafe { accessor!(value -> u32, (*self.base.reply)) }
  }

  pub fn autoCtrls(&mut self) -> u32 {
    unsafe { accessor!(autoCtrls -> u32, (*self.base.reply)) }
  }

  pub fn autoCtrlsValues(&mut self) -> u32 {
    unsafe { accessor!(autoCtrlsValues -> u32, (*self.base.reply)) }
  }

}
impl_reply_cookie!(PerClientFlagsCookie<'s>, mk_reply_xcb_xkb_per_client_flags_reply_t, PerClientFlagsReply, xcb_xkb_per_client_flags_reply);

pub fn ListComponents<'r> (c : &'r Connection,
                       deviceSpec : DeviceSpec,
                       maxNames : u16,
                       keymapsSpec : &[String8],
                       keycodesSpec : &[String8],
                       typesSpec : &[String8],
                       compatMapSpec : &[String8],
                       symbolsSpec : &[String8],
                       geometrySpec : &[String8]) -> ListComponentsCookie<'r> {
  unsafe {
    let keymapsSpec_len = keymapsSpec.len();
    let keymapsSpec_ptr = keymapsSpec.as_ptr();
    let keycodesSpec_len = keycodesSpec.len();
    let keycodesSpec_ptr = keycodesSpec.as_ptr();
    let typesSpec_len = typesSpec.len();
    let typesSpec_ptr = typesSpec.as_ptr();
    let compatMapSpec_len = compatMapSpec.len();
    let compatMapSpec_ptr = compatMapSpec.as_ptr();
    let symbolsSpec_len = symbolsSpec.len();
    let symbolsSpec_ptr = symbolsSpec.as_ptr();
    let geometrySpec_len = geometrySpec.len();
    let geometrySpec_ptr = geometrySpec.as_ptr();
    let cookie = xcb_xkb_list_components(c.get_raw_conn(),
        deviceSpec as xcb_xkb_device_spec_t, //1
        maxNames as u16, //2
        keymapsSpec_len as u8, //3
        keymapsSpec_ptr as *mut xcb_xkb_string8_t, //4
        keycodesSpec_len as u8, //5
        keycodesSpec_ptr as *mut xcb_xkb_string8_t, //6
        typesSpec_len as u8, //7
        typesSpec_ptr as *mut xcb_xkb_string8_t, //8
        compatMapSpec_len as u8, //9
        compatMapSpec_ptr as *mut xcb_xkb_string8_t, //10
        symbolsSpec_len as u8, //11
        symbolsSpec_ptr as *mut xcb_xkb_string8_t, //12
        geometrySpec_len as u8, //13
        geometrySpec_ptr as *mut xcb_xkb_string8_t); //14
    ListComponentsCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn ListComponentsUnchecked<'r> (c : &'r Connection,
                                deviceSpec : DeviceSpec,
                                maxNames : u16,
                                keymapsSpec : &[String8],
                                keycodesSpec : &[String8],
                                typesSpec : &[String8],
                                compatMapSpec : &[String8],
                                symbolsSpec : &[String8],
                                geometrySpec : &[String8]) -> ListComponentsCookie<'r> {
  unsafe {
    let keymapsSpec_len = keymapsSpec.len();
    let keymapsSpec_ptr = keymapsSpec.as_ptr();
    let keycodesSpec_len = keycodesSpec.len();
    let keycodesSpec_ptr = keycodesSpec.as_ptr();
    let typesSpec_len = typesSpec.len();
    let typesSpec_ptr = typesSpec.as_ptr();
    let compatMapSpec_len = compatMapSpec.len();
    let compatMapSpec_ptr = compatMapSpec.as_ptr();
    let symbolsSpec_len = symbolsSpec.len();
    let symbolsSpec_ptr = symbolsSpec.as_ptr();
    let geometrySpec_len = geometrySpec.len();
    let geometrySpec_ptr = geometrySpec.as_ptr();
    let cookie = xcb_xkb_list_components_unchecked(c.get_raw_conn(),
        deviceSpec as xcb_xkb_device_spec_t, //1
        maxNames as u16, //2
        keymapsSpec_len as u8, //3
        keymapsSpec_ptr as *mut xcb_xkb_string8_t, //4
        keycodesSpec_len as u8, //5
        keycodesSpec_ptr as *mut xcb_xkb_string8_t, //6
        typesSpec_len as u8, //7
        typesSpec_ptr as *mut xcb_xkb_string8_t, //8
        compatMapSpec_len as u8, //9
        compatMapSpec_ptr as *mut xcb_xkb_string8_t, //10
        symbolsSpec_len as u8, //11
        symbolsSpec_ptr as *mut xcb_xkb_string8_t, //12
        geometrySpec_len as u8, //13
        geometrySpec_ptr as *mut xcb_xkb_string8_t); //14
    ListComponentsCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl ListComponentsReply {
  pub fn deviceID(&mut self) -> u8 {
    unsafe { accessor!(deviceID -> u8, (*self.base.reply)) }
  }

  pub fn extra(&mut self) -> u16 {
    unsafe { accessor!(extra -> u16, (*self.base.reply)) }
  }

  pub fn keymaps(&mut self) -> ListingIterator {
    unsafe { accessor!(ListingIterator, xcb_xkb_list_components_keymaps_iterator, (*self.base.reply)) }
  }

  pub fn keycodes(&mut self) -> ListingIterator {
    unsafe { accessor!(ListingIterator, xcb_xkb_list_components_keycodes_iterator, (*self.base.reply)) }
  }

  pub fn types(&mut self) -> ListingIterator {
    unsafe { accessor!(ListingIterator, xcb_xkb_list_components_types_iterator, (*self.base.reply)) }
  }

  pub fn compatMaps(&mut self) -> ListingIterator {
    unsafe { accessor!(ListingIterator, xcb_xkb_list_components_compat_maps_iterator, (*self.base.reply)) }
  }

  pub fn symbols(&mut self) -> ListingIterator {
    unsafe { accessor!(ListingIterator, xcb_xkb_list_components_symbols_iterator, (*self.base.reply)) }
  }

  pub fn geometries(&mut self) -> ListingIterator {
    unsafe { accessor!(ListingIterator, xcb_xkb_list_components_geometries_iterator, (*self.base.reply)) }
  }

}
impl_reply_cookie!(ListComponentsCookie<'s>, mk_reply_xcb_xkb_list_components_reply_t, ListComponentsReply, xcb_xkb_list_components_reply);


impl GetKbdByNameRepliesTypesMap {
  pub fn types_rtrn(&mut self) -> KeyTypeIterator {
    unsafe { accessor!(KeyTypeIterator, xcb_xkb_get_kbd_by_name_replies_types_map_types_rtrn_iterator, self.base.strct) }
  }

  pub fn syms_rtrn(&mut self) -> KeySymMapIterator {
    unsafe { accessor!(KeySymMapIterator, xcb_xkb_get_kbd_by_name_replies_types_map_syms_rtrn_iterator, self.base.strct) }
  }

  pub fn acts_rtrn_count(&mut self) -> Vec<xproto::Keycode> {
    unsafe { accessor!(xproto::Keycode, xcb_xkb_get_kbd_by_name_replies_types_map_acts_rtrn_count_length, xcb_xkb_get_kbd_by_name_replies_types_map_acts_rtrn_count, self.base.strct) }
  }

  pub fn acts_rtrn_acts(&mut self) -> ActionIterator {
    unsafe { accessor!(ActionIterator, xcb_xkb_get_kbd_by_name_replies_types_map_acts_rtrn_acts_iterator, self.base.strct) }
  }

  pub fn behaviors_rtrn(&mut self) -> SetBehaviorIterator {
    unsafe { accessor!(SetBehaviorIterator, xcb_xkb_get_kbd_by_name_replies_types_map_behaviors_rtrn_iterator, self.base.strct) }
  }

  pub fn vmods_rtrn(&mut self) -> Vec<xproto::Keycode> {
    unsafe { accessor!(xproto::Keycode, xcb_xkb_get_kbd_by_name_replies_types_map_vmods_rtrn_length, xcb_xkb_get_kbd_by_name_replies_types_map_vmods_rtrn, self.base.strct) }
  }

  pub fn explicit_rtrn(&mut self) -> SetExplicitIterator {
    unsafe { accessor!(SetExplicitIterator, xcb_xkb_get_kbd_by_name_replies_types_map_explicit_rtrn_iterator, self.base.strct) }
  }

  pub fn modmap_rtrn(&mut self) -> KeyModMapIterator {
    unsafe { accessor!(KeyModMapIterator, xcb_xkb_get_kbd_by_name_replies_types_map_modmap_rtrn_iterator, self.base.strct) }
  }

  pub fn vmodmap_rtrn(&mut self) -> KeyVModMapIterator {
    unsafe { accessor!(KeyVModMapIterator, xcb_xkb_get_kbd_by_name_replies_types_map_vmodmap_rtrn_iterator, self.base.strct) }
  }

}
pub struct GetKbdByNameRepliesClientSymbolsMap {pub base : base::Struct<xcb_xkb_get_kbd_by_name_replies_client_symbols_map_t> }


impl GetKbdByNameRepliesClientSymbolsMap {
  pub fn types_rtrn(&mut self) -> KeyTypeIterator {
    unsafe { accessor!(KeyTypeIterator, xcb_xkb_get_kbd_by_name_replies_client_symbols_map_types_rtrn_iterator, self.base.strct) }
  }

  pub fn syms_rtrn(&mut self) -> KeySymMapIterator {
    unsafe { accessor!(KeySymMapIterator, xcb_xkb_get_kbd_by_name_replies_client_symbols_map_syms_rtrn_iterator, self.base.strct) }
  }

  pub fn acts_rtrn_count(&mut self) -> Vec<xproto::Keycode> {
    unsafe { accessor!(xproto::Keycode, xcb_xkb_get_kbd_by_name_replies_client_symbols_map_acts_rtrn_count_length, xcb_xkb_get_kbd_by_name_replies_client_symbols_map_acts_rtrn_count, self.base.strct) }
  }

  pub fn acts_rtrn_acts(&mut self) -> ActionIterator {
    unsafe { accessor!(ActionIterator, xcb_xkb_get_kbd_by_name_replies_client_symbols_map_acts_rtrn_acts_iterator, self.base.strct) }
  }

  pub fn behaviors_rtrn(&mut self) -> SetBehaviorIterator {
    unsafe { accessor!(SetBehaviorIterator, xcb_xkb_get_kbd_by_name_replies_client_symbols_map_behaviors_rtrn_iterator, self.base.strct) }
  }

  pub fn vmods_rtrn(&mut self) -> Vec<xproto::Keycode> {
    unsafe { accessor!(xproto::Keycode, xcb_xkb_get_kbd_by_name_replies_client_symbols_map_vmods_rtrn_length, xcb_xkb_get_kbd_by_name_replies_client_symbols_map_vmods_rtrn, self.base.strct) }
  }

  pub fn explicit_rtrn(&mut self) -> SetExplicitIterator {
    unsafe { accessor!(SetExplicitIterator, xcb_xkb_get_kbd_by_name_replies_client_symbols_map_explicit_rtrn_iterator, self.base.strct) }
  }

  pub fn modmap_rtrn(&mut self) -> KeyModMapIterator {
    unsafe { accessor!(KeyModMapIterator, xcb_xkb_get_kbd_by_name_replies_client_symbols_map_modmap_rtrn_iterator, self.base.strct) }
  }

  pub fn vmodmap_rtrn(&mut self) -> KeyVModMapIterator {
    unsafe { accessor!(KeyVModMapIterator, xcb_xkb_get_kbd_by_name_replies_client_symbols_map_vmodmap_rtrn_iterator, self.base.strct) }
  }

}
pub struct GetKbdByNameRepliesServerSymbolsMap {pub base : base::Struct<xcb_xkb_get_kbd_by_name_replies_server_symbols_map_t> }


impl GetKbdByNameRepliesServerSymbolsMap {
  pub fn types_rtrn(&mut self) -> KeyTypeIterator {
    unsafe { accessor!(KeyTypeIterator, xcb_xkb_get_kbd_by_name_replies_server_symbols_map_types_rtrn_iterator, self.base.strct) }
  }

  pub fn syms_rtrn(&mut self) -> KeySymMapIterator {
    unsafe { accessor!(KeySymMapIterator, xcb_xkb_get_kbd_by_name_replies_server_symbols_map_syms_rtrn_iterator, self.base.strct) }
  }

  pub fn acts_rtrn_count(&mut self) -> Vec<xproto::Keycode> {
    unsafe { accessor!(xproto::Keycode, xcb_xkb_get_kbd_by_name_replies_server_symbols_map_acts_rtrn_count_length, xcb_xkb_get_kbd_by_name_replies_server_symbols_map_acts_rtrn_count, self.base.strct) }
  }

  pub fn acts_rtrn_acts(&mut self) -> ActionIterator {
    unsafe { accessor!(ActionIterator, xcb_xkb_get_kbd_by_name_replies_server_symbols_map_acts_rtrn_acts_iterator, self.base.strct) }
  }

  pub fn behaviors_rtrn(&mut self) -> SetBehaviorIterator {
    unsafe { accessor!(SetBehaviorIterator, xcb_xkb_get_kbd_by_name_replies_server_symbols_map_behaviors_rtrn_iterator, self.base.strct) }
  }

  pub fn vmods_rtrn(&mut self) -> Vec<xproto::Keycode> {
    unsafe { accessor!(xproto::Keycode, xcb_xkb_get_kbd_by_name_replies_server_symbols_map_vmods_rtrn_length, xcb_xkb_get_kbd_by_name_replies_server_symbols_map_vmods_rtrn, self.base.strct) }
  }

  pub fn explicit_rtrn(&mut self) -> SetExplicitIterator {
    unsafe { accessor!(SetExplicitIterator, xcb_xkb_get_kbd_by_name_replies_server_symbols_map_explicit_rtrn_iterator, self.base.strct) }
  }

  pub fn modmap_rtrn(&mut self) -> KeyModMapIterator {
    unsafe { accessor!(KeyModMapIterator, xcb_xkb_get_kbd_by_name_replies_server_symbols_map_modmap_rtrn_iterator, self.base.strct) }
  }

  pub fn vmodmap_rtrn(&mut self) -> KeyVModMapIterator {
    unsafe { accessor!(KeyVModMapIterator, xcb_xkb_get_kbd_by_name_replies_server_symbols_map_vmodmap_rtrn_iterator, self.base.strct) }
  }

}
pub struct GetKbdByNameRepliesKeyNamesValueList {pub base : base::Struct<xcb_xkb_get_kbd_by_name_replies_key_names_value_list_t> }


impl GetKbdByNameRepliesKeyNamesValueList {
  pub fn keycodesName(&mut self) -> xproto::Atom {
    unsafe { accessor!(keycodesName -> xproto::Atom, self.base.strct) }
  }

  pub fn geometryName(&mut self) -> xproto::Atom {
    unsafe { accessor!(geometryName -> xproto::Atom, self.base.strct) }
  }

  pub fn symbolsName(&mut self) -> xproto::Atom {
    unsafe { accessor!(symbolsName -> xproto::Atom, self.base.strct) }
  }

  pub fn physSymbolsName(&mut self) -> xproto::Atom {
    unsafe { accessor!(physSymbolsName -> xproto::Atom, self.base.strct) }
  }

  pub fn typesName(&mut self) -> xproto::Atom {
    unsafe { accessor!(typesName -> xproto::Atom, self.base.strct) }
  }

  pub fn compatName(&mut self) -> xproto::Atom {
    unsafe { accessor!(compatName -> xproto::Atom, self.base.strct) }
  }

  pub fn typeNames(&mut self) -> Vec<xproto::Atom> {
    unsafe { accessor!(xproto::Atom, xcb_xkb_get_kbd_by_name_replies_key_names_value_list_type_names_length, xcb_xkb_get_kbd_by_name_replies_key_names_value_list_type_names, self.base.strct) }
  }

  pub fn ktLevelNames(&mut self) -> Vec<xproto::Atom> {
    unsafe { accessor!(xproto::Atom, xcb_xkb_get_kbd_by_name_replies_key_names_value_list_kt_level_names_length, xcb_xkb_get_kbd_by_name_replies_key_names_value_list_kt_level_names, self.base.strct) }
  }

  pub fn indicatorNames(&mut self) -> Vec<xproto::Atom> {
    unsafe { accessor!(xproto::Atom, xcb_xkb_get_kbd_by_name_replies_key_names_value_list_indicator_names_length, xcb_xkb_get_kbd_by_name_replies_key_names_value_list_indicator_names, self.base.strct) }
  }

  pub fn virtualModNames(&mut self) -> Vec<xproto::Atom> {
    unsafe { accessor!(xproto::Atom, xcb_xkb_get_kbd_by_name_replies_key_names_value_list_virtual_mod_names_length, xcb_xkb_get_kbd_by_name_replies_key_names_value_list_virtual_mod_names, self.base.strct) }
  }

  pub fn groups(&mut self) -> Vec<xproto::Atom> {
    unsafe { accessor!(xproto::Atom, xcb_xkb_get_kbd_by_name_replies_key_names_value_list_groups_length, xcb_xkb_get_kbd_by_name_replies_key_names_value_list_groups, self.base.strct) }
  }

  pub fn keyNames(&mut self) -> KeyNameIterator {
    unsafe { accessor!(KeyNameIterator, xcb_xkb_get_kbd_by_name_replies_key_names_value_list_key_names_iterator, self.base.strct) }
  }

  pub fn keyAliases(&mut self) -> KeyAliasIterator {
    unsafe { accessor!(KeyAliasIterator, xcb_xkb_get_kbd_by_name_replies_key_names_value_list_key_aliases_iterator, self.base.strct) }
  }

  pub fn radioGroupNames(&mut self) -> Vec<xproto::Atom> {
    unsafe { accessor!(xproto::Atom, xcb_xkb_get_kbd_by_name_replies_key_names_value_list_radio_group_names_length, xcb_xkb_get_kbd_by_name_replies_key_names_value_list_radio_group_names, self.base.strct) }
  }

}
pub struct GetKbdByNameRepliesOtherNamesValueList {pub base : base::Struct<xcb_xkb_get_kbd_by_name_replies_other_names_value_list_t> }


impl GetKbdByNameRepliesOtherNamesValueList {
  pub fn keycodesName(&mut self) -> xproto::Atom {
    unsafe { accessor!(keycodesName -> xproto::Atom, self.base.strct) }
  }

  pub fn geometryName(&mut self) -> xproto::Atom {
    unsafe { accessor!(geometryName -> xproto::Atom, self.base.strct) }
  }

  pub fn symbolsName(&mut self) -> xproto::Atom {
    unsafe { accessor!(symbolsName -> xproto::Atom, self.base.strct) }
  }

  pub fn physSymbolsName(&mut self) -> xproto::Atom {
    unsafe { accessor!(physSymbolsName -> xproto::Atom, self.base.strct) }
  }

  pub fn typesName(&mut self) -> xproto::Atom {
    unsafe { accessor!(typesName -> xproto::Atom, self.base.strct) }
  }

  pub fn compatName(&mut self) -> xproto::Atom {
    unsafe { accessor!(compatName -> xproto::Atom, self.base.strct) }
  }

  pub fn typeNames(&mut self) -> Vec<xproto::Atom> {
    unsafe { accessor!(xproto::Atom, xcb_xkb_get_kbd_by_name_replies_other_names_value_list_type_names_length, xcb_xkb_get_kbd_by_name_replies_other_names_value_list_type_names, self.base.strct) }
  }

  pub fn ktLevelNames(&mut self) -> Vec<xproto::Atom> {
    unsafe { accessor!(xproto::Atom, xcb_xkb_get_kbd_by_name_replies_other_names_value_list_kt_level_names_length, xcb_xkb_get_kbd_by_name_replies_other_names_value_list_kt_level_names, self.base.strct) }
  }

  pub fn indicatorNames(&mut self) -> Vec<xproto::Atom> {
    unsafe { accessor!(xproto::Atom, xcb_xkb_get_kbd_by_name_replies_other_names_value_list_indicator_names_length, xcb_xkb_get_kbd_by_name_replies_other_names_value_list_indicator_names, self.base.strct) }
  }

  pub fn virtualModNames(&mut self) -> Vec<xproto::Atom> {
    unsafe { accessor!(xproto::Atom, xcb_xkb_get_kbd_by_name_replies_other_names_value_list_virtual_mod_names_length, xcb_xkb_get_kbd_by_name_replies_other_names_value_list_virtual_mod_names, self.base.strct) }
  }

  pub fn groups(&mut self) -> Vec<xproto::Atom> {
    unsafe { accessor!(xproto::Atom, xcb_xkb_get_kbd_by_name_replies_other_names_value_list_groups_length, xcb_xkb_get_kbd_by_name_replies_other_names_value_list_groups, self.base.strct) }
  }

  pub fn keyNames(&mut self) -> KeyNameIterator {
    unsafe { accessor!(KeyNameIterator, xcb_xkb_get_kbd_by_name_replies_other_names_value_list_key_names_iterator, self.base.strct) }
  }

  pub fn keyAliases(&mut self) -> KeyAliasIterator {
    unsafe { accessor!(KeyAliasIterator, xcb_xkb_get_kbd_by_name_replies_other_names_value_list_key_aliases_iterator, self.base.strct) }
  }

  pub fn radioGroupNames(&mut self) -> Vec<xproto::Atom> {
    unsafe { accessor!(xproto::Atom, xcb_xkb_get_kbd_by_name_replies_other_names_value_list_radio_group_names_length, xcb_xkb_get_kbd_by_name_replies_other_names_value_list_radio_group_names, self.base.strct) }
  }

}
pub struct GetKbdByNameReplies {pub base : base::Struct<xcb_xkb_get_kbd_by_name_replies_t> }


impl GetKbdByNameReplies {
  pub fn getmap_type(&mut self) -> u8 {
    unsafe { accessor!(getmap_type -> u8, self.base.strct) }
  }

  pub fn typeDeviceID(&mut self) -> u8 {
    unsafe { accessor!(typeDeviceID -> u8, self.base.strct) }
  }

  pub fn getmap_sequence(&mut self) -> u16 {
    unsafe { accessor!(getmap_sequence -> u16, self.base.strct) }
  }

  pub fn getmap_length(&mut self) -> u32 {
    unsafe { accessor!(getmap_length -> u32, self.base.strct) }
  }

  pub fn typeMinKeyCode(&mut self) -> xproto::Keycode {
    unsafe { accessor!(typeMinKeyCode -> xproto::Keycode, self.base.strct) }
  }

  pub fn typeMaxKeyCode(&mut self) -> xproto::Keycode {
    unsafe { accessor!(typeMaxKeyCode -> xproto::Keycode, self.base.strct) }
  }

  pub fn present(&mut self) -> u16 {
    unsafe { accessor!(present -> u16, self.base.strct) }
  }

  pub fn firstType(&mut self) -> u8 {
    unsafe { accessor!(firstType -> u8, self.base.strct) }
  }

  pub fn nTypes(&mut self) -> u8 {
    unsafe { accessor!(nTypes -> u8, self.base.strct) }
  }

  pub fn totalTypes(&mut self) -> u8 {
    unsafe { accessor!(totalTypes -> u8, self.base.strct) }
  }

  pub fn firstKeySym(&mut self) -> xproto::Keycode {
    unsafe { accessor!(firstKeySym -> xproto::Keycode, self.base.strct) }
  }

  pub fn totalSyms(&mut self) -> u16 {
    unsafe { accessor!(totalSyms -> u16, self.base.strct) }
  }

  pub fn nKeySyms(&mut self) -> u8 {
    unsafe { accessor!(nKeySyms -> u8, self.base.strct) }
  }

  pub fn firstKeyAction(&mut self) -> xproto::Keycode {
    unsafe { accessor!(firstKeyAction -> xproto::Keycode, self.base.strct) }
  }

  pub fn totalActions(&mut self) -> u16 {
    unsafe { accessor!(totalActions -> u16, self.base.strct) }
  }

  pub fn nKeyActions(&mut self) -> u8 {
    unsafe { accessor!(nKeyActions -> u8, self.base.strct) }
  }

  pub fn firstKeyBehavior(&mut self) -> xproto::Keycode {
    unsafe { accessor!(firstKeyBehavior -> xproto::Keycode, self.base.strct) }
  }

  pub fn nKeyBehaviors(&mut self) -> u8 {
    unsafe { accessor!(nKeyBehaviors -> u8, self.base.strct) }
  }

  pub fn totalKeyBehaviors(&mut self) -> u8 {
    unsafe { accessor!(totalKeyBehaviors -> u8, self.base.strct) }
  }

  pub fn firstKeyExplicit(&mut self) -> xproto::Keycode {
    unsafe { accessor!(firstKeyExplicit -> xproto::Keycode, self.base.strct) }
  }

  pub fn nKeyExplicit(&mut self) -> u8 {
    unsafe { accessor!(nKeyExplicit -> u8, self.base.strct) }
  }

  pub fn totalKeyExplicit(&mut self) -> u8 {
    unsafe { accessor!(totalKeyExplicit -> u8, self.base.strct) }
  }

  pub fn firstModMapKey(&mut self) -> xproto::Keycode {
    unsafe { accessor!(firstModMapKey -> xproto::Keycode, self.base.strct) }
  }

  pub fn nModMapKeys(&mut self) -> u8 {
    unsafe { accessor!(nModMapKeys -> u8, self.base.strct) }
  }

  pub fn totalModMapKeys(&mut self) -> u8 {
    unsafe { accessor!(totalModMapKeys -> u8, self.base.strct) }
  }

  pub fn firstVModMapKey(&mut self) -> xproto::Keycode {
    unsafe { accessor!(firstVModMapKey -> xproto::Keycode, self.base.strct) }
  }

  pub fn nVModMapKeys(&mut self) -> u8 {
    unsafe { accessor!(nVModMapKeys -> u8, self.base.strct) }
  }

  pub fn totalVModMapKeys(&mut self) -> u8 {
    unsafe { accessor!(totalVModMapKeys -> u8, self.base.strct) }
  }

  pub fn virtualMods(&mut self) -> u16 {
    unsafe { accessor!(virtualMods -> u16, self.base.strct) }
  }

  pub fn map(&self) -> GetKbdByNameRepliesTypesMap {
    unsafe { mem::transmute(self.base.strct.map) }
  }
  pub fn compatDeviceID(&mut self) -> u8 {
    unsafe { accessor!(compatDeviceID -> u8, self.base.strct) }
  }

  pub fn firstSIRtrn(&mut self) -> u16 {
    unsafe { accessor!(firstSIRtrn -> u16, self.base.strct) }
  }

  pub fn nTotalSI(&mut self) -> u16 {
    unsafe { accessor!(nTotalSI -> u16, self.base.strct) }
  }

  pub fn si_rtrn(&mut self) -> Vec<u8> {
    unsafe { accessor!(u8, xcb_xkb_get_kbd_by_name_replies_compat_map_si_rtrn_length, xcb_xkb_get_kbd_by_name_replies_compat_map_si_rtrn, self.base.strct) }
  }

  pub fn group_rtrn(&mut self) -> ModDefIterator {
    unsafe { accessor!(ModDefIterator, xcb_xkb_get_kbd_by_name_replies_compat_map_group_rtrn_iterator, self.base.strct) }
  }

  pub fn clientDeviceID(&mut self) -> u8 {
    unsafe { accessor!(clientDeviceID -> u8, self.base.strct) }
  }

  pub fn clientMinKeyCode(&mut self) -> xproto::Keycode {
    unsafe { accessor!(clientMinKeyCode -> xproto::Keycode, self.base.strct) }
  }

  pub fn clientMaxKeyCode(&mut self) -> xproto::Keycode {
    unsafe { accessor!(clientMaxKeyCode -> xproto::Keycode, self.base.strct) }
  }

  pub fn present(&mut self) -> u16 {
    unsafe { accessor!(present -> u16, self.base.strct) }
  }

  pub fn firstType(&mut self) -> u8 {
    unsafe { accessor!(firstType -> u8, self.base.strct) }
  }

  pub fn nTypes(&mut self) -> u8 {
    unsafe { accessor!(nTypes -> u8, self.base.strct) }
  }

  pub fn totalTypes(&mut self) -> u8 {
    unsafe { accessor!(totalTypes -> u8, self.base.strct) }
  }

  pub fn firstKeySym(&mut self) -> xproto::Keycode {
    unsafe { accessor!(firstKeySym -> xproto::Keycode, self.base.strct) }
  }

  pub fn totalSyms(&mut self) -> u16 {
    unsafe { accessor!(totalSyms -> u16, self.base.strct) }
  }

  pub fn nKeySyms(&mut self) -> u8 {
    unsafe { accessor!(nKeySyms -> u8, self.base.strct) }
  }

  pub fn firstKeyAction(&mut self) -> xproto::Keycode {
    unsafe { accessor!(firstKeyAction -> xproto::Keycode, self.base.strct) }
  }

  pub fn totalActions(&mut self) -> u16 {
    unsafe { accessor!(totalActions -> u16, self.base.strct) }
  }

  pub fn nKeyActions(&mut self) -> u8 {
    unsafe { accessor!(nKeyActions -> u8, self.base.strct) }
  }

  pub fn firstKeyBehavior(&mut self) -> xproto::Keycode {
    unsafe { accessor!(firstKeyBehavior -> xproto::Keycode, self.base.strct) }
  }

  pub fn nKeyBehaviors(&mut self) -> u8 {
    unsafe { accessor!(nKeyBehaviors -> u8, self.base.strct) }
  }

  pub fn totalKeyBehaviors(&mut self) -> u8 {
    unsafe { accessor!(totalKeyBehaviors -> u8, self.base.strct) }
  }

  pub fn firstKeyExplicit(&mut self) -> xproto::Keycode {
    unsafe { accessor!(firstKeyExplicit -> xproto::Keycode, self.base.strct) }
  }

  pub fn nKeyExplicit(&mut self) -> u8 {
    unsafe { accessor!(nKeyExplicit -> u8, self.base.strct) }
  }

  pub fn totalKeyExplicit(&mut self) -> u8 {
    unsafe { accessor!(totalKeyExplicit -> u8, self.base.strct) }
  }

  pub fn firstModMapKey(&mut self) -> xproto::Keycode {
    unsafe { accessor!(firstModMapKey -> xproto::Keycode, self.base.strct) }
  }

  pub fn nModMapKeys(&mut self) -> u8 {
    unsafe { accessor!(nModMapKeys -> u8, self.base.strct) }
  }

  pub fn totalModMapKeys(&mut self) -> u8 {
    unsafe { accessor!(totalModMapKeys -> u8, self.base.strct) }
  }

  pub fn firstVModMapKey(&mut self) -> xproto::Keycode {
    unsafe { accessor!(firstVModMapKey -> xproto::Keycode, self.base.strct) }
  }

  pub fn nVModMapKeys(&mut self) -> u8 {
    unsafe { accessor!(nVModMapKeys -> u8, self.base.strct) }
  }

  pub fn totalVModMapKeys(&mut self) -> u8 {
    unsafe { accessor!(totalVModMapKeys -> u8, self.base.strct) }
  }

  pub fn virtualMods(&mut self) -> u16 {
    unsafe { accessor!(virtualMods -> u16, self.base.strct) }
  }

  pub fn map(&self) -> GetKbdByNameRepliesClientSymbolsMap {
    unsafe { mem::transmute(self.base.strct.map) }
  }
  pub fn serverDeviceID(&mut self) -> u8 {
    unsafe { accessor!(serverDeviceID -> u8, self.base.strct) }
  }

  pub fn serverMinKeyCode(&mut self) -> xproto::Keycode {
    unsafe { accessor!(serverMinKeyCode -> xproto::Keycode, self.base.strct) }
  }

  pub fn serverMaxKeyCode(&mut self) -> xproto::Keycode {
    unsafe { accessor!(serverMaxKeyCode -> xproto::Keycode, self.base.strct) }
  }

  pub fn present(&mut self) -> u16 {
    unsafe { accessor!(present -> u16, self.base.strct) }
  }

  pub fn firstType(&mut self) -> u8 {
    unsafe { accessor!(firstType -> u8, self.base.strct) }
  }

  pub fn nTypes(&mut self) -> u8 {
    unsafe { accessor!(nTypes -> u8, self.base.strct) }
  }

  pub fn totalTypes(&mut self) -> u8 {
    unsafe { accessor!(totalTypes -> u8, self.base.strct) }
  }

  pub fn firstKeySym(&mut self) -> xproto::Keycode {
    unsafe { accessor!(firstKeySym -> xproto::Keycode, self.base.strct) }
  }

  pub fn totalSyms(&mut self) -> u16 {
    unsafe { accessor!(totalSyms -> u16, self.base.strct) }
  }

  pub fn nKeySyms(&mut self) -> u8 {
    unsafe { accessor!(nKeySyms -> u8, self.base.strct) }
  }

  pub fn firstKeyAction(&mut self) -> xproto::Keycode {
    unsafe { accessor!(firstKeyAction -> xproto::Keycode, self.base.strct) }
  }

  pub fn totalActions(&mut self) -> u16 {
    unsafe { accessor!(totalActions -> u16, self.base.strct) }
  }

  pub fn nKeyActions(&mut self) -> u8 {
    unsafe { accessor!(nKeyActions -> u8, self.base.strct) }
  }

  pub fn firstKeyBehavior(&mut self) -> xproto::Keycode {
    unsafe { accessor!(firstKeyBehavior -> xproto::Keycode, self.base.strct) }
  }

  pub fn nKeyBehaviors(&mut self) -> u8 {
    unsafe { accessor!(nKeyBehaviors -> u8, self.base.strct) }
  }

  pub fn totalKeyBehaviors(&mut self) -> u8 {
    unsafe { accessor!(totalKeyBehaviors -> u8, self.base.strct) }
  }

  pub fn firstKeyExplicit(&mut self) -> xproto::Keycode {
    unsafe { accessor!(firstKeyExplicit -> xproto::Keycode, self.base.strct) }
  }

  pub fn nKeyExplicit(&mut self) -> u8 {
    unsafe { accessor!(nKeyExplicit -> u8, self.base.strct) }
  }

  pub fn totalKeyExplicit(&mut self) -> u8 {
    unsafe { accessor!(totalKeyExplicit -> u8, self.base.strct) }
  }

  pub fn firstModMapKey(&mut self) -> xproto::Keycode {
    unsafe { accessor!(firstModMapKey -> xproto::Keycode, self.base.strct) }
  }

  pub fn nModMapKeys(&mut self) -> u8 {
    unsafe { accessor!(nModMapKeys -> u8, self.base.strct) }
  }

  pub fn totalModMapKeys(&mut self) -> u8 {
    unsafe { accessor!(totalModMapKeys -> u8, self.base.strct) }
  }

  pub fn firstVModMapKey(&mut self) -> xproto::Keycode {
    unsafe { accessor!(firstVModMapKey -> xproto::Keycode, self.base.strct) }
  }

  pub fn nVModMapKeys(&mut self) -> u8 {
    unsafe { accessor!(nVModMapKeys -> u8, self.base.strct) }
  }

  pub fn totalVModMapKeys(&mut self) -> u8 {
    unsafe { accessor!(totalVModMapKeys -> u8, self.base.strct) }
  }

  pub fn virtualMods(&mut self) -> u16 {
    unsafe { accessor!(virtualMods -> u16, self.base.strct) }
  }

  pub fn map(&self) -> GetKbdByNameRepliesServerSymbolsMap {
    unsafe { mem::transmute(self.base.strct.map) }
  }
  pub fn indicatorDeviceID(&mut self) -> u8 {
    unsafe { accessor!(indicatorDeviceID -> u8, self.base.strct) }
  }

  pub fn which(&mut self) -> u32 {
    unsafe { accessor!(which -> u32, self.base.strct) }
  }

  pub fn realIndicators(&mut self) -> u32 {
    unsafe { accessor!(realIndicators -> u32, self.base.strct) }
  }

  pub fn maps(&mut self) -> IndicatorMapIterator {
    unsafe { accessor!(IndicatorMapIterator, xcb_xkb_get_kbd_by_name_replies_indicator_maps_maps_iterator, self.base.strct) }
  }

  pub fn keyDeviceID(&mut self) -> u8 {
    unsafe { accessor!(keyDeviceID -> u8, self.base.strct) }
  }

  pub fn which(&mut self) -> u32 {
    unsafe { accessor!(which -> u32, self.base.strct) }
  }

  pub fn keyMinKeyCode(&mut self) -> xproto::Keycode {
    unsafe { accessor!(keyMinKeyCode -> xproto::Keycode, self.base.strct) }
  }

  pub fn keyMaxKeyCode(&mut self) -> xproto::Keycode {
    unsafe { accessor!(keyMaxKeyCode -> xproto::Keycode, self.base.strct) }
  }

  pub fn nTypes(&mut self) -> u8 {
    unsafe { accessor!(nTypes -> u8, self.base.strct) }
  }

  pub fn groupNames(&mut self) -> u8 {
    unsafe { accessor!(groupNames -> u8, self.base.strct) }
  }

  pub fn virtualMods(&mut self) -> u16 {
    unsafe { accessor!(virtualMods -> u16, self.base.strct) }
  }

  pub fn firstKey(&mut self) -> xproto::Keycode {
    unsafe { accessor!(firstKey -> xproto::Keycode, self.base.strct) }
  }

  pub fn nKeys(&mut self) -> u8 {
    unsafe { accessor!(nKeys -> u8, self.base.strct) }
  }

  pub fn indicators(&mut self) -> u32 {
    unsafe { accessor!(indicators -> u32, self.base.strct) }
  }

  pub fn nRadioGroups(&mut self) -> u8 {
    unsafe { accessor!(nRadioGroups -> u8, self.base.strct) }
  }

  pub fn nKeyAliases(&mut self) -> u8 {
    unsafe { accessor!(nKeyAliases -> u8, self.base.strct) }
  }

  pub fn nKTLevels(&mut self) -> u16 {
    unsafe { accessor!(nKTLevels -> u16, self.base.strct) }
  }

  pub fn valueList(&self) -> GetKbdByNameRepliesKeyNamesValueList {
    unsafe { mem::transmute(self.base.strct.valueList) }
  }
  pub fn otherDeviceID(&mut self) -> u8 {
    unsafe { accessor!(otherDeviceID -> u8, self.base.strct) }
  }

  pub fn which(&mut self) -> u32 {
    unsafe { accessor!(which -> u32, self.base.strct) }
  }

  pub fn otherMinKeyCode(&mut self) -> xproto::Keycode {
    unsafe { accessor!(otherMinKeyCode -> xproto::Keycode, self.base.strct) }
  }

  pub fn otherMaxKeyCode(&mut self) -> xproto::Keycode {
    unsafe { accessor!(otherMaxKeyCode -> xproto::Keycode, self.base.strct) }
  }

  pub fn nTypes(&mut self) -> u8 {
    unsafe { accessor!(nTypes -> u8, self.base.strct) }
  }

  pub fn groupNames(&mut self) -> u8 {
    unsafe { accessor!(groupNames -> u8, self.base.strct) }
  }

  pub fn virtualMods(&mut self) -> u16 {
    unsafe { accessor!(virtualMods -> u16, self.base.strct) }
  }

  pub fn firstKey(&mut self) -> xproto::Keycode {
    unsafe { accessor!(firstKey -> xproto::Keycode, self.base.strct) }
  }

  pub fn nKeys(&mut self) -> u8 {
    unsafe { accessor!(nKeys -> u8, self.base.strct) }
  }

  pub fn indicators(&mut self) -> u32 {
    unsafe { accessor!(indicators -> u32, self.base.strct) }
  }

  pub fn nRadioGroups(&mut self) -> u8 {
    unsafe { accessor!(nRadioGroups -> u8, self.base.strct) }
  }

  pub fn nKeyAliases(&mut self) -> u8 {
    unsafe { accessor!(nKeyAliases -> u8, self.base.strct) }
  }

  pub fn nKTLevels(&mut self) -> u16 {
    unsafe { accessor!(nKTLevels -> u16, self.base.strct) }
  }

  pub fn valueList(&self) -> GetKbdByNameRepliesOtherNamesValueList {
    unsafe { mem::transmute(self.base.strct.valueList) }
  }
  pub fn geometryDeviceID(&mut self) -> u8 {
    unsafe { accessor!(geometryDeviceID -> u8, self.base.strct) }
  }

  pub fn name(&mut self) -> xproto::Atom {
    unsafe { accessor!(name -> xproto::Atom, self.base.strct) }
  }

  pub fn geometryFound(&mut self) -> u8 {
    unsafe { accessor!(geometryFound -> u8, self.base.strct) }
  }

  pub fn widthMM(&mut self) -> u16 {
    unsafe { accessor!(widthMM -> u16, self.base.strct) }
  }

  pub fn heightMM(&mut self) -> u16 {
    unsafe { accessor!(heightMM -> u16, self.base.strct) }
  }

  pub fn baseColorNdx(&mut self) -> u8 {
    unsafe { accessor!(baseColorNdx -> u8, self.base.strct) }
  }

  pub fn labelColorNdx(&mut self) -> u8 {
    unsafe { accessor!(labelColorNdx -> u8, self.base.strct) }
  }

  pub fn labelFont(&self) -> CountedString16 {
    unsafe { mem::transmute(self.base.strct.labelFont) }
  }
  pub fn properties(&mut self) -> PropertyIterator {
    unsafe { accessor!(PropertyIterator, xcb_xkb_get_kbd_by_name_replies_geometry_properties_iterator, self.base.strct) }
  }

  pub fn colors(&mut self) -> CountedString16Iterator {
    unsafe { accessor!(CountedString16Iterator, xcb_xkb_get_kbd_by_name_replies_geometry_colors_iterator, self.base.strct) }
  }

  pub fn shapes(&mut self) -> ShapeIterator {
    unsafe { accessor!(ShapeIterator, xcb_xkb_get_kbd_by_name_replies_geometry_shapes_iterator, self.base.strct) }
  }

  pub fn sections(&mut self) -> SectionIterator {
    unsafe { accessor!(SectionIterator, xcb_xkb_get_kbd_by_name_replies_geometry_sections_iterator, self.base.strct) }
  }

  pub fn doodads(&mut self) -> DoodadIterator {
    unsafe { accessor!(DoodadIterator, xcb_xkb_get_kbd_by_name_replies_geometry_doodads_iterator, self.base.strct) }
  }

  pub fn keyAliases(&mut self) -> KeyAliasIterator {
    unsafe { accessor!(KeyAliasIterator, xcb_xkb_get_kbd_by_name_replies_geometry_key_aliases_iterator, self.base.strct) }
  }

}
pub struct GetKbdByNameReply { base:  base::Reply<xcb_xkb_get_kbd_by_name_reply_t> }
fn mk_reply_xcb_xkb_get_kbd_by_name_reply_t(reply:*mut xcb_xkb_get_kbd_by_name_reply_t) -> GetKbdByNameReply { GetKbdByNameReply { base : base::mk_reply(reply) } }
pub fn GetKbdByName<'r> (c : &'r Connection,
                     deviceSpec : DeviceSpec,
                     need : u16,
                     want : u16,
                     load : u8,
                     keymapsSpec : &[String8],
                     keycodesSpec : &[String8],
                     typesSpec : &[String8],
                     compatMapSpec : &[String8],
                     symbolsSpec : &[String8],
                     geometrySpec : &[String8]) -> GetKbdByNameCookie<'r> {
  unsafe {
    let keymapsSpec_len = keymapsSpec.len();
    let keymapsSpec_ptr = keymapsSpec.as_ptr();
    let keycodesSpec_len = keycodesSpec.len();
    let keycodesSpec_ptr = keycodesSpec.as_ptr();
    let typesSpec_len = typesSpec.len();
    let typesSpec_ptr = typesSpec.as_ptr();
    let compatMapSpec_len = compatMapSpec.len();
    let compatMapSpec_ptr = compatMapSpec.as_ptr();
    let symbolsSpec_len = symbolsSpec.len();
    let symbolsSpec_ptr = symbolsSpec.as_ptr();
    let geometrySpec_len = geometrySpec.len();
    let geometrySpec_ptr = geometrySpec.as_ptr();
    let cookie = xcb_xkb_get_kbd_by_name(c.get_raw_conn(),
        deviceSpec as xcb_xkb_device_spec_t, //1
        need as u16, //2
        want as u16, //3
        load as u8, //4
        keymapsSpec_len as u8, //5
        keymapsSpec_ptr as *mut xcb_xkb_string8_t, //6
        keycodesSpec_len as u8, //7
        keycodesSpec_ptr as *mut xcb_xkb_string8_t, //8
        typesSpec_len as u8, //9
        typesSpec_ptr as *mut xcb_xkb_string8_t, //10
        compatMapSpec_len as u8, //11
        compatMapSpec_ptr as *mut xcb_xkb_string8_t, //12
        symbolsSpec_len as u8, //13
        symbolsSpec_ptr as *mut xcb_xkb_string8_t, //14
        geometrySpec_len as u8, //15
        geometrySpec_ptr as *mut xcb_xkb_string8_t); //16
    GetKbdByNameCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetKbdByNameUnchecked<'r> (c : &'r Connection,
                              deviceSpec : DeviceSpec,
                              need : u16,
                              want : u16,
                              load : u8,
                              keymapsSpec : &[String8],
                              keycodesSpec : &[String8],
                              typesSpec : &[String8],
                              compatMapSpec : &[String8],
                              symbolsSpec : &[String8],
                              geometrySpec : &[String8]) -> GetKbdByNameCookie<'r> {
  unsafe {
    let keymapsSpec_len = keymapsSpec.len();
    let keymapsSpec_ptr = keymapsSpec.as_ptr();
    let keycodesSpec_len = keycodesSpec.len();
    let keycodesSpec_ptr = keycodesSpec.as_ptr();
    let typesSpec_len = typesSpec.len();
    let typesSpec_ptr = typesSpec.as_ptr();
    let compatMapSpec_len = compatMapSpec.len();
    let compatMapSpec_ptr = compatMapSpec.as_ptr();
    let symbolsSpec_len = symbolsSpec.len();
    let symbolsSpec_ptr = symbolsSpec.as_ptr();
    let geometrySpec_len = geometrySpec.len();
    let geometrySpec_ptr = geometrySpec.as_ptr();
    let cookie = xcb_xkb_get_kbd_by_name_unchecked(c.get_raw_conn(),
        deviceSpec as xcb_xkb_device_spec_t, //1
        need as u16, //2
        want as u16, //3
        load as u8, //4
        keymapsSpec_len as u8, //5
        keymapsSpec_ptr as *mut xcb_xkb_string8_t, //6
        keycodesSpec_len as u8, //7
        keycodesSpec_ptr as *mut xcb_xkb_string8_t, //8
        typesSpec_len as u8, //9
        typesSpec_ptr as *mut xcb_xkb_string8_t, //10
        compatMapSpec_len as u8, //11
        compatMapSpec_ptr as *mut xcb_xkb_string8_t, //12
        symbolsSpec_len as u8, //13
        symbolsSpec_ptr as *mut xcb_xkb_string8_t, //14
        geometrySpec_len as u8, //15
        geometrySpec_ptr as *mut xcb_xkb_string8_t); //16
    GetKbdByNameCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetKbdByNameReply {
  pub fn deviceID(&mut self) -> u8 {
    unsafe { accessor!(deviceID -> u8, (*self.base.reply)) }
  }

  pub fn minKeyCode(&mut self) -> xproto::Keycode {
    unsafe { accessor!(minKeyCode -> xproto::Keycode, (*self.base.reply)) }
  }

  pub fn maxKeyCode(&mut self) -> xproto::Keycode {
    unsafe { accessor!(maxKeyCode -> xproto::Keycode, (*self.base.reply)) }
  }

  pub fn loaded(&mut self) -> u8 {
    unsafe { accessor!(loaded -> u8, (*self.base.reply)) }
  }

  pub fn newKeyboard(&mut self) -> u8 {
    unsafe { accessor!(newKeyboard -> u8, (*self.base.reply)) }
  }

  pub fn found(&mut self) -> u16 {
    unsafe { accessor!(found -> u16, (*self.base.reply)) }
  }

  pub fn reported(&mut self) -> u16 {
    unsafe { accessor!(reported -> u16, (*self.base.reply)) }
  }

  pub fn replies(&self) -> GetKbdByNameReplies {
    unsafe { mem::transmute((*self.base.reply).replies) }
  }
}
impl_reply_cookie!(GetKbdByNameCookie<'s>, mk_reply_xcb_xkb_get_kbd_by_name_reply_t, GetKbdByNameReply, xcb_xkb_get_kbd_by_name_reply);

pub struct GetDeviceInfoReply { base:  base::Reply<xcb_xkb_get_device_info_reply_t> }
fn mk_reply_xcb_xkb_get_device_info_reply_t(reply:*mut xcb_xkb_get_device_info_reply_t) -> GetDeviceInfoReply { GetDeviceInfoReply { base : base::mk_reply(reply) } }
pub fn GetDeviceInfo<'r> (c : &'r Connection,
                      deviceSpec : DeviceSpec,
                      wanted : u16,
                      allButtons : u8,
                      firstButton : u8,
                      nButtons : u8,
                      ledClass : LedClassSpec,
                      ledID : IdSpec) -> GetDeviceInfoCookie<'r> {
  unsafe {
    let cookie = xcb_xkb_get_device_info(c.get_raw_conn(),
        deviceSpec as xcb_xkb_device_spec_t, //1
        wanted as u16, //2
        allButtons as u8, //3
        firstButton as u8, //4
        nButtons as u8, //5
        ledClass as xcb_xkb_led_class_spec_t, //6
        ledID as xcb_xkb_id_spec_t); //7
    GetDeviceInfoCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetDeviceInfoUnchecked<'r> (c : &'r Connection,
                               deviceSpec : DeviceSpec,
                               wanted : u16,
                               allButtons : u8,
                               firstButton : u8,
                               nButtons : u8,
                               ledClass : LedClassSpec,
                               ledID : IdSpec) -> GetDeviceInfoCookie<'r> {
  unsafe {
    let cookie = xcb_xkb_get_device_info_unchecked(c.get_raw_conn(),
        deviceSpec as xcb_xkb_device_spec_t, //1
        wanted as u16, //2
        allButtons as u8, //3
        firstButton as u8, //4
        nButtons as u8, //5
        ledClass as xcb_xkb_led_class_spec_t, //6
        ledID as xcb_xkb_id_spec_t); //7
    GetDeviceInfoCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetDeviceInfoReply {
  pub fn deviceID(&mut self) -> u8 {
    unsafe { accessor!(deviceID -> u8, (*self.base.reply)) }
  }

  pub fn present(&mut self) -> u16 {
    unsafe { accessor!(present -> u16, (*self.base.reply)) }
  }

  pub fn supported(&mut self) -> u16 {
    unsafe { accessor!(supported -> u16, (*self.base.reply)) }
  }

  pub fn unsupported(&mut self) -> u16 {
    unsafe { accessor!(unsupported -> u16, (*self.base.reply)) }
  }

  pub fn firstBtnWanted(&mut self) -> u8 {
    unsafe { accessor!(firstBtnWanted -> u8, (*self.base.reply)) }
  }

  pub fn nBtnsWanted(&mut self) -> u8 {
    unsafe { accessor!(nBtnsWanted -> u8, (*self.base.reply)) }
  }

  pub fn firstBtnRtrn(&mut self) -> u8 {
    unsafe { accessor!(firstBtnRtrn -> u8, (*self.base.reply)) }
  }

  pub fn totalBtns(&mut self) -> u8 {
    unsafe { accessor!(totalBtns -> u8, (*self.base.reply)) }
  }

  pub fn hasOwnState(&mut self) -> u8 {
    unsafe { accessor!(hasOwnState -> u8, (*self.base.reply)) }
  }

  pub fn dfltKbdFB(&mut self) -> u16 {
    unsafe { accessor!(dfltKbdFB -> u16, (*self.base.reply)) }
  }

  pub fn dfltLedFB(&mut self) -> u16 {
    unsafe { accessor!(dfltLedFB -> u16, (*self.base.reply)) }
  }

  pub fn devType(&mut self) -> xproto::Atom {
    unsafe { accessor!(devType -> xproto::Atom, (*self.base.reply)) }
  }

  pub fn name(&mut self) -> Vec<String8> {
    unsafe { accessor!(String8, xcb_xkb_get_device_info_name_length, xcb_xkb_get_device_info_name, (*self.base.reply)) }
  }

  pub fn btnActions(&mut self) -> ActionIterator {
    unsafe { accessor!(ActionIterator, xcb_xkb_get_device_info_btn_actions_iterator, (*self.base.reply)) }
  }

  pub fn leds(&mut self) -> DeviceLedInfoIterator {
    unsafe { accessor!(DeviceLedInfoIterator, xcb_xkb_get_device_info_leds_iterator, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetDeviceInfoCookie<'s>, mk_reply_xcb_xkb_get_device_info_reply_t, GetDeviceInfoReply, xcb_xkb_get_device_info_reply);

pub fn SetDeviceInfoChecked<'r> (c : &'r Connection,
                             deviceSpec : DeviceSpec,
                             firstBtn : u8,
                             change : u16,
                             btnActions : &[Action],
                             leds : &[DeviceLedInfo]) -> base::VoidCookie<'r> {
  unsafe {
    let btnActions_len = btnActions.len();
    let btnActions_ptr = btnActions.as_ptr();
    let leds_len = leds.len();
    let leds_ptr = leds.as_ptr();
    let cookie = xcb_xkb_set_device_info_checked(c.get_raw_conn(),
        deviceSpec as xcb_xkb_device_spec_t, //1
        firstBtn as u8, //2
        btnActions_len as u8, //3
        change as u16, //4
        leds_len as u16, //5
        btnActions_ptr as *mut xcb_xkb_action_t, //6
        leds_ptr as *mut xcb_xkb_device_led_info_t); //7
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn SetDeviceInfo<'r> (c : &'r Connection,
                      deviceSpec : DeviceSpec,
                      firstBtn : u8,
                      change : u16,
                      btnActions : &[Action],
                      leds : &[DeviceLedInfo]) -> base::VoidCookie<'r> {
  unsafe {
    let btnActions_len = btnActions.len();
    let btnActions_ptr = btnActions.as_ptr();
    let leds_len = leds.len();
    let leds_ptr = leds.as_ptr();
    let cookie = xcb_xkb_set_device_info(c.get_raw_conn(),
        deviceSpec as xcb_xkb_device_spec_t, //1
        firstBtn as u8, //2
        btnActions_len as u8, //3
        change as u16, //4
        leds_len as u16, //5
        btnActions_ptr as *mut xcb_xkb_action_t, //6
        leds_ptr as *mut xcb_xkb_device_led_info_t); //7
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn SetDebuggingFlags<'r> (c : &'r Connection,
                          affectFlags : u32,
                          flags : u32,
                          affectCtrls : u32,
                          ctrls : u32,
                          message : &[String8]) -> SetDebuggingFlagsCookie<'r> {
  unsafe {
    let message_len = message.len();
    let message_ptr = message.as_ptr();
    let cookie = xcb_xkb_set_debugging_flags(c.get_raw_conn(),
        message_len as u16, //1
        affectFlags as u32, //2
        flags as u32, //3
        affectCtrls as u32, //4
        ctrls as u32, //5
        message_ptr as *mut xcb_xkb_string8_t); //6
    SetDebuggingFlagsCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn SetDebuggingFlagsUnchecked<'r> (c : &'r Connection,
                                   affectFlags : u32,
                                   flags : u32,
                                   affectCtrls : u32,
                                   ctrls : u32,
                                   message : &[String8]) -> SetDebuggingFlagsCookie<'r> {
  unsafe {
    let message_len = message.len();
    let message_ptr = message.as_ptr();
    let cookie = xcb_xkb_set_debugging_flags_unchecked(c.get_raw_conn(),
        message_len as u16, //1
        affectFlags as u32, //2
        flags as u32, //3
        affectCtrls as u32, //4
        ctrls as u32, //5
        message_ptr as *mut xcb_xkb_string8_t); //6
    SetDebuggingFlagsCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl SetDebuggingFlagsReply {
  pub fn currentFlags(&mut self) -> u32 {
    unsafe { accessor!(currentFlags -> u32, (*self.base.reply)) }
  }

  pub fn currentCtrls(&mut self) -> u32 {
    unsafe { accessor!(currentCtrls -> u32, (*self.base.reply)) }
  }

  pub fn supportedFlags(&mut self) -> u32 {
    unsafe { accessor!(supportedFlags -> u32, (*self.base.reply)) }
  }

  pub fn supportedCtrls(&mut self) -> u32 {
    unsafe { accessor!(supportedCtrls -> u32, (*self.base.reply)) }
  }

}
impl_reply_cookie!(SetDebuggingFlagsCookie<'s>, mk_reply_xcb_xkb_set_debugging_flags_reply_t, SetDebuggingFlagsReply, xcb_xkb_set_debugging_flags_reply);


impl NewKeyboardNotifyEvent {
  pub fn xkbType(&mut self) -> u8 {
    unsafe { accessor!(xkbType -> u8, (*self.base.event)) }
  }

  pub fn sequence(&mut self) -> u16 {
    unsafe { accessor!(sequence -> u16, (*self.base.event)) }
  }

  pub fn time(&mut self) -> xproto::Timestamp {
    unsafe { accessor!(time -> xproto::Timestamp, (*self.base.event)) }
  }

  pub fn deviceID(&mut self) -> u8 {
    unsafe { accessor!(deviceID -> u8, (*self.base.event)) }
  }

  pub fn oldDeviceID(&mut self) -> u8 {
    unsafe { accessor!(oldDeviceID -> u8, (*self.base.event)) }
  }

  pub fn minKeyCode(&mut self) -> xproto::Keycode {
    unsafe { accessor!(minKeyCode -> xproto::Keycode, (*self.base.event)) }
  }

  pub fn maxKeyCode(&mut self) -> xproto::Keycode {
    unsafe { accessor!(maxKeyCode -> xproto::Keycode, (*self.base.event)) }
  }

  pub fn oldMinKeyCode(&mut self) -> xproto::Keycode {
    unsafe { accessor!(oldMinKeyCode -> xproto::Keycode, (*self.base.event)) }
  }

  pub fn oldMaxKeyCode(&mut self) -> xproto::Keycode {
    unsafe { accessor!(oldMaxKeyCode -> xproto::Keycode, (*self.base.event)) }
  }

  pub fn requestMajor(&mut self) -> u8 {
    unsafe { accessor!(requestMajor -> u8, (*self.base.event)) }
  }

  pub fn requestMinor(&mut self) -> u8 {
    unsafe { accessor!(requestMinor -> u8, (*self.base.event)) }
  }

  pub fn changed(&mut self) -> u16 {
    unsafe { accessor!(changed -> u16, (*self.base.event)) }
  }

  pub fn new(xkbType : u8,
         sequence : u16,
         time : xproto::Timestamp,
         deviceID : u8,
         oldDeviceID : u8,
         minKeyCode : xproto::Keycode,
         maxKeyCode : xproto::Keycode,
         oldMinKeyCode : xproto::Keycode,
         oldMaxKeyCode : xproto::Keycode,
         requestMajor : u8,
         requestMinor : u8,
         changed : u16) -> NewKeyboardNotifyEvent {
    unsafe {
      let raw = malloc(32 as size_t) as *mut xcb_xkb_new_keyboard_notify_event_t;
      (*raw).xkbType = xkbType;
      (*raw).sequence = sequence;
      (*raw).time = time;
      (*raw).deviceID = deviceID;
      (*raw).oldDeviceID = oldDeviceID;
      (*raw).minKeyCode = minKeyCode;
      (*raw).maxKeyCode = maxKeyCode;
      (*raw).oldMinKeyCode = oldMinKeyCode;
      (*raw).oldMaxKeyCode = oldMaxKeyCode;
      (*raw).requestMajor = requestMajor;
      (*raw).requestMinor = requestMinor;
      (*raw).changed = changed;
      NewKeyboardNotifyEvent { base : Event { event : raw as *mut xcb_xkb_new_keyboard_notify_event_t }}
    }
  }
}

impl MapNotifyEvent {
  pub fn xkbType(&mut self) -> u8 {
    unsafe { accessor!(xkbType -> u8, (*self.base.event)) }
  }

  pub fn sequence(&mut self) -> u16 {
    unsafe { accessor!(sequence -> u16, (*self.base.event)) }
  }

  pub fn time(&mut self) -> xproto::Timestamp {
    unsafe { accessor!(time -> xproto::Timestamp, (*self.base.event)) }
  }

  pub fn deviceID(&mut self) -> u8 {
    unsafe { accessor!(deviceID -> u8, (*self.base.event)) }
  }

  pub fn ptrBtnActions(&mut self) -> u8 {
    unsafe { accessor!(ptrBtnActions -> u8, (*self.base.event)) }
  }

  pub fn changed(&mut self) -> u16 {
    unsafe { accessor!(changed -> u16, (*self.base.event)) }
  }

  pub fn minKeyCode(&mut self) -> xproto::Keycode {
    unsafe { accessor!(minKeyCode -> xproto::Keycode, (*self.base.event)) }
  }

  pub fn maxKeyCode(&mut self) -> xproto::Keycode {
    unsafe { accessor!(maxKeyCode -> xproto::Keycode, (*self.base.event)) }
  }

  pub fn firstType(&mut self) -> u8 {
    unsafe { accessor!(firstType -> u8, (*self.base.event)) }
  }

  pub fn nTypes(&mut self) -> u8 {
    unsafe { accessor!(nTypes -> u8, (*self.base.event)) }
  }

  pub fn firstKeySym(&mut self) -> xproto::Keycode {
    unsafe { accessor!(firstKeySym -> xproto::Keycode, (*self.base.event)) }
  }

  pub fn nKeySyms(&mut self) -> u8 {
    unsafe { accessor!(nKeySyms -> u8, (*self.base.event)) }
  }

  pub fn firstKeyAct(&mut self) -> xproto::Keycode {
    unsafe { accessor!(firstKeyAct -> xproto::Keycode, (*self.base.event)) }
  }

  pub fn nKeyActs(&mut self) -> u8 {
    unsafe { accessor!(nKeyActs -> u8, (*self.base.event)) }
  }

  pub fn firstKeyBehavior(&mut self) -> xproto::Keycode {
    unsafe { accessor!(firstKeyBehavior -> xproto::Keycode, (*self.base.event)) }
  }

  pub fn nKeyBehavior(&mut self) -> u8 {
    unsafe { accessor!(nKeyBehavior -> u8, (*self.base.event)) }
  }

  pub fn firstKeyExplicit(&mut self) -> xproto::Keycode {
    unsafe { accessor!(firstKeyExplicit -> xproto::Keycode, (*self.base.event)) }
  }

  pub fn nKeyExplicit(&mut self) -> u8 {
    unsafe { accessor!(nKeyExplicit -> u8, (*self.base.event)) }
  }

  pub fn firstModMapKey(&mut self) -> xproto::Keycode {
    unsafe { accessor!(firstModMapKey -> xproto::Keycode, (*self.base.event)) }
  }

  pub fn nModMapKeys(&mut self) -> u8 {
    unsafe { accessor!(nModMapKeys -> u8, (*self.base.event)) }
  }

  pub fn firstVModMapKey(&mut self) -> xproto::Keycode {
    unsafe { accessor!(firstVModMapKey -> xproto::Keycode, (*self.base.event)) }
  }

  pub fn nVModMapKeys(&mut self) -> u8 {
    unsafe { accessor!(nVModMapKeys -> u8, (*self.base.event)) }
  }

  pub fn virtualMods(&mut self) -> u16 {
    unsafe { accessor!(virtualMods -> u16, (*self.base.event)) }
  }

  pub fn new(xkbType : u8,
         sequence : u16,
         time : xproto::Timestamp,
         deviceID : u8,
         ptrBtnActions : u8,
         changed : u16,
         minKeyCode : xproto::Keycode,
         maxKeyCode : xproto::Keycode,
         firstType : u8,
         nTypes : u8,
         firstKeySym : xproto::Keycode,
         nKeySyms : u8,
         firstKeyAct : xproto::Keycode,
         nKeyActs : u8,
         firstKeyBehavior : xproto::Keycode,
         nKeyBehavior : u8,
         firstKeyExplicit : xproto::Keycode,
         nKeyExplicit : u8,
         firstModMapKey : xproto::Keycode,
         nModMapKeys : u8,
         firstVModMapKey : xproto::Keycode,
         nVModMapKeys : u8,
         virtualMods : u16) -> MapNotifyEvent {
    unsafe {
      let raw = malloc(32 as size_t) as *mut xcb_xkb_map_notify_event_t;
      (*raw).xkbType = xkbType;
      (*raw).sequence = sequence;
      (*raw).time = time;
      (*raw).deviceID = deviceID;
      (*raw).ptrBtnActions = ptrBtnActions;
      (*raw).changed = changed;
      (*raw).minKeyCode = minKeyCode;
      (*raw).maxKeyCode = maxKeyCode;
      (*raw).firstType = firstType;
      (*raw).nTypes = nTypes;
      (*raw).firstKeySym = firstKeySym;
      (*raw).nKeySyms = nKeySyms;
      (*raw).firstKeyAct = firstKeyAct;
      (*raw).nKeyActs = nKeyActs;
      (*raw).firstKeyBehavior = firstKeyBehavior;
      (*raw).nKeyBehavior = nKeyBehavior;
      (*raw).firstKeyExplicit = firstKeyExplicit;
      (*raw).nKeyExplicit = nKeyExplicit;
      (*raw).firstModMapKey = firstModMapKey;
      (*raw).nModMapKeys = nModMapKeys;
      (*raw).firstVModMapKey = firstVModMapKey;
      (*raw).nVModMapKeys = nVModMapKeys;
      (*raw).virtualMods = virtualMods;
      MapNotifyEvent { base : Event { event : raw as *mut xcb_xkb_map_notify_event_t }}
    }
  }
}

impl StateNotifyEvent {
  pub fn xkbType(&mut self) -> u8 {
    unsafe { accessor!(xkbType -> u8, (*self.base.event)) }
  }

  pub fn sequence(&mut self) -> u16 {
    unsafe { accessor!(sequence -> u16, (*self.base.event)) }
  }

  pub fn time(&mut self) -> xproto::Timestamp {
    unsafe { accessor!(time -> xproto::Timestamp, (*self.base.event)) }
  }

  pub fn deviceID(&mut self) -> u8 {
    unsafe { accessor!(deviceID -> u8, (*self.base.event)) }
  }

  pub fn mods(&mut self) -> u8 {
    unsafe { accessor!(mods -> u8, (*self.base.event)) }
  }

  pub fn baseMods(&mut self) -> u8 {
    unsafe { accessor!(baseMods -> u8, (*self.base.event)) }
  }

  pub fn latchedMods(&mut self) -> u8 {
    unsafe { accessor!(latchedMods -> u8, (*self.base.event)) }
  }

  pub fn lockedMods(&mut self) -> u8 {
    unsafe { accessor!(lockedMods -> u8, (*self.base.event)) }
  }

  pub fn group(&mut self) -> u8 {
    unsafe { accessor!(group -> u8, (*self.base.event)) }
  }

  pub fn baseGroup(&mut self) -> i16 {
    unsafe { accessor!(baseGroup -> i16, (*self.base.event)) }
  }

  pub fn latchedGroup(&mut self) -> i16 {
    unsafe { accessor!(latchedGroup -> i16, (*self.base.event)) }
  }

  pub fn lockedGroup(&mut self) -> u8 {
    unsafe { accessor!(lockedGroup -> u8, (*self.base.event)) }
  }

  pub fn compatState(&mut self) -> u8 {
    unsafe { accessor!(compatState -> u8, (*self.base.event)) }
  }

  pub fn grabMods(&mut self) -> u8 {
    unsafe { accessor!(grabMods -> u8, (*self.base.event)) }
  }

  pub fn compatGrabMods(&mut self) -> u8 {
    unsafe { accessor!(compatGrabMods -> u8, (*self.base.event)) }
  }

  pub fn lookupMods(&mut self) -> u8 {
    unsafe { accessor!(lookupMods -> u8, (*self.base.event)) }
  }

  pub fn compatLoockupMods(&mut self) -> u8 {
    unsafe { accessor!(compatLoockupMods -> u8, (*self.base.event)) }
  }

  pub fn ptrBtnState(&mut self) -> u16 {
    unsafe { accessor!(ptrBtnState -> u16, (*self.base.event)) }
  }

  pub fn changed(&mut self) -> u16 {
    unsafe { accessor!(changed -> u16, (*self.base.event)) }
  }

  pub fn keycode(&mut self) -> xproto::Keycode {
    unsafe { accessor!(keycode -> xproto::Keycode, (*self.base.event)) }
  }

  pub fn eventType(&mut self) -> u8 {
    unsafe { accessor!(eventType -> u8, (*self.base.event)) }
  }

  pub fn requestMajor(&mut self) -> u8 {
    unsafe { accessor!(requestMajor -> u8, (*self.base.event)) }
  }

  pub fn requestMinor(&mut self) -> u8 {
    unsafe { accessor!(requestMinor -> u8, (*self.base.event)) }
  }

  pub fn new(xkbType : u8,
         sequence : u16,
         time : xproto::Timestamp,
         deviceID : u8,
         mods : u8,
         baseMods : u8,
         latchedMods : u8,
         lockedMods : u8,
         group : u8,
         baseGroup : i16,
         latchedGroup : i16,
         lockedGroup : u8,
         compatState : u8,
         grabMods : u8,
         compatGrabMods : u8,
         lookupMods : u8,
         compatLoockupMods : u8,
         ptrBtnState : u16,
         changed : u16,
         keycode : xproto::Keycode,
         eventType : u8,
         requestMajor : u8,
         requestMinor : u8) -> StateNotifyEvent {
    unsafe {
      let raw = malloc(32 as size_t) as *mut xcb_xkb_state_notify_event_t;
      (*raw).xkbType = xkbType;
      (*raw).sequence = sequence;
      (*raw).time = time;
      (*raw).deviceID = deviceID;
      (*raw).mods = mods;
      (*raw).baseMods = baseMods;
      (*raw).latchedMods = latchedMods;
      (*raw).lockedMods = lockedMods;
      (*raw).group = group;
      (*raw).baseGroup = baseGroup;
      (*raw).latchedGroup = latchedGroup;
      (*raw).lockedGroup = lockedGroup;
      (*raw).compatState = compatState;
      (*raw).grabMods = grabMods;
      (*raw).compatGrabMods = compatGrabMods;
      (*raw).lookupMods = lookupMods;
      (*raw).compatLoockupMods = compatLoockupMods;
      (*raw).ptrBtnState = ptrBtnState;
      (*raw).changed = changed;
      (*raw).keycode = keycode;
      (*raw).eventType = eventType;
      (*raw).requestMajor = requestMajor;
      (*raw).requestMinor = requestMinor;
      StateNotifyEvent { base : Event { event : raw as *mut xcb_xkb_state_notify_event_t }}
    }
  }
}

impl ControlsNotifyEvent {
  pub fn xkbType(&mut self) -> u8 {
    unsafe { accessor!(xkbType -> u8, (*self.base.event)) }
  }

  pub fn sequence(&mut self) -> u16 {
    unsafe { accessor!(sequence -> u16, (*self.base.event)) }
  }

  pub fn time(&mut self) -> xproto::Timestamp {
    unsafe { accessor!(time -> xproto::Timestamp, (*self.base.event)) }
  }

  pub fn deviceID(&mut self) -> u8 {
    unsafe { accessor!(deviceID -> u8, (*self.base.event)) }
  }

  pub fn numGroups(&mut self) -> u8 {
    unsafe { accessor!(numGroups -> u8, (*self.base.event)) }
  }

  pub fn changedControls(&mut self) -> u32 {
    unsafe { accessor!(changedControls -> u32, (*self.base.event)) }
  }

  pub fn enabledControls(&mut self) -> u32 {
    unsafe { accessor!(enabledControls -> u32, (*self.base.event)) }
  }

  pub fn enabledControlChanges(&mut self) -> u32 {
    unsafe { accessor!(enabledControlChanges -> u32, (*self.base.event)) }
  }

  pub fn keycode(&mut self) -> xproto::Keycode {
    unsafe { accessor!(keycode -> xproto::Keycode, (*self.base.event)) }
  }

  pub fn eventType(&mut self) -> u8 {
    unsafe { accessor!(eventType -> u8, (*self.base.event)) }
  }

  pub fn requestMajor(&mut self) -> u8 {
    unsafe { accessor!(requestMajor -> u8, (*self.base.event)) }
  }

  pub fn requestMinor(&mut self) -> u8 {
    unsafe { accessor!(requestMinor -> u8, (*self.base.event)) }
  }

  pub fn new(xkbType : u8,
         sequence : u16,
         time : xproto::Timestamp,
         deviceID : u8,
         numGroups : u8,
         changedControls : u32,
         enabledControls : u32,
         enabledControlChanges : u32,
         keycode : xproto::Keycode,
         eventType : u8,
         requestMajor : u8,
         requestMinor : u8) -> ControlsNotifyEvent {
    unsafe {
      let raw = malloc(32 as size_t) as *mut xcb_xkb_controls_notify_event_t;
      (*raw).xkbType = xkbType;
      (*raw).sequence = sequence;
      (*raw).time = time;
      (*raw).deviceID = deviceID;
      (*raw).numGroups = numGroups;
      (*raw).changedControls = changedControls;
      (*raw).enabledControls = enabledControls;
      (*raw).enabledControlChanges = enabledControlChanges;
      (*raw).keycode = keycode;
      (*raw).eventType = eventType;
      (*raw).requestMajor = requestMajor;
      (*raw).requestMinor = requestMinor;
      ControlsNotifyEvent { base : Event { event : raw as *mut xcb_xkb_controls_notify_event_t }}
    }
  }
}

impl IndicatorStateNotifyEvent {
  pub fn xkbType(&mut self) -> u8 {
    unsafe { accessor!(xkbType -> u8, (*self.base.event)) }
  }

  pub fn sequence(&mut self) -> u16 {
    unsafe { accessor!(sequence -> u16, (*self.base.event)) }
  }

  pub fn time(&mut self) -> xproto::Timestamp {
    unsafe { accessor!(time -> xproto::Timestamp, (*self.base.event)) }
  }

  pub fn deviceID(&mut self) -> u8 {
    unsafe { accessor!(deviceID -> u8, (*self.base.event)) }
  }

  pub fn state(&mut self) -> u32 {
    unsafe { accessor!(state -> u32, (*self.base.event)) }
  }

  pub fn stateChanged(&mut self) -> u32 {
    unsafe { accessor!(stateChanged -> u32, (*self.base.event)) }
  }

  pub fn new(xkbType : u8,
         sequence : u16,
         time : xproto::Timestamp,
         deviceID : u8,
         state : u32,
         stateChanged : u32) -> IndicatorStateNotifyEvent {
    unsafe {
      let raw = malloc(32 as size_t) as *mut xcb_xkb_indicator_state_notify_event_t;
      (*raw).xkbType = xkbType;
      (*raw).sequence = sequence;
      (*raw).time = time;
      (*raw).deviceID = deviceID;
      (*raw).state = state;
      (*raw).stateChanged = stateChanged;
      IndicatorStateNotifyEvent { base : Event { event : raw as *mut xcb_xkb_indicator_state_notify_event_t }}
    }
  }
}

impl IndicatorMapNotifyEvent {
  pub fn xkbType(&mut self) -> u8 {
    unsafe { accessor!(xkbType -> u8, (*self.base.event)) }
  }

  pub fn sequence(&mut self) -> u16 {
    unsafe { accessor!(sequence -> u16, (*self.base.event)) }
  }

  pub fn time(&mut self) -> xproto::Timestamp {
    unsafe { accessor!(time -> xproto::Timestamp, (*self.base.event)) }
  }

  pub fn deviceID(&mut self) -> u8 {
    unsafe { accessor!(deviceID -> u8, (*self.base.event)) }
  }

  pub fn state(&mut self) -> u32 {
    unsafe { accessor!(state -> u32, (*self.base.event)) }
  }

  pub fn mapChanged(&mut self) -> u32 {
    unsafe { accessor!(mapChanged -> u32, (*self.base.event)) }
  }

  pub fn new(xkbType : u8,
         sequence : u16,
         time : xproto::Timestamp,
         deviceID : u8,
         state : u32,
         mapChanged : u32) -> IndicatorMapNotifyEvent {
    unsafe {
      let raw = malloc(32 as size_t) as *mut xcb_xkb_indicator_map_notify_event_t;
      (*raw).xkbType = xkbType;
      (*raw).sequence = sequence;
      (*raw).time = time;
      (*raw).deviceID = deviceID;
      (*raw).state = state;
      (*raw).mapChanged = mapChanged;
      IndicatorMapNotifyEvent { base : Event { event : raw as *mut xcb_xkb_indicator_map_notify_event_t }}
    }
  }
}

impl NamesNotifyEvent {
  pub fn xkbType(&mut self) -> u8 {
    unsafe { accessor!(xkbType -> u8, (*self.base.event)) }
  }

  pub fn sequence(&mut self) -> u16 {
    unsafe { accessor!(sequence -> u16, (*self.base.event)) }
  }

  pub fn time(&mut self) -> xproto::Timestamp {
    unsafe { accessor!(time -> xproto::Timestamp, (*self.base.event)) }
  }

  pub fn deviceID(&mut self) -> u8 {
    unsafe { accessor!(deviceID -> u8, (*self.base.event)) }
  }

  pub fn changed(&mut self) -> u16 {
    unsafe { accessor!(changed -> u16, (*self.base.event)) }
  }

  pub fn firstType(&mut self) -> u8 {
    unsafe { accessor!(firstType -> u8, (*self.base.event)) }
  }

  pub fn nTypes(&mut self) -> u8 {
    unsafe { accessor!(nTypes -> u8, (*self.base.event)) }
  }

  pub fn firstLevelName(&mut self) -> u8 {
    unsafe { accessor!(firstLevelName -> u8, (*self.base.event)) }
  }

  pub fn nLevelNames(&mut self) -> u8 {
    unsafe { accessor!(nLevelNames -> u8, (*self.base.event)) }
  }

  pub fn nRadioGroups(&mut self) -> u8 {
    unsafe { accessor!(nRadioGroups -> u8, (*self.base.event)) }
  }

  pub fn nKeyAliases(&mut self) -> u8 {
    unsafe { accessor!(nKeyAliases -> u8, (*self.base.event)) }
  }

  pub fn changedGroupNames(&mut self) -> u8 {
    unsafe { accessor!(changedGroupNames -> u8, (*self.base.event)) }
  }

  pub fn changedVirtualMods(&mut self) -> u16 {
    unsafe { accessor!(changedVirtualMods -> u16, (*self.base.event)) }
  }

  pub fn firstKey(&mut self) -> xproto::Keycode {
    unsafe { accessor!(firstKey -> xproto::Keycode, (*self.base.event)) }
  }

  pub fn nKeys(&mut self) -> u8 {
    unsafe { accessor!(nKeys -> u8, (*self.base.event)) }
  }

  pub fn changedIndicators(&mut self) -> u32 {
    unsafe { accessor!(changedIndicators -> u32, (*self.base.event)) }
  }

  pub fn new(xkbType : u8,
         sequence : u16,
         time : xproto::Timestamp,
         deviceID : u8,
         changed : u16,
         firstType : u8,
         nTypes : u8,
         firstLevelName : u8,
         nLevelNames : u8,
         nRadioGroups : u8,
         nKeyAliases : u8,
         changedGroupNames : u8,
         changedVirtualMods : u16,
         firstKey : xproto::Keycode,
         nKeys : u8,
         changedIndicators : u32) -> NamesNotifyEvent {
    unsafe {
      let raw = malloc(32 as size_t) as *mut xcb_xkb_names_notify_event_t;
      (*raw).xkbType = xkbType;
      (*raw).sequence = sequence;
      (*raw).time = time;
      (*raw).deviceID = deviceID;
      (*raw).changed = changed;
      (*raw).firstType = firstType;
      (*raw).nTypes = nTypes;
      (*raw).firstLevelName = firstLevelName;
      (*raw).nLevelNames = nLevelNames;
      (*raw).nRadioGroups = nRadioGroups;
      (*raw).nKeyAliases = nKeyAliases;
      (*raw).changedGroupNames = changedGroupNames;
      (*raw).changedVirtualMods = changedVirtualMods;
      (*raw).firstKey = firstKey;
      (*raw).nKeys = nKeys;
      (*raw).changedIndicators = changedIndicators;
      NamesNotifyEvent { base : Event { event : raw as *mut xcb_xkb_names_notify_event_t }}
    }
  }
}

impl CompatMapNotifyEvent {
  pub fn xkbType(&mut self) -> u8 {
    unsafe { accessor!(xkbType -> u8, (*self.base.event)) }
  }

  pub fn sequence(&mut self) -> u16 {
    unsafe { accessor!(sequence -> u16, (*self.base.event)) }
  }

  pub fn time(&mut self) -> xproto::Timestamp {
    unsafe { accessor!(time -> xproto::Timestamp, (*self.base.event)) }
  }

  pub fn deviceID(&mut self) -> u8 {
    unsafe { accessor!(deviceID -> u8, (*self.base.event)) }
  }

  pub fn changedGroups(&mut self) -> u8 {
    unsafe { accessor!(changedGroups -> u8, (*self.base.event)) }
  }

  pub fn firstSI(&mut self) -> u16 {
    unsafe { accessor!(firstSI -> u16, (*self.base.event)) }
  }

  pub fn nSI(&mut self) -> u16 {
    unsafe { accessor!(nSI -> u16, (*self.base.event)) }
  }

  pub fn nTotalSI(&mut self) -> u16 {
    unsafe { accessor!(nTotalSI -> u16, (*self.base.event)) }
  }

  pub fn new(xkbType : u8,
         sequence : u16,
         time : xproto::Timestamp,
         deviceID : u8,
         changedGroups : u8,
         firstSI : u16,
         nSI : u16,
         nTotalSI : u16) -> CompatMapNotifyEvent {
    unsafe {
      let raw = malloc(32 as size_t) as *mut xcb_xkb_compat_map_notify_event_t;
      (*raw).xkbType = xkbType;
      (*raw).sequence = sequence;
      (*raw).time = time;
      (*raw).deviceID = deviceID;
      (*raw).changedGroups = changedGroups;
      (*raw).firstSI = firstSI;
      (*raw).nSI = nSI;
      (*raw).nTotalSI = nTotalSI;
      CompatMapNotifyEvent { base : Event { event : raw as *mut xcb_xkb_compat_map_notify_event_t }}
    }
  }
}

impl BellNotifyEvent {
  pub fn xkbType(&mut self) -> u8 {
    unsafe { accessor!(xkbType -> u8, (*self.base.event)) }
  }

  pub fn sequence(&mut self) -> u16 {
    unsafe { accessor!(sequence -> u16, (*self.base.event)) }
  }

  pub fn time(&mut self) -> xproto::Timestamp {
    unsafe { accessor!(time -> xproto::Timestamp, (*self.base.event)) }
  }

  pub fn deviceID(&mut self) -> u8 {
    unsafe { accessor!(deviceID -> u8, (*self.base.event)) }
  }

  pub fn bellClass(&mut self) -> u8 {
    unsafe { accessor!(bellClass -> u8, (*self.base.event)) }
  }

  pub fn bellID(&mut self) -> u8 {
    unsafe { accessor!(bellID -> u8, (*self.base.event)) }
  }

  pub fn percent(&mut self) -> u8 {
    unsafe { accessor!(percent -> u8, (*self.base.event)) }
  }

  pub fn pitch(&mut self) -> u16 {
    unsafe { accessor!(pitch -> u16, (*self.base.event)) }
  }

  pub fn duration(&mut self) -> u16 {
    unsafe { accessor!(duration -> u16, (*self.base.event)) }
  }

  pub fn name(&mut self) -> xproto::Atom {
    unsafe { accessor!(name -> xproto::Atom, (*self.base.event)) }
  }

  pub fn window(&mut self) -> xproto::Window {
    unsafe { accessor!(window -> xproto::Window, (*self.base.event)) }
  }

  pub fn eventOnly(&mut self) -> u8 {
    unsafe { accessor!(eventOnly -> u8, (*self.base.event)) }
  }

  pub fn new(xkbType : u8,
         sequence : u16,
         time : xproto::Timestamp,
         deviceID : u8,
         bellClass : u8,
         bellID : u8,
         percent : u8,
         pitch : u16,
         duration : u16,
         name : xproto::Atom,
         window : xproto::Window,
         eventOnly : u8) -> BellNotifyEvent {
    unsafe {
      let raw = malloc(32 as size_t) as *mut xcb_xkb_bell_notify_event_t;
      (*raw).xkbType = xkbType;
      (*raw).sequence = sequence;
      (*raw).time = time;
      (*raw).deviceID = deviceID;
      (*raw).bellClass = bellClass;
      (*raw).bellID = bellID;
      (*raw).percent = percent;
      (*raw).pitch = pitch;
      (*raw).duration = duration;
      (*raw).name = name;
      (*raw).window = window;
      (*raw).eventOnly = eventOnly;
      BellNotifyEvent { base : Event { event : raw as *mut xcb_xkb_bell_notify_event_t }}
    }
  }
}

impl ActionMessageEvent {
  pub fn xkbType(&mut self) -> u8 {
    unsafe { accessor!(xkbType -> u8, (*self.base.event)) }
  }

  pub fn sequence(&mut self) -> u16 {
    unsafe { accessor!(sequence -> u16, (*self.base.event)) }
  }

  pub fn time(&mut self) -> xproto::Timestamp {
    unsafe { accessor!(time -> xproto::Timestamp, (*self.base.event)) }
  }

  pub fn deviceID(&mut self) -> u8 {
    unsafe { accessor!(deviceID -> u8, (*self.base.event)) }
  }

  pub fn keycode(&mut self) -> xproto::Keycode {
    unsafe { accessor!(keycode -> xproto::Keycode, (*self.base.event)) }
  }

  pub fn press(&mut self) -> u8 {
    unsafe { accessor!(press -> u8, (*self.base.event)) }
  }

  pub fn keyEventFollows(&mut self) -> u8 {
    unsafe { accessor!(keyEventFollows -> u8, (*self.base.event)) }
  }

  pub fn mods(&mut self) -> u8 {
    unsafe { accessor!(mods -> u8, (*self.base.event)) }
  }

  pub fn group(&mut self) -> u8 {
    unsafe { accessor!(group -> u8, (*self.base.event)) }
  }

  pub fn message(&self) -> Vec<String8> {
    unsafe { ((*self.base.event).message).to_vec() }
  }

  pub fn new(xkbType : u8,
         sequence : u16,
         time : xproto::Timestamp,
         deviceID : u8,
         keycode : xproto::Keycode,
         press : u8,
         keyEventFollows : u8,
         mods : u8,
         group : u8,
         message : [String8; 8]) -> ActionMessageEvent {
    unsafe {
      let raw = malloc(32 as size_t) as *mut xcb_xkb_action_message_event_t;
      (*raw).xkbType = xkbType;
      (*raw).sequence = sequence;
      (*raw).time = time;
      (*raw).deviceID = deviceID;
      (*raw).keycode = keycode;
      (*raw).press = press;
      (*raw).keyEventFollows = keyEventFollows;
      (*raw).mods = mods;
      (*raw).group = group;
      (*raw).message = message;
      ActionMessageEvent { base : Event { event : raw as *mut xcb_xkb_action_message_event_t }}
    }
  }
}

impl AccessXNotifyEvent {
  pub fn xkbType(&mut self) -> u8 {
    unsafe { accessor!(xkbType -> u8, (*self.base.event)) }
  }

  pub fn sequence(&mut self) -> u16 {
    unsafe { accessor!(sequence -> u16, (*self.base.event)) }
  }

  pub fn time(&mut self) -> xproto::Timestamp {
    unsafe { accessor!(time -> xproto::Timestamp, (*self.base.event)) }
  }

  pub fn deviceID(&mut self) -> u8 {
    unsafe { accessor!(deviceID -> u8, (*self.base.event)) }
  }

  pub fn keycode(&mut self) -> xproto::Keycode {
    unsafe { accessor!(keycode -> xproto::Keycode, (*self.base.event)) }
  }

  pub fn detailt(&mut self) -> u16 {
    unsafe { accessor!(detailt -> u16, (*self.base.event)) }
  }

  pub fn slowKeysDelay(&mut self) -> u16 {
    unsafe { accessor!(slowKeysDelay -> u16, (*self.base.event)) }
  }

  pub fn debounceDelay(&mut self) -> u16 {
    unsafe { accessor!(debounceDelay -> u16, (*self.base.event)) }
  }

  pub fn new(xkbType : u8,
         sequence : u16,
         time : xproto::Timestamp,
         deviceID : u8,
         keycode : xproto::Keycode,
         detailt : u16,
         slowKeysDelay : u16,
         debounceDelay : u16) -> AccessXNotifyEvent {
    unsafe {
      let raw = malloc(32 as size_t) as *mut xcb_xkb_access_x_notify_event_t;
      (*raw).xkbType = xkbType;
      (*raw).sequence = sequence;
      (*raw).time = time;
      (*raw).deviceID = deviceID;
      (*raw).keycode = keycode;
      (*raw).detailt = detailt;
      (*raw).slowKeysDelay = slowKeysDelay;
      (*raw).debounceDelay = debounceDelay;
      AccessXNotifyEvent { base : Event { event : raw as *mut xcb_xkb_access_x_notify_event_t }}
    }
  }
}

impl ExtensionDeviceNotifyEvent {
  pub fn xkbType(&mut self) -> u8 {
    unsafe { accessor!(xkbType -> u8, (*self.base.event)) }
  }

  pub fn sequence(&mut self) -> u16 {
    unsafe { accessor!(sequence -> u16, (*self.base.event)) }
  }

  pub fn time(&mut self) -> xproto::Timestamp {
    unsafe { accessor!(time -> xproto::Timestamp, (*self.base.event)) }
  }

  pub fn deviceID(&mut self) -> u8 {
    unsafe { accessor!(deviceID -> u8, (*self.base.event)) }
  }

  pub fn reason(&mut self) -> u16 {
    unsafe { accessor!(reason -> u16, (*self.base.event)) }
  }

  pub fn ledClass(&mut self) -> u16 {
    unsafe { accessor!(ledClass -> u16, (*self.base.event)) }
  }

  pub fn ledID(&mut self) -> u8 {
    unsafe { accessor!(ledID -> u8, (*self.base.event)) }
  }

  pub fn ledsDefined(&mut self) -> u32 {
    unsafe { accessor!(ledsDefined -> u32, (*self.base.event)) }
  }

  pub fn ledState(&mut self) -> u32 {
    unsafe { accessor!(ledState -> u32, (*self.base.event)) }
  }

  pub fn firstButton(&mut self) -> u8 {
    unsafe { accessor!(firstButton -> u8, (*self.base.event)) }
  }

  pub fn nButtons(&mut self) -> u8 {
    unsafe { accessor!(nButtons -> u8, (*self.base.event)) }
  }

  pub fn supported(&mut self) -> u16 {
    unsafe { accessor!(supported -> u16, (*self.base.event)) }
  }

  pub fn unsupported(&mut self) -> u16 {
    unsafe { accessor!(unsupported -> u16, (*self.base.event)) }
  }

  pub fn new(xkbType : u8,
         sequence : u16,
         time : xproto::Timestamp,
         deviceID : u8,
         reason : u16,
         ledClass : u16,
         ledID : u8,
         ledsDefined : u32,
         ledState : u32,
         firstButton : u8,
         nButtons : u8,
         supported : u16,
         unsupported : u16) -> ExtensionDeviceNotifyEvent {
    unsafe {
      let raw = malloc(32 as size_t) as *mut xcb_xkb_extension_device_notify_event_t;
      (*raw).xkbType = xkbType;
      (*raw).sequence = sequence;
      (*raw).time = time;
      (*raw).deviceID = deviceID;
      (*raw).reason = reason;
      (*raw).ledClass = ledClass;
      (*raw).ledID = ledID;
      (*raw).ledsDefined = ledsDefined;
      (*raw).ledState = ledState;
      (*raw).firstButton = firstButton;
      (*raw).nButtons = nButtons;
      (*raw).supported = supported;
      (*raw).unsupported = unsupported;
      ExtensionDeviceNotifyEvent { base : Event { event : raw as *mut xcb_xkb_extension_device_notify_event_t }}
    }
  }
}

