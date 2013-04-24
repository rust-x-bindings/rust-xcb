/*
 * This file generated automatically from xproto.xml by r_client.py.
 * Edit at your peril.
 */

//Make the compiler quiet
#[allow(unused_imports)];
#[allow(non_camel_case_types)];
use core;
use core::libc::*;
use ll::base::*;

pub struct char2b {
    byte1 :   u8,
    byte2 :   u8
}

/**
 * @brief char2b_iterator
 **/
pub struct char2b_iterator {
    data : *char2b,
    rem  : c_int,
    index: c_int
}


pub type window = u32;
/**
 * @brief window_iterator
 **/
pub struct window_iterator {
    data : *window,
    rem  : c_int,
    index: c_int
}


pub type pixmap = u32;
/**
 * @brief pixmap_iterator
 **/
pub struct pixmap_iterator {
    data : *pixmap,
    rem  : c_int,
    index: c_int
}


pub type cursor = u32;
/**
 * @brief cursor_iterator
 **/
pub struct cursor_iterator {
    data : *cursor,
    rem  : c_int,
    index: c_int
}


pub type font = u32;
/**
 * @brief font_iterator
 **/
pub struct font_iterator {
    data : *font,
    rem  : c_int,
    index: c_int
}


pub type gcontext = u32;
/**
 * @brief gcontext_iterator
 **/
pub struct gcontext_iterator {
    data : *gcontext,
    rem  : c_int,
    index: c_int
}


pub type colormap = u32;
/**
 * @brief colormap_iterator
 **/
pub struct colormap_iterator {
    data : *colormap,
    rem  : c_int,
    index: c_int
}


pub type atom = u32;
/**
 * @brief atom_iterator
 **/
pub struct atom_iterator {
    data : *atom,
    rem  : c_int,
    index: c_int
}


pub type drawable = u32;
/**
 * @brief drawable_iterator
 **/
pub struct drawable_iterator {
    data : *drawable,
    rem  : c_int,
    index: c_int
}


pub type fontable = u32;
/**
 * @brief fontable_iterator
 **/
pub struct fontable_iterator {
    data : *fontable,
    rem  : c_int,
    index: c_int
}


pub type visualid = u32;
/**
 * @brief visualid_iterator
 **/
pub struct visualid_iterator {
    data : *visualid,
    rem  : c_int,
    index: c_int
}


pub type timestamp = u32;
/**
 * @brief timestamp_iterator
 **/
pub struct timestamp_iterator {
    data : *timestamp,
    rem  : c_int,
    index: c_int
}


pub type keysym = u32;
/**
 * @brief keysym_iterator
 **/
pub struct keysym_iterator {
    data : *keysym,
    rem  : c_int,
    index: c_int
}


pub type keycode = u8;
/**
 * @brief keycode_iterator
 **/
pub struct keycode_iterator {
    data : *keycode,
    rem  : c_int,
    index: c_int
}


pub type button = u8;
/**
 * @brief button_iterator
 **/
pub struct button_iterator {
    data : *button,
    rem  : c_int,
    index: c_int
}


pub struct point {
    x :   i16,
    y :   i16
}

/**
 * @brief point_iterator
 **/
pub struct point_iterator {
    data : *point,
    rem  : c_int,
    index: c_int
}


pub struct rectangle {
    x :        i16,
    y :        i16,
    width :    u16,
    height :   u16
}

/**
 * @brief rectangle_iterator
 **/
pub struct rectangle_iterator {
    data : *rectangle,
    rem  : c_int,
    index: c_int
}


pub struct arc {
    x :        i16,
    y :        i16,
    width :    u16,
    height :   u16,
    angle1 :   i16,
    angle2 :   i16
}

/**
 * @brief arc_iterator
 **/
pub struct arc_iterator {
    data : *arc,
    rem  : c_int,
    index: c_int
}


pub struct format {
    depth :            u8,
    bits_per_pixel :   u8,
    scanline_pad :     u8,
    pad0 :             [u8,..5]
}

/**
 * @brief format_iterator
 **/
pub struct format_iterator {
    data : *format,
    rem  : c_int,
    index: c_int
}


pub struct visualtype {
    visual_id :            visualid,
    class :                u8,
    bits_per_rgb_value :   u8,
    colormap_entries :     u16,
    red_mask :             u32,
    green_mask :           u32,
    blue_mask :            u32,
    pad0 :                 [u8,..4]
}

/**
 * @brief visualtype_iterator
 **/
pub struct visualtype_iterator {
    data : *visualtype,
    rem  : c_int,
    index: c_int
}


pub struct depth {
    depth :         u8,
    pad0 :          u8,
    visuals_len :   u16,
    pad1 :          [u8,..4]
}

/**
 * @brief depth_iterator
 **/
pub struct depth_iterator {
    data : *depth,
    rem  : c_int,
    index: c_int
}


pub struct screen {
    root :                    window,
    default_colormap :        colormap,
    white_pixel :             u32,
    black_pixel :             u32,
    current_input_masks :     u32,
    width_in_pixels :         u16,
    height_in_pixels :        u16,
    width_in_millimeters :    u16,
    height_in_millimeters :   u16,
    min_installed_maps :      u16,
    max_installed_maps :      u16,
    root_visual :             visualid,
    backing_stores :          u8,
    save_unders :             u8,
    root_depth :              u8,
    allowed_depths_len :      u8
}

/**
 * @brief screen_iterator
 **/
pub struct screen_iterator {
    data : *screen,
    rem  : c_int,
    index: c_int
}


pub struct setup_request {
    byte_order :                        u8,
    pad0 :                              u8,
    protocol_major_version :            u16,
    protocol_minor_version :            u16,
    authorization_protocol_name_len :   u16,
    authorization_protocol_data_len :   u16,
    pad1 :                              [u8,..2]
}

/**
 * @brief setup_request_iterator
 **/
pub struct setup_request_iterator {
    data : *setup_request,
    rem  : c_int,
    index: c_int
}


pub struct setup_failed {
    status :                   u8,
    reason_len :               u8,
    protocol_major_version :   u16,
    protocol_minor_version :   u16,
    length :                   u16
}

/**
 * @brief setup_failed_iterator
 **/
pub struct setup_failed_iterator {
    data : *setup_failed,
    rem  : c_int,
    index: c_int
}


pub struct setup_authenticate {
    status :   u8,
    pad0 :     [u8,..5],
    length :   u16
}

/**
 * @brief setup_authenticate_iterator
 **/
pub struct setup_authenticate_iterator {
    data : *setup_authenticate,
    rem  : c_int,
    index: c_int
}


pub struct setup {
    status :                        u8,
    pad0 :                          u8,
    protocol_major_version :        u16,
    protocol_minor_version :        u16,
    length :                        u16,
    release_number :                u32,
    resource_id_base :              u32,
    resource_id_mask :              u32,
    motion_buffer_size :            u32,
    vendor_len :                    u16,
    maximum_request_length :        u16,
    roots_len :                     u8,
    pixmap_formats_len :            u8,
    image_byte_order :              u8,
    bitmap_format_bit_order :       u8,
    bitmap_format_scanline_unit :   u8,
    bitmap_format_scanline_pad :    u8,
    min_keycode :                   keycode,
    max_keycode :                   keycode,
    pad1 :                          [u8,..4]
}

/**
 * @brief setup_iterator
 **/
pub struct setup_iterator {
    data : *setup,
    rem  : c_int,
    index: c_int
}



pub struct key_press_event {
    response_type :   u8,
    detail :          keycode,
    sequence :        u16,
    time :            timestamp,
    root :            window,
    event :           window,
    child :           window,
    root_x :          i16,
    root_y :          i16,
    event_x :         i16,
    event_y :         i16,
    state :           u16,
    same_screen :     u8,
    pad0 :            u8
}



pub type key_release_event = key_press_event;


pub struct button_press_event {
    response_type :   u8,
    detail :          button,
    sequence :        u16,
    time :            timestamp,
    root :            window,
    event :           window,
    child :           window,
    root_x :          i16,
    root_y :          i16,
    event_x :         i16,
    event_y :         i16,
    state :           u16,
    same_screen :     u8,
    pad0 :            u8
}



pub type button_release_event = button_press_event;


pub struct motion_notify_event {
    response_type :   u8,
    detail :          u8,
    sequence :        u16,
    time :            timestamp,
    root :            window,
    event :           window,
    child :           window,
    root_x :          i16,
    root_y :          i16,
    event_x :         i16,
    event_y :         i16,
    state :           u16,
    same_screen :     u8,
    pad0 :            u8
}



pub struct enter_notify_event {
    response_type :       u8,
    detail :              u8,
    sequence :            u16,
    time :                timestamp,
    root :                window,
    event :               window,
    child :               window,
    root_x :              i16,
    root_y :              i16,
    event_x :             i16,
    event_y :             i16,
    state :               u16,
    mode :                u8,
    same_screen_focus :   u8
}



pub type leave_notify_event = enter_notify_event;


pub struct focus_in_event {
    response_type :   u8,
    detail :          u8,
    sequence :        u16,
    event :           window,
    mode :            u8,
    pad0 :            [u8,..3]
}



pub type focus_out_event = focus_in_event;


pub struct keymap_notify_event {
    response_type :   u8,
    keys :            [u8,..31]
}



pub struct expose_event {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    window :          window,
    x :               u16,
    y :               u16,
    width :           u16,
    height :          u16,
    count :           u16,
    pad1 :            [u8,..2]
}



pub struct graphics_exposure_event {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    drawable :        drawable,
    x :               u16,
    y :               u16,
    width :           u16,
    height :          u16,
    minor_opcode :    u16,
    count :           u16,
    major_opcode :    u8,
    pad1 :            [u8,..3]
}



pub struct no_exposure_event {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    drawable :        drawable,
    minor_opcode :    u16,
    major_opcode :    u8,
    pad1 :            u8
}



pub struct visibility_notify_event {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    window :          window,
    state :           u8,
    pad1 :            [u8,..3]
}



pub struct create_notify_event {
    response_type :       u8,
    pad0 :                u8,
    sequence :            u16,
    parent :              window,
    window :              window,
    x :                   i16,
    y :                   i16,
    width :               u16,
    height :              u16,
    border_width :        u16,
    override_redirect :   u8,
    pad1 :                u8
}



pub struct destroy_notify_event {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    event :           window,
    window :          window
}



pub struct unmap_notify_event {
    response_type :    u8,
    pad0 :             u8,
    sequence :         u16,
    event :            window,
    window :           window,
    from_configure :   u8,
    pad1 :             [u8,..3]
}



pub struct map_notify_event {
    response_type :       u8,
    pad0 :                u8,
    sequence :            u16,
    event :               window,
    window :              window,
    override_redirect :   u8,
    pad1 :                [u8,..3]
}



pub struct map_request_event {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    parent :          window,
    window :          window
}



pub struct reparent_notify_event {
    response_type :       u8,
    pad0 :                u8,
    sequence :            u16,
    event :               window,
    window :              window,
    parent :              window,
    x :                   i16,
    y :                   i16,
    override_redirect :   u8,
    pad1 :                [u8,..3]
}



pub struct configure_notify_event {
    response_type :       u8,
    pad0 :                u8,
    sequence :            u16,
    event :               window,
    window :              window,
    above_sibling :       window,
    x :                   i16,
    y :                   i16,
    width :               u16,
    height :              u16,
    border_width :        u16,
    override_redirect :   u8,
    pad1 :                u8
}



pub struct configure_request_event {
    response_type :   u8,
    stack_mode :      u8,
    sequence :        u16,
    parent :          window,
    window :          window,
    sibling :         window,
    x :               i16,
    y :               i16,
    width :           u16,
    height :          u16,
    border_width :    u16,
    value_mask :      u16
}



pub struct gravity_notify_event {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    event :           window,
    window :          window,
    x :               i16,
    y :               i16
}



pub struct resize_request_event {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    window :          window,
    width :           u16,
    height :          u16
}



pub struct circulate_notify_event {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    event :           window,
    window :          window,
    pad1 :            [u8,..4],
    place :           u8,
    pad2 :            [u8,..3]
}



pub type circulate_request_event = circulate_notify_event;


pub struct property_notify_event {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    window :          window,
    atom :            atom,
    time :            timestamp,
    state :           u8,
    pad1 :            [u8,..3]
}



pub struct selection_clear_event {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    time :            timestamp,
    owner :           window,
    selection :       atom
}



pub struct selection_request_event {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    time :            timestamp,
    owner :           window,
    requestor :       window,
    selection :       atom,
    target :          atom,
    property :        atom
}



pub struct selection_notify_event {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    time :            timestamp,
    requestor :       window,
    selection :       atom,
    target :          atom,
    property :        atom
}



pub struct colormap_notify_event {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    window :          window,
    colormap :        colormap,
    new_ :            u8,
    state :           u8,
    pad1 :            [u8,..2]
}


pub struct client_message_data {
    data : [u8,..20]
}
/**
 * @brief client_message_data_iterator
 **/
pub struct client_message_data_iterator {
    data : *client_message_data,
    rem  : c_int,
    index: c_int
}



pub struct client_message_event {
    response_type :   u8,
    format :          u8,
    sequence :        u16,
    window :          window,
    type_ :           atom,
    data :            client_message_data
}



pub struct mapping_notify_event {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    request :         u8,
    first_keycode :   keycode,
    count :           u8,
    pad1 :            u8
}



pub struct request_error {
    response_type :   u8,
    error_code :      u8,
    sequence :        u16,
    bad_value :       u32,
    minor_opcode :    u16,
    major_opcode :    u8,
    pad0 :            u8
}



pub struct value_error {
    response_type :   u8,
    error_code :      u8,
    sequence :        u16,
    bad_value :       u32,
    minor_opcode :    u16,
    major_opcode :    u8,
    pad0 :            u8
}



pub type window_error  = value_error;


pub type pixmap_error  = value_error;


pub type atom_error  = value_error;


pub type cursor_error  = value_error;


pub type font_error  = value_error;


pub type match_error  = request_error;


pub type drawable_error  = value_error;


pub type access_error  = request_error;


pub type alloc_error  = request_error;


pub type colormap_error  = value_error;


pub type g_context_error  = value_error;


pub type id_choice_error  = value_error;


pub type name_error  = request_error;


pub type length_error  = request_error;


pub type implementation_error  = request_error;


pub struct create_window_request {
    major_opcode :   u8,
    depth :          u8,
    length :         u16,
    wid :            window,
    parent :         window,
    x :              i16,
    y :              i16,
    width :          u16,
    height :         u16,
    border_width :   u16,
    class :          u16,
    visual :         visualid,
    value_mask :     u32
}



pub struct change_window_attributes_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16,
    window :         window,
    value_mask :     u32
}


pub struct get_window_attributes_cookie {
    sequence : c_uint
}


pub struct get_window_attributes_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16,
    window :         window
}


pub struct get_window_attributes_reply {
    response_type :           u8,
    backing_store :           u8,
    sequence :                u16,
    length :                  u32,
    visual :                  visualid,
    class :                   u16,
    bit_gravity :             u8,
    win_gravity :             u8,
    backing_planes :          u32,
    backing_pixel :           u32,
    save_under :              u8,
    map_is_installed :        u8,
    map_state :               u8,
    override_redirect :       u8,
    colormap :                colormap,
    all_event_masks :         u32,
    your_event_mask :         u32,
    do_not_propagate_mask :   u16,
    pad0 :                    [u8,..2]
}



pub struct destroy_window_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16,
    window :         window
}



pub struct destroy_subwindows_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16,
    window :         window
}



pub struct change_save_set_request {
    major_opcode :   u8,
    mode :           u8,
    length :         u16,
    window :         window
}



pub struct reparent_window_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16,
    window :         window,
    parent :         window,
    x :              i16,
    y :              i16
}



pub struct map_window_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16,
    window :         window
}



pub struct map_subwindows_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16,
    window :         window
}



pub struct unmap_window_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16,
    window :         window
}



pub struct unmap_subwindows_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16,
    window :         window
}



pub struct configure_window_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16,
    window :         window,
    value_mask :     u16,
    pad1 :           [u8,..2]
}



pub struct circulate_window_request {
    major_opcode :   u8,
    direction :      u8,
    length :         u16,
    window :         window
}


pub struct get_geometry_cookie {
    sequence : c_uint
}


pub struct get_geometry_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16,
    drawable :       drawable
}


pub struct get_geometry_reply {
    response_type :   u8,
    depth :           u8,
    sequence :        u16,
    length :          u32,
    root :            window,
    x :               i16,
    y :               i16,
    width :           u16,
    height :          u16,
    border_width :    u16,
    pad0 :            [u8,..2]
}


pub struct query_tree_cookie {
    sequence : c_uint
}


pub struct query_tree_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16,
    window :         window
}


pub struct query_tree_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    root :            window,
    parent :          window,
    children_len :    u16,
    pad1 :            [u8,..14]
}


pub struct intern_atom_cookie {
    sequence : c_uint
}


pub struct intern_atom_request {
    major_opcode :     u8,
    only_if_exists :   u8,
    length :           u16,
    name_len :         u16,
    pad0 :             [u8,..2]
}


pub struct intern_atom_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    atom :            atom
}


pub struct get_atom_name_cookie {
    sequence : c_uint
}


pub struct get_atom_name_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16,
    atom :           atom
}


pub struct get_atom_name_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    name_len :        u16,
    pad1 :            [u8,..22]
}



pub struct change_property_request {
    major_opcode :   u8,
    mode :           u8,
    length :         u16,
    window :         window,
    property :       atom,
    type_ :          atom,
    format :         u8,
    pad0 :           [u8,..3],
    data_len :       u32
}



pub struct delete_property_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16,
    window :         window,
    property :       atom
}


pub struct get_property_cookie {
    sequence : c_uint
}


pub struct get_property_request {
    major_opcode :   u8,
    delete :         u8,
    length :         u16,
    window :         window,
    property :       atom,
    type_ :          atom,
    long_offset :    u32,
    long_length :    u32
}


pub struct get_property_reply {
    response_type :   u8,
    format :          u8,
    sequence :        u16,
    length :          u32,
    type_ :           atom,
    bytes_after :     u32,
    value_len :       u32,
    pad0 :            [u8,..12]
}


pub struct list_properties_cookie {
    sequence : c_uint
}


pub struct list_properties_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16,
    window :         window
}


pub struct list_properties_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    atoms_len :       u16,
    pad1 :            [u8,..22]
}



pub struct set_selection_owner_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16,
    owner :          window,
    selection :      atom,
    time :           timestamp
}


pub struct get_selection_owner_cookie {
    sequence : c_uint
}


pub struct get_selection_owner_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16,
    selection :      atom
}


pub struct get_selection_owner_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    owner :           window
}



pub struct convert_selection_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16,
    requestor :      window,
    selection :      atom,
    target :         atom,
    property :       atom,
    time :           timestamp
}



pub struct send_event_request {
    major_opcode :   u8,
    propagate :      u8,
    length :         u16,
    destination :    window,
    event_mask :     u32,
    event :          [c_char,..32]
}


pub struct grab_pointer_cookie {
    sequence : c_uint
}


pub struct grab_pointer_request {
    major_opcode :    u8,
    owner_events :    u8,
    length :          u16,
    grab_window :     window,
    event_mask :      u16,
    pointer_mode :    u8,
    keyboard_mode :   u8,
    confine_to :      window,
    cursor :          cursor,
    time :            timestamp
}


pub struct grab_pointer_reply {
    response_type :   u8,
    status :          u8,
    sequence :        u16,
    length :          u32
}



pub struct ungrab_pointer_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16,
    time :           timestamp
}



pub struct grab_button_request {
    major_opcode :    u8,
    owner_events :    u8,
    length :          u16,
    grab_window :     window,
    event_mask :      u16,
    pointer_mode :    u8,
    keyboard_mode :   u8,
    confine_to :      window,
    cursor :          cursor,
    button :          u8,
    pad0 :            u8,
    modifiers :       u16
}



pub struct ungrab_button_request {
    major_opcode :   u8,
    button :         u8,
    length :         u16,
    grab_window :    window,
    modifiers :      u16,
    pad0 :           [u8,..2]
}



pub struct change_active_pointer_grab_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16,
    cursor :         cursor,
    time :           timestamp,
    event_mask :     u16,
    pad1 :           [u8,..2]
}


pub struct grab_keyboard_cookie {
    sequence : c_uint
}


pub struct grab_keyboard_request {
    major_opcode :    u8,
    owner_events :    u8,
    length :          u16,
    grab_window :     window,
    time :            timestamp,
    pointer_mode :    u8,
    keyboard_mode :   u8,
    pad0 :            [u8,..2]
}


pub struct grab_keyboard_reply {
    response_type :   u8,
    status :          u8,
    sequence :        u16,
    length :          u32
}



pub struct ungrab_keyboard_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16,
    time :           timestamp
}



pub struct grab_key_request {
    major_opcode :    u8,
    owner_events :    u8,
    length :          u16,
    grab_window :     window,
    modifiers :       u16,
    key :             keycode,
    pointer_mode :    u8,
    keyboard_mode :   u8,
    pad0 :            [u8,..3]
}



pub struct ungrab_key_request {
    major_opcode :   u8,
    key :            keycode,
    length :         u16,
    grab_window :    window,
    modifiers :      u16,
    pad0 :           [u8,..2]
}



pub struct allow_events_request {
    major_opcode :   u8,
    mode :           u8,
    length :         u16,
    time :           timestamp
}



pub struct grab_server_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16
}



pub struct ungrab_server_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16
}


pub struct query_pointer_cookie {
    sequence : c_uint
}


pub struct query_pointer_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16,
    window :         window
}


pub struct query_pointer_reply {
    response_type :   u8,
    same_screen :     u8,
    sequence :        u16,
    length :          u32,
    root :            window,
    child :           window,
    root_x :          i16,
    root_y :          i16,
    win_x :           i16,
    win_y :           i16,
    mask :            u16,
    pad0 :            [u8,..2]
}


pub struct timecoord {
    time :   timestamp,
    x :      i16,
    y :      i16
}

/**
 * @brief timecoord_iterator
 **/
pub struct timecoord_iterator {
    data : *timecoord,
    rem  : c_int,
    index: c_int
}


pub struct get_motion_events_cookie {
    sequence : c_uint
}


pub struct get_motion_events_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16,
    window :         window,
    start :          timestamp,
    stop :           timestamp
}


pub struct get_motion_events_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    events_len :      u32,
    pad1 :            [u8,..20]
}


pub struct translate_coordinates_cookie {
    sequence : c_uint
}


pub struct translate_coordinates_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16,
    src_window :     window,
    dst_window :     window,
    src_x :          i16,
    src_y :          i16
}


pub struct translate_coordinates_reply {
    response_type :   u8,
    same_screen :     u8,
    sequence :        u16,
    length :          u32,
    child :           window,
    dst_x :           i16,
    dst_y :           i16
}



pub struct warp_pointer_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16,
    src_window :     window,
    dst_window :     window,
    src_x :          i16,
    src_y :          i16,
    src_width :      u16,
    src_height :     u16,
    dst_x :          i16,
    dst_y :          i16
}



