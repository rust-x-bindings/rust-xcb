/*
 * This file generated automatically from xfixes.xml by r_client.py.
 * Edit at your peril.
 */

//Make the compiler quiet
#[allow(unused_imports)];
#[allow(non_camel_case_types)];
use core::libc::*;
use ll::base::*;
use ll::xproto;
use ll::render;
use ll::shape;

pub static XFIXES_MAJOR_VERSION : c_uint = 5;
pub static XFIXES_MINOR_VERSION : c_uint = 0;

pub struct query_version_cookie {
    sequence : c_uint
}

/** Opcode for xcb_xfixes_query_version. */
pub static XCB_XFIXES_QUERY_VERSION : c_int = 0;

pub struct query_version_request {
    major_opcode :           u8,
    minor_opcode :           u8,
    length :                 u16,
    client_major_version :   u32,
    client_minor_version :   u32
}

pub struct query_version_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    major_version :   u32,
    minor_version :   u32,
    pad1 :            [u8,..16]
}

pub type save_set_mode = c_uint;//{
    pub static XCB_XFIXES_SAVE_SET_MODE_INSERT : save_set_mode = 1;
    pub static XCB_XFIXES_SAVE_SET_MODE_DELETE : save_set_mode = 2;
//}

pub type save_set_target = c_uint;//{
    pub static XCB_XFIXES_SAVE_SET_TARGET_NEAREST : save_set_target = 1;
    pub static XCB_XFIXES_SAVE_SET_TARGET_ROOT : save_set_target = 2;
//}

pub type save_set_mapping = c_uint;//{
    pub static XCB_XFIXES_SAVE_SET_MAPPING_MAP : save_set_mapping = 1;
    pub static XCB_XFIXES_SAVE_SET_MAPPING_UNMAP : save_set_mapping = 2;
//}

/** Opcode for xcb_xfixes_change_save_set. */
pub static XCB_XFIXES_CHANGE_SAVE_SET : c_int = 1;

pub struct change_save_set_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    mode :           u8,
    target :         u8,
    map :            u8,
    pad0 :           u8,
    window :         xproto::window
}

pub type selection_event = c_uint;//{
    pub static XCB_XFIXES_SELECTION_EVENT_SET_SELECTION_OWNER : selection_event = 1;
    pub static XCB_XFIXES_SELECTION_EVENT_SELECTION_WINDOW_DESTROY : selection_event = 2;
    pub static XCB_XFIXES_SELECTION_EVENT_SELECTION_CLIENT_CLOSE : selection_event = 3;
//}

pub type selection_event_mask = c_uint;//{
    pub static XCB_XFIXES_SELECTION_EVENT_MASK_SET_SELECTION_OWNER : selection_event_mask = 1;
    pub static XCB_XFIXES_SELECTION_EVENT_MASK_SELECTION_WINDOW_DESTROY : selection_event_mask = 2;
    pub static XCB_XFIXES_SELECTION_EVENT_MASK_SELECTION_CLIENT_CLOSE : selection_event_mask = 4;
//}

/** Opcode for xcb_xfixes_selection_notify. */
pub static XCB_XFIXES_SELECTION_NOTIFY : c_int = 0;

pub struct selection_notify_event {
    response_type :         u8,
    subtype :               u8,
    sequence :              u16,
    window :                xproto::window,
    owner :                 xproto::window,
    selection :             xproto::atom,
    timestamp :             xproto::timestamp,
    selection_timestamp :   xproto::timestamp,
    pad0 :                  [u8,..8]
}

/** Opcode for xcb_xfixes_select_selection_input. */
pub static XCB_XFIXES_SELECT_SELECTION_INPUT : c_int = 2;

pub struct select_selection_input_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    window :         xproto::window,
    selection :      xproto::atom,
    event_mask :     u32
}

pub type cursor_notify = c_uint;//{
    pub static XCB_XFIXES_CURSOR_NOTIFY_DISPLAY_CURSOR : cursor_notify = 1;
//}

pub type cursor_notify_mask = c_uint;//{
    pub static XCB_XFIXES_CURSOR_NOTIFY_MASK_DISPLAY_CURSOR : cursor_notify_mask = 1;
//}

/** Opcode for xcb_xfixes_cursor_notify. */
pub static XCB_XFIXES_CURSOR_NOTIFY : c_int = 1;

pub struct cursor_notify_event {
    response_type :   u8,
    subtype :         u8,
    sequence :        u16,
    window :          xproto::window,
    cursor_serial :   u32,
    timestamp :       xproto::timestamp,
    name :            xproto::atom,
    pad0 :            [u8,..12]
}

