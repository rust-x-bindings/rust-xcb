/*
 * This file generated automatically from dri2.xml by r_client.py.
 * Edit at your peril.
 */

//Make the compiler quiet
#[allow(unused_imports)];
#[allow(non_camel_case_types)];
use core::libc::*;
use ll::base::*;
use ll::xproto;

pub static DRI2_MAJOR_VERSION : c_uint = 1;
pub static DRI2_MINOR_VERSION : c_uint = 4;

pub type attachment = c_uint;//{
    pub static XCB_DRI2_ATTACHMENT_BUFFER_FRONT_LEFT : attachment = 1;
    pub static XCB_DRI2_ATTACHMENT_BUFFER_BACK_LEFT : attachment = 2;
    pub static XCB_DRI2_ATTACHMENT_BUFFER_FRONT_RIGHT : attachment = 3;
    pub static XCB_DRI2_ATTACHMENT_BUFFER_BACK_RIGHT : attachment = 4;
    pub static XCB_DRI2_ATTACHMENT_BUFFER_DEPTH : attachment = 5;
    pub static XCB_DRI2_ATTACHMENT_BUFFER_STENCIL : attachment = 6;
    pub static XCB_DRI2_ATTACHMENT_BUFFER_ACCUM : attachment = 7;
    pub static XCB_DRI2_ATTACHMENT_BUFFER_FAKE_FRONT_LEFT : attachment = 8;
    pub static XCB_DRI2_ATTACHMENT_BUFFER_FAKE_FRONT_RIGHT : attachment = 9;
    pub static XCB_DRI2_ATTACHMENT_BUFFER_DEPTH_STENCIL : attachment = 10;
    pub static XCB_DRI2_ATTACHMENT_BUFFER_HIZ : attachment = 11;
//}

pub type driver_type = c_uint;//{
    pub static XCB_DRI2_DRIVER_TYPE_DRI : driver_type = 1;
    pub static XCB_DRI2_DRIVER_TYPE_VDPAU : driver_type = 2;
//}

pub type event_type = c_uint;//{
    pub static XCB_DRI2_EVENT_TYPE_EXCHANGE_COMPLETE : event_type = 1;
    pub static XCB_DRI2_EVENT_TYPE_BLIT_COMPLETE : event_type = 2;
    pub static XCB_DRI2_EVENT_TYPE_FLIP_COMPLETE : event_type = 3;
//}

pub struct dri2_buffer {
    attachment :   u32,
    name :         u32,
    pitch :        u32,
    cpp :          u32,
    flags :        u32
}

/**
 * @brief dri2_buffer_iterator
 **/
pub struct dri2_buffer_iterator {
    data : *dri2_buffer,
    rem  : c_int,
    index: c_int
}

pub struct attach_format {
    attachment :   u32,
    format :       u32
}

/**
 * @brief attach_format_iterator
 **/
pub struct attach_format_iterator {
    data : *attach_format,
    rem  : c_int,
    index: c_int
}

pub struct query_version_cookie {
    sequence : c_uint
}

/** Opcode for xcb_dri2_query_version. */
pub static XCB_DRI2_QUERY_VERSION : c_int = 0;

pub struct query_version_request {
    major_opcode :    u8,
    minor_opcode :    u8,
    length :          u16,
    major_version :   u32,
    minor_version :   u32
}

pub struct query_version_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    major_version :   u32,
    minor_version :   u32
}

pub struct connect_cookie {
    sequence : c_uint
}

/** Opcode for xcb_dri2_connect. */
pub static XCB_DRI2_CONNECT : c_int = 1;

pub struct connect_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    window :         xproto::window,
    driver_type :    u32
}

pub struct connect_reply {
    response_type :        u8,
    pad0 :                 u8,
    sequence :             u16,
    length :               u32,
    driver_name_length :   u32,
    device_name_length :   u32,
    pad1 :                 [u8,..16]
}

pub struct authenticate_cookie {
    sequence : c_uint
}

