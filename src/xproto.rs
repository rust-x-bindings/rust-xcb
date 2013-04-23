/*
 * This file generated automatically from xproto.xml by r_client.py.
 * Edit at your peril.
 */

//Make the compiler quiet
#[allow(unused_imports)];
#[allow(unused_unsafe)];
use core::libc::*;
use ll::base::*;
use base;
use ll::xproto::*;
use core::option::Option;
use core::iterator::Iterator;

pub type Char2b = base::Struct<char2b>;

pub type Char2bIterator = char2b_iterator;

pub type WindowIterator = window_iterator;

pub type PixmapIterator = pixmap_iterator;

pub type CursorIterator = cursor_iterator;

pub type FontIterator = font_iterator;

pub type GcontextIterator = gcontext_iterator;

pub type ColormapIterator = colormap_iterator;

pub type AtomIterator = atom_iterator;

pub type DrawableIterator = drawable_iterator;

pub type FontableIterator = fontable_iterator;

pub type VisualidIterator = visualid_iterator;

pub type TimestampIterator = timestamp_iterator;

pub type KeysymIterator = keysym_iterator;

pub type KeycodeIterator = keycode_iterator;

pub type ButtonIterator = button_iterator;

pub type PointIterator = point_iterator;

pub type RectangleIterator = rectangle_iterator;

pub type ArcIterator = arc_iterator;

pub type FormatIterator = format_iterator;

pub type VisualtypeIterator = visualtype_iterator;

pub type DepthIterator = depth_iterator;

pub type ScreenIterator = screen_iterator;

pub type SetupRequestIterator = setup_request_iterator;

pub type SetupFailedIterator = setup_failed_iterator;

pub type SetupAuthenticateIterator = setup_authenticate_iterator;

pub type SetupIterator = setup_iterator;

/** Opcode for xcb_key_press. */
pub static XCB_KEY_PRESS : u8 = 2;
pub type KeyPressEvent = base::Event<key_press_event>;
/** Opcode for xcb_key_release. */
pub static XCB_KEY_RELEASE : u8 = 3;
pub type KeyReleaseEvent = base::Event<key_release_event>;
/** Opcode for xcb_button_press. */
pub static XCB_BUTTON_PRESS : u8 = 4;
pub type ButtonPressEvent = base::Event<button_press_event>;
/** Opcode for xcb_button_release. */
pub static XCB_BUTTON_RELEASE : u8 = 5;
pub type ButtonReleaseEvent = base::Event<button_release_event>;
/** Opcode for xcb_motion_notify. */
pub static XCB_MOTION_NOTIFY : u8 = 6;
pub type MotionNotifyEvent = base::Event<motion_notify_event>;
/** Opcode for xcb_enter_notify. */
pub static XCB_ENTER_NOTIFY : u8 = 7;
pub type EnterNotifyEvent = base::Event<enter_notify_event>;
/** Opcode for xcb_leave_notify. */
pub static XCB_LEAVE_NOTIFY : u8 = 8;
pub type LeaveNotifyEvent = base::Event<leave_notify_event>;
/** Opcode for xcb_focus_in. */
pub static XCB_FOCUS_IN : u8 = 9;
pub type FocusInEvent = base::Event<focus_in_event>;
/** Opcode for xcb_focus_out. */
pub static XCB_FOCUS_OUT : u8 = 10;
pub type FocusOutEvent = base::Event<focus_out_event>;
/** Opcode for xcb_keymap_notify. */
pub static XCB_KEYMAP_NOTIFY : u8 = 11;
pub type KeymapNotifyEvent = base::Event<keymap_notify_event>;
/** Opcode for xcb_expose. */
pub static XCB_EXPOSE : u8 = 12;
pub type ExposeEvent = base::Event<expose_event>;
/** Opcode for xcb_graphics_exposure. */
pub static XCB_GRAPHICS_EXPOSURE : u8 = 13;
pub type GraphicsExposureEvent = base::Event<graphics_exposure_event>;
/** Opcode for xcb_no_exposure. */
pub static XCB_NO_EXPOSURE : u8 = 14;
pub type NoExposureEvent = base::Event<no_exposure_event>;
/** Opcode for xcb_visibility_notify. */
pub static XCB_VISIBILITY_NOTIFY : u8 = 15;
pub type VisibilityNotifyEvent = base::Event<visibility_notify_event>;
/** Opcode for xcb_create_notify. */
pub static XCB_CREATE_NOTIFY : u8 = 16;
pub type CreateNotifyEvent = base::Event<create_notify_event>;
/** Opcode for xcb_destroy_notify. */
pub static XCB_DESTROY_NOTIFY : u8 = 17;
pub type DestroyNotifyEvent = base::Event<destroy_notify_event>;
/** Opcode for xcb_unmap_notify. */
pub static XCB_UNMAP_NOTIFY : u8 = 18;
pub type UnmapNotifyEvent = base::Event<unmap_notify_event>;
/** Opcode for xcb_map_notify. */
pub static XCB_MAP_NOTIFY : u8 = 19;
pub type MapNotifyEvent = base::Event<map_notify_event>;
/** Opcode for xcb_map_request. */
pub static XCB_MAP_REQUEST : u8 = 20;
pub type MapRequestEvent = base::Event<map_request_event>;
/** Opcode for xcb_reparent_notify. */
pub static XCB_REPARENT_NOTIFY : u8 = 21;
pub type ReparentNotifyEvent = base::Event<reparent_notify_event>;
/** Opcode for xcb_configure_notify. */
pub static XCB_CONFIGURE_NOTIFY : u8 = 22;
pub type ConfigureNotifyEvent = base::Event<configure_notify_event>;
/** Opcode for xcb_configure_request. */
pub static XCB_CONFIGURE_REQUEST : u8 = 23;
pub type ConfigureRequestEvent = base::Event<configure_request_event>;
/** Opcode for xcb_gravity_notify. */
pub static XCB_GRAVITY_NOTIFY : u8 = 24;
pub type GravityNotifyEvent = base::Event<gravity_notify_event>;
/** Opcode for xcb_resize_request. */
pub static XCB_RESIZE_REQUEST : u8 = 25;
pub type ResizeRequestEvent = base::Event<resize_request_event>;
/** Opcode for xcb_circulate_notify. */
pub static XCB_CIRCULATE_NOTIFY : u8 = 26;
pub type CirculateNotifyEvent = base::Event<circulate_notify_event>;
/** Opcode for xcb_circulate_request. */
pub static XCB_CIRCULATE_REQUEST : u8 = 27;
pub type CirculateRequestEvent = base::Event<circulate_request_event>;
/** Opcode for xcb_property_notify. */
pub static XCB_PROPERTY_NOTIFY : u8 = 28;
pub type PropertyNotifyEvent = base::Event<property_notify_event>;
/** Opcode for xcb_selection_clear. */
pub static XCB_SELECTION_CLEAR : u8 = 29;
pub type SelectionClearEvent = base::Event<selection_clear_event>;
/** Opcode for xcb_selection_request. */
pub static XCB_SELECTION_REQUEST : u8 = 30;
pub type SelectionRequestEvent = base::Event<selection_request_event>;
/** Opcode for xcb_selection_notify. */
pub static XCB_SELECTION_NOTIFY : u8 = 31;
pub type SelectionNotifyEvent = base::Event<selection_notify_event>;
/** Opcode for xcb_colormap_notify. */
pub static XCB_COLORMAP_NOTIFY : u8 = 32;
pub type ColormapNotifyEvent = base::Event<colormap_notify_event>;
pub type ClientMessageData = base::Struct<client_message_data>;
pub type ClientMessageDataIterator = client_message_data_iterator;

/** Opcode for xcb_client_message. */
pub static XCB_CLIENT_MESSAGE : u8 = 33;
pub type ClientMessageEvent = base::Event<client_message_event>;
/** Opcode for xcb_mapping_notify. */
pub static XCB_MAPPING_NOTIFY : u8 = 34;
pub type MappingNotifyEvent = base::Event<mapping_notify_event>;
/** Opcode for xcb_request. */
pub static XCB_REQUEST : u8 = 1;
pub type RequestError = base::Error<request_error>;
/** Opcode for xcb_value. */
pub static XCB_VALUE : u8 = 2;
pub type ValueError = base::Error<value_error>;
/** Opcode for xcb_window. */
pub static XCB_WINDOW : u8 = 3;
pub type WindowError = base::Error<window_error>;
/** Opcode for xcb_pixmap. */
pub static XCB_PIXMAP : u8 = 4;
pub type PixmapError = base::Error<pixmap_error>;
/** Opcode for xcb_atom. */
pub static XCB_ATOM : u8 = 5;
pub type AtomError = base::Error<atom_error>;
/** Opcode for xcb_cursor. */
pub static XCB_CURSOR : u8 = 6;
pub type CursorError = base::Error<cursor_error>;
/** Opcode for xcb_font. */
pub static XCB_FONT : u8 = 7;
pub type FontError = base::Error<font_error>;
/** Opcode for xcb_match. */
pub static XCB_MATCH : u8 = 8;
pub type MatchError = base::Error<match_error>;
/** Opcode for xcb_drawable. */
pub static XCB_DRAWABLE : u8 = 9;
pub type DrawableError = base::Error<drawable_error>;
/** Opcode for xcb_access. */
pub static XCB_ACCESS : u8 = 10;
pub type AccessError = base::Error<access_error>;
/** Opcode for xcb_alloc. */
pub static XCB_ALLOC : u8 = 11;
pub type AllocError = base::Error<alloc_error>;
/** Opcode for xcb_colormap. */
pub static XCB_COLORMAP : u8 = 12;
pub type ColormapError = base::Error<colormap_error>;
/** Opcode for xcb_g_context. */
pub static XCB_G_CONTEXT : u8 = 13;
pub type GContextError = base::Error<g_context_error>;
/** Opcode for xcb_id_choice. */
pub static XCB_ID_CHOICE : u8 = 14;
pub type IdChoiceError = base::Error<id_choice_error>;
/** Opcode for xcb_name. */
pub static XCB_NAME : u8 = 15;
pub type NameError = base::Error<name_error>;
/** Opcode for xcb_length. */
pub static XCB_LENGTH : u8 = 16;
pub type LengthError = base::Error<length_error>;
/** Opcode for xcb_implementation. */
pub static XCB_IMPLEMENTATION : u8 = 17;
pub type ImplementationError = base::Error<implementation_error>;
/** Opcode for xcb_create_window. */
pub static XCB_CREATE_WINDOW : u8 = 1;
/** Opcode for xcb_change_window_attributes. */
pub static XCB_CHANGE_WINDOW_ATTRIBUTES : u8 = 2;
pub type GetWindowAttributesCookie = base::Cookie<get_window_attributes_cookie>;

