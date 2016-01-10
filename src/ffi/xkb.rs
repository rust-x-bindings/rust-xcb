//
// This file generated automatically from xkb.xml by r_client.py.
// Edit at your peril.
//

//Make the compiler quiet
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use std;
use libc::*;
use ffi;
use ffi::xproto;

pub const XKB_MAJOR_VERSION : c_uint = 1;
pub const XKB_MINOR_VERSION : c_uint = 0;

#[repr(C)]
pub struct xcb_xkb_ax_option_t {
    data : [u8; 2]
}
impl Copy for xcb_xkb_ax_option_t {}
impl Clone for xcb_xkb_ax_option_t {
    fn clone(&self) -> xcb_xkb_ax_option_t { *self }
}
#[repr(C)]
pub struct xcb_xkb_ax_option_iterator_t {
    pub data : *mut xcb_xkb_ax_option_t,
    pub rem  : c_int,
    pub index: c_int
}


pub type xcb_xkb_device_spec_t = u16;
#[repr(C)]
pub struct xcb_xkb_device_spec_iterator_t {
    pub data : *mut xcb_xkb_device_spec_t,
    pub rem  : c_int,
    pub index: c_int
}


pub type xcb_xkb_led_class_spec_t = u16;
#[repr(C)]
pub struct xcb_xkb_led_class_spec_iterator_t {
    pub data : *mut xcb_xkb_led_class_spec_t,
    pub rem  : c_int,
    pub index: c_int
}


pub type xcb_xkb_bell_class_spec_t = u16;
#[repr(C)]
pub struct xcb_xkb_bell_class_spec_iterator_t {
    pub data : *mut xcb_xkb_bell_class_spec_t,
    pub rem  : c_int,
    pub index: c_int
}


