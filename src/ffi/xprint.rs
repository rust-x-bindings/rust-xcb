/*
 * This file generated automatically from xprint.xml by r_client.py.
 * Edit at your peril.
 */

//Make the compiler quiet
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use std;
use libc::*;
use std::{mem,num,ptr,str};
use ffi::base::*;
use ffi;
use ffi::xproto;

pub static XPRINT_MAJOR_VERSION : c_uint = 1;
pub static XPRINT_MINOR_VERSION : c_uint = 0;

pub type string8 = c_char;
/**
 * @brief string8_iterator
 **/
pub struct string8_iterator {
    data : *mut string8,
    rem  : c_int,
    index: c_int
}


pub struct printer {
    nameLen :   u32,
    descLen :   u32
}

/**
 * @brief printer_iterator
 **/
pub struct printer_iterator {
    data : *mut printer,
    rem  : c_int,
    index: c_int
}


pub type pcontext = u32;
/**
 * @brief pcontext_iterator
 **/
pub struct pcontext_iterator {
    data : *mut pcontext,
    rem  : c_int,
    index: c_int
}


pub struct print_query_version_cookie {
    sequence : c_uint
}


pub struct print_query_version_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16
}


pub struct print_query_version_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    major_version :   u16,
    minor_version :   u16
}


pub struct print_get_printer_list_cookie {
    sequence : c_uint
}


pub struct print_get_printer_list_request {
    major_opcode :     u8,
    minor_opcode :     u8,
    length :           u16,
    printerNameLen :   u32,
    localeLen :        u32
}


pub struct print_get_printer_list_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    listCount :       u32,
    pad1 :            [u8,..20]
}



pub struct print_rehash_printer_list_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16
}



pub struct create_context_request {
    major_opcode :     u8,
    minor_opcode :     u8,
    length :           u16,
    context_id :       u32,
    printerNameLen :   u32,
    localeLen :        u32
}



pub struct print_set_context_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context :        u32
}


pub struct print_get_context_cookie {
    sequence : c_uint
}


pub struct print_get_context_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16
}


pub struct print_get_context_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    context :         u32
}



pub struct print_destroy_context_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context :        u32
}


pub struct print_get_screen_of_context_cookie {
    sequence : c_uint
}


pub struct print_get_screen_of_context_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16
}


pub struct print_get_screen_of_context_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    root :            ffi::xproto::window
}



pub struct print_start_job_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    output_mode :    u8
}



pub struct print_end_job_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    cancel :         u8
}



pub struct print_start_doc_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    driver_mode :    u8
}



pub struct print_end_doc_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    cancel :         u8
}



pub struct print_put_document_data_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    drawable :       ffi::xproto::drawable,
    len_data :       u32,
    len_fmt :        u16,
    len_options :    u16
}


pub struct print_get_document_data_cookie {
    sequence : c_uint
}


pub struct print_get_document_data_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context :        pcontext,
    max_bytes :      u32
}


pub struct print_get_document_data_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    status_code :     u32,
    finished_flag :   u32,
    dataLen :         u32,
    pad1 :            [u8,..12]
}



pub struct print_start_page_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    window :         ffi::xproto::window
}



pub struct print_end_page_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    cancel :         u8,
    pad0 :           [u8,..3]
}



pub struct print_select_input_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context :        pcontext,
    event_mask :     u32
}


pub struct print_input_selected_cookie {
    sequence : c_uint
}


pub struct print_input_selected_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context :        pcontext
}


pub struct print_input_selected_reply {
    response_type :     u8,
    pad0 :              u8,
    sequence :          u16,
    length :            u32,
    event_mask :        u32,
    all_events_mask :   u32
}


pub struct print_get_attributes_cookie {
    sequence : c_uint
}


pub struct print_get_attributes_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context :        pcontext,
    pool :           u8,
    pad0 :           [u8,..3]
}


pub struct print_get_attributes_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    stringLen :       u32,
    pad1 :            [u8,..20],
    attributes :      string8
}


pub struct print_get_one_attributes_cookie {
    sequence : c_uint
}


pub struct print_get_one_attributes_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context :        pcontext,
    nameLen :        u32,
    pool :           u8,
    pad0 :           [u8,..3]
}


pub struct print_get_one_attributes_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    valueLen :        u32,
    pad1 :            [u8,..20]
}



