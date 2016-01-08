/*
 * This file generated automatically from xprint.xml by r_client.py.
 * Edit at your peril.
 */

//Make the compiler quiet
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use std;
use libc::*;
use ffi;
use ffi::xproto;

pub static XPRINT_MAJOR_VERSION : c_uint = 1;
pub static XPRINT_MINOR_VERSION : c_uint = 0;

pub type string8 = c_char;
/**
 * @brief string8_iterator
 **/
#[repr(C)]
pub struct string8_iterator {
    pub data : *mut string8,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct printer {
     pub nameLen :   u32,
     pub descLen :   u32
}

impl Copy for printer {}
impl Clone for printer {
    fn clone(&self) -> printer { *self }
}
/**
 * @brief printer_iterator
 **/
#[repr(C)]
pub struct printer_iterator {
    pub data : *mut printer,
    pub rem  : c_int,
    pub index: c_int
}


pub type pcontext = u32;
/**
 * @brief pcontext_iterator
 **/
#[repr(C)]
pub struct pcontext_iterator {
    pub data : *mut pcontext,
    pub rem  : c_int,
    pub index: c_int
}


#[derive(Copy, Clone)]
#[repr(C)]
pub struct print_query_version_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct print_query_version_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16
}

impl Copy for print_query_version_request {}
impl Clone for print_query_version_request {
    fn clone(&self) -> print_query_version_request { *self }
}

#[repr(C)]
pub struct print_query_version_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub major_version :   u16,
     pub minor_version :   u16
}

impl Copy for print_query_version_reply {}
impl Clone for print_query_version_reply {
    fn clone(&self) -> print_query_version_reply { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct print_get_printer_list_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct print_get_printer_list_request {
     pub major_opcode :     u8,
     pub minor_opcode :     u8,
     pub length :           u16,
     pub printerNameLen :   u32,
     pub localeLen :        u32
}

impl Copy for print_get_printer_list_request {}
impl Clone for print_get_printer_list_request {
    fn clone(&self) -> print_get_printer_list_request { *self }
}

#[repr(C)]
pub struct print_get_printer_list_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub listCount :       u32,
     pub pad1 :            [u8; 20]
}

impl Copy for print_get_printer_list_reply {}
impl Clone for print_get_printer_list_reply {
    fn clone(&self) -> print_get_printer_list_reply { *self }
}


#[repr(C)]
pub struct print_rehash_printer_list_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16
}

impl Copy for print_rehash_printer_list_request {}
impl Clone for print_rehash_printer_list_request {
    fn clone(&self) -> print_rehash_printer_list_request { *self }
}


#[repr(C)]
pub struct create_context_request {
     pub major_opcode :     u8,
     pub minor_opcode :     u8,
     pub length :           u16,
     pub context_id :       u32,
     pub printerNameLen :   u32,
     pub localeLen :        u32
}

impl Copy for create_context_request {}
impl Clone for create_context_request {
    fn clone(&self) -> create_context_request { *self }
}


#[repr(C)]
pub struct print_set_context_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub context :        u32
}

impl Copy for print_set_context_request {}
impl Clone for print_set_context_request {
    fn clone(&self) -> print_set_context_request { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct print_get_context_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct print_get_context_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16
}

impl Copy for print_get_context_request {}
impl Clone for print_get_context_request {
    fn clone(&self) -> print_get_context_request { *self }
}

#[repr(C)]
pub struct print_get_context_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub context :         u32
}

impl Copy for print_get_context_reply {}
impl Clone for print_get_context_reply {
    fn clone(&self) -> print_get_context_reply { *self }
}


#[repr(C)]
pub struct print_destroy_context_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub context :        u32
}

