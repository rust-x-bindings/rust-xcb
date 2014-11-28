/*
 * This file generated automatically from xv.xml by r_client.py.
 * Edit at your peril.
 */

//Make the compiler quiet
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use std;
use libc::*;
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
    pub data : *mut port,
    pub rem  : c_int,
    pub index: c_int
}


pub type encoding = u32;
/**
 * @brief encoding_iterator
 **/
pub struct encoding_iterator {
    pub data : *mut encoding,
    pub rem  : c_int,
    pub index: c_int
}


pub struct rational {
     pub numerator :     i32,
     pub denominator :   i32
}

/**
 * @brief rational_iterator
 **/
pub struct rational_iterator {
    pub data : *mut rational,
    pub rem  : c_int,
    pub index: c_int
}


pub struct format {
     pub visual :   ffi::xproto::visualid,
     pub depth :    u8,
     pub pad0 :     [u8,..3]
}

/**
 * @brief format_iterator
 **/
pub struct format_iterator {
    pub data : *mut format,
    pub rem  : c_int,
    pub index: c_int
}


pub struct adaptor_info {
     pub base_id :       port,
     pub name_size :     u16,
     pub num_ports :     u16,
     pub num_formats :   u16,
     pub type_ :         u8,
     pub pad0 :          u8
}

/**
 * @brief adaptor_info_iterator
 **/
pub struct adaptor_info_iterator {
    pub data : *mut adaptor_info,
    pub rem  : c_int,
    pub index: c_int
}


pub struct encoding_info {
     pub encoding :    encoding,
     pub name_size :   u16,
     pub width :       u16,
     pub height :      u16,
     pub pad0 :        [u8,..2],
     pub rate :        rational
}

/**
 * @brief encoding_info_iterator
 **/
pub struct encoding_info_iterator {
    pub data : *mut encoding_info,
    pub rem  : c_int,
    pub index: c_int
}


pub struct image {
     pub id :           u32,
     pub width :        u16,
     pub height :       u16,
     pub data_size :    u32,
     pub num_planes :   u32
}

/**
 * @brief image_iterator
 **/
pub struct image_iterator {
    pub data : *mut image,
    pub rem  : c_int,
    pub index: c_int
}


pub struct attribute_info {
     pub flags :   u32,
     pub min :     i32,
     pub max :     i32,
     pub size :    u32
}

/**
 * @brief attribute_info_iterator
 **/
pub struct attribute_info_iterator {
    pub data : *mut attribute_info,
    pub rem  : c_int,
    pub index: c_int
}


pub struct image_format_info {
     pub id :                u32,
     pub type_ :             u8,
     pub byte_order :        u8,
     pub pad0 :              [u8,..2],
     pub guid :              [u8,..16],
     pub bpp :               u8,
     pub num_planes :        u8,
     pub pad1 :              [u8,..2],
     pub depth :             u8,
     pub pad2 :              [u8,..3],
     pub red_mask :          u32,
     pub green_mask :        u32,
     pub blue_mask :         u32,
     pub format :            u8,
     pub pad3 :              [u8,..3],
     pub y_sample_bits :     u32,
     pub u_sample_bits :     u32,
     pub v_sample_bits :     u32,
     pub vhorz_y_period :    u32,
     pub vhorz_u_period :    u32,
     pub vhorz_v_period :    u32,
     pub vvert_y_period :    u32,
     pub vvert_u_period :    u32,
     pub vvert_v_period :    u32,
     pub vcomp_order :       [u8,..32],
     pub vscanline_order :   u8,
     pub pad4 :              [u8,..11]
}

/**
 * @brief image_format_info_iterator
 **/
pub struct image_format_info_iterator {
    pub data : *mut image_format_info,
    pub rem  : c_int,
    pub index: c_int
}



pub struct bad_port_error {
     pub response_type :   u8,
     pub error_code :      u8,
     pub sequence :        u16
}



pub struct bad_encoding_error {
     pub response_type :   u8,
     pub error_code :      u8,
     pub sequence :        u16
}



pub struct bad_control_error {
     pub response_type :   u8,
     pub error_code :      u8,
     pub sequence :        u16
}



pub struct video_notify_event {
     pub response_type :   u8,
     pub reason :          u8,
     pub sequence :        u16,
     pub time :            ffi::xproto::timestamp,
     pub drawable :        ffi::xproto::drawable,
     pub port :            port
}



