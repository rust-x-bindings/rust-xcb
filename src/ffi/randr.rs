/*
 * This file generated automatically from randr.xml by r_client.py.
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
use ffi::render;

pub static RANDR_MAJOR_VERSION : c_uint = 1;
pub static RANDR_MINOR_VERSION : c_uint = 3;

pub type mode = u32;
/**
 * @brief mode_iterator
 **/
pub struct mode_iterator {
    data : *mode,
    rem  : c_int,
    index: c_int
}


pub type crtc = u32;
/**
 * @brief crtc_iterator
 **/
pub struct crtc_iterator {
    data : *crtc,
    rem  : c_int,
    index: c_int
}


pub type output = u32;
/**
 * @brief output_iterator
 **/
pub struct output_iterator {
    data : *output,
    rem  : c_int,
    index: c_int
}



pub struct bad_output_error {
    response_type :   u8,
    error_code :      u8,
    sequence :        u16
}



pub struct bad_crtc_error {
    response_type :   u8,
    error_code :      u8,
    sequence :        u16
}



pub struct bad_mode_error {
    response_type :   u8,
    error_code :      u8,
    sequence :        u16
}


pub struct screen_size {
    width :     u16,
    height :    u16,
    mwidth :    u16,
    mheight :   u16
}

/**
 * @brief screen_size_iterator
 **/
pub struct screen_size_iterator {
    data : *screen_size,
    rem  : c_int,
    index: c_int
}


pub struct refresh_rates {
    nRates :   u16
}

/**
 * @brief refresh_rates_iterator
 **/
pub struct refresh_rates_iterator {
    data : *refresh_rates,
    rem  : c_int,
    index: c_int
}


pub struct query_version_cookie {
    sequence : c_uint
}


pub struct query_version_request {
    major_opcode :    u8,
    minor_opcode :    u8,
    length :          u16,
    major_version :   u32,
    minor_version :   u32
}


pub struct query_version_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    major_version :   u32,
    minor_version :   u32,
    pad1 :            [u8,..16]
}


pub struct set_screen_config_cookie {
    sequence : c_uint
}


pub struct set_screen_config_request {
    major_opcode :       u8,
    minor_opcode :       u8,
    length :             u16,
    window :             ffi::xproto::window,
    timestamp :          ffi::xproto::timestamp,
    config_timestamp :   ffi::xproto::timestamp,
    sizeID :             u16,
    rotation :           u16,
    rate :               u16,
    pad0 :               [u8,..2]
}


pub struct set_screen_config_reply {
    response_type :      u8,
    status :             u8,
    sequence :           u16,
    length :             u32,
    new_timestamp :      ffi::xproto::timestamp,
    config_timestamp :   ffi::xproto::timestamp,
    root :               ffi::xproto::window,
    subpixel_order :     u16,
    pad0 :               [u8,..10]
}



pub struct select_input_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    window :         ffi::xproto::window,
    enable :         u16,
    pad0 :           [u8,..2]
}


pub struct get_screen_info_cookie {
    sequence : c_uint
}


pub struct get_screen_info_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    window :         ffi::xproto::window
}


pub struct get_screen_info_reply {
    response_type :      u8,
    rotations :          u8,
    sequence :           u16,
    length :             u32,
    root :               ffi::xproto::window,
    timestamp :          ffi::xproto::timestamp,
    config_timestamp :   ffi::xproto::timestamp,
    nSizes :             u16,
    sizeID :             u16,
    rotation :           u16,
    rate :               u16,
    nInfo :              u16,
    pad0 :               [u8,..2]
}


pub struct get_screen_size_range_cookie {
    sequence : c_uint
}


pub struct get_screen_size_range_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    window :         ffi::xproto::window
}


pub struct get_screen_size_range_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    min_width :       u16,
    min_height :      u16,
    max_width :       u16,
    max_height :      u16,
    pad1 :            [u8,..16]
}



pub struct set_screen_size_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    window :         ffi::xproto::window,
    width :          u16,
    height :         u16,
    mm_width :       u32,
    mm_height :      u32
}


pub struct mode_info {
    id :            u32,
    width :         u16,
    height :        u16,
    dot_clock :     u32,
    hsync_start :   u16,
    hsync_end :     u16,
    htotal :        u16,
    hskew :         u16,
    vsync_start :   u16,
    vsync_end :     u16,
    vtotal :        u16,
    name_len :      u16,
    mode_flags :    u32
}

/**
 * @brief mode_info_iterator
 **/
pub struct mode_info_iterator {
    data : *mode_info,
    rem  : c_int,
    index: c_int
}


pub struct get_screen_resources_cookie {
    sequence : c_uint
}


pub struct get_screen_resources_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    window :         ffi::xproto::window
}


pub struct get_screen_resources_reply {
    response_type :      u8,
    pad0 :               u8,
    sequence :           u16,
    length :             u32,
    timestamp :          ffi::xproto::timestamp,
    config_timestamp :   ffi::xproto::timestamp,
    num_crtcs :          u16,
    num_outputs :        u16,
    num_modes :          u16,
    names_len :          u16,
    pad1 :               [u8,..8]
}


pub struct get_output_info_cookie {
    sequence : c_uint
}


pub struct get_output_info_request {
    major_opcode :       u8,
    minor_opcode :       u8,
    length :             u16,
    output :             output,
    config_timestamp :   ffi::xproto::timestamp
}


pub struct get_output_info_reply {
    response_type :    u8,
    status :           u8,
    sequence :         u16,
    length :           u32,
    timestamp :        ffi::xproto::timestamp,
    crtc :             crtc,
    mm_width :         u32,
    mm_height :        u32,
    connection :       u8,
    subpixel_order :   u8,
    num_crtcs :        u16,
    num_modes :        u16,
    num_preferred :    u16,
    num_clones :       u16,
    name_len :         u16
}


