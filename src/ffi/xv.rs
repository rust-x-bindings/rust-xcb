/*
 * This file generated automatically from xv.xml by r_client.py.
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
use ffi::shm;

pub static XV_MAJOR_VERSION : c_uint = 2;
pub static XV_MINOR_VERSION : c_uint = 2;

pub type port = u32;
/**
 * @brief port_iterator
 **/
pub struct port_iterator {
    data : *port,
    rem  : c_int,
    index: c_int
}


pub type encoding = u32;
/**
 * @brief encoding_iterator
 **/
pub struct encoding_iterator {
    data : *encoding,
    rem  : c_int,
    index: c_int
}


pub struct rational {
    numerator :     i32,
    denominator :   i32
}

/**
 * @brief rational_iterator
 **/
pub struct rational_iterator {
    data : *rational,
    rem  : c_int,
    index: c_int
}


pub struct format {
    visual :   ffi::xproto::visualid,
    depth :    u8,
    pad0 :     [u8,..3]
}

/**
 * @brief format_iterator
 **/
pub struct format_iterator {
    data : *format,
    rem  : c_int,
    index: c_int
}


pub struct adaptor_info {
    base_id :       port,
    name_size :     u16,
    num_ports :     u16,
    num_formats :   u16,
    type_ :         u8,
    pad0 :          u8
}

/**
 * @brief adaptor_info_iterator
 **/
pub struct adaptor_info_iterator {
    data : *adaptor_info,
    rem  : c_int,
    index: c_int
}


pub struct encoding_info {
    encoding :    encoding,
    name_size :   u16,
    width :       u16,
    height :      u16,
    pad0 :        [u8,..2],
    rate :        rational
}

/**
 * @brief encoding_info_iterator
 **/
pub struct encoding_info_iterator {
    data : *encoding_info,
    rem  : c_int,
    index: c_int
}


pub struct image {
    id :           u32,
    width :        u16,
    height :       u16,
    data_size :    u32,
    num_planes :   u32
}

/**
 * @brief image_iterator
 **/
pub struct image_iterator {
    data : *image,
    rem  : c_int,
    index: c_int
}


pub struct attribute_info {
    flags :   u32,
    min :     i32,
    max :     i32,
    size :    u32
}

/**
 * @brief attribute_info_iterator
 **/
pub struct attribute_info_iterator {
    data : *attribute_info,
    rem  : c_int,
    index: c_int
}


pub struct image_format_info {
    id :                u32,
    type_ :             u8,
    byte_order :        u8,
    pad0 :              [u8,..2],
    guid :              [u8,..16],
    bpp :               u8,
    num_planes :        u8,
    pad1 :              [u8,..2],
    depth :             u8,
    pad2 :              [u8,..3],
    red_mask :          u32,
    green_mask :        u32,
    blue_mask :         u32,
    format :            u8,
    pad3 :              [u8,..3],
    y_sample_bits :     u32,
    u_sample_bits :     u32,
    v_sample_bits :     u32,
    vhorz_y_period :    u32,
    vhorz_u_period :    u32,
    vhorz_v_period :    u32,
    vvert_y_period :    u32,
    vvert_u_period :    u32,
    vvert_v_period :    u32,
    vcomp_order :       [u8,..32],
    vscanline_order :   u8,
    pad4 :              [u8,..11]
}

/**
 * @brief image_format_info_iterator
 **/
pub struct image_format_info_iterator {
    data : *image_format_info,
    rem  : c_int,
    index: c_int
}



pub struct bad_port_error {
    response_type :   u8,
    error_code :      u8,
    sequence :        u16
}



pub struct bad_encoding_error {
    response_type :   u8,
    error_code :      u8,
    sequence :        u16
}



pub struct bad_control_error {
    response_type :   u8,
    error_code :      u8,
    sequence :        u16
}



pub struct video_notify_event {
    response_type :   u8,
    reason :          u8,
    sequence :        u16,
    time :            ffi::xproto::timestamp,
    drawable :        ffi::xproto::drawable,
    port :            port
}