pub struct port_notify_event {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub time :            ffi::xproto::timestamp,
     pub port :            port,
     pub attribute :       ffi::xproto::atom,
     pub value :           i32
}


pub struct query_extension_cookie {
    sequence : c_uint
}


pub struct query_extension_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16
}


pub struct query_extension_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub major :           u16,
     pub minor :           u16
}


pub struct query_adaptors_cookie {
    sequence : c_uint
}


pub struct query_adaptors_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub window :         ffi::xproto::window
}


pub struct query_adaptors_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub num_adaptors :    u16,
     pub pad1 :            [u8,..22]
}


pub struct query_encodings_cookie {
    sequence : c_uint
}


pub struct query_encodings_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub port :           port
}


pub struct query_encodings_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub num_encodings :   u16,
     pub pad1 :            [u8,..22]
}


pub struct grab_port_cookie {
    sequence : c_uint
}


pub struct grab_port_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub port :           port,
     pub time :           ffi::xproto::timestamp
}


pub struct grab_port_reply {
     pub response_type :   u8,
     pub result :          u8,
     pub sequence :        u16,
     pub length :          u32
}



pub struct ungrab_port_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub port :           port,
     pub time :           ffi::xproto::timestamp
}



pub struct put_video_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub port :           port,
     pub drawable :       ffi::xproto::drawable,
     pub gc :             ffi::xproto::gcontext,
     pub vid_x :          i16,
     pub vid_y :          i16,
     pub vid_w :          u16,
     pub vid_h :          u16,
     pub drw_x :          i16,
     pub drw_y :          i16,
     pub drw_w :          u16,
     pub drw_h :          u16
}



pub struct put_still_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub port :           port,
     pub drawable :       ffi::xproto::drawable,
     pub gc :             ffi::xproto::gcontext,
     pub vid_x :          i16,
     pub vid_y :          i16,
     pub vid_w :          u16,
     pub vid_h :          u16,
     pub drw_x :          i16,
     pub drw_y :          i16,
     pub drw_w :          u16,
     pub drw_h :          u16
}



pub struct get_video_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub port :           port,
     pub drawable :       ffi::xproto::drawable,
     pub gc :             ffi::xproto::gcontext,
     pub vid_x :          i16,
     pub vid_y :          i16,
     pub vid_w :          u16,
     pub vid_h :          u16,
     pub drw_x :          i16,
     pub drw_y :          i16,
     pub drw_w :          u16,
     pub drw_h :          u16
}



pub struct get_still_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub port :           port,
     pub drawable :       ffi::xproto::drawable,
     pub gc :             ffi::xproto::gcontext,
     pub vid_x :          i16,
     pub vid_y :          i16,
     pub vid_w :          u16,
     pub vid_h :          u16,
     pub drw_x :          i16,
     pub drw_y :          i16,
     pub drw_w :          u16,
     pub drw_h :          u16
}



pub struct stop_video_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub port :           port,
     pub drawable :       ffi::xproto::drawable
}



pub struct select_video_notify_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub drawable :       ffi::xproto::drawable,
     pub onoff :          u8,
     pub pad0 :           [u8,..3]
}



pub struct select_port_notify_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub port :           port,
     pub onoff :          u8,
     pub pad0 :           [u8,..3]
}


pub struct query_best_size_cookie {
    sequence : c_uint
}


pub struct query_best_size_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub port :           port,
     pub vid_w :          u16,
     pub vid_h :          u16,
     pub drw_w :          u16,
     pub drw_h :          u16,
     pub motion :         u8,
     pub pad0 :           [u8,..3]
}


pub struct query_best_size_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub actual_width :    u16,
     pub actual_height :   u16
}



pub struct set_port_attribute_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub port :           port,
     pub attribute :      ffi::xproto::atom,
     pub value :          i32
}


pub struct get_port_attribute_cookie {
    sequence : c_uint
}


pub struct get_port_attribute_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub port :           port,
     pub attribute :      ffi::xproto::atom
}


pub struct get_port_attribute_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub value :           i32
}


pub struct query_port_attributes_cookie {
    sequence : c_uint
}


pub struct query_port_attributes_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub port :           port
}


pub struct query_port_attributes_reply {
     pub response_type :    u8,
     pub pad0 :             u8,
     pub sequence :         u16,
     pub length :           u32,
     pub num_attributes :   u32,
     pub text_size :        u32,
     pub pad1 :             [u8,..16]
}


