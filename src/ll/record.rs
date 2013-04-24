/*
 * This file generated automatically from record.xml by r_client.py.
 * Edit at your peril.
 */

//Make the compiler quiet
#[allow(unused_imports)];
#[allow(non_camel_case_types)];
use core;
use core::libc::*;
use ll::base::*;
use ll;

pub static RECORD_MAJOR_VERSION : c_uint = 1;
pub static RECORD_MINOR_VERSION : c_uint = 13;

pub type context = u32;
/**
 * @brief context_iterator
 **/
pub struct context_iterator {
    data : *context,
    rem  : c_int,
    index: c_int
}


pub struct range_8 {
    first :   u8,
    last :    u8
}

/**
 * @brief range_8_iterator
 **/
pub struct range_8_iterator {
    data : *range_8,
    rem  : c_int,
    index: c_int
}


pub struct range_16 {
    first :   u16,
    last :    u16
}

/**
 * @brief range_16_iterator
 **/
pub struct range_16_iterator {
    data : *range_16,
    rem  : c_int,
    index: c_int
}


pub struct ext_range {
    major :   range_8,
    minor :   range_16
}

/**
 * @brief ext_range_iterator
 **/
pub struct ext_range_iterator {
    data : *ext_range,
    rem  : c_int,
    index: c_int
}


pub struct range {
    core_requests :      range_8,
    core_replies :       range_8,
    ext_requests :       ext_range,
    ext_replies :        ext_range,
    delivered_events :   range_8,
    device_events :      range_8,
    errors :             range_8,
    client_started :     u8,
    client_died :        u8
}

/**
 * @brief range_iterator
 **/
pub struct range_iterator {
    data : *range,
    rem  : c_int,
    index: c_int
}


pub type element_header = u8;
/**
 * @brief element_header_iterator
 **/
pub struct element_header_iterator {
    data : *element_header,
    rem  : c_int,
    index: c_int
}


pub type client_spec = u32;
/**
 * @brief client_spec_iterator
 **/
pub struct client_spec_iterator {
    data : *client_spec,
    rem  : c_int,
    index: c_int
}


pub struct client_info {
    client_resource :   client_spec,
    num_ranges :        u32
}

/**
 * @brief client_info_iterator
 **/
pub struct client_info_iterator {
    data : *client_info,
    rem  : c_int,
    index: c_int
}



pub struct bad_context_error {
    response_type :    u8,
    error_code :       u8,
    sequence :         u16,
    invalid_record :   u32
}


pub struct query_version_cookie {
    sequence : c_uint
}


pub struct query_version_request {
    major_opcode :    u8,
    minor_opcode :    u8,
    length :          u16,
    major_version :   u16,
    minor_version :   u16
}


pub struct query_version_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    major_version :   u16,
    minor_version :   u16
}



pub struct create_context_request {
    major_opcode :       u8,
    minor_opcode :       u8,
    length :             u16,
    context :            context,
    element_header :     element_header,
    pad0 :               [u8,..3],
    num_client_specs :   u32,
    num_ranges :         u32
}



pub struct register_clients_request {
    major_opcode :       u8,
    minor_opcode :       u8,
    length :             u16,
    context :            context,
    element_header :     element_header,
    pad0 :               [u8,..3],
    num_client_specs :   u32,
    num_ranges :         u32
}



pub struct unregister_clients_request {
    major_opcode :       u8,
    minor_opcode :       u8,
    length :             u16,
    context :            context,
    num_client_specs :   u32
}


pub struct get_context_cookie {
    sequence : c_uint
}


pub struct get_context_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context :        context
}


pub struct get_context_reply {
    response_type :             u8,
    enabled :                   u8,
    sequence :                  u16,
    length :                    u32,
    element_header :            element_header,
    pad0 :                      [u8,..3],
    num_intercepted_clients :   u32,
    pad1 :                      [u8,..16]
}


pub struct enable_context_cookie {
    sequence : c_uint
}


pub struct enable_context_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context :        context
}


pub struct enable_context_reply {
    response_type :      u8,
    category :           u8,
    sequence :           u16,
    length :             u32,
    element_header :     element_header,
    client_swapped :     u8,
    pad0 :               [u8,..2],
    xid_base :           u32,
    server_time :        u32,
    rec_sequence_num :   u32,
    pad1 :               [u8,..8]
}



pub struct disable_context_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context :        context
}



pub struct free_context_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context :        context
}