pub struct port_notify_event {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    time :            ffi::xproto::timestamp,
    port :            port,
    attribute :       ffi::xproto::atom,
    value :           i32
}


pub struct query_extension_cookie {
    sequence : c_uint
}


pub struct query_extension_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16
}


pub struct query_extension_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    major :           u16,
    minor :           u16
}


pub struct query_adaptors_cookie {
    sequence : c_uint
}


pub struct query_adaptors_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    window :         ffi::xproto::window
}


pub struct query_adaptors_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    num_adaptors :    u16,
    pad1 :            [u8,..22]
}


pub struct query_encodings_cookie {
    sequence : c_uint
}


pub struct query_encodings_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    port :           port
}


pub struct query_encodings_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    num_encodings :   u16,
    pad1 :            [u8,..22]
}


pub struct grab_port_cookie {
    sequence : c_uint
}


pub struct grab_port_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    port :           port,
    time :           ffi::xproto::timestamp
}


pub struct grab_port_reply {
    response_type :   u8,
    result :          u8,
    sequence :        u16,
    length :          u32
}



pub struct ungrab_port_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    port :           port,
    time :           ffi::xproto::timestamp
}



pub struct put_video_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    port :           port,
    drawable :       ffi::xproto::drawable,
    gc :             ffi::xproto::gcontext,
    vid_x :          i16,
    vid_y :          i16,
    vid_w :          u16,
    vid_h :          u16,
    drw_x :          i16,
    drw_y :          i16,
    drw_w :          u16,
    drw_h :          u16
}



pub struct put_still_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    port :           port,
    drawable :       ffi::xproto::drawable,
    gc :             ffi::xproto::gcontext,
    vid_x :          i16,
    vid_y :          i16,
    vid_w :          u16,
    vid_h :          u16,
    drw_x :          i16,
    drw_y :          i16,
    drw_w :          u16,
    drw_h :          u16
}



pub struct get_video_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    port :           port,
    drawable :       ffi::xproto::drawable,
    gc :             ffi::xproto::gcontext,
    vid_x :          i16,
    vid_y :          i16,
    vid_w :          u16,
    vid_h :          u16,
    drw_x :          i16,
    drw_y :          i16,
    drw_w :          u16,
    drw_h :          u16
}



pub struct get_still_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    port :           port,
    drawable :       ffi::xproto::drawable,
    gc :             ffi::xproto::gcontext,
    vid_x :          i16,
    vid_y :          i16,
    vid_w :          u16,
    vid_h :          u16,
    drw_x :          i16,
    drw_y :          i16,
    drw_w :          u16,
    drw_h :          u16
}



pub struct stop_video_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    port :           port,
    drawable :       ffi::xproto::drawable
}



pub struct select_video_notify_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    drawable :       ffi::xproto::drawable,
    onoff :          u8,
    pad0 :           [u8,..3]
}



pub struct select_port_notify_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    port :           port,
    onoff :          u8,
    pad0 :           [u8,..3]
}


pub struct query_best_size_cookie {
    sequence : c_uint
}


pub struct query_best_size_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    port :           port,
    vid_w :          u16,
    vid_h :          u16,
    drw_w :          u16,
    drw_h :          u16,
    motion :         u8,
    pad0 :           [u8,..3]
}


pub struct query_best_size_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    actual_width :    u16,
    actual_height :   u16
}



pub struct set_port_attribute_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    port :           port,
    attribute :      ffi::xproto::atom,
    value :          i32
}


pub struct get_port_attribute_cookie {
    sequence : c_uint
}


pub struct get_port_attribute_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    port :           port,
    attribute :      ffi::xproto::atom
}


pub struct get_port_attribute_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    value :           i32
}


pub struct query_port_attributes_cookie {
    sequence : c_uint
}


pub struct query_port_attributes_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    port :           port
}


pub struct query_port_attributes_reply {
    response_type :    u8,
    pad0 :             u8,
    sequence :         u16,
    length :           u32,
    num_attributes :   u32,
    text_size :        u32,
    pad1 :             [u8,..16]
}


pub struct list_image_formats_cookie {
    sequence : c_uint
}


pub struct list_image_formats_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    port :           port
}