pub struct list_image_formats_cookie {
    sequence : c_uint
}


pub struct list_image_formats_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub port :           port
}


pub struct list_image_formats_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub num_formats :     u32,
     pub pad1 :            [u8,..20]
}


pub struct query_image_attributes_cookie {
    sequence : c_uint
}


pub struct query_image_attributes_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub port :           port,
     pub id :             u32,
     pub width :          u16,
     pub height :         u16
}


pub struct query_image_attributes_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub num_planes :      u32,
     pub data_size :       u32,
     pub width :           u16,
     pub height :          u16,
     pub pad1 :            [u8,..12]
}



pub struct put_image_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub port :           port,
     pub drawable :       ffi::xproto::drawable,
     pub gc :             ffi::xproto::gcontext,
     pub id :             u32,
     pub src_x :          i16,
     pub src_y :          i16,
     pub src_w :          u16,
     pub src_h :          u16,
     pub drw_x :          i16,
     pub drw_y :          i16,
     pub drw_w :          u16,
     pub drw_h :          u16,
     pub width :          u16,
     pub height :         u16
}



pub struct shm_put_image_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub port :           port,
     pub drawable :       ffi::xproto::drawable,
     pub gc :             ffi::xproto::gcontext,
     pub shmseg :         ffi::shm::seg,
     pub id :             u32,
     pub offset :         u32,
     pub src_x :          i16,
     pub src_y :          i16,
     pub src_w :          u16,
     pub src_h :          u16,
     pub drw_x :          i16,
     pub drw_y :          i16,
     pub drw_w :          u16,
     pub drw_h :          u16,
     pub width :          u16,
     pub height :         u16,
     pub send_event :     u8,
     pub pad0 :           [u8,..3]
}

