/*
 * This file generated automatically from xproto.xml by r_client.py.
 * Edit at your peril.
 */

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
use ffi::xproto::*;
use std::option::Option;
use std::iter::Iterator;

pub struct Char2b {pub base : base::Struct<xcb_char2b_t> }

pub type Char2bIterator = xcb_char2b_iterator_t;

pub type WindowIterator = xcb_window_iterator_t;

pub type PixmapIterator = xcb_pixmap_iterator_t;

pub type CursorIterator = xcb_cursor_iterator_t;

pub type FontIterator = xcb_font_iterator_t;

pub type GcontextIterator = xcb_gcontext_iterator_t;

pub type ColormapIterator = xcb_colormap_iterator_t;

pub type AtomIterator = xcb_atom_iterator_t;

pub type DrawableIterator = xcb_drawable_iterator_t;

pub type FontableIterator = xcb_fontable_iterator_t;

pub type VisualidIterator = xcb_visualid_iterator_t;

pub type TimestampIterator = xcb_timestamp_iterator_t;

pub type KeysymIterator = xcb_keysym_iterator_t;

pub type KeycodeIterator = xcb_keycode_iterator_t;

pub type ButtonIterator = xcb_button_iterator_t;

pub type PointIterator = xcb_point_iterator_t;

pub type RectangleIterator = xcb_rectangle_iterator_t;

pub type ArcIterator = xcb_arc_iterator_t;

pub type FormatIterator = xcb_format_iterator_t;


pub type xcb_visual_class_t = c_uint;//{
    pub const XCB_VISUAL_CLASS_STATIC_GRAY : xcb_visual_class_t = 0;
    pub const XCB_VISUAL_CLASS_GRAY_SCALE : xcb_visual_class_t = 1;
    pub const XCB_VISUAL_CLASS_STATIC_COLOR : xcb_visual_class_t = 2;
    pub const XCB_VISUAL_CLASS_PSEUDO_COLOR : xcb_visual_class_t = 3;
    pub const XCB_VISUAL_CLASS_TRUE_COLOR : xcb_visual_class_t = 4;
    pub const XCB_VISUAL_CLASS_DIRECT_COLOR : xcb_visual_class_t = 5;
//}
pub struct Visualtype {pub base : base::Struct<xcb_visualtype_t> }

pub type VisualtypeIterator = xcb_visualtype_iterator_t;

pub type DepthIterator = xcb_depth_iterator_t;


pub type xcb_event_mask_t = c_uint;//{
    pub const XCB_EVENT_MASK_NO_EVENT : xcb_event_mask_t = 0;
    pub const XCB_EVENT_MASK_KEY_PRESS : xcb_event_mask_t = 1;
    pub const XCB_EVENT_MASK_KEY_RELEASE : xcb_event_mask_t = 2;
    pub const XCB_EVENT_MASK_BUTTON_PRESS : xcb_event_mask_t = 4;
    pub const XCB_EVENT_MASK_BUTTON_RELEASE : xcb_event_mask_t = 8;
    pub const XCB_EVENT_MASK_ENTER_WINDOW : xcb_event_mask_t = 16;
    pub const XCB_EVENT_MASK_LEAVE_WINDOW : xcb_event_mask_t = 32;
    pub const XCB_EVENT_MASK_POINTER_MOTION : xcb_event_mask_t = 64;
    pub const XCB_EVENT_MASK_POINTER_MOTION_HINT : xcb_event_mask_t = 128;
    pub const XCB_EVENT_MASK_BUTTON_1_MOTION : xcb_event_mask_t = 256;
    pub const XCB_EVENT_MASK_BUTTON_2_MOTION : xcb_event_mask_t = 512;
    pub const XCB_EVENT_MASK_BUTTON_3_MOTION : xcb_event_mask_t = 1024;
    pub const XCB_EVENT_MASK_BUTTON_4_MOTION : xcb_event_mask_t = 2048;
    pub const XCB_EVENT_MASK_BUTTON_5_MOTION : xcb_event_mask_t = 4096;
    pub const XCB_EVENT_MASK_BUTTON_MOTION : xcb_event_mask_t = 8192;
    pub const XCB_EVENT_MASK_KEYMAP_STATE : xcb_event_mask_t = 16384;
    pub const XCB_EVENT_MASK_EXPOSURE : xcb_event_mask_t = 32768;
    pub const XCB_EVENT_MASK_VISIBILITY_CHANGE : xcb_event_mask_t = 65536;
    pub const XCB_EVENT_MASK_STRUCTURE_NOTIFY : xcb_event_mask_t = 131072;
    pub const XCB_EVENT_MASK_RESIZE_REDIRECT : xcb_event_mask_t = 262144;
    pub const XCB_EVENT_MASK_SUBSTRUCTURE_NOTIFY : xcb_event_mask_t = 524288;
    pub const XCB_EVENT_MASK_SUBSTRUCTURE_REDIRECT : xcb_event_mask_t = 1048576;
    pub const XCB_EVENT_MASK_FOCUS_CHANGE : xcb_event_mask_t = 2097152;
    pub const XCB_EVENT_MASK_PROPERTY_CHANGE : xcb_event_mask_t = 4194304;
    pub const XCB_EVENT_MASK_COLOR_MAP_CHANGE : xcb_event_mask_t = 8388608;
    pub const XCB_EVENT_MASK_OWNER_GRAB_BUTTON : xcb_event_mask_t = 16777216;
//}

pub type xcb_backing_store_t = c_uint;//{
    pub const XCB_BACKING_STORE_NOT_USEFUL : xcb_backing_store_t = 0;
    pub const XCB_BACKING_STORE_WHEN_MAPPED : xcb_backing_store_t = 1;
    pub const XCB_BACKING_STORE_ALWAYS : xcb_backing_store_t = 2;
//}
pub type ScreenIterator = xcb_screen_iterator_t;

pub type SetupRequestIterator = xcb_setup_request_iterator_t;

pub type SetupFailedIterator = xcb_setup_failed_iterator_t;

pub type SetupAuthenticateIterator = xcb_setup_authenticate_iterator_t;


pub type xcb_image_order_t = c_uint;//{
    pub const XCB_IMAGE_ORDER_LSB_FIRST : xcb_image_order_t = 0;
    pub const XCB_IMAGE_ORDER_MSB_FIRST : xcb_image_order_t = 1;
//}

pub type xcb_mod_mask_t = c_uint;//{
    pub const XCB_MOD_MASK_SHIFT : xcb_mod_mask_t = 1;
    pub const XCB_MOD_MASK_LOCK : xcb_mod_mask_t = 2;
    pub const XCB_MOD_MASK_CONTROL : xcb_mod_mask_t = 4;
    pub const XCB_MOD_MASK_1 : xcb_mod_mask_t = 8;
    pub const XCB_MOD_MASK_2 : xcb_mod_mask_t = 16;
    pub const XCB_MOD_MASK_3 : xcb_mod_mask_t = 32;
    pub const XCB_MOD_MASK_4 : xcb_mod_mask_t = 64;
    pub const XCB_MOD_MASK_5 : xcb_mod_mask_t = 128;
    pub const XCB_MOD_MASK_ANY : xcb_mod_mask_t = 32768;
//}

pub type xcb_key_but_mask_t = c_uint;//{
    pub const XCB_KEY_BUT_MASK_SHIFT : xcb_key_but_mask_t = 1;
    pub const XCB_KEY_BUT_MASK_LOCK : xcb_key_but_mask_t = 2;
    pub const XCB_KEY_BUT_MASK_CONTROL : xcb_key_but_mask_t = 4;
    pub const XCB_KEY_BUT_MASK_MOD_1 : xcb_key_but_mask_t = 8;
    pub const XCB_KEY_BUT_MASK_MOD_2 : xcb_key_but_mask_t = 16;
    pub const XCB_KEY_BUT_MASK_MOD_3 : xcb_key_but_mask_t = 32;
    pub const XCB_KEY_BUT_MASK_MOD_4 : xcb_key_but_mask_t = 64;
    pub const XCB_KEY_BUT_MASK_MOD_5 : xcb_key_but_mask_t = 128;
    pub const XCB_KEY_BUT_MASK_BUTTON_1 : xcb_key_but_mask_t = 256;
    pub const XCB_KEY_BUT_MASK_BUTTON_2 : xcb_key_but_mask_t = 512;
    pub const XCB_KEY_BUT_MASK_BUTTON_3 : xcb_key_but_mask_t = 1024;
    pub const XCB_KEY_BUT_MASK_BUTTON_4 : xcb_key_but_mask_t = 2048;
    pub const XCB_KEY_BUT_MASK_BUTTON_5 : xcb_key_but_mask_t = 4096;
//}

pub type xcb_window_enum_t = c_uint;//{
    pub const XCB_WINDOW_NONE : xcb_window_enum_t = 0;
//}
/** Opcode for xcb_key_press. */
pub const XCB_KEY_PRESS : u8 = 2;
pub struct KeyPressEvent {pub base : base::Event<xcb_key_press_event_t>}
/** Opcode for xcb_key_release. */
pub const XCB_KEY_RELEASE : u8 = 3;
pub struct KeyReleaseEvent {pub base : base::Event<xcb_key_release_event_t>}

pub type xcb_button_mask_t = c_uint;//{
    pub const XCB_BUTTON_MASK_1 : xcb_button_mask_t = 256;
    pub const XCB_BUTTON_MASK_2 : xcb_button_mask_t = 512;
    pub const XCB_BUTTON_MASK_3 : xcb_button_mask_t = 1024;
    pub const XCB_BUTTON_MASK_4 : xcb_button_mask_t = 2048;
    pub const XCB_BUTTON_MASK_5 : xcb_button_mask_t = 4096;
    pub const XCB_BUTTON_MASK_ANY : xcb_button_mask_t = 32768;
//}
/** Opcode for xcb_button_press. */
pub const XCB_BUTTON_PRESS : u8 = 4;
pub struct ButtonPressEvent {pub base : base::Event<xcb_button_press_event_t>}
/** Opcode for xcb_button_release. */
pub const XCB_BUTTON_RELEASE : u8 = 5;
pub struct ButtonReleaseEvent {pub base : base::Event<xcb_button_release_event_t>}

pub type xcb_motion_t = c_uint;//{
    pub const XCB_MOTION_NORMAL : xcb_motion_t = 0;
    pub const XCB_MOTION_HINT : xcb_motion_t = 1;
//}
/** Opcode for xcb_motion_notify. */
pub const XCB_MOTION_NOTIFY : u8 = 6;
pub struct MotionNotifyEvent {pub base : base::Event<xcb_motion_notify_event_t>}

pub type xcb_notify_detail_t = c_uint;//{
    pub const XCB_NOTIFY_DETAIL_ANCESTOR : xcb_notify_detail_t = 0;
    pub const XCB_NOTIFY_DETAIL_VIRTUAL : xcb_notify_detail_t = 1;
    pub const XCB_NOTIFY_DETAIL_INFERIOR : xcb_notify_detail_t = 2;
    pub const XCB_NOTIFY_DETAIL_NONLINEAR : xcb_notify_detail_t = 3;
    pub const XCB_NOTIFY_DETAIL_NONLINEAR_VIRTUAL : xcb_notify_detail_t = 4;
    pub const XCB_NOTIFY_DETAIL_POINTER : xcb_notify_detail_t = 5;
    pub const XCB_NOTIFY_DETAIL_POINTER_ROOT : xcb_notify_detail_t = 6;
    pub const XCB_NOTIFY_DETAIL_NONE : xcb_notify_detail_t = 7;
//}

pub type xcb_notify_mode_t = c_uint;//{
    pub const XCB_NOTIFY_MODE_NORMAL : xcb_notify_mode_t = 0;
    pub const XCB_NOTIFY_MODE_GRAB : xcb_notify_mode_t = 1;
    pub const XCB_NOTIFY_MODE_UNGRAB : xcb_notify_mode_t = 2;
    pub const XCB_NOTIFY_MODE_WHILE_GRABBED : xcb_notify_mode_t = 3;
//}
/** Opcode for xcb_enter_notify. */
pub const XCB_ENTER_NOTIFY : u8 = 7;
pub struct EnterNotifyEvent {pub base : base::Event<xcb_enter_notify_event_t>}
/** Opcode for xcb_leave_notify. */
pub const XCB_LEAVE_NOTIFY : u8 = 8;
pub struct LeaveNotifyEvent {pub base : base::Event<xcb_leave_notify_event_t>}
/** Opcode for xcb_focus_in. */
pub const XCB_FOCUS_IN : u8 = 9;
pub struct FocusInEvent {pub base : base::Event<xcb_focus_in_event_t>}
/** Opcode for xcb_focus_out. */
pub const XCB_FOCUS_OUT : u8 = 10;
pub struct FocusOutEvent {pub base : base::Event<xcb_focus_out_event_t>}
/** Opcode for xcb_keymap_notify. */
pub const XCB_KEYMAP_NOTIFY : u8 = 11;
pub struct KeymapNotifyEvent {pub base : base::Event<xcb_keymap_notify_event_t>}
/** Opcode for xcb_expose. */
pub const XCB_EXPOSE : u8 = 12;
pub struct ExposeEvent {pub base : base::Event<xcb_expose_event_t>}
/** Opcode for xcb_graphics_exposure. */
pub const XCB_GRAPHICS_EXPOSURE : u8 = 13;
pub struct GraphicsExposureEvent {pub base : base::Event<xcb_graphics_exposure_event_t>}
/** Opcode for xcb_no_exposure. */
pub const XCB_NO_EXPOSURE : u8 = 14;
pub struct NoExposureEvent {pub base : base::Event<xcb_no_exposure_event_t>}

pub type xcb_visibility_t = c_uint;//{
    pub const XCB_VISIBILITY_UNOBSCURED : xcb_visibility_t = 0;
    pub const XCB_VISIBILITY_PARTIALLY_OBSCURED : xcb_visibility_t = 1;
    pub const XCB_VISIBILITY_FULLY_OBSCURED : xcb_visibility_t = 2;
//}
/** Opcode for xcb_visibility_notify. */
pub const XCB_VISIBILITY_NOTIFY : u8 = 15;
pub struct VisibilityNotifyEvent {pub base : base::Event<xcb_visibility_notify_event_t>}
/** Opcode for xcb_create_notify. */
pub const XCB_CREATE_NOTIFY : u8 = 16;
pub struct CreateNotifyEvent {pub base : base::Event<xcb_create_notify_event_t>}
/** Opcode for xcb_destroy_notify. */
pub const XCB_DESTROY_NOTIFY : u8 = 17;
pub struct DestroyNotifyEvent {pub base : base::Event<xcb_destroy_notify_event_t>}
/** Opcode for xcb_unmap_notify. */
pub const XCB_UNMAP_NOTIFY : u8 = 18;
pub struct UnmapNotifyEvent {pub base : base::Event<xcb_unmap_notify_event_t>}
/** Opcode for xcb_map_notify. */
pub const XCB_MAP_NOTIFY : u8 = 19;
pub struct MapNotifyEvent {pub base : base::Event<xcb_map_notify_event_t>}
/** Opcode for xcb_map_request. */
pub const XCB_MAP_REQUEST : u8 = 20;
pub struct MapRequestEvent {pub base : base::Event<xcb_map_request_event_t>}
/** Opcode for xcb_reparent_notify. */
pub const XCB_REPARENT_NOTIFY : u8 = 21;
pub struct ReparentNotifyEvent {pub base : base::Event<xcb_reparent_notify_event_t>}
/** Opcode for xcb_configure_notify. */
pub const XCB_CONFIGURE_NOTIFY : u8 = 22;
pub struct ConfigureNotifyEvent {pub base : base::Event<xcb_configure_notify_event_t>}
/** Opcode for xcb_configure_request. */
pub const XCB_CONFIGURE_REQUEST : u8 = 23;
pub struct ConfigureRequestEvent {pub base : base::Event<xcb_configure_request_event_t>}
/** Opcode for xcb_gravity_notify. */
pub const XCB_GRAVITY_NOTIFY : u8 = 24;
pub struct GravityNotifyEvent {pub base : base::Event<xcb_gravity_notify_event_t>}
/** Opcode for xcb_resize_request. */
pub const XCB_RESIZE_REQUEST : u8 = 25;
pub struct ResizeRequestEvent {pub base : base::Event<xcb_resize_request_event_t>}

pub type xcb_place_t = c_uint;//{
    
/** The window is now on top of all siblings. */
    pub const XCB_PLACE_ON_TOP : xcb_place_t = 0;
    
/** The window is now below all siblings. */
    pub const XCB_PLACE_ON_BOTTOM : xcb_place_t = 1;
//}
/** Opcode for xcb_circulate_notify. */
pub const XCB_CIRCULATE_NOTIFY : u8 = 26;
pub struct CirculateNotifyEvent {pub base : base::Event<xcb_circulate_notify_event_t>}
/** Opcode for xcb_circulate_request. */
pub const XCB_CIRCULATE_REQUEST : u8 = 27;
pub struct CirculateRequestEvent {pub base : base::Event<xcb_circulate_request_event_t>}

pub type xcb_property_t = c_uint;//{
    pub const XCB_PROPERTY_NEW_VALUE : xcb_property_t = 0;
    pub const XCB_PROPERTY_DELETE : xcb_property_t = 1;
//}
/** Opcode for xcb_property_notify. */
pub const XCB_PROPERTY_NOTIFY : u8 = 28;
pub struct PropertyNotifyEvent {pub base : base::Event<xcb_property_notify_event_t>}
/** Opcode for xcb_selection_clear. */
pub const XCB_SELECTION_CLEAR : u8 = 29;
pub struct SelectionClearEvent {pub base : base::Event<xcb_selection_clear_event_t>}

pub type xcb_time_t = c_uint;//{
    pub const XCB_TIME_CURRENT_TIME : xcb_time_t = 0;
//}

pub type xcb_atom_enum_t = c_uint;//{
    pub const XCB_ATOM_NONE : xcb_atom_enum_t = 0;
    pub const XCB_ATOM_ANY : xcb_atom_enum_t = 0;
    pub const XCB_ATOM_PRIMARY : xcb_atom_enum_t = 1;
    pub const XCB_ATOM_SECONDARY : xcb_atom_enum_t = 2;
    pub const XCB_ATOM_ARC : xcb_atom_enum_t = 3;
    pub const XCB_ATOM_ATOM : xcb_atom_enum_t = 4;
    pub const XCB_ATOM_BITMAP : xcb_atom_enum_t = 5;
    pub const XCB_ATOM_CARDINAL : xcb_atom_enum_t = 6;
    pub const XCB_ATOM_COLORMAP : xcb_atom_enum_t = 7;
    pub const XCB_ATOM_CURSOR : xcb_atom_enum_t = 8;
    pub const XCB_ATOM_CUT_BUFFER0 : xcb_atom_enum_t = 9;
    pub const XCB_ATOM_CUT_BUFFER1 : xcb_atom_enum_t = 10;
    pub const XCB_ATOM_CUT_BUFFER2 : xcb_atom_enum_t = 11;
    pub const XCB_ATOM_CUT_BUFFER3 : xcb_atom_enum_t = 12;
    pub const XCB_ATOM_CUT_BUFFER4 : xcb_atom_enum_t = 13;
    pub const XCB_ATOM_CUT_BUFFER5 : xcb_atom_enum_t = 14;
    pub const XCB_ATOM_CUT_BUFFER6 : xcb_atom_enum_t = 15;
    pub const XCB_ATOM_CUT_BUFFER7 : xcb_atom_enum_t = 16;
    pub const XCB_ATOM_DRAWABLE : xcb_atom_enum_t = 17;
    pub const XCB_ATOM_FONT : xcb_atom_enum_t = 18;
    pub const XCB_ATOM_INTEGER : xcb_atom_enum_t = 19;
    pub const XCB_ATOM_PIXMAP : xcb_atom_enum_t = 20;
    pub const XCB_ATOM_POINT : xcb_atom_enum_t = 21;
    pub const XCB_ATOM_RECTANGLE : xcb_atom_enum_t = 22;
    pub const XCB_ATOM_RESOURCE_MANAGER : xcb_atom_enum_t = 23;
    pub const XCB_ATOM_RGB_COLOR_MAP : xcb_atom_enum_t = 24;
    pub const XCB_ATOM_RGB_BEST_MAP : xcb_atom_enum_t = 25;
    pub const XCB_ATOM_RGB_BLUE_MAP : xcb_atom_enum_t = 26;
    pub const XCB_ATOM_RGB_DEFAULT_MAP : xcb_atom_enum_t = 27;
    pub const XCB_ATOM_RGB_GRAY_MAP : xcb_atom_enum_t = 28;
    pub const XCB_ATOM_RGB_GREEN_MAP : xcb_atom_enum_t = 29;
    pub const XCB_ATOM_RGB_RED_MAP : xcb_atom_enum_t = 30;
    pub const XCB_ATOM_STRING : xcb_atom_enum_t = 31;
    pub const XCB_ATOM_VISUALID : xcb_atom_enum_t = 32;
    pub const XCB_ATOM_WINDOW : xcb_atom_enum_t = 33;
    pub const XCB_ATOM_WM_COMMAND : xcb_atom_enum_t = 34;
    pub const XCB_ATOM_WM_HINTS : xcb_atom_enum_t = 35;
    pub const XCB_ATOM_WM_CLIENT_MACHINE : xcb_atom_enum_t = 36;
    pub const XCB_ATOM_WM_ICON_NAME : xcb_atom_enum_t = 37;
    pub const XCB_ATOM_WM_ICON_SIZE : xcb_atom_enum_t = 38;
    pub const XCB_ATOM_WM_NAME : xcb_atom_enum_t = 39;
    pub const XCB_ATOM_WM_NORMAL_HINTS : xcb_atom_enum_t = 40;
    pub const XCB_ATOM_WM_SIZE_HINTS : xcb_atom_enum_t = 41;
    pub const XCB_ATOM_WM_ZOOM_HINTS : xcb_atom_enum_t = 42;
    pub const XCB_ATOM_MIN_SPACE : xcb_atom_enum_t = 43;
    pub const XCB_ATOM_NORM_SPACE : xcb_atom_enum_t = 44;
    pub const XCB_ATOM_MAX_SPACE : xcb_atom_enum_t = 45;
    pub const XCB_ATOM_END_SPACE : xcb_atom_enum_t = 46;
    pub const XCB_ATOM_SUPERSCRIPT_X : xcb_atom_enum_t = 47;
    pub const XCB_ATOM_SUPERSCRIPT_Y : xcb_atom_enum_t = 48;
    pub const XCB_ATOM_SUBSCRIPT_X : xcb_atom_enum_t = 49;
    pub const XCB_ATOM_SUBSCRIPT_Y : xcb_atom_enum_t = 50;
    pub const XCB_ATOM_UNDERLINE_POSITION : xcb_atom_enum_t = 51;
    pub const XCB_ATOM_UNDERLINE_THICKNESS : xcb_atom_enum_t = 52;
    pub const XCB_ATOM_STRIKEOUT_ASCENT : xcb_atom_enum_t = 53;
    pub const XCB_ATOM_STRIKEOUT_DESCENT : xcb_atom_enum_t = 54;
    pub const XCB_ATOM_ITALIC_ANGLE : xcb_atom_enum_t = 55;
    pub const XCB_ATOM_X_HEIGHT : xcb_atom_enum_t = 56;
    pub const XCB_ATOM_QUAD_WIDTH : xcb_atom_enum_t = 57;
    pub const XCB_ATOM_WEIGHT : xcb_atom_enum_t = 58;
    pub const XCB_ATOM_POINT_SIZE : xcb_atom_enum_t = 59;
    pub const XCB_ATOM_RESOLUTION : xcb_atom_enum_t = 60;
    pub const XCB_ATOM_COPYRIGHT : xcb_atom_enum_t = 61;
    pub const XCB_ATOM_NOTICE : xcb_atom_enum_t = 62;
    pub const XCB_ATOM_FONT_NAME : xcb_atom_enum_t = 63;
    pub const XCB_ATOM_FAMILY_NAME : xcb_atom_enum_t = 64;
    pub const XCB_ATOM_FULL_NAME : xcb_atom_enum_t = 65;
    pub const XCB_ATOM_CAP_HEIGHT : xcb_atom_enum_t = 66;
    pub const XCB_ATOM_WM_CLASS : xcb_atom_enum_t = 67;
    pub const XCB_ATOM_WM_TRANSIENT_FOR : xcb_atom_enum_t = 68;
//}
/** Opcode for xcb_selection_request. */
pub const XCB_SELECTION_REQUEST : u8 = 30;
pub struct SelectionRequestEvent {pub base : base::Event<xcb_selection_request_event_t>}
/** Opcode for xcb_selection_notify. */
pub const XCB_SELECTION_NOTIFY : u8 = 31;
pub struct SelectionNotifyEvent {pub base : base::Event<xcb_selection_notify_event_t>}

pub type xcb_colormap_state_t = c_uint;//{
    
/** The colormap was uninstalled. */
    pub const XCB_COLORMAP_STATE_UNINSTALLED : xcb_colormap_state_t = 0;
    
/** The colormap was installed. */
    pub const XCB_COLORMAP_STATE_INSTALLED : xcb_colormap_state_t = 1;
//}

pub type xcb_colormap_enum_t = c_uint;//{
    pub const XCB_COLORMAP_NONE : xcb_colormap_enum_t = 0;
//}
/** Opcode for xcb_colormap_notify. */
pub const XCB_COLORMAP_NOTIFY : u8 = 32;
pub struct ColormapNotifyEvent {pub base : base::Event<xcb_colormap_notify_event_t>}
pub type ClientMessageDataIterator = xcb_client_message_data_iterator_t;

/** Opcode for xcb_client_message. */
pub const XCB_CLIENT_MESSAGE : u8 = 33;
pub struct ClientMessageEvent {pub base : base::Event<xcb_client_message_event_t>}

pub type xcb_mapping_t = c_uint;//{
    pub const XCB_MAPPING_MODIFIER : xcb_mapping_t = 0;
    pub const XCB_MAPPING_KEYBOARD : xcb_mapping_t = 1;
    pub const XCB_MAPPING_POINTER : xcb_mapping_t = 2;
//}
/** Opcode for xcb_mapping_notify. */
pub const XCB_MAPPING_NOTIFY : u8 = 34;
pub struct MappingNotifyEvent {pub base : base::Event<xcb_mapping_notify_event_t>}
/** Opcode for xcb_request. */
pub const XCB_REQUEST : u8 = 1;
pub struct RequestError { pub base : base::Error<xcb_request_error_t> }
/** Opcode for xcb_value. */
pub const XCB_VALUE : u8 = 2;
pub struct ValueError { pub base : base::Error<xcb_value_error_t> }
/** Opcode for xcb_window. */
pub const XCB_WINDOW : u8 = 3;
pub struct WindowError { pub base : base::Error<xcb_window_error_t> }
/** Opcode for xcb_pixmap. */
pub const XCB_PIXMAP : u8 = 4;
pub struct PixmapError { pub base : base::Error<xcb_pixmap_error_t> }
/** Opcode for xcb_atom. */
pub const XCB_ATOM : u8 = 5;
pub struct AtomError { pub base : base::Error<xcb_atom_error_t> }
/** Opcode for xcb_cursor. */
pub const XCB_CURSOR : u8 = 6;
pub struct CursorError { pub base : base::Error<xcb_cursor_error_t> }
/** Opcode for xcb_font. */
pub const XCB_FONT : u8 = 7;
pub struct FontError { pub base : base::Error<xcb_font_error_t> }
/** Opcode for xcb_match. */
pub const XCB_MATCH : u8 = 8;
pub struct MatchError { pub base : base::Error<xcb_match_error_t> }
/** Opcode for xcb_drawable. */
pub const XCB_DRAWABLE : u8 = 9;
pub struct DrawableError { pub base : base::Error<xcb_drawable_error_t> }
/** Opcode for xcb_access. */
pub const XCB_ACCESS : u8 = 10;
pub struct AccessError { pub base : base::Error<xcb_access_error_t> }
/** Opcode for xcb_alloc. */
pub const XCB_ALLOC : u8 = 11;
pub struct AllocError { pub base : base::Error<xcb_alloc_error_t> }
/** Opcode for xcb_colormap. */
pub const XCB_COLORMAP : u8 = 12;
pub struct ColormapError { pub base : base::Error<xcb_colormap_error_t> }
/** Opcode for xcb_g_context. */
pub const XCB_G_CONTEXT : u8 = 13;
pub struct GContextError { pub base : base::Error<xcb_g_context_error_t> }
/** Opcode for xcb_id_choice. */
pub const XCB_ID_CHOICE : u8 = 14;
pub struct IdChoiceError { pub base : base::Error<xcb_id_choice_error_t> }
/** Opcode for xcb_name. */
pub const XCB_NAME : u8 = 15;
pub struct NameError { pub base : base::Error<xcb_name_error_t> }
/** Opcode for xcb_length. */
pub const XCB_LENGTH : u8 = 16;
pub struct LengthError { pub base : base::Error<xcb_length_error_t> }
/** Opcode for xcb_implementation. */
pub const XCB_IMPLEMENTATION : u8 = 17;
pub struct ImplementationError { pub base : base::Error<xcb_implementation_error_t> }

pub type xcb_window_class_t = c_uint;//{
    pub const XCB_WINDOW_CLASS_COPY_FROM_PARENT : xcb_window_class_t = 0;
    pub const XCB_WINDOW_CLASS_INPUT_OUTPUT : xcb_window_class_t = 1;
    pub const XCB_WINDOW_CLASS_INPUT_ONLY : xcb_window_class_t = 2;
//}

pub type xcb_cw_t = c_uint;//{
    
/** Overrides the default background-pixmap. The background pixmap and window must
have the same root and same depth. Any size pixmap can be used, although some
sizes may be faster than others.

If `XCB_BACK_PIXMAP_NONE` is specified, the window has no defined background.
The server may fill the contents with the previous screen contents or with
contents of its own choosing.

If `XCB_BACK_PIXMAP_PARENT_RELATIVE` is specified, the parent's background is
used, but the window must have the same depth as the parent (or a Match error
results).   The parent's background is tracked, and the current version is
used each time the window background is required. */
    pub const XCB_CW_BACK_PIXMAP : xcb_cw_t = 1;
    
/** Overrides `BackPixmap`. A pixmap of undefined size filled with the specified
background pixel is used for the background. Range-checking is not performed,
the background pixel is truncated to the appropriate number of bits. */
    pub const XCB_CW_BACK_PIXEL : xcb_cw_t = 2;
    
/** Overrides the default border-pixmap. The border pixmap and window must have the
same root and the same depth. Any size pixmap can be used, although some sizes
may be faster than others.

The special value `XCB_COPY_FROM_PARENT` means the parent's border pixmap is
copied (subsequent changes to the parent's border attribute do not affect the
child), but the window must have the same depth as the parent. */
    pub const XCB_CW_BORDER_PIXMAP : xcb_cw_t = 4;
    
/** Overrides `BorderPixmap`. A pixmap of undefined size filled with the specified
border pixel is used for the border. Range checking is not performed on the
border-pixel value, it is truncated to the appropriate number of bits. */
    pub const XCB_CW_BORDER_PIXEL : xcb_cw_t = 8;
    
/** Defines which region of the window should be retained if the window is resized. */
    pub const XCB_CW_BIT_GRAVITY : xcb_cw_t = 16;
    
/** Defines how the window should be repositioned if the parent is resized (see
`ConfigureWindow`). */
    pub const XCB_CW_WIN_GRAVITY : xcb_cw_t = 32;
    
/** A backing-store of `WhenMapped` advises the server that maintaining contents of
obscured regions when the window is mapped would be beneficial. A backing-store
of `Always` advises the server that maintaining contents even when the window
is unmapped would be beneficial. In this case, the server may generate an
exposure event when the window is created. A value of `NotUseful` advises the
server that maintaining contents is unnecessary, although a server may still
choose to maintain contents while the window is mapped. Note that if the server
maintains contents, then the server should maintain complete contents not just
the region within the parent boundaries, even if the window is larger than its
parent. While the server maintains contents, exposure events will not normally
be generated, but the server may stop maintaining contents at any time. */
    pub const XCB_CW_BACKING_STORE : xcb_cw_t = 64;
    
/** The backing-planes indicates (with bits set to 1) which bit planes of the
window hold dynamic data that must be preserved in backing-stores and during
save-unders. */
    pub const XCB_CW_BACKING_PLANES : xcb_cw_t = 128;
    
/** The backing-pixel specifies what value to use in planes not covered by
backing-planes. The server is free to save only the specified bit planes in the
backing-store or save-under and regenerate the remaining planes with the
specified pixel value. Any bits beyond the specified depth of the window in
these values are simply ignored. */
    pub const XCB_CW_BACKING_PIXEL : xcb_cw_t = 256;
    
/** The override-redirect specifies whether map and configure requests on this
window should override a SubstructureRedirect on the parent, typically to
inform a window manager not to tamper with the window. */
    pub const XCB_CW_OVERRIDE_REDIRECT : xcb_cw_t = 512;
    
/** If 1, the server is advised that when this window is mapped, saving the
contents of windows it obscures would be beneficial. */
    pub const XCB_CW_SAVE_UNDER : xcb_cw_t = 1024;
    
/** The event-mask defines which events the client is interested in for this window
(or for some event types, inferiors of the window). */
    pub const XCB_CW_EVENT_MASK : xcb_cw_t = 2048;
    
/** The do-not-propagate-mask defines which events should not be propagated to
ancestor windows when no client has the event type selected in this window. */
    pub const XCB_CW_DONT_PROPAGATE : xcb_cw_t = 4096;
    
/** The colormap specifies the colormap that best reflects the true colors of the window. Servers
capable of supporting multiple hardware colormaps may use this information, and window man-
agers may use it for InstallColormap requests. The colormap must have the same visual type
and root as the window (or a Match error results). If CopyFromParent is specified, the parent's
colormap is copied (subsequent changes to the parent's colormap attribute do not affect the child).
However, the window must have the same visual type as the parent (or a Match error results),
and the parent must not have a colormap of None (or a Match error results). For an explanation
of None, see FreeColormap request. The colormap is copied by sharing the colormap object
between the child and the parent, not by making a complete copy of the colormap contents. */
    pub const XCB_CW_COLORMAP : xcb_cw_t = 8192;
    
/** If a cursor is specified, it will be used whenever the pointer is in the window. If None is speci-
fied, the parent's cursor will be used when the pointer is in the window, and any change in the
parent's cursor will cause an immediate change in the displayed cursor. */
    pub const XCB_CW_CURSOR : xcb_cw_t = 16384;
