/*
 * This file generated automatically from randr.xml by r_client.py.
 * Edit at your peril.
 */

//Make the compiler quiet
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use std;
use libc::*;
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
    pub data : *mut mode,
    pub rem  : c_int,
    pub index: c_int
}


pub type crtc = u32;
/**
 * @brief crtc_iterator
 **/
pub struct crtc_iterator {
    pub data : *mut crtc,
    pub rem  : c_int,
    pub index: c_int
}


pub type output = u32;
/**
 * @brief output_iterator
 **/
pub struct output_iterator {
    pub data : *mut output,
    pub rem  : c_int,
    pub index: c_int
}



pub struct bad_output_error {
     pub response_type :   u8,
     pub error_code :      u8,
     pub sequence :        u16
}



pub struct bad_crtc_error {
     pub response_type :   u8,
     pub error_code :      u8,
     pub sequence :        u16
}



pub struct bad_mode_error {
     pub response_type :   u8,
     pub error_code :      u8,
     pub sequence :        u16
}


pub struct screen_size {
     pub width :     u16,
     pub height :    u16,
     pub mwidth :    u16,
     pub mheight :   u16
}

/**
 * @brief screen_size_iterator
 **/
pub struct screen_size_iterator {
    pub data : *mut screen_size,
    pub rem  : c_int,
    pub index: c_int
}


pub struct refresh_rates {
     pub nRates :   u16
}

/**
 * @brief refresh_rates_iterator
 **/
pub struct refresh_rates_iterator {
    pub data : *mut refresh_rates,
    pub rem  : c_int,
    pub index: c_int
}


pub struct query_version_cookie {
    sequence : c_uint
}


pub struct query_version_request {
     pub major_opcode :    u8,
     pub minor_opcode :    u8,
     pub length :          u16,
     pub major_version :   u32,
     pub minor_version :   u32
}


pub struct query_version_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub major_version :   u32,
     pub minor_version :   u32,
     pub pad1 :            [u8,..16]
}


pub struct set_screen_config_cookie {
    sequence : c_uint
}


pub struct set_screen_config_request {
     pub major_opcode :       u8,
     pub minor_opcode :       u8,
     pub length :             u16,
     pub window :             ffi::xproto::window,
     pub timestamp :          ffi::xproto::timestamp,
     pub config_timestamp :   ffi::xproto::timestamp,
     pub sizeID :             u16,
     pub rotation :           u16,
     pub rate :               u16,
     pub pad0 :               [u8,..2]
}


pub struct set_screen_config_reply {
     pub response_type :      u8,
     pub status :             u8,
     pub sequence :           u16,
     pub length :             u32,
     pub new_timestamp :      ffi::xproto::timestamp,
     pub config_timestamp :   ffi::xproto::timestamp,
     pub root :               ffi::xproto::window,
     pub subpixel_order :     u16,
     pub pad0 :               [u8,..10]
}



pub struct select_input_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub window :         ffi::xproto::window,
     pub enable :         u16,
     pub pad0 :           [u8,..2]
}


pub struct get_screen_info_cookie {
    sequence : c_uint
}


pub struct get_screen_info_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub window :         ffi::xproto::window
}


pub struct get_screen_info_reply {
     pub response_type :      u8,
     pub rotations :          u8,
     pub sequence :           u16,
     pub length :             u32,
     pub root :               ffi::xproto::window,
     pub timestamp :          ffi::xproto::timestamp,
     pub config_timestamp :   ffi::xproto::timestamp,
     pub nSizes :             u16,
     pub sizeID :             u16,
     pub rotation :           u16,
     pub rate :               u16,
     pub nInfo :              u16,
     pub pad0 :               [u8,..2]
}


pub struct get_screen_size_range_cookie {
    sequence : c_uint
}


pub struct get_screen_size_range_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub window :         ffi::xproto::window
}


pub struct get_screen_size_range_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub min_width :       u16,
     pub min_height :      u16,
     pub max_width :       u16,
     pub max_height :      u16,
     pub pad1 :            [u8,..16]
}



pub struct set_screen_size_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub window :         ffi::xproto::window,
     pub width :          u16,
     pub height :         u16,
     pub mm_width :       u32,
     pub mm_height :      u32
}


pub struct mode_info {
     pub id :            u32,
     pub width :         u16,
     pub height :        u16,
     pub dot_clock :     u32,
     pub hsync_start :   u16,
     pub hsync_end :     u16,
     pub htotal :        u16,
     pub hskew :         u16,
     pub vsync_start :   u16,
     pub vsync_end :     u16,
     pub vtotal :        u16,
     pub name_len :      u16,
     pub mode_flags :    u32
}

/**
 * @brief mode_info_iterator
 **/
pub struct mode_info_iterator {
    pub data : *mut mode_info,
    pub rem  : c_int,
    pub index: c_int
}


pub struct get_screen_resources_cookie {
    sequence : c_uint
}


pub struct get_screen_resources_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub window :         ffi::xproto::window
}


pub struct get_screen_resources_reply {
     pub response_type :      u8,
     pub pad0 :               u8,
     pub sequence :           u16,
     pub length :             u32,
     pub timestamp :          ffi::xproto::timestamp,
     pub config_timestamp :   ffi::xproto::timestamp,
     pub num_crtcs :          u16,
     pub num_outputs :        u16,
     pub num_modes :          u16,
     pub names_len :          u16,
     pub pad1 :               [u8,..8]
}


pub struct get_output_info_cookie {
    sequence : c_uint
}


