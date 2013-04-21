/*
 * This file generated automatically from xprint.xml by r_client.py.
 * Edit at your peril.
 */

//Make the compiler quiet
#[allow(unused_imports)];
#[allow(non_camel_case_types)];
use core::libc::*;
use ll::base::*;
use ll::xproto;

pub static XPRINT_MAJOR_VERSION : c_uint = 1;
pub static XPRINT_MINOR_VERSION : c_uint = 0;

pub type string8 = u8;

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

pub type get_doc = c_uint;//{
    pub static XCB_X_PRINT_GET_DOC_FINISHED : get_doc = 0;
    pub static XCB_X_PRINT_GET_DOC_SECOND_CONSUMER : get_doc = 1;
//}

pub type ev_mask = c_uint;//{
    pub static XCB_X_PRINT_EV_MASK_NO_EVENT_MASK : ev_mask = 0;
    pub static XCB_X_PRINT_EV_MASK_PRINT_MASK : ev_mask = 1;
    pub static XCB_X_PRINT_EV_MASK_ATTRIBUTE_MASK : ev_mask = 2;
//}

pub type detail = c_uint;//{
    pub static XCB_X_PRINT_DETAIL_START_JOB_NOTIFY : detail = 1;
    pub static XCB_X_PRINT_DETAIL_END_JOB_NOTIFY : detail = 2;
    pub static XCB_X_PRINT_DETAIL_START_DOC_NOTIFY : detail = 3;
    pub static XCB_X_PRINT_DETAIL_END_DOC_NOTIFY : detail = 4;
    pub static XCB_X_PRINT_DETAIL_START_PAGE_NOTIFY : detail = 5;
    pub static XCB_X_PRINT_DETAIL_END_PAGE_NOTIFY : detail = 6;
//}

pub type attr = c_uint;//{
    pub static XCB_X_PRINT_ATTR_JOB_ATTR : attr = 1;
    pub static XCB_X_PRINT_ATTR_DOC_ATTR : attr = 2;
    pub static XCB_X_PRINT_ATTR_PAGE_ATTR : attr = 3;
    pub static XCB_X_PRINT_ATTR_PRINTER_ATTR : attr = 4;
    pub static XCB_X_PRINT_ATTR_SERVER_ATTR : attr = 5;
    pub static XCB_X_PRINT_ATTR_MEDIUM_ATTR : attr = 6;
    pub static XCB_X_PRINT_ATTR_SPOOLER_ATTR : attr = 7;
//}

pub struct print_query_version_cookie {
    sequence : c_uint
}

/** Opcode for xcb_x_print_print_query_version. */
pub static XCB_X_PRINT_PRINT_QUERY_VERSION : c_int = 0;

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

/** Opcode for xcb_x_print_print_get_printer_list. */
pub static XCB_X_PRINT_PRINT_GET_PRINTER_LIST : c_int = 1;

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

/** Opcode for xcb_x_print_print_rehash_printer_list. */
pub static XCB_X_PRINT_PRINT_REHASH_PRINTER_LIST : c_int = 20;

pub struct print_rehash_printer_list_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16
}

/** Opcode for xcb_x_print_create_context. */
pub static XCB_X_PRINT_CREATE_CONTEXT : c_int = 2;

pub struct create_context_request {
    major_opcode :     u8,
    minor_opcode :     u8,
    length :           u16,
    context_id :       u32,
    printerNameLen :   u32,
    localeLen :        u32
}

/** Opcode for xcb_x_print_print_set_context. */
pub static XCB_X_PRINT_PRINT_SET_CONTEXT : c_int = 3;

pub struct print_set_context_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context :        u32
}

pub struct print_get_context_cookie {
    sequence : c_uint
}

/** Opcode for xcb_x_print_print_get_context. */
pub static XCB_X_PRINT_PRINT_GET_CONTEXT : c_int = 4;

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

/** Opcode for xcb_x_print_print_destroy_context. */
pub static XCB_X_PRINT_PRINT_DESTROY_CONTEXT : c_int = 5;

pub struct print_destroy_context_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context :        u32
}

