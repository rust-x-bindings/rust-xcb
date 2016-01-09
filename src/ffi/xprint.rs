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

pub type xcb_x_print_string8_t = c_char;
/**
 * @brief xcb_x_print_string8_iterator_t
 **/
#[repr(C)]
pub struct xcb_x_print_string8_iterator_t {
    pub data : *mut xcb_x_print_string8_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_x_print_printer_t {
     pub nameLen :   u32,
     pub descLen :   u32
}

impl Copy for xcb_x_print_printer_t {}
impl Clone for xcb_x_print_printer_t {
    fn clone(&self) -> xcb_x_print_printer_t { *self }
}
/**
 * @brief xcb_x_print_printer_iterator_t
 **/
#[repr(C)]
pub struct xcb_x_print_printer_iterator_t {
    pub data : *mut xcb_x_print_printer_t,
    pub rem  : c_int,
    pub index: c_int
}


pub type xcb_x_print_pcontext_t = u32;
/**
 * @brief xcb_x_print_pcontext_iterator_t
 **/
#[repr(C)]
pub struct xcb_x_print_pcontext_iterator_t {
    pub data : *mut xcb_x_print_pcontext_t,
    pub rem  : c_int,
    pub index: c_int
}


#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_x_print_print_query_version_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_x_print_print_query_version_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16
}

impl Copy for xcb_x_print_print_query_version_request_t {}
impl Clone for xcb_x_print_print_query_version_request_t {
    fn clone(&self) -> xcb_x_print_print_query_version_request_t { *self }
}

#[repr(C)]
pub struct xcb_x_print_print_query_version_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub major_version :   u16,
     pub minor_version :   u16
}

impl Copy for xcb_x_print_print_query_version_reply_t {}
impl Clone for xcb_x_print_print_query_version_reply_t {
    fn clone(&self) -> xcb_x_print_print_query_version_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_x_print_print_get_printer_list_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_x_print_print_get_printer_list_request_t {
     pub major_opcode :     u8,
     pub minor_opcode :     u8,
     pub length :           u16,
     pub printerNameLen :   u32,
     pub localeLen :        u32
}

impl Copy for xcb_x_print_print_get_printer_list_request_t {}
impl Clone for xcb_x_print_print_get_printer_list_request_t {
    fn clone(&self) -> xcb_x_print_print_get_printer_list_request_t { *self }
}

#[repr(C)]
pub struct xcb_x_print_print_get_printer_list_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub listCount :       u32,
     pub pad1 :            [u8; 20]
}

impl Copy for xcb_x_print_print_get_printer_list_reply_t {}
impl Clone for xcb_x_print_print_get_printer_list_reply_t {
    fn clone(&self) -> xcb_x_print_print_get_printer_list_reply_t { *self }
}


#[repr(C)]
pub struct xcb_x_print_print_rehash_printer_list_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16
}

impl Copy for xcb_x_print_print_rehash_printer_list_request_t {}
impl Clone for xcb_x_print_print_rehash_printer_list_request_t {
    fn clone(&self) -> xcb_x_print_print_rehash_printer_list_request_t { *self }
}


#[repr(C)]
pub struct xcb_x_print_create_context_request_t {
     pub major_opcode :     u8,
     pub minor_opcode :     u8,
     pub length :           u16,
     pub context_id :       u32,
     pub printerNameLen :   u32,
     pub localeLen :        u32
}

impl Copy for xcb_x_print_create_context_request_t {}
impl Clone for xcb_x_print_create_context_request_t {
    fn clone(&self) -> xcb_x_print_create_context_request_t { *self }
}


#[repr(C)]
pub struct xcb_x_print_print_set_context_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub context :        u32
}

impl Copy for xcb_x_print_print_set_context_request_t {}
impl Clone for xcb_x_print_print_set_context_request_t {
    fn clone(&self) -> xcb_x_print_print_set_context_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_x_print_print_get_context_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_x_print_print_get_context_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16
}

impl Copy for xcb_x_print_print_get_context_request_t {}
impl Clone for xcb_x_print_print_get_context_request_t {
    fn clone(&self) -> xcb_x_print_print_get_context_request_t { *self }
}

#[repr(C)]
pub struct xcb_x_print_print_get_context_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub context :         u32
}

impl Copy for xcb_x_print_print_get_context_reply_t {}
impl Clone for xcb_x_print_print_get_context_reply_t {
    fn clone(&self) -> xcb_x_print_print_get_context_reply_t { *self }
}


#[repr(C)]
pub struct xcb_x_print_print_destroy_context_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub context :        u32
}