pub struct get_output_info_request {
     pub major_opcode :       u8,
     pub minor_opcode :       u8,
     pub length :             u16,
     pub output :             output,
     pub config_timestamp :   ffi::xproto::timestamp
}


pub struct get_output_info_reply {
     pub response_type :    u8,
     pub status :           u8,
     pub sequence :         u16,
     pub length :           u32,
     pub timestamp :        ffi::xproto::timestamp,
     pub crtc :             crtc,
     pub mm_width :         u32,
     pub mm_height :        u32,
     pub connection :       u8,
     pub subpixel_order :   u8,
     pub num_crtcs :        u16,
     pub num_modes :        u16,
     pub num_preferred :    u16,
     pub num_clones :       u16,
     pub name_len :         u16
}


pub struct list_output_properties_cookie {
    sequence : c_uint
}


pub struct list_output_properties_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub output :         output
}


pub struct list_output_properties_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub num_atoms :       u16,
     pub pad1 :            [u8,..22]
}


pub struct query_output_property_cookie {
    sequence : c_uint
}


pub struct query_output_property_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub output :         output,
     pub property :       ffi::xproto::atom
}


pub struct query_output_property_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub pending :         u8,
     pub range :           u8,
     pub immutable :       u8,
     pub pad1 :            [u8,..21]
}



pub struct configure_output_property_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub output :         output,
     pub property :       ffi::xproto::atom,
     pub pending :        u8,
     pub range :          u8,
     pub pad0 :           [u8,..2]
}



pub struct change_output_property_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub output :         output,
     pub property :       ffi::xproto::atom,
     pub type_ :          ffi::xproto::atom,
     pub format :         u8,
     pub mode :           u8,
     pub pad0 :           [u8,..2],
     pub num_units :      u32
}



pub struct delete_output_property_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub output :         output,
     pub property :       ffi::xproto::atom
}


pub struct get_output_property_cookie {
    sequence : c_uint
}


pub struct get_output_property_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub output :         output,
     pub property :       ffi::xproto::atom,
     pub type_ :          ffi::xproto::atom,
     pub long_offset :    u32,
     pub long_length :    u32,
     pub delete :         u8,
     pub pending :        u8,
     pub pad0 :           [u8,..2]
}


pub struct get_output_property_reply {
     pub response_type :   u8,
     pub format :          u8,
     pub sequence :        u16,
     pub length :          u32,
     pub type_ :           ffi::xproto::atom,
     pub bytes_after :     u32,
     pub num_items :       u32,
     pub pad0 :            [u8,..12]
}


pub struct create_mode_cookie {
    sequence : c_uint
}


pub struct create_mode_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub window :         ffi::xproto::window,
     pub mode_info :      mode_info
}


pub struct create_mode_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub mode :            mode,
     pub pad1 :            [u8,..20]
}



pub struct destroy_mode_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub mode :           mode
}



pub struct add_output_mode_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub output :         output,
     pub mode :           mode
}



pub struct delete_output_mode_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub output :         output,
     pub mode :           mode
}


pub struct get_crtc_info_cookie {
    sequence : c_uint
}


pub struct get_crtc_info_request {
     pub major_opcode :       u8,
     pub minor_opcode :       u8,
     pub length :             u16,
     pub crtc :               crtc,
     pub config_timestamp :   ffi::xproto::timestamp
}


pub struct get_crtc_info_reply {
     pub response_type :          u8,
     pub status :                 u8,
     pub sequence :               u16,
     pub length :                 u32,
     pub timestamp :              ffi::xproto::timestamp,
     pub x :                      i16,
     pub y :                      i16,
     pub width :                  u16,
     pub height :                 u16,
     pub mode :                   mode,
     pub rotation :               u16,
     pub rotations :              u16,
     pub num_outputs :            u16,
     pub num_possible_outputs :   u16
}


pub struct set_crtc_config_cookie {
    sequence : c_uint
}


pub struct set_crtc_config_request {
     pub major_opcode :       u8,
     pub minor_opcode :       u8,
     pub length :             u16,
     pub crtc :               crtc,
     pub timestamp :          ffi::xproto::timestamp,
     pub config_timestamp :   ffi::xproto::timestamp,
     pub x :                  i16,
     pub y :                  i16,
     pub mode :               mode,
     pub rotation :           u16,
     pub pad0 :               [u8,..2]
}


pub struct set_crtc_config_reply {
     pub response_type :   u8,
     pub status :          u8,
     pub sequence :        u16,
     pub length :          u32,
     pub timestamp :       ffi::xproto::timestamp,
     pub pad0 :            [u8,..20]
}


pub struct get_crtc_gamma_size_cookie {
    sequence : c_uint
}


pub struct get_crtc_gamma_size_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub crtc :           crtc
}


pub struct get_crtc_gamma_size_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub size :            u16,
     pub pad1 :            [u8,..22]
}


pub struct get_crtc_gamma_cookie {
    sequence : c_uint
}


pub struct get_crtc_gamma_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub crtc :           crtc
}


pub struct get_crtc_gamma_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub size :            u16,
     pub pad1 :            [u8,..22]
}



pub struct set_crtc_gamma_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub crtc :           crtc,
     pub size :           u16,
     pub pad0 :           [u8,..2]
}


pub struct get_screen_resources_current_cookie {
    sequence : c_uint
}


pub struct get_screen_resources_current_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub window :         ffi::xproto::window
}


