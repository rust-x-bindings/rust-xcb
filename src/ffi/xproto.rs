//
// This file generated automatically from xproto.xml by r_client.py.
// Edit at your peril.
//

//Make the compiler quiet
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use std;
use libc::*;
use ffi;

#[repr(C)]
pub struct xcb_char2b_t {
    pub byte1 :   u8,
    pub byte2 :   u8
}

impl Copy for xcb_char2b_t {}
impl Clone for xcb_char2b_t {
    fn clone(&self) -> xcb_char2b_t { *self }
}
#[repr(C)]
pub struct xcb_char2b_iterator_t {
    pub data : *mut xcb_char2b_t,
    pub rem  : c_int,
    pub index: c_int
}


pub type xcb_window_t = u32;
#[repr(C)]
pub struct xcb_window_iterator_t {
    pub data : *mut xcb_window_t,
    pub rem  : c_int,
    pub index: c_int
}


pub type xcb_pixmap_t = u32;
#[repr(C)]
pub struct xcb_pixmap_iterator_t {
    pub data : *mut xcb_pixmap_t,
    pub rem  : c_int,
    pub index: c_int
}


pub type xcb_cursor_t = u32;
#[repr(C)]
pub struct xcb_cursor_iterator_t {
    pub data : *mut xcb_cursor_t,
    pub rem  : c_int,
    pub index: c_int
}


pub type xcb_font_t = u32;
#[repr(C)]
pub struct xcb_font_iterator_t {
    pub data : *mut xcb_font_t,
    pub rem  : c_int,
    pub index: c_int
}


pub type xcb_gcontext_t = u32;
#[repr(C)]
pub struct xcb_gcontext_iterator_t {
    pub data : *mut xcb_gcontext_t,
    pub rem  : c_int,
    pub index: c_int
}


pub type xcb_colormap_t = u32;
#[repr(C)]
pub struct xcb_colormap_iterator_t {
    pub data : *mut xcb_colormap_t,
    pub rem  : c_int,
    pub index: c_int
}


pub type xcb_atom_t = u32;
#[repr(C)]
pub struct xcb_atom_iterator_t {
    pub data : *mut xcb_atom_t,
    pub rem  : c_int,
    pub index: c_int
}


pub type xcb_drawable_t = u32;
#[repr(C)]
pub struct xcb_drawable_iterator_t {
    pub data : *mut xcb_drawable_t,
    pub rem  : c_int,
    pub index: c_int
}


pub type xcb_fontable_t = u32;
#[repr(C)]
pub struct xcb_fontable_iterator_t {
    pub data : *mut xcb_fontable_t,
    pub rem  : c_int,
    pub index: c_int
}


pub type xcb_visualid_t = u32;
#[repr(C)]
pub struct xcb_visualid_iterator_t {
    pub data : *mut xcb_visualid_t,
    pub rem  : c_int,
    pub index: c_int
}


pub type xcb_timestamp_t = u32;
#[repr(C)]
pub struct xcb_timestamp_iterator_t {
    pub data : *mut xcb_timestamp_t,
    pub rem  : c_int,
    pub index: c_int
}


pub type xcb_keysym_t = u32;
#[repr(C)]
pub struct xcb_keysym_iterator_t {
    pub data : *mut xcb_keysym_t,
    pub rem  : c_int,
    pub index: c_int
}


pub type xcb_keycode_t = u8;
#[repr(C)]
pub struct xcb_keycode_iterator_t {
    pub data : *mut xcb_keycode_t,
    pub rem  : c_int,
    pub index: c_int
}


pub type xcb_button_t = u8;
#[repr(C)]
pub struct xcb_button_iterator_t {
    pub data : *mut xcb_button_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_point_t {
    pub x :   i16,
    pub y :   i16
}

impl Copy for xcb_point_t {}
impl Clone for xcb_point_t {
    fn clone(&self) -> xcb_point_t { *self }
}
#[repr(C)]
pub struct xcb_point_iterator_t {
    pub data : *mut xcb_point_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_rectangle_t {
    pub x :        i16,
    pub y :        i16,
    pub width :    u16,
    pub height :   u16
}

impl Copy for xcb_rectangle_t {}
impl Clone for xcb_rectangle_t {
    fn clone(&self) -> xcb_rectangle_t { *self }
}
#[repr(C)]
pub struct xcb_rectangle_iterator_t {
    pub data : *mut xcb_rectangle_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_arc_t {
    pub x :        i16,
    pub y :        i16,
    pub width :    u16,
    pub height :   u16,
    pub angle1 :   i16,
    pub angle2 :   i16
}

impl Copy for xcb_arc_t {}
impl Clone for xcb_arc_t {
    fn clone(&self) -> xcb_arc_t { *self }
}
#[repr(C)]
pub struct xcb_arc_iterator_t {
    pub data : *mut xcb_arc_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_format_t {
    pub depth :            u8,
    pub bits_per_pixel :   u8,
    pub scanline_pad :     u8,
    pub pad0 :             [u8; 5]
}

impl Copy for xcb_format_t {}
impl Clone for xcb_format_t {
    fn clone(&self) -> xcb_format_t { *self }
}
#[repr(C)]
pub struct xcb_format_iterator_t {
    pub data : *mut xcb_format_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_visualtype_t {
    pub visual_id :            xcb_visualid_t,
    pub class :                u8,
    pub bits_per_rgb_value :   u8,
    pub colormap_entries :     u16,
    pub red_mask :             u32,
    pub green_mask :           u32,
    pub blue_mask :            u32,
    pub pad0 :                 [u8; 4]
}

impl Copy for xcb_visualtype_t {}
impl Clone for xcb_visualtype_t {
    fn clone(&self) -> xcb_visualtype_t { *self }
}
#[repr(C)]
pub struct xcb_visualtype_iterator_t {
    pub data : *mut xcb_visualtype_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_depth_t {
    pub depth :         u8,
    pub pad0 :          u8,
    pub visuals_len :   u16,
    pub pad1 :          [u8; 4]
}

impl Copy for xcb_depth_t {}
impl Clone for xcb_depth_t {
    fn clone(&self) -> xcb_depth_t { *self }
}
#[repr(C)]
pub struct xcb_depth_iterator_t {
    pub data : *mut xcb_depth_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_screen_t {
    pub root :                    xcb_window_t,
    pub default_colormap :        xcb_colormap_t,
    pub white_pixel :             u32,
    pub black_pixel :             u32,
    pub current_input_masks :     u32,
    pub width_in_pixels :         u16,
    pub height_in_pixels :        u16,
    pub width_in_millimeters :    u16,
    pub height_in_millimeters :   u16,
    pub min_installed_maps :      u16,
    pub max_installed_maps :      u16,
    pub root_visual :             xcb_visualid_t,
    pub backing_stores :          u8,
    pub save_unders :             u8,
    pub root_depth :              u8,
    pub allowed_depths_len :      u8
}

impl Copy for xcb_screen_t {}
impl Clone for xcb_screen_t {
    fn clone(&self) -> xcb_screen_t { *self }
}
#[repr(C)]
pub struct xcb_screen_iterator_t {
    pub data : *mut xcb_screen_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_setup_request_t {
    pub byte_order :                        u8,
    pub pad0 :                              u8,
    pub protocol_major_version :            u16,
    pub protocol_minor_version :            u16,
    pub authorization_protocol_name_len :   u16,
    pub authorization_protocol_data_len :   u16,
    pub pad1 :                              [u8; 2]
}

impl Copy for xcb_setup_request_t {}
impl Clone for xcb_setup_request_t {
    fn clone(&self) -> xcb_setup_request_t { *self }
}
#[repr(C)]
pub struct xcb_setup_request_iterator_t {
    pub data : *mut xcb_setup_request_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_setup_failed_t {
    pub status :                   u8,
    pub reason_len :               u8,
    pub protocol_major_version :   u16,
    pub protocol_minor_version :   u16,
    pub length :                   u16
}

impl Copy for xcb_setup_failed_t {}
impl Clone for xcb_setup_failed_t {
    fn clone(&self) -> xcb_setup_failed_t { *self }
}
#[repr(C)]
pub struct xcb_setup_failed_iterator_t {
    pub data : *mut xcb_setup_failed_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_setup_authenticate_t {
    pub status :   u8,
    pub pad0 :     [u8; 5],
    pub length :   u16
}

impl Copy for xcb_setup_authenticate_t {}
impl Clone for xcb_setup_authenticate_t {
    fn clone(&self) -> xcb_setup_authenticate_t { *self }
}
#[repr(C)]
pub struct xcb_setup_authenticate_iterator_t {
    pub data : *mut xcb_setup_authenticate_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_setup_t {
    pub status :                        u8,
    pub pad0 :                          u8,
    pub protocol_major_version :        u16,
    pub protocol_minor_version :        u16,
    pub length :                        u16,
    pub release_number :                u32,
    pub resource_id_base :              u32,
    pub resource_id_mask :              u32,
    pub motion_buffer_size :            u32,
    pub vendor_len :                    u16,
    pub maximum_request_length :        u16,
    pub roots_len :                     u8,
    pub pixmap_formats_len :            u8,
    pub image_byte_order :              u8,
    pub bitmap_format_bit_order :       u8,
    pub bitmap_format_scanline_unit :   u8,
    pub bitmap_format_scanline_pad :    u8,
    pub min_keycode :                   xcb_keycode_t,
    pub max_keycode :                   xcb_keycode_t,
    pub pad1 :                          [u8; 4]
}

impl Copy for xcb_setup_t {}
impl Clone for xcb_setup_t {
    fn clone(&self) -> xcb_setup_t { *self }
}


#[repr(C)]
pub struct xcb_key_press_event_t {
    pub response_type :   u8,
    pub detail :          xcb_keycode_t,
    pub sequence :        u16,
    pub time :            xcb_timestamp_t,
    pub root :            xcb_window_t,
    pub event :           xcb_window_t,
    pub child :           xcb_window_t,
    pub root_x :          i16,
    pub root_y :          i16,
    pub event_x :         i16,
    pub event_y :         i16,
    pub state :           u16,
    pub same_screen :     u8,
    pub pad0 :            u8
}

impl Copy for xcb_key_press_event_t {}
impl Clone for xcb_key_press_event_t {
    fn clone(&self) -> xcb_key_press_event_t { *self }
}


pub type xcb_key_release_event_t = xcb_key_press_event_t;


#[repr(C)]
pub struct xcb_button_press_event_t {
    pub response_type :   u8,
    pub detail :          xcb_button_t,
    pub sequence :        u16,
    pub time :            xcb_timestamp_t,
    pub root :            xcb_window_t,
    pub event :           xcb_window_t,
    pub child :           xcb_window_t,
    pub root_x :          i16,
    pub root_y :          i16,
    pub event_x :         i16,
    pub event_y :         i16,
    pub state :           u16,
    pub same_screen :     u8,
    pub pad0 :            u8
}

impl Copy for xcb_button_press_event_t {}
impl Clone for xcb_button_press_event_t {
    fn clone(&self) -> xcb_button_press_event_t { *self }
}


pub type xcb_button_release_event_t = xcb_button_press_event_t;


#[repr(C)]
pub struct xcb_motion_notify_event_t {
    pub response_type :   u8,
    pub detail :          u8,
    pub sequence :        u16,
    pub time :            xcb_timestamp_t,
    pub root :            xcb_window_t,
    pub event :           xcb_window_t,
    pub child :           xcb_window_t,
    pub root_x :          i16,
    pub root_y :          i16,
    pub event_x :         i16,
    pub event_y :         i16,
    pub state :           u16,
    pub same_screen :     u8,
    pub pad0 :            u8
}

impl Copy for xcb_motion_notify_event_t {}
impl Clone for xcb_motion_notify_event_t {
    fn clone(&self) -> xcb_motion_notify_event_t { *self }
}


#[repr(C)]
pub struct xcb_enter_notify_event_t {
    pub response_type :       u8,
    pub detail :              u8,
    pub sequence :            u16,
    pub time :                xcb_timestamp_t,
    pub root :                xcb_window_t,
    pub event :               xcb_window_t,
    pub child :               xcb_window_t,
    pub root_x :              i16,
    pub root_y :              i16,
    pub event_x :             i16,
    pub event_y :             i16,
    pub state :               u16,
    pub mode :                u8,
    pub same_screen_focus :   u8
}

impl Copy for xcb_enter_notify_event_t {}
impl Clone for xcb_enter_notify_event_t {
    fn clone(&self) -> xcb_enter_notify_event_t { *self }
}


pub type xcb_leave_notify_event_t = xcb_enter_notify_event_t;


#[repr(C)]
pub struct xcb_focus_in_event_t {
    pub response_type :   u8,
    pub detail :          u8,
    pub sequence :        u16,
    pub event :           xcb_window_t,
    pub mode :            u8,
    pub pad0 :            [u8; 3]
}

impl Copy for xcb_focus_in_event_t {}
impl Clone for xcb_focus_in_event_t {
    fn clone(&self) -> xcb_focus_in_event_t { *self }
}


pub type xcb_focus_out_event_t = xcb_focus_in_event_t;


#[repr(C)]
pub struct xcb_keymap_notify_event_t {
    pub response_type :   u8,
    pub keys :            [u8; 31]
}

impl Copy for xcb_keymap_notify_event_t {}
impl Clone for xcb_keymap_notify_event_t {
    fn clone(&self) -> xcb_keymap_notify_event_t { *self }
}


#[repr(C)]
pub struct xcb_expose_event_t {
    pub response_type :   u8,
    pub pad0 :            u8,
    pub sequence :        u16,
    pub window :          xcb_window_t,
    pub x :               u16,
    pub y :               u16,
    pub width :           u16,
    pub height :          u16,
    pub count :           u16,
    pub pad1 :            [u8; 2]
}

impl Copy for xcb_expose_event_t {}
impl Clone for xcb_expose_event_t {
    fn clone(&self) -> xcb_expose_event_t { *self }
}


#[repr(C)]
pub struct xcb_graphics_exposure_event_t {
    pub response_type :   u8,
    pub pad0 :            u8,
    pub sequence :        u16,
    pub drawable :        xcb_drawable_t,
    pub x :               u16,
    pub y :               u16,
    pub width :           u16,
    pub height :          u16,
    pub minor_opcode :    u16,
    pub count :           u16,
    pub major_opcode :    u8,
    pub pad1 :            [u8; 3]
}

impl Copy for xcb_graphics_exposure_event_t {}
impl Clone for xcb_graphics_exposure_event_t {
    fn clone(&self) -> xcb_graphics_exposure_event_t { *self }
}


#[repr(C)]
pub struct xcb_no_exposure_event_t {
    pub response_type :   u8,
    pub pad0 :            u8,
    pub sequence :        u16,
    pub drawable :        xcb_drawable_t,
    pub minor_opcode :    u16,
    pub major_opcode :    u8,
    pub pad1 :            u8
}

impl Copy for xcb_no_exposure_event_t {}
impl Clone for xcb_no_exposure_event_t {
    fn clone(&self) -> xcb_no_exposure_event_t { *self }
}


#[repr(C)]
pub struct xcb_visibility_notify_event_t {
    pub response_type :   u8,
    pub pad0 :            u8,
    pub sequence :        u16,
    pub window :          xcb_window_t,
    pub state :           u8,
    pub pad1 :            [u8; 3]
}

impl Copy for xcb_visibility_notify_event_t {}
impl Clone for xcb_visibility_notify_event_t {
    fn clone(&self) -> xcb_visibility_notify_event_t { *self }
}


#[repr(C)]
pub struct xcb_create_notify_event_t {
    pub response_type :       u8,
    pub pad0 :                u8,
    pub sequence :            u16,
    pub parent :              xcb_window_t,
    pub window :              xcb_window_t,
    pub x :                   i16,
    pub y :                   i16,
    pub width :               u16,
    pub height :              u16,
    pub border_width :        u16,
    pub override_redirect :   u8,
    pub pad1 :                u8
}

impl Copy for xcb_create_notify_event_t {}
impl Clone for xcb_create_notify_event_t {
    fn clone(&self) -> xcb_create_notify_event_t { *self }
}


#[repr(C)]
pub struct xcb_destroy_notify_event_t {
    pub response_type :   u8,
    pub pad0 :            u8,
    pub sequence :        u16,
    pub event :           xcb_window_t,
    pub window :          xcb_window_t
}

impl Copy for xcb_destroy_notify_event_t {}
impl Clone for xcb_destroy_notify_event_t {
    fn clone(&self) -> xcb_destroy_notify_event_t { *self }
}


#[repr(C)]
pub struct xcb_unmap_notify_event_t {
    pub response_type :    u8,
    pub pad0 :             u8,
    pub sequence :         u16,
    pub event :            xcb_window_t,
    pub window :           xcb_window_t,
    pub from_configure :   u8,
    pub pad1 :             [u8; 3]
}

impl Copy for xcb_unmap_notify_event_t {}
impl Clone for xcb_unmap_notify_event_t {
    fn clone(&self) -> xcb_unmap_notify_event_t { *self }
}


#[repr(C)]
pub struct xcb_map_notify_event_t {
    pub response_type :       u8,
    pub pad0 :                u8,
    pub sequence :            u16,
    pub event :               xcb_window_t,
    pub window :              xcb_window_t,
    pub override_redirect :   u8,
    pub pad1 :                [u8; 3]
}

impl Copy for xcb_map_notify_event_t {}
impl Clone for xcb_map_notify_event_t {
    fn clone(&self) -> xcb_map_notify_event_t { *self }
}


#[repr(C)]
pub struct xcb_map_request_event_t {
    pub response_type :   u8,
    pub pad0 :            u8,
    pub sequence :        u16,
    pub parent :          xcb_window_t,
    pub window :          xcb_window_t
}

impl Copy for xcb_map_request_event_t {}
impl Clone for xcb_map_request_event_t {
    fn clone(&self) -> xcb_map_request_event_t { *self }
}


#[repr(C)]
pub struct xcb_reparent_notify_event_t {
    pub response_type :       u8,
    pub pad0 :                u8,
    pub sequence :            u16,
    pub event :               xcb_window_t,
    pub window :              xcb_window_t,
    pub parent :              xcb_window_t,
    pub x :                   i16,
    pub y :                   i16,
    pub override_redirect :   u8,
    pub pad1 :                [u8; 3]
}

impl Copy for xcb_reparent_notify_event_t {}
impl Clone for xcb_reparent_notify_event_t {
    fn clone(&self) -> xcb_reparent_notify_event_t { *self }
}


#[repr(C)]
pub struct xcb_configure_notify_event_t {
    pub response_type :       u8,
    pub pad0 :                u8,
    pub sequence :            u16,
    pub event :               xcb_window_t,
    pub window :              xcb_window_t,
    pub above_sibling :       xcb_window_t,
    pub x :                   i16,
    pub y :                   i16,
    pub width :               u16,
    pub height :              u16,
    pub border_width :        u16,
    pub override_redirect :   u8,
    pub pad1 :                u8
}

impl Copy for xcb_configure_notify_event_t {}
impl Clone for xcb_configure_notify_event_t {
    fn clone(&self) -> xcb_configure_notify_event_t { *self }
}


#[repr(C)]
pub struct xcb_configure_request_event_t {
    pub response_type :   u8,
    pub stack_mode :      u8,
    pub sequence :        u16,
    pub parent :          xcb_window_t,
    pub window :          xcb_window_t,
    pub sibling :         xcb_window_t,
    pub x :               i16,
    pub y :               i16,
    pub width :           u16,
    pub height :          u16,
    pub border_width :    u16,
    pub value_mask :      u16
}

impl Copy for xcb_configure_request_event_t {}
impl Clone for xcb_configure_request_event_t {
    fn clone(&self) -> xcb_configure_request_event_t { *self }
}


#[repr(C)]
pub struct xcb_gravity_notify_event_t {
    pub response_type :   u8,
    pub pad0 :            u8,
    pub sequence :        u16,
    pub event :           xcb_window_t,
    pub window :          xcb_window_t,
    pub x :               i16,
    pub y :               i16
}

impl Copy for xcb_gravity_notify_event_t {}
impl Clone for xcb_gravity_notify_event_t {
    fn clone(&self) -> xcb_gravity_notify_event_t { *self }
}


#[repr(C)]
pub struct xcb_resize_request_event_t {
    pub response_type :   u8,
    pub pad0 :            u8,
    pub sequence :        u16,
    pub window :          xcb_window_t,
    pub width :           u16,
    pub height :          u16
}

impl Copy for xcb_resize_request_event_t {}
impl Clone for xcb_resize_request_event_t {
    fn clone(&self) -> xcb_resize_request_event_t { *self }
}


#[repr(C)]
pub struct xcb_circulate_notify_event_t {
    pub response_type :   u8,
    pub pad0 :            u8,
    pub sequence :        u16,
    pub event :           xcb_window_t,
    pub window :          xcb_window_t,
    pub pad1 :            [u8; 4],
    pub place :           u8,
    pub pad2 :            [u8; 3]
}

impl Copy for xcb_circulate_notify_event_t {}
impl Clone for xcb_circulate_notify_event_t {
    fn clone(&self) -> xcb_circulate_notify_event_t { *self }
}


pub type xcb_circulate_request_event_t = xcb_circulate_notify_event_t;


#[repr(C)]
pub struct xcb_property_notify_event_t {
    pub response_type :   u8,
    pub pad0 :            u8,
    pub sequence :        u16,
    pub window :          xcb_window_t,
    pub atom :            xcb_atom_t,
    pub time :            xcb_timestamp_t,
    pub state :           u8,
    pub pad1 :            [u8; 3]
}

impl Copy for xcb_property_notify_event_t {}
impl Clone for xcb_property_notify_event_t {
    fn clone(&self) -> xcb_property_notify_event_t { *self }
}


#[repr(C)]
pub struct xcb_selection_clear_event_t {
    pub response_type :   u8,
    pub pad0 :            u8,
    pub sequence :        u16,
    pub time :            xcb_timestamp_t,
    pub owner :           xcb_window_t,
    pub selection :       xcb_atom_t
}

impl Copy for xcb_selection_clear_event_t {}
impl Clone for xcb_selection_clear_event_t {
    fn clone(&self) -> xcb_selection_clear_event_t { *self }
}


#[repr(C)]
pub struct xcb_selection_request_event_t {
    pub response_type :   u8,
    pub pad0 :            u8,
    pub sequence :        u16,
    pub time :            xcb_timestamp_t,
    pub owner :           xcb_window_t,
    pub requestor :       xcb_window_t,
    pub selection :       xcb_atom_t,
    pub target :          xcb_atom_t,
    pub property :        xcb_atom_t
}

impl Copy for xcb_selection_request_event_t {}
impl Clone for xcb_selection_request_event_t {
    fn clone(&self) -> xcb_selection_request_event_t { *self }
}


#[repr(C)]
pub struct xcb_selection_notify_event_t {
    pub response_type :   u8,
    pub pad0 :            u8,
    pub sequence :        u16,
    pub time :            xcb_timestamp_t,
    pub requestor :       xcb_window_t,
    pub selection :       xcb_atom_t,
    pub target :          xcb_atom_t,
    pub property :        xcb_atom_t
}

impl Copy for xcb_selection_notify_event_t {}
impl Clone for xcb_selection_notify_event_t {
    fn clone(&self) -> xcb_selection_notify_event_t { *self }
}


#[repr(C)]
pub struct xcb_colormap_notify_event_t {
    pub response_type :   u8,
    pub pad0 :            u8,
    pub sequence :        u16,
    pub window :          xcb_window_t,
    pub colormap :        xcb_colormap_t,
    pub new_ :            u8,
    pub state :           u8,
    pub pad1 :            [u8; 2]
}

impl Copy for xcb_colormap_notify_event_t {}
impl Clone for xcb_colormap_notify_event_t {
    fn clone(&self) -> xcb_colormap_notify_event_t { *self }
}

#[repr(C)]
pub struct xcb_client_message_data_t {
    data : [u8; 20]
}
impl Copy for xcb_client_message_data_t {}
impl Clone for xcb_client_message_data_t {
    fn clone(&self) -> xcb_client_message_data_t { *self }
}
#[repr(C)]
pub struct xcb_client_message_data_iterator_t {
    pub data : *mut xcb_client_message_data_t,
    pub rem  : c_int,
    pub index: c_int
}



#[repr(C)]
pub struct xcb_client_message_event_t {
    pub response_type :   u8,
    pub format :          u8,
    pub sequence :        u16,
    pub window :          xcb_window_t,
    pub type_ :           xcb_atom_t,
    pub data :            xcb_client_message_data_t
}

impl Copy for xcb_client_message_event_t {}
impl Clone for xcb_client_message_event_t {
    fn clone(&self) -> xcb_client_message_event_t { *self }
}


#[repr(C)]
pub struct xcb_mapping_notify_event_t {
    pub response_type :   u8,
    pub pad0 :            u8,
    pub sequence :        u16,
    pub request :         u8,
    pub first_keycode :   xcb_keycode_t,
    pub count :           u8,
    pub pad1 :            u8
}

impl Copy for xcb_mapping_notify_event_t {}
impl Clone for xcb_mapping_notify_event_t {
    fn clone(&self) -> xcb_mapping_notify_event_t { *self }
}


#[repr(C)]
pub struct xcb_request_error_t {
    pub response_type :   u8,
    pub error_code :      u8,
    pub sequence :        u16,
    pub bad_value :       u32,
    pub minor_opcode :    u16,
    pub major_opcode :    u8,
    pub pad0 :            u8
}

impl Copy for xcb_request_error_t {}
impl Clone for xcb_request_error_t {
    fn clone(&self) -> xcb_request_error_t { *self }
}


#[repr(C)]
pub struct xcb_value_error_t {
    pub response_type :   u8,
    pub error_code :      u8,
    pub sequence :        u16,
    pub bad_value :       u32,
    pub minor_opcode :    u16,
    pub major_opcode :    u8,
    pub pad0 :            u8
}

impl Copy for xcb_value_error_t {}
impl Clone for xcb_value_error_t {
    fn clone(&self) -> xcb_value_error_t { *self }
}


pub type xcb_window_error_t  = xcb_value_error_t;


pub type xcb_pixmap_error_t  = xcb_value_error_t;


pub type xcb_atom_error_t  = xcb_value_error_t;


pub type xcb_cursor_error_t  = xcb_value_error_t;


pub type xcb_font_error_t  = xcb_value_error_t;


pub type xcb_match_error_t  = xcb_request_error_t;


pub type xcb_drawable_error_t  = xcb_value_error_t;


pub type xcb_access_error_t  = xcb_request_error_t;


pub type xcb_alloc_error_t  = xcb_request_error_t;


pub type xcb_colormap_error_t  = xcb_value_error_t;


pub type xcb_g_context_error_t  = xcb_value_error_t;


pub type xcb_id_choice_error_t  = xcb_value_error_t;


pub type xcb_name_error_t  = xcb_request_error_t;


pub type xcb_length_error_t  = xcb_request_error_t;


pub type xcb_implementation_error_t  = xcb_request_error_t;


#[repr(C)]
pub struct xcb_create_window_request_t {
    pub major_opcode :   u8,
    pub depth :          u8,
    pub length :         u16,
    pub wid :            xcb_window_t,
    pub parent :         xcb_window_t,
    pub x :              i16,
    pub y :              i16,
    pub width :          u16,
    pub height :         u16,
    pub border_width :   u16,
    pub class :          u16,
    pub visual :         xcb_visualid_t,
    pub value_mask :     u32
}

impl Copy for xcb_create_window_request_t {}
impl Clone for xcb_create_window_request_t {
    fn clone(&self) -> xcb_create_window_request_t { *self }
}


#[repr(C)]
pub struct xcb_change_window_attributes_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16,
    pub window :         xcb_window_t,
    pub value_mask :     u32
}

impl Copy for xcb_change_window_attributes_request_t {}
impl Clone for xcb_change_window_attributes_request_t {
    fn clone(&self) -> xcb_change_window_attributes_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_get_window_attributes_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_get_window_attributes_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16,
    pub window :         xcb_window_t
}

impl Copy for xcb_get_window_attributes_request_t {}
impl Clone for xcb_get_window_attributes_request_t {
    fn clone(&self) -> xcb_get_window_attributes_request_t { *self }
}

#[repr(C)]
pub struct xcb_get_window_attributes_reply_t {
    pub response_type :           u8,
    pub backing_store :           u8,
    pub sequence :                u16,
    pub length :                  u32,
    pub visual :                  xcb_visualid_t,
    pub class :                   u16,
    pub bit_gravity :             u8,
    pub win_gravity :             u8,
    pub backing_planes :          u32,
    pub backing_pixel :           u32,
    pub save_under :              u8,
    pub map_is_installed :        u8,
    pub map_state :               u8,
    pub override_redirect :       u8,
    pub colormap :                xcb_colormap_t,
    pub all_event_masks :         u32,
    pub your_event_mask :         u32,
    pub do_not_propagate_mask :   u16,
    pub pad0 :                    [u8; 2]
}

impl Copy for xcb_get_window_attributes_reply_t {}
impl Clone for xcb_get_window_attributes_reply_t {
    fn clone(&self) -> xcb_get_window_attributes_reply_t { *self }
}


#[repr(C)]
pub struct xcb_destroy_window_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16,
    pub window :         xcb_window_t
}

impl Copy for xcb_destroy_window_request_t {}
impl Clone for xcb_destroy_window_request_t {
    fn clone(&self) -> xcb_destroy_window_request_t { *self }
}


#[repr(C)]
pub struct xcb_destroy_subwindows_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16,
    pub window :         xcb_window_t
}

impl Copy for xcb_destroy_subwindows_request_t {}
impl Clone for xcb_destroy_subwindows_request_t {
    fn clone(&self) -> xcb_destroy_subwindows_request_t { *self }
}


#[repr(C)]
pub struct xcb_change_save_set_request_t {
    pub major_opcode :   u8,
    pub mode :           u8,
    pub length :         u16,
    pub window :         xcb_window_t
}

impl Copy for xcb_change_save_set_request_t {}
impl Clone for xcb_change_save_set_request_t {
    fn clone(&self) -> xcb_change_save_set_request_t { *self }
}


#[repr(C)]
pub struct xcb_reparent_window_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16,
    pub window :         xcb_window_t,
    pub parent :         xcb_window_t,
    pub x :              i16,
    pub y :              i16
}

impl Copy for xcb_reparent_window_request_t {}
impl Clone for xcb_reparent_window_request_t {
    fn clone(&self) -> xcb_reparent_window_request_t { *self }
}


#[repr(C)]
pub struct xcb_map_window_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16,
    pub window :         xcb_window_t
}

impl Copy for xcb_map_window_request_t {}
impl Clone for xcb_map_window_request_t {
    fn clone(&self) -> xcb_map_window_request_t { *self }
}


#[repr(C)]
pub struct xcb_map_subwindows_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16,
    pub window :         xcb_window_t
}

impl Copy for xcb_map_subwindows_request_t {}
impl Clone for xcb_map_subwindows_request_t {
    fn clone(&self) -> xcb_map_subwindows_request_t { *self }
}


#[repr(C)]
pub struct xcb_unmap_window_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16,
    pub window :         xcb_window_t
}

impl Copy for xcb_unmap_window_request_t {}
impl Clone for xcb_unmap_window_request_t {
    fn clone(&self) -> xcb_unmap_window_request_t { *self }
}


#[repr(C)]
pub struct xcb_unmap_subwindows_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16,
    pub window :         xcb_window_t
}

impl Copy for xcb_unmap_subwindows_request_t {}
impl Clone for xcb_unmap_subwindows_request_t {
    fn clone(&self) -> xcb_unmap_subwindows_request_t { *self }
}


#[repr(C)]
pub struct xcb_configure_window_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16,
    pub window :         xcb_window_t,
    pub value_mask :     u16,
    pub pad1 :           [u8; 2]
}

impl Copy for xcb_configure_window_request_t {}
impl Clone for xcb_configure_window_request_t {
    fn clone(&self) -> xcb_configure_window_request_t { *self }
}


#[repr(C)]
pub struct xcb_circulate_window_request_t {
    pub major_opcode :   u8,
    pub direction :      u8,
    pub length :         u16,
    pub window :         xcb_window_t
}

impl Copy for xcb_circulate_window_request_t {}
impl Clone for xcb_circulate_window_request_t {
    fn clone(&self) -> xcb_circulate_window_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_get_geometry_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_get_geometry_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16,
    pub drawable :       xcb_drawable_t
}

impl Copy for xcb_get_geometry_request_t {}
impl Clone for xcb_get_geometry_request_t {
    fn clone(&self) -> xcb_get_geometry_request_t { *self }
}

#[repr(C)]
pub struct xcb_get_geometry_reply_t {
    pub response_type :   u8,
    pub depth :           u8,
    pub sequence :        u16,
    pub length :          u32,
    pub root :            xcb_window_t,
    pub x :               i16,
    pub y :               i16,
    pub width :           u16,
    pub height :          u16,
    pub border_width :    u16,
    pub pad0 :            [u8; 2]
}

impl Copy for xcb_get_geometry_reply_t {}
impl Clone for xcb_get_geometry_reply_t {
    fn clone(&self) -> xcb_get_geometry_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_query_tree_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_query_tree_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16,
    pub window :         xcb_window_t
}

impl Copy for xcb_query_tree_request_t {}
impl Clone for xcb_query_tree_request_t {
    fn clone(&self) -> xcb_query_tree_request_t { *self }
}

#[repr(C)]
pub struct xcb_query_tree_reply_t {
    pub response_type :   u8,
    pub pad0 :            u8,
    pub sequence :        u16,
    pub length :          u32,
    pub root :            xcb_window_t,
    pub parent :          xcb_window_t,
    pub children_len :    u16,
    pub pad1 :            [u8; 14]
}

impl Copy for xcb_query_tree_reply_t {}
impl Clone for xcb_query_tree_reply_t {
    fn clone(&self) -> xcb_query_tree_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_intern_atom_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_intern_atom_request_t {
    pub major_opcode :     u8,
    pub only_if_exists :   u8,
    pub length :           u16,
    pub name_len :         u16,
    pub pad0 :             [u8; 2]
}

impl Copy for xcb_intern_atom_request_t {}
impl Clone for xcb_intern_atom_request_t {
    fn clone(&self) -> xcb_intern_atom_request_t { *self }
}

#[repr(C)]
pub struct xcb_intern_atom_reply_t {
    pub response_type :   u8,
    pub pad0 :            u8,
    pub sequence :        u16,
    pub length :          u32,
    pub atom :            xcb_atom_t
}

impl Copy for xcb_intern_atom_reply_t {}
impl Clone for xcb_intern_atom_reply_t {
    fn clone(&self) -> xcb_intern_atom_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_get_atom_name_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_get_atom_name_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16,
    pub atom :           xcb_atom_t
}

impl Copy for xcb_get_atom_name_request_t {}
impl Clone for xcb_get_atom_name_request_t {
    fn clone(&self) -> xcb_get_atom_name_request_t { *self }
}

#[repr(C)]
pub struct xcb_get_atom_name_reply_t {
    pub response_type :   u8,
    pub pad0 :            u8,
    pub sequence :        u16,
    pub length :          u32,
    pub name_len :        u16,
    pub pad1 :            [u8; 22]
}

impl Copy for xcb_get_atom_name_reply_t {}
impl Clone for xcb_get_atom_name_reply_t {
    fn clone(&self) -> xcb_get_atom_name_reply_t { *self }
}


#[repr(C)]
pub struct xcb_change_property_request_t {
    pub major_opcode :   u8,
    pub mode :           u8,
    pub length :         u16,
    pub window :         xcb_window_t,
    pub property :       xcb_atom_t,
    pub type_ :          xcb_atom_t,
    pub format :         u8,
    pub pad0 :           [u8; 3],
    pub data_len :       u32
}

impl Copy for xcb_change_property_request_t {}
impl Clone for xcb_change_property_request_t {
    fn clone(&self) -> xcb_change_property_request_t { *self }
}


#[repr(C)]
pub struct xcb_delete_property_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16,
    pub window :         xcb_window_t,
    pub property :       xcb_atom_t
}

impl Copy for xcb_delete_property_request_t {}
impl Clone for xcb_delete_property_request_t {
    fn clone(&self) -> xcb_delete_property_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_get_property_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_get_property_request_t {
    pub major_opcode :   u8,
    pub delete :         u8,
    pub length :         u16,
    pub window :         xcb_window_t,
    pub property :       xcb_atom_t,
    pub type_ :          xcb_atom_t,
    pub long_offset :    u32,
    pub long_length :    u32
}

impl Copy for xcb_get_property_request_t {}
impl Clone for xcb_get_property_request_t {
    fn clone(&self) -> xcb_get_property_request_t { *self }
}

#[repr(C)]
pub struct xcb_get_property_reply_t {
    pub response_type :   u8,
    pub format :          u8,
    pub sequence :        u16,
    pub length :          u32,
    pub type_ :           xcb_atom_t,
    pub bytes_after :     u32,
    pub value_len :       u32,
    pub pad0 :            [u8; 12]
}

impl Copy for xcb_get_property_reply_t {}
impl Clone for xcb_get_property_reply_t {
    fn clone(&self) -> xcb_get_property_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_list_properties_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_list_properties_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16,
    pub window :         xcb_window_t
}

impl Copy for xcb_list_properties_request_t {}
impl Clone for xcb_list_properties_request_t {
    fn clone(&self) -> xcb_list_properties_request_t { *self }
}

#[repr(C)]
pub struct xcb_list_properties_reply_t {
    pub response_type :   u8,
    pub pad0 :            u8,
    pub sequence :        u16,
    pub length :          u32,
    pub atoms_len :       u16,
    pub pad1 :            [u8; 22]
}

impl Copy for xcb_list_properties_reply_t {}
impl Clone for xcb_list_properties_reply_t {
    fn clone(&self) -> xcb_list_properties_reply_t { *self }
}


#[repr(C)]
pub struct xcb_set_selection_owner_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16,
    pub owner :          xcb_window_t,
    pub selection :      xcb_atom_t,
    pub time :           xcb_timestamp_t
}

impl Copy for xcb_set_selection_owner_request_t {}
impl Clone for xcb_set_selection_owner_request_t {
    fn clone(&self) -> xcb_set_selection_owner_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_get_selection_owner_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_get_selection_owner_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16,
    pub selection :      xcb_atom_t
}

impl Copy for xcb_get_selection_owner_request_t {}
impl Clone for xcb_get_selection_owner_request_t {
    fn clone(&self) -> xcb_get_selection_owner_request_t { *self }
}

#[repr(C)]
pub struct xcb_get_selection_owner_reply_t {
    pub response_type :   u8,
    pub pad0 :            u8,
    pub sequence :        u16,
    pub length :          u32,
    pub owner :           xcb_window_t
}

impl Copy for xcb_get_selection_owner_reply_t {}
impl Clone for xcb_get_selection_owner_reply_t {
    fn clone(&self) -> xcb_get_selection_owner_reply_t { *self }
}


#[repr(C)]
pub struct xcb_convert_selection_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16,
    pub requestor :      xcb_window_t,
    pub selection :      xcb_atom_t,
    pub target :         xcb_atom_t,
    pub property :       xcb_atom_t,
    pub time :           xcb_timestamp_t
}

impl Copy for xcb_convert_selection_request_t {}
impl Clone for xcb_convert_selection_request_t {
    fn clone(&self) -> xcb_convert_selection_request_t { *self }
}


#[repr(C)]
pub struct xcb_send_event_request_t {
    pub major_opcode :   u8,
    pub propagate :      u8,
    pub length :         u16,
    pub destination :    xcb_window_t,
    pub event_mask :     u32,
    pub event :          [c_char; 32]
}

impl Copy for xcb_send_event_request_t {}
impl Clone for xcb_send_event_request_t {
    fn clone(&self) -> xcb_send_event_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_grab_pointer_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_grab_pointer_request_t {
    pub major_opcode :    u8,
    pub owner_events :    u8,
    pub length :          u16,
    pub grab_window :     xcb_window_t,
    pub event_mask :      u16,
    pub pointer_mode :    u8,
    pub keyboard_mode :   u8,
    pub confine_to :      xcb_window_t,
    pub cursor :          xcb_cursor_t,
    pub time :            xcb_timestamp_t
}

impl Copy for xcb_grab_pointer_request_t {}
impl Clone for xcb_grab_pointer_request_t {
    fn clone(&self) -> xcb_grab_pointer_request_t { *self }
}

#[repr(C)]
pub struct xcb_grab_pointer_reply_t {
    pub response_type :   u8,
    pub status :          u8,
    pub sequence :        u16,
    pub length :          u32
}

impl Copy for xcb_grab_pointer_reply_t {}
impl Clone for xcb_grab_pointer_reply_t {
    fn clone(&self) -> xcb_grab_pointer_reply_t { *self }
}


#[repr(C)]
pub struct xcb_ungrab_pointer_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16,
    pub time :           xcb_timestamp_t
}

impl Copy for xcb_ungrab_pointer_request_t {}
impl Clone for xcb_ungrab_pointer_request_t {
    fn clone(&self) -> xcb_ungrab_pointer_request_t { *self }
}


#[repr(C)]
pub struct xcb_grab_button_request_t {
    pub major_opcode :    u8,
    pub owner_events :    u8,
    pub length :          u16,
    pub grab_window :     xcb_window_t,
    pub event_mask :      u16,
    pub pointer_mode :    u8,
    pub keyboard_mode :   u8,
    pub confine_to :      xcb_window_t,
    pub cursor :          xcb_cursor_t,
    pub button :          u8,
    pub pad0 :            u8,
    pub modifiers :       u16
}

impl Copy for xcb_grab_button_request_t {}
impl Clone for xcb_grab_button_request_t {
    fn clone(&self) -> xcb_grab_button_request_t { *self }
}


#[repr(C)]
pub struct xcb_ungrab_button_request_t {
    pub major_opcode :   u8,
    pub button :         u8,
    pub length :         u16,
    pub grab_window :    xcb_window_t,
    pub modifiers :      u16,
    pub pad0 :           [u8; 2]
}

impl Copy for xcb_ungrab_button_request_t {}
impl Clone for xcb_ungrab_button_request_t {
    fn clone(&self) -> xcb_ungrab_button_request_t { *self }
}


#[repr(C)]
pub struct xcb_change_active_pointer_grab_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16,
    pub cursor :         xcb_cursor_t,
    pub time :           xcb_timestamp_t,
    pub event_mask :     u16,
    pub pad1 :           [u8; 2]
}