pub struct print_get_screen_of_context_cookie {
    sequence : c_uint
}

/** Opcode for xcb_x_print_print_get_screen_of_context. */
pub static XCB_X_PRINT_PRINT_GET_SCREEN_OF_CONTEXT : c_int = 6;

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
    root :            xproto::window
}

/** Opcode for xcb_x_print_print_start_job. */
pub static XCB_X_PRINT_PRINT_START_JOB : c_int = 7;

pub struct print_start_job_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    output_mode :    u8
}

/** Opcode for xcb_x_print_print_end_job. */
pub static XCB_X_PRINT_PRINT_END_JOB : c_int = 8;

pub struct print_end_job_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    cancel :         u8
}

/** Opcode for xcb_x_print_print_start_doc. */
pub static XCB_X_PRINT_PRINT_START_DOC : c_int = 9;

pub struct print_start_doc_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    driver_mode :    u8
}

/** Opcode for xcb_x_print_print_end_doc. */
pub static XCB_X_PRINT_PRINT_END_DOC : c_int = 10;

pub struct print_end_doc_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    cancel :         u8
}

/** Opcode for xcb_x_print_print_put_document_data. */
pub static XCB_X_PRINT_PRINT_PUT_DOCUMENT_DATA : c_int = 11;

pub struct print_put_document_data_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    drawable :       xproto::drawable,
    len_data :       u32,
    len_fmt :        u16,
    len_options :    u16
}

pub struct print_get_document_data_cookie {
    sequence : c_uint
}

/** Opcode for xcb_x_print_print_get_document_data. */
pub static XCB_X_PRINT_PRINT_GET_DOCUMENT_DATA : c_int = 12;

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

/** Opcode for xcb_x_print_print_start_page. */
pub static XCB_X_PRINT_PRINT_START_PAGE : c_int = 13;

pub struct print_start_page_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    window :         xproto::window
}

/** Opcode for xcb_x_print_print_end_page. */
pub static XCB_X_PRINT_PRINT_END_PAGE : c_int = 14;

pub struct print_end_page_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    cancel :         u8,
    pad0 :           [u8,..3]
}

/** Opcode for xcb_x_print_print_select_input. */
pub static XCB_X_PRINT_PRINT_SELECT_INPUT : c_int = 15;

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

/** Opcode for xcb_x_print_print_input_selected. */
pub static XCB_X_PRINT_PRINT_INPUT_SELECTED : c_int = 16;

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

/** Opcode for xcb_x_print_print_get_attributes. */
pub static XCB_X_PRINT_PRINT_GET_ATTRIBUTES : c_int = 17;

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

/** Opcode for xcb_x_print_print_get_one_attributes. */
pub static XCB_X_PRINT_PRINT_GET_ONE_ATTRIBUTES : c_int = 19;

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

/** Opcode for xcb_x_print_print_set_attributes. */
pub static XCB_X_PRINT_PRINT_SET_ATTRIBUTES : c_int = 18;

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

/** Opcode for xcb_x_print_print_get_page_dimensions. */
pub static XCB_X_PRINT_PRINT_GET_PAGE_DIMENSIONS : c_int = 21;

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

/** Opcode for xcb_x_print_print_query_screens. */
pub static XCB_X_PRINT_PRINT_QUERY_SCREENS : c_int = 22;

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

/** Opcode for xcb_x_print_print_set_image_resolution. */
pub static XCB_X_PRINT_PRINT_SET_IMAGE_RESOLUTION : c_int = 23;

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

/** Opcode for xcb_x_print_print_get_image_resolution. */
pub static XCB_X_PRINT_PRINT_GET_IMAGE_RESOLUTION : c_int = 24;

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

/** Opcode for xcb_x_print_notify. */
pub static XCB_X_PRINT_NOTIFY : c_int = 0;

pub struct notify_event {
    response_type :   u8,
    detail :          u8,
    sequence :        u16,
    context :         pcontext,
    cancel :          u8
}

/** Opcode for xcb_x_print_attribut_notify. */
pub static XCB_X_PRINT_ATTRIBUT_NOTIFY : c_int = 1;

