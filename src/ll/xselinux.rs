/*
 * This file generated automatically from xselinux.xml by r_client.py.
 * Edit at your peril.
 */

//Make the compiler quiet
#[allow(unused_imports)];
#[allow(non_camel_case_types)];
use core::libc::*;
use ll::base::*;
use ll::xproto;

pub static SELINUX_MAJOR_VERSION : c_uint = 1;
pub static SELINUX_MINOR_VERSION : c_uint = 0;

pub struct query_version_cookie {
    sequence : c_uint
}

/** Opcode for xcb_selinux_query_version. */
pub static XCB_SELINUX_QUERY_VERSION : c_int = 0;

pub struct query_version_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    client_major :   u8,
    client_minor :   u8
}

pub struct query_version_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    server_major :    u16,
    server_minor :    u16
}

/** Opcode for xcb_selinux_set_device_create_context. */
pub static XCB_SELINUX_SET_DEVICE_CREATE_CONTEXT : c_int = 1;

pub struct set_device_create_context_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_len :    u32
}

pub struct get_device_create_context_cookie {
    sequence : c_uint
}

/** Opcode for xcb_selinux_get_device_create_context. */
pub static XCB_SELINUX_GET_DEVICE_CREATE_CONTEXT : c_int = 2;

pub struct get_device_create_context_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16
}

pub struct get_device_create_context_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    context_len :     u32,
    pad1 :            [u8,..20]
}

/** Opcode for xcb_selinux_set_device_context. */
pub static XCB_SELINUX_SET_DEVICE_CONTEXT : c_int = 3;

pub struct set_device_context_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    device :         u32,
    context_len :    u32
}

pub struct get_device_context_cookie {
    sequence : c_uint
}

/** Opcode for xcb_selinux_get_device_context. */
pub static XCB_SELINUX_GET_DEVICE_CONTEXT : c_int = 4;

pub struct get_device_context_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    device :         u32
}

pub struct get_device_context_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    context_len :     u32,
    pad1 :            [u8,..20]
}

/** Opcode for xcb_selinux_set_window_create_context. */
pub static XCB_SELINUX_SET_WINDOW_CREATE_CONTEXT : c_int = 5;

pub struct set_window_create_context_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_len :    u32
}

pub struct get_window_create_context_cookie {
    sequence : c_uint
}

/** Opcode for xcb_selinux_get_window_create_context. */
pub static XCB_SELINUX_GET_WINDOW_CREATE_CONTEXT : c_int = 6;

pub struct get_window_create_context_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16
}

pub struct get_window_create_context_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    context_len :     u32,
    pad1 :            [u8,..20]
}

pub struct get_window_context_cookie {
    sequence : c_uint
}

/** Opcode for xcb_selinux_get_window_context. */
pub static XCB_SELINUX_GET_WINDOW_CONTEXT : c_int = 7;

pub struct get_window_context_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    window :         xproto::window
}

pub struct get_window_context_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    context_len :     u32,
    pad1 :            [u8,..20]
}

pub struct list_item {
    name :                 xproto::atom,
    object_context_len :   u32,
    data_context_len :     u32
}

/**
 * @brief list_item_iterator
 **/
pub struct list_item_iterator {
    data : *list_item,
    rem  : c_int,
    index: c_int
}

/** Opcode for xcb_selinux_set_property_create_context. */
pub static XCB_SELINUX_SET_PROPERTY_CREATE_CONTEXT : c_int = 8;

pub struct set_property_create_context_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_len :    u32
}

pub struct get_property_create_context_cookie {
    sequence : c_uint
}

/** Opcode for xcb_selinux_get_property_create_context. */
pub static XCB_SELINUX_GET_PROPERTY_CREATE_CONTEXT : c_int = 9;

pub struct get_property_create_context_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16
}

pub struct get_property_create_context_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    context_len :     u32,
    pad1 :            [u8,..20]
}

/** Opcode for xcb_selinux_set_property_use_context. */
pub static XCB_SELINUX_SET_PROPERTY_USE_CONTEXT : c_int = 10;

pub struct set_property_use_context_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_len :    u32
}

pub struct get_property_use_context_cookie {
    sequence : c_uint
}