pub struct list_output_properties_cookie {
    sequence : c_uint
}


pub struct list_output_properties_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    output :         output
}


pub struct list_output_properties_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    num_atoms :       u16,
    pad1 :            [u8,..22]
}


pub struct query_output_property_cookie {
    sequence : c_uint
}


pub struct query_output_property_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    output :         output,
    property :       ffi::xproto::atom
}


pub struct query_output_property_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pending :         u8,
    range :           u8,
    immutable :       u8,
    pad1 :            [u8,..21]
}



pub struct configure_output_property_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    output :         output,
    property :       ffi::xproto::atom,
    pending :        u8,
    range :          u8,
    pad0 :           [u8,..2]
}



pub struct change_output_property_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    output :         output,
    property :       ffi::xproto::atom,
    type_ :          ffi::xproto::atom,
    format :         u8,
    mode :           u8,
    pad0 :           [u8,..2],
    num_units :      u32
}



pub struct delete_output_property_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    output :         output,
    property :       ffi::xproto::atom
}


pub struct get_output_property_cookie {
    sequence : c_uint
}


pub struct get_output_property_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    output :         output,
    property :       ffi::xproto::atom,
    type_ :          ffi::xproto::atom,
    long_offset :    u32,
    long_length :    u32,
    delete :         u8,
    pending :        u8,
    pad0 :           [u8,..2]
}


pub struct get_output_property_reply {
    response_type :   u8,
    format :          u8,
    sequence :        u16,
    length :          u32,
    type_ :           ffi::xproto::atom,
    bytes_after :     u32,
    num_items :       u32,
    pad0 :            [u8,..12]
}


pub struct create_mode_cookie {
    sequence : c_uint
}


pub struct create_mode_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    window :         ffi::xproto::window,
    mode_info :      mode_info
}


pub struct create_mode_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    mode :            mode,
    pad1 :            [u8,..20]
}



pub struct destroy_mode_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    mode :           mode
}



pub struct add_output_mode_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    output :         output,
    mode :           mode
}



pub struct delete_output_mode_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    output :         output,
    mode :           mode
}


pub struct get_crtc_info_cookie {
    sequence : c_uint
}


pub struct get_crtc_info_request {
    major_opcode :       u8,
    minor_opcode :       u8,
    length :             u16,
    crtc :               crtc,
    config_timestamp :   ffi::xproto::timestamp
}


pub struct get_crtc_info_reply {
    response_type :          u8,
    status :                 u8,
    sequence :               u16,
    length :                 u32,
    timestamp :              ffi::xproto::timestamp,
    x :                      i16,
    y :                      i16,
    width :                  u16,
    height :                 u16,
    mode :                   mode,
    rotation :               u16,
    rotations :              u16,
    num_outputs :            u16,
    num_possible_outputs :   u16
}


pub struct set_crtc_config_cookie {
    sequence : c_uint
}


pub struct set_crtc_config_request {
    major_opcode :       u8,
    minor_opcode :       u8,
    length :             u16,
    crtc :               crtc,
    timestamp :          ffi::xproto::timestamp,
    config_timestamp :   ffi::xproto::timestamp,
    x :                  i16,
    y :                  i16,
    mode :               mode,
    rotation :           u16,
    pad0 :               [u8,..2]
}


pub struct set_crtc_config_reply {
    response_type :   u8,
    status :          u8,
    sequence :        u16,
    length :          u32,
    timestamp :       ffi::xproto::timestamp,
    pad0 :            [u8,..20]
}


pub struct get_crtc_gamma_size_cookie {
    sequence : c_uint
}


pub struct get_crtc_gamma_size_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    crtc :           crtc
}


pub struct get_crtc_gamma_size_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    size :            u16,
    pad1 :            [u8,..22]
}


pub struct get_crtc_gamma_cookie {
    sequence : c_uint
}


pub struct get_crtc_gamma_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    crtc :           crtc
}


pub struct get_crtc_gamma_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    size :            u16,
    pad1 :            [u8,..22]
}



pub struct set_crtc_gamma_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    crtc :           crtc,
    size :           u16,
    pad0 :           [u8,..2]
}


pub struct get_screen_resources_current_cookie {
    sequence : c_uint
}


pub struct get_screen_resources_current_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    window :         ffi::xproto::window
}


pub struct get_screen_resources_current_reply {
    response_type :      u8,
    pad0 :               u8,
    sequence :           u16,
    length :             u32,
    timestamp :          ffi::xproto::timestamp,
    config_timestamp :   ffi::xproto::timestamp,
    num_crtcs :          u16,
    num_outputs :        u16,
    num_modes :          u16,
    names_len :          u16,
    pad1 :               [u8,..8]
}



pub struct set_crtc_transform_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    crtc :           crtc,
    transform :      ffi::render::transform,
    filter_len :     u16,
    pad0 :           [u8,..2]
}


pub struct get_crtc_transform_cookie {
    sequence : c_uint
}


pub struct get_crtc_transform_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    crtc :           crtc
}


pub struct get_crtc_transform_reply {
    response_type :       u8,
    pad0 :                u8,
    sequence :            u16,
    length :              u32,
    pending_transform :   ffi::render::transform,
    has_transforms :      u8,
    pad1 :                [u8,..3],
    current_transform :   ffi::render::transform,
    pad2 :                [u8,..4],
    pending_len :         u16,
    pending_nparams :     u16,
    current_len :         u16,
    current_nparams :     u16
}


pub struct get_panning_cookie {
    sequence : c_uint
}


