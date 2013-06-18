/*
 * This file generated automatically from xkb.xml by r_client.py.
 * Edit at your peril.
 */

//Make the compiler quiet
#[allow(unused_imports)];
#[allow(non_camel_case_types)];
use core::libc::*;
use ll::base::*;
use ll::xproto;

pub static XKB_MAJOR_VERSION : c_uint = 1;
pub static XKB_MINOR_VERSION : c_uint = 0;

pub type const = c_uint;//{
    pub static XCB_XKB_CONST_MAX_LEGAL_KEY_CODE : const = 255;
    pub static XCB_XKB_CONST_PER_KEY_BIT_ARRAY_SIZE : const = 32;
    pub static XCB_XKB_CONST_KEY_NAME_LENGTH : const = 4;
//}

pub type event_type = c_uint;//{
    pub static XCB_XKB_EVENT_TYPE_NEW_KEYBOARD_NOTIFY : event_type = 1;
    pub static XCB_XKB_EVENT_TYPE_MAP_NOTIFY : event_type = 2;
    pub static XCB_XKB_EVENT_TYPE_STATE_NOTIFY : event_type = 4;
    pub static XCB_XKB_EVENT_TYPE_CONTROLS_NOTIFY : event_type = 8;
    pub static XCB_XKB_EVENT_TYPE_INDICATOR_STATE_NOTIFY : event_type = 16;
    pub static XCB_XKB_EVENT_TYPE_INDICATOR_MAP_NOTIFY : event_type = 32;
    pub static XCB_XKB_EVENT_TYPE_NAMES_NOTIFY : event_type = 64;
    pub static XCB_XKB_EVENT_TYPE_COMPAT_MAP_NOTIFY : event_type = 128;
    pub static XCB_XKB_EVENT_TYPE_BELL_NOTIFY : event_type = 256;
    pub static XCB_XKB_EVENT_TYPE_ACTION_MESSAGE : event_type = 512;
    pub static XCB_XKB_EVENT_TYPE_ACCESS_X_NOTIFY : event_type = 1024;
    pub static XCB_XKB_EVENT_TYPE_EXTENSION_DEVICE_NOTIFY : event_type = 2048;
//}

pub type nkn_detail = c_uint;//{
    pub static XCB_XKB_NKN_DETAIL_KEYCODES : nkn_detail = 1;
    pub static XCB_XKB_NKN_DETAIL_GEOMETRY : nkn_detail = 2;
    pub static XCB_XKB_NKN_DETAIL_DEVICE_ID : nkn_detail = 4;
//}

pub type axn_detail = c_uint;//{
    pub static XCB_XKB_AXN_DETAIL_SK_PRESS : axn_detail = 1;
    pub static XCB_XKB_AXN_DETAIL_SK_ACCEPT : axn_detail = 2;
    pub static XCB_XKB_AXN_DETAIL_SK_REJECT : axn_detail = 4;
    pub static XCB_XKB_AXN_DETAIL_SK_RELEASE : axn_detail = 8;
    pub static XCB_XKB_AXN_DETAIL_BK_ACCEPT : axn_detail = 16;
    pub static XCB_XKB_AXN_DETAIL_BK_REJECT : axn_detail = 32;
    pub static XCB_XKB_AXN_DETAIL_AXK_WARNING : axn_detail = 64;
//}

pub type map_part = c_uint;//{
    pub static XCB_XKB_MAP_PART_KEY_TYPES : map_part = 1;
    pub static XCB_XKB_MAP_PART_KEY_SYMS : map_part = 2;
    pub static XCB_XKB_MAP_PART_MODIFIER_MAP : map_part = 4;
    pub static XCB_XKB_MAP_PART_EXPLICIT_COMPONENTS : map_part = 8;
    pub static XCB_XKB_MAP_PART_KEY_ACTIONS : map_part = 16;
    pub static XCB_XKB_MAP_PART_KEY_BEHAVIORS : map_part = 32;
    pub static XCB_XKB_MAP_PART_VIRTUAL_MODS : map_part = 64;
    pub static XCB_XKB_MAP_PART_VIRTUAL_MOD_MAP : map_part = 128;
//}

pub type set_map_flags = c_uint;//{
    pub static XCB_XKB_SET_MAP_FLAGS_RESIZE_TYPES : set_map_flags = 1;
    pub static XCB_XKB_SET_MAP_FLAGS_RECOMPUTE_ACTIONS : set_map_flags = 2;
//}

pub type state_part = c_uint;//{
    pub static XCB_XKB_STATE_PART_MODIFIER_STATE : state_part = 1;
    pub static XCB_XKB_STATE_PART_MODIFIER_BASE : state_part = 2;
    pub static XCB_XKB_STATE_PART_MODIFIER_LATCH : state_part = 4;
    pub static XCB_XKB_STATE_PART_MODIFIER_LOCK : state_part = 8;
    pub static XCB_XKB_STATE_PART_GROUP_STATE : state_part = 16;
    pub static XCB_XKB_STATE_PART_GROUP_BASE : state_part = 32;
    pub static XCB_XKB_STATE_PART_GROUP_LATCH : state_part = 64;
    pub static XCB_XKB_STATE_PART_GROUP_LOCK : state_part = 128;
    pub static XCB_XKB_STATE_PART_COMPAT_STATE : state_part = 256;
    pub static XCB_XKB_STATE_PART_GRAB_MODS : state_part = 512;
    pub static XCB_XKB_STATE_PART_COMPAT_GRAB_MODS : state_part = 1024;
    pub static XCB_XKB_STATE_PART_LOOKUP_MODS : state_part = 2048;
    pub static XCB_XKB_STATE_PART_COMPAT_LOOKUP_MODS : state_part = 4096;
    pub static XCB_XKB_STATE_PART_POINTER_BUTTONS : state_part = 8192;
//}

pub type bool_ctrl = c_uint;//{
    pub static XCB_XKB_BOOL_CTRL_REPEAT_KEYS : bool_ctrl = 1;
    pub static XCB_XKB_BOOL_CTRL_SLOW_KEYS : bool_ctrl = 2;
    pub static XCB_XKB_BOOL_CTRL_BOUNCE_KEYS : bool_ctrl = 4;
    pub static XCB_XKB_BOOL_CTRL_STICKY_KEYS : bool_ctrl = 8;
    pub static XCB_XKB_BOOL_CTRL_MOUSE_KEYS : bool_ctrl = 16;
    pub static XCB_XKB_BOOL_CTRL_MOUSE_KEYS_ACCEL : bool_ctrl = 32;
    pub static XCB_XKB_BOOL_CTRL_ACCESS_X_KEYS : bool_ctrl = 64;
    pub static XCB_XKB_BOOL_CTRL_ACCESS_X_TIMEOUT_MASK : bool_ctrl = 128;
    pub static XCB_XKB_BOOL_CTRL_ACCESS_X_FEEDBACK_MASK : bool_ctrl = 256;
    pub static XCB_XKB_BOOL_CTRL_AUDIBLE_BELL_MASK : bool_ctrl = 512;
    pub static XCB_XKB_BOOL_CTRL_OVERLAY_1_MASK : bool_ctrl = 1024;
    pub static XCB_XKB_BOOL_CTRL_OVERLAY_2_MASK : bool_ctrl = 2048;
    pub static XCB_XKB_BOOL_CTRL_IGNORE_GROUP_LOCK_MASK : bool_ctrl = 4096;
//}

pub type control = c_uint;//{
    pub static XCB_XKB_CONTROL_GROUPS_WRAP : control = 134217728;
    pub static XCB_XKB_CONTROL_INTERNAL_MODS : control = 268435456;
    pub static XCB_XKB_CONTROL_IGNORE_LOCK_MODS : control = 536870912;
    pub static XCB_XKB_CONTROL_PER_KEY_REPEAT : control = 1073741824;
    pub static XCB_XKB_CONTROL_CONTROLS_ENABLED : control = 2147483648;
//}

pub type axfb_opt = c_uint;//{
    pub static XCB_XKB_AXFB_OPT_SK_PRESS_FB : axfb_opt = 1;
    pub static XCB_XKB_AXFB_OPT_SK_ACCEPT_FB : axfb_opt = 2;
    pub static XCB_XKB_AXFB_OPT_FEATURE_FB : axfb_opt = 4;
    pub static XCB_XKB_AXFB_OPT_SLOW_WARN_FB : axfb_opt = 8;
    pub static XCB_XKB_AXFB_OPT_INDICATOR_FB : axfb_opt = 16;
    pub static XCB_XKB_AXFB_OPT_STICKY_KEYS_FB : axfb_opt = 32;
    pub static XCB_XKB_AXFB_OPT_SK_RELEASE_FB : axfb_opt = 64;
    pub static XCB_XKB_AXFB_OPT_SK_REJECT_FB : axfb_opt = 128;
    pub static XCB_XKB_AXFB_OPT_BK_REJECT_FB : axfb_opt = 256;
    pub static XCB_XKB_AXFB_OPT_DUMB_BELL : axfb_opt = 512;
//}

pub type axsk_opt = c_uint;//{
    pub static XCB_XKB_AXSK_OPT_TWO_KEYS : axsk_opt = 64;
    pub static XCB_XKB_AXSK_OPT_LATCH_TO_LOCK : axsk_opt = 128;
//}

pub struct ax_option {
    data : [u8,..2]
}

/**
 * @brief ax_option_iterator
 **/
pub struct ax_option_iterator {
    data : *ax_option,
    rem  : c_int,
    index: c_int
}

pub type device_spec = u16;

/**
 * @brief device_spec_iterator
 **/
pub struct device_spec_iterator {
    data : *device_spec,
    rem  : c_int,
    index: c_int
}

pub type led_class_result = c_uint;//{
    pub static XCB_XKB_LED_CLASS_RESULT_KBD_FEEDBACK_CLASS : led_class_result = 0;
    pub static XCB_XKB_LED_CLASS_RESULT_LED_FEEDBACK_CLASS : led_class_result = 4;
//}

pub type led_class = c_uint;//{
    pub static XCB_XKB_LED_CLASS_DFLT_XI_CLASS : led_class = 768;
    pub static XCB_XKB_LED_CLASS_ALL_XI_CLASSES : led_class = 1280;
//}

pub type led_class_spec = u16;

/**
 * @brief led_class_spec_iterator
 **/
pub struct led_class_spec_iterator {
    data : *led_class_spec,
    rem  : c_int,
    index: c_int
}

pub type bell_class_result = c_uint;//{
    pub static XCB_XKB_BELL_CLASS_RESULT_KBD_FEEDBACK_CLASS : bell_class_result = 0;
    pub static XCB_XKB_BELL_CLASS_RESULT_BELL_FEEDBACK_CLASS : bell_class_result = 5;
//}

pub type bell_class = c_uint;//{
    pub static XCB_XKB_BELL_CLASS_DFLT_XI_CLASS : bell_class = 768;
//}

pub type bell_class_spec = u16;

/**
 * @brief bell_class_spec_iterator
 **/
pub struct bell_class_spec_iterator {
    data : *bell_class_spec,
    rem  : c_int,
    index: c_int
}

pub type id = c_uint;//{
    pub static XCB_XKB_ID_USE_CORE_KBD : id = 256;
    pub static XCB_XKB_ID_USE_CORE_PTR : id = 512;
    pub static XCB_XKB_ID_DFLT_XI_CLASS : id = 768;
    pub static XCB_XKB_ID_DFLT_XI_ID : id = 1024;
    pub static XCB_XKB_ID_ALL_XI_CLASS : id = 1280;
    pub static XCB_XKB_ID_ALL_XI_ID : id = 1536;
    pub static XCB_XKB_ID_XI_NONE : id = 65280;
//}

pub type id_spec = u16;

/**
 * @brief id_spec_iterator
 **/
pub struct id_spec_iterator {
    data : *id_spec,
    rem  : c_int,
    index: c_int
}

pub type group = c_uint;//{
    pub static XCB_XKB_GROUP_1 : group = 0;
    pub static XCB_XKB_GROUP_2 : group = 1;
    pub static XCB_XKB_GROUP_3 : group = 2;
    pub static XCB_XKB_GROUP_4 : group = 3;
//}

pub type groups = c_uint;//{
    pub static XCB_XKB_GROUPS_ANY : groups = 254;
    pub static XCB_XKB_GROUPS_ALL : groups = 255;
//}

pub type set_of_group = c_uint;//{
    pub static XCB_XKB_SET_OF_GROUP_GROUP_1 : set_of_group = 1;
    pub static XCB_XKB_SET_OF_GROUP_GROUP_2 : set_of_group = 2;
    pub static XCB_XKB_SET_OF_GROUP_GROUP_3 : set_of_group = 4;
    pub static XCB_XKB_SET_OF_GROUP_GROUP_4 : set_of_group = 8;
//}

pub type set_of_groups = c_uint;//{
    pub static XCB_XKB_SET_OF_GROUPS_ANY : set_of_groups = 128;
//}

pub type groups_wrap = c_uint;//{
    pub static XCB_XKB_GROUPS_WRAP_WRAP_INTO_RANGE : groups_wrap = 0;
    pub static XCB_XKB_GROUPS_WRAP_CLAMP_INTO_RANGE : groups_wrap = 64;
    pub static XCB_XKB_GROUPS_WRAP_REDIRECT_INTO_RANGE : groups_wrap = 128;
//}

pub type v_mods_high = c_uint;//{
    pub static XCB_XKB_V_MODS_HIGH_15 : v_mods_high = 128;
    pub static XCB_XKB_V_MODS_HIGH_14 : v_mods_high = 64;
    pub static XCB_XKB_V_MODS_HIGH_13 : v_mods_high = 32;
    pub static XCB_XKB_V_MODS_HIGH_12 : v_mods_high = 16;
    pub static XCB_XKB_V_MODS_HIGH_11 : v_mods_high = 8;
    pub static XCB_XKB_V_MODS_HIGH_10 : v_mods_high = 4;
    pub static XCB_XKB_V_MODS_HIGH_9 : v_mods_high = 2;
    pub static XCB_XKB_V_MODS_HIGH_8 : v_mods_high = 1;
//}

pub type v_mods_low = c_uint;//{
    pub static XCB_XKB_V_MODS_LOW_7 : v_mods_low = 128;
    pub static XCB_XKB_V_MODS_LOW_6 : v_mods_low = 64;
    pub static XCB_XKB_V_MODS_LOW_5 : v_mods_low = 32;
    pub static XCB_XKB_V_MODS_LOW_4 : v_mods_low = 16;
    pub static XCB_XKB_V_MODS_LOW_3 : v_mods_low = 8;
    pub static XCB_XKB_V_MODS_LOW_2 : v_mods_low = 4;
    pub static XCB_XKB_V_MODS_LOW_1 : v_mods_low = 2;
    pub static XCB_XKB_V_MODS_LOW_0 : v_mods_low = 1;
//}

pub type v_mod = c_uint;//{
    pub static XCB_XKB_V_MOD_15 : v_mod = 32768;
    pub static XCB_XKB_V_MOD_14 : v_mod = 16384;
    pub static XCB_XKB_V_MOD_13 : v_mod = 8192;
    pub static XCB_XKB_V_MOD_12 : v_mod = 4096;
    pub static XCB_XKB_V_MOD_11 : v_mod = 2048;
    pub static XCB_XKB_V_MOD_10 : v_mod = 1024;
    pub static XCB_XKB_V_MOD_9 : v_mod = 512;
    pub static XCB_XKB_V_MOD_8 : v_mod = 256;
    pub static XCB_XKB_V_MOD_7 : v_mod = 128;
    pub static XCB_XKB_V_MOD_6 : v_mod = 64;
    pub static XCB_XKB_V_MOD_5 : v_mod = 32;
    pub static XCB_XKB_V_MOD_4 : v_mod = 16;
    pub static XCB_XKB_V_MOD_3 : v_mod = 8;
    pub static XCB_XKB_V_MOD_2 : v_mod = 4;
    pub static XCB_XKB_V_MOD_1 : v_mod = 2;
    pub static XCB_XKB_V_MOD_0 : v_mod = 1;
//}

pub type explicit = c_uint;//{
    pub static XCB_XKB_EXPLICIT_V_MOD_MAP : explicit = 128;
    pub static XCB_XKB_EXPLICIT_BEHAVIOR : explicit = 64;
    pub static XCB_XKB_EXPLICIT_AUTO_REPEAT : explicit = 32;
    pub static XCB_XKB_EXPLICIT_INTERPRET : explicit = 16;
    pub static XCB_XKB_EXPLICIT_KEY_TYPE_4 : explicit = 8;
    pub static XCB_XKB_EXPLICIT_KEY_TYPE_3 : explicit = 4;
    pub static XCB_XKB_EXPLICIT_KEY_TYPE_2 : explicit = 2;
    pub static XCB_XKB_EXPLICIT_KEY_TYPE_1 : explicit = 1;
//}

pub type sym_interpret = c_uint;//{
    pub static XCB_XKB_SYM_INTERPRET_NONE_OF : sym_interpret = 0;
    pub static XCB_XKB_SYM_INTERPRET_ANY_OF_OR_NONE : sym_interpret = 1;
    pub static XCB_XKB_SYM_INTERPRET_ANY_OF : sym_interpret = 2;
    pub static XCB_XKB_SYM_INTERPRET_ALL_OF : sym_interpret = 3;
    pub static XCB_XKB_SYM_INTERPRET_EXACTLY : sym_interpret = 4;
//}

pub type sym_interp_match = c_uint;//{
    pub static XCB_XKB_SYM_INTERP_MATCH_LEVEL_ONE_ONLY : sym_interp_match = 128;
    pub static XCB_XKB_SYM_INTERP_MATCH_OP_MASK : sym_interp_match = 127;
//}

pub type im_flag = c_uint;//{
    pub static XCB_XKB_IM_FLAG_NO_EXPLICIT : im_flag = 128;
    pub static XCB_XKB_IM_FLAG_NO_AUTOMATIC : im_flag = 64;
    pub static XCB_XKB_IM_FLAG_LED_DRIVES_KB : im_flag = 32;
//}

pub type im_mods_which = c_uint;//{
    pub static XCB_XKB_IM_MODS_WHICH_USE_COMPAT : im_mods_which = 16;
    pub static XCB_XKB_IM_MODS_WHICH_USE_EFFECTIVE : im_mods_which = 8;
    pub static XCB_XKB_IM_MODS_WHICH_USE_LOCKED : im_mods_which = 4;
    pub static XCB_XKB_IM_MODS_WHICH_USE_LATCHED : im_mods_which = 2;
    pub static XCB_XKB_IM_MODS_WHICH_USE_BASE : im_mods_which = 1;
//}

pub type im_groups_which = c_uint;//{
    pub static XCB_XKB_IM_GROUPS_WHICH_USE_COMPAT : im_groups_which = 16;
    pub static XCB_XKB_IM_GROUPS_WHICH_USE_EFFECTIVE : im_groups_which = 8;
    pub static XCB_XKB_IM_GROUPS_WHICH_USE_LOCKED : im_groups_which = 4;
    pub static XCB_XKB_IM_GROUPS_WHICH_USE_LATCHED : im_groups_which = 2;
    pub static XCB_XKB_IM_GROUPS_WHICH_USE_BASE : im_groups_which = 1;
//}

pub struct indicator_map {
    flags :         u8,
    whichGroups :   u8,
    groups :        u8,
    whichMods :     u8,
    mods :          u8,
    realMods :      u8,
    vmods :         u16,
    ctrls :         u32
}

/**
 * @brief indicator_map_iterator
 **/
pub struct indicator_map_iterator {
    data : *indicator_map,
    rem  : c_int,
    index: c_int
}

pub type cm_detail = c_uint;//{
    pub static XCB_XKB_CM_DETAIL_SYM_INTERP : cm_detail = 1;
    pub static XCB_XKB_CM_DETAIL_GROUP_COMPAT : cm_detail = 2;
//}

pub type name_detail = c_uint;//{
    pub static XCB_XKB_NAME_DETAIL_KEYCODES : name_detail = 1;
    pub static XCB_XKB_NAME_DETAIL_GEOMETRY : name_detail = 2;
    pub static XCB_XKB_NAME_DETAIL_SYMBOLS : name_detail = 4;
    pub static XCB_XKB_NAME_DETAIL_PHYS_SYMBOLS : name_detail = 8;
    pub static XCB_XKB_NAME_DETAIL_TYPES : name_detail = 16;
    pub static XCB_XKB_NAME_DETAIL_COMPAT : name_detail = 32;
    pub static XCB_XKB_NAME_DETAIL_KEY_TYPE_NAMES : name_detail = 64;
    pub static XCB_XKB_NAME_DETAIL_KT_LEVEL_NAMES : name_detail = 128;
    pub static XCB_XKB_NAME_DETAIL_INDICATOR_NAMES : name_detail = 256;
    pub static XCB_XKB_NAME_DETAIL_KEY_NAMES : name_detail = 512;
    pub static XCB_XKB_NAME_DETAIL_KEY_ALIASES : name_detail = 1024;
    pub static XCB_XKB_NAME_DETAIL_VIRTUAL_MOD_NAMES : name_detail = 2048;
    pub static XCB_XKB_NAME_DETAIL_GROUP_NAMES : name_detail = 4096;
    pub static XCB_XKB_NAME_DETAIL_RG_NAMES : name_detail = 8192;
//}

pub type gbn_detail = c_uint;//{
    pub static XCB_XKB_GBN_DETAIL_TYPES : gbn_detail = 1;
    pub static XCB_XKB_GBN_DETAIL_COMPAT_MAP : gbn_detail = 2;
    pub static XCB_XKB_GBN_DETAIL_CLIENT_SYMBOLS : gbn_detail = 4;
    pub static XCB_XKB_GBN_DETAIL_SERVER_SYMBOLS : gbn_detail = 8;
    pub static XCB_XKB_GBN_DETAIL_INDICATOR_MAPS : gbn_detail = 16;
    pub static XCB_XKB_GBN_DETAIL_KEY_NAMES : gbn_detail = 32;
    pub static XCB_XKB_GBN_DETAIL_GEOMETRY : gbn_detail = 64;
    pub static XCB_XKB_GBN_DETAIL_OTHER_NAMES : gbn_detail = 128;
//}

pub type xi_feature = c_uint;//{
    pub static XCB_XKB_XI_FEATURE_KEYBOARDS : xi_feature = 1;
    pub static XCB_XKB_XI_FEATURE_BUTTON_ACTIONS : xi_feature = 2;
    pub static XCB_XKB_XI_FEATURE_INDICATOR_NAMES : xi_feature = 4;
    pub static XCB_XKB_XI_FEATURE_INDICATOR_MAPS : xi_feature = 8;
    pub static XCB_XKB_XI_FEATURE_INDICATOR_STATE : xi_feature = 16;
//}

pub type per_client_flag = c_uint;//{
    pub static XCB_XKB_PER_CLIENT_FLAG_DETECTABLE_AUTO_REPEAT : per_client_flag = 1;
    pub static XCB_XKB_PER_CLIENT_FLAG_GRABS_USE_XKB_STATE : per_client_flag = 2;
    pub static XCB_XKB_PER_CLIENT_FLAG_AUTO_RESET_CONTROLS : per_client_flag = 4;
    pub static XCB_XKB_PER_CLIENT_FLAG_LOOKUP_STATE_WHEN_GRABBED : per_client_flag = 8;
    pub static XCB_XKB_PER_CLIENT_FLAG_SEND_EVENT_USES_XKB_STATE : per_client_flag = 16;
//}

pub struct mod_def {
    mask :       u8,
    realMods :   u8,
    vmods :      u16
}

/**
 * @brief mod_def_iterator
 **/
pub struct mod_def_iterator {
    data : *mod_def,
    rem  : c_int,
    index: c_int
}

pub struct key_name {
    name :   [u8,..4]
}

/**
 * @brief key_name_iterator
 **/
pub struct key_name_iterator {
    data : *key_name,
    rem  : c_int,
    index: c_int
}

pub struct key_alias {
    real :    [u8,..4],
    alias :   [u8,..4]
}

/**
 * @brief key_alias_iterator
 **/
pub struct key_alias_iterator {
    data : *key_alias,
    rem  : c_int,
    index: c_int
}

pub struct counted_string_8 {
    length :   u8
}

/**
 * @brief counted_string_8_iterator
 **/
pub struct counted_string_8_iterator {
    data : *counted_string_8,
    rem  : c_int,
    index: c_int
}

pub struct counted_string_16 {
    length :   u16,
    pad0 :     u8
}

/**
 * @brief counted_string_16_iterator
 **/
pub struct counted_string_16_iterator {
    data : *counted_string_16,
    rem  : c_int,
    index: c_int
}

pub struct kt_map_entry {
    active :       u8,
    level :        u8,
    mods_mask :    u8,
    mods_mods :    u8,
    mods_vmods :   u16,
    pad0 :         [u8,..2]
}

/**
 * @brief kt_map_entry_iterator
 **/
pub struct kt_map_entry_iterator {
    data : *kt_map_entry,
    rem  : c_int,
    index: c_int
}

pub struct key_type {
    mods_mask :     u8,
    mods_mods :     u8,
    mods_vmods :    u16,
    numLevels :     u8,
    nMapEntries :   u8,
    hasPreserve :   u8,
    pad0 :          u8
}

/**
 * @brief key_type_iterator
 **/
pub struct key_type_iterator {
    data : *key_type,
    rem  : c_int,
    index: c_int
}

pub struct key_sym_map {
    kt_index :    [u8,..4],
    groupInfo :   u8,
    width :       u8,
    nSyms :       u16
}

/**
 * @brief key_sym_map_iterator
 **/
pub struct key_sym_map_iterator {
    data : *key_sym_map,
    rem  : c_int,
    index: c_int
}

pub struct common_behavior {
    type_ :   u8,
    data :    u8
}

/**
 * @brief common_behavior_iterator
 **/
pub struct common_behavior_iterator {
    data : *common_behavior,
    rem  : c_int,
    index: c_int
}

pub struct default_behavior {
    type_ :   u8,
    pad0 :    u8
}

/**
 * @brief default_behavior_iterator
 **/
pub struct default_behavior_iterator {
    data : *default_behavior,
    rem  : c_int,
    index: c_int
}

pub struct lock_behavior {
    type_ :   u8,
    pad0 :    u8
}

/**
 * @brief lock_behavior_iterator
 **/
pub struct lock_behavior_iterator {
    data : *lock_behavior,
    rem  : c_int,
    index: c_int
}

pub struct radio_group_behavior {
    type_ :   u8,
    group :   u8
}

/**
 * @brief radio_group_behavior_iterator
 **/
pub struct radio_group_behavior_iterator {
    data : *radio_group_behavior,
    rem  : c_int,
    index: c_int
}

pub struct overlay_1_behavior {
    type_ :   u8,
    key :     xproto::keycode
}

/**
 * @brief overlay_1_behavior_iterator
 **/
pub struct overlay_1_behavior_iterator {
    data : *overlay_1_behavior,
    rem  : c_int,
    index: c_int
}

pub struct overlay_2_behavior {
    type_ :   u8,
    key :     u8
}

/**
 * @brief overlay_2_behavior_iterator
 **/