pub type xcb_xkb_id_spec_t = u16;
#[repr(C)]
pub struct xcb_xkb_id_spec_iterator_t {
    pub data : *mut xcb_xkb_id_spec_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_xkb_indicator_map_t {
     pub flags :         u8,
     pub whichGroups :   u8,
     pub groups :        u8,
     pub whichMods :     u8,
     pub mods :          u8,
     pub realMods :      u8,
     pub vmods :         u16,
     pub ctrls :         u32
}

impl Copy for xcb_xkb_indicator_map_t {}
impl Clone for xcb_xkb_indicator_map_t {
    fn clone(&self) -> xcb_xkb_indicator_map_t { *self }
}
#[repr(C)]
pub struct xcb_xkb_indicator_map_iterator_t {
    pub data : *mut xcb_xkb_indicator_map_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_xkb_mod_def_t {
     pub mask :       u8,
     pub realMods :   u8,
     pub vmods :      u16
}

impl Copy for xcb_xkb_mod_def_t {}
impl Clone for xcb_xkb_mod_def_t {
    fn clone(&self) -> xcb_xkb_mod_def_t { *self }
}
#[repr(C)]
pub struct xcb_xkb_mod_def_iterator_t {
    pub data : *mut xcb_xkb_mod_def_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_xkb_key_name_t {
     pub name :   [u8; 4]
}

impl Copy for xcb_xkb_key_name_t {}
impl Clone for xcb_xkb_key_name_t {
    fn clone(&self) -> xcb_xkb_key_name_t { *self }
}
#[repr(C)]
pub struct xcb_xkb_key_name_iterator_t {
    pub data : *mut xcb_xkb_key_name_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_xkb_key_alias_t {
     pub real :    [u8; 4],
     pub alias :   [u8; 4]
}

impl Copy for xcb_xkb_key_alias_t {}
impl Clone for xcb_xkb_key_alias_t {
    fn clone(&self) -> xcb_xkb_key_alias_t { *self }
}
#[repr(C)]
pub struct xcb_xkb_key_alias_iterator_t {
    pub data : *mut xcb_xkb_key_alias_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_xkb_counted_string_8_t {
     pub length :   u8
}

impl Copy for xcb_xkb_counted_string_8_t {}
impl Clone for xcb_xkb_counted_string_8_t {
    fn clone(&self) -> xcb_xkb_counted_string_8_t { *self }
}
#[repr(C)]
pub struct xcb_xkb_counted_string_8_iterator_t {
    pub data : *mut xcb_xkb_counted_string_8_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_xkb_counted_string_16_t {
     pub length :   u16,
     pub pad0 :     u8
}

impl Copy for xcb_xkb_counted_string_16_t {}
impl Clone for xcb_xkb_counted_string_16_t {
    fn clone(&self) -> xcb_xkb_counted_string_16_t { *self }
}
#[repr(C)]
pub struct xcb_xkb_counted_string_16_iterator_t {
    pub data : *mut xcb_xkb_counted_string_16_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_xkb_kt_map_entry_t {
     pub active :       u8,
     pub level :        u8,
     pub mods_mask :    u8,
     pub mods_mods :    u8,
     pub mods_vmods :   u16,
     pub pad0 :         [u8; 2]
}

impl Copy for xcb_xkb_kt_map_entry_t {}
impl Clone for xcb_xkb_kt_map_entry_t {
    fn clone(&self) -> xcb_xkb_kt_map_entry_t { *self }
}
#[repr(C)]
pub struct xcb_xkb_kt_map_entry_iterator_t {
    pub data : *mut xcb_xkb_kt_map_entry_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_xkb_key_type_t {
     pub mods_mask :     u8,
     pub mods_mods :     u8,
     pub mods_vmods :    u16,
     pub numLevels :     u8,
     pub nMapEntries :   u8,
     pub hasPreserve :   u8,
     pub pad0 :          u8
}

impl Copy for xcb_xkb_key_type_t {}
impl Clone for xcb_xkb_key_type_t {
    fn clone(&self) -> xcb_xkb_key_type_t { *self }
}
#[repr(C)]
pub struct xcb_xkb_key_type_iterator_t {
    pub data : *mut xcb_xkb_key_type_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_xkb_key_sym_map_t {
     pub kt_index :    [u8; 4],
     pub groupInfo :   u8,
     pub width :       u8,
     pub nSyms :       u16
}

impl Copy for xcb_xkb_key_sym_map_t {}
impl Clone for xcb_xkb_key_sym_map_t {
    fn clone(&self) -> xcb_xkb_key_sym_map_t { *self }
}
#[repr(C)]
pub struct xcb_xkb_key_sym_map_iterator_t {
    pub data : *mut xcb_xkb_key_sym_map_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_xkb_common_behavior_t {
     pub type_ :   u8,
     pub data :    u8
}

impl Copy for xcb_xkb_common_behavior_t {}
impl Clone for xcb_xkb_common_behavior_t {
    fn clone(&self) -> xcb_xkb_common_behavior_t { *self }
}
#[repr(C)]
pub struct xcb_xkb_common_behavior_iterator_t {
    pub data : *mut xcb_xkb_common_behavior_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_xkb_default_behavior_t {
     pub type_ :   u8,
     pub pad0 :    u8
}

impl Copy for xcb_xkb_default_behavior_t {}
impl Clone for xcb_xkb_default_behavior_t {
    fn clone(&self) -> xcb_xkb_default_behavior_t { *self }
}
#[repr(C)]
pub struct xcb_xkb_default_behavior_iterator_t {
    pub data : *mut xcb_xkb_default_behavior_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_xkb_lock_behavior_t {
     pub type_ :   u8,
     pub pad0 :    u8
}

impl Copy for xcb_xkb_lock_behavior_t {}
impl Clone for xcb_xkb_lock_behavior_t {
    fn clone(&self) -> xcb_xkb_lock_behavior_t { *self }
}
#[repr(C)]
pub struct xcb_xkb_lock_behavior_iterator_t {
    pub data : *mut xcb_xkb_lock_behavior_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_xkb_radio_group_behavior_t {
     pub type_ :   u8,
     pub group :   u8
}

impl Copy for xcb_xkb_radio_group_behavior_t {}
impl Clone for xcb_xkb_radio_group_behavior_t {
    fn clone(&self) -> xcb_xkb_radio_group_behavior_t { *self }
}
#[repr(C)]
pub struct xcb_xkb_radio_group_behavior_iterator_t {
    pub data : *mut xcb_xkb_radio_group_behavior_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_xkb_overlay_1_behavior_t {
     pub type_ :   u8,
     pub key :     ffi::xproto::xcb_keycode_t
}

impl Copy for xcb_xkb_overlay_1_behavior_t {}
impl Clone for xcb_xkb_overlay_1_behavior_t {
    fn clone(&self) -> xcb_xkb_overlay_1_behavior_t { *self }
}
#[repr(C)]
pub struct xcb_xkb_overlay_1_behavior_iterator_t {
    pub data : *mut xcb_xkb_overlay_1_behavior_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_xkb_overlay_2_behavior_t {
     pub type_ :   u8,
     pub key :     u8
}

impl Copy for xcb_xkb_overlay_2_behavior_t {}
impl Clone for xcb_xkb_overlay_2_behavior_t {
    fn clone(&self) -> xcb_xkb_overlay_2_behavior_t { *self }
}
#[repr(C)]
pub struct xcb_xkb_overlay_2_behavior_iterator_t {
    pub data : *mut xcb_xkb_overlay_2_behavior_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_xkb_permament_lock_behavior_t {
     pub type_ :   u8,
     pub pad0 :    u8
}

impl Copy for xcb_xkb_permament_lock_behavior_t {}
impl Clone for xcb_xkb_permament_lock_behavior_t {
    fn clone(&self) -> xcb_xkb_permament_lock_behavior_t { *self }
}
#[repr(C)]
pub struct xcb_xkb_permament_lock_behavior_iterator_t {
    pub data : *mut xcb_xkb_permament_lock_behavior_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_xkb_permament_radio_group_behavior_t {
     pub type_ :   u8,
     pub group :   u8
}

impl Copy for xcb_xkb_permament_radio_group_behavior_t {}
impl Clone for xcb_xkb_permament_radio_group_behavior_t {
    fn clone(&self) -> xcb_xkb_permament_radio_group_behavior_t { *self }
}
#[repr(C)]
pub struct xcb_xkb_permament_radio_group_behavior_iterator_t {
    pub data : *mut xcb_xkb_permament_radio_group_behavior_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_xkb_permament_overlay_1_behavior_t {
     pub type_ :   u8,
     pub key :     ffi::xproto::xcb_keycode_t
}

impl Copy for xcb_xkb_permament_overlay_1_behavior_t {}
impl Clone for xcb_xkb_permament_overlay_1_behavior_t {
    fn clone(&self) -> xcb_xkb_permament_overlay_1_behavior_t { *self }
}
#[repr(C)]
pub struct xcb_xkb_permament_overlay_1_behavior_iterator_t {
    pub data : *mut xcb_xkb_permament_overlay_1_behavior_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_xkb_permament_overlay_2_behavior_t {
     pub type_ :   u8,
     pub key :     u8
}

impl Copy for xcb_xkb_permament_overlay_2_behavior_t {}
impl Clone for xcb_xkb_permament_overlay_2_behavior_t {
    fn clone(&self) -> xcb_xkb_permament_overlay_2_behavior_t { *self }
}
#[repr(C)]
pub struct xcb_xkb_permament_overlay_2_behavior_iterator_t {
    pub data : *mut xcb_xkb_permament_overlay_2_behavior_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_xkb_behavior_t {
    data : [u8; 2]
}
impl Copy for xcb_xkb_behavior_t {}
impl Clone for xcb_xkb_behavior_t {
    fn clone(&self) -> xcb_xkb_behavior_t { *self }
}
#[repr(C)]
pub struct xcb_xkb_behavior_iterator_t {
    pub data : *mut xcb_xkb_behavior_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_xkb_set_behavior_t {
     pub keycode :    ffi::xproto::xcb_keycode_t,
     pub behavior :   xcb_xkb_behavior_t,
     pub pad0 :       u8
}

impl Copy for xcb_xkb_set_behavior_t {}
impl Clone for xcb_xkb_set_behavior_t {
    fn clone(&self) -> xcb_xkb_set_behavior_t { *self }
}
#[repr(C)]
pub struct xcb_xkb_set_behavior_iterator_t {
    pub data : *mut xcb_xkb_set_behavior_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_xkb_set_explicit_t {
     pub keycode :    ffi::xproto::xcb_keycode_t,
     pub explicit :   u8
}

impl Copy for xcb_xkb_set_explicit_t {}
impl Clone for xcb_xkb_set_explicit_t {
    fn clone(&self) -> xcb_xkb_set_explicit_t { *self }
}
#[repr(C)]
pub struct xcb_xkb_set_explicit_iterator_t {
    pub data : *mut xcb_xkb_set_explicit_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_xkb_key_mod_map_t {
     pub keycode :   ffi::xproto::xcb_keycode_t,
     pub mods :      u8
}

impl Copy for xcb_xkb_key_mod_map_t {}
impl Clone for xcb_xkb_key_mod_map_t {
    fn clone(&self) -> xcb_xkb_key_mod_map_t { *self }
}
#[repr(C)]
pub struct xcb_xkb_key_mod_map_iterator_t {
    pub data : *mut xcb_xkb_key_mod_map_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_xkb_key_v_mod_map_t {
     pub keycode :   ffi::xproto::xcb_keycode_t,
     pub pad0 :      u8,
     pub vmods :     u16
}

impl Copy for xcb_xkb_key_v_mod_map_t {}
impl Clone for xcb_xkb_key_v_mod_map_t {
    fn clone(&self) -> xcb_xkb_key_v_mod_map_t { *self }
}
#[repr(C)]
pub struct xcb_xkb_key_v_mod_map_iterator_t {
    pub data : *mut xcb_xkb_key_v_mod_map_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_xkb_kt_set_map_entry_t {
     pub level :         u8,
     pub realMods :      u8,
     pub virtualMods :   u16
}

impl Copy for xcb_xkb_kt_set_map_entry_t {}
impl Clone for xcb_xkb_kt_set_map_entry_t {
    fn clone(&self) -> xcb_xkb_kt_set_map_entry_t { *self }
}
#[repr(C)]
pub struct xcb_xkb_kt_set_map_entry_iterator_t {
    pub data : *mut xcb_xkb_kt_set_map_entry_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_xkb_set_key_type_t {
     pub mask :          u8,
     pub realMods :      u8,
     pub virtualMods :   u16,
     pub numLevels :     u8,
     pub nMapEntries :   u8,
     pub preserve :      u8,
     pub pad0 :          u8
}

impl Copy for xcb_xkb_set_key_type_t {}
impl Clone for xcb_xkb_set_key_type_t {
    fn clone(&self) -> xcb_xkb_set_key_type_t { *self }
}
#[repr(C)]
pub struct xcb_xkb_set_key_type_iterator_t {
    pub data : *mut xcb_xkb_set_key_type_t,
    pub rem  : c_int,
    pub index: c_int
}


pub type xcb_xkb_string8_t = c_char;
#[repr(C)]
pub struct xcb_xkb_string8_iterator_t {
    pub data : *mut xcb_xkb_string8_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_xkb_property_t {
     pub nameLength :    u16,
     pub valueLength :   u16
}

impl Copy for xcb_xkb_property_t {}
impl Clone for xcb_xkb_property_t {
    fn clone(&self) -> xcb_xkb_property_t { *self }
}
#[repr(C)]
pub struct xcb_xkb_property_iterator_t {
    pub data : *mut xcb_xkb_property_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_xkb_outline_t {
     pub nPoints :        u8,
     pub cornerRadius :   u8,
     pub pad0 :           [u8; 2]
}

impl Copy for xcb_xkb_outline_t {}
impl Clone for xcb_xkb_outline_t {
    fn clone(&self) -> xcb_xkb_outline_t { *self }
}
#[repr(C)]
pub struct xcb_xkb_outline_iterator_t {
    pub data : *mut xcb_xkb_outline_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_xkb_shape_t {
     pub name :         ffi::xproto::xcb_atom_t,
     pub nOutlines :    u8,
     pub primaryNdx :   u8,
     pub approxNdx :    u8,
     pub pad0 :         u8
}

impl Copy for xcb_xkb_shape_t {}
impl Clone for xcb_xkb_shape_t {
    fn clone(&self) -> xcb_xkb_shape_t { *self }
}
#[repr(C)]
pub struct xcb_xkb_shape_iterator_t {
    pub data : *mut xcb_xkb_shape_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_xkb_key_t {
     pub name :       [xcb_xkb_string8_t; 4],
     pub gap :        i16,
     pub shapeNdx :   u8,
     pub colorNdx :   u8
}

impl Copy for xcb_xkb_key_t {}
impl Clone for xcb_xkb_key_t {
    fn clone(&self) -> xcb_xkb_key_t { *self }
}
#[repr(C)]
pub struct xcb_xkb_key_iterator_t {
    pub data : *mut xcb_xkb_key_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_xkb_overlay_key_t {
     pub over :    [xcb_xkb_string8_t; 4],
     pub under :   [xcb_xkb_string8_t; 4]
}

impl Copy for xcb_xkb_overlay_key_t {}
impl Clone for xcb_xkb_overlay_key_t {
    fn clone(&self) -> xcb_xkb_overlay_key_t { *self }
}
#[repr(C)]
pub struct xcb_xkb_overlay_key_iterator_t {
    pub data : *mut xcb_xkb_overlay_key_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_xkb_overlay_row_t {
     pub rowUnder :   u8,
     pub nKeys :      u8,
     pub pad0 :       [u8; 2]
}

impl Copy for xcb_xkb_overlay_row_t {}
impl Clone for xcb_xkb_overlay_row_t {
    fn clone(&self) -> xcb_xkb_overlay_row_t { *self }
}
#[repr(C)]
pub struct xcb_xkb_overlay_row_iterator_t {
    pub data : *mut xcb_xkb_overlay_row_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_xkb_overlay_t {
     pub name :    ffi::xproto::xcb_atom_t,
     pub nRows :   u8,
     pub pad0 :    [u8; 3]
}

impl Copy for xcb_xkb_overlay_t {}
impl Clone for xcb_xkb_overlay_t {
    fn clone(&self) -> xcb_xkb_overlay_t { *self }
}
#[repr(C)]
pub struct xcb_xkb_overlay_iterator_t {
    pub data : *mut xcb_xkb_overlay_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_xkb_row_t {
     pub top :        i16,
     pub left :       i16,
     pub nKeys :      u8,
     pub vertical :   u8,
     pub pad0 :       [u8; 2]
}

impl Copy for xcb_xkb_row_t {}
impl Clone for xcb_xkb_row_t {
    fn clone(&self) -> xcb_xkb_row_t { *self }
}
#[repr(C)]
pub struct xcb_xkb_row_iterator_t {
    pub data : *mut xcb_xkb_row_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_xkb_common_doodad_t {
     pub name :       ffi::xproto::xcb_atom_t,
     pub type_ :      u8,
     pub priority :   u8,
     pub top :        i16,
     pub left :       i16,
     pub angle :      i16
}

impl Copy for xcb_xkb_common_doodad_t {}
impl Clone for xcb_xkb_common_doodad_t {
    fn clone(&self) -> xcb_xkb_common_doodad_t { *self }
}
#[repr(C)]
pub struct xcb_xkb_common_doodad_iterator_t {
    pub data : *mut xcb_xkb_common_doodad_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_xkb_shape_doodad_t {
     pub name :       ffi::xproto::xcb_atom_t,
     pub type_ :      u8,
     pub priority :   u8,
     pub top :        i16,
     pub left :       i16,
     pub angle :      i16,
     pub colorNdx :   u8,
     pub shapeNdx :   u8,
     pub pad0 :       [u8; 6]
}

impl Copy for xcb_xkb_shape_doodad_t {}
impl Clone for xcb_xkb_shape_doodad_t {
    fn clone(&self) -> xcb_xkb_shape_doodad_t { *self }
}
#[repr(C)]
pub struct xcb_xkb_shape_doodad_iterator_t {
    pub data : *mut xcb_xkb_shape_doodad_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_xkb_text_doodad_t {
     pub name :       ffi::xproto::xcb_atom_t,
     pub type_ :      u8,
     pub priority :   u8,
     pub top :        i16,
     pub left :       i16,
     pub angle :      i16,
     pub width :      u16,
     pub height :     u16,
     pub colorNdx :   u8,
     pub pad0 :       [u8; 3]
}

impl Copy for xcb_xkb_text_doodad_t {}
impl Clone for xcb_xkb_text_doodad_t {
    fn clone(&self) -> xcb_xkb_text_doodad_t { *self }
}
#[repr(C)]
pub struct xcb_xkb_text_doodad_iterator_t {
    pub data : *mut xcb_xkb_text_doodad_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_xkb_indicator_doodad_t {
     pub name :          ffi::xproto::xcb_atom_t,
     pub type_ :         u8,
     pub priority :      u8,
     pub top :           i16,
     pub left :          i16,
     pub angle :         i16,
     pub shapeNdx :      u8,
     pub onColorNdx :    u8,
     pub offColorNdx :   u8,
     pub pad0 :          [u8; 5]
}

impl Copy for xcb_xkb_indicator_doodad_t {}
impl Clone for xcb_xkb_indicator_doodad_t {
    fn clone(&self) -> xcb_xkb_indicator_doodad_t { *self }
}
#[repr(C)]
pub struct xcb_xkb_indicator_doodad_iterator_t {
    pub data : *mut xcb_xkb_indicator_doodad_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_xkb_logo_doodad_t {
     pub name :       ffi::xproto::xcb_atom_t,
     pub type_ :      u8,
     pub priority :   u8,
     pub top :        i16,
     pub left :       i16,
     pub angle :      i16,
     pub colorNdx :   u8,
     pub shapeNdx :   u8,
     pub pad0 :       [u8; 6]
}

impl Copy for xcb_xkb_logo_doodad_t {}
impl Clone for xcb_xkb_logo_doodad_t {
    fn clone(&self) -> xcb_xkb_logo_doodad_t { *self }
}
#[repr(C)]
pub struct xcb_xkb_logo_doodad_iterator_t {
    pub data : *mut xcb_xkb_logo_doodad_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_xkb_doodad_t {
    data : [u8; 20]
}
impl Copy for xcb_xkb_doodad_t {}
impl Clone for xcb_xkb_doodad_t {
    fn clone(&self) -> xcb_xkb_doodad_t { *self }
}
#[repr(C)]
pub struct xcb_xkb_doodad_iterator_t {
    pub data : *mut xcb_xkb_doodad_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_xkb_section_t {
     pub name :        ffi::xproto::xcb_atom_t,
     pub top :         i16,
     pub left :        i16,
     pub width :       u16,
     pub height :      u16,
     pub angle :       i16,
     pub priority :    u8,
     pub nRows :       u8,
     pub nDoodads :    u8,
     pub nOverlays :   u8,
     pub pad0 :        [u8; 2]
}

impl Copy for xcb_xkb_section_t {}
impl Clone for xcb_xkb_section_t {
    fn clone(&self) -> xcb_xkb_section_t { *self }
}
#[repr(C)]
pub struct xcb_xkb_section_iterator_t {
    pub data : *mut xcb_xkb_section_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_xkb_listing_t {
     pub flags :    u16,
     pub length :   u16
}

impl Copy for xcb_xkb_listing_t {}
impl Clone for xcb_xkb_listing_t {
    fn clone(&self) -> xcb_xkb_listing_t { *self }
}
#[repr(C)]
pub struct xcb_xkb_listing_iterator_t {
    pub data : *mut xcb_xkb_listing_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_xkb_device_led_info_t {
     pub ledClass :         xcb_xkb_led_class_spec_t,
     pub ledID :            xcb_xkb_id_spec_t,
     pub namesPresent :     u32,
     pub mapsPresent :      u32,
     pub physIndicators :   u32,
     pub state :            u32
}

impl Copy for xcb_xkb_device_led_info_t {}
impl Clone for xcb_xkb_device_led_info_t {
    fn clone(&self) -> xcb_xkb_device_led_info_t { *self }
}
#[repr(C)]
pub struct xcb_xkb_device_led_info_iterator_t {
    pub data : *mut xcb_xkb_device_led_info_t,
    pub rem  : c_int,
    pub index: c_int
}



#[repr(C)]
pub struct xcb_xkb_keyboard_error_t {
     pub response_type :   u8,
     pub error_code :      u8,
     pub sequence :        u16,
     pub value :           u32,
     pub minorOpcode :     u16,
     pub majorOpcode :     u8,
     pub pad0 :            [u8; 21]
}

impl Copy for xcb_xkb_keyboard_error_t {}
impl Clone for xcb_xkb_keyboard_error_t {
    fn clone(&self) -> xcb_xkb_keyboard_error_t { *self }
}

#[repr(C)]
pub struct xcb_xkb_sa_no_action_t {
     pub type_ :   u8,
     pub pad0 :    [u8; 7]
}

impl Copy for xcb_xkb_sa_no_action_t {}
impl Clone for xcb_xkb_sa_no_action_t {
    fn clone(&self) -> xcb_xkb_sa_no_action_t { *self }
}
#[repr(C)]
pub struct xcb_xkb_sa_no_action_iterator_t {
    pub data : *mut xcb_xkb_sa_no_action_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_xkb_sa_set_mods_t {
     pub type_ :       u8,
     pub flags :       u8,
     pub mask :        u8,
     pub realMods :    u8,
     pub vmodsHigh :   u8,
     pub vmodsLow :    u8,
     pub pad0 :        [u8; 2]
}

impl Copy for xcb_xkb_sa_set_mods_t {}
impl Clone for xcb_xkb_sa_set_mods_t {
    fn clone(&self) -> xcb_xkb_sa_set_mods_t { *self }
}
#[repr(C)]
pub struct xcb_xkb_sa_set_mods_iterator_t {
    pub data : *mut xcb_xkb_sa_set_mods_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_xkb_sa_latch_mods_t {
     pub type_ :       u8,
     pub flags :       u8,
     pub mask :        u8,
     pub realMods :    u8,
     pub vmodsHigh :   u8,
     pub vmodsLow :    u8,
     pub pad0 :        [u8; 2]
}

impl Copy for xcb_xkb_sa_latch_mods_t {}
impl Clone for xcb_xkb_sa_latch_mods_t {
    fn clone(&self) -> xcb_xkb_sa_latch_mods_t { *self }
}
#[repr(C)]
pub struct xcb_xkb_sa_latch_mods_iterator_t {
    pub data : *mut xcb_xkb_sa_latch_mods_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_xkb_sa_lock_mods_t {
     pub type_ :       u8,
     pub flags :       u8,
     pub mask :        u8,
     pub realMods :    u8,
     pub vmodsHigh :   u8,
     pub vmodsLow :    u8,
     pub pad0 :        [u8; 2]
}

impl Copy for xcb_xkb_sa_lock_mods_t {}
impl Clone for xcb_xkb_sa_lock_mods_t {
    fn clone(&self) -> xcb_xkb_sa_lock_mods_t { *self }
}
#[repr(C)]
pub struct xcb_xkb_sa_lock_mods_iterator_t {
    pub data : *mut xcb_xkb_sa_lock_mods_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_xkb_sa_set_group_t {
     pub type_ :   u8,
     pub flags :   u8,
     pub group :   i8,
     pub pad0 :    [u8; 5]
}

impl Copy for xcb_xkb_sa_set_group_t {}
impl Clone for xcb_xkb_sa_set_group_t {
    fn clone(&self) -> xcb_xkb_sa_set_group_t { *self }
}
#[repr(C)]
pub struct xcb_xkb_sa_set_group_iterator_t {
    pub data : *mut xcb_xkb_sa_set_group_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_xkb_sa_latch_group_t {
     pub type_ :   u8,
     pub flags :   u8,
     pub group :   i8,
     pub pad0 :    [u8; 5]
}

impl Copy for xcb_xkb_sa_latch_group_t {}
impl Clone for xcb_xkb_sa_latch_group_t {
    fn clone(&self) -> xcb_xkb_sa_latch_group_t { *self }
}
#[repr(C)]
pub struct xcb_xkb_sa_latch_group_iterator_t {
    pub data : *mut xcb_xkb_sa_latch_group_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_xkb_sa_lock_group_t {
     pub type_ :   u8,
     pub flags :   u8,
     pub group :   i8,
     pub pad0 :    [u8; 5]
}

impl Copy for xcb_xkb_sa_lock_group_t {}
impl Clone for xcb_xkb_sa_lock_group_t {
    fn clone(&self) -> xcb_xkb_sa_lock_group_t { *self }
}
#[repr(C)]
pub struct xcb_xkb_sa_lock_group_iterator_t {
    pub data : *mut xcb_xkb_sa_lock_group_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_xkb_sa_move_ptr_t {
     pub type_ :   u8,
     pub flags :   u8,
     pub xHigh :   i8,
     pub xLow :    u8,
     pub yHigh :   i8,
     pub yLow :    u8,
     pub pad0 :    [u8; 2]
}

impl Copy for xcb_xkb_sa_move_ptr_t {}
impl Clone for xcb_xkb_sa_move_ptr_t {
    fn clone(&self) -> xcb_xkb_sa_move_ptr_t { *self }
}
#[repr(C)]
pub struct xcb_xkb_sa_move_ptr_iterator_t {
    pub data : *mut xcb_xkb_sa_move_ptr_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_xkb_sa_ptr_btn_t {
     pub type_ :    u8,
     pub flags :    u8,
     pub count :    u8,
     pub button :   u8,
     pub pad0 :     [u8; 4]
}

impl Copy for xcb_xkb_sa_ptr_btn_t {}
impl Clone for xcb_xkb_sa_ptr_btn_t {
    fn clone(&self) -> xcb_xkb_sa_ptr_btn_t { *self }
}
#[repr(C)]
pub struct xcb_xkb_sa_ptr_btn_iterator_t {
    pub data : *mut xcb_xkb_sa_ptr_btn_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_xkb_sa_lock_ptr_btn_t {
     pub type_ :    u8,
     pub flags :    u8,
     pub pad0 :     u8,
     pub button :   u8,
     pub pad1 :     [u8; 4]
}

impl Copy for xcb_xkb_sa_lock_ptr_btn_t {}
impl Clone for xcb_xkb_sa_lock_ptr_btn_t {
    fn clone(&self) -> xcb_xkb_sa_lock_ptr_btn_t { *self }
}
#[repr(C)]
pub struct xcb_xkb_sa_lock_ptr_btn_iterator_t {
    pub data : *mut xcb_xkb_sa_lock_ptr_btn_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_xkb_sa_set_ptr_dflt_t {
     pub type_ :    u8,
     pub flags :    u8,
     pub affect :   u8,
     pub value :    i8,
     pub pad0 :     [u8; 4]
}

impl Copy for xcb_xkb_sa_set_ptr_dflt_t {}
impl Clone for xcb_xkb_sa_set_ptr_dflt_t {
    fn clone(&self) -> xcb_xkb_sa_set_ptr_dflt_t { *self }
}
#[repr(C)]
pub struct xcb_xkb_sa_set_ptr_dflt_iterator_t {
    pub data : *mut xcb_xkb_sa_set_ptr_dflt_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_xkb_sa_iso_lock_t {
     pub type_ :       u8,
     pub flags :       u8,
     pub mask :        u8,
     pub realMods :    u8,
     pub group :       i8,
     pub affect :      u8,
     pub vmodsHigh :   u8,
     pub vmodsLow :    u8
}

impl Copy for xcb_xkb_sa_iso_lock_t {}
impl Clone for xcb_xkb_sa_iso_lock_t {
    fn clone(&self) -> xcb_xkb_sa_iso_lock_t { *self }
}
#[repr(C)]
pub struct xcb_xkb_sa_iso_lock_iterator_t {
    pub data : *mut xcb_xkb_sa_iso_lock_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_xkb_sa_terminate_t {
     pub type_ :   u8,
     pub pad0 :    [u8; 7]
}

impl Copy for xcb_xkb_sa_terminate_t {}
impl Clone for xcb_xkb_sa_terminate_t {
    fn clone(&self) -> xcb_xkb_sa_terminate_t { *self }
}
#[repr(C)]
pub struct xcb_xkb_sa_terminate_iterator_t {
    pub data : *mut xcb_xkb_sa_terminate_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_xkb_sa_switch_screen_t {
     pub type_ :       u8,
     pub flags :       u8,
     pub newScreen :   i8,
     pub pad0 :        [u8; 5]
}

impl Copy for xcb_xkb_sa_switch_screen_t {}
impl Clone for xcb_xkb_sa_switch_screen_t {
    fn clone(&self) -> xcb_xkb_sa_switch_screen_t { *self }
}
#[repr(C)]
pub struct xcb_xkb_sa_switch_screen_iterator_t {
    pub data : *mut xcb_xkb_sa_switch_screen_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_xkb_sa_set_controls_t {
     pub type_ :           u8,
     pub pad0 :            [u8; 3],
     pub boolCtrlsHigh :   u8,
     pub boolCtrlsLow :    u8,
     pub pad1 :            [u8; 2]
}

impl Copy for xcb_xkb_sa_set_controls_t {}
impl Clone for xcb_xkb_sa_set_controls_t {
    fn clone(&self) -> xcb_xkb_sa_set_controls_t { *self }
}
#[repr(C)]
pub struct xcb_xkb_sa_set_controls_iterator_t {
    pub data : *mut xcb_xkb_sa_set_controls_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_xkb_sa_lock_controls_t {
     pub type_ :           u8,
     pub pad0 :            [u8; 3],
     pub boolCtrlsHigh :   u8,
     pub boolCtrlsLow :    u8,
     pub pad1 :            [u8; 2]
}

impl Copy for xcb_xkb_sa_lock_controls_t {}
impl Clone for xcb_xkb_sa_lock_controls_t {
    fn clone(&self) -> xcb_xkb_sa_lock_controls_t { *self }
}
#[repr(C)]
pub struct xcb_xkb_sa_lock_controls_iterator_t {
    pub data : *mut xcb_xkb_sa_lock_controls_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_xkb_sa_action_message_t {
     pub type_ :     u8,
     pub flags :     u8,
     pub message :   [u8; 6]
}

impl Copy for xcb_xkb_sa_action_message_t {}
impl Clone for xcb_xkb_sa_action_message_t {
    fn clone(&self) -> xcb_xkb_sa_action_message_t { *self }
}
#[repr(C)]
pub struct xcb_xkb_sa_action_message_iterator_t {
    pub data : *mut xcb_xkb_sa_action_message_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_xkb_sa_redirect_key_t {
     pub type_ :           u8,
     pub newkey :          ffi::xproto::xcb_keycode_t,
     pub mask :            u8,
     pub realModifiers :   u8,
     pub vmodsMaskHigh :   u8,
     pub vmodsMaskLow :    u8,
     pub vmodsHigh :       u8,
     pub vmodsLow :        u8
}

impl Copy for xcb_xkb_sa_redirect_key_t {}
impl Clone for xcb_xkb_sa_redirect_key_t {
    fn clone(&self) -> xcb_xkb_sa_redirect_key_t { *self }
}
#[repr(C)]
pub struct xcb_xkb_sa_redirect_key_iterator_t {
    pub data : *mut xcb_xkb_sa_redirect_key_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_xkb_sa_device_btn_t {
     pub type_ :    u8,
     pub flags :    u8,
     pub count :    u8,
     pub button :   u8,
     pub device :   u8,
     pub pad0 :     [u8; 3]
}

impl Copy for xcb_xkb_sa_device_btn_t {}
impl Clone for xcb_xkb_sa_device_btn_t {
    fn clone(&self) -> xcb_xkb_sa_device_btn_t { *self }
}
#[repr(C)]
pub struct xcb_xkb_sa_device_btn_iterator_t {
    pub data : *mut xcb_xkb_sa_device_btn_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_xkb_sa_lock_device_btn_t {
     pub type_ :    u8,
     pub flags :    u8,
     pub pad0 :     u8,
     pub button :   u8,
     pub device :   u8
}

impl Copy for xcb_xkb_sa_lock_device_btn_t {}
impl Clone for xcb_xkb_sa_lock_device_btn_t {
    fn clone(&self) -> xcb_xkb_sa_lock_device_btn_t { *self }
}
#[repr(C)]
pub struct xcb_xkb_sa_lock_device_btn_iterator_t {
    pub data : *mut xcb_xkb_sa_lock_device_btn_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_xkb_sa_device_valuator_t {
     pub type_ :       u8,
     pub device :      u8,
     pub val1what :    u8,
     pub val1index :   u8,
     pub val1value :   u8,
     pub val2what :    u8,
     pub val2index :   u8,
     pub val2value :   u8
}

impl Copy for xcb_xkb_sa_device_valuator_t {}
impl Clone for xcb_xkb_sa_device_valuator_t {
    fn clone(&self) -> xcb_xkb_sa_device_valuator_t { *self }
}
#[repr(C)]
pub struct xcb_xkb_sa_device_valuator_iterator_t {
    pub data : *mut xcb_xkb_sa_device_valuator_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_xkb_action_t {
    data : [u8; 8]
}
impl Copy for xcb_xkb_action_t {}
impl Clone for xcb_xkb_action_t {
    fn clone(&self) -> xcb_xkb_action_t { *self }
}
#[repr(C)]
pub struct xcb_xkb_action_iterator_t {
    pub data : *mut xcb_xkb_action_t,
    pub rem  : c_int,
    pub index: c_int
}


#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xkb_use_extension_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_xkb_use_extension_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub wantedMajor :    u16,
     pub wantedMinor :    u16
}

impl Copy for xcb_xkb_use_extension_request_t {}
impl Clone for xcb_xkb_use_extension_request_t {
    fn clone(&self) -> xcb_xkb_use_extension_request_t { *self }
}

#[repr(C)]
pub struct xcb_xkb_use_extension_reply_t {
     pub response_type :   u8,
     pub supported :       u8,
     pub sequence :        u16,
     pub length :          u32,
     pub serverMajor :     u16,
     pub serverMinor :     u16,
     pub pad0 :            [u8; 20]
}

impl Copy for xcb_xkb_use_extension_reply_t {}
impl Clone for xcb_xkb_use_extension_reply_t {
    fn clone(&self) -> xcb_xkb_use_extension_reply_t { *self }
}

#[repr(C)]
pub struct xcb_xkb_select_events_details_t {
     pub affectNewKeyboard :       u16,
     pub newKeyboardDetails :      u16,
     pub affectState :             u16,
     pub stateDetails :            u16,
     pub affectCtrls :             u32,
     pub ctrlDetails :             u32,
     pub affectIndicatorState :    u32,
     pub indicatorStateDetails :   u32,
     pub affectIndicatorMap :      u32,
     pub indicatorMapDetails :     u32,
     pub affectNames :             u16,
     pub namesDetails :            u16,
     pub affectCompat :            u8,
     pub compatDetails :           u8,
     pub affectBell :              u8,
     pub bellDetails :             u8,
     pub affectMsgDetails :        u8,
     pub msgDetails :              u8,
     pub affectAccessX :           u16,
     pub accessXDetails :          u16,
     pub affectExtDev :            u16
     pub extdevDetails :           u16
}

impl Copy for xcb_xkb_select_events_details_t {}
impl Clone for xcb_xkb_select_events_details_t {
    fn clone(&self) -> xcb_xkb_select_events_details_t { *self }
}


#[repr(C)]
pub struct xcb_xkb_select_events_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub deviceSpec :     xcb_xkb_device_spec_t,
     pub affectWhich :    u16,
     pub clear :          u16,
     pub selectAll :      u16,
     pub affectMap :      u16,
     pub map :            u16
}

impl Copy for xcb_xkb_select_events_request_t {}
impl Clone for xcb_xkb_select_events_request_t {
    fn clone(&self) -> xcb_xkb_select_events_request_t { *self }
}


#[repr(C)]
pub struct xcb_xkb_bell_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub deviceSpec :     xcb_xkb_device_spec_t,
     pub bellClass :      xcb_xkb_bell_class_spec_t,
     pub bellID :         xcb_xkb_id_spec_t,
     pub percent :        i8,
     pub forceSound :     u8,
     pub eventOnly :      u8,
     pub pad0 :           u8,
     pub pitch :          i16,
     pub duration :       i16,
     pub pad1 :           [u8; 2],
     pub name :           ffi::xproto::xcb_atom_t,
     pub window :         ffi::xproto::xcb_window_t
}

impl Copy for xcb_xkb_bell_request_t {}
impl Clone for xcb_xkb_bell_request_t {
    fn clone(&self) -> xcb_xkb_bell_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xkb_get_state_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_xkb_get_state_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub deviceSpec :     xcb_xkb_device_spec_t,
     pub pad0 :           [u8; 2]
}

impl Copy for xcb_xkb_get_state_request_t {}
impl Clone for xcb_xkb_get_state_request_t {
    fn clone(&self) -> xcb_xkb_get_state_request_t { *self }
}

#[repr(C)]
pub struct xcb_xkb_get_state_reply_t {
     pub response_type :      u8,
     pub deviceID :           u8,
     pub sequence :           u16,
     pub length :             u32,
     pub mods :               u8,
     pub baseMods :           u8,
     pub latchedMods :        u8,
     pub lockedMods :         u8,
     pub group :              u8,
     pub lockedGroup :        u8,
     pub baseGroup :          i16,
     pub latchedGroup :       i16,
     pub compatState :        u8,
     pub grabMods :           u8,
     pub compatGrabMods :     u8,
     pub lookupMods :         u8,
     pub compatLookupMods :   u8,
     pub pad0 :               u8,
     pub ptrBtnState :        u16,
     pub pad1 :               [u8; 6]
}

impl Copy for xcb_xkb_get_state_reply_t {}
impl Clone for xcb_xkb_get_state_reply_t {
    fn clone(&self) -> xcb_xkb_get_state_reply_t { *self }
}


#[repr(C)]
pub struct xcb_xkb_latch_lock_state_request_t {
     pub major_opcode :       u8,
     pub minor_opcode :       u8,
     pub length :             u16,
     pub deviceSpec :         xcb_xkb_device_spec_t,
     pub affectModLocks :     u8,
     pub modLocks :           u8,
     pub lockGroup :          u8,
     pub groupLock :          u8,
     pub affectModLatches :   u8,
     pub pad0 :               u8,
     pub latchGroup :         u8,
     pub groupLatch :         u16
}

impl Copy for xcb_xkb_latch_lock_state_request_t {}
impl Clone for xcb_xkb_latch_lock_state_request_t {
    fn clone(&self) -> xcb_xkb_latch_lock_state_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xkb_get_controls_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_xkb_get_controls_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub deviceSpec :     xcb_xkb_device_spec_t,
     pub pad0 :           [u8; 2]
}

impl Copy for xcb_xkb_get_controls_request_t {}
impl Clone for xcb_xkb_get_controls_request_t {
    fn clone(&self) -> xcb_xkb_get_controls_request_t { *self }
}

#[repr(C)]
pub struct xcb_xkb_get_controls_reply_t {
     pub response_type :                 u8,
     pub deviceID :                      u8,
     pub sequence :                      u16,
     pub length :                        u32,
     pub mouseKeysDfltBtn :              u8,
     pub numGroups :                     u8,
     pub groupsWrap :                    u8,
     pub internalModsMask :              u8,
     pub ignoreLockModsMask :            u8,
     pub internalModsRealMods :          u8,
     pub ignoreLockModsRealMods :        u8,
     pub pad0 :                          u8,
     pub internalModsVmods :             u16,
     pub ignoreLockModsVmods :           u16,
     pub repeatDelay :                   u16,
     pub repeatInterval :                u16,
     pub slowKeysDelay :                 u16,
     pub debounceDelay :                 u16,
     pub mouseKeysDelay :                u16,
     pub mouseKeysInterval :             u16,
     pub mouseKeysTimeToMax :            u16,
     pub mouseKeysMaxSpeed :             u16,
     pub mouseKeysCurve :                i16,
     pub accessXOption :                 xcb_xkb_ax_option_t,
     pub accessXTimeout :                u16,
     pub accessXTimeoutOptionsMask :     xcb_xkb_ax_option_t,
     pub accessXTimeoutOptionsValues :   xcb_xkb_ax_option_t,
     pub pad1 :                          [u8; 2],
     pub accessXTimeoutMask :            u32,
     pub accessXTimeoutValues :          u32,
     pub enabledControls :               u32,
     pub perKeyRepeat :                  [u8; 32]
}

impl Copy for xcb_xkb_get_controls_reply_t {}
impl Clone for xcb_xkb_get_controls_reply_t {
    fn clone(&self) -> xcb_xkb_get_controls_reply_t { *self }
}


#[repr(C)]
pub struct xcb_xkb_set_controls_request_t {
     pub major_opcode :                  u8,
     pub minor_opcode :                  u8,
     pub length :                        u16,
     pub deviceSpec :                    xcb_xkb_device_spec_t,
     pub affectInternalRealMods :        u8,
     pub internalRealMods :              u8,
     pub affectIgnoreLockRealMods :      u8,
     pub ignoreLockRealMods :            u8,
     pub affectInternalVirtualMods :     u16,
     pub internalVirtualMods :           u16,
     pub affectIgnoreLockVirtualMods :   u16,
     pub ignoreLockVirtualMods :         u16,
     pub mouseKeysDfltBtn :              u8,
     pub groupsWrap :                    u8,
     pub accessXOptions :                xcb_xkb_ax_option_t,
     pub pad0 :                          [u8; 2],
     pub affectEnabledControls :         u32,
     pub enabledControls :               u32,
     pub changeControls :                u32,
     pub repeatDelay :                   u16,
     pub repeatInterval :                u16,
     pub slowKeysDelay :                 u16,
     pub debounceDelay :                 u16,
     pub mouseKeysDelay :                u16,
     pub mouseKeysInterval :             u16,
     pub mouseKeysTimeToMax :            u16,
     pub mouseKeysMaxSpeed :             u16,
     pub mouseKeysCurve :                i16,
     pub accessXTimeout :                u16,
     pub accessXTimeoutMask :            u32,
     pub accessXTimeoutValues :          u32,
     pub accessXTimeoutOptionsMask :     xcb_xkb_ax_option_t,
     pub accessXTimeoutOptionsValues :   xcb_xkb_ax_option_t,
     pub perKeyRepeat :                  [u8; 32]
}

impl Copy for xcb_xkb_set_controls_request_t {}
impl Clone for xcb_xkb_set_controls_request_t {
    fn clone(&self) -> xcb_xkb_set_controls_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xkb_get_map_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_xkb_get_map_request_t {
     pub major_opcode :       u8,
     pub minor_opcode :       u8,
     pub length :             u16,
     pub deviceSpec :         xcb_xkb_device_spec_t,
     pub full :               u16,
     pub partial :            u16,
     pub firstType :          u8,
     pub nTypes :             u8,
     pub firstKeySym :        ffi::xproto::xcb_keycode_t,
     pub nKeySyms :           u8,
     pub firstKeyAction :     ffi::xproto::xcb_keycode_t,
     pub nKeyActions :        u8,
     pub firstKeyBehavior :   ffi::xproto::xcb_keycode_t,
     pub nKeyBehaviors :      u8,
     pub virtualMods :        u16,
     pub firstKeyExplicit :   ffi::xproto::xcb_keycode_t,
     pub nKeyExplicit :       u8,
     pub firstModMapKey :     ffi::xproto::xcb_keycode_t,
     pub nModMapKeys :        u8,
     pub firstVModMapKey :    ffi::xproto::xcb_keycode_t,
     pub nVModMapKeys :       u8,
     pub pad0 :               [u8; 2]
}

impl Copy for xcb_xkb_get_map_request_t {}
impl Clone for xcb_xkb_get_map_request_t {
    fn clone(&self) -> xcb_xkb_get_map_request_t { *self }
}

#[repr(C)]
pub struct xcb_xkb_get_map_map_t {
    pub types_rtrn :   *mut xcb_xkb_key_type_t,
    pub syms_rtrn :   *mut xcb_xkb_key_sym_map_t,
    pub acts_rtrn_count :               *mut u8,
    pub acts_rtrn_acts :   *mut xcb_xkb_action_t,
    pub behaviors_rtrn :   *mut xcb_xkb_set_behavior_t,
    pub vmods_rtrn :               *mut u8,
    pub explicit_rtrn :   *mut xcb_xkb_set_explicit_t,
    pub modmap_rtrn :   *mut xcb_xkb_key_mod_map_t,
    pub vmodmap_rtrn :   *mut xcb_xkb_key_v_mod_map_t
}

impl Copy for xcb_xkb_get_map_map_t {}
impl Clone for xcb_xkb_get_map_map_t {
    fn clone(&self) -> xcb_xkb_get_map_map_t { *self }
}

#[repr(C)]
pub struct xcb_xkb_get_map_reply_t {
     pub response_type :       u8,
     pub deviceID :            u8,
     pub sequence :            u16,
     pub length :              u32,
     pub pad0 :                [u8; 2],
     pub minKeyCode :          ffi::xproto::xcb_keycode_t,
     pub maxKeyCode :          ffi::xproto::xcb_keycode_t,
     pub present :             u16,
     pub firstType :           u8,
     pub nTypes :              u8,
     pub totalTypes :          u8,
     pub firstKeySym :         ffi::xproto::xcb_keycode_t,
     pub totalSyms :           u16,
     pub nKeySyms :            u8,
     pub firstKeyAction :      ffi::xproto::xcb_keycode_t,
     pub totalActions :        u16,
     pub nKeyActions :         u8,
     pub firstKeyBehavior :    ffi::xproto::xcb_keycode_t,
     pub nKeyBehaviors :       u8,
     pub totalKeyBehaviors :   u8,
     pub firstKeyExplicit :    ffi::xproto::xcb_keycode_t,
     pub nKeyExplicit :        u8,
     pub totalKeyExplicit :    u8,
     pub firstModMapKey :      ffi::xproto::xcb_keycode_t,
     pub nModMapKeys :         u8,
     pub totalModMapKeys :     u8,
     pub firstVModMapKey :     ffi::xproto::xcb_keycode_t,
     pub nVModMapKeys :        u8,
     pub totalVModMapKeys :    u8,
     pub pad1 :                u8,
     pub virtualMods :         u16
}

impl Copy for xcb_xkb_get_map_reply_t {}
impl Clone for xcb_xkb_get_map_reply_t {
    fn clone(&self) -> xcb_xkb_get_map_reply_t { *self }
}

#[repr(C)]
pub struct xcb_xkb_set_map_values_t {
    pub types :   *mut xcb_xkb_set_key_type_t,
    pub syms :   *mut xcb_xkb_key_sym_map_t,
    pub actionsCount :            *mut u8,
    pub actions :   *mut xcb_xkb_action_t,
    pub behaviors :   *mut xcb_xkb_set_behavior_t,
    pub vmods :            *mut u8,
    pub explicit :   *mut xcb_xkb_set_explicit_t,
    pub modmap :   *mut xcb_xkb_key_mod_map_t,
    pub vmodmap :   *mut xcb_xkb_key_v_mod_map_t
}

impl Copy for xcb_xkb_set_map_values_t {}
impl Clone for xcb_xkb_set_map_values_t {
    fn clone(&self) -> xcb_xkb_set_map_values_t { *self }
}


#[repr(C)]
pub struct xcb_xkb_set_map_request_t {
     pub major_opcode :        u8,
     pub minor_opcode :        u8,
     pub length :              u16,
     pub deviceSpec :          xcb_xkb_device_spec_t,
     pub present :             u16,
     pub flags :               u16,
     pub minKeyCode :          ffi::xproto::xcb_keycode_t,
     pub maxKeyCode :          ffi::xproto::xcb_keycode_t,
     pub firstType :           u8,
     pub nTypes :              u8,
     pub firstKeySym :         ffi::xproto::xcb_keycode_t,
     pub nKeySyms :            u8,
     pub totalSyms :           u16,
     pub firstKeyAction :      ffi::xproto::xcb_keycode_t,
     pub nKeyActions :         u8,
     pub totalActions :        u16,
     pub firstKeyBehavior :    ffi::xproto::xcb_keycode_t,
     pub nKeyBehaviors :       u8,
     pub totalKeyBehaviors :   u8,
     pub firstKeyExplicit :    ffi::xproto::xcb_keycode_t,
     pub nKeyExplicit :        u8,
     pub totalKeyExplicit :    u8,
     pub firstModMapKey :      ffi::xproto::xcb_keycode_t,
     pub nModMapKeys :         u8,
     pub totalModMapKeys :     u8,
     pub firstVModMapKey :     ffi::xproto::xcb_keycode_t,
     pub nVModMapKeys :        u8,
     pub totalVModMapKeys :    u8,
     pub virtualMods :         u16
}

impl Copy for xcb_xkb_set_map_request_t {}
impl Clone for xcb_xkb_set_map_request_t {
    fn clone(&self) -> xcb_xkb_set_map_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xkb_get_compat_map_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_xkb_get_compat_map_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub deviceSpec :     xcb_xkb_device_spec_t,
     pub groups :         u8,
     pub getAllSI :       u8,
     pub firstSI :        u16,
     pub nSI :            u16
}

impl Copy for xcb_xkb_get_compat_map_request_t {}
impl Clone for xcb_xkb_get_compat_map_request_t {
    fn clone(&self) -> xcb_xkb_get_compat_map_request_t { *self }
}

#[repr(C)]
pub struct xcb_xkb_get_compat_map_reply_t {
     pub response_type :   u8,
     pub deviceID :        u8,
     pub sequence :        u16,
     pub length :          u32,
     pub groupsRtrn :      u8,
     pub pad0 :            u8,
     pub firstSIRtrn :     u16,
     pub nSIRtrn :         u16,
     pub nTotalSI :        u16,
     pub pad1 :            [u8; 16]
}

impl Copy for xcb_xkb_get_compat_map_reply_t {}
impl Clone for xcb_xkb_get_compat_map_reply_t {
    fn clone(&self) -> xcb_xkb_get_compat_map_reply_t { *self }
}


#[repr(C)]
pub struct xcb_xkb_set_compat_map_request_t {
     pub major_opcode :       u8,
     pub minor_opcode :       u8,
     pub length :             u16,
     pub deviceSpec :         xcb_xkb_device_spec_t,
     pub pad0 :               u8,
     pub recomputeActions :   u8,
     pub truncateSI :         u8,
     pub groups :             u8,
     pub firstSI :            u16,
     pub nSI :                u16,
     pub pad1 :               [u8; 2]
}

impl Copy for xcb_xkb_set_compat_map_request_t {}
impl Clone for xcb_xkb_set_compat_map_request_t {
    fn clone(&self) -> xcb_xkb_set_compat_map_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xkb_get_indicator_state_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_xkb_get_indicator_state_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub deviceSpec :     xcb_xkb_device_spec_t,
     pub pad0 :           [u8; 2]
}

impl Copy for xcb_xkb_get_indicator_state_request_t {}
impl Clone for xcb_xkb_get_indicator_state_request_t {
    fn clone(&self) -> xcb_xkb_get_indicator_state_request_t { *self }
}

#[repr(C)]
pub struct xcb_xkb_get_indicator_state_reply_t {
     pub response_type :   u8,
     pub deviceID :        u8,
     pub sequence :        u16,
     pub length :          u32,
     pub state :           u32,
     pub pad0 :            [u8; 20]
}

impl Copy for xcb_xkb_get_indicator_state_reply_t {}
impl Clone for xcb_xkb_get_indicator_state_reply_t {
    fn clone(&self) -> xcb_xkb_get_indicator_state_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xkb_get_indicator_map_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_xkb_get_indicator_map_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub deviceSpec :     xcb_xkb_device_spec_t,
     pub pad0 :           [u8; 2],
     pub which :          u32
}

impl Copy for xcb_xkb_get_indicator_map_request_t {}
impl Clone for xcb_xkb_get_indicator_map_request_t {
    fn clone(&self) -> xcb_xkb_get_indicator_map_request_t { *self }
}

#[repr(C)]
pub struct xcb_xkb_get_indicator_map_reply_t {
     pub response_type :    u8,
     pub deviceID :         u8,
     pub sequence :         u16,
     pub length :           u32,
     pub which :            u32,
     pub realIndicators :   u32,
     pub nIndicators :      u8,
     pub pad0 :             [u8; 15]
}

impl Copy for xcb_xkb_get_indicator_map_reply_t {}
impl Clone for xcb_xkb_get_indicator_map_reply_t {
    fn clone(&self) -> xcb_xkb_get_indicator_map_reply_t { *self }
}


#[repr(C)]
pub struct xcb_xkb_set_indicator_map_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub deviceSpec :     xcb_xkb_device_spec_t,
     pub pad0 :           [u8; 2],
     pub which :          u32
}

impl Copy for xcb_xkb_set_indicator_map_request_t {}
impl Clone for xcb_xkb_set_indicator_map_request_t {
    fn clone(&self) -> xcb_xkb_set_indicator_map_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xkb_get_named_indicator_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_xkb_get_named_indicator_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub deviceSpec :     xcb_xkb_device_spec_t,
     pub ledClass :       xcb_xkb_led_class_spec_t,
     pub ledID :          xcb_xkb_id_spec_t,
     pub pad0 :           [u8; 2],
     pub indicator :      ffi::xproto::xcb_atom_t
}

impl Copy for xcb_xkb_get_named_indicator_request_t {}
impl Clone for xcb_xkb_get_named_indicator_request_t {
    fn clone(&self) -> xcb_xkb_get_named_indicator_request_t { *self }
}

#[repr(C)]
pub struct xcb_xkb_get_named_indicator_reply_t {
     pub response_type :     u8,
     pub deviceID :          u8,
     pub sequence :          u16,
     pub length :            u32,
     pub indicator :         ffi::xproto::xcb_atom_t,
     pub found :             u8,
     pub on :                u8,
     pub realIndicator :     u8,
     pub ndx :               u8,
     pub map_flags :         u8,
     pub map_whichGroups :   u8,
     pub map_groups :        u8,
     pub map_whichMods :     u8,
     pub map_mods :          u8,
     pub map_realMods :      u8,
     pub map_vmod :          u16,
     pub map_ctrls :         u32,
     pub pad0 :              [u8; 3]
}

impl Copy for xcb_xkb_get_named_indicator_reply_t {}
impl Clone for xcb_xkb_get_named_indicator_reply_t {
    fn clone(&self) -> xcb_xkb_get_named_indicator_reply_t { *self }
}


#[repr(C)]
pub struct xcb_xkb_set_named_indicator_request_t {
     pub major_opcode :      u8,
     pub minor_opcode :      u8,
     pub length :            u16,
     pub deviceSpec :        xcb_xkb_device_spec_t,
     pub ledClass :          xcb_xkb_led_class_spec_t,
     pub ledID :             xcb_xkb_id_spec_t,
     pub pad0 :              [u8; 2],
     pub indicator :         ffi::xproto::xcb_atom_t,
     pub setState :          u8,
     pub on :                u8,
     pub setMap :            u8,
     pub createMap :         u8,
     pub pad1 :              u8,
     pub map_flags :         u8,
     pub map_whichGroups :   u8,
     pub map_groups :        u8,
     pub map_whichMods :     u8,
     pub map_realMods :      u8,
     pub map_vmods :         u16,
     pub map_ctrls :         u32
}

impl Copy for xcb_xkb_set_named_indicator_request_t {}
impl Clone for xcb_xkb_set_named_indicator_request_t {
    fn clone(&self) -> xcb_xkb_set_named_indicator_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xkb_get_names_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_xkb_get_names_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub deviceSpec :     xcb_xkb_device_spec_t,
     pub pad0 :           [u8; 2],
     pub which :          u32
}

impl Copy for xcb_xkb_get_names_request_t {}
impl Clone for xcb_xkb_get_names_request_t {
    fn clone(&self) -> xcb_xkb_get_names_request_t { *self }
}

#[repr(C)]
pub struct xcb_xkb_get_names_value_list_t {
     pub keycodesName :      ffi::xproto::xcb_atom_t,
     pub geometryName :      ffi::xproto::xcb_atom_t,
     pub symbolsName :       ffi::xproto::xcb_atom_t,
     pub physSymbolsName :   ffi::xproto::xcb_atom_t,
     pub typesName :         ffi::xproto::xcb_atom_t,
     pub compatName :        ffi::xproto::xcb_atom_t,
    pub typeNames :   *mut ffi::xproto::xcb_atom_t,
    pub nLevelsPerType :               *mut u8,
    pub ktLevelNames :   *mut ffi::xproto::xcb_atom_t,
    pub indicatorNames :   *mut ffi::xproto::xcb_atom_t,
    pub virtualModNames :   *mut ffi::xproto::xcb_atom_t,
    pub groups :   *mut ffi::xproto::xcb_atom_t,
    pub keyNames :   *mut xcb_xkb_key_name_t,
    pub keyAliases :   *mut xcb_xkb_key_alias_t,
    pub radioGroupNames :   *mut ffi::xproto::xcb_atom_t
}

impl Copy for xcb_xkb_get_names_value_list_t {}
impl Clone for xcb_xkb_get_names_value_list_t {
    fn clone(&self) -> xcb_xkb_get_names_value_list_t { *self }
}

#[repr(C)]
pub struct xcb_xkb_get_names_reply_t {
     pub response_type :   u8,
     pub deviceID :        u8,
     pub sequence :        u16,
     pub length :          u32,
     pub which :           u32,
     pub minKeyCode :      ffi::xproto::xcb_keycode_t,
     pub maxKeyCode :      ffi::xproto::xcb_keycode_t,
     pub nTypes :          u8,
     pub groupNames :      u8,
     pub virtualMods :     u16,
     pub firstKey :        ffi::xproto::xcb_keycode_t,
     pub nKeys :           u8,
     pub indicators :      u32,
     pub nRadioGroups :    u8,
     pub nKeyAliases :     u8,
     pub nKTLevels :       u16,
     pub pad0 :            [u8; 4]
}

impl Copy for xcb_xkb_get_names_reply_t {}
impl Clone for xcb_xkb_get_names_reply_t {
    fn clone(&self) -> xcb_xkb_get_names_reply_t { *self }
}

#[repr(C)]
pub struct xcb_xkb_set_names_values_t {
     pub keycodesName :      ffi::xproto::xcb_atom_t,
     pub geometryName :      ffi::xproto::xcb_atom_t,
     pub symbolsName :       ffi::xproto::xcb_atom_t,
     pub physSymbolsName :   ffi::xproto::xcb_atom_t,
     pub typesName :         ffi::xproto::xcb_atom_t,
     pub compatName :        ffi::xproto::xcb_atom_t,
    pub typeNames :   *mut ffi::xproto::xcb_atom_t,
    pub nLevelsPerType :               *mut u8,
    pub ktLevelNames :   *mut ffi::xproto::xcb_atom_t,
    pub indicatorNames :   *mut ffi::xproto::xcb_atom_t,
    pub virtualModNames :   *mut ffi::xproto::xcb_atom_t,
    pub groups :   *mut ffi::xproto::xcb_atom_t,
    pub keyNames :   *mut xcb_xkb_key_name_t,
    pub keyAliases :   *mut xcb_xkb_key_alias_t,
    pub radioGroupNames :   *mut ffi::xproto::xcb_atom_t
}

impl Copy for xcb_xkb_set_names_values_t {}
impl Clone for xcb_xkb_set_names_values_t {
    fn clone(&self) -> xcb_xkb_set_names_values_t { *self }
}


#[repr(C)]
pub struct xcb_xkb_set_names_request_t {
     pub major_opcode :        u8,
     pub minor_opcode :        u8,
     pub length :              u16,
     pub deviceSpec :          xcb_xkb_device_spec_t,
     pub virtualMods :         u16,
     pub which :               u32,
     pub firstType :           u8,
     pub nTypes :              u8,
     pub firstKTLevelt :       u8,
     pub nKTLevels :           u8,
     pub indicators :          u32,
     pub groupNames :          u8,
     pub nRadioGroups :        u8,
     pub firstKey :            ffi::xproto::xcb_keycode_t,
     pub nKeys :               u8,
     pub nKeyAliases :         u8,
     pub pad0 :                u8,
     pub totalKTLevelNames :   u16
}

impl Copy for xcb_xkb_set_names_request_t {}
impl Clone for xcb_xkb_set_names_request_t {
    fn clone(&self) -> xcb_xkb_set_names_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xkb_get_geometry_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_xkb_get_geometry_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub deviceSpec :     xcb_xkb_device_spec_t,
     pub pad0 :           [u8; 2],
     pub name :           ffi::xproto::xcb_atom_t
}

impl Copy for xcb_xkb_get_geometry_request_t {}
impl Clone for xcb_xkb_get_geometry_request_t {
    fn clone(&self) -> xcb_xkb_get_geometry_request_t { *self }
}

#[repr(C)]
pub struct xcb_xkb_get_geometry_reply_t {
     pub response_type :   u8,
     pub deviceID :        u8,
     pub sequence :        u16,
     pub length :          u32,
     pub name :            ffi::xproto::xcb_atom_t,
     pub found :           u8,
     pub pad0 :            u8,
     pub widthMM :         u16,
     pub heightMM :        u16,
     pub nProperties :     u16,
     pub nColors :         u16,
     pub nShapes :         u16,
     pub nSections :       u16,
     pub nDoodads :        u16,
     pub nKeyAliases :     u16,
     pub baseColorNdx :    u8,
     pub labelColorNdx :   u8
}

impl Copy for xcb_xkb_get_geometry_reply_t {}
impl Clone for xcb_xkb_get_geometry_reply_t {
    fn clone(&self) -> xcb_xkb_get_geometry_reply_t { *self }
}


#[repr(C)]
pub struct xcb_xkb_set_geometry_request_t {
     pub major_opcode :    u8,
     pub minor_opcode :    u8,
     pub length :          u16,
     pub deviceSpec :      xcb_xkb_device_spec_t,
     pub nShapes :         u8,
     pub nSections :       u8,
     pub name :            ffi::xproto::xcb_atom_t,
     pub widthMM :         u16,
     pub heightMM :        u16,
     pub nProperties :     u16,
     pub nColors :         u16,
     pub nDoodads :        u16,
     pub nKeyAliases :     u16,
     pub baseColorNdx :    u8,
     pub labelColorNdx :   u8,
     pub pad0 :            [u8; 2]
}

impl Copy for xcb_xkb_set_geometry_request_t {}
impl Clone for xcb_xkb_set_geometry_request_t {
    fn clone(&self) -> xcb_xkb_set_geometry_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xkb_per_client_flags_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_xkb_per_client_flags_request_t {
     pub major_opcode :      u8,
     pub minor_opcode :      u8,
     pub length :            u16,
     pub deviceSpec :        xcb_xkb_device_spec_t,
     pub pad0 :              [u8; 2],
     pub change :            u32,
     pub value :             u32,
     pub ctrlsToChange :     u32,
     pub autoCtrls :         u32,
     pub autoCtrlsValues :   u32
}

impl Copy for xcb_xkb_per_client_flags_request_t {}
impl Clone for xcb_xkb_per_client_flags_request_t {
    fn clone(&self) -> xcb_xkb_per_client_flags_request_t { *self }
}

#[repr(C)]
pub struct xcb_xkb_per_client_flags_reply_t {
     pub response_type :     u8,
     pub deviceID :          u8,
     pub sequence :          u16,
     pub length :            u32,
     pub supported :         u32,
     pub value :             u32,
     pub autoCtrls :         u32,
     pub autoCtrlsValues :   u32,
     pub pad0 :              [u8; 8]
}

impl Copy for xcb_xkb_per_client_flags_reply_t {}
impl Clone for xcb_xkb_per_client_flags_reply_t {
    fn clone(&self) -> xcb_xkb_per_client_flags_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xkb_list_components_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_xkb_list_components_request_t {
     pub major_opcode :       u8,
     pub minor_opcode :       u8,
     pub length :             u16,
     pub deviceSpec :         xcb_xkb_device_spec_t,
     pub maxNames :           u16,
     pub keymapsSpecLen :     u8,
     pub keycodesSpecLen :    u8,
     pub typesSpecLen :       u8,
     pub compatMapSpecLen :   u8,
     pub symbolsSpecLen :     u8,
     pub geometrySpecLen :    u8
}

impl Copy for xcb_xkb_list_components_request_t {}
impl Clone for xcb_xkb_list_components_request_t {
    fn clone(&self) -> xcb_xkb_list_components_request_t { *self }
}

#[repr(C)]
pub struct xcb_xkb_list_components_reply_t {
     pub response_type :   u8,
     pub deviceID :        u8,
     pub sequence :        u16,
     pub length :          u32,
     pub nKeymaps :        u16,
     pub nKeycodes :       u16,
     pub nTypes :          u16,
     pub nCompatMaps :     u16,
     pub nSymbols :        u16,
     pub nGeometries :     u16,
     pub extra :           u16,
     pub pad0 :            [u8; 10]
}

impl Copy for xcb_xkb_list_components_reply_t {}
impl Clone for xcb_xkb_list_components_reply_t {
    fn clone(&self) -> xcb_xkb_list_components_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xkb_get_kbd_by_name_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_xkb_get_kbd_by_name_request_t {
     pub major_opcode :       u8,
     pub minor_opcode :       u8,
     pub length :             u16,
     pub deviceSpec :         xcb_xkb_device_spec_t,
     pub need :               u16,
     pub want :               u16,
     pub load :               u8,
     pub pad0 :               u8,
     pub keymapsSpecLen :     u8,
     pub keycodesSpecLen :    u8,
     pub typesSpecLen :       u8,
     pub compatMapSpecLen :   u8,
     pub symbolsSpecLen :     u8,
     pub geometrySpecLen :    u8
}

impl Copy for xcb_xkb_get_kbd_by_name_request_t {}
impl Clone for xcb_xkb_get_kbd_by_name_request_t {
    fn clone(&self) -> xcb_xkb_get_kbd_by_name_request_t { *self }
}

#[repr(C)]
pub struct xcb_xkb_get_kbd_by_name_replies_types_map_t {
    pub types_rtrn :   *mut xcb_xkb_key_type_t,
    pub syms_rtrn :   *mut xcb_xkb_key_sym_map_t,
    pub acts_rtrn_count :               *mut u8,
    pub acts_rtrn_acts :   *mut xcb_xkb_action_t,
    pub behaviors_rtrn :   *mut xcb_xkb_set_behavior_t,
    pub vmods_rtrn :               *mut u8,
    pub explicit_rtrn :   *mut xcb_xkb_set_explicit_t,
    pub modmap_rtrn :   *mut xcb_xkb_key_mod_map_t,
    pub vmodmap_rtrn :   *mut xcb_xkb_key_v_mod_map_t
}

impl Copy for xcb_xkb_get_kbd_by_name_replies_types_map_t {}
impl Clone for xcb_xkb_get_kbd_by_name_replies_types_map_t {
    fn clone(&self) -> xcb_xkb_get_kbd_by_name_replies_types_map_t { *self }
}

#[repr(C)]
pub struct xcb_xkb_get_kbd_by_name_replies_client_symbols_map_t {
    pub types_rtrn :   *mut xcb_xkb_key_type_t,
    pub syms_rtrn :   *mut xcb_xkb_key_sym_map_t,
    pub acts_rtrn_count :               *mut u8,
    pub acts_rtrn_acts :   *mut xcb_xkb_action_t,
    pub behaviors_rtrn :   *mut xcb_xkb_set_behavior_t,
    pub vmods_rtrn :               *mut u8,
    pub explicit_rtrn :   *mut xcb_xkb_set_explicit_t,
    pub modmap_rtrn :   *mut xcb_xkb_key_mod_map_t,
    pub vmodmap_rtrn :   *mut xcb_xkb_key_v_mod_map_t
}

impl Copy for xcb_xkb_get_kbd_by_name_replies_client_symbols_map_t {}
impl Clone for xcb_xkb_get_kbd_by_name_replies_client_symbols_map_t {
    fn clone(&self) -> xcb_xkb_get_kbd_by_name_replies_client_symbols_map_t { *self }
}

#[repr(C)]
pub struct xcb_xkb_get_kbd_by_name_replies_server_symbols_map_t {
    pub types_rtrn :   *mut xcb_xkb_key_type_t,
    pub syms_rtrn :   *mut xcb_xkb_key_sym_map_t,
    pub acts_rtrn_count :               *mut u8,
    pub acts_rtrn_acts :   *mut xcb_xkb_action_t,
    pub behaviors_rtrn :   *mut xcb_xkb_set_behavior_t,
    pub vmods_rtrn :               *mut u8,
    pub explicit_rtrn :   *mut xcb_xkb_set_explicit_t,
    pub modmap_rtrn :   *mut xcb_xkb_key_mod_map_t,
    pub vmodmap_rtrn :   *mut xcb_xkb_key_v_mod_map_t
}

impl Copy for xcb_xkb_get_kbd_by_name_replies_server_symbols_map_t {}
impl Clone for xcb_xkb_get_kbd_by_name_replies_server_symbols_map_t {
    fn clone(&self) -> xcb_xkb_get_kbd_by_name_replies_server_symbols_map_t { *self }
}

#[repr(C)]
pub struct xcb_xkb_get_kbd_by_name_replies_key_names_value_list_t {
     pub keycodesName :      ffi::xproto::xcb_atom_t,
     pub geometryName :      ffi::xproto::xcb_atom_t,
     pub symbolsName :       ffi::xproto::xcb_atom_t,
     pub physSymbolsName :   ffi::xproto::xcb_atom_t,
     pub typesName :         ffi::xproto::xcb_atom_t,
     pub compatName :        ffi::xproto::xcb_atom_t,
    pub typeNames :   *mut ffi::xproto::xcb_atom_t,
    pub nLevelsPerType :               *mut u8,
    pub ktLevelNames :   *mut ffi::xproto::xcb_atom_t,
    pub indicatorNames :   *mut ffi::xproto::xcb_atom_t,
    pub virtualModNames :   *mut ffi::xproto::xcb_atom_t,
    pub groups :   *mut ffi::xproto::xcb_atom_t,
    pub keyNames :   *mut xcb_xkb_key_name_t,
    pub keyAliases :   *mut xcb_xkb_key_alias_t,
    pub radioGroupNames :   *mut ffi::xproto::xcb_atom_t
}

impl Copy for xcb_xkb_get_kbd_by_name_replies_key_names_value_list_t {}
impl Clone for xcb_xkb_get_kbd_by_name_replies_key_names_value_list_t {
    fn clone(&self) -> xcb_xkb_get_kbd_by_name_replies_key_names_value_list_t { *self }
}

#[repr(C)]
pub struct xcb_xkb_get_kbd_by_name_replies_other_names_value_list_t {
     pub keycodesName :      ffi::xproto::xcb_atom_t,
     pub geometryName :      ffi::xproto::xcb_atom_t,
     pub symbolsName :       ffi::xproto::xcb_atom_t,
     pub physSymbolsName :   ffi::xproto::xcb_atom_t,
     pub typesName :         ffi::xproto::xcb_atom_t,
     pub compatName :        ffi::xproto::xcb_atom_t,
    pub typeNames :   *mut ffi::xproto::xcb_atom_t,
    pub nLevelsPerType :               *mut u8,
    pub ktLevelNames :   *mut ffi::xproto::xcb_atom_t,
    pub indicatorNames :   *mut ffi::xproto::xcb_atom_t,
    pub virtualModNames :   *mut ffi::xproto::xcb_atom_t,
    pub groups :   *mut ffi::xproto::xcb_atom_t,
    pub keyNames :   *mut xcb_xkb_key_name_t,
    pub keyAliases :   *mut xcb_xkb_key_alias_t,
    pub radioGroupNames :   *mut ffi::xproto::xcb_atom_t
}

impl Copy for xcb_xkb_get_kbd_by_name_replies_other_names_value_list_t {}
impl Clone for xcb_xkb_get_kbd_by_name_replies_other_names_value_list_t {
    fn clone(&self) -> xcb_xkb_get_kbd_by_name_replies_other_names_value_list_t { *self }
}

#[repr(C)]
pub struct xcb_xkb_get_kbd_by_name_replies_t {
    types : struct _types {
         pub getmap_type :         u8,
         pub typeDeviceID :        u8,
         pub getmap_sequence :     u16,
         pub getmap_length :       u32,
         pub pad0 :                [u8; 2],
         pub typeMinKeyCode :      ffi::xproto::xcb_keycode_t,
         pub typeMaxKeyCode :      ffi::xproto::xcb_keycode_t,
         pub present :             u16,
         pub firstType :           u8,
         pub nTypes :              u8,
         pub totalTypes :          u8,
         pub firstKeySym :         ffi::xproto::xcb_keycode_t,
         pub totalSyms :           u16,
         pub nKeySyms :            u8,
         pub firstKeyAction :      ffi::xproto::xcb_keycode_t,
         pub totalActions :        u16,
         pub nKeyActions :         u8,
         pub firstKeyBehavior :    ffi::xproto::xcb_keycode_t,
         pub nKeyBehaviors :       u8,
         pub totalKeyBehaviors :   u8,
         pub firstKeyExplicit :    ffi::xproto::xcb_keycode_t,
         pub nKeyExplicit :        u8,
         pub totalKeyExplicit :    u8,
         pub firstModMapKey :      ffi::xproto::xcb_keycode_t,
         pub nModMapKeys :         u8,
         pub totalModMapKeys :     u8,
         pub firstVModMapKey :     ffi::xproto::xcb_keycode_t,
         pub nVModMapKeys :        u8,
         pub totalVModMapKeys :    u8,
         pub pad1 :                u8,
         pub virtualMods :         u16,
         pub map :                 xcb_xkb_get_kbd_by_name_replies_types_map_t,
    }
    compat_map : struct _compat_map {
         pub compatDeviceID :      u8,
         pub groupsRtrn :          u8,
         pub pad0 :                u8,
         pub firstSIRtrn :         u16,
         pub nSIRtrn :             u16,
         pub nTotalSI :            u16,
         pub pad1 :                [u8; 16],
        pub si_rtrn :                 *mut u8,
        pub group_rtrn :   *mut xcb_xkb_mod_def_t,
    }
    client_symbols : struct _client_symbols {
         pub clientDeviceID :      u8,
         pub pad0 :                [u8; 2],
         pub clientMinKeyCode :    ffi::xproto::xcb_keycode_t,
         pub clientMaxKeyCode :    ffi::xproto::xcb_keycode_t,
         pub present :             u16,
         pub firstType :           u8,
         pub nTypes :              u8,
         pub totalTypes :          u8,
         pub firstKeySym :         ffi::xproto::xcb_keycode_t,
         pub totalSyms :           u16,
         pub nKeySyms :            u8,
         pub firstKeyAction :      ffi::xproto::xcb_keycode_t,
         pub totalActions :        u16,
         pub nKeyActions :         u8,
         pub firstKeyBehavior :    ffi::xproto::xcb_keycode_t,
         pub nKeyBehaviors :       u8,
         pub totalKeyBehaviors :   u8,
         pub firstKeyExplicit :    ffi::xproto::xcb_keycode_t,
         pub nKeyExplicit :        u8,
         pub totalKeyExplicit :    u8,
         pub firstModMapKey :      ffi::xproto::xcb_keycode_t,
         pub nModMapKeys :         u8,
         pub totalModMapKeys :     u8,
         pub firstVModMapKey :     ffi::xproto::xcb_keycode_t,
         pub nVModMapKeys :        u8,
         pub totalVModMapKeys :    u8,
         pub pad1 :                u8,
         pub virtualMods :         u16,
         pub map :                 xcb_xkb_get_kbd_by_name_replies_client_symbols_map_t,
    }
    server_symbols : struct _server_symbols {
         pub serverDeviceID :      u8,
         pub pad0 :                [u8; 2],
         pub serverMinKeyCode :    ffi::xproto::xcb_keycode_t,
         pub serverMaxKeyCode :    ffi::xproto::xcb_keycode_t,
         pub present :             u16,
         pub firstType :           u8,
         pub nTypes :              u8,
         pub totalTypes :          u8,
         pub firstKeySym :         ffi::xproto::xcb_keycode_t,
         pub totalSyms :           u16,
         pub nKeySyms :            u8,
         pub firstKeyAction :      ffi::xproto::xcb_keycode_t,
         pub totalActions :        u16,
         pub nKeyActions :         u8,
         pub firstKeyBehavior :    ffi::xproto::xcb_keycode_t,
         pub nKeyBehaviors :       u8,
         pub totalKeyBehaviors :   u8,
         pub firstKeyExplicit :    ffi::xproto::xcb_keycode_t,
         pub nKeyExplicit :        u8,
         pub totalKeyExplicit :    u8,
         pub firstModMapKey :      ffi::xproto::xcb_keycode_t,
         pub nModMapKeys :         u8,
         pub totalModMapKeys :     u8,
         pub firstVModMapKey :     ffi::xproto::xcb_keycode_t,
         pub nVModMapKeys :        u8,
         pub totalVModMapKeys :    u8,
         pub pad1 :                u8,
         pub virtualMods :         u16,
         pub map :                 xcb_xkb_get_kbd_by_name_replies_server_symbols_map_t,
    }
    indicator_maps : struct _indicator_maps {
         pub indicatorDeviceID :   u8,
         pub which :               u32,
         pub realIndicators :      u32,
         pub nIndicators :         u8,
         pub pad0 :                [u8; 15],
        pub maps :   *mut xcb_xkb_indicator_map_t,
    }
    key_names : struct _key_names {
         pub keyDeviceID :         u8,
         pub which :               u32,
         pub keyMinKeyCode :       ffi::xproto::xcb_keycode_t,
         pub keyMaxKeyCode :       ffi::xproto::xcb_keycode_t,
         pub nTypes :              u8,
         pub groupNames :          u8,
         pub virtualMods :         u16,
         pub firstKey :            ffi::xproto::xcb_keycode_t,
         pub nKeys :               u8,
         pub indicators :          u32,
         pub nRadioGroups :        u8,
         pub nKeyAliases :         u8,
         pub nKTLevels :           u16,
         pub pad0 :                [u8; 4],
         pub valueList :           xcb_xkb_get_kbd_by_name_replies_key_names_value_list_t,
    }
    other_names : struct _other_names {
         pub otherDeviceID :       u8,
         pub which :               u32,
         pub otherMinKeyCode :     ffi::xproto::xcb_keycode_t,
         pub otherMaxKeyCode :     ffi::xproto::xcb_keycode_t,
         pub nTypes :              u8,
         pub groupNames :          u8,
         pub virtualMods :         u16,
         pub firstKey :            ffi::xproto::xcb_keycode_t,
         pub nKeys :               u8,
         pub indicators :          u32,
         pub nRadioGroups :        u8,
         pub nKeyAliases :         u8,
         pub nKTLevels :           u16,
         pub pad0 :                [u8; 4],
         pub valueList :           xcb_xkb_get_kbd_by_name_replies_other_names_value_list_t,
    }
    geometry : struct _geometry {
         pub geometryDeviceID :    u8,
         pub name :                ffi::xproto::xcb_atom_t,
         pub geometryFound :       u8,
         pub pad0 :                u8,
         pub widthMM :             u16,
         pub heightMM :            u16,
         pub nProperties :         u16,
         pub nColors :             u16,
         pub nShapes :             u16,
         pub nSections :           u16,
         pub nDoodads :            u16,
         pub nKeyAliases :         u16,
         pub baseColorNdx :        u8,
         pub labelColorNdx :       u8,
        pub labelFont :   *mut xcb_xkb_counted_string_16_t,
        pub properties :   *mut xcb_xkb_property_t,
        pub colors :   *mut xcb_xkb_counted_string_16_t,
        pub shapes :    *mut xcb_xkb_shape_t,
        pub sections :   *mut xcb_xkb_section_t,
        pub doodads :   *mut xcb_xkb_doodad_t,
        pub keyAliases :   *mut xcb_xkb_key_alias_t,
    }
}

impl Copy for xcb_xkb_get_kbd_by_name_replies_t {}
impl Clone for xcb_xkb_get_kbd_by_name_replies_t {
    fn clone(&self) -> xcb_xkb_get_kbd_by_name_replies_t { *self }
}

#[repr(C)]
pub struct xcb_xkb_get_kbd_by_name_reply_t {
     pub response_type :   u8,
     pub deviceID :        u8,
     pub sequence :        u16,
     pub length :          u32,
     pub minKeyCode :      ffi::xproto::xcb_keycode_t,
     pub maxKeyCode :      ffi::xproto::xcb_keycode_t,
     pub loaded :          u8,
     pub newKeyboard :     u8,
     pub found :           u16,
     pub reported :        u16,
     pub pad0 :            [u8; 16]
}

impl Copy for xcb_xkb_get_kbd_by_name_reply_t {}
impl Clone for xcb_xkb_get_kbd_by_name_reply_t {
    fn clone(&self) -> xcb_xkb_get_kbd_by_name_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xkb_get_device_info_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_xkb_get_device_info_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub deviceSpec :     xcb_xkb_device_spec_t,
     pub wanted :         u16,
     pub allButtons :     u8,
     pub firstButton :    u8,
     pub nButtons :       u8,
     pub pad0 :           u8,
     pub ledClass :       xcb_xkb_led_class_spec_t,
     pub ledID :          xcb_xkb_id_spec_t
}

impl Copy for xcb_xkb_get_device_info_request_t {}
impl Clone for xcb_xkb_get_device_info_request_t {
    fn clone(&self) -> xcb_xkb_get_device_info_request_t { *self }
}

#[repr(C)]
pub struct xcb_xkb_get_device_info_reply_t {
     pub response_type :    u8,
     pub deviceID :         u8,
     pub sequence :         u16,
     pub length :           u32,
     pub present :          u16,
     pub supported :        u16,
     pub unsupported :      u16,
     pub nDeviceLedFBs :    u16,
     pub firstBtnWanted :   u8,
     pub nBtnsWanted :      u8,
     pub firstBtnRtrn :     u8,
     pub nBtnsRtrn :        u8,
     pub totalBtns :        u8,
     pub hasOwnState :      u8,
     pub dfltKbdFB :        u16,
     pub dfltLedFB :        u16,
     pub pad0 :             [u8; 2],
     pub devType :          ffi::xproto::xcb_atom_t,
     pub nameLen :          u16
}

impl Copy for xcb_xkb_get_device_info_reply_t {}
impl Clone for xcb_xkb_get_device_info_reply_t {
    fn clone(&self) -> xcb_xkb_get_device_info_reply_t { *self }
}


#[repr(C)]
pub struct xcb_xkb_set_device_info_request_t {
     pub major_opcode :    u8,
     pub minor_opcode :    u8,
     pub length :          u16,
     pub deviceSpec :      xcb_xkb_device_spec_t,
     pub firstBtn :        u8,
     pub nBtns :           u8,
     pub change :          u16,
     pub nDeviceLedFBs :   u16
}

impl Copy for xcb_xkb_set_device_info_request_t {}
impl Clone for xcb_xkb_set_device_info_request_t {
    fn clone(&self) -> xcb_xkb_set_device_info_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xkb_set_debugging_flags_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_xkb_set_debugging_flags_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub msgLength :      u16,
     pub pad0 :           [u8; 2],
     pub affectFlags :    u32,
     pub flags :          u32,
     pub affectCtrls :    u32,
     pub ctrls :          u32
}

impl Copy for xcb_xkb_set_debugging_flags_request_t {}
impl Clone for xcb_xkb_set_debugging_flags_request_t {
    fn clone(&self) -> xcb_xkb_set_debugging_flags_request_t { *self }
}

#[repr(C)]
pub struct xcb_xkb_set_debugging_flags_reply_t {
     pub response_type :    u8,
     pub pad0 :             u8,
     pub sequence :         u16,
     pub length :           u32,
     pub currentFlags :     u32,
     pub currentCtrls :     u32,
     pub supportedFlags :   u32,
     pub supportedCtrls :   u32,
     pub pad1 :             [u8; 8]
}

impl Copy for xcb_xkb_set_debugging_flags_reply_t {}
impl Clone for xcb_xkb_set_debugging_flags_reply_t {
    fn clone(&self) -> xcb_xkb_set_debugging_flags_reply_t { *self }
}


#[repr(C)]
pub struct xcb_xkb_new_keyboard_notify_event_t {
     pub response_type :   u8,
     pub xkbType :         u8,
     pub sequence :        u16,
     pub time :            ffi::xproto::xcb_timestamp_t,
     pub deviceID :        u8,
     pub oldDeviceID :     u8,
     pub minKeyCode :      ffi::xproto::xcb_keycode_t,
     pub maxKeyCode :      ffi::xproto::xcb_keycode_t,
     pub oldMinKeyCode :   ffi::xproto::xcb_keycode_t,
     pub oldMaxKeyCode :   ffi::xproto::xcb_keycode_t,
     pub requestMajor :    u8,
     pub requestMinor :    u8,
     pub changed :         u16,
     pub pad0 :            [u8; 14]
}

impl Copy for xcb_xkb_new_keyboard_notify_event_t {}
impl Clone for xcb_xkb_new_keyboard_notify_event_t {
    fn clone(&self) -> xcb_xkb_new_keyboard_notify_event_t { *self }
}


#[repr(C)]
pub struct xcb_xkb_map_notify_event_t {
     pub response_type :      u8,
     pub xkbType :            u8,
     pub sequence :           u16,
     pub time :               ffi::xproto::xcb_timestamp_t,
     pub deviceID :           u8,
     pub ptrBtnActions :      u8,
     pub changed :            u16,
     pub minKeyCode :         ffi::xproto::xcb_keycode_t,
     pub maxKeyCode :         ffi::xproto::xcb_keycode_t,
     pub firstType :          u8,
     pub nTypes :             u8,
     pub firstKeySym :        ffi::xproto::xcb_keycode_t,
     pub nKeySyms :           u8,
     pub firstKeyAct :        ffi::xproto::xcb_keycode_t,
     pub nKeyActs :           u8,
     pub firstKeyBehavior :   ffi::xproto::xcb_keycode_t,
     pub nKeyBehavior :       u8,
     pub firstKeyExplicit :   ffi::xproto::xcb_keycode_t,
     pub nKeyExplicit :       u8,
     pub firstModMapKey :     ffi::xproto::xcb_keycode_t,
     pub nModMapKeys :        u8,
     pub firstVModMapKey :    ffi::xproto::xcb_keycode_t,
     pub nVModMapKeys :       u8,
     pub virtualMods :        u16,
     pub pad0 :               [u8; 2]
}

impl Copy for xcb_xkb_map_notify_event_t {}
impl Clone for xcb_xkb_map_notify_event_t {
    fn clone(&self) -> xcb_xkb_map_notify_event_t { *self }
}


#[repr(C)]
pub struct xcb_xkb_state_notify_event_t {
     pub response_type :       u8,
     pub xkbType :             u8,
     pub sequence :            u16,
     pub time :                ffi::xproto::xcb_timestamp_t,
     pub deviceID :            u8,
     pub mods :                u8,
     pub baseMods :            u8,
     pub latchedMods :         u8,
     pub lockedMods :          u8,
     pub group :               u8,
     pub baseGroup :           i16,
     pub latchedGroup :        i16,
     pub lockedGroup :         u8,
     pub compatState :         u8,
     pub grabMods :            u8,
     pub compatGrabMods :      u8,
     pub lookupMods :          u8,
     pub compatLoockupMods :   u8,
     pub ptrBtnState :         u16,
     pub changed :             u16,
     pub keycode :             ffi::xproto::xcb_keycode_t,
     pub eventType :           u8,
     pub requestMajor :        u8,
     pub requestMinor :        u8
}

impl Copy for xcb_xkb_state_notify_event_t {}
impl Clone for xcb_xkb_state_notify_event_t {
    fn clone(&self) -> xcb_xkb_state_notify_event_t { *self }
}


#[repr(C)]
pub struct xcb_xkb_controls_notify_event_t {
     pub response_type :           u8,
     pub xkbType :                 u8,
     pub sequence :                u16,
     pub time :                    ffi::xproto::xcb_timestamp_t,
     pub deviceID :                u8,
     pub numGroups :               u8,
     pub pad0 :                    [u8; 2],
     pub changedControls :         u32,
     pub enabledControls :         u32,
     pub enabledControlChanges :   u32,
     pub keycode :                 ffi::xproto::xcb_keycode_t,
     pub eventType :               u8,
     pub requestMajor :            u8,
     pub requestMinor :            u8,
     pub pad1 :                    [u8; 4]
}

impl Copy for xcb_xkb_controls_notify_event_t {}
impl Clone for xcb_xkb_controls_notify_event_t {
    fn clone(&self) -> xcb_xkb_controls_notify_event_t { *self }
}


#[repr(C)]
pub struct xcb_xkb_indicator_state_notify_event_t {
     pub response_type :   u8,
     pub xkbType :         u8,
     pub sequence :        u16,
     pub time :            ffi::xproto::xcb_timestamp_t,
     pub deviceID :        u8,
     pub pad0 :            [u8; 3],
     pub state :           u32,
     pub stateChanged :    u32,
     pub pad1 :            [u8; 12]
}

impl Copy for xcb_xkb_indicator_state_notify_event_t {}
impl Clone for xcb_xkb_indicator_state_notify_event_t {
    fn clone(&self) -> xcb_xkb_indicator_state_notify_event_t { *self }
}


#[repr(C)]
pub struct xcb_xkb_indicator_map_notify_event_t {
     pub response_type :   u8,
     pub xkbType :         u8,
     pub sequence :        u16,
     pub time :            ffi::xproto::xcb_timestamp_t,
     pub deviceID :        u8,
     pub pad0 :            [u8; 3],
     pub state :           u32,
     pub mapChanged :      u32,
     pub pad1 :            [u8; 12]
}

impl Copy for xcb_xkb_indicator_map_notify_event_t {}
impl Clone for xcb_xkb_indicator_map_notify_event_t {
    fn clone(&self) -> xcb_xkb_indicator_map_notify_event_t { *self }
}


#[repr(C)]
pub struct xcb_xkb_names_notify_event_t {
     pub response_type :        u8,
     pub xkbType :              u8,
     pub sequence :             u16,
     pub time :                 ffi::xproto::xcb_timestamp_t,
     pub deviceID :             u8,
     pub pad0 :                 u8,
     pub changed :              u16,
     pub firstType :            u8,
     pub nTypes :               u8,
     pub firstLevelName :       u8,
     pub nLevelNames :          u8,
     pub pad1 :                 u8,
     pub nRadioGroups :         u8,
     pub nKeyAliases :          u8,
     pub changedGroupNames :    u8,
     pub changedVirtualMods :   u16,
     pub firstKey :             ffi::xproto::xcb_keycode_t,
     pub nKeys :                u8,
     pub changedIndicators :    u32,
     pub pad2 :                 [u8; 4]
}

impl Copy for xcb_xkb_names_notify_event_t {}
impl Clone for xcb_xkb_names_notify_event_t {
    fn clone(&self) -> xcb_xkb_names_notify_event_t { *self }
}


#[repr(C)]
pub struct xcb_xkb_compat_map_notify_event_t {
     pub response_type :   u8,
     pub xkbType :         u8,
     pub sequence :        u16,
     pub time :            ffi::xproto::xcb_timestamp_t,
     pub deviceID :        u8,
     pub changedGroups :   u8,
     pub firstSI :         u16,
     pub nSI :             u16,
     pub nTotalSI :        u16,
     pub pad0 :            [u8; 16]
}

impl Copy for xcb_xkb_compat_map_notify_event_t {}
impl Clone for xcb_xkb_compat_map_notify_event_t {
    fn clone(&self) -> xcb_xkb_compat_map_notify_event_t { *self }
}


#[repr(C)]
pub struct xcb_xkb_bell_notify_event_t {
     pub response_type :   u8,
     pub xkbType :         u8,
     pub sequence :        u16,
     pub time :            ffi::xproto::xcb_timestamp_t,
     pub deviceID :        u8,
     pub bellClass :       u8,
     pub bellID :          u8,
     pub percent :         u8,
     pub pitch :           u16,
     pub duration :        u16,
     pub name :            ffi::xproto::xcb_atom_t,
     pub window :          ffi::xproto::xcb_window_t,
     pub eventOnly :       u8,
     pub pad0 :            [u8; 7]
}

impl Copy for xcb_xkb_bell_notify_event_t {}
impl Clone for xcb_xkb_bell_notify_event_t {
    fn clone(&self) -> xcb_xkb_bell_notify_event_t { *self }
}


#[repr(C)]
pub struct xcb_xkb_action_message_event_t {
     pub response_type :     u8,
     pub xkbType :           u8,
     pub sequence :          u16,
     pub time :              ffi::xproto::xcb_timestamp_t,
     pub deviceID :          u8,
     pub keycode :           ffi::xproto::xcb_keycode_t,
     pub press :             u8,
     pub keyEventFollows :   u8,
     pub mods :              u8,
     pub group :             u8,
     pub message :           [xcb_xkb_string8_t; 8],
     pub pad0 :              [u8; 10]
}

impl Copy for xcb_xkb_action_message_event_t {}
impl Clone for xcb_xkb_action_message_event_t {
    fn clone(&self) -> xcb_xkb_action_message_event_t { *self }
}


#[repr(C)]
pub struct xcb_xkb_access_x_notify_event_t {
     pub response_type :   u8,
     pub xkbType :         u8,
     pub sequence :        u16,
     pub time :            ffi::xproto::xcb_timestamp_t,
     pub deviceID :        u8,
     pub keycode :         ffi::xproto::xcb_keycode_t,
     pub detailt :         u16,
     pub slowKeysDelay :   u16,
     pub debounceDelay :   u16,
     pub pad0 :            [u8; 16]
}

impl Copy for xcb_xkb_access_x_notify_event_t {}
impl Clone for xcb_xkb_access_x_notify_event_t {
    fn clone(&self) -> xcb_xkb_access_x_notify_event_t { *self }
}


#[repr(C)]
pub struct xcb_xkb_extension_device_notify_event_t {
     pub response_type :   u8,
     pub xkbType :         u8,
     pub sequence :        u16,
     pub time :            ffi::xproto::xcb_timestamp_t,
     pub deviceID :        u8,
     pub pad0 :            u8,
     pub reason :          u16,
     pub ledClass :        u16,
     pub ledID :           u8,
     pub ledsDefined :     u32,
     pub ledState :        u32,
     pub firstButton :     u8,
     pub nButtons :        u8,
     pub supported :       u16,
     pub unsupported :     u16,
     pub pad1 :            [u8; 2]
}

impl Copy for xcb_xkb_extension_device_notify_event_t {}
impl Clone for xcb_xkb_extension_device_notify_event_t {
    fn clone(&self) -> xcb_xkb_extension_device_notify_event_t { *self }
}
#[link(name="xcb-xkb")]
extern "C" {

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xkb_ax_option_t)
///
pub fn xcb_xkb_ax_option_next (i:*mut xcb_xkb_ax_option_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xkb_ax_option_end (i:xcb_xkb_ax_option_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xkb_device_spec_t)
///
pub fn xcb_xkb_device_spec_next (i:*mut xcb_xkb_device_spec_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xkb_device_spec_end (i:xcb_xkb_device_spec_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xkb_led_class_spec_t)
///
pub fn xcb_xkb_led_class_spec_next (i:*mut xcb_xkb_led_class_spec_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xkb_led_class_spec_end (i:xcb_xkb_led_class_spec_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xkb_bell_class_spec_t)
///
pub fn xcb_xkb_bell_class_spec_next (i:*mut xcb_xkb_bell_class_spec_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xkb_bell_class_spec_end (i:xcb_xkb_bell_class_spec_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xkb_id_spec_t)
///
pub fn xcb_xkb_id_spec_next (i:*mut xcb_xkb_id_spec_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xkb_id_spec_end (i:xcb_xkb_id_spec_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xkb_indicator_map_t)
///
pub fn xcb_xkb_indicator_map_next (i:*mut xcb_xkb_indicator_map_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xkb_indicator_map_end (i:xcb_xkb_indicator_map_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xkb_mod_def_t)
///
pub fn xcb_xkb_mod_def_next (i:*mut xcb_xkb_mod_def_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xkb_mod_def_end (i:xcb_xkb_mod_def_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xkb_key_name_t)
///
pub fn xcb_xkb_key_name_next (i:*mut xcb_xkb_key_name_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xkb_key_name_end (i:xcb_xkb_key_name_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xkb_key_alias_t)
///
pub fn xcb_xkb_key_alias_next (i:*mut xcb_xkb_key_alias_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xkb_key_alias_end (i:xcb_xkb_key_alias_iterator_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_xkb_counted_string_8_sizeof (_buffer :  *mut c_void) -> c_int;

pub fn xcb_xkb_counted_string_8_string (R : *mut xcb_xkb_counted_string_8_t) -> *mut u8;


pub fn xcb_xkb_counted_string_8_string_length (R : *mut xcb_xkb_counted_string_8_t) -> c_int;


pub fn xcb_xkb_counted_string_8_string_end (R : *mut xcb_xkb_counted_string_8_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xkb_counted_string_8_t)
///
pub fn xcb_xkb_counted_string_8_next (i:*mut xcb_xkb_counted_string_8_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xkb_counted_string_8_end (i:xcb_xkb_counted_string_8_iterator_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_xkb_counted_string_16_sizeof (_buffer :  *mut c_void) -> c_int;

pub fn xcb_xkb_counted_string_16_string (R : *mut xcb_xkb_counted_string_16_t) -> *mut u8;


pub fn xcb_xkb_counted_string_16_string_length (R : *mut xcb_xkb_counted_string_16_t) -> c_int;


pub fn xcb_xkb_counted_string_16_string_end (R : *mut xcb_xkb_counted_string_16_t) -> ffi::base::xcb_generic_iterator_t;


///
/// xcb_xkb_counted_string_16_pad_0 : *mut u8
///
pub fn xcb_xkb_counted_string_16_pad_0 (R : *mut xcb_xkb_counted_string_16_t) -> *mut u8;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xkb_counted_string_16_t)
///
pub fn xcb_xkb_counted_string_16_next (i:*mut xcb_xkb_counted_string_16_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xkb_counted_string_16_end (i:xcb_xkb_counted_string_16_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xkb_kt_map_entry_t)
///
pub fn xcb_xkb_kt_map_entry_next (i:*mut xcb_xkb_kt_map_entry_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xkb_kt_map_entry_end (i:xcb_xkb_kt_map_entry_iterator_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_xkb_key_type_sizeof (_buffer :  *mut c_void) -> c_int;

pub fn xcb_xkb_key_type_map (R : *mut xcb_xkb_key_type_t) -> *mut xcb_xkb_kt_map_entry_t;


pub fn xcb_xkb_key_type_map_length (R : *mut xcb_xkb_key_type_t) -> c_int;

pub fn xcb_xkb_key_type_map_iterator (R : *mut xcb_xkb_key_type_t) -> xcb_xkb_kt_map_entry_iterator_t;

pub fn xcb_xkb_key_type_preserve (R : *mut xcb_xkb_key_type_t) -> *mut xcb_xkb_mod_def_t;


pub fn xcb_xkb_key_type_preserve_length (R : *mut xcb_xkb_key_type_t) -> c_int;

pub fn xcb_xkb_key_type_preserve_iterator (R : *mut xcb_xkb_key_type_t) -> xcb_xkb_mod_def_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xkb_key_type_t)
///
pub fn xcb_xkb_key_type_next (i:*mut xcb_xkb_key_type_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xkb_key_type_end (i:xcb_xkb_key_type_iterator_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_xkb_key_sym_map_sizeof (_buffer :  *mut c_void) -> c_int;

pub fn xcb_xkb_key_sym_map_syms (R : *mut xcb_xkb_key_sym_map_t) -> *mut ffi::xproto::xcb_keysym_t;


pub fn xcb_xkb_key_sym_map_syms_length (R : *mut xcb_xkb_key_sym_map_t) -> c_int;


pub fn xcb_xkb_key_sym_map_syms_end (R : *mut xcb_xkb_key_sym_map_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xkb_key_sym_map_t)
///
pub fn xcb_xkb_key_sym_map_next (i:*mut xcb_xkb_key_sym_map_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xkb_key_sym_map_end (i:xcb_xkb_key_sym_map_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xkb_common_behavior_t)
///
pub fn xcb_xkb_common_behavior_next (i:*mut xcb_xkb_common_behavior_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xkb_common_behavior_end (i:xcb_xkb_common_behavior_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xkb_default_behavior_t)
///
pub fn xcb_xkb_default_behavior_next (i:*mut xcb_xkb_default_behavior_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xkb_default_behavior_end (i:xcb_xkb_default_behavior_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xkb_lock_behavior_t)
///
pub fn xcb_xkb_lock_behavior_next (i:*mut xcb_xkb_lock_behavior_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xkb_lock_behavior_end (i:xcb_xkb_lock_behavior_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xkb_radio_group_behavior_t)
///
pub fn xcb_xkb_radio_group_behavior_next (i:*mut xcb_xkb_radio_group_behavior_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xkb_radio_group_behavior_end (i:xcb_xkb_radio_group_behavior_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xkb_overlay_1_behavior_t)
///
pub fn xcb_xkb_overlay_1_behavior_next (i:*mut xcb_xkb_overlay_1_behavior_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xkb_overlay_1_behavior_end (i:xcb_xkb_overlay_1_behavior_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xkb_overlay_2_behavior_t)
///
pub fn xcb_xkb_overlay_2_behavior_next (i:*mut xcb_xkb_overlay_2_behavior_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xkb_overlay_2_behavior_end (i:xcb_xkb_overlay_2_behavior_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xkb_permament_lock_behavior_t)
///
pub fn xcb_xkb_permament_lock_behavior_next (i:*mut xcb_xkb_permament_lock_behavior_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xkb_permament_lock_behavior_end (i:xcb_xkb_permament_lock_behavior_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xkb_permament_radio_group_behavior_t)
///
pub fn xcb_xkb_permament_radio_group_behavior_next (i:*mut xcb_xkb_permament_radio_group_behavior_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xkb_permament_radio_group_behavior_end (i:xcb_xkb_permament_radio_group_behavior_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xkb_permament_overlay_1_behavior_t)
///
pub fn xcb_xkb_permament_overlay_1_behavior_next (i:*mut xcb_xkb_permament_overlay_1_behavior_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xkb_permament_overlay_1_behavior_end (i:xcb_xkb_permament_overlay_1_behavior_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xkb_permament_overlay_2_behavior_t)
///
pub fn xcb_xkb_permament_overlay_2_behavior_next (i:*mut xcb_xkb_permament_overlay_2_behavior_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xkb_permament_overlay_2_behavior_end (i:xcb_xkb_permament_overlay_2_behavior_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xkb_behavior_t)
///
pub fn xcb_xkb_behavior_next (i:*mut xcb_xkb_behavior_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xkb_behavior_end (i:xcb_xkb_behavior_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xkb_set_behavior_t)
///
pub fn xcb_xkb_set_behavior_next (i:*mut xcb_xkb_set_behavior_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xkb_set_behavior_end (i:xcb_xkb_set_behavior_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xkb_set_explicit_t)
///
pub fn xcb_xkb_set_explicit_next (i:*mut xcb_xkb_set_explicit_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xkb_set_explicit_end (i:xcb_xkb_set_explicit_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xkb_key_mod_map_t)
///
pub fn xcb_xkb_key_mod_map_next (i:*mut xcb_xkb_key_mod_map_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xkb_key_mod_map_end (i:xcb_xkb_key_mod_map_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xkb_key_v_mod_map_t)
///
pub fn xcb_xkb_key_v_mod_map_next (i:*mut xcb_xkb_key_v_mod_map_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xkb_key_v_mod_map_end (i:xcb_xkb_key_v_mod_map_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xkb_kt_set_map_entry_t)
///
pub fn xcb_xkb_kt_set_map_entry_next (i:*mut xcb_xkb_kt_set_map_entry_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xkb_kt_set_map_entry_end (i:xcb_xkb_kt_set_map_entry_iterator_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_xkb_set_key_type_sizeof (_buffer :  *mut c_void) -> c_int;

pub fn xcb_xkb_set_key_type_entries (R : *mut xcb_xkb_set_key_type_t) -> *mut xcb_xkb_kt_set_map_entry_t;


pub fn xcb_xkb_set_key_type_entries_length (R : *mut xcb_xkb_set_key_type_t) -> c_int;

pub fn xcb_xkb_set_key_type_entries_iterator (R : *mut xcb_xkb_set_key_type_t) -> xcb_xkb_kt_set_map_entry_iterator_t;

pub fn xcb_xkb_set_key_type_preserve_entries (R : *mut xcb_xkb_set_key_type_t) -> *mut xcb_xkb_kt_set_map_entry_t;


pub fn xcb_xkb_set_key_type_preserve_entries_length (R : *mut xcb_xkb_set_key_type_t) -> c_int;

pub fn xcb_xkb_set_key_type_preserve_entries_iterator (R : *mut xcb_xkb_set_key_type_t) -> xcb_xkb_kt_set_map_entry_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xkb_set_key_type_t)
///
pub fn xcb_xkb_set_key_type_next (i:*mut xcb_xkb_set_key_type_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xkb_set_key_type_end (i:xcb_xkb_set_key_type_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xkb_string8_t)
///
pub fn xcb_xkb_string8_next (i:*mut xcb_xkb_string8_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xkb_string8_end (i:xcb_xkb_string8_iterator_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_xkb_property_serialize (_buffer :        *mut *mut c_void,
                            _aux :                *mut xcb_xkb_property_t,
                            name :                *mut xcb_xkb_string8_t,
                            value :               *mut xcb_xkb_string8_t) -> c_int;

pub fn xcb_xkb_property_unserialize (_buffer :                  *mut c_void,
                              _aux :                *mut *mut xcb_xkb_property_t) -> c_int;

pub fn xcb_xkb_property_sizeof (_buffer :  *mut c_void) -> c_int;

pub fn xcb_xkb_property_name (R : *mut xcb_xkb_property_t) -> *mut xcb_xkb_string8_t;


pub fn xcb_xkb_property_name_length (R : *mut xcb_xkb_property_t) -> c_int;


pub fn xcb_xkb_property_name_end (R : *mut xcb_xkb_property_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_xkb_property_value (R : *mut xcb_xkb_property_t) -> *mut xcb_xkb_string8_t;


pub fn xcb_xkb_property_value_length (R : *mut xcb_xkb_property_t) -> c_int;


pub fn xcb_xkb_property_value_end (R : *mut xcb_xkb_property_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xkb_property_t)
///
pub fn xcb_xkb_property_next (i:*mut xcb_xkb_property_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xkb_property_end (i:xcb_xkb_property_iterator_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_xkb_outline_sizeof (_buffer :  *mut c_void) -> c_int;

pub fn xcb_xkb_outline_points (R : *mut xcb_xkb_outline_t) -> *mut ffi::xproto::xcb_point_t;


pub fn xcb_xkb_outline_points_length (R : *mut xcb_xkb_outline_t) -> c_int;

pub fn xcb_xkb_outline_points_iterator (R : *mut xcb_xkb_outline_t) -> ffi::xproto::xcb_point_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xkb_outline_t)
///
pub fn xcb_xkb_outline_next (i:*mut xcb_xkb_outline_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xkb_outline_end (i:xcb_xkb_outline_iterator_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_xkb_shape_sizeof (_buffer :  *mut c_void) -> c_int;


pub fn xcb_xkb_shape_outlines_length (R : *mut xcb_xkb_shape_t) -> c_int;

pub fn xcb_xkb_shape_outlines_iterator (R : *mut xcb_xkb_shape_t) -> xcb_xkb_outline_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xkb_shape_t)
///
pub fn xcb_xkb_shape_next (i:*mut xcb_xkb_shape_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xkb_shape_end (i:xcb_xkb_shape_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xkb_key_t)
///
pub fn xcb_xkb_key_next (i:*mut xcb_xkb_key_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xkb_key_end (i:xcb_xkb_key_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xkb_overlay_key_t)
///
pub fn xcb_xkb_overlay_key_next (i:*mut xcb_xkb_overlay_key_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xkb_overlay_key_end (i:xcb_xkb_overlay_key_iterator_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_xkb_overlay_row_sizeof (_buffer :  *mut c_void) -> c_int;

pub fn xcb_xkb_overlay_row_keys (R : *mut xcb_xkb_overlay_row_t) -> *mut xcb_xkb_overlay_key_t;


pub fn xcb_xkb_overlay_row_keys_length (R : *mut xcb_xkb_overlay_row_t) -> c_int;

pub fn xcb_xkb_overlay_row_keys_iterator (R : *mut xcb_xkb_overlay_row_t) -> xcb_xkb_overlay_key_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xkb_overlay_row_t)
///
pub fn xcb_xkb_overlay_row_next (i:*mut xcb_xkb_overlay_row_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xkb_overlay_row_end (i:xcb_xkb_overlay_row_iterator_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_xkb_overlay_sizeof (_buffer :  *mut c_void) -> c_int;


pub fn xcb_xkb_overlay_rows_length (R : *mut xcb_xkb_overlay_t) -> c_int;

pub fn xcb_xkb_overlay_rows_iterator (R : *mut xcb_xkb_overlay_t) -> xcb_xkb_overlay_row_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xkb_overlay_t)
///
pub fn xcb_xkb_overlay_next (i:*mut xcb_xkb_overlay_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xkb_overlay_end (i:xcb_xkb_overlay_iterator_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_xkb_row_sizeof (_buffer :  *mut c_void) -> c_int;

pub fn xcb_xkb_row_keys (R : *mut xcb_xkb_row_t) -> *mut xcb_xkb_key_t;


pub fn xcb_xkb_row_keys_length (R : *mut xcb_xkb_row_t) -> c_int;

pub fn xcb_xkb_row_keys_iterator (R : *mut xcb_xkb_row_t) -> xcb_xkb_key_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xkb_row_t)
///
pub fn xcb_xkb_row_next (i:*mut xcb_xkb_row_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xkb_row_end (i:xcb_xkb_row_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xkb_common_doodad_t)
///
pub fn xcb_xkb_common_doodad_next (i:*mut xcb_xkb_common_doodad_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xkb_common_doodad_end (i:xcb_xkb_common_doodad_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xkb_shape_doodad_t)
///
pub fn xcb_xkb_shape_doodad_next (i:*mut xcb_xkb_shape_doodad_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xkb_shape_doodad_end (i:xcb_xkb_shape_doodad_iterator_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_xkb_text_doodad_sizeof (_buffer :  *mut c_void) -> c_int;


///
/// xcb_xkb_text_doodad_text : *mut xcb_xkb_counted_string_16_t
///
pub fn xcb_xkb_text_doodad_text (R : *mut xcb_xkb_text_doodad_t) -> *mut xcb_xkb_counted_string_16_t;


///
/// xcb_xkb_text_doodad_font : *mut xcb_xkb_counted_string_16_t
///
pub fn xcb_xkb_text_doodad_font (R : *mut xcb_xkb_text_doodad_t) -> *mut xcb_xkb_counted_string_16_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xkb_text_doodad_t)
///
pub fn xcb_xkb_text_doodad_next (i:*mut xcb_xkb_text_doodad_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xkb_text_doodad_end (i:xcb_xkb_text_doodad_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xkb_indicator_doodad_t)
///
pub fn xcb_xkb_indicator_doodad_next (i:*mut xcb_xkb_indicator_doodad_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xkb_indicator_doodad_end (i:xcb_xkb_indicator_doodad_iterator_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_xkb_logo_doodad_sizeof (_buffer :  *mut c_void) -> c_int;


///
/// xcb_xkb_logo_doodad_logo_name : *mut xcb_xkb_counted_string_16_t
///
pub fn xcb_xkb_logo_doodad_logo_name (R : *mut xcb_xkb_logo_doodad_t) -> *mut xcb_xkb_counted_string_16_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xkb_logo_doodad_t)
///
pub fn xcb_xkb_logo_doodad_next (i:*mut xcb_xkb_logo_doodad_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xkb_logo_doodad_end (i:xcb_xkb_logo_doodad_iterator_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_xkb_doodad_sizeof (_buffer :  *mut c_void) -> c_int;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xkb_doodad_t)
///
pub fn xcb_xkb_doodad_next (i:*mut xcb_xkb_doodad_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xkb_doodad_end (i:xcb_xkb_doodad_iterator_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_xkb_section_sizeof (_buffer :  *mut c_void) -> c_int;


pub fn xcb_xkb_section_rows_length (R : *mut xcb_xkb_section_t) -> c_int;

pub fn xcb_xkb_section_rows_iterator (R : *mut xcb_xkb_section_t) -> xcb_xkb_row_iterator_t;


pub fn xcb_xkb_section_doodads_length (R : *mut xcb_xkb_section_t) -> c_int;

pub fn xcb_xkb_section_doodads_iterator (R : *mut xcb_xkb_section_t) -> xcb_xkb_doodad_iterator_t;


pub fn xcb_xkb_section_overlays_length (R : *mut xcb_xkb_section_t) -> c_int;

pub fn xcb_xkb_section_overlays_iterator (R : *mut xcb_xkb_section_t) -> xcb_xkb_overlay_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xkb_section_t)
///
pub fn xcb_xkb_section_next (i:*mut xcb_xkb_section_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xkb_section_end (i:xcb_xkb_section_iterator_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_xkb_listing_sizeof (_buffer :  *mut c_void) -> c_int;

pub fn xcb_xkb_listing_string (R : *mut xcb_xkb_listing_t) -> *mut xcb_xkb_string8_t;


pub fn xcb_xkb_listing_string_length (R : *mut xcb_xkb_listing_t) -> c_int;


pub fn xcb_xkb_listing_string_end (R : *mut xcb_xkb_listing_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xkb_listing_t)
///
pub fn xcb_xkb_listing_next (i:*mut xcb_xkb_listing_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xkb_listing_end (i:xcb_xkb_listing_iterator_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_xkb_device_led_info_sizeof (_buffer :  *mut c_void) -> c_int;

pub fn xcb_xkb_device_led_info_names (R : *mut xcb_xkb_device_led_info_t) -> *mut ffi::xproto::xcb_atom_t;


pub fn xcb_xkb_device_led_info_names_length (R : *mut xcb_xkb_device_led_info_t) -> c_int;


pub fn xcb_xkb_device_led_info_names_end (R : *mut xcb_xkb_device_led_info_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_xkb_device_led_info_maps (R : *mut xcb_xkb_device_led_info_t) -> *mut xcb_xkb_indicator_map_t;


pub fn xcb_xkb_device_led_info_maps_length (R : *mut xcb_xkb_device_led_info_t) -> c_int;

pub fn xcb_xkb_device_led_info_maps_iterator (R : *mut xcb_xkb_device_led_info_t) -> xcb_xkb_indicator_map_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xkb_device_led_info_t)
///
pub fn xcb_xkb_device_led_info_next (i:*mut xcb_xkb_device_led_info_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xkb_device_led_info_end (i:xcb_xkb_device_led_info_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xkb_sa_no_action_t)
///
pub fn xcb_xkb_sa_no_action_next (i:*mut xcb_xkb_sa_no_action_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xkb_sa_no_action_end (i:xcb_xkb_sa_no_action_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xkb_sa_set_mods_t)
///
pub fn xcb_xkb_sa_set_mods_next (i:*mut xcb_xkb_sa_set_mods_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xkb_sa_set_mods_end (i:xcb_xkb_sa_set_mods_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xkb_sa_latch_mods_t)
///
pub fn xcb_xkb_sa_latch_mods_next (i:*mut xcb_xkb_sa_latch_mods_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xkb_sa_latch_mods_end (i:xcb_xkb_sa_latch_mods_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xkb_sa_lock_mods_t)
///
pub fn xcb_xkb_sa_lock_mods_next (i:*mut xcb_xkb_sa_lock_mods_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xkb_sa_lock_mods_end (i:xcb_xkb_sa_lock_mods_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xkb_sa_set_group_t)
///
pub fn xcb_xkb_sa_set_group_next (i:*mut xcb_xkb_sa_set_group_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xkb_sa_set_group_end (i:xcb_xkb_sa_set_group_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xkb_sa_latch_group_t)
///
pub fn xcb_xkb_sa_latch_group_next (i:*mut xcb_xkb_sa_latch_group_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xkb_sa_latch_group_end (i:xcb_xkb_sa_latch_group_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xkb_sa_lock_group_t)
///
pub fn xcb_xkb_sa_lock_group_next (i:*mut xcb_xkb_sa_lock_group_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xkb_sa_lock_group_end (i:xcb_xkb_sa_lock_group_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xkb_sa_move_ptr_t)
///
pub fn xcb_xkb_sa_move_ptr_next (i:*mut xcb_xkb_sa_move_ptr_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xkb_sa_move_ptr_end (i:xcb_xkb_sa_move_ptr_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xkb_sa_ptr_btn_t)
///
pub fn xcb_xkb_sa_ptr_btn_next (i:*mut xcb_xkb_sa_ptr_btn_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xkb_sa_ptr_btn_end (i:xcb_xkb_sa_ptr_btn_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xkb_sa_lock_ptr_btn_t)
///
pub fn xcb_xkb_sa_lock_ptr_btn_next (i:*mut xcb_xkb_sa_lock_ptr_btn_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xkb_sa_lock_ptr_btn_end (i:xcb_xkb_sa_lock_ptr_btn_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xkb_sa_set_ptr_dflt_t)
///
pub fn xcb_xkb_sa_set_ptr_dflt_next (i:*mut xcb_xkb_sa_set_ptr_dflt_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xkb_sa_set_ptr_dflt_end (i:xcb_xkb_sa_set_ptr_dflt_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xkb_sa_iso_lock_t)
///
pub fn xcb_xkb_sa_iso_lock_next (i:*mut xcb_xkb_sa_iso_lock_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xkb_sa_iso_lock_end (i:xcb_xkb_sa_iso_lock_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xkb_sa_terminate_t)
///
pub fn xcb_xkb_sa_terminate_next (i:*mut xcb_xkb_sa_terminate_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xkb_sa_terminate_end (i:xcb_xkb_sa_terminate_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xkb_sa_switch_screen_t)
///
pub fn xcb_xkb_sa_switch_screen_next (i:*mut xcb_xkb_sa_switch_screen_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xkb_sa_switch_screen_end (i:xcb_xkb_sa_switch_screen_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xkb_sa_set_controls_t)
///
pub fn xcb_xkb_sa_set_controls_next (i:*mut xcb_xkb_sa_set_controls_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xkb_sa_set_controls_end (i:xcb_xkb_sa_set_controls_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xkb_sa_lock_controls_t)
///
pub fn xcb_xkb_sa_lock_controls_next (i:*mut xcb_xkb_sa_lock_controls_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xkb_sa_lock_controls_end (i:xcb_xkb_sa_lock_controls_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xkb_sa_action_message_t)
///
pub fn xcb_xkb_sa_action_message_next (i:*mut xcb_xkb_sa_action_message_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xkb_sa_action_message_end (i:xcb_xkb_sa_action_message_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xkb_sa_redirect_key_t)
///
pub fn xcb_xkb_sa_redirect_key_next (i:*mut xcb_xkb_sa_redirect_key_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xkb_sa_redirect_key_end (i:xcb_xkb_sa_redirect_key_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xkb_sa_device_btn_t)
///
pub fn xcb_xkb_sa_device_btn_next (i:*mut xcb_xkb_sa_device_btn_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xkb_sa_device_btn_end (i:xcb_xkb_sa_device_btn_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xkb_sa_lock_device_btn_t)
///
pub fn xcb_xkb_sa_lock_device_btn_next (i:*mut xcb_xkb_sa_lock_device_btn_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xkb_sa_lock_device_btn_end (i:xcb_xkb_sa_lock_device_btn_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xkb_sa_device_valuator_t)
///
pub fn xcb_xkb_sa_device_valuator_next (i:*mut xcb_xkb_sa_device_valuator_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xkb_sa_device_valuator_end (i:xcb_xkb_sa_device_valuator_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_xkb_action_t)
///
pub fn xcb_xkb_action_next (i:*mut xcb_xkb_action_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_xkb_action_end (i:xcb_xkb_action_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/// Delivers a request to the X server.
///
pub fn xcb_xkb_use_extension (c : *mut ffi::base::xcb_connection_t,
                                 wantedMajor :  u16,
                                 wantedMinor :  u16) -> xcb_xkb_use_extension_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_xkb_use_extension_unchecked (c : *mut ffi::base::xcb_connection_t,
                                           wantedMajor :  u16,
                                           wantedMinor :  u16) -> xcb_xkb_use_extension_cookie_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_xkb_use_extension_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_xkb_use_extension_reply (c : *mut ffi::base::xcb_connection_t,
                                       cookie : xcb_xkb_use_extension_cookie_t,
                                       e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_xkb_use_extension_reply_t;

pub fn xcb_xkb_select_events_details_serialize (_buffer :                     *mut *mut c_void,
                                         affectWhich :                           u16,
                                         clear :                                 u16,
                                         selectAll :                             u16,
                                         _aux :                             *mut xcb_xkb_select_events_details_t) -> c_int;

pub fn xcb_xkb_select_events_details_unpack (_buffer :                          *mut c_void,
                                      affectWhich :                           u16,
                                      clear :                                 u16,
                                      selectAll :                             u16,
                                      _aux :                             *mut xcb_xkb_select_events_details_t) -> c_int;

pub fn xcb_xkb_select_events_details_sizeof (_buffer :  *mut c_void,
                                      affectWhich :   u16,
                                      clear :        u16,
                                      selectAll :    u16) -> c_int;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_xkb_select_events_checked (c : *mut ffi::base::xcb_connection_t,
                                         deviceSpec :  xcb_xkb_device_spec_t,
                                         affectWhich :  u16,
                                         clear :  u16,
                                         selectAll :  u16,
                                         affectMap :  u16,
                                         map :  u16,
                                         details : *mut ()) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_xkb_select_events (c : *mut ffi::base::xcb_connection_t,
                                 deviceSpec :  xcb_xkb_device_spec_t,
                                 affectWhich :  u16,
                                 clear :  u16,
                                 selectAll :  u16,
                                 affectMap :  u16,
                                 map :  u16,
                                 details : *mut ()) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_xkb_select_events_aux_checked (c : *mut ffi::base::xcb_connection_t,
                                             deviceSpec :  xcb_xkb_device_spec_t,
                                             affectWhich :  u16,
                                             clear :  u16,
                                             selectAll :  u16,
                                             affectMap :  u16,
                                             map :  u16,
                                             details : *mut xcb_xkb_select_events_details_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_xkb_select_events_aux (c : *mut ffi::base::xcb_connection_t,
                                     deviceSpec :  xcb_xkb_device_spec_t,
                                     affectWhich :  u16,
                                     clear :  u16,
                                     selectAll :  u16,
                                     affectMap :  u16,
                                     map :  u16,
                                     details : *mut xcb_xkb_select_events_details_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_xkb_bell_checked (c : *mut ffi::base::xcb_connection_t,
                                deviceSpec :  xcb_xkb_device_spec_t,
                                bellClass :  xcb_xkb_bell_class_spec_t,
                                bellID :  xcb_xkb_id_spec_t,
                                percent :  i8,
                                forceSound :  u8,
                                eventOnly :  u8,
                                pitch :  i16,
                                duration :  i16,
                                name :  ffi::xproto::xcb_atom_t,
                                window :  ffi::xproto::xcb_window_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_xkb_bell (c : *mut ffi::base::xcb_connection_t,
                        deviceSpec :  xcb_xkb_device_spec_t,
                        bellClass :  xcb_xkb_bell_class_spec_t,
                        bellID :  xcb_xkb_id_spec_t,
                        percent :  i8,
                        forceSound :  u8,
                        eventOnly :  u8,
                        pitch :  i16,
                        duration :  i16,
                        name :  ffi::xproto::xcb_atom_t,
                        window :  ffi::xproto::xcb_window_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_xkb_get_state (c : *mut ffi::base::xcb_connection_t,
                             deviceSpec :  xcb_xkb_device_spec_t) -> xcb_xkb_get_state_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_xkb_get_state_unchecked (c : *mut ffi::base::xcb_connection_t,
                                       deviceSpec :  xcb_xkb_device_spec_t) -> xcb_xkb_get_state_cookie_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_xkb_get_state_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_xkb_get_state_reply (c : *mut ffi::base::xcb_connection_t,
                                   cookie : xcb_xkb_get_state_cookie_t,
                                   e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_xkb_get_state_reply_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_xkb_latch_lock_state_checked (c : *mut ffi::base::xcb_connection_t,
                                            deviceSpec :  xcb_xkb_device_spec_t,
                                            affectModLocks :  u8,
                                            modLocks :  u8,
                                            lockGroup :  u8,
                                            groupLock :  u8,
                                            affectModLatches :  u8,
                                            latchGroup :  u8,
                                            groupLatch :  u16) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_xkb_latch_lock_state (c : *mut ffi::base::xcb_connection_t,
                                    deviceSpec :  xcb_xkb_device_spec_t,
                                    affectModLocks :  u8,
                                    modLocks :  u8,
                                    lockGroup :  u8,
                                    groupLock :  u8,
                                    affectModLatches :  u8,
                                    latchGroup :  u8,
                                    groupLatch :  u16) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_xkb_get_controls (c : *mut ffi::base::xcb_connection_t,
                                deviceSpec :  xcb_xkb_device_spec_t) -> xcb_xkb_get_controls_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_xkb_get_controls_unchecked (c : *mut ffi::base::xcb_connection_t,
                                          deviceSpec :  xcb_xkb_device_spec_t) -> xcb_xkb_get_controls_cookie_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_xkb_get_controls_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_xkb_get_controls_reply (c : *mut ffi::base::xcb_connection_t,
                                      cookie : xcb_xkb_get_controls_cookie_t,
                                      e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_xkb_get_controls_reply_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_xkb_set_controls_checked (c : *mut ffi::base::xcb_connection_t,
                                        deviceSpec :  xcb_xkb_device_spec_t,
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
                                        accessXOptions :  xcb_xkb_ax_option_t,
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
                                        accessXTimeoutOptionsMask :  xcb_xkb_ax_option_t,
                                        accessXTimeoutOptionsValues :  xcb_xkb_ax_option_t,
                                        perKeyRepeat : *mut u8) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_xkb_set_controls (c : *mut ffi::base::xcb_connection_t,
                                deviceSpec :  xcb_xkb_device_spec_t,
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
                                accessXOptions :  xcb_xkb_ax_option_t,
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
                                accessXTimeoutOptionsMask :  xcb_xkb_ax_option_t,
                                accessXTimeoutOptionsValues :  xcb_xkb_ax_option_t,
                                perKeyRepeat : *mut u8) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_xkb_get_map_map_serialize (_buffer :           *mut *mut c_void,
                               nTypes :                      u8,
                               nKeySyms :                    u8,
                               nKeyActions :                 u8,
                               totalActions :                u16,
                               totalKeyBehaviors :           u8,
                               nVModMapKeys :                u8,
                               totalKeyExplicit :            u8,
                               totalModMapKeys :             u8,
                               totalVModMapKeys :            u8,
                               present :                     u16,
                               _aux :                   *mut xcb_xkb_get_map_map_t) -> c_int;

pub fn xcb_xkb_get_map_map_unpack (_buffer :                *mut c_void,
                            nTypes :                      u8,
                            nKeySyms :                    u8,
                            nKeyActions :                 u8,
                            totalActions :                u16,
                            totalKeyBehaviors :           u8,
                            nVModMapKeys :                u8,
                            totalKeyExplicit :            u8,
                            totalModMapKeys :             u8,
                            totalVModMapKeys :            u8,
                            present :                     u16,
                            _aux :                   *mut xcb_xkb_get_map_map_t) -> c_int;

pub fn xcb_xkb_get_map_map_sizeof (_buffer :  *mut c_void,
                            nTypes :       u8,
                            nKeySyms :     u8,
                            nKeyActions :  u8,
                            totalActions :  u16,
                            totalKeyBehaviors :  u8,
                            nVModMapKeys :  u8,
                            totalKeyExplicit :  u8,
                            totalModMapKeys :  u8,
                            totalVModMapKeys :  u8,
                            present :      u16) -> c_int;

/// Delivers a request to the X server.
///
pub fn xcb_xkb_get_map (c : *mut ffi::base::xcb_connection_t,
                           deviceSpec :  xcb_xkb_device_spec_t,
                           full :  u16,
                           partial :  u16,
                           firstType :  u8,
                           nTypes :  u8,
                           firstKeySym :  ffi::xproto::xcb_keycode_t,
                           nKeySyms :  u8,
                           firstKeyAction :  ffi::xproto::xcb_keycode_t,
                           nKeyActions :  u8,
                           firstKeyBehavior :  ffi::xproto::xcb_keycode_t,
                           nKeyBehaviors :  u8,
                           virtualMods :  u16,
                           firstKeyExplicit :  ffi::xproto::xcb_keycode_t,
                           nKeyExplicit :  u8,
                           firstModMapKey :  ffi::xproto::xcb_keycode_t,
                           nModMapKeys :  u8,
                           firstVModMapKey :  ffi::xproto::xcb_keycode_t,
                           nVModMapKeys :  u8) -> xcb_xkb_get_map_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_xkb_get_map_unchecked (c : *mut ffi::base::xcb_connection_t,
                                     deviceSpec :  xcb_xkb_device_spec_t,
                                     full :  u16,
                                     partial :  u16,
                                     firstType :  u8,
                                     nTypes :  u8,
                                     firstKeySym :  ffi::xproto::xcb_keycode_t,
                                     nKeySyms :  u8,
                                     firstKeyAction :  ffi::xproto::xcb_keycode_t,
                                     nKeyActions :  u8,
                                     firstKeyBehavior :  ffi::xproto::xcb_keycode_t,
                                     nKeyBehaviors :  u8,
                                     virtualMods :  u16,
                                     firstKeyExplicit :  ffi::xproto::xcb_keycode_t,
                                     nKeyExplicit :  u8,
                                     firstModMapKey :  ffi::xproto::xcb_keycode_t,
                                     nModMapKeys :  u8,
                                     firstVModMapKey :  ffi::xproto::xcb_keycode_t,
                                     nVModMapKeys :  u8) -> xcb_xkb_get_map_cookie_t;


///
/// xcb_xkb_get_map_map : *mut xcb_xkb_get_map_map_t
///
pub fn xcb_xkb_get_map_map (R : *mut xcb_xkb_get_map_reply_t) -> *mut c_void;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_xkb_get_map_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_xkb_get_map_reply (c : *mut ffi::base::xcb_connection_t,
                                 cookie : xcb_xkb_get_map_cookie_t,
                                 e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_xkb_get_map_reply_t;

pub fn xcb_xkb_set_map_values_serialize (_buffer :              *mut *mut c_void,
                                  nTypes :                         u8,
                                  nKeySyms :                       u8,
                                  nKeyActions :                    u8,
                                  totalActions :                   u16,
                                  totalKeyBehaviors :              u8,
                                  nVModMapKeys :                   u8,
                                  totalKeyExplicit :               u8,
                                  totalModMapKeys :                u8,
                                  totalVModMapKeys :               u8,
                                  present :                        u16,
                                  _aux :                      *mut xcb_xkb_set_map_values_t) -> c_int;

pub fn xcb_xkb_set_map_values_unpack (_buffer :                   *mut c_void,
                               nTypes :                         u8,
                               nKeySyms :                       u8,
                               nKeyActions :                    u8,
                               totalActions :                   u16,
                               totalKeyBehaviors :              u8,
                               nVModMapKeys :                   u8,
                               totalKeyExplicit :               u8,
                               totalModMapKeys :                u8,
                               totalVModMapKeys :               u8,
                               present :                        u16,
                               _aux :                      *mut xcb_xkb_set_map_values_t) -> c_int;

pub fn xcb_xkb_set_map_values_sizeof (_buffer :  *mut c_void,
                               nTypes :       u8,
                               nKeySyms :     u8,
                               nKeyActions :  u8,
                               totalActions :  u16,
                               totalKeyBehaviors :  u8,
                               nVModMapKeys :  u8,
                               totalKeyExplicit :  u8,
                               totalModMapKeys :  u8,
                               totalVModMapKeys :  u8,
                               present :      u16) -> c_int;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_xkb_set_map_checked (c : *mut ffi::base::xcb_connection_t,
                                   deviceSpec :  xcb_xkb_device_spec_t,
                                   present :  u16,
                                   flags :  u16,
                                   minKeyCode :  ffi::xproto::xcb_keycode_t,
                                   maxKeyCode :  ffi::xproto::xcb_keycode_t,
                                   firstType :  u8,
                                   nTypes :  u8,
                                   firstKeySym :  ffi::xproto::xcb_keycode_t,
                                   nKeySyms :  u8,
                                   totalSyms :  u16,
                                   firstKeyAction :  ffi::xproto::xcb_keycode_t,
                                   nKeyActions :  u8,
                                   totalActions :  u16,
                                   firstKeyBehavior :  ffi::xproto::xcb_keycode_t,
                                   nKeyBehaviors :  u8,
                                   totalKeyBehaviors :  u8,
                                   firstKeyExplicit :  ffi::xproto::xcb_keycode_t,
                                   nKeyExplicit :  u8,
                                   totalKeyExplicit :  u8,
                                   firstModMapKey :  ffi::xproto::xcb_keycode_t,
                                   nModMapKeys :  u8,
                                   totalModMapKeys :  u8,
                                   firstVModMapKey :  ffi::xproto::xcb_keycode_t,
                                   nVModMapKeys :  u8,
                                   totalVModMapKeys :  u8,
                                   virtualMods :  u16,
                                   values : *mut ()) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_xkb_set_map (c : *mut ffi::base::xcb_connection_t,
                           deviceSpec :  xcb_xkb_device_spec_t,
                           present :  u16,
                           flags :  u16,
                           minKeyCode :  ffi::xproto::xcb_keycode_t,
                           maxKeyCode :  ffi::xproto::xcb_keycode_t,
                           firstType :  u8,
                           nTypes :  u8,
                           firstKeySym :  ffi::xproto::xcb_keycode_t,
                           nKeySyms :  u8,
                           totalSyms :  u16,
                           firstKeyAction :  ffi::xproto::xcb_keycode_t,
                           nKeyActions :  u8,
                           totalActions :  u16,
                           firstKeyBehavior :  ffi::xproto::xcb_keycode_t,
                           nKeyBehaviors :  u8,
                           totalKeyBehaviors :  u8,
                           firstKeyExplicit :  ffi::xproto::xcb_keycode_t,
                           nKeyExplicit :  u8,
                           totalKeyExplicit :  u8,
                           firstModMapKey :  ffi::xproto::xcb_keycode_t,
                           nModMapKeys :  u8,
                           totalModMapKeys :  u8,
                           firstVModMapKey :  ffi::xproto::xcb_keycode_t,
                           nVModMapKeys :  u8,
                           totalVModMapKeys :  u8,
                           virtualMods :  u16,
                           values : *mut ()) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_xkb_set_map_aux_checked (c : *mut ffi::base::xcb_connection_t,
                                       deviceSpec :  xcb_xkb_device_spec_t,
                                       present :  u16,
                                       flags :  u16,
                                       minKeyCode :  ffi::xproto::xcb_keycode_t,
                                       maxKeyCode :  ffi::xproto::xcb_keycode_t,
                                       firstType :  u8,
                                       nTypes :  u8,
                                       firstKeySym :  ffi::xproto::xcb_keycode_t,
                                       nKeySyms :  u8,
                                       totalSyms :  u16,
                                       firstKeyAction :  ffi::xproto::xcb_keycode_t,
                                       nKeyActions :  u8,
                                       totalActions :  u16,
                                       firstKeyBehavior :  ffi::xproto::xcb_keycode_t,
                                       nKeyBehaviors :  u8,
                                       totalKeyBehaviors :  u8,
                                       firstKeyExplicit :  ffi::xproto::xcb_keycode_t,
                                       nKeyExplicit :  u8,
                                       totalKeyExplicit :  u8,
                                       firstModMapKey :  ffi::xproto::xcb_keycode_t,
                                       nModMapKeys :  u8,
                                       totalModMapKeys :  u8,
                                       firstVModMapKey :  ffi::xproto::xcb_keycode_t,
                                       nVModMapKeys :  u8,
                                       totalVModMapKeys :  u8,
                                       virtualMods :  u16,
                                       values : *mut xcb_xkb_set_map_values_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_xkb_set_map_aux (c : *mut ffi::base::xcb_connection_t,
                               deviceSpec :  xcb_xkb_device_spec_t,
                               present :  u16,
                               flags :  u16,
                               minKeyCode :  ffi::xproto::xcb_keycode_t,
                               maxKeyCode :  ffi::xproto::xcb_keycode_t,
                               firstType :  u8,
                               nTypes :  u8,
                               firstKeySym :  ffi::xproto::xcb_keycode_t,
                               nKeySyms :  u8,
                               totalSyms :  u16,
                               firstKeyAction :  ffi::xproto::xcb_keycode_t,
                               nKeyActions :  u8,
                               totalActions :  u16,
                               firstKeyBehavior :  ffi::xproto::xcb_keycode_t,
                               nKeyBehaviors :  u8,
                               totalKeyBehaviors :  u8,
                               firstKeyExplicit :  ffi::xproto::xcb_keycode_t,
                               nKeyExplicit :  u8,
                               totalKeyExplicit :  u8,
                               firstModMapKey :  ffi::xproto::xcb_keycode_t,
                               nModMapKeys :  u8,
                               totalModMapKeys :  u8,
                               firstVModMapKey :  ffi::xproto::xcb_keycode_t,
                               nVModMapKeys :  u8,
                               totalVModMapKeys :  u8,
                               virtualMods :  u16,
                               values : *mut xcb_xkb_set_map_values_t) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_xkb_get_compat_map_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
///
pub fn xcb_xkb_get_compat_map (c : *mut ffi::base::xcb_connection_t,
                                  deviceSpec :  xcb_xkb_device_spec_t,
                                  groups :  u8,
                                  getAllSI :  u8,
                                  firstSI :  u16,
                                  nSI :  u16) -> xcb_xkb_get_compat_map_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_xkb_get_compat_map_unchecked (c : *mut ffi::base::xcb_connection_t,
                                            deviceSpec :  xcb_xkb_device_spec_t,
                                            groups :  u8,
                                            getAllSI :  u8,
                                            firstSI :  u16,
                                            nSI :  u16) -> xcb_xkb_get_compat_map_cookie_t;

pub fn xcb_xkb_get_compat_map_si_rtrn (R : *mut xcb_xkb_get_compat_map_reply_t) -> *mut u8;


pub fn xcb_xkb_get_compat_map_si_rtrn_length (R : *mut xcb_xkb_get_compat_map_reply_t) -> c_int;


pub fn xcb_xkb_get_compat_map_si_rtrn_end (R : *mut xcb_xkb_get_compat_map_reply_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_xkb_get_compat_map_group_rtrn (R : *mut xcb_xkb_get_compat_map_reply_t) -> *mut xcb_xkb_mod_def_t;


pub fn xcb_xkb_get_compat_map_group_rtrn_length (R : *mut xcb_xkb_get_compat_map_reply_t) -> c_int;

pub fn xcb_xkb_get_compat_map_group_rtrn_iterator (R : *mut xcb_xkb_get_compat_map_reply_t) -> xcb_xkb_mod_def_iterator_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_xkb_get_compat_map_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_xkb_get_compat_map_reply (c : *mut ffi::base::xcb_connection_t,
                                        cookie : xcb_xkb_get_compat_map_cookie_t,
                                        e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_xkb_get_compat_map_reply_t;

pub fn xcb_xkb_set_compat_map_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_xkb_set_compat_map_checked (c : *mut ffi::base::xcb_connection_t,
                                          deviceSpec :  xcb_xkb_device_spec_t,
                                          recomputeActions :  u8,
                                          truncateSI :  u8,
                                          groups :  u8,
                                          firstSI :  u16,
                                          nSI :  u16,
                                          si : *mut u8,
                                          groupMaps : *mut xcb_xkb_mod_def_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_xkb_set_compat_map (c : *mut ffi::base::xcb_connection_t,
                                  deviceSpec :  xcb_xkb_device_spec_t,
                                  recomputeActions :  u8,
                                  truncateSI :  u8,
                                  groups :  u8,
                                  firstSI :  u16,
                                  nSI :  u16,
                                  si : *mut u8,
                                  groupMaps : *mut xcb_xkb_mod_def_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_xkb_get_indicator_state (c : *mut ffi::base::xcb_connection_t,
                                       deviceSpec :  xcb_xkb_device_spec_t) -> xcb_xkb_get_indicator_state_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_xkb_get_indicator_state_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                 deviceSpec :  xcb_xkb_device_spec_t) -> xcb_xkb_get_indicator_state_cookie_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_xkb_get_indicator_state_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_xkb_get_indicator_state_reply (c : *mut ffi::base::xcb_connection_t,
                                             cookie : xcb_xkb_get_indicator_state_cookie_t,
                                             e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_xkb_get_indicator_state_reply_t;

pub fn xcb_xkb_get_indicator_map_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
///
pub fn xcb_xkb_get_indicator_map (c : *mut ffi::base::xcb_connection_t,
                                     deviceSpec :  xcb_xkb_device_spec_t,
                                     which :  u32) -> xcb_xkb_get_indicator_map_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_xkb_get_indicator_map_unchecked (c : *mut ffi::base::xcb_connection_t,
                                               deviceSpec :  xcb_xkb_device_spec_t,
                                               which :  u32) -> xcb_xkb_get_indicator_map_cookie_t;

pub fn xcb_xkb_get_indicator_map_maps (R : *mut xcb_xkb_get_indicator_map_reply_t) -> *mut xcb_xkb_indicator_map_t;


pub fn xcb_xkb_get_indicator_map_maps_length (R : *mut xcb_xkb_get_indicator_map_reply_t) -> c_int;

pub fn xcb_xkb_get_indicator_map_maps_iterator (R : *mut xcb_xkb_get_indicator_map_reply_t) -> xcb_xkb_indicator_map_iterator_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_xkb_get_indicator_map_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_xkb_get_indicator_map_reply (c : *mut ffi::base::xcb_connection_t,
                                           cookie : xcb_xkb_get_indicator_map_cookie_t,
                                           e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_xkb_get_indicator_map_reply_t;

pub fn xcb_xkb_set_indicator_map_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_xkb_set_indicator_map_checked (c : *mut ffi::base::xcb_connection_t,
                                             deviceSpec :  xcb_xkb_device_spec_t,
                                             which :  u32,
                                             maps : *mut xcb_xkb_indicator_map_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_xkb_set_indicator_map (c : *mut ffi::base::xcb_connection_t,
                                     deviceSpec :  xcb_xkb_device_spec_t,
                                     which :  u32,
                                     maps : *mut xcb_xkb_indicator_map_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_xkb_get_named_indicator (c : *mut ffi::base::xcb_connection_t,
                                       deviceSpec :  xcb_xkb_device_spec_t,
                                       ledClass :  xcb_xkb_led_class_spec_t,
                                       ledID :  xcb_xkb_id_spec_t,
                                       indicator :  ffi::xproto::xcb_atom_t) -> xcb_xkb_get_named_indicator_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_xkb_get_named_indicator_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                 deviceSpec :  xcb_xkb_device_spec_t,
                                                 ledClass :  xcb_xkb_led_class_spec_t,
                                                 ledID :  xcb_xkb_id_spec_t,
                                                 indicator :  ffi::xproto::xcb_atom_t) -> xcb_xkb_get_named_indicator_cookie_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_xkb_get_named_indicator_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_xkb_get_named_indicator_reply (c : *mut ffi::base::xcb_connection_t,
                                             cookie : xcb_xkb_get_named_indicator_cookie_t,
                                             e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_xkb_get_named_indicator_reply_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_xkb_set_named_indicator_checked (c : *mut ffi::base::xcb_connection_t,
                                               deviceSpec :  xcb_xkb_device_spec_t,
                                               ledClass :  xcb_xkb_led_class_spec_t,
                                               ledID :  xcb_xkb_id_spec_t,
                                               indicator :  ffi::xproto::xcb_atom_t,
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
                                               map_ctrls :  u32) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_xkb_set_named_indicator (c : *mut ffi::base::xcb_connection_t,
                                       deviceSpec :  xcb_xkb_device_spec_t,
                                       ledClass :  xcb_xkb_led_class_spec_t,
                                       ledID :  xcb_xkb_id_spec_t,
                                       indicator :  ffi::xproto::xcb_atom_t,
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
                                       map_ctrls :  u32) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_xkb_get_names_value_list_serialize (_buffer :                    *mut *mut c_void,
                                        nTypes :                               u8,
                                        indicators :                           u32,
                                        virtualMods :                          u16,
                                        groupNames :                           u8,
                                        nKeys :                                u8,
                                        nKeyAliases :                          u8,
                                        nRadioGroups :                         u8,
                                        which :                                u32,
                                        _aux :                            *mut xcb_xkb_get_names_value_list_t) -> c_int;

pub fn xcb_xkb_get_names_value_list_unpack (_buffer :                         *mut c_void,
                                     nTypes :                               u8,
                                     indicators :                           u32,
                                     virtualMods :                          u16,
                                     groupNames :                           u8,
                                     nKeys :                                u8,
                                     nKeyAliases :                          u8,
                                     nRadioGroups :                         u8,
                                     which :                                u32,
                                     _aux :                            *mut xcb_xkb_get_names_value_list_t) -> c_int;

pub fn xcb_xkb_get_names_value_list_sizeof (_buffer :  *mut c_void,
                                     nTypes :       u8,
                                     indicators :   u32,
                                     virtualMods :  u16,
                                     groupNames :   u8,
                                     nKeys :        u8,
                                     nKeyAliases :  u8,
                                     nRadioGroups :  u8,
                                     which :        u32) -> c_int;

/// Delivers a request to the X server.
///
pub fn xcb_xkb_get_names (c : *mut ffi::base::xcb_connection_t,
                             deviceSpec :  xcb_xkb_device_spec_t,
                             which :  u32) -> xcb_xkb_get_names_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_xkb_get_names_unchecked (c : *mut ffi::base::xcb_connection_t,
                                       deviceSpec :  xcb_xkb_device_spec_t,
                                       which :  u32) -> xcb_xkb_get_names_cookie_t;


///
/// xcb_xkb_get_names_value_list : *mut xcb_xkb_get_names_value_list_t
///
pub fn xcb_xkb_get_names_value_list (R : *mut xcb_xkb_get_names_reply_t) -> *mut c_void;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_xkb_get_names_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_xkb_get_names_reply (c : *mut ffi::base::xcb_connection_t,
                                   cookie : xcb_xkb_get_names_cookie_t,
                                   e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_xkb_get_names_reply_t;

pub fn xcb_xkb_set_names_values_serialize (_buffer :                *mut *mut c_void,
                                    nTypes :                           u8,
                                    nKTLevels :                        u8,
                                    indicators :                       u32,
                                    virtualMods :                      u16,
                                    groupNames :                       u8,
                                    nKeys :                            u8,
                                    nKeyAliases :                      u8,
                                    nRadioGroups :                     u8,
                                    which :                            u32,
                                    _aux :                        *mut xcb_xkb_set_names_values_t) -> c_int;

pub fn xcb_xkb_set_names_values_unpack (_buffer :                     *mut c_void,
                                 nTypes :                           u8,
                                 nKTLevels :                        u8,
                                 indicators :                       u32,
                                 virtualMods :                      u16,
                                 groupNames :                       u8,
                                 nKeys :                            u8,
                                 nKeyAliases :                      u8,
                                 nRadioGroups :                     u8,
                                 which :                            u32,
                                 _aux :                        *mut xcb_xkb_set_names_values_t) -> c_int;

pub fn xcb_xkb_set_names_values_sizeof (_buffer :  *mut c_void,
                                 nTypes :       u8,
                                 nKTLevels :    u8,
                                 indicators :   u32,
                                 virtualMods :  u16,
                                 groupNames :   u8,
                                 nKeys :        u8,
                                 nKeyAliases :  u8,
                                 nRadioGroups :  u8,
                                 which :        u32) -> c_int;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_xkb_set_names_checked (c : *mut ffi::base::xcb_connection_t,
                                     deviceSpec :  xcb_xkb_device_spec_t,
                                     virtualMods :  u16,
                                     which :  u32,
                                     firstType :  u8,
                                     nTypes :  u8,
                                     firstKTLevelt :  u8,
                                     nKTLevels :  u8,
                                     indicators :  u32,
                                     groupNames :  u8,
                                     nRadioGroups :  u8,
                                     firstKey :  ffi::xproto::xcb_keycode_t,
                                     nKeys :  u8,
                                     nKeyAliases :  u8,
                                     totalKTLevelNames :  u16,
                                     values : *mut ()) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_xkb_set_names (c : *mut ffi::base::xcb_connection_t,
                             deviceSpec :  xcb_xkb_device_spec_t,
                             virtualMods :  u16,
                             which :  u32,
                             firstType :  u8,
                             nTypes :  u8,
                             firstKTLevelt :  u8,
                             nKTLevels :  u8,
                             indicators :  u32,
                             groupNames :  u8,
                             nRadioGroups :  u8,
                             firstKey :  ffi::xproto::xcb_keycode_t,
                             nKeys :  u8,
                             nKeyAliases :  u8,
                             totalKTLevelNames :  u16,
                             values : *mut ()) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_xkb_set_names_aux_checked (c : *mut ffi::base::xcb_connection_t,
                                         deviceSpec :  xcb_xkb_device_spec_t,
                                         virtualMods :  u16,
                                         which :  u32,
                                         firstType :  u8,
                                         nTypes :  u8,
                                         firstKTLevelt :  u8,
                                         nKTLevels :  u8,
                                         indicators :  u32,
                                         groupNames :  u8,
                                         nRadioGroups :  u8,
                                         firstKey :  ffi::xproto::xcb_keycode_t,
                                         nKeys :  u8,
                                         nKeyAliases :  u8,
                                         totalKTLevelNames :  u16,
                                         values : *mut xcb_xkb_set_names_values_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_xkb_set_names_aux (c : *mut ffi::base::xcb_connection_t,
                                 deviceSpec :  xcb_xkb_device_spec_t,
                                 virtualMods :  u16,
                                 which :  u32,
                                 firstType :  u8,
                                 nTypes :  u8,
                                 firstKTLevelt :  u8,
                                 nKTLevels :  u8,
                                 indicators :  u32,
                                 groupNames :  u8,
                                 nRadioGroups :  u8,
                                 firstKey :  ffi::xproto::xcb_keycode_t,
                                 nKeys :  u8,
                                 nKeyAliases :  u8,
                                 totalKTLevelNames :  u16,
                                 values : *mut xcb_xkb_set_names_values_t) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_xkb_get_geometry_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
///
pub fn xcb_xkb_get_geometry (c : *mut ffi::base::xcb_connection_t,
                                deviceSpec :  xcb_xkb_device_spec_t,
                                name :  ffi::xproto::xcb_atom_t) -> xcb_xkb_get_geometry_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_xkb_get_geometry_unchecked (c : *mut ffi::base::xcb_connection_t,
                                          deviceSpec :  xcb_xkb_device_spec_t,
                                          name :  ffi::xproto::xcb_atom_t) -> xcb_xkb_get_geometry_cookie_t;


///
/// xcb_xkb_get_geometry_label_font : *mut xcb_xkb_counted_string_16_t
///
pub fn xcb_xkb_get_geometry_label_font (R : *mut xcb_xkb_get_geometry_reply_t) -> *mut xcb_xkb_counted_string_16_t;


pub fn xcb_xkb_get_geometry_properties_length (R : *mut xcb_xkb_get_geometry_reply_t) -> c_int;

pub fn xcb_xkb_get_geometry_properties_iterator (R : *mut xcb_xkb_get_geometry_reply_t) -> xcb_xkb_property_iterator_t;


pub fn xcb_xkb_get_geometry_colors_length (R : *mut xcb_xkb_get_geometry_reply_t) -> c_int;

pub fn xcb_xkb_get_geometry_colors_iterator (R : *mut xcb_xkb_get_geometry_reply_t) -> xcb_xkb_counted_string_16_iterator_t;


pub fn xcb_xkb_get_geometry_shapes_length (R : *mut xcb_xkb_get_geometry_reply_t) -> c_int;

pub fn xcb_xkb_get_geometry_shapes_iterator (R : *mut xcb_xkb_get_geometry_reply_t) -> xcb_xkb_shape_iterator_t;


pub fn xcb_xkb_get_geometry_sections_length (R : *mut xcb_xkb_get_geometry_reply_t) -> c_int;

pub fn xcb_xkb_get_geometry_sections_iterator (R : *mut xcb_xkb_get_geometry_reply_t) -> xcb_xkb_section_iterator_t;


pub fn xcb_xkb_get_geometry_doodads_length (R : *mut xcb_xkb_get_geometry_reply_t) -> c_int;

pub fn xcb_xkb_get_geometry_doodads_iterator (R : *mut xcb_xkb_get_geometry_reply_t) -> xcb_xkb_doodad_iterator_t;

pub fn xcb_xkb_get_geometry_key_aliases (R : *mut xcb_xkb_get_geometry_reply_t) -> *mut xcb_xkb_key_alias_t;


pub fn xcb_xkb_get_geometry_key_aliases_length (R : *mut xcb_xkb_get_geometry_reply_t) -> c_int;

pub fn xcb_xkb_get_geometry_key_aliases_iterator (R : *mut xcb_xkb_get_geometry_reply_t) -> xcb_xkb_key_alias_iterator_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_xkb_get_geometry_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_xkb_get_geometry_reply (c : *mut ffi::base::xcb_connection_t,
                                      cookie : xcb_xkb_get_geometry_cookie_t,
                                      e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_xkb_get_geometry_reply_t;

pub fn xcb_xkb_set_geometry_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_xkb_set_geometry_checked (c : *mut ffi::base::xcb_connection_t,
                                        deviceSpec :  xcb_xkb_device_spec_t,
                                        nShapes :  u8,
                                        nSections :  u8,
                                        name :  ffi::xproto::xcb_atom_t,
                                        widthMM :  u16,
                                        heightMM :  u16,
                                        nProperties :  u16,
                                        nColors :  u16,
                                        nDoodads :  u16,
                                        nKeyAliases :  u16,
                                        baseColorNdx :  u8,
                                        labelColorNdx :  u8,
                                        labelFont : *mut xcb_xkb_counted_string_16_t,
                                        properties : *mut xcb_xkb_property_t,
                                        colors : *mut xcb_xkb_counted_string_16_t,
                                        shapes : *mut xcb_xkb_shape_t,
                                        sections : *mut xcb_xkb_section_t,
                                        doodads : *mut xcb_xkb_doodad_t,
                                        keyAliases : *mut xcb_xkb_key_alias_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_xkb_set_geometry (c : *mut ffi::base::xcb_connection_t,
                                deviceSpec :  xcb_xkb_device_spec_t,
                                nShapes :  u8,
                                nSections :  u8,
                                name :  ffi::xproto::xcb_atom_t,
                                widthMM :  u16,
                                heightMM :  u16,
                                nProperties :  u16,
                                nColors :  u16,
                                nDoodads :  u16,
                                nKeyAliases :  u16,
                                baseColorNdx :  u8,
                                labelColorNdx :  u8,
                                labelFont : *mut xcb_xkb_counted_string_16_t,
                                properties : *mut xcb_xkb_property_t,
                                colors : *mut xcb_xkb_counted_string_16_t,
                                shapes : *mut xcb_xkb_shape_t,
                                sections : *mut xcb_xkb_section_t,
                                doodads : *mut xcb_xkb_doodad_t,
                                keyAliases : *mut xcb_xkb_key_alias_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_xkb_per_client_flags (c : *mut ffi::base::xcb_connection_t,
                                    deviceSpec :  xcb_xkb_device_spec_t,
                                    change :  u32,
                                    value :  u32,
                                    ctrlsToChange :  u32,
                                    autoCtrls :  u32,
                                    autoCtrlsValues :  u32) -> xcb_xkb_per_client_flags_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_xkb_per_client_flags_unchecked (c : *mut ffi::base::xcb_connection_t,
                                              deviceSpec :  xcb_xkb_device_spec_t,
                                              change :  u32,
                                              value :  u32,
                                              ctrlsToChange :  u32,
                                              autoCtrls :  u32,
                                              autoCtrlsValues :  u32) -> xcb_xkb_per_client_flags_cookie_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_xkb_per_client_flags_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_xkb_per_client_flags_reply (c : *mut ffi::base::xcb_connection_t,
                                          cookie : xcb_xkb_per_client_flags_cookie_t,
                                          e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_xkb_per_client_flags_reply_t;

pub fn xcb_xkb_list_components_serialize (_buffer :                       *mut *mut c_void,
                                   _aux :                               *mut xcb_xkb_list_components_request_t,
                                   keymapsSpec :                        *mut xcb_xkb_string8_t,
                                   keycodesSpec :                       *mut xcb_xkb_string8_t,
                                   typesSpec :                          *mut xcb_xkb_string8_t,
                                   compatMapSpec :                      *mut xcb_xkb_string8_t,
                                   symbolsSpec :                        *mut xcb_xkb_string8_t,
                                   geometrySpec :                       *mut xcb_xkb_string8_t) -> c_int;

pub fn xcb_xkb_list_components_unserialize (_buffer :                                 *mut c_void,
                                     _aux :                               *mut *mut xcb_xkb_list_components_request_t) -> c_int;

pub fn xcb_xkb_list_components_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
///
pub fn xcb_xkb_list_components (c : *mut ffi::base::xcb_connection_t,
                                   deviceSpec :  xcb_xkb_device_spec_t,
                                   maxNames :  u16,
                                   keymapsSpecLen :  u8,
                                   keymapsSpec : *mut xcb_xkb_string8_t,
                                   keycodesSpecLen :  u8,
                                   keycodesSpec : *mut xcb_xkb_string8_t,
                                   typesSpecLen :  u8,
                                   typesSpec : *mut xcb_xkb_string8_t,
                                   compatMapSpecLen :  u8,
                                   compatMapSpec : *mut xcb_xkb_string8_t,
                                   symbolsSpecLen :  u8,
                                   symbolsSpec : *mut xcb_xkb_string8_t,
                                   geometrySpecLen :  u8,
                                   geometrySpec : *mut xcb_xkb_string8_t) -> xcb_xkb_list_components_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_xkb_list_components_unchecked (c : *mut ffi::base::xcb_connection_t,
                                             deviceSpec :  xcb_xkb_device_spec_t,
                                             maxNames :  u16,
                                             keymapsSpecLen :  u8,
                                             keymapsSpec : *mut xcb_xkb_string8_t,
                                             keycodesSpecLen :  u8,
                                             keycodesSpec : *mut xcb_xkb_string8_t,
                                             typesSpecLen :  u8,
                                             typesSpec : *mut xcb_xkb_string8_t,
                                             compatMapSpecLen :  u8,
                                             compatMapSpec : *mut xcb_xkb_string8_t,
                                             symbolsSpecLen :  u8,
                                             symbolsSpec : *mut xcb_xkb_string8_t,
                                             geometrySpecLen :  u8,
                                             geometrySpec : *mut xcb_xkb_string8_t) -> xcb_xkb_list_components_cookie_t;


pub fn xcb_xkb_list_components_keymaps_length (R : *mut xcb_xkb_list_components_reply_t) -> c_int;

pub fn xcb_xkb_list_components_keymaps_iterator (R : *mut xcb_xkb_list_components_reply_t) -> xcb_xkb_listing_iterator_t;


pub fn xcb_xkb_list_components_keycodes_length (R : *mut xcb_xkb_list_components_reply_t) -> c_int;

pub fn xcb_xkb_list_components_keycodes_iterator (R : *mut xcb_xkb_list_components_reply_t) -> xcb_xkb_listing_iterator_t;


pub fn xcb_xkb_list_components_types_length (R : *mut xcb_xkb_list_components_reply_t) -> c_int;

pub fn xcb_xkb_list_components_types_iterator (R : *mut xcb_xkb_list_components_reply_t) -> xcb_xkb_listing_iterator_t;


pub fn xcb_xkb_list_components_compat_maps_length (R : *mut xcb_xkb_list_components_reply_t) -> c_int;

pub fn xcb_xkb_list_components_compat_maps_iterator (R : *mut xcb_xkb_list_components_reply_t) -> xcb_xkb_listing_iterator_t;


pub fn xcb_xkb_list_components_symbols_length (R : *mut xcb_xkb_list_components_reply_t) -> c_int;

pub fn xcb_xkb_list_components_symbols_iterator (R : *mut xcb_xkb_list_components_reply_t) -> xcb_xkb_listing_iterator_t;


pub fn xcb_xkb_list_components_geometries_length (R : *mut xcb_xkb_list_components_reply_t) -> c_int;

pub fn xcb_xkb_list_components_geometries_iterator (R : *mut xcb_xkb_list_components_reply_t) -> xcb_xkb_listing_iterator_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_xkb_list_components_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_xkb_list_components_reply (c : *mut ffi::base::xcb_connection_t,
                                         cookie : xcb_xkb_list_components_cookie_t,
                                         e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_xkb_list_components_reply_t;

pub fn xcb_xkb_get_kbd_by_name_serialize (_buffer :                       *mut *mut c_void,
                                   _aux :                               *mut xcb_xkb_get_kbd_by_name_request_t,
                                   keymapsSpec :                        *mut xcb_xkb_string8_t,
                                   keycodesSpec :                       *mut xcb_xkb_string8_t,
                                   typesSpec :                          *mut xcb_xkb_string8_t,
                                   compatMapSpec :                      *mut xcb_xkb_string8_t,
                                   symbolsSpec :                        *mut xcb_xkb_string8_t,
                                   geometrySpec :                       *mut xcb_xkb_string8_t) -> c_int;

pub fn xcb_xkb_get_kbd_by_name_unserialize (_buffer :                                 *mut c_void,
                                     _aux :                               *mut *mut xcb_xkb_get_kbd_by_name_request_t) -> c_int;

pub fn xcb_xkb_get_kbd_by_name_sizeof (_buffer :  *mut c_void) -> c_int;

pub fn xcb_xkb_get_kbd_by_name_replies_types_map_serialize (_buffer :                                 *mut *mut c_void,
                                                     nTypes :                                            u8,
                                                     nKeySyms :                                          u8,
                                                     nKeyActions :                                       u8,
                                                     totalActions :                                      u16,
                                                     totalKeyBehaviors :                                 u8,
                                                     nVModMapKeys :                                      u8,
                                                     totalKeyExplicit :                                  u8,
                                                     totalModMapKeys :                                   u8,
                                                     totalVModMapKeys :                                  u8,
                                                     present :                                           u16,
                                                     _aux :                                         *mut xcb_xkb_get_kbd_by_name_replies_types_map_t) -> c_int;

pub fn xcb_xkb_get_kbd_by_name_replies_types_map_unpack (_buffer :                                      *mut c_void,
                                                  nTypes :                                            u8,
                                                  nKeySyms :                                          u8,
                                                  nKeyActions :                                       u8,
                                                  totalActions :                                      u16,
                                                  totalKeyBehaviors :                                 u8,
                                                  nVModMapKeys :                                      u8,
                                                  totalKeyExplicit :                                  u8,
                                                  totalModMapKeys :                                   u8,
                                                  totalVModMapKeys :                                  u8,
                                                  present :                                           u16,
                                                  _aux :                                         *mut xcb_xkb_get_kbd_by_name_replies_types_map_t) -> c_int;

pub fn xcb_xkb_get_kbd_by_name_replies_types_map_sizeof (_buffer :  *mut c_void,
                                                  nTypes :       u8,
                                                  nKeySyms :     u8,
                                                  nKeyActions :  u8,
                                                  totalActions :  u16,
                                                  totalKeyBehaviors :  u8,
                                                  nVModMapKeys :  u8,
                                                  totalKeyExplicit :  u8,
                                                  totalModMapKeys :  u8,
                                                  totalVModMapKeys :  u8,
                                                  present :      u16) -> c_int;

pub fn xcb_xkb_get_kbd_by_name_replies_client_symbols_map_serialize (_buffer :                                          *mut *mut c_void,
                                                              nTypes :                                                     u8,
                                                              nKeySyms :                                                   u8,
                                                              nKeyActions :                                                u8,
                                                              totalActions :                                               u16,
                                                              totalKeyBehaviors :                                          u8,
                                                              nVModMapKeys :                                               u8,
                                                              totalKeyExplicit :                                           u8,
                                                              totalModMapKeys :                                            u8,
                                                              totalVModMapKeys :                                           u8,
                                                              present :                                                    u16,
                                                              _aux :                                                  *mut xcb_xkb_get_kbd_by_name_replies_client_symbols_map_t) -> c_int;

pub fn xcb_xkb_get_kbd_by_name_replies_client_symbols_map_unpack (_buffer :                                               *mut c_void,
                                                           nTypes :                                                     u8,
                                                           nKeySyms :                                                   u8,
                                                           nKeyActions :                                                u8,
                                                           totalActions :                                               u16,
                                                           totalKeyBehaviors :                                          u8,
                                                           nVModMapKeys :                                               u8,
                                                           totalKeyExplicit :                                           u8,
                                                           totalModMapKeys :                                            u8,
                                                           totalVModMapKeys :                                           u8,
                                                           present :                                                    u16,
                                                           _aux :                                                  *mut xcb_xkb_get_kbd_by_name_replies_client_symbols_map_t) -> c_int;

pub fn xcb_xkb_get_kbd_by_name_replies_client_symbols_map_sizeof (_buffer :  *mut c_void,
                                                           nTypes :       u8,
                                                           nKeySyms :     u8,
                                                           nKeyActions :  u8,
                                                           totalActions :  u16,
                                                           totalKeyBehaviors :  u8,
                                                           nVModMapKeys :  u8,
                                                           totalKeyExplicit :  u8,
                                                           totalModMapKeys :  u8,
                                                           totalVModMapKeys :  u8,
                                                           present :      u16) -> c_int;

pub fn xcb_xkb_get_kbd_by_name_replies_server_symbols_map_serialize (_buffer :                                          *mut *mut c_void,
                                                              nTypes :                                                     u8,
                                                              nKeySyms :                                                   u8,
                                                              nKeyActions :                                                u8,
                                                              totalActions :                                               u16,
                                                              totalKeyBehaviors :                                          u8,
                                                              nVModMapKeys :                                               u8,
                                                              totalKeyExplicit :                                           u8,
                                                              totalModMapKeys :                                            u8,
                                                              totalVModMapKeys :                                           u8,
                                                              present :                                                    u16,
                                                              _aux :                                                  *mut xcb_xkb_get_kbd_by_name_replies_server_symbols_map_t) -> c_int;

pub fn xcb_xkb_get_kbd_by_name_replies_server_symbols_map_unpack (_buffer :                                               *mut c_void,
                                                           nTypes :                                                     u8,
                                                           nKeySyms :                                                   u8,
                                                           nKeyActions :                                                u8,
                                                           totalActions :                                               u16,
                                                           totalKeyBehaviors :                                          u8,
                                                           nVModMapKeys :                                               u8,
                                                           totalKeyExplicit :                                           u8,
                                                           totalModMapKeys :                                            u8,
                                                           totalVModMapKeys :                                           u8,
                                                           present :                                                    u16,
                                                           _aux :                                                  *mut xcb_xkb_get_kbd_by_name_replies_server_symbols_map_t) -> c_int;

pub fn xcb_xkb_get_kbd_by_name_replies_server_symbols_map_sizeof (_buffer :  *mut c_void,
                                                           nTypes :       u8,
                                                           nKeySyms :     u8,
                                                           nKeyActions :  u8,
                                                           totalActions :  u16,
                                                           totalKeyBehaviors :  u8,
                                                           nVModMapKeys :  u8,
                                                           totalKeyExplicit :  u8,
                                                           totalModMapKeys :  u8,
                                                           totalVModMapKeys :  u8,
                                                           present :      u16) -> c_int;

pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_serialize (_buffer :                                            *mut *mut c_void,
                                                                nTypes :                                                       u8,
                                                                nKTLevels :                                                    u16,
                                                                indicators :                                                   u32,
                                                                virtualMods :                                                  u16,
                                                                groupNames :                                                   u8,
                                                                nKeys :                                                        u8,
                                                                nKeyAliases :                                                  u8,
                                                                nRadioGroups :                                                 u8,
                                                                which :                                                        u32,
                                                                _aux :                                                    *mut xcb_xkb_get_kbd_by_name_replies_key_names_value_list_t) -> c_int;

pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_unpack (_buffer :                                                 *mut c_void,
                                                             nTypes :                                                       u8,
                                                             nKTLevels :                                                    u16,
                                                             indicators :                                                   u32,
                                                             virtualMods :                                                  u16,
                                                             groupNames :                                                   u8,
                                                             nKeys :                                                        u8,
                                                             nKeyAliases :                                                  u8,
                                                             nRadioGroups :                                                 u8,
                                                             which :                                                        u32,
                                                             _aux :                                                    *mut xcb_xkb_get_kbd_by_name_replies_key_names_value_list_t) -> c_int;

pub fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_sizeof (_buffer :  *mut c_void,
                                                             nTypes :       u8,
                                                             nKTLevels :    u16,
                                                             indicators :   u32,
                                                             virtualMods :  u16,
                                                             groupNames :   u8,
                                                             nKeys :        u8,
                                                             nKeyAliases :  u8,
                                                             nRadioGroups :  u8,
                                                             which :        u32) -> c_int;

pub fn xcb_xkb_get_kbd_by_name_replies_other_names_value_list_serialize (_buffer :                                              *mut *mut c_void,
                                                                  nTypes :                                                         u8,
                                                                  nKTLevels :                                                      u16,
                                                                  indicators :                                                     u32,
                                                                  virtualMods :                                                    u16,
                                                                  groupNames :                                                     u8,
                                                                  nKeys :                                                          u8,
                                                                  nKeyAliases :                                                    u8,
                                                                  nRadioGroups :                                                   u8,
                                                                  which :                                                          u32,
                                                                  _aux :                                                      *mut xcb_xkb_get_kbd_by_name_replies_other_names_value_list_t) -> c_int;

pub fn xcb_xkb_get_kbd_by_name_replies_other_names_value_list_unpack (_buffer :                                                   *mut c_void,
                                                               nTypes :                                                         u8,
                                                               nKTLevels :                                                      u16,
                                                               indicators :                                                     u32,
                                                               virtualMods :                                                    u16,
                                                               groupNames :                                                     u8,
                                                               nKeys :                                                          u8,
                                                               nKeyAliases :                                                    u8,
                                                               nRadioGroups :                                                   u8,
                                                               which :                                                          u32,
                                                               _aux :                                                      *mut xcb_xkb_get_kbd_by_name_replies_other_names_value_list_t) -> c_int;

pub fn xcb_xkb_get_kbd_by_name_replies_other_names_value_list_sizeof (_buffer :  *mut c_void,
                                                               nTypes :       u8,
                                                               nKTLevels :    u16,
                                                               indicators :   u32,
                                                               virtualMods :  u16,
                                                               groupNames :   u8,
                                                               nKeys :        u8,
                                                               nKeyAliases :  u8,
                                                               nRadioGroups :  u8,
                                                               which :        u32) -> c_int;

pub fn xcb_xkb_get_kbd_by_name_replies_serialize (_buffer :                       *mut *mut c_void,
                                           reported :                                u16,
                                           _aux :                               *mut xcb_xkb_get_kbd_by_name_replies_t) -> c_int;

pub fn xcb_xkb_get_kbd_by_name_replies_unpack (_buffer :                            *mut c_void,
                                        reported :                                u16,
                                        _aux :                               *mut xcb_xkb_get_kbd_by_name_replies_t) -> c_int;

pub fn xcb_xkb_get_kbd_by_name_replies_sizeof (_buffer :  *mut c_void,
                                        reported :     u16) -> c_int;

/// Delivers a request to the X server.
///
pub fn xcb_xkb_get_kbd_by_name (c : *mut ffi::base::xcb_connection_t,
                                   deviceSpec :  xcb_xkb_device_spec_t,
                                   need :  u16,
                                   want :  u16,
                                   load :  u8,
                                   keymapsSpecLen :  u8,
                                   keymapsSpec : *mut xcb_xkb_string8_t,
                                   keycodesSpecLen :  u8,
                                   keycodesSpec : *mut xcb_xkb_string8_t,
                                   typesSpecLen :  u8,
                                   typesSpec : *mut xcb_xkb_string8_t,
                                   compatMapSpecLen :  u8,
                                   compatMapSpec : *mut xcb_xkb_string8_t,
                                   symbolsSpecLen :  u8,
                                   symbolsSpec : *mut xcb_xkb_string8_t,
                                   geometrySpecLen :  u8,
                                   geometrySpec : *mut xcb_xkb_string8_t) -> xcb_xkb_get_kbd_by_name_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_xkb_get_kbd_by_name_unchecked (c : *mut ffi::base::xcb_connection_t,
                                             deviceSpec :  xcb_xkb_device_spec_t,
                                             need :  u16,
                                             want :  u16,
                                             load :  u8,
                                             keymapsSpecLen :  u8,
                                             keymapsSpec : *mut xcb_xkb_string8_t,
                                             keycodesSpecLen :  u8,
                                             keycodesSpec : *mut xcb_xkb_string8_t,
                                             typesSpecLen :  u8,
                                             typesSpec : *mut xcb_xkb_string8_t,
                                             compatMapSpecLen :  u8,
                                             compatMapSpec : *mut xcb_xkb_string8_t,
                                             symbolsSpecLen :  u8,
                                             symbolsSpec : *mut xcb_xkb_string8_t,
                                             geometrySpecLen :  u8,
                                             geometrySpec : *mut xcb_xkb_string8_t) -> xcb_xkb_get_kbd_by_name_cookie_t;


///
/// xcb_xkb_get_kbd_by_name_replies : *mut xcb_xkb_get_kbd_by_name_replies_t
///
pub fn xcb_xkb_get_kbd_by_name_replies (R : *mut xcb_xkb_get_kbd_by_name_reply_t) -> *mut c_void;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_xkb_get_kbd_by_name_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_xkb_get_kbd_by_name_reply (c : *mut ffi::base::xcb_connection_t,
                                         cookie : xcb_xkb_get_kbd_by_name_cookie_t,
                                         e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_xkb_get_kbd_by_name_reply_t;

pub fn xcb_xkb_get_device_info_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
///
pub fn xcb_xkb_get_device_info (c : *mut ffi::base::xcb_connection_t,
                                   deviceSpec :  xcb_xkb_device_spec_t,
                                   wanted :  u16,
                                   allButtons :  u8,
                                   firstButton :  u8,
                                   nButtons :  u8,
                                   ledClass :  xcb_xkb_led_class_spec_t,
                                   ledID :  xcb_xkb_id_spec_t) -> xcb_xkb_get_device_info_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_xkb_get_device_info_unchecked (c : *mut ffi::base::xcb_connection_t,
                                             deviceSpec :  xcb_xkb_device_spec_t,
                                             wanted :  u16,
                                             allButtons :  u8,
                                             firstButton :  u8,
                                             nButtons :  u8,
                                             ledClass :  xcb_xkb_led_class_spec_t,
                                             ledID :  xcb_xkb_id_spec_t) -> xcb_xkb_get_device_info_cookie_t;

pub fn xcb_xkb_get_device_info_name (R : *mut xcb_xkb_get_device_info_reply_t) -> *mut xcb_xkb_string8_t;


pub fn xcb_xkb_get_device_info_name_length (R : *mut xcb_xkb_get_device_info_reply_t) -> c_int;


pub fn xcb_xkb_get_device_info_name_end (R : *mut xcb_xkb_get_device_info_reply_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_xkb_get_device_info_btn_actions (R : *mut xcb_xkb_get_device_info_reply_t) -> *mut xcb_xkb_action_t;


pub fn xcb_xkb_get_device_info_btn_actions_length (R : *mut xcb_xkb_get_device_info_reply_t) -> c_int;

pub fn xcb_xkb_get_device_info_btn_actions_iterator (R : *mut xcb_xkb_get_device_info_reply_t) -> xcb_xkb_action_iterator_t;


pub fn xcb_xkb_get_device_info_leds_length (R : *mut xcb_xkb_get_device_info_reply_t) -> c_int;

pub fn xcb_xkb_get_device_info_leds_iterator (R : *mut xcb_xkb_get_device_info_reply_t) -> xcb_xkb_device_led_info_iterator_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_xkb_get_device_info_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_xkb_get_device_info_reply (c : *mut ffi::base::xcb_connection_t,
                                         cookie : xcb_xkb_get_device_info_cookie_t,
                                         e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_xkb_get_device_info_reply_t;

pub fn xcb_xkb_set_device_info_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_xkb_set_device_info_checked (c : *mut ffi::base::xcb_connection_t,
                                           deviceSpec :  xcb_xkb_device_spec_t,
                                           firstBtn :  u8,
                                           nBtns :  u8,
                                           change :  u16,
                                           nDeviceLedFBs :  u16,
                                           btnActions : *mut xcb_xkb_action_t,
                                           leds : *mut xcb_xkb_device_led_info_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_xkb_set_device_info (c : *mut ffi::base::xcb_connection_t,
                                   deviceSpec :  xcb_xkb_device_spec_t,
                                   firstBtn :  u8,
                                   nBtns :  u8,
                                   change :  u16,
                                   nDeviceLedFBs :  u16,
                                   btnActions : *mut xcb_xkb_action_t,
                                   leds : *mut xcb_xkb_device_led_info_t) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_xkb_set_debugging_flags_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
///
pub fn xcb_xkb_set_debugging_flags (c : *mut ffi::base::xcb_connection_t,
                                       msgLength :  u16,
                                       affectFlags :  u32,
                                       flags :  u32,
                                       affectCtrls :  u32,
                                       ctrls :  u32,
                                       message : *mut xcb_xkb_string8_t) -> xcb_xkb_set_debugging_flags_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_xkb_set_debugging_flags_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                 msgLength :  u16,
                                                 affectFlags :  u32,
                                                 flags :  u32,
                                                 affectCtrls :  u32,
                                                 ctrls :  u32,
                                                 message : *mut xcb_xkb_string8_t) -> xcb_xkb_set_debugging_flags_cookie_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_xkb_set_debugging_flags_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_xkb_set_debugging_flags_reply (c : *mut ffi::base::xcb_connection_t,
                                             cookie : xcb_xkb_set_debugging_flags_cookie_t,
                                             e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_xkb_set_debugging_flags_reply_t;
}