/** Opcode for xcb_get_window_attributes. */
pub static XCB_GET_WINDOW_ATTRIBUTES : u8 = 3;
pub type GetWindowAttributesReply = base::Reply<get_window_attributes_reply>;
/** Opcode for xcb_destroy_window. */
pub static XCB_DESTROY_WINDOW : u8 = 4;
/** Opcode for xcb_destroy_subwindows. */
pub static XCB_DESTROY_SUBWINDOWS : u8 = 5;
/** Opcode for xcb_change_save_set. */
pub static XCB_CHANGE_SAVE_SET : u8 = 6;
/** Opcode for xcb_reparent_window. */
pub static XCB_REPARENT_WINDOW : u8 = 7;
/** Opcode for xcb_map_window. */
pub static XCB_MAP_WINDOW : u8 = 8;
/** Opcode for xcb_map_subwindows. */
pub static XCB_MAP_SUBWINDOWS : u8 = 9;
/** Opcode for xcb_unmap_window. */
pub static XCB_UNMAP_WINDOW : u8 = 10;
/** Opcode for xcb_unmap_subwindows. */
pub static XCB_UNMAP_SUBWINDOWS : u8 = 11;
/** Opcode for xcb_configure_window. */
pub static XCB_CONFIGURE_WINDOW : u8 = 12;
/** Opcode for xcb_circulate_window. */
pub static XCB_CIRCULATE_WINDOW : u8 = 13;
pub type GetGeometryCookie = base::Cookie<get_geometry_cookie>;

/** Opcode for xcb_get_geometry. */
pub static XCB_GET_GEOMETRY : u8 = 14;
pub type GetGeometryReply = base::Reply<get_geometry_reply>;
pub type QueryTreeCookie = base::Cookie<query_tree_cookie>;

/** Opcode for xcb_query_tree. */
pub static XCB_QUERY_TREE : u8 = 15;
pub type InternAtomCookie = base::Cookie<intern_atom_cookie>;

/** Opcode for xcb_intern_atom. */
pub static XCB_INTERN_ATOM : u8 = 16;
pub type InternAtomReply = base::Reply<intern_atom_reply>;
pub type GetAtomNameCookie = base::Cookie<get_atom_name_cookie>;

/** Opcode for xcb_get_atom_name. */
pub static XCB_GET_ATOM_NAME : u8 = 17;
/** Opcode for xcb_change_property. */
pub static XCB_CHANGE_PROPERTY : u8 = 18;
/** Opcode for xcb_delete_property. */
pub static XCB_DELETE_PROPERTY : u8 = 19;
pub type GetPropertyCookie = base::Cookie<get_property_cookie>;

/** Opcode for xcb_get_property. */
pub static XCB_GET_PROPERTY : u8 = 20;
pub type ListPropertiesCookie = base::Cookie<list_properties_cookie>;

/** Opcode for xcb_list_properties. */
pub static XCB_LIST_PROPERTIES : u8 = 21;
/** Opcode for xcb_set_selection_owner. */
pub static XCB_SET_SELECTION_OWNER : u8 = 22;
pub type GetSelectionOwnerCookie = base::Cookie<get_selection_owner_cookie>;

/** Opcode for xcb_get_selection_owner. */
pub static XCB_GET_SELECTION_OWNER : u8 = 23;
pub type GetSelectionOwnerReply = base::Reply<get_selection_owner_reply>;
/** Opcode for xcb_convert_selection. */
pub static XCB_CONVERT_SELECTION : u8 = 24;
/** Opcode for xcb_send_event. */
pub static XCB_SEND_EVENT : u8 = 25;
pub type GrabPointerCookie = base::Cookie<grab_pointer_cookie>;

/** Opcode for xcb_grab_pointer. */
pub static XCB_GRAB_POINTER : u8 = 26;
pub type GrabPointerReply = base::Reply<grab_pointer_reply>;
/** Opcode for xcb_ungrab_pointer. */
pub static XCB_UNGRAB_POINTER : u8 = 27;
/** Opcode for xcb_grab_button. */
pub static XCB_GRAB_BUTTON : u8 = 28;
/** Opcode for xcb_ungrab_button. */
pub static XCB_UNGRAB_BUTTON : u8 = 29;
/** Opcode for xcb_change_active_pointer_grab. */
pub static XCB_CHANGE_ACTIVE_POINTER_GRAB : u8 = 30;
pub type GrabKeyboardCookie = base::Cookie<grab_keyboard_cookie>;

/** Opcode for xcb_grab_keyboard. */
pub static XCB_GRAB_KEYBOARD : u8 = 31;
pub type GrabKeyboardReply = base::Reply<grab_keyboard_reply>;
/** Opcode for xcb_ungrab_keyboard. */
pub static XCB_UNGRAB_KEYBOARD : u8 = 32;
/** Opcode for xcb_grab_key. */
pub static XCB_GRAB_KEY : u8 = 33;
/** Opcode for xcb_ungrab_key. */
pub static XCB_UNGRAB_KEY : u8 = 34;
/** Opcode for xcb_allow_events. */
pub static XCB_ALLOW_EVENTS : u8 = 35;
/** Opcode for xcb_grab_server. */
pub static XCB_GRAB_SERVER : u8 = 36;
/** Opcode for xcb_ungrab_server. */
pub static XCB_UNGRAB_SERVER : u8 = 37;
pub type QueryPointerCookie = base::Cookie<query_pointer_cookie>;

/** Opcode for xcb_query_pointer. */
pub static XCB_QUERY_POINTER : u8 = 38;
pub type QueryPointerReply = base::Reply<query_pointer_reply>;
pub type TimecoordIterator = timecoord_iterator;

pub type GetMotionEventsCookie = base::Cookie<get_motion_events_cookie>;

/** Opcode for xcb_get_motion_events. */
pub static XCB_GET_MOTION_EVENTS : u8 = 39;
pub type TranslateCoordinatesCookie = base::Cookie<translate_coordinates_cookie>;

/** Opcode for xcb_translate_coordinates. */
pub static XCB_TRANSLATE_COORDINATES : u8 = 40;
pub type TranslateCoordinatesReply = base::Reply<translate_coordinates_reply>;
/** Opcode for xcb_warp_pointer. */
pub static XCB_WARP_POINTER : u8 = 41;
/** Opcode for xcb_set_input_focus. */
pub static XCB_SET_INPUT_FOCUS : u8 = 42;
pub type GetInputFocusCookie = base::Cookie<get_input_focus_cookie>;

/** Opcode for xcb_get_input_focus. */
pub static XCB_GET_INPUT_FOCUS : u8 = 43;
pub type GetInputFocusReply = base::Reply<get_input_focus_reply>;
pub type QueryKeymapCookie = base::Cookie<query_keymap_cookie>;

/** Opcode for xcb_query_keymap. */
pub static XCB_QUERY_KEYMAP : u8 = 44;
pub type QueryKeymapReply = base::Reply<query_keymap_reply>;
/** Opcode for xcb_open_font. */
pub static XCB_OPEN_FONT : u8 = 45;
/** Opcode for xcb_close_font. */
pub static XCB_CLOSE_FONT : u8 = 46;
pub type FontpropIterator = fontprop_iterator;

pub type CharinfoIterator = charinfo_iterator;

pub type QueryFontCookie = base::Cookie<query_font_cookie>;

/** Opcode for xcb_query_font. */
pub static XCB_QUERY_FONT : u8 = 47;
pub type QueryTextExtentsCookie = base::Cookie<query_text_extents_cookie>;

/** Opcode for xcb_query_text_extents. */
pub static XCB_QUERY_TEXT_EXTENTS : u8 = 48;
pub type QueryTextExtentsReply = base::Reply<query_text_extents_reply>;
pub type StrIterator = str_iterator;

pub type ListFontsCookie = base::Cookie<list_fonts_cookie>;

/** Opcode for xcb_list_fonts. */
pub static XCB_LIST_FONTS : u8 = 49;
pub type ListFontsReply = base::Reply<list_fonts_reply>;
pub type ListFontsWithInfoCookie = base::Cookie<list_fonts_with_info_cookie>;

/** Opcode for xcb_list_fonts_with_info. */
pub static XCB_LIST_FONTS_WITH_INFO : u8 = 50;
pub type ListFontsWithInfoReply = base::Reply<list_fonts_with_info_reply>;
/** Opcode for xcb_set_font_path. */
pub static XCB_SET_FONT_PATH : u8 = 51;
pub type GetFontPathCookie = base::Cookie<get_font_path_cookie>;