pub struct overlay_2_behavior_iterator {
    data : *overlay_2_behavior,
    rem  : c_int,
    index: c_int
}

pub struct permament_lock_behavior {
    type_ :   u8,
    pad0 :    u8
}

/**
 * @brief permament_lock_behavior_iterator
 **/
pub struct permament_lock_behavior_iterator {
    data : *permament_lock_behavior,
    rem  : c_int,
    index: c_int
}

pub struct permament_radio_group_behavior {
    type_ :   u8,
    group :   u8
}

/**
 * @brief permament_radio_group_behavior_iterator
 **/
pub struct permament_radio_group_behavior_iterator {
    data : *permament_radio_group_behavior,
    rem  : c_int,
    index: c_int
}

pub struct permament_overlay_1_behavior {
    type_ :   u8,
    key :     xproto::keycode
}

/**
 * @brief permament_overlay_1_behavior_iterator
 **/
pub struct permament_overlay_1_behavior_iterator {
    data : *permament_overlay_1_behavior,
    rem  : c_int,
    index: c_int
}

pub struct permament_overlay_2_behavior {
    type_ :   u8,
    key :     u8
}

/**
 * @brief permament_overlay_2_behavior_iterator
 **/
pub struct permament_overlay_2_behavior_iterator {
    data : *permament_overlay_2_behavior,
    rem  : c_int,
    index: c_int
}

pub struct behavior {
    data : [u8,..2]
}

/**
 * @brief behavior_iterator
 **/
pub struct behavior_iterator {
    data : *behavior,
    rem  : c_int,
    index: c_int
}

pub type behavior_type = c_uint;//{
    pub static XCB_XKB_BEHAVIOR_TYPE_DEFAULT : behavior_type = 0;
    pub static XCB_XKB_BEHAVIOR_TYPE_LOCK : behavior_type = 1;
    pub static XCB_XKB_BEHAVIOR_TYPE_RADIO_GROUP : behavior_type = 2;
    pub static XCB_XKB_BEHAVIOR_TYPE_OVERLAY_1 : behavior_type = 3;
    pub static XCB_XKB_BEHAVIOR_TYPE_OVERLAY_2 : behavior_type = 4;
    pub static XCB_XKB_BEHAVIOR_TYPE_PERMAMENT_LOCK : behavior_type = 129;
    pub static XCB_XKB_BEHAVIOR_TYPE_PERMAMENT_RADIO_GROUP : behavior_type = 130;
    pub static XCB_XKB_BEHAVIOR_TYPE_PERMAMENT_OVERLAY_1 : behavior_type = 131;
    pub static XCB_XKB_BEHAVIOR_TYPE_PERMAMENT_OVERLAY_2 : behavior_type = 132;
//}

pub struct set_behavior {
    keycode :    xproto::keycode,
    behavior :   behavior,
    pad0 :       u8
}

/**
 * @brief set_behavior_iterator
 **/
pub struct set_behavior_iterator {
    data : *set_behavior,
    rem  : c_int,
    index: c_int
}

pub struct set_explicit {
    keycode :    xproto::keycode,
    explicit :   u8
}

/**
 * @brief set_explicit_iterator
 **/
pub struct set_explicit_iterator {
    data : *set_explicit,
    rem  : c_int,
    index: c_int
}

pub struct key_mod_map {
    keycode :   xproto::keycode,
    mods :      u8
}

/**
 * @brief key_mod_map_iterator
 **/
pub struct key_mod_map_iterator {
    data : *key_mod_map,
    rem  : c_int,
    index: c_int
}

pub struct key_v_mod_map {
    keycode :   xproto::keycode,
    pad0 :      u8,
    vmods :     u16
}

/**
 * @brief key_v_mod_map_iterator
 **/
pub struct key_v_mod_map_iterator {
    data : *key_v_mod_map,
    rem  : c_int,
    index: c_int
}

pub struct kt_set_map_entry {
    level :         u8,
    realMods :      u8,
    virtualMods :   u16
}

/**
 * @brief kt_set_map_entry_iterator
 **/
pub struct kt_set_map_entry_iterator {
    data : *kt_set_map_entry,
    rem  : c_int,
    index: c_int
}

pub struct set_key_type {
    mask :          u8,
    realMods :      u8,
    virtualMods :   u16,
    numLevels :     u8,
    nMapEntries :   u8,
    preserve :      u8,
    pad0 :          u8
}

/**
 * @brief set_key_type_iterator
 **/
pub struct set_key_type_iterator {
    data : *set_key_type,
    rem  : c_int,
    index: c_int
}

pub type string8 = u8;

/**
 * @brief string8_iterator
 **/
pub struct string8_iterator {
    data : *string8,
    rem  : c_int,
    index: c_int
}

pub struct property {
    nameLength :    u16,
    valueLength :   u16
}

/**
 * @brief property_iterator
 **/
pub struct property_iterator {
    data : *property,
    rem  : c_int,
    index: c_int
}

pub struct outline {
    nPoints :        u8,
    cornerRadius :   u8,
    pad0 :           [u8,..2]
}

/**
 * @brief outline_iterator
 **/
pub struct outline_iterator {
    data : *outline,
    rem  : c_int,
    index: c_int
}

pub struct shape {
    name :         xproto::atom,
    nOutlines :    u8,
    primaryNdx :   u8,
    approxNdx :    u8,
    pad0 :         u8
}

/**
 * @brief shape_iterator
 **/
pub struct shape_iterator {
    data : *shape,
    rem  : c_int,
    index: c_int
}

pub struct key {
    name :       [string8,..4],
    gap :        i16,
    shapeNdx :   u8,
    colorNdx :   u8
}

/**
 * @brief key_iterator
 **/
pub struct key_iterator {
    data : *key,
    rem  : c_int,
    index: c_int
}

pub struct overlay_key {
    over :    [string8,..4],
    under :   [string8,..4]
}

/**
 * @brief overlay_key_iterator
 **/
pub struct overlay_key_iterator {
    data : *overlay_key,
    rem  : c_int,
    index: c_int
}

pub struct overlay_row {
    rowUnder :   u8,
    nKeys :      u8,
    pad0 :       [u8,..2]
}

/**
 * @brief overlay_row_iterator
 **/
pub struct overlay_row_iterator {
    data : *overlay_row,
    rem  : c_int,
    index: c_int
}

pub struct overlay {
    name :    xproto::atom,
    nRows :   u8,
    pad0 :    [u8,..3]
}

/**
 * @brief overlay_iterator
 **/
pub struct overlay_iterator {
    data : *overlay,
    rem  : c_int,
    index: c_int
}

pub struct row {
    top :        i16,
    left :       i16,
    nKeys :      u8,
    vertical :   u8,
    pad0 :       [u8,..2]
}

/**
 * @brief row_iterator
 **/
pub struct row_iterator {
    data : *row,
    rem  : c_int,
    index: c_int
}

pub type doodad_type = c_uint;//{
    pub static XCB_XKB_DOODAD_TYPE_OUTLINE : doodad_type = 1;
    pub static XCB_XKB_DOODAD_TYPE_SOLID : doodad_type = 2;
    pub static XCB_XKB_DOODAD_TYPE_TEXT : doodad_type = 3;
    pub static XCB_XKB_DOODAD_TYPE_INDICATOR : doodad_type = 4;
    pub static XCB_XKB_DOODAD_TYPE_LOGO : doodad_type = 5;
//}

pub struct common_doodad {
    name :       xproto::atom,
    type_ :      u8,
    priority :   u8,
    top :        i16,
    left :       i16,
    angle :      i16
}

/**
 * @brief common_doodad_iterator
 **/
pub struct common_doodad_iterator {
    data : *common_doodad,
    rem  : c_int,
    index: c_int
}

pub struct shape_doodad {
    name :       xproto::atom,
    type_ :      u8,
    priority :   u8,
    top :        i16,
    left :       i16,
    angle :      i16,
    colorNdx :   u8,
    shapeNdx :   u8,
    pad0 :       [u8,..6]
}

/**
 * @brief shape_doodad_iterator
 **/
pub struct shape_doodad_iterator {
    data : *shape_doodad,
    rem  : c_int,
    index: c_int
}

pub struct text_doodad {
    name :       xproto::atom,
    type_ :      u8,
    priority :   u8,
    top :        i16,
    left :       i16,
    angle :      i16,
    width :      u16,
    height :     u16,
    colorNdx :   u8,
    pad0 :       [u8,..3]
}

/**
 * @brief text_doodad_iterator
 **/
pub struct text_doodad_iterator {
    data : *text_doodad,
    rem  : c_int,
    index: c_int
}

pub struct indicator_doodad {
    name :          xproto::atom,
    type_ :         u8,
    priority :      u8,
    top :           i16,
    left :          i16,
    angle :         i16,
    shapeNdx :      u8,
    onColorNdx :    u8,
    offColorNdx :   u8,
    pad0 :          [u8,..5]
}

/**
 * @brief indicator_doodad_iterator
 **/
pub struct indicator_doodad_iterator {
    data : *indicator_doodad,
    rem  : c_int,
    index: c_int
}

pub struct logo_doodad {
    name :       xproto::atom,
    type_ :      u8,
    priority :   u8,
    top :        i16,
    left :       i16,
    angle :      i16,
    colorNdx :   u8,
    shapeNdx :   u8,
    pad0 :       [u8,..6]
}

/**
 * @brief logo_doodad_iterator
 **/
pub struct logo_doodad_iterator {
    data : *logo_doodad,
    rem  : c_int,
    index: c_int
}

pub struct doodad {
    data : [u8,..20]
}

/**
 * @brief doodad_iterator
 **/
pub struct doodad_iterator {
    data : *doodad,
    rem  : c_int,
    index: c_int
}

pub struct section {
    name :        xproto::atom,
    top :         i16,
    left :        i16,
    width :       u16,
    height :      u16,
    angle :       i16,
    priority :    u8,
    nRows :       u8,
    nDoodads :    u8,
    nOverlays :   u8,
    pad0 :        [u8,..2]
}

/**
 * @brief section_iterator
 **/
pub struct section_iterator {
    data : *section,
    rem  : c_int,
    index: c_int
}

pub struct listing {
    flags :    u16,
    length :   u16
}

/**
 * @brief listing_iterator
 **/
pub struct listing_iterator {
    data : *listing,
    rem  : c_int,
    index: c_int
}

pub struct device_led_info {
    ledClass :         led_class_spec,
    ledID :            id_spec,
    namesPresent :     u32,
    mapsPresent :      u32,
    physIndicators :   u32,
    state :            u32
}

/**
 * @brief device_led_info_iterator
 **/
pub struct device_led_info_iterator {
    data : *device_led_info,
    rem  : c_int,
    index: c_int
}

pub type error = c_uint;//{
    pub static XCB_XKB_ERROR_BAD_DEVICE : error = 255;
    pub static XCB_XKB_ERROR_BAD_CLASS : error = 254;
    pub static XCB_XKB_ERROR_BAD_ID : error = 253;
//}

/** Opcode for xcb_xkb_keyboard. */
pub static XCB_XKB_KEYBOARD : c_int = 0;

pub struct keyboard_error {
    response_type :   u8,
    error_code :      u8,
    sequence :        u16,
    value :           u32,
    minorOpcode :     u16,
    majorOpcode :     u8,
    pad0 :            [u8,..21]
}

pub type sa = c_uint;//{
    pub static XCB_XKB_SA_CLEAR_LOCKS : sa = 1;
    pub static XCB_XKB_SA_LATCH_TO_LOCK : sa = 2;
    pub static XCB_XKB_SA_USE_MOD_MAP_MODS : sa = 4;
    pub static XCB_XKB_SA_GROUP_ABSOLUTE : sa = 4;
//}

pub type sa_type = c_uint;//{
    pub static XCB_XKB_SA_TYPE_NO_ACTION : sa_type = 0;
    pub static XCB_XKB_SA_TYPE_SET_MODS : sa_type = 1;
    pub static XCB_XKB_SA_TYPE_LATCH_MODS : sa_type = 2;
    pub static XCB_XKB_SA_TYPE_LOCK_MODS : sa_type = 3;
    pub static XCB_XKB_SA_TYPE_SET_GROUP : sa_type = 4;
    pub static XCB_XKB_SA_TYPE_LATCH_GROUP : sa_type = 5;
    pub static XCB_XKB_SA_TYPE_LOCK_GROUP : sa_type = 6;
    pub static XCB_XKB_SA_TYPE_MOVE_PTR : sa_type = 7;
    pub static XCB_XKB_SA_TYPE_PTR_BTN : sa_type = 8;
    pub static XCB_XKB_SA_TYPE_LOCK_PTR_BTN : sa_type = 9;
    pub static XCB_XKB_SA_TYPE_SET_PTR_DFLT : sa_type = 10;
    pub static XCB_XKB_SA_TYPE_ISO_LOCK : sa_type = 11;
    pub static XCB_XKB_SA_TYPE_TERMINATE : sa_type = 12;
    pub static XCB_XKB_SA_TYPE_SWITCH_SCREEN : sa_type = 13;
    pub static XCB_XKB_SA_TYPE_SET_CONTROLS : sa_type = 14;
    pub static XCB_XKB_SA_TYPE_LOCK_CONTROLS : sa_type = 15;
    pub static XCB_XKB_SA_TYPE_ACTION_MESSAGE : sa_type = 16;
    pub static XCB_XKB_SA_TYPE_REDIRECT_KEY : sa_type = 17;
    pub static XCB_XKB_SA_TYPE_DEVICE_BTN : sa_type = 18;
    pub static XCB_XKB_SA_TYPE_LOCK_DEVICE_BTN : sa_type = 19;
    pub static XCB_XKB_SA_TYPE_DEVICE_VALUATOR : sa_type = 20;
//}

pub struct sa_no_action {
    type_ :   u8,
    pad0 :    [u8,..7]
}

/**
 * @brief sa_no_action_iterator
 **/
pub struct sa_no_action_iterator {
    data : *sa_no_action,
    rem  : c_int,
    index: c_int
}

pub struct sa_set_mods {
    type_ :       u8,
    flags :       u8,
    mask :        u8,
    realMods :    u8,
    vmodsHigh :   u8,
    vmodsLow :    u8,
    pad0 :        [u8,..2]
}

/**
 * @brief sa_set_mods_iterator
 **/
pub struct sa_set_mods_iterator {
    data : *sa_set_mods,
    rem  : c_int,
    index: c_int
}

pub struct sa_latch_mods {
    type_ :       u8,
    flags :       u8,
    mask :        u8,
    realMods :    u8,
    vmodsHigh :   u8,
    vmodsLow :    u8,
    pad0 :        [u8,..2]
}

/**
 * @brief sa_latch_mods_iterator
 **/
pub struct sa_latch_mods_iterator {
    data : *sa_latch_mods,
    rem  : c_int,
    index: c_int
}

pub struct sa_lock_mods {
    type_ :       u8,
    flags :       u8,
    mask :        u8,
    realMods :    u8,
    vmodsHigh :   u8,
    vmodsLow :    u8,
    pad0 :        [u8,..2]
}

/**
 * @brief sa_lock_mods_iterator
 **/
pub struct sa_lock_mods_iterator {
    data : *sa_lock_mods,
    rem  : c_int,
    index: c_int
}

pub struct sa_set_group {
    type_ :   u8,
    flags :   u8,
    group :   i8,
    pad0 :    [u8,..5]
}

/**
 * @brief sa_set_group_iterator
 **/
pub struct sa_set_group_iterator {
    data : *sa_set_group,
    rem  : c_int,
    index: c_int
}

pub struct sa_latch_group {
    type_ :   u8,
    flags :   u8,
    group :   i8,
    pad0 :    [u8,..5]
}

/**
 * @brief sa_latch_group_iterator
 **/
pub struct sa_latch_group_iterator {
    data : *sa_latch_group,
    rem  : c_int,
    index: c_int
}

pub struct sa_lock_group {
    type_ :   u8,
    flags :   u8,
    group :   i8,
    pad0 :    [u8,..5]
}

/**
 * @brief sa_lock_group_iterator
 **/
pub struct sa_lock_group_iterator {
    data : *sa_lock_group,
    rem  : c_int,
    index: c_int
}

pub type sa_move_ptr_flag = c_uint;//{
    pub static XCB_XKB_SA_MOVE_PTR_FLAG_NO_ACCELERATION : sa_move_ptr_flag = 1;
    pub static XCB_XKB_SA_MOVE_PTR_FLAG_MOVE_ABSOLUTE_X : sa_move_ptr_flag = 2;
    pub static XCB_XKB_SA_MOVE_PTR_FLAG_MOVE_ABSOLUTE_Y : sa_move_ptr_flag = 4;
//}

pub struct sa_move_ptr {
    type_ :   u8,
    flags :   u8,
    xHigh :   i8,
    xLow :    u8,
    yHigh :   i8,
    yLow :    u8,
    pad0 :    [u8,..2]
}

/**
 * @brief sa_move_ptr_iterator
 **/
pub struct sa_move_ptr_iterator {
    data : *sa_move_ptr,
    rem  : c_int,
    index: c_int
}

pub struct sa_ptr_btn {
    type_ :    u8,
    flags :    u8,
    count :    u8,
    button :   u8,
    pad0 :     [u8,..4]
}

/**
 * @brief sa_ptr_btn_iterator
 **/
pub struct sa_ptr_btn_iterator {
    data : *sa_ptr_btn,
    rem  : c_int,
    index: c_int
}

pub struct sa_lock_ptr_btn {
    type_ :    u8,
    flags :    u8,
    pad0 :     u8,
    button :   u8,
    pad1 :     [u8,..4]
}

/**
 * @brief sa_lock_ptr_btn_iterator
 **/
pub struct sa_lock_ptr_btn_iterator {
    data : *sa_lock_ptr_btn,
    rem  : c_int,
    index: c_int
}

pub type sa_set_ptr_dflt_flag = c_uint;//{
    pub static XCB_XKB_SA_SET_PTR_DFLT_FLAG_DFLT_BTN_ABSOLUTE : sa_set_ptr_dflt_flag = 2;
    pub static XCB_XKB_SA_SET_PTR_DFLT_FLAG_AFFECT_DFLT_BUTTON : sa_set_ptr_dflt_flag = 1;
//}

pub struct sa_set_ptr_dflt {
    type_ :    u8,
    flags :    u8,
    affect :   u8,
    value :    i8,
    pad0 :     [u8,..4]
}

/**
 * @brief sa_set_ptr_dflt_iterator
 **/
pub struct sa_set_ptr_dflt_iterator {
    data : *sa_set_ptr_dflt,
    rem  : c_int,
    index: c_int
}

pub type sa_iso_lock_flag = c_uint;//{
    pub static XCB_XKB_SA_ISO_LOCK_FLAG_NO_LOCK : sa_iso_lock_flag = 1;
    pub static XCB_XKB_SA_ISO_LOCK_FLAG_NO_UNLOCK : sa_iso_lock_flag = 2;
    pub static XCB_XKB_SA_ISO_LOCK_FLAG_USE_MOD_MAP_MODS : sa_iso_lock_flag = 4;
    pub static XCB_XKB_SA_ISO_LOCK_FLAG_GROUP_ABSOLUTE : sa_iso_lock_flag = 4;
    pub static XCB_XKB_SA_ISO_LOCK_FLAG_ISO_DFLT_IS_GROUP : sa_iso_lock_flag = 8;
//}

pub type sa_iso_lock_no_affect = c_uint;//{
    pub static XCB_XKB_SA_ISO_LOCK_NO_AFFECT_CTRLS : sa_iso_lock_no_affect = 8;
    pub static XCB_XKB_SA_ISO_LOCK_NO_AFFECT_PTR : sa_iso_lock_no_affect = 16;
    pub static XCB_XKB_SA_ISO_LOCK_NO_AFFECT_GROUP : sa_iso_lock_no_affect = 32;
    pub static XCB_XKB_SA_ISO_LOCK_NO_AFFECT_MODS : sa_iso_lock_no_affect = 64;
//}

pub struct sa_iso_lock {
    type_ :       u8,
    flags :       u8,
    mask :        u8,
    realMods :    u8,
    group :       i8,
    affect :      u8,
    vmodsHigh :   u8,
    vmodsLow :    u8
}

/**
 * @brief sa_iso_lock_iterator
 **/
pub struct sa_iso_lock_iterator {
    data : *sa_iso_lock,
    rem  : c_int,
    index: c_int
}

pub struct sa_terminate {
    type_ :   u8,
    pad0 :    [u8,..7]
}

/**
 * @brief sa_terminate_iterator
 **/
pub struct sa_terminate_iterator {
    data : *sa_terminate,
    rem  : c_int,
    index: c_int
}

pub type switch_screen_flag = c_uint;//{
    pub static XCB_XKB_SWITCH_SCREEN_FLAG_APPLICATION : switch_screen_flag = 1;
    pub static XCB_XKB_SWITCH_SCREEN_FLAG_ABSOLUTE : switch_screen_flag = 4;
//}

pub struct sa_switch_screen {
    type_ :       u8,
    flags :       u8,
    newScreen :   i8,
    pad0 :        [u8,..5]
}

/**
 * @brief sa_switch_screen_iterator
 **/
pub struct sa_switch_screen_iterator {
    data : *sa_switch_screen,
    rem  : c_int,
    index: c_int
}

pub type bool_ctrls_high = c_uint;//{
    pub static XCB_XKB_BOOL_CTRLS_HIGH_ACCESS_X_FEEDBACK : bool_ctrls_high = 1;
    pub static XCB_XKB_BOOL_CTRLS_HIGH_AUDIBLE_BELL : bool_ctrls_high = 2;
    pub static XCB_XKB_BOOL_CTRLS_HIGH_OVERLAY_1 : bool_ctrls_high = 4;
    pub static XCB_XKB_BOOL_CTRLS_HIGH_OVERLAY_2 : bool_ctrls_high = 8;
    pub static XCB_XKB_BOOL_CTRLS_HIGH_IGNORE_GROUP_LOCK : bool_ctrls_high = 16;
//}

pub type bool_ctrls_low = c_uint;//{
    pub static XCB_XKB_BOOL_CTRLS_LOW_REPEAT_KEYS : bool_ctrls_low = 1;
    pub static XCB_XKB_BOOL_CTRLS_LOW_SLOW_KEYS : bool_ctrls_low = 2;
    pub static XCB_XKB_BOOL_CTRLS_LOW_BOUNCE_KEYS : bool_ctrls_low = 4;
    pub static XCB_XKB_BOOL_CTRLS_LOW_STICKY_KEYS : bool_ctrls_low = 8;
    pub static XCB_XKB_BOOL_CTRLS_LOW_MOUSE_KEYS : bool_ctrls_low = 16;
    pub static XCB_XKB_BOOL_CTRLS_LOW_MOUSE_KEYS_ACCEL : bool_ctrls_low = 32;
    pub static XCB_XKB_BOOL_CTRLS_LOW_ACCESS_X_KEYS : bool_ctrls_low = 64;
    pub static XCB_XKB_BOOL_CTRLS_LOW_ACCESS_X_TIMEOUT : bool_ctrls_low = 128;
//}

pub struct sa_set_controls {
    type_ :           u8,
    pad0 :            [u8,..3],
    boolCtrlsHigh :   u8,
    boolCtrlsLow :    u8,
    pad1 :            [u8,..2]
}

/**
 * @brief sa_set_controls_iterator
 **/
pub struct sa_set_controls_iterator {
    data : *sa_set_controls,
    rem  : c_int,
    index: c_int
}

pub struct sa_lock_controls {
    type_ :           u8,
    pad0 :            [u8,..3],
    boolCtrlsHigh :   u8,
    boolCtrlsLow :    u8,
    pad1 :            [u8,..2]
}

/**
 * @brief sa_lock_controls_iterator
 **/
pub struct sa_lock_controls_iterator {
    data : *sa_lock_controls,
    rem  : c_int,
    index: c_int
}

pub type action_message_flag = c_uint;//{
    pub static XCB_XKB_ACTION_MESSAGE_FLAG_ON_PRESS : action_message_flag = 1;
    pub static XCB_XKB_ACTION_MESSAGE_FLAG_ON_RELEASE : action_message_flag = 2;
    pub static XCB_XKB_ACTION_MESSAGE_FLAG_GEN_KEY_EVENT : action_message_flag = 4;
//}

pub struct sa_action_message {
    type_ :     u8,
    flags :     u8,
    message :   [u8,..6]
}

/**
 * @brief sa_action_message_iterator
 **/
pub struct sa_action_message_iterator {
    data : *sa_action_message,
    rem  : c_int,
    index: c_int
}

pub struct sa_redirect_key {
    type_ :           u8,
    newkey :          xproto::keycode,
    mask :            u8,
    realModifiers :   u8,
    vmodsMaskHigh :   u8,
    vmodsMaskLow :    u8,
    vmodsHigh :       u8,
    vmodsLow :        u8
}

/**
 * @brief sa_redirect_key_iterator
 **/
pub struct sa_redirect_key_iterator {
    data : *sa_redirect_key,
    rem  : c_int,
    index: c_int
}

pub struct sa_device_btn {
    type_ :    u8,
    flags :    u8,
    count :    u8,
    button :   u8,
    device :   u8,
    pad0 :     [u8,..3]
}

/**
 * @brief sa_device_btn_iterator
 **/
pub struct sa_device_btn_iterator {
    data : *sa_device_btn,
    rem  : c_int,
    index: c_int
}

pub type lock_device_flags = c_uint;//{
    pub static XCB_XKB_LOCK_DEVICE_FLAGS_NO_LOCK : lock_device_flags = 1;
    pub static XCB_XKB_LOCK_DEVICE_FLAGS_NO_UNLOCK : lock_device_flags = 2;
//}

pub struct sa_lock_device_btn {
    type_ :    u8,
    flags :    u8,
    pad0 :     u8,
    button :   u8,
    device :   u8
}

/**
 * @brief sa_lock_device_btn_iterator
 **/
pub struct sa_lock_device_btn_iterator {
    data : *sa_lock_device_btn,
    rem  : c_int,
    index: c_int
}

pub type sa_val_what = c_uint;//{
    pub static XCB_XKB_SA_VAL_WHAT_IGNORE_VAL : sa_val_what = 0;
    pub static XCB_XKB_SA_VAL_WHAT_SET_VAL_MIN : sa_val_what = 1;
    pub static XCB_XKB_SA_VAL_WHAT_SET_VAL_CENTER : sa_val_what = 2;
    pub static XCB_XKB_SA_VAL_WHAT_SET_VAL_MAX : sa_val_what = 3;
    pub static XCB_XKB_SA_VAL_WHAT_SET_VAL_RELATIVE : sa_val_what = 4;
    pub static XCB_XKB_SA_VAL_WHAT_SET_VAL_ABSOLUTE : sa_val_what = 5;
//}

pub struct sa_device_valuator {
    type_ :       u8,
    device :      u8,
    val1what :    u8,
    val1index :   u8,
    val1value :   u8,
    val2what :    u8,
    val2index :   u8,
    val2value :   u8
}

/**
 * @brief sa_device_valuator_iterator
 **/