/** Opcode for xcb_xfixes_select_cursor_input. */
pub static XCB_XFIXES_SELECT_CURSOR_INPUT : c_int = 3;

pub struct select_cursor_input_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    window :         xproto::window,
    event_mask :     u32
}

pub struct get_cursor_image_cookie {
    sequence : c_uint
}

/** Opcode for xcb_xfixes_get_cursor_image. */
pub static XCB_XFIXES_GET_CURSOR_IMAGE : c_int = 4;

pub struct get_cursor_image_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16
}

pub struct get_cursor_image_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    x :               i16,
    y :               i16,
    width :           u16,
    height :          u16,
    xhot :            u16,
    yhot :            u16,
    cursor_serial :   u32,
    pad1 :            [u8,..8]
}

pub type region = u32;

/**
 * @brief region_iterator
 **/
pub struct region_iterator {
    data : *region,
    rem  : c_int,
    index: c_int
}

/** Opcode for xcb_xfixes_bad_region. */
pub static XCB_XFIXES_BAD_REGION : c_int = 0;

pub struct bad_region_error {
    response_type :   u8,
    error_code :      u8,
    sequence :        u16
}

pub type region_enum = c_uint;//{
    pub static XCB_XFIXES_REGION_NONE : region_enum = 1;
//}

/** Opcode for xcb_xfixes_create_region. */
pub static XCB_XFIXES_CREATE_REGION : c_int = 5;

pub struct create_region_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    region :         region
}

/** Opcode for xcb_xfixes_create_region_from_bitmap. */
pub static XCB_XFIXES_CREATE_REGION_FROM_BITMAP : c_int = 6;

pub struct create_region_from_bitmap_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    region :         region,
    bitmap :         xproto::pixmap
}

/** Opcode for xcb_xfixes_create_region_from_window. */
pub static XCB_XFIXES_CREATE_REGION_FROM_WINDOW : c_int = 7;

pub struct create_region_from_window_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    region :         region,
    window :         xproto::window,
    kind :           shape::kind,
    pad0 :           [u8,..3]
}

/** Opcode for xcb_xfixes_create_region_from_gc. */
pub static XCB_XFIXES_CREATE_REGION_FROM_GC : c_int = 8;

pub struct create_region_from_gc_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    region :         region,
    gc :             xproto::gcontext
}

/** Opcode for xcb_xfixes_create_region_from_picture. */
pub static XCB_XFIXES_CREATE_REGION_FROM_PICTURE : c_int = 9;

pub struct create_region_from_picture_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    region :         region,
    picture :        render::picture
}

/** Opcode for xcb_xfixes_destroy_region. */
pub static XCB_XFIXES_DESTROY_REGION : c_int = 10;

pub struct destroy_region_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    region :         region
}

/** Opcode for xcb_xfixes_set_region. */
pub static XCB_XFIXES_SET_REGION : c_int = 11;

pub struct set_region_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    region :         region
}

/** Opcode for xcb_xfixes_copy_region. */
pub static XCB_XFIXES_COPY_REGION : c_int = 12;

pub struct copy_region_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    source :         region,
    destination :    region
}

/** Opcode for xcb_xfixes_union_region. */
pub static XCB_XFIXES_UNION_REGION : c_int = 13;

pub struct union_region_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    source1 :        region,
    source2 :        region,
    destination :    region
}

/** Opcode for xcb_xfixes_intersect_region. */
pub static XCB_XFIXES_INTERSECT_REGION : c_int = 14;

pub struct intersect_region_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    source1 :        region,
    source2 :        region,
    destination :    region
}

/** Opcode for xcb_xfixes_subtract_region. */
pub static XCB_XFIXES_SUBTRACT_REGION : c_int = 15;

pub struct subtract_region_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    source1 :        region,
    source2 :        region,
    destination :    region
}

/** Opcode for xcb_xfixes_invert_region. */
pub static XCB_XFIXES_INVERT_REGION : c_int = 16;

pub struct invert_region_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    source :         region,
    bounds :         xproto::rectangle,
    destination :    region
}

/** Opcode for xcb_xfixes_translate_region. */
pub static XCB_XFIXES_TRANSLATE_REGION : c_int = 17;

pub struct translate_region_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    region :         region,
    dx :             i16,
    dy :             i16
}