pub struct list_image_formats_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    num_formats :     u32,
    pad1 :            [u8,..20]
}


pub struct query_image_attributes_cookie {
    sequence : c_uint
}


pub struct query_image_attributes_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    port :           port,
    id :             u32,
    width :          u16,
    height :         u16
}


pub struct query_image_attributes_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    num_planes :      u32,
    data_size :       u32,
    width :           u16,
    height :          u16,
    pad1 :            [u8,..12]
}



pub struct put_image_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    port :           port,
    drawable :       ffi::xproto::drawable,
    gc :             ffi::xproto::gcontext,
    id :             u32,
    src_x :          i16,
    src_y :          i16,
    src_w :          u16,
    src_h :          u16,
    drw_x :          i16,
    drw_y :          i16,
    drw_w :          u16,
    drw_h :          u16,
    width :          u16,
    height :         u16
}



pub struct shm_put_image_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    port :           port,
    drawable :       ffi::xproto::drawable,
    gc :             ffi::xproto::gcontext,
    shmseg :         ffi::shm::seg,
    id :             u32,
    offset :         u32,
    src_x :          i16,
    src_y :          i16,
    src_w :          u16,
    src_h :          u16,
    drw_x :          i16,
    drw_y :          i16,
    drw_w :          u16,
    drw_h :          u16,
    width :          u16,
    height :         u16,
    send_event :     u8,
    pad0 :           [u8,..3]
}