/** Opcode for xcb_selinux_get_property_use_context. */
pub static XCB_SELINUX_GET_PROPERTY_USE_CONTEXT : c_int = 11;

pub struct get_property_use_context_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16
}

pub struct get_property_use_context_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    context_len :     u32,
    pad1 :            [u8,..20]
}

pub struct get_property_context_cookie {
    sequence : c_uint
}

/** Opcode for xcb_selinux_get_property_context. */
pub static XCB_SELINUX_GET_PROPERTY_CONTEXT : c_int = 12;

pub struct get_property_context_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    window :         xproto::window,
    property :       xproto::atom
}

pub struct get_property_context_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    context_len :     u32,
    pad1 :            [u8,..20]
}

pub struct get_property_data_context_cookie {
    sequence : c_uint
}

/** Opcode for xcb_selinux_get_property_data_context. */
pub static XCB_SELINUX_GET_PROPERTY_DATA_CONTEXT : c_int = 13;

pub struct get_property_data_context_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    window :         xproto::window,
    property :       xproto::atom
}

pub struct get_property_data_context_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    context_len :     u32,
    pad1 :            [u8,..20]
}

pub struct list_properties_cookie {
    sequence : c_uint
}

/** Opcode for xcb_selinux_list_properties. */
pub static XCB_SELINUX_LIST_PROPERTIES : c_int = 14;

pub struct list_properties_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    window :         xproto::window
}

pub struct list_properties_reply {
    response_type :    u8,
    pad0 :             u8,
    sequence :         u16,
    length :           u32,
    properties_len :   u32,
    pad1 :             [u8,..20]
}

/** Opcode for xcb_selinux_set_selection_create_context. */
pub static XCB_SELINUX_SET_SELECTION_CREATE_CONTEXT : c_int = 15;

pub struct set_selection_create_context_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_len :    u32
}

pub struct get_selection_create_context_cookie {
    sequence : c_uint
}

/** Opcode for xcb_selinux_get_selection_create_context. */
pub static XCB_SELINUX_GET_SELECTION_CREATE_CONTEXT : c_int = 16;

pub struct get_selection_create_context_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16
}

pub struct get_selection_create_context_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    context_len :     u32,
    pad1 :            [u8,..20]
}

/** Opcode for xcb_selinux_set_selection_use_context. */
pub static XCB_SELINUX_SET_SELECTION_USE_CONTEXT : c_int = 17;

pub struct set_selection_use_context_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_len :    u32
}

pub struct get_selection_use_context_cookie {
    sequence : c_uint
}

/** Opcode for xcb_selinux_get_selection_use_context. */
pub static XCB_SELINUX_GET_SELECTION_USE_CONTEXT : c_int = 18;

pub struct get_selection_use_context_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16
}

pub struct get_selection_use_context_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    context_len :     u32,
    pad1 :            [u8,..20]
}

pub struct get_selection_context_cookie {
    sequence : c_uint
}

/** Opcode for xcb_selinux_get_selection_context. */
pub static XCB_SELINUX_GET_SELECTION_CONTEXT : c_int = 19;

pub struct get_selection_context_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    selection :      xproto::atom
}

pub struct get_selection_context_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    context_len :     u32,
    pad1 :            [u8,..20]
}

pub struct get_selection_data_context_cookie {
    sequence : c_uint
}

/** Opcode for xcb_selinux_get_selection_data_context. */
pub static XCB_SELINUX_GET_SELECTION_DATA_CONTEXT : c_int = 20;

pub struct get_selection_data_context_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    selection :      xproto::atom
}

pub struct get_selection_data_context_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    context_len :     u32,
    pad1 :            [u8,..20]
}

pub struct list_selections_cookie {
    sequence : c_uint
}

/** Opcode for xcb_selinux_list_selections. */
pub static XCB_SELINUX_LIST_SELECTIONS : c_int = 21;

pub struct list_selections_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16
}

pub struct list_selections_reply {
    response_type :    u8,
    pad0 :             u8,
    sequence :         u16,
    length :           u32,
    selections_len :   u32,
    pad1 :             [u8,..20]
}

pub struct get_client_context_cookie {
    sequence : c_uint
}