pub struct get_screen_resources_current_reply {
     pub response_type :      u8,
     pub pad0 :               u8,
     pub sequence :           u16,
     pub length :             u32,
     pub timestamp :          ffi::xproto::timestamp,
     pub config_timestamp :   ffi::xproto::timestamp,
     pub num_crtcs :          u16,
     pub num_outputs :        u16,
     pub num_modes :          u16,
     pub names_len :          u16,
     pub pad1 :               [u8,..8]
}



pub struct set_crtc_transform_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub crtc :           crtc,
     pub transform :      ffi::render::transform,
     pub filter_len :     u16,
     pub pad0 :           [u8,..2]
}


pub struct get_crtc_transform_cookie {
    sequence : c_uint
}


pub struct get_crtc_transform_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub crtc :           crtc
}


pub struct get_crtc_transform_reply {
     pub response_type :       u8,
     pub pad0 :                u8,
     pub sequence :            u16,
     pub length :              u32,
     pub pending_transform :   ffi::render::transform,
     pub has_transforms :      u8,
     pub pad1 :                [u8,..3],
     pub current_transform :   ffi::render::transform,
     pub pad2 :                [u8,..4],
     pub pending_len :         u16,
     pub pending_nparams :     u16,
     pub current_len :         u16,
     pub current_nparams :     u16
}


pub struct get_panning_cookie {
    sequence : c_uint
}


pub struct get_panning_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub crtc :           crtc
}


pub struct get_panning_reply {
     pub response_type :   u8,
     pub status :          u8,
     pub sequence :        u16,
     pub length :          u32,
     pub timestamp :       ffi::xproto::timestamp,
     pub left :            u16,
     pub top :             u16,
     pub width :           u16,
     pub height :          u16,
     pub track_left :      u16,
     pub track_top :       u16,
     pub track_width :     u16,
     pub track_height :    u16,
     pub border_left :     i16,
     pub border_top :      i16,
     pub border_right :    i16,
     pub border_bottom :   i16
}


pub struct set_panning_cookie {
    sequence : c_uint
}


pub struct set_panning_request {
     pub major_opcode :    u8,
     pub minor_opcode :    u8,
     pub length :          u16,
     pub crtc :            crtc,
     pub timestamp :       ffi::xproto::timestamp,
     pub left :            u16,
     pub top :             u16,
     pub width :           u16,
     pub height :          u16,
     pub track_left :      u16,
     pub track_top :       u16,
     pub track_width :     u16,
     pub track_height :    u16,
     pub border_left :     i16,
     pub border_top :      i16,
     pub border_right :    i16,
     pub border_bottom :   i16
}


pub struct set_panning_reply {
     pub response_type :   u8,
     pub status :          u8,
     pub sequence :        u16,
     pub length :          u32,
     pub timestamp :       ffi::xproto::timestamp
}



pub struct set_output_primary_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub window :         ffi::xproto::window,
     pub output :         output
}


pub struct get_output_primary_cookie {
    sequence : c_uint
}


pub struct get_output_primary_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub window :         ffi::xproto::window
}


pub struct get_output_primary_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub output :          output
}



pub struct screen_change_notify_event {
     pub response_type :      u8,
     pub rotation :           u8,
     pub sequence :           u16,
     pub timestamp :          ffi::xproto::timestamp,
     pub config_timestamp :   ffi::xproto::timestamp,
     pub root :               ffi::xproto::window,
     pub request_window :     ffi::xproto::window,
     pub sizeID :             u16,
     pub subpixel_order :     u16,
     pub width :              u16,
     pub height :             u16,
     pub mwidth :             u16,
     pub mheight :            u16
}


pub struct crtc_change {
     pub timestamp :   ffi::xproto::timestamp,
     pub window :      ffi::xproto::window,
     pub crtc :        crtc,
     pub mode :        mode,
     pub rotation :    u16,
     pub pad0 :        [u8,..2],
     pub x :           i16,
     pub y :           i16,
     pub width :       u16,
     pub height :      u16
}

/**
 * @brief crtc_change_iterator
 **/
pub struct crtc_change_iterator {
    pub data : *mut crtc_change,
    pub rem  : c_int,
    pub index: c_int
}


pub struct output_change {
     pub timestamp :          ffi::xproto::timestamp,
     pub config_timestamp :   ffi::xproto::timestamp,
     pub window :             ffi::xproto::window,
     pub output :             output,
     pub crtc :               crtc,
     pub mode :               mode,
     pub rotation :           u16,
     pub connection :         u8,
     pub subpixel_order :     u8
}

/**
 * @brief output_change_iterator
 **/
pub struct output_change_iterator {
    pub data : *mut output_change,
    pub rem  : c_int,
    pub index: c_int
}


pub struct output_property {
     pub window :      ffi::xproto::window,
     pub output :      output,
     pub atom :        ffi::xproto::atom,
     pub timestamp :   ffi::xproto::timestamp,
     pub status :      u8,
     pub pad0 :        [u8,..11]
}

/**
 * @brief output_property_iterator
 **/
pub struct output_property_iterator {
    pub data : *mut output_property,
    pub rem  : c_int,
    pub index: c_int
}


pub struct notify_data {
    data : [u8,..28]
}
/**
 * @brief notify_data_iterator
 **/
pub struct notify_data_iterator {
    pub data : *mut notify_data,
    pub rem  : c_int,
    pub index: c_int
}



pub struct notify_event {
     pub response_type :   u8,
     pub subCode :         u8,
     pub sequence :        u16,
     pub u :               notify_data
}

