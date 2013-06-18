/*
 * This file generated automatically from xprint.xml by r_client.py.
 * Edit at your peril.
 */

//Make the compiler quiet
#[allow(unused_imports)];
#[allow(non_camel_case_types)];
use std;
use std::libc::*;
use std::{cast,num,ptr,str,libc};
use std::to_bytes::ToBytes;
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
    data : *string8,
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
    data : *printer,
    rem  : c_int,
    index: c_int
}


pub type pcontext = u32;
/**
 * @brief pcontext_iterator
 **/
pub struct pcontext_iterator {
    data : *pcontext,
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

#[link_args="-lxcb-xprint"]
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
pub unsafe fn xcb_x_print_string8_next (i:*string8_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An string8_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub unsafe fn xcb_x_print_string8_end (i:string8_iterator) -> generic_iterator;

pub unsafe fn xcb_x_print_printer_serialize (_buffer :  **c_void,
                               _aux :     *printer,
                               name :     *string8,
                               description :  *string8) -> c_int;

pub unsafe fn xcb_x_print_printer_unserialize (_buffer :   *c_void,
                                 _aux :     **printer) -> c_int;

pub unsafe fn xcb_x_print_printer_sizeof (_buffer :  *c_void) -> c_int;

pub unsafe fn xcb_x_print_printer_name (R : *printer) -> *string8;


pub unsafe fn xcb_x_print_printer_name_length (R : *printer) -> c_int;


pub unsafe fn xcb_x_print_printer_name_end (R : *printer) -> generic_iterator;

pub unsafe fn xcb_x_print_printer_description (R : *printer) -> *string8;


pub unsafe fn xcb_x_print_printer_description_length (R : *printer) -> c_int;


pub unsafe fn xcb_x_print_printer_description_end (R : *printer) -> generic_iterator;

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
pub unsafe fn xcb_x_print_printer_next (i:*printer_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An printer_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub unsafe fn xcb_x_print_printer_end (i:printer_iterator) -> generic_iterator;

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
pub unsafe fn xcb_x_print_pcontext_next (i:*pcontext_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An pcontext_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub unsafe fn xcb_x_print_pcontext_end (i:pcontext_iterator) -> generic_iterator;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_x_print_print_query_version (c : *connection) -> print_query_version_cookie;

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
pub unsafe fn xcb_x_print_print_query_version_unchecked (c : *connection) -> print_query_version_cookie;

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
pub unsafe fn xcb_x_print_print_query_version_reply (c : *connection,
                                                 cookie : print_query_version_cookie,
                                                 e : **generic_error) -> *print_query_version_reply;

pub unsafe fn xcb_x_print_print_get_printer_list_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_x_print_print_get_printer_list (c : *connection,
                                              printerNameLen :  u32,
                                              localeLen :  u32,
                                              printer_name : *string8,
                                              locale : *string8) -> print_get_printer_list_cookie;

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
pub unsafe fn xcb_x_print_print_get_printer_list_unchecked (c : *connection,
                                                        printerNameLen :  u32,
                                                        localeLen :  u32,
                                                        printer_name : *string8,
                                                        locale : *string8) -> print_get_printer_list_cookie;


pub unsafe fn xcb_x_print_print_get_printer_list_printers_length (R : *print_get_printer_list_reply) -> c_int;

pub unsafe fn xcb_x_print_print_get_printer_list_printers_iterator (R : *print_get_printer_list_reply) -> printer_iterator;

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
pub unsafe fn xcb_x_print_print_get_printer_list_reply (c : *connection,
                                                    cookie : print_get_printer_list_cookie,
                                                    e : **generic_error) -> *print_get_printer_list_reply;

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
pub unsafe fn xcb_x_print_print_rehash_printer_list_checked (c : *connection) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_x_print_print_rehash_printer_list (c : *connection) -> void_cookie;

pub unsafe fn xcb_x_print_create_context_sizeof (_buffer :  *c_void) -> c_int;

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
pub unsafe fn xcb_x_print_create_context_checked (c : *connection,
                                              context_id :  u32,
                                              printerNameLen :  u32,
                                              localeLen :  u32,
                                              printerName : *string8,
                                              locale : *string8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_x_print_create_context (c : *connection,
                                      context_id :  u32,
                                      printerNameLen :  u32,
                                      localeLen :  u32,
                                      printerName : *string8,
                                      locale : *string8) -> void_cookie;

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
pub unsafe fn xcb_x_print_print_set_context_checked (c : *connection,
                                                 context :  u32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_x_print_print_set_context (c : *connection,
                                         context :  u32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_x_print_print_get_context (c : *connection) -> print_get_context_cookie;

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
pub unsafe fn xcb_x_print_print_get_context_unchecked (c : *connection) -> print_get_context_cookie;

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
pub unsafe fn xcb_x_print_print_get_context_reply (c : *connection,
                                               cookie : print_get_context_cookie,
                                               e : **generic_error) -> *print_get_context_reply;

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
pub unsafe fn xcb_x_print_print_destroy_context_checked (c : *connection,
                                                     context :  u32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_x_print_print_destroy_context (c : *connection,
                                             context :  u32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_x_print_print_get_screen_of_context (c : *connection) -> print_get_screen_of_context_cookie;

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
pub unsafe fn xcb_x_print_print_get_screen_of_context_unchecked (c : *connection) -> print_get_screen_of_context_cookie;

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
pub unsafe fn xcb_x_print_print_get_screen_of_context_reply (c : *connection,
                                                         cookie : print_get_screen_of_context_cookie,
                                                         e : **generic_error) -> *print_get_screen_of_context_reply;

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
pub unsafe fn xcb_x_print_print_start_job_checked (c : *connection,
                                               output_mode :  u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_x_print_print_start_job (c : *connection,
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
pub unsafe fn xcb_x_print_print_end_job_checked (c : *connection,
                                             cancel :  u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_x_print_print_end_job (c : *connection,
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
pub unsafe fn xcb_x_print_print_start_doc_checked (c : *connection,
                                               driver_mode :  u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_x_print_print_start_doc (c : *connection,
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
pub unsafe fn xcb_x_print_print_end_doc_checked (c : *connection,
                                             cancel :  u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_x_print_print_end_doc (c : *connection,
                                     cancel :  u8) -> void_cookie;

pub unsafe fn xcb_x_print_print_put_document_data_sizeof (_buffer :  *c_void,
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
pub unsafe fn xcb_x_print_print_put_document_data_checked (c : *connection,
                                                       drawable :  ffi::xproto::drawable,
                                                       len_data :  u32,
                                                       len_fmt :  u16,
                                                       len_options :  u16,
                                                       data : *u8,
                                                       doc_format_len :  u32,
                                                       doc_format : *string8,
                                                       options_len :  u32,
                                                       options : *string8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_x_print_print_put_document_data (c : *connection,
                                               drawable :  ffi::xproto::drawable,
                                               len_data :  u32,
                                               len_fmt :  u16,
                                               len_options :  u16,
                                               data : *u8,
                                               doc_format_len :  u32,
                                               doc_format : *string8,
                                               options_len :  u32,
                                               options : *string8) -> void_cookie;

pub unsafe fn xcb_x_print_print_get_document_data_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_x_print_print_get_document_data (c : *connection,
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
pub unsafe fn xcb_x_print_print_get_document_data_unchecked (c : *connection,
                                                         context :  pcontext,
                                                         max_bytes :  u32) -> print_get_document_data_cookie;

pub unsafe fn xcb_x_print_print_get_document_data_data (R : *print_get_document_data_reply) -> *u8;


pub unsafe fn xcb_x_print_print_get_document_data_data_length (R : *print_get_document_data_reply) -> c_int;


pub unsafe fn xcb_x_print_print_get_document_data_data_end (R : *print_get_document_data_reply) -> generic_iterator;

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
pub unsafe fn xcb_x_print_print_get_document_data_reply (c : *connection,
                                                     cookie : print_get_document_data_cookie,
                                                     e : **generic_error) -> *print_get_document_data_reply;

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
pub unsafe fn xcb_x_print_print_start_page_checked (c : *connection,
                                                window :  ffi::xproto::window) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_x_print_print_start_page (c : *connection,
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
pub unsafe fn xcb_x_print_print_end_page_checked (c : *connection,
                                              cancel :  u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_x_print_print_end_page (c : *connection,
                                      cancel :  u8) -> void_cookie;

pub unsafe fn xcb_x_print_print_select_input_sizeof (_buffer :  *c_void) -> c_int;

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
pub unsafe fn xcb_x_print_print_select_input_checked (c : *connection,
                                                  context :  pcontext,
                                                  event_mask :  u32,
                                                  event_list : *u32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_x_print_print_select_input (c : *connection,
                                          context :  pcontext,
                                          event_mask :  u32,
                                          event_list : *u32) -> void_cookie;

pub unsafe fn xcb_x_print_print_input_selected_serialize (_buffer :                    **c_void,
                                            _aux :                        *print_input_selected_reply,
                                            event_list :                  *u32,
                                            all_events_list :             *u32) -> c_int;

pub unsafe fn xcb_x_print_print_input_selected_unserialize (_buffer :                      *c_void,
                                              _aux :                        **print_input_selected_reply) -> c_int;

pub unsafe fn xcb_x_print_print_input_selected_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_x_print_print_input_selected (c : *connection,
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
pub unsafe fn xcb_x_print_print_input_selected_unchecked (c : *connection,
                                                      context :  pcontext) -> print_input_selected_cookie;

pub unsafe fn xcb_x_print_print_input_selected_event_list (R : *print_input_selected_reply) -> *u32;


pub unsafe fn xcb_x_print_print_input_selected_event_list_length (R : *print_input_selected_reply) -> c_int;


pub unsafe fn xcb_x_print_print_input_selected_event_list_end (R : *print_input_selected_reply) -> generic_iterator;

pub unsafe fn xcb_x_print_print_input_selected_all_events_list (R : *print_input_selected_reply) -> *u32;


pub unsafe fn xcb_x_print_print_input_selected_all_events_list_length (R : *print_input_selected_reply) -> c_int;


pub unsafe fn xcb_x_print_print_input_selected_all_events_list_end (R : *print_input_selected_reply) -> generic_iterator;

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
pub unsafe fn xcb_x_print_print_input_selected_reply (c : *connection,
                                                  cookie : print_input_selected_cookie,
                                                  e : **generic_error) -> *print_input_selected_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_x_print_print_get_attributes (c : *connection,
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
pub unsafe fn xcb_x_print_print_get_attributes_unchecked (c : *connection,
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
pub unsafe fn xcb_x_print_print_get_attributes_reply (c : *connection,
                                                  cookie : print_get_attributes_cookie,
                                                  e : **generic_error) -> *print_get_attributes_reply;

pub unsafe fn xcb_x_print_print_get_one_attributes_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_x_print_print_get_one_attributes (c : *connection,
                                                context :  pcontext,
                                                nameLen :  u32,
                                                pool :  u8,
                                                name : *string8) -> print_get_one_attributes_cookie;

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
pub unsafe fn xcb_x_print_print_get_one_attributes_unchecked (c : *connection,
                                                          context :  pcontext,
                                                          nameLen :  u32,
                                                          pool :  u8,
                                                          name : *string8) -> print_get_one_attributes_cookie;

pub unsafe fn xcb_x_print_print_get_one_attributes_value (R : *print_get_one_attributes_reply) -> *string8;


pub unsafe fn xcb_x_print_print_get_one_attributes_value_length (R : *print_get_one_attributes_reply) -> c_int;


pub unsafe fn xcb_x_print_print_get_one_attributes_value_end (R : *print_get_one_attributes_reply) -> generic_iterator;

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
pub unsafe fn xcb_x_print_print_get_one_attributes_reply (c : *connection,
                                                      cookie : print_get_one_attributes_cookie,
                                                      e : **generic_error) -> *print_get_one_attributes_reply;

pub unsafe fn xcb_x_print_print_set_attributes_sizeof (_buffer :  *c_void,
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
pub unsafe fn xcb_x_print_print_set_attributes_checked (c : *connection,
                                                    context :  pcontext,
                                                    stringLen :  u32,
                                                    pool :  u8,
                                                    rule :  u8,
                                                    attributes_len :  u32,
                                                    attributes : *string8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_x_print_print_set_attributes (c : *connection,
                                            context :  pcontext,
                                            stringLen :  u32,
                                            pool :  u8,
                                            rule :  u8,
                                            attributes_len :  u32,
                                            attributes : *string8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_x_print_print_get_page_dimensions (c : *connection,
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
pub unsafe fn xcb_x_print_print_get_page_dimensions_unchecked (c : *connection,
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
pub unsafe fn xcb_x_print_print_get_page_dimensions_reply (c : *connection,
                                                       cookie : print_get_page_dimensions_cookie,
                                                       e : **generic_error) -> *print_get_page_dimensions_reply;

pub unsafe fn xcb_x_print_print_query_screens_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_x_print_print_query_screens (c : *connection) -> print_query_screens_cookie;

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
pub unsafe fn xcb_x_print_print_query_screens_unchecked (c : *connection) -> print_query_screens_cookie;

pub unsafe fn xcb_x_print_print_query_screens_roots (R : *print_query_screens_reply) -> *ffi::xproto::window;


pub unsafe fn xcb_x_print_print_query_screens_roots_length (R : *print_query_screens_reply) -> c_int;


pub unsafe fn xcb_x_print_print_query_screens_roots_end (R : *print_query_screens_reply) -> generic_iterator;

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
pub unsafe fn xcb_x_print_print_query_screens_reply (c : *connection,
                                                 cookie : print_query_screens_cookie,
                                                 e : **generic_error) -> *print_query_screens_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_x_print_print_set_image_resolution (c : *connection,
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
pub unsafe fn xcb_x_print_print_set_image_resolution_unchecked (c : *connection,
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
pub unsafe fn xcb_x_print_print_set_image_resolution_reply (c : *connection,
                                                        cookie : print_set_image_resolution_cookie,
                                                        e : **generic_error) -> *print_set_image_resolution_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_x_print_print_get_image_resolution (c : *connection,
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
pub unsafe fn xcb_x_print_print_get_image_resolution_unchecked (c : *connection,
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
pub unsafe fn xcb_x_print_print_get_image_resolution_reply (c : *connection,
                                                        cookie : print_get_image_resolution_cookie,
                                                        e : **generic_error) -> *print_get_image_resolution_reply;
}