/** Opcode for xcb_xfixes_region_extents. */
pub static XCB_XFIXES_REGION_EXTENTS : c_int = 18;

pub struct region_extents_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    source :         region,
    destination :    region
}

pub struct fetch_region_cookie {
    sequence : c_uint
}

/** Opcode for xcb_xfixes_fetch_region. */
pub static XCB_XFIXES_FETCH_REGION : c_int = 19;

pub struct fetch_region_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    region :         region
}

pub struct fetch_region_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    extents :         xproto::rectangle,
    pad1 :            [u8,..16]
}

/** Opcode for xcb_xfixes_set_gc_clip_region. */
pub static XCB_XFIXES_SET_GC_CLIP_REGION : c_int = 20;

pub struct set_gc_clip_region_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    gc :             xproto::gcontext,
    region :         region,
    x_origin :       i16,
    y_origin :       i16
}

/** Opcode for xcb_xfixes_set_window_shape_region. */
pub static XCB_XFIXES_SET_WINDOW_SHAPE_REGION : c_int = 21;

pub struct set_window_shape_region_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    dest :           xproto::window,
    dest_kind :      shape::kind,
    pad0 :           [u8,..3],
    x_offset :       i16,
    y_offset :       i16,
    region :         region
}

/** Opcode for xcb_xfixes_set_picture_clip_region. */
pub static XCB_XFIXES_SET_PICTURE_CLIP_REGION : c_int = 22;

pub struct set_picture_clip_region_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    picture :        render::picture,
    region :         region,
    x_origin :       i16,
    y_origin :       i16
}

/** Opcode for xcb_xfixes_set_cursor_name. */
pub static XCB_XFIXES_SET_CURSOR_NAME : c_int = 23;

pub struct set_cursor_name_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    cursor :         xproto::cursor,
    nbytes :         u16,
    pad0 :           [u8,..2]
}

pub struct get_cursor_name_cookie {
    sequence : c_uint
}

/** Opcode for xcb_xfixes_get_cursor_name. */
pub static XCB_XFIXES_GET_CURSOR_NAME : c_int = 24;

pub struct get_cursor_name_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    cursor :         xproto::cursor
}

pub struct get_cursor_name_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    atom :            xproto::atom,
    nbytes :          u16,
    pad1 :            [u8,..18]
}

pub struct get_cursor_image_and_name_cookie {
    sequence : c_uint
}

/** Opcode for xcb_xfixes_get_cursor_image_and_name. */
pub static XCB_XFIXES_GET_CURSOR_IMAGE_AND_NAME : c_int = 25;

pub struct get_cursor_image_and_name_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16
}

pub struct get_cursor_image_and_name_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    x :               i16,
    y :               i16,
    width :           u16,
    height :          u16,
    xhot :            u16,
    yhot :            u16,
    cursor_serial :   u32,
    cursor_atom :     xproto::atom,
    nbytes :          u16,
    pad1 :            [u8,..2]
}

/** Opcode for xcb_xfixes_change_cursor. */
pub static XCB_XFIXES_CHANGE_CURSOR : c_int = 26;

pub struct change_cursor_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    source :         xproto::cursor,
    destination :    xproto::cursor
}

/** Opcode for xcb_xfixes_change_cursor_by_name. */
pub static XCB_XFIXES_CHANGE_CURSOR_BY_NAME : c_int = 27;

pub struct change_cursor_by_name_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    src :            xproto::cursor,
    nbytes :         u16,
    pad0 :           [u8,..2]
}

/** Opcode for xcb_xfixes_expand_region. */
pub static XCB_XFIXES_EXPAND_REGION : c_int = 28;

pub struct expand_region_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    source :         region,
    destination :    region,
    left :           u16,
    right :          u16,
    top :            u16,
    bottom :         u16
}

/** Opcode for xcb_xfixes_hide_cursor. */
pub static XCB_XFIXES_HIDE_CURSOR : c_int = 29;

pub struct hide_cursor_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    window :         xproto::window
}

/** Opcode for xcb_xfixes_show_cursor. */
pub static XCB_XFIXES_SHOW_CURSOR : c_int = 30;