impl Copy for xcb_x_print_print_destroy_context_request_t {}
impl Clone for xcb_x_print_print_destroy_context_request_t {
    fn clone(&self) -> xcb_x_print_print_destroy_context_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_x_print_print_get_screen_of_context_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_x_print_print_get_screen_of_context_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16
}

impl Copy for xcb_x_print_print_get_screen_of_context_request_t {}
impl Clone for xcb_x_print_print_get_screen_of_context_request_t {
    fn clone(&self) -> xcb_x_print_print_get_screen_of_context_request_t { *self }
}

#[repr(C)]
pub struct xcb_x_print_print_get_screen_of_context_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub root :            ffi::xproto::xcb_window_t
}

impl Copy for xcb_x_print_print_get_screen_of_context_reply_t {}
impl Clone for xcb_x_print_print_get_screen_of_context_reply_t {
    fn clone(&self) -> xcb_x_print_print_get_screen_of_context_reply_t { *self }
}


#[repr(C)]
pub struct xcb_x_print_print_start_job_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub output_mode :    u8
}

impl Copy for xcb_x_print_print_start_job_request_t {}
impl Clone for xcb_x_print_print_start_job_request_t {
    fn clone(&self) -> xcb_x_print_print_start_job_request_t { *self }
}


#[repr(C)]
pub struct xcb_x_print_print_end_job_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub cancel :         u8
}

impl Copy for xcb_x_print_print_end_job_request_t {}
impl Clone for xcb_x_print_print_end_job_request_t {
    fn clone(&self) -> xcb_x_print_print_end_job_request_t { *self }
}


#[repr(C)]
pub struct xcb_x_print_print_start_doc_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub driver_mode :    u8
}

impl Copy for xcb_x_print_print_start_doc_request_t {}
impl Clone for xcb_x_print_print_start_doc_request_t {
    fn clone(&self) -> xcb_x_print_print_start_doc_request_t { *self }
}


#[repr(C)]
pub struct xcb_x_print_print_end_doc_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub cancel :         u8
}

impl Copy for xcb_x_print_print_end_doc_request_t {}
impl Clone for xcb_x_print_print_end_doc_request_t {
    fn clone(&self) -> xcb_x_print_print_end_doc_request_t { *self }
}


#[repr(C)]
pub struct xcb_x_print_print_put_document_data_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub drawable :       ffi::xproto::xcb_drawable_t,
     pub len_data :       u32,
     pub len_fmt :        u16,
     pub len_options :    u16
}

impl Copy for xcb_x_print_print_put_document_data_request_t {}
impl Clone for xcb_x_print_print_put_document_data_request_t {
    fn clone(&self) -> xcb_x_print_print_put_document_data_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_x_print_print_get_document_data_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_x_print_print_get_document_data_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub context :        xcb_x_print_pcontext_t,
     pub max_bytes :      u32
}

impl Copy for xcb_x_print_print_get_document_data_request_t {}
impl Clone for xcb_x_print_print_get_document_data_request_t {
    fn clone(&self) -> xcb_x_print_print_get_document_data_request_t { *self }
}

#[repr(C)]
pub struct xcb_x_print_print_get_document_data_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub status_code :     u32,
     pub finished_flag :   u32,
     pub dataLen :         u32,
     pub pad1 :            [u8; 12]
}

impl Copy for xcb_x_print_print_get_document_data_reply_t {}
impl Clone for xcb_x_print_print_get_document_data_reply_t {
    fn clone(&self) -> xcb_x_print_print_get_document_data_reply_t { *self }
}


#[repr(C)]
pub struct xcb_x_print_print_start_page_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub window :         ffi::xproto::xcb_window_t
}

impl Copy for xcb_x_print_print_start_page_request_t {}
impl Clone for xcb_x_print_print_start_page_request_t {
    fn clone(&self) -> xcb_x_print_print_start_page_request_t { *self }
}


#[repr(C)]
pub struct xcb_x_print_print_end_page_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub cancel :         u8,
     pub pad0 :           [u8; 3]
}

impl Copy for xcb_x_print_print_end_page_request_t {}
impl Clone for xcb_x_print_print_end_page_request_t {
    fn clone(&self) -> xcb_x_print_print_end_page_request_t { *self }
}


#[repr(C)]
pub struct xcb_x_print_print_select_input_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub context :        xcb_x_print_pcontext_t,
     pub event_mask :     u32
}