/** Opcode for xcb_get_font_path. */
pub static XCB_GET_FONT_PATH : u8 = 52;
/** Opcode for xcb_create_pixmap. */
pub static XCB_CREATE_PIXMAP : u8 = 53;
/** Opcode for xcb_free_pixmap. */
pub static XCB_FREE_PIXMAP : u8 = 54;
/** Opcode for xcb_create_gc. */
pub static XCB_CREATE_GC : u8 = 55;
/** Opcode for xcb_change_gc. */
pub static XCB_CHANGE_GC : u8 = 56;
/** Opcode for xcb_copy_gc. */
pub static XCB_COPY_GC : u8 = 57;
/** Opcode for xcb_set_dashes. */
pub static XCB_SET_DASHES : u8 = 58;
/** Opcode for xcb_set_clip_rectangles. */
pub static XCB_SET_CLIP_RECTANGLES : u8 = 59;
/** Opcode for xcb_free_gc. */
pub static XCB_FREE_GC : u8 = 60;
/** Opcode for xcb_clear_area. */
pub static XCB_CLEAR_AREA : u8 = 61;
/** Opcode for xcb_copy_area. */
pub static XCB_COPY_AREA : u8 = 62;
/** Opcode for xcb_copy_plane. */
pub static XCB_COPY_PLANE : u8 = 63;
/** Opcode for xcb_poly_point. */
pub static XCB_POLY_POINT : u8 = 64;
/** Opcode for xcb_poly_line. */
pub static XCB_POLY_LINE : u8 = 65;
pub type SegmentIterator = segment_iterator;

/** Opcode for xcb_poly_segment. */
pub static XCB_POLY_SEGMENT : u8 = 66;
/** Opcode for xcb_poly_rectangle. */
pub static XCB_POLY_RECTANGLE : u8 = 67;
/** Opcode for xcb_poly_arc. */
pub static XCB_POLY_ARC : u8 = 68;
/** Opcode for xcb_fill_poly. */
pub static XCB_FILL_POLY : u8 = 69;
/** Opcode for xcb_poly_fill_rectangle. */
pub static XCB_POLY_FILL_RECTANGLE : u8 = 70;
/** Opcode for xcb_poly_fill_arc. */
pub static XCB_POLY_FILL_ARC : u8 = 71;
/** Opcode for xcb_put_image. */
pub static XCB_PUT_IMAGE : u8 = 72;
pub type GetImageCookie = base::Cookie<get_image_cookie>;

/** Opcode for xcb_get_image. */
pub static XCB_GET_IMAGE : u8 = 73;
/** Opcode for xcb_poly_text_8. */
pub static XCB_POLY_TEXT_8 : u8 = 74;
/** Opcode for xcb_poly_text_16. */
pub static XCB_POLY_TEXT_16 : u8 = 75;
/** Opcode for xcb_image_text_8. */
pub static XCB_IMAGE_TEXT_8 : u8 = 76;
/** Opcode for xcb_image_text_16. */
pub static XCB_IMAGE_TEXT_16 : u8 = 77;
/** Opcode for xcb_create_colormap. */
pub static XCB_CREATE_COLORMAP : u8 = 78;
/** Opcode for xcb_free_colormap. */
pub static XCB_FREE_COLORMAP : u8 = 79;
/** Opcode for xcb_copy_colormap_and_free. */
pub static XCB_COPY_COLORMAP_AND_FREE : u8 = 80;
/** Opcode for xcb_install_colormap. */
pub static XCB_INSTALL_COLORMAP : u8 = 81;
/** Opcode for xcb_uninstall_colormap. */
pub static XCB_UNINSTALL_COLORMAP : u8 = 82;
pub type ListInstalledColormapsCookie = base::Cookie<list_installed_colormaps_cookie>;

/** Opcode for xcb_list_installed_colormaps. */
pub static XCB_LIST_INSTALLED_COLORMAPS : u8 = 83;
pub type AllocColorCookie = base::Cookie<alloc_color_cookie>;

/** Opcode for xcb_alloc_color. */
pub static XCB_ALLOC_COLOR : u8 = 84;
pub type AllocColorReply = base::Reply<alloc_color_reply>;
pub type AllocNamedColorCookie = base::Cookie<alloc_named_color_cookie>;

/** Opcode for xcb_alloc_named_color. */
pub static XCB_ALLOC_NAMED_COLOR : u8 = 85;
pub type AllocNamedColorReply = base::Reply<alloc_named_color_reply>;
pub type AllocColorCellsCookie = base::Cookie<alloc_color_cells_cookie>;

/** Opcode for xcb_alloc_color_cells. */
pub static XCB_ALLOC_COLOR_CELLS : u8 = 86;
pub type AllocColorPlanesCookie = base::Cookie<alloc_color_planes_cookie>;

/** Opcode for xcb_alloc_color_planes. */
pub static XCB_ALLOC_COLOR_PLANES : u8 = 87;
/** Opcode for xcb_free_colors. */
pub static XCB_FREE_COLORS : u8 = 88;
pub type ColoritemIterator = coloritem_iterator;

/** Opcode for xcb_store_colors. */
pub static XCB_STORE_COLORS : u8 = 89;
/** Opcode for xcb_store_named_color. */
pub static XCB_STORE_NAMED_COLOR : u8 = 90;
pub type RgbIterator = rgb_iterator;

pub type QueryColorsCookie = base::Cookie<query_colors_cookie>;

/** Opcode for xcb_query_colors. */
pub static XCB_QUERY_COLORS : u8 = 91;
pub type QueryColorsReply = base::Reply<query_colors_reply>;
pub type LookupColorCookie = base::Cookie<lookup_color_cookie>;

/** Opcode for xcb_lookup_color. */
pub static XCB_LOOKUP_COLOR : u8 = 92;
pub type LookupColorReply = base::Reply<lookup_color_reply>;
/** Opcode for xcb_create_cursor. */
pub static XCB_CREATE_CURSOR : u8 = 93;
/** Opcode for xcb_create_glyph_cursor. */
pub static XCB_CREATE_GLYPH_CURSOR : u8 = 94;
/** Opcode for xcb_free_cursor. */
pub static XCB_FREE_CURSOR : u8 = 95;
/** Opcode for xcb_recolor_cursor. */
pub static XCB_RECOLOR_CURSOR : u8 = 96;
pub type QueryBestSizeCookie = base::Cookie<query_best_size_cookie>;

/** Opcode for xcb_query_best_size. */
pub static XCB_QUERY_BEST_SIZE : u8 = 97;
pub type QueryBestSizeReply = base::Reply<query_best_size_reply>;
pub type QueryExtensionCookie = base::Cookie<query_extension_cookie>;

/** Opcode for xcb_query_extension. */
pub static XCB_QUERY_EXTENSION : u8 = 98;
pub type QueryExtensionReply = base::Reply<query_extension_reply>;
pub type ListExtensionsCookie = base::Cookie<list_extensions_cookie>;

/** Opcode for xcb_list_extensions. */
pub static XCB_LIST_EXTENSIONS : u8 = 99;
/** Opcode for xcb_change_keyboard_mapping. */
pub static XCB_CHANGE_KEYBOARD_MAPPING : u8 = 100;
pub type GetKeyboardMappingCookie = base::Cookie<get_keyboard_mapping_cookie>;

/** Opcode for xcb_get_keyboard_mapping. */
pub static XCB_GET_KEYBOARD_MAPPING : u8 = 101;
/** Opcode for xcb_change_keyboard_control. */
pub static XCB_CHANGE_KEYBOARD_CONTROL : u8 = 102;
pub type GetKeyboardControlCookie = base::Cookie<get_keyboard_control_cookie>;

/** Opcode for xcb_get_keyboard_control. */
pub static XCB_GET_KEYBOARD_CONTROL : u8 = 103;
pub type GetKeyboardControlReply = base::Reply<get_keyboard_control_reply>;
/** Opcode for xcb_bell. */
pub static XCB_BELL : u8 = 104;
/** Opcode for xcb_change_pointer_control. */
pub static XCB_CHANGE_POINTER_CONTROL : u8 = 105;
pub type GetPointerControlCookie = base::Cookie<get_pointer_control_cookie>;

/** Opcode for xcb_get_pointer_control. */
pub static XCB_GET_POINTER_CONTROL : u8 = 106;
pub type GetPointerControlReply = base::Reply<get_pointer_control_reply>;
/** Opcode for xcb_set_screen_saver. */
pub static XCB_SET_SCREEN_SAVER : u8 = 107;
pub type GetScreenSaverCookie = base::Cookie<get_screen_saver_cookie>;

/** Opcode for xcb_get_screen_saver. */
pub static XCB_GET_SCREEN_SAVER : u8 = 108;
pub type GetScreenSaverReply = base::Reply<get_screen_saver_reply>;
/** Opcode for xcb_change_hosts. */
pub static XCB_CHANGE_HOSTS : u8 = 109;
pub type HostIterator = host_iterator;

pub type ListHostsCookie = base::Cookie<list_hosts_cookie>;

/** Opcode for xcb_list_hosts. */
pub static XCB_LIST_HOSTS : u8 = 110;
/** Opcode for xcb_set_access_control. */
pub static XCB_SET_ACCESS_CONTROL : u8 = 111;
/** Opcode for xcb_set_close_down_mode. */
pub static XCB_SET_CLOSE_DOWN_MODE : u8 = 112;
/** Opcode for xcb_kill_client. */
pub static XCB_KILL_CLIENT : u8 = 113;
/** Opcode for xcb_rotate_properties. */
pub static XCB_ROTATE_PROPERTIES : u8 = 114;
/** Opcode for xcb_force_screen_saver. */
pub static XCB_FORCE_SCREEN_SAVER : u8 = 115;
pub type SetPointerMappingCookie = base::Cookie<set_pointer_mapping_cookie>;

/** Opcode for xcb_set_pointer_mapping. */
pub static XCB_SET_POINTER_MAPPING : u8 = 116;
pub type SetPointerMappingReply = base::Reply<set_pointer_mapping_reply>;
pub type GetPointerMappingCookie = base::Cookie<get_pointer_mapping_cookie>;

/** Opcode for xcb_get_pointer_mapping. */
pub static XCB_GET_POINTER_MAPPING : u8 = 117;
pub type SetModifierMappingCookie = base::Cookie<set_modifier_mapping_cookie>;