//}

pub type xcb_back_pixmap_t = c_uint;//{
    pub const XCB_BACK_PIXMAP_NONE : xcb_back_pixmap_t = 0;
    pub const XCB_BACK_PIXMAP_PARENT_RELATIVE : xcb_back_pixmap_t = 1;
//}

pub type xcb_gravity_t = c_uint;//{
    pub const XCB_GRAVITY_BIT_FORGET : xcb_gravity_t = 0;
    pub const XCB_GRAVITY_WIN_UNMAP : xcb_gravity_t = 0;
    pub const XCB_GRAVITY_NORTH_WEST : xcb_gravity_t = 1;
    pub const XCB_GRAVITY_NORTH : xcb_gravity_t = 2;
    pub const XCB_GRAVITY_NORTH_EAST : xcb_gravity_t = 3;
    pub const XCB_GRAVITY_WEST : xcb_gravity_t = 4;
    pub const XCB_GRAVITY_CENTER : xcb_gravity_t = 5;
    pub const XCB_GRAVITY_EAST : xcb_gravity_t = 6;
    pub const XCB_GRAVITY_SOUTH_WEST : xcb_gravity_t = 7;
    pub const XCB_GRAVITY_SOUTH : xcb_gravity_t = 8;
    pub const XCB_GRAVITY_SOUTH_EAST : xcb_gravity_t = 9;
    pub const XCB_GRAVITY_STATIC : xcb_gravity_t = 10;
//}
/** Opcode for xcb_create_window. */
pub const XCB_CREATE_WINDOW : u8 = 1;
/** Opcode for xcb_change_window_attributes. */
pub const XCB_CHANGE_WINDOW_ATTRIBUTES : u8 = 2;

pub type xcb_map_state_t = c_uint;//{
    pub const XCB_MAP_STATE_UNMAPPED : xcb_map_state_t = 0;
    pub const XCB_MAP_STATE_UNVIEWABLE : xcb_map_state_t = 1;
    pub const XCB_MAP_STATE_VIEWABLE : xcb_map_state_t = 2;
//}
pub struct  GetWindowAttributesCookie<'s> { pub base : base::Cookie<'s, xcb_get_window_attributes_cookie_t> }

/** Opcode for xcb_get_window_attributes. */
pub const XCB_GET_WINDOW_ATTRIBUTES : u8 = 3;
pub struct GetWindowAttributesReply { base:  base::Reply<xcb_get_window_attributes_reply_t> }
fn mk_reply_xcb_get_window_attributes_reply_t(reply:*mut xcb_get_window_attributes_reply_t) -> GetWindowAttributesReply { GetWindowAttributesReply { base : base::mk_reply(reply) } }
/** Opcode for xcb_destroy_window. */
pub const XCB_DESTROY_WINDOW : u8 = 4;
/** Opcode for xcb_destroy_subwindows. */
pub const XCB_DESTROY_SUBWINDOWS : u8 = 5;

pub type xcb_set_mode_t = c_uint;//{
    pub const XCB_SET_MODE_INSERT : xcb_set_mode_t = 0;
    pub const XCB_SET_MODE_DELETE : xcb_set_mode_t = 1;
//}
/** Opcode for xcb_change_save_set. */
pub const XCB_CHANGE_SAVE_SET : u8 = 6;
/** Opcode for xcb_reparent_window. */
pub const XCB_REPARENT_WINDOW : u8 = 7;
/** Opcode for xcb_map_window. */
pub const XCB_MAP_WINDOW : u8 = 8;
/** Opcode for xcb_map_subwindows. */
pub const XCB_MAP_SUBWINDOWS : u8 = 9;
/** Opcode for xcb_unmap_window. */
pub const XCB_UNMAP_WINDOW : u8 = 10;
/** Opcode for xcb_unmap_subwindows. */
pub const XCB_UNMAP_SUBWINDOWS : u8 = 11;

pub type xcb_config_window_t = c_uint;//{
    pub const XCB_CONFIG_WINDOW_X : xcb_config_window_t = 1;
    pub const XCB_CONFIG_WINDOW_Y : xcb_config_window_t = 2;
    pub const XCB_CONFIG_WINDOW_WIDTH : xcb_config_window_t = 4;
    pub const XCB_CONFIG_WINDOW_HEIGHT : xcb_config_window_t = 8;
    pub const XCB_CONFIG_WINDOW_BORDER_WIDTH : xcb_config_window_t = 16;
    pub const XCB_CONFIG_WINDOW_SIBLING : xcb_config_window_t = 32;
    pub const XCB_CONFIG_WINDOW_STACK_MODE : xcb_config_window_t = 64;
//}

pub type xcb_stack_mode_t = c_uint;//{
    pub const XCB_STACK_MODE_ABOVE : xcb_stack_mode_t = 0;
    pub const XCB_STACK_MODE_BELOW : xcb_stack_mode_t = 1;
    pub const XCB_STACK_MODE_TOP_IF : xcb_stack_mode_t = 2;
    pub const XCB_STACK_MODE_BOTTOM_IF : xcb_stack_mode_t = 3;
    pub const XCB_STACK_MODE_OPPOSITE : xcb_stack_mode_t = 4;
//}
/** Opcode for xcb_configure_window. */
pub const XCB_CONFIGURE_WINDOW : u8 = 12;

pub type xcb_circulate_t = c_uint;//{
    pub const XCB_CIRCULATE_RAISE_LOWEST : xcb_circulate_t = 0;
    pub const XCB_CIRCULATE_LOWER_HIGHEST : xcb_circulate_t = 1;
//}
/** Opcode for xcb_circulate_window. */
pub const XCB_CIRCULATE_WINDOW : u8 = 13;
pub struct  GetGeometryCookie<'s> { pub base : base::Cookie<'s, xcb_get_geometry_cookie_t> }

/** Opcode for xcb_get_geometry. */
pub const XCB_GET_GEOMETRY : u8 = 14;
pub struct GetGeometryReply { base:  base::Reply<xcb_get_geometry_reply_t> }
fn mk_reply_xcb_get_geometry_reply_t(reply:*mut xcb_get_geometry_reply_t) -> GetGeometryReply { GetGeometryReply { base : base::mk_reply(reply) } }
pub struct  QueryTreeCookie<'s> { pub base : base::Cookie<'s, xcb_query_tree_cookie_t> }

/** Opcode for xcb_query_tree. */
pub const XCB_QUERY_TREE : u8 = 15;
pub struct  InternAtomCookie<'s> { pub base : base::Cookie<'s, xcb_intern_atom_cookie_t> }

/** Opcode for xcb_intern_atom. */
pub const XCB_INTERN_ATOM : u8 = 16;
pub struct InternAtomReply { base:  base::Reply<xcb_intern_atom_reply_t> }
fn mk_reply_xcb_intern_atom_reply_t(reply:*mut xcb_intern_atom_reply_t) -> InternAtomReply { InternAtomReply { base : base::mk_reply(reply) } }
pub struct  GetAtomNameCookie<'s> { pub base : base::Cookie<'s, xcb_get_atom_name_cookie_t> }

/** Opcode for xcb_get_atom_name. */
pub const XCB_GET_ATOM_NAME : u8 = 17;

pub type xcb_prop_mode_t = c_uint;//{
    
/** Discard the previous property value and store the new data. */
    pub const XCB_PROP_MODE_REPLACE : xcb_prop_mode_t = 0;
    
/** Insert the new data before the beginning of existing data. The `format` must
match existing property value. If the property is undefined, it is treated as
defined with the correct type and format with zero-length data. */
    pub const XCB_PROP_MODE_PREPEND : xcb_prop_mode_t = 1;
    
/** Insert the new data after the beginning of existing data. The `format` must
match existing property value. If the property is undefined, it is treated as
defined with the correct type and format with zero-length data. */
    pub const XCB_PROP_MODE_APPEND : xcb_prop_mode_t = 2;
//}
/** Opcode for xcb_change_property. */
pub const XCB_CHANGE_PROPERTY : u8 = 18;
/** Opcode for xcb_delete_property. */
pub const XCB_DELETE_PROPERTY : u8 = 19;

pub type xcb_get_property_type_t = c_uint;//{
    pub const XCB_GET_PROPERTY_TYPE_ANY : xcb_get_property_type_t = 0;
//}
pub struct  GetPropertyCookie<'s> { pub base : base::Cookie<'s, xcb_get_property_cookie_t> }

/** Opcode for xcb_get_property. */
pub const XCB_GET_PROPERTY : u8 = 20;
pub struct  ListPropertiesCookie<'s> { pub base : base::Cookie<'s, xcb_list_properties_cookie_t> }

/** Opcode for xcb_list_properties. */
pub const XCB_LIST_PROPERTIES : u8 = 21;
/** Opcode for xcb_set_selection_owner. */
pub const XCB_SET_SELECTION_OWNER : u8 = 22;
pub struct  GetSelectionOwnerCookie<'s> { pub base : base::Cookie<'s, xcb_get_selection_owner_cookie_t> }

/** Opcode for xcb_get_selection_owner. */
pub const XCB_GET_SELECTION_OWNER : u8 = 23;
pub struct GetSelectionOwnerReply { base:  base::Reply<xcb_get_selection_owner_reply_t> }
fn mk_reply_xcb_get_selection_owner_reply_t(reply:*mut xcb_get_selection_owner_reply_t) -> GetSelectionOwnerReply { GetSelectionOwnerReply { base : base::mk_reply(reply) } }
/** Opcode for xcb_convert_selection. */
pub const XCB_CONVERT_SELECTION : u8 = 24;

pub type xcb_send_event_dest_t = c_uint;//{
    pub const XCB_SEND_EVENT_DEST_POINTER_WINDOW : xcb_send_event_dest_t = 0;
    pub const XCB_SEND_EVENT_DEST_ITEM_FOCUS : xcb_send_event_dest_t = 1;
//}
/** Opcode for xcb_send_event. */
pub const XCB_SEND_EVENT : u8 = 25;

pub type xcb_grab_mode_t = c_uint;//{
    
/** The state of the keyboard appears to freeze: No further keyboard events are
generated by the server until the grabbing client issues a releasing
`AllowEvents` request or until the keyboard grab is released. */
    pub const XCB_GRAB_MODE_SYNC : xcb_grab_mode_t = 0;
    
/** Keyboard event processing continues normally. */
    pub const XCB_GRAB_MODE_ASYNC : xcb_grab_mode_t = 1;
//}

pub type xcb_grab_status_t = c_uint;//{
    pub const XCB_GRAB_STATUS_SUCCESS : xcb_grab_status_t = 0;
    pub const XCB_GRAB_STATUS_ALREADY_GRABBED : xcb_grab_status_t = 1;
    pub const XCB_GRAB_STATUS_INVALID_TIME : xcb_grab_status_t = 2;
    pub const XCB_GRAB_STATUS_NOT_VIEWABLE : xcb_grab_status_t = 3;
    pub const XCB_GRAB_STATUS_FROZEN : xcb_grab_status_t = 4;
//}

pub type xcb_cursor_enum_t = c_uint;//{
    pub const XCB_CURSOR_NONE : xcb_cursor_enum_t = 0;
//}
pub struct  GrabPointerCookie<'s> { pub base : base::Cookie<'s, xcb_grab_pointer_cookie_t> }

/** Opcode for xcb_grab_pointer. */
pub const XCB_GRAB_POINTER : u8 = 26;
pub struct GrabPointerReply { base:  base::Reply<xcb_grab_pointer_reply_t> }
fn mk_reply_xcb_grab_pointer_reply_t(reply:*mut xcb_grab_pointer_reply_t) -> GrabPointerReply { GrabPointerReply { base : base::mk_reply(reply) } }
/** Opcode for xcb_ungrab_pointer. */
pub const XCB_UNGRAB_POINTER : u8 = 27;

pub type xcb_button_index_t = c_uint;//{
    
/** Any of the following (or none): */
    pub const XCB_BUTTON_INDEX_ANY : xcb_button_index_t = 0;
    
/** The left mouse button. */
    pub const XCB_BUTTON_INDEX_1 : xcb_button_index_t = 1;
    
/** The right mouse button. */
    pub const XCB_BUTTON_INDEX_2 : xcb_button_index_t = 2;
    
/** The middle mouse button. */
    pub const XCB_BUTTON_INDEX_3 : xcb_button_index_t = 3;
    
/** Scroll wheel. TODO: direction? */
    pub const XCB_BUTTON_INDEX_4 : xcb_button_index_t = 4;
    
/** Scroll wheel. TODO: direction? */
    pub const XCB_BUTTON_INDEX_5 : xcb_button_index_t = 5;
//}
/** Opcode for xcb_grab_button. */
pub const XCB_GRAB_BUTTON : u8 = 28;
/** Opcode for xcb_ungrab_button. */
pub const XCB_UNGRAB_BUTTON : u8 = 29;
/** Opcode for xcb_change_active_pointer_grab. */
pub const XCB_CHANGE_ACTIVE_POINTER_GRAB : u8 = 30;
pub struct  GrabKeyboardCookie<'s> { pub base : base::Cookie<'s, xcb_grab_keyboard_cookie_t> }

/** Opcode for xcb_grab_keyboard. */
pub const XCB_GRAB_KEYBOARD : u8 = 31;
pub struct GrabKeyboardReply { base:  base::Reply<xcb_grab_keyboard_reply_t> }
fn mk_reply_xcb_grab_keyboard_reply_t(reply:*mut xcb_grab_keyboard_reply_t) -> GrabKeyboardReply { GrabKeyboardReply { base : base::mk_reply(reply) } }
/** Opcode for xcb_ungrab_keyboard. */
pub const XCB_UNGRAB_KEYBOARD : u8 = 32;

pub type xcb_grab_t = c_uint;//{
    pub const XCB_GRAB_ANY : xcb_grab_t = 0;
//}
/** Opcode for xcb_grab_key. */
pub const XCB_GRAB_KEY : u8 = 33;
/** Opcode for xcb_ungrab_key. */
pub const XCB_UNGRAB_KEY : u8 = 34;

pub type xcb_allow_t = c_uint;//{
    
/** For AsyncPointer, if the pointer is frozen by the client, pointer event
processing continues normally. If the pointer is frozen twice by the client on
behalf of two separate grabs, AsyncPointer thaws for both. AsyncPointer has no
effect if the pointer is not frozen by the client, but the pointer need not be
grabbed by the client.

TODO: rewrite this in more understandable terms. */
    pub const XCB_ALLOW_ASYNC_POINTER : xcb_allow_t = 0;
    
/** For SyncPointer, if the pointer is frozen and actively grabbed by the client,
pointer event processing continues normally until the next ButtonPress or
ButtonRelease event is reported to the client, at which time the pointer again
appears to freeze. However, if the reported event causes the pointer grab to be
released, then the pointer does not freeze. SyncPointer has no effect if the
pointer is not frozen by the client or if the pointer is not grabbed by the
client. */
    pub const XCB_ALLOW_SYNC_POINTER : xcb_allow_t = 1;
    
/** For ReplayPointer, if the pointer is actively grabbed by the client and is
frozen as the result of an event having been sent to the client (either from
the activation of a GrabButton or from a previous AllowEvents with mode
SyncPointer but not from a GrabPointer), then the pointer grab is released and
that event is completely reprocessed, this time ignoring any passive grabs at
or above (towards the root) the grab-window of the grab just released. The
request has no effect if the pointer is not grabbed by the client or if the
pointer is not frozen as the result of an event. */
    pub const XCB_ALLOW_REPLAY_POINTER : xcb_allow_t = 2;
    
/** For AsyncKeyboard, if the keyboard is frozen by the client, keyboard event
processing continues normally. If the keyboard is frozen twice by the client on
behalf of two separate grabs, AsyncKeyboard thaws for both. AsyncKeyboard has
no effect if the keyboard is not frozen by the client, but the keyboard need
not be grabbed by the client. */
    pub const XCB_ALLOW_ASYNC_KEYBOARD : xcb_allow_t = 3;
    
/** For SyncKeyboard, if the keyboard is frozen and actively grabbed by the client,
keyboard event processing continues normally until the next KeyPress or
KeyRelease event is reported to the client, at which time the keyboard again
appears to freeze. However, if the reported event causes the keyboard grab to
be released, then the keyboard does not freeze. SyncKeyboard has no effect if
the keyboard is not frozen by the client or if the keyboard is not grabbed by
the client. */
    pub const XCB_ALLOW_SYNC_KEYBOARD : xcb_allow_t = 4;
    
/** For ReplayKeyboard, if the keyboard is actively grabbed by the client and is
frozen as the result of an event having been sent to the client (either from
the activation of a GrabKey or from a previous AllowEvents with mode
SyncKeyboard but not from a GrabKeyboard), then the keyboard grab is released
and that event is completely reprocessed, this time ignoring any passive grabs
at or above (towards the root) the grab-window of the grab just released. The
request has no effect if the keyboard is not grabbed by the client or if the
keyboard is not frozen as the result of an event. */
    pub const XCB_ALLOW_REPLAY_KEYBOARD : xcb_allow_t = 5;
    
/** For AsyncBoth, if the pointer and the keyboard are frozen by the client, event
processing for both devices continues normally. If a device is frozen twice by
the client on behalf of two separate grabs, AsyncBoth thaws for both. AsyncBoth
has no effect unless both pointer and keyboard are frozen by the client. */
    pub const XCB_ALLOW_ASYNC_BOTH : xcb_allow_t = 6;
    
/** For SyncBoth, if both pointer and keyboard are frozen by the client, event
processing (for both devices) continues normally until the next ButtonPress,
ButtonRelease, KeyPress, or KeyRelease event is reported to the client for a
grabbed device (button event for the pointer, key event for the keyboard), at
which time the devices again appear to freeze. However, if the reported event
causes the grab to be released, then the devices do not freeze (but if the
other device is still grabbed, then a subsequent event for it will still cause
both devices to freeze). SyncBoth has no effect unless both pointer and
keyboard are frozen by the client. If the pointer or keyboard is frozen twice
by the client on behalf of two separate grabs, SyncBoth thaws for both (but a
subsequent freeze for SyncBoth will only freeze each device once). */
    pub const XCB_ALLOW_SYNC_BOTH : xcb_allow_t = 7;
//}
/** Opcode for xcb_allow_events. */
pub const XCB_ALLOW_EVENTS : u8 = 35;
/** Opcode for xcb_grab_server. */
pub const XCB_GRAB_SERVER : u8 = 36;
/** Opcode for xcb_ungrab_server. */
pub const XCB_UNGRAB_SERVER : u8 = 37;
pub struct  QueryPointerCookie<'s> { pub base : base::Cookie<'s, xcb_query_pointer_cookie_t> }

/** Opcode for xcb_query_pointer. */
pub const XCB_QUERY_POINTER : u8 = 38;
pub struct QueryPointerReply { base:  base::Reply<xcb_query_pointer_reply_t> }
fn mk_reply_xcb_query_pointer_reply_t(reply:*mut xcb_query_pointer_reply_t) -> QueryPointerReply { QueryPointerReply { base : base::mk_reply(reply) } }
pub type TimecoordIterator = xcb_timecoord_iterator_t;

pub struct  GetMotionEventsCookie<'s> { pub base : base::Cookie<'s, xcb_get_motion_events_cookie_t> }

/** Opcode for xcb_get_motion_events. */
pub const XCB_GET_MOTION_EVENTS : u8 = 39;
pub struct  TranslateCoordinatesCookie<'s> { pub base : base::Cookie<'s, xcb_translate_coordinates_cookie_t> }

/** Opcode for xcb_translate_coordinates. */
pub const XCB_TRANSLATE_COORDINATES : u8 = 40;
pub struct TranslateCoordinatesReply { base:  base::Reply<xcb_translate_coordinates_reply_t> }
fn mk_reply_xcb_translate_coordinates_reply_t(reply:*mut xcb_translate_coordinates_reply_t) -> TranslateCoordinatesReply { TranslateCoordinatesReply { base : base::mk_reply(reply) } }
/** Opcode for xcb_warp_pointer. */
pub const XCB_WARP_POINTER : u8 = 41;

pub type xcb_input_focus_t = c_uint;//{
    
/** The focus reverts to `XCB_NONE`, so no window will have the input focus. */
    pub const XCB_INPUT_FOCUS_NONE : xcb_input_focus_t = 0;
    
/** The focus reverts to `XCB_POINTER_ROOT` respectively. When the focus reverts,
FocusIn and FocusOut events are generated, but the last-focus-change time is
not changed. */
    pub const XCB_INPUT_FOCUS_POINTER_ROOT : xcb_input_focus_t = 1;
    
/** The focus reverts to the parent (or closest viewable ancestor) and the new
revert_to value is `XCB_INPUT_FOCUS_NONE`. */
    pub const XCB_INPUT_FOCUS_PARENT : xcb_input_focus_t = 2;
    
/** NOT YET DOCUMENTED. Only relevant for the xinput extension. */
    pub const XCB_INPUT_FOCUS_FOLLOW_KEYBOARD : xcb_input_focus_t = 3;
//}
/** Opcode for xcb_set_input_focus. */
pub const XCB_SET_INPUT_FOCUS : u8 = 42;
pub struct  GetInputFocusCookie<'s> { pub base : base::Cookie<'s, xcb_get_input_focus_cookie_t> }

/** Opcode for xcb_get_input_focus. */
pub const XCB_GET_INPUT_FOCUS : u8 = 43;
pub struct GetInputFocusReply { base:  base::Reply<xcb_get_input_focus_reply_t> }
fn mk_reply_xcb_get_input_focus_reply_t(reply:*mut xcb_get_input_focus_reply_t) -> GetInputFocusReply { GetInputFocusReply { base : base::mk_reply(reply) } }
pub struct  QueryKeymapCookie<'s> { pub base : base::Cookie<'s, xcb_query_keymap_cookie_t> }

/** Opcode for xcb_query_keymap. */
pub const XCB_QUERY_KEYMAP : u8 = 44;
pub struct QueryKeymapReply { base:  base::Reply<xcb_query_keymap_reply_t> }
fn mk_reply_xcb_query_keymap_reply_t(reply:*mut xcb_query_keymap_reply_t) -> QueryKeymapReply { QueryKeymapReply { base : base::mk_reply(reply) } }
/** Opcode for xcb_open_font. */
pub const XCB_OPEN_FONT : u8 = 45;
/** Opcode for xcb_close_font. */
pub const XCB_CLOSE_FONT : u8 = 46;

pub type xcb_font_draw_t = c_uint;//{
    pub const XCB_FONT_DRAW_LEFT_TO_RIGHT : xcb_font_draw_t = 0;
    pub const XCB_FONT_DRAW_RIGHT_TO_LEFT : xcb_font_draw_t = 1;
//}
pub struct Fontprop {pub base : base::Struct<xcb_fontprop_t> }

pub type FontpropIterator = xcb_fontprop_iterator_t;

pub type CharinfoIterator = xcb_charinfo_iterator_t;

pub struct  QueryFontCookie<'s> { pub base : base::Cookie<'s, xcb_query_font_cookie_t> }

/** Opcode for xcb_query_font. */
pub const XCB_QUERY_FONT : u8 = 47;
pub struct  QueryTextExtentsCookie<'s> { pub base : base::Cookie<'s, xcb_query_text_extents_cookie_t> }

/** Opcode for xcb_query_text_extents. */
pub const XCB_QUERY_TEXT_EXTENTS : u8 = 48;
pub struct QueryTextExtentsReply { base:  base::Reply<xcb_query_text_extents_reply_t> }
fn mk_reply_xcb_query_text_extents_reply_t(reply:*mut xcb_query_text_extents_reply_t) -> QueryTextExtentsReply { QueryTextExtentsReply { base : base::mk_reply(reply) } }
pub type StrIterator = xcb_str_iterator_t;

pub struct  ListFontsCookie<'s> { pub base : base::Cookie<'s, xcb_list_fonts_cookie_t> }

/** Opcode for xcb_list_fonts. */
pub const XCB_LIST_FONTS : u8 = 49;
pub struct ListFontsReply { base:  base::Reply<xcb_list_fonts_reply_t> }
fn mk_reply_xcb_list_fonts_reply_t(reply:*mut xcb_list_fonts_reply_t) -> ListFontsReply { ListFontsReply { base : base::mk_reply(reply) } }
pub struct  ListFontsWithInfoCookie<'s> { pub base : base::Cookie<'s, xcb_list_fonts_with_info_cookie_t> }

/** Opcode for xcb_list_fonts_with_info. */
pub const XCB_LIST_FONTS_WITH_INFO : u8 = 50;
pub struct ListFontsWithInfoReply { base:  base::Reply<xcb_list_fonts_with_info_reply_t> }
fn mk_reply_xcb_list_fonts_with_info_reply_t(reply:*mut xcb_list_fonts_with_info_reply_t) -> ListFontsWithInfoReply { ListFontsWithInfoReply { base : base::mk_reply(reply) } }
/** Opcode for xcb_set_font_path. */
pub const XCB_SET_FONT_PATH : u8 = 51;
pub struct  GetFontPathCookie<'s> { pub base : base::Cookie<'s, xcb_get_font_path_cookie_t> }

/** Opcode for xcb_get_font_path. */
pub const XCB_GET_FONT_PATH : u8 = 52;
/** Opcode for xcb_create_pixmap. */
pub const XCB_CREATE_PIXMAP : u8 = 53;
/** Opcode for xcb_free_pixmap. */
pub const XCB_FREE_PIXMAP : u8 = 54;

pub type xcb_gc_t = c_uint;//{
    
/** TODO: Refer to GX */
    pub const XCB_GC_FUNCTION : xcb_gc_t = 1;
    
/** In graphics operations, given a source and destination pixel, the result is
computed bitwise on corresponding bits of the pixels; that is, a Boolean
operation is performed in each bit plane. The plane-mask restricts the
operation to a subset of planes, so the result is:

        ((src FUNC dst) AND plane-mask) OR (dst AND (NOT plane-mask)) */
    pub const XCB_GC_PLANE_MASK : xcb_gc_t = 2;
    
/** Foreground colorpixel. */
    pub const XCB_GC_FOREGROUND : xcb_gc_t = 4;
    
/** Background colorpixel. */
    pub const XCB_GC_BACKGROUND : xcb_gc_t = 8;
    
/** The line-width is measured in pixels and can be greater than or equal to one, a wide line, or the
special value zero, a thin line. */
    pub const XCB_GC_LINE_WIDTH : xcb_gc_t = 16;
    
/** The line-style defines which sections of a line are drawn:
Solid                The full path of the line is drawn.
DoubleDash           The full path of the line is drawn, but the even dashes are filled differently
                     than the odd dashes (see fill-style), with Butt cap-style used where even and
                     odd dashes meet.
OnOffDash            Only the even dashes are drawn, and cap-style applies to all internal ends of
                     the individual dashes (except NotLast is treated as Butt). */
    pub const XCB_GC_LINE_STYLE : xcb_gc_t = 32;
    
/** The cap-style defines how the endpoints of a path are drawn:
NotLast    The result is equivalent to Butt, except that for a line-width of zero the final
           endpoint is not drawn.
Butt       The result is square at the endpoint (perpendicular to the slope of the line)
           with no projection beyond.
Round      The result is a circular arc with its diameter equal to the line-width, centered
           on the endpoint; it is equivalent to Butt for line-width zero.
Projecting The result is square at the end, but the path continues beyond the endpoint for
           a distance equal to half the line-width; it is equivalent to Butt for line-width
           zero. */
    pub const XCB_GC_CAP_STYLE : xcb_gc_t = 64;
    
/** The join-style defines how corners are drawn for wide lines:
Miter               The outer edges of the two lines extend to meet at an angle. However, if the
                    angle is less than 11 degrees, a Bevel join-style is used instead.
Round               The result is a circular arc with a diameter equal to the line-width, centered
                    on the joinpoint.
Bevel               The result is Butt endpoint styles, and then the triangular notch is filled. */
    pub const XCB_GC_JOIN_STYLE : xcb_gc_t = 128;
    
/** The fill-style defines the contents of the source for line, text, and fill requests. For all text and fill
requests (for example, PolyText8, PolyText16, PolyFillRectangle, FillPoly, and PolyFillArc)
as well as for line requests with line-style Solid, (for example, PolyLine, PolySegment,
PolyRectangle, PolyArc) and for the even dashes for line requests with line-style OnOffDash
or DoubleDash:
Solid                     Foreground
Tiled                     Tile
OpaqueStippled            A tile with the same width and height as stipple but with background
                          everywhere stipple has a zero and with foreground everywhere stipple
                          has a one
Stippled                  Foreground masked by stipple
For the odd dashes for line requests with line-style DoubleDash:
Solid                     Background
Tiled                     Same as for even dashes
OpaqueStippled            Same as for even dashes
Stippled                  Background masked by stipple */
    pub const XCB_GC_FILL_STYLE : xcb_gc_t = 256;
    
/**  */
    pub const XCB_GC_FILL_RULE : xcb_gc_t = 512;
    
/** The tile/stipple represents an infinite two-dimensional plane with the tile/stipple replicated in all
dimensions. When that plane is superimposed on the drawable for use in a graphics operation,
the upper-left corner of some instance of the tile/stipple is at the coordinates within the drawable
specified by the tile/stipple origin. The tile/stipple and clip origins are interpreted relative to the
origin of whatever destination drawable is specified in a graphics request.
The tile pixmap must have the same root and depth as the gcontext (or a Match error results).
The stipple pixmap must have depth one and must have the same root as the gcontext (or a
Match error results). For fill-style Stippled (but not fill-style
OpaqueStippled), the stipple pattern is tiled in a single plane and acts as an
additional clip mask to be ANDed with the clip-mask.
Any size pixmap can be used for tiling or stippling, although some sizes may be faster to use than
others. */
    pub const XCB_GC_TILE : xcb_gc_t = 1024;
    
/** The tile/stipple represents an infinite two-dimensional plane with the tile/stipple replicated in all
dimensions. When that plane is superimposed on the drawable for use in a graphics operation,
the upper-left corner of some instance of the tile/stipple is at the coordinates within the drawable
specified by the tile/stipple origin. The tile/stipple and clip origins are interpreted relative to the
origin of whatever destination drawable is specified in a graphics request.
The tile pixmap must have the same root and depth as the gcontext (or a Match error results).
The stipple pixmap must have depth one and must have the same root as the gcontext (or a
Match error results). For fill-style Stippled (but not fill-style
OpaqueStippled), the stipple pattern is tiled in a single plane and acts as an
additional clip mask to be ANDed with the clip-mask.
Any size pixmap can be used for tiling or stippling, although some sizes may be faster to use than
others. */
    pub const XCB_GC_STIPPLE : xcb_gc_t = 2048;
    
/** TODO */
    pub const XCB_GC_TILE_STIPPLE_ORIGIN_X : xcb_gc_t = 4096;
    
/** TODO */
    pub const XCB_GC_TILE_STIPPLE_ORIGIN_Y : xcb_gc_t = 8192;
    
/** Which font to use for the `ImageText8` and `ImageText16` requests. */
    pub const XCB_GC_FONT : xcb_gc_t = 16384;
    
/** For ClipByChildren, both source and destination windows are additionally
clipped by all viewable InputOutput children. For IncludeInferiors, neither
source nor destination window is
clipped by inferiors. This will result in including subwindow contents in the source and drawing
through subwindow boundaries of the destination. The use of IncludeInferiors with a source or
destination window of one depth with mapped inferiors of differing depth is not illegal, but the
semantics is undefined by the core protocol. */
    pub const XCB_GC_SUBWINDOW_MODE : xcb_gc_t = 32768;
    
/** Whether ExposureEvents should be generated (1) or not (0).

The default is 1. */
    pub const XCB_GC_GRAPHICS_EXPOSURES : xcb_gc_t = 65536;
    
/** TODO */
    pub const XCB_GC_CLIP_ORIGIN_X : xcb_gc_t = 131072;
    
/** TODO */
    pub const XCB_GC_CLIP_ORIGIN_Y : xcb_gc_t = 262144;
    
/** The clip-mask restricts writes to the destination drawable. Only pixels where the clip-mask has
bits set to 1 are drawn. Pixels are not drawn outside the area covered by the clip-mask or where
the clip-mask has bits set to 0. The clip-mask affects all graphics requests, but it does not clip
sources. The clip-mask origin is interpreted relative to the origin of whatever destination drawable is specified in a graphics request. If a pixmap is specified as the clip-mask, it must have
depth 1 and have the same root as the gcontext (or a Match error results). If clip-mask is None,
then pixels are always drawn, regardless of the clip origin. The clip-mask can also be set with the
SetClipRectangles request. */
    pub const XCB_GC_CLIP_MASK : xcb_gc_t = 524288;
    
/** TODO */
    pub const XCB_GC_DASH_OFFSET : xcb_gc_t = 1048576;
    
/** TODO */
    pub const XCB_GC_DASH_LIST : xcb_gc_t = 2097152;
    
/** TODO */
    pub const XCB_GC_ARC_MODE : xcb_gc_t = 4194304;
//}

pub type xcb_gx_t = c_uint;//{
    pub const XCB_GX_CLEAR : xcb_gx_t = 0;
    pub const XCB_GX_AND : xcb_gx_t = 1;
    pub const XCB_GX_AND_REVERSE : xcb_gx_t = 2;
    pub const XCB_GX_COPY : xcb_gx_t = 3;
    pub const XCB_GX_AND_INVERTED : xcb_gx_t = 4;
    pub const XCB_GX_NOOP : xcb_gx_t = 5;
    pub const XCB_GX_XOR : xcb_gx_t = 6;
    pub const XCB_GX_OR : xcb_gx_t = 7;
    pub const XCB_GX_NOR : xcb_gx_t = 8;
    pub const XCB_GX_EQUIV : xcb_gx_t = 9;
    pub const XCB_GX_INVERT : xcb_gx_t = 10;
    pub const XCB_GX_OR_REVERSE : xcb_gx_t = 11;
    pub const XCB_GX_COPY_INVERTED : xcb_gx_t = 12;
    pub const XCB_GX_OR_INVERTED : xcb_gx_t = 13;
    pub const XCB_GX_NAND : xcb_gx_t = 14;
    pub const XCB_GX_SET : xcb_gx_t = 15;
//}

pub type xcb_line_style_t = c_uint;//{
    pub const XCB_LINE_STYLE_SOLID : xcb_line_style_t = 0;
    pub const XCB_LINE_STYLE_ON_OFF_DASH : xcb_line_style_t = 1;
    pub const XCB_LINE_STYLE_DOUBLE_DASH : xcb_line_style_t = 2;