/** Opcode for xcb_dri2_authenticate. */
pub static XCB_DRI2_AUTHENTICATE : c_int = 2;

pub struct authenticate_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    window :         xproto::window,
    magic :          u32
}

pub struct authenticate_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    authenticated :   u32
}

/** Opcode for xcb_dri2_create_drawable. */
pub static XCB_DRI2_CREATE_DRAWABLE : c_int = 3;

pub struct create_drawable_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    drawable :       xproto::drawable
}

/** Opcode for xcb_dri2_destroy_drawable. */
pub static XCB_DRI2_DESTROY_DRAWABLE : c_int = 4;

pub struct destroy_drawable_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    drawable :       xproto::drawable
}

pub struct get_buffers_cookie {
    sequence : c_uint
}

/** Opcode for xcb_dri2_get_buffers. */
pub static XCB_DRI2_GET_BUFFERS : c_int = 5;

pub struct get_buffers_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    drawable :       xproto::drawable,
    count :          u32
}

pub struct get_buffers_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    width :           u32,
    height :          u32,
    count :           u32,
    pad1 :            [u8,..12]
}

pub struct copy_region_cookie {
    sequence : c_uint
}

/** Opcode for xcb_dri2_copy_region. */
pub static XCB_DRI2_COPY_REGION : c_int = 6;

pub struct copy_region_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    drawable :       xproto::drawable,
    region :         u32,
    dest :           u32,
    src :            u32
}

pub struct copy_region_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32
}

pub struct get_buffers_with_format_cookie {
    sequence : c_uint
}

/** Opcode for xcb_dri2_get_buffers_with_format. */
pub static XCB_DRI2_GET_BUFFERS_WITH_FORMAT : c_int = 7;

pub struct get_buffers_with_format_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    drawable :       xproto::drawable,
    count :          u32
}

pub struct get_buffers_with_format_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    width :           u32,
    height :          u32,
    count :           u32,
    pad1 :            [u8,..12]
}

pub struct swap_buffers_cookie {
    sequence : c_uint
}

/** Opcode for xcb_dri2_swap_buffers. */
pub static XCB_DRI2_SWAP_BUFFERS : c_int = 8;

pub struct swap_buffers_request {
    major_opcode :    u8,
    minor_opcode :    u8,
    length :          u16,
    drawable :        xproto::drawable,
    target_msc_hi :   u32,
    target_msc_lo :   u32,
    divisor_hi :      u32,
    divisor_lo :      u32,
    remainder_hi :    u32,
    remainder_lo :    u32
}

pub struct swap_buffers_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    swap_hi :         u32,
    swap_lo :         u32
}

pub struct get_msc_cookie {
    sequence : c_uint
}

/** Opcode for xcb_dri2_get_msc. */
pub static XCB_DRI2_GET_MSC : c_int = 9;

pub struct get_msc_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    drawable :       xproto::drawable
}

pub struct get_msc_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    ust_hi :          u32,
    ust_lo :          u32,
    msc_hi :          u32,
    msc_lo :          u32,
    sbc_hi :          u32,
    sbc_lo :          u32
}

pub struct wait_msc_cookie {
    sequence : c_uint
}

/** Opcode for xcb_dri2_wait_msc. */
pub static XCB_DRI2_WAIT_MSC : c_int = 10;

pub struct wait_msc_request {
    major_opcode :    u8,
    minor_opcode :    u8,
    length :          u16,
    drawable :        xproto::drawable,
    target_msc_hi :   u32,
    target_msc_lo :   u32,
    divisor_hi :      u32,
    divisor_lo :      u32,
    remainder_hi :    u32,
    remainder_lo :    u32
}

pub struct wait_msc_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    ust_hi :          u32,
    ust_lo :          u32,
    msc_hi :          u32,
    msc_lo :          u32,
    sbc_hi :          u32,
    sbc_lo :          u32
}

pub struct wait_sbc_cookie {
    sequence : c_uint
}