impl Copy for xcb_change_active_pointer_grab_request_t {}
impl Clone for xcb_change_active_pointer_grab_request_t {
    fn clone(&self) -> xcb_change_active_pointer_grab_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_grab_keyboard_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_grab_keyboard_request_t {
    pub major_opcode :    u8,
    pub owner_events :    u8,
    pub length :          u16,
    pub grab_window :     xcb_window_t,
    pub time :            xcb_timestamp_t,
    pub pointer_mode :    u8,
    pub keyboard_mode :   u8,
    pub pad0 :            [u8; 2]
}

impl Copy for xcb_grab_keyboard_request_t {}
impl Clone for xcb_grab_keyboard_request_t {
    fn clone(&self) -> xcb_grab_keyboard_request_t { *self }
}

#[repr(C)]
pub struct xcb_grab_keyboard_reply_t {
    pub response_type :   u8,
    pub status :          u8,
    pub sequence :        u16,
    pub length :          u32
}

impl Copy for xcb_grab_keyboard_reply_t {}
impl Clone for xcb_grab_keyboard_reply_t {
    fn clone(&self) -> xcb_grab_keyboard_reply_t { *self }
}


#[repr(C)]
pub struct xcb_ungrab_keyboard_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16,
    pub time :           xcb_timestamp_t
}

impl Copy for xcb_ungrab_keyboard_request_t {}
impl Clone for xcb_ungrab_keyboard_request_t {
    fn clone(&self) -> xcb_ungrab_keyboard_request_t { *self }
}


#[repr(C)]
pub struct xcb_grab_key_request_t {
    pub major_opcode :    u8,
    pub owner_events :    u8,
    pub length :          u16,
    pub grab_window :     xcb_window_t,
    pub modifiers :       u16,
    pub key :             xcb_keycode_t,
    pub pointer_mode :    u8,
    pub keyboard_mode :   u8,
    pub pad0 :            [u8; 3]
}

impl Copy for xcb_grab_key_request_t {}
impl Clone for xcb_grab_key_request_t {
    fn clone(&self) -> xcb_grab_key_request_t { *self }
}


#[repr(C)]
pub struct xcb_ungrab_key_request_t {
    pub major_opcode :   u8,
    pub key :            xcb_keycode_t,
    pub length :         u16,
    pub grab_window :    xcb_window_t,
    pub modifiers :      u16,
    pub pad0 :           [u8; 2]
}

impl Copy for xcb_ungrab_key_request_t {}
impl Clone for xcb_ungrab_key_request_t {
    fn clone(&self) -> xcb_ungrab_key_request_t { *self }
}


#[repr(C)]
pub struct xcb_allow_events_request_t {
    pub major_opcode :   u8,
    pub mode :           u8,
    pub length :         u16,
    pub time :           xcb_timestamp_t
}

impl Copy for xcb_allow_events_request_t {}
impl Clone for xcb_allow_events_request_t {
    fn clone(&self) -> xcb_allow_events_request_t { *self }
}


#[repr(C)]
pub struct xcb_grab_server_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16
}

impl Copy for xcb_grab_server_request_t {}
impl Clone for xcb_grab_server_request_t {
    fn clone(&self) -> xcb_grab_server_request_t { *self }
}


#[repr(C)]
pub struct xcb_ungrab_server_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16
}

impl Copy for xcb_ungrab_server_request_t {}
impl Clone for xcb_ungrab_server_request_t {
    fn clone(&self) -> xcb_ungrab_server_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_query_pointer_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_query_pointer_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16,
    pub window :         xcb_window_t
}

impl Copy for xcb_query_pointer_request_t {}
impl Clone for xcb_query_pointer_request_t {
    fn clone(&self) -> xcb_query_pointer_request_t { *self }
}

#[repr(C)]
pub struct xcb_query_pointer_reply_t {
    pub response_type :   u8,
    pub same_screen :     u8,
    pub sequence :        u16,
    pub length :          u32,
    pub root :            xcb_window_t,
    pub child :           xcb_window_t,
    pub root_x :          i16,
    pub root_y :          i16,
    pub win_x :           i16,
    pub win_y :           i16,
    pub mask :            u16,
    pub pad0 :            [u8; 2]
}

impl Copy for xcb_query_pointer_reply_t {}
impl Clone for xcb_query_pointer_reply_t {
    fn clone(&self) -> xcb_query_pointer_reply_t { *self }
}

#[repr(C)]
pub struct xcb_timecoord_t {
    pub time :   xcb_timestamp_t,
    pub x :      i16,
    pub y :      i16
}

impl Copy for xcb_timecoord_t {}
impl Clone for xcb_timecoord_t {
    fn clone(&self) -> xcb_timecoord_t { *self }
}
#[repr(C)]
pub struct xcb_timecoord_iterator_t {
    pub data : *mut xcb_timecoord_t,
    pub rem  : c_int,
    pub index: c_int
}


#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_get_motion_events_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_get_motion_events_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16,
    pub window :         xcb_window_t,
    pub start :          xcb_timestamp_t,
    pub stop :           xcb_timestamp_t
}

impl Copy for xcb_get_motion_events_request_t {}
impl Clone for xcb_get_motion_events_request_t {
    fn clone(&self) -> xcb_get_motion_events_request_t { *self }
}

#[repr(C)]
pub struct xcb_get_motion_events_reply_t {
    pub response_type :   u8,
    pub pad0 :            u8,
    pub sequence :        u16,
    pub length :          u32,
    pub events_len :      u32,
    pub pad1 :            [u8; 20]
}

impl Copy for xcb_get_motion_events_reply_t {}
impl Clone for xcb_get_motion_events_reply_t {
    fn clone(&self) -> xcb_get_motion_events_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_translate_coordinates_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_translate_coordinates_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16,
    pub src_window :     xcb_window_t,
    pub dst_window :     xcb_window_t,
    pub src_x :          i16,
    pub src_y :          i16
}

impl Copy for xcb_translate_coordinates_request_t {}
impl Clone for xcb_translate_coordinates_request_t {
    fn clone(&self) -> xcb_translate_coordinates_request_t { *self }
}

#[repr(C)]
pub struct xcb_translate_coordinates_reply_t {
    pub response_type :   u8,
    pub same_screen :     u8,
    pub sequence :        u16,
    pub length :          u32,
    pub child :           xcb_window_t,
    pub dst_x :           i16,
    pub dst_y :           i16
}

impl Copy for xcb_translate_coordinates_reply_t {}
impl Clone for xcb_translate_coordinates_reply_t {
    fn clone(&self) -> xcb_translate_coordinates_reply_t { *self }
}


#[repr(C)]
pub struct xcb_warp_pointer_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16,
    pub src_window :     xcb_window_t,
    pub dst_window :     xcb_window_t,
    pub src_x :          i16,
    pub src_y :          i16,
    pub src_width :      u16,
    pub src_height :     u16,
    pub dst_x :          i16,
    pub dst_y :          i16
}

impl Copy for xcb_warp_pointer_request_t {}
impl Clone for xcb_warp_pointer_request_t {
    fn clone(&self) -> xcb_warp_pointer_request_t { *self }
}


#[repr(C)]
pub struct xcb_set_input_focus_request_t {
    pub major_opcode :   u8,
    pub revert_to :      u8,
    pub length :         u16,
    pub focus :          xcb_window_t,
    pub time :           xcb_timestamp_t
}

impl Copy for xcb_set_input_focus_request_t {}
impl Clone for xcb_set_input_focus_request_t {
    fn clone(&self) -> xcb_set_input_focus_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_get_input_focus_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_get_input_focus_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16
}

impl Copy for xcb_get_input_focus_request_t {}
impl Clone for xcb_get_input_focus_request_t {
    fn clone(&self) -> xcb_get_input_focus_request_t { *self }
}

#[repr(C)]
pub struct xcb_get_input_focus_reply_t {
    pub response_type :   u8,
    pub revert_to :       u8,
    pub sequence :        u16,
    pub length :          u32,
    pub focus :           xcb_window_t
}

impl Copy for xcb_get_input_focus_reply_t {}
impl Clone for xcb_get_input_focus_reply_t {
    fn clone(&self) -> xcb_get_input_focus_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_query_keymap_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_query_keymap_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16
}

impl Copy for xcb_query_keymap_request_t {}
impl Clone for xcb_query_keymap_request_t {
    fn clone(&self) -> xcb_query_keymap_request_t { *self }
}

#[repr(C)]
pub struct xcb_query_keymap_reply_t {
    pub response_type :   u8,
    pub pad0 :            u8,
    pub sequence :        u16,
    pub length :          u32,
    pub keys :            [u8; 32]
}

impl Copy for xcb_query_keymap_reply_t {}
impl Clone for xcb_query_keymap_reply_t {
    fn clone(&self) -> xcb_query_keymap_reply_t { *self }
}


#[repr(C)]
pub struct xcb_open_font_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16,
    pub fid :            xcb_font_t,
    pub name_len :       u16,
    pub pad1 :           [u8; 2]
}

impl Copy for xcb_open_font_request_t {}
impl Clone for xcb_open_font_request_t {
    fn clone(&self) -> xcb_open_font_request_t { *self }
}


#[repr(C)]
pub struct xcb_close_font_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16,
    pub font :           xcb_font_t
}

impl Copy for xcb_close_font_request_t {}
impl Clone for xcb_close_font_request_t {
    fn clone(&self) -> xcb_close_font_request_t { *self }
}

#[repr(C)]
pub struct xcb_fontprop_t {
    pub name :    xcb_atom_t,
    pub value :   u32
}

impl Copy for xcb_fontprop_t {}
impl Clone for xcb_fontprop_t {
    fn clone(&self) -> xcb_fontprop_t { *self }
}
#[repr(C)]
pub struct xcb_fontprop_iterator_t {
    pub data : *mut xcb_fontprop_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_charinfo_t {
    pub left_side_bearing :    i16,
    pub right_side_bearing :   i16,
    pub character_width :      i16,
    pub ascent :               i16,
    pub descent :              i16,
    pub attributes :           u16
}

impl Copy for xcb_charinfo_t {}
impl Clone for xcb_charinfo_t {
    fn clone(&self) -> xcb_charinfo_t { *self }
}
#[repr(C)]
pub struct xcb_charinfo_iterator_t {
    pub data : *mut xcb_charinfo_t,
    pub rem  : c_int,
    pub index: c_int
}


#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_query_font_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_query_font_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16,
    pub font :           xcb_fontable_t
}

impl Copy for xcb_query_font_request_t {}
impl Clone for xcb_query_font_request_t {
    fn clone(&self) -> xcb_query_font_request_t { *self }
}

#[repr(C)]
pub struct xcb_query_font_reply_t {
    pub response_type :       u8,
    pub pad0 :                u8,
    pub sequence :            u16,
    pub length :              u32,
    pub min_bounds :          xcb_charinfo_t,
    pub pad1 :                [u8; 4],
    pub max_bounds :          xcb_charinfo_t,
    pub pad2 :                [u8; 4],
    pub min_char_or_byte2 :   u16,
    pub max_char_or_byte2 :   u16,
    pub default_char :        u16,
    pub properties_len :      u16,
    pub draw_direction :      u8,
    pub min_byte1 :           u8,
    pub max_byte1 :           u8,
    pub all_chars_exist :     u8,
    pub font_ascent :         i16,
    pub font_descent :        i16,
    pub char_infos_len :      u32
}

impl Copy for xcb_query_font_reply_t {}
impl Clone for xcb_query_font_reply_t {
    fn clone(&self) -> xcb_query_font_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_query_text_extents_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_query_text_extents_request_t {
    pub major_opcode :   u8,
    pub odd_length :     u8,
    pub length :         u16,
    pub font :           xcb_fontable_t
}

impl Copy for xcb_query_text_extents_request_t {}
impl Clone for xcb_query_text_extents_request_t {
    fn clone(&self) -> xcb_query_text_extents_request_t { *self }
}

#[repr(C)]
pub struct xcb_query_text_extents_reply_t {
    pub response_type :     u8,
    pub draw_direction :    u8,
    pub sequence :          u16,
    pub length :            u32,
    pub font_ascent :       i16,
    pub font_descent :      i16,
    pub overall_ascent :    i16,
    pub overall_descent :   i16,
    pub overall_width :     i32,
    pub overall_left :      i32,
    pub overall_right :     i32
}

impl Copy for xcb_query_text_extents_reply_t {}
impl Clone for xcb_query_text_extents_reply_t {
    fn clone(&self) -> xcb_query_text_extents_reply_t { *self }
}

#[repr(C)]
pub struct xcb_str_t {
    pub name_len :   u8
}

impl Copy for xcb_str_t {}
impl Clone for xcb_str_t {
    fn clone(&self) -> xcb_str_t { *self }
}
#[repr(C)]
pub struct xcb_str_iterator_t {
    pub data : *mut xcb_str_t,
    pub rem  : c_int,
    pub index: c_int
}


#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_list_fonts_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_list_fonts_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16,
    pub max_names :      u16,
    pub pattern_len :    u16
}

impl Copy for xcb_list_fonts_request_t {}
impl Clone for xcb_list_fonts_request_t {
    fn clone(&self) -> xcb_list_fonts_request_t { *self }
}

#[repr(C)]
pub struct xcb_list_fonts_reply_t {
    pub response_type :   u8,
    pub pad0 :            u8,
    pub sequence :        u16,
    pub length :          u32,
    pub names_len :       u16,
    pub pad1 :            [u8; 22]
}

impl Copy for xcb_list_fonts_reply_t {}
impl Clone for xcb_list_fonts_reply_t {
    fn clone(&self) -> xcb_list_fonts_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_list_fonts_with_info_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_list_fonts_with_info_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16,
    pub max_names :      u16,
    pub pattern_len :    u16
}

impl Copy for xcb_list_fonts_with_info_request_t {}
impl Clone for xcb_list_fonts_with_info_request_t {
    fn clone(&self) -> xcb_list_fonts_with_info_request_t { *self }
}

#[repr(C)]
pub struct xcb_list_fonts_with_info_reply_t {
    pub response_type :       u8,
    pub name_len :            u8,
    pub sequence :            u16,
    pub length :              u32,
    pub min_bounds :          xcb_charinfo_t,
    pub pad0 :                [u8; 4],
    pub max_bounds :          xcb_charinfo_t,
    pub pad1 :                [u8; 4],
    pub min_char_or_byte2 :   u16,
    pub max_char_or_byte2 :   u16,
    pub default_char :        u16,
    pub properties_len :      u16,
    pub draw_direction :      u8,
    pub min_byte1 :           u8,
    pub max_byte1 :           u8,
    pub all_chars_exist :     u8,
    pub font_ascent :         i16,
    pub font_descent :        i16,
    pub replies_hint :        u32
}

impl Copy for xcb_list_fonts_with_info_reply_t {}
impl Clone for xcb_list_fonts_with_info_reply_t {
    fn clone(&self) -> xcb_list_fonts_with_info_reply_t { *self }
}


#[repr(C)]
pub struct xcb_set_font_path_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16,
    pub font_qty :       u16,
    pub pad1 :           [u8; 2]
}

impl Copy for xcb_set_font_path_request_t {}
impl Clone for xcb_set_font_path_request_t {
    fn clone(&self) -> xcb_set_font_path_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_get_font_path_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_get_font_path_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16
}

impl Copy for xcb_get_font_path_request_t {}
impl Clone for xcb_get_font_path_request_t {
    fn clone(&self) -> xcb_get_font_path_request_t { *self }
}

#[repr(C)]
pub struct xcb_get_font_path_reply_t {
    pub response_type :   u8,
    pub pad0 :            u8,
    pub sequence :        u16,
    pub length :          u32,
    pub path_len :        u16,
    pub pad1 :            [u8; 22]
}

impl Copy for xcb_get_font_path_reply_t {}
impl Clone for xcb_get_font_path_reply_t {
    fn clone(&self) -> xcb_get_font_path_reply_t { *self }
}


#[repr(C)]
pub struct xcb_create_pixmap_request_t {
    pub major_opcode :   u8,
    pub depth :          u8,
    pub length :         u16,
    pub pid :            xcb_pixmap_t,
    pub drawable :       xcb_drawable_t,
    pub width :          u16,
    pub height :         u16
}

impl Copy for xcb_create_pixmap_request_t {}
impl Clone for xcb_create_pixmap_request_t {
    fn clone(&self) -> xcb_create_pixmap_request_t { *self }
}


#[repr(C)]
pub struct xcb_free_pixmap_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16,
    pub pixmap :         xcb_pixmap_t
}

impl Copy for xcb_free_pixmap_request_t {}
impl Clone for xcb_free_pixmap_request_t {
    fn clone(&self) -> xcb_free_pixmap_request_t { *self }
}


#[repr(C)]
pub struct xcb_create_gc_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16,
    pub cid :            xcb_gcontext_t,
    pub drawable :       xcb_drawable_t,
    pub value_mask :     u32
}

impl Copy for xcb_create_gc_request_t {}
impl Clone for xcb_create_gc_request_t {
    fn clone(&self) -> xcb_create_gc_request_t { *self }
}


#[repr(C)]
pub struct xcb_change_gc_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16,
    pub gc :             xcb_gcontext_t,
    pub value_mask :     u32
}

impl Copy for xcb_change_gc_request_t {}
impl Clone for xcb_change_gc_request_t {
    fn clone(&self) -> xcb_change_gc_request_t { *self }
}


#[repr(C)]
pub struct xcb_copy_gc_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16,
    pub src_gc :         xcb_gcontext_t,
    pub dst_gc :         xcb_gcontext_t,
    pub value_mask :     u32
}

impl Copy for xcb_copy_gc_request_t {}
impl Clone for xcb_copy_gc_request_t {
    fn clone(&self) -> xcb_copy_gc_request_t { *self }
}


#[repr(C)]
pub struct xcb_set_dashes_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16,
    pub gc :             xcb_gcontext_t,
    pub dash_offset :    u16,
    pub dashes_len :     u16
}

impl Copy for xcb_set_dashes_request_t {}
impl Clone for xcb_set_dashes_request_t {
    fn clone(&self) -> xcb_set_dashes_request_t { *self }
}


#[repr(C)]
pub struct xcb_set_clip_rectangles_request_t {
    pub major_opcode :    u8,
    pub ordering :        u8,
    pub length :          u16,
    pub gc :              xcb_gcontext_t,
    pub clip_x_origin :   i16,
    pub clip_y_origin :   i16
}

impl Copy for xcb_set_clip_rectangles_request_t {}
impl Clone for xcb_set_clip_rectangles_request_t {
    fn clone(&self) -> xcb_set_clip_rectangles_request_t { *self }
}


#[repr(C)]
pub struct xcb_free_gc_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16,
    pub gc :             xcb_gcontext_t
}

impl Copy for xcb_free_gc_request_t {}
impl Clone for xcb_free_gc_request_t {
    fn clone(&self) -> xcb_free_gc_request_t { *self }
}