impl Copy for print_destroy_context_request {}
impl Clone for print_destroy_context_request {
    fn clone(&self) -> print_destroy_context_request { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct print_get_screen_of_context_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct print_get_screen_of_context_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16
}

impl Copy for print_get_screen_of_context_request {}
impl Clone for print_get_screen_of_context_request {
    fn clone(&self) -> print_get_screen_of_context_request { *self }
}

#[repr(C)]
pub struct print_get_screen_of_context_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub root :            ffi::xproto::window
}

impl Copy for print_get_screen_of_context_reply {}
impl Clone for print_get_screen_of_context_reply {
    fn clone(&self) -> print_get_screen_of_context_reply { *self }
}


#[repr(C)]
pub struct print_start_job_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub output_mode :    u8
}

impl Copy for print_start_job_request {}
impl Clone for print_start_job_request {
    fn clone(&self) -> print_start_job_request { *self }
}


#[repr(C)]
pub struct print_end_job_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub cancel :         u8
}

impl Copy for print_end_job_request {}
impl Clone for print_end_job_request {
    fn clone(&self) -> print_end_job_request { *self }
}


#[repr(C)]
pub struct print_start_doc_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub driver_mode :    u8
}

impl Copy for print_start_doc_request {}
impl Clone for print_start_doc_request {
    fn clone(&self) -> print_start_doc_request { *self }
}


#[repr(C)]
pub struct print_end_doc_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub cancel :         u8
}

impl Copy for print_end_doc_request {}
impl Clone for print_end_doc_request {
    fn clone(&self) -> print_end_doc_request { *self }
}


#[repr(C)]
pub struct print_put_document_data_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub drawable :       ffi::xproto::drawable,
     pub len_data :       u32,
     pub len_fmt :        u16,
     pub len_options :    u16
}

impl Copy for print_put_document_data_request {}
impl Clone for print_put_document_data_request {
    fn clone(&self) -> print_put_document_data_request { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct print_get_document_data_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct print_get_document_data_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub context :        pcontext,
     pub max_bytes :      u32
}

impl Copy for print_get_document_data_request {}
impl Clone for print_get_document_data_request {
    fn clone(&self) -> print_get_document_data_request { *self }
}

#[repr(C)]
pub struct print_get_document_data_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub status_code :     u32,
     pub finished_flag :   u32,
     pub dataLen :         u32,
     pub pad1 :            [u8; 12]
}

impl Copy for print_get_document_data_reply {}
impl Clone for print_get_document_data_reply {
    fn clone(&self) -> print_get_document_data_reply { *self }
}


#[repr(C)]
pub struct print_start_page_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub window :         ffi::xproto::window
}

impl Copy for print_start_page_request {}
impl Clone for print_start_page_request {
    fn clone(&self) -> print_start_page_request { *self }
}


#[repr(C)]
pub struct print_end_page_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub cancel :         u8,
     pub pad0 :           [u8; 3]
}

impl Copy for print_end_page_request {}
impl Clone for print_end_page_request {
    fn clone(&self) -> print_end_page_request { *self }
}


#[repr(C)]
pub struct print_select_input_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub context :        pcontext,
     pub event_mask :     u32
}