pub struct print_set_attributes_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context :        pcontext,
    stringLen :      u32,
    pool :           u8,
    rule :           u8,
    pad0 :           [u8,..2]
}


pub struct print_get_page_dimensions_cookie {
    sequence : c_uint
}


pub struct print_get_page_dimensions_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context :        pcontext
}


pub struct print_get_page_dimensions_reply {
    response_type :         u8,
    pad0 :                  u8,
    sequence :              u16,
    length :                u32,
    width :                 u16,
    height :                u16,
    offset_x :              u16,
    offset_y :              u16,
    reproducible_width :    u16,
    reproducible_height :   u16
}


pub struct print_query_screens_cookie {
    sequence : c_uint
}


pub struct print_query_screens_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16
}


pub struct print_query_screens_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    listCount :       u32,
    pad1 :            [u8,..20]
}


pub struct print_set_image_resolution_cookie {
    sequence : c_uint
}


pub struct print_set_image_resolution_request {
    major_opcode :       u8,
    minor_opcode :       u8,
    length :             u16,
    context :            pcontext,
    image_resolution :   u16
}


pub struct print_set_image_resolution_reply {
    response_type :          u8,
    status :                 u8,
    sequence :               u16,
    length :                 u32,
    previous_resolutions :   u16
}


pub struct print_get_image_resolution_cookie {
    sequence : c_uint
}


pub struct print_get_image_resolution_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context :        pcontext
}


pub struct print_get_image_resolution_reply {
    response_type :      u8,
    pad0 :               u8,
    sequence :           u16,
    length :             u32,
    image_resolution :   u16
}



pub struct notify_event {
    response_type :   u8,
    detail :          u8,
    sequence :        u16,
    context :         pcontext,
    cancel :          u8
}



pub struct attribut_notify_event {
    response_type :   u8,
    detail :          u8,
    sequence :        u16,
    context :         pcontext
}



pub struct bad_context_error {
    response_type :   u8,
    error_code :      u8,
    sequence :        u16
}



pub struct bad_sequence_error {
    response_type :   u8,
    error_code :      u8,
    sequence :        u16
}