//}

pub type xcb_cap_style_t = c_uint;//{
    pub const XCB_CAP_STYLE_NOT_LAST : xcb_cap_style_t = 0;
    pub const XCB_CAP_STYLE_BUTT : xcb_cap_style_t = 1;
    pub const XCB_CAP_STYLE_ROUND : xcb_cap_style_t = 2;
    pub const XCB_CAP_STYLE_PROJECTING : xcb_cap_style_t = 3;
//}

pub type xcb_join_style_t = c_uint;//{
    pub const XCB_JOIN_STYLE_MITER : xcb_join_style_t = 0;
    pub const XCB_JOIN_STYLE_ROUND : xcb_join_style_t = 1;
    pub const XCB_JOIN_STYLE_BEVEL : xcb_join_style_t = 2;
//}

pub type xcb_fill_style_t = c_uint;//{
    pub const XCB_FILL_STYLE_SOLID : xcb_fill_style_t = 0;
    pub const XCB_FILL_STYLE_TILED : xcb_fill_style_t = 1;
    pub const XCB_FILL_STYLE_STIPPLED : xcb_fill_style_t = 2;
    pub const XCB_FILL_STYLE_OPAQUE_STIPPLED : xcb_fill_style_t = 3;
//}

pub type xcb_fill_rule_t = c_uint;//{
    pub const XCB_FILL_RULE_EVEN_ODD : xcb_fill_rule_t = 0;
    pub const XCB_FILL_RULE_WINDING : xcb_fill_rule_t = 1;
//}

pub type xcb_subwindow_mode_t = c_uint;//{
    pub const XCB_SUBWINDOW_MODE_CLIP_BY_CHILDREN : xcb_subwindow_mode_t = 0;
    pub const XCB_SUBWINDOW_MODE_INCLUDE_INFERIORS : xcb_subwindow_mode_t = 1;
//}

pub type xcb_arc_mode_t = c_uint;//{
    pub const XCB_ARC_MODE_CHORD : xcb_arc_mode_t = 0;
    pub const XCB_ARC_MODE_PIE_SLICE : xcb_arc_mode_t = 1;
//}
/** Opcode for xcb_create_gc. */
pub const XCB_CREATE_GC : u8 = 55;
/** Opcode for xcb_change_gc. */
pub const XCB_CHANGE_GC : u8 = 56;
/** Opcode for xcb_copy_gc. */
pub const XCB_COPY_GC : u8 = 57;
/** Opcode for xcb_set_dashes. */
pub const XCB_SET_DASHES : u8 = 58;

pub type xcb_clip_ordering_t = c_uint;//{
    pub const XCB_CLIP_ORDERING_UNSORTED : xcb_clip_ordering_t = 0;
    pub const XCB_CLIP_ORDERING_Y_SORTED : xcb_clip_ordering_t = 1;
    pub const XCB_CLIP_ORDERING_YX_SORTED : xcb_clip_ordering_t = 2;
    pub const XCB_CLIP_ORDERING_YX_BANDED : xcb_clip_ordering_t = 3;
//}
/** Opcode for xcb_set_clip_rectangles. */
pub const XCB_SET_CLIP_RECTANGLES : u8 = 59;
/** Opcode for xcb_free_gc. */
pub const XCB_FREE_GC : u8 = 60;
/** Opcode for xcb_clear_area. */
pub const XCB_CLEAR_AREA : u8 = 61;
/** Opcode for xcb_copy_area. */
pub const XCB_COPY_AREA : u8 = 62;
/** Opcode for xcb_copy_plane. */
pub const XCB_COPY_PLANE : u8 = 63;

pub type xcb_coord_mode_t = c_uint;//{
    
/** Treats all coordinates as relative to the origin. */
    pub const XCB_COORD_MODE_ORIGIN : xcb_coord_mode_t = 0;
    
/** Treats all coordinates after the first as relative to the previous coordinate. */
    pub const XCB_COORD_MODE_PREVIOUS : xcb_coord_mode_t = 1;
//}
/** Opcode for xcb_poly_point. */
pub const XCB_POLY_POINT : u8 = 64;
/** Opcode for xcb_poly_line. */
pub const XCB_POLY_LINE : u8 = 65;
pub type SegmentIterator = xcb_segment_iterator_t;

/** Opcode for xcb_poly_segment. */
pub const XCB_POLY_SEGMENT : u8 = 66;
/** Opcode for xcb_poly_rectangle. */
pub const XCB_POLY_RECTANGLE : u8 = 67;
/** Opcode for xcb_poly_arc. */
pub const XCB_POLY_ARC : u8 = 68;

pub type xcb_poly_shape_t = c_uint;//{
    pub const XCB_POLY_SHAPE_COMPLEX : xcb_poly_shape_t = 0;
    pub const XCB_POLY_SHAPE_NONCONVEX : xcb_poly_shape_t = 1;
    pub const XCB_POLY_SHAPE_CONVEX : xcb_poly_shape_t = 2;
//}
/** Opcode for xcb_fill_poly. */
pub const XCB_FILL_POLY : u8 = 69;
/** Opcode for xcb_poly_fill_rectangle. */
pub const XCB_POLY_FILL_RECTANGLE : u8 = 70;
/** Opcode for xcb_poly_fill_arc. */
pub const XCB_POLY_FILL_ARC : u8 = 71;

pub type xcb_image_format_t = c_uint;//{
    pub const XCB_IMAGE_FORMAT_XY_BITMAP : xcb_image_format_t = 0;
    pub const XCB_IMAGE_FORMAT_XY_PIXMAP : xcb_image_format_t = 1;
    pub const XCB_IMAGE_FORMAT_Z_PIXMAP : xcb_image_format_t = 2;
//}
/** Opcode for xcb_put_image. */
pub const XCB_PUT_IMAGE : u8 = 72;
pub struct  GetImageCookie<'s> { pub base : base::Cookie<'s, xcb_get_image_cookie_t> }

/** Opcode for xcb_get_image. */
pub const XCB_GET_IMAGE : u8 = 73;
/** Opcode for xcb_poly_text_8. */
pub const XCB_POLY_TEXT_8 : u8 = 74;
/** Opcode for xcb_poly_text_16. */
pub const XCB_POLY_TEXT_16 : u8 = 75;
/** Opcode for xcb_image_text_8. */
pub const XCB_IMAGE_TEXT_8 : u8 = 76;
/** Opcode for xcb_image_text_16. */
pub const XCB_IMAGE_TEXT_16 : u8 = 77;

pub type xcb_colormap_alloc_t = c_uint;//{
    pub const XCB_COLORMAP_ALLOC_NONE : xcb_colormap_alloc_t = 0;
    pub const XCB_COLORMAP_ALLOC_ALL : xcb_colormap_alloc_t = 1;
//}
/** Opcode for xcb_create_colormap. */
pub const XCB_CREATE_COLORMAP : u8 = 78;
/** Opcode for xcb_free_colormap. */
pub const XCB_FREE_COLORMAP : u8 = 79;
/** Opcode for xcb_copy_colormap_and_free. */
pub const XCB_COPY_COLORMAP_AND_FREE : u8 = 80;
/** Opcode for xcb_install_colormap. */
pub const XCB_INSTALL_COLORMAP : u8 = 81;
/** Opcode for xcb_uninstall_colormap. */
pub const XCB_UNINSTALL_COLORMAP : u8 = 82;
pub struct  ListInstalledColormapsCookie<'s> { pub base : base::Cookie<'s, xcb_list_installed_colormaps_cookie_t> }

/** Opcode for xcb_list_installed_colormaps. */
pub const XCB_LIST_INSTALLED_COLORMAPS : u8 = 83;
pub struct  AllocColorCookie<'s> { pub base : base::Cookie<'s, xcb_alloc_color_cookie_t> }

/** Opcode for xcb_alloc_color. */
pub const XCB_ALLOC_COLOR : u8 = 84;
pub struct AllocColorReply { base:  base::Reply<xcb_alloc_color_reply_t> }
fn mk_reply_xcb_alloc_color_reply_t(reply:*mut xcb_alloc_color_reply_t) -> AllocColorReply { AllocColorReply { base : base::mk_reply(reply) } }
pub struct  AllocNamedColorCookie<'s> { pub base : base::Cookie<'s, xcb_alloc_named_color_cookie_t> }

/** Opcode for xcb_alloc_named_color. */
pub const XCB_ALLOC_NAMED_COLOR : u8 = 85;
pub struct AllocNamedColorReply { base:  base::Reply<xcb_alloc_named_color_reply_t> }
fn mk_reply_xcb_alloc_named_color_reply_t(reply:*mut xcb_alloc_named_color_reply_t) -> AllocNamedColorReply { AllocNamedColorReply { base : base::mk_reply(reply) } }
pub struct  AllocColorCellsCookie<'s> { pub base : base::Cookie<'s, xcb_alloc_color_cells_cookie_t> }

/** Opcode for xcb_alloc_color_cells. */
pub const XCB_ALLOC_COLOR_CELLS : u8 = 86;
pub struct  AllocColorPlanesCookie<'s> { pub base : base::Cookie<'s, xcb_alloc_color_planes_cookie_t> }

/** Opcode for xcb_alloc_color_planes. */
pub const XCB_ALLOC_COLOR_PLANES : u8 = 87;
/** Opcode for xcb_free_colors. */
pub const XCB_FREE_COLORS : u8 = 88;

pub type xcb_color_flag_t = c_uint;//{
    pub const XCB_COLOR_FLAG_RED : xcb_color_flag_t = 1;
    pub const XCB_COLOR_FLAG_GREEN : xcb_color_flag_t = 2;
    pub const XCB_COLOR_FLAG_BLUE : xcb_color_flag_t = 4;
//}
pub struct Coloritem {pub base : base::Struct<xcb_coloritem_t> }

pub type ColoritemIterator = xcb_coloritem_iterator_t;

/** Opcode for xcb_store_colors. */
pub const XCB_STORE_COLORS : u8 = 89;
/** Opcode for xcb_store_named_color. */
pub const XCB_STORE_NAMED_COLOR : u8 = 90;
pub type RgbIterator = xcb_rgb_iterator_t;

pub struct  QueryColorsCookie<'s> { pub base : base::Cookie<'s, xcb_query_colors_cookie_t> }

/** Opcode for xcb_query_colors. */
pub const XCB_QUERY_COLORS : u8 = 91;
pub struct QueryColorsReply { base:  base::Reply<xcb_query_colors_reply_t> }
fn mk_reply_xcb_query_colors_reply_t(reply:*mut xcb_query_colors_reply_t) -> QueryColorsReply { QueryColorsReply { base : base::mk_reply(reply) } }
pub struct  LookupColorCookie<'s> { pub base : base::Cookie<'s, xcb_lookup_color_cookie_t> }

/** Opcode for xcb_lookup_color. */
pub const XCB_LOOKUP_COLOR : u8 = 92;
pub struct LookupColorReply { base:  base::Reply<xcb_lookup_color_reply_t> }
fn mk_reply_xcb_lookup_color_reply_t(reply:*mut xcb_lookup_color_reply_t) -> LookupColorReply { LookupColorReply { base : base::mk_reply(reply) } }

pub type xcb_pixmap_enum_t = c_uint;//{
    pub const XCB_PIXMAP_NONE : xcb_pixmap_enum_t = 0;
//}
/** Opcode for xcb_create_cursor. */
pub const XCB_CREATE_CURSOR : u8 = 93;

pub type xcb_font_enum_t = c_uint;//{
    pub const XCB_FONT_NONE : xcb_font_enum_t = 0;
//}
/** Opcode for xcb_create_glyph_cursor. */
pub const XCB_CREATE_GLYPH_CURSOR : u8 = 94;
/** Opcode for xcb_free_cursor. */
pub const XCB_FREE_CURSOR : u8 = 95;
/** Opcode for xcb_recolor_cursor. */
pub const XCB_RECOLOR_CURSOR : u8 = 96;

pub type xcb_query_shape_of_t = c_uint;//{
    pub const XCB_QUERY_SHAPE_OF_LARGEST_CURSOR : xcb_query_shape_of_t = 0;
    pub const XCB_QUERY_SHAPE_OF_FASTEST_TILE : xcb_query_shape_of_t = 1;
    pub const XCB_QUERY_SHAPE_OF_FASTEST_STIPPLE : xcb_query_shape_of_t = 2;
//}
pub struct  QueryBestSizeCookie<'s> { pub base : base::Cookie<'s, xcb_query_best_size_cookie_t> }

/** Opcode for xcb_query_best_size. */
pub const XCB_QUERY_BEST_SIZE : u8 = 97;
pub struct QueryBestSizeReply { base:  base::Reply<xcb_query_best_size_reply_t> }
fn mk_reply_xcb_query_best_size_reply_t(reply:*mut xcb_query_best_size_reply_t) -> QueryBestSizeReply { QueryBestSizeReply { base : base::mk_reply(reply) } }
pub struct  QueryExtensionCookie<'s> { pub base : base::Cookie<'s, xcb_query_extension_cookie_t> }

/** Opcode for xcb_query_extension. */
pub const XCB_QUERY_EXTENSION : u8 = 98;
pub struct QueryExtensionReply { base:  base::Reply<xcb_query_extension_reply_t> }
fn mk_reply_xcb_query_extension_reply_t(reply:*mut xcb_query_extension_reply_t) -> QueryExtensionReply { QueryExtensionReply { base : base::mk_reply(reply) } }
pub struct  ListExtensionsCookie<'s> { pub base : base::Cookie<'s, xcb_list_extensions_cookie_t> }

/** Opcode for xcb_list_extensions. */
pub const XCB_LIST_EXTENSIONS : u8 = 99;
/** Opcode for xcb_change_keyboard_mapping. */
pub const XCB_CHANGE_KEYBOARD_MAPPING : u8 = 100;
pub struct  GetKeyboardMappingCookie<'s> { pub base : base::Cookie<'s, xcb_get_keyboard_mapping_cookie_t> }

/** Opcode for xcb_get_keyboard_mapping. */
pub const XCB_GET_KEYBOARD_MAPPING : u8 = 101;

pub type xcb_kb_t = c_uint;//{
    pub const XCB_KB_KEY_CLICK_PERCENT : xcb_kb_t = 1;
    pub const XCB_KB_BELL_PERCENT : xcb_kb_t = 2;
    pub const XCB_KB_BELL_PITCH : xcb_kb_t = 4;
    pub const XCB_KB_BELL_DURATION : xcb_kb_t = 8;
    pub const XCB_KB_LED : xcb_kb_t = 16;
    pub const XCB_KB_LED_MODE : xcb_kb_t = 32;
    pub const XCB_KB_KEY : xcb_kb_t = 64;
    pub const XCB_KB_AUTO_REPEAT_MODE : xcb_kb_t = 128;
//}

pub type xcb_led_mode_t = c_uint;//{
    pub const XCB_LED_MODE_OFF : xcb_led_mode_t = 0;
    pub const XCB_LED_MODE_ON : xcb_led_mode_t = 1;
//}

pub type xcb_auto_repeat_mode_t = c_uint;//{
    pub const XCB_AUTO_REPEAT_MODE_OFF : xcb_auto_repeat_mode_t = 0;
    pub const XCB_AUTO_REPEAT_MODE_ON : xcb_auto_repeat_mode_t = 1;
    pub const XCB_AUTO_REPEAT_MODE_DEFAULT : xcb_auto_repeat_mode_t = 2;
//}
/** Opcode for xcb_change_keyboard_control. */
pub const XCB_CHANGE_KEYBOARD_CONTROL : u8 = 102;
pub struct  GetKeyboardControlCookie<'s> { pub base : base::Cookie<'s, xcb_get_keyboard_control_cookie_t> }

/** Opcode for xcb_get_keyboard_control. */
pub const XCB_GET_KEYBOARD_CONTROL : u8 = 103;
pub struct GetKeyboardControlReply { base:  base::Reply<xcb_get_keyboard_control_reply_t> }
fn mk_reply_xcb_get_keyboard_control_reply_t(reply:*mut xcb_get_keyboard_control_reply_t) -> GetKeyboardControlReply { GetKeyboardControlReply { base : base::mk_reply(reply) } }
/** Opcode for xcb_bell. */
pub const XCB_BELL : u8 = 104;
/** Opcode for xcb_change_pointer_control. */
pub const XCB_CHANGE_POINTER_CONTROL : u8 = 105;
pub struct  GetPointerControlCookie<'s> { pub base : base::Cookie<'s, xcb_get_pointer_control_cookie_t> }

/** Opcode for xcb_get_pointer_control. */
pub const XCB_GET_POINTER_CONTROL : u8 = 106;
pub struct GetPointerControlReply { base:  base::Reply<xcb_get_pointer_control_reply_t> }
fn mk_reply_xcb_get_pointer_control_reply_t(reply:*mut xcb_get_pointer_control_reply_t) -> GetPointerControlReply { GetPointerControlReply { base : base::mk_reply(reply) } }

pub type xcb_blanking_t = c_uint;//{
    pub const XCB_BLANKING_NOT_PREFERRED : xcb_blanking_t = 0;
    pub const XCB_BLANKING_PREFERRED : xcb_blanking_t = 1;
    pub const XCB_BLANKING_DEFAULT : xcb_blanking_t = 2;
//}

pub type xcb_exposures_t = c_uint;//{
    pub const XCB_EXPOSURES_NOT_ALLOWED : xcb_exposures_t = 0;
    pub const XCB_EXPOSURES_ALLOWED : xcb_exposures_t = 1;
    pub const XCB_EXPOSURES_DEFAULT : xcb_exposures_t = 2;
//}
/** Opcode for xcb_set_screen_saver. */
pub const XCB_SET_SCREEN_SAVER : u8 = 107;
pub struct  GetScreenSaverCookie<'s> { pub base : base::Cookie<'s, xcb_get_screen_saver_cookie_t> }

/** Opcode for xcb_get_screen_saver. */
pub const XCB_GET_SCREEN_SAVER : u8 = 108;
pub struct GetScreenSaverReply { base:  base::Reply<xcb_get_screen_saver_reply_t> }
fn mk_reply_xcb_get_screen_saver_reply_t(reply:*mut xcb_get_screen_saver_reply_t) -> GetScreenSaverReply { GetScreenSaverReply { base : base::mk_reply(reply) } }

pub type xcb_host_mode_t = c_uint;//{
    pub const XCB_HOST_MODE_INSERT : xcb_host_mode_t = 0;
    pub const XCB_HOST_MODE_DELETE : xcb_host_mode_t = 1;
//}

pub type xcb_family_t = c_uint;//{
    pub const XCB_FAMILY_INTERNET : xcb_family_t = 0;
    pub const XCB_FAMILY_DECNET : xcb_family_t = 1;
    pub const XCB_FAMILY_CHAOS : xcb_family_t = 2;
    pub const XCB_FAMILY_SERVER_INTERPRETED : xcb_family_t = 5;
    pub const XCB_FAMILY_INTERNET_6 : xcb_family_t = 6;
//}
/** Opcode for xcb_change_hosts. */
pub const XCB_CHANGE_HOSTS : u8 = 109;
pub type HostIterator = xcb_host_iterator_t;

pub struct  ListHostsCookie<'s> { pub base : base::Cookie<'s, xcb_list_hosts_cookie_t> }

/** Opcode for xcb_list_hosts. */
pub const XCB_LIST_HOSTS : u8 = 110;

pub type xcb_access_control_t = c_uint;//{
    pub const XCB_ACCESS_CONTROL_DISABLE : xcb_access_control_t = 0;
    pub const XCB_ACCESS_CONTROL_ENABLE : xcb_access_control_t = 1;
//}
/** Opcode for xcb_set_access_control. */
pub const XCB_SET_ACCESS_CONTROL : u8 = 111;

pub type xcb_close_down_t = c_uint;//{
    pub const XCB_CLOSE_DOWN_DESTROY_ALL : xcb_close_down_t = 0;
    pub const XCB_CLOSE_DOWN_RETAIN_PERMANENT : xcb_close_down_t = 1;
    pub const XCB_CLOSE_DOWN_RETAIN_TEMPORARY : xcb_close_down_t = 2;
//}
/** Opcode for xcb_set_close_down_mode. */
pub const XCB_SET_CLOSE_DOWN_MODE : u8 = 112;

pub type xcb_kill_t = c_uint;//{
    pub const XCB_KILL_ALL_TEMPORARY : xcb_kill_t = 0;
//}
/** Opcode for xcb_kill_client. */
pub const XCB_KILL_CLIENT : u8 = 113;
/** Opcode for xcb_rotate_properties. */
pub const XCB_ROTATE_PROPERTIES : u8 = 114;

pub type xcb_screen_saver_t = c_uint;//{
    pub const XCB_SCREEN_SAVER_RESET : xcb_screen_saver_t = 0;
    pub const XCB_SCREEN_SAVER_ACTIVE : xcb_screen_saver_t = 1;
//}
/** Opcode for xcb_force_screen_saver. */
pub const XCB_FORCE_SCREEN_SAVER : u8 = 115;

pub type xcb_mapping_status_t = c_uint;//{
    pub const XCB_MAPPING_STATUS_SUCCESS : xcb_mapping_status_t = 0;
    pub const XCB_MAPPING_STATUS_BUSY : xcb_mapping_status_t = 1;
    pub const XCB_MAPPING_STATUS_FAILURE : xcb_mapping_status_t = 2;
//}
pub struct  SetPointerMappingCookie<'s> { pub base : base::Cookie<'s, xcb_set_pointer_mapping_cookie_t> }

/** Opcode for xcb_set_pointer_mapping. */
pub const XCB_SET_POINTER_MAPPING : u8 = 116;
pub struct SetPointerMappingReply { base:  base::Reply<xcb_set_pointer_mapping_reply_t> }
fn mk_reply_xcb_set_pointer_mapping_reply_t(reply:*mut xcb_set_pointer_mapping_reply_t) -> SetPointerMappingReply { SetPointerMappingReply { base : base::mk_reply(reply) } }
pub struct  GetPointerMappingCookie<'s> { pub base : base::Cookie<'s, xcb_get_pointer_mapping_cookie_t> }

/** Opcode for xcb_get_pointer_mapping. */
pub const XCB_GET_POINTER_MAPPING : u8 = 117;

pub type xcb_map_index_t = c_uint;//{
    pub const XCB_MAP_INDEX_SHIFT : xcb_map_index_t = 0;
    pub const XCB_MAP_INDEX_LOCK : xcb_map_index_t = 1;
    pub const XCB_MAP_INDEX_CONTROL : xcb_map_index_t = 2;
    pub const XCB_MAP_INDEX_1 : xcb_map_index_t = 3;
    pub const XCB_MAP_INDEX_2 : xcb_map_index_t = 4;
    pub const XCB_MAP_INDEX_3 : xcb_map_index_t = 5;
    pub const XCB_MAP_INDEX_4 : xcb_map_index_t = 6;
    pub const XCB_MAP_INDEX_5 : xcb_map_index_t = 7;
//}
pub struct  SetModifierMappingCookie<'s> { pub base : base::Cookie<'s, xcb_set_modifier_mapping_cookie_t> }

/** Opcode for xcb_set_modifier_mapping. */
pub const XCB_SET_MODIFIER_MAPPING : u8 = 118;
pub struct SetModifierMappingReply { base:  base::Reply<xcb_set_modifier_mapping_reply_t> }
fn mk_reply_xcb_set_modifier_mapping_reply_t(reply:*mut xcb_set_modifier_mapping_reply_t) -> SetModifierMappingReply { SetModifierMappingReply { base : base::mk_reply(reply) } }
pub struct  GetModifierMappingCookie<'s> { pub base : base::Cookie<'s, xcb_get_modifier_mapping_cookie_t> }

/** Opcode for xcb_get_modifier_mapping. */
pub const XCB_GET_MODIFIER_MAPPING : u8 = 119;
/** Opcode for xcb_no_operation. */
pub const XCB_NO_OPERATION : u8 = 127;

impl Char2b {
  pub fn byte1(&mut self) -> u8 {
    unsafe { accessor!(byte1 -> u8, self.base.strct) }
  }

  pub fn byte2(&mut self) -> u8 {
    unsafe { accessor!(byte2 -> u8, self.base.strct) }
  }

}

impl Iterator for Char2bIterator {
    type Item = Char2b;
    fn next(&mut self) -> Option<Char2b> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_char2b_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_char2b_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub type Window = xcb_window_t;


impl Iterator for WindowIterator {
    type Item = Window;
    fn next(&mut self) -> Option<Window> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_window_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_window_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub type Pixmap = xcb_pixmap_t;


impl Iterator for PixmapIterator {
    type Item = Pixmap;
    fn next(&mut self) -> Option<Pixmap> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_pixmap_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_pixmap_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub type Cursor = xcb_cursor_t;


impl Iterator for CursorIterator {
    type Item = Cursor;
    fn next(&mut self) -> Option<Cursor> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_cursor_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_cursor_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub type Font = xcb_font_t;


impl Iterator for FontIterator {
    type Item = Font;
    fn next(&mut self) -> Option<Font> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_font_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_font_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub type Gcontext = xcb_gcontext_t;


impl Iterator for GcontextIterator {
    type Item = Gcontext;
    fn next(&mut self) -> Option<Gcontext> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_gcontext_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_gcontext_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub type Colormap = xcb_colormap_t;


impl Iterator for ColormapIterator {
    type Item = Colormap;
    fn next(&mut self) -> Option<Colormap> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_colormap_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_colormap_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub type Atom = xcb_atom_t;


impl Iterator for AtomIterator {
    type Item = Atom;
    fn next(&mut self) -> Option<Atom> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_atom_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_atom_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub type Drawable = xcb_drawable_t;


impl Iterator for DrawableIterator {
    type Item = Drawable;
    fn next(&mut self) -> Option<Drawable> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_drawable_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_drawable_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub type Fontable = xcb_fontable_t;


impl Iterator for FontableIterator {
    type Item = Fontable;
    fn next(&mut self) -> Option<Fontable> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_fontable_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_fontable_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub type Visualid = xcb_visualid_t;


impl Iterator for VisualidIterator {
    type Item = Visualid;
    fn next(&mut self) -> Option<Visualid> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_visualid_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_visualid_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub type Timestamp = xcb_timestamp_t;


impl Iterator for TimestampIterator {
    type Item = Timestamp;
    fn next(&mut self) -> Option<Timestamp> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_timestamp_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_timestamp_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub type Keysym = xcb_keysym_t;


impl Iterator for KeysymIterator {
    type Item = Keysym;
    fn next(&mut self) -> Option<Keysym> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_keysym_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_keysym_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub type Keycode = xcb_keycode_t;


impl Iterator for KeycodeIterator {
    type Item = Keycode;
    fn next(&mut self) -> Option<Keycode> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_keycode_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_keycode_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub type Button = xcb_button_t;


impl Iterator for ButtonIterator {
    type Item = Button;
    fn next(&mut self) -> Option<Button> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_button_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_button_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct Point {pub base : base::Struct<xcb_point_t> }


impl Point {
  pub fn x(&mut self) -> i16 {
    unsafe { accessor!(x -> i16, self.base.strct) }
  }

  pub fn y(&mut self) -> i16 {
    unsafe { accessor!(y -> i16, self.base.strct) }
  }

}

impl Iterator for PointIterator {
    type Item = Point;
    fn next(&mut self) -> Option<Point> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_point_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_point_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct Rectangle {pub base : base::Struct<xcb_rectangle_t> }


impl Rectangle {
  pub fn x(&mut self) -> i16 {
    unsafe { accessor!(x -> i16, self.base.strct) }
  }

  pub fn y(&mut self) -> i16 {
    unsafe { accessor!(y -> i16, self.base.strct) }
  }

  pub fn width(&mut self) -> u16 {
    unsafe { accessor!(width -> u16, self.base.strct) }
  }

  pub fn height(&mut self) -> u16 {
    unsafe { accessor!(height -> u16, self.base.strct) }
  }

}

impl Iterator for RectangleIterator {
    type Item = Rectangle;
    fn next(&mut self) -> Option<Rectangle> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_rectangle_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_rectangle_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct Arc {pub base : base::Struct<xcb_arc_t> }


impl Arc {
  pub fn x(&mut self) -> i16 {
    unsafe { accessor!(x -> i16, self.base.strct) }
  }

  pub fn y(&mut self) -> i16 {
    unsafe { accessor!(y -> i16, self.base.strct) }
  }

  pub fn width(&mut self) -> u16 {
    unsafe { accessor!(width -> u16, self.base.strct) }
  }

  pub fn height(&mut self) -> u16 {
    unsafe { accessor!(height -> u16, self.base.strct) }
  }

  pub fn angle1(&mut self) -> i16 {
    unsafe { accessor!(angle1 -> i16, self.base.strct) }
  }

  pub fn angle2(&mut self) -> i16 {
    unsafe { accessor!(angle2 -> i16, self.base.strct) }
  }

}

impl Iterator for ArcIterator {
    type Item = Arc;
    fn next(&mut self) -> Option<Arc> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_arc_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_arc_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct Format {pub base : base::Struct<xcb_format_t> }


impl Format {
  pub fn depth(&mut self) -> u8 {
    unsafe { accessor!(depth -> u8, self.base.strct) }
  }

  pub fn bits_per_pixel(&mut self) -> u8 {
    unsafe { accessor!(bits_per_pixel -> u8, self.base.strct) }
  }

  pub fn scanline_pad(&mut self) -> u8 {
    unsafe { accessor!(scanline_pad -> u8, self.base.strct) }
  }

}

impl Iterator for FormatIterator {
    type Item = Format;
    fn next(&mut self) -> Option<Format> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_format_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_format_next(iter);
            Some(mem::transmute(*data))
        }
    }
}


impl Visualtype {
  pub fn visual_id(&mut self) -> Visualid {
    unsafe { accessor!(visual_id -> Visualid, self.base.strct) }
  }

  pub fn class(&mut self) -> u8 {
    unsafe { accessor!(class -> u8, self.base.strct) }
  }

  pub fn bits_per_rgb_value(&mut self) -> u8 {
    unsafe { accessor!(bits_per_rgb_value -> u8, self.base.strct) }
  }

  pub fn colormap_entries(&mut self) -> u16 {
    unsafe { accessor!(colormap_entries -> u16, self.base.strct) }
  }

  pub fn red_mask(&mut self) -> u32 {
    unsafe { accessor!(red_mask -> u32, self.base.strct) }
  }

  pub fn green_mask(&mut self) -> u32 {
    unsafe { accessor!(green_mask -> u32, self.base.strct) }
  }

  pub fn blue_mask(&mut self) -> u32 {
    unsafe { accessor!(blue_mask -> u32, self.base.strct) }
  }

}

impl Iterator for VisualtypeIterator {
    type Item = Visualtype;
    fn next(&mut self) -> Option<Visualtype> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_visualtype_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_visualtype_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct Depth {pub base : base::Struct<xcb_depth_t> }


impl Depth {
  pub fn depth(&mut self) -> u8 {
    unsafe { accessor!(depth -> u8, self.base.strct) }
  }

  pub fn visuals(&mut self) -> VisualtypeIterator {
    unsafe { accessor!(VisualtypeIterator, xcb_depth_visuals_iterator, self.base.strct) }
  }

}

impl Iterator for DepthIterator {
    type Item = Depth;
    fn next(&mut self) -> Option<Depth> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_depth_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_depth_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct Screen {pub base : base::Struct<xcb_screen_t> }


impl Screen {
  pub fn root(&mut self) -> Window {
    unsafe { accessor!(root -> Window, self.base.strct) }
  }

  pub fn default_colormap(&mut self) -> Colormap {
    unsafe { accessor!(default_colormap -> Colormap, self.base.strct) }
  }

  pub fn white_pixel(&mut self) -> u32 {
    unsafe { accessor!(white_pixel -> u32, self.base.strct) }
  }

  pub fn black_pixel(&mut self) -> u32 {
    unsafe { accessor!(black_pixel -> u32, self.base.strct) }
  }

  pub fn current_input_masks(&mut self) -> u32 {
    unsafe { accessor!(current_input_masks -> u32, self.base.strct) }
  }

  pub fn width_in_pixels(&mut self) -> u16 {
    unsafe { accessor!(width_in_pixels -> u16, self.base.strct) }
  }

  pub fn height_in_pixels(&mut self) -> u16 {
    unsafe { accessor!(height_in_pixels -> u16, self.base.strct) }
  }

  pub fn width_in_millimeters(&mut self) -> u16 {
    unsafe { accessor!(width_in_millimeters -> u16, self.base.strct) }
  }

  pub fn height_in_millimeters(&mut self) -> u16 {
    unsafe { accessor!(height_in_millimeters -> u16, self.base.strct) }
  }

  pub fn min_installed_maps(&mut self) -> u16 {
    unsafe { accessor!(min_installed_maps -> u16, self.base.strct) }
  }

  pub fn max_installed_maps(&mut self) -> u16 {
    unsafe { accessor!(max_installed_maps -> u16, self.base.strct) }
  }

  pub fn root_visual(&mut self) -> Visualid {
    unsafe { accessor!(root_visual -> Visualid, self.base.strct) }
  }

  pub fn backing_stores(&mut self) -> u8 {
    unsafe { accessor!(backing_stores -> u8, self.base.strct) }
  }

  pub fn save_unders(&mut self) -> u8 {
    unsafe { accessor!(save_unders -> u8, self.base.strct) }
  }

  pub fn root_depth(&mut self) -> u8 {
    unsafe { accessor!(root_depth -> u8, self.base.strct) }
  }

  pub fn allowed_depths(&mut self) -> DepthIterator {
    unsafe { accessor!(DepthIterator, xcb_screen_allowed_depths_iterator, self.base.strct) }
  }

}

impl Iterator for ScreenIterator {
    type Item = Screen;
    fn next(&mut self) -> Option<Screen> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_screen_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_screen_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct SetupRequest {pub base : base::Struct<xcb_setup_request_t> }


impl SetupRequest {
  pub fn byte_order(&mut self) -> u8 {
    unsafe { accessor!(byte_order -> u8, self.base.strct) }
  }

  pub fn protocol_major_version(&mut self) -> u16 {
    unsafe { accessor!(protocol_major_version -> u16, self.base.strct) }
  }

  pub fn protocol_minor_version(&mut self) -> u16 {
    unsafe { accessor!(protocol_minor_version -> u16, self.base.strct) }
  }

  pub fn authorization_protocol_name(&mut self) -> String {
    unsafe { accessor!(str, xcb_setup_request_authorization_protocol_name_length, xcb_setup_request_authorization_protocol_name, self.base.strct) }
  }