/** Opcode for xcb_set_modifier_mapping. */
pub static XCB_SET_MODIFIER_MAPPING : u8 = 118;
pub type SetModifierMappingReply = base::Reply<set_modifier_mapping_reply>;
pub type GetModifierMappingCookie = base::Cookie<get_modifier_mapping_cookie>;

/** Opcode for xcb_get_modifier_mapping. */
pub static XCB_GET_MODIFIER_MAPPING : u8 = 119;
/** Opcode for xcb_no_operation. */
pub static XCB_NO_OPERATION : u8 = 127;

pub impl Char2b {
  fn byte1(&self) -> u8 {
    unsafe { accessor!(byte1 -> u8, self.strct) }
  }

  fn byte2(&self) -> u8 {
    unsafe { accessor!(byte2 -> u8, self.strct) }
  }

}

impl<'self, Char2b> Iterator<&'self Char2b> for Char2bIterator {
    fn next(&mut self) -> Option<&'self Char2b> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *char2b_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_char2b_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type Window = window;


impl<'self, Window> Iterator<&'self Window> for WindowIterator {
    fn next(&mut self) -> Option<&'self Window> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *window_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_window_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type Pixmap = pixmap;


impl<'self, Pixmap> Iterator<&'self Pixmap> for PixmapIterator {
    fn next(&mut self) -> Option<&'self Pixmap> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *pixmap_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_pixmap_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type Cursor = cursor;


impl<'self, Cursor> Iterator<&'self Cursor> for CursorIterator {
    fn next(&mut self) -> Option<&'self Cursor> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *cursor_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_cursor_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type Font = font;


impl<'self, Font> Iterator<&'self Font> for FontIterator {
    fn next(&mut self) -> Option<&'self Font> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *font_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_font_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type Gcontext = gcontext;


impl<'self, Gcontext> Iterator<&'self Gcontext> for GcontextIterator {
    fn next(&mut self) -> Option<&'self Gcontext> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *gcontext_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_gcontext_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type Colormap = colormap;


impl<'self, Colormap> Iterator<&'self Colormap> for ColormapIterator {
    fn next(&mut self) -> Option<&'self Colormap> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *colormap_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_colormap_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type Atom = atom;


impl<'self, Atom> Iterator<&'self Atom> for AtomIterator {
    fn next(&mut self) -> Option<&'self Atom> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *atom_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_atom_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type Drawable = drawable;


impl<'self, Drawable> Iterator<&'self Drawable> for DrawableIterator {
    fn next(&mut self) -> Option<&'self Drawable> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *drawable_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_drawable_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type Fontable = fontable;


impl<'self, Fontable> Iterator<&'self Fontable> for FontableIterator {
    fn next(&mut self) -> Option<&'self Fontable> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *fontable_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_fontable_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type Visualid = visualid;


impl<'self, Visualid> Iterator<&'self Visualid> for VisualidIterator {
    fn next(&mut self) -> Option<&'self Visualid> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *visualid_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_visualid_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type Timestamp = timestamp;


impl<'self, Timestamp> Iterator<&'self Timestamp> for TimestampIterator {
    fn next(&mut self) -> Option<&'self Timestamp> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *timestamp_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_timestamp_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type Keysym = keysym;


impl<'self, Keysym> Iterator<&'self Keysym> for KeysymIterator {
    fn next(&mut self) -> Option<&'self Keysym> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *keysym_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_keysym_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type Keycode = keycode;


impl<'self, Keycode> Iterator<&'self Keycode> for KeycodeIterator {
    fn next(&mut self) -> Option<&'self Keycode> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *keycode_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_keycode_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type Button = button;


impl<'self, Button> Iterator<&'self Button> for ButtonIterator {
    fn next(&mut self) -> Option<&'self Button> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *button_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_button_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type Point = base::Struct<point>;


pub impl Point {
  fn x(&self) -> i16 {
    unsafe { accessor!(x -> i16, self.strct) }
  }

  fn y(&self) -> i16 {
    unsafe { accessor!(y -> i16, self.strct) }
  }

}

impl<'self, Point> Iterator<&'self Point> for PointIterator {
    fn next(&mut self) -> Option<&'self Point> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *point_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_point_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type Rectangle = base::Struct<rectangle>;


pub impl Rectangle {
  fn x(&self) -> i16 {
    unsafe { accessor!(x -> i16, self.strct) }
  }

  fn y(&self) -> i16 {
    unsafe { accessor!(y -> i16, self.strct) }
  }

  fn width(&self) -> u16 {
    unsafe { accessor!(width -> u16, self.strct) }
  }

  fn height(&self) -> u16 {
    unsafe { accessor!(height -> u16, self.strct) }
  }

}

impl<'self, Rectangle> Iterator<&'self Rectangle> for RectangleIterator {
    fn next(&mut self) -> Option<&'self Rectangle> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *rectangle_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_rectangle_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type Arc = base::Struct<arc>;


pub impl Arc {
  fn x(&self) -> i16 {
    unsafe { accessor!(x -> i16, self.strct) }
  }

  fn y(&self) -> i16 {
    unsafe { accessor!(y -> i16, self.strct) }
  }

  fn width(&self) -> u16 {
    unsafe { accessor!(width -> u16, self.strct) }
  }

  fn height(&self) -> u16 {
    unsafe { accessor!(height -> u16, self.strct) }
  }

  fn angle1(&self) -> i16 {
    unsafe { accessor!(angle1 -> i16, self.strct) }
  }

  fn angle2(&self) -> i16 {
    unsafe { accessor!(angle2 -> i16, self.strct) }
  }

}

impl<'self, Arc> Iterator<&'self Arc> for ArcIterator {
    fn next(&mut self) -> Option<&'self Arc> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *arc_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_arc_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type Format = base::Struct<format>;


pub impl Format {
  fn depth(&self) -> u8 {
    unsafe { accessor!(depth -> u8, self.strct) }
  }

  fn bits_per_pixel(&self) -> u8 {
    unsafe { accessor!(bits_per_pixel -> u8, self.strct) }
  }

  fn scanline_pad(&self) -> u8 {
    unsafe { accessor!(scanline_pad -> u8, self.strct) }
  }

}

impl<'self, Format> Iterator<&'self Format> for FormatIterator {
    fn next(&mut self) -> Option<&'self Format> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *format_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_format_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type Visualtype = base::Struct<visualtype>;


pub impl Visualtype {
  fn visual_id(&self) -> Visualid {
    unsafe { accessor!(visual_id -> Visualid, self.strct) }
  }

  fn class(&self) -> u8 {
    unsafe { accessor!(class -> u8, self.strct) }
  }

  fn bits_per_rgb_value(&self) -> u8 {
    unsafe { accessor!(bits_per_rgb_value -> u8, self.strct) }
  }

  fn colormap_entries(&self) -> u16 {
    unsafe { accessor!(colormap_entries -> u16, self.strct) }
  }

  fn red_mask(&self) -> u32 {
    unsafe { accessor!(red_mask -> u32, self.strct) }
  }

  fn green_mask(&self) -> u32 {
    unsafe { accessor!(green_mask -> u32, self.strct) }
  }

  fn blue_mask(&self) -> u32 {
    unsafe { accessor!(blue_mask -> u32, self.strct) }
  }

}

impl<'self, Visualtype> Iterator<&'self Visualtype> for VisualtypeIterator {
    fn next(&mut self) -> Option<&'self Visualtype> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *visualtype_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_visualtype_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type Depth = base::Struct<depth>;


pub impl Depth {
  fn depth(&self) -> u8 {
    unsafe { accessor!(depth -> u8, self.strct) }
  }

  fn visuals(&self) -> VisualtypeIterator {
    unsafe { accessor!(VisualtypeIterator, xcb_depth_visuals_iterator, self.strct) }
  }

}

impl<'self, Depth> Iterator<&'self Depth> for DepthIterator {
    fn next(&mut self) -> Option<&'self Depth> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *depth_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_depth_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type Screen = base::Struct<screen>;


pub impl Screen {
  fn root(&self) -> Window {
    unsafe { accessor!(root -> Window, self.strct) }
  }

  fn default_colormap(&self) -> Colormap {
    unsafe { accessor!(default_colormap -> Colormap, self.strct) }
  }

  fn white_pixel(&self) -> u32 {
    unsafe { accessor!(white_pixel -> u32, self.strct) }
  }

  fn black_pixel(&self) -> u32 {
    unsafe { accessor!(black_pixel -> u32, self.strct) }
  }

  fn current_input_masks(&self) -> u32 {
    unsafe { accessor!(current_input_masks -> u32, self.strct) }
  }

  fn width_in_pixels(&self) -> u16 {
    unsafe { accessor!(width_in_pixels -> u16, self.strct) }
  }

  fn height_in_pixels(&self) -> u16 {
    unsafe { accessor!(height_in_pixels -> u16, self.strct) }
  }

  fn width_in_millimeters(&self) -> u16 {
    unsafe { accessor!(width_in_millimeters -> u16, self.strct) }
  }

  fn height_in_millimeters(&self) -> u16 {
    unsafe { accessor!(height_in_millimeters -> u16, self.strct) }
  }

  fn min_installed_maps(&self) -> u16 {
    unsafe { accessor!(min_installed_maps -> u16, self.strct) }
  }

  fn max_installed_maps(&self) -> u16 {
    unsafe { accessor!(max_installed_maps -> u16, self.strct) }
  }

  fn root_visual(&self) -> Visualid {
    unsafe { accessor!(root_visual -> Visualid, self.strct) }
  }

  fn backing_stores(&self) -> u8 {
    unsafe { accessor!(backing_stores -> u8, self.strct) }
  }

  fn save_unders(&self) -> u8 {
    unsafe { accessor!(save_unders -> u8, self.strct) }
  }

  fn root_depth(&self) -> u8 {
    unsafe { accessor!(root_depth -> u8, self.strct) }
  }