pub struct get_panning_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    crtc :           crtc
}


pub struct get_panning_reply {
    response_type :   u8,
    status :          u8,
    sequence :        u16,
    length :          u32,
    timestamp :       ffi::xproto::timestamp,
    left :            u16,
    top :             u16,
    width :           u16,
    height :          u16,
    track_left :      u16,
    track_top :       u16,
    track_width :     u16,
    track_height :    u16,
    border_left :     i16,
    border_top :      i16,
    border_right :    i16,
    border_bottom :   i16
}


pub struct set_panning_cookie {
    sequence : c_uint
}


pub struct set_panning_request {
    major_opcode :    u8,
    minor_opcode :    u8,
    length :          u16,
    crtc :            crtc,
    timestamp :       ffi::xproto::timestamp,
    left :            u16,
    top :             u16,
    width :           u16,
    height :          u16,
    track_left :      u16,
    track_top :       u16,
    track_width :     u16,
    track_height :    u16,
    border_left :     i16,
    border_top :      i16,
    border_right :    i16,
    border_bottom :   i16
}


pub struct set_panning_reply {
    response_type :   u8,
    status :          u8,
    sequence :        u16,
    length :          u32,
    timestamp :       ffi::xproto::timestamp
}



pub struct set_output_primary_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    window :         ffi::xproto::window,
    output :         output
}


pub struct get_output_primary_cookie {
    sequence : c_uint
}


pub struct get_output_primary_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    window :         ffi::xproto::window
}


pub struct get_output_primary_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    output :          output
}



pub struct screen_change_notify_event {
    response_type :      u8,
    rotation :           u8,
    sequence :           u16,
    timestamp :          ffi::xproto::timestamp,
    config_timestamp :   ffi::xproto::timestamp,
    root :               ffi::xproto::window,
    request_window :     ffi::xproto::window,
    sizeID :             u16,
    subpixel_order :     u16,
    width :              u16,
    height :             u16,
    mwidth :             u16,
    mheight :            u16
}


pub struct crtc_change {
    timestamp :   ffi::xproto::timestamp,
    window :      ffi::xproto::window,
    crtc :        crtc,
    mode :        mode,
    rotation :    u16,
    pad0 :        [u8,..2],
    x :           i16,
    y :           i16,
    width :       u16,
    height :      u16
}

/**
 * @brief crtc_change_iterator
 **/
pub struct crtc_change_iterator {
    data : *crtc_change,
    rem  : c_int,
    index: c_int
}


pub struct output_change {
    timestamp :          ffi::xproto::timestamp,
    config_timestamp :   ffi::xproto::timestamp,
    window :             ffi::xproto::window,
    output :             output,
    crtc :               crtc,
    mode :               mode,
    rotation :           u16,
    connection :         u8,
    subpixel_order :     u8
}

/**
 * @brief output_change_iterator
 **/
pub struct output_change_iterator {
    data : *output_change,
    rem  : c_int,
    index: c_int
}


pub struct output_property {
    window :      ffi::xproto::window,
    output :      output,
    atom :        ffi::xproto::atom,
    timestamp :   ffi::xproto::timestamp,
    status :      u8,
    pad0 :        [u8,..11]
}

/**
 * @brief output_property_iterator
 **/
pub struct output_property_iterator {
    data : *output_property,
    rem  : c_int,
    index: c_int
}


pub struct notify_data {
    data : [u8,..28]
}
/**
 * @brief notify_data_iterator
 **/
pub struct notify_data_iterator {
    data : *notify_data,
    rem  : c_int,
    index: c_int
}



pub struct notify_event {
    response_type :   u8,
    subCode :         u8,
    sequence :        u16,
    u :               notify_data
}