  pub fn authorization_protocol_data(&mut self) -> String {
    unsafe { accessor!(str, xcb_setup_request_authorization_protocol_data_length, xcb_setup_request_authorization_protocol_data, self.base.strct) }
  }

}

impl Iterator for SetupRequestIterator {
    type Item = SetupRequest;
    fn next(&mut self) -> Option<SetupRequest> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_setup_request_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_setup_request_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct SetupFailed {pub base : base::Struct<xcb_setup_failed_t> }


impl SetupFailed {
  pub fn status(&mut self) -> u8 {
    unsafe { accessor!(status -> u8, self.base.strct) }
  }

  pub fn protocol_major_version(&mut self) -> u16 {
    unsafe { accessor!(protocol_major_version -> u16, self.base.strct) }
  }

  pub fn protocol_minor_version(&mut self) -> u16 {
    unsafe { accessor!(protocol_minor_version -> u16, self.base.strct) }
  }

  pub fn length(&mut self) -> u16 {
    unsafe { accessor!(length -> u16, self.base.strct) }
  }

  pub fn reason(&mut self) -> String {
    unsafe { accessor!(str, xcb_setup_failed_reason_length, xcb_setup_failed_reason, self.base.strct) }
  }

}

impl Iterator for SetupFailedIterator {
    type Item = SetupFailed;
    fn next(&mut self) -> Option<SetupFailed> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_setup_failed_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_setup_failed_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct SetupAuthenticate {pub base : base::Struct<xcb_setup_authenticate_t> }


impl SetupAuthenticate {
  pub fn status(&mut self) -> u8 {
    unsafe { accessor!(status -> u8, self.base.strct) }
  }

  pub fn reason(&mut self) -> String {
    unsafe { accessor!(str, xcb_setup_authenticate_reason_length, xcb_setup_authenticate_reason, self.base.strct) }
  }

}

impl Iterator for SetupAuthenticateIterator {
    type Item = SetupAuthenticate;
    fn next(&mut self) -> Option<SetupAuthenticate> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_setup_authenticate_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_setup_authenticate_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct Setup<'a> {pub base : base::StructPtr<'a, xcb_setup_t>}


impl<'a> Setup<'a> {
  pub fn status(&mut self) -> u8 {
    unsafe { accessor!(status -> u8, *self.base.ptr) }
  }

  pub fn protocol_major_version(&mut self) -> u16 {
    unsafe { accessor!(protocol_major_version -> u16, *self.base.ptr) }
  }

  pub fn protocol_minor_version(&mut self) -> u16 {
    unsafe { accessor!(protocol_minor_version -> u16, *self.base.ptr) }
  }

  pub fn length(&mut self) -> u16 {
    unsafe { accessor!(length -> u16, *self.base.ptr) }
  }

  pub fn release_number(&mut self) -> u32 {
    unsafe { accessor!(release_number -> u32, *self.base.ptr) }
  }

  pub fn resource_id_base(&mut self) -> u32 {
    unsafe { accessor!(resource_id_base -> u32, *self.base.ptr) }
  }

  pub fn resource_id_mask(&mut self) -> u32 {
    unsafe { accessor!(resource_id_mask -> u32, *self.base.ptr) }
  }

  pub fn motion_buffer_size(&mut self) -> u32 {
    unsafe { accessor!(motion_buffer_size -> u32, *self.base.ptr) }
  }

  pub fn maximum_request_length(&mut self) -> u16 {
    unsafe { accessor!(maximum_request_length -> u16, *self.base.ptr) }
  }

  pub fn image_byte_order(&mut self) -> u8 {
    unsafe { accessor!(image_byte_order -> u8, *self.base.ptr) }
  }

  pub fn bitmap_format_bit_order(&mut self) -> u8 {
    unsafe { accessor!(bitmap_format_bit_order -> u8, *self.base.ptr) }
  }

  pub fn bitmap_format_scanline_unit(&mut self) -> u8 {
    unsafe { accessor!(bitmap_format_scanline_unit -> u8, *self.base.ptr) }
  }

  pub fn bitmap_format_scanline_pad(&mut self) -> u8 {
    unsafe { accessor!(bitmap_format_scanline_pad -> u8, *self.base.ptr) }
  }

  pub fn min_keycode(&mut self) -> Keycode {
    unsafe { accessor!(min_keycode -> Keycode, *self.base.ptr) }
  }

  pub fn max_keycode(&mut self) -> Keycode {
    unsafe { accessor!(max_keycode -> Keycode, *self.base.ptr) }
  }

  pub fn vendor(&mut self) -> String {
    unsafe { accessor!(str, xcb_setup_vendor_length, xcb_setup_vendor, *self.base.ptr) }
  }

  pub fn pixmap_formats(&mut self) -> FormatIterator {
    unsafe { accessor!(FormatIterator, xcb_setup_pixmap_formats_iterator, *self.base.ptr) }
  }

  pub fn roots(&mut self) -> ScreenIterator {
    unsafe { accessor!(ScreenIterator, xcb_setup_roots_iterator, *self.base.ptr) }
  }

}

impl KeyPressEvent {
  pub fn detail(&mut self) -> Keycode {
    unsafe { accessor!(detail -> Keycode, (*self.base.event)) }
  }

  pub fn time(&mut self) -> Timestamp {
    unsafe { accessor!(time -> Timestamp, (*self.base.event)) }
  }

  pub fn root(&mut self) -> Window {
    unsafe { accessor!(root -> Window, (*self.base.event)) }
  }

  pub fn event(&mut self) -> Window {
    unsafe { accessor!(event -> Window, (*self.base.event)) }
  }

  pub fn child(&mut self) -> Window {
    unsafe { accessor!(child -> Window, (*self.base.event)) }
  }

  pub fn root_x(&mut self) -> i16 {
    unsafe { accessor!(root_x -> i16, (*self.base.event)) }
  }

  pub fn root_y(&mut self) -> i16 {
    unsafe { accessor!(root_y -> i16, (*self.base.event)) }
  }

  pub fn event_x(&mut self) -> i16 {
    unsafe { accessor!(event_x -> i16, (*self.base.event)) }
  }

  pub fn event_y(&mut self) -> i16 {
    unsafe { accessor!(event_y -> i16, (*self.base.event)) }
  }

  pub fn state(&mut self) -> u16 {
    unsafe { accessor!(state -> u16, (*self.base.event)) }
  }

  pub fn same_screen(&mut self) -> u8 {
    unsafe { accessor!(same_screen -> u8, (*self.base.event)) }
  }

  pub fn new(detail : Keycode,
         time : Timestamp,
         root : Window,
         event : Window,
         child : Window,
         root_x : i16,
         root_y : i16,
         event_x : i16,
         event_y : i16,
         state : u16,
         same_screen : u8) -> KeyPressEvent {
    unsafe {
      let raw = malloc(32 as size_t) as *mut xcb_key_press_event_t;
      (*raw).detail = detail;
      (*raw).time = time;
      (*raw).root = root;
      (*raw).event = event;
      (*raw).child = child;
      (*raw).root_x = root_x;
      (*raw).root_y = root_y;
      (*raw).event_x = event_x;
      (*raw).event_y = event_y;
      (*raw).state = state;
      (*raw).same_screen = same_screen;
      KeyPressEvent { base : Event { event : raw as *mut xcb_key_press_event_t }}
    }
  }
}

impl ButtonPressEvent {
  pub fn detail(&mut self) -> Button {
    unsafe { accessor!(detail -> Button, (*self.base.event)) }
  }

  pub fn time(&mut self) -> Timestamp {
    unsafe { accessor!(time -> Timestamp, (*self.base.event)) }
  }

  pub fn root(&mut self) -> Window {
    unsafe { accessor!(root -> Window, (*self.base.event)) }
  }

  pub fn event(&mut self) -> Window {
    unsafe { accessor!(event -> Window, (*self.base.event)) }
  }

  pub fn child(&mut self) -> Window {
    unsafe { accessor!(child -> Window, (*self.base.event)) }
  }

  pub fn root_x(&mut self) -> i16 {
    unsafe { accessor!(root_x -> i16, (*self.base.event)) }
  }

  pub fn root_y(&mut self) -> i16 {
    unsafe { accessor!(root_y -> i16, (*self.base.event)) }
  }

  pub fn event_x(&mut self) -> i16 {
    unsafe { accessor!(event_x -> i16, (*self.base.event)) }
  }

  pub fn event_y(&mut self) -> i16 {
    unsafe { accessor!(event_y -> i16, (*self.base.event)) }
  }

  pub fn state(&mut self) -> u16 {
    unsafe { accessor!(state -> u16, (*self.base.event)) }
  }

  pub fn same_screen(&mut self) -> u8 {
    unsafe { accessor!(same_screen -> u8, (*self.base.event)) }
  }

  pub fn new(detail : Button,
         time : Timestamp,
         root : Window,
         event : Window,
         child : Window,
         root_x : i16,
         root_y : i16,
         event_x : i16,
         event_y : i16,
         state : u16,
         same_screen : u8) -> ButtonPressEvent {
    unsafe {
      let raw = malloc(32 as size_t) as *mut xcb_button_press_event_t;
      (*raw).detail = detail;
      (*raw).time = time;
      (*raw).root = root;
      (*raw).event = event;
      (*raw).child = child;
      (*raw).root_x = root_x;
      (*raw).root_y = root_y;
      (*raw).event_x = event_x;
      (*raw).event_y = event_y;
      (*raw).state = state;
      (*raw).same_screen = same_screen;
      ButtonPressEvent { base : Event { event : raw as *mut xcb_button_press_event_t }}
    }
  }
}

impl MotionNotifyEvent {
  pub fn detail(&mut self) -> u8 {
    unsafe { accessor!(detail -> u8, (*self.base.event)) }
  }

  pub fn time(&mut self) -> Timestamp {
    unsafe { accessor!(time -> Timestamp, (*self.base.event)) }
  }

  pub fn root(&mut self) -> Window {
    unsafe { accessor!(root -> Window, (*self.base.event)) }
  }

  pub fn event(&mut self) -> Window {
    unsafe { accessor!(event -> Window, (*self.base.event)) }
  }

  pub fn child(&mut self) -> Window {
    unsafe { accessor!(child -> Window, (*self.base.event)) }
  }

  pub fn root_x(&mut self) -> i16 {
    unsafe { accessor!(root_x -> i16, (*self.base.event)) }
  }

  pub fn root_y(&mut self) -> i16 {
    unsafe { accessor!(root_y -> i16, (*self.base.event)) }
  }

  pub fn event_x(&mut self) -> i16 {
    unsafe { accessor!(event_x -> i16, (*self.base.event)) }
  }

  pub fn event_y(&mut self) -> i16 {
    unsafe { accessor!(event_y -> i16, (*self.base.event)) }
  }

  pub fn state(&mut self) -> u16 {
    unsafe { accessor!(state -> u16, (*self.base.event)) }
  }

  pub fn same_screen(&mut self) -> u8 {
    unsafe { accessor!(same_screen -> u8, (*self.base.event)) }
  }

  pub fn new(detail : u8,
         time : Timestamp,
         root : Window,
         event : Window,
         child : Window,
         root_x : i16,
         root_y : i16,
         event_x : i16,
         event_y : i16,
         state : u16,
         same_screen : u8) -> MotionNotifyEvent {
    unsafe {
      let raw = malloc(32 as size_t) as *mut xcb_motion_notify_event_t;
      (*raw).detail = detail;
      (*raw).time = time;
      (*raw).root = root;
      (*raw).event = event;
      (*raw).child = child;
      (*raw).root_x = root_x;
      (*raw).root_y = root_y;
      (*raw).event_x = event_x;
      (*raw).event_y = event_y;
      (*raw).state = state;
      (*raw).same_screen = same_screen;
      MotionNotifyEvent { base : Event { event : raw as *mut xcb_motion_notify_event_t }}
    }
  }
}

impl EnterNotifyEvent {
  pub fn detail(&mut self) -> u8 {
    unsafe { accessor!(detail -> u8, (*self.base.event)) }
  }

  pub fn time(&mut self) -> Timestamp {
    unsafe { accessor!(time -> Timestamp, (*self.base.event)) }
  }

  pub fn root(&mut self) -> Window {
    unsafe { accessor!(root -> Window, (*self.base.event)) }
  }

  pub fn event(&mut self) -> Window {
    unsafe { accessor!(event -> Window, (*self.base.event)) }
  }

  pub fn child(&mut self) -> Window {
    unsafe { accessor!(child -> Window, (*self.base.event)) }
  }

  pub fn root_x(&mut self) -> i16 {
    unsafe { accessor!(root_x -> i16, (*self.base.event)) }
  }

  pub fn root_y(&mut self) -> i16 {
    unsafe { accessor!(root_y -> i16, (*self.base.event)) }
  }

  pub fn event_x(&mut self) -> i16 {
    unsafe { accessor!(event_x -> i16, (*self.base.event)) }
  }

  pub fn event_y(&mut self) -> i16 {
    unsafe { accessor!(event_y -> i16, (*self.base.event)) }
  }

  pub fn state(&mut self) -> u16 {
    unsafe { accessor!(state -> u16, (*self.base.event)) }
  }

  pub fn mode(&mut self) -> u8 {
    unsafe { accessor!(mode -> u8, (*self.base.event)) }
  }

  pub fn same_screen_focus(&mut self) -> u8 {
    unsafe { accessor!(same_screen_focus -> u8, (*self.base.event)) }
  }

  pub fn new(detail : u8,
         time : Timestamp,
         root : Window,
         event : Window,
         child : Window,
         root_x : i16,
         root_y : i16,
         event_x : i16,
         event_y : i16,
         state : u16,
         mode : u8,
         same_screen_focus : u8) -> EnterNotifyEvent {
    unsafe {
      let raw = malloc(32 as size_t) as *mut xcb_enter_notify_event_t;
      (*raw).detail = detail;
      (*raw).time = time;
      (*raw).root = root;
      (*raw).event = event;
      (*raw).child = child;
      (*raw).root_x = root_x;
      (*raw).root_y = root_y;
      (*raw).event_x = event_x;
      (*raw).event_y = event_y;
      (*raw).state = state;
      (*raw).mode = mode;
      (*raw).same_screen_focus = same_screen_focus;
      EnterNotifyEvent { base : Event { event : raw as *mut xcb_enter_notify_event_t }}
    }
  }
}

impl FocusInEvent {
  pub fn detail(&mut self) -> u8 {
    unsafe { accessor!(detail -> u8, (*self.base.event)) }
  }

  pub fn event(&mut self) -> Window {
    unsafe { accessor!(event -> Window, (*self.base.event)) }
  }

  pub fn mode(&mut self) -> u8 {
    unsafe { accessor!(mode -> u8, (*self.base.event)) }
  }

  pub fn new(detail : u8,
         event : Window,
         mode : u8) -> FocusInEvent {
    unsafe {
      let raw = malloc(32 as size_t) as *mut xcb_focus_in_event_t;
      (*raw).detail = detail;
      (*raw).event = event;
      (*raw).mode = mode;
      FocusInEvent { base : Event { event : raw as *mut xcb_focus_in_event_t }}
    }
  }
}

impl KeymapNotifyEvent {
  pub fn keys(&self) -> Vec<u8> {
    unsafe { ((*self.base.event).keys).to_vec() }
  }

  pub fn new(keys : [u8; 31]) -> KeymapNotifyEvent {
    unsafe {
      let raw = malloc(32 as size_t) as *mut xcb_keymap_notify_event_t;
      (*raw).keys = keys;
      KeymapNotifyEvent { base : Event { event : raw as *mut xcb_keymap_notify_event_t }}
    }
  }
}

impl ExposeEvent {
  pub fn window(&mut self) -> Window {
    unsafe { accessor!(window -> Window, (*self.base.event)) }
  }

  pub fn x(&mut self) -> u16 {
    unsafe { accessor!(x -> u16, (*self.base.event)) }
  }

  pub fn y(&mut self) -> u16 {
    unsafe { accessor!(y -> u16, (*self.base.event)) }
  }

  pub fn width(&mut self) -> u16 {
    unsafe { accessor!(width -> u16, (*self.base.event)) }
  }

  pub fn height(&mut self) -> u16 {
    unsafe { accessor!(height -> u16, (*self.base.event)) }
  }

  pub fn count(&mut self) -> u16 {
    unsafe { accessor!(count -> u16, (*self.base.event)) }
  }

  pub fn new(window : Window,
         x : u16,
         y : u16,
         width : u16,
         height : u16,
         count : u16) -> ExposeEvent {
    unsafe {
      let raw = malloc(32 as size_t) as *mut xcb_expose_event_t;
      (*raw).window = window;
      (*raw).x = x;
      (*raw).y = y;
      (*raw).width = width;
      (*raw).height = height;
      (*raw).count = count;
      ExposeEvent { base : Event { event : raw as *mut xcb_expose_event_t }}
    }
  }
}

impl GraphicsExposureEvent {
  pub fn drawable(&mut self) -> Drawable {
    unsafe { accessor!(drawable -> Drawable, (*self.base.event)) }
  }

  pub fn x(&mut self) -> u16 {
    unsafe { accessor!(x -> u16, (*self.base.event)) }
  }

  pub fn y(&mut self) -> u16 {
    unsafe { accessor!(y -> u16, (*self.base.event)) }
  }

  pub fn width(&mut self) -> u16 {
    unsafe { accessor!(width -> u16, (*self.base.event)) }
  }

  pub fn height(&mut self) -> u16 {
    unsafe { accessor!(height -> u16, (*self.base.event)) }
  }

  pub fn minor_opcode(&mut self) -> u16 {
    unsafe { accessor!(minor_opcode -> u16, (*self.base.event)) }
  }

  pub fn count(&mut self) -> u16 {
    unsafe { accessor!(count -> u16, (*self.base.event)) }
  }

  pub fn major_opcode(&mut self) -> u8 {
    unsafe { accessor!(major_opcode -> u8, (*self.base.event)) }
  }

  pub fn new(drawable : Drawable,
         x : u16,
         y : u16,
         width : u16,
         height : u16,
         minor_opcode : u16,
         count : u16,
         major_opcode : u8) -> GraphicsExposureEvent {
    unsafe {
      let raw = malloc(32 as size_t) as *mut xcb_graphics_exposure_event_t;
      (*raw).drawable = drawable;
      (*raw).x = x;
      (*raw).y = y;
      (*raw).width = width;
      (*raw).height = height;
      (*raw).minor_opcode = minor_opcode;
      (*raw).count = count;
      (*raw).major_opcode = major_opcode;
      GraphicsExposureEvent { base : Event { event : raw as *mut xcb_graphics_exposure_event_t }}
    }
  }
}

impl NoExposureEvent {
  pub fn drawable(&mut self) -> Drawable {
    unsafe { accessor!(drawable -> Drawable, (*self.base.event)) }
  }

  pub fn minor_opcode(&mut self) -> u16 {
    unsafe { accessor!(minor_opcode -> u16, (*self.base.event)) }
  }

  pub fn major_opcode(&mut self) -> u8 {
    unsafe { accessor!(major_opcode -> u8, (*self.base.event)) }
  }

  pub fn new(drawable : Drawable,
         minor_opcode : u16,
         major_opcode : u8) -> NoExposureEvent {
    unsafe {
      let raw = malloc(32 as size_t) as *mut xcb_no_exposure_event_t;
      (*raw).drawable = drawable;
      (*raw).minor_opcode = minor_opcode;
      (*raw).major_opcode = major_opcode;
      NoExposureEvent { base : Event { event : raw as *mut xcb_no_exposure_event_t }}
    }
  }
}

impl VisibilityNotifyEvent {
  pub fn window(&mut self) -> Window {
    unsafe { accessor!(window -> Window, (*self.base.event)) }
  }

  pub fn state(&mut self) -> u8 {
    unsafe { accessor!(state -> u8, (*self.base.event)) }
  }

  pub fn new(window : Window,
         state : u8) -> VisibilityNotifyEvent {
    unsafe {
      let raw = malloc(32 as size_t) as *mut xcb_visibility_notify_event_t;
      (*raw).window = window;
      (*raw).state = state;
      VisibilityNotifyEvent { base : Event { event : raw as *mut xcb_visibility_notify_event_t }}
    }
  }
}

impl CreateNotifyEvent {
  pub fn parent(&mut self) -> Window {
    unsafe { accessor!(parent -> Window, (*self.base.event)) }
  }

  pub fn window(&mut self) -> Window {
    unsafe { accessor!(window -> Window, (*self.base.event)) }
  }

  pub fn x(&mut self) -> i16 {
    unsafe { accessor!(x -> i16, (*self.base.event)) }
  }

  pub fn y(&mut self) -> i16 {
    unsafe { accessor!(y -> i16, (*self.base.event)) }
  }

  pub fn width(&mut self) -> u16 {
    unsafe { accessor!(width -> u16, (*self.base.event)) }
  }

  pub fn height(&mut self) -> u16 {
    unsafe { accessor!(height -> u16, (*self.base.event)) }
  }

  pub fn border_width(&mut self) -> u16 {
    unsafe { accessor!(border_width -> u16, (*self.base.event)) }
  }

  pub fn override_redirect(&mut self) -> u8 {
    unsafe { accessor!(override_redirect -> u8, (*self.base.event)) }
  }

  pub fn new(parent : Window,
         window : Window,
         x : i16,
         y : i16,
         width : u16,
         height : u16,
         border_width : u16,
         override_redirect : u8) -> CreateNotifyEvent {
    unsafe {
      let raw = malloc(32 as size_t) as *mut xcb_create_notify_event_t;
      (*raw).parent = parent;
      (*raw).window = window;
      (*raw).x = x;
      (*raw).y = y;
      (*raw).width = width;
      (*raw).height = height;
      (*raw).border_width = border_width;
      (*raw).override_redirect = override_redirect;
      CreateNotifyEvent { base : Event { event : raw as *mut xcb_create_notify_event_t }}
    }
  }
}

impl DestroyNotifyEvent {
  pub fn event(&mut self) -> Window {
    unsafe { accessor!(event -> Window, (*self.base.event)) }
  }

  pub fn window(&mut self) -> Window {
    unsafe { accessor!(window -> Window, (*self.base.event)) }
  }

  pub fn new(event : Window,
         window : Window) -> DestroyNotifyEvent {
    unsafe {
      let raw = malloc(32 as size_t) as *mut xcb_destroy_notify_event_t;
      (*raw).event = event;
      (*raw).window = window;
      DestroyNotifyEvent { base : Event { event : raw as *mut xcb_destroy_notify_event_t }}
    }
  }
}

impl UnmapNotifyEvent {
  pub fn event(&mut self) -> Window {
    unsafe { accessor!(event -> Window, (*self.base.event)) }
  }

  pub fn window(&mut self) -> Window {
    unsafe { accessor!(window -> Window, (*self.base.event)) }
  }

  pub fn from_configure(&mut self) -> u8 {
    unsafe { accessor!(from_configure -> u8, (*self.base.event)) }
  }

  pub fn new(event : Window,
         window : Window,
         from_configure : u8) -> UnmapNotifyEvent {
    unsafe {
      let raw = malloc(32 as size_t) as *mut xcb_unmap_notify_event_t;
      (*raw).event = event;
      (*raw).window = window;
      (*raw).from_configure = from_configure;
      UnmapNotifyEvent { base : Event { event : raw as *mut xcb_unmap_notify_event_t }}
    }
  }
}

impl MapNotifyEvent {
  pub fn event(&mut self) -> Window {
    unsafe { accessor!(event -> Window, (*self.base.event)) }
  }

  pub fn window(&mut self) -> Window {
    unsafe { accessor!(window -> Window, (*self.base.event)) }
  }

  pub fn override_redirect(&mut self) -> u8 {
    unsafe { accessor!(override_redirect -> u8, (*self.base.event)) }
  }

  pub fn new(event : Window,
         window : Window,
         override_redirect : u8) -> MapNotifyEvent {
    unsafe {
      let raw = malloc(32 as size_t) as *mut xcb_map_notify_event_t;
      (*raw).event = event;
      (*raw).window = window;
      (*raw).override_redirect = override_redirect;
      MapNotifyEvent { base : Event { event : raw as *mut xcb_map_notify_event_t }}
    }
  }
}

impl MapRequestEvent {
  pub fn parent(&mut self) -> Window {
    unsafe { accessor!(parent -> Window, (*self.base.event)) }
  }

  pub fn window(&mut self) -> Window {
    unsafe { accessor!(window -> Window, (*self.base.event)) }
  }

  pub fn new(parent : Window,
         window : Window) -> MapRequestEvent {
    unsafe {
      let raw = malloc(32 as size_t) as *mut xcb_map_request_event_t;
      (*raw).parent = parent;
      (*raw).window = window;
      MapRequestEvent { base : Event { event : raw as *mut xcb_map_request_event_t }}
    }
  }
}

impl ReparentNotifyEvent {
  pub fn event(&mut self) -> Window {
    unsafe { accessor!(event -> Window, (*self.base.event)) }
  }

  pub fn window(&mut self) -> Window {
    unsafe { accessor!(window -> Window, (*self.base.event)) }
  }

  pub fn parent(&mut self) -> Window {
    unsafe { accessor!(parent -> Window, (*self.base.event)) }
  }

  pub fn x(&mut self) -> i16 {
    unsafe { accessor!(x -> i16, (*self.base.event)) }
  }

  pub fn y(&mut self) -> i16 {
    unsafe { accessor!(y -> i16, (*self.base.event)) }
  }

  pub fn override_redirect(&mut self) -> u8 {
    unsafe { accessor!(override_redirect -> u8, (*self.base.event)) }
  }

  pub fn new(event : Window,
         window : Window,
         parent : Window,
         x : i16,
         y : i16,
         override_redirect : u8) -> ReparentNotifyEvent {
    unsafe {
      let raw = malloc(32 as size_t) as *mut xcb_reparent_notify_event_t;
      (*raw).event = event;
      (*raw).window = window;
      (*raw).parent = parent;
      (*raw).x = x;
      (*raw).y = y;
      (*raw).override_redirect = override_redirect;
      ReparentNotifyEvent { base : Event { event : raw as *mut xcb_reparent_notify_event_t }}
    }
  }
}

impl ConfigureNotifyEvent {
  pub fn event(&mut self) -> Window {
    unsafe { accessor!(event -> Window, (*self.base.event)) }
  }

  pub fn window(&mut self) -> Window {
    unsafe { accessor!(window -> Window, (*self.base.event)) }
  }

  pub fn above_sibling(&mut self) -> Window {
    unsafe { accessor!(above_sibling -> Window, (*self.base.event)) }
  }

  pub fn x(&mut self) -> i16 {
    unsafe { accessor!(x -> i16, (*self.base.event)) }
  }

  pub fn y(&mut self) -> i16 {
    unsafe { accessor!(y -> i16, (*self.base.event)) }
  }

  pub fn width(&mut self) -> u16 {
    unsafe { accessor!(width -> u16, (*self.base.event)) }
  }

  pub fn height(&mut self) -> u16 {
    unsafe { accessor!(height -> u16, (*self.base.event)) }
  }

  pub fn border_width(&mut self) -> u16 {
    unsafe { accessor!(border_width -> u16, (*self.base.event)) }
  }

  pub fn override_redirect(&mut self) -> u8 {
    unsafe { accessor!(override_redirect -> u8, (*self.base.event)) }
  }

  pub fn new(event : Window,
         window : Window,
         above_sibling : Window,
         x : i16,
         y : i16,
         width : u16,
         height : u16,
         border_width : u16,
         override_redirect : u8) -> ConfigureNotifyEvent {
    unsafe {
      let raw = malloc(32 as size_t) as *mut xcb_configure_notify_event_t;
      (*raw).event = event;
      (*raw).window = window;
      (*raw).above_sibling = above_sibling;
      (*raw).x = x;
      (*raw).y = y;
      (*raw).width = width;
      (*raw).height = height;
      (*raw).border_width = border_width;
      (*raw).override_redirect = override_redirect;
      ConfigureNotifyEvent { base : Event { event : raw as *mut xcb_configure_notify_event_t }}
    }
  }
}

impl ConfigureRequestEvent {
  pub fn stack_mode(&mut self) -> u8 {
    unsafe { accessor!(stack_mode -> u8, (*self.base.event)) }
  }

  pub fn parent(&mut self) -> Window {
    unsafe { accessor!(parent -> Window, (*self.base.event)) }
  }

  pub fn window(&mut self) -> Window {
    unsafe { accessor!(window -> Window, (*self.base.event)) }
  }

  pub fn sibling(&mut self) -> Window {
    unsafe { accessor!(sibling -> Window, (*self.base.event)) }
  }

  pub fn x(&mut self) -> i16 {
    unsafe { accessor!(x -> i16, (*self.base.event)) }
  }

  pub fn y(&mut self) -> i16 {
    unsafe { accessor!(y -> i16, (*self.base.event)) }
  }

  pub fn width(&mut self) -> u16 {
    unsafe { accessor!(width -> u16, (*self.base.event)) }
  }

  pub fn height(&mut self) -> u16 {
    unsafe { accessor!(height -> u16, (*self.base.event)) }
  }

  pub fn border_width(&mut self) -> u16 {
    unsafe { accessor!(border_width -> u16, (*self.base.event)) }
  }

  pub fn value_mask(&mut self) -> u16 {
    unsafe { accessor!(value_mask -> u16, (*self.base.event)) }
  }

  pub fn new(stack_mode : u8,
         parent : Window,
         window : Window,
         sibling : Window,
         x : i16,
         y : i16,
         width : u16,
         height : u16,
         border_width : u16,
         value_mask : u16) -> ConfigureRequestEvent {
    unsafe {
      let raw = malloc(32 as size_t) as *mut xcb_configure_request_event_t;
      (*raw).stack_mode = stack_mode;
      (*raw).parent = parent;
      (*raw).window = window;
      (*raw).sibling = sibling;
      (*raw).x = x;
      (*raw).y = y;
      (*raw).width = width;
      (*raw).height = height;
      (*raw).border_width = border_width;
      (*raw).value_mask = value_mask;
      ConfigureRequestEvent { base : Event { event : raw as *mut xcb_configure_request_event_t }}
    }
  }
}

impl GravityNotifyEvent {
  pub fn event(&mut self) -> Window {
    unsafe { accessor!(event -> Window, (*self.base.event)) }
  }

  pub fn window(&mut self) -> Window {
    unsafe { accessor!(window -> Window, (*self.base.event)) }
  }

  pub fn x(&mut self) -> i16 {
    unsafe { accessor!(x -> i16, (*self.base.event)) }
  }

  pub fn y(&mut self) -> i16 {
    unsafe { accessor!(y -> i16, (*self.base.event)) }
  }

  pub fn new(event : Window,
         window : Window,
         x : i16,
         y : i16) -> GravityNotifyEvent {
    unsafe {
      let raw = malloc(32 as size_t) as *mut xcb_gravity_notify_event_t;
      (*raw).event = event;
      (*raw).window = window;
      (*raw).x = x;
      (*raw).y = y;
      GravityNotifyEvent { base : Event { event : raw as *mut xcb_gravity_notify_event_t }}
    }
  }
}

impl ResizeRequestEvent {
  pub fn window(&mut self) -> Window {
    unsafe { accessor!(window -> Window, (*self.base.event)) }
  }

  pub fn width(&mut self) -> u16 {
    unsafe { accessor!(width -> u16, (*self.base.event)) }
  }

  pub fn height(&mut self) -> u16 {
    unsafe { accessor!(height -> u16, (*self.base.event)) }
  }

  pub fn new(window : Window,
         width : u16,
         height : u16) -> ResizeRequestEvent {
    unsafe {
      let raw = malloc(32 as size_t) as *mut xcb_resize_request_event_t;
      (*raw).window = window;
      (*raw).width = width;
      (*raw).height = height;
      ResizeRequestEvent { base : Event { event : raw as *mut xcb_resize_request_event_t }}
    }
  }
}

impl CirculateNotifyEvent {
  pub fn event(&mut self) -> Window {
    unsafe { accessor!(event -> Window, (*self.base.event)) }
  }

  pub fn window(&mut self) -> Window {
    unsafe { accessor!(window -> Window, (*self.base.event)) }
  }

  pub fn place(&mut self) -> u8 {
    unsafe { accessor!(place -> u8, (*self.base.event)) }
  }

  pub fn new(event : Window,
         window : Window,
         place : u8) -> CirculateNotifyEvent {
    unsafe {
      let raw = malloc(32 as size_t) as *mut xcb_circulate_notify_event_t;
      (*raw).event = event;
      (*raw).window = window;
      (*raw).place = place;
      CirculateNotifyEvent { base : Event { event : raw as *mut xcb_circulate_notify_event_t }}
    }
  }
}

impl PropertyNotifyEvent {
  pub fn window(&mut self) -> Window {
    unsafe { accessor!(window -> Window, (*self.base.event)) }
  }

  pub fn atom(&mut self) -> Atom {
    unsafe { accessor!(atom -> Atom, (*self.base.event)) }
  }

  pub fn time(&mut self) -> Timestamp {
    unsafe { accessor!(time -> Timestamp, (*self.base.event)) }
  }

  pub fn state(&mut self) -> u8 {
    unsafe { accessor!(state -> u8, (*self.base.event)) }
  }

  pub fn new(window : Window,
         atom : Atom,
         time : Timestamp,
         state : u8) -> PropertyNotifyEvent {
    unsafe {
      let raw = malloc(32 as size_t) as *mut xcb_property_notify_event_t;
      (*raw).window = window;
      (*raw).atom = atom;
      (*raw).time = time;
      (*raw).state = state;
      PropertyNotifyEvent { base : Event { event : raw as *mut xcb_property_notify_event_t }}
    }
  }
}

impl SelectionClearEvent {
  pub fn time(&mut self) -> Timestamp {
    unsafe { accessor!(time -> Timestamp, (*self.base.event)) }
  }

  pub fn owner(&mut self) -> Window {
    unsafe { accessor!(owner -> Window, (*self.base.event)) }
  }

  pub fn selection(&mut self) -> Atom {
    unsafe { accessor!(selection -> Atom, (*self.base.event)) }
  }

  pub fn new(time : Timestamp,
         owner : Window,
         selection : Atom) -> SelectionClearEvent {
    unsafe {
      let raw = malloc(32 as size_t) as *mut xcb_selection_clear_event_t;
      (*raw).time = time;
      (*raw).owner = owner;
      (*raw).selection = selection;
      SelectionClearEvent { base : Event { event : raw as *mut xcb_selection_clear_event_t }}
    }
  }
}

impl SelectionRequestEvent {
  pub fn time(&mut self) -> Timestamp {
    unsafe { accessor!(time -> Timestamp, (*self.base.event)) }
  }