  fn allowed_depths(&self) -> DepthIterator {
    unsafe { accessor!(DepthIterator, xcb_screen_allowed_depths_iterator, self.strct) }
  }

}

impl<'self, Screen> Iterator<&'self Screen> for ScreenIterator {
    fn next(&mut self) -> Option<&'self Screen> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *screen_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_screen_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type SetupRequest = base::Struct<setup_request>;


pub impl SetupRequest {
  fn byte_order(&self) -> u8 {
    unsafe { accessor!(byte_order -> u8, self.strct) }
  }

  fn protocol_major_version(&self) -> u16 {
    unsafe { accessor!(protocol_major_version -> u16, self.strct) }
  }

  fn protocol_minor_version(&self) -> u16 {
    unsafe { accessor!(protocol_minor_version -> u16, self.strct) }
  }

  fn authorization_protocol_name(&self) -> ~str {
    unsafe { accessor!(str, xcb_setup_request_authorization_protocol_name_length, xcb_setup_request_authorization_protocol_name, self.strct) }
  }

  fn authorization_protocol_data(&self) -> ~str {
    unsafe { accessor!(str, xcb_setup_request_authorization_protocol_data_length, xcb_setup_request_authorization_protocol_data, self.strct) }
  }

}

impl<'self, SetupRequest> Iterator<&'self SetupRequest> for SetupRequestIterator {
    fn next(&mut self) -> Option<&'self SetupRequest> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *setup_request_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_setup_request_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type SetupFailed = base::Struct<setup_failed>;


pub impl SetupFailed {
  fn status(&self) -> u8 {
    unsafe { accessor!(status -> u8, self.strct) }
  }

  fn protocol_major_version(&self) -> u16 {
    unsafe { accessor!(protocol_major_version -> u16, self.strct) }
  }

  fn protocol_minor_version(&self) -> u16 {
    unsafe { accessor!(protocol_minor_version -> u16, self.strct) }
  }

  fn length(&self) -> u16 {
    unsafe { accessor!(length -> u16, self.strct) }
  }

  fn reason(&self) -> ~str {
    unsafe { accessor!(str, xcb_setup_failed_reason_length, xcb_setup_failed_reason, self.strct) }
  }

}

impl<'self, SetupFailed> Iterator<&'self SetupFailed> for SetupFailedIterator {
    fn next(&mut self) -> Option<&'self SetupFailed> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *setup_failed_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_setup_failed_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type SetupAuthenticate = base::Struct<setup_authenticate>;


pub impl SetupAuthenticate {
  fn status(&self) -> u8 {
    unsafe { accessor!(status -> u8, self.strct) }
  }

  fn reason(&self) -> ~str {
    unsafe { accessor!(str, xcb_setup_authenticate_reason_length, xcb_setup_authenticate_reason, self.strct) }
  }

}

impl<'self, SetupAuthenticate> Iterator<&'self SetupAuthenticate> for SetupAuthenticateIterator {
    fn next(&mut self) -> Option<&'self SetupAuthenticate> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *setup_authenticate_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_setup_authenticate_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type Setup = base::Struct<setup>;


pub impl Setup {
  fn status(&self) -> u8 {
    unsafe { accessor!(status -> u8, self.strct) }
  }

  fn protocol_major_version(&self) -> u16 {
    unsafe { accessor!(protocol_major_version -> u16, self.strct) }
  }

  fn protocol_minor_version(&self) -> u16 {
    unsafe { accessor!(protocol_minor_version -> u16, self.strct) }
  }

  fn length(&self) -> u16 {
    unsafe { accessor!(length -> u16, self.strct) }
  }

  fn release_number(&self) -> u32 {
    unsafe { accessor!(release_number -> u32, self.strct) }
  }

  fn resource_id_base(&self) -> u32 {
    unsafe { accessor!(resource_id_base -> u32, self.strct) }
  }

  fn resource_id_mask(&self) -> u32 {
    unsafe { accessor!(resource_id_mask -> u32, self.strct) }
  }

  fn motion_buffer_size(&self) -> u32 {
    unsafe { accessor!(motion_buffer_size -> u32, self.strct) }
  }

  fn maximum_request_length(&self) -> u16 {
    unsafe { accessor!(maximum_request_length -> u16, self.strct) }
  }

  fn image_byte_order(&self) -> u8 {
    unsafe { accessor!(image_byte_order -> u8, self.strct) }
  }

  fn bitmap_format_bit_order(&self) -> u8 {
    unsafe { accessor!(bitmap_format_bit_order -> u8, self.strct) }
  }

  fn bitmap_format_scanline_unit(&self) -> u8 {
    unsafe { accessor!(bitmap_format_scanline_unit -> u8, self.strct) }
  }

  fn bitmap_format_scanline_pad(&self) -> u8 {
    unsafe { accessor!(bitmap_format_scanline_pad -> u8, self.strct) }
  }

  fn min_keycode(&self) -> Keycode {
    unsafe { accessor!(min_keycode -> Keycode, self.strct) }
  }

  fn max_keycode(&self) -> Keycode {
    unsafe { accessor!(max_keycode -> Keycode, self.strct) }
  }

  fn vendor(&self) -> ~str {
    unsafe { accessor!(str, xcb_setup_vendor_length, xcb_setup_vendor, self.strct) }
  }

  fn pixmap_formats(&self) -> FormatIterator {
    unsafe { accessor!(FormatIterator, xcb_setup_pixmap_formats_iterator, self.strct) }
  }

  fn roots(&self) -> ScreenIterator {
    unsafe { accessor!(ScreenIterator, xcb_setup_roots_iterator, self.strct) }
  }

}

impl<'self, Setup> Iterator<&'self Setup> for SetupIterator {
    fn next(&mut self) -> Option<&'self Setup> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *setup_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_setup_next(iter);
            Some(cast::transmute(data))
        }
    }
}


impl<'self, ClientMessageData> Iterator<&'self ClientMessageData> for ClientMessageDataIterator {
    fn next(&mut self) -> Option<&'self ClientMessageData> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *client_message_data_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_client_message_data_next(iter);
            Some(cast::transmute(data))
        }
    }
}


pub impl GetWindowAttributesReply {
  fn backing_store(&self) -> u8 {
    unsafe { accessor!(backing_store -> u8, (*self.reply)) }
  }

  fn visual(&self) -> Visualid {
    unsafe { accessor!(visual -> Visualid, (*self.reply)) }
  }

  fn class(&self) -> u16 {
    unsafe { accessor!(class -> u16, (*self.reply)) }
  }

  fn bit_gravity(&self) -> u8 {
    unsafe { accessor!(bit_gravity -> u8, (*self.reply)) }
  }

  fn win_gravity(&self) -> u8 {
    unsafe { accessor!(win_gravity -> u8, (*self.reply)) }
  }

  fn backing_planes(&self) -> u32 {
    unsafe { accessor!(backing_planes -> u32, (*self.reply)) }
  }

  fn backing_pixel(&self) -> u32 {
    unsafe { accessor!(backing_pixel -> u32, (*self.reply)) }
  }

  fn save_under(&self) -> u8 {
    unsafe { accessor!(save_under -> u8, (*self.reply)) }
  }

  fn map_is_installed(&self) -> u8 {
    unsafe { accessor!(map_is_installed -> u8, (*self.reply)) }
  }

  fn map_state(&self) -> u8 {
    unsafe { accessor!(map_state -> u8, (*self.reply)) }
  }

  fn override_redirect(&self) -> u8 {
    unsafe { accessor!(override_redirect -> u8, (*self.reply)) }
  }

  fn colormap(&self) -> Colormap {
    unsafe { accessor!(colormap -> Colormap, (*self.reply)) }
  }

  fn all_event_masks(&self) -> u32 {
    unsafe { accessor!(all_event_masks -> u32, (*self.reply)) }
  }

  fn your_event_mask(&self) -> u32 {
    unsafe { accessor!(your_event_mask -> u32, (*self.reply)) }
  }

  fn do_not_propagate_mask(&self) -> u16 {
    unsafe { accessor!(do_not_propagate_mask -> u16, (*self.reply)) }
  }

}
impl_reply_cookie!(GetWindowAttributesCookie, get_window_attributes_reply, GetWindowAttributesReply, xcb_get_window_attributes_reply)


pub impl GetGeometryReply {
  fn depth(&self) -> u8 {
    unsafe { accessor!(depth -> u8, (*self.reply)) }
  }

  fn root(&self) -> Window {
    unsafe { accessor!(root -> Window, (*self.reply)) }
  }

  fn x(&self) -> i16 {
    unsafe { accessor!(x -> i16, (*self.reply)) }
  }

  fn y(&self) -> i16 {
    unsafe { accessor!(y -> i16, (*self.reply)) }
  }

  fn width(&self) -> u16 {
    unsafe { accessor!(width -> u16, (*self.reply)) }
  }

  fn height(&self) -> u16 {
    unsafe { accessor!(height -> u16, (*self.reply)) }
  }

  fn border_width(&self) -> u16 {
    unsafe { accessor!(border_width -> u16, (*self.reply)) }
  }

}
impl_reply_cookie!(GetGeometryCookie, get_geometry_reply, GetGeometryReply, xcb_get_geometry_reply)

pub type QueryTreeReply = base::Reply<query_tree_reply>;

pub impl QueryTreeReply {
  fn root(&self) -> Window {
    unsafe { accessor!(root -> Window, (*self.reply)) }
  }

  fn parent(&self) -> Window {
    unsafe { accessor!(parent -> Window, (*self.reply)) }
  }