/** Opcode for xcb_selinux_get_client_context. */
pub static XCB_SELINUX_GET_CLIENT_CONTEXT : c_int = 22;

pub struct get_client_context_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    resource :       u32
}

pub struct get_client_context_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    context_len :     u32,
    pad1 :            [u8,..20]
}
#[link_args="-lxcb-xselinux"]
extern "C" {

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_selinux_query_version (c : *connection,
                                     client_major :  u8,
                                     client_minor :  u8) -> query_version_cookie;

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
unsafe fn xcb_selinux_query_version_unchecked (c : *connection,
                                               client_major :  u8,
                                               client_minor :  u8) -> query_version_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_selinux_query_version_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_selinux_query_version_reply (c : *connection,
                                           cookie : query_version_cookie,
                                           e : **generic_error) -> *query_version_reply;

unsafe fn xcb_selinux_set_device_create_context_sizeof (_buffer :  *c_void) -> c_int;

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
unsafe fn xcb_selinux_set_device_create_context_checked (c : *connection,
                                                         context_len :  u32,
                                                         context : *u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_selinux_set_device_create_context (c : *connection,
                                                 context_len :  u32,
                                                 context : *u8) -> void_cookie;

unsafe fn xcb_selinux_get_device_create_context_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_selinux_get_device_create_context (c : *connection) -> get_device_create_context_cookie;

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
unsafe fn xcb_selinux_get_device_create_context_unchecked (c : *connection) -> get_device_create_context_cookie;

unsafe fn xcb_selinux_get_device_create_context_context (R : *get_device_create_context_reply) -> *u8;


unsafe fn xcb_selinux_get_device_create_context_context_length (R : *get_device_create_context_reply) -> c_int;


unsafe fn xcb_selinux_get_device_create_context_context_end (R : *get_device_create_context_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_selinux_get_device_create_context_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_selinux_get_device_create_context_reply (c : *connection,
                                                       cookie : get_device_create_context_cookie,
                                                       e : **generic_error) -> *get_device_create_context_reply;

unsafe fn xcb_selinux_set_device_context_sizeof (_buffer :  *c_void) -> c_int;

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
unsafe fn xcb_selinux_set_device_context_checked (c : *connection,
                                                  device :  u32,
                                                  context_len :  u32,
                                                  context : *u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_selinux_set_device_context (c : *connection,
                                          device :  u32,
                                          context_len :  u32,
                                          context : *u8) -> void_cookie;

unsafe fn xcb_selinux_get_device_context_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_selinux_get_device_context (c : *connection,
                                          device :  u32) -> get_device_context_cookie;

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
unsafe fn xcb_selinux_get_device_context_unchecked (c : *connection,
                                                    device :  u32) -> get_device_context_cookie;

unsafe fn xcb_selinux_get_device_context_context (R : *get_device_context_reply) -> *u8;


unsafe fn xcb_selinux_get_device_context_context_length (R : *get_device_context_reply) -> c_int;


unsafe fn xcb_selinux_get_device_context_context_end (R : *get_device_context_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_selinux_get_device_context_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_selinux_get_device_context_reply (c : *connection,
                                                cookie : get_device_context_cookie,
                                                e : **generic_error) -> *get_device_context_reply;

unsafe fn xcb_selinux_set_window_create_context_sizeof (_buffer :  *c_void) -> c_int;

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
unsafe fn xcb_selinux_set_window_create_context_checked (c : *connection,
                                                         context_len :  u32,
                                                         context : *u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_selinux_set_window_create_context (c : *connection,
                                                 context_len :  u32,
                                                 context : *u8) -> void_cookie;

unsafe fn xcb_selinux_get_window_create_context_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_selinux_get_window_create_context (c : *connection) -> get_window_create_context_cookie;

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
unsafe fn xcb_selinux_get_window_create_context_unchecked (c : *connection) -> get_window_create_context_cookie;

unsafe fn xcb_selinux_get_window_create_context_context (R : *get_window_create_context_reply) -> *u8;


unsafe fn xcb_selinux_get_window_create_context_context_length (R : *get_window_create_context_reply) -> c_int;


unsafe fn xcb_selinux_get_window_create_context_context_end (R : *get_window_create_context_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_selinux_get_window_create_context_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_selinux_get_window_create_context_reply (c : *connection,
                                                       cookie : get_window_create_context_cookie,
                                                       e : **generic_error) -> *get_window_create_context_reply;

unsafe fn xcb_selinux_get_window_context_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_selinux_get_window_context (c : *connection,
                                          window :  xproto::window) -> get_window_context_cookie;

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
unsafe fn xcb_selinux_get_window_context_unchecked (c : *connection,
                                                    window :  xproto::window) -> get_window_context_cookie;

unsafe fn xcb_selinux_get_window_context_context (R : *get_window_context_reply) -> *u8;


unsafe fn xcb_selinux_get_window_context_context_length (R : *get_window_context_reply) -> c_int;


unsafe fn xcb_selinux_get_window_context_context_end (R : *get_window_context_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_selinux_get_window_context_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_selinux_get_window_context_reply (c : *connection,
                                                cookie : get_window_context_cookie,
                                                e : **generic_error) -> *get_window_context_reply;

unsafe fn xcb_selinux_list_item_sizeof (_buffer :  *c_void) -> c_int;

unsafe fn xcb_selinux_list_item_object_context (R : *list_item) -> *u8;


unsafe fn xcb_selinux_list_item_object_context_length (R : *list_item) -> c_int;


unsafe fn xcb_selinux_list_item_object_context_end (R : *list_item) -> generic_iterator;

unsafe fn xcb_selinux_list_item_data_context (R : *list_item) -> *u8;


unsafe fn xcb_selinux_list_item_data_context_length (R : *list_item) -> c_int;


unsafe fn xcb_selinux_list_item_data_context_end (R : *list_item) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a list_item_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(list_item)
 *
 *
 */
unsafe fn xcb_selinux_list_item_next (i:*list_item_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An list_item_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_selinux_list_item_end (i:list_item_iterator) -> generic_iterator;

unsafe fn xcb_selinux_set_property_create_context_sizeof (_buffer :  *c_void) -> c_int;

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
unsafe fn xcb_selinux_set_property_create_context_checked (c : *connection,
                                                           context_len :  u32,
                                                           context : *u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_selinux_set_property_create_context (c : *connection,
                                                   context_len :  u32,
                                                   context : *u8) -> void_cookie;

unsafe fn xcb_selinux_get_property_create_context_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_selinux_get_property_create_context (c : *connection) -> get_property_create_context_cookie;

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
unsafe fn xcb_selinux_get_property_create_context_unchecked (c : *connection) -> get_property_create_context_cookie;

unsafe fn xcb_selinux_get_property_create_context_context (R : *get_property_create_context_reply) -> *u8;


unsafe fn xcb_selinux_get_property_create_context_context_length (R : *get_property_create_context_reply) -> c_int;


unsafe fn xcb_selinux_get_property_create_context_context_end (R : *get_property_create_context_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_selinux_get_property_create_context_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_selinux_get_property_create_context_reply (c : *connection,
                                                         cookie : get_property_create_context_cookie,
                                                         e : **generic_error) -> *get_property_create_context_reply;

unsafe fn xcb_selinux_set_property_use_context_sizeof (_buffer :  *c_void) -> c_int;

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
unsafe fn xcb_selinux_set_property_use_context_checked (c : *connection,
                                                        context_len :  u32,
                                                        context : *u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_selinux_set_property_use_context (c : *connection,
                                                context_len :  u32,
                                                context : *u8) -> void_cookie;

unsafe fn xcb_selinux_get_property_use_context_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_selinux_get_property_use_context (c : *connection) -> get_property_use_context_cookie;

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
unsafe fn xcb_selinux_get_property_use_context_unchecked (c : *connection) -> get_property_use_context_cookie;

unsafe fn xcb_selinux_get_property_use_context_context (R : *get_property_use_context_reply) -> *u8;


unsafe fn xcb_selinux_get_property_use_context_context_length (R : *get_property_use_context_reply) -> c_int;


unsafe fn xcb_selinux_get_property_use_context_context_end (R : *get_property_use_context_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_selinux_get_property_use_context_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_selinux_get_property_use_context_reply (c : *connection,
                                                      cookie : get_property_use_context_cookie,
                                                      e : **generic_error) -> *get_property_use_context_reply;

unsafe fn xcb_selinux_get_property_context_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_selinux_get_property_context (c : *connection,
                                            window :  xproto::window,
                                            property :  xproto::atom) -> get_property_context_cookie;

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
unsafe fn xcb_selinux_get_property_context_unchecked (c : *connection,
                                                      window :  xproto::window,
                                                      property :  xproto::atom) -> get_property_context_cookie;

unsafe fn xcb_selinux_get_property_context_context (R : *get_property_context_reply) -> *u8;


unsafe fn xcb_selinux_get_property_context_context_length (R : *get_property_context_reply) -> c_int;


unsafe fn xcb_selinux_get_property_context_context_end (R : *get_property_context_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_selinux_get_property_context_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_selinux_get_property_context_reply (c : *connection,
                                                  cookie : get_property_context_cookie,
                                                  e : **generic_error) -> *get_property_context_reply;

unsafe fn xcb_selinux_get_property_data_context_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_selinux_get_property_data_context (c : *connection,
                                                 window :  xproto::window,
                                                 property :  xproto::atom) -> get_property_data_context_cookie;

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
unsafe fn xcb_selinux_get_property_data_context_unchecked (c : *connection,
                                                           window :  xproto::window,
                                                           property :  xproto::atom) -> get_property_data_context_cookie;

unsafe fn xcb_selinux_get_property_data_context_context (R : *get_property_data_context_reply) -> *u8;


unsafe fn xcb_selinux_get_property_data_context_context_length (R : *get_property_data_context_reply) -> c_int;


unsafe fn xcb_selinux_get_property_data_context_context_end (R : *get_property_data_context_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_selinux_get_property_data_context_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_selinux_get_property_data_context_reply (c : *connection,
                                                       cookie : get_property_data_context_cookie,
                                                       e : **generic_error) -> *get_property_data_context_reply;

unsafe fn xcb_selinux_list_properties_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_selinux_list_properties (c : *connection,
                                       window :  xproto::window) -> list_properties_cookie;

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
unsafe fn xcb_selinux_list_properties_unchecked (c : *connection,
                                                 window :  xproto::window) -> list_properties_cookie;


unsafe fn xcb_selinux_list_properties_properties_length (R : *list_properties_reply) -> c_int;

unsafe fn xcb_selinux_list_properties_properties_iterator (R : *list_properties_reply) -> list_item_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_selinux_list_properties_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_selinux_list_properties_reply (c : *connection,
                                             cookie : list_properties_cookie,
                                             e : **generic_error) -> *list_properties_reply;

unsafe fn xcb_selinux_set_selection_create_context_sizeof (_buffer :  *c_void) -> c_int;

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
unsafe fn xcb_selinux_set_selection_create_context_checked (c : *connection,
                                                            context_len :  u32,
                                                            context : *u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_selinux_set_selection_create_context (c : *connection,
                                                    context_len :  u32,
                                                    context : *u8) -> void_cookie;

unsafe fn xcb_selinux_get_selection_create_context_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_selinux_get_selection_create_context (c : *connection) -> get_selection_create_context_cookie;

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
unsafe fn xcb_selinux_get_selection_create_context_unchecked (c : *connection) -> get_selection_create_context_cookie;

unsafe fn xcb_selinux_get_selection_create_context_context (R : *get_selection_create_context_reply) -> *u8;


unsafe fn xcb_selinux_get_selection_create_context_context_length (R : *get_selection_create_context_reply) -> c_int;


unsafe fn xcb_selinux_get_selection_create_context_context_end (R : *get_selection_create_context_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_selinux_get_selection_create_context_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_selinux_get_selection_create_context_reply (c : *connection,
                                                          cookie : get_selection_create_context_cookie,
                                                          e : **generic_error) -> *get_selection_create_context_reply;

unsafe fn xcb_selinux_set_selection_use_context_sizeof (_buffer :  *c_void) -> c_int;

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
unsafe fn xcb_selinux_set_selection_use_context_checked (c : *connection,
                                                         context_len :  u32,
                                                         context : *u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_selinux_set_selection_use_context (c : *connection,
                                                 context_len :  u32,
                                                 context : *u8) -> void_cookie;

unsafe fn xcb_selinux_get_selection_use_context_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_selinux_get_selection_use_context (c : *connection) -> get_selection_use_context_cookie;

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
unsafe fn xcb_selinux_get_selection_use_context_unchecked (c : *connection) -> get_selection_use_context_cookie;

unsafe fn xcb_selinux_get_selection_use_context_context (R : *get_selection_use_context_reply) -> *u8;


unsafe fn xcb_selinux_get_selection_use_context_context_length (R : *get_selection_use_context_reply) -> c_int;


unsafe fn xcb_selinux_get_selection_use_context_context_end (R : *get_selection_use_context_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_selinux_get_selection_use_context_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_selinux_get_selection_use_context_reply (c : *connection,
                                                       cookie : get_selection_use_context_cookie,
                                                       e : **generic_error) -> *get_selection_use_context_reply;

unsafe fn xcb_selinux_get_selection_context_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_selinux_get_selection_context (c : *connection,
                                             selection :  xproto::atom) -> get_selection_context_cookie;

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
unsafe fn xcb_selinux_get_selection_context_unchecked (c : *connection,
                                                       selection :  xproto::atom) -> get_selection_context_cookie;

unsafe fn xcb_selinux_get_selection_context_context (R : *get_selection_context_reply) -> *u8;


unsafe fn xcb_selinux_get_selection_context_context_length (R : *get_selection_context_reply) -> c_int;


unsafe fn xcb_selinux_get_selection_context_context_end (R : *get_selection_context_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_selinux_get_selection_context_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_selinux_get_selection_context_reply (c : *connection,
                                                   cookie : get_selection_context_cookie,
                                                   e : **generic_error) -> *get_selection_context_reply;

unsafe fn xcb_selinux_get_selection_data_context_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_selinux_get_selection_data_context (c : *connection,
                                                  selection :  xproto::atom) -> get_selection_data_context_cookie;

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
unsafe fn xcb_selinux_get_selection_data_context_unchecked (c : *connection,
                                                            selection :  xproto::atom) -> get_selection_data_context_cookie;

unsafe fn xcb_selinux_get_selection_data_context_context (R : *get_selection_data_context_reply) -> *u8;


unsafe fn xcb_selinux_get_selection_data_context_context_length (R : *get_selection_data_context_reply) -> c_int;


unsafe fn xcb_selinux_get_selection_data_context_context_end (R : *get_selection_data_context_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_selinux_get_selection_data_context_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_selinux_get_selection_data_context_reply (c : *connection,
                                                        cookie : get_selection_data_context_cookie,
                                                        e : **generic_error) -> *get_selection_data_context_reply;

unsafe fn xcb_selinux_list_selections_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_selinux_list_selections (c : *connection) -> list_selections_cookie;

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
unsafe fn xcb_selinux_list_selections_unchecked (c : *connection) -> list_selections_cookie;


unsafe fn xcb_selinux_list_selections_selections_length (R : *list_selections_reply) -> c_int;

unsafe fn xcb_selinux_list_selections_selections_iterator (R : *list_selections_reply) -> list_item_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_selinux_list_selections_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_selinux_list_selections_reply (c : *connection,
                                             cookie : list_selections_cookie,
                                             e : **generic_error) -> *list_selections_reply;

unsafe fn xcb_selinux_get_client_context_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_selinux_get_client_context (c : *connection,
                                          resource :  u32) -> get_client_context_cookie;

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
unsafe fn xcb_selinux_get_client_context_unchecked (c : *connection,
                                                    resource :  u32) -> get_client_context_cookie;

unsafe fn xcb_selinux_get_client_context_context (R : *get_client_context_reply) -> *u8;


unsafe fn xcb_selinux_get_client_context_context_length (R : *get_client_context_reply) -> c_int;


unsafe fn xcb_selinux_get_client_context_context_end (R : *get_client_context_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_selinux_get_client_context_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_selinux_get_client_context_reply (c : *connection,
                                                cookie : get_client_context_cookie,
                                                e : **generic_error) -> *get_client_context_reply;
}