impl Copy for xcb_x_print_print_select_input_request_t {}
impl Clone for xcb_x_print_print_select_input_request_t {
    fn clone(&self) -> xcb_x_print_print_select_input_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_x_print_print_input_selected_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_x_print_print_input_selected_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub context :        xcb_x_print_pcontext_t
}

impl Copy for xcb_x_print_print_input_selected_request_t {}
impl Clone for xcb_x_print_print_input_selected_request_t {
    fn clone(&self) -> xcb_x_print_print_input_selected_request_t { *self }
}

#[repr(C)]
pub struct xcb_x_print_print_input_selected_reply_t {
     pub response_type :     u8,
     pub pad0 :              u8,
     pub sequence :          u16,
     pub length :            u32,
     pub event_mask :        u32,
     pub all_events_mask :   u32
}

impl Copy for xcb_x_print_print_input_selected_reply_t {}
impl Clone for xcb_x_print_print_input_selected_reply_t {
    fn clone(&self) -> xcb_x_print_print_input_selected_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_x_print_print_get_attributes_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_x_print_print_get_attributes_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub context :        xcb_x_print_pcontext_t,
     pub pool :           u8,
     pub pad0 :           [u8; 3]
}

impl Copy for xcb_x_print_print_get_attributes_request_t {}
impl Clone for xcb_x_print_print_get_attributes_request_t {
    fn clone(&self) -> xcb_x_print_print_get_attributes_request_t { *self }
}

#[repr(C)]
pub struct xcb_x_print_print_get_attributes_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub stringLen :       u32,
     pub pad1 :            [u8; 20],
     pub attributes :      xcb_x_print_string8_t
}

impl Copy for xcb_x_print_print_get_attributes_reply_t {}
impl Clone for xcb_x_print_print_get_attributes_reply_t {
    fn clone(&self) -> xcb_x_print_print_get_attributes_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_x_print_print_get_one_attributes_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_x_print_print_get_one_attributes_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub context :        xcb_x_print_pcontext_t,
     pub nameLen :        u32,
     pub pool :           u8,
     pub pad0 :           [u8; 3]
}

impl Copy for xcb_x_print_print_get_one_attributes_request_t {}
impl Clone for xcb_x_print_print_get_one_attributes_request_t {
    fn clone(&self) -> xcb_x_print_print_get_one_attributes_request_t { *self }
}

#[repr(C)]
pub struct xcb_x_print_print_get_one_attributes_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub valueLen :        u32,
     pub pad1 :            [u8; 20]
}

impl Copy for xcb_x_print_print_get_one_attributes_reply_t {}
impl Clone for xcb_x_print_print_get_one_attributes_reply_t {
    fn clone(&self) -> xcb_x_print_print_get_one_attributes_reply_t { *self }
}


#[repr(C)]
pub struct xcb_x_print_print_set_attributes_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub context :        xcb_x_print_pcontext_t,
     pub stringLen :      u32,
     pub pool :           u8,
     pub rule :           u8,
     pub pad0 :           [u8; 2]
}

impl Copy for xcb_x_print_print_set_attributes_request_t {}
impl Clone for xcb_x_print_print_set_attributes_request_t {
    fn clone(&self) -> xcb_x_print_print_set_attributes_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_x_print_print_get_page_dimensions_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_x_print_print_get_page_dimensions_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub context :        xcb_x_print_pcontext_t
}

impl Copy for xcb_x_print_print_get_page_dimensions_request_t {}
impl Clone for xcb_x_print_print_get_page_dimensions_request_t {
    fn clone(&self) -> xcb_x_print_print_get_page_dimensions_request_t { *self }
}

#[repr(C)]
pub struct xcb_x_print_print_get_page_dimensions_reply_t {
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

impl Copy for xcb_x_print_print_get_page_dimensions_reply_t {}
impl Clone for xcb_x_print_print_get_page_dimensions_reply_t {
    fn clone(&self) -> xcb_x_print_print_get_page_dimensions_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_x_print_print_query_screens_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_x_print_print_query_screens_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16
}

impl Copy for xcb_x_print_print_query_screens_request_t {}
impl Clone for xcb_x_print_print_query_screens_request_t {
    fn clone(&self) -> xcb_x_print_print_query_screens_request_t { *self }
}

#[repr(C)]
pub struct xcb_x_print_print_query_screens_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub listCount :       u32,
     pub pad1 :            [u8; 20]
}