  fn children(&self) -> ~[Window] {
    unsafe { accessor!(Window, xcb_query_tree_children_length, xcb_query_tree_children, (*self.reply)) }
  }

}
impl_reply_cookie!(QueryTreeCookie, query_tree_reply, QueryTreeReply, xcb_query_tree_reply)


pub impl InternAtomReply {
  fn atom(&self) -> Atom {
    unsafe { accessor!(atom -> Atom, (*self.reply)) }
  }

}
impl_reply_cookie!(InternAtomCookie, intern_atom_reply, InternAtomReply, xcb_intern_atom_reply)

pub type GetAtomNameReply = base::Reply<get_atom_name_reply>;

pub impl GetAtomNameReply {
  fn name(&self) -> ~str {
    unsafe { accessor!(str, xcb_get_atom_name_name_length, xcb_get_atom_name_name, (*self.reply)) }
  }

}
impl_reply_cookie!(GetAtomNameCookie, get_atom_name_reply, GetAtomNameReply, xcb_get_atom_name_reply)

pub type GetPropertyReply = base::Reply<get_property_reply>;

pub impl GetPropertyReply {
  fn format(&self) -> u8 {
    unsafe { accessor!(format -> u8, (*self.reply)) }
  }

  fn type_(&self) -> Atom {
    unsafe { accessor!(type_ -> Atom, (*self.reply)) }
  }

  fn bytes_after(&self) -> u32 {
    unsafe { accessor!(bytes_after -> u32, (*self.reply)) }
  }

  fn value(&self) -> ~[c_void] {
    unsafe { accessor!(c_void, xcb_get_property_value_length, xcb_get_property_value, (*self.reply)) }
  }

}
impl_reply_cookie!(GetPropertyCookie, get_property_reply, GetPropertyReply, xcb_get_property_reply)

pub type ListPropertiesReply = base::Reply<list_properties_reply>;

pub impl ListPropertiesReply {
  fn atoms(&self) -> ~[Atom] {
    unsafe { accessor!(Atom, xcb_list_properties_atoms_length, xcb_list_properties_atoms, (*self.reply)) }
  }

}
impl_reply_cookie!(ListPropertiesCookie, list_properties_reply, ListPropertiesReply, xcb_list_properties_reply)


pub impl GetSelectionOwnerReply {
  fn owner(&self) -> Window {
    unsafe { accessor!(owner -> Window, (*self.reply)) }
  }

}
impl_reply_cookie!(GetSelectionOwnerCookie, get_selection_owner_reply, GetSelectionOwnerReply, xcb_get_selection_owner_reply)


pub impl GrabPointerReply {
  fn status(&self) -> u8 {
    unsafe { accessor!(status -> u8, (*self.reply)) }
  }

}
impl_reply_cookie!(GrabPointerCookie, grab_pointer_reply, GrabPointerReply, xcb_grab_pointer_reply)


pub impl GrabKeyboardReply {
  fn status(&self) -> u8 {
    unsafe { accessor!(status -> u8, (*self.reply)) }
  }

}
impl_reply_cookie!(GrabKeyboardCookie, grab_keyboard_reply, GrabKeyboardReply, xcb_grab_keyboard_reply)


pub impl QueryPointerReply {
  fn same_screen(&self) -> u8 {
    unsafe { accessor!(same_screen -> u8, (*self.reply)) }
  }

  fn root(&self) -> Window {
    unsafe { accessor!(root -> Window, (*self.reply)) }
  }

  fn child(&self) -> Window {
    unsafe { accessor!(child -> Window, (*self.reply)) }
  }

  fn root_x(&self) -> i16 {
    unsafe { accessor!(root_x -> i16, (*self.reply)) }
  }

  fn root_y(&self) -> i16 {
    unsafe { accessor!(root_y -> i16, (*self.reply)) }
  }

  fn win_x(&self) -> i16 {
    unsafe { accessor!(win_x -> i16, (*self.reply)) }
  }

  fn win_y(&self) -> i16 {
    unsafe { accessor!(win_y -> i16, (*self.reply)) }
  }

  fn mask(&self) -> u16 {
    unsafe { accessor!(mask -> u16, (*self.reply)) }
  }

}
impl_reply_cookie!(QueryPointerCookie, query_pointer_reply, QueryPointerReply, xcb_query_pointer_reply)

pub type Timecoord = base::Struct<timecoord>;


pub impl Timecoord {
  fn time(&self) -> Timestamp {
    unsafe { accessor!(time -> Timestamp, self.strct) }
  }

  fn x(&self) -> i16 {
    unsafe { accessor!(x -> i16, self.strct) }
  }

  fn y(&self) -> i16 {
    unsafe { accessor!(y -> i16, self.strct) }
  }

}

impl<'self, Timecoord> Iterator<&'self Timecoord> for TimecoordIterator {
    fn next(&mut self) -> Option<&'self Timecoord> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *timecoord_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_timecoord_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type GetMotionEventsReply = base::Reply<get_motion_events_reply>;

pub impl GetMotionEventsReply {
  fn events(&self) -> TimecoordIterator {
    unsafe { accessor!(TimecoordIterator, xcb_get_motion_events_events_iterator, (*self.reply)) }
  }

}
impl_reply_cookie!(GetMotionEventsCookie, get_motion_events_reply, GetMotionEventsReply, xcb_get_motion_events_reply)


pub impl TranslateCoordinatesReply {
  fn same_screen(&self) -> u8 {
    unsafe { accessor!(same_screen -> u8, (*self.reply)) }
  }

  fn child(&self) -> Window {
    unsafe { accessor!(child -> Window, (*self.reply)) }
  }

  fn dst_x(&self) -> i16 {
    unsafe { accessor!(dst_x -> i16, (*self.reply)) }
  }

  fn dst_y(&self) -> i16 {
    unsafe { accessor!(dst_y -> i16, (*self.reply)) }
  }

}
impl_reply_cookie!(TranslateCoordinatesCookie, translate_coordinates_reply, TranslateCoordinatesReply, xcb_translate_coordinates_reply)


pub impl GetInputFocusReply {
  fn revert_to(&self) -> u8 {
    unsafe { accessor!(revert_to -> u8, (*self.reply)) }
  }

  fn focus(&self) -> Window {
    unsafe { accessor!(focus -> Window, (*self.reply)) }
  }

}
impl_reply_cookie!(GetInputFocusCookie, get_input_focus_reply, GetInputFocusReply, xcb_get_input_focus_reply)


pub impl QueryKeymapReply {
  fn keys(&self) -> ~[u8,..32] {
    unsafe { ~(copy (*self.reply).keys) }
  }

}
impl_reply_cookie!(QueryKeymapCookie, query_keymap_reply, QueryKeymapReply, xcb_query_keymap_reply)

pub type Fontprop = base::Struct<fontprop>;


pub impl Fontprop {
  fn name(&self) -> Atom {
    unsafe { accessor!(name -> Atom, self.strct) }
  }

  fn value(&self) -> u32 {
    unsafe { accessor!(value -> u32, self.strct) }
  }

}

impl<'self, Fontprop> Iterator<&'self Fontprop> for FontpropIterator {
    fn next(&mut self) -> Option<&'self Fontprop> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *fontprop_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_fontprop_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type Charinfo = base::Struct<charinfo>;


pub impl Charinfo {
  fn left_side_bearing(&self) -> i16 {
    unsafe { accessor!(left_side_bearing -> i16, self.strct) }
  }

  fn right_side_bearing(&self) -> i16 {
    unsafe { accessor!(right_side_bearing -> i16, self.strct) }
  }

  fn character_width(&self) -> i16 {
    unsafe { accessor!(character_width -> i16, self.strct) }
  }

  fn ascent(&self) -> i16 {
    unsafe { accessor!(ascent -> i16, self.strct) }
  }

  fn descent(&self) -> i16 {
    unsafe { accessor!(descent -> i16, self.strct) }
  }

  fn attributes(&self) -> u16 {
    unsafe { accessor!(attributes -> u16, self.strct) }
  }

}

impl<'self, Charinfo> Iterator<&'self Charinfo> for CharinfoIterator {
    fn next(&mut self) -> Option<&'self Charinfo> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *charinfo_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_charinfo_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type QueryFontReply = base::Reply<query_font_reply>;

pub impl QueryFontReply {
  fn min_bounds(&self) -> Charinfo {
    unsafe { cast::transmute((*self.reply).min_bounds) }
  }
  fn max_bounds(&self) -> Charinfo {
    unsafe { cast::transmute((*self.reply).max_bounds) }
  }
  fn min_char_or_byte2(&self) -> u16 {
    unsafe { accessor!(min_char_or_byte2 -> u16, (*self.reply)) }
  }

  fn max_char_or_byte2(&self) -> u16 {
    unsafe { accessor!(max_char_or_byte2 -> u16, (*self.reply)) }
  }

  fn default_char(&self) -> u16 {
    unsafe { accessor!(default_char -> u16, (*self.reply)) }
  }

  fn draw_direction(&self) -> u8 {
    unsafe { accessor!(draw_direction -> u8, (*self.reply)) }
  }

  fn min_byte1(&self) -> u8 {
    unsafe { accessor!(min_byte1 -> u8, (*self.reply)) }
  }

  fn max_byte1(&self) -> u8 {
    unsafe { accessor!(max_byte1 -> u8, (*self.reply)) }
  }

  fn all_chars_exist(&self) -> u8 {
    unsafe { accessor!(all_chars_exist -> u8, (*self.reply)) }
  }

  fn font_ascent(&self) -> i16 {
    unsafe { accessor!(font_ascent -> i16, (*self.reply)) }
  }

  fn font_descent(&self) -> i16 {
    unsafe { accessor!(font_descent -> i16, (*self.reply)) }
  }

  fn properties(&self) -> FontpropIterator {
    unsafe { accessor!(FontpropIterator, xcb_query_font_properties_iterator, (*self.reply)) }
  }

  fn char_infos(&self) -> CharinfoIterator {
    unsafe { accessor!(CharinfoIterator, xcb_query_font_char_infos_iterator, (*self.reply)) }
  }

}
impl_reply_cookie!(QueryFontCookie, query_font_reply, QueryFontReply, xcb_query_font_reply)


pub impl QueryTextExtentsReply {
  fn draw_direction(&self) -> u8 {
    unsafe { accessor!(draw_direction -> u8, (*self.reply)) }
  }