impl Copy for print_select_input_request {}
impl Clone for print_select_input_request {
    fn clone(&self) -> print_select_input_request { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct print_input_selected_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct print_input_selected_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub context :        pcontext
}

impl Copy for print_input_selected_request {}
impl Clone for print_input_selected_request {
    fn clone(&self) -> print_input_selected_request { *self }
}

#[repr(C)]
pub struct print_input_selected_reply {
     pub response_type :     u8,
     pub pad0 :              u8,
     pub sequence :          u16,
     pub length :            u32,
     pub event_mask :        u32,
     pub all_events_mask :   u32
}

impl Copy for print_input_selected_reply {}
impl Clone for print_input_selected_reply {
    fn clone(&self) -> print_input_selected_reply { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct print_get_attributes_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct print_get_attributes_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub context :        pcontext,
     pub pool :           u8,
     pub pad0 :           [u8; 3]
}

impl Copy for print_get_attributes_request {}
impl Clone for print_get_attributes_request {
    fn clone(&self) -> print_get_attributes_request { *self }
}

#[repr(C)]
pub struct print_get_attributes_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub stringLen :       u32,
     pub pad1 :            [u8; 20],
     pub attributes :      string8
}

impl Copy for print_get_attributes_reply {}
impl Clone for print_get_attributes_reply {
    fn clone(&self) -> print_get_attributes_reply { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct print_get_one_attributes_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct print_get_one_attributes_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub context :        pcontext,
     pub nameLen :        u32,
     pub pool :           u8,
     pub pad0 :           [u8; 3]
}

impl Copy for print_get_one_attributes_request {}
impl Clone for print_get_one_attributes_request {
    fn clone(&self) -> print_get_one_attributes_request { *self }
}

#[repr(C)]
pub struct print_get_one_attributes_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub valueLen :        u32,
     pub pad1 :            [u8; 20]
}

impl Copy for print_get_one_attributes_reply {}
impl Clone for print_get_one_attributes_reply {
    fn clone(&self) -> print_get_one_attributes_reply { *self }
}


#[repr(C)]
pub struct print_set_attributes_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub context :        pcontext,
     pub stringLen :      u32,
     pub pool :           u8,
     pub rule :           u8,
     pub pad0 :           [u8; 2]
}