  pub fn owner(&mut self) -> Window {
    unsafe { accessor!(owner -> Window, (*self.base.event)) }
  }

  pub fn requestor(&mut self) -> Window {
    unsafe { accessor!(requestor -> Window, (*self.base.event)) }
  }

  pub fn selection(&mut self) -> Atom {
    unsafe { accessor!(selection -> Atom, (*self.base.event)) }
  }

  pub fn target(&mut self) -> Atom {
    unsafe { accessor!(target -> Atom, (*self.base.event)) }
  }

  pub fn property(&mut self) -> Atom {
    unsafe { accessor!(property -> Atom, (*self.base.event)) }
  }

  pub fn new(time : Timestamp,
         owner : Window,
         requestor : Window,
         selection : Atom,
         target : Atom,
         property : Atom) -> SelectionRequestEvent {
    unsafe {
      let raw = malloc(32 as size_t) as *mut xcb_selection_request_event_t;
      (*raw).time = time;
      (*raw).owner = owner;
      (*raw).requestor = requestor;
      (*raw).selection = selection;
      (*raw).target = target;
      (*raw).property = property;
      SelectionRequestEvent { base : Event { event : raw as *mut xcb_selection_request_event_t }}
    }
  }
}

impl SelectionNotifyEvent {
  pub fn time(&mut self) -> Timestamp {
    unsafe { accessor!(time -> Timestamp, (*self.base.event)) }
  }

  pub fn requestor(&mut self) -> Window {
    unsafe { accessor!(requestor -> Window, (*self.base.event)) }
  }

  pub fn selection(&mut self) -> Atom {
    unsafe { accessor!(selection -> Atom, (*self.base.event)) }
  }

  pub fn target(&mut self) -> Atom {
    unsafe { accessor!(target -> Atom, (*self.base.event)) }
  }

  pub fn property(&mut self) -> Atom {
    unsafe { accessor!(property -> Atom, (*self.base.event)) }
  }

  pub fn new(time : Timestamp,
         requestor : Window,
         selection : Atom,
         target : Atom,
         property : Atom) -> SelectionNotifyEvent {
    unsafe {
      let raw = malloc(32 as size_t) as *mut xcb_selection_notify_event_t;
      (*raw).time = time;
      (*raw).requestor = requestor;
      (*raw).selection = selection;
      (*raw).target = target;
      (*raw).property = property;
      SelectionNotifyEvent { base : Event { event : raw as *mut xcb_selection_notify_event_t }}
    }
  }
}

impl ColormapNotifyEvent {
  pub fn window(&mut self) -> Window {
    unsafe { accessor!(window -> Window, (*self.base.event)) }
  }

  pub fn colormap(&mut self) -> Colormap {
    unsafe { accessor!(colormap -> Colormap, (*self.base.event)) }
  }

  pub fn new_(&mut self) -> u8 {
    unsafe { accessor!(new_ -> u8, (*self.base.event)) }
  }

  pub fn state(&mut self) -> u8 {
    unsafe { accessor!(state -> u8, (*self.base.event)) }
  }

  pub fn new(window : Window,
         colormap : Colormap,
         new_ : u8,
         state : u8) -> ColormapNotifyEvent {
    unsafe {
      let raw = malloc(32 as size_t) as *mut xcb_colormap_notify_event_t;
      (*raw).window = window;
      (*raw).colormap = colormap;
      (*raw).new_ = new_;
      (*raw).state = state;
      ColormapNotifyEvent { base : Event { event : raw as *mut xcb_colormap_notify_event_t }}
    }
  }
}
pub struct ClientMessageData {pub base : base::Struct<xcb_client_message_data_t>}

impl Iterator for ClientMessageDataIterator {
    type Item = ClientMessageData;
    fn next(&mut self) -> Option<ClientMessageData> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_client_message_data_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_client_message_data_next(iter);
            Some(mem::transmute(*data))
        }
    }
}


impl ClientMessageEvent {
  pub fn format(&mut self) -> u8 {
    unsafe { accessor!(format -> u8, (*self.base.event)) }
  }

  pub fn window(&mut self) -> Window {
    unsafe { accessor!(window -> Window, (*self.base.event)) }
  }

  pub fn type_(&mut self) -> Atom {
    unsafe { accessor!(type_ -> Atom, (*self.base.event)) }
  }

  pub fn data(&self) -> ClientMessageData {
    unsafe { mem::transmute((*self.base.event).data) }
  }
  pub fn new(format : u8,
         window : Window,
         type_ : Atom,
         data : ClientMessageData) -> ClientMessageEvent {
    unsafe {
      let raw = malloc(32 as size_t) as *mut xcb_client_message_event_t;
      (*raw).format = format;
      (*raw).window = window;
      (*raw).type_ = type_;
      (*raw).data = data.base.strct;
      ClientMessageEvent { base : Event { event : raw as *mut xcb_client_message_event_t }}
    }
  }
}

impl MappingNotifyEvent {
  pub fn request(&mut self) -> u8 {
    unsafe { accessor!(request -> u8, (*self.base.event)) }
  }

  pub fn first_keycode(&mut self) -> Keycode {
    unsafe { accessor!(first_keycode -> Keycode, (*self.base.event)) }
  }

  pub fn count(&mut self) -> u8 {
    unsafe { accessor!(count -> u8, (*self.base.event)) }
  }