pub struct sa_device_valuator_iterator {
    data : *sa_device_valuator,
    rem  : c_int,
    index: c_int
}

pub struct action {
    data : [u8,..8]
}

/**
 * @brief action_iterator
 **/
pub struct action_iterator {
    data : *action,
    rem  : c_int,
    index: c_int
}

pub struct use_extension_cookie {
    sequence : c_uint
}

/** Opcode for xcb_xkb_use_extension. */
pub static XCB_XKB_USE_EXTENSION : c_int = 0;

pub struct use_extension_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    wantedMajor :    u16,
    wantedMinor :    u16
}

pub struct use_extension_reply {
    response_type :   u8,
    supported :       u8,
    sequence :        u16,
    length :          u32,
    serverMajor :     u16,
    serverMinor :     u16,
    pad0 :            [u8,..20]
}

pub struct select_events_details {
    affectNewKeyboard :       u16,
    newKeyboardDetails :      u16,
    affectState :             u16,
    stateDetails :            u16,
    affectCtrls :             u32,
    ctrlDetails :             u32,
    affectIndicatorState :    u32,
    indicatorStateDetails :   u32,
    affectIndicatorMap :      u32,
    indicatorMapDetails :     u32,
    affectNames :             u16,
    namesDetails :            u16,
    affectCompat :            u8,
    compatDetails :           u8,
    affectBell :              u8,
    bellDetails :             u8,
    affectMsgDetails :        u8,
    msgDetails :              u8,
    affectAccessX :           u16,
    accessXDetails :          u16,
    affectExtDev :            u16
    extdevDetails :           u16
}

/** Opcode for xcb_xkb_select_events. */
pub static XCB_XKB_SELECT_EVENTS : c_int = 1;

pub struct select_events_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    deviceSpec :     device_spec,
    affectWhich :    u16,
    clear :          u16,
    selectAll :      u16,
    affectMap :      u16,
    map :            u16
}

/** Opcode for xcb_xkb_bell. */
pub static XCB_XKB_BELL : c_int = 3;

pub struct bell_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    deviceSpec :     device_spec,
    bellClass :      bell_class_spec,
    bellID :         id_spec,
    percent :        i8,
    forceSound :     u8,
    eventOnly :      u8,
    pad0 :           u8,
    pitch :          i16,
    duration :       i16,
    pad1 :           [u8,..2],
    name :           xproto::atom,
    window :         xproto::window
}

pub struct get_state_cookie {
    sequence : c_uint
}

/** Opcode for xcb_xkb_get_state. */
pub static XCB_XKB_GET_STATE : c_int = 4;

pub struct get_state_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    deviceSpec :     device_spec,
    pad0 :           [u8,..2]
}

pub struct get_state_reply {
    response_type :      u8,
    deviceID :           u8,
    sequence :           u16,
    length :             u32,
    mods :               u8,
    baseMods :           u8,
    latchedMods :        u8,
    lockedMods :         u8,
    group :              u8,
    lockedGroup :        u8,
    baseGroup :          i16,
    latchedGroup :       i16,
    compatState :        u8,
    grabMods :           u8,
    compatGrabMods :     u8,
    lookupMods :         u8,
    compatLookupMods :   u8,
    pad0 :               u8,
    ptrBtnState :        u16,
    pad1 :               [u8,..6]
}

/** Opcode for xcb_xkb_latch_lock_state. */
pub static XCB_XKB_LATCH_LOCK_STATE : c_int = 5;

pub struct latch_lock_state_request {
    major_opcode :       u8,
    minor_opcode :       u8,
    length :             u16,
    deviceSpec :         device_spec,
    affectModLocks :     u8,
    modLocks :           u8,
    lockGroup :          u8,
    groupLock :          u8,
    affectModLatches :   u8,
    pad0 :               u8,
    latchGroup :         u8,
    groupLatch :         u16
}

pub struct get_controls_cookie {
    sequence : c_uint
}

/** Opcode for xcb_xkb_get_controls. */
pub static XCB_XKB_GET_CONTROLS : c_int = 6;

pub struct get_controls_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    deviceSpec :     device_spec,
    pad0 :           [u8,..2]
}

pub struct get_controls_reply {
    response_type :                 u8,
    deviceID :                      u8,
    sequence :                      u16,
    length :                        u32,
    mouseKeysDfltBtn :              u8,
    numGroups :                     u8,
    groupsWrap :                    u8,
    internalModsMask :              u8,
    ignoreLockModsMask :            u8,
    internalModsRealMods :          u8,
    ignoreLockModsRealMods :        u8,
    pad0 :                          u8,
    internalModsVmods :             u16,
    ignoreLockModsVmods :           u16,
    repeatDelay :                   u16,
    repeatInterval :                u16,
    slowKeysDelay :                 u16,
    debounceDelay :                 u16,
    mouseKeysDelay :                u16,
    mouseKeysInterval :             u16,
    mouseKeysTimeToMax :            u16,
    mouseKeysMaxSpeed :             u16,
    mouseKeysCurve :                i16,
    accessXOption :                 ax_option,
    accessXTimeout :                u16,
    accessXTimeoutOptionsMask :     ax_option,
    accessXTimeoutOptionsValues :   ax_option,
    pad1 :                          [u8,..2],
    accessXTimeoutMask :            u32,
    accessXTimeoutValues :          u32,
    enabledControls :               u32,
    perKeyRepeat :                  [u8,..32]
}

/** Opcode for xcb_xkb_set_controls. */
pub static XCB_XKB_SET_CONTROLS : c_int = 7;

pub struct set_controls_request {
    major_opcode :                  u8,
    minor_opcode :                  u8,
    length :                        u16,
    deviceSpec :                    device_spec,
    affectInternalRealMods :        u8,
    internalRealMods :              u8,
    affectIgnoreLockRealMods :      u8,
    ignoreLockRealMods :            u8,
    affectInternalVirtualMods :     u16,
    internalVirtualMods :           u16,
    affectIgnoreLockVirtualMods :   u16,
    ignoreLockVirtualMods :         u16,
    mouseKeysDfltBtn :              u8,
    groupsWrap :                    u8,
    accessXOptions :                ax_option,
    pad0 :                          [u8,..2],
    affectEnabledControls :         u32,
    enabledControls :               u32,
    changeControls :                u32,
    repeatDelay :                   u16,
    repeatInterval :                u16,
    slowKeysDelay :                 u16,
    debounceDelay :                 u16,
    mouseKeysDelay :                u16,
    mouseKeysInterval :             u16,
    mouseKeysTimeToMax :            u16,
    mouseKeysMaxSpeed :             u16,
    mouseKeysCurve :                i16,
    accessXTimeout :                u16,
    accessXTimeoutMask :            u32,
    accessXTimeoutValues :          u32,
    accessXTimeoutOptionsMask :     ax_option,
    accessXTimeoutOptionsValues :   ax_option,
    perKeyRepeat :                  [u8,..32]
}

pub struct get_map_cookie {
    sequence : c_uint
}

/** Opcode for xcb_xkb_get_map. */
pub static XCB_XKB_GET_MAP : c_int = 8;

pub struct get_map_request {
    major_opcode :       u8,
    minor_opcode :       u8,
    length :             u16,
    deviceSpec :         device_spec,
    full :               u16,
    partial :            u16,
    firstType :          u8,
    nTypes :             u8,
    firstKeySym :        xproto::keycode,
    nKeySyms :           u8,
    firstKeyAction :     xproto::keycode,
    nKeyActions :        u8,
    firstKeyBehavior :   xproto::keycode,
    nKeyBehaviors :      u8,
    virtualMods :        u16,
    firstKeyExplicit :   xproto::keycode,
    nKeyExplicit :       u8,
    firstModMapKey :     xproto::keycode,
    nModMapKeys :        u8,
    firstVModMapKey :    xproto::keycode,
    nVModMapKeys :       u8,
    pad0 :               [u8,..2]
}

pub struct get_map_map {
    types_rtrn :         *key_type,
    syms_rtrn :      *key_sym_map,
    acts_rtrn_count :               *u8,
    acts_rtrn_acts :           *action,
    behaviors_rtrn :     *set_behavior,
    vmods_rtrn :               *u8,
    explicit_rtrn :     *set_explicit,
    modmap_rtrn :      *key_mod_map,
    vmodmap_rtrn :    *key_v_mod_map
}

pub struct get_map_reply {
    response_type :       u8,
    deviceID :            u8,
    sequence :            u16,
    length :              u32,
    pad0 :                [u8,..2],
    minKeyCode :          xproto::keycode,
    maxKeyCode :          xproto::keycode,
    present :             u16,
    firstType :           u8,
    nTypes :              u8,
    totalTypes :          u8,
    firstKeySym :         xproto::keycode,
    totalSyms :           u16,
    nKeySyms :            u8,
    firstKeyAction :      xproto::keycode,
    totalActions :        u16,
    nKeyActions :         u8,
    firstKeyBehavior :    xproto::keycode,
    nKeyBehaviors :       u8,
    totalKeyBehaviors :   u8,
    firstKeyExplicit :    xproto::keycode,
    nKeyExplicit :        u8,
    totalKeyExplicit :    u8,
    firstModMapKey :      xproto::keycode,
    nModMapKeys :         u8,
    totalModMapKeys :     u8,
    firstVModMapKey :     xproto::keycode,
    nVModMapKeys :        u8,
    totalVModMapKeys :    u8,
    pad1 :                u8,
    virtualMods :         u16
}

pub struct set_map_values {
    types :   *set_key_type,
    syms :   *key_sym_map,
    actionsCount :            *u8,
    actions :        *action,
    behaviors :   *set_behavior,
    vmods :            *u8,
    explicit :   *set_explicit,
    modmap :   *key_mod_map,
    vmodmap :   *key_v_mod_map
}

/** Opcode for xcb_xkb_set_map. */
pub static XCB_XKB_SET_MAP : c_int = 9;

pub struct set_map_request {
    major_opcode :        u8,
    minor_opcode :        u8,
    length :              u16,
    deviceSpec :          device_spec,
    present :             u16,
    flags :               u16,
    minKeyCode :          xproto::keycode,
    maxKeyCode :          xproto::keycode,
    firstType :           u8,
    nTypes :              u8,
    firstKeySym :         xproto::keycode,
    nKeySyms :            u8,
    totalSyms :           u16,
    firstKeyAction :      xproto::keycode,
    nKeyActions :         u8,
    totalActions :        u16,
    firstKeyBehavior :    xproto::keycode,
    nKeyBehaviors :       u8,
    totalKeyBehaviors :   u8,
    firstKeyExplicit :    xproto::keycode,
    nKeyExplicit :        u8,
    totalKeyExplicit :    u8,
    firstModMapKey :      xproto::keycode,
    nModMapKeys :         u8,
    totalModMapKeys :     u8,
    firstVModMapKey :     xproto::keycode,
    nVModMapKeys :        u8,
    totalVModMapKeys :    u8,
    virtualMods :         u16
}

pub struct get_compat_map_cookie {
    sequence : c_uint
}

/** Opcode for xcb_xkb_get_compat_map. */
pub static XCB_XKB_GET_COMPAT_MAP : c_int = 10;

pub struct get_compat_map_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    deviceSpec :     device_spec,
    groups :         u8,
    getAllSI :       u8,
    firstSI :        u16,
    nSI :            u16
}

pub struct get_compat_map_reply {
    response_type :   u8,
    deviceID :        u8,
    sequence :        u16,
    length :          u32,
    groupsRtrn :      u8,
    pad0 :            u8,
    firstSIRtrn :     u16,
    nSIRtrn :         u16,
    nTotalSI :        u16,
    pad1 :            [u8,..16]
}

/** Opcode for xcb_xkb_set_compat_map. */
pub static XCB_XKB_SET_COMPAT_MAP : c_int = 11;

pub struct set_compat_map_request {
    major_opcode :       u8,
    minor_opcode :       u8,
    length :             u16,
    deviceSpec :         device_spec,
    pad0 :               u8,
    recomputeActions :   u8,
    truncateSI :         u8,
    groups :             u8,
    firstSI :            u16,
    nSI :                u16,
    pad1 :               [u8,..2]
}

pub struct get_indicator_state_cookie {
    sequence : c_uint
}

/** Opcode for xcb_xkb_get_indicator_state. */
pub static XCB_XKB_GET_INDICATOR_STATE : c_int = 12;

pub struct get_indicator_state_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    deviceSpec :     device_spec,
    pad0 :           [u8,..2]
}

pub struct get_indicator_state_reply {
    response_type :   u8,
    deviceID :        u8,
    sequence :        u16,
    length :          u32,
    state :           u32,
    pad0 :            [u8,..20]
}

pub struct get_indicator_map_cookie {
    sequence : c_uint
}

/** Opcode for xcb_xkb_get_indicator_map. */
pub static XCB_XKB_GET_INDICATOR_MAP : c_int = 13;

pub struct get_indicator_map_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    deviceSpec :     device_spec,
    pad0 :           [u8,..2],
    which :          u32
}

pub struct get_indicator_map_reply {
    response_type :    u8,
    deviceID :         u8,
    sequence :         u16,
    length :           u32,
    which :            u32,
    realIndicators :   u32,
    nIndicators :      u8,
    pad0 :             [u8,..15]
}

/** Opcode for xcb_xkb_set_indicator_map. */
pub static XCB_XKB_SET_INDICATOR_MAP : c_int = 14;

pub struct set_indicator_map_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    deviceSpec :     device_spec,
    pad0 :           [u8,..2],
    which :          u32
}

pub struct get_named_indicator_cookie {
    sequence : c_uint
}

/** Opcode for xcb_xkb_get_named_indicator. */
pub static XCB_XKB_GET_NAMED_INDICATOR : c_int = 15;

pub struct get_named_indicator_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    deviceSpec :     device_spec,
    ledClass :       led_class_spec,
    ledID :          id_spec,
    pad0 :           [u8,..2],
    indicator :      xproto::atom
}

pub struct get_named_indicator_reply {
    response_type :     u8,
    deviceID :          u8,
    sequence :          u16,
    length :            u32,
    indicator :         xproto::atom,
    found :             u8,
    on :                u8,
    realIndicator :     u8,
    ndx :               u8,
    map_flags :         u8,
    map_whichGroups :   u8,
    map_groups :        u8,
    map_whichMods :     u8,
    map_mods :          u8,
    map_realMods :      u8,
    map_vmod :          u16,
    map_ctrls :         u32,
    pad0 :              [u8,..3]
}

/** Opcode for xcb_xkb_set_named_indicator. */
pub static XCB_XKB_SET_NAMED_INDICATOR : c_int = 16;

pub struct set_named_indicator_request {
    major_opcode :      u8,
    minor_opcode :      u8,
    length :            u16,
    deviceSpec :        device_spec,
    ledClass :          led_class_spec,
    ledID :             id_spec,
    pad0 :              [u8,..2],
    indicator :         xproto::atom,
    setState :          u8,
    on :                u8,
    setMap :            u8,
    createMap :         u8,
    pad1 :              u8,
    map_flags :         u8,
    map_whichGroups :   u8,
    map_groups :        u8,
    map_whichMods :     u8,
    map_realMods :      u8,
    map_vmods :         u16,
    map_ctrls :         u32
}

pub struct get_names_cookie {
    sequence : c_uint
}

/** Opcode for xcb_xkb_get_names. */
pub static XCB_XKB_GET_NAMES : c_int = 17;

pub struct get_names_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    deviceSpec :     device_spec,
    pad0 :           [u8,..2],
    which :          u32
}

pub struct get_names_value_list {
    keycodesName :      xproto::atom,
    geometryName :      xproto::atom,
    symbolsName :       xproto::atom,
    physSymbolsName :   xproto::atom,
    typesName :         xproto::atom,
    compatName :        xproto::atom,
    typeNames :     *xproto::atom,
    nLevelsPerType :               *u8,
    ktLevelNames :     *xproto::atom,
    indicatorNames :     *xproto::atom,
    virtualModNames :     *xproto::atom,
    groups :     *xproto::atom,
    keyNames :         *key_name,
    keyAliases :        *key_alias,
    radioGroupNames :     *xproto::atom
}

pub struct get_names_reply {
    response_type :   u8,
    deviceID :        u8,
    sequence :        u16,
    length :          u32,
    which :           u32,
    minKeyCode :      xproto::keycode,
    maxKeyCode :      xproto::keycode,
    nTypes :          u8,
    groupNames :      u8,
    virtualMods :     u16,
    firstKey :        xproto::keycode,
    nKeys :           u8,
    indicators :      u32,
    nRadioGroups :    u8,
    nKeyAliases :     u8,
    nKTLevels :       u16,
    pad0 :            [u8,..4]
}

pub struct set_names_values {
    keycodesName :      xproto::atom,
    geometryName :      xproto::atom,
    symbolsName :       xproto::atom,
    physSymbolsName :   xproto::atom,
    typesName :         xproto::atom,
    compatName :        xproto::atom,
    typeNames :     *xproto::atom,
    nLevelsPerType :               *u8,
    ktLevelNames :     *xproto::atom,
    indicatorNames :     *xproto::atom,
    virtualModNames :     *xproto::atom,
    groups :     *xproto::atom,
    keyNames :         *key_name,
    keyAliases :        *key_alias,
    radioGroupNames :     *xproto::atom
}

/** Opcode for xcb_xkb_set_names. */
pub static XCB_XKB_SET_NAMES : c_int = 18;

pub struct set_names_request {
    major_opcode :        u8,
    minor_opcode :        u8,
    length :              u16,
    deviceSpec :          device_spec,
    virtualMods :         u16,
    which :               u32,
    firstType :           u8,
    nTypes :              u8,
    firstKTLevelt :       u8,
    nKTLevels :           u8,
    indicators :          u32,
    groupNames :          u8,
    nRadioGroups :        u8,
    firstKey :            xproto::keycode,
    nKeys :               u8,
    nKeyAliases :         u8,
    pad0 :                u8,
    totalKTLevelNames :   u16
}

pub struct get_geometry_cookie {
    sequence : c_uint
}

/** Opcode for xcb_xkb_get_geometry. */
pub static XCB_XKB_GET_GEOMETRY : c_int = 19;

pub struct get_geometry_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    deviceSpec :     device_spec,
    pad0 :           [u8,..2],
    name :           xproto::atom
}

pub struct get_geometry_reply {
    response_type :   u8,
    deviceID :        u8,
    sequence :        u16,
    length :          u32,
    name :            xproto::atom,
    found :           u8,
    pad0 :            u8,
    widthMM :         u16,
    heightMM :        u16,
    nProperties :     u16,
    nColors :         u16,
    nShapes :         u16,
    nSections :       u16,
    nDoodads :        u16,
    nKeyAliases :     u16,
    baseColorNdx :    u8,
    labelColorNdx :   u8
}

/** Opcode for xcb_xkb_set_geometry. */
pub static XCB_XKB_SET_GEOMETRY : c_int = 20;

pub struct set_geometry_request {
    major_opcode :    u8,
    minor_opcode :    u8,
    length :          u16,
    deviceSpec :      device_spec,
    nShapes :         u8,
    nSections :       u8,
    name :            xproto::atom,
    widthMM :         u16,
    heightMM :        u16,
    nProperties :     u16,
    nColors :         u16,
    nDoodads :        u16,
    nKeyAliases :     u16,
    baseColorNdx :    u8,
    labelColorNdx :   u8,
    pad0 :            [u8,..2]
}

pub struct per_client_flags_cookie {
    sequence : c_uint
}

/** Opcode for xcb_xkb_per_client_flags. */
pub static XCB_XKB_PER_CLIENT_FLAGS : c_int = 21;

pub struct per_client_flags_request {
    major_opcode :      u8,
    minor_opcode :      u8,
    length :            u16,
    deviceSpec :        device_spec,
    pad0 :              [u8,..2],
    change :            u32,
    value :             u32,
    ctrlsToChange :     u32,
    autoCtrls :         u32,
    autoCtrlsValues :   u32
}

pub struct per_client_flags_reply {
    response_type :     u8,
    deviceID :          u8,
    sequence :          u16,
    length :            u32,
    supported :         u32,
    value :             u32,
    autoCtrls :         u32,
    autoCtrlsValues :   u32,
    pad0 :              [u8,..8]
}

pub struct list_components_cookie {
    sequence : c_uint
}

/** Opcode for xcb_xkb_list_components. */
pub static XCB_XKB_LIST_COMPONENTS : c_int = 22;

pub struct list_components_request {
    major_opcode :       u8,
    minor_opcode :       u8,
    length :             u16,
    deviceSpec :         device_spec,
    maxNames :           u16,
    keymapsSpecLen :     u8,
    keycodesSpecLen :    u8,
    typesSpecLen :       u8,
    compatMapSpecLen :   u8,
    symbolsSpecLen :     u8,
    geometrySpecLen :    u8
}

pub struct list_components_reply {
    response_type :   u8,
    deviceID :        u8,
    sequence :        u16,
    length :          u32,
    nKeymaps :        u16,
    nKeycodes :       u16,
    nTypes :          u16,
    nCompatMaps :     u16,
    nSymbols :        u16,
    nGeometries :     u16,
    extra :           u16,
    pad0 :            [u8,..10]
}

pub struct get_kbd_by_name_cookie {
    sequence : c_uint
}

/** Opcode for xcb_xkb_get_kbd_by_name. */
pub static XCB_XKB_GET_KBD_BY_NAME : c_int = 23;

pub struct get_kbd_by_name_request {
    major_opcode :       u8,
    minor_opcode :       u8,
    length :             u16,
    deviceSpec :         device_spec,
    need :               u16,
    want :               u16,
    load :               u8,
    pad0 :               u8,
    keymapsSpecLen :     u8,
    keycodesSpecLen :    u8,
    typesSpecLen :       u8,
    compatMapSpecLen :   u8,
    symbolsSpecLen :     u8,
    geometrySpecLen :    u8
}

pub struct get_kbd_by_name_replies_types_map {
    types_rtrn :         *key_type,
    syms_rtrn :      *key_sym_map,
    acts_rtrn_count :               *u8,
    acts_rtrn_acts :           *action,
    behaviors_rtrn :     *set_behavior,
    vmods_rtrn :               *u8,
    explicit_rtrn :     *set_explicit,
    modmap_rtrn :      *key_mod_map,
    vmodmap_rtrn :    *key_v_mod_map
}

pub struct get_kbd_by_name_replies_client_symbols_map {
    types_rtrn :         *key_type,
    syms_rtrn :      *key_sym_map,
    acts_rtrn_count :               *u8,
    acts_rtrn_acts :           *action,
    behaviors_rtrn :     *set_behavior,
    vmods_rtrn :               *u8,
    explicit_rtrn :     *set_explicit,
    modmap_rtrn :      *key_mod_map,
    vmodmap_rtrn :    *key_v_mod_map
}

pub struct get_kbd_by_name_replies_server_symbols_map {
    types_rtrn :         *key_type,
    syms_rtrn :      *key_sym_map,
    acts_rtrn_count :               *u8,
    acts_rtrn_acts :           *action,
    behaviors_rtrn :     *set_behavior,
    vmods_rtrn :               *u8,
    explicit_rtrn :     *set_explicit,
    modmap_rtrn :      *key_mod_map,
    vmodmap_rtrn :    *key_v_mod_map
}

pub struct get_kbd_by_name_replies_key_names_value_list {
    keycodesName :      xproto::atom,
    geometryName :      xproto::atom,
    symbolsName :       xproto::atom,
    physSymbolsName :   xproto::atom,
    typesName :         xproto::atom,
    compatName :        xproto::atom,
    typeNames :     *xproto::atom,
    nLevelsPerType :               *u8,
    ktLevelNames :     *xproto::atom,
    indicatorNames :     *xproto::atom,
    virtualModNames :     *xproto::atom,
    groups :     *xproto::atom,
    keyNames :         *key_name,
    keyAliases :        *key_alias,
    radioGroupNames :     *xproto::atom
}

pub struct get_kbd_by_name_replies_other_names_value_list {
    keycodesName :      xproto::atom,
    geometryName :      xproto::atom,
    symbolsName :       xproto::atom,
    physSymbolsName :   xproto::atom,
    typesName :         xproto::atom,
    compatName :        xproto::atom,
    typeNames :     *xproto::atom,
    nLevelsPerType :               *u8,
    ktLevelNames :     *xproto::atom,
    indicatorNames :     *xproto::atom,
    virtualModNames :     *xproto::atom,
    groups :     *xproto::atom,
    keyNames :         *key_name,
    keyAliases :        *key_alias,
    radioGroupNames :     *xproto::atom
}