#[link(name="lxcb-xprint")]
pub extern "C" {

/**
 * Get the next element of the iterator
 * @param i Pointer to a string8_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(string8)
 *
 *
 */
pub fn xcb_x_print_string8_next (i:*mut string8_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An string8_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_x_print_string8_end (i:string8_iterator) -> generic_iterator;

pub fn xcb_x_print_printer_serialize (_buffer :  *mut *mut c_void,
                               _aux :         *mut printer,
                               name :         *mut string8,
                               description :  *mut string8) -> c_int;

pub fn xcb_x_print_printer_unserialize (_buffer :       *mut c_void,
                                 _aux :     *mut *mut printer) -> c_int;

pub fn xcb_x_print_printer_sizeof (_buffer :  *mut c_void) -> c_int;

pub fn xcb_x_print_printer_name (R : *mut printer) -> *mut string8;


pub fn xcb_x_print_printer_name_length (R : *mut printer) -> c_int;


pub fn xcb_x_print_printer_name_end (R : *mut printer) -> generic_iterator;

pub fn xcb_x_print_printer_description (R : *mut printer) -> *mut string8;


pub fn xcb_x_print_printer_description_length (R : *mut printer) -> c_int;


pub fn xcb_x_print_printer_description_end (R : *mut printer) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a printer_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(printer)
 *
 *
 */
pub fn xcb_x_print_printer_next (i:*mut printer_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An printer_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_x_print_printer_end (i:printer_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a pcontext_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(pcontext)
 *
 *
 */
pub fn xcb_x_print_pcontext_next (i:*mut pcontext_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An pcontext_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_x_print_pcontext_end (i:pcontext_iterator) -> generic_iterator;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_x_print_print_query_version (c : *mut connection) -> print_query_version_cookie;

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
pub fn xcb_x_print_print_query_version_unchecked (c : *mut connection) -> print_query_version_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_x_print_print_query_version_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_x_print_print_query_version_reply (c : *mut connection,
                                                 cookie : print_query_version_cookie,
                                                 e : *mut *mut generic_error) -> *mut print_query_version_reply;

pub fn xcb_x_print_print_get_printer_list_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_x_print_print_get_printer_list (c : *mut connection,
                                              printerNameLen :  u32,
                                              localeLen :  u32,
                                              printer_name : *mut string8,
                                              locale : *mut string8) -> print_get_printer_list_cookie;

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
pub fn xcb_x_print_print_get_printer_list_unchecked (c : *mut connection,
                                                        printerNameLen :  u32,
                                                        localeLen :  u32,
                                                        printer_name : *mut string8,
                                                        locale : *mut string8) -> print_get_printer_list_cookie;


pub fn xcb_x_print_print_get_printer_list_printers_length (R : *mut print_get_printer_list_reply) -> c_int;

pub fn xcb_x_print_print_get_printer_list_printers_iterator (R : *mut print_get_printer_list_reply) -> printer_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_x_print_print_get_printer_list_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_x_print_print_get_printer_list_reply (c : *mut connection,
                                                    cookie : print_get_printer_list_cookie,
                                                    e : *mut *mut generic_error) -> *mut print_get_printer_list_reply;

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
pub fn xcb_x_print_print_rehash_printer_list_checked (c : *mut connection) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_x_print_print_rehash_printer_list (c : *mut connection) -> void_cookie;

pub fn xcb_x_print_create_context_sizeof (_buffer :  *mut c_void) -> c_int;

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
pub fn xcb_x_print_create_context_checked (c : *mut connection,
                                              context_id :  u32,
                                              printerNameLen :  u32,
                                              localeLen :  u32,
                                              printerName : *mut string8,
                                              locale : *mut string8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_x_print_create_context (c : *mut connection,
                                      context_id :  u32,
                                      printerNameLen :  u32,
                                      localeLen :  u32,
                                      printerName : *mut string8,
                                      locale : *mut string8) -> void_cookie;

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
pub fn xcb_x_print_print_set_context_checked (c : *mut connection,
                                                 context :  u32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_x_print_print_set_context (c : *mut connection,
                                         context :  u32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_x_print_print_get_context (c : *mut connection) -> print_get_context_cookie;

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
pub fn xcb_x_print_print_get_context_unchecked (c : *mut connection) -> print_get_context_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_x_print_print_get_context_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_x_print_print_get_context_reply (c : *mut connection,
                                               cookie : print_get_context_cookie,
                                               e : *mut *mut generic_error) -> *mut print_get_context_reply;

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
pub fn xcb_x_print_print_destroy_context_checked (c : *mut connection,
                                                     context :  u32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_x_print_print_destroy_context (c : *mut connection,
                                             context :  u32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_x_print_print_get_screen_of_context (c : *mut connection) -> print_get_screen_of_context_cookie;

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
pub fn xcb_x_print_print_get_screen_of_context_unchecked (c : *mut connection) -> print_get_screen_of_context_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_x_print_print_get_screen_of_context_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_x_print_print_get_screen_of_context_reply (c : *mut connection,
                                                         cookie : print_get_screen_of_context_cookie,
                                                         e : *mut *mut generic_error) -> *mut print_get_screen_of_context_reply;

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
pub fn xcb_x_print_print_start_job_checked (c : *mut connection,
                                               output_mode :  u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_x_print_print_start_job (c : *mut connection,
                                       output_mode :  u8) -> void_cookie;

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
pub fn xcb_x_print_print_end_job_checked (c : *mut connection,
                                             cancel :  u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_x_print_print_end_job (c : *mut connection,
                                     cancel :  u8) -> void_cookie;

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
pub fn xcb_x_print_print_start_doc_checked (c : *mut connection,
                                               driver_mode :  u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_x_print_print_start_doc (c : *mut connection,
                                       driver_mode :  u8) -> void_cookie;

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
pub fn xcb_x_print_print_end_doc_checked (c : *mut connection,
                                             cancel :  u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_x_print_print_end_doc (c : *mut connection,
                                     cancel :  u8) -> void_cookie;

pub fn xcb_x_print_print_put_document_data_sizeof (_buffer :  *mut c_void,
                                            doc_format_len :  u32,
                                            options_len :  u32) -> c_int;

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
pub fn xcb_x_print_print_put_document_data_checked (c : *mut connection,
                                                       drawable :  ffi::xproto::drawable,
                                                       len_data :  u32,
                                                       len_fmt :  u16,
                                                       len_options :  u16,
                                                       data : *mut u8,
                                                       doc_format_len :  u32,
                                                       doc_format : *mut string8,
                                                       options_len :  u32,
                                                       options : *mut string8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_x_print_print_put_document_data (c : *mut connection,
                                               drawable :  ffi::xproto::drawable,
                                               len_data :  u32,
                                               len_fmt :  u16,
                                               len_options :  u16,
                                               data : *mut u8,
                                               doc_format_len :  u32,
                                               doc_format : *mut string8,
                                               options_len :  u32,
                                               options : *mut string8) -> void_cookie;

pub fn xcb_x_print_print_get_document_data_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_x_print_print_get_document_data (c : *mut connection,
                                               context :  pcontext,
                                               max_bytes :  u32) -> print_get_document_data_cookie;

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
pub fn xcb_x_print_print_get_document_data_unchecked (c : *mut connection,
                                                         context :  pcontext,
                                                         max_bytes :  u32) -> print_get_document_data_cookie;

pub fn xcb_x_print_print_get_document_data_data (R : *mut print_get_document_data_reply) -> *mut u8;


pub fn xcb_x_print_print_get_document_data_data_length (R : *mut print_get_document_data_reply) -> c_int;


pub fn xcb_x_print_print_get_document_data_data_end (R : *mut print_get_document_data_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_x_print_print_get_document_data_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_x_print_print_get_document_data_reply (c : *mut connection,
                                                     cookie : print_get_document_data_cookie,
                                                     e : *mut *mut generic_error) -> *mut print_get_document_data_reply;

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
pub fn xcb_x_print_print_start_page_checked (c : *mut connection,
                                                window :  ffi::xproto::window) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_x_print_print_start_page (c : *mut connection,
                                        window :  ffi::xproto::window) -> void_cookie;

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
pub fn xcb_x_print_print_end_page_checked (c : *mut connection,
                                              cancel :  u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_x_print_print_end_page (c : *mut connection,
                                      cancel :  u8) -> void_cookie;

pub fn xcb_x_print_print_select_input_sizeof (_buffer :  *mut c_void) -> c_int;

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
pub fn xcb_x_print_print_select_input_checked (c : *mut connection,
                                                  context :  pcontext,
                                                  event_mask :  u32,
                                                  event_list : *mut u32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_x_print_print_select_input (c : *mut connection,
                                          context :  pcontext,
                                          event_mask :  u32,
                                          event_list : *mut u32) -> void_cookie;

pub fn xcb_x_print_print_input_selected_serialize (_buffer :                *mut *mut c_void,
                                            _aux :                        *mut print_input_selected_reply,
                                            event_list :                  *mut u32,
                                            all_events_list :             *mut u32) -> c_int;

pub fn xcb_x_print_print_input_selected_unserialize (_buffer :                          *mut c_void,
                                              _aux :                        *mut *mut print_input_selected_reply) -> c_int;

pub fn xcb_x_print_print_input_selected_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_x_print_print_input_selected (c : *mut connection,
                                            context :  pcontext) -> print_input_selected_cookie;

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
pub fn xcb_x_print_print_input_selected_unchecked (c : *mut connection,
                                                      context :  pcontext) -> print_input_selected_cookie;

pub fn xcb_x_print_print_input_selected_event_list (R : *mut print_input_selected_reply) -> *mut u32;


pub fn xcb_x_print_print_input_selected_event_list_length (R : *mut print_input_selected_reply) -> c_int;


pub fn xcb_x_print_print_input_selected_event_list_end (R : *mut print_input_selected_reply) -> generic_iterator;

pub fn xcb_x_print_print_input_selected_all_events_list (R : *mut print_input_selected_reply) -> *mut u32;


pub fn xcb_x_print_print_input_selected_all_events_list_length (R : *mut print_input_selected_reply) -> c_int;


pub fn xcb_x_print_print_input_selected_all_events_list_end (R : *mut print_input_selected_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_x_print_print_input_selected_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_x_print_print_input_selected_reply (c : *mut connection,
                                                  cookie : print_input_selected_cookie,
                                                  e : *mut *mut generic_error) -> *mut print_input_selected_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_x_print_print_get_attributes (c : *mut connection,
                                            context :  pcontext,
                                            pool :  u8) -> print_get_attributes_cookie;

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
pub fn xcb_x_print_print_get_attributes_unchecked (c : *mut connection,
                                                      context :  pcontext,
                                                      pool :  u8) -> print_get_attributes_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_x_print_print_get_attributes_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_x_print_print_get_attributes_reply (c : *mut connection,
                                                  cookie : print_get_attributes_cookie,
                                                  e : *mut *mut generic_error) -> *mut print_get_attributes_reply;

pub fn xcb_x_print_print_get_one_attributes_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_x_print_print_get_one_attributes (c : *mut connection,
                                                context :  pcontext,
                                                nameLen :  u32,
                                                pool :  u8,
                                                name : *mut string8) -> print_get_one_attributes_cookie;

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
pub fn xcb_x_print_print_get_one_attributes_unchecked (c : *mut connection,
                                                          context :  pcontext,
                                                          nameLen :  u32,
                                                          pool :  u8,
                                                          name : *mut string8) -> print_get_one_attributes_cookie;

pub fn xcb_x_print_print_get_one_attributes_value (R : *mut print_get_one_attributes_reply) -> *mut string8;


pub fn xcb_x_print_print_get_one_attributes_value_length (R : *mut print_get_one_attributes_reply) -> c_int;


pub fn xcb_x_print_print_get_one_attributes_value_end (R : *mut print_get_one_attributes_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_x_print_print_get_one_attributes_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_x_print_print_get_one_attributes_reply (c : *mut connection,
                                                      cookie : print_get_one_attributes_cookie,
                                                      e : *mut *mut generic_error) -> *mut print_get_one_attributes_reply;

pub fn xcb_x_print_print_set_attributes_sizeof (_buffer :  *mut c_void,
                                         attributes_len :  u32) -> c_int;

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
pub fn xcb_x_print_print_set_attributes_checked (c : *mut connection,
                                                    context :  pcontext,
                                                    stringLen :  u32,
                                                    pool :  u8,
                                                    rule :  u8,
                                                    attributes_len :  u32,
                                                    attributes : *mut string8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_x_print_print_set_attributes (c : *mut connection,
                                            context :  pcontext,
                                            stringLen :  u32,
                                            pool :  u8,
                                            rule :  u8,
                                            attributes_len :  u32,
                                            attributes : *mut string8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_x_print_print_get_page_dimensions (c : *mut connection,
                                                 context :  pcontext) -> print_get_page_dimensions_cookie;

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
pub fn xcb_x_print_print_get_page_dimensions_unchecked (c : *mut connection,
                                                           context :  pcontext) -> print_get_page_dimensions_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_x_print_print_get_page_dimensions_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_x_print_print_get_page_dimensions_reply (c : *mut connection,
                                                       cookie : print_get_page_dimensions_cookie,
                                                       e : *mut *mut generic_error) -> *mut print_get_page_dimensions_reply;

pub fn xcb_x_print_print_query_screens_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_x_print_print_query_screens (c : *mut connection) -> print_query_screens_cookie;

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
pub fn xcb_x_print_print_query_screens_unchecked (c : *mut connection) -> print_query_screens_cookie;

pub fn xcb_x_print_print_query_screens_roots (R : *mut print_query_screens_reply) -> *mut ffi::xproto::window;


pub fn xcb_x_print_print_query_screens_roots_length (R : *mut print_query_screens_reply) -> c_int;


pub fn xcb_x_print_print_query_screens_roots_end (R : *mut print_query_screens_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_x_print_print_query_screens_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_x_print_print_query_screens_reply (c : *mut connection,
                                                 cookie : print_query_screens_cookie,
                                                 e : *mut *mut generic_error) -> *mut print_query_screens_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_x_print_print_set_image_resolution (c : *mut connection,
                                                  context :  pcontext,
                                                  image_resolution :  u16) -> print_set_image_resolution_cookie;

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
pub fn xcb_x_print_print_set_image_resolution_unchecked (c : *mut connection,
                                                            context :  pcontext,
                                                            image_resolution :  u16) -> print_set_image_resolution_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_x_print_print_set_image_resolution_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_x_print_print_set_image_resolution_reply (c : *mut connection,
                                                        cookie : print_set_image_resolution_cookie,
                                                        e : *mut *mut generic_error) -> *mut print_set_image_resolution_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_x_print_print_get_image_resolution (c : *mut connection,
                                                  context :  pcontext) -> print_get_image_resolution_cookie;

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
pub fn xcb_x_print_print_get_image_resolution_unchecked (c : *mut connection,
                                                            context :  pcontext) -> print_get_image_resolution_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_x_print_print_get_image_resolution_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_x_print_print_get_image_resolution_reply (c : *mut connection,
                                                        cookie : print_get_image_resolution_cookie,
                                                        e : *mut *mut generic_error) -> *mut print_get_image_resolution_reply;
}