impl Copy for xcb_x_print_print_query_screens_reply_t {}
impl Clone for xcb_x_print_print_query_screens_reply_t {
    fn clone(&self) -> xcb_x_print_print_query_screens_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_x_print_print_set_image_resolution_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_x_print_print_set_image_resolution_request_t {
     pub major_opcode :       u8,
     pub minor_opcode :       u8,
     pub length :             u16,
     pub context :            xcb_x_print_pcontext_t,
     pub image_resolution :   u16
}

impl Copy for xcb_x_print_print_set_image_resolution_request_t {}
impl Clone for xcb_x_print_print_set_image_resolution_request_t {
    fn clone(&self) -> xcb_x_print_print_set_image_resolution_request_t { *self }
}

#[repr(C)]
pub struct xcb_x_print_print_set_image_resolution_reply_t {
     pub response_type :          u8,
     pub status :                 u8,
     pub sequence :               u16,
     pub length :                 u32,
     pub previous_resolutions :   u16
}

impl Copy for xcb_x_print_print_set_image_resolution_reply_t {}
impl Clone for xcb_x_print_print_set_image_resolution_reply_t {
    fn clone(&self) -> xcb_x_print_print_set_image_resolution_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_x_print_print_get_image_resolution_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_x_print_print_get_image_resolution_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub context :        xcb_x_print_pcontext_t
}

impl Copy for xcb_x_print_print_get_image_resolution_request_t {}
impl Clone for xcb_x_print_print_get_image_resolution_request_t {
    fn clone(&self) -> xcb_x_print_print_get_image_resolution_request_t { *self }
}

#[repr(C)]
pub struct xcb_x_print_print_get_image_resolution_reply_t {
     pub response_type :      u8,
     pub pad0 :               u8,
     pub sequence :           u16,
     pub length :             u32,
     pub image_resolution :   u16
}

impl Copy for xcb_x_print_print_get_image_resolution_reply_t {}
impl Clone for xcb_x_print_print_get_image_resolution_reply_t {
    fn clone(&self) -> xcb_x_print_print_get_image_resolution_reply_t { *self }
}


#[repr(C)]
pub struct xcb_x_print_notify_event_t {
     pub response_type :   u8,
     pub detail :          u8,
     pub sequence :        u16,
     pub context :         xcb_x_print_pcontext_t,
     pub cancel :          u8
}

impl Copy for xcb_x_print_notify_event_t {}
impl Clone for xcb_x_print_notify_event_t {
    fn clone(&self) -> xcb_x_print_notify_event_t { *self }
}


#[repr(C)]
pub struct xcb_x_print_attribut_notify_event_t {
     pub response_type :   u8,
     pub detail :          u8,
     pub sequence :        u16,
     pub context :         xcb_x_print_pcontext_t
}

impl Copy for xcb_x_print_attribut_notify_event_t {}
impl Clone for xcb_x_print_attribut_notify_event_t {
    fn clone(&self) -> xcb_x_print_attribut_notify_event_t { *self }
}


#[repr(C)]
pub struct xcb_x_print_bad_context_error_t {
     pub response_type :   u8,
     pub error_code :      u8,
     pub sequence :        u16
}

impl Copy for xcb_x_print_bad_context_error_t {}
impl Clone for xcb_x_print_bad_context_error_t {
    fn clone(&self) -> xcb_x_print_bad_context_error_t { *self }
}


#[repr(C)]
pub struct xcb_x_print_bad_sequence_error_t {
     pub response_type :   u8,
     pub error_code :      u8,
     pub sequence :        u16
}

impl Copy for xcb_x_print_bad_sequence_error_t {}
impl Clone for xcb_x_print_bad_sequence_error_t {
    fn clone(&self) -> xcb_x_print_bad_sequence_error_t { *self }
}
#[link(name="xcb-xprint")]
extern "C" {

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_x_print_string8_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_x_print_string8_t)
 *
 *
 */
pub fn xcb_x_print_string8_next (i:*mut xcb_x_print_string8_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_x_print_string8_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_x_print_string8_end (i:xcb_x_print_string8_iterator_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_x_print_printer_serialize (_buffer :           *mut *mut c_void,
                               _aux :                   *mut xcb_x_print_printer_t,
                               name :                   *mut xcb_x_print_string8_t,
                               description :            *mut xcb_x_print_string8_t) -> c_int;

pub fn xcb_x_print_printer_unserialize (_buffer :                     *mut c_void,
                                 _aux :                   *mut *mut xcb_x_print_printer_t) -> c_int;

pub fn xcb_x_print_printer_sizeof (_buffer :  *mut c_void) -> c_int;

pub fn xcb_x_print_printer_name (R : *mut xcb_x_print_printer_t) -> *mut xcb_x_print_string8_t;


pub fn xcb_x_print_printer_name_length (R : *mut xcb_x_print_printer_t) -> c_int;


pub fn xcb_x_print_printer_name_end (R : *mut xcb_x_print_printer_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_x_print_printer_description (R : *mut xcb_x_print_printer_t) -> *mut xcb_x_print_string8_t;


pub fn xcb_x_print_printer_description_length (R : *mut xcb_x_print_printer_t) -> c_int;


pub fn xcb_x_print_printer_description_end (R : *mut xcb_x_print_printer_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_x_print_printer_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_x_print_printer_t)
 *
 *
 */
pub fn xcb_x_print_printer_next (i:*mut xcb_x_print_printer_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_x_print_printer_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_x_print_printer_end (i:xcb_x_print_printer_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_x_print_pcontext_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_x_print_pcontext_t)
 *
 *
 */
pub fn xcb_x_print_pcontext_next (i:*mut xcb_x_print_pcontext_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_x_print_pcontext_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_x_print_pcontext_end (i:xcb_x_print_pcontext_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_x_print_print_query_version (c : *mut ffi::base::xcb_connection_t) -> xcb_x_print_print_query_version_cookie_t;

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
pub fn xcb_x_print_print_query_version_unchecked (c : *mut ffi::base::xcb_connection_t) -> xcb_x_print_print_query_version_cookie_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_x_print_print_query_version_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_x_print_print_query_version_reply (c : *mut ffi::base::xcb_connection_t,
                                                 cookie : xcb_x_print_print_query_version_cookie_t,
                                                 e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_x_print_print_query_version_reply_t;

pub fn xcb_x_print_print_get_printer_list_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_x_print_print_get_printer_list (c : *mut ffi::base::xcb_connection_t,
                                              printerNameLen :  u32,
                                              localeLen :  u32,
                                              printer_name : *mut xcb_x_print_string8_t,
                                              locale : *mut xcb_x_print_string8_t) -> xcb_x_print_print_get_printer_list_cookie_t;

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
pub fn xcb_x_print_print_get_printer_list_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                        printerNameLen :  u32,
                                                        localeLen :  u32,
                                                        printer_name : *mut xcb_x_print_string8_t,
                                                        locale : *mut xcb_x_print_string8_t) -> xcb_x_print_print_get_printer_list_cookie_t;


pub fn xcb_x_print_print_get_printer_list_printers_length (R : *mut xcb_x_print_print_get_printer_list_reply_t) -> c_int;

pub fn xcb_x_print_print_get_printer_list_printers_iterator (R : *mut xcb_x_print_print_get_printer_list_reply_t) -> xcb_x_print_printer_iterator_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_x_print_print_get_printer_list_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_x_print_print_get_printer_list_reply (c : *mut ffi::base::xcb_connection_t,
                                                    cookie : xcb_x_print_print_get_printer_list_cookie_t,
                                                    e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_x_print_print_get_printer_list_reply_t;

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
pub fn xcb_x_print_print_rehash_printer_list_checked (c : *mut ffi::base::xcb_connection_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_x_print_print_rehash_printer_list (c : *mut ffi::base::xcb_connection_t) -> ffi::base::xcb_void_cookie_t;

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
pub fn xcb_x_print_create_context_checked (c : *mut ffi::base::xcb_connection_t,
                                              context_id :  u32,
                                              printerNameLen :  u32,
                                              localeLen :  u32,
                                              printerName : *mut xcb_x_print_string8_t,
                                              locale : *mut xcb_x_print_string8_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_x_print_create_context (c : *mut ffi::base::xcb_connection_t,
                                      context_id :  u32,
                                      printerNameLen :  u32,
                                      localeLen :  u32,
                                      printerName : *mut xcb_x_print_string8_t,
                                      locale : *mut xcb_x_print_string8_t) -> ffi::base::xcb_void_cookie_t;

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
pub fn xcb_x_print_print_set_context_checked (c : *mut ffi::base::xcb_connection_t,
                                                 context :  u32) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_x_print_print_set_context (c : *mut ffi::base::xcb_connection_t,
                                         context :  u32) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_x_print_print_get_context (c : *mut ffi::base::xcb_connection_t) -> xcb_x_print_print_get_context_cookie_t;

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
pub fn xcb_x_print_print_get_context_unchecked (c : *mut ffi::base::xcb_connection_t) -> xcb_x_print_print_get_context_cookie_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_x_print_print_get_context_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_x_print_print_get_context_reply (c : *mut ffi::base::xcb_connection_t,
                                               cookie : xcb_x_print_print_get_context_cookie_t,
                                               e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_x_print_print_get_context_reply_t;

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
pub fn xcb_x_print_print_destroy_context_checked (c : *mut ffi::base::xcb_connection_t,
                                                     context :  u32) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_x_print_print_destroy_context (c : *mut ffi::base::xcb_connection_t,
                                             context :  u32) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_x_print_print_get_screen_of_context (c : *mut ffi::base::xcb_connection_t) -> xcb_x_print_print_get_screen_of_context_cookie_t;

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
pub fn xcb_x_print_print_get_screen_of_context_unchecked (c : *mut ffi::base::xcb_connection_t) -> xcb_x_print_print_get_screen_of_context_cookie_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_x_print_print_get_screen_of_context_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_x_print_print_get_screen_of_context_reply (c : *mut ffi::base::xcb_connection_t,
                                                         cookie : xcb_x_print_print_get_screen_of_context_cookie_t,
                                                         e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_x_print_print_get_screen_of_context_reply_t;

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
pub fn xcb_x_print_print_start_job_checked (c : *mut ffi::base::xcb_connection_t,
                                               output_mode :  u8) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_x_print_print_start_job (c : *mut ffi::base::xcb_connection_t,
                                       output_mode :  u8) -> ffi::base::xcb_void_cookie_t;

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
pub fn xcb_x_print_print_end_job_checked (c : *mut ffi::base::xcb_connection_t,
                                             cancel :  u8) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_x_print_print_end_job (c : *mut ffi::base::xcb_connection_t,
                                     cancel :  u8) -> ffi::base::xcb_void_cookie_t;

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
pub fn xcb_x_print_print_start_doc_checked (c : *mut ffi::base::xcb_connection_t,
                                               driver_mode :  u8) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_x_print_print_start_doc (c : *mut ffi::base::xcb_connection_t,
                                       driver_mode :  u8) -> ffi::base::xcb_void_cookie_t;

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
pub fn xcb_x_print_print_end_doc_checked (c : *mut ffi::base::xcb_connection_t,
                                             cancel :  u8) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_x_print_print_end_doc (c : *mut ffi::base::xcb_connection_t,
                                     cancel :  u8) -> ffi::base::xcb_void_cookie_t;

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
pub fn xcb_x_print_print_put_document_data_checked (c : *mut ffi::base::xcb_connection_t,
                                                       drawable :  ffi::xproto::xcb_drawable_t,
                                                       len_data :  u32,
                                                       len_fmt :  u16,
                                                       len_options :  u16,
                                                       data : *mut u8,
                                                       doc_format_len :  u32,
                                                       doc_format : *mut xcb_x_print_string8_t,
                                                       options_len :  u32,
                                                       options : *mut xcb_x_print_string8_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_x_print_print_put_document_data (c : *mut ffi::base::xcb_connection_t,
                                               drawable :  ffi::xproto::xcb_drawable_t,
                                               len_data :  u32,
                                               len_fmt :  u16,
                                               len_options :  u16,
                                               data : *mut u8,
                                               doc_format_len :  u32,
                                               doc_format : *mut xcb_x_print_string8_t,
                                               options_len :  u32,
                                               options : *mut xcb_x_print_string8_t) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_x_print_print_get_document_data_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_x_print_print_get_document_data (c : *mut ffi::base::xcb_connection_t,
                                               context :  xcb_x_print_pcontext_t,
                                               max_bytes :  u32) -> xcb_x_print_print_get_document_data_cookie_t;

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
pub fn xcb_x_print_print_get_document_data_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                         context :  xcb_x_print_pcontext_t,
                                                         max_bytes :  u32) -> xcb_x_print_print_get_document_data_cookie_t;

pub fn xcb_x_print_print_get_document_data_data (R : *mut xcb_x_print_print_get_document_data_reply_t) -> *mut u8;


pub fn xcb_x_print_print_get_document_data_data_length (R : *mut xcb_x_print_print_get_document_data_reply_t) -> c_int;


pub fn xcb_x_print_print_get_document_data_data_end (R : *mut xcb_x_print_print_get_document_data_reply_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_x_print_print_get_document_data_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_x_print_print_get_document_data_reply (c : *mut ffi::base::xcb_connection_t,
                                                     cookie : xcb_x_print_print_get_document_data_cookie_t,
                                                     e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_x_print_print_get_document_data_reply_t;

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
pub fn xcb_x_print_print_start_page_checked (c : *mut ffi::base::xcb_connection_t,
                                                window :  ffi::xproto::xcb_window_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_x_print_print_start_page (c : *mut ffi::base::xcb_connection_t,
                                        window :  ffi::xproto::xcb_window_t) -> ffi::base::xcb_void_cookie_t;

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
pub fn xcb_x_print_print_end_page_checked (c : *mut ffi::base::xcb_connection_t,
                                              cancel :  u8) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_x_print_print_end_page (c : *mut ffi::base::xcb_connection_t,
                                      cancel :  u8) -> ffi::base::xcb_void_cookie_t;

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
pub fn xcb_x_print_print_select_input_checked (c : *mut ffi::base::xcb_connection_t,
                                                  context :  xcb_x_print_pcontext_t,
                                                  event_mask :  u32,
                                                  event_list : *mut u32) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_x_print_print_select_input (c : *mut ffi::base::xcb_connection_t,
                                          context :  xcb_x_print_pcontext_t,
                                          event_mask :  u32,
                                          event_list : *mut u32) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_x_print_print_input_selected_serialize (_buffer :                              *mut *mut c_void,
                                            _aux :                                      *mut xcb_x_print_print_input_selected_reply_t,
                                            event_list :                                *mut u32,
                                            all_events_list :                           *mut u32) -> c_int;

pub fn xcb_x_print_print_input_selected_unserialize (_buffer :                                        *mut c_void,
                                              _aux :                                      *mut *mut xcb_x_print_print_input_selected_reply_t) -> c_int;

pub fn xcb_x_print_print_input_selected_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_x_print_print_input_selected (c : *mut ffi::base::xcb_connection_t,
                                            context :  xcb_x_print_pcontext_t) -> xcb_x_print_print_input_selected_cookie_t;

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
pub fn xcb_x_print_print_input_selected_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                      context :  xcb_x_print_pcontext_t) -> xcb_x_print_print_input_selected_cookie_t;

pub fn xcb_x_print_print_input_selected_event_list (R : *mut xcb_x_print_print_input_selected_reply_t) -> *mut u32;


pub fn xcb_x_print_print_input_selected_event_list_length (R : *mut xcb_x_print_print_input_selected_reply_t) -> c_int;


pub fn xcb_x_print_print_input_selected_event_list_end (R : *mut xcb_x_print_print_input_selected_reply_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_x_print_print_input_selected_all_events_list (R : *mut xcb_x_print_print_input_selected_reply_t) -> *mut u32;


pub fn xcb_x_print_print_input_selected_all_events_list_length (R : *mut xcb_x_print_print_input_selected_reply_t) -> c_int;


pub fn xcb_x_print_print_input_selected_all_events_list_end (R : *mut xcb_x_print_print_input_selected_reply_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_x_print_print_input_selected_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_x_print_print_input_selected_reply (c : *mut ffi::base::xcb_connection_t,
                                                  cookie : xcb_x_print_print_input_selected_cookie_t,
                                                  e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_x_print_print_input_selected_reply_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_x_print_print_get_attributes (c : *mut ffi::base::xcb_connection_t,
                                            context :  xcb_x_print_pcontext_t,
                                            pool :  u8) -> xcb_x_print_print_get_attributes_cookie_t;

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
pub fn xcb_x_print_print_get_attributes_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                      context :  xcb_x_print_pcontext_t,
                                                      pool :  u8) -> xcb_x_print_print_get_attributes_cookie_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_x_print_print_get_attributes_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_x_print_print_get_attributes_reply (c : *mut ffi::base::xcb_connection_t,
                                                  cookie : xcb_x_print_print_get_attributes_cookie_t,
                                                  e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_x_print_print_get_attributes_reply_t;

pub fn xcb_x_print_print_get_one_attributes_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_x_print_print_get_one_attributes (c : *mut ffi::base::xcb_connection_t,
                                                context :  xcb_x_print_pcontext_t,
                                                nameLen :  u32,
                                                pool :  u8,
                                                name : *mut xcb_x_print_string8_t) -> xcb_x_print_print_get_one_attributes_cookie_t;

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
pub fn xcb_x_print_print_get_one_attributes_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                          context :  xcb_x_print_pcontext_t,
                                                          nameLen :  u32,
                                                          pool :  u8,
                                                          name : *mut xcb_x_print_string8_t) -> xcb_x_print_print_get_one_attributes_cookie_t;

pub fn xcb_x_print_print_get_one_attributes_value (R : *mut xcb_x_print_print_get_one_attributes_reply_t) -> *mut xcb_x_print_string8_t;


pub fn xcb_x_print_print_get_one_attributes_value_length (R : *mut xcb_x_print_print_get_one_attributes_reply_t) -> c_int;


pub fn xcb_x_print_print_get_one_attributes_value_end (R : *mut xcb_x_print_print_get_one_attributes_reply_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_x_print_print_get_one_attributes_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_x_print_print_get_one_attributes_reply (c : *mut ffi::base::xcb_connection_t,
                                                      cookie : xcb_x_print_print_get_one_attributes_cookie_t,
                                                      e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_x_print_print_get_one_attributes_reply_t;

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
pub fn xcb_x_print_print_set_attributes_checked (c : *mut ffi::base::xcb_connection_t,
                                                    context :  xcb_x_print_pcontext_t,
                                                    stringLen :  u32,
                                                    pool :  u8,
                                                    rule :  u8,
                                                    attributes_len :  u32,
                                                    attributes : *mut xcb_x_print_string8_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_x_print_print_set_attributes (c : *mut ffi::base::xcb_connection_t,
                                            context :  xcb_x_print_pcontext_t,
                                            stringLen :  u32,
                                            pool :  u8,
                                            rule :  u8,
                                            attributes_len :  u32,
                                            attributes : *mut xcb_x_print_string8_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_x_print_print_get_page_dimensions (c : *mut ffi::base::xcb_connection_t,
                                                 context :  xcb_x_print_pcontext_t) -> xcb_x_print_print_get_page_dimensions_cookie_t;

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
pub fn xcb_x_print_print_get_page_dimensions_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                           context :  xcb_x_print_pcontext_t) -> xcb_x_print_print_get_page_dimensions_cookie_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_x_print_print_get_page_dimensions_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_x_print_print_get_page_dimensions_reply (c : *mut ffi::base::xcb_connection_t,
                                                       cookie : xcb_x_print_print_get_page_dimensions_cookie_t,
                                                       e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_x_print_print_get_page_dimensions_reply_t;

pub fn xcb_x_print_print_query_screens_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_x_print_print_query_screens (c : *mut ffi::base::xcb_connection_t) -> xcb_x_print_print_query_screens_cookie_t;

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
pub fn xcb_x_print_print_query_screens_unchecked (c : *mut ffi::base::xcb_connection_t) -> xcb_x_print_print_query_screens_cookie_t;

pub fn xcb_x_print_print_query_screens_roots (R : *mut xcb_x_print_print_query_screens_reply_t) -> *mut ffi::xproto::xcb_window_t;


pub fn xcb_x_print_print_query_screens_roots_length (R : *mut xcb_x_print_print_query_screens_reply_t) -> c_int;


pub fn xcb_x_print_print_query_screens_roots_end (R : *mut xcb_x_print_print_query_screens_reply_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_x_print_print_query_screens_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_x_print_print_query_screens_reply (c : *mut ffi::base::xcb_connection_t,
                                                 cookie : xcb_x_print_print_query_screens_cookie_t,
                                                 e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_x_print_print_query_screens_reply_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_x_print_print_set_image_resolution (c : *mut ffi::base::xcb_connection_t,
                                                  context :  xcb_x_print_pcontext_t,
                                                  image_resolution :  u16) -> xcb_x_print_print_set_image_resolution_cookie_t;

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
pub fn xcb_x_print_print_set_image_resolution_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                            context :  xcb_x_print_pcontext_t,
                                                            image_resolution :  u16) -> xcb_x_print_print_set_image_resolution_cookie_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_x_print_print_set_image_resolution_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_x_print_print_set_image_resolution_reply (c : *mut ffi::base::xcb_connection_t,
                                                        cookie : xcb_x_print_print_set_image_resolution_cookie_t,
                                                        e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_x_print_print_set_image_resolution_reply_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_x_print_print_get_image_resolution (c : *mut ffi::base::xcb_connection_t,
                                                  context :  xcb_x_print_pcontext_t) -> xcb_x_print_print_get_image_resolution_cookie_t;

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
pub fn xcb_x_print_print_get_image_resolution_unchecked (c : *mut ffi::base::xcb_connection_t,
                                                            context :  xcb_x_print_pcontext_t) -> xcb_x_print_print_get_image_resolution_cookie_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_x_print_print_get_image_resolution_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_x_print_print_get_image_resolution_reply (c : *mut ffi::base::xcb_connection_t,
                                                        cookie : xcb_x_print_print_get_image_resolution_cookie_t,
                                                        e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_x_print_print_get_image_resolution_reply_t;
}