pub struct get_kbd_by_name_replies {
    types : struct _types {
        getmap_type :         u8,
        typeDeviceID :        u8,
        getmap_sequence :     u16,
        getmap_length :       u32,
        pad0 :                [u8,..2],
        typeMinKeyCode :      xproto::keycode,
        typeMaxKeyCode :      xproto::keycode,
        present :             u16,
        firstType :           u8,
        nTypes :              u8,
        totalTypes :          u8,
        firstKeySym :         xproto::keycode,
        totalSyms :           u16,
        nKeySyms :            u8,
        firstKeyAction :      xproto::keycode,
        totalActions :        u16,
        nKeyActions :         u8,
        firstKeyBehavior :    xproto::keycode,
        nKeyBehaviors :       u8,
        totalKeyBehaviors :   u8,
        firstKeyExplicit :    xproto::keycode,
        nKeyExplicit :        u8,
        totalKeyExplicit :    u8,
        firstModMapKey :      xproto::keycode,
        nModMapKeys :         u8,
        totalModMapKeys :     u8,
        firstVModMapKey :     xproto::keycode,
        nVModMapKeys :        u8,
        totalVModMapKeys :    u8,
        pad1 :                u8,
        virtualMods :         u16,
        map :                 get_kbd_by_name_replies_types_map,
    }
    compat_map : struct _compat_map {
        compatDeviceID :      u8,
        groupsRtrn :          u8,
        pad0 :                u8,
        firstSIRtrn :         u16,
        nSIRtrn :             u16,
        nTotalSI :            u16,
        pad1 :                [u8,..16],
        si_rtrn :                 *u8,
        group_rtrn :            *mod_def,
    }
    client_symbols : struct _client_symbols {
        clientDeviceID :      u8,
        pad0 :                [u8,..2],
        clientMinKeyCode :    xproto::keycode,
        clientMaxKeyCode :    xproto::keycode,
        present :             u16,
        firstType :           u8,
        nTypes :              u8,
        totalTypes :          u8,
        firstKeySym :         xproto::keycode,
        totalSyms :           u16,
        nKeySyms :            u8,
        firstKeyAction :      xproto::keycode,
        totalActions :        u16,
        nKeyActions :         u8,
        firstKeyBehavior :    xproto::keycode,
        nKeyBehaviors :       u8,
        totalKeyBehaviors :   u8,
        firstKeyExplicit :    xproto::keycode,
        nKeyExplicit :        u8,
        totalKeyExplicit :    u8,
        firstModMapKey :      xproto::keycode,
        nModMapKeys :         u8,
        totalModMapKeys :     u8,
        firstVModMapKey :     xproto::keycode,
        nVModMapKeys :        u8,
        totalVModMapKeys :    u8,
        pad1 :                u8,
        virtualMods :         u16,
        map :                 get_kbd_by_name_replies_client_symbols_map,
    }
    server_symbols : struct _server_symbols {
        serverDeviceID :      u8,
        pad0 :                [u8,..2],
        serverMinKeyCode :    xproto::keycode,
        serverMaxKeyCode :    xproto::keycode,
        present :             u16,
        firstType :           u8,
        nTypes :              u8,
        totalTypes :          u8,
        firstKeySym :         xproto::keycode,
        totalSyms :           u16,
        nKeySyms :            u8,
        firstKeyAction :      xproto::keycode,
        totalActions :        u16,
        nKeyActions :         u8,
        firstKeyBehavior :    xproto::keycode,
        nKeyBehaviors :       u8,
        totalKeyBehaviors :   u8,
        firstKeyExplicit :    xproto::keycode,
        nKeyExplicit :        u8,
        totalKeyExplicit :    u8,
        firstModMapKey :      xproto::keycode,
        nModMapKeys :         u8,
        totalModMapKeys :     u8,
        firstVModMapKey :     xproto::keycode,
        nVModMapKeys :        u8,
        totalVModMapKeys :    u8,
        pad1 :                u8,
        virtualMods :         u16,
        map :                 get_kbd_by_name_replies_server_symbols_map,
    }
    indicator_maps : struct _indicator_maps {
        indicatorDeviceID :   u8,
        which :               u32,
        realIndicators :      u32,
        nIndicators :         u8,
        pad0 :                [u8,..15],
        maps :      *indicator_map,
    }
    key_names : struct _key_names {
        keyDeviceID :         u8,
        which :               u32,
        keyMinKeyCode :       xproto::keycode,
        keyMaxKeyCode :       xproto::keycode,
        nTypes :              u8,
        groupNames :          u8,
        virtualMods :         u16,
        firstKey :            xproto::keycode,
        nKeys :               u8,
        indicators :          u32,
        nRadioGroups :        u8,
        nKeyAliases :         u8,
        nKTLevels :           u16,
        pad0 :                [u8,..4],
        valueList :           get_kbd_by_name_replies_key_names_value_list,
    }
    other_names : struct _other_names {
        otherDeviceID :       u8,
        which :               u32,
        otherMinKeyCode :     xproto::keycode,
        otherMaxKeyCode :     xproto::keycode,
        nTypes :              u8,
        groupNames :          u8,
        virtualMods :         u16,
        firstKey :            xproto::keycode,
        nKeys :               u8,
        indicators :          u32,
        nRadioGroups :        u8,
        nKeyAliases :         u8,
        nKTLevels :           u16,
        pad0 :                [u8,..4],
        valueList :           get_kbd_by_name_replies_other_names_value_list,
    }
    geometry : struct _geometry {
        geometryDeviceID :    u8,
        name :                xproto::atom,
        geometryFound :       u8,
        pad0 :                u8,
        widthMM :             u16,
        heightMM :            u16,
        nProperties :         u16,
        nColors :             u16,
        nShapes :             u16,
        nSections :           u16,
        nDoodads :            u16,
        nKeyAliases :         u16,
        baseColorNdx :        u8,
        labelColorNdx :       u8,
        labelFont :   *counted_string_16,
        properties :           *property,
        colors :   *counted_string_16,
        shapes :              *shape,
        sections :            *section,
        doodads :             *doodad,
        keyAliases :          *key_alias,
    }
}

pub struct get_kbd_by_name_reply {
    response_type :   u8,
    deviceID :        u8,
    sequence :        u16,
    length :          u32,
    minKeyCode :      xproto::keycode,
    maxKeyCode :      xproto::keycode,
    loaded :          u8,
    newKeyboard :     u8,
    found :           u16,
    reported :        u16,
    pad0 :            [u8,..16]
}

pub struct get_device_info_cookie {
    sequence : c_uint
}

/** Opcode for xcb_xkb_get_device_info. */
pub static XCB_XKB_GET_DEVICE_INFO : c_int = 24;

pub struct get_device_info_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    deviceSpec :     device_spec,
    wanted :         u16,
    allButtons :     u8,
    firstButton :    u8,
    nButtons :       u8,
    pad0 :           u8,
    ledClass :       led_class_spec,
    ledID :          id_spec
}

pub struct get_device_info_reply {
    response_type :    u8,
    deviceID :         u8,
    sequence :         u16,
    length :           u32,
    present :          u16,
    supported :        u16,
    unsupported :      u16,
    nDeviceLedFBs :    u16,
    firstBtnWanted :   u8,
    nBtnsWanted :      u8,
    firstBtnRtrn :     u8,
    nBtnsRtrn :        u8,
    totalBtns :        u8,
    hasOwnState :      u8,
    dfltKbdFB :        u16,
    dfltLedFB :        u16,
    pad0 :             [u8,..2],
    devType :          xproto::atom,
    nameLen :          u16
}

/** Opcode for xcb_xkb_set_device_info. */
pub static XCB_XKB_SET_DEVICE_INFO : c_int = 25;

pub struct set_device_info_request {
    major_opcode :    u8,
    minor_opcode :    u8,
    length :          u16,
    deviceSpec :      device_spec,
    firstBtn :        u8,
    nBtns :           u8,
    change :          u16,
    nDeviceLedFBs :   u16
}

pub struct set_debugging_flags_cookie {
    sequence : c_uint
}

/** Opcode for xcb_xkb_set_debugging_flags. */
pub static XCB_XKB_SET_DEBUGGING_FLAGS : c_int = 101;

pub struct set_debugging_flags_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    msgLength :      u16,
    pad0 :           [u8,..2],
    affectFlags :    u32,
    flags :          u32,
    affectCtrls :    u32,
    ctrls :          u32
}

pub struct set_debugging_flags_reply {
    response_type :    u8,
    pad0 :             u8,
    sequence :         u16,
    length :           u32,
    currentFlags :     u32,
    currentCtrls :     u32,
    supportedFlags :   u32,
    supportedCtrls :   u32,
    pad1 :             [u8,..8]
}

/** Opcode for xcb_xkb_new_keyboard_notify. */
pub static XCB_XKB_NEW_KEYBOARD_NOTIFY : c_int = 0;

pub struct new_keyboard_notify_event {
    response_type :   u8,
    xkbType :         u8,
    sequence :        u16,
    time :            xproto::timestamp,
    deviceID :        u8,
    oldDeviceID :     u8,
    minKeyCode :      xproto::keycode,
    maxKeyCode :      xproto::keycode,
    oldMinKeyCode :   xproto::keycode,
    oldMaxKeyCode :   xproto::keycode,
    requestMajor :    u8,
    requestMinor :    u8,
    changed :         u16,
    pad0 :            [u8,..14]
}

/** Opcode for xcb_xkb_map_notify. */
pub static XCB_XKB_MAP_NOTIFY : c_int = 1;

pub struct map_notify_event {
    response_type :      u8,
    xkbType :            u8,
    sequence :           u16,
    time :               xproto::timestamp,
    deviceID :           u8,
    ptrBtnActions :      u8,
    changed :            u16,
    minKeyCode :         xproto::keycode,
    maxKeyCode :         xproto::keycode,
    firstType :          u8,
    nTypes :             u8,
    firstKeySym :        xproto::keycode,
    nKeySyms :           u8,
    firstKeyAct :        xproto::keycode,
    nKeyActs :           u8,
    firstKeyBehavior :   xproto::keycode,
    nKeyBehavior :       u8,
    firstKeyExplicit :   xproto::keycode,
    nKeyExplicit :       u8,
    firstModMapKey :     xproto::keycode,
    nModMapKeys :        u8,
    firstVModMapKey :    xproto::keycode,
    nVModMapKeys :       u8,
    virtualMods :        u16,
    pad0 :               [u8,..2]
}

/** Opcode for xcb_xkb_state_notify. */
pub static XCB_XKB_STATE_NOTIFY : c_int = 2;

pub struct state_notify_event {
    response_type :       u8,
    xkbType :             u8,
    sequence :            u16,
    time :                xproto::timestamp,
    deviceID :            u8,
    mods :                u8,
    baseMods :            u8,
    latchedMods :         u8,
    lockedMods :          u8,
    group :               u8,
    baseGroup :           i16,
    latchedGroup :        i16,
    lockedGroup :         u8,
    compatState :         u8,
    grabMods :            u8,
    compatGrabMods :      u8,
    lookupMods :          u8,
    compatLoockupMods :   u8,
    ptrBtnState :         u16,
    changed :             u16,
    keycode :             xproto::keycode,
    eventType :           u8,
    requestMajor :        u8,
    requestMinor :        u8
}

/** Opcode for xcb_xkb_controls_notify. */
pub static XCB_XKB_CONTROLS_NOTIFY : c_int = 3;

pub struct controls_notify_event {
    response_type :           u8,
    xkbType :                 u8,
    sequence :                u16,
    time :                    xproto::timestamp,
    deviceID :                u8,
    numGroups :               u8,
    pad0 :                    [u8,..2],
    changedControls :         u32,
    enabledControls :         u32,
    enabledControlChanges :   u32,
    keycode :                 xproto::keycode,
    eventType :               u8,
    requestMajor :            u8,
    requestMinor :            u8,
    pad1 :                    [u8,..4]
}

/** Opcode for xcb_xkb_indicator_state_notify. */
pub static XCB_XKB_INDICATOR_STATE_NOTIFY : c_int = 4;

pub struct indicator_state_notify_event {
    response_type :   u8,
    xkbType :         u8,
    sequence :        u16,
    time :            xproto::timestamp,
    deviceID :        u8,
    pad0 :            [u8,..3],
    state :           u32,
    stateChanged :    u32,
    pad1 :            [u8,..12]
}

/** Opcode for xcb_xkb_indicator_map_notify. */
pub static XCB_XKB_INDICATOR_MAP_NOTIFY : c_int = 5;

pub struct indicator_map_notify_event {
    response_type :   u8,
    xkbType :         u8,
    sequence :        u16,
    time :            xproto::timestamp,
    deviceID :        u8,
    pad0 :            [u8,..3],
    state :           u32,
    mapChanged :      u32,
    pad1 :            [u8,..12]
}

/** Opcode for xcb_xkb_names_notify. */
pub static XCB_XKB_NAMES_NOTIFY : c_int = 6;

pub struct names_notify_event {
    response_type :        u8,
    xkbType :              u8,
    sequence :             u16,
    time :                 xproto::timestamp,
    deviceID :             u8,
    pad0 :                 u8,
    changed :              u16,
    firstType :            u8,
    nTypes :               u8,
    firstLevelName :       u8,
    nLevelNames :          u8,
    pad1 :                 u8,
    nRadioGroups :         u8,
    nKeyAliases :          u8,
    changedGroupNames :    u8,
    changedVirtualMods :   u16,
    firstKey :             xproto::keycode,
    nKeys :                u8,
    changedIndicators :    u32,
    pad2 :                 [u8,..4]
}

/** Opcode for xcb_xkb_compat_map_notify. */
pub static XCB_XKB_COMPAT_MAP_NOTIFY : c_int = 7;

pub struct compat_map_notify_event {
    response_type :   u8,
    xkbType :         u8,
    sequence :        u16,
    time :            xproto::timestamp,
    deviceID :        u8,
    changedGroups :   u8,
    firstSI :         u16,
    nSI :             u16,
    nTotalSI :        u16,
    pad0 :            [u8,..16]
}

/** Opcode for xcb_xkb_bell_notify. */
pub static XCB_XKB_BELL_NOTIFY : c_int = 8;

pub struct bell_notify_event {
    response_type :   u8,
    xkbType :         u8,
    sequence :        u16,
    time :            xproto::timestamp,
    deviceID :        u8,
    bellClass :       u8,
    bellID :          u8,
    percent :         u8,
    pitch :           u16,
    duration :        u16,
    name :            xproto::atom,
    window :          xproto::window,
    eventOnly :       u8,
    pad0 :            [u8,..7]
}

/** Opcode for xcb_xkb_action_message. */
pub static XCB_XKB_ACTION_MESSAGE : c_int = 9;

pub struct action_message_event {
    response_type :     u8,
    xkbType :           u8,
    sequence :          u16,
    time :              xproto::timestamp,
    deviceID :          u8,
    keycode :           xproto::keycode,
    press :             u8,
    keyEventFollows :   u8,
    mods :              u8,
    group :             u8,
    message :           [string8,..8],
    pad0 :              [u8,..10]
}

/** Opcode for xcb_xkb_access_x_notify. */
pub static XCB_XKB_ACCESS_X_NOTIFY : c_int = 10;

pub struct access_x_notify_event {
    response_type :   u8,
    xkbType :         u8,
    sequence :        u16,
    time :            xproto::timestamp,
    deviceID :        u8,
    keycode :         xproto::keycode,
    detailt :         u16,
    slowKeysDelay :   u16,
    debounceDelay :   u16,
    pad0 :            [u8,..16]
}

/** Opcode for xcb_xkb_extension_device_notify. */
pub static XCB_XKB_EXTENSION_DEVICE_NOTIFY : c_int = 11;

