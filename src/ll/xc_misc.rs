/*
 * This file generated automatically from xc_misc.xml by r_client.py.
 * Edit at your peril.
 */

//Make the compiler quiet
#[allow(unused_imports)];
#[allow(non_camel_case_types)];
use core::libc::*;
use ll::base::*;

pub static XCMISC_MAJOR_VERSION : c_uint = 1;
pub static XCMISC_MINOR_VERSION : c_uint = 1;

pub struct get_version_cookie {
    sequence : c_uint
}

/** Opcode for xcb_xc_misc_get_version. */
pub static XCB_XC_MISC_GET_VERSION : c_int = 0;

pub struct get_version_request {
    major_opcode :           u8,
    minor_opcode :           u8,
    length :                 u16,
    client_major_version :   u16,
    client_minor_version :   u16
}

pub struct get_version_reply {
    response_type :          u8,
    pad0 :                   u8,
    sequence :               u16,
    length :                 u32,
    server_major_version :   u16,
    server_minor_version :   u16
}

pub struct get_xid_range_cookie {
    sequence : c_uint
}

/** Opcode for xcb_xc_misc_get_xid_range. */
pub static XCB_XC_MISC_GET_XID_RANGE : c_int = 1;

pub struct get_xid_range_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16
}

pub struct get_xid_range_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    start_id :        u32,
    count :           u32
}

pub struct get_xid_list_cookie {
    sequence : c_uint
}

/** Opcode for xcb_xc_misc_get_xid_list. */
pub static XCB_XC_MISC_GET_XID_LIST : c_int = 2;

pub struct get_xid_list_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    count :          u32
}

pub struct get_xid_list_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    ids_len :         u32,
    pad1 :            [u8,..20]
}
extern "C" {

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xc_misc_get_version (c : *connection,
                                   client_major_version :  u16,
                                   client_minor_version :  u16) -> get_version_cookie;

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
unsafe fn xcb_xc_misc_get_version_unchecked (c : *connection,
                                             client_major_version :  u16,
                                             client_minor_version :  u16) -> get_version_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xc_misc_get_version_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_xc_misc_get_version_reply (c : *connection,
                                         cookie : get_version_cookie,
                                         e : **generic_error) -> *get_version_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xc_misc_get_xid_range (c : *connection) -> get_xid_range_cookie;

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
unsafe fn xcb_xc_misc_get_xid_range_unchecked (c : *connection) -> get_xid_range_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xc_misc_get_xid_range_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_xc_misc_get_xid_range_reply (c : *connection,
                                           cookie : get_xid_range_cookie,
                                           e : **generic_error) -> *get_xid_range_reply;

unsafe fn xcb_xc_misc_get_xid_list_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_xc_misc_get_xid_list (c : *connection,
                                    count :  u32) -> get_xid_list_cookie;

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
unsafe fn xcb_xc_misc_get_xid_list_unchecked (c : *connection,
                                              count :  u32) -> get_xid_list_cookie;

unsafe fn xcb_xc_misc_get_xid_list_ids (R : *get_xid_list_reply) -> *u32;


unsafe fn xcb_xc_misc_get_xid_list_ids_length (R : *get_xid_list_reply) -> c_int;


unsafe fn xcb_xc_misc_get_xid_list_ids_end (R : *get_xid_list_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xc_misc_get_xid_list_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_xc_misc_get_xid_list_reply (c : *connection,
                                          cookie : get_xid_list_cookie,
                                          e : **generic_error) -> *get_xid_list_reply;
}