  fn font_ascent(&self) -> i16 {
    unsafe { accessor!(font_ascent -> i16, (*self.reply)) }
  }

  fn font_descent(&self) -> i16 {
    unsafe { accessor!(font_descent -> i16, (*self.reply)) }
  }

  fn overall_ascent(&self) -> i16 {
    unsafe { accessor!(overall_ascent -> i16, (*self.reply)) }
  }

  fn overall_descent(&self) -> i16 {
    unsafe { accessor!(overall_descent -> i16, (*self.reply)) }
  }

  fn overall_width(&self) -> i32 {
    unsafe { accessor!(overall_width -> i32, (*self.reply)) }
  }

  fn overall_left(&self) -> i32 {
    unsafe { accessor!(overall_left -> i32, (*self.reply)) }
  }

  fn overall_right(&self) -> i32 {
    unsafe { accessor!(overall_right -> i32, (*self.reply)) }
  }

}
impl_reply_cookie!(QueryTextExtentsCookie, query_text_extents_reply, QueryTextExtentsReply, xcb_query_text_extents_reply)

pub type Str = base::Struct<str_>;


pub impl Str {
  fn name(&self) -> ~str {
    unsafe { accessor!(str, xcb_str_name_length, xcb_str_name, self.strct) }
  }

}

impl<'self, Str> Iterator<&'self Str> for StrIterator {
    fn next(&mut self) -> Option<&'self Str> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *str_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_str_next(iter);
            Some(cast::transmute(data))
        }
    }
}


pub impl ListFontsReply {
  fn names(&self) -> StrIterator {
    unsafe { accessor!(StrIterator, xcb_list_fonts_names_iterator, (*self.reply)) }
  }

}
impl_reply_cookie!(ListFontsCookie, list_fonts_reply, ListFontsReply, xcb_list_fonts_reply)


pub impl ListFontsWithInfoReply {
  fn min_bounds(&self) -> Charinfo {
    unsafe { cast::transmute((*self.reply).min_bounds) }
  }
  fn max_bounds(&self) -> Charinfo {
    unsafe { cast::transmute((*self.reply).max_bounds) }
  }
  fn min_char_or_byte2(&self) -> u16 {
    unsafe { accessor!(min_char_or_byte2 -> u16, (*self.reply)) }
  }

  fn max_char_or_byte2(&self) -> u16 {
    unsafe { accessor!(max_char_or_byte2 -> u16, (*self.reply)) }
  }

  fn default_char(&self) -> u16 {
    unsafe { accessor!(default_char -> u16, (*self.reply)) }
  }

  fn draw_direction(&self) -> u8 {
    unsafe { accessor!(draw_direction -> u8, (*self.reply)) }
  }

  fn min_byte1(&self) -> u8 {
    unsafe { accessor!(min_byte1 -> u8, (*self.reply)) }
  }

  fn max_byte1(&self) -> u8 {
    unsafe { accessor!(max_byte1 -> u8, (*self.reply)) }
  }

  fn all_chars_exist(&self) -> u8 {
    unsafe { accessor!(all_chars_exist -> u8, (*self.reply)) }
  }

  fn font_ascent(&self) -> i16 {
    unsafe { accessor!(font_ascent -> i16, (*self.reply)) }
  }

  fn font_descent(&self) -> i16 {
    unsafe { accessor!(font_descent -> i16, (*self.reply)) }
  }

  fn replies_hint(&self) -> u32 {
    unsafe { accessor!(replies_hint -> u32, (*self.reply)) }
  }

  fn properties(&self) -> FontpropIterator {
    unsafe { accessor!(FontpropIterator, xcb_list_fonts_with_info_properties_iterator, (*self.reply)) }
  }

  fn name(&self) -> ~str {
    unsafe { accessor!(str, xcb_list_fonts_with_info_name_length, xcb_list_fonts_with_info_name, (*self.reply)) }
  }

}
impl_reply_cookie!(ListFontsWithInfoCookie, list_fonts_with_info_reply, ListFontsWithInfoReply, xcb_list_fonts_with_info_reply)

pub type GetFontPathReply = base::Reply<get_font_path_reply>;

pub impl GetFontPathReply {
  fn path(&self) -> StrIterator {
    unsafe { accessor!(StrIterator, xcb_get_font_path_path_iterator, (*self.reply)) }
  }

}
impl_reply_cookie!(GetFontPathCookie, get_font_path_reply, GetFontPathReply, xcb_get_font_path_reply)

pub type Segment = base::Struct<segment>;


pub impl Segment {
  fn x1(&self) -> i16 {
    unsafe { accessor!(x1 -> i16, self.strct) }
  }

  fn y1(&self) -> i16 {
    unsafe { accessor!(y1 -> i16, self.strct) }
  }

  fn x2(&self) -> i16 {
    unsafe { accessor!(x2 -> i16, self.strct) }
  }

  fn y2(&self) -> i16 {
    unsafe { accessor!(y2 -> i16, self.strct) }
  }

}

impl<'self, Segment> Iterator<&'self Segment> for SegmentIterator {
    fn next(&mut self) -> Option<&'self Segment> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *segment_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_segment_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type GetImageReply = base::Reply<get_image_reply>;

pub impl GetImageReply {
  fn depth(&self) -> u8 {
    unsafe { accessor!(depth -> u8, (*self.reply)) }
  }

  fn visual(&self) -> Visualid {
    unsafe { accessor!(visual -> Visualid, (*self.reply)) }
  }

  fn data(&self) -> ~[u8] {
    unsafe { accessor!(u8, xcb_get_image_data_length, xcb_get_image_data, (*self.reply)) }
  }

}
impl_reply_cookie!(GetImageCookie, get_image_reply, GetImageReply, xcb_get_image_reply)

pub type ListInstalledColormapsReply = base::Reply<list_installed_colormaps_reply>;

pub impl ListInstalledColormapsReply {
  fn cmaps(&self) -> ~[Colormap] {
    unsafe { accessor!(Colormap, xcb_list_installed_colormaps_cmaps_length, xcb_list_installed_colormaps_cmaps, (*self.reply)) }
  }

}
impl_reply_cookie!(ListInstalledColormapsCookie, list_installed_colormaps_reply, ListInstalledColormapsReply, xcb_list_installed_colormaps_reply)


pub impl AllocColorReply {
  fn red(&self) -> u16 {
    unsafe { accessor!(red -> u16, (*self.reply)) }
  }

  fn green(&self) -> u16 {
    unsafe { accessor!(green -> u16, (*self.reply)) }
  }

  fn blue(&self) -> u16 {
    unsafe { accessor!(blue -> u16, (*self.reply)) }
  }

  fn pixel(&self) -> u32 {
    unsafe { accessor!(pixel -> u32, (*self.reply)) }
  }

}
impl_reply_cookie!(AllocColorCookie, alloc_color_reply, AllocColorReply, xcb_alloc_color_reply)


pub impl AllocNamedColorReply {
  fn pixel(&self) -> u32 {
    unsafe { accessor!(pixel -> u32, (*self.reply)) }
  }

  fn exact_red(&self) -> u16 {
    unsafe { accessor!(exact_red -> u16, (*self.reply)) }
  }

  fn exact_green(&self) -> u16 {
    unsafe { accessor!(exact_green -> u16, (*self.reply)) }
  }

  fn exact_blue(&self) -> u16 {
    unsafe { accessor!(exact_blue -> u16, (*self.reply)) }
  }

  fn visual_red(&self) -> u16 {
    unsafe { accessor!(visual_red -> u16, (*self.reply)) }
  }

  fn visual_green(&self) -> u16 {
    unsafe { accessor!(visual_green -> u16, (*self.reply)) }
  }

  fn visual_blue(&self) -> u16 {
    unsafe { accessor!(visual_blue -> u16, (*self.reply)) }
  }

}
impl_reply_cookie!(AllocNamedColorCookie, alloc_named_color_reply, AllocNamedColorReply, xcb_alloc_named_color_reply)

pub type AllocColorCellsReply = base::Reply<alloc_color_cells_reply>;

pub impl AllocColorCellsReply {
  fn pixels(&self) -> ~[u32] {
    unsafe { accessor!(u32, xcb_alloc_color_cells_pixels_length, xcb_alloc_color_cells_pixels, (*self.reply)) }
  }

  fn masks(&self) -> ~[u32] {
    unsafe { accessor!(u32, xcb_alloc_color_cells_masks_length, xcb_alloc_color_cells_masks, (*self.reply)) }
  }

}
impl_reply_cookie!(AllocColorCellsCookie, alloc_color_cells_reply, AllocColorCellsReply, xcb_alloc_color_cells_reply)

pub type AllocColorPlanesReply = base::Reply<alloc_color_planes_reply>;

pub impl AllocColorPlanesReply {
  fn red_mask(&self) -> u32 {
    unsafe { accessor!(red_mask -> u32, (*self.reply)) }
  }

  fn green_mask(&self) -> u32 {
    unsafe { accessor!(green_mask -> u32, (*self.reply)) }
  }

  fn blue_mask(&self) -> u32 {
    unsafe { accessor!(blue_mask -> u32, (*self.reply)) }
  }

  fn pixels(&self) -> ~[u32] {
    unsafe { accessor!(u32, xcb_alloc_color_planes_pixels_length, xcb_alloc_color_planes_pixels, (*self.reply)) }
  }

}
impl_reply_cookie!(AllocColorPlanesCookie, alloc_color_planes_reply, AllocColorPlanesReply, xcb_alloc_color_planes_reply)

pub type Coloritem = base::Struct<coloritem>;


pub impl Coloritem {
  fn pixel(&self) -> u32 {
    unsafe { accessor!(pixel -> u32, self.strct) }
  }