/** Opcode for xcb_dri2_wait_sbc. */
pub static XCB_DRI2_WAIT_SBC : c_int = 11;

pub struct wait_sbc_request {
    major_opcode :    u8,
    minor_opcode :    u8,
    length :          u16,
    drawable :        xproto::drawable,
    target_sbc_hi :   u32,
    target_sbc_lo :   u32
}

pub struct wait_sbc_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    ust_hi :          u32,
    ust_lo :          u32,
    msc_hi :          u32,
    msc_lo :          u32,
    sbc_hi :          u32,
    sbc_lo :          u32
}

/** Opcode for xcb_dri2_swap_interval. */
pub static XCB_DRI2_SWAP_INTERVAL : c_int = 12;

pub struct swap_interval_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    drawable :       xproto::drawable,
    interval :       u32
}

pub struct get_param_cookie {
    sequence : c_uint
}

/** Opcode for xcb_dri2_get_param. */
pub static XCB_DRI2_GET_PARAM : c_int = 13;

pub struct get_param_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    drawable :       xproto::drawable,
    param :          u32
}

pub struct get_param_reply {
    response_type :         u8,
    is_param_recognized :   u8,
    sequence :              u16,
    length :                u32,
    value_hi :              u32,
    value_lo :              u32
}

/** Opcode for xcb_dri2_buffer_swap_complete. */
pub static XCB_DRI2_BUFFER_SWAP_COMPLETE : c_int = 0;

pub struct buffer_swap_complete_event {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    event_type :      u16,
    pad1 :            [u8,..2],
    drawable :        xproto::drawable,
    ust_hi :          u32,
    ust_lo :          u32,
    msc_hi :          u32,
    msc_lo :          u32,
    sbc :             u32
}

/** Opcode for xcb_dri2_invalidate_buffers. */
pub static XCB_DRI2_INVALIDATE_BUFFERS : c_int = 1;