#[repr(C)]
pub struct xcb_clear_area_request_t {
    pub major_opcode :   u8,
    pub exposures :      u8,
    pub length :         u16,
    pub window :         xcb_window_t,
    pub x :              i16,
    pub y :              i16,
    pub width :          u16,
    pub height :         u16
}

impl Copy for xcb_clear_area_request_t {}
impl Clone for xcb_clear_area_request_t {
    fn clone(&self) -> xcb_clear_area_request_t { *self }
}


#[repr(C)]
pub struct xcb_copy_area_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16,
    pub src_drawable :   xcb_drawable_t,
    pub dst_drawable :   xcb_drawable_t,
    pub gc :             xcb_gcontext_t,
    pub src_x :          i16,
    pub src_y :          i16,
    pub dst_x :          i16,
    pub dst_y :          i16,
    pub width :          u16,
    pub height :         u16
}

impl Copy for xcb_copy_area_request_t {}
impl Clone for xcb_copy_area_request_t {
    fn clone(&self) -> xcb_copy_area_request_t { *self }
}


#[repr(C)]
pub struct xcb_copy_plane_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16,
    pub src_drawable :   xcb_drawable_t,
    pub dst_drawable :   xcb_drawable_t,
    pub gc :             xcb_gcontext_t,
    pub src_x :          i16,
    pub src_y :          i16,
    pub dst_x :          i16,
    pub dst_y :          i16,
    pub width :          u16,
    pub height :         u16,
    pub bit_plane :      u32
}

impl Copy for xcb_copy_plane_request_t {}
impl Clone for xcb_copy_plane_request_t {
    fn clone(&self) -> xcb_copy_plane_request_t { *self }
}


#[repr(C)]
pub struct xcb_poly_point_request_t {
    pub major_opcode :      u8,
    pub coordinate_mode :   u8,
    pub length :            u16,
    pub drawable :          xcb_drawable_t,
    pub gc :                xcb_gcontext_t
}

impl Copy for xcb_poly_point_request_t {}
impl Clone for xcb_poly_point_request_t {
    fn clone(&self) -> xcb_poly_point_request_t { *self }
}


#[repr(C)]
pub struct xcb_poly_line_request_t {
    pub major_opcode :      u8,
    pub coordinate_mode :   u8,
    pub length :            u16,
    pub drawable :          xcb_drawable_t,
    pub gc :                xcb_gcontext_t
}

impl Copy for xcb_poly_line_request_t {}
impl Clone for xcb_poly_line_request_t {
    fn clone(&self) -> xcb_poly_line_request_t { *self }
}

#[repr(C)]
pub struct xcb_segment_t {
    pub x1 :   i16,
    pub y1 :   i16,
    pub x2 :   i16,
    pub y2 :   i16
}

impl Copy for xcb_segment_t {}
impl Clone for xcb_segment_t {
    fn clone(&self) -> xcb_segment_t { *self }
}
#[repr(C)]
pub struct xcb_segment_iterator_t {
    pub data : *mut xcb_segment_t,
    pub rem  : c_int,
    pub index: c_int
}



#[repr(C)]
pub struct xcb_poly_segment_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16,
    pub drawable :       xcb_drawable_t,
    pub gc :             xcb_gcontext_t
}

impl Copy for xcb_poly_segment_request_t {}
impl Clone for xcb_poly_segment_request_t {
    fn clone(&self) -> xcb_poly_segment_request_t { *self }
}


#[repr(C)]
pub struct xcb_poly_rectangle_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16,
    pub drawable :       xcb_drawable_t,
    pub gc :             xcb_gcontext_t
}

impl Copy for xcb_poly_rectangle_request_t {}
impl Clone for xcb_poly_rectangle_request_t {
    fn clone(&self) -> xcb_poly_rectangle_request_t { *self }
}


#[repr(C)]
pub struct xcb_poly_arc_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16,
    pub drawable :       xcb_drawable_t,
    pub gc :             xcb_gcontext_t
}

impl Copy for xcb_poly_arc_request_t {}
impl Clone for xcb_poly_arc_request_t {
    fn clone(&self) -> xcb_poly_arc_request_t { *self }
}


#[repr(C)]
pub struct xcb_fill_poly_request_t {
    pub major_opcode :      u8,
    pub pad0 :              u8,
    pub length :            u16,
    pub drawable :          xcb_drawable_t,
    pub gc :                xcb_gcontext_t,
    pub shape :             u8,
    pub coordinate_mode :   u8,
    pub pad1 :              [u8; 2]
}

impl Copy for xcb_fill_poly_request_t {}
impl Clone for xcb_fill_poly_request_t {
    fn clone(&self) -> xcb_fill_poly_request_t { *self }
}


#[repr(C)]
pub struct xcb_poly_fill_rectangle_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16,
    pub drawable :       xcb_drawable_t,
    pub gc :             xcb_gcontext_t
}

impl Copy for xcb_poly_fill_rectangle_request_t {}
impl Clone for xcb_poly_fill_rectangle_request_t {
    fn clone(&self) -> xcb_poly_fill_rectangle_request_t { *self }
}


#[repr(C)]
pub struct xcb_poly_fill_arc_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16,
    pub drawable :       xcb_drawable_t,
    pub gc :             xcb_gcontext_t
}

impl Copy for xcb_poly_fill_arc_request_t {}
impl Clone for xcb_poly_fill_arc_request_t {
    fn clone(&self) -> xcb_poly_fill_arc_request_t { *self }
}


#[repr(C)]
pub struct xcb_put_image_request_t {
    pub major_opcode :   u8,
    pub format :         u8,
    pub length :         u16,
    pub drawable :       xcb_drawable_t,
    pub gc :             xcb_gcontext_t,
    pub width :          u16,
    pub height :         u16,
    pub dst_x :          i16,
    pub dst_y :          i16,
    pub left_pad :       u8,
    pub depth :          u8,
    pub pad0 :           [u8; 2]
}

impl Copy for xcb_put_image_request_t {}
impl Clone for xcb_put_image_request_t {
    fn clone(&self) -> xcb_put_image_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_get_image_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_get_image_request_t {
    pub major_opcode :   u8,
    pub format :         u8,
    pub length :         u16,
    pub drawable :       xcb_drawable_t,
    pub x :              i16,
    pub y :              i16,
    pub width :          u16,
    pub height :         u16,
    pub plane_mask :     u32
}

impl Copy for xcb_get_image_request_t {}
impl Clone for xcb_get_image_request_t {
    fn clone(&self) -> xcb_get_image_request_t { *self }
}

#[repr(C)]
pub struct xcb_get_image_reply_t {
    pub response_type :   u8,
    pub depth :           u8,
    pub sequence :        u16,
    pub length :          u32,
    pub visual :          xcb_visualid_t,
    pub pad0 :            [u8; 20]
}

impl Copy for xcb_get_image_reply_t {}
impl Clone for xcb_get_image_reply_t {
    fn clone(&self) -> xcb_get_image_reply_t { *self }
}


#[repr(C)]
pub struct xcb_poly_text_8_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16,
    pub drawable :       xcb_drawable_t,
    pub gc :             xcb_gcontext_t,
    pub x :              i16,
    pub y :              i16
}

impl Copy for xcb_poly_text_8_request_t {}
impl Clone for xcb_poly_text_8_request_t {
    fn clone(&self) -> xcb_poly_text_8_request_t { *self }
}


#[repr(C)]
pub struct xcb_poly_text_16_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16,
    pub drawable :       xcb_drawable_t,
    pub gc :             xcb_gcontext_t,
    pub x :              i16,
    pub y :              i16
}

impl Copy for xcb_poly_text_16_request_t {}
impl Clone for xcb_poly_text_16_request_t {
    fn clone(&self) -> xcb_poly_text_16_request_t { *self }
}


#[repr(C)]
pub struct xcb_image_text_8_request_t {
    pub major_opcode :   u8,
    pub string_len :     u8,
    pub length :         u16,
    pub drawable :       xcb_drawable_t,
    pub gc :             xcb_gcontext_t,
    pub x :              i16,
    pub y :              i16
}

impl Copy for xcb_image_text_8_request_t {}
impl Clone for xcb_image_text_8_request_t {
    fn clone(&self) -> xcb_image_text_8_request_t { *self }
}


#[repr(C)]
pub struct xcb_image_text_16_request_t {
    pub major_opcode :   u8,
    pub string_len :     u8,
    pub length :         u16,
    pub drawable :       xcb_drawable_t,
    pub gc :             xcb_gcontext_t,
    pub x :              i16,
    pub y :              i16
}

impl Copy for xcb_image_text_16_request_t {}
impl Clone for xcb_image_text_16_request_t {
    fn clone(&self) -> xcb_image_text_16_request_t { *self }
}


#[repr(C)]
pub struct xcb_create_colormap_request_t {
    pub major_opcode :   u8,
    pub alloc :          u8,
    pub length :         u16,
    pub mid :            xcb_colormap_t,
    pub window :         xcb_window_t,
    pub visual :         xcb_visualid_t
}

impl Copy for xcb_create_colormap_request_t {}
impl Clone for xcb_create_colormap_request_t {
    fn clone(&self) -> xcb_create_colormap_request_t { *self }
}


#[repr(C)]
pub struct xcb_free_colormap_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16,
    pub cmap :           xcb_colormap_t
}

impl Copy for xcb_free_colormap_request_t {}
impl Clone for xcb_free_colormap_request_t {
    fn clone(&self) -> xcb_free_colormap_request_t { *self }
}


#[repr(C)]
pub struct xcb_copy_colormap_and_free_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16,
    pub mid :            xcb_colormap_t,
    pub src_cmap :       xcb_colormap_t
}

impl Copy for xcb_copy_colormap_and_free_request_t {}
impl Clone for xcb_copy_colormap_and_free_request_t {
    fn clone(&self) -> xcb_copy_colormap_and_free_request_t { *self }
}


#[repr(C)]
pub struct xcb_install_colormap_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16,
    pub cmap :           xcb_colormap_t
}

impl Copy for xcb_install_colormap_request_t {}
impl Clone for xcb_install_colormap_request_t {
    fn clone(&self) -> xcb_install_colormap_request_t { *self }
}


#[repr(C)]
pub struct xcb_uninstall_colormap_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16,
    pub cmap :           xcb_colormap_t
}

impl Copy for xcb_uninstall_colormap_request_t {}
impl Clone for xcb_uninstall_colormap_request_t {
    fn clone(&self) -> xcb_uninstall_colormap_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_list_installed_colormaps_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_list_installed_colormaps_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16,
    pub window :         xcb_window_t
}

impl Copy for xcb_list_installed_colormaps_request_t {}
impl Clone for xcb_list_installed_colormaps_request_t {
    fn clone(&self) -> xcb_list_installed_colormaps_request_t { *self }
}

#[repr(C)]
pub struct xcb_list_installed_colormaps_reply_t {
    pub response_type :   u8,
    pub pad0 :            u8,
    pub sequence :        u16,
    pub length :          u32,
    pub cmaps_len :       u16,
    pub pad1 :            [u8; 22]
}

impl Copy for xcb_list_installed_colormaps_reply_t {}
impl Clone for xcb_list_installed_colormaps_reply_t {
    fn clone(&self) -> xcb_list_installed_colormaps_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_alloc_color_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_alloc_color_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16,
    pub cmap :           xcb_colormap_t,
    pub red :            u16,
    pub green :          u16,
    pub blue :           u16,
    pub pad1 :           [u8; 2]
}

impl Copy for xcb_alloc_color_request_t {}
impl Clone for xcb_alloc_color_request_t {
    fn clone(&self) -> xcb_alloc_color_request_t { *self }
}

#[repr(C)]
pub struct xcb_alloc_color_reply_t {
    pub response_type :   u8,
    pub pad0 :            u8,
    pub sequence :        u16,
    pub length :          u32,
    pub red :             u16,
    pub green :           u16,
    pub blue :            u16,
    pub pad1 :            [u8; 2],
    pub pixel :           u32
}

impl Copy for xcb_alloc_color_reply_t {}
impl Clone for xcb_alloc_color_reply_t {
    fn clone(&self) -> xcb_alloc_color_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_alloc_named_color_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_alloc_named_color_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16,
    pub cmap :           xcb_colormap_t,
    pub name_len :       u16,
    pub pad1 :           [u8; 2]
}

impl Copy for xcb_alloc_named_color_request_t {}
impl Clone for xcb_alloc_named_color_request_t {
    fn clone(&self) -> xcb_alloc_named_color_request_t { *self }
}

#[repr(C)]
pub struct xcb_alloc_named_color_reply_t {
    pub response_type :   u8,
    pub pad0 :            u8,
    pub sequence :        u16,
    pub length :          u32,
    pub pixel :           u32,
    pub exact_red :       u16,
    pub exact_green :     u16,
    pub exact_blue :      u16,
    pub visual_red :      u16,
    pub visual_green :    u16,
    pub visual_blue :     u16
}

impl Copy for xcb_alloc_named_color_reply_t {}
impl Clone for xcb_alloc_named_color_reply_t {
    fn clone(&self) -> xcb_alloc_named_color_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_alloc_color_cells_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_alloc_color_cells_request_t {
    pub major_opcode :   u8,
    pub contiguous :     u8,
    pub length :         u16,
    pub cmap :           xcb_colormap_t,
    pub colors :         u16,
    pub planes :         u16
}

impl Copy for xcb_alloc_color_cells_request_t {}
impl Clone for xcb_alloc_color_cells_request_t {
    fn clone(&self) -> xcb_alloc_color_cells_request_t { *self }
}

#[repr(C)]
pub struct xcb_alloc_color_cells_reply_t {
    pub response_type :   u8,
    pub pad0 :            u8,
    pub sequence :        u16,
    pub length :          u32,
    pub pixels_len :      u16,
    pub masks_len :       u16,
    pub pad1 :            [u8; 20]
}

impl Copy for xcb_alloc_color_cells_reply_t {}
impl Clone for xcb_alloc_color_cells_reply_t {
    fn clone(&self) -> xcb_alloc_color_cells_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_alloc_color_planes_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_alloc_color_planes_request_t {
    pub major_opcode :   u8,
    pub contiguous :     u8,
    pub length :         u16,
    pub cmap :           xcb_colormap_t,
    pub colors :         u16,
    pub reds :           u16,
    pub greens :         u16,
    pub blues :          u16
}

impl Copy for xcb_alloc_color_planes_request_t {}
impl Clone for xcb_alloc_color_planes_request_t {
    fn clone(&self) -> xcb_alloc_color_planes_request_t { *self }
}

#[repr(C)]
pub struct xcb_alloc_color_planes_reply_t {
    pub response_type :   u8,
    pub pad0 :            u8,
    pub sequence :        u16,
    pub length :          u32,
    pub pixels_len :      u16,
    pub pad1 :            [u8; 2],
    pub red_mask :        u32,
    pub green_mask :      u32,
    pub blue_mask :       u32,
    pub pad2 :            [u8; 8]
}

impl Copy for xcb_alloc_color_planes_reply_t {}
impl Clone for xcb_alloc_color_planes_reply_t {
    fn clone(&self) -> xcb_alloc_color_planes_reply_t { *self }
}


#[repr(C)]
pub struct xcb_free_colors_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16,
    pub cmap :           xcb_colormap_t,
    pub plane_mask :     u32
}

impl Copy for xcb_free_colors_request_t {}
impl Clone for xcb_free_colors_request_t {
    fn clone(&self) -> xcb_free_colors_request_t { *self }
}

#[repr(C)]
pub struct xcb_coloritem_t {
    pub pixel :   u32,
    pub red :     u16,
    pub green :   u16,
    pub blue :    u16,
    pub flags :   u8,
    pub pad0 :    u8
}

impl Copy for xcb_coloritem_t {}
impl Clone for xcb_coloritem_t {
    fn clone(&self) -> xcb_coloritem_t { *self }
}
#[repr(C)]
pub struct xcb_coloritem_iterator_t {
    pub data : *mut xcb_coloritem_t,
    pub rem  : c_int,
    pub index: c_int
}



#[repr(C)]
pub struct xcb_store_colors_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16,
    pub cmap :           xcb_colormap_t
}

impl Copy for xcb_store_colors_request_t {}
impl Clone for xcb_store_colors_request_t {
    fn clone(&self) -> xcb_store_colors_request_t { *self }
}


#[repr(C)]
pub struct xcb_store_named_color_request_t {
    pub major_opcode :   u8,
    pub flags :          u8,
    pub length :         u16,
    pub cmap :           xcb_colormap_t,
    pub pixel :          u32,
    pub name_len :       u16,
    pub pad0 :           [u8; 2]
}

impl Copy for xcb_store_named_color_request_t {}
impl Clone for xcb_store_named_color_request_t {
    fn clone(&self) -> xcb_store_named_color_request_t { *self }
}

#[repr(C)]
pub struct xcb_rgb_t {
    pub red :     u16,
    pub green :   u16,
    pub blue :    u16,
    pub pad0 :    [u8; 2]
}

impl Copy for xcb_rgb_t {}
impl Clone for xcb_rgb_t {
    fn clone(&self) -> xcb_rgb_t { *self }
}
#[repr(C)]
pub struct xcb_rgb_iterator_t {
    pub data : *mut xcb_rgb_t,
    pub rem  : c_int,
    pub index: c_int
}


#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_query_colors_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_query_colors_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16,
    pub cmap :           xcb_colormap_t
}

impl Copy for xcb_query_colors_request_t {}
impl Clone for xcb_query_colors_request_t {
    fn clone(&self) -> xcb_query_colors_request_t { *self }
}

#[repr(C)]
pub struct xcb_query_colors_reply_t {
    pub response_type :   u8,
    pub pad0 :            u8,
    pub sequence :        u16,
    pub length :          u32,
    pub colors_len :      u16,
    pub pad1 :            [u8; 22]
}

impl Copy for xcb_query_colors_reply_t {}
impl Clone for xcb_query_colors_reply_t {
    fn clone(&self) -> xcb_query_colors_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_lookup_color_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_lookup_color_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16,
    pub cmap :           xcb_colormap_t,
    pub name_len :       u16,
    pub pad1 :           [u8; 2]
}

impl Copy for xcb_lookup_color_request_t {}
impl Clone for xcb_lookup_color_request_t {
    fn clone(&self) -> xcb_lookup_color_request_t { *self }
}

#[repr(C)]
pub struct xcb_lookup_color_reply_t {
    pub response_type :   u8,
    pub pad0 :            u8,
    pub sequence :        u16,
    pub length :          u32,
    pub exact_red :       u16,
    pub exact_green :     u16,
    pub exact_blue :      u16,
    pub visual_red :      u16,
    pub visual_green :    u16,
    pub visual_blue :     u16
}

impl Copy for xcb_lookup_color_reply_t {}
impl Clone for xcb_lookup_color_reply_t {
    fn clone(&self) -> xcb_lookup_color_reply_t { *self }
}


#[repr(C)]
pub struct xcb_create_cursor_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16,
    pub cid :            xcb_cursor_t,
    pub source :         xcb_pixmap_t,
    pub mask :           xcb_pixmap_t,
    pub fore_red :       u16,
    pub fore_green :     u16,
    pub fore_blue :      u16,
    pub back_red :       u16,
    pub back_green :     u16,
    pub back_blue :      u16,
    pub x :              u16,
    pub y :              u16
}

impl Copy for xcb_create_cursor_request_t {}
impl Clone for xcb_create_cursor_request_t {
    fn clone(&self) -> xcb_create_cursor_request_t { *self }
}


#[repr(C)]
pub struct xcb_create_glyph_cursor_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16,
    pub cid :            xcb_cursor_t,
    pub source_font :    xcb_font_t,
    pub mask_font :      xcb_font_t,
    pub source_char :    u16,
    pub mask_char :      u16,
    pub fore_red :       u16,
    pub fore_green :     u16,
    pub fore_blue :      u16,
    pub back_red :       u16,
    pub back_green :     u16,
    pub back_blue :      u16
}

impl Copy for xcb_create_glyph_cursor_request_t {}
impl Clone for xcb_create_glyph_cursor_request_t {
    fn clone(&self) -> xcb_create_glyph_cursor_request_t { *self }
}


#[repr(C)]
pub struct xcb_free_cursor_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16,
    pub cursor :         xcb_cursor_t
}

impl Copy for xcb_free_cursor_request_t {}
impl Clone for xcb_free_cursor_request_t {
    fn clone(&self) -> xcb_free_cursor_request_t { *self }
}


#[repr(C)]
pub struct xcb_recolor_cursor_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16,
    pub cursor :         xcb_cursor_t,
    pub fore_red :       u16,
    pub fore_green :     u16,
    pub fore_blue :      u16,
    pub back_red :       u16,
    pub back_green :     u16,
    pub back_blue :      u16
}

impl Copy for xcb_recolor_cursor_request_t {}
impl Clone for xcb_recolor_cursor_request_t {
    fn clone(&self) -> xcb_recolor_cursor_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_query_best_size_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_query_best_size_request_t {
    pub major_opcode :   u8,
    pub class :          u8,
    pub length :         u16,
    pub drawable :       xcb_drawable_t,
    pub width :          u16,
    pub height :         u16
}

impl Copy for xcb_query_best_size_request_t {}
impl Clone for xcb_query_best_size_request_t {
    fn clone(&self) -> xcb_query_best_size_request_t { *self }
}

#[repr(C)]
pub struct xcb_query_best_size_reply_t {
    pub response_type :   u8,
    pub pad0 :            u8,
    pub sequence :        u16,
    pub length :          u32,
    pub width :           u16,
    pub height :          u16
}

impl Copy for xcb_query_best_size_reply_t {}
impl Clone for xcb_query_best_size_reply_t {
    fn clone(&self) -> xcb_query_best_size_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_query_extension_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_query_extension_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16,
    pub name_len :       u16,
    pub pad1 :           [u8; 2]
}

impl Copy for xcb_query_extension_request_t {}
impl Clone for xcb_query_extension_request_t {
    fn clone(&self) -> xcb_query_extension_request_t { *self }
}

#[repr(C)]
pub struct xcb_query_extension_reply_t {
    pub response_type :   u8,
    pub pad0 :            u8,
    pub sequence :        u16,
    pub length :          u32,
    pub present :         u8,
    pub major_opcode :    u8,
    pub first_event :     u8,
    pub first_error :     u8
}

impl Copy for xcb_query_extension_reply_t {}
impl Clone for xcb_query_extension_reply_t {
    fn clone(&self) -> xcb_query_extension_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_list_extensions_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_list_extensions_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16
}

impl Copy for xcb_list_extensions_request_t {}
impl Clone for xcb_list_extensions_request_t {
    fn clone(&self) -> xcb_list_extensions_request_t { *self }
}

#[repr(C)]
pub struct xcb_list_extensions_reply_t {
    pub response_type :   u8,
    pub names_len :       u8,
    pub sequence :        u16,
    pub length :          u32,
    pub pad0 :            [u8; 24]
}

impl Copy for xcb_list_extensions_reply_t {}
impl Clone for xcb_list_extensions_reply_t {
    fn clone(&self) -> xcb_list_extensions_reply_t { *self }
}


#[repr(C)]
pub struct xcb_change_keyboard_mapping_request_t {
    pub major_opcode :          u8,
    pub keycode_count :         u8,
    pub length :                u16,
    pub first_keycode :         xcb_keycode_t,
    pub keysyms_per_keycode :   u8,
    pub pad0 :                  [u8; 2]
}

impl Copy for xcb_change_keyboard_mapping_request_t {}
impl Clone for xcb_change_keyboard_mapping_request_t {
    fn clone(&self) -> xcb_change_keyboard_mapping_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_get_keyboard_mapping_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_get_keyboard_mapping_request_t {
    pub major_opcode :    u8,
    pub pad0 :            u8,
    pub length :          u16,
    pub first_keycode :   xcb_keycode_t,
    pub count :           u8
}

impl Copy for xcb_get_keyboard_mapping_request_t {}
impl Clone for xcb_get_keyboard_mapping_request_t {
    fn clone(&self) -> xcb_get_keyboard_mapping_request_t { *self }
}

#[repr(C)]
pub struct xcb_get_keyboard_mapping_reply_t {
    pub response_type :         u8,
    pub keysyms_per_keycode :   u8,
    pub sequence :              u16,
    pub length :                u32,
    pub pad0 :                  [u8; 24]
}

impl Copy for xcb_get_keyboard_mapping_reply_t {}
impl Clone for xcb_get_keyboard_mapping_reply_t {
    fn clone(&self) -> xcb_get_keyboard_mapping_reply_t { *self }
}


#[repr(C)]
pub struct xcb_change_keyboard_control_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16,
    pub value_mask :     u32
}

impl Copy for xcb_change_keyboard_control_request_t {}
impl Clone for xcb_change_keyboard_control_request_t {
    fn clone(&self) -> xcb_change_keyboard_control_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_get_keyboard_control_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_get_keyboard_control_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16
}

impl Copy for xcb_get_keyboard_control_request_t {}
impl Clone for xcb_get_keyboard_control_request_t {
    fn clone(&self) -> xcb_get_keyboard_control_request_t { *self }
}

#[repr(C)]
pub struct xcb_get_keyboard_control_reply_t {
    pub response_type :        u8,
    pub global_auto_repeat :   u8,
    pub sequence :             u16,
    pub length :               u32,
    pub led_mask :             u32,
    pub key_click_percent :    u8,
    pub bell_percent :         u8,
    pub bell_pitch :           u16,
    pub bell_duration :        u16,
    pub pad0 :                 [u8; 2],
    pub auto_repeats :         [u8; 32]
}

impl Copy for xcb_get_keyboard_control_reply_t {}
impl Clone for xcb_get_keyboard_control_reply_t {
    fn clone(&self) -> xcb_get_keyboard_control_reply_t { *self }
}


#[repr(C)]
pub struct xcb_bell_request_t {
    pub major_opcode :   u8,
    pub percent :        i8,
    pub length :         u16
}

impl Copy for xcb_bell_request_t {}
impl Clone for xcb_bell_request_t {
    fn clone(&self) -> xcb_bell_request_t { *self }
}


#[repr(C)]
pub struct xcb_change_pointer_control_request_t {
    pub major_opcode :               u8,
    pub pad0 :                       u8,
    pub length :                     u16,
    pub acceleration_numerator :     i16,
    pub acceleration_denominator :   i16,
    pub threshold :                  i16,
    pub do_acceleration :            u8,
    pub do_threshold :               u8
}

impl Copy for xcb_change_pointer_control_request_t {}
impl Clone for xcb_change_pointer_control_request_t {
    fn clone(&self) -> xcb_change_pointer_control_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_get_pointer_control_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_get_pointer_control_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16
}

impl Copy for xcb_get_pointer_control_request_t {}
impl Clone for xcb_get_pointer_control_request_t {
    fn clone(&self) -> xcb_get_pointer_control_request_t { *self }
}

#[repr(C)]
pub struct xcb_get_pointer_control_reply_t {
    pub response_type :              u8,
    pub pad0 :                       u8,
    pub sequence :                   u16,
    pub length :                     u32,
    pub acceleration_numerator :     u16,
    pub acceleration_denominator :   u16,
    pub threshold :                  u16,
    pub pad1 :                       [u8; 18]
}

impl Copy for xcb_get_pointer_control_reply_t {}
impl Clone for xcb_get_pointer_control_reply_t {
    fn clone(&self) -> xcb_get_pointer_control_reply_t { *self }
}


#[repr(C)]
pub struct xcb_set_screen_saver_request_t {
    pub major_opcode :      u8,
    pub pad0 :              u8,
    pub length :            u16,
    pub timeout :           i16,
    pub interval :          i16,
    pub prefer_blanking :   u8,
    pub allow_exposures :   u8
}

impl Copy for xcb_set_screen_saver_request_t {}
impl Clone for xcb_set_screen_saver_request_t {
    fn clone(&self) -> xcb_set_screen_saver_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_get_screen_saver_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_get_screen_saver_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16
}

impl Copy for xcb_get_screen_saver_request_t {}
impl Clone for xcb_get_screen_saver_request_t {
    fn clone(&self) -> xcb_get_screen_saver_request_t { *self }
}

#[repr(C)]
pub struct xcb_get_screen_saver_reply_t {
    pub response_type :     u8,
    pub pad0 :              u8,
    pub sequence :          u16,
    pub length :            u32,
    pub timeout :           u16,
    pub interval :          u16,
    pub prefer_blanking :   u8,
    pub allow_exposures :   u8,
    pub pad1 :              [u8; 18]
}

impl Copy for xcb_get_screen_saver_reply_t {}
impl Clone for xcb_get_screen_saver_reply_t {
    fn clone(&self) -> xcb_get_screen_saver_reply_t { *self }
}


#[repr(C)]
pub struct xcb_change_hosts_request_t {
    pub major_opcode :   u8,
    pub mode :           u8,
    pub length :         u16,
    pub family :         u8,
    pub pad0 :           u8,
    pub address_len :    u16
}

impl Copy for xcb_change_hosts_request_t {}
impl Clone for xcb_change_hosts_request_t {
    fn clone(&self) -> xcb_change_hosts_request_t { *self }
}

#[repr(C)]
pub struct xcb_host_t {
    pub family :        u8,
    pub pad0 :          u8,
    pub address_len :   u16
}

impl Copy for xcb_host_t {}
impl Clone for xcb_host_t {
    fn clone(&self) -> xcb_host_t { *self }
}
#[repr(C)]
pub struct xcb_host_iterator_t {
    pub data : *mut xcb_host_t,
    pub rem  : c_int,
    pub index: c_int
}


#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_list_hosts_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_list_hosts_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16
}

impl Copy for xcb_list_hosts_request_t {}
impl Clone for xcb_list_hosts_request_t {
    fn clone(&self) -> xcb_list_hosts_request_t { *self }
}

#[repr(C)]
pub struct xcb_list_hosts_reply_t {
    pub response_type :   u8,
    pub mode :            u8,
    pub sequence :        u16,
    pub length :          u32,
    pub hosts_len :       u16,
    pub pad0 :            [u8; 22]
}

impl Copy for xcb_list_hosts_reply_t {}
impl Clone for xcb_list_hosts_reply_t {
    fn clone(&self) -> xcb_list_hosts_reply_t { *self }
}


#[repr(C)]
pub struct xcb_set_access_control_request_t {
    pub major_opcode :   u8,
    pub mode :           u8,
    pub length :         u16
}

impl Copy for xcb_set_access_control_request_t {}
impl Clone for xcb_set_access_control_request_t {
    fn clone(&self) -> xcb_set_access_control_request_t { *self }
}


#[repr(C)]
pub struct xcb_set_close_down_mode_request_t {
    pub major_opcode :   u8,
    pub mode :           u8,
    pub length :         u16
}