#[link_args="-lxcb-record"]
pub extern "C" {

/**
 * Get the next element of the iterator
 * @param i Pointer to a context_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(context)
 *
 *
 */
unsafe fn xcb_record_context_next (i:*context_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An context_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_record_context_end (i:context_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a range_8_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(range_8)
 *
 *
 */
unsafe fn xcb_record_range_8_next (i:*range_8_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An range_8_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_record_range_8_end (i:range_8_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a range_16_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(range_16)
 *
 *
 */
unsafe fn xcb_record_range_16_next (i:*range_16_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An range_16_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_record_range_16_end (i:range_16_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a ext_range_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(ext_range)
 *
 *
 */
unsafe fn xcb_record_ext_range_next (i:*ext_range_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An ext_range_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_record_ext_range_end (i:ext_range_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a range_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(range)
 *
 *
 */
unsafe fn xcb_record_range_next (i:*range_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An range_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_record_range_end (i:range_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a element_header_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(element_header)
 *
 *
 */
unsafe fn xcb_record_element_header_next (i:*element_header_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An element_header_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_record_element_header_end (i:element_header_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a client_spec_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(client_spec)
 *
 *
 */
unsafe fn xcb_record_client_spec_next (i:*client_spec_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An client_spec_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_record_client_spec_end (i:client_spec_iterator) -> generic_iterator;

unsafe fn xcb_record_client_info_sizeof (_buffer :  *c_void) -> c_int;

unsafe fn xcb_record_client_info_ranges (R : *client_info) -> *range;


unsafe fn xcb_record_client_info_ranges_length (R : *client_info) -> c_int;

unsafe fn xcb_record_client_info_ranges_iterator (R : *client_info) -> range_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a client_info_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(client_info)
 *
 *
 */
unsafe fn xcb_record_client_info_next (i:*client_info_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An client_info_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_record_client_info_end (i:client_info_iterator) -> generic_iterator;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_record_query_version (c : *connection,
                                    major_version :  u16,
                                    minor_version :  u16) -> query_version_cookie;

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
unsafe fn xcb_record_query_version_unchecked (c : *connection,
                                              major_version :  u16,
                                              minor_version :  u16) -> query_version_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_record_query_version_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_record_query_version_reply (c : *connection,
                                          cookie : query_version_cookie,
                                          e : **generic_error) -> *query_version_reply;

unsafe fn xcb_record_create_context_sizeof (_buffer :  *c_void) -> c_int;

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
unsafe fn xcb_record_create_context_checked (c : *connection,
                                             context :  context,
                                             element_header :  element_header,
                                             num_client_specs :  u32,
                                             num_ranges :  u32,
                                             client_specs : *client_spec,
                                             ranges : *range) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_record_create_context (c : *connection,
                                     context :  context,
                                     element_header :  element_header,
                                     num_client_specs :  u32,
                                     num_ranges :  u32,
                                     client_specs : *client_spec,
                                     ranges : *range) -> void_cookie;

unsafe fn xcb_record_register_clients_sizeof (_buffer :  *c_void) -> c_int;

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
unsafe fn xcb_record_register_clients_checked (c : *connection,
                                               context :  context,
                                               element_header :  element_header,
                                               num_client_specs :  u32,
                                               num_ranges :  u32,
                                               client_specs : *client_spec,
                                               ranges : *range) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_record_register_clients (c : *connection,
                                       context :  context,
                                       element_header :  element_header,
                                       num_client_specs :  u32,
                                       num_ranges :  u32,
                                       client_specs : *client_spec,
                                       ranges : *range) -> void_cookie;

unsafe fn xcb_record_unregister_clients_sizeof (_buffer :  *c_void) -> c_int;

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
unsafe fn xcb_record_unregister_clients_checked (c : *connection,
                                                 context :  context,
                                                 num_client_specs :  u32,
                                                 client_specs : *client_spec) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_record_unregister_clients (c : *connection,
                                         context :  context,
                                         num_client_specs :  u32,
                                         client_specs : *client_spec) -> void_cookie;

unsafe fn xcb_record_get_context_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_record_get_context (c : *connection,
                                  context :  context) -> get_context_cookie;

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
unsafe fn xcb_record_get_context_unchecked (c : *connection,
                                            context :  context) -> get_context_cookie;


unsafe fn xcb_record_get_context_intercepted_clients_length (R : *get_context_reply) -> c_int;

unsafe fn xcb_record_get_context_intercepted_clients_iterator (R : *get_context_reply) -> client_info_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_record_get_context_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_record_get_context_reply (c : *connection,
                                        cookie : get_context_cookie,
                                        e : **generic_error) -> *get_context_reply;

unsafe fn xcb_record_enable_context_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_record_enable_context (c : *connection,
                                     context :  context) -> enable_context_cookie;

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
unsafe fn xcb_record_enable_context_unchecked (c : *connection,
                                               context :  context) -> enable_context_cookie;

unsafe fn xcb_record_enable_context_data (R : *enable_context_reply) -> *u8;


unsafe fn xcb_record_enable_context_data_length (R : *enable_context_reply) -> c_int;


unsafe fn xcb_record_enable_context_data_end (R : *enable_context_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_record_enable_context_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_record_enable_context_reply (c : *connection,
                                           cookie : enable_context_cookie,
                                           e : **generic_error) -> *enable_context_reply;

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
unsafe fn xcb_record_disable_context_checked (c : *connection,
                                              context :  context) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_record_disable_context (c : *connection,
                                      context :  context) -> void_cookie;

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
unsafe fn xcb_record_free_context_checked (c : *connection,
                                           context :  context) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_record_free_context (c : *connection,
                                   context :  context) -> void_cookie;
}