pub struct attribut_notify_event {
    response_type :   u8,
    detail :          u8,
    sequence :        u16,
    context :         pcontext
}

/** Opcode for xcb_x_print_bad_context. */
pub static XCB_X_PRINT_BAD_CONTEXT : c_int = 0;

pub struct bad_context_error {
    response_type :   u8,
    error_code :      u8,
    sequence :        u16
}

/** Opcode for xcb_x_print_bad_sequence. */
pub static XCB_X_PRINT_BAD_SEQUENCE : c_int = 1;

pub struct bad_sequence_error {
    response_type :   u8,
    error_code :      u8,
    sequence :        u16
}
#[link_args="-lxcb-xprint"]
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
unsafe fn xcb_x_print_string8_next (i:*string8_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An string8_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_x_print_string8_end (i:string8_iterator) -> generic_iterator;

unsafe fn xcb_x_print_printer_serialize (_buffer :  **c_void,
                               _aux :     *printer,
                               name :     *string8,
                               description :  *string8) -> c_int;

unsafe fn xcb_x_print_printer_unserialize (_buffer :   *c_void,
                                 _aux :     **printer) -> c_int;

unsafe fn xcb_x_print_printer_sizeof (_buffer :  *c_void) -> c_int;

unsafe fn xcb_x_print_printer_name (R : *printer) -> *string8;


unsafe fn xcb_x_print_printer_name_length (R : *printer) -> c_int;


unsafe fn xcb_x_print_printer_name_end (R : *printer) -> generic_iterator;

unsafe fn xcb_x_print_printer_description (R : *printer) -> *string8;


unsafe fn xcb_x_print_printer_description_length (R : *printer) -> c_int;


unsafe fn xcb_x_print_printer_description_end (R : *printer) -> generic_iterator;

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
unsafe fn xcb_x_print_printer_next (i:*printer_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An printer_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_x_print_printer_end (i:printer_iterator) -> generic_iterator;

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
unsafe fn xcb_x_print_pcontext_next (i:*pcontext_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An pcontext_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_x_print_pcontext_end (i:pcontext_iterator) -> generic_iterator;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_x_print_print_query_version (c : *connection) -> print_query_version_cookie;

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
unsafe fn xcb_x_print_print_query_version_unchecked (c : *connection) -> print_query_version_cookie;

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
unsafe fn xcb_x_print_print_query_version_reply (c : *connection,
                                                 cookie : print_query_version_cookie,
                                                 e : **generic_error) -> *print_query_version_reply;

unsafe fn xcb_x_print_print_get_printer_list_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_x_print_print_get_printer_list (c : *connection,
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
unsafe fn xcb_x_print_print_get_printer_list_unchecked (c : *connection,
                                                        printerNameLen :  u32,
                                                        localeLen :  u32,
                                                        printer_name : *string8,
                                                        locale : *string8) -> print_get_printer_list_cookie;


unsafe fn xcb_x_print_print_get_printer_list_printers_length (R : *print_get_printer_list_reply) -> c_int;

unsafe fn xcb_x_print_print_get_printer_list_printers_iterator (R : *print_get_printer_list_reply) -> printer_iterator;

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
unsafe fn xcb_x_print_print_get_printer_list_reply (c : *connection,
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
unsafe fn xcb_x_print_print_rehash_printer_list_checked (c : *connection) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_x_print_print_rehash_printer_list (c : *connection) -> void_cookie;

unsafe fn xcb_x_print_create_context_sizeof (_buffer :  *c_void) -> c_int;

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
unsafe fn xcb_x_print_create_context_checked (c : *connection,
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
unsafe fn xcb_x_print_create_context (c : *connection,
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
unsafe fn xcb_x_print_print_set_context_checked (c : *connection,
                                                 context :  u32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_x_print_print_set_context (c : *connection,
                                         context :  u32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_x_print_print_get_context (c : *connection) -> print_get_context_cookie;

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
unsafe fn xcb_x_print_print_get_context_unchecked (c : *connection) -> print_get_context_cookie;

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
unsafe fn xcb_x_print_print_get_context_reply (c : *connection,
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
unsafe fn xcb_x_print_print_destroy_context_checked (c : *connection,
                                                     context :  u32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_x_print_print_destroy_context (c : *connection,
                                             context :  u32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_x_print_print_get_screen_of_context (c : *connection) -> print_get_screen_of_context_cookie;

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
unsafe fn xcb_x_print_print_get_screen_of_context_unchecked (c : *connection) -> print_get_screen_of_context_cookie;

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
unsafe fn xcb_x_print_print_get_screen_of_context_reply (c : *connection,
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
unsafe fn xcb_x_print_print_start_job_checked (c : *connection,
                                               output_mode :  u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_x_print_print_start_job (c : *connection,
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
unsafe fn xcb_x_print_print_end_job_checked (c : *connection,
                                             cancel :  u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_x_print_print_end_job (c : *connection,
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
unsafe fn xcb_x_print_print_start_doc_checked (c : *connection,
                                               driver_mode :  u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_x_print_print_start_doc (c : *connection,
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
unsafe fn xcb_x_print_print_end_doc_checked (c : *connection,
                                             cancel :  u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_x_print_print_end_doc (c : *connection,
                                     cancel :  u8) -> void_cookie;

unsafe fn xcb_x_print_print_put_document_data_sizeof (_buffer :  *c_void,
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
unsafe fn xcb_x_print_print_put_document_data_checked (c : *connection,
                                                       drawable :  xproto::drawable,
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
unsafe fn xcb_x_print_print_put_document_data (c : *connection,
                                               drawable :  xproto::drawable,
                                               len_data :  u32,
                                               len_fmt :  u16,
                                               len_options :  u16,
                                               data : *u8,
                                               doc_format_len :  u32,
                                               doc_format : *string8,
                                               options_len :  u32,
                                               options : *string8) -> void_cookie;

unsafe fn xcb_x_print_print_get_document_data_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_x_print_print_get_document_data (c : *connection,
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
unsafe fn xcb_x_print_print_get_document_data_unchecked (c : *connection,
                                                         context :  pcontext,
                                                         max_bytes :  u32) -> print_get_document_data_cookie;

unsafe fn xcb_x_print_print_get_document_data_data (R : *print_get_document_data_reply) -> *u8;


unsafe fn xcb_x_print_print_get_document_data_data_length (R : *print_get_document_data_reply) -> c_int;


unsafe fn xcb_x_print_print_get_document_data_data_end (R : *print_get_document_data_reply) -> generic_iterator;

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
unsafe fn xcb_x_print_print_get_document_data_reply (c : *connection,
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
unsafe fn xcb_x_print_print_start_page_checked (c : *connection,
                                                window :  xproto::window) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_x_print_print_start_page (c : *connection,
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
unsafe fn xcb_x_print_print_end_page_checked (c : *connection,
                                              cancel :  u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_x_print_print_end_page (c : *connection,
                                      cancel :  u8) -> void_cookie;

unsafe fn xcb_x_print_print_select_input_sizeof (_buffer :  *c_void) -> c_int;

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
unsafe fn xcb_x_print_print_select_input_checked (c : *connection,
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
unsafe fn xcb_x_print_print_select_input (c : *connection,
                                          context :  pcontext,
                                          event_mask :  u32,
                                          event_list : *u32) -> void_cookie;

unsafe fn xcb_x_print_print_input_selected_serialize (_buffer :                    **c_void,
                                            _aux :                        *print_input_selected_reply,
                                            event_list :                  *u32,
                                            all_events_list :             *u32) -> c_int;

unsafe fn xcb_x_print_print_input_selected_unserialize (_buffer :                      *c_void,
                                              _aux :                        **print_input_selected_reply) -> c_int;

unsafe fn xcb_x_print_print_input_selected_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_x_print_print_input_selected (c : *connection,
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
unsafe fn xcb_x_print_print_input_selected_unchecked (c : *connection,
                                                      context :  pcontext) -> print_input_selected_cookie;

unsafe fn xcb_x_print_print_input_selected_event_list (R : *print_input_selected_reply) -> *u32;


unsafe fn xcb_x_print_print_input_selected_event_list_length (R : *print_input_selected_reply) -> c_int;


unsafe fn xcb_x_print_print_input_selected_event_list_end (R : *print_input_selected_reply) -> generic_iterator;

unsafe fn xcb_x_print_print_input_selected_all_events_list (R : *print_input_selected_reply) -> *u32;


unsafe fn xcb_x_print_print_input_selected_all_events_list_length (R : *print_input_selected_reply) -> c_int;


unsafe fn xcb_x_print_print_input_selected_all_events_list_end (R : *print_input_selected_reply) -> generic_iterator;

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
unsafe fn xcb_x_print_print_input_selected_reply (c : *connection,
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
unsafe fn xcb_x_print_print_get_attributes (c : *connection,
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
unsafe fn xcb_x_print_print_get_attributes_unchecked (c : *connection,
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
unsafe fn xcb_x_print_print_get_attributes_reply (c : *connection,
                                                  cookie : print_get_attributes_cookie,
                                                  e : **generic_error) -> *print_get_attributes_reply;

unsafe fn xcb_x_print_print_get_one_attributes_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_x_print_print_get_one_attributes (c : *connection,
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
unsafe fn xcb_x_print_print_get_one_attributes_unchecked (c : *connection,
                                                          context :  pcontext,
                                                          nameLen :  u32,
                                                          pool :  u8,
                                                          name : *string8) -> print_get_one_attributes_cookie;

unsafe fn xcb_x_print_print_get_one_attributes_value (R : *print_get_one_attributes_reply) -> *string8;


unsafe fn xcb_x_print_print_get_one_attributes_value_length (R : *print_get_one_attributes_reply) -> c_int;


unsafe fn xcb_x_print_print_get_one_attributes_value_end (R : *print_get_one_attributes_reply) -> generic_iterator;

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
unsafe fn xcb_x_print_print_get_one_attributes_reply (c : *connection,
                                                      cookie : print_get_one_attributes_cookie,
                                                      e : **generic_error) -> *print_get_one_attributes_reply;

unsafe fn xcb_x_print_print_set_attributes_sizeof (_buffer :  *c_void,
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
unsafe fn xcb_x_print_print_set_attributes_checked (c : *connection,
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
unsafe fn xcb_x_print_print_set_attributes (c : *connection,
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
unsafe fn xcb_x_print_print_get_page_dimensions (c : *connection,
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
unsafe fn xcb_x_print_print_get_page_dimensions_unchecked (c : *connection,
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
unsafe fn xcb_x_print_print_get_page_dimensions_reply (c : *connection,
                                                       cookie : print_get_page_dimensions_cookie,
                                                       e : **generic_error) -> *print_get_page_dimensions_reply;

unsafe fn xcb_x_print_print_query_screens_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_x_print_print_query_screens (c : *connection) -> print_query_screens_cookie;

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
unsafe fn xcb_x_print_print_query_screens_unchecked (c : *connection) -> print_query_screens_cookie;

unsafe fn xcb_x_print_print_query_screens_roots (R : *print_query_screens_reply) -> *xproto::window;


unsafe fn xcb_x_print_print_query_screens_roots_length (R : *print_query_screens_reply) -> c_int;


unsafe fn xcb_x_print_print_query_screens_roots_end (R : *print_query_screens_reply) -> generic_iterator;

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
unsafe fn xcb_x_print_print_query_screens_reply (c : *connection,
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
unsafe fn xcb_x_print_print_set_image_resolution (c : *connection,
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
unsafe fn xcb_x_print_print_set_image_resolution_unchecked (c : *connection,
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
unsafe fn xcb_x_print_print_set_image_resolution_reply (c : *connection,
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
unsafe fn xcb_x_print_print_get_image_resolution (c : *connection,
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
unsafe fn xcb_x_print_print_get_image_resolution_unchecked (c : *connection,
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
unsafe fn xcb_x_print_print_get_image_resolution_reply (c : *connection,
                                                        cookie : print_get_image_resolution_cookie,
                                                        e : **generic_error) -> *print_get_image_resolution_reply;
}