  fn red(&self) -> u16 {
    unsafe { accessor!(red -> u16, self.strct) }
  }

  fn green(&self) -> u16 {
    unsafe { accessor!(green -> u16, self.strct) }
  }

  fn blue(&self) -> u16 {
    unsafe { accessor!(blue -> u16, self.strct) }
  }

  fn flags(&self) -> u8 {
    unsafe { accessor!(flags -> u8, self.strct) }
  }

}

impl<'self, Coloritem> Iterator<&'self Coloritem> for ColoritemIterator {
    fn next(&mut self) -> Option<&'self Coloritem> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *coloritem_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_coloritem_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type Rgb = base::Struct<rgb>;


pub impl Rgb {
  fn red(&self) -> u16 {
    unsafe { accessor!(red -> u16, self.strct) }
  }

  fn green(&self) -> u16 {
    unsafe { accessor!(green -> u16, self.strct) }
  }

  fn blue(&self) -> u16 {
    unsafe { accessor!(blue -> u16, self.strct) }
  }

}

impl<'self, Rgb> Iterator<&'self Rgb> for RgbIterator {
    fn next(&mut self) -> Option<&'self Rgb> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *rgb_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_rgb_next(iter);
            Some(cast::transmute(data))
        }
    }
}


pub impl QueryColorsReply {
  fn colors(&self) -> RgbIterator {
    unsafe { accessor!(RgbIterator, xcb_query_colors_colors_iterator, (*self.reply)) }
  }

}
impl_reply_cookie!(QueryColorsCookie, query_colors_reply, QueryColorsReply, xcb_query_colors_reply)


pub impl LookupColorReply {
  fn exact_red(&self) -> u16 {
    unsafe { accessor!(exact_red -> u16, (*self.reply)) }
  }

  fn exact_green(&self) -> u16 {
    unsafe { accessor!(exact_green -> u16, (*self.reply)) }
  }

  fn exact_blue(&self) -> u16 {
    unsafe { accessor!(exact_blue -> u16, (*self.reply)) }
  }

  fn visual_red(&self) -> u16 {
    unsafe { accessor!(visual_red -> u16, (*self.reply)) }
  }

  fn visual_green(&self) -> u16 {
    unsafe { accessor!(visual_green -> u16, (*self.reply)) }
  }

  fn visual_blue(&self) -> u16 {
    unsafe { accessor!(visual_blue -> u16, (*self.reply)) }
  }

}
impl_reply_cookie!(LookupColorCookie, lookup_color_reply, LookupColorReply, xcb_lookup_color_reply)


pub impl QueryBestSizeReply {
  fn width(&self) -> u16 {
    unsafe { accessor!(width -> u16, (*self.reply)) }
  }

  fn height(&self) -> u16 {
    unsafe { accessor!(height -> u16, (*self.reply)) }
  }

}
impl_reply_cookie!(QueryBestSizeCookie, query_best_size_reply, QueryBestSizeReply, xcb_query_best_size_reply)


pub impl QueryExtensionReply {
  fn present(&self) -> u8 {
    unsafe { accessor!(present -> u8, (*self.reply)) }
  }

  fn major_opcode(&self) -> u8 {
    unsafe { accessor!(major_opcode -> u8, (*self.reply)) }
  }

  fn first_event(&self) -> u8 {
    unsafe { accessor!(first_event -> u8, (*self.reply)) }
  }

  fn first_error(&self) -> u8 {
    unsafe { accessor!(first_error -> u8, (*self.reply)) }
  }

}
impl_reply_cookie!(QueryExtensionCookie, query_extension_reply, QueryExtensionReply, xcb_query_extension_reply)

pub type ListExtensionsReply = base::Reply<list_extensions_reply>;

pub impl ListExtensionsReply {
  fn names(&self) -> StrIterator {
    unsafe { accessor!(StrIterator, xcb_list_extensions_names_iterator, (*self.reply)) }
  }

}
impl_reply_cookie!(ListExtensionsCookie, list_extensions_reply, ListExtensionsReply, xcb_list_extensions_reply)

pub type GetKeyboardMappingReply = base::Reply<get_keyboard_mapping_reply>;

pub impl GetKeyboardMappingReply {
  fn keysyms_per_keycode(&self) -> u8 {
    unsafe { accessor!(keysyms_per_keycode -> u8, (*self.reply)) }
  }

  fn keysyms(&self) -> ~[Keysym] {
    unsafe { accessor!(Keysym, xcb_get_keyboard_mapping_keysyms_length, xcb_get_keyboard_mapping_keysyms, (*self.reply)) }
  }

}
impl_reply_cookie!(GetKeyboardMappingCookie, get_keyboard_mapping_reply, GetKeyboardMappingReply, xcb_get_keyboard_mapping_reply)


pub impl GetKeyboardControlReply {
  fn global_auto_repeat(&self) -> u8 {
    unsafe { accessor!(global_auto_repeat -> u8, (*self.reply)) }
  }

  fn led_mask(&self) -> u32 {
    unsafe { accessor!(led_mask -> u32, (*self.reply)) }
  }

  fn key_click_percent(&self) -> u8 {
    unsafe { accessor!(key_click_percent -> u8, (*self.reply)) }
  }

  fn bell_percent(&self) -> u8 {
    unsafe { accessor!(bell_percent -> u8, (*self.reply)) }
  }

  fn bell_pitch(&self) -> u16 {
    unsafe { accessor!(bell_pitch -> u16, (*self.reply)) }
  }

  fn bell_duration(&self) -> u16 {
    unsafe { accessor!(bell_duration -> u16, (*self.reply)) }
  }

  fn auto_repeats(&self) -> ~[u8,..32] {
    unsafe { ~(copy (*self.reply).auto_repeats) }
  }

}
impl_reply_cookie!(GetKeyboardControlCookie, get_keyboard_control_reply, GetKeyboardControlReply, xcb_get_keyboard_control_reply)


pub impl GetPointerControlReply {
  fn acceleration_numerator(&self) -> u16 {
    unsafe { accessor!(acceleration_numerator -> u16, (*self.reply)) }
  }

  fn acceleration_denominator(&self) -> u16 {
    unsafe { accessor!(acceleration_denominator -> u16, (*self.reply)) }
  }

  fn threshold(&self) -> u16 {
    unsafe { accessor!(threshold -> u16, (*self.reply)) }
  }

}
impl_reply_cookie!(GetPointerControlCookie, get_pointer_control_reply, GetPointerControlReply, xcb_get_pointer_control_reply)


pub impl GetScreenSaverReply {
  fn timeout(&self) -> u16 {
    unsafe { accessor!(timeout -> u16, (*self.reply)) }
  }

  fn interval(&self) -> u16 {
    unsafe { accessor!(interval -> u16, (*self.reply)) }
  }

  fn prefer_blanking(&self) -> u8 {
    unsafe { accessor!(prefer_blanking -> u8, (*self.reply)) }
  }

  fn allow_exposures(&self) -> u8 {
    unsafe { accessor!(allow_exposures -> u8, (*self.reply)) }
  }

}
impl_reply_cookie!(GetScreenSaverCookie, get_screen_saver_reply, GetScreenSaverReply, xcb_get_screen_saver_reply)

pub type Host = base::Struct<host>;


pub impl Host {
  fn family(&self) -> u8 {
    unsafe { accessor!(family -> u8, self.strct) }
  }

  fn address(&self) -> ~[u8] {
    unsafe { accessor!(u8, xcb_host_address_length, xcb_host_address, self.strct) }
  }

}

impl<'self, Host> Iterator<&'self Host> for HostIterator {
    fn next(&mut self) -> Option<&'self Host> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *host_iterator = cast::transmute(self);
            let data = (*iter).data;
            xcb_host_next(iter);
            Some(cast::transmute(data))
        }
    }
}

pub type ListHostsReply = base::Reply<list_hosts_reply>;

pub impl ListHostsReply {
  fn mode(&self) -> u8 {
    unsafe { accessor!(mode -> u8, (*self.reply)) }
  }

  fn hosts(&self) -> HostIterator {
    unsafe { accessor!(HostIterator, xcb_list_hosts_hosts_iterator, (*self.reply)) }
  }

}
impl_reply_cookie!(ListHostsCookie, list_hosts_reply, ListHostsReply, xcb_list_hosts_reply)


pub impl SetPointerMappingReply {
  fn status(&self) -> u8 {
    unsafe { accessor!(status -> u8, (*self.reply)) }
  }

}
impl_reply_cookie!(SetPointerMappingCookie, set_pointer_mapping_reply, SetPointerMappingReply, xcb_set_pointer_mapping_reply)

pub type GetPointerMappingReply = base::Reply<get_pointer_mapping_reply>;

pub impl GetPointerMappingReply {
  fn map(&self) -> ~[u8] {
    unsafe { accessor!(u8, xcb_get_pointer_mapping_map_length, xcb_get_pointer_mapping_map, (*self.reply)) }
  }

}
impl_reply_cookie!(GetPointerMappingCookie, get_pointer_mapping_reply, GetPointerMappingReply, xcb_get_pointer_mapping_reply)


pub impl SetModifierMappingReply {
  fn status(&self) -> u8 {
    unsafe { accessor!(status -> u8, (*self.reply)) }
  }

}
impl_reply_cookie!(SetModifierMappingCookie, set_modifier_mapping_reply, SetModifierMappingReply, xcb_set_modifier_mapping_reply)

pub type GetModifierMappingReply = base::Reply<get_modifier_mapping_reply>;

pub impl GetModifierMappingReply {
  fn keycodes(&self) -> ~[Keycode] {
    unsafe { accessor!(Keycode, xcb_get_modifier_mapping_keycodes_length, xcb_get_modifier_mapping_keycodes, (*self.reply)) }
  }

}
impl_reply_cookie!(GetModifierMappingCookie, get_modifier_mapping_reply, GetModifierMappingReply, xcb_get_modifier_mapping_reply)