  pub fn new(request : u8,
         first_keycode : Keycode,
         count : u8) -> MappingNotifyEvent {
    unsafe {
      let raw = malloc(32 as size_t) as *mut xcb_mapping_notify_event_t;
      (*raw).request = request;
      (*raw).first_keycode = first_keycode;
      (*raw).count = count;
      MappingNotifyEvent { base : Event { event : raw as *mut xcb_mapping_notify_event_t }}
    }
  }
}
pub fn CreateWindowChecked<'r> (c : &'r Connection,
                            depth : u8,
                            wid : Window,
                            parent : Window,
                            x : i16,
                            y : i16,
                            width : u16,
                            height : u16,
                            border_width : u16,
                            class : u16,
                            visual : Visualid,
                            value_list : &[(u32,u32)]) -> base::VoidCookie<'r> {
  unsafe {
    let mut value_list_copy = value_list.to_vec();
    let (value_list_mask, value_list_vec) = pack_bitfield(&mut value_list_copy);
    let value_list_ptr = value_list_vec.as_ptr();
    let cookie = xcb_create_window_checked(c.get_raw_conn(),
        depth as u8, //1
        wid as xcb_window_t, //2
        parent as xcb_window_t, //3
        x as i16, //4
        y as i16, //5
        width as u16, //6
        height as u16, //7
        border_width as u16, //8
        class as u16, //9
        visual as xcb_visualid_t, //10
        value_list_mask as u32, //11
        value_list_ptr as *mut u32); //12
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn CreateWindow<'r> (c : &'r Connection,
                     depth : u8,
                     wid : Window,
                     parent : Window,
                     x : i16,
                     y : i16,
                     width : u16,
                     height : u16,
                     border_width : u16,
                     class : u16,
                     visual : Visualid,
                     value_list : &[(u32,u32)]) -> base::VoidCookie<'r> {
  unsafe {
    let mut value_list_copy = value_list.to_vec();
    let (value_list_mask, value_list_vec) = pack_bitfield(&mut value_list_copy);
    let value_list_ptr = value_list_vec.as_ptr();
    let cookie = xcb_create_window(c.get_raw_conn(),
        depth as u8, //1
        wid as xcb_window_t, //2
        parent as xcb_window_t, //3
        x as i16, //4
        y as i16, //5
        width as u16, //6
        height as u16, //7
        border_width as u16, //8
        class as u16, //9
        visual as xcb_visualid_t, //10
        value_list_mask as u32, //11
        value_list_ptr as *mut u32); //12
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn ChangeWindowAttributesChecked<'r> (c : &'r Connection,
                                      window : Window,
                                      value_list : &[(u32,u32)]) -> base::VoidCookie<'r> {
  unsafe {
    let mut value_list_copy = value_list.to_vec();
    let (value_list_mask, value_list_vec) = pack_bitfield(&mut value_list_copy);
    let value_list_ptr = value_list_vec.as_ptr();
    let cookie = xcb_change_window_attributes_checked(c.get_raw_conn(),
        window as xcb_window_t, //1
        value_list_mask as u32, //2
        value_list_ptr as *mut u32); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn ChangeWindowAttributes<'r> (c : &'r Connection,
                               window : Window,
                               value_list : &[(u32,u32)]) -> base::VoidCookie<'r> {
  unsafe {
    let mut value_list_copy = value_list.to_vec();
    let (value_list_mask, value_list_vec) = pack_bitfield(&mut value_list_copy);
    let value_list_ptr = value_list_vec.as_ptr();
    let cookie = xcb_change_window_attributes(c.get_raw_conn(),
        window as xcb_window_t, //1
        value_list_mask as u32, //2
        value_list_ptr as *mut u32); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetWindowAttributes<'r> (c : &'r Connection,
                            window : Window) -> GetWindowAttributesCookie<'r> {
  unsafe {
    let cookie = xcb_get_window_attributes(c.get_raw_conn(),
        window as xcb_window_t); //1
    GetWindowAttributesCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetWindowAttributesUnchecked<'r> (c : &'r Connection,
                                     window : Window) -> GetWindowAttributesCookie<'r> {
  unsafe {
    let cookie = xcb_get_window_attributes_unchecked(c.get_raw_conn(),
        window as xcb_window_t); //1
    GetWindowAttributesCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetWindowAttributesReply {
  pub fn backing_store(&mut self) -> u8 {
    unsafe { accessor!(backing_store -> u8, (*self.base.reply)) }
  }

  pub fn visual(&mut self) -> Visualid {
    unsafe { accessor!(visual -> Visualid, (*self.base.reply)) }
  }

  pub fn class(&mut self) -> u16 {
    unsafe { accessor!(class -> u16, (*self.base.reply)) }
  }

  pub fn bit_gravity(&mut self) -> u8 {
    unsafe { accessor!(bit_gravity -> u8, (*self.base.reply)) }
  }

  pub fn win_gravity(&mut self) -> u8 {
    unsafe { accessor!(win_gravity -> u8, (*self.base.reply)) }
  }

  pub fn backing_planes(&mut self) -> u32 {
    unsafe { accessor!(backing_planes -> u32, (*self.base.reply)) }
  }

  pub fn backing_pixel(&mut self) -> u32 {
    unsafe { accessor!(backing_pixel -> u32, (*self.base.reply)) }
  }

  pub fn save_under(&mut self) -> u8 {
    unsafe { accessor!(save_under -> u8, (*self.base.reply)) }
  }

  pub fn map_is_installed(&mut self) -> u8 {
    unsafe { accessor!(map_is_installed -> u8, (*self.base.reply)) }
  }

  pub fn map_state(&mut self) -> u8 {
    unsafe { accessor!(map_state -> u8, (*self.base.reply)) }
  }

  pub fn override_redirect(&mut self) -> u8 {
    unsafe { accessor!(override_redirect -> u8, (*self.base.reply)) }
  }

  pub fn colormap(&mut self) -> Colormap {
    unsafe { accessor!(colormap -> Colormap, (*self.base.reply)) }
  }

  pub fn all_event_masks(&mut self) -> u32 {
    unsafe { accessor!(all_event_masks -> u32, (*self.base.reply)) }
  }

  pub fn your_event_mask(&mut self) -> u32 {
    unsafe { accessor!(your_event_mask -> u32, (*self.base.reply)) }
  }

  pub fn do_not_propagate_mask(&mut self) -> u16 {
    unsafe { accessor!(do_not_propagate_mask -> u16, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetWindowAttributesCookie<'s>, mk_reply_xcb_get_window_attributes_reply_t, GetWindowAttributesReply, xcb_get_window_attributes_reply);

pub fn DestroyWindowChecked<'r> (c : &'r Connection,
                             window : Window) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_destroy_window_checked(c.get_raw_conn(),
        window as xcb_window_t); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn DestroyWindow<'r> (c : &'r Connection,
                      window : Window) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_destroy_window(c.get_raw_conn(),
        window as xcb_window_t); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn DestroySubwindowsChecked<'r> (c : &'r Connection,
                                 window : Window) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_destroy_subwindows_checked(c.get_raw_conn(),
        window as xcb_window_t); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn DestroySubwindows<'r> (c : &'r Connection,
                          window : Window) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_destroy_subwindows(c.get_raw_conn(),
        window as xcb_window_t); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn ChangeSaveSetChecked<'r> (c : &'r Connection,
                             mode : u8,
                             window : Window) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_change_save_set_checked(c.get_raw_conn(),
        mode as u8, //1
        window as xcb_window_t); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn ChangeSaveSet<'r> (c : &'r Connection,
                      mode : u8,
                      window : Window) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_change_save_set(c.get_raw_conn(),
        mode as u8, //1
        window as xcb_window_t); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn ReparentWindowChecked<'r> (c : &'r Connection,
                              window : Window,
                              parent : Window,
                              x : i16,
                              y : i16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_reparent_window_checked(c.get_raw_conn(),
        window as xcb_window_t, //1
        parent as xcb_window_t, //2
        x as i16, //3
        y as i16); //4
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn ReparentWindow<'r> (c : &'r Connection,
                       window : Window,
                       parent : Window,
                       x : i16,
                       y : i16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_reparent_window(c.get_raw_conn(),
        window as xcb_window_t, //1
        parent as xcb_window_t, //2
        x as i16, //3
        y as i16); //4
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn MapWindowChecked<'r> (c : &'r Connection,
                         window : Window) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_map_window_checked(c.get_raw_conn(),
        window as xcb_window_t); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn MapWindow<'r> (c : &'r Connection,
                  window : Window) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_map_window(c.get_raw_conn(),
        window as xcb_window_t); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn MapSubwindowsChecked<'r> (c : &'r Connection,
                             window : Window) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_map_subwindows_checked(c.get_raw_conn(),
        window as xcb_window_t); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn MapSubwindows<'r> (c : &'r Connection,
                      window : Window) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_map_subwindows(c.get_raw_conn(),
        window as xcb_window_t); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn UnmapWindowChecked<'r> (c : &'r Connection,
                           window : Window) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_unmap_window_checked(c.get_raw_conn(),
        window as xcb_window_t); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn UnmapWindow<'r> (c : &'r Connection,
                    window : Window) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_unmap_window(c.get_raw_conn(),
        window as xcb_window_t); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn UnmapSubwindowsChecked<'r> (c : &'r Connection,
                               window : Window) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_unmap_subwindows_checked(c.get_raw_conn(),
        window as xcb_window_t); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn UnmapSubwindows<'r> (c : &'r Connection,
                        window : Window) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_unmap_subwindows(c.get_raw_conn(),
        window as xcb_window_t); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn ConfigureWindowChecked<'r> (c : &'r Connection,
                               window : Window,
                               value_list : &[(u16,u32)]) -> base::VoidCookie<'r> {
  unsafe {
    let mut value_list_copy = value_list.to_vec();
    let (value_list_mask, value_list_vec) = pack_bitfield(&mut value_list_copy);
    let value_list_ptr = value_list_vec.as_ptr();
    let cookie = xcb_configure_window_checked(c.get_raw_conn(),
        window as xcb_window_t, //1
        value_list_mask as u16, //2
        value_list_ptr as *mut u32); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn ConfigureWindow<'r> (c : &'r Connection,
                        window : Window,
                        value_list : &[(u16,u32)]) -> base::VoidCookie<'r> {
  unsafe {
    let mut value_list_copy = value_list.to_vec();
    let (value_list_mask, value_list_vec) = pack_bitfield(&mut value_list_copy);
    let value_list_ptr = value_list_vec.as_ptr();
    let cookie = xcb_configure_window(c.get_raw_conn(),
        window as xcb_window_t, //1
        value_list_mask as u16, //2
        value_list_ptr as *mut u32); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn CirculateWindowChecked<'r> (c : &'r Connection,
                               direction : u8,
                               window : Window) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_circulate_window_checked(c.get_raw_conn(),
        direction as u8, //1
        window as xcb_window_t); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn CirculateWindow<'r> (c : &'r Connection,
                        direction : u8,
                        window : Window) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_circulate_window(c.get_raw_conn(),
        direction as u8, //1
        window as xcb_window_t); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetGeometry<'r> (c : &'r Connection,
                    drawable : Drawable) -> GetGeometryCookie<'r> {
  unsafe {
    let cookie = xcb_get_geometry(c.get_raw_conn(),
        drawable as xcb_drawable_t); //1
    GetGeometryCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetGeometryUnchecked<'r> (c : &'r Connection,
                             drawable : Drawable) -> GetGeometryCookie<'r> {
  unsafe {
    let cookie = xcb_get_geometry_unchecked(c.get_raw_conn(),
        drawable as xcb_drawable_t); //1
    GetGeometryCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetGeometryReply {
  pub fn depth(&mut self) -> u8 {
    unsafe { accessor!(depth -> u8, (*self.base.reply)) }
  }

  pub fn root(&mut self) -> Window {
    unsafe { accessor!(root -> Window, (*self.base.reply)) }
  }

  pub fn x(&mut self) -> i16 {
    unsafe { accessor!(x -> i16, (*self.base.reply)) }
  }

  pub fn y(&mut self) -> i16 {
    unsafe { accessor!(y -> i16, (*self.base.reply)) }
  }

  pub fn width(&mut self) -> u16 {
    unsafe { accessor!(width -> u16, (*self.base.reply)) }
  }

  pub fn height(&mut self) -> u16 {
    unsafe { accessor!(height -> u16, (*self.base.reply)) }
  }

  pub fn border_width(&mut self) -> u16 {
    unsafe { accessor!(border_width -> u16, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetGeometryCookie<'s>, mk_reply_xcb_get_geometry_reply_t, GetGeometryReply, xcb_get_geometry_reply);

pub struct QueryTreeReply { base:  base::Reply<xcb_query_tree_reply_t> }
fn mk_reply_xcb_query_tree_reply_t(reply:*mut xcb_query_tree_reply_t) -> QueryTreeReply { QueryTreeReply { base : base::mk_reply(reply) } }
pub fn QueryTree<'r> (c : &'r Connection,
                  window : Window) -> QueryTreeCookie<'r> {
  unsafe {
    let cookie = xcb_query_tree(c.get_raw_conn(),
        window as xcb_window_t); //1
    QueryTreeCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn QueryTreeUnchecked<'r> (c : &'r Connection,
                           window : Window) -> QueryTreeCookie<'r> {
  unsafe {
    let cookie = xcb_query_tree_unchecked(c.get_raw_conn(),
        window as xcb_window_t); //1
    QueryTreeCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl QueryTreeReply {
  pub fn root(&mut self) -> Window {
    unsafe { accessor!(root -> Window, (*self.base.reply)) }
  }

  pub fn parent(&mut self) -> Window {
    unsafe { accessor!(parent -> Window, (*self.base.reply)) }
  }

  pub fn children(&mut self) -> Vec<Window> {
    unsafe { accessor!(Window, xcb_query_tree_children_length, xcb_query_tree_children, (*self.base.reply)) }
  }

}
impl_reply_cookie!(QueryTreeCookie<'s>, mk_reply_xcb_query_tree_reply_t, QueryTreeReply, xcb_query_tree_reply);

pub fn InternAtom<'r> (c : &'r Connection,
                   only_if_exists : u8,
                   name : &str) -> InternAtomCookie<'r> {
  unsafe {
    let name = (name).as_bytes();
    let name_len = name.len();
    let name_ptr = name.as_ptr();
    let cookie = xcb_intern_atom(c.get_raw_conn(),
        only_if_exists as u8, //1
        name_len as u16, //2
        name_ptr as *mut c_char); //3
    InternAtomCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn InternAtomUnchecked<'r> (c : &'r Connection,
                            only_if_exists : u8,
                            name : &str) -> InternAtomCookie<'r> {
  unsafe {
    let name = (name).as_bytes();
    let name_len = name.len();
    let name_ptr = name.as_ptr();
    let cookie = xcb_intern_atom_unchecked(c.get_raw_conn(),
        only_if_exists as u8, //1
        name_len as u16, //2
        name_ptr as *mut c_char); //3
    InternAtomCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl InternAtomReply {
  pub fn atom(&mut self) -> Atom {
    unsafe { accessor!(atom -> Atom, (*self.base.reply)) }
  }

}
impl_reply_cookie!(InternAtomCookie<'s>, mk_reply_xcb_intern_atom_reply_t, InternAtomReply, xcb_intern_atom_reply);

pub struct GetAtomNameReply { base:  base::Reply<xcb_get_atom_name_reply_t> }
fn mk_reply_xcb_get_atom_name_reply_t(reply:*mut xcb_get_atom_name_reply_t) -> GetAtomNameReply { GetAtomNameReply { base : base::mk_reply(reply) } }
pub fn GetAtomName<'r> (c : &'r Connection,
                    atom : Atom) -> GetAtomNameCookie<'r> {
  unsafe {
    let cookie = xcb_get_atom_name(c.get_raw_conn(),
        atom as xcb_atom_t); //1
    GetAtomNameCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetAtomNameUnchecked<'r> (c : &'r Connection,
                             atom : Atom) -> GetAtomNameCookie<'r> {
  unsafe {
    let cookie = xcb_get_atom_name_unchecked(c.get_raw_conn(),
        atom as xcb_atom_t); //1
    GetAtomNameCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetAtomNameReply {
  pub fn name(&mut self) -> String {
    unsafe { accessor!(str, xcb_get_atom_name_name_length, xcb_get_atom_name_name, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetAtomNameCookie<'s>, mk_reply_xcb_get_atom_name_reply_t, GetAtomNameReply, xcb_get_atom_name_reply);

pub fn ChangePropertyChecked<'r> (c : &'r Connection,
                              mode : u8,
                              window : Window,
                              property : Atom,
                              type_ : Atom,
                              format : u8,
                              data : &[u8]) -> base::VoidCookie<'r> {
  unsafe {
    let data_len = data.len();
    let data_ptr = data.as_ptr();
    let cookie = xcb_change_property_checked(c.get_raw_conn(),
        mode as u8, //1
        window as xcb_window_t, //2
        property as xcb_atom_t, //3
        type_ as xcb_atom_t, //4
        format as u8, //5
        data_len as u32, //6
        data_ptr as *mut c_void); //7
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn ChangeProperty<'r> (c : &'r Connection,
                       mode : u8,
                       window : Window,
                       property : Atom,
                       type_ : Atom,
                       format : u8,
                       data : &[u8]) -> base::VoidCookie<'r> {
  unsafe {
    let data_len = data.len();
    let data_ptr = data.as_ptr();
    let cookie = xcb_change_property(c.get_raw_conn(),
        mode as u8, //1
        window as xcb_window_t, //2
        property as xcb_atom_t, //3
        type_ as xcb_atom_t, //4
        format as u8, //5
        data_len as u32, //6
        data_ptr as *mut c_void); //7
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn DeletePropertyChecked<'r> (c : &'r Connection,
                              window : Window,
                              property : Atom) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_delete_property_checked(c.get_raw_conn(),
        window as xcb_window_t, //1
        property as xcb_atom_t); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn DeleteProperty<'r> (c : &'r Connection,
                       window : Window,
                       property : Atom) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_delete_property(c.get_raw_conn(),
        window as xcb_window_t, //1
        property as xcb_atom_t); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub struct GetPropertyReply { base:  base::Reply<xcb_get_property_reply_t> }
fn mk_reply_xcb_get_property_reply_t(reply:*mut xcb_get_property_reply_t) -> GetPropertyReply { GetPropertyReply { base : base::mk_reply(reply) } }
pub fn GetProperty<'r> (c : &'r Connection,
                    delete : u8,
                    window : Window,
                    property : Atom,
                    type_ : Atom,
                    long_offset : u32,
                    long_length : u32) -> GetPropertyCookie<'r> {
  unsafe {
    let cookie = xcb_get_property(c.get_raw_conn(),
        delete as u8, //1
        window as xcb_window_t, //2
        property as xcb_atom_t, //3
        type_ as xcb_atom_t, //4
        long_offset as u32, //5
        long_length as u32); //6
    GetPropertyCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetPropertyUnchecked<'r> (c : &'r Connection,
                             delete : u8,
                             window : Window,
                             property : Atom,
                             type_ : Atom,
                             long_offset : u32,
                             long_length : u32) -> GetPropertyCookie<'r> {
  unsafe {
    let cookie = xcb_get_property_unchecked(c.get_raw_conn(),
        delete as u8, //1
        window as xcb_window_t, //2
        property as xcb_atom_t, //3
        type_ as xcb_atom_t, //4
        long_offset as u32, //5
        long_length as u32); //6
    GetPropertyCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetPropertyReply {
  pub fn format(&mut self) -> u8 {
    unsafe { accessor!(format -> u8, (*self.base.reply)) }
  }

  pub fn type_(&mut self) -> Atom {
    unsafe { accessor!(type_ -> Atom, (*self.base.reply)) }
  }

  pub fn bytes_after(&mut self) -> u32 {
    unsafe { accessor!(bytes_after -> u32, (*self.base.reply)) }
  }

  pub fn value(&mut self) -> Vec<c_void> {
    unsafe { accessor!(c_void, xcb_get_property_value_length, xcb_get_property_value, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetPropertyCookie<'s>, mk_reply_xcb_get_property_reply_t, GetPropertyReply, xcb_get_property_reply);

pub struct ListPropertiesReply { base:  base::Reply<xcb_list_properties_reply_t> }
fn mk_reply_xcb_list_properties_reply_t(reply:*mut xcb_list_properties_reply_t) -> ListPropertiesReply { ListPropertiesReply { base : base::mk_reply(reply) } }
pub fn ListProperties<'r> (c : &'r Connection,
                       window : Window) -> ListPropertiesCookie<'r> {
  unsafe {
    let cookie = xcb_list_properties(c.get_raw_conn(),
        window as xcb_window_t); //1
    ListPropertiesCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn ListPropertiesUnchecked<'r> (c : &'r Connection,
                                window : Window) -> ListPropertiesCookie<'r> {
  unsafe {
    let cookie = xcb_list_properties_unchecked(c.get_raw_conn(),
        window as xcb_window_t); //1
    ListPropertiesCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl ListPropertiesReply {
  pub fn atoms(&mut self) -> Vec<Atom> {
    unsafe { accessor!(Atom, xcb_list_properties_atoms_length, xcb_list_properties_atoms, (*self.base.reply)) }
  }

}
impl_reply_cookie!(ListPropertiesCookie<'s>, mk_reply_xcb_list_properties_reply_t, ListPropertiesReply, xcb_list_properties_reply);

pub fn SetSelectionOwnerChecked<'r> (c : &'r Connection,
                                 owner : Window,
                                 selection : Atom,
                                 time : Timestamp) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_set_selection_owner_checked(c.get_raw_conn(),
        owner as xcb_window_t, //1
        selection as xcb_atom_t, //2
        time as xcb_timestamp_t); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn SetSelectionOwner<'r> (c : &'r Connection,
                          owner : Window,
                          selection : Atom,
                          time : Timestamp) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_set_selection_owner(c.get_raw_conn(),
        owner as xcb_window_t, //1
        selection as xcb_atom_t, //2
        time as xcb_timestamp_t); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetSelectionOwner<'r> (c : &'r Connection,
                          selection : Atom) -> GetSelectionOwnerCookie<'r> {
  unsafe {
    let cookie = xcb_get_selection_owner(c.get_raw_conn(),
        selection as xcb_atom_t); //1
    GetSelectionOwnerCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetSelectionOwnerUnchecked<'r> (c : &'r Connection,
                                   selection : Atom) -> GetSelectionOwnerCookie<'r> {
  unsafe {
    let cookie = xcb_get_selection_owner_unchecked(c.get_raw_conn(),
        selection as xcb_atom_t); //1
    GetSelectionOwnerCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetSelectionOwnerReply {
  pub fn owner(&mut self) -> Window {
    unsafe { accessor!(owner -> Window, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetSelectionOwnerCookie<'s>, mk_reply_xcb_get_selection_owner_reply_t, GetSelectionOwnerReply, xcb_get_selection_owner_reply);

pub fn ConvertSelectionChecked<'r> (c : &'r Connection,
                                requestor : Window,
                                selection : Atom,
                                target : Atom,
                                property : Atom,
                                time : Timestamp) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_convert_selection_checked(c.get_raw_conn(),
        requestor as xcb_window_t, //1
        selection as xcb_atom_t, //2
        target as xcb_atom_t, //3
        property as xcb_atom_t, //4
        time as xcb_timestamp_t); //5
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn ConvertSelection<'r> (c : &'r Connection,
                         requestor : Window,
                         selection : Atom,
                         target : Atom,
                         property : Atom,
                         time : Timestamp) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_convert_selection(c.get_raw_conn(),
        requestor as xcb_window_t, //1
        selection as xcb_atom_t, //2
        target as xcb_atom_t, //3
        property as xcb_atom_t, //4
        time as xcb_timestamp_t); //5
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn SendEventChecked<'r> (c : &'r Connection,
                         propagate : u8,
                         destination : Window,
                         event_mask : u32,
                         event : &str) -> base::VoidCookie<'r> {
  unsafe {
    let event = (event).as_bytes();
    let event_ptr = event.as_ptr();
    let cookie = xcb_send_event_checked(c.get_raw_conn(),
        propagate as u8, //1
        destination as xcb_window_t, //2
        event_mask as u32, //3
        event_ptr as *mut c_char); //4
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn SendEvent<'r> (c : &'r Connection,
                  propagate : u8,
                  destination : Window,
                  event_mask : u32,
                  event : &str) -> base::VoidCookie<'r> {
  unsafe {
    let event = (event).as_bytes();
    let event_ptr = event.as_ptr();
    let cookie = xcb_send_event(c.get_raw_conn(),
        propagate as u8, //1
        destination as xcb_window_t, //2
        event_mask as u32, //3
        event_ptr as *mut c_char); //4
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GrabPointer<'r> (c : &'r Connection,
                    owner_events : u8,
                    grab_window : Window,
                    event_mask : u16,
                    pointer_mode : u8,
                    keyboard_mode : u8,
                    confine_to : Window,
                    cursor : Cursor,
                    time : Timestamp) -> GrabPointerCookie<'r> {
  unsafe {
    let cookie = xcb_grab_pointer(c.get_raw_conn(),
        owner_events as u8, //1
        grab_window as xcb_window_t, //2
        event_mask as u16, //3
        pointer_mode as u8, //4
        keyboard_mode as u8, //5
        confine_to as xcb_window_t, //6
        cursor as xcb_cursor_t, //7
        time as xcb_timestamp_t); //8
    GrabPointerCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GrabPointerUnchecked<'r> (c : &'r Connection,
                             owner_events : u8,
                             grab_window : Window,
                             event_mask : u16,
                             pointer_mode : u8,
                             keyboard_mode : u8,
                             confine_to : Window,
                             cursor : Cursor,
                             time : Timestamp) -> GrabPointerCookie<'r> {
  unsafe {
    let cookie = xcb_grab_pointer_unchecked(c.get_raw_conn(),
        owner_events as u8, //1
        grab_window as xcb_window_t, //2
        event_mask as u16, //3
        pointer_mode as u8, //4
        keyboard_mode as u8, //5
        confine_to as xcb_window_t, //6
        cursor as xcb_cursor_t, //7
        time as xcb_timestamp_t); //8
    GrabPointerCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GrabPointerReply {
  pub fn status(&mut self) -> u8 {
    unsafe { accessor!(status -> u8, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GrabPointerCookie<'s>, mk_reply_xcb_grab_pointer_reply_t, GrabPointerReply, xcb_grab_pointer_reply);

pub fn UngrabPointerChecked<'r> (c : &'r Connection,
                             time : Timestamp) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_ungrab_pointer_checked(c.get_raw_conn(),
        time as xcb_timestamp_t); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn UngrabPointer<'r> (c : &'r Connection,
                      time : Timestamp) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_ungrab_pointer(c.get_raw_conn(),
        time as xcb_timestamp_t); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GrabButtonChecked<'r> (c : &'r Connection,
                          owner_events : u8,
                          grab_window : Window,
                          event_mask : u16,
                          pointer_mode : u8,
                          keyboard_mode : u8,
                          confine_to : Window,
                          cursor : Cursor,
                          button : u8,
                          modifiers : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_grab_button_checked(c.get_raw_conn(),
        owner_events as u8, //1
        grab_window as xcb_window_t, //2
        event_mask as u16, //3
        pointer_mode as u8, //4
        keyboard_mode as u8, //5
        confine_to as xcb_window_t, //6
        cursor as xcb_cursor_t, //7
        button as u8, //8
        modifiers as u16); //9
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn GrabButton<'r> (c : &'r Connection,
                   owner_events : u8,
                   grab_window : Window,
                   event_mask : u16,
                   pointer_mode : u8,
                   keyboard_mode : u8,
                   confine_to : Window,
                   cursor : Cursor,
                   button : u8,
                   modifiers : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_grab_button(c.get_raw_conn(),
        owner_events as u8, //1
        grab_window as xcb_window_t, //2
        event_mask as u16, //3
        pointer_mode as u8, //4
        keyboard_mode as u8, //5
        confine_to as xcb_window_t, //6
        cursor as xcb_cursor_t, //7
        button as u8, //8
        modifiers as u16); //9
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn UngrabButtonChecked<'r> (c : &'r Connection,
                            button : u8,
                            grab_window : Window,
                            modifiers : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_ungrab_button_checked(c.get_raw_conn(),
        button as u8, //1
        grab_window as xcb_window_t, //2
        modifiers as u16); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn UngrabButton<'r> (c : &'r Connection,
                     button : u8,
                     grab_window : Window,
                     modifiers : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_ungrab_button(c.get_raw_conn(),
        button as u8, //1
        grab_window as xcb_window_t, //2
        modifiers as u16); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn ChangeActivePointerGrabChecked<'r> (c : &'r Connection,
                                       cursor : Cursor,
                                       time : Timestamp,
                                       event_mask : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_change_active_pointer_grab_checked(c.get_raw_conn(),
        cursor as xcb_cursor_t, //1
        time as xcb_timestamp_t, //2
        event_mask as u16); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn ChangeActivePointerGrab<'r> (c : &'r Connection,
                                cursor : Cursor,
                                time : Timestamp,
                                event_mask : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_change_active_pointer_grab(c.get_raw_conn(),
        cursor as xcb_cursor_t, //1
        time as xcb_timestamp_t, //2
        event_mask as u16); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GrabKeyboard<'r> (c : &'r Connection,
                     owner_events : u8,
                     grab_window : Window,
                     time : Timestamp,
                     pointer_mode : u8,
                     keyboard_mode : u8) -> GrabKeyboardCookie<'r> {
  unsafe {
    let cookie = xcb_grab_keyboard(c.get_raw_conn(),
        owner_events as u8, //1
        grab_window as xcb_window_t, //2
        time as xcb_timestamp_t, //3
        pointer_mode as u8, //4
        keyboard_mode as u8); //5
    GrabKeyboardCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GrabKeyboardUnchecked<'r> (c : &'r Connection,
                              owner_events : u8,
                              grab_window : Window,
                              time : Timestamp,
                              pointer_mode : u8,
                              keyboard_mode : u8) -> GrabKeyboardCookie<'r> {
  unsafe {
    let cookie = xcb_grab_keyboard_unchecked(c.get_raw_conn(),
        owner_events as u8, //1
        grab_window as xcb_window_t, //2
        time as xcb_timestamp_t, //3
        pointer_mode as u8, //4
        keyboard_mode as u8); //5
    GrabKeyboardCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GrabKeyboardReply {
  pub fn status(&mut self) -> u8 {
    unsafe { accessor!(status -> u8, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GrabKeyboardCookie<'s>, mk_reply_xcb_grab_keyboard_reply_t, GrabKeyboardReply, xcb_grab_keyboard_reply);

pub fn UngrabKeyboardChecked<'r> (c : &'r Connection,
                              time : Timestamp) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_ungrab_keyboard_checked(c.get_raw_conn(),
        time as xcb_timestamp_t); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn UngrabKeyboard<'r> (c : &'r Connection,
                       time : Timestamp) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_ungrab_keyboard(c.get_raw_conn(),
        time as xcb_timestamp_t); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GrabKeyChecked<'r> (c : &'r Connection,
                       owner_events : u8,
                       grab_window : Window,
                       modifiers : u16,
                       key : Keycode,
                       pointer_mode : u8,
                       keyboard_mode : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_grab_key_checked(c.get_raw_conn(),
        owner_events as u8, //1
        grab_window as xcb_window_t, //2
        modifiers as u16, //3
        key as xcb_keycode_t, //4
        pointer_mode as u8, //5
        keyboard_mode as u8); //6
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn GrabKey<'r> (c : &'r Connection,
                owner_events : u8,
                grab_window : Window,
                modifiers : u16,
                key : Keycode,
                pointer_mode : u8,
                keyboard_mode : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_grab_key(c.get_raw_conn(),
        owner_events as u8, //1
        grab_window as xcb_window_t, //2
        modifiers as u16, //3
        key as xcb_keycode_t, //4
        pointer_mode as u8, //5
        keyboard_mode as u8); //6
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn UngrabKeyChecked<'r> (c : &'r Connection,
                         key : Keycode,
                         grab_window : Window,
                         modifiers : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_ungrab_key_checked(c.get_raw_conn(),
        key as xcb_keycode_t, //1
        grab_window as xcb_window_t, //2
        modifiers as u16); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn UngrabKey<'r> (c : &'r Connection,
                  key : Keycode,
                  grab_window : Window,
                  modifiers : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_ungrab_key(c.get_raw_conn(),
        key as xcb_keycode_t, //1
        grab_window as xcb_window_t, //2
        modifiers as u16); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn AllowEventsChecked<'r> (c : &'r Connection,
                           mode : u8,
                           time : Timestamp) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_allow_events_checked(c.get_raw_conn(),
        mode as u8, //1
        time as xcb_timestamp_t); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn AllowEvents<'r> (c : &'r Connection,
                    mode : u8,
                    time : Timestamp) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_allow_events(c.get_raw_conn(),
        mode as u8, //1
        time as xcb_timestamp_t); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GrabServerChecked<'r> (c : &'r Connection) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_grab_server_checked(c.get_raw_conn());
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn GrabServer<'r> (c : &'r Connection) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_grab_server(c.get_raw_conn());
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn UngrabServerChecked<'r> (c : &'r Connection) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_ungrab_server_checked(c.get_raw_conn());
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn UngrabServer<'r> (c : &'r Connection) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_ungrab_server(c.get_raw_conn());
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn QueryPointer<'r> (c : &'r Connection,
                     window : Window) -> QueryPointerCookie<'r> {
  unsafe {
    let cookie = xcb_query_pointer(c.get_raw_conn(),
        window as xcb_window_t); //1
    QueryPointerCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn QueryPointerUnchecked<'r> (c : &'r Connection,
                              window : Window) -> QueryPointerCookie<'r> {
  unsafe {
    let cookie = xcb_query_pointer_unchecked(c.get_raw_conn(),
        window as xcb_window_t); //1
    QueryPointerCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl QueryPointerReply {
  pub fn same_screen(&mut self) -> u8 {
    unsafe { accessor!(same_screen -> u8, (*self.base.reply)) }
  }

  pub fn root(&mut self) -> Window {
    unsafe { accessor!(root -> Window, (*self.base.reply)) }
  }

  pub fn child(&mut self) -> Window {
    unsafe { accessor!(child -> Window, (*self.base.reply)) }
  }

  pub fn root_x(&mut self) -> i16 {
    unsafe { accessor!(root_x -> i16, (*self.base.reply)) }
  }

  pub fn root_y(&mut self) -> i16 {
    unsafe { accessor!(root_y -> i16, (*self.base.reply)) }
  }

  pub fn win_x(&mut self) -> i16 {
    unsafe { accessor!(win_x -> i16, (*self.base.reply)) }
  }

  pub fn win_y(&mut self) -> i16 {
    unsafe { accessor!(win_y -> i16, (*self.base.reply)) }
  }

  pub fn mask(&mut self) -> u16 {
    unsafe { accessor!(mask -> u16, (*self.base.reply)) }
  }

}
impl_reply_cookie!(QueryPointerCookie<'s>, mk_reply_xcb_query_pointer_reply_t, QueryPointerReply, xcb_query_pointer_reply);

pub struct Timecoord {pub base : base::Struct<xcb_timecoord_t> }


impl Timecoord {
  pub fn time(&mut self) -> Timestamp {
    unsafe { accessor!(time -> Timestamp, self.base.strct) }
  }

  pub fn x(&mut self) -> i16 {
    unsafe { accessor!(x -> i16, self.base.strct) }
  }

  pub fn y(&mut self) -> i16 {
    unsafe { accessor!(y -> i16, self.base.strct) }
  }

}

impl Iterator for TimecoordIterator {
    type Item = Timecoord;
    fn next(&mut self) -> Option<Timecoord> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_timecoord_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_timecoord_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct GetMotionEventsReply { base:  base::Reply<xcb_get_motion_events_reply_t> }
fn mk_reply_xcb_get_motion_events_reply_t(reply:*mut xcb_get_motion_events_reply_t) -> GetMotionEventsReply { GetMotionEventsReply { base : base::mk_reply(reply) } }
pub fn GetMotionEvents<'r> (c : &'r Connection,
                        window : Window,
                        start : Timestamp,
                        stop : Timestamp) -> GetMotionEventsCookie<'r> {
  unsafe {
    let cookie = xcb_get_motion_events(c.get_raw_conn(),
        window as xcb_window_t, //1
        start as xcb_timestamp_t, //2
        stop as xcb_timestamp_t); //3
    GetMotionEventsCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetMotionEventsUnchecked<'r> (c : &'r Connection,
                                 window : Window,
                                 start : Timestamp,
                                 stop : Timestamp) -> GetMotionEventsCookie<'r> {
  unsafe {
    let cookie = xcb_get_motion_events_unchecked(c.get_raw_conn(),
        window as xcb_window_t, //1
        start as xcb_timestamp_t, //2
        stop as xcb_timestamp_t); //3
    GetMotionEventsCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetMotionEventsReply {
  pub fn events(&mut self) -> TimecoordIterator {
    unsafe { accessor!(TimecoordIterator, xcb_get_motion_events_events_iterator, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetMotionEventsCookie<'s>, mk_reply_xcb_get_motion_events_reply_t, GetMotionEventsReply, xcb_get_motion_events_reply);

pub fn TranslateCoordinates<'r> (c : &'r Connection,
                             src_window : Window,
                             dst_window : Window,
                             src_x : i16,
                             src_y : i16) -> TranslateCoordinatesCookie<'r> {
  unsafe {
    let cookie = xcb_translate_coordinates(c.get_raw_conn(),
        src_window as xcb_window_t, //1
        dst_window as xcb_window_t, //2
        src_x as i16, //3
        src_y as i16); //4
    TranslateCoordinatesCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn TranslateCoordinatesUnchecked<'r> (c : &'r Connection,
                                      src_window : Window,
                                      dst_window : Window,
                                      src_x : i16,
                                      src_y : i16) -> TranslateCoordinatesCookie<'r> {
  unsafe {
    let cookie = xcb_translate_coordinates_unchecked(c.get_raw_conn(),
        src_window as xcb_window_t, //1
        dst_window as xcb_window_t, //2
        src_x as i16, //3
        src_y as i16); //4
    TranslateCoordinatesCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl TranslateCoordinatesReply {
  pub fn same_screen(&mut self) -> u8 {
    unsafe { accessor!(same_screen -> u8, (*self.base.reply)) }
  }

  pub fn child(&mut self) -> Window {
    unsafe { accessor!(child -> Window, (*self.base.reply)) }
  }

  pub fn dst_x(&mut self) -> i16 {
    unsafe { accessor!(dst_x -> i16, (*self.base.reply)) }
  }

  pub fn dst_y(&mut self) -> i16 {
    unsafe { accessor!(dst_y -> i16, (*self.base.reply)) }
  }

}
impl_reply_cookie!(TranslateCoordinatesCookie<'s>, mk_reply_xcb_translate_coordinates_reply_t, TranslateCoordinatesReply, xcb_translate_coordinates_reply);

pub fn WarpPointerChecked<'r> (c : &'r Connection,
                           src_window : Window,
                           dst_window : Window,
                           src_x : i16,
                           src_y : i16,
                           src_width : u16,
                           src_height : u16,
                           dst_x : i16,
                           dst_y : i16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_warp_pointer_checked(c.get_raw_conn(),
        src_window as xcb_window_t, //1
        dst_window as xcb_window_t, //2
        src_x as i16, //3
        src_y as i16, //4
        src_width as u16, //5
        src_height as u16, //6
        dst_x as i16, //7
        dst_y as i16); //8
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn WarpPointer<'r> (c : &'r Connection,
                    src_window : Window,
                    dst_window : Window,
                    src_x : i16,
                    src_y : i16,
                    src_width : u16,
                    src_height : u16,
                    dst_x : i16,
                    dst_y : i16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_warp_pointer(c.get_raw_conn(),
        src_window as xcb_window_t, //1
        dst_window as xcb_window_t, //2
        src_x as i16, //3
        src_y as i16, //4
        src_width as u16, //5
        src_height as u16, //6
        dst_x as i16, //7
        dst_y as i16); //8
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn SetInputFocusChecked<'r> (c : &'r Connection,
                             revert_to : u8,
                             focus : Window,
                             time : Timestamp) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_set_input_focus_checked(c.get_raw_conn(),
        revert_to as u8, //1
        focus as xcb_window_t, //2
        time as xcb_timestamp_t); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn SetInputFocus<'r> (c : &'r Connection,
                      revert_to : u8,
                      focus : Window,
                      time : Timestamp) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_set_input_focus(c.get_raw_conn(),
        revert_to as u8, //1
        focus as xcb_window_t, //2
        time as xcb_timestamp_t); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetInputFocus<'r> (c : &'r Connection) -> GetInputFocusCookie<'r> {
  unsafe {
    let cookie = xcb_get_input_focus(c.get_raw_conn());
    GetInputFocusCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetInputFocusUnchecked<'r> (c : &'r Connection) -> GetInputFocusCookie<'r> {
  unsafe {
    let cookie = xcb_get_input_focus_unchecked(c.get_raw_conn());
    GetInputFocusCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetInputFocusReply {
  pub fn revert_to(&mut self) -> u8 {
    unsafe { accessor!(revert_to -> u8, (*self.base.reply)) }
  }

  pub fn focus(&mut self) -> Window {
    unsafe { accessor!(focus -> Window, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetInputFocusCookie<'s>, mk_reply_xcb_get_input_focus_reply_t, GetInputFocusReply, xcb_get_input_focus_reply);

pub fn QueryKeymap<'r> (c : &'r Connection) -> QueryKeymapCookie<'r> {
  unsafe {
    let cookie = xcb_query_keymap(c.get_raw_conn());
    QueryKeymapCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn QueryKeymapUnchecked<'r> (c : &'r Connection) -> QueryKeymapCookie<'r> {
  unsafe {
    let cookie = xcb_query_keymap_unchecked(c.get_raw_conn());
    QueryKeymapCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl QueryKeymapReply {
  pub fn keys(&self) -> Vec<u8> {
    unsafe { ((*self.base.reply).keys).to_vec() }
  }

}
impl_reply_cookie!(QueryKeymapCookie<'s>, mk_reply_xcb_query_keymap_reply_t, QueryKeymapReply, xcb_query_keymap_reply);

pub fn OpenFontChecked<'r> (c : &'r Connection,
                        fid : Font,
                        name : &str) -> base::VoidCookie<'r> {
  unsafe {
    let name = (name).as_bytes();
    let name_len = name.len();
    let name_ptr = name.as_ptr();
    let cookie = xcb_open_font_checked(c.get_raw_conn(),
        fid as xcb_font_t, //1
        name_len as u16, //2
        name_ptr as *mut c_char); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn OpenFont<'r> (c : &'r Connection,
                 fid : Font,
                 name : &str) -> base::VoidCookie<'r> {
  unsafe {
    let name = (name).as_bytes();
    let name_len = name.len();
    let name_ptr = name.as_ptr();
    let cookie = xcb_open_font(c.get_raw_conn(),
        fid as xcb_font_t, //1
        name_len as u16, //2
        name_ptr as *mut c_char); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn CloseFontChecked<'r> (c : &'r Connection,
                         font : Font) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_close_font_checked(c.get_raw_conn(),
        font as xcb_font_t); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn CloseFont<'r> (c : &'r Connection,
                  font : Font) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_close_font(c.get_raw_conn(),
        font as xcb_font_t); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl Fontprop {
  pub fn name(&mut self) -> Atom {
    unsafe { accessor!(name -> Atom, self.base.strct) }
  }

  pub fn value(&mut self) -> u32 {
    unsafe { accessor!(value -> u32, self.base.strct) }
  }

}

impl Iterator for FontpropIterator {
    type Item = Fontprop;
    fn next(&mut self) -> Option<Fontprop> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_fontprop_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_fontprop_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct Charinfo {pub base : base::Struct<xcb_charinfo_t> }


impl Charinfo {
  pub fn left_side_bearing(&mut self) -> i16 {
    unsafe { accessor!(left_side_bearing -> i16, self.base.strct) }
  }

  pub fn right_side_bearing(&mut self) -> i16 {
    unsafe { accessor!(right_side_bearing -> i16, self.base.strct) }
  }

  pub fn character_width(&mut self) -> i16 {
    unsafe { accessor!(character_width -> i16, self.base.strct) }
  }

  pub fn ascent(&mut self) -> i16 {
    unsafe { accessor!(ascent -> i16, self.base.strct) }
  }

  pub fn descent(&mut self) -> i16 {
    unsafe { accessor!(descent -> i16, self.base.strct) }
  }

  pub fn attributes(&mut self) -> u16 {
    unsafe { accessor!(attributes -> u16, self.base.strct) }
  }

}

impl Iterator for CharinfoIterator {
    type Item = Charinfo;
    fn next(&mut self) -> Option<Charinfo> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_charinfo_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_charinfo_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct QueryFontReply { base:  base::Reply<xcb_query_font_reply_t> }
fn mk_reply_xcb_query_font_reply_t(reply:*mut xcb_query_font_reply_t) -> QueryFontReply { QueryFontReply { base : base::mk_reply(reply) } }
pub fn QueryFont<'r> (c : &'r Connection,
                  font : Fontable) -> QueryFontCookie<'r> {
  unsafe {
    let cookie = xcb_query_font(c.get_raw_conn(),
        font as xcb_fontable_t); //1
    QueryFontCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn QueryFontUnchecked<'r> (c : &'r Connection,
                           font : Fontable) -> QueryFontCookie<'r> {
  unsafe {
    let cookie = xcb_query_font_unchecked(c.get_raw_conn(),
        font as xcb_fontable_t); //1
    QueryFontCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl QueryFontReply {
  pub fn min_bounds(&self) -> Charinfo {
    unsafe { mem::transmute((*self.base.reply).min_bounds) }
  }
  pub fn max_bounds(&self) -> Charinfo {
    unsafe { mem::transmute((*self.base.reply).max_bounds) }
  }
  pub fn min_char_or_byte2(&mut self) -> u16 {
    unsafe { accessor!(min_char_or_byte2 -> u16, (*self.base.reply)) }
  }

  pub fn max_char_or_byte2(&mut self) -> u16 {
    unsafe { accessor!(max_char_or_byte2 -> u16, (*self.base.reply)) }
  }

  pub fn default_char(&mut self) -> u16 {
    unsafe { accessor!(default_char -> u16, (*self.base.reply)) }
  }

  pub fn draw_direction(&mut self) -> u8 {
    unsafe { accessor!(draw_direction -> u8, (*self.base.reply)) }
  }

  pub fn min_byte1(&mut self) -> u8 {
    unsafe { accessor!(min_byte1 -> u8, (*self.base.reply)) }
  }

  pub fn max_byte1(&mut self) -> u8 {
    unsafe { accessor!(max_byte1 -> u8, (*self.base.reply)) }
  }

  pub fn all_chars_exist(&mut self) -> u8 {
    unsafe { accessor!(all_chars_exist -> u8, (*self.base.reply)) }
  }

  pub fn font_ascent(&mut self) -> i16 {
    unsafe { accessor!(font_ascent -> i16, (*self.base.reply)) }
  }

  pub fn font_descent(&mut self) -> i16 {
    unsafe { accessor!(font_descent -> i16, (*self.base.reply)) }
  }

  pub fn properties(&mut self) -> FontpropIterator {
    unsafe { accessor!(FontpropIterator, xcb_query_font_properties_iterator, (*self.base.reply)) }
  }

  pub fn char_infos(&mut self) -> CharinfoIterator {
    unsafe { accessor!(CharinfoIterator, xcb_query_font_char_infos_iterator, (*self.base.reply)) }
  }

}
impl_reply_cookie!(QueryFontCookie<'s>, mk_reply_xcb_query_font_reply_t, QueryFontReply, xcb_query_font_reply);

pub fn QueryTextExtents<'r> (c : &'r Connection,
                         font : Fontable,
                         string : &[Char2b]) -> QueryTextExtentsCookie<'r> {
  unsafe {
    let string_len = string.len();
    let string_ptr = string.as_ptr();
    let cookie = xcb_query_text_extents(c.get_raw_conn(),
        font as xcb_fontable_t, //1
        string_len as u32, //2
        string_ptr as *mut xcb_char2b_t); //3
    QueryTextExtentsCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn QueryTextExtentsUnchecked<'r> (c : &'r Connection,
                                  font : Fontable,
                                  string : &[Char2b]) -> QueryTextExtentsCookie<'r> {
  unsafe {
    let string_len = string.len();
    let string_ptr = string.as_ptr();
    let cookie = xcb_query_text_extents_unchecked(c.get_raw_conn(),
        font as xcb_fontable_t, //1
        string_len as u32, //2
        string_ptr as *mut xcb_char2b_t); //3
    QueryTextExtentsCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl QueryTextExtentsReply {
  pub fn draw_direction(&mut self) -> u8 {
    unsafe { accessor!(draw_direction -> u8, (*self.base.reply)) }
  }

  pub fn font_ascent(&mut self) -> i16 {
    unsafe { accessor!(font_ascent -> i16, (*self.base.reply)) }
  }

  pub fn font_descent(&mut self) -> i16 {
    unsafe { accessor!(font_descent -> i16, (*self.base.reply)) }
  }

  pub fn overall_ascent(&mut self) -> i16 {
    unsafe { accessor!(overall_ascent -> i16, (*self.base.reply)) }
  }

  pub fn overall_descent(&mut self) -> i16 {
    unsafe { accessor!(overall_descent -> i16, (*self.base.reply)) }
  }

  pub fn overall_width(&mut self) -> i32 {
    unsafe { accessor!(overall_width -> i32, (*self.base.reply)) }
  }

  pub fn overall_left(&mut self) -> i32 {
    unsafe { accessor!(overall_left -> i32, (*self.base.reply)) }
  }

  pub fn overall_right(&mut self) -> i32 {
    unsafe { accessor!(overall_right -> i32, (*self.base.reply)) }
  }

}
impl_reply_cookie!(QueryTextExtentsCookie<'s>, mk_reply_xcb_query_text_extents_reply_t, QueryTextExtentsReply, xcb_query_text_extents_reply);

pub struct Str {pub base : base::Struct<xcb_str_t> }


impl Str {
  pub fn name(&mut self) -> String {
    unsafe { accessor!(str, xcb_str_name_length, xcb_str_name, self.base.strct) }
  }

}

impl Iterator for StrIterator {
    type Item = Str;
    fn next(&mut self) -> Option<Str> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_str_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_str_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub fn ListFonts<'r> (c : &'r Connection,
                  max_names : u16,
                  pattern : &str) -> ListFontsCookie<'r> {
  unsafe {
    let pattern = (pattern).as_bytes();
    let pattern_len = pattern.len();
    let pattern_ptr = pattern.as_ptr();
    let cookie = xcb_list_fonts(c.get_raw_conn(),
        max_names as u16, //1
        pattern_len as u16, //2
        pattern_ptr as *mut c_char); //3
    ListFontsCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn ListFontsUnchecked<'r> (c : &'r Connection,
                           max_names : u16,
                           pattern : &str) -> ListFontsCookie<'r> {
  unsafe {
    let pattern = (pattern).as_bytes();
    let pattern_len = pattern.len();
    let pattern_ptr = pattern.as_ptr();
    let cookie = xcb_list_fonts_unchecked(c.get_raw_conn(),
        max_names as u16, //1
        pattern_len as u16, //2
        pattern_ptr as *mut c_char); //3
    ListFontsCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl ListFontsReply {
  pub fn names(&mut self) -> StrIterator {
    unsafe { accessor!(StrIterator, xcb_list_fonts_names_iterator, (*self.base.reply)) }
  }

}
impl_reply_cookie!(ListFontsCookie<'s>, mk_reply_xcb_list_fonts_reply_t, ListFontsReply, xcb_list_fonts_reply);

pub fn ListFontsWithInfo<'r> (c : &'r Connection,
                          max_names : u16,
                          pattern : &str) -> ListFontsWithInfoCookie<'r> {
  unsafe {
    let pattern = (pattern).as_bytes();
    let pattern_len = pattern.len();
    let pattern_ptr = pattern.as_ptr();
    let cookie = xcb_list_fonts_with_info(c.get_raw_conn(),
        max_names as u16, //1
        pattern_len as u16, //2
        pattern_ptr as *mut c_char); //3
    ListFontsWithInfoCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn ListFontsWithInfoUnchecked<'r> (c : &'r Connection,
                                   max_names : u16,
                                   pattern : &str) -> ListFontsWithInfoCookie<'r> {
  unsafe {
    let pattern = (pattern).as_bytes();
    let pattern_len = pattern.len();
    let pattern_ptr = pattern.as_ptr();
    let cookie = xcb_list_fonts_with_info_unchecked(c.get_raw_conn(),
        max_names as u16, //1
        pattern_len as u16, //2
        pattern_ptr as *mut c_char); //3
    ListFontsWithInfoCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl ListFontsWithInfoReply {
  pub fn min_bounds(&self) -> Charinfo {
    unsafe { mem::transmute((*self.base.reply).min_bounds) }
  }
  pub fn max_bounds(&self) -> Charinfo {
    unsafe { mem::transmute((*self.base.reply).max_bounds) }
  }
  pub fn min_char_or_byte2(&mut self) -> u16 {
    unsafe { accessor!(min_char_or_byte2 -> u16, (*self.base.reply)) }
  }

  pub fn max_char_or_byte2(&mut self) -> u16 {
    unsafe { accessor!(max_char_or_byte2 -> u16, (*self.base.reply)) }
  }

  pub fn default_char(&mut self) -> u16 {
    unsafe { accessor!(default_char -> u16, (*self.base.reply)) }
  }

  pub fn draw_direction(&mut self) -> u8 {
    unsafe { accessor!(draw_direction -> u8, (*self.base.reply)) }
  }

  pub fn min_byte1(&mut self) -> u8 {
    unsafe { accessor!(min_byte1 -> u8, (*self.base.reply)) }
  }

  pub fn max_byte1(&mut self) -> u8 {
    unsafe { accessor!(max_byte1 -> u8, (*self.base.reply)) }
  }

  pub fn all_chars_exist(&mut self) -> u8 {
    unsafe { accessor!(all_chars_exist -> u8, (*self.base.reply)) }
  }

  pub fn font_ascent(&mut self) -> i16 {
    unsafe { accessor!(font_ascent -> i16, (*self.base.reply)) }
  }

  pub fn font_descent(&mut self) -> i16 {
    unsafe { accessor!(font_descent -> i16, (*self.base.reply)) }
  }

  pub fn replies_hint(&mut self) -> u32 {
    unsafe { accessor!(replies_hint -> u32, (*self.base.reply)) }
  }

  pub fn properties(&mut self) -> FontpropIterator {
    unsafe { accessor!(FontpropIterator, xcb_list_fonts_with_info_properties_iterator, (*self.base.reply)) }
  }

  pub fn name(&mut self) -> String {
    unsafe { accessor!(str, xcb_list_fonts_with_info_name_length, xcb_list_fonts_with_info_name, (*self.base.reply)) }
  }

}
impl_reply_cookie!(ListFontsWithInfoCookie<'s>, mk_reply_xcb_list_fonts_with_info_reply_t, ListFontsWithInfoReply, xcb_list_fonts_with_info_reply);

pub fn SetFontPathChecked<'r> (c : &'r Connection,
                           font : &[Str]) -> base::VoidCookie<'r> {
  unsafe {
    let font_len = font.len();
    let font_ptr = font.as_ptr();
    let cookie = xcb_set_font_path_checked(c.get_raw_conn(),
        font_len as u16, //1
        font_ptr as *mut xcb_str_t); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn SetFontPath<'r> (c : &'r Connection,
                    font : &[Str]) -> base::VoidCookie<'r> {
  unsafe {
    let font_len = font.len();
    let font_ptr = font.as_ptr();
    let cookie = xcb_set_font_path(c.get_raw_conn(),
        font_len as u16, //1
        font_ptr as *mut xcb_str_t); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub struct GetFontPathReply { base:  base::Reply<xcb_get_font_path_reply_t> }
fn mk_reply_xcb_get_font_path_reply_t(reply:*mut xcb_get_font_path_reply_t) -> GetFontPathReply { GetFontPathReply { base : base::mk_reply(reply) } }
pub fn GetFontPath<'r> (c : &'r Connection) -> GetFontPathCookie<'r> {
  unsafe {
    let cookie = xcb_get_font_path(c.get_raw_conn());
    GetFontPathCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetFontPathUnchecked<'r> (c : &'r Connection) -> GetFontPathCookie<'r> {
  unsafe {
    let cookie = xcb_get_font_path_unchecked(c.get_raw_conn());
    GetFontPathCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetFontPathReply {
  pub fn path(&mut self) -> StrIterator {
    unsafe { accessor!(StrIterator, xcb_get_font_path_path_iterator, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetFontPathCookie<'s>, mk_reply_xcb_get_font_path_reply_t, GetFontPathReply, xcb_get_font_path_reply);

pub fn CreatePixmapChecked<'r> (c : &'r Connection,
                            depth : u8,
                            pid : Pixmap,
                            drawable : Drawable,
                            width : u16,
                            height : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_create_pixmap_checked(c.get_raw_conn(),
        depth as u8, //1
        pid as xcb_pixmap_t, //2
        drawable as xcb_drawable_t, //3
        width as u16, //4
        height as u16); //5
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn CreatePixmap<'r> (c : &'r Connection,
                     depth : u8,
                     pid : Pixmap,
                     drawable : Drawable,
                     width : u16,
                     height : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_create_pixmap(c.get_raw_conn(),
        depth as u8, //1
        pid as xcb_pixmap_t, //2
        drawable as xcb_drawable_t, //3
        width as u16, //4
        height as u16); //5
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn FreePixmapChecked<'r> (c : &'r Connection,
                          pixmap : Pixmap) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_free_pixmap_checked(c.get_raw_conn(),
        pixmap as xcb_pixmap_t); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn FreePixmap<'r> (c : &'r Connection,
                   pixmap : Pixmap) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_free_pixmap(c.get_raw_conn(),
        pixmap as xcb_pixmap_t); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn CreateGcChecked<'r> (c : &'r Connection,
                        cid : Gcontext,
                        drawable : Drawable,
                        value_list : &[(u32,u32)]) -> base::VoidCookie<'r> {
  unsafe {
    let mut value_list_copy = value_list.to_vec();
    let (value_list_mask, value_list_vec) = pack_bitfield(&mut value_list_copy);
    let value_list_ptr = value_list_vec.as_ptr();
    let cookie = xcb_create_gc_checked(c.get_raw_conn(),
        cid as xcb_gcontext_t, //1
        drawable as xcb_drawable_t, //2
        value_list_mask as u32, //3
        value_list_ptr as *mut u32); //4
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn CreateGc<'r> (c : &'r Connection,
                 cid : Gcontext,
                 drawable : Drawable,
                 value_list : &[(u32,u32)]) -> base::VoidCookie<'r> {
  unsafe {
    let mut value_list_copy = value_list.to_vec();
    let (value_list_mask, value_list_vec) = pack_bitfield(&mut value_list_copy);
    let value_list_ptr = value_list_vec.as_ptr();
    let cookie = xcb_create_gc(c.get_raw_conn(),
        cid as xcb_gcontext_t, //1
        drawable as xcb_drawable_t, //2
        value_list_mask as u32, //3
        value_list_ptr as *mut u32); //4
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn ChangeGcChecked<'r> (c : &'r Connection,
                        gc : Gcontext,
                        value_list : &[(u32,u32)]) -> base::VoidCookie<'r> {
  unsafe {
    let mut value_list_copy = value_list.to_vec();
    let (value_list_mask, value_list_vec) = pack_bitfield(&mut value_list_copy);
    let value_list_ptr = value_list_vec.as_ptr();
    let cookie = xcb_change_gc_checked(c.get_raw_conn(),
        gc as xcb_gcontext_t, //1
        value_list_mask as u32, //2
        value_list_ptr as *mut u32); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn ChangeGc<'r> (c : &'r Connection,
                 gc : Gcontext,
                 value_list : &[(u32,u32)]) -> base::VoidCookie<'r> {
  unsafe {
    let mut value_list_copy = value_list.to_vec();
    let (value_list_mask, value_list_vec) = pack_bitfield(&mut value_list_copy);
    let value_list_ptr = value_list_vec.as_ptr();
    let cookie = xcb_change_gc(c.get_raw_conn(),
        gc as xcb_gcontext_t, //1
        value_list_mask as u32, //2
        value_list_ptr as *mut u32); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn CopyGcChecked<'r> (c : &'r Connection,
                      src_gc : Gcontext,
                      dst_gc : Gcontext,
                      value_mask : u32) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_copy_gc_checked(c.get_raw_conn(),
        src_gc as xcb_gcontext_t, //1
        dst_gc as xcb_gcontext_t, //2
        value_mask as u32); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn CopyGc<'r> (c : &'r Connection,
               src_gc : Gcontext,
               dst_gc : Gcontext,
               value_mask : u32) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_copy_gc(c.get_raw_conn(),
        src_gc as xcb_gcontext_t, //1
        dst_gc as xcb_gcontext_t, //2
        value_mask as u32); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn SetDashesChecked<'r> (c : &'r Connection,
                         gc : Gcontext,
                         dash_offset : u16,
                         dashes : &[u8]) -> base::VoidCookie<'r> {
  unsafe {
    let dashes_len = dashes.len();
    let dashes_ptr = dashes.as_ptr();
    let cookie = xcb_set_dashes_checked(c.get_raw_conn(),
        gc as xcb_gcontext_t, //1
        dash_offset as u16, //2
        dashes_len as u16, //3
        dashes_ptr as *mut u8); //4
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn SetDashes<'r> (c : &'r Connection,
                  gc : Gcontext,
                  dash_offset : u16,
                  dashes : &[u8]) -> base::VoidCookie<'r> {
  unsafe {
    let dashes_len = dashes.len();
    let dashes_ptr = dashes.as_ptr();
    let cookie = xcb_set_dashes(c.get_raw_conn(),
        gc as xcb_gcontext_t, //1
        dash_offset as u16, //2
        dashes_len as u16, //3
        dashes_ptr as *mut u8); //4
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn SetClipRectanglesChecked<'r> (c : &'r Connection,
                                 ordering : u8,
                                 gc : Gcontext,
                                 clip_x_origin : i16,
                                 clip_y_origin : i16,
                                 rectangles : &[Rectangle]) -> base::VoidCookie<'r> {
  unsafe {
    let rectangles_len = rectangles.len();
    let rectangles_ptr = rectangles.as_ptr();
    let cookie = xcb_set_clip_rectangles_checked(c.get_raw_conn(),
        ordering as u8, //1
        gc as xcb_gcontext_t, //2
        clip_x_origin as i16, //3
        clip_y_origin as i16, //4
        rectangles_len as u32, //5
        rectangles_ptr as *mut xcb_rectangle_t); //6
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn SetClipRectangles<'r> (c : &'r Connection,
                          ordering : u8,
                          gc : Gcontext,
                          clip_x_origin : i16,
                          clip_y_origin : i16,
                          rectangles : &[Rectangle]) -> base::VoidCookie<'r> {
  unsafe {
    let rectangles_len = rectangles.len();
    let rectangles_ptr = rectangles.as_ptr();
    let cookie = xcb_set_clip_rectangles(c.get_raw_conn(),
        ordering as u8, //1
        gc as xcb_gcontext_t, //2
        clip_x_origin as i16, //3
        clip_y_origin as i16, //4
        rectangles_len as u32, //5
        rectangles_ptr as *mut xcb_rectangle_t); //6
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn FreeGcChecked<'r> (c : &'r Connection,
                      gc : Gcontext) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_free_gc_checked(c.get_raw_conn(),
        gc as xcb_gcontext_t); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn FreeGc<'r> (c : &'r Connection,
               gc : Gcontext) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_free_gc(c.get_raw_conn(),
        gc as xcb_gcontext_t); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn ClearAreaChecked<'r> (c : &'r Connection,
                         exposures : u8,
                         window : Window,
                         x : i16,
                         y : i16,
                         width : u16,
                         height : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_clear_area_checked(c.get_raw_conn(),
        exposures as u8, //1
        window as xcb_window_t, //2
        x as i16, //3
        y as i16, //4
        width as u16, //5
        height as u16); //6
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn ClearArea<'r> (c : &'r Connection,
                  exposures : u8,
                  window : Window,
                  x : i16,
                  y : i16,
                  width : u16,
                  height : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_clear_area(c.get_raw_conn(),
        exposures as u8, //1
        window as xcb_window_t, //2
        x as i16, //3
        y as i16, //4
        width as u16, //5
        height as u16); //6
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn CopyAreaChecked<'r> (c : &'r Connection,
                        src_drawable : Drawable,
                        dst_drawable : Drawable,
                        gc : Gcontext,
                        src_x : i16,
                        src_y : i16,
                        dst_x : i16,
                        dst_y : i16,
                        width : u16,
                        height : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_copy_area_checked(c.get_raw_conn(),
        src_drawable as xcb_drawable_t, //1
        dst_drawable as xcb_drawable_t, //2
        gc as xcb_gcontext_t, //3
        src_x as i16, //4
        src_y as i16, //5
        dst_x as i16, //6
        dst_y as i16, //7
        width as u16, //8
        height as u16); //9
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn CopyArea<'r> (c : &'r Connection,
                 src_drawable : Drawable,
                 dst_drawable : Drawable,
                 gc : Gcontext,
                 src_x : i16,
                 src_y : i16,
                 dst_x : i16,
                 dst_y : i16,
                 width : u16,
                 height : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_copy_area(c.get_raw_conn(),
        src_drawable as xcb_drawable_t, //1
        dst_drawable as xcb_drawable_t, //2
        gc as xcb_gcontext_t, //3
        src_x as i16, //4
        src_y as i16, //5
        dst_x as i16, //6
        dst_y as i16, //7
        width as u16, //8
        height as u16); //9
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn CopyPlaneChecked<'r> (c : &'r Connection,
                         src_drawable : Drawable,
                         dst_drawable : Drawable,
                         gc : Gcontext,
                         src_x : i16,
                         src_y : i16,
                         dst_x : i16,
                         dst_y : i16,
                         width : u16,
                         height : u16,
                         bit_plane : u32) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_copy_plane_checked(c.get_raw_conn(),
        src_drawable as xcb_drawable_t, //1
        dst_drawable as xcb_drawable_t, //2
        gc as xcb_gcontext_t, //3
        src_x as i16, //4
        src_y as i16, //5
        dst_x as i16, //6
        dst_y as i16, //7
        width as u16, //8
        height as u16, //9
        bit_plane as u32); //10
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn CopyPlane<'r> (c : &'r Connection,
                  src_drawable : Drawable,
                  dst_drawable : Drawable,
                  gc : Gcontext,
                  src_x : i16,
                  src_y : i16,
                  dst_x : i16,
                  dst_y : i16,
                  width : u16,
                  height : u16,
                  bit_plane : u32) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_copy_plane(c.get_raw_conn(),
        src_drawable as xcb_drawable_t, //1
        dst_drawable as xcb_drawable_t, //2
        gc as xcb_gcontext_t, //3
        src_x as i16, //4
        src_y as i16, //5
        dst_x as i16, //6
        dst_y as i16, //7
        width as u16, //8
        height as u16, //9
        bit_plane as u32); //10
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn PolyPointChecked<'r> (c : &'r Connection,
                         coordinate_mode : u8,
                         drawable : Drawable,
                         gc : Gcontext,
                         points : &[Point]) -> base::VoidCookie<'r> {
  unsafe {
    let points_len = points.len();
    let points_ptr = points.as_ptr();
    let cookie = xcb_poly_point_checked(c.get_raw_conn(),
        coordinate_mode as u8, //1
        drawable as xcb_drawable_t, //2
        gc as xcb_gcontext_t, //3
        points_len as u32, //4
        points_ptr as *mut xcb_point_t); //5
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn PolyPoint<'r> (c : &'r Connection,
                  coordinate_mode : u8,
                  drawable : Drawable,
                  gc : Gcontext,
                  points : &[Point]) -> base::VoidCookie<'r> {
  unsafe {
    let points_len = points.len();
    let points_ptr = points.as_ptr();
    let cookie = xcb_poly_point(c.get_raw_conn(),
        coordinate_mode as u8, //1
        drawable as xcb_drawable_t, //2
        gc as xcb_gcontext_t, //3
        points_len as u32, //4
        points_ptr as *mut xcb_point_t); //5
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn PolyLineChecked<'r> (c : &'r Connection,
                        coordinate_mode : u8,
                        drawable : Drawable,
                        gc : Gcontext,
                        points : &[Point]) -> base::VoidCookie<'r> {
  unsafe {
    let points_len = points.len();
    let points_ptr = points.as_ptr();
    let cookie = xcb_poly_line_checked(c.get_raw_conn(),
        coordinate_mode as u8, //1
        drawable as xcb_drawable_t, //2
        gc as xcb_gcontext_t, //3
        points_len as u32, //4
        points_ptr as *mut xcb_point_t); //5
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn PolyLine<'r> (c : &'r Connection,
                 coordinate_mode : u8,
                 drawable : Drawable,
                 gc : Gcontext,
                 points : &[Point]) -> base::VoidCookie<'r> {
  unsafe {
    let points_len = points.len();
    let points_ptr = points.as_ptr();
    let cookie = xcb_poly_line(c.get_raw_conn(),
        coordinate_mode as u8, //1
        drawable as xcb_drawable_t, //2
        gc as xcb_gcontext_t, //3
        points_len as u32, //4
        points_ptr as *mut xcb_point_t); //5
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub struct Segment {pub base : base::Struct<xcb_segment_t> }


impl Segment {
  pub fn x1(&mut self) -> i16 {
    unsafe { accessor!(x1 -> i16, self.base.strct) }
  }

  pub fn y1(&mut self) -> i16 {
    unsafe { accessor!(y1 -> i16, self.base.strct) }
  }

  pub fn x2(&mut self) -> i16 {
    unsafe { accessor!(x2 -> i16, self.base.strct) }
  }

  pub fn y2(&mut self) -> i16 {
    unsafe { accessor!(y2 -> i16, self.base.strct) }
  }

}

impl Iterator for SegmentIterator {
    type Item = Segment;
    fn next(&mut self) -> Option<Segment> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_segment_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_segment_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub fn PolySegmentChecked<'r> (c : &'r Connection,
                           drawable : Drawable,
                           gc : Gcontext,
                           segments : &[Segment]) -> base::VoidCookie<'r> {
  unsafe {
    let segments_len = segments.len();
    let segments_ptr = segments.as_ptr();
    let cookie = xcb_poly_segment_checked(c.get_raw_conn(),
        drawable as xcb_drawable_t, //1
        gc as xcb_gcontext_t, //2
        segments_len as u32, //3
        segments_ptr as *mut xcb_segment_t); //4
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn PolySegment<'r> (c : &'r Connection,
                    drawable : Drawable,
                    gc : Gcontext,
                    segments : &[Segment]) -> base::VoidCookie<'r> {
  unsafe {
    let segments_len = segments.len();
    let segments_ptr = segments.as_ptr();
    let cookie = xcb_poly_segment(c.get_raw_conn(),
        drawable as xcb_drawable_t, //1
        gc as xcb_gcontext_t, //2
        segments_len as u32, //3
        segments_ptr as *mut xcb_segment_t); //4
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn PolyRectangleChecked<'r> (c : &'r Connection,
                             drawable : Drawable,
                             gc : Gcontext,
                             rectangles : &[Rectangle]) -> base::VoidCookie<'r> {
  unsafe {
    let rectangles_len = rectangles.len();
    let rectangles_ptr = rectangles.as_ptr();
    let cookie = xcb_poly_rectangle_checked(c.get_raw_conn(),
        drawable as xcb_drawable_t, //1
        gc as xcb_gcontext_t, //2
        rectangles_len as u32, //3
        rectangles_ptr as *mut xcb_rectangle_t); //4
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn PolyRectangle<'r> (c : &'r Connection,
                      drawable : Drawable,
                      gc : Gcontext,
                      rectangles : &[Rectangle]) -> base::VoidCookie<'r> {
  unsafe {
    let rectangles_len = rectangles.len();
    let rectangles_ptr = rectangles.as_ptr();
    let cookie = xcb_poly_rectangle(c.get_raw_conn(),
        drawable as xcb_drawable_t, //1
        gc as xcb_gcontext_t, //2
        rectangles_len as u32, //3
        rectangles_ptr as *mut xcb_rectangle_t); //4
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn PolyArcChecked<'r> (c : &'r Connection,
                       drawable : Drawable,
                       gc : Gcontext,
                       arcs : &[Arc]) -> base::VoidCookie<'r> {
  unsafe {
    let arcs_len = arcs.len();
    let arcs_ptr = arcs.as_ptr();
    let cookie = xcb_poly_arc_checked(c.get_raw_conn(),
        drawable as xcb_drawable_t, //1
        gc as xcb_gcontext_t, //2
        arcs_len as u32, //3
        arcs_ptr as *mut xcb_arc_t); //4
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn PolyArc<'r> (c : &'r Connection,
                drawable : Drawable,
                gc : Gcontext,
                arcs : &[Arc]) -> base::VoidCookie<'r> {
  unsafe {
    let arcs_len = arcs.len();
    let arcs_ptr = arcs.as_ptr();
    let cookie = xcb_poly_arc(c.get_raw_conn(),
        drawable as xcb_drawable_t, //1
        gc as xcb_gcontext_t, //2
        arcs_len as u32, //3
        arcs_ptr as *mut xcb_arc_t); //4
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn FillPolyChecked<'r> (c : &'r Connection,
                        drawable : Drawable,
                        gc : Gcontext,
                        shape : u8,
                        coordinate_mode : u8,
                        points : &[Point]) -> base::VoidCookie<'r> {
  unsafe {
    let points_len = points.len();
    let points_ptr = points.as_ptr();
    let cookie = xcb_fill_poly_checked(c.get_raw_conn(),
        drawable as xcb_drawable_t, //1
        gc as xcb_gcontext_t, //2
        shape as u8, //3
        coordinate_mode as u8, //4
        points_len as u32, //5
        points_ptr as *mut xcb_point_t); //6
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn FillPoly<'r> (c : &'r Connection,
                 drawable : Drawable,
                 gc : Gcontext,
                 shape : u8,
                 coordinate_mode : u8,
                 points : &[Point]) -> base::VoidCookie<'r> {
  unsafe {
    let points_len = points.len();
    let points_ptr = points.as_ptr();
    let cookie = xcb_fill_poly(c.get_raw_conn(),
        drawable as xcb_drawable_t, //1
        gc as xcb_gcontext_t, //2
        shape as u8, //3
        coordinate_mode as u8, //4
        points_len as u32, //5
        points_ptr as *mut xcb_point_t); //6
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn PolyFillRectangleChecked<'r> (c : &'r Connection,
                                 drawable : Drawable,
                                 gc : Gcontext,
                                 rectangles : &[Rectangle]) -> base::VoidCookie<'r> {
  unsafe {
    let rectangles_len = rectangles.len();
    let rectangles_ptr = rectangles.as_ptr();
    let cookie = xcb_poly_fill_rectangle_checked(c.get_raw_conn(),
        drawable as xcb_drawable_t, //1
        gc as xcb_gcontext_t, //2
        rectangles_len as u32, //3
        rectangles_ptr as *mut xcb_rectangle_t); //4
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn PolyFillRectangle<'r> (c : &'r Connection,
                          drawable : Drawable,
                          gc : Gcontext,
                          rectangles : &[Rectangle]) -> base::VoidCookie<'r> {
  unsafe {
    let rectangles_len = rectangles.len();
    let rectangles_ptr = rectangles.as_ptr();
    let cookie = xcb_poly_fill_rectangle(c.get_raw_conn(),
        drawable as xcb_drawable_t, //1
        gc as xcb_gcontext_t, //2
        rectangles_len as u32, //3
        rectangles_ptr as *mut xcb_rectangle_t); //4
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn PolyFillArcChecked<'r> (c : &'r Connection,
                           drawable : Drawable,
                           gc : Gcontext,
                           arcs : &[Arc]) -> base::VoidCookie<'r> {
  unsafe {
    let arcs_len = arcs.len();
    let arcs_ptr = arcs.as_ptr();
    let cookie = xcb_poly_fill_arc_checked(c.get_raw_conn(),
        drawable as xcb_drawable_t, //1
        gc as xcb_gcontext_t, //2
        arcs_len as u32, //3
        arcs_ptr as *mut xcb_arc_t); //4
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn PolyFillArc<'r> (c : &'r Connection,
                    drawable : Drawable,
                    gc : Gcontext,
                    arcs : &[Arc]) -> base::VoidCookie<'r> {
  unsafe {
    let arcs_len = arcs.len();
    let arcs_ptr = arcs.as_ptr();
    let cookie = xcb_poly_fill_arc(c.get_raw_conn(),
        drawable as xcb_drawable_t, //1
        gc as xcb_gcontext_t, //2
        arcs_len as u32, //3
        arcs_ptr as *mut xcb_arc_t); //4
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn PutImageChecked<'r> (c : &'r Connection,
                        format : u8,
                        drawable : Drawable,
                        gc : Gcontext,
                        width : u16,
                        height : u16,
                        dst_x : i16,
                        dst_y : i16,
                        left_pad : u8,
                        depth : u8,
                        data : &[u8]) -> base::VoidCookie<'r> {
  unsafe {
    let data_len = data.len();
    let data_ptr = data.as_ptr();
    let cookie = xcb_put_image_checked(c.get_raw_conn(),
        format as u8, //1
        drawable as xcb_drawable_t, //2
        gc as xcb_gcontext_t, //3
        width as u16, //4
        height as u16, //5
        dst_x as i16, //6
        dst_y as i16, //7
        left_pad as u8, //8
        depth as u8, //9
        data_len as u32, //10
        data_ptr as *mut u8); //11
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn PutImage<'r> (c : &'r Connection,
                 format : u8,
                 drawable : Drawable,
                 gc : Gcontext,
                 width : u16,
                 height : u16,
                 dst_x : i16,
                 dst_y : i16,
                 left_pad : u8,
                 depth : u8,
                 data : &[u8]) -> base::VoidCookie<'r> {
  unsafe {
    let data_len = data.len();
    let data_ptr = data.as_ptr();
    let cookie = xcb_put_image(c.get_raw_conn(),
        format as u8, //1
        drawable as xcb_drawable_t, //2
        gc as xcb_gcontext_t, //3
        width as u16, //4
        height as u16, //5
        dst_x as i16, //6
        dst_y as i16, //7
        left_pad as u8, //8
        depth as u8, //9
        data_len as u32, //10
        data_ptr as *mut u8); //11
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub struct GetImageReply { base:  base::Reply<xcb_get_image_reply_t> }
fn mk_reply_xcb_get_image_reply_t(reply:*mut xcb_get_image_reply_t) -> GetImageReply { GetImageReply { base : base::mk_reply(reply) } }
pub fn GetImage<'r> (c : &'r Connection,
                 format : u8,
                 drawable : Drawable,
                 x : i16,
                 y : i16,
                 width : u16,
                 height : u16,
                 plane_mask : u32) -> GetImageCookie<'r> {
  unsafe {
    let cookie = xcb_get_image(c.get_raw_conn(),
        format as u8, //1
        drawable as xcb_drawable_t, //2
        x as i16, //3
        y as i16, //4
        width as u16, //5
        height as u16, //6
        plane_mask as u32); //7
    GetImageCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetImageUnchecked<'r> (c : &'r Connection,
                          format : u8,
                          drawable : Drawable,
                          x : i16,
                          y : i16,
                          width : u16,
                          height : u16,
                          plane_mask : u32) -> GetImageCookie<'r> {
  unsafe {
    let cookie = xcb_get_image_unchecked(c.get_raw_conn(),
        format as u8, //1
        drawable as xcb_drawable_t, //2
        x as i16, //3
        y as i16, //4
        width as u16, //5
        height as u16, //6
        plane_mask as u32); //7
    GetImageCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetImageReply {
  pub fn depth(&mut self) -> u8 {
    unsafe { accessor!(depth -> u8, (*self.base.reply)) }
  }

  pub fn visual(&mut self) -> Visualid {
    unsafe { accessor!(visual -> Visualid, (*self.base.reply)) }
  }

  pub fn data(&mut self) -> Vec<u8> {
    unsafe { accessor!(u8, xcb_get_image_data_length, xcb_get_image_data, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetImageCookie<'s>, mk_reply_xcb_get_image_reply_t, GetImageReply, xcb_get_image_reply);

pub fn PolyText8Checked<'r> (c : &'r Connection,
                         drawable : Drawable,
                         gc : Gcontext,
                         x : i16,
                         y : i16,
                         items : &[u8]) -> base::VoidCookie<'r> {
  unsafe {
    let items_len = items.len();
    let items_ptr = items.as_ptr();
    let cookie = xcb_poly_text_8_checked(c.get_raw_conn(),
        drawable as xcb_drawable_t, //1
        gc as xcb_gcontext_t, //2
        x as i16, //3
        y as i16, //4
        items_len as u32, //5
        items_ptr as *mut u8); //6
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn PolyText8<'r> (c : &'r Connection,
                  drawable : Drawable,
                  gc : Gcontext,
                  x : i16,
                  y : i16,
                  items : &[u8]) -> base::VoidCookie<'r> {
  unsafe {
    let items_len = items.len();
    let items_ptr = items.as_ptr();
    let cookie = xcb_poly_text_8(c.get_raw_conn(),
        drawable as xcb_drawable_t, //1
        gc as xcb_gcontext_t, //2
        x as i16, //3
        y as i16, //4
        items_len as u32, //5
        items_ptr as *mut u8); //6
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn PolyText16Checked<'r> (c : &'r Connection,
                          drawable : Drawable,
                          gc : Gcontext,
                          x : i16,
                          y : i16,
                          items : &[u8]) -> base::VoidCookie<'r> {
  unsafe {
    let items_len = items.len();
    let items_ptr = items.as_ptr();
    let cookie = xcb_poly_text_16_checked(c.get_raw_conn(),
        drawable as xcb_drawable_t, //1
        gc as xcb_gcontext_t, //2
        x as i16, //3
        y as i16, //4
        items_len as u32, //5
        items_ptr as *mut u8); //6
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn PolyText16<'r> (c : &'r Connection,
                   drawable : Drawable,
                   gc : Gcontext,
                   x : i16,
                   y : i16,
                   items : &[u8]) -> base::VoidCookie<'r> {
  unsafe {
    let items_len = items.len();
    let items_ptr = items.as_ptr();
    let cookie = xcb_poly_text_16(c.get_raw_conn(),
        drawable as xcb_drawable_t, //1
        gc as xcb_gcontext_t, //2
        x as i16, //3
        y as i16, //4
        items_len as u32, //5
        items_ptr as *mut u8); //6
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn ImageText8Checked<'r> (c : &'r Connection,
                          drawable : Drawable,
                          gc : Gcontext,
                          x : i16,
                          y : i16,
                          string : &str) -> base::VoidCookie<'r> {
  unsafe {
    let string = (string).as_bytes();
    let string_len = string.len();
    let string_ptr = string.as_ptr();
    let cookie = xcb_image_text_8_checked(c.get_raw_conn(),
        string_len as u8, //1
        drawable as xcb_drawable_t, //2
        gc as xcb_gcontext_t, //3
        x as i16, //4
        y as i16, //5
        string_ptr as *mut c_char); //6
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn ImageText8<'r> (c : &'r Connection,
                   drawable : Drawable,
                   gc : Gcontext,
                   x : i16,
                   y : i16,
                   string : &str) -> base::VoidCookie<'r> {
  unsafe {
    let string = (string).as_bytes();
    let string_len = string.len();
    let string_ptr = string.as_ptr();
    let cookie = xcb_image_text_8(c.get_raw_conn(),
        string_len as u8, //1
        drawable as xcb_drawable_t, //2
        gc as xcb_gcontext_t, //3
        x as i16, //4
        y as i16, //5
        string_ptr as *mut c_char); //6
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn ImageText16Checked<'r> (c : &'r Connection,
                           drawable : Drawable,
                           gc : Gcontext,
                           x : i16,
                           y : i16,
                           string : &[Char2b]) -> base::VoidCookie<'r> {
  unsafe {
    let string_len = string.len();
    let string_ptr = string.as_ptr();
    let cookie = xcb_image_text_16_checked(c.get_raw_conn(),
        string_len as u8, //1
        drawable as xcb_drawable_t, //2
        gc as xcb_gcontext_t, //3
        x as i16, //4
        y as i16, //5
        string_ptr as *mut xcb_char2b_t); //6
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn ImageText16<'r> (c : &'r Connection,
                    drawable : Drawable,
                    gc : Gcontext,
                    x : i16,
                    y : i16,
                    string : &[Char2b]) -> base::VoidCookie<'r> {
  unsafe {
    let string_len = string.len();
    let string_ptr = string.as_ptr();
    let cookie = xcb_image_text_16(c.get_raw_conn(),
        string_len as u8, //1
        drawable as xcb_drawable_t, //2
        gc as xcb_gcontext_t, //3
        x as i16, //4
        y as i16, //5
        string_ptr as *mut xcb_char2b_t); //6
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn CreateColormapChecked<'r> (c : &'r Connection,
                              alloc : u8,
                              mid : Colormap,
                              window : Window,
                              visual : Visualid) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_create_colormap_checked(c.get_raw_conn(),
        alloc as u8, //1
        mid as xcb_colormap_t, //2
        window as xcb_window_t, //3
        visual as xcb_visualid_t); //4
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn CreateColormap<'r> (c : &'r Connection,
                       alloc : u8,
                       mid : Colormap,
                       window : Window,
                       visual : Visualid) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_create_colormap(c.get_raw_conn(),
        alloc as u8, //1
        mid as xcb_colormap_t, //2
        window as xcb_window_t, //3
        visual as xcb_visualid_t); //4
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn FreeColormapChecked<'r> (c : &'r Connection,
                            cmap : Colormap) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_free_colormap_checked(c.get_raw_conn(),
        cmap as xcb_colormap_t); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn FreeColormap<'r> (c : &'r Connection,
                     cmap : Colormap) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_free_colormap(c.get_raw_conn(),
        cmap as xcb_colormap_t); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn CopyColormapAndFreeChecked<'r> (c : &'r Connection,
                                   mid : Colormap,
                                   src_cmap : Colormap) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_copy_colormap_and_free_checked(c.get_raw_conn(),
        mid as xcb_colormap_t, //1
        src_cmap as xcb_colormap_t); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn CopyColormapAndFree<'r> (c : &'r Connection,
                            mid : Colormap,
                            src_cmap : Colormap) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_copy_colormap_and_free(c.get_raw_conn(),
        mid as xcb_colormap_t, //1
        src_cmap as xcb_colormap_t); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn InstallColormapChecked<'r> (c : &'r Connection,
                               cmap : Colormap) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_install_colormap_checked(c.get_raw_conn(),
        cmap as xcb_colormap_t); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn InstallColormap<'r> (c : &'r Connection,
                        cmap : Colormap) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_install_colormap(c.get_raw_conn(),
        cmap as xcb_colormap_t); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn UninstallColormapChecked<'r> (c : &'r Connection,
                                 cmap : Colormap) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_uninstall_colormap_checked(c.get_raw_conn(),
        cmap as xcb_colormap_t); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn UninstallColormap<'r> (c : &'r Connection,
                          cmap : Colormap) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_uninstall_colormap(c.get_raw_conn(),
        cmap as xcb_colormap_t); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub struct ListInstalledColormapsReply { base:  base::Reply<xcb_list_installed_colormaps_reply_t> }
fn mk_reply_xcb_list_installed_colormaps_reply_t(reply:*mut xcb_list_installed_colormaps_reply_t) -> ListInstalledColormapsReply { ListInstalledColormapsReply { base : base::mk_reply(reply) } }
pub fn ListInstalledColormaps<'r> (c : &'r Connection,
                               window : Window) -> ListInstalledColormapsCookie<'r> {
  unsafe {
    let cookie = xcb_list_installed_colormaps(c.get_raw_conn(),
        window as xcb_window_t); //1
    ListInstalledColormapsCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn ListInstalledColormapsUnchecked<'r> (c : &'r Connection,
                                        window : Window) -> ListInstalledColormapsCookie<'r> {
  unsafe {
    let cookie = xcb_list_installed_colormaps_unchecked(c.get_raw_conn(),
        window as xcb_window_t); //1
    ListInstalledColormapsCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl ListInstalledColormapsReply {
  pub fn cmaps(&mut self) -> Vec<Colormap> {
    unsafe { accessor!(Colormap, xcb_list_installed_colormaps_cmaps_length, xcb_list_installed_colormaps_cmaps, (*self.base.reply)) }
  }

}
impl_reply_cookie!(ListInstalledColormapsCookie<'s>, mk_reply_xcb_list_installed_colormaps_reply_t, ListInstalledColormapsReply, xcb_list_installed_colormaps_reply);

pub fn AllocColor<'r> (c : &'r Connection,
                   cmap : Colormap,
                   red : u16,
                   green : u16,
                   blue : u16) -> AllocColorCookie<'r> {
  unsafe {
    let cookie = xcb_alloc_color(c.get_raw_conn(),
        cmap as xcb_colormap_t, //1
        red as u16, //2
        green as u16, //3
        blue as u16); //4
    AllocColorCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn AllocColorUnchecked<'r> (c : &'r Connection,
                            cmap : Colormap,
                            red : u16,
                            green : u16,
                            blue : u16) -> AllocColorCookie<'r> {
  unsafe {
    let cookie = xcb_alloc_color_unchecked(c.get_raw_conn(),
        cmap as xcb_colormap_t, //1
        red as u16, //2
        green as u16, //3
        blue as u16); //4
    AllocColorCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl AllocColorReply {
  pub fn red(&mut self) -> u16 {
    unsafe { accessor!(red -> u16, (*self.base.reply)) }
  }

  pub fn green(&mut self) -> u16 {
    unsafe { accessor!(green -> u16, (*self.base.reply)) }
  }

  pub fn blue(&mut self) -> u16 {
    unsafe { accessor!(blue -> u16, (*self.base.reply)) }
  }

  pub fn pixel(&mut self) -> u32 {
    unsafe { accessor!(pixel -> u32, (*self.base.reply)) }
  }

}
impl_reply_cookie!(AllocColorCookie<'s>, mk_reply_xcb_alloc_color_reply_t, AllocColorReply, xcb_alloc_color_reply);

pub fn AllocNamedColor<'r> (c : &'r Connection,
                        cmap : Colormap,
                        name : &str) -> AllocNamedColorCookie<'r> {
  unsafe {
    let name = (name).as_bytes();
    let name_len = name.len();
    let name_ptr = name.as_ptr();
    let cookie = xcb_alloc_named_color(c.get_raw_conn(),
        cmap as xcb_colormap_t, //1
        name_len as u16, //2
        name_ptr as *mut c_char); //3
    AllocNamedColorCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn AllocNamedColorUnchecked<'r> (c : &'r Connection,
                                 cmap : Colormap,
                                 name : &str) -> AllocNamedColorCookie<'r> {
  unsafe {
    let name = (name).as_bytes();
    let name_len = name.len();
    let name_ptr = name.as_ptr();
    let cookie = xcb_alloc_named_color_unchecked(c.get_raw_conn(),
        cmap as xcb_colormap_t, //1
        name_len as u16, //2
        name_ptr as *mut c_char); //3
    AllocNamedColorCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl AllocNamedColorReply {
  pub fn pixel(&mut self) -> u32 {
    unsafe { accessor!(pixel -> u32, (*self.base.reply)) }
  }

  pub fn exact_red(&mut self) -> u16 {
    unsafe { accessor!(exact_red -> u16, (*self.base.reply)) }
  }

  pub fn exact_green(&mut self) -> u16 {
    unsafe { accessor!(exact_green -> u16, (*self.base.reply)) }
  }

  pub fn exact_blue(&mut self) -> u16 {
    unsafe { accessor!(exact_blue -> u16, (*self.base.reply)) }
  }

  pub fn visual_red(&mut self) -> u16 {
    unsafe { accessor!(visual_red -> u16, (*self.base.reply)) }
  }

  pub fn visual_green(&mut self) -> u16 {
    unsafe { accessor!(visual_green -> u16, (*self.base.reply)) }
  }

  pub fn visual_blue(&mut self) -> u16 {
    unsafe { accessor!(visual_blue -> u16, (*self.base.reply)) }
  }

}
impl_reply_cookie!(AllocNamedColorCookie<'s>, mk_reply_xcb_alloc_named_color_reply_t, AllocNamedColorReply, xcb_alloc_named_color_reply);

pub struct AllocColorCellsReply { base:  base::Reply<xcb_alloc_color_cells_reply_t> }
fn mk_reply_xcb_alloc_color_cells_reply_t(reply:*mut xcb_alloc_color_cells_reply_t) -> AllocColorCellsReply { AllocColorCellsReply { base : base::mk_reply(reply) } }
pub fn AllocColorCells<'r> (c : &'r Connection,
                        contiguous : u8,
                        cmap : Colormap,
                        colors : u16,
                        planes : u16) -> AllocColorCellsCookie<'r> {
  unsafe {
    let cookie = xcb_alloc_color_cells(c.get_raw_conn(),
        contiguous as u8, //1
        cmap as xcb_colormap_t, //2
        colors as u16, //3
        planes as u16); //4
    AllocColorCellsCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn AllocColorCellsUnchecked<'r> (c : &'r Connection,
                                 contiguous : u8,
                                 cmap : Colormap,
                                 colors : u16,
                                 planes : u16) -> AllocColorCellsCookie<'r> {
  unsafe {
    let cookie = xcb_alloc_color_cells_unchecked(c.get_raw_conn(),
        contiguous as u8, //1
        cmap as xcb_colormap_t, //2
        colors as u16, //3
        planes as u16); //4
    AllocColorCellsCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl AllocColorCellsReply {
  pub fn pixels(&mut self) -> Vec<u32> {
    unsafe { accessor!(u32, xcb_alloc_color_cells_pixels_length, xcb_alloc_color_cells_pixels, (*self.base.reply)) }
  }

  pub fn masks(&mut self) -> Vec<u32> {
    unsafe { accessor!(u32, xcb_alloc_color_cells_masks_length, xcb_alloc_color_cells_masks, (*self.base.reply)) }
  }

}
impl_reply_cookie!(AllocColorCellsCookie<'s>, mk_reply_xcb_alloc_color_cells_reply_t, AllocColorCellsReply, xcb_alloc_color_cells_reply);

pub struct AllocColorPlanesReply { base:  base::Reply<xcb_alloc_color_planes_reply_t> }
fn mk_reply_xcb_alloc_color_planes_reply_t(reply:*mut xcb_alloc_color_planes_reply_t) -> AllocColorPlanesReply { AllocColorPlanesReply { base : base::mk_reply(reply) } }
pub fn AllocColorPlanes<'r> (c : &'r Connection,
                         contiguous : u8,
                         cmap : Colormap,
                         colors : u16,
                         reds : u16,
                         greens : u16,
                         blues : u16) -> AllocColorPlanesCookie<'r> {
  unsafe {
    let cookie = xcb_alloc_color_planes(c.get_raw_conn(),
        contiguous as u8, //1
        cmap as xcb_colormap_t, //2
        colors as u16, //3
        reds as u16, //4
        greens as u16, //5
        blues as u16); //6
    AllocColorPlanesCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn AllocColorPlanesUnchecked<'r> (c : &'r Connection,
                                  contiguous : u8,
                                  cmap : Colormap,
                                  colors : u16,
                                  reds : u16,
                                  greens : u16,
                                  blues : u16) -> AllocColorPlanesCookie<'r> {
  unsafe {
    let cookie = xcb_alloc_color_planes_unchecked(c.get_raw_conn(),
        contiguous as u8, //1
        cmap as xcb_colormap_t, //2
        colors as u16, //3
        reds as u16, //4
        greens as u16, //5
        blues as u16); //6
    AllocColorPlanesCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl AllocColorPlanesReply {
  pub fn red_mask(&mut self) -> u32 {
    unsafe { accessor!(red_mask -> u32, (*self.base.reply)) }
  }

  pub fn green_mask(&mut self) -> u32 {
    unsafe { accessor!(green_mask -> u32, (*self.base.reply)) }
  }

  pub fn blue_mask(&mut self) -> u32 {
    unsafe { accessor!(blue_mask -> u32, (*self.base.reply)) }
  }

  pub fn pixels(&mut self) -> Vec<u32> {
    unsafe { accessor!(u32, xcb_alloc_color_planes_pixels_length, xcb_alloc_color_planes_pixels, (*self.base.reply)) }
  }

}
impl_reply_cookie!(AllocColorPlanesCookie<'s>, mk_reply_xcb_alloc_color_planes_reply_t, AllocColorPlanesReply, xcb_alloc_color_planes_reply);

pub fn FreeColorsChecked<'r> (c : &'r Connection,
                          cmap : Colormap,
                          plane_mask : u32,
                          pixels : &[u32]) -> base::VoidCookie<'r> {
  unsafe {
    let pixels_len = pixels.len();
    let pixels_ptr = pixels.as_ptr();
    let cookie = xcb_free_colors_checked(c.get_raw_conn(),
        cmap as xcb_colormap_t, //1
        plane_mask as u32, //2
        pixels_len as u32, //3
        pixels_ptr as *mut u32); //4
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn FreeColors<'r> (c : &'r Connection,
                   cmap : Colormap,
                   plane_mask : u32,
                   pixels : &[u32]) -> base::VoidCookie<'r> {
  unsafe {
    let pixels_len = pixels.len();
    let pixels_ptr = pixels.as_ptr();
    let cookie = xcb_free_colors(c.get_raw_conn(),
        cmap as xcb_colormap_t, //1
        plane_mask as u32, //2
        pixels_len as u32, //3
        pixels_ptr as *mut u32); //4
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl Coloritem {
  pub fn pixel(&mut self) -> u32 {
    unsafe { accessor!(pixel -> u32, self.base.strct) }
  }

  pub fn red(&mut self) -> u16 {
    unsafe { accessor!(red -> u16, self.base.strct) }
  }

  pub fn green(&mut self) -> u16 {
    unsafe { accessor!(green -> u16, self.base.strct) }
  }

  pub fn blue(&mut self) -> u16 {
    unsafe { accessor!(blue -> u16, self.base.strct) }
  }

  pub fn flags(&mut self) -> u8 {
    unsafe { accessor!(flags -> u8, self.base.strct) }
  }

}

impl Iterator for ColoritemIterator {
    type Item = Coloritem;
    fn next(&mut self) -> Option<Coloritem> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_coloritem_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_coloritem_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub fn StoreColorsChecked<'r> (c : &'r Connection,
                           cmap : Colormap,
                           items : &[Coloritem]) -> base::VoidCookie<'r> {
  unsafe {
    let items_len = items.len();
    let items_ptr = items.as_ptr();
    let cookie = xcb_store_colors_checked(c.get_raw_conn(),
        cmap as xcb_colormap_t, //1
        items_len as u32, //2
        items_ptr as *mut xcb_coloritem_t); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn StoreColors<'r> (c : &'r Connection,
                    cmap : Colormap,
                    items : &[Coloritem]) -> base::VoidCookie<'r> {
  unsafe {
    let items_len = items.len();
    let items_ptr = items.as_ptr();
    let cookie = xcb_store_colors(c.get_raw_conn(),
        cmap as xcb_colormap_t, //1
        items_len as u32, //2
        items_ptr as *mut xcb_coloritem_t); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn StoreNamedColorChecked<'r> (c : &'r Connection,
                               flags : u8,
                               cmap : Colormap,
                               pixel : u32,
                               name : &str) -> base::VoidCookie<'r> {
  unsafe {
    let name = (name).as_bytes();
    let name_len = name.len();
    let name_ptr = name.as_ptr();
    let cookie = xcb_store_named_color_checked(c.get_raw_conn(),
        flags as u8, //1
        cmap as xcb_colormap_t, //2
        pixel as u32, //3
        name_len as u16, //4
        name_ptr as *mut c_char); //5
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn StoreNamedColor<'r> (c : &'r Connection,
                        flags : u8,
                        cmap : Colormap,
                        pixel : u32,
                        name : &str) -> base::VoidCookie<'r> {
  unsafe {
    let name = (name).as_bytes();
    let name_len = name.len();
    let name_ptr = name.as_ptr();
    let cookie = xcb_store_named_color(c.get_raw_conn(),
        flags as u8, //1
        cmap as xcb_colormap_t, //2
        pixel as u32, //3
        name_len as u16, //4
        name_ptr as *mut c_char); //5
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub struct Rgb {pub base : base::Struct<xcb_rgb_t> }


impl Rgb {
  pub fn red(&mut self) -> u16 {
    unsafe { accessor!(red -> u16, self.base.strct) }
  }

  pub fn green(&mut self) -> u16 {
    unsafe { accessor!(green -> u16, self.base.strct) }
  }

  pub fn blue(&mut self) -> u16 {
    unsafe { accessor!(blue -> u16, self.base.strct) }
  }

}

impl Iterator for RgbIterator {
    type Item = Rgb;
    fn next(&mut self) -> Option<Rgb> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_rgb_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_rgb_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub fn QueryColors<'r> (c : &'r Connection,
                    cmap : Colormap,
                    pixels : &[u32]) -> QueryColorsCookie<'r> {
  unsafe {
    let pixels_len = pixels.len();
    let pixels_ptr = pixels.as_ptr();
    let cookie = xcb_query_colors(c.get_raw_conn(),
        cmap as xcb_colormap_t, //1
        pixels_len as u32, //2
        pixels_ptr as *mut u32); //3
    QueryColorsCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn QueryColorsUnchecked<'r> (c : &'r Connection,
                             cmap : Colormap,
                             pixels : &[u32]) -> QueryColorsCookie<'r> {
  unsafe {
    let pixels_len = pixels.len();
    let pixels_ptr = pixels.as_ptr();
    let cookie = xcb_query_colors_unchecked(c.get_raw_conn(),
        cmap as xcb_colormap_t, //1
        pixels_len as u32, //2
        pixels_ptr as *mut u32); //3
    QueryColorsCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl QueryColorsReply {
  pub fn colors(&mut self) -> RgbIterator {
    unsafe { accessor!(RgbIterator, xcb_query_colors_colors_iterator, (*self.base.reply)) }
  }

}
impl_reply_cookie!(QueryColorsCookie<'s>, mk_reply_xcb_query_colors_reply_t, QueryColorsReply, xcb_query_colors_reply);

pub fn LookupColor<'r> (c : &'r Connection,
                    cmap : Colormap,
                    name : &str) -> LookupColorCookie<'r> {
  unsafe {
    let name = (name).as_bytes();
    let name_len = name.len();
    let name_ptr = name.as_ptr();
    let cookie = xcb_lookup_color(c.get_raw_conn(),
        cmap as xcb_colormap_t, //1
        name_len as u16, //2
        name_ptr as *mut c_char); //3
    LookupColorCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn LookupColorUnchecked<'r> (c : &'r Connection,
                             cmap : Colormap,
                             name : &str) -> LookupColorCookie<'r> {
  unsafe {
    let name = (name).as_bytes();
    let name_len = name.len();
    let name_ptr = name.as_ptr();
    let cookie = xcb_lookup_color_unchecked(c.get_raw_conn(),
        cmap as xcb_colormap_t, //1
        name_len as u16, //2
        name_ptr as *mut c_char); //3
    LookupColorCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl LookupColorReply {
  pub fn exact_red(&mut self) -> u16 {
    unsafe { accessor!(exact_red -> u16, (*self.base.reply)) }
  }

  pub fn exact_green(&mut self) -> u16 {
    unsafe { accessor!(exact_green -> u16, (*self.base.reply)) }
  }

  pub fn exact_blue(&mut self) -> u16 {
    unsafe { accessor!(exact_blue -> u16, (*self.base.reply)) }
  }

  pub fn visual_red(&mut self) -> u16 {
    unsafe { accessor!(visual_red -> u16, (*self.base.reply)) }
  }

  pub fn visual_green(&mut self) -> u16 {
    unsafe { accessor!(visual_green -> u16, (*self.base.reply)) }
  }

  pub fn visual_blue(&mut self) -> u16 {
    unsafe { accessor!(visual_blue -> u16, (*self.base.reply)) }
  }

}
impl_reply_cookie!(LookupColorCookie<'s>, mk_reply_xcb_lookup_color_reply_t, LookupColorReply, xcb_lookup_color_reply);

pub fn CreateCursorChecked<'r> (c : &'r Connection,
                            cid : Cursor,
                            source : Pixmap,
                            mask : Pixmap,
                            fore_red : u16,
                            fore_green : u16,
                            fore_blue : u16,
                            back_red : u16,
                            back_green : u16,
                            back_blue : u16,
                            x : u16,
                            y : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_create_cursor_checked(c.get_raw_conn(),
        cid as xcb_cursor_t, //1
        source as xcb_pixmap_t, //2
        mask as xcb_pixmap_t, //3
        fore_red as u16, //4
        fore_green as u16, //5
        fore_blue as u16, //6
        back_red as u16, //7
        back_green as u16, //8
        back_blue as u16, //9
        x as u16, //10
        y as u16); //11
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn CreateCursor<'r> (c : &'r Connection,
                     cid : Cursor,
                     source : Pixmap,
                     mask : Pixmap,
                     fore_red : u16,
                     fore_green : u16,
                     fore_blue : u16,
                     back_red : u16,
                     back_green : u16,
                     back_blue : u16,
                     x : u16,
                     y : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_create_cursor(c.get_raw_conn(),
        cid as xcb_cursor_t, //1
        source as xcb_pixmap_t, //2
        mask as xcb_pixmap_t, //3
        fore_red as u16, //4
        fore_green as u16, //5
        fore_blue as u16, //6
        back_red as u16, //7
        back_green as u16, //8
        back_blue as u16, //9
        x as u16, //10
        y as u16); //11
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn CreateGlyphCursorChecked<'r> (c : &'r Connection,
                                 cid : Cursor,
                                 source_font : Font,
                                 mask_font : Font,
                                 source_char : u16,
                                 mask_char : u16,
                                 fore_red : u16,
                                 fore_green : u16,
                                 fore_blue : u16,
                                 back_red : u16,
                                 back_green : u16,
                                 back_blue : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_create_glyph_cursor_checked(c.get_raw_conn(),
        cid as xcb_cursor_t, //1
        source_font as xcb_font_t, //2
        mask_font as xcb_font_t, //3
        source_char as u16, //4
        mask_char as u16, //5
        fore_red as u16, //6
        fore_green as u16, //7
        fore_blue as u16, //8
        back_red as u16, //9
        back_green as u16, //10
        back_blue as u16); //11
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn CreateGlyphCursor<'r> (c : &'r Connection,
                          cid : Cursor,
                          source_font : Font,
                          mask_font : Font,
                          source_char : u16,
                          mask_char : u16,
                          fore_red : u16,
                          fore_green : u16,
                          fore_blue : u16,
                          back_red : u16,
                          back_green : u16,
                          back_blue : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_create_glyph_cursor(c.get_raw_conn(),
        cid as xcb_cursor_t, //1
        source_font as xcb_font_t, //2
        mask_font as xcb_font_t, //3
        source_char as u16, //4
        mask_char as u16, //5
        fore_red as u16, //6
        fore_green as u16, //7
        fore_blue as u16, //8
        back_red as u16, //9
        back_green as u16, //10
        back_blue as u16); //11
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn FreeCursorChecked<'r> (c : &'r Connection,
                          cursor : Cursor) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_free_cursor_checked(c.get_raw_conn(),
        cursor as xcb_cursor_t); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn FreeCursor<'r> (c : &'r Connection,
                   cursor : Cursor) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_free_cursor(c.get_raw_conn(),
        cursor as xcb_cursor_t); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn RecolorCursorChecked<'r> (c : &'r Connection,
                             cursor : Cursor,
                             fore_red : u16,
                             fore_green : u16,
                             fore_blue : u16,
                             back_red : u16,
                             back_green : u16,
                             back_blue : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_recolor_cursor_checked(c.get_raw_conn(),
        cursor as xcb_cursor_t, //1
        fore_red as u16, //2
        fore_green as u16, //3
        fore_blue as u16, //4
        back_red as u16, //5
        back_green as u16, //6
        back_blue as u16); //7
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn RecolorCursor<'r> (c : &'r Connection,
                      cursor : Cursor,
                      fore_red : u16,
                      fore_green : u16,
                      fore_blue : u16,
                      back_red : u16,
                      back_green : u16,
                      back_blue : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_recolor_cursor(c.get_raw_conn(),
        cursor as xcb_cursor_t, //1
        fore_red as u16, //2
        fore_green as u16, //3
        fore_blue as u16, //4
        back_red as u16, //5
        back_green as u16, //6
        back_blue as u16); //7
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn QueryBestSize<'r> (c : &'r Connection,
                      class : u8,
                      drawable : Drawable,
                      width : u16,
                      height : u16) -> QueryBestSizeCookie<'r> {
  unsafe {
    let cookie = xcb_query_best_size(c.get_raw_conn(),
        class as u8, //1
        drawable as xcb_drawable_t, //2
        width as u16, //3
        height as u16); //4
    QueryBestSizeCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn QueryBestSizeUnchecked<'r> (c : &'r Connection,
                               class : u8,
                               drawable : Drawable,
                               width : u16,
                               height : u16) -> QueryBestSizeCookie<'r> {
  unsafe {
    let cookie = xcb_query_best_size_unchecked(c.get_raw_conn(),
        class as u8, //1
        drawable as xcb_drawable_t, //2
        width as u16, //3
        height as u16); //4
    QueryBestSizeCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl QueryBestSizeReply {
  pub fn width(&mut self) -> u16 {
    unsafe { accessor!(width -> u16, (*self.base.reply)) }
  }

  pub fn height(&mut self) -> u16 {
    unsafe { accessor!(height -> u16, (*self.base.reply)) }
  }

}
impl_reply_cookie!(QueryBestSizeCookie<'s>, mk_reply_xcb_query_best_size_reply_t, QueryBestSizeReply, xcb_query_best_size_reply);

pub fn QueryExtension<'r> (c : &'r Connection,
                       name : &str) -> QueryExtensionCookie<'r> {
  unsafe {
    let name = (name).as_bytes();
    let name_len = name.len();
    let name_ptr = name.as_ptr();
    let cookie = xcb_query_extension(c.get_raw_conn(),
        name_len as u16, //1
        name_ptr as *mut c_char); //2
    QueryExtensionCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn QueryExtensionUnchecked<'r> (c : &'r Connection,
                                name : &str) -> QueryExtensionCookie<'r> {
  unsafe {
    let name = (name).as_bytes();
    let name_len = name.len();
    let name_ptr = name.as_ptr();
    let cookie = xcb_query_extension_unchecked(c.get_raw_conn(),
        name_len as u16, //1
        name_ptr as *mut c_char); //2
    QueryExtensionCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl QueryExtensionReply {
  pub fn present(&mut self) -> u8 {
    unsafe { accessor!(present -> u8, (*self.base.reply)) }
  }

  pub fn major_opcode(&mut self) -> u8 {
    unsafe { accessor!(major_opcode -> u8, (*self.base.reply)) }
  }

  pub fn first_event(&mut self) -> u8 {
    unsafe { accessor!(first_event -> u8, (*self.base.reply)) }
  }

  pub fn first_error(&mut self) -> u8 {
    unsafe { accessor!(first_error -> u8, (*self.base.reply)) }
  }

}
impl_reply_cookie!(QueryExtensionCookie<'s>, mk_reply_xcb_query_extension_reply_t, QueryExtensionReply, xcb_query_extension_reply);

pub struct ListExtensionsReply { base:  base::Reply<xcb_list_extensions_reply_t> }
fn mk_reply_xcb_list_extensions_reply_t(reply:*mut xcb_list_extensions_reply_t) -> ListExtensionsReply { ListExtensionsReply { base : base::mk_reply(reply) } }
pub fn ListExtensions<'r> (c : &'r Connection) -> ListExtensionsCookie<'r> {
  unsafe {
    let cookie = xcb_list_extensions(c.get_raw_conn());
    ListExtensionsCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn ListExtensionsUnchecked<'r> (c : &'r Connection) -> ListExtensionsCookie<'r> {
  unsafe {
    let cookie = xcb_list_extensions_unchecked(c.get_raw_conn());
    ListExtensionsCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl ListExtensionsReply {
  pub fn names(&mut self) -> StrIterator {
    unsafe { accessor!(StrIterator, xcb_list_extensions_names_iterator, (*self.base.reply)) }
  }

}
impl_reply_cookie!(ListExtensionsCookie<'s>, mk_reply_xcb_list_extensions_reply_t, ListExtensionsReply, xcb_list_extensions_reply);

pub fn ChangeKeyboardMappingChecked<'r> (c : &'r Connection,
                                     first_keycode : Keycode,
                                     keysyms_per_keycode : u8,
                                     keysyms : &[Keysym]) -> base::VoidCookie<'r> {
  unsafe {
    let keysyms_len = keysyms.len();
    let keysyms_ptr = keysyms.as_ptr();
    let cookie = xcb_change_keyboard_mapping_checked(c.get_raw_conn(),
        keysyms_len as u8, //1
        first_keycode as xcb_keycode_t, //2
        keysyms_per_keycode as u8, //3
        keysyms_ptr as *mut xcb_keysym_t); //4
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn ChangeKeyboardMapping<'r> (c : &'r Connection,
                              first_keycode : Keycode,
                              keysyms_per_keycode : u8,
                              keysyms : &[Keysym]) -> base::VoidCookie<'r> {
  unsafe {
    let keysyms_len = keysyms.len();
    let keysyms_ptr = keysyms.as_ptr();
    let cookie = xcb_change_keyboard_mapping(c.get_raw_conn(),
        keysyms_len as u8, //1
        first_keycode as xcb_keycode_t, //2
        keysyms_per_keycode as u8, //3
        keysyms_ptr as *mut xcb_keysym_t); //4
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub struct GetKeyboardMappingReply { base:  base::Reply<xcb_get_keyboard_mapping_reply_t> }
fn mk_reply_xcb_get_keyboard_mapping_reply_t(reply:*mut xcb_get_keyboard_mapping_reply_t) -> GetKeyboardMappingReply { GetKeyboardMappingReply { base : base::mk_reply(reply) } }
pub fn GetKeyboardMapping<'r> (c : &'r Connection,
                           first_keycode : Keycode,
                           count : u8) -> GetKeyboardMappingCookie<'r> {
  unsafe {
    let cookie = xcb_get_keyboard_mapping(c.get_raw_conn(),
        first_keycode as xcb_keycode_t, //1
        count as u8); //2
    GetKeyboardMappingCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetKeyboardMappingUnchecked<'r> (c : &'r Connection,
                                    first_keycode : Keycode,
                                    count : u8) -> GetKeyboardMappingCookie<'r> {
  unsafe {
    let cookie = xcb_get_keyboard_mapping_unchecked(c.get_raw_conn(),
        first_keycode as xcb_keycode_t, //1
        count as u8); //2
    GetKeyboardMappingCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetKeyboardMappingReply {
  pub fn keysyms_per_keycode(&mut self) -> u8 {
    unsafe { accessor!(keysyms_per_keycode -> u8, (*self.base.reply)) }
  }

  pub fn keysyms(&mut self) -> Vec<Keysym> {
    unsafe { accessor!(Keysym, xcb_get_keyboard_mapping_keysyms_length, xcb_get_keyboard_mapping_keysyms, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetKeyboardMappingCookie<'s>, mk_reply_xcb_get_keyboard_mapping_reply_t, GetKeyboardMappingReply, xcb_get_keyboard_mapping_reply);

pub fn ChangeKeyboardControlChecked<'r> (c : &'r Connection,
                                     value_list : &[(u32,u32)]) -> base::VoidCookie<'r> {
  unsafe {
    let mut value_list_copy = value_list.to_vec();
    let (value_list_mask, value_list_vec) = pack_bitfield(&mut value_list_copy);
    let value_list_ptr = value_list_vec.as_ptr();
    let cookie = xcb_change_keyboard_control_checked(c.get_raw_conn(),
        value_list_mask as u32, //1
        value_list_ptr as *mut u32); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn ChangeKeyboardControl<'r> (c : &'r Connection,
                              value_list : &[(u32,u32)]) -> base::VoidCookie<'r> {
  unsafe {
    let mut value_list_copy = value_list.to_vec();
    let (value_list_mask, value_list_vec) = pack_bitfield(&mut value_list_copy);
    let value_list_ptr = value_list_vec.as_ptr();
    let cookie = xcb_change_keyboard_control(c.get_raw_conn(),
        value_list_mask as u32, //1
        value_list_ptr as *mut u32); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetKeyboardControl<'r> (c : &'r Connection) -> GetKeyboardControlCookie<'r> {
  unsafe {
    let cookie = xcb_get_keyboard_control(c.get_raw_conn());
    GetKeyboardControlCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetKeyboardControlUnchecked<'r> (c : &'r Connection) -> GetKeyboardControlCookie<'r> {
  unsafe {
    let cookie = xcb_get_keyboard_control_unchecked(c.get_raw_conn());
    GetKeyboardControlCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetKeyboardControlReply {
  pub fn global_auto_repeat(&mut self) -> u8 {
    unsafe { accessor!(global_auto_repeat -> u8, (*self.base.reply)) }
  }

  pub fn led_mask(&mut self) -> u32 {
    unsafe { accessor!(led_mask -> u32, (*self.base.reply)) }
  }

  pub fn key_click_percent(&mut self) -> u8 {
    unsafe { accessor!(key_click_percent -> u8, (*self.base.reply)) }
  }

  pub fn bell_percent(&mut self) -> u8 {
    unsafe { accessor!(bell_percent -> u8, (*self.base.reply)) }
  }

  pub fn bell_pitch(&mut self) -> u16 {
    unsafe { accessor!(bell_pitch -> u16, (*self.base.reply)) }
  }

  pub fn bell_duration(&mut self) -> u16 {
    unsafe { accessor!(bell_duration -> u16, (*self.base.reply)) }
  }

  pub fn auto_repeats(&self) -> Vec<u8> {
    unsafe { ((*self.base.reply).auto_repeats).to_vec() }
  }

}
impl_reply_cookie!(GetKeyboardControlCookie<'s>, mk_reply_xcb_get_keyboard_control_reply_t, GetKeyboardControlReply, xcb_get_keyboard_control_reply);

pub fn BellChecked<'r> (c : &'r Connection,
                    percent : i8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_bell_checked(c.get_raw_conn(),
        percent as i8); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn Bell<'r> (c : &'r Connection,
             percent : i8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_bell(c.get_raw_conn(),
        percent as i8); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn ChangePointerControlChecked<'r> (c : &'r Connection,
                                    acceleration_numerator : i16,
                                    acceleration_denominator : i16,
                                    threshold : i16,
                                    do_acceleration : u8,
                                    do_threshold : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_change_pointer_control_checked(c.get_raw_conn(),
        acceleration_numerator as i16, //1
        acceleration_denominator as i16, //2
        threshold as i16, //3
        do_acceleration as u8, //4
        do_threshold as u8); //5
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn ChangePointerControl<'r> (c : &'r Connection,
                             acceleration_numerator : i16,
                             acceleration_denominator : i16,
                             threshold : i16,
                             do_acceleration : u8,
                             do_threshold : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_change_pointer_control(c.get_raw_conn(),
        acceleration_numerator as i16, //1
        acceleration_denominator as i16, //2
        threshold as i16, //3
        do_acceleration as u8, //4
        do_threshold as u8); //5
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetPointerControl<'r> (c : &'r Connection) -> GetPointerControlCookie<'r> {
  unsafe {
    let cookie = xcb_get_pointer_control(c.get_raw_conn());
    GetPointerControlCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetPointerControlUnchecked<'r> (c : &'r Connection) -> GetPointerControlCookie<'r> {
  unsafe {
    let cookie = xcb_get_pointer_control_unchecked(c.get_raw_conn());
    GetPointerControlCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetPointerControlReply {
  pub fn acceleration_numerator(&mut self) -> u16 {
    unsafe { accessor!(acceleration_numerator -> u16, (*self.base.reply)) }
  }

  pub fn acceleration_denominator(&mut self) -> u16 {
    unsafe { accessor!(acceleration_denominator -> u16, (*self.base.reply)) }
  }

  pub fn threshold(&mut self) -> u16 {
    unsafe { accessor!(threshold -> u16, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetPointerControlCookie<'s>, mk_reply_xcb_get_pointer_control_reply_t, GetPointerControlReply, xcb_get_pointer_control_reply);

pub fn SetScreenSaverChecked<'r> (c : &'r Connection,
                              timeout : i16,
                              interval : i16,
                              prefer_blanking : u8,
                              allow_exposures : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_set_screen_saver_checked(c.get_raw_conn(),
        timeout as i16, //1
        interval as i16, //2
        prefer_blanking as u8, //3
        allow_exposures as u8); //4
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn SetScreenSaver<'r> (c : &'r Connection,
                       timeout : i16,
                       interval : i16,
                       prefer_blanking : u8,
                       allow_exposures : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_set_screen_saver(c.get_raw_conn(),
        timeout as i16, //1
        interval as i16, //2
        prefer_blanking as u8, //3
        allow_exposures as u8); //4
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetScreenSaver<'r> (c : &'r Connection) -> GetScreenSaverCookie<'r> {
  unsafe {
    let cookie = xcb_get_screen_saver(c.get_raw_conn());
    GetScreenSaverCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetScreenSaverUnchecked<'r> (c : &'r Connection) -> GetScreenSaverCookie<'r> {
  unsafe {
    let cookie = xcb_get_screen_saver_unchecked(c.get_raw_conn());
    GetScreenSaverCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetScreenSaverReply {
  pub fn timeout(&mut self) -> u16 {
    unsafe { accessor!(timeout -> u16, (*self.base.reply)) }
  }

  pub fn interval(&mut self) -> u16 {
    unsafe { accessor!(interval -> u16, (*self.base.reply)) }
  }

  pub fn prefer_blanking(&mut self) -> u8 {
    unsafe { accessor!(prefer_blanking -> u8, (*self.base.reply)) }
  }

  pub fn allow_exposures(&mut self) -> u8 {
    unsafe { accessor!(allow_exposures -> u8, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetScreenSaverCookie<'s>, mk_reply_xcb_get_screen_saver_reply_t, GetScreenSaverReply, xcb_get_screen_saver_reply);

pub fn ChangeHostsChecked<'r> (c : &'r Connection,
                           mode : u8,
                           family : u8,
                           address : &[u8]) -> base::VoidCookie<'r> {
  unsafe {
    let address_len = address.len();
    let address_ptr = address.as_ptr();
    let cookie = xcb_change_hosts_checked(c.get_raw_conn(),
        mode as u8, //1
        family as u8, //2
        address_len as u16, //3
        address_ptr as *mut u8); //4
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn ChangeHosts<'r> (c : &'r Connection,
                    mode : u8,
                    family : u8,
                    address : &[u8]) -> base::VoidCookie<'r> {
  unsafe {
    let address_len = address.len();
    let address_ptr = address.as_ptr();
    let cookie = xcb_change_hosts(c.get_raw_conn(),
        mode as u8, //1
        family as u8, //2
        address_len as u16, //3
        address_ptr as *mut u8); //4
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub struct Host {pub base : base::Struct<xcb_host_t> }


impl Host {
  pub fn family(&mut self) -> u8 {
    unsafe { accessor!(family -> u8, self.base.strct) }
  }

  pub fn address(&mut self) -> Vec<u8> {
    unsafe { accessor!(u8, xcb_host_address_length, xcb_host_address, self.base.strct) }
  }

}

impl Iterator for HostIterator {
    type Item = Host;
    fn next(&mut self) -> Option<Host> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_host_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_host_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct ListHostsReply { base:  base::Reply<xcb_list_hosts_reply_t> }
fn mk_reply_xcb_list_hosts_reply_t(reply:*mut xcb_list_hosts_reply_t) -> ListHostsReply { ListHostsReply { base : base::mk_reply(reply) } }
pub fn ListHosts<'r> (c : &'r Connection) -> ListHostsCookie<'r> {
  unsafe {
    let cookie = xcb_list_hosts(c.get_raw_conn());
    ListHostsCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn ListHostsUnchecked<'r> (c : &'r Connection) -> ListHostsCookie<'r> {
  unsafe {
    let cookie = xcb_list_hosts_unchecked(c.get_raw_conn());
    ListHostsCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl ListHostsReply {
  pub fn mode(&mut self) -> u8 {
    unsafe { accessor!(mode -> u8, (*self.base.reply)) }
  }

  pub fn hosts(&mut self) -> HostIterator {
    unsafe { accessor!(HostIterator, xcb_list_hosts_hosts_iterator, (*self.base.reply)) }
  }

}
impl_reply_cookie!(ListHostsCookie<'s>, mk_reply_xcb_list_hosts_reply_t, ListHostsReply, xcb_list_hosts_reply);

pub fn SetAccessControlChecked<'r> (c : &'r Connection,
                                mode : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_set_access_control_checked(c.get_raw_conn(),
        mode as u8); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn SetAccessControl<'r> (c : &'r Connection,
                         mode : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_set_access_control(c.get_raw_conn(),
        mode as u8); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn SetCloseDownModeChecked<'r> (c : &'r Connection,
                                mode : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_set_close_down_mode_checked(c.get_raw_conn(),
        mode as u8); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn SetCloseDownMode<'r> (c : &'r Connection,
                         mode : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_set_close_down_mode(c.get_raw_conn(),
        mode as u8); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn KillClientChecked<'r> (c : &'r Connection,
                          resource : u32) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_kill_client_checked(c.get_raw_conn(),
        resource as u32); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn KillClient<'r> (c : &'r Connection,
                   resource : u32) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_kill_client(c.get_raw_conn(),
        resource as u32); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn RotatePropertiesChecked<'r> (c : &'r Connection,
                                window : Window,
                                delta : i16,
                                atoms : &[Atom]) -> base::VoidCookie<'r> {
  unsafe {
    let atoms_len = atoms.len();
    let atoms_ptr = atoms.as_ptr();
    let cookie = xcb_rotate_properties_checked(c.get_raw_conn(),
        window as xcb_window_t, //1
        atoms_len as u16, //2
        delta as i16, //3
        atoms_ptr as *mut xcb_atom_t); //4
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn RotateProperties<'r> (c : &'r Connection,
                         window : Window,
                         delta : i16,
                         atoms : &[Atom]) -> base::VoidCookie<'r> {
  unsafe {
    let atoms_len = atoms.len();
    let atoms_ptr = atoms.as_ptr();
    let cookie = xcb_rotate_properties(c.get_raw_conn(),
        window as xcb_window_t, //1
        atoms_len as u16, //2
        delta as i16, //3
        atoms_ptr as *mut xcb_atom_t); //4
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn ForceScreenSaverChecked<'r> (c : &'r Connection,
                                mode : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_force_screen_saver_checked(c.get_raw_conn(),
        mode as u8); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn ForceScreenSaver<'r> (c : &'r Connection,
                         mode : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_force_screen_saver(c.get_raw_conn(),
        mode as u8); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn SetPointerMapping<'r> (c : &'r Connection,
                          map : &[u8]) -> SetPointerMappingCookie<'r> {
  unsafe {
    let map_len = map.len();
    let map_ptr = map.as_ptr();
    let cookie = xcb_set_pointer_mapping(c.get_raw_conn(),
        map_len as u8, //1
        map_ptr as *mut u8); //2
    SetPointerMappingCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn SetPointerMappingUnchecked<'r> (c : &'r Connection,
                                   map : &[u8]) -> SetPointerMappingCookie<'r> {
  unsafe {
    let map_len = map.len();
    let map_ptr = map.as_ptr();
    let cookie = xcb_set_pointer_mapping_unchecked(c.get_raw_conn(),
        map_len as u8, //1
        map_ptr as *mut u8); //2
    SetPointerMappingCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl SetPointerMappingReply {
  pub fn status(&mut self) -> u8 {
    unsafe { accessor!(status -> u8, (*self.base.reply)) }
  }

}
impl_reply_cookie!(SetPointerMappingCookie<'s>, mk_reply_xcb_set_pointer_mapping_reply_t, SetPointerMappingReply, xcb_set_pointer_mapping_reply);

pub struct GetPointerMappingReply { base:  base::Reply<xcb_get_pointer_mapping_reply_t> }
fn mk_reply_xcb_get_pointer_mapping_reply_t(reply:*mut xcb_get_pointer_mapping_reply_t) -> GetPointerMappingReply { GetPointerMappingReply { base : base::mk_reply(reply) } }
pub fn GetPointerMapping<'r> (c : &'r Connection) -> GetPointerMappingCookie<'r> {
  unsafe {
    let cookie = xcb_get_pointer_mapping(c.get_raw_conn());
    GetPointerMappingCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetPointerMappingUnchecked<'r> (c : &'r Connection) -> GetPointerMappingCookie<'r> {
  unsafe {
    let cookie = xcb_get_pointer_mapping_unchecked(c.get_raw_conn());
    GetPointerMappingCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetPointerMappingReply {
  pub fn map(&mut self) -> Vec<u8> {
    unsafe { accessor!(u8, xcb_get_pointer_mapping_map_length, xcb_get_pointer_mapping_map, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetPointerMappingCookie<'s>, mk_reply_xcb_get_pointer_mapping_reply_t, GetPointerMappingReply, xcb_get_pointer_mapping_reply);

pub fn SetModifierMapping<'r> (c : &'r Connection,
                           keycodes : &[Keycode]) -> SetModifierMappingCookie<'r> {
  unsafe {
    let keycodes_len = keycodes.len();
    let keycodes_ptr = keycodes.as_ptr();
    let cookie = xcb_set_modifier_mapping(c.get_raw_conn(),
        keycodes_len as u8, //1
        keycodes_ptr as *mut xcb_keycode_t); //2
    SetModifierMappingCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn SetModifierMappingUnchecked<'r> (c : &'r Connection,
                                    keycodes : &[Keycode]) -> SetModifierMappingCookie<'r> {
  unsafe {
    let keycodes_len = keycodes.len();
    let keycodes_ptr = keycodes.as_ptr();
    let cookie = xcb_set_modifier_mapping_unchecked(c.get_raw_conn(),
        keycodes_len as u8, //1
        keycodes_ptr as *mut xcb_keycode_t); //2
    SetModifierMappingCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl SetModifierMappingReply {
  pub fn status(&mut self) -> u8 {
    unsafe { accessor!(status -> u8, (*self.base.reply)) }
  }

}
impl_reply_cookie!(SetModifierMappingCookie<'s>, mk_reply_xcb_set_modifier_mapping_reply_t, SetModifierMappingReply, xcb_set_modifier_mapping_reply);

pub struct GetModifierMappingReply { base:  base::Reply<xcb_get_modifier_mapping_reply_t> }
fn mk_reply_xcb_get_modifier_mapping_reply_t(reply:*mut xcb_get_modifier_mapping_reply_t) -> GetModifierMappingReply { GetModifierMappingReply { base : base::mk_reply(reply) } }
pub fn GetModifierMapping<'r> (c : &'r Connection) -> GetModifierMappingCookie<'r> {
  unsafe {
    let cookie = xcb_get_modifier_mapping(c.get_raw_conn());
    GetModifierMappingCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetModifierMappingUnchecked<'r> (c : &'r Connection) -> GetModifierMappingCookie<'r> {
  unsafe {
    let cookie = xcb_get_modifier_mapping_unchecked(c.get_raw_conn());
    GetModifierMappingCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetModifierMappingReply {
  pub fn keycodes(&mut self) -> Vec<Keycode> {
    unsafe { accessor!(Keycode, xcb_get_modifier_mapping_keycodes_length, xcb_get_modifier_mapping_keycodes, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetModifierMappingCookie<'s>, mk_reply_xcb_get_modifier_mapping_reply_t, GetModifierMappingReply, xcb_get_modifier_mapping_reply);

pub fn NoOperationChecked<'r> (c : &'r Connection) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_no_operation_checked(c.get_raw_conn());
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn NoOperation<'r> (c : &'r Connection) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_no_operation(c.get_raw_conn());
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