pub struct show_cursor_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    window :         xproto::window
}
#[link_args="-lxcb-xfixes"]
extern "C" {

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xfixes_query_version (c : *connection,
                                    client_major_version :  u32,
                                    client_minor_version :  u32) -> query_version_cookie;

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
unsafe fn xcb_xfixes_query_version_unchecked (c : *connection,
                                              client_major_version :  u32,
                                              client_minor_version :  u32) -> query_version_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xfixes_query_version_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_xfixes_query_version_reply (c : *connection,
                                          cookie : query_version_cookie,
                                          e : **generic_error) -> *query_version_reply;

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
unsafe fn xcb_xfixes_change_save_set_checked (c : *connection,
                                              mode :  u8,
                                              target :  u8,
                                              map :  u8,
                                              window :  xproto::window) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xfixes_change_save_set (c : *connection,
                                      mode :  u8,
                                      target :  u8,
                                      map :  u8,
                                      window :  xproto::window) -> void_cookie;

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
unsafe fn xcb_xfixes_select_selection_input_checked (c : *connection,
                                                     window :  xproto::window,
                                                     selection :  xproto::atom,
                                                     event_mask :  u32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xfixes_select_selection_input (c : *connection,
                                             window :  xproto::window,
                                             selection :  xproto::atom,
                                             event_mask :  u32) -> void_cookie;

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
unsafe fn xcb_xfixes_select_cursor_input_checked (c : *connection,
                                                  window :  xproto::window,
                                                  event_mask :  u32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xfixes_select_cursor_input (c : *connection,
                                          window :  xproto::window,
                                          event_mask :  u32) -> void_cookie;

unsafe fn xcb_xfixes_get_cursor_image_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xfixes_get_cursor_image (c : *connection) -> get_cursor_image_cookie;

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
unsafe fn xcb_xfixes_get_cursor_image_unchecked (c : *connection) -> get_cursor_image_cookie;

unsafe fn xcb_xfixes_get_cursor_image_cursor_image (R : *get_cursor_image_reply) -> *u32;


unsafe fn xcb_xfixes_get_cursor_image_cursor_image_length (R : *get_cursor_image_reply) -> c_int;


unsafe fn xcb_xfixes_get_cursor_image_cursor_image_end (R : *get_cursor_image_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xfixes_get_cursor_image_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_xfixes_get_cursor_image_reply (c : *connection,
                                             cookie : get_cursor_image_cookie,
                                             e : **generic_error) -> *get_cursor_image_reply;

/**
 * Get the next element of the iterator
 * @param i Pointer to a region_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(region)
 *
 *
 */
unsafe fn xcb_xfixes_region_next (i:*region_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An region_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_xfixes_region_end (i:region_iterator) -> generic_iterator;

unsafe fn xcb_xfixes_create_region_sizeof (_buffer :  *c_void,
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
unsafe fn xcb_xfixes_create_region_checked (c : *connection,
                                            region :  region,
                                            rectangles_len :  u32,
                                            rectangles : *xproto::rectangle) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xfixes_create_region (c : *connection,
                                    region :  region,
                                    rectangles_len :  u32,
                                    rectangles : *xproto::rectangle) -> void_cookie;

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
unsafe fn xcb_xfixes_create_region_from_bitmap_checked (c : *connection,
                                                        region :  region,
                                                        bitmap :  xproto::pixmap) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xfixes_create_region_from_bitmap (c : *connection,
                                                region :  region,
                                                bitmap :  xproto::pixmap) -> void_cookie;

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
unsafe fn xcb_xfixes_create_region_from_window_checked (c : *connection,
                                                        region :  region,
                                                        window :  xproto::window,
                                                        kind :  shape::kind) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xfixes_create_region_from_window (c : *connection,
                                                region :  region,
                                                window :  xproto::window,
                                                kind :  shape::kind) -> void_cookie;

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
unsafe fn xcb_xfixes_create_region_from_gc_checked (c : *connection,
                                                    region :  region,
                                                    gc :  xproto::gcontext) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xfixes_create_region_from_gc (c : *connection,
                                            region :  region,
                                            gc :  xproto::gcontext) -> void_cookie;

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
unsafe fn xcb_xfixes_create_region_from_picture_checked (c : *connection,
                                                         region :  region,
                                                         picture :  render::picture) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xfixes_create_region_from_picture (c : *connection,
                                                 region :  region,
                                                 picture :  render::picture) -> void_cookie;

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
unsafe fn xcb_xfixes_destroy_region_checked (c : *connection,
                                             region :  region) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xfixes_destroy_region (c : *connection,
                                     region :  region) -> void_cookie;

unsafe fn xcb_xfixes_set_region_sizeof (_buffer :  *c_void,
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
unsafe fn xcb_xfixes_set_region_checked (c : *connection,
                                         region :  region,
                                         rectangles_len :  u32,
                                         rectangles : *xproto::rectangle) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xfixes_set_region (c : *connection,
                                 region :  region,
                                 rectangles_len :  u32,
                                 rectangles : *xproto::rectangle) -> void_cookie;

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
unsafe fn xcb_xfixes_copy_region_checked (c : *connection,
                                          source :  region,
                                          destination :  region) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xfixes_copy_region (c : *connection,
                                  source :  region,
                                  destination :  region) -> void_cookie;

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
unsafe fn xcb_xfixes_union_region_checked (c : *connection,
                                           source1 :  region,
                                           source2 :  region,
                                           destination :  region) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xfixes_union_region (c : *connection,
                                   source1 :  region,
                                   source2 :  region,
                                   destination :  region) -> void_cookie;

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
unsafe fn xcb_xfixes_intersect_region_checked (c : *connection,
                                               source1 :  region,
                                               source2 :  region,
                                               destination :  region) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xfixes_intersect_region (c : *connection,
                                       source1 :  region,
                                       source2 :  region,
                                       destination :  region) -> void_cookie;

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
unsafe fn xcb_xfixes_subtract_region_checked (c : *connection,
                                              source1 :  region,
                                              source2 :  region,
                                              destination :  region) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xfixes_subtract_region (c : *connection,
                                      source1 :  region,
                                      source2 :  region,
                                      destination :  region) -> void_cookie;

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
unsafe fn xcb_xfixes_invert_region_checked (c : *connection,
                                            source :  region,
                                            bounds :  xproto::rectangle,
                                            destination :  region) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xfixes_invert_region (c : *connection,
                                    source :  region,
                                    bounds :  xproto::rectangle,
                                    destination :  region) -> void_cookie;

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
unsafe fn xcb_xfixes_translate_region_checked (c : *connection,
                                               region :  region,
                                               dx :  i16,
                                               dy :  i16) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xfixes_translate_region (c : *connection,
                                       region :  region,
                                       dx :  i16,
                                       dy :  i16) -> void_cookie;

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
unsafe fn xcb_xfixes_region_extents_checked (c : *connection,
                                             source :  region,
                                             destination :  region) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xfixes_region_extents (c : *connection,
                                     source :  region,
                                     destination :  region) -> void_cookie;

unsafe fn xcb_xfixes_fetch_region_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xfixes_fetch_region (c : *connection,
                                   region :  region) -> fetch_region_cookie;

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
unsafe fn xcb_xfixes_fetch_region_unchecked (c : *connection,
                                             region :  region) -> fetch_region_cookie;

unsafe fn xcb_xfixes_fetch_region_rectangles (R : *fetch_region_reply) -> *xproto::rectangle;


unsafe fn xcb_xfixes_fetch_region_rectangles_length (R : *fetch_region_reply) -> c_int;

unsafe fn xcb_xfixes_fetch_region_rectangles_iterator (R : *fetch_region_reply) -> xproto::rectangle_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xfixes_fetch_region_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_xfixes_fetch_region_reply (c : *connection,
                                         cookie : fetch_region_cookie,
                                         e : **generic_error) -> *fetch_region_reply;

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
unsafe fn xcb_xfixes_set_gc_clip_region_checked (c : *connection,
                                                 gc :  xproto::gcontext,
                                                 region :  region,
                                                 x_origin :  i16,
                                                 y_origin :  i16) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xfixes_set_gc_clip_region (c : *connection,
                                         gc :  xproto::gcontext,
                                         region :  region,
                                         x_origin :  i16,
                                         y_origin :  i16) -> void_cookie;

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
unsafe fn xcb_xfixes_set_window_shape_region_checked (c : *connection,
                                                      dest :  xproto::window,
                                                      dest_kind :  shape::kind,
                                                      x_offset :  i16,
                                                      y_offset :  i16,
                                                      region :  region) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xfixes_set_window_shape_region (c : *connection,
                                              dest :  xproto::window,
                                              dest_kind :  shape::kind,
                                              x_offset :  i16,
                                              y_offset :  i16,
                                              region :  region) -> void_cookie;

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
unsafe fn xcb_xfixes_set_picture_clip_region_checked (c : *connection,
                                                      picture :  render::picture,
                                                      region :  region,
                                                      x_origin :  i16,
                                                      y_origin :  i16) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xfixes_set_picture_clip_region (c : *connection,
                                              picture :  render::picture,
                                              region :  region,
                                              x_origin :  i16,
                                              y_origin :  i16) -> void_cookie;

unsafe fn xcb_xfixes_set_cursor_name_sizeof (_buffer :  *c_void) -> c_int;

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
unsafe fn xcb_xfixes_set_cursor_name_checked (c : *connection,
                                              cursor :  xproto::cursor,
                                              nbytes :  u16,
                                              name : *u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xfixes_set_cursor_name (c : *connection,
                                      cursor :  xproto::cursor,
                                      nbytes :  u16,
                                      name : *u8) -> void_cookie;

unsafe fn xcb_xfixes_get_cursor_name_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xfixes_get_cursor_name (c : *connection,
                                      cursor :  xproto::cursor) -> get_cursor_name_cookie;

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
unsafe fn xcb_xfixes_get_cursor_name_unchecked (c : *connection,
                                                cursor :  xproto::cursor) -> get_cursor_name_cookie;

unsafe fn xcb_xfixes_get_cursor_name_name (R : *get_cursor_name_reply) -> *u8;


unsafe fn xcb_xfixes_get_cursor_name_name_length (R : *get_cursor_name_reply) -> c_int;


unsafe fn xcb_xfixes_get_cursor_name_name_end (R : *get_cursor_name_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xfixes_get_cursor_name_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_xfixes_get_cursor_name_reply (c : *connection,
                                            cookie : get_cursor_name_cookie,
                                            e : **generic_error) -> *get_cursor_name_reply;

unsafe fn xcb_xfixes_get_cursor_image_and_name_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xfixes_get_cursor_image_and_name (c : *connection) -> get_cursor_image_and_name_cookie;

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
unsafe fn xcb_xfixes_get_cursor_image_and_name_unchecked (c : *connection) -> get_cursor_image_and_name_cookie;

unsafe fn xcb_xfixes_get_cursor_image_and_name_name (R : *get_cursor_image_and_name_reply) -> *u8;


unsafe fn xcb_xfixes_get_cursor_image_and_name_name_length (R : *get_cursor_image_and_name_reply) -> c_int;


unsafe fn xcb_xfixes_get_cursor_image_and_name_name_end (R : *get_cursor_image_and_name_reply) -> generic_iterator;

unsafe fn xcb_xfixes_get_cursor_image_and_name_cursor_image (R : *get_cursor_image_and_name_reply) -> *u32;


unsafe fn xcb_xfixes_get_cursor_image_and_name_cursor_image_length (R : *get_cursor_image_and_name_reply) -> c_int;


unsafe fn xcb_xfixes_get_cursor_image_and_name_cursor_image_end (R : *get_cursor_image_and_name_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xfixes_get_cursor_image_and_name_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_xfixes_get_cursor_image_and_name_reply (c : *connection,
                                                      cookie : get_cursor_image_and_name_cookie,
                                                      e : **generic_error) -> *get_cursor_image_and_name_reply;

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
unsafe fn xcb_xfixes_change_cursor_checked (c : *connection,
                                            source :  xproto::cursor,
                                            destination :  xproto::cursor) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xfixes_change_cursor (c : *connection,
                                    source :  xproto::cursor,
                                    destination :  xproto::cursor) -> void_cookie;

unsafe fn xcb_xfixes_change_cursor_by_name_sizeof (_buffer :  *c_void) -> c_int;

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
unsafe fn xcb_xfixes_change_cursor_by_name_checked (c : *connection,
                                                    src :  xproto::cursor,
                                                    nbytes :  u16,
                                                    name : *u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xfixes_change_cursor_by_name (c : *connection,
                                            src :  xproto::cursor,
                                            nbytes :  u16,
                                            name : *u8) -> void_cookie;

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
unsafe fn xcb_xfixes_expand_region_checked (c : *connection,
                                            source :  region,
                                            destination :  region,
                                            left :  u16,
                                            right :  u16,
                                            top :  u16,
                                            bottom :  u16) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xfixes_expand_region (c : *connection,
                                    source :  region,
                                    destination :  region,
                                    left :  u16,
                                    right :  u16,
                                    top :  u16,
                                    bottom :  u16) -> void_cookie;

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
unsafe fn xcb_xfixes_hide_cursor_checked (c : *connection,
                                          window :  xproto::window) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xfixes_hide_cursor (c : *connection,
                                  window :  xproto::window) -> void_cookie;

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
unsafe fn xcb_xfixes_show_cursor_checked (c : *connection,
                                          window :  xproto::window) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xfixes_show_cursor (c : *connection,
                                  window :  xproto::window) -> void_cookie;
}