#[link_args="-lxcb-randr"]
pub extern "C" {

/**
 * Get the next element of the iterator
 * @param i Pointer to a mode_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(mode)
 *
 *
 */
pub unsafe fn xcb_randr_mode_next (i:*mode_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An mode_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub unsafe fn xcb_randr_mode_end (i:mode_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a crtc_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(crtc)
 *
 *
 */
pub unsafe fn xcb_randr_crtc_next (i:*crtc_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An crtc_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub unsafe fn xcb_randr_crtc_end (i:crtc_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a output_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(output)
 *
 *
 */
pub unsafe fn xcb_randr_output_next (i:*output_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An output_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub unsafe fn xcb_randr_output_end (i:output_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a screen_size_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(screen_size)
 *
 *
 */
pub unsafe fn xcb_randr_screen_size_next (i:*screen_size_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An screen_size_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub unsafe fn xcb_randr_screen_size_end (i:screen_size_iterator) -> generic_iterator;

pub unsafe fn xcb_randr_refresh_rates_sizeof (_buffer :  *c_void) -> c_int;

pub unsafe fn xcb_randr_refresh_rates_rates (R : *refresh_rates) -> *u16;


pub unsafe fn xcb_randr_refresh_rates_rates_length (R : *refresh_rates) -> c_int;


pub unsafe fn xcb_randr_refresh_rates_rates_end (R : *refresh_rates) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a refresh_rates_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(refresh_rates)
 *
 *
 */
pub unsafe fn xcb_randr_refresh_rates_next (i:*refresh_rates_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An refresh_rates_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub unsafe fn xcb_randr_refresh_rates_end (i:refresh_rates_iterator) -> generic_iterator;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_randr_query_version (c : *connection,
                                   major_version :  u32,
                                   minor_version :  u32) -> query_version_cookie;

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
pub unsafe fn xcb_randr_query_version_unchecked (c : *connection,
                                             major_version :  u32,
                                             minor_version :  u32) -> query_version_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_randr_query_version_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub unsafe fn xcb_randr_query_version_reply (c : *connection,
                                         cookie : query_version_cookie,
                                         e : **generic_error) -> *query_version_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_randr_set_screen_config (c : *connection,
                                       window :  ffi::xproto::window,
                                       timestamp :  ffi::xproto::timestamp,
                                       config_timestamp :  ffi::xproto::timestamp,
                                       sizeID :  u16,
                                       rotation :  u16,
                                       rate :  u16) -> set_screen_config_cookie;

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
pub unsafe fn xcb_randr_set_screen_config_unchecked (c : *connection,
                                                 window :  ffi::xproto::window,
                                                 timestamp :  ffi::xproto::timestamp,
                                                 config_timestamp :  ffi::xproto::timestamp,
                                                 sizeID :  u16,
                                                 rotation :  u16,
                                                 rate :  u16) -> set_screen_config_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_randr_set_screen_config_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub unsafe fn xcb_randr_set_screen_config_reply (c : *connection,
                                             cookie : set_screen_config_cookie,
                                             e : **generic_error) -> *set_screen_config_reply;

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
pub unsafe fn xcb_randr_select_input_checked (c : *connection,
                                          window :  ffi::xproto::window,
                                          enable :  u16) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_randr_select_input (c : *connection,
                                  window :  ffi::xproto::window,
                                  enable :  u16) -> void_cookie;

pub unsafe fn xcb_randr_get_screen_info_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_randr_get_screen_info (c : *connection,
                                     window :  ffi::xproto::window) -> get_screen_info_cookie;

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
pub unsafe fn xcb_randr_get_screen_info_unchecked (c : *connection,
                                               window :  ffi::xproto::window) -> get_screen_info_cookie;

pub unsafe fn xcb_randr_get_screen_info_sizes (R : *get_screen_info_reply) -> *screen_size;


pub unsafe fn xcb_randr_get_screen_info_sizes_length (R : *get_screen_info_reply) -> c_int;

pub unsafe fn xcb_randr_get_screen_info_sizes_iterator (R : *get_screen_info_reply) -> screen_size_iterator;


pub unsafe fn xcb_randr_get_screen_info_rates_length (R : *get_screen_info_reply) -> c_int;

pub unsafe fn xcb_randr_get_screen_info_rates_iterator (R : *get_screen_info_reply) -> refresh_rates_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_randr_get_screen_info_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub unsafe fn xcb_randr_get_screen_info_reply (c : *connection,
                                           cookie : get_screen_info_cookie,
                                           e : **generic_error) -> *get_screen_info_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_randr_get_screen_size_range (c : *connection,
                                           window :  ffi::xproto::window) -> get_screen_size_range_cookie;

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
pub unsafe fn xcb_randr_get_screen_size_range_unchecked (c : *connection,
                                                     window :  ffi::xproto::window) -> get_screen_size_range_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_randr_get_screen_size_range_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub unsafe fn xcb_randr_get_screen_size_range_reply (c : *connection,
                                                 cookie : get_screen_size_range_cookie,
                                                 e : **generic_error) -> *get_screen_size_range_reply;

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
pub unsafe fn xcb_randr_set_screen_size_checked (c : *connection,
                                             window :  ffi::xproto::window,
                                             width :  u16,
                                             height :  u16,
                                             mm_width :  u32,
                                             mm_height :  u32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_randr_set_screen_size (c : *connection,
                                     window :  ffi::xproto::window,
                                     width :  u16,
                                     height :  u16,
                                     mm_width :  u32,
                                     mm_height :  u32) -> void_cookie;

/**
 * Get the next element of the iterator
 * @param i Pointer to a mode_info_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(mode_info)
 *
 *
 */
pub unsafe fn xcb_randr_mode_info_next (i:*mode_info_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An mode_info_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub unsafe fn xcb_randr_mode_info_end (i:mode_info_iterator) -> generic_iterator;

pub unsafe fn xcb_randr_get_screen_resources_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_randr_get_screen_resources (c : *connection,
                                          window :  ffi::xproto::window) -> get_screen_resources_cookie;

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
pub unsafe fn xcb_randr_get_screen_resources_unchecked (c : *connection,
                                                    window :  ffi::xproto::window) -> get_screen_resources_cookie;

pub unsafe fn xcb_randr_get_screen_resources_crtcs (R : *get_screen_resources_reply) -> *crtc;


pub unsafe fn xcb_randr_get_screen_resources_crtcs_length (R : *get_screen_resources_reply) -> c_int;


pub unsafe fn xcb_randr_get_screen_resources_crtcs_end (R : *get_screen_resources_reply) -> generic_iterator;

pub unsafe fn xcb_randr_get_screen_resources_outputs (R : *get_screen_resources_reply) -> *output;


pub unsafe fn xcb_randr_get_screen_resources_outputs_length (R : *get_screen_resources_reply) -> c_int;


pub unsafe fn xcb_randr_get_screen_resources_outputs_end (R : *get_screen_resources_reply) -> generic_iterator;

pub unsafe fn xcb_randr_get_screen_resources_modes (R : *get_screen_resources_reply) -> *mode_info;


pub unsafe fn xcb_randr_get_screen_resources_modes_length (R : *get_screen_resources_reply) -> c_int;

pub unsafe fn xcb_randr_get_screen_resources_modes_iterator (R : *get_screen_resources_reply) -> mode_info_iterator;

pub unsafe fn xcb_randr_get_screen_resources_names (R : *get_screen_resources_reply) -> *u8;


pub unsafe fn xcb_randr_get_screen_resources_names_length (R : *get_screen_resources_reply) -> c_int;


pub unsafe fn xcb_randr_get_screen_resources_names_end (R : *get_screen_resources_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_randr_get_screen_resources_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub unsafe fn xcb_randr_get_screen_resources_reply (c : *connection,
                                                cookie : get_screen_resources_cookie,
                                                e : **generic_error) -> *get_screen_resources_reply;

pub unsafe fn xcb_randr_get_output_info_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_randr_get_output_info (c : *connection,
                                     output :  output,
                                     config_timestamp :  ffi::xproto::timestamp) -> get_output_info_cookie;

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
pub unsafe fn xcb_randr_get_output_info_unchecked (c : *connection,
                                               output :  output,
                                               config_timestamp :  ffi::xproto::timestamp) -> get_output_info_cookie;

pub unsafe fn xcb_randr_get_output_info_crtcs (R : *get_output_info_reply) -> *crtc;


pub unsafe fn xcb_randr_get_output_info_crtcs_length (R : *get_output_info_reply) -> c_int;


pub unsafe fn xcb_randr_get_output_info_crtcs_end (R : *get_output_info_reply) -> generic_iterator;

pub unsafe fn xcb_randr_get_output_info_modes (R : *get_output_info_reply) -> *mode;


pub unsafe fn xcb_randr_get_output_info_modes_length (R : *get_output_info_reply) -> c_int;


pub unsafe fn xcb_randr_get_output_info_modes_end (R : *get_output_info_reply) -> generic_iterator;

pub unsafe fn xcb_randr_get_output_info_clones (R : *get_output_info_reply) -> *output;


pub unsafe fn xcb_randr_get_output_info_clones_length (R : *get_output_info_reply) -> c_int;


pub unsafe fn xcb_randr_get_output_info_clones_end (R : *get_output_info_reply) -> generic_iterator;

pub unsafe fn xcb_randr_get_output_info_name (R : *get_output_info_reply) -> *u8;


pub unsafe fn xcb_randr_get_output_info_name_length (R : *get_output_info_reply) -> c_int;


pub unsafe fn xcb_randr_get_output_info_name_end (R : *get_output_info_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_randr_get_output_info_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub unsafe fn xcb_randr_get_output_info_reply (c : *connection,
                                           cookie : get_output_info_cookie,
                                           e : **generic_error) -> *get_output_info_reply;

pub unsafe fn xcb_randr_list_output_properties_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_randr_list_output_properties (c : *connection,
                                            output :  output) -> list_output_properties_cookie;

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
pub unsafe fn xcb_randr_list_output_properties_unchecked (c : *connection,
                                                      output :  output) -> list_output_properties_cookie;

pub unsafe fn xcb_randr_list_output_properties_atoms (R : *list_output_properties_reply) -> *ffi::xproto::atom;


pub unsafe fn xcb_randr_list_output_properties_atoms_length (R : *list_output_properties_reply) -> c_int;


pub unsafe fn xcb_randr_list_output_properties_atoms_end (R : *list_output_properties_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_randr_list_output_properties_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub unsafe fn xcb_randr_list_output_properties_reply (c : *connection,
                                                  cookie : list_output_properties_cookie,
                                                  e : **generic_error) -> *list_output_properties_reply;

pub unsafe fn xcb_randr_query_output_property_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_randr_query_output_property (c : *connection,
                                           output :  output,
                                           property :  ffi::xproto::atom) -> query_output_property_cookie;

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
pub unsafe fn xcb_randr_query_output_property_unchecked (c : *connection,
                                                     output :  output,
                                                     property :  ffi::xproto::atom) -> query_output_property_cookie;

pub unsafe fn xcb_randr_query_output_property_valid_values (R : *query_output_property_reply) -> *i32;


pub unsafe fn xcb_randr_query_output_property_valid_values_length (R : *query_output_property_reply) -> c_int;


pub unsafe fn xcb_randr_query_output_property_valid_values_end (R : *query_output_property_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_randr_query_output_property_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub unsafe fn xcb_randr_query_output_property_reply (c : *connection,
                                                 cookie : query_output_property_cookie,
                                                 e : **generic_error) -> *query_output_property_reply;

pub unsafe fn xcb_randr_configure_output_property_sizeof (_buffer :  *c_void,
                                            values_len :  u32) -> c_int;

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
pub unsafe fn xcb_randr_configure_output_property_checked (c : *connection,
                                                       output :  output,
                                                       property :  ffi::xproto::atom,
                                                       pending :  u8,
                                                       range :  u8,
                                                       values_len :  u32,
                                                       values : *i32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_randr_configure_output_property (c : *connection,
                                               output :  output,
                                               property :  ffi::xproto::atom,
                                               pending :  u8,
                                               range :  u8,
                                               values_len :  u32,
                                               values : *i32) -> void_cookie;

pub unsafe fn xcb_randr_change_output_property_sizeof (_buffer :  *c_void) -> c_int;

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
pub unsafe fn xcb_randr_change_output_property_checked (c : *connection,
                                                    output :  output,
                                                    property :  ffi::xproto::atom,
                                                    type_ :  ffi::xproto::atom,
                                                    format :  u8,
                                                    mode :  u8,
                                                    num_units :  u32,
                                                    data : *c_void) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_randr_change_output_property (c : *connection,
                                            output :  output,
                                            property :  ffi::xproto::atom,
                                            type_ :  ffi::xproto::atom,
                                            format :  u8,
                                            mode :  u8,
                                            num_units :  u32,
                                            data : *c_void) -> void_cookie;

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
pub unsafe fn xcb_randr_delete_output_property_checked (c : *connection,
                                                    output :  output,
                                                    property :  ffi::xproto::atom) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_randr_delete_output_property (c : *connection,
                                            output :  output,
                                            property :  ffi::xproto::atom) -> void_cookie;

pub unsafe fn xcb_randr_get_output_property_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_randr_get_output_property (c : *connection,
                                         output :  output,
                                         property :  ffi::xproto::atom,
                                         type_ :  ffi::xproto::atom,
                                         long_offset :  u32,
                                         long_length :  u32,
                                         delete :  u8,
                                         pending :  u8) -> get_output_property_cookie;

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
pub unsafe fn xcb_randr_get_output_property_unchecked (c : *connection,
                                                   output :  output,
                                                   property :  ffi::xproto::atom,
                                                   type_ :  ffi::xproto::atom,
                                                   long_offset :  u32,
                                                   long_length :  u32,
                                                   delete :  u8,
                                                   pending :  u8) -> get_output_property_cookie;

pub unsafe fn xcb_randr_get_output_property_data (R : *get_output_property_reply) -> *u8;


pub unsafe fn xcb_randr_get_output_property_data_length (R : *get_output_property_reply) -> c_int;


pub unsafe fn xcb_randr_get_output_property_data_end (R : *get_output_property_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_randr_get_output_property_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub unsafe fn xcb_randr_get_output_property_reply (c : *connection,
                                               cookie : get_output_property_cookie,
                                               e : **generic_error) -> *get_output_property_reply;

pub unsafe fn xcb_randr_create_mode_sizeof (_buffer :  *c_void,
                              name_len :  u32) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_randr_create_mode (c : *connection,
                                 window :  ffi::xproto::window,
                                 mode_info :  mode_info,
                                 name_len :  u32,
                                 name : *c_char) -> create_mode_cookie;

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
pub unsafe fn xcb_randr_create_mode_unchecked (c : *connection,
                                           window :  ffi::xproto::window,
                                           mode_info :  mode_info,
                                           name_len :  u32,
                                           name : *c_char) -> create_mode_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_randr_create_mode_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub unsafe fn xcb_randr_create_mode_reply (c : *connection,
                                       cookie : create_mode_cookie,
                                       e : **generic_error) -> *create_mode_reply;

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
pub unsafe fn xcb_randr_destroy_mode_checked (c : *connection,
                                          mode :  mode) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_randr_destroy_mode (c : *connection,
                                  mode :  mode) -> void_cookie;

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
pub unsafe fn xcb_randr_add_output_mode_checked (c : *connection,
                                             output :  output,
                                             mode :  mode) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_randr_add_output_mode (c : *connection,
                                     output :  output,
                                     mode :  mode) -> void_cookie;

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
pub unsafe fn xcb_randr_delete_output_mode_checked (c : *connection,
                                                output :  output,
                                                mode :  mode) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_randr_delete_output_mode (c : *connection,
                                        output :  output,
                                        mode :  mode) -> void_cookie;

pub unsafe fn xcb_randr_get_crtc_info_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_randr_get_crtc_info (c : *connection,
                                   crtc :  crtc,
                                   config_timestamp :  ffi::xproto::timestamp) -> get_crtc_info_cookie;

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
pub unsafe fn xcb_randr_get_crtc_info_unchecked (c : *connection,
                                             crtc :  crtc,
                                             config_timestamp :  ffi::xproto::timestamp) -> get_crtc_info_cookie;

pub unsafe fn xcb_randr_get_crtc_info_outputs (R : *get_crtc_info_reply) -> *output;


pub unsafe fn xcb_randr_get_crtc_info_outputs_length (R : *get_crtc_info_reply) -> c_int;


pub unsafe fn xcb_randr_get_crtc_info_outputs_end (R : *get_crtc_info_reply) -> generic_iterator;

pub unsafe fn xcb_randr_get_crtc_info_possible (R : *get_crtc_info_reply) -> *output;


pub unsafe fn xcb_randr_get_crtc_info_possible_length (R : *get_crtc_info_reply) -> c_int;


pub unsafe fn xcb_randr_get_crtc_info_possible_end (R : *get_crtc_info_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_randr_get_crtc_info_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub unsafe fn xcb_randr_get_crtc_info_reply (c : *connection,
                                         cookie : get_crtc_info_cookie,
                                         e : **generic_error) -> *get_crtc_info_reply;

pub unsafe fn xcb_randr_set_crtc_config_sizeof (_buffer :  *c_void,
                                  outputs_len :  u32) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_randr_set_crtc_config (c : *connection,
                                     crtc :  crtc,
                                     timestamp :  ffi::xproto::timestamp,
                                     config_timestamp :  ffi::xproto::timestamp,
                                     x :  i16,
                                     y :  i16,
                                     mode :  mode,
                                     rotation :  u16,
                                     outputs_len :  u32,
                                     outputs : *output) -> set_crtc_config_cookie;

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
pub unsafe fn xcb_randr_set_crtc_config_unchecked (c : *connection,
                                               crtc :  crtc,
                                               timestamp :  ffi::xproto::timestamp,
                                               config_timestamp :  ffi::xproto::timestamp,
                                               x :  i16,
                                               y :  i16,
                                               mode :  mode,
                                               rotation :  u16,
                                               outputs_len :  u32,
                                               outputs : *output) -> set_crtc_config_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_randr_set_crtc_config_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub unsafe fn xcb_randr_set_crtc_config_reply (c : *connection,
                                           cookie : set_crtc_config_cookie,
                                           e : **generic_error) -> *set_crtc_config_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_randr_get_crtc_gamma_size (c : *connection,
                                         crtc :  crtc) -> get_crtc_gamma_size_cookie;

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
pub unsafe fn xcb_randr_get_crtc_gamma_size_unchecked (c : *connection,
                                                   crtc :  crtc) -> get_crtc_gamma_size_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_randr_get_crtc_gamma_size_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub unsafe fn xcb_randr_get_crtc_gamma_size_reply (c : *connection,
                                               cookie : get_crtc_gamma_size_cookie,
                                               e : **generic_error) -> *get_crtc_gamma_size_reply;

pub unsafe fn xcb_randr_get_crtc_gamma_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_randr_get_crtc_gamma (c : *connection,
                                    crtc :  crtc) -> get_crtc_gamma_cookie;

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
pub unsafe fn xcb_randr_get_crtc_gamma_unchecked (c : *connection,
                                              crtc :  crtc) -> get_crtc_gamma_cookie;

pub unsafe fn xcb_randr_get_crtc_gamma_red (R : *get_crtc_gamma_reply) -> *u16;


pub unsafe fn xcb_randr_get_crtc_gamma_red_length (R : *get_crtc_gamma_reply) -> c_int;


pub unsafe fn xcb_randr_get_crtc_gamma_red_end (R : *get_crtc_gamma_reply) -> generic_iterator;

pub unsafe fn xcb_randr_get_crtc_gamma_green (R : *get_crtc_gamma_reply) -> *u16;


pub unsafe fn xcb_randr_get_crtc_gamma_green_length (R : *get_crtc_gamma_reply) -> c_int;


pub unsafe fn xcb_randr_get_crtc_gamma_green_end (R : *get_crtc_gamma_reply) -> generic_iterator;

pub unsafe fn xcb_randr_get_crtc_gamma_blue (R : *get_crtc_gamma_reply) -> *u16;


pub unsafe fn xcb_randr_get_crtc_gamma_blue_length (R : *get_crtc_gamma_reply) -> c_int;


pub unsafe fn xcb_randr_get_crtc_gamma_blue_end (R : *get_crtc_gamma_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_randr_get_crtc_gamma_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub unsafe fn xcb_randr_get_crtc_gamma_reply (c : *connection,
                                          cookie : get_crtc_gamma_cookie,
                                          e : **generic_error) -> *get_crtc_gamma_reply;

pub unsafe fn xcb_randr_set_crtc_gamma_sizeof (_buffer :  *c_void) -> c_int;

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
pub unsafe fn xcb_randr_set_crtc_gamma_checked (c : *connection,
                                            crtc :  crtc,
                                            size :  u16,
                                            red : *u16,
                                            green : *u16,
                                            blue : *u16) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_randr_set_crtc_gamma (c : *connection,
                                    crtc :  crtc,
                                    size :  u16,
                                    red : *u16,
                                    green : *u16,
                                    blue : *u16) -> void_cookie;

pub unsafe fn xcb_randr_get_screen_resources_current_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_randr_get_screen_resources_current (c : *connection,
                                                  window :  ffi::xproto::window) -> get_screen_resources_current_cookie;

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
pub unsafe fn xcb_randr_get_screen_resources_current_unchecked (c : *connection,
                                                            window :  ffi::xproto::window) -> get_screen_resources_current_cookie;

pub unsafe fn xcb_randr_get_screen_resources_current_crtcs (R : *get_screen_resources_current_reply) -> *crtc;


pub unsafe fn xcb_randr_get_screen_resources_current_crtcs_length (R : *get_screen_resources_current_reply) -> c_int;


pub unsafe fn xcb_randr_get_screen_resources_current_crtcs_end (R : *get_screen_resources_current_reply) -> generic_iterator;

pub unsafe fn xcb_randr_get_screen_resources_current_outputs (R : *get_screen_resources_current_reply) -> *output;


pub unsafe fn xcb_randr_get_screen_resources_current_outputs_length (R : *get_screen_resources_current_reply) -> c_int;


pub unsafe fn xcb_randr_get_screen_resources_current_outputs_end (R : *get_screen_resources_current_reply) -> generic_iterator;

pub unsafe fn xcb_randr_get_screen_resources_current_modes (R : *get_screen_resources_current_reply) -> *mode_info;


pub unsafe fn xcb_randr_get_screen_resources_current_modes_length (R : *get_screen_resources_current_reply) -> c_int;

pub unsafe fn xcb_randr_get_screen_resources_current_modes_iterator (R : *get_screen_resources_current_reply) -> mode_info_iterator;

pub unsafe fn xcb_randr_get_screen_resources_current_names (R : *get_screen_resources_current_reply) -> *u8;


pub unsafe fn xcb_randr_get_screen_resources_current_names_length (R : *get_screen_resources_current_reply) -> c_int;


pub unsafe fn xcb_randr_get_screen_resources_current_names_end (R : *get_screen_resources_current_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_randr_get_screen_resources_current_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub unsafe fn xcb_randr_get_screen_resources_current_reply (c : *connection,
                                                        cookie : get_screen_resources_current_cookie,
                                                        e : **generic_error) -> *get_screen_resources_current_reply;

pub unsafe fn xcb_randr_set_crtc_transform_sizeof (_buffer :  *c_void,
                                     filter_params_len :  u32) -> c_int;

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
pub unsafe fn xcb_randr_set_crtc_transform_checked (c : *connection,
                                                crtc :  crtc,
                                                transform :  ffi::render::transform,
                                                filter_len :  u16,
                                                filter_name : *c_char,
                                                filter_params_len :  u32,
                                                filter_params : *ffi::render::fixed) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_randr_set_crtc_transform (c : *connection,
                                        crtc :  crtc,
                                        transform :  ffi::render::transform,
                                        filter_len :  u16,
                                        filter_name : *c_char,
                                        filter_params_len :  u32,
                                        filter_params : *ffi::render::fixed) -> void_cookie;

pub unsafe fn xcb_randr_get_crtc_transform_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_randr_get_crtc_transform (c : *connection,
                                        crtc :  crtc) -> get_crtc_transform_cookie;

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
pub unsafe fn xcb_randr_get_crtc_transform_unchecked (c : *connection,
                                                  crtc :  crtc) -> get_crtc_transform_cookie;

pub unsafe fn xcb_randr_get_crtc_transform_pending_filter_name (R : *get_crtc_transform_reply) -> *c_char;


pub unsafe fn xcb_randr_get_crtc_transform_pending_filter_name_length (R : *get_crtc_transform_reply) -> c_int;


pub unsafe fn xcb_randr_get_crtc_transform_pending_filter_name_end (R : *get_crtc_transform_reply) -> generic_iterator;

pub unsafe fn xcb_randr_get_crtc_transform_pending_params (R : *get_crtc_transform_reply) -> *ffi::render::fixed;


pub unsafe fn xcb_randr_get_crtc_transform_pending_params_length (R : *get_crtc_transform_reply) -> c_int;


pub unsafe fn xcb_randr_get_crtc_transform_pending_params_end (R : *get_crtc_transform_reply) -> generic_iterator;

pub unsafe fn xcb_randr_get_crtc_transform_current_filter_name (R : *get_crtc_transform_reply) -> *c_char;


pub unsafe fn xcb_randr_get_crtc_transform_current_filter_name_length (R : *get_crtc_transform_reply) -> c_int;


pub unsafe fn xcb_randr_get_crtc_transform_current_filter_name_end (R : *get_crtc_transform_reply) -> generic_iterator;

pub unsafe fn xcb_randr_get_crtc_transform_current_params (R : *get_crtc_transform_reply) -> *ffi::render::fixed;


pub unsafe fn xcb_randr_get_crtc_transform_current_params_length (R : *get_crtc_transform_reply) -> c_int;


pub unsafe fn xcb_randr_get_crtc_transform_current_params_end (R : *get_crtc_transform_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_randr_get_crtc_transform_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub unsafe fn xcb_randr_get_crtc_transform_reply (c : *connection,
                                              cookie : get_crtc_transform_cookie,
                                              e : **generic_error) -> *get_crtc_transform_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_randr_get_panning (c : *connection,
                                 crtc :  crtc) -> get_panning_cookie;

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
pub unsafe fn xcb_randr_get_panning_unchecked (c : *connection,
                                           crtc :  crtc) -> get_panning_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_randr_get_panning_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub unsafe fn xcb_randr_get_panning_reply (c : *connection,
                                       cookie : get_panning_cookie,
                                       e : **generic_error) -> *get_panning_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_randr_set_panning (c : *connection,
                                 crtc :  crtc,
                                 timestamp :  ffi::xproto::timestamp,
                                 left :  u16,
                                 top :  u16,
                                 width :  u16,
                                 height :  u16,
                                 track_left :  u16,
                                 track_top :  u16,
                                 track_width :  u16,
                                 track_height :  u16,
                                 border_left :  i16,
                                 border_top :  i16,
                                 border_right :  i16,
                                 border_bottom :  i16) -> set_panning_cookie;

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
pub unsafe fn xcb_randr_set_panning_unchecked (c : *connection,
                                           crtc :  crtc,
                                           timestamp :  ffi::xproto::timestamp,
                                           left :  u16,
                                           top :  u16,
                                           width :  u16,
                                           height :  u16,
                                           track_left :  u16,
                                           track_top :  u16,
                                           track_width :  u16,
                                           track_height :  u16,
                                           border_left :  i16,
                                           border_top :  i16,
                                           border_right :  i16,
                                           border_bottom :  i16) -> set_panning_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_randr_set_panning_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub unsafe fn xcb_randr_set_panning_reply (c : *connection,
                                       cookie : set_panning_cookie,
                                       e : **generic_error) -> *set_panning_reply;

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
pub unsafe fn xcb_randr_set_output_primary_checked (c : *connection,
                                                window :  ffi::xproto::window,
                                                output :  output) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_randr_set_output_primary (c : *connection,
                                        window :  ffi::xproto::window,
                                        output :  output) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub unsafe fn xcb_randr_get_output_primary (c : *connection,
                                        window :  ffi::xproto::window) -> get_output_primary_cookie;

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
pub unsafe fn xcb_randr_get_output_primary_unchecked (c : *connection,
                                                  window :  ffi::xproto::window) -> get_output_primary_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_randr_get_output_primary_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub unsafe fn xcb_randr_get_output_primary_reply (c : *connection,
                                              cookie : get_output_primary_cookie,
                                              e : **generic_error) -> *get_output_primary_reply;

/**
 * Get the next element of the iterator
 * @param i Pointer to a crtc_change_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(crtc_change)
 *
 *
 */
pub unsafe fn xcb_randr_crtc_change_next (i:*crtc_change_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An crtc_change_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub unsafe fn xcb_randr_crtc_change_end (i:crtc_change_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a output_change_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(output_change)
 *
 *
 */
pub unsafe fn xcb_randr_output_change_next (i:*output_change_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An output_change_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub unsafe fn xcb_randr_output_change_end (i:output_change_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a output_property_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(output_property)
 *
 *
 */
pub unsafe fn xcb_randr_output_property_next (i:*output_property_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An output_property_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub unsafe fn xcb_randr_output_property_end (i:output_property_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a notify_data_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(notify_data)
 *
 *
 */
pub unsafe fn xcb_randr_notify_data_next (i:*notify_data_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An notify_data_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub unsafe fn xcb_randr_notify_data_end (i:notify_data_iterator) -> generic_iterator;
}