impl Copy for xcb_set_close_down_mode_request_t {}
impl Clone for xcb_set_close_down_mode_request_t {
    fn clone(&self) -> xcb_set_close_down_mode_request_t { *self }
}


#[repr(C)]
pub struct xcb_kill_client_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16,
    pub resource :       u32
}

impl Copy for xcb_kill_client_request_t {}
impl Clone for xcb_kill_client_request_t {
    fn clone(&self) -> xcb_kill_client_request_t { *self }
}


#[repr(C)]
pub struct xcb_rotate_properties_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16,
    pub window :         xcb_window_t,
    pub atoms_len :      u16,
    pub delta :          i16
}

impl Copy for xcb_rotate_properties_request_t {}
impl Clone for xcb_rotate_properties_request_t {
    fn clone(&self) -> xcb_rotate_properties_request_t { *self }
}


#[repr(C)]
pub struct xcb_force_screen_saver_request_t {
    pub major_opcode :   u8,
    pub mode :           u8,
    pub length :         u16
}

impl Copy for xcb_force_screen_saver_request_t {}
impl Clone for xcb_force_screen_saver_request_t {
    fn clone(&self) -> xcb_force_screen_saver_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_set_pointer_mapping_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_set_pointer_mapping_request_t {
    pub major_opcode :   u8,
    pub map_len :        u8,
    pub length :         u16
}

impl Copy for xcb_set_pointer_mapping_request_t {}
impl Clone for xcb_set_pointer_mapping_request_t {
    fn clone(&self) -> xcb_set_pointer_mapping_request_t { *self }
}

#[repr(C)]
pub struct xcb_set_pointer_mapping_reply_t {
    pub response_type :   u8,
    pub status :          u8,
    pub sequence :        u16,
    pub length :          u32
}

impl Copy for xcb_set_pointer_mapping_reply_t {}
impl Clone for xcb_set_pointer_mapping_reply_t {
    fn clone(&self) -> xcb_set_pointer_mapping_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_get_pointer_mapping_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_get_pointer_mapping_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16
}

impl Copy for xcb_get_pointer_mapping_request_t {}
impl Clone for xcb_get_pointer_mapping_request_t {
    fn clone(&self) -> xcb_get_pointer_mapping_request_t { *self }
}

#[repr(C)]
pub struct xcb_get_pointer_mapping_reply_t {
    pub response_type :   u8,
    pub map_len :         u8,
    pub sequence :        u16,
    pub length :          u32,
    pub pad0 :            [u8; 24]
}

impl Copy for xcb_get_pointer_mapping_reply_t {}
impl Clone for xcb_get_pointer_mapping_reply_t {
    fn clone(&self) -> xcb_get_pointer_mapping_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_set_modifier_mapping_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_set_modifier_mapping_request_t {
    pub major_opcode :            u8,
    pub keycodes_per_modifier :   u8,
    pub length :                  u16
}

impl Copy for xcb_set_modifier_mapping_request_t {}
impl Clone for xcb_set_modifier_mapping_request_t {
    fn clone(&self) -> xcb_set_modifier_mapping_request_t { *self }
}

#[repr(C)]
pub struct xcb_set_modifier_mapping_reply_t {
    pub response_type :   u8,
    pub status :          u8,
    pub sequence :        u16,
    pub length :          u32
}

impl Copy for xcb_set_modifier_mapping_reply_t {}
impl Clone for xcb_set_modifier_mapping_reply_t {
    fn clone(&self) -> xcb_set_modifier_mapping_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_get_modifier_mapping_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_get_modifier_mapping_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16
}

impl Copy for xcb_get_modifier_mapping_request_t {}
impl Clone for xcb_get_modifier_mapping_request_t {
    fn clone(&self) -> xcb_get_modifier_mapping_request_t { *self }
}

#[repr(C)]
pub struct xcb_get_modifier_mapping_reply_t {
    pub response_type :           u8,
    pub keycodes_per_modifier :   u8,
    pub sequence :                u16,
    pub length :                  u32,
    pub pad0 :                    [u8; 24]
}

impl Copy for xcb_get_modifier_mapping_reply_t {}
impl Clone for xcb_get_modifier_mapping_reply_t {
    fn clone(&self) -> xcb_get_modifier_mapping_reply_t { *self }
}


#[repr(C)]
pub struct xcb_no_operation_request_t {
    pub major_opcode :   u8,
    pub pad0 :           u8,
    pub length :         u16
}

impl Copy for xcb_no_operation_request_t {}
impl Clone for xcb_no_operation_request_t {
    fn clone(&self) -> xcb_no_operation_request_t { *self }
}
extern "C" {

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_char2b_t)
///
pub fn xcb_char2b_next (i:*mut xcb_char2b_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_char2b_end (i:xcb_char2b_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_window_t)
///
pub fn xcb_window_next (i:*mut xcb_window_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_window_end (i:xcb_window_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_pixmap_t)
///
pub fn xcb_pixmap_next (i:*mut xcb_pixmap_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_pixmap_end (i:xcb_pixmap_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_cursor_t)
///
pub fn xcb_cursor_next (i:*mut xcb_cursor_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_cursor_end (i:xcb_cursor_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_font_t)
///
pub fn xcb_font_next (i:*mut xcb_font_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_font_end (i:xcb_font_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_gcontext_t)
///
pub fn xcb_gcontext_next (i:*mut xcb_gcontext_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_gcontext_end (i:xcb_gcontext_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_colormap_t)
///
pub fn xcb_colormap_next (i:*mut xcb_colormap_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_colormap_end (i:xcb_colormap_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_atom_t)
///
pub fn xcb_atom_next (i:*mut xcb_atom_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_atom_end (i:xcb_atom_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_drawable_t)
///
pub fn xcb_drawable_next (i:*mut xcb_drawable_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_drawable_end (i:xcb_drawable_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_fontable_t)
///
pub fn xcb_fontable_next (i:*mut xcb_fontable_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_fontable_end (i:xcb_fontable_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_visualid_t)
///
pub fn xcb_visualid_next (i:*mut xcb_visualid_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_visualid_end (i:xcb_visualid_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_timestamp_t)
///
pub fn xcb_timestamp_next (i:*mut xcb_timestamp_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_timestamp_end (i:xcb_timestamp_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_keysym_t)
///
pub fn xcb_keysym_next (i:*mut xcb_keysym_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_keysym_end (i:xcb_keysym_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_keycode_t)
///
pub fn xcb_keycode_next (i:*mut xcb_keycode_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_keycode_end (i:xcb_keycode_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_button_t)
///
pub fn xcb_button_next (i:*mut xcb_button_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_button_end (i:xcb_button_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_point_t)
///
pub fn xcb_point_next (i:*mut xcb_point_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_point_end (i:xcb_point_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_rectangle_t)
///
pub fn xcb_rectangle_next (i:*mut xcb_rectangle_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_rectangle_end (i:xcb_rectangle_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_arc_t)
///
pub fn xcb_arc_next (i:*mut xcb_arc_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_arc_end (i:xcb_arc_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_format_t)
///
pub fn xcb_format_next (i:*mut xcb_format_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_format_end (i:xcb_format_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_visualtype_t)
///
pub fn xcb_visualtype_next (i:*mut xcb_visualtype_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_visualtype_end (i:xcb_visualtype_iterator_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_depth_sizeof (_buffer :  *mut c_void) -> c_int;

pub fn xcb_depth_visuals (R : *mut xcb_depth_t) -> *mut xcb_visualtype_t;


pub fn xcb_depth_visuals_length (R : *mut xcb_depth_t) -> c_int;

pub fn xcb_depth_visuals_iterator (R : *mut xcb_depth_t) -> xcb_visualtype_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_depth_t)
///
pub fn xcb_depth_next (i:*mut xcb_depth_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_depth_end (i:xcb_depth_iterator_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_screen_sizeof (_buffer :  *mut c_void) -> c_int;


pub fn xcb_screen_allowed_depths_length (R : *mut xcb_screen_t) -> c_int;

pub fn xcb_screen_allowed_depths_iterator (R : *mut xcb_screen_t) -> xcb_depth_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_screen_t)
///
pub fn xcb_screen_next (i:*mut xcb_screen_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_screen_end (i:xcb_screen_iterator_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_setup_request_sizeof (_buffer :  *mut c_void) -> c_int;

pub fn xcb_setup_request_authorization_protocol_name (R : *mut xcb_setup_request_t) -> *mut c_char;


pub fn xcb_setup_request_authorization_protocol_name_length (R : *mut xcb_setup_request_t) -> c_int;


pub fn xcb_setup_request_authorization_protocol_name_end (R : *mut xcb_setup_request_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_setup_request_authorization_protocol_data (R : *mut xcb_setup_request_t) -> *mut c_char;


pub fn xcb_setup_request_authorization_protocol_data_length (R : *mut xcb_setup_request_t) -> c_int;


pub fn xcb_setup_request_authorization_protocol_data_end (R : *mut xcb_setup_request_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_setup_request_t)
///
pub fn xcb_setup_request_next (i:*mut xcb_setup_request_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_setup_request_end (i:xcb_setup_request_iterator_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_setup_failed_sizeof (_buffer :  *mut c_void) -> c_int;

pub fn xcb_setup_failed_reason (R : *mut xcb_setup_failed_t) -> *mut c_char;


pub fn xcb_setup_failed_reason_length (R : *mut xcb_setup_failed_t) -> c_int;


pub fn xcb_setup_failed_reason_end (R : *mut xcb_setup_failed_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_setup_failed_t)
///
pub fn xcb_setup_failed_next (i:*mut xcb_setup_failed_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_setup_failed_end (i:xcb_setup_failed_iterator_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_setup_authenticate_sizeof (_buffer :  *mut c_void) -> c_int;

pub fn xcb_setup_authenticate_reason (R : *mut xcb_setup_authenticate_t) -> *mut c_char;


pub fn xcb_setup_authenticate_reason_length (R : *mut xcb_setup_authenticate_t) -> c_int;


pub fn xcb_setup_authenticate_reason_end (R : *mut xcb_setup_authenticate_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_setup_authenticate_t)
///
pub fn xcb_setup_authenticate_next (i:*mut xcb_setup_authenticate_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_setup_authenticate_end (i:xcb_setup_authenticate_iterator_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_setup_sizeof (_buffer :  *mut c_void) -> c_int;

pub fn xcb_setup_vendor (R : *mut xcb_setup_t) -> *mut c_char;


pub fn xcb_setup_vendor_length (R : *mut xcb_setup_t) -> c_int;


pub fn xcb_setup_vendor_end (R : *mut xcb_setup_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_setup_pixmap_formats (R : *mut xcb_setup_t) -> *mut xcb_format_t;


pub fn xcb_setup_pixmap_formats_length (R : *mut xcb_setup_t) -> c_int;

pub fn xcb_setup_pixmap_formats_iterator (R : *mut xcb_setup_t) -> xcb_format_iterator_t;


pub fn xcb_setup_roots_length (R : *mut xcb_setup_t) -> c_int;

pub fn xcb_setup_roots_iterator (R : *mut xcb_setup_t) -> xcb_screen_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_client_message_data_t)
///
pub fn xcb_client_message_data_next (i:*mut xcb_client_message_data_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_client_message_data_end (i:xcb_client_message_data_iterator_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_create_window_sizeof (_buffer :  *mut c_void) -> c_int;

/// Creates a window
///
/// # Arguments
///  * `depth` Specifies the new window's depth (TODO: what unit?).
///
///       The special value `XCB_COPY_FROM_PARENT` means the depth is taken from the
///       `parent` window.
///  * `wid` The ID with which you will refer to the new window, created by
///       `xcb_generate_id`.
///  * `parent` The parent window of the new window.
///  * `x` The X coordinate of the new window.
///  * `y` The Y coordinate of the new window.
///  * `width` The width of the new window.
///  * `height` The height of the new window.
///  * `border_width` TODO:
///
///       Must be zero if the `class` is `InputOnly` or a `xcb_match_error_t` occurs.
///  * `class`
///  * `visual` Specifies the id for the new window's visual.
///
///       The special value `XCB_COPY_FROM_PARENT` means the visual is taken from the
///       `parent` window.
///
/// Creates an unmapped window as child of the specified `parent` window. A
/// CreateNotify event will be generated. The new window is placed on top in the
/// stacking order with respect to siblings.
///
/// The coordinate system has the X axis horizontal and the Y axis vertical with
/// the origin [0, 0] at the upper-left corner. Coordinates are integral, in terms
/// of pixels, and coincide with pixel centers. Each window and pixmap has its own
/// coordinate system. For a window, the origin is inside the border at the inside,
/// upper-left corner.
///
/// The created window is not yet displayed (mapped), call `xcb_map_window` to
/// display it.
///
/// The created window will initially use the same cursor as its parent.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_create_window_checked (c : *mut ffi::base::xcb_connection_t,
                                     depth :  u8,
                                     wid :  xcb_window_t,
                                     parent :  xcb_window_t,
                                     x :  i16,
                                     y :  i16,
                                     width :  u16,
                                     height :  u16,
                                     border_width :  u16,
                                     class :  u16,
                                     visual :  xcb_visualid_t,
                                     value_mask :  u32,
                                     value_list : *mut u32) -> ffi::base::xcb_void_cookie_t;

/// Creates a window
///
/// # Arguments
///  * `depth` Specifies the new window's depth (TODO: what unit?).
///
///       The special value `XCB_COPY_FROM_PARENT` means the depth is taken from the
///       `parent` window.
///  * `wid` The ID with which you will refer to the new window, created by
///       `xcb_generate_id`.
///  * `parent` The parent window of the new window.
///  * `x` The X coordinate of the new window.
///  * `y` The Y coordinate of the new window.
///  * `width` The width of the new window.
///  * `height` The height of the new window.
///  * `border_width` TODO:
///
///       Must be zero if the `class` is `InputOnly` or a `xcb_match_error_t` occurs.
///  * `class`
///  * `visual` Specifies the id for the new window's visual.
///
///       The special value `XCB_COPY_FROM_PARENT` means the visual is taken from the
///       `parent` window.
///
/// Creates an unmapped window as child of the specified `parent` window. A
/// CreateNotify event will be generated. The new window is placed on top in the
/// stacking order with respect to siblings.
///
/// The coordinate system has the X axis horizontal and the Y axis vertical with
/// the origin [0, 0] at the upper-left corner. Coordinates are integral, in terms
/// of pixels, and coincide with pixel centers. Each window and pixmap has its own
/// coordinate system. For a window, the origin is inside the border at the inside,
/// upper-left corner.
///
/// The created window is not yet displayed (mapped), call `xcb_map_window` to
/// display it.
///
/// The created window will initially use the same cursor as its parent.
///
pub fn xcb_create_window (c : *mut ffi::base::xcb_connection_t,
                             depth :  u8,
                             wid :  xcb_window_t,
                             parent :  xcb_window_t,
                             x :  i16,
                             y :  i16,
                             width :  u16,
                             height :  u16,
                             border_width :  u16,
                             class :  u16,
                             visual :  xcb_visualid_t,
                             value_mask :  u32,
                             value_list : *mut u32) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_change_window_attributes_sizeof (_buffer :  *mut c_void) -> c_int;

/// change window attributes
///
/// # Arguments
///  * `window` The window to change.
///  * `value_mask`
///  * `value_list` Values for each of the attributes specified in the bitmask `value_mask`. The
///       order has to correspond to the order of possible `value_mask` bits. See the
///       example.
///
/// Changes the attributes specified by `value_mask` for the specified `window`.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_change_window_attributes_checked (c : *mut ffi::base::xcb_connection_t,
                                                window :  xcb_window_t,
                                                value_mask :  u32,
                                                value_list : *mut u32) -> ffi::base::xcb_void_cookie_t;

/// change window attributes
///
/// # Arguments
///  * `window` The window to change.
///  * `value_mask`
///  * `value_list` Values for each of the attributes specified in the bitmask `value_mask`. The
///       order has to correspond to the order of possible `value_mask` bits. See the
///       example.
///
/// Changes the attributes specified by `value_mask` for the specified `window`.
///
pub fn xcb_change_window_attributes (c : *mut ffi::base::xcb_connection_t,
                                        window :  xcb_window_t,
                                        value_mask :  u32,
                                        value_list : *mut u32) -> ffi::base::xcb_void_cookie_t;

/// Gets window attributes
///
/// # Arguments
///  * `window` The window to get the attributes from.
///
/// Gets the current attributes for the specified `window`.
///
pub fn xcb_get_window_attributes (c : *mut ffi::base::xcb_connection_t,
                                     window :  xcb_window_t) -> xcb_get_window_attributes_cookie_t;

/// Gets window attributes
///
/// # Arguments
///  * `window` The window to get the attributes from.
///
/// Gets the current attributes for the specified `window`.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_get_window_attributes_unchecked (c : *mut ffi::base::xcb_connection_t,
                                               window :  xcb_window_t) -> xcb_get_window_attributes_cookie_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_get_window_attributes_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_get_window_attributes_reply (c : *mut ffi::base::xcb_connection_t,
                                           cookie : xcb_get_window_attributes_cookie_t,
                                           e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_get_window_attributes_reply_t;

/// Destroys a window
///
/// # Arguments
///  * `window` The window to destroy.
///
/// Destroys the specified window and all of its subwindows. A DestroyNotify event
/// is generated for each destroyed window (a DestroyNotify event is first generated
/// for any given window's inferiors). If the window was mapped, it will be
/// automatically unmapped before destroying.
///
/// Calling DestroyWindow on the root window will do nothing.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_destroy_window_checked (c : *mut ffi::base::xcb_connection_t,
                                      window :  xcb_window_t) -> ffi::base::xcb_void_cookie_t;

/// Destroys a window
///
/// # Arguments
///  * `window` The window to destroy.
///
/// Destroys the specified window and all of its subwindows. A DestroyNotify event
/// is generated for each destroyed window (a DestroyNotify event is first generated
/// for any given window's inferiors). If the window was mapped, it will be
/// automatically unmapped before destroying.
///
/// Calling DestroyWindow on the root window will do nothing.
///
pub fn xcb_destroy_window (c : *mut ffi::base::xcb_connection_t,
                              window :  xcb_window_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_destroy_subwindows_checked (c : *mut ffi::base::xcb_connection_t,
                                          window :  xcb_window_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_destroy_subwindows (c : *mut ffi::base::xcb_connection_t,
                                  window :  xcb_window_t) -> ffi::base::xcb_void_cookie_t;

/// Changes a client's save set
///
/// # Arguments
///  * `mode` Insert to add the specified window to the save set or Delete to delete it from the save set.
///  * `window` The window to add or delete to/from your save set.
///
/// TODO: explain what the save set is for.
///
/// This function either adds or removes the specified window to the client's (your
/// application's) save set.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_change_save_set_checked (c : *mut ffi::base::xcb_connection_t,
                                       mode :  u8,
                                       window :  xcb_window_t) -> ffi::base::xcb_void_cookie_t;

/// Changes a client's save set
///
/// # Arguments
///  * `mode` Insert to add the specified window to the save set or Delete to delete it from the save set.
///  * `window` The window to add or delete to/from your save set.
///
/// TODO: explain what the save set is for.
///
/// This function either adds or removes the specified window to the client's (your
/// application's) save set.
///
pub fn xcb_change_save_set (c : *mut ffi::base::xcb_connection_t,
                               mode :  u8,
                               window :  xcb_window_t) -> ffi::base::xcb_void_cookie_t;

/// Reparents a window
///
/// # Arguments
///  * `window` The window to reparent.
///  * `parent` The new parent of the window.
///  * `x` The X position of the window within its new parent.
///  * `y` The Y position of the window within its new parent.
///
/// Makes the specified window a child of the specified parent window. If the
/// window is mapped, it will automatically be unmapped before reparenting and
/// re-mapped after reparenting. The window is placed in the stacking order on top
/// with respect to sibling windows.
///
/// After reparenting, a ReparentNotify event is generated.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_reparent_window_checked (c : *mut ffi::base::xcb_connection_t,
                                       window :  xcb_window_t,
                                       parent :  xcb_window_t,
                                       x :  i16,
                                       y :  i16) -> ffi::base::xcb_void_cookie_t;

/// Reparents a window
///
/// # Arguments
///  * `window` The window to reparent.
///  * `parent` The new parent of the window.
///  * `x` The X position of the window within its new parent.
///  * `y` The Y position of the window within its new parent.
///
/// Makes the specified window a child of the specified parent window. If the
/// window is mapped, it will automatically be unmapped before reparenting and
/// re-mapped after reparenting. The window is placed in the stacking order on top
/// with respect to sibling windows.
///
/// After reparenting, a ReparentNotify event is generated.
///
pub fn xcb_reparent_window (c : *mut ffi::base::xcb_connection_t,
                               window :  xcb_window_t,
                               parent :  xcb_window_t,
                               x :  i16,
                               y :  i16) -> ffi::base::xcb_void_cookie_t;

/// Makes a window visible
///
/// # Arguments
///  * `window` The window to make visible.
///
/// Maps the specified window. This means making the window visible (as long as its
/// parent is visible).
///
/// This MapWindow request will be translated to a MapRequest request if a window
/// manager is running. The window manager then decides to either map the window or
/// not. Set the override-redirect window attribute to true if you want to bypass
/// this mechanism.
///
/// If the window manager decides to map the window (or if no window manager is
/// running), a MapNotify event is generated.
///
/// If the window becomes viewable and no earlier contents for it are remembered,
/// the X server tiles the window with its background. If the window's background
/// is undefined, the existing screen contents are not altered, and the X server
/// generates zero or more Expose events.
///
/// If the window type is InputOutput, an Expose event will be generated when the
/// window becomes visible. The normal response to an Expose event should be to
/// repaint the window.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_map_window_checked (c : *mut ffi::base::xcb_connection_t,
                                  window :  xcb_window_t) -> ffi::base::xcb_void_cookie_t;

/// Makes a window visible
///
/// # Arguments
///  * `window` The window to make visible.
///
/// Maps the specified window. This means making the window visible (as long as its
/// parent is visible).
///
/// This MapWindow request will be translated to a MapRequest request if a window
/// manager is running. The window manager then decides to either map the window or
/// not. Set the override-redirect window attribute to true if you want to bypass
/// this mechanism.
///
/// If the window manager decides to map the window (or if no window manager is
/// running), a MapNotify event is generated.
///
/// If the window becomes viewable and no earlier contents for it are remembered,
/// the X server tiles the window with its background. If the window's background
/// is undefined, the existing screen contents are not altered, and the X server
/// generates zero or more Expose events.
///
/// If the window type is InputOutput, an Expose event will be generated when the
/// window becomes visible. The normal response to an Expose event should be to
/// repaint the window.
///
pub fn xcb_map_window (c : *mut ffi::base::xcb_connection_t,
                          window :  xcb_window_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_map_subwindows_checked (c : *mut ffi::base::xcb_connection_t,
                                      window :  xcb_window_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_map_subwindows (c : *mut ffi::base::xcb_connection_t,
                              window :  xcb_window_t) -> ffi::base::xcb_void_cookie_t;

/// Makes a window invisible
///
/// # Arguments
///  * `window` The window to make invisible.
///
/// Unmaps the specified window. This means making the window invisible (and all
/// its child windows).
///
/// Unmapping a window leads to the `UnmapNotify` event being generated. Also,
/// `Expose` events are generated for formerly obscured windows.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_unmap_window_checked (c : *mut ffi::base::xcb_connection_t,
                                    window :  xcb_window_t) -> ffi::base::xcb_void_cookie_t;

/// Makes a window invisible
///
/// # Arguments
///  * `window` The window to make invisible.
///
/// Unmaps the specified window. This means making the window invisible (and all
/// its child windows).
///
/// Unmapping a window leads to the `UnmapNotify` event being generated. Also,
/// `Expose` events are generated for formerly obscured windows.
///
pub fn xcb_unmap_window (c : *mut ffi::base::xcb_connection_t,
                            window :  xcb_window_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_unmap_subwindows_checked (c : *mut ffi::base::xcb_connection_t,
                                        window :  xcb_window_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_unmap_subwindows (c : *mut ffi::base::xcb_connection_t,
                                window :  xcb_window_t) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_configure_window_sizeof (_buffer :  *mut c_void) -> c_int;

/// Configures window attributes
///
/// # Arguments
///  * `window` The window to configure.
///  * `value_mask` Bitmask of attributes to change.
///  * `value_list` New values, corresponding to the attributes in value_mask. The order has to
///       correspond to the order of possible `value_mask` bits. See the example.
///
/// Configures a window's size, position, border width and stacking order.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_configure_window_checked (c : *mut ffi::base::xcb_connection_t,
                                        window :  xcb_window_t,
                                        value_mask :  u16,
                                        value_list : *mut u32) -> ffi::base::xcb_void_cookie_t;

/// Configures window attributes
///
/// # Arguments
///  * `window` The window to configure.
///  * `value_mask` Bitmask of attributes to change.
///  * `value_list` New values, corresponding to the attributes in value_mask. The order has to
///       correspond to the order of possible `value_mask` bits. See the example.
///
/// Configures a window's size, position, border width and stacking order.
///
pub fn xcb_configure_window (c : *mut ffi::base::xcb_connection_t,
                                window :  xcb_window_t,
                                value_mask :  u16,
                                value_list : *mut u32) -> ffi::base::xcb_void_cookie_t;

/// Change window stacking order
///
/// # Arguments
///  * `direction`
///  * `window` The window to raise/lower (depending on `direction`).
///
/// If `direction` is `XCB_CIRCULATE_RAISE_LOWEST`, the lowest mapped child (if
/// any) will be raised to the top of the stack.
///
/// If `direction` is `XCB_CIRCULATE_LOWER_HIGHEST`, the highest mapped child will
/// be lowered to the bottom of the stack.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_circulate_window_checked (c : *mut ffi::base::xcb_connection_t,
                                        direction :  u8,
                                        window :  xcb_window_t) -> ffi::base::xcb_void_cookie_t;

/// Change window stacking order
///
/// # Arguments
///  * `direction`
///  * `window` The window to raise/lower (depending on `direction`).
///
/// If `direction` is `XCB_CIRCULATE_RAISE_LOWEST`, the lowest mapped child (if
/// any) will be raised to the top of the stack.
///
/// If `direction` is `XCB_CIRCULATE_LOWER_HIGHEST`, the highest mapped child will
/// be lowered to the bottom of the stack.
///
pub fn xcb_circulate_window (c : *mut ffi::base::xcb_connection_t,
                                direction :  u8,
                                window :  xcb_window_t) -> ffi::base::xcb_void_cookie_t;

/// Get current window geometry
///
/// # Arguments
///  * `drawable` The drawable (`Window` or `Pixmap`) of which the geometry will be received.
///
/// Gets the current geometry of the specified drawable (either `Window` or `Pixmap`).
///
pub fn xcb_get_geometry (c : *mut ffi::base::xcb_connection_t,
                            drawable :  xcb_drawable_t) -> xcb_get_geometry_cookie_t;

/// Get current window geometry
///
/// # Arguments
///  * `drawable` The drawable (`Window` or `Pixmap`) of which the geometry will be received.
///
/// Gets the current geometry of the specified drawable (either `Window` or `Pixmap`).
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_get_geometry_unchecked (c : *mut ffi::base::xcb_connection_t,
                                      drawable :  xcb_drawable_t) -> xcb_get_geometry_cookie_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_get_geometry_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_get_geometry_reply (c : *mut ffi::base::xcb_connection_t,
                                  cookie : xcb_get_geometry_cookie_t,
                                  e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_get_geometry_reply_t;

pub fn xcb_query_tree_sizeof (_buffer :  *mut c_void) -> c_int;

/// query the window tree
///
/// # Arguments
///  * `window` The `window` to query.
///
/// Gets the root window ID, parent window ID and list of children windows for the
/// specified `window`. The children are listed in bottom-to-top stacking order.
///
pub fn xcb_query_tree (c : *mut ffi::base::xcb_connection_t,
                          window :  xcb_window_t) -> xcb_query_tree_cookie_t;

/// query the window tree
///
/// # Arguments
///  * `window` The `window` to query.
///
/// Gets the root window ID, parent window ID and list of children windows for the
/// specified `window`. The children are listed in bottom-to-top stacking order.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_query_tree_unchecked (c : *mut ffi::base::xcb_connection_t,
                                    window :  xcb_window_t) -> xcb_query_tree_cookie_t;

pub fn xcb_query_tree_children (R : *mut xcb_query_tree_reply_t) -> *mut xcb_window_t;


pub fn xcb_query_tree_children_length (R : *mut xcb_query_tree_reply_t) -> c_int;


pub fn xcb_query_tree_children_end (R : *mut xcb_query_tree_reply_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_query_tree_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_query_tree_reply (c : *mut ffi::base::xcb_connection_t,
                                cookie : xcb_query_tree_cookie_t,
                                e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_query_tree_reply_t;

pub fn xcb_intern_atom_sizeof (_buffer :  *mut c_void) -> c_int;

/// Get atom identifier by name
///
/// # Arguments
///  * `only_if_exists` Return a valid atom id only if the atom already exists.
///  * `name_len` The length of the following `name`.
///  * `name` The name of the atom.
///
/// Retrieves the identifier (xcb_atom_t TODO) for the atom with the specified
/// name. Atoms are used in protocols like EWMH, for example to store window titles
/// (`_NET_WM_NAME` atom) as property of a window.
///
/// If `only_if_exists` is 0, the atom will be created if it does not already exist.
/// If `only_if_exists` is 1, `XCB_ATOM_NONE` will be returned if the atom does
/// not yet exist.
///
pub fn xcb_intern_atom (c : *mut ffi::base::xcb_connection_t,
                           only_if_exists :  u8,
                           name_len :  u16,
                           name : *mut c_char) -> xcb_intern_atom_cookie_t;

/// Get atom identifier by name
///
/// # Arguments
///  * `only_if_exists` Return a valid atom id only if the atom already exists.
///  * `name_len` The length of the following `name`.
///  * `name` The name of the atom.
///
/// Retrieves the identifier (xcb_atom_t TODO) for the atom with the specified
/// name. Atoms are used in protocols like EWMH, for example to store window titles
/// (`_NET_WM_NAME` atom) as property of a window.
///
/// If `only_if_exists` is 0, the atom will be created if it does not already exist.
/// If `only_if_exists` is 1, `XCB_ATOM_NONE` will be returned if the atom does
/// not yet exist.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_intern_atom_unchecked (c : *mut ffi::base::xcb_connection_t,
                                     only_if_exists :  u8,
                                     name_len :  u16,
                                     name : *mut c_char) -> xcb_intern_atom_cookie_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_intern_atom_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_intern_atom_reply (c : *mut ffi::base::xcb_connection_t,
                                 cookie : xcb_intern_atom_cookie_t,
                                 e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_intern_atom_reply_t;

pub fn xcb_get_atom_name_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
///
pub fn xcb_get_atom_name (c : *mut ffi::base::xcb_connection_t,
                             atom :  xcb_atom_t) -> xcb_get_atom_name_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_get_atom_name_unchecked (c : *mut ffi::base::xcb_connection_t,
                                       atom :  xcb_atom_t) -> xcb_get_atom_name_cookie_t;

pub fn xcb_get_atom_name_name (R : *mut xcb_get_atom_name_reply_t) -> *mut c_char;


pub fn xcb_get_atom_name_name_length (R : *mut xcb_get_atom_name_reply_t) -> c_int;


pub fn xcb_get_atom_name_name_end (R : *mut xcb_get_atom_name_reply_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_get_atom_name_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_get_atom_name_reply (c : *mut ffi::base::xcb_connection_t,
                                   cookie : xcb_get_atom_name_cookie_t,
                                   e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_get_atom_name_reply_t;

pub fn xcb_change_property_sizeof (_buffer :  *mut c_void) -> c_int;

/// Changes a window property
///
/// # Arguments
///  * `mode`
///  * `window` The window whose property you want to change.
///  * `property` The property you want to change (an atom).
///  * `type_` The type of the property you want to change (an atom).
///  * `format` Specifies whether the data should be viewed as a list of 8-bit, 16-bit or
///       32-bit quantities. Possible values are 8, 16 and 32. This information allows
///       the X server to correctly perform byte-swap operations as necessary.
///  * `data_len` Specifies the number of elements (see `format`).
///  * `data` The property data.
///
/// Sets or updates a property on the specified `window`. Properties are for
/// example the window title (`WM_NAME`) or its minimum size (`WM_NORMAL_HINTS`).
/// Protocols such as EWMH also use properties - for example EWMH defines the
/// window title, encoded as UTF-8 string, in the `_NET_WM_NAME` property.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_change_property_checked (c : *mut ffi::base::xcb_connection_t,
                                       mode :  u8,
                                       window :  xcb_window_t,
                                       property :  xcb_atom_t,
                                       type_ :  xcb_atom_t,
                                       format :  u8,
                                       data_len :  u32,
                                       data : *mut c_void) -> ffi::base::xcb_void_cookie_t;

/// Changes a window property
///
/// # Arguments
///  * `mode`
///  * `window` The window whose property you want to change.
///  * `property` The property you want to change (an atom).
///  * `type_` The type of the property you want to change (an atom).
///  * `format` Specifies whether the data should be viewed as a list of 8-bit, 16-bit or
///       32-bit quantities. Possible values are 8, 16 and 32. This information allows
///       the X server to correctly perform byte-swap operations as necessary.
///  * `data_len` Specifies the number of elements (see `format`).
///  * `data` The property data.
///
/// Sets or updates a property on the specified `window`. Properties are for
/// example the window title (`WM_NAME`) or its minimum size (`WM_NORMAL_HINTS`).
/// Protocols such as EWMH also use properties - for example EWMH defines the
/// window title, encoded as UTF-8 string, in the `_NET_WM_NAME` property.
///
pub fn xcb_change_property (c : *mut ffi::base::xcb_connection_t,
                               mode :  u8,
                               window :  xcb_window_t,
                               property :  xcb_atom_t,
                               type_ :  xcb_atom_t,
                               format :  u8,
                               data_len :  u32,
                               data : *mut c_void) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_delete_property_checked (c : *mut ffi::base::xcb_connection_t,
                                       window :  xcb_window_t,
                                       property :  xcb_atom_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_delete_property (c : *mut ffi::base::xcb_connection_t,
                               window :  xcb_window_t,
                               property :  xcb_atom_t) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_get_property_sizeof (_buffer :  *mut c_void) -> c_int;

/// Gets a window property
///
/// # Arguments
///  * `delete` Whether the property should actually be deleted. For deleting a property, the
///       specified `type` has to match the actual property type.
///  * `window` The window whose property you want to get.
///  * `property` The property you want to get (an atom).
///  * `type_` The type of the property you want to get (an atom).
///  * `long_offset` Specifies the offset (in 32-bit multiples) in the specified property where the
///       data is to be retrieved.
///  * `long_length` Specifies how many 32-bit multiples of data should be retrieved (e.g. if you
///       set `long_length` to 4, you will receive 16 bytes of data).
///
/// Gets the specified `property` from the specified `window`. Properties are for
/// example the window title (`WM_NAME`) or its minimum size (`WM_NORMAL_HINTS`).
/// Protocols such as EWMH also use properties - for example EWMH defines the
/// window title, encoded as UTF-8 string, in the `_NET_WM_NAME` property.
///
/// TODO: talk about `type`
///
/// TODO: talk about `delete`
///
/// TODO: talk about the offset/length thing. what's a valid use case?
///
pub fn xcb_get_property (c : *mut ffi::base::xcb_connection_t,
                            delete :  u8,
                            window :  xcb_window_t,
                            property :  xcb_atom_t,
                            type_ :  xcb_atom_t,
                            long_offset :  u32,
                            long_length :  u32) -> xcb_get_property_cookie_t;

/// Gets a window property
///
/// # Arguments
///  * `delete` Whether the property should actually be deleted. For deleting a property, the
///       specified `type` has to match the actual property type.
///  * `window` The window whose property you want to get.
///  * `property` The property you want to get (an atom).
///  * `type_` The type of the property you want to get (an atom).
///  * `long_offset` Specifies the offset (in 32-bit multiples) in the specified property where the
///       data is to be retrieved.
///  * `long_length` Specifies how many 32-bit multiples of data should be retrieved (e.g. if you
///       set `long_length` to 4, you will receive 16 bytes of data).
///
/// Gets the specified `property` from the specified `window`. Properties are for
/// example the window title (`WM_NAME`) or its minimum size (`WM_NORMAL_HINTS`).
/// Protocols such as EWMH also use properties - for example EWMH defines the
/// window title, encoded as UTF-8 string, in the `_NET_WM_NAME` property.
///
/// TODO: talk about `type`
///
/// TODO: talk about `delete`
///
/// TODO: talk about the offset/length thing. what's a valid use case?
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_get_property_unchecked (c : *mut ffi::base::xcb_connection_t,
                                      delete :  u8,
                                      window :  xcb_window_t,
                                      property :  xcb_atom_t,
                                      type_ :  xcb_atom_t,
                                      long_offset :  u32,
                                      long_length :  u32) -> xcb_get_property_cookie_t;

pub fn xcb_get_property_value (R : *mut xcb_get_property_reply_t) -> *mut c_void;


pub fn xcb_get_property_value_length (R : *mut xcb_get_property_reply_t) -> c_int;


pub fn xcb_get_property_value_end (R : *mut xcb_get_property_reply_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_get_property_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_get_property_reply (c : *mut ffi::base::xcb_connection_t,
                                  cookie : xcb_get_property_cookie_t,
                                  e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_get_property_reply_t;

pub fn xcb_list_properties_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
///
pub fn xcb_list_properties (c : *mut ffi::base::xcb_connection_t,
                               window :  xcb_window_t) -> xcb_list_properties_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_list_properties_unchecked (c : *mut ffi::base::xcb_connection_t,
                                         window :  xcb_window_t) -> xcb_list_properties_cookie_t;

pub fn xcb_list_properties_atoms (R : *mut xcb_list_properties_reply_t) -> *mut xcb_atom_t;


pub fn xcb_list_properties_atoms_length (R : *mut xcb_list_properties_reply_t) -> c_int;


pub fn xcb_list_properties_atoms_end (R : *mut xcb_list_properties_reply_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_list_properties_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_list_properties_reply (c : *mut ffi::base::xcb_connection_t,
                                     cookie : xcb_list_properties_cookie_t,
                                     e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_list_properties_reply_t;

/// Sets the owner of a selection
///
/// # Arguments
///  * `owner` The new owner of the selection.
///
///       The special value `XCB_NONE` means that the selection will have no owner.
///  * `selection` The selection.
///  * `time` Timestamp to avoid race conditions when running X over the network.
///
///       The selection will not be changed if `time` is earlier than the current
///       last-change time of the `selection` or is later than the current X server time.
///       Otherwise, the last-change time is set to the specified time.
///
///       The special value `XCB_CURRENT_TIME` will be replaced with the current server
///       time.
///
/// Makes `window` the owner of the selection `selection` and updates the
/// last-change time of the specified selection.
///
/// TODO: briefly explain what a selection is.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_set_selection_owner_checked (c : *mut ffi::base::xcb_connection_t,
                                           owner :  xcb_window_t,
                                           selection :  xcb_atom_t,
                                           time :  xcb_timestamp_t) -> ffi::base::xcb_void_cookie_t;

/// Sets the owner of a selection
///
/// # Arguments
///  * `owner` The new owner of the selection.
///
///       The special value `XCB_NONE` means that the selection will have no owner.
///  * `selection` The selection.
///  * `time` Timestamp to avoid race conditions when running X over the network.
///
///       The selection will not be changed if `time` is earlier than the current
///       last-change time of the `selection` or is later than the current X server time.
///       Otherwise, the last-change time is set to the specified time.
///
///       The special value `XCB_CURRENT_TIME` will be replaced with the current server
///       time.
///
/// Makes `window` the owner of the selection `selection` and updates the
/// last-change time of the specified selection.
///
/// TODO: briefly explain what a selection is.
///
pub fn xcb_set_selection_owner (c : *mut ffi::base::xcb_connection_t,
                                   owner :  xcb_window_t,
                                   selection :  xcb_atom_t,
                                   time :  xcb_timestamp_t) -> ffi::base::xcb_void_cookie_t;

/// Gets the owner of a selection
///
/// # Arguments
///  * `selection` The selection.
///
/// Gets the owner of the specified selection.
///
/// TODO: briefly explain what a selection is.
///
pub fn xcb_get_selection_owner (c : *mut ffi::base::xcb_connection_t,
                                   selection :  xcb_atom_t) -> xcb_get_selection_owner_cookie_t;

/// Gets the owner of a selection
///
/// # Arguments
///  * `selection` The selection.
///
/// Gets the owner of the specified selection.
///
/// TODO: briefly explain what a selection is.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_get_selection_owner_unchecked (c : *mut ffi::base::xcb_connection_t,
                                             selection :  xcb_atom_t) -> xcb_get_selection_owner_cookie_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_get_selection_owner_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_get_selection_owner_reply (c : *mut ffi::base::xcb_connection_t,
                                         cookie : xcb_get_selection_owner_cookie_t,
                                         e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_get_selection_owner_reply_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_convert_selection_checked (c : *mut ffi::base::xcb_connection_t,
                                         requestor :  xcb_window_t,
                                         selection :  xcb_atom_t,
                                         target :  xcb_atom_t,
                                         property :  xcb_atom_t,
                                         time :  xcb_timestamp_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_convert_selection (c : *mut ffi::base::xcb_connection_t,
                                 requestor :  xcb_window_t,
                                 selection :  xcb_atom_t,
                                 target :  xcb_atom_t,
                                 property :  xcb_atom_t,
                                 time :  xcb_timestamp_t) -> ffi::base::xcb_void_cookie_t;

/// send an event
///
/// # Arguments
///  * `propagate` If `propagate` is true and no clients have selected any event on `destination`,
///       the destination is replaced with the closest ancestor of `destination` for
///       which some client has selected a type in `event_mask` and for which no
///       intervening window has that type in its do-not-propagate-mask. If no such
///       window exists or if the window is an ancestor of the focus window and
///       `InputFocus` was originally specified as the destination, the event is not sent
///       to any clients. Otherwise, the event is reported to every client selecting on
///       the final destination any of the types specified in `event_mask`.
///  * `destination` The window to send this event to. Every client which selects any event within
///       `event_mask` on `destination` will get the event.
///
///       The special value `XCB_SEND_EVENT_DEST_POINTER_WINDOW` refers to the window
///       that contains the mouse pointer.
///
///       The special value `XCB_SEND_EVENT_DEST_ITEM_FOCUS` refers to the window which
///       has the keyboard focus.
///  * `event_mask` Event_mask for determining which clients should receive the specified event.
///       See `destination` and `propagate`.
///  * `event` The event to send to the specified `destination`.
///
/// Identifies the `destination` window, determines which clients should receive
/// the specified event and ignores any active grabs.
///
/// The `event` must be one of the core events or an event defined by an extension,
/// so that the X server can correctly byte-swap the contents as necessary. The
/// contents of `event` are otherwise unaltered and unchecked except for the
/// `send_event` field which is forced to 'true'.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_send_event_checked (c : *mut ffi::base::xcb_connection_t,
                                  propagate :  u8,
                                  destination :  xcb_window_t,
                                  event_mask :  u32,
                                  event : *mut c_char) -> ffi::base::xcb_void_cookie_t;

/// send an event
///
/// # Arguments
///  * `propagate` If `propagate` is true and no clients have selected any event on `destination`,
///       the destination is replaced with the closest ancestor of `destination` for
///       which some client has selected a type in `event_mask` and for which no
///       intervening window has that type in its do-not-propagate-mask. If no such
///       window exists or if the window is an ancestor of the focus window and
///       `InputFocus` was originally specified as the destination, the event is not sent
///       to any clients. Otherwise, the event is reported to every client selecting on
///       the final destination any of the types specified in `event_mask`.
///  * `destination` The window to send this event to. Every client which selects any event within
///       `event_mask` on `destination` will get the event.
///
///       The special value `XCB_SEND_EVENT_DEST_POINTER_WINDOW` refers to the window
///       that contains the mouse pointer.
///
///       The special value `XCB_SEND_EVENT_DEST_ITEM_FOCUS` refers to the window which
///       has the keyboard focus.
///  * `event_mask` Event_mask for determining which clients should receive the specified event.
///       See `destination` and `propagate`.
///  * `event` The event to send to the specified `destination`.
///
/// Identifies the `destination` window, determines which clients should receive
/// the specified event and ignores any active grabs.
///
/// The `event` must be one of the core events or an event defined by an extension,
/// so that the X server can correctly byte-swap the contents as necessary. The
/// contents of `event` are otherwise unaltered and unchecked except for the
/// `send_event` field which is forced to 'true'.
///
pub fn xcb_send_event (c : *mut ffi::base::xcb_connection_t,
                          propagate :  u8,
                          destination :  xcb_window_t,
                          event_mask :  u32,
                          event : *mut c_char) -> ffi::base::xcb_void_cookie_t;

/// Grab the pointer
///
/// # Arguments
///  * `owner_events` If 1, the `grab_window` will still get the pointer events. If 0, events are not
///       reported to the `grab_window`.
///  * `grab_window` Specifies the window on which the pointer should be grabbed.
///  * `event_mask` Specifies which pointer events are reported to the client.
///
///       TODO: which values?
///  * `pointer_mode`
///  * `keyboard_mode`
///  * `confine_to` Specifies the window to confine the pointer in (the user will not be able to
///       move the pointer out of that window).
///
///       The special value `XCB_NONE` means don't confine the pointer.
///  * `cursor` Specifies the cursor that should be displayed or `XCB_NONE` to not change the
///       cursor.
///  * `time` The time argument allows you to avoid certain circumstances that come up if
///       applications take a long time to respond or if there are long network delays.
///       Consider a situation where you have two applications, both of which normally
///       grab the pointer when clicked on. If both applications specify the timestamp
///       from the event, the second application may wake up faster and successfully grab
///       the pointer before the first application. The first application then will get
///       an indication that the other application grabbed the pointer before its request
///       was processed.
///
///       The special value `XCB_CURRENT_TIME` will be replaced with the current server
///       time.
///
/// Actively grabs control of the pointer. Further pointer events are reported only to the grabbing client. Overrides any active pointer grab by this client.
///
pub fn xcb_grab_pointer (c : *mut ffi::base::xcb_connection_t,
                            owner_events :  u8,
                            grab_window :  xcb_window_t,
                            event_mask :  u16,
                            pointer_mode :  u8,
                            keyboard_mode :  u8,
                            confine_to :  xcb_window_t,
                            cursor :  xcb_cursor_t,
                            time :  xcb_timestamp_t) -> xcb_grab_pointer_cookie_t;

/// Grab the pointer
///
/// # Arguments
///  * `owner_events` If 1, the `grab_window` will still get the pointer events. If 0, events are not
///       reported to the `grab_window`.
///  * `grab_window` Specifies the window on which the pointer should be grabbed.
///  * `event_mask` Specifies which pointer events are reported to the client.
///
///       TODO: which values?
///  * `pointer_mode`
///  * `keyboard_mode`
///  * `confine_to` Specifies the window to confine the pointer in (the user will not be able to
///       move the pointer out of that window).
///
///       The special value `XCB_NONE` means don't confine the pointer.
///  * `cursor` Specifies the cursor that should be displayed or `XCB_NONE` to not change the
///       cursor.
///  * `time` The time argument allows you to avoid certain circumstances that come up if
///       applications take a long time to respond or if there are long network delays.
///       Consider a situation where you have two applications, both of which normally
///       grab the pointer when clicked on. If both applications specify the timestamp
///       from the event, the second application may wake up faster and successfully grab
///       the pointer before the first application. The first application then will get
///       an indication that the other application grabbed the pointer before its request
///       was processed.
///
///       The special value `XCB_CURRENT_TIME` will be replaced with the current server
///       time.
///
/// Actively grabs control of the pointer. Further pointer events are reported only to the grabbing client. Overrides any active pointer grab by this client.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_grab_pointer_unchecked (c : *mut ffi::base::xcb_connection_t,
                                      owner_events :  u8,
                                      grab_window :  xcb_window_t,
                                      event_mask :  u16,
                                      pointer_mode :  u8,
                                      keyboard_mode :  u8,
                                      confine_to :  xcb_window_t,
                                      cursor :  xcb_cursor_t,
                                      time :  xcb_timestamp_t) -> xcb_grab_pointer_cookie_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_grab_pointer_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_grab_pointer_reply (c : *mut ffi::base::xcb_connection_t,
                                  cookie : xcb_grab_pointer_cookie_t,
                                  e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_grab_pointer_reply_t;

/// release the pointer
///
/// # Arguments
///  * `time` Timestamp to avoid race conditions when running X over the network.
///
///       The pointer will not be released if `time` is earlier than the
///       last-pointer-grab time or later than the current X server time.
///
/// Releases the pointer and any queued events if you actively grabbed the pointer
/// before using `xcb_grab_pointer`, `xcb_grab_button` or within a normal button
/// press.
///
/// EnterNotify and LeaveNotify events are generated.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_ungrab_pointer_checked (c : *mut ffi::base::xcb_connection_t,
                                      time :  xcb_timestamp_t) -> ffi::base::xcb_void_cookie_t;

/// release the pointer
///
/// # Arguments
///  * `time` Timestamp to avoid race conditions when running X over the network.
///
///       The pointer will not be released if `time` is earlier than the
///       last-pointer-grab time or later than the current X server time.
///
/// Releases the pointer and any queued events if you actively grabbed the pointer
/// before using `xcb_grab_pointer`, `xcb_grab_button` or within a normal button
/// press.
///
/// EnterNotify and LeaveNotify events are generated.
///
pub fn xcb_ungrab_pointer (c : *mut ffi::base::xcb_connection_t,
                              time :  xcb_timestamp_t) -> ffi::base::xcb_void_cookie_t;

/// Grab pointer button(s)
///
/// # Arguments
///  * `owner_events` If 1, the `grab_window` will still get the pointer events. If 0, events are not
///       reported to the `grab_window`.
///  * `grab_window` Specifies the window on which the pointer should be grabbed.
///  * `event_mask` Specifies which pointer events are reported to the client.
///
///       TODO: which values?
///  * `pointer_mode`
///  * `keyboard_mode`
///  * `confine_to` Specifies the window to confine the pointer in (the user will not be able to
///       move the pointer out of that window).
///
///       The special value `XCB_NONE` means don't confine the pointer.
///  * `cursor` Specifies the cursor that should be displayed or `XCB_NONE` to not change the
///       cursor.
///  * `button`
///  * `modifiers` The modifiers to grab.
///
///       Using the special value `XCB_MOD_MASK_ANY` means grab the pointer with all
///       possible modifier combinations.
///
/// This request establishes a passive grab. The pointer is actively grabbed as
/// described in GrabPointer, the last-pointer-grab time is set to the time at
/// which the button was pressed (as transmitted in the ButtonPress event), and the
/// ButtonPress event is reported if all of the following conditions are true:
///
/// The pointer is not grabbed and the specified button is logically pressed when
/// the specified modifier keys are logically down, and no other buttons or
/// modifier keys are logically down.
///
/// The grab-window contains the pointer.
///
/// The confine-to window (if any) is viewable.
///
/// A passive grab on the same button/key combination does not exist on any
/// ancestor of grab-window.
///
/// The interpretation of the remaining arguments is the same as for GrabPointer.
/// The active grab is terminated automatically when the logical state of the
/// pointer has all buttons released, independent of the logical state of modifier
/// keys. Note that the logical state of a device (as seen by means of the
/// protocol) may lag the physical state if device event processing is frozen. This
/// request overrides all previous passive grabs by the same client on the same
/// button/key combinations on the same window. A modifier of AnyModifier is
/// equivalent to issuing the request for all possible modifier combinations
/// (including the combination of no modifiers). It is not required that all
/// specified modifiers have currently assigned keycodes. A button of AnyButton is
/// equivalent to issuing the request for all possible buttons. Otherwise, it is
/// not required that the button specified currently be assigned to a physical
/// button.
///
/// An Access error is generated if some other client has already issued a
/// GrabButton request with the same button/key combination on the same window.
/// When using AnyModifier or AnyButton, the request fails completely (no grabs are
/// established), and an Access error is generated if there is a conflicting grab
/// for any combination. The request has no effect on an active grab.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_grab_button_checked (c : *mut ffi::base::xcb_connection_t,
                                   owner_events :  u8,
                                   grab_window :  xcb_window_t,
                                   event_mask :  u16,
                                   pointer_mode :  u8,
                                   keyboard_mode :  u8,
                                   confine_to :  xcb_window_t,
                                   cursor :  xcb_cursor_t,
                                   button :  u8,
                                   modifiers :  u16) -> ffi::base::xcb_void_cookie_t;

/// Grab pointer button(s)
///
/// # Arguments
///  * `owner_events` If 1, the `grab_window` will still get the pointer events. If 0, events are not
///       reported to the `grab_window`.
///  * `grab_window` Specifies the window on which the pointer should be grabbed.
///  * `event_mask` Specifies which pointer events are reported to the client.
///
///       TODO: which values?
///  * `pointer_mode`
///  * `keyboard_mode`
///  * `confine_to` Specifies the window to confine the pointer in (the user will not be able to
///       move the pointer out of that window).
///
///       The special value `XCB_NONE` means don't confine the pointer.
///  * `cursor` Specifies the cursor that should be displayed or `XCB_NONE` to not change the
///       cursor.
///  * `button`
///  * `modifiers` The modifiers to grab.
///
///       Using the special value `XCB_MOD_MASK_ANY` means grab the pointer with all
///       possible modifier combinations.
///
/// This request establishes a passive grab. The pointer is actively grabbed as
/// described in GrabPointer, the last-pointer-grab time is set to the time at
/// which the button was pressed (as transmitted in the ButtonPress event), and the
/// ButtonPress event is reported if all of the following conditions are true:
///
/// The pointer is not grabbed and the specified button is logically pressed when
/// the specified modifier keys are logically down, and no other buttons or
/// modifier keys are logically down.
///
/// The grab-window contains the pointer.
///
/// The confine-to window (if any) is viewable.
///
/// A passive grab on the same button/key combination does not exist on any
/// ancestor of grab-window.
///
/// The interpretation of the remaining arguments is the same as for GrabPointer.
/// The active grab is terminated automatically when the logical state of the
/// pointer has all buttons released, independent of the logical state of modifier
/// keys. Note that the logical state of a device (as seen by means of the
/// protocol) may lag the physical state if device event processing is frozen. This
/// request overrides all previous passive grabs by the same client on the same
/// button/key combinations on the same window. A modifier of AnyModifier is
/// equivalent to issuing the request for all possible modifier combinations
/// (including the combination of no modifiers). It is not required that all
/// specified modifiers have currently assigned keycodes. A button of AnyButton is
/// equivalent to issuing the request for all possible buttons. Otherwise, it is
/// not required that the button specified currently be assigned to a physical
/// button.
///
/// An Access error is generated if some other client has already issued a
/// GrabButton request with the same button/key combination on the same window.
/// When using AnyModifier or AnyButton, the request fails completely (no grabs are
/// established), and an Access error is generated if there is a conflicting grab
/// for any combination. The request has no effect on an active grab.
///
pub fn xcb_grab_button (c : *mut ffi::base::xcb_connection_t,
                           owner_events :  u8,
                           grab_window :  xcb_window_t,
                           event_mask :  u16,
                           pointer_mode :  u8,
                           keyboard_mode :  u8,
                           confine_to :  xcb_window_t,
                           cursor :  xcb_cursor_t,
                           button :  u8,
                           modifiers :  u16) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_ungrab_button_checked (c : *mut ffi::base::xcb_connection_t,
                                     button :  u8,
                                     grab_window :  xcb_window_t,
                                     modifiers :  u16) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_ungrab_button (c : *mut ffi::base::xcb_connection_t,
                             button :  u8,
                             grab_window :  xcb_window_t,
                             modifiers :  u16) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_change_active_pointer_grab_checked (c : *mut ffi::base::xcb_connection_t,
                                                  cursor :  xcb_cursor_t,
                                                  time :  xcb_timestamp_t,
                                                  event_mask :  u16) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_change_active_pointer_grab (c : *mut ffi::base::xcb_connection_t,
                                          cursor :  xcb_cursor_t,
                                          time :  xcb_timestamp_t,
                                          event_mask :  u16) -> ffi::base::xcb_void_cookie_t;

/// Grab the keyboard
///
/// # Arguments
///  * `owner_events` If 1, the `grab_window` will still get the pointer events. If 0, events are not
///       reported to the `grab_window`.
///  * `grab_window` Specifies the window on which the pointer should be grabbed.
///  * `time` Timestamp to avoid race conditions when running X over the network.
///
///       The special value `XCB_CURRENT_TIME` will be replaced with the current server
///       time.
///  * `pointer_mode`
///  * `keyboard_mode`
///
/// Actively grabs control of the keyboard and generates FocusIn and FocusOut
/// events. Further key events are reported only to the grabbing client.
///
/// Any active keyboard grab by this client is overridden. If the keyboard is
/// actively grabbed by some other client, `AlreadyGrabbed` is returned. If
/// `grab_window` is not viewable, `GrabNotViewable` is returned. If the keyboard
/// is frozen by an active grab of another client, `GrabFrozen` is returned. If the
/// specified `time` is earlier than the last-keyboard-grab time or later than the
/// current X server time, `GrabInvalidTime` is returned. Otherwise, the
/// last-keyboard-grab time is set to the specified time.
///
pub fn xcb_grab_keyboard (c : *mut ffi::base::xcb_connection_t,
                             owner_events :  u8,
                             grab_window :  xcb_window_t,
                             time :  xcb_timestamp_t,
                             pointer_mode :  u8,
                             keyboard_mode :  u8) -> xcb_grab_keyboard_cookie_t;

/// Grab the keyboard
///
/// # Arguments
///  * `owner_events` If 1, the `grab_window` will still get the pointer events. If 0, events are not
///       reported to the `grab_window`.
///  * `grab_window` Specifies the window on which the pointer should be grabbed.
///  * `time` Timestamp to avoid race conditions when running X over the network.
///
///       The special value `XCB_CURRENT_TIME` will be replaced with the current server
///       time.
///  * `pointer_mode`
///  * `keyboard_mode`
///
/// Actively grabs control of the keyboard and generates FocusIn and FocusOut
/// events. Further key events are reported only to the grabbing client.
///
/// Any active keyboard grab by this client is overridden. If the keyboard is
/// actively grabbed by some other client, `AlreadyGrabbed` is returned. If
/// `grab_window` is not viewable, `GrabNotViewable` is returned. If the keyboard
/// is frozen by an active grab of another client, `GrabFrozen` is returned. If the
/// specified `time` is earlier than the last-keyboard-grab time or later than the
/// current X server time, `GrabInvalidTime` is returned. Otherwise, the
/// last-keyboard-grab time is set to the specified time.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_grab_keyboard_unchecked (c : *mut ffi::base::xcb_connection_t,
                                       owner_events :  u8,
                                       grab_window :  xcb_window_t,
                                       time :  xcb_timestamp_t,
                                       pointer_mode :  u8,
                                       keyboard_mode :  u8) -> xcb_grab_keyboard_cookie_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_grab_keyboard_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_grab_keyboard_reply (c : *mut ffi::base::xcb_connection_t,
                                   cookie : xcb_grab_keyboard_cookie_t,
                                   e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_grab_keyboard_reply_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_ungrab_keyboard_checked (c : *mut ffi::base::xcb_connection_t,
                                       time :  xcb_timestamp_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_ungrab_keyboard (c : *mut ffi::base::xcb_connection_t,
                               time :  xcb_timestamp_t) -> ffi::base::xcb_void_cookie_t;

/// Grab keyboard key(s)
///
/// # Arguments
///  * `owner_events` If 1, the `grab_window` will still get the pointer events. If 0, events are not
///       reported to the `grab_window`.
///  * `grab_window` Specifies the window on which the pointer should be grabbed.
///  * `modifiers` The modifiers to grab.
///
///       Using the special value `XCB_MOD_MASK_ANY` means grab the pointer with all
///       possible modifier combinations.
///  * `key` The keycode of the key to grab.
///
///       The special value `XCB_GRAB_ANY` means grab any key.
///  * `pointer_mode`
///  * `keyboard_mode`
///
/// Establishes a passive grab on the keyboard. In the future, the keyboard is
/// actively grabbed (as for `GrabKeyboard`), the last-keyboard-grab time is set to
/// the time at which the key was pressed (as transmitted in the KeyPress event),
/// and the KeyPress event is reported if all of the following conditions are true:
///
/// The keyboard is not grabbed and the specified key (which can itself be a
/// modifier key) is logically pressed when the specified modifier keys are
/// logically down, and no other modifier keys are logically down.
///
/// Either the grab_window is an ancestor of (or is) the focus window, or the
/// grab_window is a descendant of the focus window and contains the pointer.
///
/// A passive grab on the same key combination does not exist on any ancestor of
/// grab_window.
///
/// The interpretation of the remaining arguments is as for XGrabKeyboard.  The active grab is terminated
/// automatically when the logical state of the keyboard has the specified key released (independent of the
/// logical state of the modifier keys), at which point a KeyRelease event is reported to the grabbing window.
///
/// Note that the logical state of a device (as seen by client applications) may lag the physical state if
/// device event processing is frozen.
///
/// A modifiers argument of AnyModifier is equivalent to issuing the request for all possible modifier combinations (including the combination of no modifiers).  It is not required that all modifiers specified
/// have currently assigned KeyCodes.  A keycode argument of AnyKey is equivalent to issuing the request for
/// all possible KeyCodes.  Otherwise, the specified keycode must be in the range specified by min_keycode
/// and max_keycode in the connection setup, or a BadValue error results.
///
/// If some other client has issued a XGrabKey with the same key combination on the same window, a BadAccess
/// error results.  When using AnyModifier or AnyKey, the request fails completely, and a BadAccess error
/// results (no grabs are established) if there is a conflicting grab for any combination.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_grab_key_checked (c : *mut ffi::base::xcb_connection_t,
                                owner_events :  u8,
                                grab_window :  xcb_window_t,
                                modifiers :  u16,
                                key :  xcb_keycode_t,
                                pointer_mode :  u8,
                                keyboard_mode :  u8) -> ffi::base::xcb_void_cookie_t;

/// Grab keyboard key(s)
///
/// # Arguments
///  * `owner_events` If 1, the `grab_window` will still get the pointer events. If 0, events are not
///       reported to the `grab_window`.
///  * `grab_window` Specifies the window on which the pointer should be grabbed.
///  * `modifiers` The modifiers to grab.
///
///       Using the special value `XCB_MOD_MASK_ANY` means grab the pointer with all
///       possible modifier combinations.
///  * `key` The keycode of the key to grab.
///
///       The special value `XCB_GRAB_ANY` means grab any key.
///  * `pointer_mode`
///  * `keyboard_mode`
///
/// Establishes a passive grab on the keyboard. In the future, the keyboard is
/// actively grabbed (as for `GrabKeyboard`), the last-keyboard-grab time is set to
/// the time at which the key was pressed (as transmitted in the KeyPress event),
/// and the KeyPress event is reported if all of the following conditions are true:
///
/// The keyboard is not grabbed and the specified key (which can itself be a
/// modifier key) is logically pressed when the specified modifier keys are
/// logically down, and no other modifier keys are logically down.
///
/// Either the grab_window is an ancestor of (or is) the focus window, or the
/// grab_window is a descendant of the focus window and contains the pointer.
///
/// A passive grab on the same key combination does not exist on any ancestor of
/// grab_window.
///
/// The interpretation of the remaining arguments is as for XGrabKeyboard.  The active grab is terminated
/// automatically when the logical state of the keyboard has the specified key released (independent of the
/// logical state of the modifier keys), at which point a KeyRelease event is reported to the grabbing window.
///
/// Note that the logical state of a device (as seen by client applications) may lag the physical state if
/// device event processing is frozen.
///
/// A modifiers argument of AnyModifier is equivalent to issuing the request for all possible modifier combinations (including the combination of no modifiers).  It is not required that all modifiers specified
/// have currently assigned KeyCodes.  A keycode argument of AnyKey is equivalent to issuing the request for
/// all possible KeyCodes.  Otherwise, the specified keycode must be in the range specified by min_keycode
/// and max_keycode in the connection setup, or a BadValue error results.
///
/// If some other client has issued a XGrabKey with the same key combination on the same window, a BadAccess
/// error results.  When using AnyModifier or AnyKey, the request fails completely, and a BadAccess error
/// results (no grabs are established) if there is a conflicting grab for any combination.
///
pub fn xcb_grab_key (c : *mut ffi::base::xcb_connection_t,
                        owner_events :  u8,
                        grab_window :  xcb_window_t,
                        modifiers :  u16,
                        key :  xcb_keycode_t,
                        pointer_mode :  u8,
                        keyboard_mode :  u8) -> ffi::base::xcb_void_cookie_t;

/// release a key combination
///
/// # Arguments
///  * `key` The keycode of the specified key combination.
///
///       Using the special value `XCB_GRAB_ANY` means releasing all possible key codes.
///  * `grab_window` The window on which the grabbed key combination will be released.
///  * `modifiers` The modifiers of the specified key combination.
///
///       Using the special value `XCB_MOD_MASK_ANY` means releasing the key combination
///       with every possible modifier combination.
///
/// Releases the key combination on `grab_window` if you grabbed it using
/// `xcb_grab_key` before.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_ungrab_key_checked (c : *mut ffi::base::xcb_connection_t,
                                  key :  xcb_keycode_t,
                                  grab_window :  xcb_window_t,
                                  modifiers :  u16) -> ffi::base::xcb_void_cookie_t;

/// release a key combination
///
/// # Arguments
///  * `key` The keycode of the specified key combination.
///
///       Using the special value `XCB_GRAB_ANY` means releasing all possible key codes.
///  * `grab_window` The window on which the grabbed key combination will be released.
///  * `modifiers` The modifiers of the specified key combination.
///
///       Using the special value `XCB_MOD_MASK_ANY` means releasing the key combination
///       with every possible modifier combination.
///
/// Releases the key combination on `grab_window` if you grabbed it using
/// `xcb_grab_key` before.
///
pub fn xcb_ungrab_key (c : *mut ffi::base::xcb_connection_t,
                          key :  xcb_keycode_t,
                          grab_window :  xcb_window_t,
                          modifiers :  u16) -> ffi::base::xcb_void_cookie_t;

/// release queued events
///
/// # Arguments
///  * `mode`
///  * `time` Timestamp to avoid race conditions when running X over the network.
///
///       The special value `XCB_CURRENT_TIME` will be replaced with the current server
///       time.
///
/// Releases queued events if the client has caused a device (pointer/keyboard) to
/// freeze due to grabbing it actively. This request has no effect if `time` is
/// earlier than the last-grab time of the most recent active grab for this client
/// or if `time` is later than the current X server time.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_allow_events_checked (c : *mut ffi::base::xcb_connection_t,
                                    mode :  u8,
                                    time :  xcb_timestamp_t) -> ffi::base::xcb_void_cookie_t;

/// release queued events
///
/// # Arguments
///  * `mode`
///  * `time` Timestamp to avoid race conditions when running X over the network.
///
///       The special value `XCB_CURRENT_TIME` will be replaced with the current server
///       time.
///
/// Releases queued events if the client has caused a device (pointer/keyboard) to
/// freeze due to grabbing it actively. This request has no effect if `time` is
/// earlier than the last-grab time of the most recent active grab for this client
/// or if `time` is later than the current X server time.
///
pub fn xcb_allow_events (c : *mut ffi::base::xcb_connection_t,
                            mode :  u8,
                            time :  xcb_timestamp_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_grab_server_checked (c : *mut ffi::base::xcb_connection_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_grab_server (c : *mut ffi::base::xcb_connection_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_ungrab_server_checked (c : *mut ffi::base::xcb_connection_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_ungrab_server (c : *mut ffi::base::xcb_connection_t) -> ffi::base::xcb_void_cookie_t;

/// get pointer coordinates
///
/// # Arguments
///  * `window` A window to check if the pointer is on the same screen as `window` (see the
///       `same_screen` field in the reply).
///
/// Gets the root window the pointer is logically on and the pointer coordinates
/// relative to the root window's origin.
///
pub fn xcb_query_pointer (c : *mut ffi::base::xcb_connection_t,
                             window :  xcb_window_t) -> xcb_query_pointer_cookie_t;

/// get pointer coordinates
///
/// # Arguments
///  * `window` A window to check if the pointer is on the same screen as `window` (see the
///       `same_screen` field in the reply).
///
/// Gets the root window the pointer is logically on and the pointer coordinates
/// relative to the root window's origin.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_query_pointer_unchecked (c : *mut ffi::base::xcb_connection_t,
                                       window :  xcb_window_t) -> xcb_query_pointer_cookie_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_query_pointer_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_query_pointer_reply (c : *mut ffi::base::xcb_connection_t,
                                   cookie : xcb_query_pointer_cookie_t,
                                   e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_query_pointer_reply_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_timecoord_t)
///
pub fn xcb_timecoord_next (i:*mut xcb_timecoord_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_timecoord_end (i:xcb_timecoord_iterator_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_get_motion_events_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
///
pub fn xcb_get_motion_events (c : *mut ffi::base::xcb_connection_t,
                                 window :  xcb_window_t,
                                 start :  xcb_timestamp_t,
                                 stop :  xcb_timestamp_t) -> xcb_get_motion_events_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_get_motion_events_unchecked (c : *mut ffi::base::xcb_connection_t,
                                           window :  xcb_window_t,
                                           start :  xcb_timestamp_t,
                                           stop :  xcb_timestamp_t) -> xcb_get_motion_events_cookie_t;

pub fn xcb_get_motion_events_events (R : *mut xcb_get_motion_events_reply_t) -> *mut xcb_timecoord_t;


pub fn xcb_get_motion_events_events_length (R : *mut xcb_get_motion_events_reply_t) -> c_int;

pub fn xcb_get_motion_events_events_iterator (R : *mut xcb_get_motion_events_reply_t) -> xcb_timecoord_iterator_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_get_motion_events_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_get_motion_events_reply (c : *mut ffi::base::xcb_connection_t,
                                       cookie : xcb_get_motion_events_cookie_t,
                                       e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_get_motion_events_reply_t;

/// Delivers a request to the X server.
///
pub fn xcb_translate_coordinates (c : *mut ffi::base::xcb_connection_t,
                                     src_window :  xcb_window_t,
                                     dst_window :  xcb_window_t,
                                     src_x :  i16,
                                     src_y :  i16) -> xcb_translate_coordinates_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_translate_coordinates_unchecked (c : *mut ffi::base::xcb_connection_t,
                                               src_window :  xcb_window_t,
                                               dst_window :  xcb_window_t,
                                               src_x :  i16,
                                               src_y :  i16) -> xcb_translate_coordinates_cookie_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_translate_coordinates_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_translate_coordinates_reply (c : *mut ffi::base::xcb_connection_t,
                                           cookie : xcb_translate_coordinates_cookie_t,
                                           e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_translate_coordinates_reply_t;

/// move mouse pointer
///
/// # Arguments
///  * `src_window` If `src_window` is not `XCB_NONE` (TODO), the move will only take place if the
///       pointer is inside `src_window` and within the rectangle specified by (`src_x`,
///       `src_y`, `src_width`, `src_height`). The rectangle coordinates are relative to
///       `src_window`.
///  * `dst_window` If `dst_window` is not `XCB_NONE` (TODO), the pointer will be moved to the
///       offsets (`dst_x`, `dst_y`) relative to `dst_window`. If `dst_window` is
///       `XCB_NONE` (TODO), the pointer will be moved by the offsets (`dst_x`, `dst_y`)
///       relative to the current position of the pointer.
///
/// Moves the mouse pointer to the specified position.
///
/// If `src_window` is not `XCB_NONE` (TODO), the move will only take place if the
/// pointer is inside `src_window` and within the rectangle specified by (`src_x`,
/// `src_y`, `src_width`, `src_height`). The rectangle coordinates are relative to
/// `src_window`.
///
/// If `dst_window` is not `XCB_NONE` (TODO), the pointer will be moved to the
/// offsets (`dst_x`, `dst_y`) relative to `dst_window`. If `dst_window` is
/// `XCB_NONE` (TODO), the pointer will be moved by the offsets (`dst_x`, `dst_y`)
/// relative to the current position of the pointer.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_warp_pointer_checked (c : *mut ffi::base::xcb_connection_t,
                                    src_window :  xcb_window_t,
                                    dst_window :  xcb_window_t,
                                    src_x :  i16,
                                    src_y :  i16,
                                    src_width :  u16,
                                    src_height :  u16,
                                    dst_x :  i16,
                                    dst_y :  i16) -> ffi::base::xcb_void_cookie_t;

/// move mouse pointer
///
/// # Arguments
///  * `src_window` If `src_window` is not `XCB_NONE` (TODO), the move will only take place if the
///       pointer is inside `src_window` and within the rectangle specified by (`src_x`,
///       `src_y`, `src_width`, `src_height`). The rectangle coordinates are relative to
///       `src_window`.
///  * `dst_window` If `dst_window` is not `XCB_NONE` (TODO), the pointer will be moved to the
///       offsets (`dst_x`, `dst_y`) relative to `dst_window`. If `dst_window` is
///       `XCB_NONE` (TODO), the pointer will be moved by the offsets (`dst_x`, `dst_y`)
///       relative to the current position of the pointer.
///
/// Moves the mouse pointer to the specified position.
///
/// If `src_window` is not `XCB_NONE` (TODO), the move will only take place if the
/// pointer is inside `src_window` and within the rectangle specified by (`src_x`,
/// `src_y`, `src_width`, `src_height`). The rectangle coordinates are relative to
/// `src_window`.
///
/// If `dst_window` is not `XCB_NONE` (TODO), the pointer will be moved to the
/// offsets (`dst_x`, `dst_y`) relative to `dst_window`. If `dst_window` is
/// `XCB_NONE` (TODO), the pointer will be moved by the offsets (`dst_x`, `dst_y`)
/// relative to the current position of the pointer.
///
pub fn xcb_warp_pointer (c : *mut ffi::base::xcb_connection_t,
                            src_window :  xcb_window_t,
                            dst_window :  xcb_window_t,
                            src_x :  i16,
                            src_y :  i16,
                            src_width :  u16,
                            src_height :  u16,
                            dst_x :  i16,
                            dst_y :  i16) -> ffi::base::xcb_void_cookie_t;

/// Sets input focus
///
/// # Arguments
///  * `revert_to` Specifies what happens when the `focus` window becomes unviewable (if `focus`
///       is neither `XCB_NONE` nor `XCB_POINTER_ROOT`).
///  * `focus` The window to focus. All keyboard events will be reported to this window. The
///       window must be viewable (TODO), or a `xcb_match_error_t` occurs (TODO).
///
///       If `focus` is `XCB_NONE` (TODO), all keyboard events are
///       discarded until a new focus window is set.
///
///       If `focus` is `XCB_POINTER_ROOT` (TODO), focus is on the root window of the
///       screen on which the pointer is on currently.
///  * `time` Timestamp to avoid race conditions when running X over the network.
///
///       The special value `XCB_CURRENT_TIME` will be replaced with the current server
///       time.
///
/// Changes the input focus and the last-focus-change time. If the specified `time`
/// is earlier than the current last-focus-change time, the request is ignored (to
/// avoid race conditions when running X over the network).
///
/// A FocusIn and FocusOut event is generated when focus is changed.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_set_input_focus_checked (c : *mut ffi::base::xcb_connection_t,
                                       revert_to :  u8,
                                       focus :  xcb_window_t,
                                       time :  xcb_timestamp_t) -> ffi::base::xcb_void_cookie_t;

/// Sets input focus
///
/// # Arguments
///  * `revert_to` Specifies what happens when the `focus` window becomes unviewable (if `focus`
///       is neither `XCB_NONE` nor `XCB_POINTER_ROOT`).
///  * `focus` The window to focus. All keyboard events will be reported to this window. The
///       window must be viewable (TODO), or a `xcb_match_error_t` occurs (TODO).
///
///       If `focus` is `XCB_NONE` (TODO), all keyboard events are
///       discarded until a new focus window is set.
///
///       If `focus` is `XCB_POINTER_ROOT` (TODO), focus is on the root window of the
///       screen on which the pointer is on currently.
///  * `time` Timestamp to avoid race conditions when running X over the network.
///
///       The special value `XCB_CURRENT_TIME` will be replaced with the current server
///       time.
///
/// Changes the input focus and the last-focus-change time. If the specified `time`
/// is earlier than the current last-focus-change time, the request is ignored (to
/// avoid race conditions when running X over the network).
///
/// A FocusIn and FocusOut event is generated when focus is changed.
///
pub fn xcb_set_input_focus (c : *mut ffi::base::xcb_connection_t,
                               revert_to :  u8,
                               focus :  xcb_window_t,
                               time :  xcb_timestamp_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_get_input_focus (c : *mut ffi::base::xcb_connection_t) -> xcb_get_input_focus_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_get_input_focus_unchecked (c : *mut ffi::base::xcb_connection_t) -> xcb_get_input_focus_cookie_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_get_input_focus_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_get_input_focus_reply (c : *mut ffi::base::xcb_connection_t,
                                     cookie : xcb_get_input_focus_cookie_t,
                                     e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_get_input_focus_reply_t;

/// Delivers a request to the X server.
///
pub fn xcb_query_keymap (c : *mut ffi::base::xcb_connection_t) -> xcb_query_keymap_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_query_keymap_unchecked (c : *mut ffi::base::xcb_connection_t) -> xcb_query_keymap_cookie_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_query_keymap_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_query_keymap_reply (c : *mut ffi::base::xcb_connection_t,
                                  cookie : xcb_query_keymap_cookie_t,
                                  e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_query_keymap_reply_t;

pub fn xcb_open_font_sizeof (_buffer :  *mut c_void) -> c_int;

/// opens a font
///
/// # Arguments
///  * `fid` The ID with which you will refer to the font, created by `xcb_generate_id`.
///  * `name_len` Length (in bytes) of `name`.
///  * `name` A pattern describing an X core font.
///
/// Opens any X core font matching the given `name` (for example "-misc-fixed-*").
///
/// Note that X core fonts are deprecated (but still supported) in favor of
/// client-side rendering using Xft.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_open_font_checked (c : *mut ffi::base::xcb_connection_t,
                                 fid :  xcb_font_t,
                                 name_len :  u16,
                                 name : *mut c_char) -> ffi::base::xcb_void_cookie_t;

/// opens a font
///
/// # Arguments
///  * `fid` The ID with which you will refer to the font, created by `xcb_generate_id`.
///  * `name_len` Length (in bytes) of `name`.
///  * `name` A pattern describing an X core font.
///
/// Opens any X core font matching the given `name` (for example "-misc-fixed-*").
///
/// Note that X core fonts are deprecated (but still supported) in favor of
/// client-side rendering using Xft.
///
pub fn xcb_open_font (c : *mut ffi::base::xcb_connection_t,
                         fid :  xcb_font_t,
                         name_len :  u16,
                         name : *mut c_char) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_close_font_checked (c : *mut ffi::base::xcb_connection_t,
                                  font :  xcb_font_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_close_font (c : *mut ffi::base::xcb_connection_t,
                          font :  xcb_font_t) -> ffi::base::xcb_void_cookie_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_fontprop_t)
///
pub fn xcb_fontprop_next (i:*mut xcb_fontprop_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_fontprop_end (i:xcb_fontprop_iterator_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_charinfo_t)
///
pub fn xcb_charinfo_next (i:*mut xcb_charinfo_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_charinfo_end (i:xcb_charinfo_iterator_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_query_font_sizeof (_buffer :  *mut c_void) -> c_int;

/// query font metrics
///
/// # Arguments
///  * `font` The fontable (Font or Graphics Context) to query.
///
/// Queries information associated with the font.
///
pub fn xcb_query_font (c : *mut ffi::base::xcb_connection_t,
                          font :  xcb_fontable_t) -> xcb_query_font_cookie_t;

/// query font metrics
///
/// # Arguments
///  * `font` The fontable (Font or Graphics Context) to query.
///
/// Queries information associated with the font.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_query_font_unchecked (c : *mut ffi::base::xcb_connection_t,
                                    font :  xcb_fontable_t) -> xcb_query_font_cookie_t;

pub fn xcb_query_font_properties (R : *mut xcb_query_font_reply_t) -> *mut xcb_fontprop_t;


pub fn xcb_query_font_properties_length (R : *mut xcb_query_font_reply_t) -> c_int;

pub fn xcb_query_font_properties_iterator (R : *mut xcb_query_font_reply_t) -> xcb_fontprop_iterator_t;

pub fn xcb_query_font_char_infos (R : *mut xcb_query_font_reply_t) -> *mut xcb_charinfo_t;


pub fn xcb_query_font_char_infos_length (R : *mut xcb_query_font_reply_t) -> c_int;

pub fn xcb_query_font_char_infos_iterator (R : *mut xcb_query_font_reply_t) -> xcb_charinfo_iterator_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_query_font_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_query_font_reply (c : *mut ffi::base::xcb_connection_t,
                                cookie : xcb_query_font_cookie_t,
                                e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_query_font_reply_t;

pub fn xcb_query_text_extents_sizeof (_buffer :  *mut c_void,
                               string_len :   u32) -> c_int;

/// get text extents
///
/// # Arguments
///  * `font` The `font` to calculate text extents in. You can also pass a graphics context.
///  * `string_len` The number of characters in `string`.
///  * `string` The text to get text extents for.
///
/// Query text extents from the X11 server. This request returns the bounding box
/// of the specified 16-bit character string in the specified `font` or the font
/// contained in the specified graphics context.
///
/// `font_ascent` is set to the maximum of the ascent metrics of all characters in
/// the string. `font_descent` is set to the maximum of the descent metrics.
/// `overall_width` is set to the sum of the character-width metrics of all
/// characters in the string. For each character in the string, let W be the sum of
/// the character-width metrics of all characters preceding it in the string. Let L
/// be the left-side-bearing metric of the character plus W. Let R be the
/// right-side-bearing metric of the character plus W. The lbearing member is set
/// to the minimum L of all characters in the string. The rbearing member is set to
/// the maximum R.
///
/// For fonts defined with linear indexing rather than 2-byte matrix indexing, each
/// `xcb_char2b_t` structure is interpreted as a 16-bit number with byte1 as the
/// most significant byte. If the font has no defined default character, undefined
/// characters in the string are taken to have all zero metrics.
///
/// Characters with all zero metrics are ignored. If the font has no defined
/// default_char, the undefined characters in the string are also ignored.
///
pub fn xcb_query_text_extents (c : *mut ffi::base::xcb_connection_t,
                                  font :  xcb_fontable_t,
                                  string_len :  u32,
                                  string : *mut xcb_char2b_t) -> xcb_query_text_extents_cookie_t;

/// get text extents
///
/// # Arguments
///  * `font` The `font` to calculate text extents in. You can also pass a graphics context.
///  * `string_len` The number of characters in `string`.
///  * `string` The text to get text extents for.
///
/// Query text extents from the X11 server. This request returns the bounding box
/// of the specified 16-bit character string in the specified `font` or the font
/// contained in the specified graphics context.
///
/// `font_ascent` is set to the maximum of the ascent metrics of all characters in
/// the string. `font_descent` is set to the maximum of the descent metrics.
/// `overall_width` is set to the sum of the character-width metrics of all
/// characters in the string. For each character in the string, let W be the sum of
/// the character-width metrics of all characters preceding it in the string. Let L
/// be the left-side-bearing metric of the character plus W. Let R be the
/// right-side-bearing metric of the character plus W. The lbearing member is set
/// to the minimum L of all characters in the string. The rbearing member is set to
/// the maximum R.
///
/// For fonts defined with linear indexing rather than 2-byte matrix indexing, each
/// `xcb_char2b_t` structure is interpreted as a 16-bit number with byte1 as the
/// most significant byte. If the font has no defined default character, undefined
/// characters in the string are taken to have all zero metrics.
///
/// Characters with all zero metrics are ignored. If the font has no defined
/// default_char, the undefined characters in the string are also ignored.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_query_text_extents_unchecked (c : *mut ffi::base::xcb_connection_t,
                                            font :  xcb_fontable_t,
                                            string_len :  u32,
                                            string : *mut xcb_char2b_t) -> xcb_query_text_extents_cookie_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_query_text_extents_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_query_text_extents_reply (c : *mut ffi::base::xcb_connection_t,
                                        cookie : xcb_query_text_extents_cookie_t,
                                        e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_query_text_extents_reply_t;

pub fn xcb_str_sizeof (_buffer :  *mut c_void) -> c_int;

pub fn xcb_str_name (R : *mut xcb_str_t) -> *mut c_char;


pub fn xcb_str_name_length (R : *mut xcb_str_t) -> c_int;


pub fn xcb_str_name_end (R : *mut xcb_str_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_str_t)
///
pub fn xcb_str_next (i:*mut xcb_str_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_str_end (i:xcb_str_iterator_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_list_fonts_sizeof (_buffer :  *mut c_void) -> c_int;

/// get matching font names
///
/// # Arguments
///  * `max_names` The maximum number of fonts to be returned.
///  * `pattern_len` The length (in bytes) of `pattern`.
///  * `pattern` A font pattern, for example "-misc-fixed-*".
///
///       The asterisk (*) is a wildcard for any number of characters. The question mark
///       (?) is a wildcard for a single character. Use of uppercase or lowercase does
///       not matter.
///
/// Gets a list of available font names which match the given `pattern`.
///
pub fn xcb_list_fonts (c : *mut ffi::base::xcb_connection_t,
                          max_names :  u16,
                          pattern_len :  u16,
                          pattern : *mut c_char) -> xcb_list_fonts_cookie_t;

/// get matching font names
///
/// # Arguments
///  * `max_names` The maximum number of fonts to be returned.
///  * `pattern_len` The length (in bytes) of `pattern`.
///  * `pattern` A font pattern, for example "-misc-fixed-*".
///
///       The asterisk (*) is a wildcard for any number of characters. The question mark
///       (?) is a wildcard for a single character. Use of uppercase or lowercase does
///       not matter.
///
/// Gets a list of available font names which match the given `pattern`.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_list_fonts_unchecked (c : *mut ffi::base::xcb_connection_t,
                                    max_names :  u16,
                                    pattern_len :  u16,
                                    pattern : *mut c_char) -> xcb_list_fonts_cookie_t;


pub fn xcb_list_fonts_names_length (R : *mut xcb_list_fonts_reply_t) -> c_int;

pub fn xcb_list_fonts_names_iterator (R : *mut xcb_list_fonts_reply_t) -> xcb_str_iterator_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_list_fonts_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_list_fonts_reply (c : *mut ffi::base::xcb_connection_t,
                                cookie : xcb_list_fonts_cookie_t,
                                e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_list_fonts_reply_t;

pub fn xcb_list_fonts_with_info_sizeof (_buffer :  *mut c_void) -> c_int;

/// get matching font names and information
///
/// # Arguments
///  * `max_names` The maximum number of fonts to be returned.
///  * `pattern_len` The length (in bytes) of `pattern`.
///  * `pattern` A font pattern, for example "-misc-fixed-*".
///
///       The asterisk (*) is a wildcard for any number of characters. The question mark
///       (?) is a wildcard for a single character. Use of uppercase or lowercase does
///       not matter.
///
/// Gets a list of available font names which match the given `pattern`.
///
pub fn xcb_list_fonts_with_info (c : *mut ffi::base::xcb_connection_t,
                                    max_names :  u16,
                                    pattern_len :  u16,
                                    pattern : *mut c_char) -> xcb_list_fonts_with_info_cookie_t;

/// get matching font names and information
///
/// # Arguments
///  * `max_names` The maximum number of fonts to be returned.
///  * `pattern_len` The length (in bytes) of `pattern`.
///  * `pattern` A font pattern, for example "-misc-fixed-*".
///
///       The asterisk (*) is a wildcard for any number of characters. The question mark
///       (?) is a wildcard for a single character. Use of uppercase or lowercase does
///       not matter.
///
/// Gets a list of available font names which match the given `pattern`.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_list_fonts_with_info_unchecked (c : *mut ffi::base::xcb_connection_t,
                                              max_names :  u16,
                                              pattern_len :  u16,
                                              pattern : *mut c_char) -> xcb_list_fonts_with_info_cookie_t;

pub fn xcb_list_fonts_with_info_properties (R : *mut xcb_list_fonts_with_info_reply_t) -> *mut xcb_fontprop_t;


pub fn xcb_list_fonts_with_info_properties_length (R : *mut xcb_list_fonts_with_info_reply_t) -> c_int;

pub fn xcb_list_fonts_with_info_properties_iterator (R : *mut xcb_list_fonts_with_info_reply_t) -> xcb_fontprop_iterator_t;

pub fn xcb_list_fonts_with_info_name (R : *mut xcb_list_fonts_with_info_reply_t) -> *mut c_char;


pub fn xcb_list_fonts_with_info_name_length (R : *mut xcb_list_fonts_with_info_reply_t) -> c_int;


pub fn xcb_list_fonts_with_info_name_end (R : *mut xcb_list_fonts_with_info_reply_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_list_fonts_with_info_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_list_fonts_with_info_reply (c : *mut ffi::base::xcb_connection_t,
                                          cookie : xcb_list_fonts_with_info_cookie_t,
                                          e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_list_fonts_with_info_reply_t;

pub fn xcb_set_font_path_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_set_font_path_checked (c : *mut ffi::base::xcb_connection_t,
                                     font_qty :  u16,
                                     font : *mut xcb_str_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_set_font_path (c : *mut ffi::base::xcb_connection_t,
                             font_qty :  u16,
                             font : *mut xcb_str_t) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_get_font_path_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
///
pub fn xcb_get_font_path (c : *mut ffi::base::xcb_connection_t) -> xcb_get_font_path_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_get_font_path_unchecked (c : *mut ffi::base::xcb_connection_t) -> xcb_get_font_path_cookie_t;


pub fn xcb_get_font_path_path_length (R : *mut xcb_get_font_path_reply_t) -> c_int;

pub fn xcb_get_font_path_path_iterator (R : *mut xcb_get_font_path_reply_t) -> xcb_str_iterator_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_get_font_path_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_get_font_path_reply (c : *mut ffi::base::xcb_connection_t,
                                   cookie : xcb_get_font_path_cookie_t,
                                   e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_get_font_path_reply_t;

/// Creates a pixmap
///
/// # Arguments
///  * `depth` TODO
///  * `pid` The ID with which you will refer to the new pixmap, created by
///       `xcb_generate_id`.
///  * `drawable` Drawable to get the screen from.
///  * `width` The width of the new pixmap.
///  * `height` The height of the new pixmap.
///
/// Creates a pixmap. The pixmap can only be used on the same screen as `drawable`
/// is on and only with drawables of the same `depth`.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_create_pixmap_checked (c : *mut ffi::base::xcb_connection_t,
                                     depth :  u8,
                                     pid :  xcb_pixmap_t,
                                     drawable :  xcb_drawable_t,
                                     width :  u16,
                                     height :  u16) -> ffi::base::xcb_void_cookie_t;

/// Creates a pixmap
///
/// # Arguments
///  * `depth` TODO
///  * `pid` The ID with which you will refer to the new pixmap, created by
///       `xcb_generate_id`.
///  * `drawable` Drawable to get the screen from.
///  * `width` The width of the new pixmap.
///  * `height` The height of the new pixmap.
///
/// Creates a pixmap. The pixmap can only be used on the same screen as `drawable`
/// is on and only with drawables of the same `depth`.
///
pub fn xcb_create_pixmap (c : *mut ffi::base::xcb_connection_t,
                             depth :  u8,
                             pid :  xcb_pixmap_t,
                             drawable :  xcb_drawable_t,
                             width :  u16,
                             height :  u16) -> ffi::base::xcb_void_cookie_t;

/// Destroys a pixmap
///
/// # Arguments
///  * `pixmap` The pixmap to destroy.
///
/// Deletes the association between the pixmap ID and the pixmap. The pixmap
/// storage will be freed when there are no more references to it.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_free_pixmap_checked (c : *mut ffi::base::xcb_connection_t,
                                   pixmap :  xcb_pixmap_t) -> ffi::base::xcb_void_cookie_t;

/// Destroys a pixmap
///
/// # Arguments
///  * `pixmap` The pixmap to destroy.
///
/// Deletes the association between the pixmap ID and the pixmap. The pixmap
/// storage will be freed when there are no more references to it.
///
pub fn xcb_free_pixmap (c : *mut ffi::base::xcb_connection_t,
                           pixmap :  xcb_pixmap_t) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_create_gc_sizeof (_buffer :  *mut c_void) -> c_int;

/// Creates a graphics context
///
/// # Arguments
///  * `cid` The ID with which you will refer to the graphics context, created by
///       `xcb_generate_id`.
///  * `drawable` Drawable to get the root/depth from.
///
/// Creates a graphics context. The graphics context can be used with any drawable
/// that has the same root and depth as the specified drawable.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_create_gc_checked (c : *mut ffi::base::xcb_connection_t,
                                 cid :  xcb_gcontext_t,
                                 drawable :  xcb_drawable_t,
                                 value_mask :  u32,
                                 value_list : *mut u32) -> ffi::base::xcb_void_cookie_t;

/// Creates a graphics context
///
/// # Arguments
///  * `cid` The ID with which you will refer to the graphics context, created by
///       `xcb_generate_id`.
///  * `drawable` Drawable to get the root/depth from.
///
/// Creates a graphics context. The graphics context can be used with any drawable
/// that has the same root and depth as the specified drawable.
///
pub fn xcb_create_gc (c : *mut ffi::base::xcb_connection_t,
                         cid :  xcb_gcontext_t,
                         drawable :  xcb_drawable_t,
                         value_mask :  u32,
                         value_list : *mut u32) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_change_gc_sizeof (_buffer :  *mut c_void) -> c_int;

/// change graphics context components
///
/// # Arguments
///  * `gc` The graphics context to change.
///  * `value_mask`
///  * `value_list` Values for each of the components specified in the bitmask `value_mask`. The
///       order has to correspond to the order of possible `value_mask` bits. See the
///       example.
///
/// Changes the components specified by `value_mask` for the specified graphics context.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_change_gc_checked (c : *mut ffi::base::xcb_connection_t,
                                 gc :  xcb_gcontext_t,
                                 value_mask :  u32,
                                 value_list : *mut u32) -> ffi::base::xcb_void_cookie_t;

/// change graphics context components
///
/// # Arguments
///  * `gc` The graphics context to change.
///  * `value_mask`
///  * `value_list` Values for each of the components specified in the bitmask `value_mask`. The
///       order has to correspond to the order of possible `value_mask` bits. See the
///       example.
///
/// Changes the components specified by `value_mask` for the specified graphics context.
///
pub fn xcb_change_gc (c : *mut ffi::base::xcb_connection_t,
                         gc :  xcb_gcontext_t,
                         value_mask :  u32,
                         value_list : *mut u32) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_copy_gc_checked (c : *mut ffi::base::xcb_connection_t,
                               src_gc :  xcb_gcontext_t,
                               dst_gc :  xcb_gcontext_t,
                               value_mask :  u32) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_copy_gc (c : *mut ffi::base::xcb_connection_t,
                       src_gc :  xcb_gcontext_t,
                       dst_gc :  xcb_gcontext_t,
                       value_mask :  u32) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_set_dashes_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_set_dashes_checked (c : *mut ffi::base::xcb_connection_t,
                                  gc :  xcb_gcontext_t,
                                  dash_offset :  u16,
                                  dashes_len :  u16,
                                  dashes : *mut u8) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_set_dashes (c : *mut ffi::base::xcb_connection_t,
                          gc :  xcb_gcontext_t,
                          dash_offset :  u16,
                          dashes_len :  u16,
                          dashes : *mut u8) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_set_clip_rectangles_sizeof (_buffer :  *mut c_void,
                                rectangles_len :  u32) -> c_int;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_set_clip_rectangles_checked (c : *mut ffi::base::xcb_connection_t,
                                           ordering :  u8,
                                           gc :  xcb_gcontext_t,
                                           clip_x_origin :  i16,
                                           clip_y_origin :  i16,
                                           rectangles_len :  u32,
                                           rectangles : *mut xcb_rectangle_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_set_clip_rectangles (c : *mut ffi::base::xcb_connection_t,
                                   ordering :  u8,
                                   gc :  xcb_gcontext_t,
                                   clip_x_origin :  i16,
                                   clip_y_origin :  i16,
                                   rectangles_len :  u32,
                                   rectangles : *mut xcb_rectangle_t) -> ffi::base::xcb_void_cookie_t;

/// Destroys a graphics context
///
/// # Arguments
///  * `gc` The graphics context to destroy.
///
/// Destroys the specified `gc` and all associated storage.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_free_gc_checked (c : *mut ffi::base::xcb_connection_t,
                               gc :  xcb_gcontext_t) -> ffi::base::xcb_void_cookie_t;

/// Destroys a graphics context
///
/// # Arguments
///  * `gc` The graphics context to destroy.
///
/// Destroys the specified `gc` and all associated storage.
///
pub fn xcb_free_gc (c : *mut ffi::base::xcb_connection_t,
                       gc :  xcb_gcontext_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_clear_area_checked (c : *mut ffi::base::xcb_connection_t,
                                  exposures :  u8,
                                  window :  xcb_window_t,
                                  x :  i16,
                                  y :  i16,
                                  width :  u16,
                                  height :  u16) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_clear_area (c : *mut ffi::base::xcb_connection_t,
                          exposures :  u8,
                          window :  xcb_window_t,
                          x :  i16,
                          y :  i16,
                          width :  u16,
                          height :  u16) -> ffi::base::xcb_void_cookie_t;

/// copy areas
///
/// # Arguments
///  * `src_drawable` The source drawable (Window or Pixmap).
///  * `dst_drawable` The destination drawable (Window or Pixmap).
///  * `gc` The graphics context to use.
///  * `src_x` The source X coordinate.
///  * `src_y` The source Y coordinate.
///  * `dst_x` The destination X coordinate.
///  * `dst_y` The destination Y coordinate.
///  * `width` The width of the area to copy (in pixels).
///  * `height` The height of the area to copy (in pixels).
///
/// Copies the specified rectangle from `src_drawable` to `dst_drawable`.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_copy_area_checked (c : *mut ffi::base::xcb_connection_t,
                                 src_drawable :  xcb_drawable_t,
                                 dst_drawable :  xcb_drawable_t,
                                 gc :  xcb_gcontext_t,
                                 src_x :  i16,
                                 src_y :  i16,
                                 dst_x :  i16,
                                 dst_y :  i16,
                                 width :  u16,
                                 height :  u16) -> ffi::base::xcb_void_cookie_t;

/// copy areas
///
/// # Arguments
///  * `src_drawable` The source drawable (Window or Pixmap).
///  * `dst_drawable` The destination drawable (Window or Pixmap).
///  * `gc` The graphics context to use.
///  * `src_x` The source X coordinate.
///  * `src_y` The source Y coordinate.
///  * `dst_x` The destination X coordinate.
///  * `dst_y` The destination Y coordinate.
///  * `width` The width of the area to copy (in pixels).
///  * `height` The height of the area to copy (in pixels).
///
/// Copies the specified rectangle from `src_drawable` to `dst_drawable`.
///
pub fn xcb_copy_area (c : *mut ffi::base::xcb_connection_t,
                         src_drawable :  xcb_drawable_t,
                         dst_drawable :  xcb_drawable_t,
                         gc :  xcb_gcontext_t,
                         src_x :  i16,
                         src_y :  i16,
                         dst_x :  i16,
                         dst_y :  i16,
                         width :  u16,
                         height :  u16) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_copy_plane_checked (c : *mut ffi::base::xcb_connection_t,
                                  src_drawable :  xcb_drawable_t,
                                  dst_drawable :  xcb_drawable_t,
                                  gc :  xcb_gcontext_t,
                                  src_x :  i16,
                                  src_y :  i16,
                                  dst_x :  i16,
                                  dst_y :  i16,
                                  width :  u16,
                                  height :  u16,
                                  bit_plane :  u32) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_copy_plane (c : *mut ffi::base::xcb_connection_t,
                          src_drawable :  xcb_drawable_t,
                          dst_drawable :  xcb_drawable_t,
                          gc :  xcb_gcontext_t,
                          src_x :  i16,
                          src_y :  i16,
                          dst_x :  i16,
                          dst_y :  i16,
                          width :  u16,
                          height :  u16,
                          bit_plane :  u32) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_poly_point_sizeof (_buffer :  *mut c_void,
                       points_len :   u32) -> c_int;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_poly_point_checked (c : *mut ffi::base::xcb_connection_t,
                                  coordinate_mode :  u8,
                                  drawable :  xcb_drawable_t,
                                  gc :  xcb_gcontext_t,
                                  points_len :  u32,
                                  points : *mut xcb_point_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_poly_point (c : *mut ffi::base::xcb_connection_t,
                          coordinate_mode :  u8,
                          drawable :  xcb_drawable_t,
                          gc :  xcb_gcontext_t,
                          points_len :  u32,
                          points : *mut xcb_point_t) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_poly_line_sizeof (_buffer :  *mut c_void,
                      points_len :   u32) -> c_int;

/// draw lines
///
/// # Arguments
///  * `coordinate_mode`
///  * `drawable` The drawable to draw the line(s) on.
///  * `gc` The graphics context to use.
///  * `points_len` The number of `xcb_point_t` structures in `points`.
///  * `points` An array of points.
///
/// Draws `points_len`-1 lines between each pair of points (point[i], point[i+1])
/// in the `points` array. The lines are drawn in the order listed in the array.
/// They join correctly at all intermediate points, and if the first and last
/// points coincide, the first and last lines also join correctly. For any given
/// line, a pixel is not drawn more than once. If thin (zero line-width) lines
/// intersect, the intersecting pixels are drawn multiple times. If wide lines
/// intersect, the intersecting pixels are drawn only once, as though the entire
/// request were a single, filled shape.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_poly_line_checked (c : *mut ffi::base::xcb_connection_t,
                                 coordinate_mode :  u8,
                                 drawable :  xcb_drawable_t,
                                 gc :  xcb_gcontext_t,
                                 points_len :  u32,
                                 points : *mut xcb_point_t) -> ffi::base::xcb_void_cookie_t;

/// draw lines
///
/// # Arguments
///  * `coordinate_mode`
///  * `drawable` The drawable to draw the line(s) on.
///  * `gc` The graphics context to use.
///  * `points_len` The number of `xcb_point_t` structures in `points`.
///  * `points` An array of points.
///
/// Draws `points_len`-1 lines between each pair of points (point[i], point[i+1])
/// in the `points` array. The lines are drawn in the order listed in the array.
/// They join correctly at all intermediate points, and if the first and last
/// points coincide, the first and last lines also join correctly. For any given
/// line, a pixel is not drawn more than once. If thin (zero line-width) lines
/// intersect, the intersecting pixels are drawn multiple times. If wide lines
/// intersect, the intersecting pixels are drawn only once, as though the entire
/// request were a single, filled shape.
///
pub fn xcb_poly_line (c : *mut ffi::base::xcb_connection_t,
                         coordinate_mode :  u8,
                         drawable :  xcb_drawable_t,
                         gc :  xcb_gcontext_t,
                         points_len :  u32,
                         points : *mut xcb_point_t) -> ffi::base::xcb_void_cookie_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_segment_t)
///
pub fn xcb_segment_next (i:*mut xcb_segment_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_segment_end (i:xcb_segment_iterator_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_poly_segment_sizeof (_buffer :  *mut c_void,
                         segments_len :  u32) -> c_int;

/// draw lines
///
/// # Arguments
///  * `drawable` A drawable (Window or Pixmap) to draw on.
///  * `gc` The graphics context to use.
///
///       TODO: document which attributes of a gc are used
///  * `segments_len` The number of `xcb_segment_t` structures in `segments`.
///  * `segments` An array of `xcb_segment_t` structures.
///
/// Draws multiple, unconnected lines. For each segment, a line is drawn between
/// (x1, y1) and (x2, y2). The lines are drawn in the order listed in the array of
/// `xcb_segment_t` structures and does not perform joining at coincident
/// endpoints. For any given line, a pixel is not drawn more than once. If lines
/// intersect, the intersecting pixels are drawn multiple times.
///
/// TODO: include the xcb_segment_t data structure
///
/// TODO: an example
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_poly_segment_checked (c : *mut ffi::base::xcb_connection_t,
                                    drawable :  xcb_drawable_t,
                                    gc :  xcb_gcontext_t,
                                    segments_len :  u32,
                                    segments : *mut xcb_segment_t) -> ffi::base::xcb_void_cookie_t;

/// draw lines
///
/// # Arguments
///  * `drawable` A drawable (Window or Pixmap) to draw on.
///  * `gc` The graphics context to use.
///
///       TODO: document which attributes of a gc are used
///  * `segments_len` The number of `xcb_segment_t` structures in `segments`.
///  * `segments` An array of `xcb_segment_t` structures.
///
/// Draws multiple, unconnected lines. For each segment, a line is drawn between
/// (x1, y1) and (x2, y2). The lines are drawn in the order listed in the array of
/// `xcb_segment_t` structures and does not perform joining at coincident
/// endpoints. For any given line, a pixel is not drawn more than once. If lines
/// intersect, the intersecting pixels are drawn multiple times.
///
/// TODO: include the xcb_segment_t data structure
///
/// TODO: an example
///
pub fn xcb_poly_segment (c : *mut ffi::base::xcb_connection_t,
                            drawable :  xcb_drawable_t,
                            gc :  xcb_gcontext_t,
                            segments_len :  u32,
                            segments : *mut xcb_segment_t) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_poly_rectangle_sizeof (_buffer :  *mut c_void,
                           rectangles_len :  u32) -> c_int;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_poly_rectangle_checked (c : *mut ffi::base::xcb_connection_t,
                                      drawable :  xcb_drawable_t,
                                      gc :  xcb_gcontext_t,
                                      rectangles_len :  u32,
                                      rectangles : *mut xcb_rectangle_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_poly_rectangle (c : *mut ffi::base::xcb_connection_t,
                              drawable :  xcb_drawable_t,
                              gc :  xcb_gcontext_t,
                              rectangles_len :  u32,
                              rectangles : *mut xcb_rectangle_t) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_poly_arc_sizeof (_buffer :  *mut c_void,
                     arcs_len :     u32) -> c_int;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_poly_arc_checked (c : *mut ffi::base::xcb_connection_t,
                                drawable :  xcb_drawable_t,
                                gc :  xcb_gcontext_t,
                                arcs_len :  u32,
                                arcs : *mut xcb_arc_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_poly_arc (c : *mut ffi::base::xcb_connection_t,
                        drawable :  xcb_drawable_t,
                        gc :  xcb_gcontext_t,
                        arcs_len :  u32,
                        arcs : *mut xcb_arc_t) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_fill_poly_sizeof (_buffer :  *mut c_void,
                      points_len :   u32) -> c_int;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_fill_poly_checked (c : *mut ffi::base::xcb_connection_t,
                                 drawable :  xcb_drawable_t,
                                 gc :  xcb_gcontext_t,
                                 shape :  u8,
                                 coordinate_mode :  u8,
                                 points_len :  u32,
                                 points : *mut xcb_point_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_fill_poly (c : *mut ffi::base::xcb_connection_t,
                         drawable :  xcb_drawable_t,
                         gc :  xcb_gcontext_t,
                         shape :  u8,
                         coordinate_mode :  u8,
                         points_len :  u32,
                         points : *mut xcb_point_t) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_poly_fill_rectangle_sizeof (_buffer :  *mut c_void,
                                rectangles_len :  u32) -> c_int;

/// Fills rectangles
///
/// # Arguments
///  * `drawable` The drawable (Window or Pixmap) to draw on.
///  * `gc` The graphics context to use.
///
///       The following graphics context components are used: function, plane-mask,
///       fill-style, subwindow-mode, clip-x-origin, clip-y-origin, and clip-mask.
///
///       The following graphics context mode-dependent components are used:
///       foreground, background, tile, stipple, tile-stipple-x-origin, and
///       tile-stipple-y-origin.
///  * `rectangles_len` The number of `xcb_rectangle_t` structures in `rectangles`.
///  * `rectangles` The rectangles to fill.
///
/// Fills the specified rectangle(s) in the order listed in the array. For any
/// given rectangle, each pixel is not drawn more than once. If rectangles
/// intersect, the intersecting pixels are drawn multiple times.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_poly_fill_rectangle_checked (c : *mut ffi::base::xcb_connection_t,
                                           drawable :  xcb_drawable_t,
                                           gc :  xcb_gcontext_t,
                                           rectangles_len :  u32,
                                           rectangles : *mut xcb_rectangle_t) -> ffi::base::xcb_void_cookie_t;

/// Fills rectangles
///
/// # Arguments
///  * `drawable` The drawable (Window or Pixmap) to draw on.
///  * `gc` The graphics context to use.
///
///       The following graphics context components are used: function, plane-mask,
///       fill-style, subwindow-mode, clip-x-origin, clip-y-origin, and clip-mask.
///
///       The following graphics context mode-dependent components are used:
///       foreground, background, tile, stipple, tile-stipple-x-origin, and
///       tile-stipple-y-origin.
///  * `rectangles_len` The number of `xcb_rectangle_t` structures in `rectangles`.
///  * `rectangles` The rectangles to fill.
///
/// Fills the specified rectangle(s) in the order listed in the array. For any
/// given rectangle, each pixel is not drawn more than once. If rectangles
/// intersect, the intersecting pixels are drawn multiple times.
///
pub fn xcb_poly_fill_rectangle (c : *mut ffi::base::xcb_connection_t,
                                   drawable :  xcb_drawable_t,
                                   gc :  xcb_gcontext_t,
                                   rectangles_len :  u32,
                                   rectangles : *mut xcb_rectangle_t) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_poly_fill_arc_sizeof (_buffer :  *mut c_void,
                          arcs_len :     u32) -> c_int;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_poly_fill_arc_checked (c : *mut ffi::base::xcb_connection_t,
                                     drawable :  xcb_drawable_t,
                                     gc :  xcb_gcontext_t,
                                     arcs_len :  u32,
                                     arcs : *mut xcb_arc_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_poly_fill_arc (c : *mut ffi::base::xcb_connection_t,
                             drawable :  xcb_drawable_t,
                             gc :  xcb_gcontext_t,
                             arcs_len :  u32,
                             arcs : *mut xcb_arc_t) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_put_image_sizeof (_buffer :  *mut c_void,
                      data_len :     u32) -> c_int;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_put_image_checked (c : *mut ffi::base::xcb_connection_t,
                                 format :  u8,
                                 drawable :  xcb_drawable_t,
                                 gc :  xcb_gcontext_t,
                                 width :  u16,
                                 height :  u16,
                                 dst_x :  i16,
                                 dst_y :  i16,
                                 left_pad :  u8,
                                 depth :  u8,
                                 data_len :  u32,
                                 data : *mut u8) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_put_image (c : *mut ffi::base::xcb_connection_t,
                         format :  u8,
                         drawable :  xcb_drawable_t,
                         gc :  xcb_gcontext_t,
                         width :  u16,
                         height :  u16,
                         dst_x :  i16,
                         dst_y :  i16,
                         left_pad :  u8,
                         depth :  u8,
                         data_len :  u32,
                         data : *mut u8) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_get_image_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
///
pub fn xcb_get_image (c : *mut ffi::base::xcb_connection_t,
                         format :  u8,
                         drawable :  xcb_drawable_t,
                         x :  i16,
                         y :  i16,
                         width :  u16,
                         height :  u16,
                         plane_mask :  u32) -> xcb_get_image_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_get_image_unchecked (c : *mut ffi::base::xcb_connection_t,
                                   format :  u8,
                                   drawable :  xcb_drawable_t,
                                   x :  i16,
                                   y :  i16,
                                   width :  u16,
                                   height :  u16,
                                   plane_mask :  u32) -> xcb_get_image_cookie_t;

pub fn xcb_get_image_data (R : *mut xcb_get_image_reply_t) -> *mut u8;


pub fn xcb_get_image_data_length (R : *mut xcb_get_image_reply_t) -> c_int;


pub fn xcb_get_image_data_end (R : *mut xcb_get_image_reply_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_get_image_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_get_image_reply (c : *mut ffi::base::xcb_connection_t,
                               cookie : xcb_get_image_cookie_t,
                               e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_get_image_reply_t;

pub fn xcb_poly_text_8_sizeof (_buffer :  *mut c_void,
                        items_len :    u32) -> c_int;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_poly_text_8_checked (c : *mut ffi::base::xcb_connection_t,
                                   drawable :  xcb_drawable_t,
                                   gc :  xcb_gcontext_t,
                                   x :  i16,
                                   y :  i16,
                                   items_len :  u32,
                                   items : *mut u8) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_poly_text_8 (c : *mut ffi::base::xcb_connection_t,
                           drawable :  xcb_drawable_t,
                           gc :  xcb_gcontext_t,
                           x :  i16,
                           y :  i16,
                           items_len :  u32,
                           items : *mut u8) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_poly_text_16_sizeof (_buffer :  *mut c_void,
                         items_len :    u32) -> c_int;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_poly_text_16_checked (c : *mut ffi::base::xcb_connection_t,
                                    drawable :  xcb_drawable_t,
                                    gc :  xcb_gcontext_t,
                                    x :  i16,
                                    y :  i16,
                                    items_len :  u32,
                                    items : *mut u8) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_poly_text_16 (c : *mut ffi::base::xcb_connection_t,
                            drawable :  xcb_drawable_t,
                            gc :  xcb_gcontext_t,
                            x :  i16,
                            y :  i16,
                            items_len :  u32,
                            items : *mut u8) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_image_text_8_sizeof (_buffer :  *mut c_void) -> c_int;

/// Draws text
///
/// # Arguments
///  * `string_len` The length of the `string`. Note that this parameter limited by 255 due to
///       using 8 bits!
///  * `drawable` The drawable (Window or Pixmap) to draw text on.
///  * `gc` The graphics context to use.
///
///       The following graphics context components are used: plane-mask, foreground,
///       background, font, subwindow-mode, clip-x-origin, clip-y-origin, and clip-mask.
///  * `x` The x coordinate of the first character, relative to the origin of `drawable`.
///  * `y` The y coordinate of the first character, relative to the origin of `drawable`.
///  * `string` The string to draw. Only the first 255 characters are relevant due to the data
///       type of `string_len`.
///
/// Fills the destination rectangle with the background pixel from `gc`, then
/// paints the text with the foreground pixel from `gc`. The upper-left corner of
/// the filled rectangle is at [x, y - font-ascent]. The width is overall-width,
/// the height is font-ascent + font-descent. The overall-width, font-ascent and
/// font-descent are as returned by `xcb_query_text_extents` (TODO).
///
/// Note that using X core fonts is deprecated (but still supported) in favor of
/// client-side rendering using Xft.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_image_text_8_checked (c : *mut ffi::base::xcb_connection_t,
                                    string_len :  u8,
                                    drawable :  xcb_drawable_t,
                                    gc :  xcb_gcontext_t,
                                    x :  i16,
                                    y :  i16,
                                    string : *mut c_char) -> ffi::base::xcb_void_cookie_t;

/// Draws text
///
/// # Arguments
///  * `string_len` The length of the `string`. Note that this parameter limited by 255 due to
///       using 8 bits!
///  * `drawable` The drawable (Window or Pixmap) to draw text on.
///  * `gc` The graphics context to use.
///
///       The following graphics context components are used: plane-mask, foreground,
///       background, font, subwindow-mode, clip-x-origin, clip-y-origin, and clip-mask.
///  * `x` The x coordinate of the first character, relative to the origin of `drawable`.
///  * `y` The y coordinate of the first character, relative to the origin of `drawable`.
///  * `string` The string to draw. Only the first 255 characters are relevant due to the data
///       type of `string_len`.
///
/// Fills the destination rectangle with the background pixel from `gc`, then
/// paints the text with the foreground pixel from `gc`. The upper-left corner of
/// the filled rectangle is at [x, y - font-ascent]. The width is overall-width,
/// the height is font-ascent + font-descent. The overall-width, font-ascent and
/// font-descent are as returned by `xcb_query_text_extents` (TODO).
///
/// Note that using X core fonts is deprecated (but still supported) in favor of
/// client-side rendering using Xft.
///
pub fn xcb_image_text_8 (c : *mut ffi::base::xcb_connection_t,
                            string_len :  u8,
                            drawable :  xcb_drawable_t,
                            gc :  xcb_gcontext_t,
                            x :  i16,
                            y :  i16,
                            string : *mut c_char) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_image_text_16_sizeof (_buffer :  *mut c_void) -> c_int;

/// Draws text
///
/// # Arguments
///  * `string_len` The length of the `string` in characters. Note that this parameter limited by
///       255 due to using 8 bits!
///  * `drawable` The drawable (Window or Pixmap) to draw text on.
///  * `gc` The graphics context to use.
///
///       The following graphics context components are used: plane-mask, foreground,
///       background, font, subwindow-mode, clip-x-origin, clip-y-origin, and clip-mask.
///  * `x` The x coordinate of the first character, relative to the origin of `drawable`.
///  * `y` The y coordinate of the first character, relative to the origin of `drawable`.
///  * `string` The string to draw. Only the first 255 characters are relevant due to the data
///       type of `string_len`. Every character uses 2 bytes (hence the 16 in this
///       request's name).
///
/// Fills the destination rectangle with the background pixel from `gc`, then
/// paints the text with the foreground pixel from `gc`. The upper-left corner of
/// the filled rectangle is at [x, y - font-ascent]. The width is overall-width,
/// the height is font-ascent + font-descent. The overall-width, font-ascent and
/// font-descent are as returned by `xcb_query_text_extents` (TODO).
///
/// Note that using X core fonts is deprecated (but still supported) in favor of
/// client-side rendering using Xft.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_image_text_16_checked (c : *mut ffi::base::xcb_connection_t,
                                     string_len :  u8,
                                     drawable :  xcb_drawable_t,
                                     gc :  xcb_gcontext_t,
                                     x :  i16,
                                     y :  i16,
                                     string : *mut xcb_char2b_t) -> ffi::base::xcb_void_cookie_t;

/// Draws text
///
/// # Arguments
///  * `string_len` The length of the `string` in characters. Note that this parameter limited by
///       255 due to using 8 bits!
///  * `drawable` The drawable (Window or Pixmap) to draw text on.
///  * `gc` The graphics context to use.
///
///       The following graphics context components are used: plane-mask, foreground,
///       background, font, subwindow-mode, clip-x-origin, clip-y-origin, and clip-mask.
///  * `x` The x coordinate of the first character, relative to the origin of `drawable`.
///  * `y` The y coordinate of the first character, relative to the origin of `drawable`.
///  * `string` The string to draw. Only the first 255 characters are relevant due to the data
///       type of `string_len`. Every character uses 2 bytes (hence the 16 in this
///       request's name).
///
/// Fills the destination rectangle with the background pixel from `gc`, then
/// paints the text with the foreground pixel from `gc`. The upper-left corner of
/// the filled rectangle is at [x, y - font-ascent]. The width is overall-width,
/// the height is font-ascent + font-descent. The overall-width, font-ascent and
/// font-descent are as returned by `xcb_query_text_extents` (TODO).
///
/// Note that using X core fonts is deprecated (but still supported) in favor of
/// client-side rendering using Xft.
///
pub fn xcb_image_text_16 (c : *mut ffi::base::xcb_connection_t,
                             string_len :  u8,
                             drawable :  xcb_drawable_t,
                             gc :  xcb_gcontext_t,
                             x :  i16,
                             y :  i16,
                             string : *mut xcb_char2b_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_create_colormap_checked (c : *mut ffi::base::xcb_connection_t,
                                       alloc :  u8,
                                       mid :  xcb_colormap_t,
                                       window :  xcb_window_t,
                                       visual :  xcb_visualid_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_create_colormap (c : *mut ffi::base::xcb_connection_t,
                               alloc :  u8,
                               mid :  xcb_colormap_t,
                               window :  xcb_window_t,
                               visual :  xcb_visualid_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_free_colormap_checked (c : *mut ffi::base::xcb_connection_t,
                                     cmap :  xcb_colormap_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_free_colormap (c : *mut ffi::base::xcb_connection_t,
                             cmap :  xcb_colormap_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_copy_colormap_and_free_checked (c : *mut ffi::base::xcb_connection_t,
                                              mid :  xcb_colormap_t,
                                              src_cmap :  xcb_colormap_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_copy_colormap_and_free (c : *mut ffi::base::xcb_connection_t,
                                      mid :  xcb_colormap_t,
                                      src_cmap :  xcb_colormap_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_install_colormap_checked (c : *mut ffi::base::xcb_connection_t,
                                        cmap :  xcb_colormap_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_install_colormap (c : *mut ffi::base::xcb_connection_t,
                                cmap :  xcb_colormap_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_uninstall_colormap_checked (c : *mut ffi::base::xcb_connection_t,
                                          cmap :  xcb_colormap_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_uninstall_colormap (c : *mut ffi::base::xcb_connection_t,
                                  cmap :  xcb_colormap_t) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_list_installed_colormaps_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
///
pub fn xcb_list_installed_colormaps (c : *mut ffi::base::xcb_connection_t,
                                        window :  xcb_window_t) -> xcb_list_installed_colormaps_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_list_installed_colormaps_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                  window :  xcb_window_t) -> xcb_list_installed_colormaps_cookie_t;

pub fn xcb_list_installed_colormaps_cmaps (R : *mut xcb_list_installed_colormaps_reply_t) -> *mut xcb_colormap_t;


pub fn xcb_list_installed_colormaps_cmaps_length (R : *mut xcb_list_installed_colormaps_reply_t) -> c_int;


pub fn xcb_list_installed_colormaps_cmaps_end (R : *mut xcb_list_installed_colormaps_reply_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_list_installed_colormaps_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_list_installed_colormaps_reply (c : *mut ffi::base::xcb_connection_t,
                                              cookie : xcb_list_installed_colormaps_cookie_t,
                                              e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_list_installed_colormaps_reply_t;

/// Allocate a color
///
/// # Arguments
///  * `cmap` TODO
///  * `red` The red value of your color.
///  * `green` The green value of your color.
///  * `blue` The blue value of your color.
///
/// Allocates a read-only colormap entry corresponding to the closest RGB value
/// supported by the hardware. If you are using TrueColor, you can take a shortcut
/// and directly calculate the color pixel value to avoid the round trip. But, for
/// example, on 16-bit color setups (VNC), you can easily get the closest supported
/// RGB value to the RGB value you are specifying.
///
pub fn xcb_alloc_color (c : *mut ffi::base::xcb_connection_t,
                           cmap :  xcb_colormap_t,
                           red :  u16,
                           green :  u16,
                           blue :  u16) -> xcb_alloc_color_cookie_t;

/// Allocate a color
///
/// # Arguments
///  * `cmap` TODO
///  * `red` The red value of your color.
///  * `green` The green value of your color.
///  * `blue` The blue value of your color.
///
/// Allocates a read-only colormap entry corresponding to the closest RGB value
/// supported by the hardware. If you are using TrueColor, you can take a shortcut
/// and directly calculate the color pixel value to avoid the round trip. But, for
/// example, on 16-bit color setups (VNC), you can easily get the closest supported
/// RGB value to the RGB value you are specifying.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_alloc_color_unchecked (c : *mut ffi::base::xcb_connection_t,
                                     cmap :  xcb_colormap_t,
                                     red :  u16,
                                     green :  u16,
                                     blue :  u16) -> xcb_alloc_color_cookie_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_alloc_color_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_alloc_color_reply (c : *mut ffi::base::xcb_connection_t,
                                 cookie : xcb_alloc_color_cookie_t,
                                 e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_alloc_color_reply_t;

pub fn xcb_alloc_named_color_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
///
pub fn xcb_alloc_named_color (c : *mut ffi::base::xcb_connection_t,
                                 cmap :  xcb_colormap_t,
                                 name_len :  u16,
                                 name : *mut c_char) -> xcb_alloc_named_color_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_alloc_named_color_unchecked (c : *mut ffi::base::xcb_connection_t,
                                           cmap :  xcb_colormap_t,
                                           name_len :  u16,
                                           name : *mut c_char) -> xcb_alloc_named_color_cookie_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_alloc_named_color_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_alloc_named_color_reply (c : *mut ffi::base::xcb_connection_t,
                                       cookie : xcb_alloc_named_color_cookie_t,
                                       e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_alloc_named_color_reply_t;

pub fn xcb_alloc_color_cells_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
///
pub fn xcb_alloc_color_cells (c : *mut ffi::base::xcb_connection_t,
                                 contiguous :  u8,
                                 cmap :  xcb_colormap_t,
                                 colors :  u16,
                                 planes :  u16) -> xcb_alloc_color_cells_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_alloc_color_cells_unchecked (c : *mut ffi::base::xcb_connection_t,
                                           contiguous :  u8,
                                           cmap :  xcb_colormap_t,
                                           colors :  u16,
                                           planes :  u16) -> xcb_alloc_color_cells_cookie_t;

pub fn xcb_alloc_color_cells_pixels (R : *mut xcb_alloc_color_cells_reply_t) -> *mut u32;


pub fn xcb_alloc_color_cells_pixels_length (R : *mut xcb_alloc_color_cells_reply_t) -> c_int;


pub fn xcb_alloc_color_cells_pixels_end (R : *mut xcb_alloc_color_cells_reply_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_alloc_color_cells_masks (R : *mut xcb_alloc_color_cells_reply_t) -> *mut u32;


pub fn xcb_alloc_color_cells_masks_length (R : *mut xcb_alloc_color_cells_reply_t) -> c_int;


pub fn xcb_alloc_color_cells_masks_end (R : *mut xcb_alloc_color_cells_reply_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_alloc_color_cells_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_alloc_color_cells_reply (c : *mut ffi::base::xcb_connection_t,
                                       cookie : xcb_alloc_color_cells_cookie_t,
                                       e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_alloc_color_cells_reply_t;

pub fn xcb_alloc_color_planes_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
///
pub fn xcb_alloc_color_planes (c : *mut ffi::base::xcb_connection_t,
                                  contiguous :  u8,
                                  cmap :  xcb_colormap_t,
                                  colors :  u16,
                                  reds :  u16,
                                  greens :  u16,
                                  blues :  u16) -> xcb_alloc_color_planes_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_alloc_color_planes_unchecked (c : *mut ffi::base::xcb_connection_t,
                                            contiguous :  u8,
                                            cmap :  xcb_colormap_t,
                                            colors :  u16,
                                            reds :  u16,
                                            greens :  u16,
                                            blues :  u16) -> xcb_alloc_color_planes_cookie_t;

pub fn xcb_alloc_color_planes_pixels (R : *mut xcb_alloc_color_planes_reply_t) -> *mut u32;


pub fn xcb_alloc_color_planes_pixels_length (R : *mut xcb_alloc_color_planes_reply_t) -> c_int;


pub fn xcb_alloc_color_planes_pixels_end (R : *mut xcb_alloc_color_planes_reply_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_alloc_color_planes_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_alloc_color_planes_reply (c : *mut ffi::base::xcb_connection_t,
                                        cookie : xcb_alloc_color_planes_cookie_t,
                                        e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_alloc_color_planes_reply_t;

pub fn xcb_free_colors_sizeof (_buffer :  *mut c_void,
                        pixels_len :   u32) -> c_int;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_free_colors_checked (c : *mut ffi::base::xcb_connection_t,
                                   cmap :  xcb_colormap_t,
                                   plane_mask :  u32,
                                   pixels_len :  u32,
                                   pixels : *mut u32) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_free_colors (c : *mut ffi::base::xcb_connection_t,
                           cmap :  xcb_colormap_t,
                           plane_mask :  u32,
                           pixels_len :  u32,
                           pixels : *mut u32) -> ffi::base::xcb_void_cookie_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_coloritem_t)
///
pub fn xcb_coloritem_next (i:*mut xcb_coloritem_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_coloritem_end (i:xcb_coloritem_iterator_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_store_colors_sizeof (_buffer :  *mut c_void,
                         items_len :    u32) -> c_int;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_store_colors_checked (c : *mut ffi::base::xcb_connection_t,
                                    cmap :  xcb_colormap_t,
                                    items_len :  u32,
                                    items : *mut xcb_coloritem_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_store_colors (c : *mut ffi::base::xcb_connection_t,
                            cmap :  xcb_colormap_t,
                            items_len :  u32,
                            items : *mut xcb_coloritem_t) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_store_named_color_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_store_named_color_checked (c : *mut ffi::base::xcb_connection_t,
                                         flags :  u8,
                                         cmap :  xcb_colormap_t,
                                         pixel :  u32,
                                         name_len :  u16,
                                         name : *mut c_char) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_store_named_color (c : *mut ffi::base::xcb_connection_t,
                                 flags :  u8,
                                 cmap :  xcb_colormap_t,
                                 pixel :  u32,
                                 name_len :  u16,
                                 name : *mut c_char) -> ffi::base::xcb_void_cookie_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_rgb_t)
///
pub fn xcb_rgb_next (i:*mut xcb_rgb_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_rgb_end (i:xcb_rgb_iterator_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_query_colors_sizeof (_buffer :  *mut c_void,
                         pixels_len :   u32) -> c_int;

/// Delivers a request to the X server.
///
pub fn xcb_query_colors (c : *mut ffi::base::xcb_connection_t,
                            cmap :  xcb_colormap_t,
                            pixels_len :  u32,
                            pixels : *mut u32) -> xcb_query_colors_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_query_colors_unchecked (c : *mut ffi::base::xcb_connection_t,
                                      cmap :  xcb_colormap_t,
                                      pixels_len :  u32,
                                      pixels : *mut u32) -> xcb_query_colors_cookie_t;

pub fn xcb_query_colors_colors (R : *mut xcb_query_colors_reply_t) -> *mut xcb_rgb_t;


pub fn xcb_query_colors_colors_length (R : *mut xcb_query_colors_reply_t) -> c_int;

pub fn xcb_query_colors_colors_iterator (R : *mut xcb_query_colors_reply_t) -> xcb_rgb_iterator_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_query_colors_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_query_colors_reply (c : *mut ffi::base::xcb_connection_t,
                                  cookie : xcb_query_colors_cookie_t,
                                  e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_query_colors_reply_t;

pub fn xcb_lookup_color_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
///
pub fn xcb_lookup_color (c : *mut ffi::base::xcb_connection_t,
                            cmap :  xcb_colormap_t,
                            name_len :  u16,
                            name : *mut c_char) -> xcb_lookup_color_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_lookup_color_unchecked (c : *mut ffi::base::xcb_connection_t,
                                      cmap :  xcb_colormap_t,
                                      name_len :  u16,
                                      name : *mut c_char) -> xcb_lookup_color_cookie_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_lookup_color_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_lookup_color_reply (c : *mut ffi::base::xcb_connection_t,
                                  cookie : xcb_lookup_color_cookie_t,
                                  e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_lookup_color_reply_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_create_cursor_checked (c : *mut ffi::base::xcb_connection_t,
                                     cid :  xcb_cursor_t,
                                     source :  xcb_pixmap_t,
                                     mask :  xcb_pixmap_t,
                                     fore_red :  u16,
                                     fore_green :  u16,
                                     fore_blue :  u16,
                                     back_red :  u16,
                                     back_green :  u16,
                                     back_blue :  u16,
                                     x :  u16,
                                     y :  u16) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_create_cursor (c : *mut ffi::base::xcb_connection_t,
                             cid :  xcb_cursor_t,
                             source :  xcb_pixmap_t,
                             mask :  xcb_pixmap_t,
                             fore_red :  u16,
                             fore_green :  u16,
                             fore_blue :  u16,
                             back_red :  u16,
                             back_green :  u16,
                             back_blue :  u16,
                             x :  u16,
                             y :  u16) -> ffi::base::xcb_void_cookie_t;

/// create cursor
///
/// # Arguments
///  * `cid` The ID with which you will refer to the cursor, created by `xcb_generate_id`.
///  * `source_font` In which font to look for the cursor glyph.
///  * `mask_font` In which font to look for the mask glyph.
///  * `source_char` The glyph of `source_font` to use.
///  * `mask_char` The glyph of `mask_font` to use as a mask: Pixels which are set to 1 define
///       which source pixels are displayed. All pixels which are set to 0 are not
///       displayed.
///  * `fore_red` The red value of the foreground color.
///  * `fore_green` The green value of the foreground color.
///  * `fore_blue` The blue value of the foreground color.
///  * `back_red` The red value of the background color.
///  * `back_green` The green value of the background color.
///  * `back_blue` The blue value of the background color.
///
/// Creates a cursor from a font glyph. X provides a set of standard cursor shapes
/// in a special font named cursor. Applications are encouraged to use this
/// interface for their cursors because the font can be customized for the
/// individual display type.
///
/// All pixels which are set to 1 in the source will use the foreground color (as
/// specified by `fore_red`, `fore_green` and `fore_blue`). All pixels set to 0
/// will use the background color (as specified by `back_red`, `back_green` and
/// `back_blue`).
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_create_glyph_cursor_checked (c : *mut ffi::base::xcb_connection_t,
                                           cid :  xcb_cursor_t,
                                           source_font :  xcb_font_t,
                                           mask_font :  xcb_font_t,
                                           source_char :  u16,
                                           mask_char :  u16,
                                           fore_red :  u16,
                                           fore_green :  u16,
                                           fore_blue :  u16,
                                           back_red :  u16,
                                           back_green :  u16,
                                           back_blue :  u16) -> ffi::base::xcb_void_cookie_t;

/// create cursor
///
/// # Arguments
///  * `cid` The ID with which you will refer to the cursor, created by `xcb_generate_id`.
///  * `source_font` In which font to look for the cursor glyph.
///  * `mask_font` In which font to look for the mask glyph.
///  * `source_char` The glyph of `source_font` to use.
///  * `mask_char` The glyph of `mask_font` to use as a mask: Pixels which are set to 1 define
///       which source pixels are displayed. All pixels which are set to 0 are not
///       displayed.
///  * `fore_red` The red value of the foreground color.
///  * `fore_green` The green value of the foreground color.
///  * `fore_blue` The blue value of the foreground color.
///  * `back_red` The red value of the background color.
///  * `back_green` The green value of the background color.
///  * `back_blue` The blue value of the background color.
///
/// Creates a cursor from a font glyph. X provides a set of standard cursor shapes
/// in a special font named cursor. Applications are encouraged to use this
/// interface for their cursors because the font can be customized for the
/// individual display type.
///
/// All pixels which are set to 1 in the source will use the foreground color (as
/// specified by `fore_red`, `fore_green` and `fore_blue`). All pixels set to 0
/// will use the background color (as specified by `back_red`, `back_green` and
/// `back_blue`).
///
pub fn xcb_create_glyph_cursor (c : *mut ffi::base::xcb_connection_t,
                                   cid :  xcb_cursor_t,
                                   source_font :  xcb_font_t,
                                   mask_font :  xcb_font_t,
                                   source_char :  u16,
                                   mask_char :  u16,
                                   fore_red :  u16,
                                   fore_green :  u16,
                                   fore_blue :  u16,
                                   back_red :  u16,
                                   back_green :  u16,
                                   back_blue :  u16) -> ffi::base::xcb_void_cookie_t;

/// Deletes a cursor
///
/// # Arguments
///  * `cursor` The cursor to destroy.
///
/// Deletes the association between the cursor resource ID and the specified
/// cursor. The cursor is freed when no other resource references it.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_free_cursor_checked (c : *mut ffi::base::xcb_connection_t,
                                   cursor :  xcb_cursor_t) -> ffi::base::xcb_void_cookie_t;

/// Deletes a cursor
///
/// # Arguments
///  * `cursor` The cursor to destroy.
///
/// Deletes the association between the cursor resource ID and the specified
/// cursor. The cursor is freed when no other resource references it.
///
pub fn xcb_free_cursor (c : *mut ffi::base::xcb_connection_t,
                           cursor :  xcb_cursor_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_recolor_cursor_checked (c : *mut ffi::base::xcb_connection_t,
                                      cursor :  xcb_cursor_t,
                                      fore_red :  u16,
                                      fore_green :  u16,
                                      fore_blue :  u16,
                                      back_red :  u16,
                                      back_green :  u16,
                                      back_blue :  u16) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_recolor_cursor (c : *mut ffi::base::xcb_connection_t,
                              cursor :  xcb_cursor_t,
                              fore_red :  u16,
                              fore_green :  u16,
                              fore_blue :  u16,
                              back_red :  u16,
                              back_green :  u16,
                              back_blue :  u16) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_query_best_size (c : *mut ffi::base::xcb_connection_t,
                               class :  u8,
                               drawable :  xcb_drawable_t,
                               width :  u16,
                               height :  u16) -> xcb_query_best_size_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_query_best_size_unchecked (c : *mut ffi::base::xcb_connection_t,
                                         class :  u8,
                                         drawable :  xcb_drawable_t,
                                         width :  u16,
                                         height :  u16) -> xcb_query_best_size_cookie_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_query_best_size_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_query_best_size_reply (c : *mut ffi::base::xcb_connection_t,
                                     cookie : xcb_query_best_size_cookie_t,
                                     e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_query_best_size_reply_t;

pub fn xcb_query_extension_sizeof (_buffer :  *mut c_void) -> c_int;

/// check if extension is present
///
/// # Arguments
///  * `name_len` The length of `name` in bytes.
///  * `name` The name of the extension to query, for example "RANDR". This is case
///       sensitive!
///
/// Determines if the specified extension is present on this X11 server.
///
/// Every extension has a unique `major_opcode` to identify requests, the minor
/// opcodes and request formats are extension-specific. If the extension provides
/// events and errors, the `first_event` and `first_error` fields in the reply are
/// set accordingly.
///
/// There should rarely be a need to use this request directly, XCB provides the
/// `xcb_get_extension_data` function instead.
///
pub fn xcb_query_extension (c : *mut ffi::base::xcb_connection_t,
                               name_len :  u16,
                               name : *mut c_char) -> xcb_query_extension_cookie_t;

/// check if extension is present
///
/// # Arguments
///  * `name_len` The length of `name` in bytes.
///  * `name` The name of the extension to query, for example "RANDR". This is case
///       sensitive!
///
/// Determines if the specified extension is present on this X11 server.
///
/// Every extension has a unique `major_opcode` to identify requests, the minor
/// opcodes and request formats are extension-specific. If the extension provides
/// events and errors, the `first_event` and `first_error` fields in the reply are
/// set accordingly.
///
/// There should rarely be a need to use this request directly, XCB provides the
/// `xcb_get_extension_data` function instead.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_query_extension_unchecked (c : *mut ffi::base::xcb_connection_t,
                                         name_len :  u16,
                                         name : *mut c_char) -> xcb_query_extension_cookie_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_query_extension_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_query_extension_reply (c : *mut ffi::base::xcb_connection_t,
                                     cookie : xcb_query_extension_cookie_t,
                                     e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_query_extension_reply_t;

pub fn xcb_list_extensions_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
///
pub fn xcb_list_extensions (c : *mut ffi::base::xcb_connection_t) -> xcb_list_extensions_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_list_extensions_unchecked (c : *mut ffi::base::xcb_connection_t) -> xcb_list_extensions_cookie_t;


pub fn xcb_list_extensions_names_length (R : *mut xcb_list_extensions_reply_t) -> c_int;

pub fn xcb_list_extensions_names_iterator (R : *mut xcb_list_extensions_reply_t) -> xcb_str_iterator_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_list_extensions_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_list_extensions_reply (c : *mut ffi::base::xcb_connection_t,
                                     cookie : xcb_list_extensions_cookie_t,
                                     e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_list_extensions_reply_t;

pub fn xcb_change_keyboard_mapping_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_change_keyboard_mapping_checked (c : *mut ffi::base::xcb_connection_t,
                                               keycode_count :  u8,
                                               first_keycode :  xcb_keycode_t,
                                               keysyms_per_keycode :  u8,
                                               keysyms : *mut xcb_keysym_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_change_keyboard_mapping (c : *mut ffi::base::xcb_connection_t,
                                       keycode_count :  u8,
                                       first_keycode :  xcb_keycode_t,
                                       keysyms_per_keycode :  u8,
                                       keysyms : *mut xcb_keysym_t) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_get_keyboard_mapping_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
///
pub fn xcb_get_keyboard_mapping (c : *mut ffi::base::xcb_connection_t,
                                    first_keycode :  xcb_keycode_t,
                                    count :  u8) -> xcb_get_keyboard_mapping_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_get_keyboard_mapping_unchecked (c : *mut ffi::base::xcb_connection_t,
                                              first_keycode :  xcb_keycode_t,
                                              count :  u8) -> xcb_get_keyboard_mapping_cookie_t;

pub fn xcb_get_keyboard_mapping_keysyms (R : *mut xcb_get_keyboard_mapping_reply_t) -> *mut xcb_keysym_t;


pub fn xcb_get_keyboard_mapping_keysyms_length (R : *mut xcb_get_keyboard_mapping_reply_t) -> c_int;


pub fn xcb_get_keyboard_mapping_keysyms_end (R : *mut xcb_get_keyboard_mapping_reply_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_get_keyboard_mapping_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_get_keyboard_mapping_reply (c : *mut ffi::base::xcb_connection_t,
                                          cookie : xcb_get_keyboard_mapping_cookie_t,
                                          e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_get_keyboard_mapping_reply_t;

pub fn xcb_change_keyboard_control_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_change_keyboard_control_checked (c : *mut ffi::base::xcb_connection_t,
                                               value_mask :  u32,
                                               value_list : *mut u32) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_change_keyboard_control (c : *mut ffi::base::xcb_connection_t,
                                       value_mask :  u32,
                                       value_list : *mut u32) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_get_keyboard_control (c : *mut ffi::base::xcb_connection_t) -> xcb_get_keyboard_control_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_get_keyboard_control_unchecked (c : *mut ffi::base::xcb_connection_t) -> xcb_get_keyboard_control_cookie_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_get_keyboard_control_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_get_keyboard_control_reply (c : *mut ffi::base::xcb_connection_t,
                                          cookie : xcb_get_keyboard_control_cookie_t,
                                          e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_get_keyboard_control_reply_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_bell_checked (c : *mut ffi::base::xcb_connection_t,
                            percent :  i8) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_bell (c : *mut ffi::base::xcb_connection_t,
                    percent :  i8) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_change_pointer_control_checked (c : *mut ffi::base::xcb_connection_t,
                                              acceleration_numerator :  i16,
                                              acceleration_denominator :  i16,
                                              threshold :  i16,
                                              do_acceleration :  u8,
                                              do_threshold :  u8) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_change_pointer_control (c : *mut ffi::base::xcb_connection_t,
                                      acceleration_numerator :  i16,
                                      acceleration_denominator :  i16,
                                      threshold :  i16,
                                      do_acceleration :  u8,
                                      do_threshold :  u8) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_get_pointer_control (c : *mut ffi::base::xcb_connection_t) -> xcb_get_pointer_control_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_get_pointer_control_unchecked (c : *mut ffi::base::xcb_connection_t) -> xcb_get_pointer_control_cookie_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_get_pointer_control_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_get_pointer_control_reply (c : *mut ffi::base::xcb_connection_t,
                                         cookie : xcb_get_pointer_control_cookie_t,
                                         e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_get_pointer_control_reply_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_set_screen_saver_checked (c : *mut ffi::base::xcb_connection_t,
                                        timeout :  i16,
                                        interval :  i16,
                                        prefer_blanking :  u8,
                                        allow_exposures :  u8) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_set_screen_saver (c : *mut ffi::base::xcb_connection_t,
                                timeout :  i16,
                                interval :  i16,
                                prefer_blanking :  u8,
                                allow_exposures :  u8) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_get_screen_saver (c : *mut ffi::base::xcb_connection_t) -> xcb_get_screen_saver_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_get_screen_saver_unchecked (c : *mut ffi::base::xcb_connection_t) -> xcb_get_screen_saver_cookie_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_get_screen_saver_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_get_screen_saver_reply (c : *mut ffi::base::xcb_connection_t,
                                      cookie : xcb_get_screen_saver_cookie_t,
                                      e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_get_screen_saver_reply_t;

pub fn xcb_change_hosts_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_change_hosts_checked (c : *mut ffi::base::xcb_connection_t,
                                    mode :  u8,
                                    family :  u8,
                                    address_len :  u16,
                                    address : *mut u8) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_change_hosts (c : *mut ffi::base::xcb_connection_t,
                            mode :  u8,
                            family :  u8,
                            address_len :  u16,
                            address : *mut u8) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_host_sizeof (_buffer :  *mut c_void) -> c_int;

pub fn xcb_host_address (R : *mut xcb_host_t) -> *mut u8;


pub fn xcb_host_address_length (R : *mut xcb_host_t) -> c_int;


pub fn xcb_host_address_end (R : *mut xcb_host_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Get the next element of the iterator
///
/// Get the next element in the iterator. The member rem is
/// decreased by one. The member data points to the next
/// element. The member index is increased by sizeof(xcb_host_t)
///
pub fn xcb_host_next (i:*mut xcb_host_iterator_t) -> c_void;

///
/// Return the iterator pointing to the last element
///
/// Set the current element in the iterator to the last element.
/// The member rem is set to 0. The member data points to the
/// last element.
///
pub fn xcb_host_end (i:xcb_host_iterator_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_list_hosts_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
///
pub fn xcb_list_hosts (c : *mut ffi::base::xcb_connection_t) -> xcb_list_hosts_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_list_hosts_unchecked (c : *mut ffi::base::xcb_connection_t) -> xcb_list_hosts_cookie_t;


pub fn xcb_list_hosts_hosts_length (R : *mut xcb_list_hosts_reply_t) -> c_int;

pub fn xcb_list_hosts_hosts_iterator (R : *mut xcb_list_hosts_reply_t) -> xcb_host_iterator_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_list_hosts_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_list_hosts_reply (c : *mut ffi::base::xcb_connection_t,
                                cookie : xcb_list_hosts_cookie_t,
                                e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_list_hosts_reply_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_set_access_control_checked (c : *mut ffi::base::xcb_connection_t,
                                          mode :  u8) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_set_access_control (c : *mut ffi::base::xcb_connection_t,
                                  mode :  u8) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_set_close_down_mode_checked (c : *mut ffi::base::xcb_connection_t,
                                           mode :  u8) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_set_close_down_mode (c : *mut ffi::base::xcb_connection_t,
                                   mode :  u8) -> ffi::base::xcb_void_cookie_t;

/// kills a client
///
/// # Arguments
///  * `resource` Any resource belonging to the client (for example a Window), used to identify
///       the client connection.
///
///       The special value of `XCB_KILL_ALL_TEMPORARY`, the resources of all clients
///       that have terminated in `RetainTemporary` (TODO) are destroyed.
///
/// Forces a close down of the client that created the specified `resource`.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_kill_client_checked (c : *mut ffi::base::xcb_connection_t,
                                   resource :  u32) -> ffi::base::xcb_void_cookie_t;

/// kills a client
///
/// # Arguments
///  * `resource` Any resource belonging to the client (for example a Window), used to identify
///       the client connection.
///
///       The special value of `XCB_KILL_ALL_TEMPORARY`, the resources of all clients
///       that have terminated in `RetainTemporary` (TODO) are destroyed.
///
/// Forces a close down of the client that created the specified `resource`.
///
pub fn xcb_kill_client (c : *mut ffi::base::xcb_connection_t,
                           resource :  u32) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_rotate_properties_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_rotate_properties_checked (c : *mut ffi::base::xcb_connection_t,
                                         window :  xcb_window_t,
                                         atoms_len :  u16,
                                         delta :  i16,
                                         atoms : *mut xcb_atom_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_rotate_properties (c : *mut ffi::base::xcb_connection_t,
                                 window :  xcb_window_t,
                                 atoms_len :  u16,
                                 delta :  i16,
                                 atoms : *mut xcb_atom_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_force_screen_saver_checked (c : *mut ffi::base::xcb_connection_t,
                                          mode :  u8) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_force_screen_saver (c : *mut ffi::base::xcb_connection_t,
                                  mode :  u8) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_set_pointer_mapping_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
///
pub fn xcb_set_pointer_mapping (c : *mut ffi::base::xcb_connection_t,
                                   map_len :  u8,
                                   map : *mut u8) -> xcb_set_pointer_mapping_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_set_pointer_mapping_unchecked (c : *mut ffi::base::xcb_connection_t,
                                             map_len :  u8,
                                             map : *mut u8) -> xcb_set_pointer_mapping_cookie_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_set_pointer_mapping_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_set_pointer_mapping_reply (c : *mut ffi::base::xcb_connection_t,
                                         cookie : xcb_set_pointer_mapping_cookie_t,
                                         e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_set_pointer_mapping_reply_t;

pub fn xcb_get_pointer_mapping_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
///
pub fn xcb_get_pointer_mapping (c : *mut ffi::base::xcb_connection_t) -> xcb_get_pointer_mapping_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_get_pointer_mapping_unchecked (c : *mut ffi::base::xcb_connection_t) -> xcb_get_pointer_mapping_cookie_t;

pub fn xcb_get_pointer_mapping_map (R : *mut xcb_get_pointer_mapping_reply_t) -> *mut u8;


pub fn xcb_get_pointer_mapping_map_length (R : *mut xcb_get_pointer_mapping_reply_t) -> c_int;


pub fn xcb_get_pointer_mapping_map_end (R : *mut xcb_get_pointer_mapping_reply_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_get_pointer_mapping_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_get_pointer_mapping_reply (c : *mut ffi::base::xcb_connection_t,
                                         cookie : xcb_get_pointer_mapping_cookie_t,
                                         e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_get_pointer_mapping_reply_t;

pub fn xcb_set_modifier_mapping_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
///
pub fn xcb_set_modifier_mapping (c : *mut ffi::base::xcb_connection_t,
                                    keycodes_per_modifier :  u8,
                                    keycodes : *mut xcb_keycode_t) -> xcb_set_modifier_mapping_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_set_modifier_mapping_unchecked (c : *mut ffi::base::xcb_connection_t,
                                              keycodes_per_modifier :  u8,
                                              keycodes : *mut xcb_keycode_t) -> xcb_set_modifier_mapping_cookie_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_set_modifier_mapping_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_set_modifier_mapping_reply (c : *mut ffi::base::xcb_connection_t,
                                          cookie : xcb_set_modifier_mapping_cookie_t,
                                          e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_set_modifier_mapping_reply_t;

pub fn xcb_get_modifier_mapping_sizeof (_buffer :  *mut c_void) -> c_int;

/// Delivers a request to the X server.
///
pub fn xcb_get_modifier_mapping (c : *mut ffi::base::xcb_connection_t) -> xcb_get_modifier_mapping_cookie_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will causea reply to be generated. Any returned error will beplaced in the event queue.
pub fn xcb_get_modifier_mapping_unchecked (c : *mut ffi::base::xcb_connection_t) -> xcb_get_modifier_mapping_cookie_t;

pub fn xcb_get_modifier_mapping_keycodes (R : *mut xcb_get_modifier_mapping_reply_t) -> *mut xcb_keycode_t;


pub fn xcb_get_modifier_mapping_keycodes_length (R : *mut xcb_get_modifier_mapping_reply_t) -> c_int;


pub fn xcb_get_modifier_mapping_keycodes_end (R : *mut xcb_get_modifier_mapping_reply_t) -> ffi::base::xcb_generic_iterator_t;

///
/// Return the reply
/// `c`      The xcb_connection_t
/// `cookie` The cookie
/// `e`      The xcb_generic_error_t supplied
///
/// The parameter @p e supplied to this function must be NULL if
/// xcb_get_modifier_mapping_unchecked(). is used.
/// Otherwise, it stores the error if any.
///
/// The returned value must be freed by the caller using free().
///
pub fn xcb_get_modifier_mapping_reply (c : *mut ffi::base::xcb_connection_t,
                                          cookie : xcb_get_modifier_mapping_cookie_t,
                                          e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_get_modifier_mapping_reply_t;

/// Delivers a request to the X server.
/// This form can be used only if the request will not cause
/// a reply to be generated. Any returned error will besaved for handling by xcb_request_check().
pub fn xcb_no_operation_checked (c : *mut ffi::base::xcb_connection_t) -> ffi::base::xcb_void_cookie_t;

/// Delivers a request to the X server.
///
pub fn xcb_no_operation (c : *mut ffi::base::xcb_connection_t) -> ffi::base::xcb_void_cookie_t;
}

