/*
 * This file generated automatically from xv.xml by r_client.py.
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
use ffi::shm;

pub static XV_MAJOR_VERSION : c_uint = 2;
pub static XV_MINOR_VERSION : c_uint = 2;

pub type port = u32;
/**
 * @brief port_iterator
 **/
pub struct port_iterator {
    data : *mut port,
    rem  : c_int,
    index: c_int
}


pub type encoding = u32;
/**
 * @brief encoding_iterator
 **/
pub struct encoding_iterator {
    data : *mut encoding,
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
    data : *mut rational,
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
    data : *mut format,
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
    data : *mut adaptor_info,
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
    data : *mut encoding_info,
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
    data : *mut image,
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
    data : *mut attribute_info,
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
    data : *mut image_format_info,
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

#[link(name="lxcb-xv")]
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
pub fn xcb_xv_port_next (i:*mut port_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An port_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_xv_port_end (i:port_iterator) -> generic_iterator;

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
pub fn xcb_xv_encoding_next (i:*mut encoding_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An encoding_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_xv_encoding_end (i:encoding_iterator) -> generic_iterator;

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
pub fn xcb_xv_rational_next (i:*mut rational_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An rational_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_xv_rational_end (i:rational_iterator) -> generic_iterator;

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
pub fn xcb_xv_format_next (i:*mut format_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An format_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_xv_format_end (i:format_iterator) -> generic_iterator;

pub fn xcb_xv_adaptor_info_sizeof (_buffer :  *mut c_void) -> c_int;

pub fn xcb_xv_adaptor_info_name (R : *mut adaptor_info) -> *mut c_char;


pub fn xcb_xv_adaptor_info_name_length (R : *mut adaptor_info) -> c_int;


pub fn xcb_xv_adaptor_info_name_end (R : *mut adaptor_info) -> generic_iterator;

pub fn xcb_xv_adaptor_info_formats (R : *mut adaptor_info) -> *mut format;


pub fn xcb_xv_adaptor_info_formats_length (R : *mut adaptor_info) -> c_int;

pub fn xcb_xv_adaptor_info_formats_iterator (R : *mut adaptor_info) -> format_iterator;

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
pub fn xcb_xv_adaptor_info_next (i:*mut adaptor_info_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An adaptor_info_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_xv_adaptor_info_end (i:adaptor_info_iterator) -> generic_iterator;

pub fn xcb_xv_encoding_info_sizeof (_buffer :  *mut c_void) -> c_int;

pub fn xcb_xv_encoding_info_name (R : *mut encoding_info) -> *mut c_char;


pub fn xcb_xv_encoding_info_name_length (R : *mut encoding_info) -> c_int;


pub fn xcb_xv_encoding_info_name_end (R : *mut encoding_info) -> generic_iterator;

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
pub fn xcb_xv_encoding_info_next (i:*mut encoding_info_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An encoding_info_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_xv_encoding_info_end (i:encoding_info_iterator) -> generic_iterator;

pub fn xcb_xv_image_sizeof (_buffer :  *mut c_void) -> c_int;

pub fn xcb_xv_image_pitches (R : *mut image) -> *mut u32;


pub fn xcb_xv_image_pitches_length (R : *mut image) -> c_int;


pub fn xcb_xv_image_pitches_end (R : *mut image) -> generic_iterator;

pub fn xcb_xv_image_offsets (R : *mut image) -> *mut u32;


pub fn xcb_xv_image_offsets_length (R : *mut image) -> c_int;


pub fn xcb_xv_image_offsets_end (R : *mut image) -> generic_iterator;

pub fn xcb_xv_image_data (R : *mut image) -> *mut u8;


pub fn xcb_xv_image_data_length (R : *mut image) -> c_int;


pub fn xcb_xv_image_data_end (R : *mut image) -> generic_iterator;

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
pub fn xcb_xv_image_next (i:*mut image_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An image_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_xv_image_end (i:image_iterator) -> generic_iterator;

pub fn xcb_xv_attribute_info_sizeof (_buffer :  *mut c_void) -> c_int;

pub fn xcb_xv_attribute_info_name (R : *mut attribute_info) -> *mut c_char;


pub fn xcb_xv_attribute_info_name_length (R : *mut attribute_info) -> c_int;


pub fn xcb_xv_attribute_info_name_end (R : *mut attribute_info) -> generic_iterator;

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
pub fn xcb_xv_attribute_info_next (i:*mut attribute_info_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An attribute_info_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_xv_attribute_info_end (i:attribute_info_iterator) -> generic_iterator;

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
pub fn xcb_xv_image_format_info_next (i:*mut image_format_info_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An image_format_info_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_xv_image_format_info_end (i:image_format_info_iterator) -> generic_iterator;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xv_query_extension (c : *mut connection) -> query_extension_cookie;

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
pub fn xcb_xv_query_extension_unchecked (c : *mut connection) -> query_extension_cookie;

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
pub fn xcb_xv_query_extension_reply (c : *mut connection,
                                        cookie : query_extension_cookie,
                                        e : *mut *mut generic_error) -> *mut query_extension_reply;

pub fn xcb_xv_query_adaptors_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xv_query_adaptors (c : *mut connection,
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
pub fn xcb_xv_query_adaptors_unchecked (c : *mut connection,
                                           window :  ffi::xproto::window) -> query_adaptors_cookie;


pub fn xcb_xv_query_adaptors_info_length (R : *mut query_adaptors_reply) -> c_int;

pub fn xcb_xv_query_adaptors_info_iterator (R : *mut query_adaptors_reply) -> adaptor_info_iterator;

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
pub fn xcb_xv_query_adaptors_reply (c : *mut connection,
                                       cookie : query_adaptors_cookie,
                                       e : *mut *mut generic_error) -> *mut query_adaptors_reply;

pub fn xcb_xv_query_encodings_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xv_query_encodings (c : *mut connection,
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
pub fn xcb_xv_query_encodings_unchecked (c : *mut connection,
                                            port :  port) -> query_encodings_cookie;


pub fn xcb_xv_query_encodings_info_length (R : *mut query_encodings_reply) -> c_int;

pub fn xcb_xv_query_encodings_info_iterator (R : *mut query_encodings_reply) -> encoding_info_iterator;

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
pub fn xcb_xv_query_encodings_reply (c : *mut connection,
                                        cookie : query_encodings_cookie,
                                        e : *mut *mut generic_error) -> *mut query_encodings_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xv_grab_port (c : *mut connection,
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
pub fn xcb_xv_grab_port_unchecked (c : *mut connection,
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
pub fn xcb_xv_grab_port_reply (c : *mut connection,
                                  cookie : grab_port_cookie,
                                  e : *mut *mut generic_error) -> *mut grab_port_reply;

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
pub fn xcb_xv_ungrab_port_checked (c : *mut connection,
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
pub fn xcb_xv_ungrab_port (c : *mut connection,
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
pub fn xcb_xv_put_video_checked (c : *mut connection,
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
pub fn xcb_xv_put_video (c : *mut connection,
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
pub fn xcb_xv_put_still_checked (c : *mut connection,
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
pub fn xcb_xv_put_still (c : *mut connection,
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
pub fn xcb_xv_get_video_checked (c : *mut connection,
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
pub fn xcb_xv_get_video (c : *mut connection,
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
pub fn xcb_xv_get_still_checked (c : *mut connection,
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
pub fn xcb_xv_get_still (c : *mut connection,
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
pub fn xcb_xv_stop_video_checked (c : *mut connection,
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
pub fn xcb_xv_stop_video (c : *mut connection,
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
pub fn xcb_xv_select_video_notify_checked (c : *mut connection,
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
pub fn xcb_xv_select_video_notify (c : *mut connection,
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
pub fn xcb_xv_select_port_notify_checked (c : *mut connection,
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
pub fn xcb_xv_select_port_notify (c : *mut connection,
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
pub fn xcb_xv_query_best_size (c : *mut connection,
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
pub fn xcb_xv_query_best_size_unchecked (c : *mut connection,
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
pub fn xcb_xv_query_best_size_reply (c : *mut connection,
                                        cookie : query_best_size_cookie,
                                        e : *mut *mut generic_error) -> *mut query_best_size_reply;

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
pub fn xcb_xv_set_port_attribute_checked (c : *mut connection,
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
pub fn xcb_xv_set_port_attribute (c : *mut connection,
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
pub fn xcb_xv_get_port_attribute (c : *mut connection,
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
pub fn xcb_xv_get_port_attribute_unchecked (c : *mut connection,
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
pub fn xcb_xv_get_port_attribute_reply (c : *mut connection,
                                           cookie : get_port_attribute_cookie,
                                           e : *mut *mut generic_error) -> *mut get_port_attribute_reply;

pub fn xcb_xv_query_port_attributes_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xv_query_port_attributes (c : *mut connection,
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
pub fn xcb_xv_query_port_attributes_unchecked (c : *mut connection,
                                                  port :  port) -> query_port_attributes_cookie;


pub fn xcb_xv_query_port_attributes_attributes_length (R : *mut query_port_attributes_reply) -> c_int;

pub fn xcb_xv_query_port_attributes_attributes_iterator (R : *mut query_port_attributes_reply) -> attribute_info_iterator;

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
pub fn xcb_xv_query_port_attributes_reply (c : *mut connection,
                                              cookie : query_port_attributes_cookie,
                                              e : *mut *mut generic_error) -> *mut query_port_attributes_reply;

pub fn xcb_xv_list_image_formats_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xv_list_image_formats (c : *mut connection,
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
pub fn xcb_xv_list_image_formats_unchecked (c : *mut connection,
                                               port :  port) -> list_image_formats_cookie;

pub fn xcb_xv_list_image_formats_format (R : *mut list_image_formats_reply) -> *mut image_format_info;


pub fn xcb_xv_list_image_formats_format_length (R : *mut list_image_formats_reply) -> c_int;

pub fn xcb_xv_list_image_formats_format_iterator (R : *mut list_image_formats_reply) -> image_format_info_iterator;

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
pub fn xcb_xv_list_image_formats_reply (c : *mut connection,
                                           cookie : list_image_formats_cookie,
                                           e : *mut *mut generic_error) -> *mut list_image_formats_reply;

pub fn xcb_xv_query_image_attributes_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xv_query_image_attributes (c : *mut connection,
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
pub fn xcb_xv_query_image_attributes_unchecked (c : *mut connection,
                                                   port :  port,
                                                   id :  u32,
                                                   width :  u16,
                                                   height :  u16) -> query_image_attributes_cookie;

pub fn xcb_xv_query_image_attributes_pitches (R : *mut query_image_attributes_reply) -> *mut u32;


pub fn xcb_xv_query_image_attributes_pitches_length (R : *mut query_image_attributes_reply) -> c_int;


pub fn xcb_xv_query_image_attributes_pitches_end (R : *mut query_image_attributes_reply) -> generic_iterator;

pub fn xcb_xv_query_image_attributes_offsets (R : *mut query_image_attributes_reply) -> *mut u32;


pub fn xcb_xv_query_image_attributes_offsets_length (R : *mut query_image_attributes_reply) -> c_int;


pub fn xcb_xv_query_image_attributes_offsets_end (R : *mut query_image_attributes_reply) -> generic_iterator;

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
pub fn xcb_xv_query_image_attributes_reply (c : *mut connection,
                                               cookie : query_image_attributes_cookie,
                                               e : *mut *mut generic_error) -> *mut query_image_attributes_reply;

pub fn xcb_xv_put_image_sizeof (_buffer :  *mut c_void,
                         data_len :     u32) -> c_int;

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
pub fn xcb_xv_put_image_checked (c : *mut connection,
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
                                    data : *mut u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xv_put_image (c : *mut connection,
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
                            data : *mut u8) -> void_cookie;

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
pub fn xcb_xv_shm_put_image_checked (c : *mut connection,
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
pub fn xcb_xv_shm_put_image (c : *mut connection,
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