pub struct set_input_focus_request {
    major_opcode :   u8,
    revert_to :      u8,
    length :         u16,
    focus :          window,
    time :           timestamp
}


pub struct get_input_focus_cookie {
    sequence : c_uint
}


pub struct get_input_focus_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16
}


pub struct get_input_focus_reply {
    response_type :   u8,
    revert_to :       u8,
    sequence :        u16,
    length :          u32,
    focus :           window
}


pub struct query_keymap_cookie {
    sequence : c_uint
}


pub struct query_keymap_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16
}


pub struct query_keymap_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    keys :            [u8,..32]
}



pub struct open_font_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16,
    fid :            font,
    name_len :       u16,
    pad1 :           [u8,..2]
}



pub struct close_font_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16,
    font :           font
}


pub struct fontprop {
    name :    atom,
    value :   u32
}

/**
 * @brief fontprop_iterator
 **/
pub struct fontprop_iterator {
    data : *fontprop,
    rem  : c_int,
    index: c_int
}


pub struct charinfo {
    left_side_bearing :    i16,
    right_side_bearing :   i16,
    character_width :      i16,
    ascent :               i16,
    descent :              i16,
    attributes :           u16
}

/**
 * @brief charinfo_iterator
 **/
pub struct charinfo_iterator {
    data : *charinfo,
    rem  : c_int,
    index: c_int
}


pub struct query_font_cookie {
    sequence : c_uint
}


pub struct query_font_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16,
    font :           fontable
}


pub struct query_font_reply {
    response_type :       u8,
    pad0 :                u8,
    sequence :            u16,
    length :              u32,
    min_bounds :          charinfo,
    pad1 :                [u8,..4],
    max_bounds :          charinfo,
    pad2 :                [u8,..4],
    min_char_or_byte2 :   u16,
    max_char_or_byte2 :   u16,
    default_char :        u16,
    properties_len :      u16,
    draw_direction :      u8,
    min_byte1 :           u8,
    max_byte1 :           u8,
    all_chars_exist :     u8,
    font_ascent :         i16,
    font_descent :        i16,
    char_infos_len :      u32
}


pub struct query_text_extents_cookie {
    sequence : c_uint
}


pub struct query_text_extents_request {
    major_opcode :   u8,
    odd_length :     u8,
    length :         u16,
    font :           fontable
}


pub struct query_text_extents_reply {
    response_type :     u8,
    draw_direction :    u8,
    sequence :          u16,
    length :            u32,
    font_ascent :       i16,
    font_descent :      i16,
    overall_ascent :    i16,
    overall_descent :   i16,
    overall_width :     i32,
    overall_left :      i32,
    overall_right :     i32
}


pub struct str_ {
    name_len :   u8
}

/**
 * @brief str_iterator
 **/
pub struct str_iterator {
    data : *str_,
    rem  : c_int,
    index: c_int
}


pub struct list_fonts_cookie {
    sequence : c_uint
}


pub struct list_fonts_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16,
    max_names :      u16,
    pattern_len :    u16
}


pub struct list_fonts_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    names_len :       u16,
    pad1 :            [u8,..22]
}


pub struct list_fonts_with_info_cookie {
    sequence : c_uint
}


pub struct list_fonts_with_info_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16,
    max_names :      u16,
    pattern_len :    u16
}


pub struct list_fonts_with_info_reply {
    response_type :       u8,
    name_len :            u8,
    sequence :            u16,
    length :              u32,
    min_bounds :          charinfo,
    pad0 :                [u8,..4],
    max_bounds :          charinfo,
    pad1 :                [u8,..4],
    min_char_or_byte2 :   u16,
    max_char_or_byte2 :   u16,
    default_char :        u16,
    properties_len :      u16,
    draw_direction :      u8,
    min_byte1 :           u8,
    max_byte1 :           u8,
    all_chars_exist :     u8,
    font_ascent :         i16,
    font_descent :        i16,
    replies_hint :        u32
}



pub struct set_font_path_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16,
    font_qty :       u16,
    pad1 :           [u8,..2]
}


pub struct get_font_path_cookie {
    sequence : c_uint
}


pub struct get_font_path_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16
}


pub struct get_font_path_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    path_len :        u16,
    pad1 :            [u8,..22]
}



pub struct create_pixmap_request {
    major_opcode :   u8,
    depth :          u8,
    length :         u16,
    pid :            pixmap,
    drawable :       drawable,
    width :          u16,
    height :         u16
}



pub struct free_pixmap_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16,
    pixmap :         pixmap
}



pub struct create_gc_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16,
    cid :            gcontext,
    drawable :       drawable,
    value_mask :     u32
}



pub struct change_gc_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16,
    gc :             gcontext,
    value_mask :     u32
}



pub struct copy_gc_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16,
    src_gc :         gcontext,
    dst_gc :         gcontext,
    value_mask :     u32
}



pub struct set_dashes_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16,
    gc :             gcontext,
    dash_offset :    u16,
    dashes_len :     u16
}



pub struct set_clip_rectangles_request {
    major_opcode :    u8,
    ordering :        u8,
    length :          u16,
    gc :              gcontext,
    clip_x_origin :   i16,
    clip_y_origin :   i16
}



pub struct free_gc_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16,
    gc :             gcontext
}



pub struct clear_area_request {
    major_opcode :   u8,
    exposures :      u8,
    length :         u16,
    window :         window,
    x :              i16,
    y :              i16,
    width :          u16,
    height :         u16
}



pub struct copy_area_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16,
    src_drawable :   drawable,
    dst_drawable :   drawable,
    gc :             gcontext,
    src_x :          i16,
    src_y :          i16,
    dst_x :          i16,
    dst_y :          i16,
    width :          u16,
    height :         u16
}



pub struct copy_plane_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16,
    src_drawable :   drawable,
    dst_drawable :   drawable,
    gc :             gcontext,
    src_x :          i16,
    src_y :          i16,
    dst_x :          i16,
    dst_y :          i16,
    width :          u16,
    height :         u16,
    bit_plane :      u32
}



pub struct poly_point_request {
    major_opcode :      u8,
    coordinate_mode :   u8,
    length :            u16,
    drawable :          drawable,
    gc :                gcontext
}



pub struct poly_line_request {
    major_opcode :      u8,
    coordinate_mode :   u8,
    length :            u16,
    drawable :          drawable,
    gc :                gcontext
}


pub struct segment {
    x1 :   i16,
    y1 :   i16,
    x2 :   i16,
    y2 :   i16
}

/**
 * @brief segment_iterator
 **/
pub struct segment_iterator {
    data : *segment,
    rem  : c_int,
    index: c_int
}



pub struct poly_segment_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16,
    drawable :       drawable,
    gc :             gcontext
}



pub struct poly_rectangle_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16,
    drawable :       drawable,
    gc :             gcontext
}



pub struct poly_arc_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16,
    drawable :       drawable,
    gc :             gcontext
}



pub struct fill_poly_request {
    major_opcode :      u8,
    pad0 :              u8,
    length :            u16,
    drawable :          drawable,
    gc :                gcontext,
    shape :             u8,
    coordinate_mode :   u8,
    pad1 :              [u8,..2]
}



pub struct poly_fill_rectangle_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16,
    drawable :       drawable,
    gc :             gcontext
}



pub struct poly_fill_arc_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16,
    drawable :       drawable,
    gc :             gcontext
}



pub struct put_image_request {
    major_opcode :   u8,
    format :         u8,
    length :         u16,
    drawable :       drawable,
    gc :             gcontext,
    width :          u16,
    height :         u16,
    dst_x :          i16,
    dst_y :          i16,
    left_pad :       u8,
    depth :          u8,
    pad0 :           [u8,..2]
}


pub struct get_image_cookie {
    sequence : c_uint
}


pub struct get_image_request {
    major_opcode :   u8,
    format :         u8,
    length :         u16,
    drawable :       drawable,
    x :              i16,
    y :              i16,
    width :          u16,
    height :         u16,
    plane_mask :     u32
}


pub struct get_image_reply {
    response_type :   u8,
    depth :           u8,
    sequence :        u16,
    length :          u32,
    visual :          visualid,
    pad0 :            [u8,..20]
}



pub struct poly_text_8_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16,
    drawable :       drawable,
    gc :             gcontext,
    x :              i16,
    y :              i16
}



pub struct poly_text_16_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16,
    drawable :       drawable,
    gc :             gcontext,
    x :              i16,
    y :              i16
}



pub struct image_text_8_request {
    major_opcode :   u8,
    string_len :     u8,
    length :         u16,
    drawable :       drawable,
    gc :             gcontext,
    x :              i16,
    y :              i16
}



pub struct image_text_16_request {
    major_opcode :   u8,
    string_len :     u8,
    length :         u16,
    drawable :       drawable,
    gc :             gcontext,
    x :              i16,
    y :              i16
}



pub struct create_colormap_request {
    major_opcode :   u8,
    alloc :          u8,
    length :         u16,
    mid :            colormap,
    window :         window,
    visual :         visualid
}



pub struct free_colormap_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16,
    cmap :           colormap
}



pub struct copy_colormap_and_free_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16,
    mid :            colormap,
    src_cmap :       colormap
}



pub struct install_colormap_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16,
    cmap :           colormap
}



pub struct uninstall_colormap_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16,
    cmap :           colormap
}


pub struct list_installed_colormaps_cookie {
    sequence : c_uint
}


pub struct list_installed_colormaps_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16,
    window :         window
}


pub struct list_installed_colormaps_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    cmaps_len :       u16,
    pad1 :            [u8,..22]
}


pub struct alloc_color_cookie {
    sequence : c_uint
}


pub struct alloc_color_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16,
    cmap :           colormap,
    red :            u16,
    green :          u16,
    blue :           u16,
    pad1 :           [u8,..2]
}


pub struct alloc_color_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    red :             u16,
    green :           u16,
    blue :            u16,
    pad1 :            [u8,..2],
    pixel :           u32
}


pub struct alloc_named_color_cookie {
    sequence : c_uint
}


pub struct alloc_named_color_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16,
    cmap :           colormap,
    name_len :       u16,
    pad1 :           [u8,..2]
}


pub struct alloc_named_color_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pixel :           u32,
    exact_red :       u16,
    exact_green :     u16,
    exact_blue :      u16,
    visual_red :      u16,
    visual_green :    u16,
    visual_blue :     u16
}


pub struct alloc_color_cells_cookie {
    sequence : c_uint
}


pub struct alloc_color_cells_request {
    major_opcode :   u8,
    contiguous :     u8,
    length :         u16,
    cmap :           colormap,
    colors :         u16,
    planes :         u16
}


pub struct alloc_color_cells_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pixels_len :      u16,
    masks_len :       u16,
    pad1 :            [u8,..20]
}


pub struct alloc_color_planes_cookie {
    sequence : c_uint
}


pub struct alloc_color_planes_request {
    major_opcode :   u8,
    contiguous :     u8,
    length :         u16,
    cmap :           colormap,
    colors :         u16,
    reds :           u16,
    greens :         u16,
    blues :          u16
}


pub struct alloc_color_planes_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pixels_len :      u16,
    pad1 :            [u8,..2],
    red_mask :        u32,
    green_mask :      u32,
    blue_mask :       u32,
    pad2 :            [u8,..8]
}



pub struct free_colors_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16,
    cmap :           colormap,
    plane_mask :     u32
}


pub struct coloritem {
    pixel :   u32,
    red :     u16,
    green :   u16,
    blue :    u16,
    flags :   u8,
    pad0 :    u8
}

/**
 * @brief coloritem_iterator
 **/
pub struct coloritem_iterator {
    data : *coloritem,
    rem  : c_int,
    index: c_int
}



pub struct store_colors_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16,
    cmap :           colormap
}



pub struct store_named_color_request {
    major_opcode :   u8,
    flags :          u8,
    length :         u16,
    cmap :           colormap,
    pixel :          u32,
    name_len :       u16,
    pad0 :           [u8,..2]
}


pub struct rgb {
    red :     u16,
    green :   u16,
    blue :    u16,
    pad0 :    [u8,..2]
}

/**
 * @brief rgb_iterator
 **/
pub struct rgb_iterator {
    data : *rgb,
    rem  : c_int,
    index: c_int
}


pub struct query_colors_cookie {
    sequence : c_uint
}


pub struct query_colors_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16,
    cmap :           colormap
}


pub struct query_colors_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    colors_len :      u16,
    pad1 :            [u8,..22]
}


pub struct lookup_color_cookie {
    sequence : c_uint
}


pub struct lookup_color_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16,
    cmap :           colormap,
    name_len :       u16,
    pad1 :           [u8,..2]
}


pub struct lookup_color_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    exact_red :       u16,
    exact_green :     u16,
    exact_blue :      u16,
    visual_red :      u16,
    visual_green :    u16,
    visual_blue :     u16
}



pub struct create_cursor_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16,
    cid :            cursor,
    source :         pixmap,
    mask :           pixmap,
    fore_red :       u16,
    fore_green :     u16,
    fore_blue :      u16,
    back_red :       u16,
    back_green :     u16,
    back_blue :      u16,
    x :              u16,
    y :              u16
}



pub struct create_glyph_cursor_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16,
    cid :            cursor,
    source_font :    font,
    mask_font :      font,
    source_char :    u16,
    mask_char :      u16,
    fore_red :       u16,
    fore_green :     u16,
    fore_blue :      u16,
    back_red :       u16,
    back_green :     u16,
    back_blue :      u16
}



pub struct free_cursor_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16,
    cursor :         cursor
}



pub struct recolor_cursor_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16,
    cursor :         cursor,
    fore_red :       u16,
    fore_green :     u16,
    fore_blue :      u16,
    back_red :       u16,
    back_green :     u16,
    back_blue :      u16
}


pub struct query_best_size_cookie {
    sequence : c_uint
}


pub struct query_best_size_request {
    major_opcode :   u8,
    class :          u8,
    length :         u16,
    drawable :       drawable,
    width :          u16,
    height :         u16
}


pub struct query_best_size_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    width :           u16,
    height :          u16
}


pub struct query_extension_cookie {
    sequence : c_uint
}


pub struct query_extension_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16,
    name_len :       u16,
    pad1 :           [u8,..2]
}


pub struct query_extension_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    present :         u8,
    major_opcode :    u8,
    first_event :     u8,
    first_error :     u8
}


pub struct list_extensions_cookie {
    sequence : c_uint
}


pub struct list_extensions_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16
}


pub struct list_extensions_reply {
    response_type :   u8,
    names_len :       u8,
    sequence :        u16,
    length :          u32,
    pad0 :            [u8,..24]
}



pub struct change_keyboard_mapping_request {
    major_opcode :          u8,
    keycode_count :         u8,
    length :                u16,
    first_keycode :         keycode,
    keysyms_per_keycode :   u8,
    pad0 :                  [u8,..2]
}


pub struct get_keyboard_mapping_cookie {
    sequence : c_uint
}


pub struct get_keyboard_mapping_request {
    major_opcode :    u8,
    pad0 :            u8,
    length :          u16,
    first_keycode :   keycode,
    count :           u8
}


pub struct get_keyboard_mapping_reply {
    response_type :         u8,
    keysyms_per_keycode :   u8,
    sequence :              u16,
    length :                u32,
    pad0 :                  [u8,..24]
}



pub struct change_keyboard_control_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16,
    value_mask :     u32
}


pub struct get_keyboard_control_cookie {
    sequence : c_uint
}


pub struct get_keyboard_control_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16
}


pub struct get_keyboard_control_reply {
    response_type :        u8,
    global_auto_repeat :   u8,
    sequence :             u16,
    length :               u32,
    led_mask :             u32,
    key_click_percent :    u8,
    bell_percent :         u8,
    bell_pitch :           u16,
    bell_duration :        u16,
    pad0 :                 [u8,..2],
    auto_repeats :         [u8,..32]
}



pub struct bell_request {
    major_opcode :   u8,
    percent :        i8,
    length :         u16
}



pub struct change_pointer_control_request {
    major_opcode :               u8,
    pad0 :                       u8,
    length :                     u16,
    acceleration_numerator :     i16,
    acceleration_denominator :   i16,
    threshold :                  i16,
    do_acceleration :            u8,
    do_threshold :               u8
}


pub struct get_pointer_control_cookie {
    sequence : c_uint
}


pub struct get_pointer_control_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16
}


pub struct get_pointer_control_reply {
    response_type :              u8,
    pad0 :                       u8,
    sequence :                   u16,
    length :                     u32,
    acceleration_numerator :     u16,
    acceleration_denominator :   u16,
    threshold :                  u16,
    pad1 :                       [u8,..18]
}



pub struct set_screen_saver_request {
    major_opcode :      u8,
    pad0 :              u8,
    length :            u16,
    timeout :           i16,
    interval :          i16,
    prefer_blanking :   u8,
    allow_exposures :   u8
}


pub struct get_screen_saver_cookie {
    sequence : c_uint
}


pub struct get_screen_saver_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16
}


pub struct get_screen_saver_reply {
    response_type :     u8,
    pad0 :              u8,
    sequence :          u16,
    length :            u32,
    timeout :           u16,
    interval :          u16,
    prefer_blanking :   u8,
    allow_exposures :   u8,
    pad1 :              [u8,..18]
}



pub struct change_hosts_request {
    major_opcode :   u8,
    mode :           u8,
    length :         u16,
    family :         u8,
    pad0 :           u8,
    address_len :    u16
}


pub struct host {
    family :        u8,
    pad0 :          u8,
    address_len :   u16
}

/**
 * @brief host_iterator
 **/
pub struct host_iterator {
    data : *host,
    rem  : c_int,
    index: c_int
}


pub struct list_hosts_cookie {
    sequence : c_uint
}


pub struct list_hosts_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16
}


pub struct list_hosts_reply {
    response_type :   u8,
    mode :            u8,
    sequence :        u16,
    length :          u32,
    hosts_len :       u16,
    pad0 :            [u8,..22]
}



pub struct set_access_control_request {
    major_opcode :   u8,
    mode :           u8,
    length :         u16
}



pub struct set_close_down_mode_request {
    major_opcode :   u8,
    mode :           u8,
    length :         u16
}



pub struct kill_client_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16,
    resource :       u32
}



pub struct rotate_properties_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16,
    window :         window,
    atoms_len :      u16,
    delta :          i16
}



pub struct force_screen_saver_request {
    major_opcode :   u8,
    mode :           u8,
    length :         u16
}


pub struct set_pointer_mapping_cookie {
    sequence : c_uint
}


pub struct set_pointer_mapping_request {
    major_opcode :   u8,
    map_len :        u8,
    length :         u16
}


pub struct set_pointer_mapping_reply {
    response_type :   u8,
    status :          u8,
    sequence :        u16,
    length :          u32
}


pub struct get_pointer_mapping_cookie {
    sequence : c_uint
}


pub struct get_pointer_mapping_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16
}


pub struct get_pointer_mapping_reply {
    response_type :   u8,
    map_len :         u8,
    sequence :        u16,
    length :          u32,
    pad0 :            [u8,..24]
}


pub struct set_modifier_mapping_cookie {
    sequence : c_uint
}


pub struct set_modifier_mapping_request {
    major_opcode :            u8,
    keycodes_per_modifier :   u8,
    length :                  u16
}


pub struct set_modifier_mapping_reply {
    response_type :   u8,
    status :          u8,
    sequence :        u16,
    length :          u32
}


pub struct get_modifier_mapping_cookie {
    sequence : c_uint
}


pub struct get_modifier_mapping_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16
}


pub struct get_modifier_mapping_reply {
    response_type :           u8,
    keycodes_per_modifier :   u8,
    sequence :                u16,
    length :                  u32,
    pad0 :                    [u8,..24]
}



pub struct no_operation_request {
    major_opcode :   u8,
    pad0 :           u8,
    length :         u16
}