pub struct extension_device_notify_event {
    response_type :   u8,
    xkbType :         u8,
    sequence :        u16,
    time :            xproto::timestamp,
    deviceID :        u8,
    pad0 :            u8,
    reason :          u16,
    ledClass :        u16,
    ledID :           u8,
    ledsDefined :     u32,
    ledState :        u32,
    firstButton :     u8,
    nButtons :        u8,
    supported :       u16,
    unsupported :     u16,
    pad1 :            [u8,..2]
}
#[link_args="-lxcb-xkb"]
extern "C" {

/**
 * Get the next element of the iterator
 * @param i Pointer to a ax_option_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(ax_option)
 *
 *
 */
unsafe fn xcb_xkb_ax_option_next (i:*ax_option_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An ax_option_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xkb_ax_option_end (i:ax_option_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a device_spec_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(device_spec)
 *
 *
 */
unsafe fn xcb_xkb_device_spec_next (i:*device_spec_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An device_spec_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xkb_device_spec_end (i:device_spec_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a led_class_spec_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(led_class_spec)
 *
 *
 */
unsafe fn xcb_xkb_led_class_spec_next (i:*led_class_spec_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An led_class_spec_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xkb_led_class_spec_end (i:led_class_spec_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a bell_class_spec_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(bell_class_spec)
 *
 *
 */
unsafe fn xcb_xkb_bell_class_spec_next (i:*bell_class_spec_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An bell_class_spec_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xkb_bell_class_spec_end (i:bell_class_spec_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a id_spec_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(id_spec)
 *
 *
 */
unsafe fn xcb_xkb_id_spec_next (i:*id_spec_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An id_spec_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xkb_id_spec_end (i:id_spec_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a indicator_map_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(indicator_map)
 *
 *
 */
unsafe fn xcb_xkb_indicator_map_next (i:*indicator_map_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An indicator_map_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xkb_indicator_map_end (i:indicator_map_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a mod_def_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(mod_def)
 *
 *
 */
unsafe fn xcb_xkb_mod_def_next (i:*mod_def_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An mod_def_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xkb_mod_def_end (i:mod_def_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a key_name_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(key_name)
 *
 *
 */
unsafe fn xcb_xkb_key_name_next (i:*key_name_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An key_name_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xkb_key_name_end (i:key_name_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a key_alias_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(key_alias)
 *
 *
 */
unsafe fn xcb_xkb_key_alias_next (i:*key_alias_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An key_alias_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xkb_key_alias_end (i:key_alias_iterator) -> generic_iterator;

unsafe fn xcb_xkb_counted_string_8_sizeof (_buffer :  *c_void) -> c_int;

unsafe fn xcb_xkb_counted_string_8_string (R : *counted_string_8) -> *u8;


unsafe fn xcb_xkb_counted_string_8_string_length (R : *counted_string_8) -> c_int;


unsafe fn xcb_xkb_counted_string_8_string_end (R : *counted_string_8) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a counted_string_8_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(counted_string_8)
 *
 *
 */
unsafe fn xcb_xkb_counted_string_8_next (i:*counted_string_8_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An counted_string_8_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xkb_counted_string_8_end (i:counted_string_8_iterator) -> generic_iterator;

unsafe fn xcb_xkb_counted_string_16_sizeof (_buffer :  *c_void) -> c_int;

unsafe fn xcb_xkb_counted_string_16_string (R : *counted_string_16) -> *u8;


unsafe fn xcb_xkb_counted_string_16_string_length (R : *counted_string_16) -> c_int;


unsafe fn xcb_xkb_counted_string_16_string_end (R : *counted_string_16) -> generic_iterator;


/**
 *
 * xcb_xkb_counted_string_16_pad_0 : *u8
 * 
 *
 */
unsafe fn xcb_xkb_counted_string_16_pad_0 (R : *counted_string_16) -> *u8;

/**
 * Get the next element of the iterator
 * @param i Pointer to a counted_string_16_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(counted_string_16)
 *
 *
 */
unsafe fn xcb_xkb_counted_string_16_next (i:*counted_string_16_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An counted_string_16_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xkb_counted_string_16_end (i:counted_string_16_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a kt_map_entry_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(kt_map_entry)
 *
 *
 */
unsafe fn xcb_xkb_kt_map_entry_next (i:*kt_map_entry_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An kt_map_entry_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xkb_kt_map_entry_end (i:kt_map_entry_iterator) -> generic_iterator;

unsafe fn xcb_xkb_key_type_sizeof (_buffer :  *c_void) -> c_int;

unsafe fn xcb_xkb_key_type_map (R : *key_type) -> *kt_map_entry;


unsafe fn xcb_xkb_key_type_map_length (R : *key_type) -> c_int;

unsafe fn xcb_xkb_key_type_map_iterator (R : *key_type) -> kt_map_entry_iterator;

unsafe fn xcb_xkb_key_type_preserve (R : *key_type) -> *mod_def;


unsafe fn xcb_xkb_key_type_preserve_length (R : *key_type) -> c_int;

unsafe fn xcb_xkb_key_type_preserve_iterator (R : *key_type) -> mod_def_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a key_type_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(key_type)
 *
 *
 */
unsafe fn xcb_xkb_key_type_next (i:*key_type_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An key_type_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xkb_key_type_end (i:key_type_iterator) -> generic_iterator;

unsafe fn xcb_xkb_key_sym_map_sizeof (_buffer :  *c_void) -> c_int;

unsafe fn xcb_xkb_key_sym_map_syms (R : *key_sym_map) -> *xproto::keysym;


unsafe fn xcb_xkb_key_sym_map_syms_length (R : *key_sym_map) -> c_int;


unsafe fn xcb_xkb_key_sym_map_syms_end (R : *key_sym_map) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a key_sym_map_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(key_sym_map)
 *
 *
 */
unsafe fn xcb_xkb_key_sym_map_next (i:*key_sym_map_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An key_sym_map_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xkb_key_sym_map_end (i:key_sym_map_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a common_behavior_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(common_behavior)
 *
 *
 */
unsafe fn xcb_xkb_common_behavior_next (i:*common_behavior_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An common_behavior_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xkb_common_behavior_end (i:common_behavior_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a default_behavior_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(default_behavior)
 *
 *
 */
unsafe fn xcb_xkb_default_behavior_next (i:*default_behavior_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An default_behavior_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xkb_default_behavior_end (i:default_behavior_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a lock_behavior_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(lock_behavior)
 *
 *
 */
unsafe fn xcb_xkb_lock_behavior_next (i:*lock_behavior_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An lock_behavior_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xkb_lock_behavior_end (i:lock_behavior_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a radio_group_behavior_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(radio_group_behavior)
 *
 *
 */
unsafe fn xcb_xkb_radio_group_behavior_next (i:*radio_group_behavior_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An radio_group_behavior_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xkb_radio_group_behavior_end (i:radio_group_behavior_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a overlay_1_behavior_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(overlay_1_behavior)
 *
 *
 */
unsafe fn xcb_xkb_overlay_1_behavior_next (i:*overlay_1_behavior_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An overlay_1_behavior_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xkb_overlay_1_behavior_end (i:overlay_1_behavior_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a overlay_2_behavior_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(overlay_2_behavior)
 *
 *
 */
unsafe fn xcb_xkb_overlay_2_behavior_next (i:*overlay_2_behavior_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An overlay_2_behavior_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xkb_overlay_2_behavior_end (i:overlay_2_behavior_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a permament_lock_behavior_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(permament_lock_behavior)
 *
 *
 */
unsafe fn xcb_xkb_permament_lock_behavior_next (i:*permament_lock_behavior_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An permament_lock_behavior_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xkb_permament_lock_behavior_end (i:permament_lock_behavior_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a permament_radio_group_behavior_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(permament_radio_group_behavior)
 *
 *
 */
unsafe fn xcb_xkb_permament_radio_group_behavior_next (i:*permament_radio_group_behavior_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An permament_radio_group_behavior_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xkb_permament_radio_group_behavior_end (i:permament_radio_group_behavior_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a permament_overlay_1_behavior_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(permament_overlay_1_behavior)
 *
 *
 */
unsafe fn xcb_xkb_permament_overlay_1_behavior_next (i:*permament_overlay_1_behavior_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An permament_overlay_1_behavior_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xkb_permament_overlay_1_behavior_end (i:permament_overlay_1_behavior_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a permament_overlay_2_behavior_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(permament_overlay_2_behavior)
 *
 *
 */
unsafe fn xcb_xkb_permament_overlay_2_behavior_next (i:*permament_overlay_2_behavior_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An permament_overlay_2_behavior_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xkb_permament_overlay_2_behavior_end (i:permament_overlay_2_behavior_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a behavior_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(behavior)
 *
 *
 */
unsafe fn xcb_xkb_behavior_next (i:*behavior_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An behavior_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xkb_behavior_end (i:behavior_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a set_behavior_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(set_behavior)
 *
 *
 */
unsafe fn xcb_xkb_set_behavior_next (i:*set_behavior_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An set_behavior_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xkb_set_behavior_end (i:set_behavior_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a set_explicit_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(set_explicit)
 *
 *
 */
unsafe fn xcb_xkb_set_explicit_next (i:*set_explicit_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An set_explicit_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xkb_set_explicit_end (i:set_explicit_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a key_mod_map_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(key_mod_map)
 *
 *
 */
unsafe fn xcb_xkb_key_mod_map_next (i:*key_mod_map_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An key_mod_map_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xkb_key_mod_map_end (i:key_mod_map_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a key_v_mod_map_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(key_v_mod_map)
 *
 *
 */
unsafe fn xcb_xkb_key_v_mod_map_next (i:*key_v_mod_map_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An key_v_mod_map_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xkb_key_v_mod_map_end (i:key_v_mod_map_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a kt_set_map_entry_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(kt_set_map_entry)
 *
 *
 */
unsafe fn xcb_xkb_kt_set_map_entry_next (i:*kt_set_map_entry_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An kt_set_map_entry_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xkb_kt_set_map_entry_end (i:kt_set_map_entry_iterator) -> generic_iterator;

unsafe fn xcb_xkb_set_key_type_sizeof (_buffer :  *c_void) -> c_int;

unsafe fn xcb_xkb_set_key_type_entries (R : *set_key_type) -> *kt_set_map_entry;


unsafe fn xcb_xkb_set_key_type_entries_length (R : *set_key_type) -> c_int;

unsafe fn xcb_xkb_set_key_type_entries_iterator (R : *set_key_type) -> kt_set_map_entry_iterator;

unsafe fn xcb_xkb_set_key_type_preserve_entries (R : *set_key_type) -> *kt_set_map_entry;


unsafe fn xcb_xkb_set_key_type_preserve_entries_length (R : *set_key_type) -> c_int;

unsafe fn xcb_xkb_set_key_type_preserve_entries_iterator (R : *set_key_type) -> kt_set_map_entry_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a set_key_type_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(set_key_type)
 *
 *
 */
unsafe fn xcb_xkb_set_key_type_next (i:*set_key_type_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An set_key_type_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xkb_set_key_type_end (i:set_key_type_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a string8_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(string8)
 *
 *
 */
unsafe fn xcb_xkb_string8_next (i:*string8_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An string8_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xkb_string8_end (i:string8_iterator) -> generic_iterator;

unsafe fn xcb_xkb_property_serialize (_buffer :  **c_void,
                            _aux :      *property,
                            name :      *string8,
                            value :     *string8) -> c_int;

unsafe fn xcb_xkb_property_unserialize (_buffer :    *c_void,
                              _aux :      **property) -> c_int;

unsafe fn xcb_xkb_property_sizeof (_buffer :  *c_void) -> c_int;

unsafe fn xcb_xkb_property_name (R : *property) -> *string8;


unsafe fn xcb_xkb_property_name_length (R : *property) -> c_int;


unsafe fn xcb_xkb_property_name_end (R : *property) -> generic_iterator;

unsafe fn xcb_xkb_property_value (R : *property) -> *string8;


unsafe fn xcb_xkb_property_value_length (R : *property) -> c_int;


unsafe fn xcb_xkb_property_value_end (R : *property) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a property_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(property)
 *
 *
 */
unsafe fn xcb_xkb_property_next (i:*property_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An property_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xkb_property_end (i:property_iterator) -> generic_iterator;

unsafe fn xcb_xkb_outline_sizeof (_buffer :  *c_void) -> c_int;

unsafe fn xcb_xkb_outline_points (R : *outline) -> *xproto::point;


unsafe fn xcb_xkb_outline_points_length (R : *outline) -> c_int;

unsafe fn xcb_xkb_outline_points_iterator (R : *outline) -> xproto::point_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a outline_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(outline)
 *
 *
 */
unsafe fn xcb_xkb_outline_next (i:*outline_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An outline_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xkb_outline_end (i:outline_iterator) -> generic_iterator;

unsafe fn xcb_xkb_shape_sizeof (_buffer :  *c_void) -> c_int;


unsafe fn xcb_xkb_shape_outlines_length (R : *shape) -> c_int;

unsafe fn xcb_xkb_shape_outlines_iterator (R : *shape) -> outline_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a shape_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(shape)
 *
 *
 */
unsafe fn xcb_xkb_shape_next (i:*shape_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An shape_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xkb_shape_end (i:shape_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a key_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(key)
 *
 *
 */
unsafe fn xcb_xkb_key_next (i:*key_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An key_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xkb_key_end (i:key_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a overlay_key_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(overlay_key)
 *
 *
 */
unsafe fn xcb_xkb_overlay_key_next (i:*overlay_key_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An overlay_key_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xkb_overlay_key_end (i:overlay_key_iterator) -> generic_iterator;

unsafe fn xcb_xkb_overlay_row_sizeof (_buffer :  *c_void) -> c_int;

unsafe fn xcb_xkb_overlay_row_keys (R : *overlay_row) -> *overlay_key;


unsafe fn xcb_xkb_overlay_row_keys_length (R : *overlay_row) -> c_int;

unsafe fn xcb_xkb_overlay_row_keys_iterator (R : *overlay_row) -> overlay_key_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a overlay_row_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(overlay_row)
 *
 *
 */
unsafe fn xcb_xkb_overlay_row_next (i:*overlay_row_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An overlay_row_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xkb_overlay_row_end (i:overlay_row_iterator) -> generic_iterator;

unsafe fn xcb_xkb_overlay_sizeof (_buffer :  *c_void) -> c_int;


unsafe fn xcb_xkb_overlay_rows_length (R : *overlay) -> c_int;

unsafe fn xcb_xkb_overlay_rows_iterator (R : *overlay) -> overlay_row_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a overlay_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(overlay)
 *
 *
 */
unsafe fn xcb_xkb_overlay_next (i:*overlay_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An overlay_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xkb_overlay_end (i:overlay_iterator) -> generic_iterator;

unsafe fn xcb_xkb_row_sizeof (_buffer :  *c_void) -> c_int;

unsafe fn xcb_xkb_row_keys (R : *row) -> *key;


unsafe fn xcb_xkb_row_keys_length (R : *row) -> c_int;

unsafe fn xcb_xkb_row_keys_iterator (R : *row) -> key_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a row_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(row)
 *
 *
 */
unsafe fn xcb_xkb_row_next (i:*row_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An row_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xkb_row_end (i:row_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a common_doodad_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(common_doodad)
 *
 *
 */
unsafe fn xcb_xkb_common_doodad_next (i:*common_doodad_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An common_doodad_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xkb_common_doodad_end (i:common_doodad_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a shape_doodad_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(shape_doodad)
 *
 *
 */
unsafe fn xcb_xkb_shape_doodad_next (i:*shape_doodad_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An shape_doodad_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xkb_shape_doodad_end (i:shape_doodad_iterator) -> generic_iterator;

unsafe fn xcb_xkb_text_doodad_sizeof (_buffer :  *c_void) -> c_int;


/**
 *
 * xcb_xkb_text_doodad_text : *counted_string_16
 * 
 *
 */
unsafe fn xcb_xkb_text_doodad_text (R : *text_doodad) -> *counted_string_16;


/**
 *
 * xcb_xkb_text_doodad_font : *counted_string_16
 * 
 *
 */
unsafe fn xcb_xkb_text_doodad_font (R : *text_doodad) -> *counted_string_16;

/**
 * Get the next element of the iterator
 * @param i Pointer to a text_doodad_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(text_doodad)
 *
 *
 */
unsafe fn xcb_xkb_text_doodad_next (i:*text_doodad_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An text_doodad_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xkb_text_doodad_end (i:text_doodad_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a indicator_doodad_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(indicator_doodad)
 *
 *
 */
unsafe fn xcb_xkb_indicator_doodad_next (i:*indicator_doodad_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An indicator_doodad_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xkb_indicator_doodad_end (i:indicator_doodad_iterator) -> generic_iterator;

unsafe fn xcb_xkb_logo_doodad_sizeof (_buffer :  *c_void) -> c_int;


/**
 *
 * xcb_xkb_logo_doodad_logo_name : *counted_string_16
 * 
 *
 */
unsafe fn xcb_xkb_logo_doodad_logo_name (R : *logo_doodad) -> *counted_string_16;

/**
 * Get the next element of the iterator
 * @param i Pointer to a logo_doodad_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(logo_doodad)
 *
 *
 */
unsafe fn xcb_xkb_logo_doodad_next (i:*logo_doodad_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An logo_doodad_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xkb_logo_doodad_end (i:logo_doodad_iterator) -> generic_iterator;

unsafe fn xcb_xkb_doodad_sizeof (_buffer :  *c_void) -> c_int;

/**
 * Get the next element of the iterator
 * @param i Pointer to a doodad_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(doodad)
 *
 *
 */
unsafe fn xcb_xkb_doodad_next (i:*doodad_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An doodad_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xkb_doodad_end (i:doodad_iterator) -> generic_iterator;

unsafe fn xcb_xkb_section_sizeof (_buffer :  *c_void) -> c_int;


unsafe fn xcb_xkb_section_rows_length (R : *section) -> c_int;

unsafe fn xcb_xkb_section_rows_iterator (R : *section) -> row_iterator;


unsafe fn xcb_xkb_section_doodads_length (R : *section) -> c_int;

unsafe fn xcb_xkb_section_doodads_iterator (R : *section) -> doodad_iterator;


unsafe fn xcb_xkb_section_overlays_length (R : *section) -> c_int;

unsafe fn xcb_xkb_section_overlays_iterator (R : *section) -> overlay_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a section_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(section)
 *
 *
 */
unsafe fn xcb_xkb_section_next (i:*section_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An section_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xkb_section_end (i:section_iterator) -> generic_iterator;

unsafe fn xcb_xkb_listing_sizeof (_buffer :  *c_void) -> c_int;

unsafe fn xcb_xkb_listing_string (R : *listing) -> *string8;


unsafe fn xcb_xkb_listing_string_length (R : *listing) -> c_int;


unsafe fn xcb_xkb_listing_string_end (R : *listing) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a listing_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(listing)
 *
 *
 */
unsafe fn xcb_xkb_listing_next (i:*listing_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An listing_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xkb_listing_end (i:listing_iterator) -> generic_iterator;

unsafe fn xcb_xkb_device_led_info_sizeof (_buffer :  *c_void) -> c_int;

unsafe fn xcb_xkb_device_led_info_names (R : *device_led_info) -> *xproto::atom;


unsafe fn xcb_xkb_device_led_info_names_length (R : *device_led_info) -> c_int;


unsafe fn xcb_xkb_device_led_info_names_end (R : *device_led_info) -> generic_iterator;

unsafe fn xcb_xkb_device_led_info_maps (R : *device_led_info) -> *indicator_map;


unsafe fn xcb_xkb_device_led_info_maps_length (R : *device_led_info) -> c_int;

unsafe fn xcb_xkb_device_led_info_maps_iterator (R : *device_led_info) -> indicator_map_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a device_led_info_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(device_led_info)
 *
 *
 */
unsafe fn xcb_xkb_device_led_info_next (i:*device_led_info_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An device_led_info_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xkb_device_led_info_end (i:device_led_info_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a sa_no_action_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(sa_no_action)
 *
 *
 */
unsafe fn xcb_xkb_sa_no_action_next (i:*sa_no_action_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An sa_no_action_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xkb_sa_no_action_end (i:sa_no_action_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a sa_set_mods_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(sa_set_mods)
 *
 *
 */
unsafe fn xcb_xkb_sa_set_mods_next (i:*sa_set_mods_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An sa_set_mods_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xkb_sa_set_mods_end (i:sa_set_mods_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a sa_latch_mods_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(sa_latch_mods)
 *
 *
 */
unsafe fn xcb_xkb_sa_latch_mods_next (i:*sa_latch_mods_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An sa_latch_mods_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xkb_sa_latch_mods_end (i:sa_latch_mods_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a sa_lock_mods_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(sa_lock_mods)
 *
 *
 */
unsafe fn xcb_xkb_sa_lock_mods_next (i:*sa_lock_mods_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An sa_lock_mods_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xkb_sa_lock_mods_end (i:sa_lock_mods_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a sa_set_group_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(sa_set_group)
 *
 *
 */
unsafe fn xcb_xkb_sa_set_group_next (i:*sa_set_group_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An sa_set_group_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xkb_sa_set_group_end (i:sa_set_group_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a sa_latch_group_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(sa_latch_group)
 *
 *
 */
unsafe fn xcb_xkb_sa_latch_group_next (i:*sa_latch_group_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An sa_latch_group_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xkb_sa_latch_group_end (i:sa_latch_group_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a sa_lock_group_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(sa_lock_group)
 *
 *
 */
unsafe fn xcb_xkb_sa_lock_group_next (i:*sa_lock_group_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An sa_lock_group_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xkb_sa_lock_group_end (i:sa_lock_group_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a sa_move_ptr_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(sa_move_ptr)
 *
 *
 */
unsafe fn xcb_xkb_sa_move_ptr_next (i:*sa_move_ptr_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An sa_move_ptr_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xkb_sa_move_ptr_end (i:sa_move_ptr_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a sa_ptr_btn_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(sa_ptr_btn)
 *
 *
 */
unsafe fn xcb_xkb_sa_ptr_btn_next (i:*sa_ptr_btn_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An sa_ptr_btn_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xkb_sa_ptr_btn_end (i:sa_ptr_btn_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a sa_lock_ptr_btn_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(sa_lock_ptr_btn)
 *
 *
 */
unsafe fn xcb_xkb_sa_lock_ptr_btn_next (i:*sa_lock_ptr_btn_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An sa_lock_ptr_btn_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xkb_sa_lock_ptr_btn_end (i:sa_lock_ptr_btn_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a sa_set_ptr_dflt_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(sa_set_ptr_dflt)
 *
 *
 */
unsafe fn xcb_xkb_sa_set_ptr_dflt_next (i:*sa_set_ptr_dflt_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An sa_set_ptr_dflt_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xkb_sa_set_ptr_dflt_end (i:sa_set_ptr_dflt_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a sa_iso_lock_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(sa_iso_lock)
 *
 *
 */
unsafe fn xcb_xkb_sa_iso_lock_next (i:*sa_iso_lock_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An sa_iso_lock_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xkb_sa_iso_lock_end (i:sa_iso_lock_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a sa_terminate_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(sa_terminate)
 *
 *
 */
unsafe fn xcb_xkb_sa_terminate_next (i:*sa_terminate_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An sa_terminate_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xkb_sa_terminate_end (i:sa_terminate_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a sa_switch_screen_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(sa_switch_screen)
 *
 *
 */
unsafe fn xcb_xkb_sa_switch_screen_next (i:*sa_switch_screen_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An sa_switch_screen_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xkb_sa_switch_screen_end (i:sa_switch_screen_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a sa_set_controls_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(sa_set_controls)
 *
 *
 */
unsafe fn xcb_xkb_sa_set_controls_next (i:*sa_set_controls_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An sa_set_controls_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xkb_sa_set_controls_end (i:sa_set_controls_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a sa_lock_controls_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(sa_lock_controls)
 *
 *
 */
unsafe fn xcb_xkb_sa_lock_controls_next (i:*sa_lock_controls_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An sa_lock_controls_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xkb_sa_lock_controls_end (i:sa_lock_controls_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a sa_action_message_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(sa_action_message)
 *
 *
 */
unsafe fn xcb_xkb_sa_action_message_next (i:*sa_action_message_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An sa_action_message_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xkb_sa_action_message_end (i:sa_action_message_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a sa_redirect_key_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(sa_redirect_key)
 *
 *
 */
unsafe fn xcb_xkb_sa_redirect_key_next (i:*sa_redirect_key_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An sa_redirect_key_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xkb_sa_redirect_key_end (i:sa_redirect_key_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a sa_device_btn_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(sa_device_btn)
 *
 *
 */
unsafe fn xcb_xkb_sa_device_btn_next (i:*sa_device_btn_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An sa_device_btn_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xkb_sa_device_btn_end (i:sa_device_btn_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a sa_lock_device_btn_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(sa_lock_device_btn)
 *
 *
 */
unsafe fn xcb_xkb_sa_lock_device_btn_next (i:*sa_lock_device_btn_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An sa_lock_device_btn_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xkb_sa_lock_device_btn_end (i:sa_lock_device_btn_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a sa_device_valuator_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(sa_device_valuator)
 *
 *
 */
unsafe fn xcb_xkb_sa_device_valuator_next (i:*sa_device_valuator_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An sa_device_valuator_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xkb_sa_device_valuator_end (i:sa_device_valuator_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a action_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(action)
 *
 *
 */
unsafe fn xcb_xkb_action_next (i:*action_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An action_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xkb_action_end (i:action_iterator) -> generic_iterator;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xkb_use_extension (c : *connection,
                                 wantedMajor :  u16,
                                 wantedMinor :  u16) -> use_extension_cookie;

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
unsafe fn xcb_xkb_use_extension_unchecked (c : *connection,
                                           wantedMajor :  u16,
                                           wantedMinor :  u16) -> use_extension_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xkb_use_extension_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_xkb_use_extension_reply (c : *connection,
                                       cookie : use_extension_cookie,
                                       e : **generic_error) -> *use_extension_reply;

unsafe fn xcb_xkb_select_events_details_serialize (_buffer :               **c_void,
                                         affectWhich :             u16,
                                         clear :                   u16,
                                         selectAll :               u16,
                                         _aux :                   *select_events_details) -> c_int;

unsafe fn xcb_xkb_select_events_details_unpack (_buffer :                *c_void,
                                      affectWhich :             u16,
                                      clear :                   u16,
                                      selectAll :               u16,
                                      _aux :                   *select_events_details) -> c_int;

unsafe fn xcb_xkb_select_events_details_sizeof (_buffer :  *c_void,
                                      affectWhich :   u16,
                                      clear :    u16,
                                      selectAll :   u16) -> c_int;

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
unsafe fn xcb_xkb_select_events_checked (c : *connection,
                                         deviceSpec :  device_spec,
                                         affectWhich :  u16,
                                         clear :  u16,
                                         selectAll :  u16,
                                         affectMap :  u16,
                                         map :  u16,
                                         details : *()) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xkb_select_events (c : *connection,
                                 deviceSpec :  device_spec,
                                 affectWhich :  u16,
                                 clear :  u16,
                                 selectAll :  u16,
                                 affectMap :  u16,
                                 map :  u16,
                                 details : *()) -> void_cookie;

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
unsafe fn xcb_xkb_select_events_aux_checked (c : *connection,
                                             deviceSpec :  device_spec,
                                             affectWhich :  u16,
                                             clear :  u16,
                                             selectAll :  u16,
                                             affectMap :  u16,
                                             map :  u16,
                                             details : *select_events_details) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xkb_select_events_aux (c : *connection,
                                     deviceSpec :  device_spec,
                                     affectWhich :  u16,
                                     clear :  u16,
                                     selectAll :  u16,
                                     affectMap :  u16,
                                     map :  u16,
                                     details : *select_events_details) -> void_cookie;

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
unsafe fn xcb_xkb_bell_checked (c : *connection,
                                deviceSpec :  device_spec,
                                bellClass :  bell_class_spec,
                                bellID :  id_spec,
                                percent :  i8,
                                forceSound :  u8,
                                eventOnly :  u8,
                                pitch :  i16,
                                duration :  i16,
                                name :  xproto::atom,
                                window :  xproto::window) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xkb_bell (c : *connection,
                        deviceSpec :  device_spec,
                        bellClass :  bell_class_spec,
                        bellID :  id_spec,
                        percent :  i8,
                        forceSound :  u8,
                        eventOnly :  u8,
                        pitch :  i16,
                        duration :  i16,
                        name :  xproto::atom,
                        window :  xproto::window) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xkb_get_state (c : *connection,
                             deviceSpec :  device_spec) -> get_state_cookie;

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
unsafe fn xcb_xkb_get_state_unchecked (c : *connection,
                                       deviceSpec :  device_spec) -> get_state_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xkb_get_state_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_xkb_get_state_reply (c : *connection,
                                   cookie : get_state_cookie,
                                   e : **generic_error) -> *get_state_reply;

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
unsafe fn xcb_xkb_latch_lock_state_checked (c : *connection,
                                            deviceSpec :  device_spec,
                                            affectModLocks :  u8,
                                            modLocks :  u8,
                                            lockGroup :  u8,
                                            groupLock :  u8,
                                            affectModLatches :  u8,
                                            latchGroup :  u8,
                                            groupLatch :  u16) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xkb_latch_lock_state (c : *connection,
                                    deviceSpec :  device_spec,
                                    affectModLocks :  u8,
                                    modLocks :  u8,
                                    lockGroup :  u8,
                                    groupLock :  u8,
                                    affectModLatches :  u8,
                                    latchGroup :  u8,
                                    groupLatch :  u16) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xkb_get_controls (c : *connection,
                                deviceSpec :  device_spec) -> get_controls_cookie;

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
unsafe fn xcb_xkb_get_controls_unchecked (c : *connection,
                                          deviceSpec :  device_spec) -> get_controls_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xkb_get_controls_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_xkb_get_controls_reply (c : *connection,
                                      cookie : get_controls_cookie,
                                      e : **generic_error) -> *get_controls_reply;

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
unsafe fn xcb_xkb_set_controls_checked (c : *connection,
                                        deviceSpec :  device_spec,
                                        affectInternalRealMods :  u8,
                                        internalRealMods :  u8,
                                        affectIgnoreLockRealMods :  u8,
                                        ignoreLockRealMods :  u8,
                                        affectInternalVirtualMods :  u16,
                                        internalVirtualMods :  u16,
                                        affectIgnoreLockVirtualMods :  u16,
                                        ignoreLockVirtualMods :  u16,
                                        mouseKeysDfltBtn :  u8,
                                        groupsWrap :  u8,
                                        accessXOptions :  ax_option,
                                        affectEnabledControls :  u32,
                                        enabledControls :  u32,
                                        changeControls :  u32,
                                        repeatDelay :  u16,
                                        repeatInterval :  u16,
                                        slowKeysDelay :  u16,
                                        debounceDelay :  u16,
                                        mouseKeysDelay :  u16,
                                        mouseKeysInterval :  u16,
                                        mouseKeysTimeToMax :  u16,
                                        mouseKeysMaxSpeed :  u16,
                                        mouseKeysCurve :  i16,
                                        accessXTimeout :  u16,
                                        accessXTimeoutMask :  u32,
                                        accessXTimeoutValues :  u32,
                                        accessXTimeoutOptionsMask :  ax_option,
                                        accessXTimeoutOptionsValues :  ax_option,
                                        perKeyRepeat : *u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xkb_set_controls (c : *connection,
                                deviceSpec :  device_spec,
                                affectInternalRealMods :  u8,
                                internalRealMods :  u8,
                                affectIgnoreLockRealMods :  u8,
                                ignoreLockRealMods :  u8,
                                affectInternalVirtualMods :  u16,
                                internalVirtualMods :  u16,
                                affectIgnoreLockVirtualMods :  u16,
                                ignoreLockVirtualMods :  u16,
                                mouseKeysDfltBtn :  u8,
                                groupsWrap :  u8,
                                accessXOptions :  ax_option,
                                affectEnabledControls :  u32,
                                enabledControls :  u32,
                                changeControls :  u32,
                                repeatDelay :  u16,
                                repeatInterval :  u16,
                                slowKeysDelay :  u16,
                                debounceDelay :  u16,
                                mouseKeysDelay :  u16,
                                mouseKeysInterval :  u16,
                                mouseKeysTimeToMax :  u16,
                                mouseKeysMaxSpeed :  u16,
                                mouseKeysCurve :  i16,
                                accessXTimeout :  u16,
                                accessXTimeoutMask :  u32,
                                accessXTimeoutValues :  u32,
                                accessXTimeoutOptionsMask :  ax_option,
                                accessXTimeoutOptionsValues :  ax_option,
                                perKeyRepeat : *u8) -> void_cookie;


unsafe fn xcb_xkb_get_map_map_types_rtrn_length (R : *get_map_reply,
                                            S : *get_map_map) -> c_int;

unsafe fn xcb_xkb_get_map_map_types_rtrn_iterator (R : get_map_reply,
                                         S : *get_map_map /**< */) -> key_type_iterator;


unsafe fn xcb_xkb_get_map_map_syms_rtrn_length (R : *get_map_reply,
                                           S : *get_map_map) -> c_int;

unsafe fn xcb_xkb_get_map_map_syms_rtrn_iterator (R : get_map_reply,
                                        S : *get_map_map /**< */) -> key_sym_map_iterator;

unsafe fn xcb_xkb_get_map_map_acts_rtrn_count (S : *get_map_map) -> *u8;


unsafe fn xcb_xkb_get_map_map_acts_rtrn_count_length (R : *get_map_reply,
                                                 S : *get_map_map) -> c_int;


unsafe fn xcb_xkb_get_map_map_acts_rtrn_count_end (R : get_map_reply,
                                         S : *get_map_map ) -> generic_iterator;

unsafe fn xcb_xkb_get_map_map_acts_rtrn_acts (S : *get_map_map) -> *action;


unsafe fn xcb_xkb_get_map_map_acts_rtrn_acts_length (R : *get_map_reply,
                                                S : *get_map_map) -> c_int;

unsafe fn xcb_xkb_get_map_map_acts_rtrn_acts_iterator (R : get_map_reply,
                                             S : *get_map_map /**< */) -> action_iterator;

unsafe fn xcb_xkb_get_map_map_behaviors_rtrn (S : *get_map_map) -> *set_behavior;


unsafe fn xcb_xkb_get_map_map_behaviors_rtrn_length (R : *get_map_reply,
                                                S : *get_map_map) -> c_int;

unsafe fn xcb_xkb_get_map_map_behaviors_rtrn_iterator (R : get_map_reply,
                                             S : *get_map_map /**< */) -> set_behavior_iterator;

unsafe fn xcb_xkb_get_map_map_vmods_rtrn (S : *get_map_map) -> *u8;


unsafe fn xcb_xkb_get_map_map_vmods_rtrn_length (R : *get_map_reply,
                                            S : *get_map_map) -> c_int;


unsafe fn xcb_xkb_get_map_map_vmods_rtrn_end (R : get_map_reply,
                                    S : *get_map_map ) -> generic_iterator;

unsafe fn xcb_xkb_get_map_map_explicit_rtrn (S : *get_map_map) -> *set_explicit;


unsafe fn xcb_xkb_get_map_map_explicit_rtrn_length (R : *get_map_reply,
                                               S : *get_map_map) -> c_int;

unsafe fn xcb_xkb_get_map_map_explicit_rtrn_iterator (R : get_map_reply,
                                            S : *get_map_map /**< */) -> set_explicit_iterator;

unsafe fn xcb_xkb_get_map_map_modmap_rtrn (S : *get_map_map) -> *key_mod_map;


unsafe fn xcb_xkb_get_map_map_modmap_rtrn_length (R : *get_map_reply,
                                             S : *get_map_map) -> c_int;

unsafe fn xcb_xkb_get_map_map_modmap_rtrn_iterator (R : get_map_reply,
                                          S : *get_map_map /**< */) -> key_mod_map_iterator;

unsafe fn xcb_xkb_get_map_map_vmodmap_rtrn (S : *get_map_map) -> *key_v_mod_map;


unsafe fn xcb_xkb_get_map_map_vmodmap_rtrn_length (R : *get_map_reply,
                                              S : *get_map_map) -> c_int;

unsafe fn xcb_xkb_get_map_map_vmodmap_rtrn_iterator (R : get_map_reply,
                                           S : *get_map_map /**< */) -> key_v_mod_map_iterator;

unsafe fn xcb_xkb_get_map_map_serialize (_buffer :     **c_void,
                               nTypes :        u8,
                               nKeySyms :      u8,
                               nKeyActions :   u8,
                               totalActions :  u16,
                               totalKeyBehaviors :  u8,
                               nVModMapKeys :  u8,
                               totalKeyExplicit :  u8,
                               totalModMapKeys :  u8,
                               totalVModMapKeys :  u8,
                               present :       u16,
                               _aux :         *get_map_map) -> c_int;

unsafe fn xcb_xkb_get_map_map_unpack (_buffer :      *c_void,
                            nTypes :        u8,
                            nKeySyms :      u8,
                            nKeyActions :   u8,
                            totalActions :  u16,
                            totalKeyBehaviors :  u8,
                            nVModMapKeys :  u8,
                            totalKeyExplicit :  u8,
                            totalModMapKeys :  u8,
                            totalVModMapKeys :  u8,
                            present :       u16,
                            _aux :         *get_map_map) -> c_int;

unsafe fn xcb_xkb_get_map_map_sizeof (_buffer :  *c_void,
                            nTypes :   u8,
                            nKeySyms :  u8,
                            nKeyActions :  u8,
                            totalActions :  u16,
                            totalKeyBehaviors :  u8,
                            nVModMapKeys :  u8,
                            totalKeyExplicit :  u8,
                            totalModMapKeys :  u8,
                            totalVModMapKeys :  u8,
                            present :   u16) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xkb_get_map (c : *connection,
                           deviceSpec :  device_spec,
                           full :  u16,
                           partial :  u16,
                           firstType :  u8,
                           nTypes :  u8,
                           firstKeySym :  xproto::keycode,
                           nKeySyms :  u8,
                           firstKeyAction :  xproto::keycode,
                           nKeyActions :  u8,
                           firstKeyBehavior :  xproto::keycode,
                           nKeyBehaviors :  u8,
                           virtualMods :  u16,
                           firstKeyExplicit :  xproto::keycode,
                           nKeyExplicit :  u8,
                           firstModMapKey :  xproto::keycode,
                           nModMapKeys :  u8,
                           firstVModMapKey :  xproto::keycode,
                           nVModMapKeys :  u8) -> get_map_cookie;

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
unsafe fn xcb_xkb_get_map_unchecked (c : *connection,
                                     deviceSpec :  device_spec,
                                     full :  u16,
                                     partial :  u16,
                                     firstType :  u8,
                                     nTypes :  u8,
                                     firstKeySym :  xproto::keycode,
                                     nKeySyms :  u8,
                                     firstKeyAction :  xproto::keycode,
                                     nKeyActions :  u8,
                                     firstKeyBehavior :  xproto::keycode,
                                     nKeyBehaviors :  u8,
                                     virtualMods :  u16,
                                     firstKeyExplicit :  xproto::keycode,
                                     nKeyExplicit :  u8,
                                     firstModMapKey :  xproto::keycode,
                                     nModMapKeys :  u8,
                                     firstVModMapKey :  xproto::keycode,
                                     nVModMapKeys :  u8) -> get_map_cookie;


/**
 *
 * xcb_xkb_get_map_map : *get_map_map
 * 
 *
 */
unsafe fn xcb_xkb_get_map_map (R : *get_map_reply) -> *c_void;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xkb_get_map_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_xkb_get_map_reply (c : *connection,
                                 cookie : get_map_cookie,
                                 e : **generic_error) -> *get_map_reply;


unsafe fn xcb_xkb_set_map_values_types_length (R : *set_map_request,
                                          S : *set_map_values) -> c_int;

unsafe fn xcb_xkb_set_map_values_types_iterator (R : set_map_request,
                                       S : *set_map_values /**< */) -> set_key_type_iterator;


unsafe fn xcb_xkb_set_map_values_syms_length (R : *set_map_request,
                                         S : *set_map_values) -> c_int;

unsafe fn xcb_xkb_set_map_values_syms_iterator (R : set_map_request,
                                      S : *set_map_values /**< */) -> key_sym_map_iterator;

unsafe fn xcb_xkb_set_map_values_actions_count (S : *set_map_values) -> *u8;


unsafe fn xcb_xkb_set_map_values_actions_count_length (R : *set_map_request,
                                                  S : *set_map_values) -> c_int;


unsafe fn xcb_xkb_set_map_values_actions_count_end (R : set_map_request,
                                          S : *set_map_values ) -> generic_iterator;

unsafe fn xcb_xkb_set_map_values_actions (S : *set_map_values) -> *action;


unsafe fn xcb_xkb_set_map_values_actions_length (R : *set_map_request,
                                            S : *set_map_values) -> c_int;

unsafe fn xcb_xkb_set_map_values_actions_iterator (R : set_map_request,
                                         S : *set_map_values /**< */) -> action_iterator;

unsafe fn xcb_xkb_set_map_values_behaviors (S : *set_map_values) -> *set_behavior;


unsafe fn xcb_xkb_set_map_values_behaviors_length (R : *set_map_request,
                                              S : *set_map_values) -> c_int;

unsafe fn xcb_xkb_set_map_values_behaviors_iterator (R : set_map_request,
                                           S : *set_map_values /**< */) -> set_behavior_iterator;

unsafe fn xcb_xkb_set_map_values_vmods (S : *set_map_values) -> *u8;


unsafe fn xcb_xkb_set_map_values_vmods_length (R : *set_map_request,
                                          S : *set_map_values) -> c_int;


unsafe fn xcb_xkb_set_map_values_vmods_end (R : set_map_request,
                                  S : *set_map_values ) -> generic_iterator;

unsafe fn xcb_xkb_set_map_values_explicit (S : *set_map_values) -> *set_explicit;


unsafe fn xcb_xkb_set_map_values_explicit_length (R : *set_map_request,
                                             S : *set_map_values) -> c_int;

unsafe fn xcb_xkb_set_map_values_explicit_iterator (R : set_map_request,
                                          S : *set_map_values /**< */) -> set_explicit_iterator;

unsafe fn xcb_xkb_set_map_values_modmap (S : *set_map_values) -> *key_mod_map;


unsafe fn xcb_xkb_set_map_values_modmap_length (R : *set_map_request,
                                           S : *set_map_values) -> c_int;

unsafe fn xcb_xkb_set_map_values_modmap_iterator (R : set_map_request,
                                        S : *set_map_values /**< */) -> key_mod_map_iterator;

unsafe fn xcb_xkb_set_map_values_vmodmap (S : *set_map_values) -> *key_v_mod_map;


unsafe fn xcb_xkb_set_map_values_vmodmap_length (R : *set_map_request,
                                            S : *set_map_values) -> c_int;

unsafe fn xcb_xkb_set_map_values_vmodmap_iterator (R : set_map_request,
                                         S : *set_map_values /**< */) -> key_v_mod_map_iterator;

unsafe fn xcb_xkb_set_map_values_serialize (_buffer :        **c_void,
                                  nTypes :           u8,
                                  nKeySyms :         u8,
                                  nKeyActions :      u8,
                                  totalActions :     u16,
                                  totalKeyBehaviors :  u8,
                                  nVModMapKeys :     u8,
                                  totalKeyExplicit :  u8,
                                  totalModMapKeys :  u8,
                                  totalVModMapKeys :  u8,
                                  present :          u16,
                                  _aux :            *set_map_values) -> c_int;

unsafe fn xcb_xkb_set_map_values_unpack (_buffer :         *c_void,
                               nTypes :           u8,
                               nKeySyms :         u8,
                               nKeyActions :      u8,
                               totalActions :     u16,
                               totalKeyBehaviors :  u8,
                               nVModMapKeys :     u8,
                               totalKeyExplicit :  u8,
                               totalModMapKeys :  u8,
                               totalVModMapKeys :  u8,
                               present :          u16,
                               _aux :            *set_map_values) -> c_int;

unsafe fn xcb_xkb_set_map_values_sizeof (_buffer :  *c_void,
                               nTypes :   u8,
                               nKeySyms :  u8,
                               nKeyActions :  u8,
                               totalActions :  u16,
                               totalKeyBehaviors :  u8,
                               nVModMapKeys :  u8,
                               totalKeyExplicit :  u8,
                               totalModMapKeys :  u8,
                               totalVModMapKeys :  u8,
                               present :   u16) -> c_int;

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
unsafe fn xcb_xkb_set_map_checked (c : *connection,
                                   deviceSpec :  device_spec,
                                   present :  u16,
                                   flags :  u16,
                                   minKeyCode :  xproto::keycode,
                                   maxKeyCode :  xproto::keycode,
                                   firstType :  u8,
                                   nTypes :  u8,
                                   firstKeySym :  xproto::keycode,
                                   nKeySyms :  u8,
                                   totalSyms :  u16,
                                   firstKeyAction :  xproto::keycode,
                                   nKeyActions :  u8,
                                   totalActions :  u16,
                                   firstKeyBehavior :  xproto::keycode,
                                   nKeyBehaviors :  u8,
                                   totalKeyBehaviors :  u8,
                                   firstKeyExplicit :  xproto::keycode,
                                   nKeyExplicit :  u8,
                                   totalKeyExplicit :  u8,
                                   firstModMapKey :  xproto::keycode,
                                   nModMapKeys :  u8,
                                   totalModMapKeys :  u8,
                                   firstVModMapKey :  xproto::keycode,
                                   nVModMapKeys :  u8,
                                   totalVModMapKeys :  u8,
                                   virtualMods :  u16,
                                   values : *()) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xkb_set_map (c : *connection,
                           deviceSpec :  device_spec,
                           present :  u16,
                           flags :  u16,
                           minKeyCode :  xproto::keycode,
                           maxKeyCode :  xproto::keycode,
                           firstType :  u8,
                           nTypes :  u8,
                           firstKeySym :  xproto::keycode,
                           nKeySyms :  u8,
                           totalSyms :  u16,
                           firstKeyAction :  xproto::keycode,
                           nKeyActions :  u8,
                           totalActions :  u16,
                           firstKeyBehavior :  xproto::keycode,
                           nKeyBehaviors :  u8,
                           totalKeyBehaviors :  u8,
                           firstKeyExplicit :  xproto::keycode,
                           nKeyExplicit :  u8,
                           totalKeyExplicit :  u8,
                           firstModMapKey :  xproto::keycode,
                           nModMapKeys :  u8,
                           totalModMapKeys :  u8,
                           firstVModMapKey :  xproto::keycode,
                           nVModMapKeys :  u8,
                           totalVModMapKeys :  u8,
                           virtualMods :  u16,
                           values : *()) -> void_cookie;

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
unsafe fn xcb_xkb_set_map_aux_checked (c : *connection,
                                       deviceSpec :  device_spec,
                                       present :  u16,
                                       flags :  u16,
                                       minKeyCode :  xproto::keycode,
                                       maxKeyCode :  xproto::keycode,
                                       firstType :  u8,
                                       nTypes :  u8,
                                       firstKeySym :  xproto::keycode,
                                       nKeySyms :  u8,
                                       totalSyms :  u16,
                                       firstKeyAction :  xproto::keycode,
                                       nKeyActions :  u8,
                                       totalActions :  u16,
                                       firstKeyBehavior :  xproto::keycode,
                                       nKeyBehaviors :  u8,
                                       totalKeyBehaviors :  u8,
                                       firstKeyExplicit :  xproto::keycode,
                                       nKeyExplicit :  u8,
                                       totalKeyExplicit :  u8,
                                       firstModMapKey :  xproto::keycode,
                                       nModMapKeys :  u8,
                                       totalModMapKeys :  u8,
                                       firstVModMapKey :  xproto::keycode,
                                       nVModMapKeys :  u8,
                                       totalVModMapKeys :  u8,
                                       virtualMods :  u16,
                                       values : *set_map_values) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xkb_set_map_aux (c : *connection,
                               deviceSpec :  device_spec,
                               present :  u16,
                               flags :  u16,
                               minKeyCode :  xproto::keycode,
                               maxKeyCode :  xproto::keycode,
                               firstType :  u8,
                               nTypes :  u8,
                               firstKeySym :  xproto::keycode,
                               nKeySyms :  u8,
                               totalSyms :  u16,
                               firstKeyAction :  xproto::keycode,
                               nKeyActions :  u8,
                               totalActions :  u16,
                               firstKeyBehavior :  xproto::keycode,
                               nKeyBehaviors :  u8,
                               totalKeyBehaviors :  u8,
                               firstKeyExplicit :  xproto::keycode,
                               nKeyExplicit :  u8,
                               totalKeyExplicit :  u8,
                               firstModMapKey :  xproto::keycode,
                               nModMapKeys :  u8,
                               totalModMapKeys :  u8,
                               firstVModMapKey :  xproto::keycode,
                               nVModMapKeys :  u8,
                               totalVModMapKeys :  u8,
                               virtualMods :  u16,
                               values : *set_map_values) -> void_cookie;

unsafe fn xcb_xkb_get_compat_map_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xkb_get_compat_map (c : *connection,
                                  deviceSpec :  device_spec,
                                  groups :  u8,
                                  getAllSI :  u8,
                                  firstSI :  u16,
                                  nSI :  u16) -> get_compat_map_cookie;

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
unsafe fn xcb_xkb_get_compat_map_unchecked (c : *connection,
                                            deviceSpec :  device_spec,
                                            groups :  u8,
                                            getAllSI :  u8,
                                            firstSI :  u16,
                                            nSI :  u16) -> get_compat_map_cookie;

unsafe fn xcb_xkb_get_compat_map_si_rtrn (R : *get_compat_map_reply) -> *u8;


unsafe fn xcb_xkb_get_compat_map_si_rtrn_length (R : *get_compat_map_reply) -> c_int;


unsafe fn xcb_xkb_get_compat_map_si_rtrn_end (R : *get_compat_map_reply) -> generic_iterator;

unsafe fn xcb_xkb_get_compat_map_group_rtrn (R : *get_compat_map_reply) -> *mod_def;


unsafe fn xcb_xkb_get_compat_map_group_rtrn_length (R : *get_compat_map_reply) -> c_int;

unsafe fn xcb_xkb_get_compat_map_group_rtrn_iterator (R : *get_compat_map_reply) -> mod_def_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xkb_get_compat_map_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_xkb_get_compat_map_reply (c : *connection,
                                        cookie : get_compat_map_cookie,
                                        e : **generic_error) -> *get_compat_map_reply;

unsafe fn xcb_xkb_set_compat_map_sizeof (_buffer :  *c_void) -> c_int;

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
unsafe fn xcb_xkb_set_compat_map_checked (c : *connection,
                                          deviceSpec :  device_spec,
                                          recomputeActions :  u8,
                                          truncateSI :  u8,
                                          groups :  u8,
                                          firstSI :  u16,
                                          nSI :  u16,
                                          si : *u8,
                                          groupMaps : *mod_def) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xkb_set_compat_map (c : *connection,
                                  deviceSpec :  device_spec,
                                  recomputeActions :  u8,
                                  truncateSI :  u8,
                                  groups :  u8,
                                  firstSI :  u16,
                                  nSI :  u16,
                                  si : *u8,
                                  groupMaps : *mod_def) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xkb_get_indicator_state (c : *connection,
                                       deviceSpec :  device_spec) -> get_indicator_state_cookie;

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
unsafe fn xcb_xkb_get_indicator_state_unchecked (c : *connection,
                                                 deviceSpec :  device_spec) -> get_indicator_state_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xkb_get_indicator_state_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_xkb_get_indicator_state_reply (c : *connection,
                                             cookie : get_indicator_state_cookie,
                                             e : **generic_error) -> *get_indicator_state_reply;

unsafe fn xcb_xkb_get_indicator_map_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xkb_get_indicator_map (c : *connection,
                                     deviceSpec :  device_spec,
                                     which :  u32) -> get_indicator_map_cookie;

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
unsafe fn xcb_xkb_get_indicator_map_unchecked (c : *connection,
                                               deviceSpec :  device_spec,
                                               which :  u32) -> get_indicator_map_cookie;

unsafe fn xcb_xkb_get_indicator_map_maps (R : *get_indicator_map_reply) -> *indicator_map;


unsafe fn xcb_xkb_get_indicator_map_maps_length (R : *get_indicator_map_reply) -> c_int;

unsafe fn xcb_xkb_get_indicator_map_maps_iterator (R : *get_indicator_map_reply) -> indicator_map_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xkb_get_indicator_map_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_xkb_get_indicator_map_reply (c : *connection,
                                           cookie : get_indicator_map_cookie,
                                           e : **generic_error) -> *get_indicator_map_reply;

unsafe fn xcb_xkb_set_indicator_map_sizeof (_buffer :  *c_void) -> c_int;

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
unsafe fn xcb_xkb_set_indicator_map_checked (c : *connection,
                                             deviceSpec :  device_spec,
                                             which :  u32,
                                             maps : *indicator_map) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xkb_set_indicator_map (c : *connection,
                                     deviceSpec :  device_spec,
                                     which :  u32,
                                     maps : *indicator_map) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xkb_get_named_indicator (c : *connection,
                                       deviceSpec :  device_spec,
                                       ledClass :  led_class_spec,
                                       ledID :  id_spec,
                                       indicator :  xproto::atom) -> get_named_indicator_cookie;

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
unsafe fn xcb_xkb_get_named_indicator_unchecked (c : *connection,
                                                 deviceSpec :  device_spec,
                                                 ledClass :  led_class_spec,
                                                 ledID :  id_spec,
                                                 indicator :  xproto::atom) -> get_named_indicator_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xkb_get_named_indicator_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_xkb_get_named_indicator_reply (c : *connection,
                                             cookie : get_named_indicator_cookie,
                                             e : **generic_error) -> *get_named_indicator_reply;

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
unsafe fn xcb_xkb_set_named_indicator_checked (c : *connection,
                                               deviceSpec :  device_spec,
                                               ledClass :  led_class_spec,
                                               ledID :  id_spec,
                                               indicator :  xproto::atom,
                                               setState :  u8,
                                               on :  u8,
                                               setMap :  u8,
                                               createMap :  u8,
                                               map_flags :  u8,
                                               map_whichGroups :  u8,
                                               map_groups :  u8,
                                               map_whichMods :  u8,
                                               map_realMods :  u8,
                                               map_vmods :  u16,
                                               map_ctrls :  u32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xkb_set_named_indicator (c : *connection,
                                       deviceSpec :  device_spec,
                                       ledClass :  led_class_spec,
                                       ledID :  id_spec,
                                       indicator :  xproto::atom,
                                       setState :  u8,
                                       on :  u8,
                                       setMap :  u8,
                                       createMap :  u8,
                                       map_flags :  u8,
                                       map_whichGroups :  u8,
                                       map_groups :  u8,
                                       map_whichMods :  u8,
                                       map_realMods :  u8,
                                       map_vmods :  u16,
                                       map_ctrls :  u32) -> void_cookie;

unsafe fn xcb_xkb_get_names_value_list_type_names (S : *get_names_value_list) -> *xproto::atom;


unsafe fn xcb_xkb_get_names_value_list_type_names_length (R : *get_names_reply,
                                                     S : *get_names_value_list) -> c_int;


unsafe fn xcb_xkb_get_names_value_list_type_names_end (R : get_names_reply,
                                             S : *get_names_value_list ) -> generic_iterator;

unsafe fn xcb_xkb_get_names_value_list_n_levels_per_type (S : *get_names_value_list) -> *u8;


unsafe fn xcb_xkb_get_names_value_list_n_levels_per_type_length (R : *get_names_reply,
                                                            S : *get_names_value_list) -> c_int;


unsafe fn xcb_xkb_get_names_value_list_n_levels_per_type_end (R : get_names_reply,
                                                    S : *get_names_value_list ) -> generic_iterator;

unsafe fn xcb_xkb_get_names_value_list_kt_level_names (S : *get_names_value_list) -> *xproto::atom;


unsafe fn xcb_xkb_get_names_value_list_kt_level_names_length (R : *get_names_reply,
                                                         S : *get_names_value_list) -> c_int;


unsafe fn xcb_xkb_get_names_value_list_kt_level_names_end (R : get_names_reply,
                                                 S : *get_names_value_list ) -> generic_iterator;

unsafe fn xcb_xkb_get_names_value_list_indicator_names (S : *get_names_value_list) -> *xproto::atom;


unsafe fn xcb_xkb_get_names_value_list_indicator_names_length (R : *get_names_reply,
                                                          S : *get_names_value_list) -> c_int;


unsafe fn xcb_xkb_get_names_value_list_indicator_names_end (R : get_names_reply,
                                                  S : *get_names_value_list ) -> generic_iterator;

unsafe fn xcb_xkb_get_names_value_list_virtual_mod_names (S : *get_names_value_list) -> *xproto::atom;


unsafe fn xcb_xkb_get_names_value_list_virtual_mod_names_length (R : *get_names_reply,
                                                            S : *get_names_value_list) -> c_int;


unsafe fn xcb_xkb_get_names_value_list_virtual_mod_names_end (R : get_names_reply,
                                                    S : *get_names_value_list ) -> generic_iterator;

unsafe fn xcb_xkb_get_names_value_list_groups (S : *get_names_value_list) -> *xproto::atom;


unsafe fn xcb_xkb_get_names_value_list_groups_length (R : *get_names_reply,
                                                 S : *get_names_value_list) -> c_int;


unsafe fn xcb_xkb_get_names_value_list_groups_end (R : get_names_reply,
                                         S : *get_names_value_list ) -> generic_iterator;

unsafe fn xcb_xkb_get_names_value_list_key_names (S : *get_names_value_list) -> *key_name;


unsafe fn xcb_xkb_get_names_value_list_key_names_length (R : *get_names_reply,
                                                    S : *get_names_value_list) -> c_int;

unsafe fn xcb_xkb_get_names_value_list_key_names_iterator (R : get_names_reply,
                                                 S : *get_names_value_list /**< */) -> key_name_iterator;

unsafe fn xcb_xkb_get_names_value_list_key_aliases (S : *get_names_value_list) -> *key_alias;


unsafe fn xcb_xkb_get_names_value_list_key_aliases_length (R : *get_names_reply,
                                                      S : *get_names_value_list) -> c_int;

unsafe fn xcb_xkb_get_names_value_list_key_aliases_iterator (R : get_names_reply,
                                                   S : *get_names_value_list /**< */) -> key_alias_iterator;

unsafe fn xcb_xkb_get_names_value_list_radio_group_names (S : *get_names_value_list) -> *xproto::atom;


unsafe fn xcb_xkb_get_names_value_list_radio_group_names_length (R : *get_names_reply,
                                                            S : *get_names_value_list) -> c_int;


unsafe fn xcb_xkb_get_names_value_list_radio_group_names_end (R : get_names_reply,
                                                    S : *get_names_value_list ) -> generic_iterator;

unsafe fn xcb_xkb_get_names_value_list_serialize (_buffer :              **c_void,
                                        nTypes :                 u8,
                                        indicators :             u32,
                                        virtualMods :            u16,
                                        groupNames :             u8,
                                        nKeys :                  u8,
                                        nKeyAliases :            u8,
                                        nRadioGroups :           u8,
                                        which :                  u32,
                                        _aux :                  *get_names_value_list) -> c_int;

unsafe fn xcb_xkb_get_names_value_list_unpack (_buffer :               *c_void,
                                     nTypes :                 u8,
                                     indicators :             u32,
                                     virtualMods :            u16,
                                     groupNames :             u8,
                                     nKeys :                  u8,
                                     nKeyAliases :            u8,
                                     nRadioGroups :           u8,
                                     which :                  u32,
                                     _aux :                  *get_names_value_list) -> c_int;

unsafe fn xcb_xkb_get_names_value_list_sizeof (_buffer :  *c_void,
                                     nTypes :   u8,
                                     indicators :  u32,
                                     virtualMods :  u16,
                                     groupNames :  u8,
                                     nKeys :    u8,
                                     nKeyAliases :  u8,
                                     nRadioGroups :  u8,
                                     which :    u32) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xkb_get_names (c : *connection,
                             deviceSpec :  device_spec,
                             which :  u32) -> get_names_cookie;

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
unsafe fn xcb_xkb_get_names_unchecked (c : *connection,
                                       deviceSpec :  device_spec,
                                       which :  u32) -> get_names_cookie;


/**
 *
 * xcb_xkb_get_names_value_list : *get_names_value_list
 * 
 *
 */
unsafe fn xcb_xkb_get_names_value_list (R : *get_names_reply) -> *c_void;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xkb_get_names_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_xkb_get_names_reply (c : *connection,
                                   cookie : get_names_cookie,
                                   e : **generic_error) -> *get_names_reply;

unsafe fn xcb_xkb_set_names_values_type_names (S : *set_names_values) -> *xproto::atom;


unsafe fn xcb_xkb_set_names_values_type_names_length (R : *set_names_request,
                                                 S : *set_names_values) -> c_int;


unsafe fn xcb_xkb_set_names_values_type_names_end (R : set_names_request,
                                         S : *set_names_values ) -> generic_iterator;

unsafe fn xcb_xkb_set_names_values_n_levels_per_type (S : *set_names_values) -> *u8;


unsafe fn xcb_xkb_set_names_values_n_levels_per_type_length (R : *set_names_request,
                                                        S : *set_names_values) -> c_int;


unsafe fn xcb_xkb_set_names_values_n_levels_per_type_end (R : set_names_request,
                                                S : *set_names_values ) -> generic_iterator;

unsafe fn xcb_xkb_set_names_values_kt_level_names (S : *set_names_values) -> *xproto::atom;


unsafe fn xcb_xkb_set_names_values_kt_level_names_length (R : *set_names_request,
                                                     S : *set_names_values) -> c_int;


unsafe fn xcb_xkb_set_names_values_kt_level_names_end (R : set_names_request,
                                             S : *set_names_values ) -> generic_iterator;

unsafe fn xcb_xkb_set_names_values_indicator_names (S : *set_names_values) -> *xproto::atom;


unsafe fn xcb_xkb_set_names_values_indicator_names_length (R : *set_names_request,
                                                      S : *set_names_values) -> c_int;


unsafe fn xcb_xkb_set_names_values_indicator_names_end (R : set_names_request,
                                              S : *set_names_values ) -> generic_iterator;

unsafe fn xcb_xkb_set_names_values_virtual_mod_names (S : *set_names_values) -> *xproto::atom;


unsafe fn xcb_xkb_set_names_values_virtual_mod_names_length (R : *set_names_request,
                                                        S : *set_names_values) -> c_int;


unsafe fn xcb_xkb_set_names_values_virtual_mod_names_end (R : set_names_request,
                                                S : *set_names_values ) -> generic_iterator;

unsafe fn xcb_xkb_set_names_values_groups (S : *set_names_values) -> *xproto::atom;


unsafe fn xcb_xkb_set_names_values_groups_length (R : *set_names_request,
                                             S : *set_names_values) -> c_int;


unsafe fn xcb_xkb_set_names_values_groups_end (R : set_names_request,
                                     S : *set_names_values ) -> generic_iterator;

unsafe fn xcb_xkb_set_names_values_key_names (S : *set_names_values) -> *key_name;


unsafe fn xcb_xkb_set_names_values_key_names_length (R : *set_names_request,
                                                S : *set_names_values) -> c_int;

unsafe fn xcb_xkb_set_names_values_key_names_iterator (R : set_names_request,
                                             S : *set_names_values /**< */) -> key_name_iterator;

unsafe fn xcb_xkb_set_names_values_key_aliases (S : *set_names_values) -> *key_alias;


unsafe fn xcb_xkb_set_names_values_key_aliases_length (R : *set_names_request,
                                                  S : *set_names_values) -> c_int;

unsafe fn xcb_xkb_set_names_values_key_aliases_iterator (R : set_names_request,
                                               S : *set_names_values /**< */) -> key_alias_iterator;

unsafe fn xcb_xkb_set_names_values_radio_group_names (S : *set_names_values) -> *xproto::atom;


unsafe fn xcb_xkb_set_names_values_radio_group_names_length (R : *set_names_request,
                                                        S : *set_names_values) -> c_int;


unsafe fn xcb_xkb_set_names_values_radio_group_names_end (R : set_names_request,
                                                S : *set_names_values ) -> generic_iterator;

unsafe fn xcb_xkb_set_names_values_serialize (_buffer :          **c_void,
                                    nTypes :             u8,
                                    nKTLevels :          u8,
                                    indicators :         u32,
                                    virtualMods :        u16,
                                    groupNames :         u8,
                                    nKeys :              u8,
                                    nKeyAliases :        u8,
                                    nRadioGroups :       u8,
                                    which :              u32,
                                    _aux :              *set_names_values) -> c_int;

unsafe fn xcb_xkb_set_names_values_unpack (_buffer :           *c_void,
                                 nTypes :             u8,
                                 nKTLevels :          u8,
                                 indicators :         u32,
                                 virtualMods :        u16,
                                 groupNames :         u8,
                                 nKeys :              u8,
                                 nKeyAliases :        u8,
                                 nRadioGroups :       u8,
                                 which :              u32,
                                 _aux :              *set_names_values) -> c_int;

unsafe fn xcb_xkb_set_names_values_sizeof (_buffer :  *c_void,
                                 nTypes :   u8,
                                 nKTLevels :  u8,
                                 indicators :  u32,
                                 virtualMods :  u16,
                                 groupNames :  u8,
                                 nKeys :    u8,
                                 nKeyAliases :  u8,
                                 nRadioGroups :  u8,
                                 which :    u32) -> c_int;

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
unsafe fn xcb_xkb_set_names_checked (c : *connection,
                                     deviceSpec :  device_spec,
                                     virtualMods :  u16,
                                     which :  u32,
                                     firstType :  u8,
                                     nTypes :  u8,
                                     firstKTLevelt :  u8,
                                     nKTLevels :  u8,
                                     indicators :  u32,
                                     groupNames :  u8,
                                     nRadioGroups :  u8,
                                     firstKey :  xproto::keycode,
                                     nKeys :  u8,
                                     nKeyAliases :  u8,
                                     totalKTLevelNames :  u16,
                                     values : *()) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xkb_set_names (c : *connection,
                             deviceSpec :  device_spec,
                             virtualMods :  u16,
                             which :  u32,
                             firstType :  u8,
                             nTypes :  u8,
                             firstKTLevelt :  u8,
                             nKTLevels :  u8,
                             indicators :  u32,
                             groupNames :  u8,
                             nRadioGroups :  u8,
                             firstKey :  xproto::keycode,
                             nKeys :  u8,
                             nKeyAliases :  u8,
                             totalKTLevelNames :  u16,
                             values : *()) -> void_cookie;

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
unsafe fn xcb_xkb_set_names_aux_checked (c : *connection,
                                         deviceSpec :  device_spec,
                                         virtualMods :  u16,
                                         which :  u32,
                                         firstType :  u8,
                                         nTypes :  u8,
                                         firstKTLevelt :  u8,
                                         nKTLevels :  u8,
                                         indicators :  u32,
                                         groupNames :  u8,
                                         nRadioGroups :  u8,
                                         firstKey :  xproto::keycode,
                                         nKeys :  u8,
                                         nKeyAliases :  u8,
                                         totalKTLevelNames :  u16,
                                         values : *set_names_values) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xkb_set_names_aux (c : *connection,
                                 deviceSpec :  device_spec,
                                 virtualMods :  u16,
                                 which :  u32,
                                 firstType :  u8,
                                 nTypes :  u8,
                                 firstKTLevelt :  u8,
                                 nKTLevels :  u8,
                                 indicators :  u32,
                                 groupNames :  u8,
                                 nRadioGroups :  u8,
                                 firstKey :  xproto::keycode,
                                 nKeys :  u8,
                                 nKeyAliases :  u8,
                                 totalKTLevelNames :  u16,
                                 values : *set_names_values) -> void_cookie;

unsafe fn xcb_xkb_get_geometry_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xkb_get_geometry (c : *connection,
                                deviceSpec :  device_spec,
                                name :  xproto::atom) -> get_geometry_cookie;

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
unsafe fn xcb_xkb_get_geometry_unchecked (c : *connection,
                                          deviceSpec :  device_spec,
                                          name :  xproto::atom) -> get_geometry_cookie;


/**
 *
 * xcb_xkb_get_geometry_label_font : *counted_string_16
 * 
 *
 */
unsafe fn xcb_xkb_get_geometry_label_font (R : *get_geometry_reply) -> *counted_string_16;


unsafe fn xcb_xkb_get_geometry_properties_length (R : *get_geometry_reply) -> c_int;

unsafe fn xcb_xkb_get_geometry_properties_iterator (R : *get_geometry_reply) -> property_iterator;


unsafe fn xcb_xkb_get_geometry_colors_length (R : *get_geometry_reply) -> c_int;

unsafe fn xcb_xkb_get_geometry_colors_iterator (R : *get_geometry_reply) -> counted_string_16_iterator;


unsafe fn xcb_xkb_get_geometry_shapes_length (R : *get_geometry_reply) -> c_int;

unsafe fn xcb_xkb_get_geometry_shapes_iterator (R : *get_geometry_reply) -> shape_iterator;


unsafe fn xcb_xkb_get_geometry_sections_length (R : *get_geometry_reply) -> c_int;

unsafe fn xcb_xkb_get_geometry_sections_iterator (R : *get_geometry_reply) -> section_iterator;


unsafe fn xcb_xkb_get_geometry_doodads_length (R : *get_geometry_reply) -> c_int;

unsafe fn xcb_xkb_get_geometry_doodads_iterator (R : *get_geometry_reply) -> doodad_iterator;

unsafe fn xcb_xkb_get_geometry_key_aliases (R : *get_geometry_reply) -> *key_alias;


unsafe fn xcb_xkb_get_geometry_key_aliases_length (R : *get_geometry_reply) -> c_int;

unsafe fn xcb_xkb_get_geometry_key_aliases_iterator (R : *get_geometry_reply) -> key_alias_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xkb_get_geometry_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_xkb_get_geometry_reply (c : *connection,
                                      cookie : get_geometry_cookie,
                                      e : **generic_error) -> *get_geometry_reply;

unsafe fn xcb_xkb_set_geometry_sizeof (_buffer :  *c_void) -> c_int;

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
unsafe fn xcb_xkb_set_geometry_checked (c : *connection,
                                        deviceSpec :  device_spec,
                                        nShapes :  u8,
                                        nSections :  u8,
                                        name :  xproto::atom,
                                        widthMM :  u16,
                                        heightMM :  u16,
                                        nProperties :  u16,
                                        nColors :  u16,
                                        nDoodads :  u16,
                                        nKeyAliases :  u16,
                                        baseColorNdx :  u8,
                                        labelColorNdx :  u8,
                                        labelFont : *counted_string_16,
                                        properties : *property,
                                        colors : *counted_string_16,
                                        shapes : *shape,
                                        sections : *section,
                                        doodads : *doodad,
                                        keyAliases : *key_alias) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xkb_set_geometry (c : *connection,
                                deviceSpec :  device_spec,
                                nShapes :  u8,
                                nSections :  u8,
                                name :  xproto::atom,
                                widthMM :  u16,
                                heightMM :  u16,
                                nProperties :  u16,
                                nColors :  u16,
                                nDoodads :  u16,
                                nKeyAliases :  u16,
                                baseColorNdx :  u8,
                                labelColorNdx :  u8,
                                labelFont : *counted_string_16,
                                properties : *property,
                                colors : *counted_string_16,
                                shapes : *shape,
                                sections : *section,
                                doodads : *doodad,
                                keyAliases : *key_alias) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xkb_per_client_flags (c : *connection,
                                    deviceSpec :  device_spec,
                                    change :  u32,
                                    value :  u32,
                                    ctrlsToChange :  u32,
                                    autoCtrls :  u32,
                                    autoCtrlsValues :  u32) -> per_client_flags_cookie;

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
unsafe fn xcb_xkb_per_client_flags_unchecked (c : *connection,
                                              deviceSpec :  device_spec,
                                              change :  u32,
                                              value :  u32,
                                              ctrlsToChange :  u32,
                                              autoCtrls :  u32,
                                              autoCtrlsValues :  u32) -> per_client_flags_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xkb_per_client_flags_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_xkb_per_client_flags_reply (c : *connection,
                                          cookie : per_client_flags_cookie,
                                          e : **generic_error) -> *per_client_flags_reply;

unsafe fn xcb_xkb_list_components_serialize (_buffer :                 **c_void,
                                   _aux :                     *list_components_request,
                                   keymapsSpec :              *string8,
                                   keycodesSpec :             *string8,
                                   typesSpec :                *string8,
                                   compatMapSpec :            *string8,
                                   symbolsSpec :              *string8,
                                   geometrySpec :             *string8) -> c_int;

unsafe fn xcb_xkb_list_components_unserialize (_buffer :                   *c_void,
                                     _aux :                     **list_components_request) -> c_int;

unsafe fn xcb_xkb_list_components_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xkb_list_components (c : *connection,
                                   deviceSpec :  device_spec,
                                   maxNames :  u16,
                                   keymapsSpecLen :  u8,
                                   keymapsSpec : *string8,
                                   keycodesSpecLen :  u8,
                                   keycodesSpec : *string8,
                                   typesSpecLen :  u8,
                                   typesSpec : *string8,
                                   compatMapSpecLen :  u8,
                                   compatMapSpec : *string8,
                                   symbolsSpecLen :  u8,
                                   symbolsSpec : *string8,
                                   geometrySpecLen :  u8,
                                   geometrySpec : *string8) -> list_components_cookie;

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
unsafe fn xcb_xkb_list_components_unchecked (c : *connection,
                                             deviceSpec :  device_spec,
                                             maxNames :  u16,
                                             keymapsSpecLen :  u8,
                                             keymapsSpec : *string8,
                                             keycodesSpecLen :  u8,
                                             keycodesSpec : *string8,
                                             typesSpecLen :  u8,
                                             typesSpec : *string8,
                                             compatMapSpecLen :  u8,
                                             compatMapSpec : *string8,
                                             symbolsSpecLen :  u8,
                                             symbolsSpec : *string8,
                                             geometrySpecLen :  u8,
                                             geometrySpec : *string8) -> list_components_cookie;


unsafe fn xcb_xkb_list_components_keymaps_length (R : *list_components_reply) -> c_int;

unsafe fn xcb_xkb_list_components_keymaps_iterator (R : *list_components_reply) -> listing_iterator;


unsafe fn xcb_xkb_list_components_keycodes_length (R : *list_components_reply) -> c_int;

unsafe fn xcb_xkb_list_components_keycodes_iterator (R : *list_components_reply) -> listing_iterator;


unsafe fn xcb_xkb_list_components_types_length (R : *list_components_reply) -> c_int;

unsafe fn xcb_xkb_list_components_types_iterator (R : *list_components_reply) -> listing_iterator;


unsafe fn xcb_xkb_list_components_compat_maps_length (R : *list_components_reply) -> c_int;

unsafe fn xcb_xkb_list_components_compat_maps_iterator (R : *list_components_reply) -> listing_iterator;


unsafe fn xcb_xkb_list_components_symbols_length (R : *list_components_reply) -> c_int;

unsafe fn xcb_xkb_list_components_symbols_iterator (R : *list_components_reply) -> listing_iterator;


unsafe fn xcb_xkb_list_components_geometries_length (R : *list_components_reply) -> c_int;

unsafe fn xcb_xkb_list_components_geometries_iterator (R : *list_components_reply) -> listing_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xkb_list_components_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_xkb_list_components_reply (c : *connection,
                                         cookie : list_components_cookie,
                                         e : **generic_error) -> *list_components_reply;

unsafe fn xcb_xkb_get_kbd_by_name_serialize (_buffer :                 **c_void,
                                   _aux :                     *get_kbd_by_name_request,
                                   keymapsSpec :              *string8,
                                   keycodesSpec :             *string8,
                                   typesSpec :                *string8,
                                   compatMapSpec :            *string8,
                                   symbolsSpec :              *string8,
                                   geometrySpec :             *string8) -> c_int;

unsafe fn xcb_xkb_get_kbd_by_name_unserialize (_buffer :                   *c_void,
                                     _aux :                     **get_kbd_by_name_request) -> c_int;

unsafe fn xcb_xkb_get_kbd_by_name_sizeof (_buffer :  *c_void) -> c_int;


unsafe fn xcb_xkb_get_kbd_by_name_replies_types_map_types_rtrn_length (R : *get_kbd_by_name_reply,
                                                                  S : *get_kbd_by_name_replies) -> c_int;

unsafe fn xcb_xkb_get_kbd_by_name_replies_types_map_types_rtrn_iterator (R : get_kbd_by_name_reply,
                                                               S : *get_kbd_by_name_replies /**< */) -> key_type_iterator;


unsafe fn xcb_xkb_get_kbd_by_name_replies_types_map_syms_rtrn_length (R : *get_kbd_by_name_reply,
                                                                 S : *get_kbd_by_name_replies) -> c_int;

unsafe fn xcb_xkb_get_kbd_by_name_replies_types_map_syms_rtrn_iterator (R : get_kbd_by_name_reply,
                                                              S : *get_kbd_by_name_replies /**< */) -> key_sym_map_iterator;

unsafe fn xcb_xkb_get_kbd_by_name_replies_types_map_acts_rtrn_count (S : *get_kbd_by_name_replies) -> *u8;


unsafe fn xcb_xkb_get_kbd_by_name_replies_types_map_acts_rtrn_count_length (R : *get_kbd_by_name_reply,
                                                                       S : *get_kbd_by_name_replies) -> c_int;


unsafe fn xcb_xkb_get_kbd_by_name_replies_types_map_acts_rtrn_count_end (R : get_kbd_by_name_reply,
                                                               S : *get_kbd_by_name_replies ) -> generic_iterator;

unsafe fn xcb_xkb_get_kbd_by_name_replies_types_map_acts_rtrn_acts (S : *get_kbd_by_name_replies) -> *action;


unsafe fn xcb_xkb_get_kbd_by_name_replies_types_map_acts_rtrn_acts_length (R : *get_kbd_by_name_reply,
                                                                      S : *get_kbd_by_name_replies) -> c_int;

unsafe fn xcb_xkb_get_kbd_by_name_replies_types_map_acts_rtrn_acts_iterator (R : get_kbd_by_name_reply,
                                                                   S : *get_kbd_by_name_replies /**< */) -> action_iterator;

unsafe fn xcb_xkb_get_kbd_by_name_replies_types_map_behaviors_rtrn (S : *get_kbd_by_name_replies) -> *set_behavior;


unsafe fn xcb_xkb_get_kbd_by_name_replies_types_map_behaviors_rtrn_length (R : *get_kbd_by_name_reply,
                                                                      S : *get_kbd_by_name_replies) -> c_int;

unsafe fn xcb_xkb_get_kbd_by_name_replies_types_map_behaviors_rtrn_iterator (R : get_kbd_by_name_reply,
                                                                   S : *get_kbd_by_name_replies /**< */) -> set_behavior_iterator;

unsafe fn xcb_xkb_get_kbd_by_name_replies_types_map_vmods_rtrn (S : *get_kbd_by_name_replies) -> *u8;


unsafe fn xcb_xkb_get_kbd_by_name_replies_types_map_vmods_rtrn_length (R : *get_kbd_by_name_reply,
                                                                  S : *get_kbd_by_name_replies) -> c_int;


unsafe fn xcb_xkb_get_kbd_by_name_replies_types_map_vmods_rtrn_end (R : get_kbd_by_name_reply,
                                                          S : *get_kbd_by_name_replies ) -> generic_iterator;

unsafe fn xcb_xkb_get_kbd_by_name_replies_types_map_explicit_rtrn (S : *get_kbd_by_name_replies) -> *set_explicit;


unsafe fn xcb_xkb_get_kbd_by_name_replies_types_map_explicit_rtrn_length (R : *get_kbd_by_name_reply,
                                                                     S : *get_kbd_by_name_replies) -> c_int;

unsafe fn xcb_xkb_get_kbd_by_name_replies_types_map_explicit_rtrn_iterator (R : get_kbd_by_name_reply,
                                                                  S : *get_kbd_by_name_replies /**< */) -> set_explicit_iterator;

unsafe fn xcb_xkb_get_kbd_by_name_replies_types_map_modmap_rtrn (S : *get_kbd_by_name_replies) -> *key_mod_map;


unsafe fn xcb_xkb_get_kbd_by_name_replies_types_map_modmap_rtrn_length (R : *get_kbd_by_name_reply,
                                                                   S : *get_kbd_by_name_replies) -> c_int;

unsafe fn xcb_xkb_get_kbd_by_name_replies_types_map_modmap_rtrn_iterator (R : get_kbd_by_name_reply,
                                                                S : *get_kbd_by_name_replies /**< */) -> key_mod_map_iterator;

unsafe fn xcb_xkb_get_kbd_by_name_replies_types_map_vmodmap_rtrn (S : *get_kbd_by_name_replies) -> *key_v_mod_map;


unsafe fn xcb_xkb_get_kbd_by_name_replies_types_map_vmodmap_rtrn_length (R : *get_kbd_by_name_reply,
                                                                    S : *get_kbd_by_name_replies) -> c_int;

unsafe fn xcb_xkb_get_kbd_by_name_replies_types_map_vmodmap_rtrn_iterator (R : get_kbd_by_name_reply,
                                                                 S : *get_kbd_by_name_replies /**< */) -> key_v_mod_map_iterator;

unsafe fn xcb_xkb_get_kbd_by_name_replies_types_map_serialize (_buffer :                           **c_void,
                                                     nTypes :                              u8,
                                                     nKeySyms :                            u8,
                                                     nKeyActions :                         u8,
                                                     totalActions :                        u16,
                                                     totalKeyBehaviors :                   u8,
                                                     nVModMapKeys :                        u8,
                                                     totalKeyExplicit :                    u8,
                                                     totalModMapKeys :                     u8,
                                                     totalVModMapKeys :                    u8,
                                                     present :                             u16,
                                                     _aux :                               *get_kbd_by_name_replies_types_map) -> c_int;

unsafe fn xcb_xkb_get_kbd_by_name_replies_types_map_unpack (_buffer :                            *c_void,
                                                  nTypes :                              u8,
                                                  nKeySyms :                            u8,
                                                  nKeyActions :                         u8,
                                                  totalActions :                        u16,
                                                  totalKeyBehaviors :                   u8,
                                                  nVModMapKeys :                        u8,
                                                  totalKeyExplicit :                    u8,
                                                  totalModMapKeys :                     u8,
                                                  totalVModMapKeys :                    u8,
                                                  present :                             u16,
                                                  _aux :                               *get_kbd_by_name_replies_types_map) -> c_int;

unsafe fn xcb_xkb_get_kbd_by_name_replies_types_map_sizeof (_buffer :  *c_void,
                                                  nTypes :   u8,
                                                  nKeySyms :  u8,
                                                  nKeyActions :  u8,
                                                  totalActions :  u16,
                                                  totalKeyBehaviors :  u8,
                                                  nVModMapKeys :  u8,
                                                  totalKeyExplicit :  u8,
                                                  totalModMapKeys :  u8,
                                                  totalVModMapKeys :  u8,
                                                  present :   u16) -> c_int;


unsafe fn xcb_xkb_get_kbd_by_name_replies_client_symbols_map_types_rtrn_length (R : *get_kbd_by_name_reply,
                                                                           S : *get_kbd_by_name_replies) -> c_int;

unsafe fn xcb_xkb_get_kbd_by_name_replies_client_symbols_map_types_rtrn_iterator (R : get_kbd_by_name_reply,
                                                                        S : *get_kbd_by_name_replies /**< */) -> key_type_iterator;


unsafe fn xcb_xkb_get_kbd_by_name_replies_client_symbols_map_syms_rtrn_length (R : *get_kbd_by_name_reply,
                                                                          S : *get_kbd_by_name_replies) -> c_int;

unsafe fn xcb_xkb_get_kbd_by_name_replies_client_symbols_map_syms_rtrn_iterator (R : get_kbd_by_name_reply,
                                                                       S : *get_kbd_by_name_replies /**< */) -> key_sym_map_iterator;

unsafe fn xcb_xkb_get_kbd_by_name_replies_client_symbols_map_acts_rtrn_count (S : *get_kbd_by_name_replies) -> *u8;


unsafe fn xcb_xkb_get_kbd_by_name_replies_client_symbols_map_acts_rtrn_count_length (R : *get_kbd_by_name_reply,
                                                                                S : *get_kbd_by_name_replies) -> c_int;


unsafe fn xcb_xkb_get_kbd_by_name_replies_client_symbols_map_acts_rtrn_count_end (R : get_kbd_by_name_reply,
                                                                        S : *get_kbd_by_name_replies ) -> generic_iterator;

unsafe fn xcb_xkb_get_kbd_by_name_replies_client_symbols_map_acts_rtrn_acts (S : *get_kbd_by_name_replies) -> *action;


unsafe fn xcb_xkb_get_kbd_by_name_replies_client_symbols_map_acts_rtrn_acts_length (R : *get_kbd_by_name_reply,
                                                                               S : *get_kbd_by_name_replies) -> c_int;

unsafe fn xcb_xkb_get_kbd_by_name_replies_client_symbols_map_acts_rtrn_acts_iterator (R : get_kbd_by_name_reply,
                                                                            S : *get_kbd_by_name_replies /**< */) -> action_iterator;

unsafe fn xcb_xkb_get_kbd_by_name_replies_client_symbols_map_behaviors_rtrn (S : *get_kbd_by_name_replies) -> *set_behavior;


unsafe fn xcb_xkb_get_kbd_by_name_replies_client_symbols_map_behaviors_rtrn_length (R : *get_kbd_by_name_reply,
                                                                               S : *get_kbd_by_name_replies) -> c_int;

unsafe fn xcb_xkb_get_kbd_by_name_replies_client_symbols_map_behaviors_rtrn_iterator (R : get_kbd_by_name_reply,
                                                                            S : *get_kbd_by_name_replies /**< */) -> set_behavior_iterator;

unsafe fn xcb_xkb_get_kbd_by_name_replies_client_symbols_map_vmods_rtrn (S : *get_kbd_by_name_replies) -> *u8;


unsafe fn xcb_xkb_get_kbd_by_name_replies_client_symbols_map_vmods_rtrn_length (R : *get_kbd_by_name_reply,
                                                                           S : *get_kbd_by_name_replies) -> c_int;


unsafe fn xcb_xkb_get_kbd_by_name_replies_client_symbols_map_vmods_rtrn_end (R : get_kbd_by_name_reply,
                                                                   S : *get_kbd_by_name_replies ) -> generic_iterator;

unsafe fn xcb_xkb_get_kbd_by_name_replies_client_symbols_map_explicit_rtrn (S : *get_kbd_by_name_replies) -> *set_explicit;


unsafe fn xcb_xkb_get_kbd_by_name_replies_client_symbols_map_explicit_rtrn_length (R : *get_kbd_by_name_reply,
                                                                              S : *get_kbd_by_name_replies) -> c_int;

unsafe fn xcb_xkb_get_kbd_by_name_replies_client_symbols_map_explicit_rtrn_iterator (R : get_kbd_by_name_reply,
                                                                           S : *get_kbd_by_name_replies /**< */) -> set_explicit_iterator;

unsafe fn xcb_xkb_get_kbd_by_name_replies_client_symbols_map_modmap_rtrn (S : *get_kbd_by_name_replies) -> *key_mod_map;


unsafe fn xcb_xkb_get_kbd_by_name_replies_client_symbols_map_modmap_rtrn_length (R : *get_kbd_by_name_reply,
                                                                            S : *get_kbd_by_name_replies) -> c_int;

unsafe fn xcb_xkb_get_kbd_by_name_replies_client_symbols_map_modmap_rtrn_iterator (R : get_kbd_by_name_reply,
                                                                         S : *get_kbd_by_name_replies /**< */) -> key_mod_map_iterator;

unsafe fn xcb_xkb_get_kbd_by_name_replies_client_symbols_map_vmodmap_rtrn (S : *get_kbd_by_name_replies) -> *key_v_mod_map;


unsafe fn xcb_xkb_get_kbd_by_name_replies_client_symbols_map_vmodmap_rtrn_length (R : *get_kbd_by_name_reply,
                                                                             S : *get_kbd_by_name_replies) -> c_int;

unsafe fn xcb_xkb_get_kbd_by_name_replies_client_symbols_map_vmodmap_rtrn_iterator (R : get_kbd_by_name_reply,
                                                                          S : *get_kbd_by_name_replies /**< */) -> key_v_mod_map_iterator;

unsafe fn xcb_xkb_get_kbd_by_name_replies_client_symbols_map_serialize (_buffer :                                    **c_void,
                                                              nTypes :                                       u8,
                                                              nKeySyms :                                     u8,
                                                              nKeyActions :                                  u8,
                                                              totalActions :                                 u16,
                                                              totalKeyBehaviors :                            u8,
                                                              nVModMapKeys :                                 u8,
                                                              totalKeyExplicit :                             u8,
                                                              totalModMapKeys :                              u8,
                                                              totalVModMapKeys :                             u8,
                                                              present :                                      u16,
                                                              _aux :                                        *get_kbd_by_name_replies_client_symbols_map) -> c_int;

unsafe fn xcb_xkb_get_kbd_by_name_replies_client_symbols_map_unpack (_buffer :                                     *c_void,
                                                           nTypes :                                       u8,
                                                           nKeySyms :                                     u8,
                                                           nKeyActions :                                  u8,
                                                           totalActions :                                 u16,
                                                           totalKeyBehaviors :                            u8,
                                                           nVModMapKeys :                                 u8,
                                                           totalKeyExplicit :                             u8,
                                                           totalModMapKeys :                              u8,
                                                           totalVModMapKeys :                             u8,
                                                           present :                                      u16,
                                                           _aux :                                        *get_kbd_by_name_replies_client_symbols_map) -> c_int;

unsafe fn xcb_xkb_get_kbd_by_name_replies_client_symbols_map_sizeof (_buffer :  *c_void,
                                                           nTypes :   u8,
                                                           nKeySyms :  u8,
                                                           nKeyActions :  u8,
                                                           totalActions :  u16,
                                                           totalKeyBehaviors :  u8,
                                                           nVModMapKeys :  u8,
                                                           totalKeyExplicit :  u8,
                                                           totalModMapKeys :  u8,
                                                           totalVModMapKeys :  u8,
                                                           present :   u16) -> c_int;


unsafe fn xcb_xkb_get_kbd_by_name_replies_server_symbols_map_types_rtrn_length (R : *get_kbd_by_name_reply,
                                                                           S : *get_kbd_by_name_replies) -> c_int;

unsafe fn xcb_xkb_get_kbd_by_name_replies_server_symbols_map_types_rtrn_iterator (R : get_kbd_by_name_reply,
                                                                        S : *get_kbd_by_name_replies /**< */) -> key_type_iterator;


unsafe fn xcb_xkb_get_kbd_by_name_replies_server_symbols_map_syms_rtrn_length (R : *get_kbd_by_name_reply,
                                                                          S : *get_kbd_by_name_replies) -> c_int;

unsafe fn xcb_xkb_get_kbd_by_name_replies_server_symbols_map_syms_rtrn_iterator (R : get_kbd_by_name_reply,
                                                                       S : *get_kbd_by_name_replies /**< */) -> key_sym_map_iterator;

unsafe fn xcb_xkb_get_kbd_by_name_replies_server_symbols_map_acts_rtrn_count (S : *get_kbd_by_name_replies) -> *u8;


unsafe fn xcb_xkb_get_kbd_by_name_replies_server_symbols_map_acts_rtrn_count_length (R : *get_kbd_by_name_reply,
                                                                                S : *get_kbd_by_name_replies) -> c_int;


unsafe fn xcb_xkb_get_kbd_by_name_replies_server_symbols_map_acts_rtrn_count_end (R : get_kbd_by_name_reply,
                                                                        S : *get_kbd_by_name_replies ) -> generic_iterator;

unsafe fn xcb_xkb_get_kbd_by_name_replies_server_symbols_map_acts_rtrn_acts (S : *get_kbd_by_name_replies) -> *action;


unsafe fn xcb_xkb_get_kbd_by_name_replies_server_symbols_map_acts_rtrn_acts_length (R : *get_kbd_by_name_reply,
                                                                               S : *get_kbd_by_name_replies) -> c_int;

unsafe fn xcb_xkb_get_kbd_by_name_replies_server_symbols_map_acts_rtrn_acts_iterator (R : get_kbd_by_name_reply,
                                                                            S : *get_kbd_by_name_replies /**< */) -> action_iterator;

unsafe fn xcb_xkb_get_kbd_by_name_replies_server_symbols_map_behaviors_rtrn (S : *get_kbd_by_name_replies) -> *set_behavior;


unsafe fn xcb_xkb_get_kbd_by_name_replies_server_symbols_map_behaviors_rtrn_length (R : *get_kbd_by_name_reply,
                                                                               S : *get_kbd_by_name_replies) -> c_int;

unsafe fn xcb_xkb_get_kbd_by_name_replies_server_symbols_map_behaviors_rtrn_iterator (R : get_kbd_by_name_reply,
                                                                            S : *get_kbd_by_name_replies /**< */) -> set_behavior_iterator;

unsafe fn xcb_xkb_get_kbd_by_name_replies_server_symbols_map_vmods_rtrn (S : *get_kbd_by_name_replies) -> *u8;


unsafe fn xcb_xkb_get_kbd_by_name_replies_server_symbols_map_vmods_rtrn_length (R : *get_kbd_by_name_reply,
                                                                           S : *get_kbd_by_name_replies) -> c_int;


unsafe fn xcb_xkb_get_kbd_by_name_replies_server_symbols_map_vmods_rtrn_end (R : get_kbd_by_name_reply,
                                                                   S : *get_kbd_by_name_replies ) -> generic_iterator;

unsafe fn xcb_xkb_get_kbd_by_name_replies_server_symbols_map_explicit_rtrn (S : *get_kbd_by_name_replies) -> *set_explicit;


unsafe fn xcb_xkb_get_kbd_by_name_replies_server_symbols_map_explicit_rtrn_length (R : *get_kbd_by_name_reply,
                                                                              S : *get_kbd_by_name_replies) -> c_int;

unsafe fn xcb_xkb_get_kbd_by_name_replies_server_symbols_map_explicit_rtrn_iterator (R : get_kbd_by_name_reply,
                                                                           S : *get_kbd_by_name_replies /**< */) -> set_explicit_iterator;

unsafe fn xcb_xkb_get_kbd_by_name_replies_server_symbols_map_modmap_rtrn (S : *get_kbd_by_name_replies) -> *key_mod_map;


unsafe fn xcb_xkb_get_kbd_by_name_replies_server_symbols_map_modmap_rtrn_length (R : *get_kbd_by_name_reply,
                                                                            S : *get_kbd_by_name_replies) -> c_int;

unsafe fn xcb_xkb_get_kbd_by_name_replies_server_symbols_map_modmap_rtrn_iterator (R : get_kbd_by_name_reply,
                                                                         S : *get_kbd_by_name_replies /**< */) -> key_mod_map_iterator;

unsafe fn xcb_xkb_get_kbd_by_name_replies_server_symbols_map_vmodmap_rtrn (S : *get_kbd_by_name_replies) -> *key_v_mod_map;


unsafe fn xcb_xkb_get_kbd_by_name_replies_server_symbols_map_vmodmap_rtrn_length (R : *get_kbd_by_name_reply,
                                                                             S : *get_kbd_by_name_replies) -> c_int;

unsafe fn xcb_xkb_get_kbd_by_name_replies_server_symbols_map_vmodmap_rtrn_iterator (R : get_kbd_by_name_reply,
                                                                          S : *get_kbd_by_name_replies /**< */) -> key_v_mod_map_iterator;

unsafe fn xcb_xkb_get_kbd_by_name_replies_server_symbols_map_serialize (_buffer :                                    **c_void,
                                                              nTypes :                                       u8,
                                                              nKeySyms :                                     u8,
                                                              nKeyActions :                                  u8,
                                                              totalActions :                                 u16,
                                                              totalKeyBehaviors :                            u8,
                                                              nVModMapKeys :                                 u8,
                                                              totalKeyExplicit :                             u8,
                                                              totalModMapKeys :                              u8,
                                                              totalVModMapKeys :                             u8,
                                                              present :                                      u16,
                                                              _aux :                                        *get_kbd_by_name_replies_server_symbols_map) -> c_int;

unsafe fn xcb_xkb_get_kbd_by_name_replies_server_symbols_map_unpack (_buffer :                                     *c_void,
                                                           nTypes :                                       u8,
                                                           nKeySyms :                                     u8,
                                                           nKeyActions :                                  u8,
                                                           totalActions :                                 u16,
                                                           totalKeyBehaviors :                            u8,
                                                           nVModMapKeys :                                 u8,
                                                           totalKeyExplicit :                             u8,
                                                           totalModMapKeys :                              u8,
                                                           totalVModMapKeys :                             u8,
                                                           present :                                      u16,
                                                           _aux :                                        *get_kbd_by_name_replies_server_symbols_map) -> c_int;

unsafe fn xcb_xkb_get_kbd_by_name_replies_server_symbols_map_sizeof (_buffer :  *c_void,
                                                           nTypes :   u8,
                                                           nKeySyms :  u8,
                                                           nKeyActions :  u8,
                                                           totalActions :  u16,
                                                           totalKeyBehaviors :  u8,
                                                           nVModMapKeys :  u8,
                                                           totalKeyExplicit :  u8,
                                                           totalModMapKeys :  u8,
                                                           totalVModMapKeys :  u8,
                                                           present :   u16) -> c_int;

unsafe fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_type_names (S : *get_kbd_by_name_replies) -> *xproto::atom;


unsafe fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_type_names_length (R : *get_kbd_by_name_reply,
                                                                             S : *get_kbd_by_name_replies) -> c_int;


unsafe fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_type_names_end (R : get_kbd_by_name_reply,
                                                                     S : *get_kbd_by_name_replies ) -> generic_iterator;

unsafe fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_n_levels_per_type (S : *get_kbd_by_name_replies) -> *u8;


unsafe fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_n_levels_per_type_length (R : *get_kbd_by_name_reply,
                                                                                    S : *get_kbd_by_name_replies) -> c_int;


unsafe fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_n_levels_per_type_end (R : get_kbd_by_name_reply,
                                                                            S : *get_kbd_by_name_replies ) -> generic_iterator;

unsafe fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_kt_level_names (S : *get_kbd_by_name_replies) -> *xproto::atom;


unsafe fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_kt_level_names_length (R : *get_kbd_by_name_reply,
                                                                                 S : *get_kbd_by_name_replies) -> c_int;


unsafe fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_kt_level_names_end (R : get_kbd_by_name_reply,
                                                                         S : *get_kbd_by_name_replies ) -> generic_iterator;

unsafe fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_indicator_names (S : *get_kbd_by_name_replies) -> *xproto::atom;


unsafe fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_indicator_names_length (R : *get_kbd_by_name_reply,
                                                                                  S : *get_kbd_by_name_replies) -> c_int;


unsafe fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_indicator_names_end (R : get_kbd_by_name_reply,
                                                                          S : *get_kbd_by_name_replies ) -> generic_iterator;

unsafe fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_virtual_mod_names (S : *get_kbd_by_name_replies) -> *xproto::atom;


unsafe fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_virtual_mod_names_length (R : *get_kbd_by_name_reply,
                                                                                    S : *get_kbd_by_name_replies) -> c_int;


unsafe fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_virtual_mod_names_end (R : get_kbd_by_name_reply,
                                                                            S : *get_kbd_by_name_replies ) -> generic_iterator;

unsafe fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_groups (S : *get_kbd_by_name_replies) -> *xproto::atom;


unsafe fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_groups_length (R : *get_kbd_by_name_reply,
                                                                         S : *get_kbd_by_name_replies) -> c_int;


unsafe fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_groups_end (R : get_kbd_by_name_reply,
                                                                 S : *get_kbd_by_name_replies ) -> generic_iterator;

unsafe fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_key_names (S : *get_kbd_by_name_replies) -> *key_name;


unsafe fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_key_names_length (R : *get_kbd_by_name_reply,
                                                                            S : *get_kbd_by_name_replies) -> c_int;

unsafe fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_key_names_iterator (R : get_kbd_by_name_reply,
                                                                         S : *get_kbd_by_name_replies /**< */) -> key_name_iterator;

unsafe fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_key_aliases (S : *get_kbd_by_name_replies) -> *key_alias;


unsafe fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_key_aliases_length (R : *get_kbd_by_name_reply,
                                                                              S : *get_kbd_by_name_replies) -> c_int;

unsafe fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_key_aliases_iterator (R : get_kbd_by_name_reply,
                                                                           S : *get_kbd_by_name_replies /**< */) -> key_alias_iterator;

unsafe fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_radio_group_names (S : *get_kbd_by_name_replies) -> *xproto::atom;


unsafe fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_radio_group_names_length (R : *get_kbd_by_name_reply,
                                                                                    S : *get_kbd_by_name_replies) -> c_int;


unsafe fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_radio_group_names_end (R : get_kbd_by_name_reply,
                                                                            S : *get_kbd_by_name_replies ) -> generic_iterator;

unsafe fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_serialize (_buffer :                                      **c_void,
                                                                nTypes :                                         u8,
                                                                nKTLevels :                                      u16,
                                                                indicators :                                     u32,
                                                                virtualMods :                                    u16,
                                                                groupNames :                                     u8,
                                                                nKeys :                                          u8,
                                                                nKeyAliases :                                    u8,
                                                                nRadioGroups :                                   u8,
                                                                which :                                          u32,
                                                                _aux :                                          *get_kbd_by_name_replies_key_names_value_list) -> c_int;

unsafe fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_unpack (_buffer :                                       *c_void,
                                                             nTypes :                                         u8,
                                                             nKTLevels :                                      u16,
                                                             indicators :                                     u32,
                                                             virtualMods :                                    u16,
                                                             groupNames :                                     u8,
                                                             nKeys :                                          u8,
                                                             nKeyAliases :                                    u8,
                                                             nRadioGroups :                                   u8,
                                                             which :                                          u32,
                                                             _aux :                                          *get_kbd_by_name_replies_key_names_value_list) -> c_int;

unsafe fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_sizeof (_buffer :  *c_void,
                                                             nTypes :   u8,
                                                             nKTLevels :  u16,
                                                             indicators :  u32,
                                                             virtualMods :  u16,
                                                             groupNames :  u8,
                                                             nKeys :    u8,
                                                             nKeyAliases :  u8,
                                                             nRadioGroups :  u8,
                                                             which :    u32) -> c_int;

unsafe fn xcb_xkb_get_kbd_by_name_replies_other_names_value_list_type_names (S : *get_kbd_by_name_replies) -> *xproto::atom;


unsafe fn xcb_xkb_get_kbd_by_name_replies_other_names_value_list_type_names_length (R : *get_kbd_by_name_reply,
                                                                               S : *get_kbd_by_name_replies) -> c_int;


unsafe fn xcb_xkb_get_kbd_by_name_replies_other_names_value_list_type_names_end (R : get_kbd_by_name_reply,
                                                                       S : *get_kbd_by_name_replies ) -> generic_iterator;

unsafe fn xcb_xkb_get_kbd_by_name_replies_other_names_value_list_n_levels_per_type (S : *get_kbd_by_name_replies) -> *u8;


unsafe fn xcb_xkb_get_kbd_by_name_replies_other_names_value_list_n_levels_per_type_length (R : *get_kbd_by_name_reply,
                                                                                      S : *get_kbd_by_name_replies) -> c_int;


unsafe fn xcb_xkb_get_kbd_by_name_replies_other_names_value_list_n_levels_per_type_end (R : get_kbd_by_name_reply,
                                                                              S : *get_kbd_by_name_replies ) -> generic_iterator;

unsafe fn xcb_xkb_get_kbd_by_name_replies_other_names_value_list_kt_level_names (S : *get_kbd_by_name_replies) -> *xproto::atom;


unsafe fn xcb_xkb_get_kbd_by_name_replies_other_names_value_list_kt_level_names_length (R : *get_kbd_by_name_reply,
                                                                                   S : *get_kbd_by_name_replies) -> c_int;


unsafe fn xcb_xkb_get_kbd_by_name_replies_other_names_value_list_kt_level_names_end (R : get_kbd_by_name_reply,
                                                                           S : *get_kbd_by_name_replies ) -> generic_iterator;

unsafe fn xcb_xkb_get_kbd_by_name_replies_other_names_value_list_indicator_names (S : *get_kbd_by_name_replies) -> *xproto::atom;


unsafe fn xcb_xkb_get_kbd_by_name_replies_other_names_value_list_indicator_names_length (R : *get_kbd_by_name_reply,
                                                                                    S : *get_kbd_by_name_replies) -> c_int;


unsafe fn xcb_xkb_get_kbd_by_name_replies_other_names_value_list_indicator_names_end (R : get_kbd_by_name_reply,
                                                                            S : *get_kbd_by_name_replies ) -> generic_iterator;

unsafe fn xcb_xkb_get_kbd_by_name_replies_other_names_value_list_virtual_mod_names (S : *get_kbd_by_name_replies) -> *xproto::atom;


unsafe fn xcb_xkb_get_kbd_by_name_replies_other_names_value_list_virtual_mod_names_length (R : *get_kbd_by_name_reply,
                                                                                      S : *get_kbd_by_name_replies) -> c_int;


unsafe fn xcb_xkb_get_kbd_by_name_replies_other_names_value_list_virtual_mod_names_end (R : get_kbd_by_name_reply,
                                                                              S : *get_kbd_by_name_replies ) -> generic_iterator;

unsafe fn xcb_xkb_get_kbd_by_name_replies_other_names_value_list_groups (S : *get_kbd_by_name_replies) -> *xproto::atom;


unsafe fn xcb_xkb_get_kbd_by_name_replies_other_names_value_list_groups_length (R : *get_kbd_by_name_reply,
                                                                           S : *get_kbd_by_name_replies) -> c_int;


unsafe fn xcb_xkb_get_kbd_by_name_replies_other_names_value_list_groups_end (R : get_kbd_by_name_reply,
                                                                   S : *get_kbd_by_name_replies ) -> generic_iterator;

unsafe fn xcb_xkb_get_kbd_by_name_replies_other_names_value_list_key_names (S : *get_kbd_by_name_replies) -> *key_name;


unsafe fn xcb_xkb_get_kbd_by_name_replies_other_names_value_list_key_names_length (R : *get_kbd_by_name_reply,
                                                                              S : *get_kbd_by_name_replies) -> c_int;

unsafe fn xcb_xkb_get_kbd_by_name_replies_other_names_value_list_key_names_iterator (R : get_kbd_by_name_reply,
                                                                           S : *get_kbd_by_name_replies /**< */) -> key_name_iterator;

unsafe fn xcb_xkb_get_kbd_by_name_replies_other_names_value_list_key_aliases (S : *get_kbd_by_name_replies) -> *key_alias;


unsafe fn xcb_xkb_get_kbd_by_name_replies_other_names_value_list_key_aliases_length (R : *get_kbd_by_name_reply,
                                                                                S : *get_kbd_by_name_replies) -> c_int;

unsafe fn xcb_xkb_get_kbd_by_name_replies_other_names_value_list_key_aliases_iterator (R : get_kbd_by_name_reply,
                                                                             S : *get_kbd_by_name_replies /**< */) -> key_alias_iterator;

unsafe fn xcb_xkb_get_kbd_by_name_replies_other_names_value_list_radio_group_names (S : *get_kbd_by_name_replies) -> *xproto::atom;


unsafe fn xcb_xkb_get_kbd_by_name_replies_other_names_value_list_radio_group_names_length (R : *get_kbd_by_name_reply,
                                                                                      S : *get_kbd_by_name_replies) -> c_int;


unsafe fn xcb_xkb_get_kbd_by_name_replies_other_names_value_list_radio_group_names_end (R : get_kbd_by_name_reply,
                                                                              S : *get_kbd_by_name_replies ) -> generic_iterator;

unsafe fn xcb_xkb_get_kbd_by_name_replies_other_names_value_list_serialize (_buffer :                                        **c_void,
                                                                  nTypes :                                           u8,
                                                                  nKTLevels :                                        u16,
                                                                  indicators :                                       u32,
                                                                  virtualMods :                                      u16,
                                                                  groupNames :                                       u8,
                                                                  nKeys :                                            u8,
                                                                  nKeyAliases :                                      u8,
                                                                  nRadioGroups :                                     u8,
                                                                  which :                                            u32,
                                                                  _aux :                                            *get_kbd_by_name_replies_other_names_value_list) -> c_int;

unsafe fn xcb_xkb_get_kbd_by_name_replies_other_names_value_list_unpack (_buffer :                                         *c_void,
                                                               nTypes :                                           u8,
                                                               nKTLevels :                                        u16,
                                                               indicators :                                       u32,
                                                               virtualMods :                                      u16,
                                                               groupNames :                                       u8,
                                                               nKeys :                                            u8,
                                                               nKeyAliases :                                      u8,
                                                               nRadioGroups :                                     u8,
                                                               which :                                            u32,
                                                               _aux :                                            *get_kbd_by_name_replies_other_names_value_list) -> c_int;

unsafe fn xcb_xkb_get_kbd_by_name_replies_other_names_value_list_sizeof (_buffer :  *c_void,
                                                               nTypes :   u8,
                                                               nKTLevels :  u16,
                                                               indicators :  u32,
                                                               virtualMods :  u16,
                                                               groupNames :  u8,
                                                               nKeys :    u8,
                                                               nKeyAliases :  u8,
                                                               nRadioGroups :  u8,
                                                               which :    u32) -> c_int;


/**
 *
 * xcb_xkb_get_kbd_by_name_replies_types_map : *get_kbd_by_name_replies_types_map
 * 
 *
 */
unsafe fn xcb_xkb_get_kbd_by_name_replies_types_map (R : *get_kbd_by_name_replies) -> *get_kbd_by_name_replies_types_map;

unsafe fn xcb_xkb_get_kbd_by_name_replies_compat_map_si_rtrn (S : *get_kbd_by_name_replies) -> *u8;


unsafe fn xcb_xkb_get_kbd_by_name_replies_compat_map_si_rtrn_length (R : *get_kbd_by_name_reply,
                                                                S : *get_kbd_by_name_replies) -> c_int;


unsafe fn xcb_xkb_get_kbd_by_name_replies_compat_map_si_rtrn_end (R : get_kbd_by_name_reply,
                                                        S : *get_kbd_by_name_replies ) -> generic_iterator;

unsafe fn xcb_xkb_get_kbd_by_name_replies_compat_map_group_rtrn (S : *get_kbd_by_name_replies) -> *mod_def;


unsafe fn xcb_xkb_get_kbd_by_name_replies_compat_map_group_rtrn_length (R : *get_kbd_by_name_reply,
                                                                   S : *get_kbd_by_name_replies) -> c_int;

unsafe fn xcb_xkb_get_kbd_by_name_replies_compat_map_group_rtrn_iterator (R : get_kbd_by_name_reply,
                                                                S : *get_kbd_by_name_replies /**< */) -> mod_def_iterator;


/**
 *
 * xcb_xkb_get_kbd_by_name_replies_client_symbols_map : *get_kbd_by_name_replies_client_symbols_map
 * 
 *
 */
unsafe fn xcb_xkb_get_kbd_by_name_replies_client_symbols_map (R : *get_kbd_by_name_replies) -> *get_kbd_by_name_replies_client_symbols_map;


/**
 *
 * xcb_xkb_get_kbd_by_name_replies_server_symbols_map : *get_kbd_by_name_replies_server_symbols_map
 * 
 *
 */
unsafe fn xcb_xkb_get_kbd_by_name_replies_server_symbols_map (R : *get_kbd_by_name_replies) -> *get_kbd_by_name_replies_server_symbols_map;

unsafe fn xcb_xkb_get_kbd_by_name_replies_indicator_maps_maps (S : *get_kbd_by_name_replies) -> *indicator_map;


unsafe fn xcb_xkb_get_kbd_by_name_replies_indicator_maps_maps_length (R : *get_kbd_by_name_reply,
                                                                 S : *get_kbd_by_name_replies) -> c_int;

unsafe fn xcb_xkb_get_kbd_by_name_replies_indicator_maps_maps_iterator (R : get_kbd_by_name_reply,
                                                              S : *get_kbd_by_name_replies /**< */) -> indicator_map_iterator;


/**
 *
 * xcb_xkb_get_kbd_by_name_replies_key_names_value_list : *get_kbd_by_name_replies_key_names_value_list
 * 
 *
 */
unsafe fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list (R : *get_kbd_by_name_replies) -> *get_kbd_by_name_replies_key_names_value_list;


/**
 *
 * xcb_xkb_get_kbd_by_name_replies_other_names_value_list : *get_kbd_by_name_replies_other_names_value_list
 * 
 *
 */
unsafe fn xcb_xkb_get_kbd_by_name_replies_other_names_value_list (R : *get_kbd_by_name_replies) -> *get_kbd_by_name_replies_other_names_value_list;


/**
 *
 * xcb_xkb_get_kbd_by_name_replies_geometry_label_font : *counted_string_16
 * 
 *
 */
unsafe fn xcb_xkb_get_kbd_by_name_replies_geometry_label_font (R : *get_kbd_by_name_replies) -> *counted_string_16;


unsafe fn xcb_xkb_get_kbd_by_name_replies_geometry_properties_length (R : *get_kbd_by_name_reply,
                                                                 S : *get_kbd_by_name_replies) -> c_int;

unsafe fn xcb_xkb_get_kbd_by_name_replies_geometry_properties_iterator (R : get_kbd_by_name_reply,
                                                              S : *get_kbd_by_name_replies /**< */) -> property_iterator;


unsafe fn xcb_xkb_get_kbd_by_name_replies_geometry_colors_length (R : *get_kbd_by_name_reply,
                                                             S : *get_kbd_by_name_replies) -> c_int;

unsafe fn xcb_xkb_get_kbd_by_name_replies_geometry_colors_iterator (R : get_kbd_by_name_reply,
                                                          S : *get_kbd_by_name_replies /**< */) -> counted_string_16_iterator;


unsafe fn xcb_xkb_get_kbd_by_name_replies_geometry_shapes_length (R : *get_kbd_by_name_reply,
                                                             S : *get_kbd_by_name_replies) -> c_int;

unsafe fn xcb_xkb_get_kbd_by_name_replies_geometry_shapes_iterator (R : get_kbd_by_name_reply,
                                                          S : *get_kbd_by_name_replies /**< */) -> shape_iterator;


unsafe fn xcb_xkb_get_kbd_by_name_replies_geometry_sections_length (R : *get_kbd_by_name_reply,
                                                               S : *get_kbd_by_name_replies) -> c_int;

unsafe fn xcb_xkb_get_kbd_by_name_replies_geometry_sections_iterator (R : get_kbd_by_name_reply,
                                                            S : *get_kbd_by_name_replies /**< */) -> section_iterator;


unsafe fn xcb_xkb_get_kbd_by_name_replies_geometry_doodads_length (R : *get_kbd_by_name_reply,
                                                              S : *get_kbd_by_name_replies) -> c_int;

unsafe fn xcb_xkb_get_kbd_by_name_replies_geometry_doodads_iterator (R : get_kbd_by_name_reply,
                                                           S : *get_kbd_by_name_replies /**< */) -> doodad_iterator;

unsafe fn xcb_xkb_get_kbd_by_name_replies_geometry_key_aliases (S : *get_kbd_by_name_replies) -> *key_alias;


unsafe fn xcb_xkb_get_kbd_by_name_replies_geometry_key_aliases_length (R : *get_kbd_by_name_reply,
                                                                  S : *get_kbd_by_name_replies) -> c_int;

unsafe fn xcb_xkb_get_kbd_by_name_replies_geometry_key_aliases_iterator (R : get_kbd_by_name_reply,
                                                               S : *get_kbd_by_name_replies /**< */) -> key_alias_iterator;

unsafe fn xcb_xkb_get_kbd_by_name_replies_serialize (_buffer :                 **c_void,
                                           reported :                  u16,
                                           _aux :                     *get_kbd_by_name_replies) -> c_int;

unsafe fn xcb_xkb_get_kbd_by_name_replies_unpack (_buffer :                  *c_void,
                                        reported :                  u16,
                                        _aux :                     *get_kbd_by_name_replies) -> c_int;

unsafe fn xcb_xkb_get_kbd_by_name_replies_sizeof (_buffer :  *c_void,
                                        reported :   u16) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xkb_get_kbd_by_name (c : *connection,
                                   deviceSpec :  device_spec,
                                   need :  u16,
                                   want :  u16,
                                   load :  u8,
                                   keymapsSpecLen :  u8,
                                   keymapsSpec : *string8,
                                   keycodesSpecLen :  u8,
                                   keycodesSpec : *string8,
                                   typesSpecLen :  u8,
                                   typesSpec : *string8,
                                   compatMapSpecLen :  u8,
                                   compatMapSpec : *string8,
                                   symbolsSpecLen :  u8,
                                   symbolsSpec : *string8,
                                   geometrySpecLen :  u8,
                                   geometrySpec : *string8) -> get_kbd_by_name_cookie;

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
unsafe fn xcb_xkb_get_kbd_by_name_unchecked (c : *connection,
                                             deviceSpec :  device_spec,
                                             need :  u16,
                                             want :  u16,
                                             load :  u8,
                                             keymapsSpecLen :  u8,
                                             keymapsSpec : *string8,
                                             keycodesSpecLen :  u8,
                                             keycodesSpec : *string8,
                                             typesSpecLen :  u8,
                                             typesSpec : *string8,
                                             compatMapSpecLen :  u8,
                                             compatMapSpec : *string8,
                                             symbolsSpecLen :  u8,
                                             symbolsSpec : *string8,
                                             geometrySpecLen :  u8,
                                             geometrySpec : *string8) -> get_kbd_by_name_cookie;


/**
 *
 * xcb_xkb_get_kbd_by_name_replies : *get_kbd_by_name_replies
 * 
 *
 */
unsafe fn xcb_xkb_get_kbd_by_name_replies (R : *get_kbd_by_name_reply) -> *c_void;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xkb_get_kbd_by_name_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_xkb_get_kbd_by_name_reply (c : *connection,
                                         cookie : get_kbd_by_name_cookie,
                                         e : **generic_error) -> *get_kbd_by_name_reply;

unsafe fn xcb_xkb_get_device_info_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xkb_get_device_info (c : *connection,
                                   deviceSpec :  device_spec,
                                   wanted :  u16,
                                   allButtons :  u8,
                                   firstButton :  u8,
                                   nButtons :  u8,
                                   ledClass :  led_class_spec,
                                   ledID :  id_spec) -> get_device_info_cookie;

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
unsafe fn xcb_xkb_get_device_info_unchecked (c : *connection,
                                             deviceSpec :  device_spec,
                                             wanted :  u16,
                                             allButtons :  u8,
                                             firstButton :  u8,
                                             nButtons :  u8,
                                             ledClass :  led_class_spec,
                                             ledID :  id_spec) -> get_device_info_cookie;

unsafe fn xcb_xkb_get_device_info_name (R : *get_device_info_reply) -> *string8;


unsafe fn xcb_xkb_get_device_info_name_length (R : *get_device_info_reply) -> c_int;


unsafe fn xcb_xkb_get_device_info_name_end (R : *get_device_info_reply) -> generic_iterator;

unsafe fn xcb_xkb_get_device_info_btn_actions (R : *get_device_info_reply) -> *action;


unsafe fn xcb_xkb_get_device_info_btn_actions_length (R : *get_device_info_reply) -> c_int;

unsafe fn xcb_xkb_get_device_info_btn_actions_iterator (R : *get_device_info_reply) -> action_iterator;


unsafe fn xcb_xkb_get_device_info_leds_length (R : *get_device_info_reply) -> c_int;

unsafe fn xcb_xkb_get_device_info_leds_iterator (R : *get_device_info_reply) -> device_led_info_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xkb_get_device_info_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_xkb_get_device_info_reply (c : *connection,
                                         cookie : get_device_info_cookie,
                                         e : **generic_error) -> *get_device_info_reply;

unsafe fn xcb_xkb_set_device_info_sizeof (_buffer :  *c_void) -> c_int;

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
unsafe fn xcb_xkb_set_device_info_checked (c : *connection,
                                           deviceSpec :  device_spec,
                                           firstBtn :  u8,
                                           nBtns :  u8,
                                           change :  u16,
                                           nDeviceLedFBs :  u16,
                                           btnActions : *action,
                                           leds : *device_led_info) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xkb_set_device_info (c : *connection,
                                   deviceSpec :  device_spec,
                                   firstBtn :  u8,
                                   nBtns :  u8,
                                   change :  u16,
                                   nDeviceLedFBs :  u16,
                                   btnActions : *action,
                                   leds : *device_led_info) -> void_cookie;

unsafe fn xcb_xkb_set_debugging_flags_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xkb_set_debugging_flags (c : *connection,
                                       msgLength :  u16,
                                       affectFlags :  u32,
                                       flags :  u32,
                                       affectCtrls :  u32,
                                       ctrls :  u32,
                                       message : *string8) -> set_debugging_flags_cookie;

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
unsafe fn xcb_xkb_set_debugging_flags_unchecked (c : *connection,
                                                 msgLength :  u16,
                                                 affectFlags :  u32,
                                                 flags :  u32,
                                                 affectCtrls :  u32,
                                                 ctrls :  u32,
                                                 message : *string8) -> set_debugging_flags_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xkb_set_debugging_flags_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_xkb_set_debugging_flags_reply (c : *connection,
                                             cookie : set_debugging_flags_cookie,
                                             e : **generic_error) -> *set_debugging_flags_reply;
}