#[link_args="-lxcb-xv"]
pub extern "C" {

/**
 * Get the next element of the iterator
 * @param i Pointer to a port_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(port)
 *
 *
 */
pub unsafe fn xcb_xv_port_next (i:*port_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An port_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub unsafe fn xcb_xv_port_end (i:port_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a encoding_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(encoding)
 *
 *
 */
pub unsafe fn xcb_xv_encoding_next (i:*encoding_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An encoding_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub unsafe fn xcb_xv_encoding_end (i:encoding_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a rational_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(rational)
 *
 *
 */
pub unsafe fn xcb_xv_rational_next (i:*rational_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An rational_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub unsafe fn xcb_xv_rational_end (i:rational_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a format_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(format)
 *
 *
 */
pub unsafe fn xcb_xv_format_next (i:*format_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An format_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub unsafe fn xcb_xv_format_end (i:format_iterator) -> generic_iterator;

pub unsafe fn xcb_xv_adaptor_info_sizeof (_buffer :  *c_void) -> c_int;

pub unsafe fn xcb_xv_adaptor_info_name (R : *adaptor_info) -> *c_char;


pub unsafe fn xcb_xv_adaptor_info_name_length (R : *adaptor_info) -> c_int;


pub unsafe fn xcb_xv_adaptor_info_name_end (R : *adaptor_info) -> generic_iterator;

pub unsafe fn xcb_xv_adaptor_info_formats (R : *adaptor_info) -> *format;


pub unsafe fn xcb_xv_adaptor_info_formats_length (R : *adaptor_info) -> c_int;

pub unsafe fn xcb_xv_adaptor_info_formats_iterator (R : *adaptor_info) -> format_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a adaptor_info_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(adaptor_info)
 *
 *
 */
pub unsafe fn xcb_xv_adaptor_info_next (i:*adaptor_info_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An adaptor_info_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub unsafe fn xcb_xv_adaptor_info_end (i:adaptor_info_iterator) -> generic_iterator;

pub unsafe fn xcb_xv_encoding_info_sizeof (_buffer :  *c_void) -> c_int;

pub unsafe fn xcb_xv_encoding_info_name (R : *encoding_info) -> *c_char;


pub unsafe fn xcb_xv_encoding_info_name_length (R : *encoding_info) -> c_int;


pub unsafe fn xcb_xv_encoding_info_name_end (R : *encoding_info) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a encoding_info_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(encoding_info)
 *
 *
 */
pub unsafe fn xcb_xv_encoding_info_next (i:*encoding_info_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An encoding_info_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub unsafe fn xcb_xv_encoding_info_end (i:encoding_info_iterator) -> generic_iterator;

pub unsafe fn xcb_xv_image_sizeof (_buffer :  *c_void) -> c_int;

pub unsafe fn xcb_xv_image_pitches (R : *image) -> *u32;


pub unsafe fn xcb_xv_image_pitches_length (R : *image) -> c_int;


pub unsafe fn xcb_xv_image_pitches_end (R : *image) -> generic_iterator;

pub unsafe fn xcb_xv_image_offsets (R : *image) -> *u32;


pub unsafe fn xcb_xv_image_offsets_length (R : *image) -> c_int;


pub unsafe fn xcb_xv_image_offsets_end (R : *image) -> generic_iterator;

pub unsafe fn xcb_xv_image_data (R : *image) -> *u8;


pub unsafe fn xcb_xv_image_data_length (R : *image) -> c_int;


pub unsafe fn xcb_xv_image_data_end (R : *image) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a image_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(image)
 *
 *
 */
pub unsafe fn xcb_xv_image_next (i:*image_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An image_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub unsafe fn xcb_xv_image_end (i:image_iterator) -> generic_iterator;

pub unsafe fn xcb_xv_attribute_info_sizeof (_buffer :  *c_void) -> c_int;

pub unsafe fn xcb_xv_attribute_info_name (R : *attribute_info) -> *c_char;


pub unsafe fn xcb_xv_attribute_info_name_length (R : *attribute_info) -> c_int;


pub unsafe fn xcb_xv_attribute_info_name_end (R : *attribute_info) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a attribute_info_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(attribute_info)
 *
 *
 */
pub unsafe fn xcb_xv_attribute_info_next (i:*attribute_info_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An attribute_info_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub unsafe fn xcb_xv_attribute_info_end (i:attribute_info_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a image_format_info_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(image_format_info)
 *
 *
 */
pub unsafe fn xcb_xv_image_format_info_next (i:*image_format_info_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An image_format_info_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub unsafe fn xcb_xv_image_format_info_end (i:image_format_info_iterator) -> generic_iterator;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_xv_query_extension (c : *connection) -> query_extension_cookie;

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
pub unsafe fn xcb_xv_query_extension_unchecked (c : *connection) -> query_extension_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xv_query_extension_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub unsafe fn xcb_xv_query_extension_reply (c : *connection,
                                        cookie : query_extension_cookie,
                                        e : **generic_error) -> *query_extension_reply;

pub unsafe fn xcb_xv_query_adaptors_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_xv_query_adaptors (c : *connection,
                                 window :  ffi::xproto::window) -> query_adaptors_cookie;

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
pub unsafe fn xcb_xv_query_adaptors_unchecked (c : *connection,
                                           window :  ffi::xproto::window) -> query_adaptors_cookie;


pub unsafe fn xcb_xv_query_adaptors_info_length (R : *query_adaptors_reply) -> c_int;

pub unsafe fn xcb_xv_query_adaptors_info_iterator (R : *query_adaptors_reply) -> adaptor_info_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xv_query_adaptors_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub unsafe fn xcb_xv_query_adaptors_reply (c : *connection,
                                       cookie : query_adaptors_cookie,
                                       e : **generic_error) -> *query_adaptors_reply;

pub unsafe fn xcb_xv_query_encodings_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_xv_query_encodings (c : *connection,
                                  port :  port) -> query_encodings_cookie;

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
pub unsafe fn xcb_xv_query_encodings_unchecked (c : *connection,
                                            port :  port) -> query_encodings_cookie;


pub unsafe fn xcb_xv_query_encodings_info_length (R : *query_encodings_reply) -> c_int;

pub unsafe fn xcb_xv_query_encodings_info_iterator (R : *query_encodings_reply) -> encoding_info_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xv_query_encodings_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub unsafe fn xcb_xv_query_encodings_reply (c : *connection,
                                        cookie : query_encodings_cookie,
                                        e : **generic_error) -> *query_encodings_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_xv_grab_port (c : *connection,
                            port :  port,
                            time :  ffi::xproto::timestamp) -> grab_port_cookie;

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
pub unsafe fn xcb_xv_grab_port_unchecked (c : *connection,
                                      port :  port,
                                      time :  ffi::xproto::timestamp) -> grab_port_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xv_grab_port_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub unsafe fn xcb_xv_grab_port_reply (c : *connection,
                                  cookie : grab_port_cookie,
                                  e : **generic_error) -> *grab_port_reply;

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
pub unsafe fn xcb_xv_ungrab_port_checked (c : *connection,
                                      port :  port,
                                      time :  ffi::xproto::timestamp) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_xv_ungrab_port (c : *connection,
                              port :  port,
                              time :  ffi::xproto::timestamp) -> void_cookie;

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
pub unsafe fn xcb_xv_put_video_checked (c : *connection,
                                    port :  port,
                                    drawable :  ffi::xproto::drawable,
                                    gc :  ffi::xproto::gcontext,
                                    vid_x :  i16,
                                    vid_y :  i16,
                                    vid_w :  u16,
                                    vid_h :  u16,
                                    drw_x :  i16,
                                    drw_y :  i16,
                                    drw_w :  u16,
                                    drw_h :  u16) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_xv_put_video (c : *connection,
                            port :  port,
                            drawable :  ffi::xproto::drawable,
                            gc :  ffi::xproto::gcontext,
                            vid_x :  i16,
                            vid_y :  i16,
                            vid_w :  u16,
                            vid_h :  u16,
                            drw_x :  i16,
                            drw_y :  i16,
                            drw_w :  u16,
                            drw_h :  u16) -> void_cookie;

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
pub unsafe fn xcb_xv_put_still_checked (c : *connection,
                                    port :  port,
                                    drawable :  ffi::xproto::drawable,
                                    gc :  ffi::xproto::gcontext,
                                    vid_x :  i16,
                                    vid_y :  i16,
                                    vid_w :  u16,
                                    vid_h :  u16,
                                    drw_x :  i16,
                                    drw_y :  i16,
                                    drw_w :  u16,
                                    drw_h :  u16) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_xv_put_still (c : *connection,
                            port :  port,
                            drawable :  ffi::xproto::drawable,
                            gc :  ffi::xproto::gcontext,
                            vid_x :  i16,
                            vid_y :  i16,
                            vid_w :  u16,
                            vid_h :  u16,
                            drw_x :  i16,
                            drw_y :  i16,
                            drw_w :  u16,
                            drw_h :  u16) -> void_cookie;

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
pub unsafe fn xcb_xv_get_video_checked (c : *connection,
                                    port :  port,
                                    drawable :  ffi::xproto::drawable,
                                    gc :  ffi::xproto::gcontext,
                                    vid_x :  i16,
                                    vid_y :  i16,
                                    vid_w :  u16,
                                    vid_h :  u16,
                                    drw_x :  i16,
                                    drw_y :  i16,
                                    drw_w :  u16,
                                    drw_h :  u16) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_xv_get_video (c : *connection,
                            port :  port,
                            drawable :  ffi::xproto::drawable,
                            gc :  ffi::xproto::gcontext,
                            vid_x :  i16,
                            vid_y :  i16,
                            vid_w :  u16,
                            vid_h :  u16,
                            drw_x :  i16,
                            drw_y :  i16,
                            drw_w :  u16,
                            drw_h :  u16) -> void_cookie;

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
pub unsafe fn xcb_xv_get_still_checked (c : *connection,
                                    port :  port,
                                    drawable :  ffi::xproto::drawable,
                                    gc :  ffi::xproto::gcontext,
                                    vid_x :  i16,
                                    vid_y :  i16,
                                    vid_w :  u16,
                                    vid_h :  u16,
                                    drw_x :  i16,
                                    drw_y :  i16,
                                    drw_w :  u16,
                                    drw_h :  u16) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_xv_get_still (c : *connection,
                            port :  port,
                            drawable :  ffi::xproto::drawable,
                            gc :  ffi::xproto::gcontext,
                            vid_x :  i16,
                            vid_y :  i16,
                            vid_w :  u16,
                            vid_h :  u16,
                            drw_x :  i16,
                            drw_y :  i16,
                            drw_w :  u16,
                            drw_h :  u16) -> void_cookie;

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
pub unsafe fn xcb_xv_stop_video_checked (c : *connection,
                                     port :  port,
                                     drawable :  ffi::xproto::drawable) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_xv_stop_video (c : *connection,
                             port :  port,
                             drawable :  ffi::xproto::drawable) -> void_cookie;

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
pub unsafe fn xcb_xv_select_video_notify_checked (c : *connection,
                                              drawable :  ffi::xproto::drawable,
                                              onoff :  u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_xv_select_video_notify (c : *connection,
                                      drawable :  ffi::xproto::drawable,
                                      onoff :  u8) -> void_cookie;

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
pub unsafe fn xcb_xv_select_port_notify_checked (c : *connection,
                                             port :  port,
                                             onoff :  u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_xv_select_port_notify (c : *connection,
                                     port :  port,
                                     onoff :  u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_xv_query_best_size (c : *connection,
                                  port :  port,
                                  vid_w :  u16,
                                  vid_h :  u16,
                                  drw_w :  u16,
                                  drw_h :  u16,
                                  motion :  u8) -> query_best_size_cookie;

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
pub unsafe fn xcb_xv_query_best_size_unchecked (c : *connection,
                                            port :  port,
                                            vid_w :  u16,
                                            vid_h :  u16,
                                            drw_w :  u16,
                                            drw_h :  u16,
                                            motion :  u8) -> query_best_size_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xv_query_best_size_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub unsafe fn xcb_xv_query_best_size_reply (c : *connection,
                                        cookie : query_best_size_cookie,
                                        e : **generic_error) -> *query_best_size_reply;

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
pub unsafe fn xcb_xv_set_port_attribute_checked (c : *connection,
                                             port :  port,
                                             attribute :  ffi::xproto::atom,
                                             value :  i32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_xv_set_port_attribute (c : *connection,
                                     port :  port,
                                     attribute :  ffi::xproto::atom,
                                     value :  i32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_xv_get_port_attribute (c : *connection,
                                     port :  port,
                                     attribute :  ffi::xproto::atom) -> get_port_attribute_cookie;

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
pub unsafe fn xcb_xv_get_port_attribute_unchecked (c : *connection,
                                               port :  port,
                                               attribute :  ffi::xproto::atom) -> get_port_attribute_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xv_get_port_attribute_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub unsafe fn xcb_xv_get_port_attribute_reply (c : *connection,
                                           cookie : get_port_attribute_cookie,
                                           e : **generic_error) -> *get_port_attribute_reply;

pub unsafe fn xcb_xv_query_port_attributes_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_xv_query_port_attributes (c : *connection,
                                        port :  port) -> query_port_attributes_cookie;

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
pub unsafe fn xcb_xv_query_port_attributes_unchecked (c : *connection,
                                                  port :  port) -> query_port_attributes_cookie;


pub unsafe fn xcb_xv_query_port_attributes_attributes_length (R : *query_port_attributes_reply) -> c_int;

pub unsafe fn xcb_xv_query_port_attributes_attributes_iterator (R : *query_port_attributes_reply) -> attribute_info_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xv_query_port_attributes_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub unsafe fn xcb_xv_query_port_attributes_reply (c : *connection,
                                              cookie : query_port_attributes_cookie,
                                              e : **generic_error) -> *query_port_attributes_reply;

pub unsafe fn xcb_xv_list_image_formats_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_xv_list_image_formats (c : *connection,
                                     port :  port) -> list_image_formats_cookie;

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
pub unsafe fn xcb_xv_list_image_formats_unchecked (c : *connection,
                                               port :  port) -> list_image_formats_cookie;

pub unsafe fn xcb_xv_list_image_formats_format (R : *list_image_formats_reply) -> *image_format_info;


pub unsafe fn xcb_xv_list_image_formats_format_length (R : *list_image_formats_reply) -> c_int;

pub unsafe fn xcb_xv_list_image_formats_format_iterator (R : *list_image_formats_reply) -> image_format_info_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xv_list_image_formats_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub unsafe fn xcb_xv_list_image_formats_reply (c : *connection,
                                           cookie : list_image_formats_cookie,
                                           e : **generic_error) -> *list_image_formats_reply;

pub unsafe fn xcb_xv_query_image_attributes_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_xv_query_image_attributes (c : *connection,
                                         port :  port,
                                         id :  u32,
                                         width :  u16,
                                         height :  u16) -> query_image_attributes_cookie;

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
pub unsafe fn xcb_xv_query_image_attributes_unchecked (c : *connection,
                                                   port :  port,
                                                   id :  u32,
                                                   width :  u16,
                                                   height :  u16) -> query_image_attributes_cookie;

pub unsafe fn xcb_xv_query_image_attributes_pitches (R : *query_image_attributes_reply) -> *u32;


pub unsafe fn xcb_xv_query_image_attributes_pitches_length (R : *query_image_attributes_reply) -> c_int;


pub unsafe fn xcb_xv_query_image_attributes_pitches_end (R : *query_image_attributes_reply) -> generic_iterator;

pub unsafe fn xcb_xv_query_image_attributes_offsets (R : *query_image_attributes_reply) -> *u32;


pub unsafe fn xcb_xv_query_image_attributes_offsets_length (R : *query_image_attributes_reply) -> c_int;


pub unsafe fn xcb_xv_query_image_attributes_offsets_end (R : *query_image_attributes_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xv_query_image_attributes_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub unsafe fn xcb_xv_query_image_attributes_reply (c : *connection,
                                               cookie : query_image_attributes_cookie,
                                               e : **generic_error) -> *query_image_attributes_reply;

pub unsafe fn xcb_xv_put_image_sizeof (_buffer :  *c_void,
                         data_len :  u32) -> c_int;

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
pub unsafe fn xcb_xv_put_image_checked (c : *connection,
                                    port :  port,
                                    drawable :  ffi::xproto::drawable,
                                    gc :  ffi::xproto::gcontext,
                                    id :  u32,
                                    src_x :  i16,
                                    src_y :  i16,
                                    src_w :  u16,
                                    src_h :  u16,
                                    drw_x :  i16,
                                    drw_y :  i16,
                                    drw_w :  u16,
                                    drw_h :  u16,
                                    width :  u16,
                                    height :  u16,
                                    data_len :  u32,
                                    data : *u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_xv_put_image (c : *connection,
                            port :  port,
                            drawable :  ffi::xproto::drawable,
                            gc :  ffi::xproto::gcontext,
                            id :  u32,
                            src_x :  i16,
                            src_y :  i16,
                            src_w :  u16,
                            src_h :  u16,
                            drw_x :  i16,
                            drw_y :  i16,
                            drw_w :  u16,
                            drw_h :  u16,
                            width :  u16,
                            height :  u16,
                            data_len :  u32,
                            data : *u8) -> void_cookie;

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
pub unsafe fn xcb_xv_shm_put_image_checked (c : *connection,
                                        port :  port,
                                        drawable :  ffi::xproto::drawable,
                                        gc :  ffi::xproto::gcontext,
                                        shmseg :  ffi::shm::seg,
                                        id :  u32,
                                        offset :  u32,
                                        src_x :  i16,
                                        src_y :  i16,
                                        src_w :  u16,
                                        src_h :  u16,
                                        drw_x :  i16,
                                        drw_y :  i16,
                                        drw_w :  u16,
                                        drw_h :  u16,
                                        width :  u16,
                                        height :  u16,
                                        send_event :  u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_xv_shm_put_image (c : *connection,
                                port :  port,
                                drawable :  ffi::xproto::drawable,
                                gc :  ffi::xproto::gcontext,
                                shmseg :  ffi::shm::seg,
                                id :  u32,
                                offset :  u32,
                                src_x :  i16,
                                src_y :  i16,
                                src_w :  u16,
                                src_h :  u16,
                                drw_x :  i16,
                                drw_y :  i16,
                                drw_w :  u16,
                                drw_h :  u16,
                                width :  u16,
                                height :  u16,
                                send_event :  u8) -> void_cookie;
}

