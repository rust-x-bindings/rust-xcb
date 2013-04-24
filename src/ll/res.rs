/*
 * This file generated automatically from res.xml by r_client.py.
 * Edit at your peril.
 */

//Make the compiler quiet
#[allow(unused_imports)];
#[allow(non_camel_case_types)];
use core;
use core::libc::*;
use ll::base::*;
use ll;
use ll::xproto;

pub static RES_MAJOR_VERSION : c_uint = 1;
pub static RES_MINOR_VERSION : c_uint = 0;

pub struct client {
    resource_base :   u32,
    resource_mask :   u32
}

/**
 * @brief client_iterator
 **/
pub struct client_iterator {
    data : *client,
    rem  : c_int,
    index: c_int
}


pub struct type_ {
    resource_type :   ll::xproto::atom,
    count :           u32
}

/**
 * @brief type_iterator
 **/
pub struct type_iterator {
    data : *type_,
    rem  : c_int,
    index: c_int
}


pub struct query_version_cookie {
    sequence : c_uint
}


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


pub struct query_clients_cookie {
    sequence : c_uint
}


pub struct query_clients_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16
}


pub struct query_clients_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    num_clients :     u32,
    pad1 :            [u8,..20]
}


pub struct query_client_resources_cookie {
    sequence : c_uint
}


pub struct query_client_resources_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    xid :            u32
}


pub struct query_client_resources_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    num_types :       u32,
    pad1 :            [u8,..20]
}


pub struct query_client_pixmap_bytes_cookie {
    sequence : c_uint
}


pub struct query_client_pixmap_bytes_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    xid :            u32
}


pub struct query_client_pixmap_bytes_reply {
    response_type :    u8,
    pad0 :             u8,
    sequence :         u16,
    length :           u32,
    bytes :            u32,
    bytes_overflow :   u32
}

#[link_args="-lxcb-res"]
pub extern "C" {

/**
 * Get the next element of the iterator
 * @param i Pointer to a client_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(client)
 *
 *
 */
unsafe fn xcb_res_client_next (i:*client_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An client_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_res_client_end (i:client_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a type_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(type_)
 *
 *
 */
unsafe fn xcb_res_type_next (i:*type_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An type_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_res_type_end (i:type_iterator) -> generic_iterator;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_res_query_version (c : *connection,
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
unsafe fn xcb_res_query_version_unchecked (c : *connection,
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
 * xcb_res_query_version_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_res_query_version_reply (c : *connection,
                                       cookie : query_version_cookie,
                                       e : **generic_error) -> *query_version_reply;

unsafe fn xcb_res_query_clients_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_res_query_clients (c : *connection) -> query_clients_cookie;

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
unsafe fn xcb_res_query_clients_unchecked (c : *connection) -> query_clients_cookie;

unsafe fn xcb_res_query_clients_clients (R : *query_clients_reply) -> *client;


unsafe fn xcb_res_query_clients_clients_length (R : *query_clients_reply) -> c_int;

unsafe fn xcb_res_query_clients_clients_iterator (R : *query_clients_reply) -> client_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_res_query_clients_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_res_query_clients_reply (c : *connection,
                                       cookie : query_clients_cookie,
                                       e : **generic_error) -> *query_clients_reply;

unsafe fn xcb_res_query_client_resources_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_res_query_client_resources (c : *connection,
                                          xid :  u32) -> query_client_resources_cookie;

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
unsafe fn xcb_res_query_client_resources_unchecked (c : *connection,
                                                    xid :  u32) -> query_client_resources_cookie;

unsafe fn xcb_res_query_client_resources_types (R : *query_client_resources_reply) -> *type_;


unsafe fn xcb_res_query_client_resources_types_length (R : *query_client_resources_reply) -> c_int;

unsafe fn xcb_res_query_client_resources_types_iterator (R : *query_client_resources_reply) -> type_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_res_query_client_resources_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_res_query_client_resources_reply (c : *connection,
                                                cookie : query_client_resources_cookie,
                                                e : **generic_error) -> *query_client_resources_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_res_query_client_pixmap_bytes (c : *connection,
                                             xid :  u32) -> query_client_pixmap_bytes_cookie;

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
unsafe fn xcb_res_query_client_pixmap_bytes_unchecked (c : *connection,
                                                       xid :  u32) -> query_client_pixmap_bytes_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_res_query_client_pixmap_bytes_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_res_query_client_pixmap_bytes_reply (c : *connection,
                                                   cookie : query_client_pixmap_bytes_cookie,
                                                   e : **generic_error) -> *query_client_pixmap_bytes_reply;
}