pub struct invalidate_buffers_event {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    drawable :        xproto::drawable
}
#[link_args="-lxcb-dri2"]
extern "C" {

/**
 * Get the next element of the iterator
 * @param i Pointer to a dri2_buffer_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(dri2_buffer)
 *
 *
 */
unsafe fn xcb_dri2_dri2_buffer_next (i:*dri2_buffer_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An dri2_buffer_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_dri2_dri2_buffer_end (i:dri2_buffer_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a attach_format_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(attach_format)
 *
 *
 */
unsafe fn xcb_dri2_attach_format_next (i:*attach_format_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An attach_format_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_dri2_attach_format_end (i:attach_format_iterator) -> generic_iterator;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_dri2_query_version (c : *connection,
                                  major_version :  u32,
                                  minor_version :  u32) -> query_version_cookie;

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
unsafe fn xcb_dri2_query_version_unchecked (c : *connection,
                                            major_version :  u32,
                                            minor_version :  u32) -> query_version_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_dri2_query_version_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_dri2_query_version_reply (c : *connection,
                                        cookie : query_version_cookie,
                                        e : **generic_error) -> *query_version_reply;

unsafe fn xcb_dri2_connect_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_dri2_connect (c : *connection,
                            window :  xproto::window,
                            driver_type :  u32) -> connect_cookie;

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
unsafe fn xcb_dri2_connect_unchecked (c : *connection,
                                      window :  xproto::window,
                                      driver_type :  u32) -> connect_cookie;

unsafe fn xcb_dri2_connect_driver_name (R : *connect_reply) -> *u8;


unsafe fn xcb_dri2_connect_driver_name_length (R : *connect_reply) -> c_int;


unsafe fn xcb_dri2_connect_driver_name_end (R : *connect_reply) -> generic_iterator;

unsafe fn xcb_dri2_connect_alignment_pad (R : *connect_reply) -> *c_void;


unsafe fn xcb_dri2_connect_alignment_pad_length (R : *connect_reply) -> c_int;


unsafe fn xcb_dri2_connect_alignment_pad_end (R : *connect_reply) -> generic_iterator;

unsafe fn xcb_dri2_connect_device_name (R : *connect_reply) -> *u8;


unsafe fn xcb_dri2_connect_device_name_length (R : *connect_reply) -> c_int;


unsafe fn xcb_dri2_connect_device_name_end (R : *connect_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_dri2_connect_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_dri2_connect_reply (c : *connection,
                                  cookie : connect_cookie,
                                  e : **generic_error) -> *connect_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_dri2_authenticate (c : *connection,
                                 window :  xproto::window,
                                 magic :  u32) -> authenticate_cookie;

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
unsafe fn xcb_dri2_authenticate_unchecked (c : *connection,
                                           window :  xproto::window,
                                           magic :  u32) -> authenticate_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_dri2_authenticate_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_dri2_authenticate_reply (c : *connection,
                                       cookie : authenticate_cookie,
                                       e : **generic_error) -> *authenticate_reply;

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
unsafe fn xcb_dri2_create_drawable_checked (c : *connection,
                                            drawable :  xproto::drawable) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_dri2_create_drawable (c : *connection,
                                    drawable :  xproto::drawable) -> void_cookie;

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
unsafe fn xcb_dri2_destroy_drawable_checked (c : *connection,
                                             drawable :  xproto::drawable) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_dri2_destroy_drawable (c : *connection,
                                     drawable :  xproto::drawable) -> void_cookie;

unsafe fn xcb_dri2_get_buffers_sizeof (_buffer :  *c_void,
                             attachments_len :  u32) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_dri2_get_buffers (c : *connection,
                                drawable :  xproto::drawable,
                                count :  u32,
                                attachments_len :  u32,
                                attachments : *u32) -> get_buffers_cookie;

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
unsafe fn xcb_dri2_get_buffers_unchecked (c : *connection,
                                          drawable :  xproto::drawable,
                                          count :  u32,
                                          attachments_len :  u32,
                                          attachments : *u32) -> get_buffers_cookie;

unsafe fn xcb_dri2_get_buffers_buffers (R : *get_buffers_reply) -> *dri2_buffer;


unsafe fn xcb_dri2_get_buffers_buffers_length (R : *get_buffers_reply) -> c_int;

unsafe fn xcb_dri2_get_buffers_buffers_iterator (R : *get_buffers_reply) -> dri2_buffer_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_dri2_get_buffers_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_dri2_get_buffers_reply (c : *connection,
                                      cookie : get_buffers_cookie,
                                      e : **generic_error) -> *get_buffers_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_dri2_copy_region (c : *connection,
                                drawable :  xproto::drawable,
                                region :  u32,
                                dest :  u32,
                                src :  u32) -> copy_region_cookie;

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
unsafe fn xcb_dri2_copy_region_unchecked (c : *connection,
                                          drawable :  xproto::drawable,
                                          region :  u32,
                                          dest :  u32,
                                          src :  u32) -> copy_region_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_dri2_copy_region_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_dri2_copy_region_reply (c : *connection,
                                      cookie : copy_region_cookie,
                                      e : **generic_error) -> *copy_region_reply;

unsafe fn xcb_dri2_get_buffers_with_format_sizeof (_buffer :  *c_void,
                                         attachments_len :  u32) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_dri2_get_buffers_with_format (c : *connection,
                                            drawable :  xproto::drawable,
                                            count :  u32,
                                            attachments_len :  u32,
                                            attachments : *attach_format) -> get_buffers_with_format_cookie;

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
unsafe fn xcb_dri2_get_buffers_with_format_unchecked (c : *connection,
                                                      drawable :  xproto::drawable,
                                                      count :  u32,
                                                      attachments_len :  u32,
                                                      attachments : *attach_format) -> get_buffers_with_format_cookie;

unsafe fn xcb_dri2_get_buffers_with_format_buffers (R : *get_buffers_with_format_reply) -> *dri2_buffer;


unsafe fn xcb_dri2_get_buffers_with_format_buffers_length (R : *get_buffers_with_format_reply) -> c_int;

unsafe fn xcb_dri2_get_buffers_with_format_buffers_iterator (R : *get_buffers_with_format_reply) -> dri2_buffer_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_dri2_get_buffers_with_format_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_dri2_get_buffers_with_format_reply (c : *connection,
                                                  cookie : get_buffers_with_format_cookie,
                                                  e : **generic_error) -> *get_buffers_with_format_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_dri2_swap_buffers (c : *connection,
                                 drawable :  xproto::drawable,
                                 target_msc_hi :  u32,
                                 target_msc_lo :  u32,
                                 divisor_hi :  u32,
                                 divisor_lo :  u32,
                                 remainder_hi :  u32,
                                 remainder_lo :  u32) -> swap_buffers_cookie;

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
unsafe fn xcb_dri2_swap_buffers_unchecked (c : *connection,
                                           drawable :  xproto::drawable,
                                           target_msc_hi :  u32,
                                           target_msc_lo :  u32,
                                           divisor_hi :  u32,
                                           divisor_lo :  u32,
                                           remainder_hi :  u32,
                                           remainder_lo :  u32) -> swap_buffers_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_dri2_swap_buffers_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_dri2_swap_buffers_reply (c : *connection,
                                       cookie : swap_buffers_cookie,
                                       e : **generic_error) -> *swap_buffers_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_dri2_get_msc (c : *connection,
                            drawable :  xproto::drawable) -> get_msc_cookie;

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
unsafe fn xcb_dri2_get_msc_unchecked (c : *connection,
                                      drawable :  xproto::drawable) -> get_msc_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_dri2_get_msc_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_dri2_get_msc_reply (c : *connection,
                                  cookie : get_msc_cookie,
                                  e : **generic_error) -> *get_msc_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_dri2_wait_msc (c : *connection,
                             drawable :  xproto::drawable,
                             target_msc_hi :  u32,
                             target_msc_lo :  u32,
                             divisor_hi :  u32,
                             divisor_lo :  u32,
                             remainder_hi :  u32,
                             remainder_lo :  u32) -> wait_msc_cookie;

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
unsafe fn xcb_dri2_wait_msc_unchecked (c : *connection,
                                       drawable :  xproto::drawable,
                                       target_msc_hi :  u32,
                                       target_msc_lo :  u32,
                                       divisor_hi :  u32,
                                       divisor_lo :  u32,
                                       remainder_hi :  u32,
                                       remainder_lo :  u32) -> wait_msc_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_dri2_wait_msc_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_dri2_wait_msc_reply (c : *connection,
                                   cookie : wait_msc_cookie,
                                   e : **generic_error) -> *wait_msc_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_dri2_wait_sbc (c : *connection,
                             drawable :  xproto::drawable,
                             target_sbc_hi :  u32,
                             target_sbc_lo :  u32) -> wait_sbc_cookie;

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
unsafe fn xcb_dri2_wait_sbc_unchecked (c : *connection,
                                       drawable :  xproto::drawable,
                                       target_sbc_hi :  u32,
                                       target_sbc_lo :  u32) -> wait_sbc_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_dri2_wait_sbc_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_dri2_wait_sbc_reply (c : *connection,
                                   cookie : wait_sbc_cookie,
                                   e : **generic_error) -> *wait_sbc_reply;

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
unsafe fn xcb_dri2_swap_interval_checked (c : *connection,
                                          drawable :  xproto::drawable,
                                          interval :  u32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_dri2_swap_interval (c : *connection,
                                  drawable :  xproto::drawable,
                                  interval :  u32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_dri2_get_param (c : *connection,
                              drawable :  xproto::drawable,
                              param :  u32) -> get_param_cookie;

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
unsafe fn xcb_dri2_get_param_unchecked (c : *connection,
                                        drawable :  xproto::drawable,
                                        param :  u32) -> get_param_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_dri2_get_param_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_dri2_get_param_reply (c : *connection,
                                    cookie : get_param_cookie,
                                    e : **generic_error) -> *get_param_reply;
}