#[link(name="lxcb-randr")]
extern "C" {

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
pub fn xcb_randr_mode_next (i:*mut mode_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An mode_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_randr_mode_end (i:mode_iterator) -> ffi::base::generic_iterator;

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
pub fn xcb_randr_crtc_next (i:*mut crtc_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An crtc_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_randr_crtc_end (i:crtc_iterator) -> ffi::base::generic_iterator;

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
pub fn xcb_randr_output_next (i:*mut output_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An output_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_randr_output_end (i:output_iterator) -> ffi::base::generic_iterator;

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
pub fn xcb_randr_screen_size_next (i:*mut screen_size_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An screen_size_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_randr_screen_size_end (i:screen_size_iterator) -> ffi::base::generic_iterator;

pub fn xcb_randr_refresh_rates_sizeof (_buffer :  *mut c_void) -> c_int;

pub fn xcb_randr_refresh_rates_rates (R : *mut refresh_rates) -> *mut u16;


pub fn xcb_randr_refresh_rates_rates_length (R : *mut refresh_rates) -> c_int;


pub fn xcb_randr_refresh_rates_rates_end (R : *mut refresh_rates) -> ffi::base::generic_iterator;

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
pub fn xcb_randr_refresh_rates_next (i:*mut refresh_rates_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An refresh_rates_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_randr_refresh_rates_end (i:refresh_rates_iterator) -> ffi::base::generic_iterator;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_randr_query_version (c : *mut ffi::base::connection,
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
pub fn xcb_randr_query_version_unchecked (c : *mut ffi::base::connection,
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
pub fn xcb_randr_query_version_reply (c : *mut ffi::base::connection,
                                         cookie : query_version_cookie,
                                         e : *mut *mut ffi::base::generic_error) -> *mut query_version_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_randr_set_screen_config (c : *mut ffi::base::connection,
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
pub fn xcb_randr_set_screen_config_unchecked (c : *mut ffi::base::connection,
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
pub fn xcb_randr_set_screen_config_reply (c : *mut ffi::base::connection,
                                             cookie : set_screen_config_cookie,
                                             e : *mut *mut ffi::base::generic_error) -> *mut set_screen_config_reply;

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
pub fn xcb_randr_select_input_checked (c : *mut ffi::base::connection,
                                          window :  ffi::xproto::window,
                                          enable :  u16) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_randr_select_input (c : *mut ffi::base::connection,
                                  window :  ffi::xproto::window,
                                  enable :  u16) -> ffi::base::void_cookie;

pub fn xcb_randr_get_screen_info_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_randr_get_screen_info (c : *mut ffi::base::connection,
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
pub fn xcb_randr_get_screen_info_unchecked (c : *mut ffi::base::connection,
                                               window :  ffi::xproto::window) -> get_screen_info_cookie;

pub fn xcb_randr_get_screen_info_sizes (R : *mut get_screen_info_reply) -> *mut screen_size;


pub fn xcb_randr_get_screen_info_sizes_length (R : *mut get_screen_info_reply) -> c_int;

pub fn xcb_randr_get_screen_info_sizes_iterator (R : *mut get_screen_info_reply) -> screen_size_iterator;


pub fn xcb_randr_get_screen_info_rates_length (R : *mut get_screen_info_reply) -> c_int;

pub fn xcb_randr_get_screen_info_rates_iterator (R : *mut get_screen_info_reply) -> refresh_rates_iterator;

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
pub fn xcb_randr_get_screen_info_reply (c : *mut ffi::base::connection,
                                           cookie : get_screen_info_cookie,
                                           e : *mut *mut ffi::base::generic_error) -> *mut get_screen_info_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_randr_get_screen_size_range (c : *mut ffi::base::connection,
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
pub fn xcb_randr_get_screen_size_range_unchecked (c : *mut ffi::base::connection,
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
pub fn xcb_randr_get_screen_size_range_reply (c : *mut ffi::base::connection,
                                                 cookie : get_screen_size_range_cookie,
                                                 e : *mut *mut ffi::base::generic_error) -> *mut get_screen_size_range_reply;

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
pub fn xcb_randr_set_screen_size_checked (c : *mut ffi::base::connection,
                                             window :  ffi::xproto::window,
                                             width :  u16,
                                             height :  u16,
                                             mm_width :  u32,
                                             mm_height :  u32) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_randr_set_screen_size (c : *mut ffi::base::connection,
                                     window :  ffi::xproto::window,
                                     width :  u16,
                                     height :  u16,
                                     mm_width :  u32,
                                     mm_height :  u32) -> ffi::base::void_cookie;

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
pub fn xcb_randr_mode_info_next (i:*mut mode_info_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An mode_info_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_randr_mode_info_end (i:mode_info_iterator) -> ffi::base::generic_iterator;

pub fn xcb_randr_get_screen_resources_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_randr_get_screen_resources (c : *mut ffi::base::connection,
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
pub fn xcb_randr_get_screen_resources_unchecked (c : *mut ffi::base::connection,
                                                    window :  ffi::xproto::window) -> get_screen_resources_cookie;

pub fn xcb_randr_get_screen_resources_crtcs (R : *mut get_screen_resources_reply) -> *mut crtc;


pub fn xcb_randr_get_screen_resources_crtcs_length (R : *mut get_screen_resources_reply) -> c_int;


pub fn xcb_randr_get_screen_resources_crtcs_end (R : *mut get_screen_resources_reply) -> ffi::base::generic_iterator;

pub fn xcb_randr_get_screen_resources_outputs (R : *mut get_screen_resources_reply) -> *mut output;


pub fn xcb_randr_get_screen_resources_outputs_length (R : *mut get_screen_resources_reply) -> c_int;


pub fn xcb_randr_get_screen_resources_outputs_end (R : *mut get_screen_resources_reply) -> ffi::base::generic_iterator;

pub fn xcb_randr_get_screen_resources_modes (R : *mut get_screen_resources_reply) -> *mut mode_info;


pub fn xcb_randr_get_screen_resources_modes_length (R : *mut get_screen_resources_reply) -> c_int;

pub fn xcb_randr_get_screen_resources_modes_iterator (R : *mut get_screen_resources_reply) -> mode_info_iterator;

pub fn xcb_randr_get_screen_resources_names (R : *mut get_screen_resources_reply) -> *mut u8;


pub fn xcb_randr_get_screen_resources_names_length (R : *mut get_screen_resources_reply) -> c_int;


pub fn xcb_randr_get_screen_resources_names_end (R : *mut get_screen_resources_reply) -> ffi::base::generic_iterator;

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
pub fn xcb_randr_get_screen_resources_reply (c : *mut ffi::base::connection,
                                                cookie : get_screen_resources_cookie,
                                                e : *mut *mut ffi::base::generic_error) -> *mut get_screen_resources_reply;

pub fn xcb_randr_get_output_info_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_randr_get_output_info (c : *mut ffi::base::connection,
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
pub fn xcb_randr_get_output_info_unchecked (c : *mut ffi::base::connection,
                                               output :  output,
                                               config_timestamp :  ffi::xproto::timestamp) -> get_output_info_cookie;

pub fn xcb_randr_get_output_info_crtcs (R : *mut get_output_info_reply) -> *mut crtc;


pub fn xcb_randr_get_output_info_crtcs_length (R : *mut get_output_info_reply) -> c_int;


pub fn xcb_randr_get_output_info_crtcs_end (R : *mut get_output_info_reply) -> ffi::base::generic_iterator;

pub fn xcb_randr_get_output_info_modes (R : *mut get_output_info_reply) -> *mut mode;


pub fn xcb_randr_get_output_info_modes_length (R : *mut get_output_info_reply) -> c_int;


pub fn xcb_randr_get_output_info_modes_end (R : *mut get_output_info_reply) -> ffi::base::generic_iterator;

pub fn xcb_randr_get_output_info_clones (R : *mut get_output_info_reply) -> *mut output;


pub fn xcb_randr_get_output_info_clones_length (R : *mut get_output_info_reply) -> c_int;


pub fn xcb_randr_get_output_info_clones_end (R : *mut get_output_info_reply) -> ffi::base::generic_iterator;

pub fn xcb_randr_get_output_info_name (R : *mut get_output_info_reply) -> *mut u8;


pub fn xcb_randr_get_output_info_name_length (R : *mut get_output_info_reply) -> c_int;


pub fn xcb_randr_get_output_info_name_end (R : *mut get_output_info_reply) -> ffi::base::generic_iterator;

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
pub fn xcb_randr_get_output_info_reply (c : *mut ffi::base::connection,
                                           cookie : get_output_info_cookie,
                                           e : *mut *mut ffi::base::generic_error) -> *mut get_output_info_reply;

pub fn xcb_randr_list_output_properties_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_randr_list_output_properties (c : *mut ffi::base::connection,
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
pub fn xcb_randr_list_output_properties_unchecked (c : *mut ffi::base::connection,
                                                      output :  output) -> list_output_properties_cookie;

pub fn xcb_randr_list_output_properties_atoms (R : *mut list_output_properties_reply) -> *mut ffi::xproto::atom;


pub fn xcb_randr_list_output_properties_atoms_length (R : *mut list_output_properties_reply) -> c_int;


pub fn xcb_randr_list_output_properties_atoms_end (R : *mut list_output_properties_reply) -> ffi::base::generic_iterator;

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
pub fn xcb_randr_list_output_properties_reply (c : *mut ffi::base::connection,
                                                  cookie : list_output_properties_cookie,
                                                  e : *mut *mut ffi::base::generic_error) -> *mut list_output_properties_reply;

pub fn xcb_randr_query_output_property_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_randr_query_output_property (c : *mut ffi::base::connection,
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
pub fn xcb_randr_query_output_property_unchecked (c : *mut ffi::base::connection,
                                                     output :  output,
                                                     property :  ffi::xproto::atom) -> query_output_property_cookie;

pub fn xcb_randr_query_output_property_valid_values (R : *mut query_output_property_reply) -> *mut i32;


pub fn xcb_randr_query_output_property_valid_values_length (R : *mut query_output_property_reply) -> c_int;


pub fn xcb_randr_query_output_property_valid_values_end (R : *mut query_output_property_reply) -> ffi::base::generic_iterator;

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
pub fn xcb_randr_query_output_property_reply (c : *mut ffi::base::connection,
                                                 cookie : query_output_property_cookie,
                                                 e : *mut *mut ffi::base::generic_error) -> *mut query_output_property_reply;

pub fn xcb_randr_configure_output_property_sizeof (_buffer :  *mut c_void,
                                            values_len :   u32) -> c_int;

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
pub fn xcb_randr_configure_output_property_checked (c : *mut ffi::base::connection,
                                                       output :  output,
                                                       property :  ffi::xproto::atom,
                                                       pending :  u8,
                                                       range :  u8,
                                                       values_len :  u32,
                                                       values : *mut i32) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_randr_configure_output_property (c : *mut ffi::base::connection,
                                               output :  output,
                                               property :  ffi::xproto::atom,
                                               pending :  u8,
                                               range :  u8,
                                               values_len :  u32,
                                               values : *mut i32) -> ffi::base::void_cookie;

pub fn xcb_randr_change_output_property_sizeof (_buffer :  *mut c_void) -> c_int;

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
pub fn xcb_randr_change_output_property_checked (c : *mut ffi::base::connection,
                                                    output :  output,
                                                    property :  ffi::xproto::atom,
                                                    type_ :  ffi::xproto::atom,
                                                    format :  u8,
                                                    mode :  u8,
                                                    num_units :  u32,
                                                    data : *mut c_void) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_randr_change_output_property (c : *mut ffi::base::connection,
                                            output :  output,
                                            property :  ffi::xproto::atom,
                                            type_ :  ffi::xproto::atom,
                                            format :  u8,
                                            mode :  u8,
                                            num_units :  u32,
                                            data : *mut c_void) -> ffi::base::void_cookie;

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
pub fn xcb_randr_delete_output_property_checked (c : *mut ffi::base::connection,
                                                    output :  output,
                                                    property :  ffi::xproto::atom) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_randr_delete_output_property (c : *mut ffi::base::connection,
                                            output :  output,
                                            property :  ffi::xproto::atom) -> ffi::base::void_cookie;

pub fn xcb_randr_get_output_property_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_randr_get_output_property (c : *mut ffi::base::connection,
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
pub fn xcb_randr_get_output_property_unchecked (c : *mut ffi::base::connection,
                                                   output :  output,
                                                   property :  ffi::xproto::atom,
                                                   type_ :  ffi::xproto::atom,
                                                   long_offset :  u32,
                                                   long_length :  u32,
                                                   delete :  u8,
                                                   pending :  u8) -> get_output_property_cookie;

pub fn xcb_randr_get_output_property_data (R : *mut get_output_property_reply) -> *mut u8;


pub fn xcb_randr_get_output_property_data_length (R : *mut get_output_property_reply) -> c_int;


pub fn xcb_randr_get_output_property_data_end (R : *mut get_output_property_reply) -> ffi::base::generic_iterator;

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
pub fn xcb_randr_get_output_property_reply (c : *mut ffi::base::connection,
                                               cookie : get_output_property_cookie,
                                               e : *mut *mut ffi::base::generic_error) -> *mut get_output_property_reply;

pub fn xcb_randr_create_mode_sizeof (_buffer :  *mut c_void,
                              name_len :     u32) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_randr_create_mode (c : *mut ffi::base::connection,
                                 window :  ffi::xproto::window,
                                 mode_info :  mode_info,
                                 name_len :  u32,
                                 name : *mut c_char) -> create_mode_cookie;

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
pub fn xcb_randr_create_mode_unchecked (c : *mut ffi::base::connection,
                                           window :  ffi::xproto::window,
                                           mode_info :  mode_info,
                                           name_len :  u32,
                                           name : *mut c_char) -> create_mode_cookie;

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
pub fn xcb_randr_create_mode_reply (c : *mut ffi::base::connection,
                                       cookie : create_mode_cookie,
                                       e : *mut *mut ffi::base::generic_error) -> *mut create_mode_reply;

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
pub fn xcb_randr_destroy_mode_checked (c : *mut ffi::base::connection,
                                          mode :  mode) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_randr_destroy_mode (c : *mut ffi::base::connection,
                                  mode :  mode) -> ffi::base::void_cookie;

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
pub fn xcb_randr_add_output_mode_checked (c : *mut ffi::base::connection,
                                             output :  output,
                                             mode :  mode) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_randr_add_output_mode (c : *mut ffi::base::connection,
                                     output :  output,
                                     mode :  mode) -> ffi::base::void_cookie;

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
pub fn xcb_randr_delete_output_mode_checked (c : *mut ffi::base::connection,
                                                output :  output,
                                                mode :  mode) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_randr_delete_output_mode (c : *mut ffi::base::connection,
                                        output :  output,
                                        mode :  mode) -> ffi::base::void_cookie;

pub fn xcb_randr_get_crtc_info_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_randr_get_crtc_info (c : *mut ffi::base::connection,
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
pub fn xcb_randr_get_crtc_info_unchecked (c : *mut ffi::base::connection,
                                             crtc :  crtc,
                                             config_timestamp :  ffi::xproto::timestamp) -> get_crtc_info_cookie;

pub fn xcb_randr_get_crtc_info_outputs (R : *mut get_crtc_info_reply) -> *mut output;


pub fn xcb_randr_get_crtc_info_outputs_length (R : *mut get_crtc_info_reply) -> c_int;


pub fn xcb_randr_get_crtc_info_outputs_end (R : *mut get_crtc_info_reply) -> ffi::base::generic_iterator;

pub fn xcb_randr_get_crtc_info_possible (R : *mut get_crtc_info_reply) -> *mut output;


pub fn xcb_randr_get_crtc_info_possible_length (R : *mut get_crtc_info_reply) -> c_int;


pub fn xcb_randr_get_crtc_info_possible_end (R : *mut get_crtc_info_reply) -> ffi::base::generic_iterator;

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
pub fn xcb_randr_get_crtc_info_reply (c : *mut ffi::base::connection,
                                         cookie : get_crtc_info_cookie,
                                         e : *mut *mut ffi::base::generic_error) -> *mut get_crtc_info_reply;

pub fn xcb_randr_set_crtc_config_sizeof (_buffer :  *mut c_void,
                                  outputs_len :  u32) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_randr_set_crtc_config (c : *mut ffi::base::connection,
                                     crtc :  crtc,
                                     timestamp :  ffi::xproto::timestamp,
                                     config_timestamp :  ffi::xproto::timestamp,
                                     x :  i16,
                                     y :  i16,
                                     mode :  mode,
                                     rotation :  u16,
                                     outputs_len :  u32,
                                     outputs : *mut output) -> set_crtc_config_cookie;

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
pub fn xcb_randr_set_crtc_config_unchecked (c : *mut ffi::base::connection,
                                               crtc :  crtc,
                                               timestamp :  ffi::xproto::timestamp,
                                               config_timestamp :  ffi::xproto::timestamp,
                                               x :  i16,
                                               y :  i16,
                                               mode :  mode,
                                               rotation :  u16,
                                               outputs_len :  u32,
                                               outputs : *mut output) -> set_crtc_config_cookie;

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
pub fn xcb_randr_set_crtc_config_reply (c : *mut ffi::base::connection,
                                           cookie : set_crtc_config_cookie,
                                           e : *mut *mut ffi::base::generic_error) -> *mut set_crtc_config_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_randr_get_crtc_gamma_size (c : *mut ffi::base::connection,
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
pub fn xcb_randr_get_crtc_gamma_size_unchecked (c : *mut ffi::base::connection,
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
pub fn xcb_randr_get_crtc_gamma_size_reply (c : *mut ffi::base::connection,
                                               cookie : get_crtc_gamma_size_cookie,
                                               e : *mut *mut ffi::base::generic_error) -> *mut get_crtc_gamma_size_reply;

pub fn xcb_randr_get_crtc_gamma_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_randr_get_crtc_gamma (c : *mut ffi::base::connection,
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
pub fn xcb_randr_get_crtc_gamma_unchecked (c : *mut ffi::base::connection,
                                              crtc :  crtc) -> get_crtc_gamma_cookie;

pub fn xcb_randr_get_crtc_gamma_red (R : *mut get_crtc_gamma_reply) -> *mut u16;


pub fn xcb_randr_get_crtc_gamma_red_length (R : *mut get_crtc_gamma_reply) -> c_int;


pub fn xcb_randr_get_crtc_gamma_red_end (R : *mut get_crtc_gamma_reply) -> ffi::base::generic_iterator;

pub fn xcb_randr_get_crtc_gamma_green (R : *mut get_crtc_gamma_reply) -> *mut u16;


pub fn xcb_randr_get_crtc_gamma_green_length (R : *mut get_crtc_gamma_reply) -> c_int;


pub fn xcb_randr_get_crtc_gamma_green_end (R : *mut get_crtc_gamma_reply) -> ffi::base::generic_iterator;

pub fn xcb_randr_get_crtc_gamma_blue (R : *mut get_crtc_gamma_reply) -> *mut u16;


pub fn xcb_randr_get_crtc_gamma_blue_length (R : *mut get_crtc_gamma_reply) -> c_int;


pub fn xcb_randr_get_crtc_gamma_blue_end (R : *mut get_crtc_gamma_reply) -> ffi::base::generic_iterator;

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
pub fn xcb_randr_get_crtc_gamma_reply (c : *mut ffi::base::connection,
                                          cookie : get_crtc_gamma_cookie,
                                          e : *mut *mut ffi::base::generic_error) -> *mut get_crtc_gamma_reply;

pub fn xcb_randr_set_crtc_gamma_sizeof (_buffer :  *mut c_void) -> c_int;

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
pub fn xcb_randr_set_crtc_gamma_checked (c : *mut ffi::base::connection,
                                            crtc :  crtc,
                                            size :  u16,
                                            red : *mut u16,
                                            green : *mut u16,
                                            blue : *mut u16) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_randr_set_crtc_gamma (c : *mut ffi::base::connection,
                                    crtc :  crtc,
                                    size :  u16,
                                    red : *mut u16,
                                    green : *mut u16,
                                    blue : *mut u16) -> ffi::base::void_cookie;

pub fn xcb_randr_get_screen_resources_current_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_randr_get_screen_resources_current (c : *mut ffi::base::connection,
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
pub fn xcb_randr_get_screen_resources_current_unchecked (c : *mut ffi::base::connection,
                                                            window :  ffi::xproto::window) -> get_screen_resources_current_cookie;

pub fn xcb_randr_get_screen_resources_current_crtcs (R : *mut get_screen_resources_current_reply) -> *mut crtc;


pub fn xcb_randr_get_screen_resources_current_crtcs_length (R : *mut get_screen_resources_current_reply) -> c_int;


pub fn xcb_randr_get_screen_resources_current_crtcs_end (R : *mut get_screen_resources_current_reply) -> ffi::base::generic_iterator;

pub fn xcb_randr_get_screen_resources_current_outputs (R : *mut get_screen_resources_current_reply) -> *mut output;


pub fn xcb_randr_get_screen_resources_current_outputs_length (R : *mut get_screen_resources_current_reply) -> c_int;


pub fn xcb_randr_get_screen_resources_current_outputs_end (R : *mut get_screen_resources_current_reply) -> ffi::base::generic_iterator;

pub fn xcb_randr_get_screen_resources_current_modes (R : *mut get_screen_resources_current_reply) -> *mut mode_info;


pub fn xcb_randr_get_screen_resources_current_modes_length (R : *mut get_screen_resources_current_reply) -> c_int;

pub fn xcb_randr_get_screen_resources_current_modes_iterator (R : *mut get_screen_resources_current_reply) -> mode_info_iterator;

pub fn xcb_randr_get_screen_resources_current_names (R : *mut get_screen_resources_current_reply) -> *mut u8;


pub fn xcb_randr_get_screen_resources_current_names_length (R : *mut get_screen_resources_current_reply) -> c_int;


pub fn xcb_randr_get_screen_resources_current_names_end (R : *mut get_screen_resources_current_reply) -> ffi::base::generic_iterator;

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
pub fn xcb_randr_get_screen_resources_current_reply (c : *mut ffi::base::connection,
                                                        cookie : get_screen_resources_current_cookie,
                                                        e : *mut *mut ffi::base::generic_error) -> *mut get_screen_resources_current_reply;

pub fn xcb_randr_set_crtc_transform_sizeof (_buffer :  *mut c_void,
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
pub fn xcb_randr_set_crtc_transform_checked (c : *mut ffi::base::connection,
                                                crtc :  crtc,
                                                transform :  ffi::render::transform,
                                                filter_len :  u16,
                                                filter_name : *mut c_char,
                                                filter_params_len :  u32,
                                                filter_params : *mut ffi::render::fixed) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_randr_set_crtc_transform (c : *mut ffi::base::connection,
                                        crtc :  crtc,
                                        transform :  ffi::render::transform,
                                        filter_len :  u16,
                                        filter_name : *mut c_char,
                                        filter_params_len :  u32,
                                        filter_params : *mut ffi::render::fixed) -> ffi::base::void_cookie;

pub fn xcb_randr_get_crtc_transform_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_randr_get_crtc_transform (c : *mut ffi::base::connection,
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
pub fn xcb_randr_get_crtc_transform_unchecked (c : *mut ffi::base::connection,
                                                  crtc :  crtc) -> get_crtc_transform_cookie;

pub fn xcb_randr_get_crtc_transform_pending_filter_name (R : *mut get_crtc_transform_reply) -> *mut c_char;


pub fn xcb_randr_get_crtc_transform_pending_filter_name_length (R : *mut get_crtc_transform_reply) -> c_int;


pub fn xcb_randr_get_crtc_transform_pending_filter_name_end (R : *mut get_crtc_transform_reply) -> ffi::base::generic_iterator;

pub fn xcb_randr_get_crtc_transform_pending_params (R : *mut get_crtc_transform_reply) -> *mut ffi::render::fixed;


pub fn xcb_randr_get_crtc_transform_pending_params_length (R : *mut get_crtc_transform_reply) -> c_int;


pub fn xcb_randr_get_crtc_transform_pending_params_end (R : *mut get_crtc_transform_reply) -> ffi::base::generic_iterator;

pub fn xcb_randr_get_crtc_transform_current_filter_name (R : *mut get_crtc_transform_reply) -> *mut c_char;


pub fn xcb_randr_get_crtc_transform_current_filter_name_length (R : *mut get_crtc_transform_reply) -> c_int;


pub fn xcb_randr_get_crtc_transform_current_filter_name_end (R : *mut get_crtc_transform_reply) -> ffi::base::generic_iterator;

pub fn xcb_randr_get_crtc_transform_current_params (R : *mut get_crtc_transform_reply) -> *mut ffi::render::fixed;


pub fn xcb_randr_get_crtc_transform_current_params_length (R : *mut get_crtc_transform_reply) -> c_int;


pub fn xcb_randr_get_crtc_transform_current_params_end (R : *mut get_crtc_transform_reply) -> ffi::base::generic_iterator;

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
pub fn xcb_randr_get_crtc_transform_reply (c : *mut ffi::base::connection,
                                              cookie : get_crtc_transform_cookie,
                                              e : *mut *mut ffi::base::generic_error) -> *mut get_crtc_transform_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_randr_get_panning (c : *mut ffi::base::connection,
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
pub fn xcb_randr_get_panning_unchecked (c : *mut ffi::base::connection,
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
pub fn xcb_randr_get_panning_reply (c : *mut ffi::base::connection,
                                       cookie : get_panning_cookie,
                                       e : *mut *mut ffi::base::generic_error) -> *mut get_panning_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_randr_set_panning (c : *mut ffi::base::connection,
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
pub fn xcb_randr_set_panning_unchecked (c : *mut ffi::base::connection,
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
pub fn xcb_randr_set_panning_reply (c : *mut ffi::base::connection,
                                       cookie : set_panning_cookie,
                                       e : *mut *mut ffi::base::generic_error) -> *mut set_panning_reply;

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
pub fn xcb_randr_set_output_primary_checked (c : *mut ffi::base::connection,
                                                window :  ffi::xproto::window,
                                                output :  output) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_randr_set_output_primary (c : *mut ffi::base::connection,
                                        window :  ffi::xproto::window,
                                        output :  output) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_randr_get_output_primary (c : *mut ffi::base::connection,
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
pub fn xcb_randr_get_output_primary_unchecked (c : *mut ffi::base::connection,
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
pub fn xcb_randr_get_output_primary_reply (c : *mut ffi::base::connection,
                                              cookie : get_output_primary_cookie,
                                              e : *mut *mut ffi::base::generic_error) -> *mut get_output_primary_reply;

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
pub fn xcb_randr_crtc_change_next (i:*mut crtc_change_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An crtc_change_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_randr_crtc_change_end (i:crtc_change_iterator) -> ffi::base::generic_iterator;

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
pub fn xcb_randr_output_change_next (i:*mut output_change_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An output_change_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_randr_output_change_end (i:output_change_iterator) -> ffi::base::generic_iterator;

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
pub fn xcb_randr_output_property_next (i:*mut output_property_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An output_property_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_randr_output_property_end (i:output_property_iterator) -> ffi::base::generic_iterator;

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
pub fn xcb_randr_notify_data_next (i:*mut notify_data_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An notify_data_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_randr_notify_data_end (i:notify_data_iterator) -> ffi::base::generic_iterator;
}