pub extern "C" {

/**
 * Get the next element of the iterator
 * @param i Pointer to a char2b_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(char2b)
 *
 *
 */
unsafe fn xcb_char2b_next (i:*char2b_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An char2b_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_char2b_end (i:char2b_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a window_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(window)
 *
 *
 */
unsafe fn xcb_window_next (i:*window_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An window_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_window_end (i:window_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a pixmap_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(pixmap)
 *
 *
 */
unsafe fn xcb_pixmap_next (i:*pixmap_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An pixmap_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_pixmap_end (i:pixmap_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a cursor_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(cursor)
 *
 *
 */
unsafe fn xcb_cursor_next (i:*cursor_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An cursor_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_cursor_end (i:cursor_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a font_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(font)
 *
 *
 */
unsafe fn xcb_font_next (i:*font_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An font_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_font_end (i:font_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a gcontext_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(gcontext)
 *
 *
 */
unsafe fn xcb_gcontext_next (i:*gcontext_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An gcontext_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_gcontext_end (i:gcontext_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a colormap_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(colormap)
 *
 *
 */
unsafe fn xcb_colormap_next (i:*colormap_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An colormap_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_colormap_end (i:colormap_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a atom_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(atom)
 *
 *
 */
unsafe fn xcb_atom_next (i:*atom_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An atom_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_atom_end (i:atom_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a drawable_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(drawable)
 *
 *
 */
unsafe fn xcb_drawable_next (i:*drawable_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An drawable_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_drawable_end (i:drawable_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a fontable_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(fontable)
 *
 *
 */
unsafe fn xcb_fontable_next (i:*fontable_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An fontable_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_fontable_end (i:fontable_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a visualid_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(visualid)
 *
 *
 */
unsafe fn xcb_visualid_next (i:*visualid_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An visualid_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_visualid_end (i:visualid_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a timestamp_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(timestamp)
 *
 *
 */
unsafe fn xcb_timestamp_next (i:*timestamp_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An timestamp_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_timestamp_end (i:timestamp_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a keysym_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(keysym)
 *
 *
 */
unsafe fn xcb_keysym_next (i:*keysym_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An keysym_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_keysym_end (i:keysym_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a keycode_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(keycode)
 *
 *
 */
unsafe fn xcb_keycode_next (i:*keycode_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An keycode_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_keycode_end (i:keycode_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a button_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(button)
 *
 *
 */
unsafe fn xcb_button_next (i:*button_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An button_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_button_end (i:button_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a point_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(point)
 *
 *
 */
unsafe fn xcb_point_next (i:*point_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An point_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_point_end (i:point_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a rectangle_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(rectangle)
 *
 *
 */
unsafe fn xcb_rectangle_next (i:*rectangle_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An rectangle_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_rectangle_end (i:rectangle_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a arc_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(arc)
 *
 *
 */
unsafe fn xcb_arc_next (i:*arc_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An arc_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_arc_end (i:arc_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a format_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(format)
 *
 *
 */
unsafe fn xcb_format_next (i:*format_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An format_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_format_end (i:format_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a visualtype_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(visualtype)
 *
 *
 */
unsafe fn xcb_visualtype_next (i:*visualtype_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An visualtype_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_visualtype_end (i:visualtype_iterator) -> generic_iterator;

unsafe fn xcb_depth_sizeof (_buffer :  *c_void) -> c_int;

unsafe fn xcb_depth_visuals (R : *depth) -> *visualtype;


unsafe fn xcb_depth_visuals_length (R : *depth) -> c_int;

unsafe fn xcb_depth_visuals_iterator (R : *depth) -> visualtype_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a depth_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(depth)
 *
 *
 */
unsafe fn xcb_depth_next (i:*depth_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An depth_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_depth_end (i:depth_iterator) -> generic_iterator;

unsafe fn xcb_screen_sizeof (_buffer :  *c_void) -> c_int;


unsafe fn xcb_screen_allowed_depths_length (R : *screen) -> c_int;

unsafe fn xcb_screen_allowed_depths_iterator (R : *screen) -> depth_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a screen_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(screen)
 *
 *
 */
unsafe fn xcb_screen_next (i:*screen_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An screen_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_screen_end (i:screen_iterator) -> generic_iterator;

unsafe fn xcb_setup_request_sizeof (_buffer :  *c_void) -> c_int;

unsafe fn xcb_setup_request_authorization_protocol_name (R : *setup_request) -> *c_char;


unsafe fn xcb_setup_request_authorization_protocol_name_length (R : *setup_request) -> c_int;


unsafe fn xcb_setup_request_authorization_protocol_name_end (R : *setup_request) -> generic_iterator;

unsafe fn xcb_setup_request_authorization_protocol_data (R : *setup_request) -> *c_char;


unsafe fn xcb_setup_request_authorization_protocol_data_length (R : *setup_request) -> c_int;


unsafe fn xcb_setup_request_authorization_protocol_data_end (R : *setup_request) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a setup_request_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(setup_request)
 *
 *
 */
unsafe fn xcb_setup_request_next (i:*setup_request_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An setup_request_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_setup_request_end (i:setup_request_iterator) -> generic_iterator;

unsafe fn xcb_setup_failed_sizeof (_buffer :  *c_void) -> c_int;

unsafe fn xcb_setup_failed_reason (R : *setup_failed) -> *c_char;


unsafe fn xcb_setup_failed_reason_length (R : *setup_failed) -> c_int;


unsafe fn xcb_setup_failed_reason_end (R : *setup_failed) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a setup_failed_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(setup_failed)
 *
 *
 */
unsafe fn xcb_setup_failed_next (i:*setup_failed_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An setup_failed_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_setup_failed_end (i:setup_failed_iterator) -> generic_iterator;

unsafe fn xcb_setup_authenticate_sizeof (_buffer :  *c_void) -> c_int;

unsafe fn xcb_setup_authenticate_reason (R : *setup_authenticate) -> *c_char;


unsafe fn xcb_setup_authenticate_reason_length (R : *setup_authenticate) -> c_int;


unsafe fn xcb_setup_authenticate_reason_end (R : *setup_authenticate) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a setup_authenticate_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(setup_authenticate)
 *
 *
 */
unsafe fn xcb_setup_authenticate_next (i:*setup_authenticate_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An setup_authenticate_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_setup_authenticate_end (i:setup_authenticate_iterator) -> generic_iterator;

unsafe fn xcb_setup_sizeof (_buffer :  *c_void) -> c_int;

unsafe fn xcb_setup_vendor (R : *setup) -> *c_char;


unsafe fn xcb_setup_vendor_length (R : *setup) -> c_int;


unsafe fn xcb_setup_vendor_end (R : *setup) -> generic_iterator;

unsafe fn xcb_setup_pixmap_formats (R : *setup) -> *format;


unsafe fn xcb_setup_pixmap_formats_length (R : *setup) -> c_int;

unsafe fn xcb_setup_pixmap_formats_iterator (R : *setup) -> format_iterator;


unsafe fn xcb_setup_roots_length (R : *setup) -> c_int;

unsafe fn xcb_setup_roots_iterator (R : *setup) -> screen_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a setup_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(setup)
 *
 *
 */
unsafe fn xcb_setup_next (i:*setup_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An setup_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_setup_end (i:setup_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a client_message_data_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(client_message_data)
 *
 *
 */
unsafe fn xcb_client_message_data_next (i:*client_message_data_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An client_message_data_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_client_message_data_end (i:client_message_data_iterator) -> generic_iterator;

unsafe fn xcb_create_window_sizeof (_buffer :  *c_void) -> c_int;

/**
 * Creates a window
 *
 * @param c The connection
 * @param depth Specifies the new window's depth (TODO: what unit?).
 * \n
 * The special value `XCB_COPY_FROM_PARENT` means the depth is taken from the
 * \a parent window.
 * @param wid The ID with which you will refer to the new window, created by
 * `xcb_generate_id`.
 * @param parent The parent window of the new window.
 * @param x The X coordinate of the new window.
 * @param y The Y coordinate of the new window.
 * @param width The width of the new window.
 * @param height The height of the new window.
 * @param border_width TODO:
 * \n
 * Must be zero if the \a class is `InputOnly` or a `xcb_match_error_t` occurs.
 * @param class A bitmask of #window_class values.
 * @param class \n
 * @param visual Specifies the id for the new window's visual.
 * \n
 * The special value `XCB_COPY_FROM_PARENT` means the visual is taken from the
 * \a parent window.
 * @return A cookie
 *
 * Creates an unmapped window as child of the specified \a parent window. A
 * CreateNotify event will be generated. The new window is placed on top in the
 * stacking order with respect to siblings.
 * 
 * The coordinate system has the X axis horizontal and the Y axis vertical with
 * the origin [0, 0] at the upper-left corner. Coordinates are integral, in terms
 * of pixels, and coincide with pixel centers. Each window and pixmap has its own
 * coordinate system. For a window, the origin is inside the border at the inside,
 * upper-left corner.
 * 
 * The created window is not yet displayed (mapped), call `xcb_map_window` to
 * display it.
 * 
 * The created window will initially use the same cursor as its parent.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
unsafe fn xcb_create_window_checked (c : *connection,
                                     depth :  u8,
                                     wid :  window,
                                     parent :  window,
                                     x :  i16,
                                     y :  i16,
                                     width :  u16,
                                     height :  u16,
                                     border_width :  u16,
                                     class :  u16,
                                     visual :  visualid,
                                     value_mask :  u32,
                                     value_list : *u32) -> void_cookie;

/**
 * Creates a window
 *
 * @param c The connection
 * @param depth Specifies the new window's depth (TODO: what unit?).
 * \n
 * The special value `XCB_COPY_FROM_PARENT` means the depth is taken from the
 * \a parent window.
 * @param wid The ID with which you will refer to the new window, created by
 * `xcb_generate_id`.
 * @param parent The parent window of the new window.
 * @param x The X coordinate of the new window.
 * @param y The Y coordinate of the new window.
 * @param width The width of the new window.
 * @param height The height of the new window.
 * @param border_width TODO:
 * \n
 * Must be zero if the \a class is `InputOnly` or a `xcb_match_error_t` occurs.
 * @param class A bitmask of #window_class values.
 * @param class \n
 * @param visual Specifies the id for the new window's visual.
 * \n
 * The special value `XCB_COPY_FROM_PARENT` means the visual is taken from the
 * \a parent window.
 * @return A cookie
 *
 * Creates an unmapped window as child of the specified \a parent window. A
 * CreateNotify event will be generated. The new window is placed on top in the
 * stacking order with respect to siblings.
 * 
 * The coordinate system has the X axis horizontal and the Y axis vertical with
 * the origin [0, 0] at the upper-left corner. Coordinates are integral, in terms
 * of pixels, and coincide with pixel centers. Each window and pixmap has its own
 * coordinate system. For a window, the origin is inside the border at the inside,
 * upper-left corner.
 * 
 * The created window is not yet displayed (mapped), call `xcb_map_window` to
 * display it.
 * 
 * The created window will initially use the same cursor as its parent.
 * 
 */
unsafe fn xcb_create_window (c : *connection,
                             depth :  u8,
                             wid :  window,
                             parent :  window,
                             x :  i16,
                             y :  i16,
                             width :  u16,
                             height :  u16,
                             border_width :  u16,
                             class :  u16,
                             visual :  visualid,
                             value_mask :  u32,
                             value_list : *u32) -> void_cookie;

unsafe fn xcb_change_window_attributes_sizeof (_buffer :  *c_void) -> c_int;

/**
 * change window attributes
 *
 * @param c The connection
 * @param window The window to change.
 * @param value_mask \n
 * @param value_list Values for each of the attributes specified in the bitmask \a value_mask. The
 * order has to correspond to the order of possible \a value_mask bits. See the
 * example.
 * @return A cookie
 *
 * Changes the attributes specified by \a value_mask for the specified \a window.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
unsafe fn xcb_change_window_attributes_checked (c : *connection,
                                                window :  window,
                                                value_mask :  u32,
                                                value_list : *u32) -> void_cookie;

/**
 * change window attributes
 *
 * @param c The connection
 * @param window The window to change.
 * @param value_mask \n
 * @param value_list Values for each of the attributes specified in the bitmask \a value_mask. The
 * order has to correspond to the order of possible \a value_mask bits. See the
 * example.
 * @return A cookie
 *
 * Changes the attributes specified by \a value_mask for the specified \a window.
 * 
 */
unsafe fn xcb_change_window_attributes (c : *connection,
                                        window :  window,
                                        value_mask :  u32,
                                        value_list : *u32) -> void_cookie;

/**
 * Gets window attributes
 *
 * @param c The connection
 * @param window The window to get the attributes from.
 * @return A cookie
 *
 * Gets the current attributes for the specified \a window.
 * 
 */
unsafe fn xcb_get_window_attributes (c : *connection,
                                     window :  window) -> get_window_attributes_cookie;

/**
 * Gets window attributes
 *
 * @param c The connection
 * @param window The window to get the attributes from.
 * @return A cookie
 *
 * Gets the current attributes for the specified \a window.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
unsafe fn xcb_get_window_attributes_unchecked (c : *connection,
                                               window :  window) -> get_window_attributes_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_get_window_attributes_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_get_window_attributes_reply (c : *connection,
                                           cookie : get_window_attributes_cookie,
                                           e : **generic_error) -> *get_window_attributes_reply;

/**
 * Destroys a window
 *
 * @param c The connection
 * @param window The window to destroy.
 * @return A cookie
 *
 * Destroys the specified window and all of its subwindows. A DestroyNotify event
 * is generated for each destroyed window (a DestroyNotify event is first generated
 * for any given window's inferiors). If the window was mapped, it will be
 * automatically unmapped before destroying.
 * 
 * Calling DestroyWindow on the root window will do nothing.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
unsafe fn xcb_destroy_window_checked (c : *connection,
                                      window :  window) -> void_cookie;

/**
 * Destroys a window
 *
 * @param c The connection
 * @param window The window to destroy.
 * @return A cookie
 *
 * Destroys the specified window and all of its subwindows. A DestroyNotify event
 * is generated for each destroyed window (a DestroyNotify event is first generated
 * for any given window's inferiors). If the window was mapped, it will be
 * automatically unmapped before destroying.
 * 
 * Calling DestroyWindow on the root window will do nothing.
 * 
 */
unsafe fn xcb_destroy_window (c : *connection,
                              window :  window) -> void_cookie;

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
unsafe fn xcb_destroy_subwindows_checked (c : *connection,
                                          window :  window) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_destroy_subwindows (c : *connection,
                                  window :  window) -> void_cookie;

/**
 * Changes a client's save set
 *
 * @param c The connection
 * @param mode A bitmask of #set_mode values.
 * @param mode Insert to add the specified window to the save set or Delete to delete it from the save set.
 * @param window The window to add or delete to/from your save set.
 * @return A cookie
 *
 * TODO: explain what the save set is for.
 * 
 * This function either adds or removes the specified window to the client's (your
 * application's) save set.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
unsafe fn xcb_change_save_set_checked (c : *connection,
                                       mode :  u8,
                                       window :  window) -> void_cookie;

/**
 * Changes a client's save set
 *
 * @param c The connection
 * @param mode A bitmask of #set_mode values.
 * @param mode Insert to add the specified window to the save set or Delete to delete it from the save set.
 * @param window The window to add or delete to/from your save set.
 * @return A cookie
 *
 * TODO: explain what the save set is for.
 * 
 * This function either adds or removes the specified window to the client's (your
 * application's) save set.
 * 
 */
unsafe fn xcb_change_save_set (c : *connection,
                               mode :  u8,
                               window :  window) -> void_cookie;

/**
 * Reparents a window
 *
 * @param c The connection
 * @param window The window to reparent.
 * @param parent The new parent of the window.
 * @param x The X position of the window within its new parent.
 * @param y The Y position of the window within its new parent.
 * @return A cookie
 *
 * Makes the specified window a child of the specified parent window. If the
 * window is mapped, it will automatically be unmapped before reparenting and
 * re-mapped after reparenting. The window is placed in the stacking order on top
 * with respect to sibling windows.
 * 
 * After reparenting, a ReparentNotify event is generated.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
unsafe fn xcb_reparent_window_checked (c : *connection,
                                       window :  window,
                                       parent :  window,
                                       x :  i16,
                                       y :  i16) -> void_cookie;

/**
 * Reparents a window
 *
 * @param c The connection
 * @param window The window to reparent.
 * @param parent The new parent of the window.
 * @param x The X position of the window within its new parent.
 * @param y The Y position of the window within its new parent.
 * @return A cookie
 *
 * Makes the specified window a child of the specified parent window. If the
 * window is mapped, it will automatically be unmapped before reparenting and
 * re-mapped after reparenting. The window is placed in the stacking order on top
 * with respect to sibling windows.
 * 
 * After reparenting, a ReparentNotify event is generated.
 * 
 */
unsafe fn xcb_reparent_window (c : *connection,
                               window :  window,
                               parent :  window,
                               x :  i16,
                               y :  i16) -> void_cookie;

/**
 * Makes a window visible
 *
 * @param c The connection
 * @param window The window to make visible.
 * @return A cookie
 *
 * Maps the specified window. This means making the window visible (as long as its
 * parent is visible).
 * 
 * This MapWindow request will be translated to a MapRequest request if a window
 * manager is running. The window manager then decides to either map the window or
 * not. Set the override-redirect window attribute to true if you want to bypass
 * this mechanism.
 * 
 * If the window manager decides to map the window (or if no window manager is
 * running), a MapNotify event is generated.
 * 
 * If the window becomes viewable and no earlier contents for it are remembered,
 * the X server tiles the window with its background. If the window's background
 * is undefined, the existing screen contents are not altered, and the X server
 * generates zero or more Expose events.
 * 
 * If the window type is InputOutput, an Expose event will be generated when the
 * window becomes visible. The normal response to an Expose event should be to
 * repaint the window.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
unsafe fn xcb_map_window_checked (c : *connection,
                                  window :  window) -> void_cookie;

/**
 * Makes a window visible
 *
 * @param c The connection
 * @param window The window to make visible.
 * @return A cookie
 *
 * Maps the specified window. This means making the window visible (as long as its
 * parent is visible).
 * 
 * This MapWindow request will be translated to a MapRequest request if a window
 * manager is running. The window manager then decides to either map the window or
 * not. Set the override-redirect window attribute to true if you want to bypass
 * this mechanism.
 * 
 * If the window manager decides to map the window (or if no window manager is
 * running), a MapNotify event is generated.
 * 
 * If the window becomes viewable and no earlier contents for it are remembered,
 * the X server tiles the window with its background. If the window's background
 * is undefined, the existing screen contents are not altered, and the X server
 * generates zero or more Expose events.
 * 
 * If the window type is InputOutput, an Expose event will be generated when the
 * window becomes visible. The normal response to an Expose event should be to
 * repaint the window.
 * 
 */
unsafe fn xcb_map_window (c : *connection,
                          window :  window) -> void_cookie;

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
unsafe fn xcb_map_subwindows_checked (c : *connection,
                                      window :  window) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_map_subwindows (c : *connection,
                              window :  window) -> void_cookie;

/**
 * Makes a window invisible
 *
 * @param c The connection
 * @param window The window to make invisible.
 * @return A cookie
 *
 * Unmaps the specified window. This means making the window invisible (and all
 * its child windows).
 * 
 * Unmapping a window leads to the `UnmapNotify` event being generated. Also,
 * `Expose` events are generated for formerly obscured windows.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
unsafe fn xcb_unmap_window_checked (c : *connection,
                                    window :  window) -> void_cookie;

/**
 * Makes a window invisible
 *
 * @param c The connection
 * @param window The window to make invisible.
 * @return A cookie
 *
 * Unmaps the specified window. This means making the window invisible (and all
 * its child windows).
 * 
 * Unmapping a window leads to the `UnmapNotify` event being generated. Also,
 * `Expose` events are generated for formerly obscured windows.
 * 
 */
unsafe fn xcb_unmap_window (c : *connection,
                            window :  window) -> void_cookie;

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
unsafe fn xcb_unmap_subwindows_checked (c : *connection,
                                        window :  window) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_unmap_subwindows (c : *connection,
                                window :  window) -> void_cookie;

unsafe fn xcb_configure_window_sizeof (_buffer :  *c_void) -> c_int;

/**
 * Configures window attributes
 *
 * @param c The connection
 * @param window The window to configure.
 * @param value_mask Bitmask of attributes to change.
 * @param value_list New values, corresponding to the attributes in value_mask. The order has to
 * correspond to the order of possible \a value_mask bits. See the example.
 * @return A cookie
 *
 * Configures a window's size, position, border width and stacking order.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
unsafe fn xcb_configure_window_checked (c : *connection,
                                        window :  window,
                                        value_mask :  u16,
                                        value_list : *u32) -> void_cookie;

/**
 * Configures window attributes
 *
 * @param c The connection
 * @param window The window to configure.
 * @param value_mask Bitmask of attributes to change.
 * @param value_list New values, corresponding to the attributes in value_mask. The order has to
 * correspond to the order of possible \a value_mask bits. See the example.
 * @return A cookie
 *
 * Configures a window's size, position, border width and stacking order.
 * 
 */
unsafe fn xcb_configure_window (c : *connection,
                                window :  window,
                                value_mask :  u16,
                                value_list : *u32) -> void_cookie;

/**
 * Change window stacking order
 *
 * @param c The connection
 * @param direction A bitmask of #circulate values.
 * @param direction \n
 * @param window The window to raise/lower (depending on \a direction).
 * @return A cookie
 *
 * If \a direction is `XCB_CIRCULATE_RAISE_LOWEST`, the lowest mapped child (if
 * any) will be raised to the top of the stack.
 * 
 * If \a direction is `XCB_CIRCULATE_LOWER_HIGHEST`, the highest mapped child will
 * be lowered to the bottom of the stack.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
unsafe fn xcb_circulate_window_checked (c : *connection,
                                        direction :  u8,
                                        window :  window) -> void_cookie;

/**
 * Change window stacking order
 *
 * @param c The connection
 * @param direction A bitmask of #circulate values.
 * @param direction \n
 * @param window The window to raise/lower (depending on \a direction).
 * @return A cookie
 *
 * If \a direction is `XCB_CIRCULATE_RAISE_LOWEST`, the lowest mapped child (if
 * any) will be raised to the top of the stack.
 * 
 * If \a direction is `XCB_CIRCULATE_LOWER_HIGHEST`, the highest mapped child will
 * be lowered to the bottom of the stack.
 * 
 */
unsafe fn xcb_circulate_window (c : *connection,
                                direction :  u8,
                                window :  window) -> void_cookie;

/**
 * Get current window geometry
 *
 * @param c The connection
 * @param drawable The drawable (`Window` or `Pixmap`) of which the geometry will be received.
 * @return A cookie
 *
 * Gets the current geometry of the specified drawable (either `Window` or `Pixmap`).
 * 
 */
unsafe fn xcb_get_geometry (c : *connection,
                            drawable :  drawable) -> get_geometry_cookie;

/**
 * Get current window geometry
 *
 * @param c The connection
 * @param drawable The drawable (`Window` or `Pixmap`) of which the geometry will be received.
 * @return A cookie
 *
 * Gets the current geometry of the specified drawable (either `Window` or `Pixmap`).
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
unsafe fn xcb_get_geometry_unchecked (c : *connection,
                                      drawable :  drawable) -> get_geometry_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_get_geometry_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_get_geometry_reply (c : *connection,
                                  cookie : get_geometry_cookie,
                                  e : **generic_error) -> *get_geometry_reply;

unsafe fn xcb_query_tree_sizeof (_buffer :  *c_void) -> c_int;

/**
 * query the window tree
 *
 * @param c The connection
 * @param window The \a window to query.
 * @return A cookie
 *
 * Gets the root window ID, parent window ID and list of children windows for the
 * specified \a window. The children are listed in bottom-to-top stacking order.
 * 
 */
unsafe fn xcb_query_tree (c : *connection,
                          window :  window) -> query_tree_cookie;

/**
 * query the window tree
 *
 * @param c The connection
 * @param window The \a window to query.
 * @return A cookie
 *
 * Gets the root window ID, parent window ID and list of children windows for the
 * specified \a window. The children are listed in bottom-to-top stacking order.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
unsafe fn xcb_query_tree_unchecked (c : *connection,
                                    window :  window) -> query_tree_cookie;

unsafe fn xcb_query_tree_children (R : *query_tree_reply) -> *window;


unsafe fn xcb_query_tree_children_length (R : *query_tree_reply) -> c_int;


unsafe fn xcb_query_tree_children_end (R : *query_tree_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_query_tree_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_query_tree_reply (c : *connection,
                                cookie : query_tree_cookie,
                                e : **generic_error) -> *query_tree_reply;

unsafe fn xcb_intern_atom_sizeof (_buffer :  *c_void) -> c_int;

/**
 * Get atom identifier by name
 *
 * @param c The connection
 * @param only_if_exists Return a valid atom id only if the atom already exists.
 * @param name_len The length of the following \a name.
 * @param name The name of the atom.
 * @return A cookie
 *
 * Retrieves the identifier (xcb_atom_t TODO) for the atom with the specified
 * name. Atoms are used in protocols like EWMH, for example to store window titles
 * (`_NET_WM_NAME` atom) as property of a window.
 * 
 * If \a only_if_exists is 0, the atom will be created if it does not already exist.
 * If \a only_if_exists is 1, `XCB_ATOM_NONE` will be returned if the atom does
 * not yet exist.
 * 
 */
unsafe fn xcb_intern_atom (c : *connection,
                           only_if_exists :  u8,
                           name_len :  u16,
                           name : *c_char) -> intern_atom_cookie;

/**
 * Get atom identifier by name
 *
 * @param c The connection
 * @param only_if_exists Return a valid atom id only if the atom already exists.
 * @param name_len The length of the following \a name.
 * @param name The name of the atom.
 * @return A cookie
 *
 * Retrieves the identifier (xcb_atom_t TODO) for the atom with the specified
 * name. Atoms are used in protocols like EWMH, for example to store window titles
 * (`_NET_WM_NAME` atom) as property of a window.
 * 
 * If \a only_if_exists is 0, the atom will be created if it does not already exist.
 * If \a only_if_exists is 1, `XCB_ATOM_NONE` will be returned if the atom does
 * not yet exist.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
unsafe fn xcb_intern_atom_unchecked (c : *connection,
                                     only_if_exists :  u8,
                                     name_len :  u16,
                                     name : *c_char) -> intern_atom_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_intern_atom_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_intern_atom_reply (c : *connection,
                                 cookie : intern_atom_cookie,
                                 e : **generic_error) -> *intern_atom_reply;

unsafe fn xcb_get_atom_name_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_get_atom_name (c : *connection,
                             atom :  atom) -> get_atom_name_cookie;

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
unsafe fn xcb_get_atom_name_unchecked (c : *connection,
                                       atom :  atom) -> get_atom_name_cookie;

unsafe fn xcb_get_atom_name_name (R : *get_atom_name_reply) -> *c_char;


unsafe fn xcb_get_atom_name_name_length (R : *get_atom_name_reply) -> c_int;


unsafe fn xcb_get_atom_name_name_end (R : *get_atom_name_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_get_atom_name_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_get_atom_name_reply (c : *connection,
                                   cookie : get_atom_name_cookie,
                                   e : **generic_error) -> *get_atom_name_reply;

unsafe fn xcb_change_property_sizeof (_buffer :  *c_void) -> c_int;

/**
 * Changes a window property
 *
 * @param c The connection
 * @param mode A bitmask of #prop_mode values.
 * @param mode \n
 * @param window The window whose property you want to change.
 * @param property The property you want to change (an atom).
 * @param type_ The type of the property you want to change (an atom).
 * @param format Specifies whether the data should be viewed as a list of 8-bit, 16-bit or
 * 32-bit quantities. Possible values are 8, 16 and 32. This information allows
 * the X server to correctly perform byte-swap operations as necessary.
 * @param data_len Specifies the number of elements (see \a format).
 * @param data The property data.
 * @return A cookie
 *
 * Sets or updates a property on the specified \a window. Properties are for
 * example the window title (`WM_NAME`) or its minimum size (`WM_NORMAL_HINTS`).
 * Protocols such as EWMH also use properties - for example EWMH defines the
 * window title, encoded as UTF-8 string, in the `_NET_WM_NAME` property.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
unsafe fn xcb_change_property_checked (c : *connection,
                                       mode :  u8,
                                       window :  window,
                                       property :  atom,
                                       type_ :  atom,
                                       format :  u8,
                                       data_len :  u32,
                                       data : *c_void) -> void_cookie;

/**
 * Changes a window property
 *
 * @param c The connection
 * @param mode A bitmask of #prop_mode values.
 * @param mode \n
 * @param window The window whose property you want to change.
 * @param property The property you want to change (an atom).
 * @param type_ The type of the property you want to change (an atom).
 * @param format Specifies whether the data should be viewed as a list of 8-bit, 16-bit or
 * 32-bit quantities. Possible values are 8, 16 and 32. This information allows
 * the X server to correctly perform byte-swap operations as necessary.
 * @param data_len Specifies the number of elements (see \a format).
 * @param data The property data.
 * @return A cookie
 *
 * Sets or updates a property on the specified \a window. Properties are for
 * example the window title (`WM_NAME`) or its minimum size (`WM_NORMAL_HINTS`).
 * Protocols such as EWMH also use properties - for example EWMH defines the
 * window title, encoded as UTF-8 string, in the `_NET_WM_NAME` property.
 * 
 */
unsafe fn xcb_change_property (c : *connection,
                               mode :  u8,
                               window :  window,
                               property :  atom,
                               type_ :  atom,
                               format :  u8,
                               data_len :  u32,
                               data : *c_void) -> void_cookie;

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
unsafe fn xcb_delete_property_checked (c : *connection,
                                       window :  window,
                                       property :  atom) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_delete_property (c : *connection,
                               window :  window,
                               property :  atom) -> void_cookie;

unsafe fn xcb_get_property_sizeof (_buffer :  *c_void) -> c_int;

/**
 * Gets a window property
 *
 * @param c The connection
 * @param delete Whether the property should actually be deleted. For deleting a property, the
 * specified `type` has to match the actual property type.
 * @param window The window whose property you want to get.
 * @param property The property you want to get (an atom).
 * @param type_ The type of the property you want to get (an atom).
 * @param long_offset Specifies the offset (in 32-bit multiples) in the specified property where the
 * data is to be retrieved.
 * @param long_length Specifies how many 32-bit multiples of data should be retrieved (e.g. if you
 * set \a long_length to 4, you will receive 16 bytes of data).
 * @return A cookie
 *
 * Gets the specified \a property from the specified \a window. Properties are for
 * example the window title (`WM_NAME`) or its minimum size (`WM_NORMAL_HINTS`).
 * Protocols such as EWMH also use properties - for example EWMH defines the
 * window title, encoded as UTF-8 string, in the `_NET_WM_NAME` property.
 * 
 * TODO: talk about `type`
 * 
 * TODO: talk about \a delete
 * 
 * TODO: talk about the offset/length thing. what's a valid use case?
 * 
 */
unsafe fn xcb_get_property (c : *connection,
                            delete :  u8,
                            window :  window,
                            property :  atom,
                            type_ :  atom,
                            long_offset :  u32,
                            long_length :  u32) -> get_property_cookie;

/**
 * Gets a window property
 *
 * @param c The connection
 * @param delete Whether the property should actually be deleted. For deleting a property, the
 * specified `type` has to match the actual property type.
 * @param window The window whose property you want to get.
 * @param property The property you want to get (an atom).
 * @param type_ The type of the property you want to get (an atom).
 * @param long_offset Specifies the offset (in 32-bit multiples) in the specified property where the
 * data is to be retrieved.
 * @param long_length Specifies how many 32-bit multiples of data should be retrieved (e.g. if you
 * set \a long_length to 4, you will receive 16 bytes of data).
 * @return A cookie
 *
 * Gets the specified \a property from the specified \a window. Properties are for
 * example the window title (`WM_NAME`) or its minimum size (`WM_NORMAL_HINTS`).
 * Protocols such as EWMH also use properties - for example EWMH defines the
 * window title, encoded as UTF-8 string, in the `_NET_WM_NAME` property.
 * 
 * TODO: talk about `type`
 * 
 * TODO: talk about \a delete
 * 
 * TODO: talk about the offset/length thing. what's a valid use case?
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
unsafe fn xcb_get_property_unchecked (c : *connection,
                                      delete :  u8,
                                      window :  window,
                                      property :  atom,
                                      type_ :  atom,
                                      long_offset :  u32,
                                      long_length :  u32) -> get_property_cookie;

unsafe fn xcb_get_property_value (R : *get_property_reply) -> *c_void;


unsafe fn xcb_get_property_value_length (R : *get_property_reply) -> c_int;


unsafe fn xcb_get_property_value_end (R : *get_property_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_get_property_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_get_property_reply (c : *connection,
                                  cookie : get_property_cookie,
                                  e : **generic_error) -> *get_property_reply;

unsafe fn xcb_list_properties_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_list_properties (c : *connection,
                               window :  window) -> list_properties_cookie;

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
unsafe fn xcb_list_properties_unchecked (c : *connection,
                                         window :  window) -> list_properties_cookie;

unsafe fn xcb_list_properties_atoms (R : *list_properties_reply) -> *atom;


unsafe fn xcb_list_properties_atoms_length (R : *list_properties_reply) -> c_int;


unsafe fn xcb_list_properties_atoms_end (R : *list_properties_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_list_properties_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_list_properties_reply (c : *connection,
                                     cookie : list_properties_cookie,
                                     e : **generic_error) -> *list_properties_reply;

/**
 * Sets the owner of a selection
 *
 * @param c The connection
 * @param owner The new owner of the selection.
 * \n
 * The special value `XCB_NONE` means that the selection will have no owner.
 * @param selection The selection.
 * @param time Timestamp to avoid race conditions when running X over the network.
 * \n
 * The selection will not be changed if \a time is earlier than the current
 * last-change time of the \a selection or is later than the current X server time.
 * Otherwise, the last-change time is set to the specified time.
 * \n
 * The special value `XCB_CURRENT_TIME` will be replaced with the current server
 * time.
 * @return A cookie
 *
 * Makes `window` the owner of the selection \a selection and updates the
 * last-change time of the specified selection.
 * 
 * TODO: briefly explain what a selection is.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
unsafe fn xcb_set_selection_owner_checked (c : *connection,
                                           owner :  window,
                                           selection :  atom,
                                           time :  timestamp) -> void_cookie;

/**
 * Sets the owner of a selection
 *
 * @param c The connection
 * @param owner The new owner of the selection.
 * \n
 * The special value `XCB_NONE` means that the selection will have no owner.
 * @param selection The selection.
 * @param time Timestamp to avoid race conditions when running X over the network.
 * \n
 * The selection will not be changed if \a time is earlier than the current
 * last-change time of the \a selection or is later than the current X server time.
 * Otherwise, the last-change time is set to the specified time.
 * \n
 * The special value `XCB_CURRENT_TIME` will be replaced with the current server
 * time.
 * @return A cookie
 *
 * Makes `window` the owner of the selection \a selection and updates the
 * last-change time of the specified selection.
 * 
 * TODO: briefly explain what a selection is.
 * 
 */
unsafe fn xcb_set_selection_owner (c : *connection,
                                   owner :  window,
                                   selection :  atom,
                                   time :  timestamp) -> void_cookie;

/**
 * Gets the owner of a selection
 *
 * @param c The connection
 * @param selection The selection.
 * @return A cookie
 *
 * Gets the owner of the specified selection.
 * 
 * TODO: briefly explain what a selection is.
 * 
 */
unsafe fn xcb_get_selection_owner (c : *connection,
                                   selection :  atom) -> get_selection_owner_cookie;

/**
 * Gets the owner of a selection
 *
 * @param c The connection
 * @param selection The selection.
 * @return A cookie
 *
 * Gets the owner of the specified selection.
 * 
 * TODO: briefly explain what a selection is.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
unsafe fn xcb_get_selection_owner_unchecked (c : *connection,
                                             selection :  atom) -> get_selection_owner_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_get_selection_owner_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_get_selection_owner_reply (c : *connection,
                                         cookie : get_selection_owner_cookie,
                                         e : **generic_error) -> *get_selection_owner_reply;

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
unsafe fn xcb_convert_selection_checked (c : *connection,
                                         requestor :  window,
                                         selection :  atom,
                                         target :  atom,
                                         property :  atom,
                                         time :  timestamp) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_convert_selection (c : *connection,
                                 requestor :  window,
                                 selection :  atom,
                                 target :  atom,
                                 property :  atom,
                                 time :  timestamp) -> void_cookie;

/**
 * send an event
 *
 * @param c The connection
 * @param propagate If \a propagate is true and no clients have selected any event on \a destination,
 * the destination is replaced with the closest ancestor of \a destination for
 * which some client has selected a type in \a event_mask and for which no
 * intervening window has that type in its do-not-propagate-mask. If no such
 * window exists or if the window is an ancestor of the focus window and
 * `InputFocus` was originally specified as the destination, the event is not sent
 * to any clients. Otherwise, the event is reported to every client selecting on
 * the final destination any of the types specified in \a event_mask.
 * @param destination The window to send this event to. Every client which selects any event within
 * \a event_mask on \a destination will get the event.
 * \n
 * The special value `XCB_SEND_EVENT_DEST_POINTER_WINDOW` refers to the window
 * that contains the mouse pointer.
 * \n
 * The special value `XCB_SEND_EVENT_DEST_ITEM_FOCUS` refers to the window which
 * has the keyboard focus.
 * @param event_mask Event_mask for determining which clients should receive the specified event.
 * See \a destination and \a propagate.
 * @param event The event to send to the specified \a destination.
 * @return A cookie
 *
 * Identifies the \a destination window, determines which clients should receive
 * the specified event and ignores any active grabs.
 * 
 * The \a event must be one of the core events or an event defined by an extension,
 * so that the X server can correctly byte-swap the contents as necessary. The
 * contents of \a event are otherwise unaltered and unchecked except for the
 * `send_event` field which is forced to 'true'.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
unsafe fn xcb_send_event_checked (c : *connection,
                                  propagate :  u8,
                                  destination :  window,
                                  event_mask :  u32,
                                  event : *c_char) -> void_cookie;

/**
 * send an event
 *
 * @param c The connection
 * @param propagate If \a propagate is true and no clients have selected any event on \a destination,
 * the destination is replaced with the closest ancestor of \a destination for
 * which some client has selected a type in \a event_mask and for which no
 * intervening window has that type in its do-not-propagate-mask. If no such
 * window exists or if the window is an ancestor of the focus window and
 * `InputFocus` was originally specified as the destination, the event is not sent
 * to any clients. Otherwise, the event is reported to every client selecting on
 * the final destination any of the types specified in \a event_mask.
 * @param destination The window to send this event to. Every client which selects any event within
 * \a event_mask on \a destination will get the event.
 * \n
 * The special value `XCB_SEND_EVENT_DEST_POINTER_WINDOW` refers to the window
 * that contains the mouse pointer.
 * \n
 * The special value `XCB_SEND_EVENT_DEST_ITEM_FOCUS` refers to the window which
 * has the keyboard focus.
 * @param event_mask Event_mask for determining which clients should receive the specified event.
 * See \a destination and \a propagate.
 * @param event The event to send to the specified \a destination.
 * @return A cookie
 *
 * Identifies the \a destination window, determines which clients should receive
 * the specified event and ignores any active grabs.
 * 
 * The \a event must be one of the core events or an event defined by an extension,
 * so that the X server can correctly byte-swap the contents as necessary. The
 * contents of \a event are otherwise unaltered and unchecked except for the
 * `send_event` field which is forced to 'true'.
 * 
 */
unsafe fn xcb_send_event (c : *connection,
                          propagate :  u8,
                          destination :  window,
                          event_mask :  u32,
                          event : *c_char) -> void_cookie;

/**
 * Grab the pointer
 *
 * @param c The connection
 * @param owner_events If 1, the \a grab_window will still get the pointer events. If 0, events are not
 * reported to the \a grab_window.
 * @param grab_window Specifies the window on which the pointer should be grabbed.
 * @param event_mask Specifies which pointer events are reported to the client.
 * \n
 * TODO: which values?
 * @param pointer_mode A bitmask of #grab_mode values.
 * @param pointer_mode \n
 * @param keyboard_mode A bitmask of #grab_mode values.
 * @param keyboard_mode \n
 * @param confine_to Specifies the window to confine the pointer in (the user will not be able to
 * move the pointer out of that window).
 * \n
 * The special value `XCB_NONE` means don't confine the pointer.
 * @param cursor Specifies the cursor that should be displayed or `XCB_NONE` to not change the
 * cursor.
 * @param time The time argument allows you to avoid certain circumstances that come up if
 * applications take a long time to respond or if there are long network delays.
 * Consider a situation where you have two applications, both of which normally
 * grab the pointer when clicked on. If both applications specify the timestamp
 * from the event, the second application may wake up faster and successfully grab
 * the pointer before the first application. The first application then will get
 * an indication that the other application grabbed the pointer before its request
 * was processed.
 * \n
 * The special value `XCB_CURRENT_TIME` will be replaced with the current server
 * time.
 * @return A cookie
 *
 * Actively grabs control of the pointer. Further pointer events are reported only to the grabbing client. Overrides any active pointer grab by this client.
 * 
 */
unsafe fn xcb_grab_pointer (c : *connection,
                            owner_events :  u8,
                            grab_window :  window,
                            event_mask :  u16,
                            pointer_mode :  u8,
                            keyboard_mode :  u8,
                            confine_to :  window,
                            cursor :  cursor,
                            time :  timestamp) -> grab_pointer_cookie;

/**
 * Grab the pointer
 *
 * @param c The connection
 * @param owner_events If 1, the \a grab_window will still get the pointer events. If 0, events are not
 * reported to the \a grab_window.
 * @param grab_window Specifies the window on which the pointer should be grabbed.
 * @param event_mask Specifies which pointer events are reported to the client.
 * \n
 * TODO: which values?
 * @param pointer_mode A bitmask of #grab_mode values.
 * @param pointer_mode \n
 * @param keyboard_mode A bitmask of #grab_mode values.
 * @param keyboard_mode \n
 * @param confine_to Specifies the window to confine the pointer in (the user will not be able to
 * move the pointer out of that window).
 * \n
 * The special value `XCB_NONE` means don't confine the pointer.
 * @param cursor Specifies the cursor that should be displayed or `XCB_NONE` to not change the
 * cursor.
 * @param time The time argument allows you to avoid certain circumstances that come up if
 * applications take a long time to respond or if there are long network delays.
 * Consider a situation where you have two applications, both of which normally
 * grab the pointer when clicked on. If both applications specify the timestamp
 * from the event, the second application may wake up faster and successfully grab
 * the pointer before the first application. The first application then will get
 * an indication that the other application grabbed the pointer before its request
 * was processed.
 * \n
 * The special value `XCB_CURRENT_TIME` will be replaced with the current server
 * time.
 * @return A cookie
 *
 * Actively grabs control of the pointer. Further pointer events are reported only to the grabbing client. Overrides any active pointer grab by this client.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
unsafe fn xcb_grab_pointer_unchecked (c : *connection,
                                      owner_events :  u8,
                                      grab_window :  window,
                                      event_mask :  u16,
                                      pointer_mode :  u8,
                                      keyboard_mode :  u8,
                                      confine_to :  window,
                                      cursor :  cursor,
                                      time :  timestamp) -> grab_pointer_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_grab_pointer_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_grab_pointer_reply (c : *connection,
                                  cookie : grab_pointer_cookie,
                                  e : **generic_error) -> *grab_pointer_reply;

/**
 * release the pointer
 *
 * @param c The connection
 * @param time Timestamp to avoid race conditions when running X over the network.
 * \n
 * The pointer will not be released if \a time is earlier than the
 * last-pointer-grab time or later than the current X server time.
 * @return A cookie
 *
 * Releases the pointer and any queued events if you actively grabbed the pointer
 * before using `xcb_grab_pointer`, `xcb_grab_button` or within a normal button
 * press.
 * 
 * EnterNotify and LeaveNotify events are generated.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
unsafe fn xcb_ungrab_pointer_checked (c : *connection,
                                      time :  timestamp) -> void_cookie;

/**
 * release the pointer
 *
 * @param c The connection
 * @param time Timestamp to avoid race conditions when running X over the network.
 * \n
 * The pointer will not be released if \a time is earlier than the
 * last-pointer-grab time or later than the current X server time.
 * @return A cookie
 *
 * Releases the pointer and any queued events if you actively grabbed the pointer
 * before using `xcb_grab_pointer`, `xcb_grab_button` or within a normal button
 * press.
 * 
 * EnterNotify and LeaveNotify events are generated.
 * 
 */
unsafe fn xcb_ungrab_pointer (c : *connection,
                              time :  timestamp) -> void_cookie;

/**
 * Grab pointer button(s)
 *
 * @param c The connection
 * @param owner_events If 1, the \a grab_window will still get the pointer events. If 0, events are not
 * reported to the \a grab_window.
 * @param grab_window Specifies the window on which the pointer should be grabbed.
 * @param event_mask Specifies which pointer events are reported to the client.
 * \n
 * TODO: which values?
 * @param pointer_mode A bitmask of #grab_mode values.
 * @param pointer_mode \n
 * @param keyboard_mode A bitmask of #grab_mode values.
 * @param keyboard_mode \n
 * @param confine_to Specifies the window to confine the pointer in (the user will not be able to
 * move the pointer out of that window).
 * \n
 * The special value `XCB_NONE` means don't confine the pointer.
 * @param cursor Specifies the cursor that should be displayed or `XCB_NONE` to not change the
 * cursor.
 * @param button A bitmask of #button_index values.
 * @param button \n
 * @param modifiers The modifiers to grab.
 * \n
 * Using the special value `XCB_MOD_MASK_ANY` means grab the pointer with all
 * possible modifier combinations.
 * @return A cookie
 *
 * This request establishes a passive grab. The pointer is actively grabbed as
 * described in GrabPointer, the last-pointer-grab time is set to the time at
 * which the button was pressed (as transmitted in the ButtonPress event), and the
 * ButtonPress event is reported if all of the following conditions are true:
 * 
 * The pointer is not grabbed and the specified button is logically pressed when
 * the specified modifier keys are logically down, and no other buttons or
 * modifier keys are logically down.
 * 
 * The grab-window contains the pointer.
 * 
 * The confine-to window (if any) is viewable.
 * 
 * A passive grab on the same button/key combination does not exist on any
 * ancestor of grab-window.
 * 
 * The interpretation of the remaining arguments is the same as for GrabPointer.
 * The active grab is terminated automatically when the logical state of the
 * pointer has all buttons released, independent of the logical state of modifier
 * keys. Note that the logical state of a device (as seen by means of the
 * protocol) may lag the physical state if device event processing is frozen. This
 * request overrides all previous passive grabs by the same client on the same
 * button/key combinations on the same window. A modifier of AnyModifier is
 * equivalent to issuing the request for all possible modifier combinations
 * (including the combination of no modifiers). It is not required that all
 * specified modifiers have currently assigned keycodes. A button of AnyButton is
 * equivalent to issuing the request for all possible buttons. Otherwise, it is
 * not required that the button specified currently be assigned to a physical
 * button.
 * 
 * An Access error is generated if some other client has already issued a
 * GrabButton request with the same button/key combination on the same window.
 * When using AnyModifier or AnyButton, the request fails completely (no grabs are
 * established), and an Access error is generated if there is a conflicting grab
 * for any combination. The request has no effect on an active grab.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
unsafe fn xcb_grab_button_checked (c : *connection,
                                   owner_events :  u8,
                                   grab_window :  window,
                                   event_mask :  u16,
                                   pointer_mode :  u8,
                                   keyboard_mode :  u8,
                                   confine_to :  window,
                                   cursor :  cursor,
                                   button :  u8,
                                   modifiers :  u16) -> void_cookie;

/**
 * Grab pointer button(s)
 *
 * @param c The connection
 * @param owner_events If 1, the \a grab_window will still get the pointer events. If 0, events are not
 * reported to the \a grab_window.
 * @param grab_window Specifies the window on which the pointer should be grabbed.
 * @param event_mask Specifies which pointer events are reported to the client.
 * \n
 * TODO: which values?
 * @param pointer_mode A bitmask of #grab_mode values.
 * @param pointer_mode \n
 * @param keyboard_mode A bitmask of #grab_mode values.
 * @param keyboard_mode \n
 * @param confine_to Specifies the window to confine the pointer in (the user will not be able to
 * move the pointer out of that window).
 * \n
 * The special value `XCB_NONE` means don't confine the pointer.
 * @param cursor Specifies the cursor that should be displayed or `XCB_NONE` to not change the
 * cursor.
 * @param button A bitmask of #button_index values.
 * @param button \n
 * @param modifiers The modifiers to grab.
 * \n
 * Using the special value `XCB_MOD_MASK_ANY` means grab the pointer with all
 * possible modifier combinations.
 * @return A cookie
 *
 * This request establishes a passive grab. The pointer is actively grabbed as
 * described in GrabPointer, the last-pointer-grab time is set to the time at
 * which the button was pressed (as transmitted in the ButtonPress event), and the
 * ButtonPress event is reported if all of the following conditions are true:
 * 
 * The pointer is not grabbed and the specified button is logically pressed when
 * the specified modifier keys are logically down, and no other buttons or
 * modifier keys are logically down.
 * 
 * The grab-window contains the pointer.
 * 
 * The confine-to window (if any) is viewable.
 * 
 * A passive grab on the same button/key combination does not exist on any
 * ancestor of grab-window.
 * 
 * The interpretation of the remaining arguments is the same as for GrabPointer.
 * The active grab is terminated automatically when the logical state of the
 * pointer has all buttons released, independent of the logical state of modifier
 * keys. Note that the logical state of a device (as seen by means of the
 * protocol) may lag the physical state if device event processing is frozen. This
 * request overrides all previous passive grabs by the same client on the same
 * button/key combinations on the same window. A modifier of AnyModifier is
 * equivalent to issuing the request for all possible modifier combinations
 * (including the combination of no modifiers). It is not required that all
 * specified modifiers have currently assigned keycodes. A button of AnyButton is
 * equivalent to issuing the request for all possible buttons. Otherwise, it is
 * not required that the button specified currently be assigned to a physical
 * button.
 * 
 * An Access error is generated if some other client has already issued a
 * GrabButton request with the same button/key combination on the same window.
 * When using AnyModifier or AnyButton, the request fails completely (no grabs are
 * established), and an Access error is generated if there is a conflicting grab
 * for any combination. The request has no effect on an active grab.
 * 
 */
unsafe fn xcb_grab_button (c : *connection,
                           owner_events :  u8,
                           grab_window :  window,
                           event_mask :  u16,
                           pointer_mode :  u8,
                           keyboard_mode :  u8,
                           confine_to :  window,
                           cursor :  cursor,
                           button :  u8,
                           modifiers :  u16) -> void_cookie;

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
unsafe fn xcb_ungrab_button_checked (c : *connection,
                                     button :  u8,
                                     grab_window :  window,
                                     modifiers :  u16) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_ungrab_button (c : *connection,
                             button :  u8,
                             grab_window :  window,
                             modifiers :  u16) -> void_cookie;

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
unsafe fn xcb_change_active_pointer_grab_checked (c : *connection,
                                                  cursor :  cursor,
                                                  time :  timestamp,
                                                  event_mask :  u16) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_change_active_pointer_grab (c : *connection,
                                          cursor :  cursor,
                                          time :  timestamp,
                                          event_mask :  u16) -> void_cookie;

/**
 * Grab the keyboard
 *
 * @param c The connection
 * @param owner_events If 1, the \a grab_window will still get the pointer events. If 0, events are not
 * reported to the \a grab_window.
 * @param grab_window Specifies the window on which the pointer should be grabbed.
 * @param time Timestamp to avoid race conditions when running X over the network.
 * \n
 * The special value `XCB_CURRENT_TIME` will be replaced with the current server
 * time.
 * @param pointer_mode A bitmask of #grab_mode values.
 * @param pointer_mode \n
 * @param keyboard_mode A bitmask of #grab_mode values.
 * @param keyboard_mode \n
 * @return A cookie
 *
 * Actively grabs control of the keyboard and generates FocusIn and FocusOut
 * events. Further key events are reported only to the grabbing client.
 * 
 * Any active keyboard grab by this client is overridden. If the keyboard is
 * actively grabbed by some other client, `AlreadyGrabbed` is returned. If
 * \a grab_window is not viewable, `GrabNotViewable` is returned. If the keyboard
 * is frozen by an active grab of another client, `GrabFrozen` is returned. If the
 * specified \a time is earlier than the last-keyboard-grab time or later than the
 * current X server time, `GrabInvalidTime` is returned. Otherwise, the
 * last-keyboard-grab time is set to the specified time.
 * 
 */
unsafe fn xcb_grab_keyboard (c : *connection,
                             owner_events :  u8,
                             grab_window :  window,
                             time :  timestamp,
                             pointer_mode :  u8,
                             keyboard_mode :  u8) -> grab_keyboard_cookie;

/**
 * Grab the keyboard
 *
 * @param c The connection
 * @param owner_events If 1, the \a grab_window will still get the pointer events. If 0, events are not
 * reported to the \a grab_window.
 * @param grab_window Specifies the window on which the pointer should be grabbed.
 * @param time Timestamp to avoid race conditions when running X over the network.
 * \n
 * The special value `XCB_CURRENT_TIME` will be replaced with the current server
 * time.
 * @param pointer_mode A bitmask of #grab_mode values.
 * @param pointer_mode \n
 * @param keyboard_mode A bitmask of #grab_mode values.
 * @param keyboard_mode \n
 * @return A cookie
 *
 * Actively grabs control of the keyboard and generates FocusIn and FocusOut
 * events. Further key events are reported only to the grabbing client.
 * 
 * Any active keyboard grab by this client is overridden. If the keyboard is
 * actively grabbed by some other client, `AlreadyGrabbed` is returned. If
 * \a grab_window is not viewable, `GrabNotViewable` is returned. If the keyboard
 * is frozen by an active grab of another client, `GrabFrozen` is returned. If the
 * specified \a time is earlier than the last-keyboard-grab time or later than the
 * current X server time, `GrabInvalidTime` is returned. Otherwise, the
 * last-keyboard-grab time is set to the specified time.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
unsafe fn xcb_grab_keyboard_unchecked (c : *connection,
                                       owner_events :  u8,
                                       grab_window :  window,
                                       time :  timestamp,
                                       pointer_mode :  u8,
                                       keyboard_mode :  u8) -> grab_keyboard_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_grab_keyboard_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_grab_keyboard_reply (c : *connection,
                                   cookie : grab_keyboard_cookie,
                                   e : **generic_error) -> *grab_keyboard_reply;

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
unsafe fn xcb_ungrab_keyboard_checked (c : *connection,
                                       time :  timestamp) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_ungrab_keyboard (c : *connection,
                               time :  timestamp) -> void_cookie;

/**
 * Grab keyboard key(s)
 *
 * @param c The connection
 * @param owner_events If 1, the \a grab_window will still get the pointer events. If 0, events are not
 * reported to the \a grab_window.
 * @param grab_window Specifies the window on which the pointer should be grabbed.
 * @param modifiers The modifiers to grab.
 * \n
 * Using the special value `XCB_MOD_MASK_ANY` means grab the pointer with all
 * possible modifier combinations.
 * @param key The keycode of the key to grab.
 * \n
 * The special value `XCB_GRAB_ANY` means grab any key.
 * @param pointer_mode A bitmask of #grab_mode values.
 * @param pointer_mode \n
 * @param keyboard_mode A bitmask of #grab_mode values.
 * @param keyboard_mode \n
 * @return A cookie
 *
 * Establishes a passive grab on the keyboard. In the future, the keyboard is
 * actively grabbed (as for `GrabKeyboard`), the last-keyboard-grab time is set to
 * the time at which the key was pressed (as transmitted in the KeyPress event),
 * and the KeyPress event is reported if all of the following conditions are true:
 * 
 * The keyboard is not grabbed and the specified key (which can itself be a
 * modifier key) is logically pressed when the specified modifier keys are
 * logically down, and no other modifier keys are logically down.
 * 
 * Either the grab_window is an ancestor of (or is) the focus window, or the
 * grab_window is a descendant of the focus window and contains the pointer.
 * 
 * A passive grab on the same key combination does not exist on any ancestor of
 * grab_window.
 * 
 * The interpretation of the remaining arguments is as for XGrabKeyboard.  The active grab is terminated
 * automatically when the logical state of the keyboard has the specified key released (independent of the
 * logical state of the modifier keys), at which point a KeyRelease event is reported to the grabbing window.
 * 
 * Note that the logical state of a device (as seen by client applications) may lag the physical state if
 * device event processing is frozen.
 * 
 * A modifiers argument of AnyModifier is equivalent to issuing the request for all possible modifier combinations (including the combination of no modifiers).  It is not required that all modifiers specified
 * have currently assigned KeyCodes.  A keycode argument of AnyKey is equivalent to issuing the request for
 * all possible KeyCodes.  Otherwise, the specified keycode must be in the range specified by min_keycode
 * and max_keycode in the connection setup, or a BadValue error results.
 * 
 * If some other client has issued a XGrabKey with the same key combination on the same window, a BadAccess
 * error results.  When using AnyModifier or AnyKey, the request fails completely, and a BadAccess error
 * results (no grabs are established) if there is a conflicting grab for any combination.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
unsafe fn xcb_grab_key_checked (c : *connection,
                                owner_events :  u8,
                                grab_window :  window,
                                modifiers :  u16,
                                key :  keycode,
                                pointer_mode :  u8,
                                keyboard_mode :  u8) -> void_cookie;

/**
 * Grab keyboard key(s)
 *
 * @param c The connection
 * @param owner_events If 1, the \a grab_window will still get the pointer events. If 0, events are not
 * reported to the \a grab_window.
 * @param grab_window Specifies the window on which the pointer should be grabbed.
 * @param modifiers The modifiers to grab.
 * \n
 * Using the special value `XCB_MOD_MASK_ANY` means grab the pointer with all
 * possible modifier combinations.
 * @param key The keycode of the key to grab.
 * \n
 * The special value `XCB_GRAB_ANY` means grab any key.
 * @param pointer_mode A bitmask of #grab_mode values.
 * @param pointer_mode \n
 * @param keyboard_mode A bitmask of #grab_mode values.
 * @param keyboard_mode \n
 * @return A cookie
 *
 * Establishes a passive grab on the keyboard. In the future, the keyboard is
 * actively grabbed (as for `GrabKeyboard`), the last-keyboard-grab time is set to
 * the time at which the key was pressed (as transmitted in the KeyPress event),
 * and the KeyPress event is reported if all of the following conditions are true:
 * 
 * The keyboard is not grabbed and the specified key (which can itself be a
 * modifier key) is logically pressed when the specified modifier keys are
 * logically down, and no other modifier keys are logically down.
 * 
 * Either the grab_window is an ancestor of (or is) the focus window, or the
 * grab_window is a descendant of the focus window and contains the pointer.
 * 
 * A passive grab on the same key combination does not exist on any ancestor of
 * grab_window.
 * 
 * The interpretation of the remaining arguments is as for XGrabKeyboard.  The active grab is terminated
 * automatically when the logical state of the keyboard has the specified key released (independent of the
 * logical state of the modifier keys), at which point a KeyRelease event is reported to the grabbing window.
 * 
 * Note that the logical state of a device (as seen by client applications) may lag the physical state if
 * device event processing is frozen.
 * 
 * A modifiers argument of AnyModifier is equivalent to issuing the request for all possible modifier combinations (including the combination of no modifiers).  It is not required that all modifiers specified
 * have currently assigned KeyCodes.  A keycode argument of AnyKey is equivalent to issuing the request for
 * all possible KeyCodes.  Otherwise, the specified keycode must be in the range specified by min_keycode
 * and max_keycode in the connection setup, or a BadValue error results.
 * 
 * If some other client has issued a XGrabKey with the same key combination on the same window, a BadAccess
 * error results.  When using AnyModifier or AnyKey, the request fails completely, and a BadAccess error
 * results (no grabs are established) if there is a conflicting grab for any combination.
 * 
 */
unsafe fn xcb_grab_key (c : *connection,
                        owner_events :  u8,
                        grab_window :  window,
                        modifiers :  u16,
                        key :  keycode,
                        pointer_mode :  u8,
                        keyboard_mode :  u8) -> void_cookie;

/**
 * release a key combination
 *
 * @param c The connection
 * @param key The keycode of the specified key combination.
 * \n
 * Using the special value `XCB_GRAB_ANY` means releasing all possible key codes.
 * @param grab_window The window on which the grabbed key combination will be released.
 * @param modifiers The modifiers of the specified key combination.
 * \n
 * Using the special value `XCB_MOD_MASK_ANY` means releasing the key combination
 * with every possible modifier combination.
 * @return A cookie
 *
 * Releases the key combination on \a grab_window if you grabbed it using
 * `xcb_grab_key` before.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
unsafe fn xcb_ungrab_key_checked (c : *connection,
                                  key :  keycode,
                                  grab_window :  window,
                                  modifiers :  u16) -> void_cookie;

/**
 * release a key combination
 *
 * @param c The connection
 * @param key The keycode of the specified key combination.
 * \n
 * Using the special value `XCB_GRAB_ANY` means releasing all possible key codes.
 * @param grab_window The window on which the grabbed key combination will be released.
 * @param modifiers The modifiers of the specified key combination.
 * \n
 * Using the special value `XCB_MOD_MASK_ANY` means releasing the key combination
 * with every possible modifier combination.
 * @return A cookie
 *
 * Releases the key combination on \a grab_window if you grabbed it using
 * `xcb_grab_key` before.
 * 
 */
unsafe fn xcb_ungrab_key (c : *connection,
                          key :  keycode,
                          grab_window :  window,
                          modifiers :  u16) -> void_cookie;

/**
 * release queued events
 *
 * @param c The connection
 * @param mode A bitmask of #allow values.
 * @param mode \n
 * @param time Timestamp to avoid race conditions when running X over the network.
 * \n
 * The special value `XCB_CURRENT_TIME` will be replaced with the current server
 * time.
 * @return A cookie
 *
 * Releases queued events if the client has caused a device (pointer/keyboard) to
 * freeze due to grabbing it actively. This request has no effect if \a time is
 * earlier than the last-grab time of the most recent active grab for this client
 * or if \a time is later than the current X server time.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
unsafe fn xcb_allow_events_checked (c : *connection,
                                    mode :  u8,
                                    time :  timestamp) -> void_cookie;

/**
 * release queued events
 *
 * @param c The connection
 * @param mode A bitmask of #allow values.
 * @param mode \n
 * @param time Timestamp to avoid race conditions when running X over the network.
 * \n
 * The special value `XCB_CURRENT_TIME` will be replaced with the current server
 * time.
 * @return A cookie
 *
 * Releases queued events if the client has caused a device (pointer/keyboard) to
 * freeze due to grabbing it actively. This request has no effect if \a time is
 * earlier than the last-grab time of the most recent active grab for this client
 * or if \a time is later than the current X server time.
 * 
 */
unsafe fn xcb_allow_events (c : *connection,
                            mode :  u8,
                            time :  timestamp) -> void_cookie;

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
unsafe fn xcb_grab_server_checked (c : *connection) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_grab_server (c : *connection) -> void_cookie;

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
unsafe fn xcb_ungrab_server_checked (c : *connection) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_ungrab_server (c : *connection) -> void_cookie;

/**
 * get pointer coordinates
 *
 * @param c The connection
 * @param window A window to check if the pointer is on the same screen as \a window (see the
 * `same_screen` field in the reply).
 * @return A cookie
 *
 * Gets the root window the pointer is logically on and the pointer coordinates
 * relative to the root window's origin.
 * 
 */
unsafe fn xcb_query_pointer (c : *connection,
                             window :  window) -> query_pointer_cookie;

/**
 * get pointer coordinates
 *
 * @param c The connection
 * @param window A window to check if the pointer is on the same screen as \a window (see the
 * `same_screen` field in the reply).
 * @return A cookie
 *
 * Gets the root window the pointer is logically on and the pointer coordinates
 * relative to the root window's origin.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
unsafe fn xcb_query_pointer_unchecked (c : *connection,
                                       window :  window) -> query_pointer_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_query_pointer_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_query_pointer_reply (c : *connection,
                                   cookie : query_pointer_cookie,
                                   e : **generic_error) -> *query_pointer_reply;

/**
 * Get the next element of the iterator
 * @param i Pointer to a timecoord_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(timecoord)
 *
 *
 */
unsafe fn xcb_timecoord_next (i:*timecoord_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An timecoord_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_timecoord_end (i:timecoord_iterator) -> generic_iterator;

unsafe fn xcb_get_motion_events_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_get_motion_events (c : *connection,
                                 window :  window,
                                 start :  timestamp,
                                 stop :  timestamp) -> get_motion_events_cookie;

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
unsafe fn xcb_get_motion_events_unchecked (c : *connection,
                                           window :  window,
                                           start :  timestamp,
                                           stop :  timestamp) -> get_motion_events_cookie;

unsafe fn xcb_get_motion_events_events (R : *get_motion_events_reply) -> *timecoord;


unsafe fn xcb_get_motion_events_events_length (R : *get_motion_events_reply) -> c_int;

unsafe fn xcb_get_motion_events_events_iterator (R : *get_motion_events_reply) -> timecoord_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_get_motion_events_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_get_motion_events_reply (c : *connection,
                                       cookie : get_motion_events_cookie,
                                       e : **generic_error) -> *get_motion_events_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_translate_coordinates (c : *connection,
                                     src_window :  window,
                                     dst_window :  window,
                                     src_x :  i16,
                                     src_y :  i16) -> translate_coordinates_cookie;

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
unsafe fn xcb_translate_coordinates_unchecked (c : *connection,
                                               src_window :  window,
                                               dst_window :  window,
                                               src_x :  i16,
                                               src_y :  i16) -> translate_coordinates_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_translate_coordinates_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_translate_coordinates_reply (c : *connection,
                                           cookie : translate_coordinates_cookie,
                                           e : **generic_error) -> *translate_coordinates_reply;

/**
 * move mouse pointer
 *
 * @param c The connection
 * @param src_window If \a src_window is not `XCB_NONE` (TODO), the move will only take place if the
 * pointer is inside \a src_window and within the rectangle specified by (\a src_x,
 * \a src_y, \a src_width, \a src_height). The rectangle coordinates are relative to
 * \a src_window.
 * @param dst_window If \a dst_window is not `XCB_NONE` (TODO), the pointer will be moved to the
 * offsets (\a dst_x, \a dst_y) relative to \a dst_window. If \a dst_window is
 * `XCB_NONE` (TODO), the pointer will be moved by the offsets (\a dst_x, \a dst_y)
 * relative to the current position of the pointer.
 * @return A cookie
 *
 * Moves the mouse pointer to the specified position.
 * 
 * If \a src_window is not `XCB_NONE` (TODO), the move will only take place if the
 * pointer is inside \a src_window and within the rectangle specified by (\a src_x,
 * \a src_y, \a src_width, \a src_height). The rectangle coordinates are relative to
 * \a src_window.
 * 
 * If \a dst_window is not `XCB_NONE` (TODO), the pointer will be moved to the
 * offsets (\a dst_x, \a dst_y) relative to \a dst_window. If \a dst_window is
 * `XCB_NONE` (TODO), the pointer will be moved by the offsets (\a dst_x, \a dst_y)
 * relative to the current position of the pointer.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
unsafe fn xcb_warp_pointer_checked (c : *connection,
                                    src_window :  window,
                                    dst_window :  window,
                                    src_x :  i16,
                                    src_y :  i16,
                                    src_width :  u16,
                                    src_height :  u16,
                                    dst_x :  i16,
                                    dst_y :  i16) -> void_cookie;

/**
 * move mouse pointer
 *
 * @param c The connection
 * @param src_window If \a src_window is not `XCB_NONE` (TODO), the move will only take place if the
 * pointer is inside \a src_window and within the rectangle specified by (\a src_x,
 * \a src_y, \a src_width, \a src_height). The rectangle coordinates are relative to
 * \a src_window.
 * @param dst_window If \a dst_window is not `XCB_NONE` (TODO), the pointer will be moved to the
 * offsets (\a dst_x, \a dst_y) relative to \a dst_window. If \a dst_window is
 * `XCB_NONE` (TODO), the pointer will be moved by the offsets (\a dst_x, \a dst_y)
 * relative to the current position of the pointer.
 * @return A cookie
 *
 * Moves the mouse pointer to the specified position.
 * 
 * If \a src_window is not `XCB_NONE` (TODO), the move will only take place if the
 * pointer is inside \a src_window and within the rectangle specified by (\a src_x,
 * \a src_y, \a src_width, \a src_height). The rectangle coordinates are relative to
 * \a src_window.
 * 
 * If \a dst_window is not `XCB_NONE` (TODO), the pointer will be moved to the
 * offsets (\a dst_x, \a dst_y) relative to \a dst_window. If \a dst_window is
 * `XCB_NONE` (TODO), the pointer will be moved by the offsets (\a dst_x, \a dst_y)
 * relative to the current position of the pointer.
 * 
 */
unsafe fn xcb_warp_pointer (c : *connection,
                            src_window :  window,
                            dst_window :  window,
                            src_x :  i16,
                            src_y :  i16,
                            src_width :  u16,
                            src_height :  u16,
                            dst_x :  i16,
                            dst_y :  i16) -> void_cookie;

/**
 * Sets input focus
 *
 * @param c The connection
 * @param revert_to A bitmask of #input_focus values.
 * @param revert_to Specifies what happens when the \a focus window becomes unviewable (if \a focus
 * is neither `XCB_NONE` nor `XCB_POINTER_ROOT`).
 * @param focus The window to focus. All keyboard events will be reported to this window. The
 * window must be viewable (TODO), or a `xcb_match_error_t` occurs (TODO).
 * \n
 * If \a focus is `XCB_NONE` (TODO), all keyboard events are
 * discarded until a new focus window is set.
 * \n
 * If \a focus is `XCB_POINTER_ROOT` (TODO), focus is on the root window of the
 * screen on which the pointer is on currently.
 * @param time Timestamp to avoid race conditions when running X over the network.
 * \n
 * The special value `XCB_CURRENT_TIME` will be replaced with the current server
 * time.
 * @return A cookie
 *
 * Changes the input focus and the last-focus-change time. If the specified \a time
 * is earlier than the current last-focus-change time, the request is ignored (to
 * avoid race conditions when running X over the network).
 * 
 * A FocusIn and FocusOut event is generated when focus is changed.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
unsafe fn xcb_set_input_focus_checked (c : *connection,
                                       revert_to :  u8,
                                       focus :  window,
                                       time :  timestamp) -> void_cookie;

/**
 * Sets input focus
 *
 * @param c The connection
 * @param revert_to A bitmask of #input_focus values.
 * @param revert_to Specifies what happens when the \a focus window becomes unviewable (if \a focus
 * is neither `XCB_NONE` nor `XCB_POINTER_ROOT`).
 * @param focus The window to focus. All keyboard events will be reported to this window. The
 * window must be viewable (TODO), or a `xcb_match_error_t` occurs (TODO).
 * \n
 * If \a focus is `XCB_NONE` (TODO), all keyboard events are
 * discarded until a new focus window is set.
 * \n
 * If \a focus is `XCB_POINTER_ROOT` (TODO), focus is on the root window of the
 * screen on which the pointer is on currently.
 * @param time Timestamp to avoid race conditions when running X over the network.
 * \n
 * The special value `XCB_CURRENT_TIME` will be replaced with the current server
 * time.
 * @return A cookie
 *
 * Changes the input focus and the last-focus-change time. If the specified \a time
 * is earlier than the current last-focus-change time, the request is ignored (to
 * avoid race conditions when running X over the network).
 * 
 * A FocusIn and FocusOut event is generated when focus is changed.
 * 
 */
unsafe fn xcb_set_input_focus (c : *connection,
                               revert_to :  u8,
                               focus :  window,
                               time :  timestamp) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_get_input_focus (c : *connection) -> get_input_focus_cookie;

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
unsafe fn xcb_get_input_focus_unchecked (c : *connection) -> get_input_focus_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_get_input_focus_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_get_input_focus_reply (c : *connection,
                                     cookie : get_input_focus_cookie,
                                     e : **generic_error) -> *get_input_focus_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_query_keymap (c : *connection) -> query_keymap_cookie;

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
unsafe fn xcb_query_keymap_unchecked (c : *connection) -> query_keymap_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_query_keymap_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_query_keymap_reply (c : *connection,
                                  cookie : query_keymap_cookie,
                                  e : **generic_error) -> *query_keymap_reply;

unsafe fn xcb_open_font_sizeof (_buffer :  *c_void) -> c_int;

/**
 * opens a font
 *
 * @param c The connection
 * @param fid The ID with which you will refer to the font, created by `xcb_generate_id`.
 * @param name_len Length (in bytes) of \a name.
 * @param name A pattern describing an X core font.
 * @return A cookie
 *
 * Opens any X core font matching the given \a name (for example "-misc-fixed-*").
 * 
 * Note that X core fonts are deprecated (but still supported) in favor of
 * client-side rendering using Xft.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
unsafe fn xcb_open_font_checked (c : *connection,
                                 fid :  font,
                                 name_len :  u16,
                                 name : *c_char) -> void_cookie;

/**
 * opens a font
 *
 * @param c The connection
 * @param fid The ID with which you will refer to the font, created by `xcb_generate_id`.
 * @param name_len Length (in bytes) of \a name.
 * @param name A pattern describing an X core font.
 * @return A cookie
 *
 * Opens any X core font matching the given \a name (for example "-misc-fixed-*").
 * 
 * Note that X core fonts are deprecated (but still supported) in favor of
 * client-side rendering using Xft.
 * 
 */
unsafe fn xcb_open_font (c : *connection,
                         fid :  font,
                         name_len :  u16,
                         name : *c_char) -> void_cookie;

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
unsafe fn xcb_close_font_checked (c : *connection,
                                  font :  font) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_close_font (c : *connection,
                          font :  font) -> void_cookie;

/**
 * Get the next element of the iterator
 * @param i Pointer to a fontprop_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(fontprop)
 *
 *
 */
unsafe fn xcb_fontprop_next (i:*fontprop_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An fontprop_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_fontprop_end (i:fontprop_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a charinfo_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(charinfo)
 *
 *
 */
unsafe fn xcb_charinfo_next (i:*charinfo_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An charinfo_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_charinfo_end (i:charinfo_iterator) -> generic_iterator;

unsafe fn xcb_query_font_sizeof (_buffer :  *c_void) -> c_int;

/**
 * query font metrics
 *
 * @param c The connection
 * @param font The fontable (Font or Graphics Context) to query.
 * @return A cookie
 *
 * Queries information associated with the font.
 * 
 */
unsafe fn xcb_query_font (c : *connection,
                          font :  fontable) -> query_font_cookie;

/**
 * query font metrics
 *
 * @param c The connection
 * @param font The fontable (Font or Graphics Context) to query.
 * @return A cookie
 *
 * Queries information associated with the font.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
unsafe fn xcb_query_font_unchecked (c : *connection,
                                    font :  fontable) -> query_font_cookie;

unsafe fn xcb_query_font_properties (R : *query_font_reply) -> *fontprop;


unsafe fn xcb_query_font_properties_length (R : *query_font_reply) -> c_int;

unsafe fn xcb_query_font_properties_iterator (R : *query_font_reply) -> fontprop_iterator;

unsafe fn xcb_query_font_char_infos (R : *query_font_reply) -> *charinfo;


unsafe fn xcb_query_font_char_infos_length (R : *query_font_reply) -> c_int;

unsafe fn xcb_query_font_char_infos_iterator (R : *query_font_reply) -> charinfo_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_query_font_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_query_font_reply (c : *connection,
                                cookie : query_font_cookie,
                                e : **generic_error) -> *query_font_reply;

unsafe fn xcb_query_text_extents_sizeof (_buffer :  *c_void,
                               string_len :  u32) -> c_int;

/**
 * get text extents
 *
 * @param c The connection
 * @param font The \a font to calculate text extents in. You can also pass a graphics context.
 * @param string_len The number of characters in \a string.
 * @param string The text to get text extents for.
 * @return A cookie
 *
 * Query text extents from the X11 server. This request returns the bounding box
 * of the specified 16-bit character string in the specified \a font or the font
 * contained in the specified graphics context.
 * 
 * `font_ascent` is set to the maximum of the ascent metrics of all characters in
 * the string. `font_descent` is set to the maximum of the descent metrics.
 * `overall_width` is set to the sum of the character-width metrics of all
 * characters in the string. For each character in the string, let W be the sum of
 * the character-width metrics of all characters preceding it in the string. Let L
 * be the left-side-bearing metric of the character plus W. Let R be the
 * right-side-bearing metric of the character plus W. The lbearing member is set
 * to the minimum L of all characters in the string. The rbearing member is set to
 * the maximum R.
 * 
 * For fonts defined with linear indexing rather than 2-byte matrix indexing, each
 * `xcb_char2b_t` structure is interpreted as a 16-bit number with byte1 as the
 * most significant byte. If the font has no defined default character, undefined
 * characters in the string are taken to have all zero metrics.
 * 
 * Characters with all zero metrics are ignored. If the font has no defined
 * default_char, the undefined characters in the string are also ignored.
 * 
 */
unsafe fn xcb_query_text_extents (c : *connection,
                                  font :  fontable,
                                  string_len :  u32,
                                  string : *char2b) -> query_text_extents_cookie;

/**
 * get text extents
 *
 * @param c The connection
 * @param font The \a font to calculate text extents in. You can also pass a graphics context.
 * @param string_len The number of characters in \a string.
 * @param string The text to get text extents for.
 * @return A cookie
 *
 * Query text extents from the X11 server. This request returns the bounding box
 * of the specified 16-bit character string in the specified \a font or the font
 * contained in the specified graphics context.
 * 
 * `font_ascent` is set to the maximum of the ascent metrics of all characters in
 * the string. `font_descent` is set to the maximum of the descent metrics.
 * `overall_width` is set to the sum of the character-width metrics of all
 * characters in the string. For each character in the string, let W be the sum of
 * the character-width metrics of all characters preceding it in the string. Let L
 * be the left-side-bearing metric of the character plus W. Let R be the
 * right-side-bearing metric of the character plus W. The lbearing member is set
 * to the minimum L of all characters in the string. The rbearing member is set to
 * the maximum R.
 * 
 * For fonts defined with linear indexing rather than 2-byte matrix indexing, each
 * `xcb_char2b_t` structure is interpreted as a 16-bit number with byte1 as the
 * most significant byte. If the font has no defined default character, undefined
 * characters in the string are taken to have all zero metrics.
 * 
 * Characters with all zero metrics are ignored. If the font has no defined
 * default_char, the undefined characters in the string are also ignored.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
unsafe fn xcb_query_text_extents_unchecked (c : *connection,
                                            font :  fontable,
                                            string_len :  u32,
                                            string : *char2b) -> query_text_extents_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_query_text_extents_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_query_text_extents_reply (c : *connection,
                                        cookie : query_text_extents_cookie,
                                        e : **generic_error) -> *query_text_extents_reply;

unsafe fn xcb_str_sizeof (_buffer :  *c_void) -> c_int;

unsafe fn xcb_str_name (R : *str_) -> *c_char;


unsafe fn xcb_str_name_length (R : *str_) -> c_int;


unsafe fn xcb_str_name_end (R : *str_) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a str_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(str_)
 *
 *
 */
unsafe fn xcb_str_next (i:*str_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An str_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_str_end (i:str_iterator) -> generic_iterator;

unsafe fn xcb_list_fonts_sizeof (_buffer :  *c_void) -> c_int;

/**
 * get matching font names
 *
 * @param c The connection
 * @param max_names The maximum number of fonts to be returned.
 * @param pattern_len The length (in bytes) of \a pattern.
 * @param pattern A font pattern, for example "-misc-fixed-*".
 * \n
 * The asterisk (*) is a wildcard for any number of characters. The question mark
 * (?) is a wildcard for a single character. Use of uppercase or lowercase does
 * not matter.
 * @return A cookie
 *
 * Gets a list of available font names which match the given \a pattern.
 * 
 */
unsafe fn xcb_list_fonts (c : *connection,
                          max_names :  u16,
                          pattern_len :  u16,
                          pattern : *c_char) -> list_fonts_cookie;

/**
 * get matching font names
 *
 * @param c The connection
 * @param max_names The maximum number of fonts to be returned.
 * @param pattern_len The length (in bytes) of \a pattern.
 * @param pattern A font pattern, for example "-misc-fixed-*".
 * \n
 * The asterisk (*) is a wildcard for any number of characters. The question mark
 * (?) is a wildcard for a single character. Use of uppercase or lowercase does
 * not matter.
 * @return A cookie
 *
 * Gets a list of available font names which match the given \a pattern.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
unsafe fn xcb_list_fonts_unchecked (c : *connection,
                                    max_names :  u16,
                                    pattern_len :  u16,
                                    pattern : *c_char) -> list_fonts_cookie;


unsafe fn xcb_list_fonts_names_length (R : *list_fonts_reply) -> c_int;

unsafe fn xcb_list_fonts_names_iterator (R : *list_fonts_reply) -> str_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_list_fonts_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_list_fonts_reply (c : *connection,
                                cookie : list_fonts_cookie,
                                e : **generic_error) -> *list_fonts_reply;

unsafe fn xcb_list_fonts_with_info_sizeof (_buffer :  *c_void) -> c_int;

/**
 * get matching font names and information
 *
 * @param c The connection
 * @param max_names The maximum number of fonts to be returned.
 * @param pattern_len The length (in bytes) of \a pattern.
 * @param pattern A font pattern, for example "-misc-fixed-*".
 * \n
 * The asterisk (*) is a wildcard for any number of characters. The question mark
 * (?) is a wildcard for a single character. Use of uppercase or lowercase does
 * not matter.
 * @return A cookie
 *
 * Gets a list of available font names which match the given \a pattern.
 * 
 */
unsafe fn xcb_list_fonts_with_info (c : *connection,
                                    max_names :  u16,
                                    pattern_len :  u16,
                                    pattern : *c_char) -> list_fonts_with_info_cookie;

/**
 * get matching font names and information
 *
 * @param c The connection
 * @param max_names The maximum number of fonts to be returned.
 * @param pattern_len The length (in bytes) of \a pattern.
 * @param pattern A font pattern, for example "-misc-fixed-*".
 * \n
 * The asterisk (*) is a wildcard for any number of characters. The question mark
 * (?) is a wildcard for a single character. Use of uppercase or lowercase does
 * not matter.
 * @return A cookie
 *
 * Gets a list of available font names which match the given \a pattern.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
unsafe fn xcb_list_fonts_with_info_unchecked (c : *connection,
                                              max_names :  u16,
                                              pattern_len :  u16,
                                              pattern : *c_char) -> list_fonts_with_info_cookie;

unsafe fn xcb_list_fonts_with_info_properties (R : *list_fonts_with_info_reply) -> *fontprop;


unsafe fn xcb_list_fonts_with_info_properties_length (R : *list_fonts_with_info_reply) -> c_int;

unsafe fn xcb_list_fonts_with_info_properties_iterator (R : *list_fonts_with_info_reply) -> fontprop_iterator;

unsafe fn xcb_list_fonts_with_info_name (R : *list_fonts_with_info_reply) -> *c_char;


unsafe fn xcb_list_fonts_with_info_name_length (R : *list_fonts_with_info_reply) -> c_int;


unsafe fn xcb_list_fonts_with_info_name_end (R : *list_fonts_with_info_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_list_fonts_with_info_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_list_fonts_with_info_reply (c : *connection,
                                          cookie : list_fonts_with_info_cookie,
                                          e : **generic_error) -> *list_fonts_with_info_reply;

unsafe fn xcb_set_font_path_sizeof (_buffer :  *c_void) -> c_int;

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
unsafe fn xcb_set_font_path_checked (c : *connection,
                                     font_qty :  u16,
                                     font : *str_) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_set_font_path (c : *connection,
                             font_qty :  u16,
                             font : *str_) -> void_cookie;

unsafe fn xcb_get_font_path_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_get_font_path (c : *connection) -> get_font_path_cookie;

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
unsafe fn xcb_get_font_path_unchecked (c : *connection) -> get_font_path_cookie;


unsafe fn xcb_get_font_path_path_length (R : *get_font_path_reply) -> c_int;

unsafe fn xcb_get_font_path_path_iterator (R : *get_font_path_reply) -> str_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_get_font_path_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_get_font_path_reply (c : *connection,
                                   cookie : get_font_path_cookie,
                                   e : **generic_error) -> *get_font_path_reply;

/**
 * Creates a pixmap
 *
 * @param c The connection
 * @param depth TODO
 * @param pid The ID with which you will refer to the new pixmap, created by
 * `xcb_generate_id`.
 * @param drawable Drawable to get the screen from.
 * @param width The width of the new pixmap.
 * @param height The height of the new pixmap.
 * @return A cookie
 *
 * Creates a pixmap. The pixmap can only be used on the same screen as \a drawable
 * is on and only with drawables of the same \a depth.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
unsafe fn xcb_create_pixmap_checked (c : *connection,
                                     depth :  u8,
                                     pid :  pixmap,
                                     drawable :  drawable,
                                     width :  u16,
                                     height :  u16) -> void_cookie;

/**
 * Creates a pixmap
 *
 * @param c The connection
 * @param depth TODO
 * @param pid The ID with which you will refer to the new pixmap, created by
 * `xcb_generate_id`.
 * @param drawable Drawable to get the screen from.
 * @param width The width of the new pixmap.
 * @param height The height of the new pixmap.
 * @return A cookie
 *
 * Creates a pixmap. The pixmap can only be used on the same screen as \a drawable
 * is on and only with drawables of the same \a depth.
 * 
 */
unsafe fn xcb_create_pixmap (c : *connection,
                             depth :  u8,
                             pid :  pixmap,
                             drawable :  drawable,
                             width :  u16,
                             height :  u16) -> void_cookie;

/**
 * Destroys a pixmap
 *
 * @param c The connection
 * @param pixmap The pixmap to destroy.
 * @return A cookie
 *
 * Deletes the association between the pixmap ID and the pixmap. The pixmap
 * storage will be freed when there are no more references to it.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
unsafe fn xcb_free_pixmap_checked (c : *connection,
                                   pixmap :  pixmap) -> void_cookie;

/**
 * Destroys a pixmap
 *
 * @param c The connection
 * @param pixmap The pixmap to destroy.
 * @return A cookie
 *
 * Deletes the association between the pixmap ID and the pixmap. The pixmap
 * storage will be freed when there are no more references to it.
 * 
 */
unsafe fn xcb_free_pixmap (c : *connection,
                           pixmap :  pixmap) -> void_cookie;

unsafe fn xcb_create_gc_sizeof (_buffer :  *c_void) -> c_int;

/**
 * Creates a graphics context
 *
 * @param c The connection
 * @param cid The ID with which you will refer to the graphics context, created by
 * `xcb_generate_id`.
 * @param drawable Drawable to get the root/depth from.
 * @return A cookie
 *
 * Creates a graphics context. The graphics context can be used with any drawable
 * that has the same root and depth as the specified drawable.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
unsafe fn xcb_create_gc_checked (c : *connection,
                                 cid :  gcontext,
                                 drawable :  drawable,
                                 value_mask :  u32,
                                 value_list : *u32) -> void_cookie;

/**
 * Creates a graphics context
 *
 * @param c The connection
 * @param cid The ID with which you will refer to the graphics context, created by
 * `xcb_generate_id`.
 * @param drawable Drawable to get the root/depth from.
 * @return A cookie
 *
 * Creates a graphics context. The graphics context can be used with any drawable
 * that has the same root and depth as the specified drawable.
 * 
 */
unsafe fn xcb_create_gc (c : *connection,
                         cid :  gcontext,
                         drawable :  drawable,
                         value_mask :  u32,
                         value_list : *u32) -> void_cookie;

unsafe fn xcb_change_gc_sizeof (_buffer :  *c_void) -> c_int;

/**
 * change graphics context components
 *
 * @param c The connection
 * @param gc The graphics context to change.
 * @param value_mask \n
 * @param value_list Values for each of the components specified in the bitmask \a value_mask. The
 * order has to correspond to the order of possible \a value_mask bits. See the
 * example.
 * @return A cookie
 *
 * Changes the components specified by \a value_mask for the specified graphics context.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
unsafe fn xcb_change_gc_checked (c : *connection,
                                 gc :  gcontext,
                                 value_mask :  u32,
                                 value_list : *u32) -> void_cookie;

/**
 * change graphics context components
 *
 * @param c The connection
 * @param gc The graphics context to change.
 * @param value_mask \n
 * @param value_list Values for each of the components specified in the bitmask \a value_mask. The
 * order has to correspond to the order of possible \a value_mask bits. See the
 * example.
 * @return A cookie
 *
 * Changes the components specified by \a value_mask for the specified graphics context.
 * 
 */
unsafe fn xcb_change_gc (c : *connection,
                         gc :  gcontext,
                         value_mask :  u32,
                         value_list : *u32) -> void_cookie;

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
unsafe fn xcb_copy_gc_checked (c : *connection,
                               src_gc :  gcontext,
                               dst_gc :  gcontext,
                               value_mask :  u32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_copy_gc (c : *connection,
                       src_gc :  gcontext,
                       dst_gc :  gcontext,
                       value_mask :  u32) -> void_cookie;

unsafe fn xcb_set_dashes_sizeof (_buffer :  *c_void) -> c_int;

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
unsafe fn xcb_set_dashes_checked (c : *connection,
                                  gc :  gcontext,
                                  dash_offset :  u16,
                                  dashes_len :  u16,
                                  dashes : *u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_set_dashes (c : *connection,
                          gc :  gcontext,
                          dash_offset :  u16,
                          dashes_len :  u16,
                          dashes : *u8) -> void_cookie;

unsafe fn xcb_set_clip_rectangles_sizeof (_buffer :  *c_void,
                                rectangles_len :  u32) -> c_int;

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
unsafe fn xcb_set_clip_rectangles_checked (c : *connection,
                                           ordering :  u8,
                                           gc :  gcontext,
                                           clip_x_origin :  i16,
                                           clip_y_origin :  i16,
                                           rectangles_len :  u32,
                                           rectangles : *rectangle) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_set_clip_rectangles (c : *connection,
                                   ordering :  u8,
                                   gc :  gcontext,
                                   clip_x_origin :  i16,
                                   clip_y_origin :  i16,
                                   rectangles_len :  u32,
                                   rectangles : *rectangle) -> void_cookie;

/**
 * Destroys a graphics context
 *
 * @param c The connection
 * @param gc The graphics context to destroy.
 * @return A cookie
 *
 * Destroys the specified \a gc and all associated storage.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
unsafe fn xcb_free_gc_checked (c : *connection,
                               gc :  gcontext) -> void_cookie;

/**
 * Destroys a graphics context
 *
 * @param c The connection
 * @param gc The graphics context to destroy.
 * @return A cookie
 *
 * Destroys the specified \a gc and all associated storage.
 * 
 */
unsafe fn xcb_free_gc (c : *connection,
                       gc :  gcontext) -> void_cookie;

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
unsafe fn xcb_clear_area_checked (c : *connection,
                                  exposures :  u8,
                                  window :  window,
                                  x :  i16,
                                  y :  i16,
                                  width :  u16,
                                  height :  u16) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_clear_area (c : *connection,
                          exposures :  u8,
                          window :  window,
                          x :  i16,
                          y :  i16,
                          width :  u16,
                          height :  u16) -> void_cookie;

/**
 * copy areas
 *
 * @param c The connection
 * @param src_drawable The source drawable (Window or Pixmap).
 * @param dst_drawable The destination drawable (Window or Pixmap).
 * @param gc The graphics context to use.
 * @param src_x The source X coordinate.
 * @param src_y The source Y coordinate.
 * @param dst_x The destination X coordinate.
 * @param dst_y The destination Y coordinate.
 * @param width The width of the area to copy (in pixels).
 * @param height The height of the area to copy (in pixels).
 * @return A cookie
 *
 * Copies the specified rectangle from \a src_drawable to \a dst_drawable.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
unsafe fn xcb_copy_area_checked (c : *connection,
                                 src_drawable :  drawable,
                                 dst_drawable :  drawable,
                                 gc :  gcontext,
                                 src_x :  i16,
                                 src_y :  i16,
                                 dst_x :  i16,
                                 dst_y :  i16,
                                 width :  u16,
                                 height :  u16) -> void_cookie;

/**
 * copy areas
 *
 * @param c The connection
 * @param src_drawable The source drawable (Window or Pixmap).
 * @param dst_drawable The destination drawable (Window or Pixmap).
 * @param gc The graphics context to use.
 * @param src_x The source X coordinate.
 * @param src_y The source Y coordinate.
 * @param dst_x The destination X coordinate.
 * @param dst_y The destination Y coordinate.
 * @param width The width of the area to copy (in pixels).
 * @param height The height of the area to copy (in pixels).
 * @return A cookie
 *
 * Copies the specified rectangle from \a src_drawable to \a dst_drawable.
 * 
 */
unsafe fn xcb_copy_area (c : *connection,
                         src_drawable :  drawable,
                         dst_drawable :  drawable,
                         gc :  gcontext,
                         src_x :  i16,
                         src_y :  i16,
                         dst_x :  i16,
                         dst_y :  i16,
                         width :  u16,
                         height :  u16) -> void_cookie;

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
unsafe fn xcb_copy_plane_checked (c : *connection,
                                  src_drawable :  drawable,
                                  dst_drawable :  drawable,
                                  gc :  gcontext,
                                  src_x :  i16,
                                  src_y :  i16,
                                  dst_x :  i16,
                                  dst_y :  i16,
                                  width :  u16,
                                  height :  u16,
                                  bit_plane :  u32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_copy_plane (c : *connection,
                          src_drawable :  drawable,
                          dst_drawable :  drawable,
                          gc :  gcontext,
                          src_x :  i16,
                          src_y :  i16,
                          dst_x :  i16,
                          dst_y :  i16,
                          width :  u16,
                          height :  u16,
                          bit_plane :  u32) -> void_cookie;

unsafe fn xcb_poly_point_sizeof (_buffer :  *c_void,
                       points_len :  u32) -> c_int;

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
unsafe fn xcb_poly_point_checked (c : *connection,
                                  coordinate_mode :  u8,
                                  drawable :  drawable,
                                  gc :  gcontext,
                                  points_len :  u32,
                                  points : *point) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_poly_point (c : *connection,
                          coordinate_mode :  u8,
                          drawable :  drawable,
                          gc :  gcontext,
                          points_len :  u32,
                          points : *point) -> void_cookie;

unsafe fn xcb_poly_line_sizeof (_buffer :  *c_void,
                      points_len :  u32) -> c_int;

/**
 * draw lines
 *
 * @param c The connection
 * @param coordinate_mode A bitmask of #coord_mode values.
 * @param coordinate_mode \n
 * @param drawable The drawable to draw the line(s) on.
 * @param gc The graphics context to use.
 * @param points_len The number of `xcb_point_t` structures in \a points.
 * @param points An array of points.
 * @return A cookie
 *
 * Draws \a points_len-1 lines between each pair of points (point[i], point[i+1])
 * in the \a points array. The lines are drawn in the order listed in the array.
 * They join correctly at all intermediate points, and if the first and last
 * points coincide, the first and last lines also join correctly. For any given
 * line, a pixel is not drawn more than once. If thin (zero line-width) lines
 * intersect, the intersecting pixels are drawn multiple times. If wide lines
 * intersect, the intersecting pixels are drawn only once, as though the entire
 * request were a single, filled shape.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
unsafe fn xcb_poly_line_checked (c : *connection,
                                 coordinate_mode :  u8,
                                 drawable :  drawable,
                                 gc :  gcontext,
                                 points_len :  u32,
                                 points : *point) -> void_cookie;

/**
 * draw lines
 *
 * @param c The connection
 * @param coordinate_mode A bitmask of #coord_mode values.
 * @param coordinate_mode \n
 * @param drawable The drawable to draw the line(s) on.
 * @param gc The graphics context to use.
 * @param points_len The number of `xcb_point_t` structures in \a points.
 * @param points An array of points.
 * @return A cookie
 *
 * Draws \a points_len-1 lines between each pair of points (point[i], point[i+1])
 * in the \a points array. The lines are drawn in the order listed in the array.
 * They join correctly at all intermediate points, and if the first and last
 * points coincide, the first and last lines also join correctly. For any given
 * line, a pixel is not drawn more than once. If thin (zero line-width) lines
 * intersect, the intersecting pixels are drawn multiple times. If wide lines
 * intersect, the intersecting pixels are drawn only once, as though the entire
 * request were a single, filled shape.
 * 
 */
unsafe fn xcb_poly_line (c : *connection,
                         coordinate_mode :  u8,
                         drawable :  drawable,
                         gc :  gcontext,
                         points_len :  u32,
                         points : *point) -> void_cookie;

/**
 * Get the next element of the iterator
 * @param i Pointer to a segment_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(segment)
 *
 *
 */
unsafe fn xcb_segment_next (i:*segment_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An segment_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_segment_end (i:segment_iterator) -> generic_iterator;

unsafe fn xcb_poly_segment_sizeof (_buffer :  *c_void,
                         segments_len :  u32) -> c_int;

/**
 * draw lines
 *
 * @param c The connection
 * @param drawable A drawable (Window or Pixmap) to draw on.
 * @param gc The graphics context to use.
 * \n
 * TODO: document which attributes of a gc are used
 * @param segments_len The number of `xcb_segment_t` structures in \a segments.
 * @param segments An array of `xcb_segment_t` structures.
 * @return A cookie
 *
 * Draws multiple, unconnected lines. For each segment, a line is drawn between
 * (x1, y1) and (x2, y2). The lines are drawn in the order listed in the array of
 * `xcb_segment_t` structures and does not perform joining at coincident
 * endpoints. For any given line, a pixel is not drawn more than once. If lines
 * intersect, the intersecting pixels are drawn multiple times.
 * 
 * TODO: include the xcb_segment_t data structure
 * 
 * TODO: an example
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
unsafe fn xcb_poly_segment_checked (c : *connection,
                                    drawable :  drawable,
                                    gc :  gcontext,
                                    segments_len :  u32,
                                    segments : *segment) -> void_cookie;

/**
 * draw lines
 *
 * @param c The connection
 * @param drawable A drawable (Window or Pixmap) to draw on.
 * @param gc The graphics context to use.
 * \n
 * TODO: document which attributes of a gc are used
 * @param segments_len The number of `xcb_segment_t` structures in \a segments.
 * @param segments An array of `xcb_segment_t` structures.
 * @return A cookie
 *
 * Draws multiple, unconnected lines. For each segment, a line is drawn between
 * (x1, y1) and (x2, y2). The lines are drawn in the order listed in the array of
 * `xcb_segment_t` structures and does not perform joining at coincident
 * endpoints. For any given line, a pixel is not drawn more than once. If lines
 * intersect, the intersecting pixels are drawn multiple times.
 * 
 * TODO: include the xcb_segment_t data structure
 * 
 * TODO: an example
 * 
 */
unsafe fn xcb_poly_segment (c : *connection,
                            drawable :  drawable,
                            gc :  gcontext,
                            segments_len :  u32,
                            segments : *segment) -> void_cookie;

unsafe fn xcb_poly_rectangle_sizeof (_buffer :  *c_void,
                           rectangles_len :  u32) -> c_int;

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
unsafe fn xcb_poly_rectangle_checked (c : *connection,
                                      drawable :  drawable,
                                      gc :  gcontext,
                                      rectangles_len :  u32,
                                      rectangles : *rectangle) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_poly_rectangle (c : *connection,
                              drawable :  drawable,
                              gc :  gcontext,
                              rectangles_len :  u32,
                              rectangles : *rectangle) -> void_cookie;

unsafe fn xcb_poly_arc_sizeof (_buffer :  *c_void,
                     arcs_len :  u32) -> c_int;

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
unsafe fn xcb_poly_arc_checked (c : *connection,
                                drawable :  drawable,
                                gc :  gcontext,
                                arcs_len :  u32,
                                arcs : *arc) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_poly_arc (c : *connection,
                        drawable :  drawable,
                        gc :  gcontext,
                        arcs_len :  u32,
                        arcs : *arc) -> void_cookie;

unsafe fn xcb_fill_poly_sizeof (_buffer :  *c_void,
                      points_len :  u32) -> c_int;

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
unsafe fn xcb_fill_poly_checked (c : *connection,
                                 drawable :  drawable,
                                 gc :  gcontext,
                                 shape :  u8,
                                 coordinate_mode :  u8,
                                 points_len :  u32,
                                 points : *point) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_fill_poly (c : *connection,
                         drawable :  drawable,
                         gc :  gcontext,
                         shape :  u8,
                         coordinate_mode :  u8,
                         points_len :  u32,
                         points : *point) -> void_cookie;

unsafe fn xcb_poly_fill_rectangle_sizeof (_buffer :  *c_void,
                                rectangles_len :  u32) -> c_int;

/**
 * Fills rectangles
 *
 * @param c The connection
 * @param drawable The drawable (Window or Pixmap) to draw on.
 * @param gc The graphics context to use.
 * \n
 * The following graphics context components are used: function, plane-mask,
 * fill-style, subwindow-mode, clip-x-origin, clip-y-origin, and clip-mask.
 * \n
 * The following graphics context mode-dependent components are used:
 * foreground, background, tile, stipple, tile-stipple-x-origin, and
 * tile-stipple-y-origin.
 * @param rectangles_len The number of `xcb_rectangle_t` structures in \a rectangles.
 * @param rectangles The rectangles to fill.
 * @return A cookie
 *
 * Fills the specified rectangle(s) in the order listed in the array. For any
 * given rectangle, each pixel is not drawn more than once. If rectangles
 * intersect, the intersecting pixels are drawn multiple times.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
unsafe fn xcb_poly_fill_rectangle_checked (c : *connection,
                                           drawable :  drawable,
                                           gc :  gcontext,
                                           rectangles_len :  u32,
                                           rectangles : *rectangle) -> void_cookie;

/**
 * Fills rectangles
 *
 * @param c The connection
 * @param drawable The drawable (Window or Pixmap) to draw on.
 * @param gc The graphics context to use.
 * \n
 * The following graphics context components are used: function, plane-mask,
 * fill-style, subwindow-mode, clip-x-origin, clip-y-origin, and clip-mask.
 * \n
 * The following graphics context mode-dependent components are used:
 * foreground, background, tile, stipple, tile-stipple-x-origin, and
 * tile-stipple-y-origin.
 * @param rectangles_len The number of `xcb_rectangle_t` structures in \a rectangles.
 * @param rectangles The rectangles to fill.
 * @return A cookie
 *
 * Fills the specified rectangle(s) in the order listed in the array. For any
 * given rectangle, each pixel is not drawn more than once. If rectangles
 * intersect, the intersecting pixels are drawn multiple times.
 * 
 */
unsafe fn xcb_poly_fill_rectangle (c : *connection,
                                   drawable :  drawable,
                                   gc :  gcontext,
                                   rectangles_len :  u32,
                                   rectangles : *rectangle) -> void_cookie;

unsafe fn xcb_poly_fill_arc_sizeof (_buffer :  *c_void,
                          arcs_len :  u32) -> c_int;

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
unsafe fn xcb_poly_fill_arc_checked (c : *connection,
                                     drawable :  drawable,
                                     gc :  gcontext,
                                     arcs_len :  u32,
                                     arcs : *arc) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_poly_fill_arc (c : *connection,
                             drawable :  drawable,
                             gc :  gcontext,
                             arcs_len :  u32,
                             arcs : *arc) -> void_cookie;

unsafe fn xcb_put_image_sizeof (_buffer :  *c_void,
                      data_len :  u32) -> c_int;

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
unsafe fn xcb_put_image_checked (c : *connection,
                                 format :  u8,
                                 drawable :  drawable,
                                 gc :  gcontext,
                                 width :  u16,
                                 height :  u16,
                                 dst_x :  i16,
                                 dst_y :  i16,
                                 left_pad :  u8,
                                 depth :  u8,
                                 data_len :  u32,
                                 data : *u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_put_image (c : *connection,
                         format :  u8,
                         drawable :  drawable,
                         gc :  gcontext,
                         width :  u16,
                         height :  u16,
                         dst_x :  i16,
                         dst_y :  i16,
                         left_pad :  u8,
                         depth :  u8,
                         data_len :  u32,
                         data : *u8) -> void_cookie;

unsafe fn xcb_get_image_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_get_image (c : *connection,
                         format :  u8,
                         drawable :  drawable,
                         x :  i16,
                         y :  i16,
                         width :  u16,
                         height :  u16,
                         plane_mask :  u32) -> get_image_cookie;

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
unsafe fn xcb_get_image_unchecked (c : *connection,
                                   format :  u8,
                                   drawable :  drawable,
                                   x :  i16,
                                   y :  i16,
                                   width :  u16,
                                   height :  u16,
                                   plane_mask :  u32) -> get_image_cookie;

unsafe fn xcb_get_image_data (R : *get_image_reply) -> *u8;


unsafe fn xcb_get_image_data_length (R : *get_image_reply) -> c_int;


unsafe fn xcb_get_image_data_end (R : *get_image_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_get_image_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_get_image_reply (c : *connection,
                               cookie : get_image_cookie,
                               e : **generic_error) -> *get_image_reply;

unsafe fn xcb_poly_text_8_sizeof (_buffer :  *c_void,
                        items_len :  u32) -> c_int;

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
unsafe fn xcb_poly_text_8_checked (c : *connection,
                                   drawable :  drawable,
                                   gc :  gcontext,
                                   x :  i16,
                                   y :  i16,
                                   items_len :  u32,
                                   items : *u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_poly_text_8 (c : *connection,
                           drawable :  drawable,
                           gc :  gcontext,
                           x :  i16,
                           y :  i16,
                           items_len :  u32,
                           items : *u8) -> void_cookie;

unsafe fn xcb_poly_text_16_sizeof (_buffer :  *c_void,
                         items_len :  u32) -> c_int;

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
unsafe fn xcb_poly_text_16_checked (c : *connection,
                                    drawable :  drawable,
                                    gc :  gcontext,
                                    x :  i16,
                                    y :  i16,
                                    items_len :  u32,
                                    items : *u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_poly_text_16 (c : *connection,
                            drawable :  drawable,
                            gc :  gcontext,
                            x :  i16,
                            y :  i16,
                            items_len :  u32,
                            items : *u8) -> void_cookie;

unsafe fn xcb_image_text_8_sizeof (_buffer :  *c_void) -> c_int;

/**
 * Draws text
 *
 * @param c The connection
 * @param string_len The length of the \a string. Note that this parameter limited by 255 due to
 * using 8 bits!
 * @param drawable The drawable (Window or Pixmap) to draw text on.
 * @param gc The graphics context to use.
 * \n
 * The following graphics context components are used: plane-mask, foreground,
 * background, font, subwindow-mode, clip-x-origin, clip-y-origin, and clip-mask.
 * @param x The x coordinate of the first character, relative to the origin of \a drawable.
 * @param y The y coordinate of the first character, relative to the origin of \a drawable.
 * @param string The string to draw. Only the first 255 characters are relevant due to the data
 * type of \a string_len.
 * @return A cookie
 *
 * Fills the destination rectangle with the background pixel from \a gc, then
 * paints the text with the foreground pixel from \a gc. The upper-left corner of
 * the filled rectangle is at [x, y - font-ascent]. The width is overall-width,
 * the height is font-ascent + font-descent. The overall-width, font-ascent and
 * font-descent are as returned by `xcb_query_text_extents` (TODO).
 * 
 * Note that using X core fonts is deprecated (but still supported) in favor of
 * client-side rendering using Xft.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
unsafe fn xcb_image_text_8_checked (c : *connection,
                                    string_len :  u8,
                                    drawable :  drawable,
                                    gc :  gcontext,
                                    x :  i16,
                                    y :  i16,
                                    string : *c_char) -> void_cookie;

/**
 * Draws text
 *
 * @param c The connection
 * @param string_len The length of the \a string. Note that this parameter limited by 255 due to
 * using 8 bits!
 * @param drawable The drawable (Window or Pixmap) to draw text on.
 * @param gc The graphics context to use.
 * \n
 * The following graphics context components are used: plane-mask, foreground,
 * background, font, subwindow-mode, clip-x-origin, clip-y-origin, and clip-mask.
 * @param x The x coordinate of the first character, relative to the origin of \a drawable.
 * @param y The y coordinate of the first character, relative to the origin of \a drawable.
 * @param string The string to draw. Only the first 255 characters are relevant due to the data
 * type of \a string_len.
 * @return A cookie
 *
 * Fills the destination rectangle with the background pixel from \a gc, then
 * paints the text with the foreground pixel from \a gc. The upper-left corner of
 * the filled rectangle is at [x, y - font-ascent]. The width is overall-width,
 * the height is font-ascent + font-descent. The overall-width, font-ascent and
 * font-descent are as returned by `xcb_query_text_extents` (TODO).
 * 
 * Note that using X core fonts is deprecated (but still supported) in favor of
 * client-side rendering using Xft.
 * 
 */
unsafe fn xcb_image_text_8 (c : *connection,
                            string_len :  u8,
                            drawable :  drawable,
                            gc :  gcontext,
                            x :  i16,
                            y :  i16,
                            string : *c_char) -> void_cookie;

unsafe fn xcb_image_text_16_sizeof (_buffer :  *c_void) -> c_int;

/**
 * Draws text
 *
 * @param c The connection
 * @param string_len The length of the \a string in characters. Note that this parameter limited by
 * 255 due to using 8 bits!
 * @param drawable The drawable (Window or Pixmap) to draw text on.
 * @param gc The graphics context to use.
 * \n
 * The following graphics context components are used: plane-mask, foreground,
 * background, font, subwindow-mode, clip-x-origin, clip-y-origin, and clip-mask.
 * @param x The x coordinate of the first character, relative to the origin of \a drawable.
 * @param y The y coordinate of the first character, relative to the origin of \a drawable.
 * @param string The string to draw. Only the first 255 characters are relevant due to the data
 * type of \a string_len. Every character uses 2 bytes (hence the 16 in this
 * request's name).
 * @return A cookie
 *
 * Fills the destination rectangle with the background pixel from \a gc, then
 * paints the text with the foreground pixel from \a gc. The upper-left corner of
 * the filled rectangle is at [x, y - font-ascent]. The width is overall-width,
 * the height is font-ascent + font-descent. The overall-width, font-ascent and
 * font-descent are as returned by `xcb_query_text_extents` (TODO).
 * 
 * Note that using X core fonts is deprecated (but still supported) in favor of
 * client-side rendering using Xft.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
unsafe fn xcb_image_text_16_checked (c : *connection,
                                     string_len :  u8,
                                     drawable :  drawable,
                                     gc :  gcontext,
                                     x :  i16,
                                     y :  i16,
                                     string : *char2b) -> void_cookie;

/**
 * Draws text
 *
 * @param c The connection
 * @param string_len The length of the \a string in characters. Note that this parameter limited by
 * 255 due to using 8 bits!
 * @param drawable The drawable (Window or Pixmap) to draw text on.
 * @param gc The graphics context to use.
 * \n
 * The following graphics context components are used: plane-mask, foreground,
 * background, font, subwindow-mode, clip-x-origin, clip-y-origin, and clip-mask.
 * @param x The x coordinate of the first character, relative to the origin of \a drawable.
 * @param y The y coordinate of the first character, relative to the origin of \a drawable.
 * @param string The string to draw. Only the first 255 characters are relevant due to the data
 * type of \a string_len. Every character uses 2 bytes (hence the 16 in this
 * request's name).
 * @return A cookie
 *
 * Fills the destination rectangle with the background pixel from \a gc, then
 * paints the text with the foreground pixel from \a gc. The upper-left corner of
 * the filled rectangle is at [x, y - font-ascent]. The width is overall-width,
 * the height is font-ascent + font-descent. The overall-width, font-ascent and
 * font-descent are as returned by `xcb_query_text_extents` (TODO).
 * 
 * Note that using X core fonts is deprecated (but still supported) in favor of
 * client-side rendering using Xft.
 * 
 */
unsafe fn xcb_image_text_16 (c : *connection,
                             string_len :  u8,
                             drawable :  drawable,
                             gc :  gcontext,
                             x :  i16,
                             y :  i16,
                             string : *char2b) -> void_cookie;

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
unsafe fn xcb_create_colormap_checked (c : *connection,
                                       alloc :  u8,
                                       mid :  colormap,
                                       window :  window,
                                       visual :  visualid) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_create_colormap (c : *connection,
                               alloc :  u8,
                               mid :  colormap,
                               window :  window,
                               visual :  visualid) -> void_cookie;

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
unsafe fn xcb_free_colormap_checked (c : *connection,
                                     cmap :  colormap) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_free_colormap (c : *connection,
                             cmap :  colormap) -> void_cookie;

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
unsafe fn xcb_copy_colormap_and_free_checked (c : *connection,
                                              mid :  colormap,
                                              src_cmap :  colormap) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_copy_colormap_and_free (c : *connection,
                                      mid :  colormap,
                                      src_cmap :  colormap) -> void_cookie;

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
unsafe fn xcb_install_colormap_checked (c : *connection,
                                        cmap :  colormap) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_install_colormap (c : *connection,
                                cmap :  colormap) -> void_cookie;

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
unsafe fn xcb_uninstall_colormap_checked (c : *connection,
                                          cmap :  colormap) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_uninstall_colormap (c : *connection,
                                  cmap :  colormap) -> void_cookie;

unsafe fn xcb_list_installed_colormaps_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_list_installed_colormaps (c : *connection,
                                        window :  window) -> list_installed_colormaps_cookie;

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
unsafe fn xcb_list_installed_colormaps_unchecked (c : *connection,
                                                  window :  window) -> list_installed_colormaps_cookie;

unsafe fn xcb_list_installed_colormaps_cmaps (R : *list_installed_colormaps_reply) -> *colormap;


unsafe fn xcb_list_installed_colormaps_cmaps_length (R : *list_installed_colormaps_reply) -> c_int;


unsafe fn xcb_list_installed_colormaps_cmaps_end (R : *list_installed_colormaps_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_list_installed_colormaps_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_list_installed_colormaps_reply (c : *connection,
                                              cookie : list_installed_colormaps_cookie,
                                              e : **generic_error) -> *list_installed_colormaps_reply;

/**
 * Allocate a color
 *
 * @param c The connection
 * @param cmap TODO
 * @param red The red value of your color.
 * @param green The green value of your color.
 * @param blue The blue value of your color.
 * @return A cookie
 *
 * Allocates a read-only colormap entry corresponding to the closest RGB value
 * supported by the hardware. If you are using TrueColor, you can take a shortcut
 * and directly calculate the color pixel value to avoid the round trip. But, for
 * example, on 16-bit color setups (VNC), you can easily get the closest supported
 * RGB value to the RGB value you are specifying.
 * 
 */
unsafe fn xcb_alloc_color (c : *connection,
                           cmap :  colormap,
                           red :  u16,
                           green :  u16,
                           blue :  u16) -> alloc_color_cookie;

/**
 * Allocate a color
 *
 * @param c The connection
 * @param cmap TODO
 * @param red The red value of your color.
 * @param green The green value of your color.
 * @param blue The blue value of your color.
 * @return A cookie
 *
 * Allocates a read-only colormap entry corresponding to the closest RGB value
 * supported by the hardware. If you are using TrueColor, you can take a shortcut
 * and directly calculate the color pixel value to avoid the round trip. But, for
 * example, on 16-bit color setups (VNC), you can easily get the closest supported
 * RGB value to the RGB value you are specifying.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
unsafe fn xcb_alloc_color_unchecked (c : *connection,
                                     cmap :  colormap,
                                     red :  u16,
                                     green :  u16,
                                     blue :  u16) -> alloc_color_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_alloc_color_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_alloc_color_reply (c : *connection,
                                 cookie : alloc_color_cookie,
                                 e : **generic_error) -> *alloc_color_reply;

unsafe fn xcb_alloc_named_color_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_alloc_named_color (c : *connection,
                                 cmap :  colormap,
                                 name_len :  u16,
                                 name : *c_char) -> alloc_named_color_cookie;

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
unsafe fn xcb_alloc_named_color_unchecked (c : *connection,
                                           cmap :  colormap,
                                           name_len :  u16,
                                           name : *c_char) -> alloc_named_color_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_alloc_named_color_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_alloc_named_color_reply (c : *connection,
                                       cookie : alloc_named_color_cookie,
                                       e : **generic_error) -> *alloc_named_color_reply;

unsafe fn xcb_alloc_color_cells_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_alloc_color_cells (c : *connection,
                                 contiguous :  u8,
                                 cmap :  colormap,
                                 colors :  u16,
                                 planes :  u16) -> alloc_color_cells_cookie;

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
unsafe fn xcb_alloc_color_cells_unchecked (c : *connection,
                                           contiguous :  u8,
                                           cmap :  colormap,
                                           colors :  u16,
                                           planes :  u16) -> alloc_color_cells_cookie;

unsafe fn xcb_alloc_color_cells_pixels (R : *alloc_color_cells_reply) -> *u32;


unsafe fn xcb_alloc_color_cells_pixels_length (R : *alloc_color_cells_reply) -> c_int;


unsafe fn xcb_alloc_color_cells_pixels_end (R : *alloc_color_cells_reply) -> generic_iterator;

unsafe fn xcb_alloc_color_cells_masks (R : *alloc_color_cells_reply) -> *u32;


unsafe fn xcb_alloc_color_cells_masks_length (R : *alloc_color_cells_reply) -> c_int;


unsafe fn xcb_alloc_color_cells_masks_end (R : *alloc_color_cells_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_alloc_color_cells_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_alloc_color_cells_reply (c : *connection,
                                       cookie : alloc_color_cells_cookie,
                                       e : **generic_error) -> *alloc_color_cells_reply;

unsafe fn xcb_alloc_color_planes_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_alloc_color_planes (c : *connection,
                                  contiguous :  u8,
                                  cmap :  colormap,
                                  colors :  u16,
                                  reds :  u16,
                                  greens :  u16,
                                  blues :  u16) -> alloc_color_planes_cookie;

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
unsafe fn xcb_alloc_color_planes_unchecked (c : *connection,
                                            contiguous :  u8,
                                            cmap :  colormap,
                                            colors :  u16,
                                            reds :  u16,
                                            greens :  u16,
                                            blues :  u16) -> alloc_color_planes_cookie;

unsafe fn xcb_alloc_color_planes_pixels (R : *alloc_color_planes_reply) -> *u32;


unsafe fn xcb_alloc_color_planes_pixels_length (R : *alloc_color_planes_reply) -> c_int;


unsafe fn xcb_alloc_color_planes_pixels_end (R : *alloc_color_planes_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_alloc_color_planes_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_alloc_color_planes_reply (c : *connection,
                                        cookie : alloc_color_planes_cookie,
                                        e : **generic_error) -> *alloc_color_planes_reply;

unsafe fn xcb_free_colors_sizeof (_buffer :  *c_void,
                        pixels_len :  u32) -> c_int;

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
unsafe fn xcb_free_colors_checked (c : *connection,
                                   cmap :  colormap,
                                   plane_mask :  u32,
                                   pixels_len :  u32,
                                   pixels : *u32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_free_colors (c : *connection,
                           cmap :  colormap,
                           plane_mask :  u32,
                           pixels_len :  u32,
                           pixels : *u32) -> void_cookie;

/**
 * Get the next element of the iterator
 * @param i Pointer to a coloritem_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(coloritem)
 *
 *
 */
unsafe fn xcb_coloritem_next (i:*coloritem_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An coloritem_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_coloritem_end (i:coloritem_iterator) -> generic_iterator;

unsafe fn xcb_store_colors_sizeof (_buffer :  *c_void,
                         items_len :  u32) -> c_int;

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
unsafe fn xcb_store_colors_checked (c : *connection,
                                    cmap :  colormap,
                                    items_len :  u32,
                                    items : *coloritem) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_store_colors (c : *connection,
                            cmap :  colormap,
                            items_len :  u32,
                            items : *coloritem) -> void_cookie;

unsafe fn xcb_store_named_color_sizeof (_buffer :  *c_void) -> c_int;

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
unsafe fn xcb_store_named_color_checked (c : *connection,
                                         flags :  u8,
                                         cmap :  colormap,
                                         pixel :  u32,
                                         name_len :  u16,
                                         name : *c_char) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_store_named_color (c : *connection,
                                 flags :  u8,
                                 cmap :  colormap,
                                 pixel :  u32,
                                 name_len :  u16,
                                 name : *c_char) -> void_cookie;

/**
 * Get the next element of the iterator
 * @param i Pointer to a rgb_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(rgb)
 *
 *
 */
unsafe fn xcb_rgb_next (i:*rgb_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An rgb_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_rgb_end (i:rgb_iterator) -> generic_iterator;

unsafe fn xcb_query_colors_sizeof (_buffer :  *c_void,
                         pixels_len :  u32) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_query_colors (c : *connection,
                            cmap :  colormap,
                            pixels_len :  u32,
                            pixels : *u32) -> query_colors_cookie;

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
unsafe fn xcb_query_colors_unchecked (c : *connection,
                                      cmap :  colormap,
                                      pixels_len :  u32,
                                      pixels : *u32) -> query_colors_cookie;

unsafe fn xcb_query_colors_colors (R : *query_colors_reply) -> *rgb;


unsafe fn xcb_query_colors_colors_length (R : *query_colors_reply) -> c_int;

unsafe fn xcb_query_colors_colors_iterator (R : *query_colors_reply) -> rgb_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_query_colors_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_query_colors_reply (c : *connection,
                                  cookie : query_colors_cookie,
                                  e : **generic_error) -> *query_colors_reply;

unsafe fn xcb_lookup_color_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_lookup_color (c : *connection,
                            cmap :  colormap,
                            name_len :  u16,
                            name : *c_char) -> lookup_color_cookie;

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
unsafe fn xcb_lookup_color_unchecked (c : *connection,
                                      cmap :  colormap,
                                      name_len :  u16,
                                      name : *c_char) -> lookup_color_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_lookup_color_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_lookup_color_reply (c : *connection,
                                  cookie : lookup_color_cookie,
                                  e : **generic_error) -> *lookup_color_reply;

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
unsafe fn xcb_create_cursor_checked (c : *connection,
                                     cid :  cursor,
                                     source :  pixmap,
                                     mask :  pixmap,
                                     fore_red :  u16,
                                     fore_green :  u16,
                                     fore_blue :  u16,
                                     back_red :  u16,
                                     back_green :  u16,
                                     back_blue :  u16,
                                     x :  u16,
                                     y :  u16) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_create_cursor (c : *connection,
                             cid :  cursor,
                             source :  pixmap,
                             mask :  pixmap,
                             fore_red :  u16,
                             fore_green :  u16,
                             fore_blue :  u16,
                             back_red :  u16,
                             back_green :  u16,
                             back_blue :  u16,
                             x :  u16,
                             y :  u16) -> void_cookie;

/**
 * create cursor
 *
 * @param c The connection
 * @param cid The ID with which you will refer to the cursor, created by `xcb_generate_id`.
 * @param source_font In which font to look for the cursor glyph.
 * @param mask_font In which font to look for the mask glyph.
 * @param source_char The glyph of \a source_font to use.
 * @param mask_char The glyph of \a mask_font to use as a mask: Pixels which are set to 1 define
 * which source pixels are displayed. All pixels which are set to 0 are not
 * displayed.
 * @param fore_red The red value of the foreground color.
 * @param fore_green The green value of the foreground color.
 * @param fore_blue The blue value of the foreground color.
 * @param back_red The red value of the background color.
 * @param back_green The green value of the background color.
 * @param back_blue The blue value of the background color.
 * @return A cookie
 *
 * Creates a cursor from a font glyph. X provides a set of standard cursor shapes
 * in a special font named cursor. Applications are encouraged to use this
 * interface for their cursors because the font can be customized for the
 * individual display type.
 * 
 * All pixels which are set to 1 in the source will use the foreground color (as
 * specified by \a fore_red, \a fore_green and \a fore_blue). All pixels set to 0
 * will use the background color (as specified by \a back_red, \a back_green and
 * \a back_blue).
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
unsafe fn xcb_create_glyph_cursor_checked (c : *connection,
                                           cid :  cursor,
                                           source_font :  font,
                                           mask_font :  font,
                                           source_char :  u16,
                                           mask_char :  u16,
                                           fore_red :  u16,
                                           fore_green :  u16,
                                           fore_blue :  u16,
                                           back_red :  u16,
                                           back_green :  u16,
                                           back_blue :  u16) -> void_cookie;

/**
 * create cursor
 *
 * @param c The connection
 * @param cid The ID with which you will refer to the cursor, created by `xcb_generate_id`.
 * @param source_font In which font to look for the cursor glyph.
 * @param mask_font In which font to look for the mask glyph.
 * @param source_char The glyph of \a source_font to use.
 * @param mask_char The glyph of \a mask_font to use as a mask: Pixels which are set to 1 define
 * which source pixels are displayed. All pixels which are set to 0 are not
 * displayed.
 * @param fore_red The red value of the foreground color.
 * @param fore_green The green value of the foreground color.
 * @param fore_blue The blue value of the foreground color.
 * @param back_red The red value of the background color.
 * @param back_green The green value of the background color.
 * @param back_blue The blue value of the background color.
 * @return A cookie
 *
 * Creates a cursor from a font glyph. X provides a set of standard cursor shapes
 * in a special font named cursor. Applications are encouraged to use this
 * interface for their cursors because the font can be customized for the
 * individual display type.
 * 
 * All pixels which are set to 1 in the source will use the foreground color (as
 * specified by \a fore_red, \a fore_green and \a fore_blue). All pixels set to 0
 * will use the background color (as specified by \a back_red, \a back_green and
 * \a back_blue).
 * 
 */
unsafe fn xcb_create_glyph_cursor (c : *connection,
                                   cid :  cursor,
                                   source_font :  font,
                                   mask_font :  font,
                                   source_char :  u16,
                                   mask_char :  u16,
                                   fore_red :  u16,
                                   fore_green :  u16,
                                   fore_blue :  u16,
                                   back_red :  u16,
                                   back_green :  u16,
                                   back_blue :  u16) -> void_cookie;

/**
 * Deletes a cursor
 *
 * @param c The connection
 * @param cursor The cursor to destroy.
 * @return A cookie
 *
 * Deletes the association between the cursor resource ID and the specified
 * cursor. The cursor is freed when no other resource references it.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
unsafe fn xcb_free_cursor_checked (c : *connection,
                                   cursor :  cursor) -> void_cookie;

/**
 * Deletes a cursor
 *
 * @param c The connection
 * @param cursor The cursor to destroy.
 * @return A cookie
 *
 * Deletes the association between the cursor resource ID and the specified
 * cursor. The cursor is freed when no other resource references it.
 * 
 */
unsafe fn xcb_free_cursor (c : *connection,
                           cursor :  cursor) -> void_cookie;

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
unsafe fn xcb_recolor_cursor_checked (c : *connection,
                                      cursor :  cursor,
                                      fore_red :  u16,
                                      fore_green :  u16,
                                      fore_blue :  u16,
                                      back_red :  u16,
                                      back_green :  u16,
                                      back_blue :  u16) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_recolor_cursor (c : *connection,
                              cursor :  cursor,
                              fore_red :  u16,
                              fore_green :  u16,
                              fore_blue :  u16,
                              back_red :  u16,
                              back_green :  u16,
                              back_blue :  u16) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_query_best_size (c : *connection,
                               class :  u8,
                               drawable :  drawable,
                               width :  u16,
                               height :  u16) -> query_best_size_cookie;

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
unsafe fn xcb_query_best_size_unchecked (c : *connection,
                                         class :  u8,
                                         drawable :  drawable,
                                         width :  u16,
                                         height :  u16) -> query_best_size_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_query_best_size_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_query_best_size_reply (c : *connection,
                                     cookie : query_best_size_cookie,
                                     e : **generic_error) -> *query_best_size_reply;

unsafe fn xcb_query_extension_sizeof (_buffer :  *c_void) -> c_int;

/**
 * check if extension is present
 *
 * @param c The connection
 * @param name_len The length of \a name in bytes.
 * @param name The name of the extension to query, for example "RANDR". This is case
 * sensitive!
 * @return A cookie
 *
 * Determines if the specified extension is present on this X11 server.
 * 
 * Every extension has a unique `major_opcode` to identify requests, the minor
 * opcodes and request formats are extension-specific. If the extension provides
 * events and errors, the `first_event` and `first_error` fields in the reply are
 * set accordingly.
 * 
 * There should rarely be a need to use this request directly, XCB provides the
 * `xcb_get_extension_data` function instead.
 * 
 */
unsafe fn xcb_query_extension (c : *connection,
                               name_len :  u16,
                               name : *c_char) -> query_extension_cookie;

/**
 * check if extension is present
 *
 * @param c The connection
 * @param name_len The length of \a name in bytes.
 * @param name The name of the extension to query, for example "RANDR". This is case
 * sensitive!
 * @return A cookie
 *
 * Determines if the specified extension is present on this X11 server.
 * 
 * Every extension has a unique `major_opcode` to identify requests, the minor
 * opcodes and request formats are extension-specific. If the extension provides
 * events and errors, the `first_event` and `first_error` fields in the reply are
 * set accordingly.
 * 
 * There should rarely be a need to use this request directly, XCB provides the
 * `xcb_get_extension_data` function instead.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
unsafe fn xcb_query_extension_unchecked (c : *connection,
                                         name_len :  u16,
                                         name : *c_char) -> query_extension_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_query_extension_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_query_extension_reply (c : *connection,
                                     cookie : query_extension_cookie,
                                     e : **generic_error) -> *query_extension_reply;

unsafe fn xcb_list_extensions_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_list_extensions (c : *connection) -> list_extensions_cookie;

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
unsafe fn xcb_list_extensions_unchecked (c : *connection) -> list_extensions_cookie;


unsafe fn xcb_list_extensions_names_length (R : *list_extensions_reply) -> c_int;

unsafe fn xcb_list_extensions_names_iterator (R : *list_extensions_reply) -> str_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_list_extensions_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_list_extensions_reply (c : *connection,
                                     cookie : list_extensions_cookie,
                                     e : **generic_error) -> *list_extensions_reply;

unsafe fn xcb_change_keyboard_mapping_sizeof (_buffer :  *c_void) -> c_int;

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
unsafe fn xcb_change_keyboard_mapping_checked (c : *connection,
                                               keycode_count :  u8,
                                               first_keycode :  keycode,
                                               keysyms_per_keycode :  u8,
                                               keysyms : *keysym) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_change_keyboard_mapping (c : *connection,
                                       keycode_count :  u8,
                                       first_keycode :  keycode,
                                       keysyms_per_keycode :  u8,
                                       keysyms : *keysym) -> void_cookie;

unsafe fn xcb_get_keyboard_mapping_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_get_keyboard_mapping (c : *connection,
                                    first_keycode :  keycode,
                                    count :  u8) -> get_keyboard_mapping_cookie;

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
unsafe fn xcb_get_keyboard_mapping_unchecked (c : *connection,
                                              first_keycode :  keycode,
                                              count :  u8) -> get_keyboard_mapping_cookie;

unsafe fn xcb_get_keyboard_mapping_keysyms (R : *get_keyboard_mapping_reply) -> *keysym;


unsafe fn xcb_get_keyboard_mapping_keysyms_length (R : *get_keyboard_mapping_reply) -> c_int;


unsafe fn xcb_get_keyboard_mapping_keysyms_end (R : *get_keyboard_mapping_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_get_keyboard_mapping_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_get_keyboard_mapping_reply (c : *connection,
                                          cookie : get_keyboard_mapping_cookie,
                                          e : **generic_error) -> *get_keyboard_mapping_reply;

unsafe fn xcb_change_keyboard_control_sizeof (_buffer :  *c_void) -> c_int;

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
unsafe fn xcb_change_keyboard_control_checked (c : *connection,
                                               value_mask :  u32,
                                               value_list : *u32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_change_keyboard_control (c : *connection,
                                       value_mask :  u32,
                                       value_list : *u32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_get_keyboard_control (c : *connection) -> get_keyboard_control_cookie;

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
unsafe fn xcb_get_keyboard_control_unchecked (c : *connection) -> get_keyboard_control_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_get_keyboard_control_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_get_keyboard_control_reply (c : *connection,
                                          cookie : get_keyboard_control_cookie,
                                          e : **generic_error) -> *get_keyboard_control_reply;

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
unsafe fn xcb_bell_checked (c : *connection,
                            percent :  i8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_bell (c : *connection,
                    percent :  i8) -> void_cookie;

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
unsafe fn xcb_change_pointer_control_checked (c : *connection,
                                              acceleration_numerator :  i16,
                                              acceleration_denominator :  i16,
                                              threshold :  i16,
                                              do_acceleration :  u8,
                                              do_threshold :  u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_change_pointer_control (c : *connection,
                                      acceleration_numerator :  i16,
                                      acceleration_denominator :  i16,
                                      threshold :  i16,
                                      do_acceleration :  u8,
                                      do_threshold :  u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_get_pointer_control (c : *connection) -> get_pointer_control_cookie;

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
unsafe fn xcb_get_pointer_control_unchecked (c : *connection) -> get_pointer_control_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_get_pointer_control_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_get_pointer_control_reply (c : *connection,
                                         cookie : get_pointer_control_cookie,
                                         e : **generic_error) -> *get_pointer_control_reply;

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
unsafe fn xcb_set_screen_saver_checked (c : *connection,
                                        timeout :  i16,
                                        interval :  i16,
                                        prefer_blanking :  u8,
                                        allow_exposures :  u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_set_screen_saver (c : *connection,
                                timeout :  i16,
                                interval :  i16,
                                prefer_blanking :  u8,
                                allow_exposures :  u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_get_screen_saver (c : *connection) -> get_screen_saver_cookie;

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
unsafe fn xcb_get_screen_saver_unchecked (c : *connection) -> get_screen_saver_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_get_screen_saver_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_get_screen_saver_reply (c : *connection,
                                      cookie : get_screen_saver_cookie,
                                      e : **generic_error) -> *get_screen_saver_reply;

unsafe fn xcb_change_hosts_sizeof (_buffer :  *c_void) -> c_int;

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
unsafe fn xcb_change_hosts_checked (c : *connection,
                                    mode :  u8,
                                    family :  u8,
                                    address_len :  u16,
                                    address : *u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_change_hosts (c : *connection,
                            mode :  u8,
                            family :  u8,
                            address_len :  u16,
                            address : *u8) -> void_cookie;

unsafe fn xcb_host_sizeof (_buffer :  *c_void) -> c_int;

unsafe fn xcb_host_address (R : *host) -> *u8;


unsafe fn xcb_host_address_length (R : *host) -> c_int;


unsafe fn xcb_host_address_end (R : *host) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a host_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(host)
 *
 *
 */
unsafe fn xcb_host_next (i:*host_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An host_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_host_end (i:host_iterator) -> generic_iterator;

unsafe fn xcb_list_hosts_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_list_hosts (c : *connection) -> list_hosts_cookie;

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
unsafe fn xcb_list_hosts_unchecked (c : *connection) -> list_hosts_cookie;


unsafe fn xcb_list_hosts_hosts_length (R : *list_hosts_reply) -> c_int;

unsafe fn xcb_list_hosts_hosts_iterator (R : *list_hosts_reply) -> host_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_list_hosts_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_list_hosts_reply (c : *connection,
                                cookie : list_hosts_cookie,
                                e : **generic_error) -> *list_hosts_reply;

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
unsafe fn xcb_set_access_control_checked (c : *connection,
                                          mode :  u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_set_access_control (c : *connection,
                                  mode :  u8) -> void_cookie;

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
unsafe fn xcb_set_close_down_mode_checked (c : *connection,
                                           mode :  u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_set_close_down_mode (c : *connection,
                                   mode :  u8) -> void_cookie;

/**
 * kills a client
 *
 * @param c The connection
 * @param resource Any resource belonging to the client (for example a Window), used to identify
 * the client connection.
 * \n
 * The special value of `XCB_KILL_ALL_TEMPORARY`, the resources of all clients
 * that have terminated in `RetainTemporary` (TODO) are destroyed.
 * @return A cookie
 *
 * Forces a close down of the client that created the specified \a resource.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
unsafe fn xcb_kill_client_checked (c : *connection,
                                   resource :  u32) -> void_cookie;

/**
 * kills a client
 *
 * @param c The connection
 * @param resource Any resource belonging to the client (for example a Window), used to identify
 * the client connection.
 * \n
 * The special value of `XCB_KILL_ALL_TEMPORARY`, the resources of all clients
 * that have terminated in `RetainTemporary` (TODO) are destroyed.
 * @return A cookie
 *
 * Forces a close down of the client that created the specified \a resource.
 * 
 */
unsafe fn xcb_kill_client (c : *connection,
                           resource :  u32) -> void_cookie;

unsafe fn xcb_rotate_properties_sizeof (_buffer :  *c_void) -> c_int;

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
unsafe fn xcb_rotate_properties_checked (c : *connection,
                                         window :  window,
                                         atoms_len :  u16,
                                         delta :  i16,
                                         atoms : *atom) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_rotate_properties (c : *connection,
                                 window :  window,
                                 atoms_len :  u16,
                                 delta :  i16,
                                 atoms : *atom) -> void_cookie;

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
unsafe fn xcb_force_screen_saver_checked (c : *connection,
                                          mode :  u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_force_screen_saver (c : *connection,
                                  mode :  u8) -> void_cookie;

unsafe fn xcb_set_pointer_mapping_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_set_pointer_mapping (c : *connection,
                                   map_len :  u8,
                                   map : *u8) -> set_pointer_mapping_cookie;

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
unsafe fn xcb_set_pointer_mapping_unchecked (c : *connection,
                                             map_len :  u8,
                                             map : *u8) -> set_pointer_mapping_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_set_pointer_mapping_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_set_pointer_mapping_reply (c : *connection,
                                         cookie : set_pointer_mapping_cookie,
                                         e : **generic_error) -> *set_pointer_mapping_reply;

unsafe fn xcb_get_pointer_mapping_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_get_pointer_mapping (c : *connection) -> get_pointer_mapping_cookie;

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
unsafe fn xcb_get_pointer_mapping_unchecked (c : *connection) -> get_pointer_mapping_cookie;

unsafe fn xcb_get_pointer_mapping_map (R : *get_pointer_mapping_reply) -> *u8;


unsafe fn xcb_get_pointer_mapping_map_length (R : *get_pointer_mapping_reply) -> c_int;


unsafe fn xcb_get_pointer_mapping_map_end (R : *get_pointer_mapping_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_get_pointer_mapping_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_get_pointer_mapping_reply (c : *connection,
                                         cookie : get_pointer_mapping_cookie,
                                         e : **generic_error) -> *get_pointer_mapping_reply;

unsafe fn xcb_set_modifier_mapping_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_set_modifier_mapping (c : *connection,
                                    keycodes_per_modifier :  u8,
                                    keycodes : *keycode) -> set_modifier_mapping_cookie;

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
unsafe fn xcb_set_modifier_mapping_unchecked (c : *connection,
                                              keycodes_per_modifier :  u8,
                                              keycodes : *keycode) -> set_modifier_mapping_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_set_modifier_mapping_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_set_modifier_mapping_reply (c : *connection,
                                          cookie : set_modifier_mapping_cookie,
                                          e : **generic_error) -> *set_modifier_mapping_reply;

unsafe fn xcb_get_modifier_mapping_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_get_modifier_mapping (c : *connection) -> get_modifier_mapping_cookie;

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
unsafe fn xcb_get_modifier_mapping_unchecked (c : *connection) -> get_modifier_mapping_cookie;

unsafe fn xcb_get_modifier_mapping_keycodes (R : *get_modifier_mapping_reply) -> *keycode;


unsafe fn xcb_get_modifier_mapping_keycodes_length (R : *get_modifier_mapping_reply) -> c_int;


unsafe fn xcb_get_modifier_mapping_keycodes_end (R : *get_modifier_mapping_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_get_modifier_mapping_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_get_modifier_mapping_reply (c : *connection,
                                          cookie : get_modifier_mapping_cookie,
                                          e : **generic_error) -> *get_modifier_mapping_reply;

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
unsafe fn xcb_no_operation_checked (c : *connection) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_no_operation (c : *connection) -> void_cookie;
}