impl Copy for print_set_attributes_request {}
impl Clone for print_set_attributes_request {
    fn clone(&self) -> print_set_attributes_request { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct print_get_page_dimensions_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct print_get_page_dimensions_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub context :        pcontext
}

impl Copy for print_get_page_dimensions_request {}
impl Clone for print_get_page_dimensions_request {
    fn clone(&self) -> print_get_page_dimensions_request { *self }
}

#[repr(C)]
pub struct print_get_page_dimensions_reply {
     pub response_type :         u8,
     pub pad0 :                  u8,
     pub sequence :              u16,
     pub length :                u32,
     pub width :                 u16,
     pub height :                u16,
     pub offset_x :              u16,
     pub offset_y :              u16,
     pub reproducible_width :    u16,
     pub reproducible_height :   u16
}

impl Copy for print_get_page_dimensions_reply {}
impl Clone for print_get_page_dimensions_reply {
    fn clone(&self) -> print_get_page_dimensions_reply { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct print_query_screens_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct print_query_screens_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16
}

impl Copy for print_query_screens_request {}
impl Clone for print_query_screens_request {
    fn clone(&self) -> print_query_screens_request { *self }
}

#[repr(C)]
pub struct print_query_screens_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub listCount :       u32,
     pub pad1 :            [u8; 20]
}

impl Copy for print_query_screens_reply {}
impl Clone for print_query_screens_reply {
    fn clone(&self) -> print_query_screens_reply { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct print_set_image_resolution_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct print_set_image_resolution_request {
     pub major_opcode :       u8,
     pub minor_opcode :       u8,
     pub length :             u16,
     pub context :            pcontext,
     pub image_resolution :   u16
}

impl Copy for print_set_image_resolution_request {}
impl Clone for print_set_image_resolution_request {
    fn clone(&self) -> print_set_image_resolution_request { *self }
}

#[repr(C)]
pub struct print_set_image_resolution_reply {
     pub response_type :          u8,
     pub status :                 u8,
     pub sequence :               u16,
     pub length :                 u32,
     pub previous_resolutions :   u16
}

impl Copy for print_set_image_resolution_reply {}
impl Clone for print_set_image_resolution_reply {
    fn clone(&self) -> print_set_image_resolution_reply { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct print_get_image_resolution_cookie {
    sequence : c_uint
}


#[repr(C)]
pub struct print_get_image_resolution_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub context :        pcontext
}

impl Copy for print_get_image_resolution_request {}
impl Clone for print_get_image_resolution_request {
    fn clone(&self) -> print_get_image_resolution_request { *self }
}

#[repr(C)]
pub struct print_get_image_resolution_reply {
     pub response_type :      u8,
     pub pad0 :               u8,
     pub sequence :           u16,
     pub length :             u32,
     pub image_resolution :   u16
}

impl Copy for print_get_image_resolution_reply {}
impl Clone for print_get_image_resolution_reply {
    fn clone(&self) -> print_get_image_resolution_reply { *self }
}


#[repr(C)]
pub struct notify_event {
     pub response_type :   u8,
     pub detail :          u8,
     pub sequence :        u16,
     pub context :         pcontext,
     pub cancel :          u8
}

impl Copy for notify_event {}
impl Clone for notify_event {
    fn clone(&self) -> notify_event { *self }
}


#[repr(C)]
pub struct attribut_notify_event {
     pub response_type :   u8,
     pub detail :          u8,
     pub sequence :        u16,
     pub context :         pcontext
}

impl Copy for attribut_notify_event {}
impl Clone for attribut_notify_event {
    fn clone(&self) -> attribut_notify_event { *self }
}


#[repr(C)]
pub struct bad_context_error {
     pub response_type :   u8,
     pub error_code :      u8,
     pub sequence :        u16
}

impl Copy for bad_context_error {}
impl Clone for bad_context_error {
    fn clone(&self) -> bad_context_error { *self }
}


#[repr(C)]
pub struct bad_sequence_error {
     pub response_type :   u8,
     pub error_code :      u8,
     pub sequence :        u16
}

impl Copy for bad_sequence_error {}
impl Clone for bad_sequence_error {
    fn clone(&self) -> bad_sequence_error { *self }
}
#[link(name="xcb-xprint")]
extern "C" {

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
pub fn xcb_x_print_string8_end (i:string8_iterator) -> ffi::base::generic_iterator;

pub fn xcb_x_print_printer_serialize (_buffer :  *mut *mut c_void,
                               _aux :         *mut printer,
                               name :         *mut string8,
                               description :  *mut string8) -> c_int;

pub fn xcb_x_print_printer_unserialize (_buffer :       *mut c_void,
                                 _aux :     *mut *mut printer) -> c_int;

pub fn xcb_x_print_printer_sizeof (_buffer :  *mut c_void) -> c_int;

pub fn xcb_x_print_printer_name (R : *mut printer) -> *mut string8;


pub fn xcb_x_print_printer_name_length (R : *mut printer) -> c_int;


pub fn xcb_x_print_printer_name_end (R : *mut printer) -> ffi::base::generic_iterator;

pub fn xcb_x_print_printer_description (R : *mut printer) -> *mut string8;


pub fn xcb_x_print_printer_description_length (R : *mut printer) -> c_int;


pub fn xcb_x_print_printer_description_end (R : *mut printer) -> ffi::base::generic_iterator;

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
pub fn xcb_x_print_printer_end (i:printer_iterator) -> ffi::base::generic_iterator;

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
pub fn xcb_x_print_pcontext_end (i:pcontext_iterator) -> ffi::base::generic_iterator;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_x_print_print_query_version (c : *mut ffi::base::connection) -> print_query_version_cookie;

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
pub fn xcb_x_print_print_query_version_unchecked (c : *mut ffi::base::connection) -> print_query_version_cookie;

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
pub fn xcb_x_print_print_query_version_reply (c : *mut ffi::base::connection,
                                                 cookie : print_query_version_cookie,
                                                 e : *mut *mut ffi::base::generic_error) -> *mut print_query_version_reply;

pub fn xcb_x_print_print_get_printer_list_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_x_print_print_get_printer_list (c : *mut ffi::base::connection,
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
pub fn xcb_x_print_print_get_printer_list_unchecked (c : *mut ffi::base::connection,
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
pub fn xcb_x_print_print_get_printer_list_reply (c : *mut ffi::base::connection,
                                                    cookie : print_get_printer_list_cookie,
                                                    e : *mut *mut ffi::base::generic_error) -> *mut print_get_printer_list_reply;

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
pub fn xcb_x_print_print_rehash_printer_list_checked (c : *mut ffi::base::connection) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_x_print_print_rehash_printer_list (c : *mut ffi::base::connection) -> ffi::base::void_cookie;

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
pub fn xcb_x_print_create_context_checked (c : *mut ffi::base::connection,
                                              context_id :  u32,
                                              printerNameLen :  u32,
                                              localeLen :  u32,
                                              printerName : *mut string8,
                                              locale : *mut string8) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_x_print_create_context (c : *mut ffi::base::connection,
                                      context_id :  u32,
                                      printerNameLen :  u32,
                                      localeLen :  u32,
                                      printerName : *mut string8,
                                      locale : *mut string8) -> ffi::base::void_cookie;

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
pub fn xcb_x_print_print_set_context_checked (c : *mut ffi::base::connection,
                                                 context :  u32) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_x_print_print_set_context (c : *mut ffi::base::connection,
                                         context :  u32) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_x_print_print_get_context (c : *mut ffi::base::connection) -> print_get_context_cookie;

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
pub fn xcb_x_print_print_get_context_unchecked (c : *mut ffi::base::connection) -> print_get_context_cookie;

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
pub fn xcb_x_print_print_get_context_reply (c : *mut ffi::base::connection,
                                               cookie : print_get_context_cookie,
                                               e : *mut *mut ffi::base::generic_error) -> *mut print_get_context_reply;

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
pub fn xcb_x_print_print_destroy_context_checked (c : *mut ffi::base::connection,
                                                     context :  u32) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_x_print_print_destroy_context (c : *mut ffi::base::connection,
                                             context :  u32) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_x_print_print_get_screen_of_context (c : *mut ffi::base::connection) -> print_get_screen_of_context_cookie;

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
pub fn xcb_x_print_print_get_screen_of_context_unchecked (c : *mut ffi::base::connection) -> print_get_screen_of_context_cookie;

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
pub fn xcb_x_print_print_get_screen_of_context_reply (c : *mut ffi::base::connection,
                                                         cookie : print_get_screen_of_context_cookie,
                                                         e : *mut *mut ffi::base::generic_error) -> *mut print_get_screen_of_context_reply;

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
pub fn xcb_x_print_print_start_job_checked (c : *mut ffi::base::connection,
                                               output_mode :  u8) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_x_print_print_start_job (c : *mut ffi::base::connection,
                                       output_mode :  u8) -> ffi::base::void_cookie;

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
pub fn xcb_x_print_print_end_job_checked (c : *mut ffi::base::connection,
                                             cancel :  u8) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_x_print_print_end_job (c : *mut ffi::base::connection,
                                     cancel :  u8) -> ffi::base::void_cookie;

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
pub fn xcb_x_print_print_start_doc_checked (c : *mut ffi::base::connection,
                                               driver_mode :  u8) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_x_print_print_start_doc (c : *mut ffi::base::connection,
                                       driver_mode :  u8) -> ffi::base::void_cookie;

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
pub fn xcb_x_print_print_end_doc_checked (c : *mut ffi::base::connection,
                                             cancel :  u8) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_x_print_print_end_doc (c : *mut ffi::base::connection,
                                     cancel :  u8) -> ffi::base::void_cookie;

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
pub fn xcb_x_print_print_put_document_data_checked (c : *mut ffi::base::connection,
                                                       drawable :  ffi::xproto::drawable,
                                                       len_data :  u32,
                                                       len_fmt :  u16,
                                                       len_options :  u16,
                                                       data : *mut u8,
                                                       doc_format_len :  u32,
                                                       doc_format : *mut string8,
                                                       options_len :  u32,
                                                       options : *mut string8) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_x_print_print_put_document_data (c : *mut ffi::base::connection,
                                               drawable :  ffi::xproto::drawable,
                                               len_data :  u32,
                                               len_fmt :  u16,
                                               len_options :  u16,
                                               data : *mut u8,
                                               doc_format_len :  u32,
                                               doc_format : *mut string8,
                                               options_len :  u32,
                                               options : *mut string8) -> ffi::base::void_cookie;

pub fn xcb_x_print_print_get_document_data_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_x_print_print_get_document_data (c : *mut ffi::base::connection,
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
pub fn xcb_x_print_print_get_document_data_unchecked (c : *mut ffi::base::connection,
                                                         context :  pcontext,
                                                         max_bytes :  u32) -> print_get_document_data_cookie;

pub fn xcb_x_print_print_get_document_data_data (R : *mut print_get_document_data_reply) -> *mut u8;


pub fn xcb_x_print_print_get_document_data_data_length (R : *mut print_get_document_data_reply) -> c_int;


pub fn xcb_x_print_print_get_document_data_data_end (R : *mut print_get_document_data_reply) -> ffi::base::generic_iterator;

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
pub fn xcb_x_print_print_get_document_data_reply (c : *mut ffi::base::connection,
                                                     cookie : print_get_document_data_cookie,
                                                     e : *mut *mut ffi::base::generic_error) -> *mut print_get_document_data_reply;

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
pub fn xcb_x_print_print_start_page_checked (c : *mut ffi::base::connection,
                                                window :  ffi::xproto::window) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_x_print_print_start_page (c : *mut ffi::base::connection,
                                        window :  ffi::xproto::window) -> ffi::base::void_cookie;

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
pub fn xcb_x_print_print_end_page_checked (c : *mut ffi::base::connection,
                                              cancel :  u8) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_x_print_print_end_page (c : *mut ffi::base::connection,
                                      cancel :  u8) -> ffi::base::void_cookie;

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
pub fn xcb_x_print_print_select_input_checked (c : *mut ffi::base::connection,
                                                  context :  pcontext,
                                                  event_mask :  u32,
                                                  event_list : *mut u32) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_x_print_print_select_input (c : *mut ffi::base::connection,
                                          context :  pcontext,
                                          event_mask :  u32,
                                          event_list : *mut u32) -> ffi::base::void_cookie;

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
pub fn xcb_x_print_print_input_selected (c : *mut ffi::base::connection,
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
pub fn xcb_x_print_print_input_selected_unchecked (c : *mut ffi::base::connection,
                                                      context :  pcontext) -> print_input_selected_cookie;

pub fn xcb_x_print_print_input_selected_event_list (R : *mut print_input_selected_reply) -> *mut u32;


pub fn xcb_x_print_print_input_selected_event_list_length (R : *mut print_input_selected_reply) -> c_int;


pub fn xcb_x_print_print_input_selected_event_list_end (R : *mut print_input_selected_reply) -> ffi::base::generic_iterator;

pub fn xcb_x_print_print_input_selected_all_events_list (R : *mut print_input_selected_reply) -> *mut u32;


pub fn xcb_x_print_print_input_selected_all_events_list_length (R : *mut print_input_selected_reply) -> c_int;


pub fn xcb_x_print_print_input_selected_all_events_list_end (R : *mut print_input_selected_reply) -> ffi::base::generic_iterator;

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
pub fn xcb_x_print_print_input_selected_reply (c : *mut ffi::base::connection,
                                                  cookie : print_input_selected_cookie,
                                                  e : *mut *mut ffi::base::generic_error) -> *mut print_input_selected_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_x_print_print_get_attributes (c : *mut ffi::base::connection,
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
pub fn xcb_x_print_print_get_attributes_unchecked (c : *mut ffi::base::connection,
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
pub fn xcb_x_print_print_get_attributes_reply (c : *mut ffi::base::connection,
                                                  cookie : print_get_attributes_cookie,
                                                  e : *mut *mut ffi::base::generic_error) -> *mut print_get_attributes_reply;

pub fn xcb_x_print_print_get_one_attributes_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_x_print_print_get_one_attributes (c : *mut ffi::base::connection,
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
pub fn xcb_x_print_print_get_one_attributes_unchecked (c : *mut ffi::base::connection,
                                                          context :  pcontext,
                                                          nameLen :  u32,
                                                          pool :  u8,
                                                          name : *mut string8) -> print_get_one_attributes_cookie;

pub fn xcb_x_print_print_get_one_attributes_value (R : *mut print_get_one_attributes_reply) -> *mut string8;


pub fn xcb_x_print_print_get_one_attributes_value_length (R : *mut print_get_one_attributes_reply) -> c_int;


pub fn xcb_x_print_print_get_one_attributes_value_end (R : *mut print_get_one_attributes_reply) -> ffi::base::generic_iterator;

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
pub fn xcb_x_print_print_get_one_attributes_reply (c : *mut ffi::base::connection,
                                                      cookie : print_get_one_attributes_cookie,
                                                      e : *mut *mut ffi::base::generic_error) -> *mut print_get_one_attributes_reply;

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
pub fn xcb_x_print_print_set_attributes_checked (c : *mut ffi::base::connection,
                                                    context :  pcontext,
                                                    stringLen :  u32,
                                                    pool :  u8,
                                                    rule :  u8,
                                                    attributes_len :  u32,
                                                    attributes : *mut string8) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_x_print_print_set_attributes (c : *mut ffi::base::connection,
                                            context :  pcontext,
                                            stringLen :  u32,
                                            pool :  u8,
                                            rule :  u8,
                                            attributes_len :  u32,
                                            attributes : *mut string8) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_x_print_print_get_page_dimensions (c : *mut ffi::base::connection,
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
pub fn xcb_x_print_print_get_page_dimensions_unchecked (c : *mut ffi::base::connection,
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
pub fn xcb_x_print_print_get_page_dimensions_reply (c : *mut ffi::base::connection,
                                                       cookie : print_get_page_dimensions_cookie,
                                                       e : *mut *mut ffi::base::generic_error) -> *mut print_get_page_dimensions_reply;

pub fn xcb_x_print_print_query_screens_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_x_print_print_query_screens (c : *mut ffi::base::connection) -> print_query_screens_cookie;

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
pub fn xcb_x_print_print_query_screens_unchecked (c : *mut ffi::base::connection) -> print_query_screens_cookie;

pub fn xcb_x_print_print_query_screens_roots (R : *mut print_query_screens_reply) -> *mut ffi::xproto::window;


pub fn xcb_x_print_print_query_screens_roots_length (R : *mut print_query_screens_reply) -> c_int;


pub fn xcb_x_print_print_query_screens_roots_end (R : *mut print_query_screens_reply) -> ffi::base::generic_iterator;

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
pub fn xcb_x_print_print_query_screens_reply (c : *mut ffi::base::connection,
                                                 cookie : print_query_screens_cookie,
                                                 e : *mut *mut ffi::base::generic_error) -> *mut print_query_screens_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_x_print_print_set_image_resolution (c : *mut ffi::base::connection,
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
pub fn xcb_x_print_print_set_image_resolution_unchecked (c : *mut ffi::base::connection,
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
pub fn xcb_x_print_print_set_image_resolution_reply (c : *mut ffi::base::connection,
                                                        cookie : print_set_image_resolution_cookie,
                                                        e : *mut *mut ffi::base::generic_error) -> *mut print_set_image_resolution_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_x_print_print_get_image_resolution (c : *mut ffi::base::connection,
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
pub fn xcb_x_print_print_get_image_resolution_unchecked (c : *mut ffi::base::connection,
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
pub fn xcb_x_print_print_get_image_resolution_reply (c : *mut ffi::base::connection,
                                                        cookie : print_get_image_resolution_cookie,
                                                        e : *mut *mut ffi::base::generic_error) -> *mut print_get_image_resolution_reply;
}