#[link(name="xcb-xv")]
extern "C" {

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
pub fn xcb_xv_port_end (i:port_iterator) -> ffi::base::generic_iterator;

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
pub fn xcb_xv_encoding_end (i:encoding_iterator) -> ffi::base::generic_iterator;

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
pub fn xcb_xv_rational_end (i:rational_iterator) -> ffi::base::generic_iterator;

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
pub fn xcb_xv_format_end (i:format_iterator) -> ffi::base::generic_iterator;

pub fn xcb_xv_adaptor_info_sizeof (_buffer :  *mut c_void) -> c_int;

pub fn xcb_xv_adaptor_info_name (R : *mut adaptor_info) -> *mut c_char;


pub fn xcb_xv_adaptor_info_name_length (R : *mut adaptor_info) -> c_int;


pub fn xcb_xv_adaptor_info_name_end (R : *mut adaptor_info) -> ffi::base::generic_iterator;

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
pub fn xcb_xv_adaptor_info_end (i:adaptor_info_iterator) -> ffi::base::generic_iterator;

pub fn xcb_xv_encoding_info_sizeof (_buffer :  *mut c_void) -> c_int;

pub fn xcb_xv_encoding_info_name (R : *mut encoding_info) -> *mut c_char;


pub fn xcb_xv_encoding_info_name_length (R : *mut encoding_info) -> c_int;


pub fn xcb_xv_encoding_info_name_end (R : *mut encoding_info) -> ffi::base::generic_iterator;

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
pub fn xcb_xv_encoding_info_end (i:encoding_info_iterator) -> ffi::base::generic_iterator;

pub fn xcb_xv_image_sizeof (_buffer :  *mut c_void) -> c_int;

pub fn xcb_xv_image_pitches (R : *mut image) -> *mut u32;


pub fn xcb_xv_image_pitches_length (R : *mut image) -> c_int;


pub fn xcb_xv_image_pitches_end (R : *mut image) -> ffi::base::generic_iterator;

pub fn xcb_xv_image_offsets (R : *mut image) -> *mut u32;


pub fn xcb_xv_image_offsets_length (R : *mut image) -> c_int;


pub fn xcb_xv_image_offsets_end (R : *mut image) -> ffi::base::generic_iterator;

pub fn xcb_xv_image_data (R : *mut image) -> *mut u8;


pub fn xcb_xv_image_data_length (R : *mut image) -> c_int;


pub fn xcb_xv_image_data_end (R : *mut image) -> ffi::base::generic_iterator;

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
pub fn xcb_xv_image_end (i:image_iterator) -> ffi::base::generic_iterator;

pub fn xcb_xv_attribute_info_sizeof (_buffer :  *mut c_void) -> c_int;

pub fn xcb_xv_attribute_info_name (R : *mut attribute_info) -> *mut c_char;


pub fn xcb_xv_attribute_info_name_length (R : *mut attribute_info) -> c_int;


pub fn xcb_xv_attribute_info_name_end (R : *mut attribute_info) -> ffi::base::generic_iterator;

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
pub fn xcb_xv_attribute_info_end (i:attribute_info_iterator) -> ffi::base::generic_iterator;

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
pub fn xcb_xv_image_format_info_end (i:image_format_info_iterator) -> ffi::base::generic_iterator;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xv_query_extension (c : *mut ffi::base::connection) -> query_extension_cookie;

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
pub fn xcb_xv_query_extension_unchecked (c : *mut ffi::base::connection) -> query_extension_cookie;

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
pub fn xcb_xv_query_extension_reply (c : *mut ffi::base::connection,
                                        cookie : query_extension_cookie,
                                        e : *mut *mut ffi::base::generic_error) -> *mut query_extension_reply;

pub fn xcb_xv_query_adaptors_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xv_query_adaptors (c : *mut ffi::base::connection,
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
pub fn xcb_xv_query_adaptors_unchecked (c : *mut ffi::base::connection,
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
pub fn xcb_xv_query_adaptors_reply (c : *mut ffi::base::connection,
                                       cookie : query_adaptors_cookie,
                                       e : *mut *mut ffi::base::generic_error) -> *mut query_adaptors_reply;

pub fn xcb_xv_query_encodings_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xv_query_encodings (c : *mut ffi::base::connection,
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
pub fn xcb_xv_query_encodings_unchecked (c : *mut ffi::base::connection,
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
pub fn xcb_xv_query_encodings_reply (c : *mut ffi::base::connection,
                                        cookie : query_encodings_cookie,
                                        e : *mut *mut ffi::base::generic_error) -> *mut query_encodings_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xv_grab_port (c : *mut ffi::base::connection,
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
pub fn xcb_xv_grab_port_unchecked (c : *mut ffi::base::connection,
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
pub fn xcb_xv_grab_port_reply (c : *mut ffi::base::connection,
                                  cookie : grab_port_cookie,
                                  e : *mut *mut ffi::base::generic_error) -> *mut grab_port_reply;

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
pub fn xcb_xv_ungrab_port_checked (c : *mut ffi::base::connection,
                                      port :  port,
                                      time :  ffi::xproto::timestamp) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xv_ungrab_port (c : *mut ffi::base::connection,
                              port :  port,
                              time :  ffi::xproto::timestamp) -> ffi::base::void_cookie;

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
pub fn xcb_xv_put_video_checked (c : *mut ffi::base::connection,
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
                                    drw_h :  u16) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xv_put_video (c : *mut ffi::base::connection,
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
                            drw_h :  u16) -> ffi::base::void_cookie;

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
pub fn xcb_xv_put_still_checked (c : *mut ffi::base::connection,
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
                                    drw_h :  u16) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xv_put_still (c : *mut ffi::base::connection,
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
                            drw_h :  u16) -> ffi::base::void_cookie;

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
pub fn xcb_xv_get_video_checked (c : *mut ffi::base::connection,
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
                                    drw_h :  u16) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xv_get_video (c : *mut ffi::base::connection,
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
                            drw_h :  u16) -> ffi::base::void_cookie;

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
pub fn xcb_xv_get_still_checked (c : *mut ffi::base::connection,
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
                                    drw_h :  u16) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xv_get_still (c : *mut ffi::base::connection,
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
                            drw_h :  u16) -> ffi::base::void_cookie;

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
pub fn xcb_xv_stop_video_checked (c : *mut ffi::base::connection,
                                     port :  port,
                                     drawable :  ffi::xproto::drawable) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xv_stop_video (c : *mut ffi::base::connection,
                             port :  port,
                             drawable :  ffi::xproto::drawable) -> ffi::base::void_cookie;

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
pub fn xcb_xv_select_video_notify_checked (c : *mut ffi::base::connection,
                                              drawable :  ffi::xproto::drawable,
                                              onoff :  u8) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xv_select_video_notify (c : *mut ffi::base::connection,
                                      drawable :  ffi::xproto::drawable,
                                      onoff :  u8) -> ffi::base::void_cookie;

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
pub fn xcb_xv_select_port_notify_checked (c : *mut ffi::base::connection,
                                             port :  port,
                                             onoff :  u8) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xv_select_port_notify (c : *mut ffi::base::connection,
                                     port :  port,
                                     onoff :  u8) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xv_query_best_size (c : *mut ffi::base::connection,
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
pub fn xcb_xv_query_best_size_unchecked (c : *mut ffi::base::connection,
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
pub fn xcb_xv_query_best_size_reply (c : *mut ffi::base::connection,
                                        cookie : query_best_size_cookie,
                                        e : *mut *mut ffi::base::generic_error) -> *mut query_best_size_reply;

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
pub fn xcb_xv_set_port_attribute_checked (c : *mut ffi::base::connection,
                                             port :  port,
                                             attribute :  ffi::xproto::atom,
                                             value :  i32) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xv_set_port_attribute (c : *mut ffi::base::connection,
                                     port :  port,
                                     attribute :  ffi::xproto::atom,
                                     value :  i32) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xv_get_port_attribute (c : *mut ffi::base::connection,
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
pub fn xcb_xv_get_port_attribute_unchecked (c : *mut ffi::base::connection,
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
pub fn xcb_xv_get_port_attribute_reply (c : *mut ffi::base::connection,
                                           cookie : get_port_attribute_cookie,
                                           e : *mut *mut ffi::base::generic_error) -> *mut get_port_attribute_reply;

pub fn xcb_xv_query_port_attributes_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xv_query_port_attributes (c : *mut ffi::base::connection,
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
pub fn xcb_xv_query_port_attributes_unchecked (c : *mut ffi::base::connection,
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
pub fn xcb_xv_query_port_attributes_reply (c : *mut ffi::base::connection,
                                              cookie : query_port_attributes_cookie,
                                              e : *mut *mut ffi::base::generic_error) -> *mut query_port_attributes_reply;

pub fn xcb_xv_list_image_formats_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xv_list_image_formats (c : *mut ffi::base::connection,
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
pub fn xcb_xv_list_image_formats_unchecked (c : *mut ffi::base::connection,
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
pub fn xcb_xv_list_image_formats_reply (c : *mut ffi::base::connection,
                                           cookie : list_image_formats_cookie,
                                           e : *mut *mut ffi::base::generic_error) -> *mut list_image_formats_reply;

pub fn xcb_xv_query_image_attributes_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xv_query_image_attributes (c : *mut ffi::base::connection,
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
pub fn xcb_xv_query_image_attributes_unchecked (c : *mut ffi::base::connection,
                                                   port :  port,
                                                   id :  u32,
                                                   width :  u16,
                                                   height :  u16) -> query_image_attributes_cookie;

pub fn xcb_xv_query_image_attributes_pitches (R : *mut query_image_attributes_reply) -> *mut u32;


pub fn xcb_xv_query_image_attributes_pitches_length (R : *mut query_image_attributes_reply) -> c_int;


pub fn xcb_xv_query_image_attributes_pitches_end (R : *mut query_image_attributes_reply) -> ffi::base::generic_iterator;

pub fn xcb_xv_query_image_attributes_offsets (R : *mut query_image_attributes_reply) -> *mut u32;


pub fn xcb_xv_query_image_attributes_offsets_length (R : *mut query_image_attributes_reply) -> c_int;


pub fn xcb_xv_query_image_attributes_offsets_end (R : *mut query_image_attributes_reply) -> ffi::base::generic_iterator;

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
pub fn xcb_xv_query_image_attributes_reply (c : *mut ffi::base::connection,
                                               cookie : query_image_attributes_cookie,
                                               e : *mut *mut ffi::base::generic_error) -> *mut query_image_attributes_reply;

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
pub fn xcb_xv_put_image_checked (c : *mut ffi::base::connection,
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
                                    data : *mut u8) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xv_put_image (c : *mut ffi::base::connection,
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
                            data : *mut u8) -> ffi::base::void_cookie;

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
pub fn xcb_xv_shm_put_image_checked (c : *mut ffi::base::connection,
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
                                        send_event :  u8) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xv_shm_put_image (c : *mut ffi::base::connection,
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
                                send_event :  u8) -> ffi::base::void_cookie;
}

