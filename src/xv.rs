/*
 * This file generated automatically from xv.xml by r_client.py.
 * Edit at your peril.
 */

//Make the compiler quiet
#![allow(unused_imports)]
#![allow(unused_unsafe)]
use std;
use libc::*;
use std::{mem,num,ptr,str};
use ffi::base::*;
use base;
use base::*;
use ffi;
use ffi::xv::*;
use std::option::Option;
use std::iter::Iterator;

use xproto;
use shm;
pub type Port = port;

pub type PortIterator = port_iterator;

pub type EncodingIterator = encoding_iterator;


pub type type_ = c_uint;//{
    pub static XCB_XV_TYPE_INPUT_MASK : type_ = 1;
    pub static XCB_XV_TYPE_OUTPUT_MASK : type_ = 2;
    pub static XCB_XV_TYPE_VIDEO_MASK : type_ = 4;
    pub static XCB_XV_TYPE_STILL_MASK : type_ = 8;
    pub static XCB_XV_TYPE_IMAGE_MASK : type_ = 16;
//}

pub type image_format_info_type = c_uint;//{
    pub static XCB_XV_IMAGE_FORMAT_INFO_TYPE_RGB : image_format_info_type = 1;
    pub static XCB_XV_IMAGE_FORMAT_INFO_TYPE_YUV : image_format_info_type = 2;
//}

pub type image_format_info_format = c_uint;//{
    pub static XCB_XV_IMAGE_FORMAT_INFO_FORMAT_PACKED : image_format_info_format = 1;
    pub static XCB_XV_IMAGE_FORMAT_INFO_FORMAT_PLANAR : image_format_info_format = 2;
//}

pub type attribute_flag = c_uint;//{
    pub static XCB_XV_ATTRIBUTE_FLAG_GETTABLE : attribute_flag = 1;
    pub static XCB_XV_ATTRIBUTE_FLAG_SETTABLE : attribute_flag = 2;
//}

pub type video_notify_reason = c_uint;//{
    pub static XCB_XV_VIDEO_NOTIFY_REASON_STARTED : video_notify_reason = 1;
    pub static XCB_XV_VIDEO_NOTIFY_REASON_STOPPED : video_notify_reason = 2;
    pub static XCB_XV_VIDEO_NOTIFY_REASON_BUSY : video_notify_reason = 3;
    pub static XCB_XV_VIDEO_NOTIFY_REASON_PREEMPTED : video_notify_reason = 4;
    pub static XCB_XV_VIDEO_NOTIFY_REASON_HARD_ERROR : video_notify_reason = 5;
//}

pub type scanline_order = c_uint;//{
    pub static XCB_XV_SCANLINE_ORDER_TOP_TO_BOTTOM : scanline_order = 1;
    pub static XCB_XV_SCANLINE_ORDER_BOTTOM_TO_TOP : scanline_order = 2;
//}

pub type grab_port_status = c_uint;//{
    pub static XCB_XV_GRAB_PORT_STATUS_SUCCESS : grab_port_status = 1;
    pub static XCB_XV_GRAB_PORT_STATUS_BAD_EXTENSION : grab_port_status = 2;
    pub static XCB_XV_GRAB_PORT_STATUS_ALREADY_GRABBED : grab_port_status = 3;
    pub static XCB_XV_GRAB_PORT_STATUS_INVALID_TIME : grab_port_status = 4;
    pub static XCB_XV_GRAB_PORT_STATUS_BAD_REPLY : grab_port_status = 5;
    pub static XCB_XV_GRAB_PORT_STATUS_BAD_ALLOC : grab_port_status = 6;
//}
pub struct Rational {pub base : base::Struct<rational> }

pub type RationalIterator = rational_iterator;

pub type FormatIterator = format_iterator;

pub type AdaptorInfoIterator = adaptor_info_iterator;

pub type EncodingInfoIterator = encoding_info_iterator;

pub type ImageIterator = image_iterator;

pub type AttributeInfoIterator = attribute_info_iterator;

pub type ImageFormatInfoIterator = image_format_info_iterator;

/** Opcode for xcb_xv_bad_port. */
pub static XCB_XV_BAD_PORT : u8 = 0;
pub struct BadPortError { pub base : base::Error<bad_port_error> }
/** Opcode for xcb_xv_bad_encoding. */
pub static XCB_XV_BAD_ENCODING : u8 = 1;
pub struct BadEncodingError { pub base : base::Error<bad_encoding_error> }
/** Opcode for xcb_xv_bad_control. */
pub static XCB_XV_BAD_CONTROL : u8 = 2;
pub struct BadControlError { pub base : base::Error<bad_control_error> }
/** Opcode for xcb_xv_video_notify. */
pub static XCB_XV_VIDEO_NOTIFY : u8 = 0;
pub struct VideoNotifyEvent {pub base : base::Event<video_notify_event>}
/** Opcode for xcb_xv_port_notify. */
pub static XCB_XV_PORT_NOTIFY : u8 = 1;
pub struct PortNotifyEvent {pub base : base::Event<port_notify_event>}
pub struct  QueryExtensionCookie<'s> { pub base : base::Cookie<'s, query_extension_cookie> }

/** Opcode for xcb_xv_query_extension. */
pub static XCB_XV_QUERY_EXTENSION : u8 = 0;
pub struct QueryExtensionReply { base:  base::Reply<query_extension_reply> }
fn mk_reply_query_extension_reply(reply:*mut query_extension_reply) -> QueryExtensionReply { QueryExtensionReply { base : base::mk_reply(reply) } }
pub struct  QueryAdaptorsCookie<'s> { pub base : base::Cookie<'s, query_adaptors_cookie> }

/** Opcode for xcb_xv_query_adaptors. */
pub static XCB_XV_QUERY_ADAPTORS : u8 = 1;
pub struct  QueryEncodingsCookie<'s> { pub base : base::Cookie<'s, query_encodings_cookie> }

/** Opcode for xcb_xv_query_encodings. */
pub static XCB_XV_QUERY_ENCODINGS : u8 = 2;
pub struct  GrabPortCookie<'s> { pub base : base::Cookie<'s, grab_port_cookie> }

/** Opcode for xcb_xv_grab_port. */
pub static XCB_XV_GRAB_PORT : u8 = 3;
pub struct GrabPortReply { base:  base::Reply<grab_port_reply> }
fn mk_reply_grab_port_reply(reply:*mut grab_port_reply) -> GrabPortReply { GrabPortReply { base : base::mk_reply(reply) } }
/** Opcode for xcb_xv_ungrab_port. */
pub static XCB_XV_UNGRAB_PORT : u8 = 4;
/** Opcode for xcb_xv_put_video. */
pub static XCB_XV_PUT_VIDEO : u8 = 5;
/** Opcode for xcb_xv_put_still. */
pub static XCB_XV_PUT_STILL : u8 = 6;
/** Opcode for xcb_xv_get_video. */
pub static XCB_XV_GET_VIDEO : u8 = 7;
/** Opcode for xcb_xv_get_still. */
pub static XCB_XV_GET_STILL : u8 = 8;
/** Opcode for xcb_xv_stop_video. */
pub static XCB_XV_STOP_VIDEO : u8 = 9;
/** Opcode for xcb_xv_select_video_notify. */
pub static XCB_XV_SELECT_VIDEO_NOTIFY : u8 = 10;
/** Opcode for xcb_xv_select_port_notify. */
pub static XCB_XV_SELECT_PORT_NOTIFY : u8 = 11;
pub struct  QueryBestSizeCookie<'s> { pub base : base::Cookie<'s, query_best_size_cookie> }

/** Opcode for xcb_xv_query_best_size. */
pub static XCB_XV_QUERY_BEST_SIZE : u8 = 12;
pub struct QueryBestSizeReply { base:  base::Reply<query_best_size_reply> }
fn mk_reply_query_best_size_reply(reply:*mut query_best_size_reply) -> QueryBestSizeReply { QueryBestSizeReply { base : base::mk_reply(reply) } }
/** Opcode for xcb_xv_set_port_attribute. */
pub static XCB_XV_SET_PORT_ATTRIBUTE : u8 = 13;
pub struct  GetPortAttributeCookie<'s> { pub base : base::Cookie<'s, get_port_attribute_cookie> }

/** Opcode for xcb_xv_get_port_attribute. */
pub static XCB_XV_GET_PORT_ATTRIBUTE : u8 = 14;
pub struct GetPortAttributeReply { base:  base::Reply<get_port_attribute_reply> }
fn mk_reply_get_port_attribute_reply(reply:*mut get_port_attribute_reply) -> GetPortAttributeReply { GetPortAttributeReply { base : base::mk_reply(reply) } }
pub struct  QueryPortAttributesCookie<'s> { pub base : base::Cookie<'s, query_port_attributes_cookie> }

/** Opcode for xcb_xv_query_port_attributes. */
pub static XCB_XV_QUERY_PORT_ATTRIBUTES : u8 = 15;
pub struct  ListImageFormatsCookie<'s> { pub base : base::Cookie<'s, list_image_formats_cookie> }

/** Opcode for xcb_xv_list_image_formats. */
pub static XCB_XV_LIST_IMAGE_FORMATS : u8 = 16;
pub struct  QueryImageAttributesCookie<'s> { pub base : base::Cookie<'s, query_image_attributes_cookie> }

/** Opcode for xcb_xv_query_image_attributes. */
pub static XCB_XV_QUERY_IMAGE_ATTRIBUTES : u8 = 17;
/** Opcode for xcb_xv_put_image. */
pub static XCB_XV_PUT_IMAGE : u8 = 18;
/** Opcode for xcb_xv_shm_put_image. */
pub static XCB_XV_SHM_PUT_IMAGE : u8 = 19;

impl Iterator for PortIterator {
    type Item = Port;
    fn next(&mut self) -> Option<Port> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut port_iterator = mem::transmute(self);
            let data = (*iter).data;
            xcb_xv_port_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub type Encoding = encoding;


impl Iterator for EncodingIterator {
    type Item = Encoding;
    fn next(&mut self) -> Option<Encoding> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut encoding_iterator = mem::transmute(self);
            let data = (*iter).data;
            xcb_xv_encoding_next(iter);
            Some(mem::transmute(*data))
        }
    }
}


impl Rational {
  pub fn numerator(&mut self) -> i32 {
    unsafe { accessor!(numerator -> i32, self.base.strct) }
  }

  pub fn denominator(&mut self) -> i32 {
    unsafe { accessor!(denominator -> i32, self.base.strct) }
  }

}

impl Iterator for RationalIterator {
    type Item = Rational;
    fn next(&mut self) -> Option<Rational> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut rational_iterator = mem::transmute(self);
            let data = (*iter).data;
            xcb_xv_rational_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct Format {pub base : base::Struct<format> }


impl Format {
  pub fn visual(&mut self) -> xproto::Visualid {
    unsafe { accessor!(visual -> xproto::Visualid, self.base.strct) }
  }

  pub fn depth(&mut self) -> u8 {
    unsafe { accessor!(depth -> u8, self.base.strct) }
  }

}

impl Iterator for FormatIterator {
    type Item = Format;
    fn next(&mut self) -> Option<Format> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut format_iterator = mem::transmute(self);
            let data = (*iter).data;
            xcb_xv_format_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct AdaptorInfo {pub base : base::Struct<adaptor_info> }


impl AdaptorInfo {
  pub fn base_id(&mut self) -> Port {
    unsafe { accessor!(base_id -> Port, self.base.strct) }
  }

  pub fn num_ports(&mut self) -> u16 {
    unsafe { accessor!(num_ports -> u16, self.base.strct) }
  }

  pub fn type_(&mut self) -> u8 {
    unsafe { accessor!(type_ -> u8, self.base.strct) }
  }

  pub fn name(&mut self) -> String {
    unsafe { accessor!(str, xcb_xv_adaptor_info_name_length, xcb_xv_adaptor_info_name, self.base.strct) }
  }

  pub fn formats(&mut self) -> FormatIterator {
    unsafe { accessor!(FormatIterator, xcb_xv_adaptor_info_formats_iterator, self.base.strct) }
  }

}

impl Iterator for AdaptorInfoIterator {
    type Item = AdaptorInfo;
    fn next(&mut self) -> Option<AdaptorInfo> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut adaptor_info_iterator = mem::transmute(self);
            let data = (*iter).data;
            xcb_xv_adaptor_info_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct EncodingInfo {pub base : base::Struct<encoding_info> }


impl EncodingInfo {
  pub fn encoding(&mut self) -> Encoding {
    unsafe { accessor!(encoding -> Encoding, self.base.strct) }
  }

  pub fn width(&mut self) -> u16 {
    unsafe { accessor!(width -> u16, self.base.strct) }
  }

  pub fn height(&mut self) -> u16 {
    unsafe { accessor!(height -> u16, self.base.strct) }
  }

  pub fn rate(&self) -> Rational {
    unsafe { mem::transmute(self.base.strct.rate) }
  }
  pub fn name(&mut self) -> String {
    unsafe { accessor!(str, xcb_xv_encoding_info_name_length, xcb_xv_encoding_info_name, self.base.strct) }
  }

}

impl Iterator for EncodingInfoIterator {
    type Item = EncodingInfo;
    fn next(&mut self) -> Option<EncodingInfo> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut encoding_info_iterator = mem::transmute(self);
            let data = (*iter).data;
            xcb_xv_encoding_info_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct Image {pub base : base::Struct<image> }


impl Image {
  pub fn id(&mut self) -> u32 {
    unsafe { accessor!(id -> u32, self.base.strct) }
  }

  pub fn width(&mut self) -> u16 {
    unsafe { accessor!(width -> u16, self.base.strct) }
  }

  pub fn height(&mut self) -> u16 {
    unsafe { accessor!(height -> u16, self.base.strct) }
  }

  pub fn pitches(&mut self) -> Vec<u32> {
    unsafe { accessor!(u32, xcb_xv_image_pitches_length, xcb_xv_image_pitches, self.base.strct) }
  }

  pub fn offsets(&mut self) -> Vec<u32> {
    unsafe { accessor!(u32, xcb_xv_image_offsets_length, xcb_xv_image_offsets, self.base.strct) }
  }

  pub fn data(&mut self) -> Vec<u8> {
    unsafe { accessor!(u8, xcb_xv_image_data_length, xcb_xv_image_data, self.base.strct) }
  }

}

impl Iterator for ImageIterator {
    type Item = Image;
    fn next(&mut self) -> Option<Image> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut image_iterator = mem::transmute(self);
            let data = (*iter).data;
            xcb_xv_image_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct AttributeInfo {pub base : base::Struct<attribute_info> }


impl AttributeInfo {
  pub fn flags(&mut self) -> u32 {
    unsafe { accessor!(flags -> u32, self.base.strct) }
  }

  pub fn min(&mut self) -> i32 {
    unsafe { accessor!(min -> i32, self.base.strct) }
  }

  pub fn max(&mut self) -> i32 {
    unsafe { accessor!(max -> i32, self.base.strct) }
  }

  pub fn name(&mut self) -> String {
    unsafe { accessor!(str, xcb_xv_attribute_info_name_length, xcb_xv_attribute_info_name, self.base.strct) }
  }

}

impl Iterator for AttributeInfoIterator {
    type Item = AttributeInfo;
    fn next(&mut self) -> Option<AttributeInfo> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut attribute_info_iterator = mem::transmute(self);
            let data = (*iter).data;
            xcb_xv_attribute_info_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct ImageFormatInfo {pub base : base::Struct<image_format_info> }


impl ImageFormatInfo {
  pub fn id(&mut self) -> u32 {
    unsafe { accessor!(id -> u32, self.base.strct) }
  }

  pub fn type_(&mut self) -> u8 {
    unsafe { accessor!(type_ -> u8, self.base.strct) }
  }

  pub fn byte_order(&mut self) -> u8 {
    unsafe { accessor!(byte_order -> u8, self.base.strct) }
  }

  pub fn guid(&self) -> Vec<u8> {
    unsafe { (self.base.strct.guid).to_vec() }
  }

  pub fn bpp(&mut self) -> u8 {
    unsafe { accessor!(bpp -> u8, self.base.strct) }
  }

  pub fn num_planes(&mut self) -> u8 {
    unsafe { accessor!(num_planes -> u8, self.base.strct) }
  }

  pub fn depth(&mut self) -> u8 {
    unsafe { accessor!(depth -> u8, self.base.strct) }
  }

  pub fn red_mask(&mut self) -> u32 {
    unsafe { accessor!(red_mask -> u32, self.base.strct) }
  }

  pub fn green_mask(&mut self) -> u32 {
    unsafe { accessor!(green_mask -> u32, self.base.strct) }
  }

  pub fn blue_mask(&mut self) -> u32 {
    unsafe { accessor!(blue_mask -> u32, self.base.strct) }
  }

  pub fn format(&mut self) -> u8 {
    unsafe { accessor!(format -> u8, self.base.strct) }
  }

  pub fn y_sample_bits(&mut self) -> u32 {
    unsafe { accessor!(y_sample_bits -> u32, self.base.strct) }
  }

  pub fn u_sample_bits(&mut self) -> u32 {
    unsafe { accessor!(u_sample_bits -> u32, self.base.strct) }
  }

  pub fn v_sample_bits(&mut self) -> u32 {
    unsafe { accessor!(v_sample_bits -> u32, self.base.strct) }
  }

  pub fn vhorz_y_period(&mut self) -> u32 {
    unsafe { accessor!(vhorz_y_period -> u32, self.base.strct) }
  }

  pub fn vhorz_u_period(&mut self) -> u32 {
    unsafe { accessor!(vhorz_u_period -> u32, self.base.strct) }
  }

  pub fn vhorz_v_period(&mut self) -> u32 {
    unsafe { accessor!(vhorz_v_period -> u32, self.base.strct) }
  }

  pub fn vvert_y_period(&mut self) -> u32 {
    unsafe { accessor!(vvert_y_period -> u32, self.base.strct) }
  }

  pub fn vvert_u_period(&mut self) -> u32 {
    unsafe { accessor!(vvert_u_period -> u32, self.base.strct) }
  }

  pub fn vvert_v_period(&mut self) -> u32 {
    unsafe { accessor!(vvert_v_period -> u32, self.base.strct) }
  }

  pub fn vcomp_order(&self) -> Vec<u8> {
    unsafe { (self.base.strct.vcomp_order).to_vec() }
  }

  pub fn vscanline_order(&mut self) -> u8 {
    unsafe { accessor!(vscanline_order -> u8, self.base.strct) }
  }

}

impl Iterator for ImageFormatInfoIterator {
    type Item = ImageFormatInfo;
    fn next(&mut self) -> Option<ImageFormatInfo> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut image_format_info_iterator = mem::transmute(self);
            let data = (*iter).data;
            xcb_xv_image_format_info_next(iter);
            Some(mem::transmute(*data))
        }
    }
}


impl VideoNotifyEvent {
  pub fn reason(&mut self) -> u8 {
    unsafe { accessor!(reason -> u8, (*self.base.event)) }
  }

  pub fn time(&mut self) -> xproto::Timestamp {
    unsafe { accessor!(time -> xproto::Timestamp, (*self.base.event)) }
  }

  pub fn drawable(&mut self) -> xproto::Drawable {
    unsafe { accessor!(drawable -> xproto::Drawable, (*self.base.event)) }
  }

  pub fn port(&mut self) -> Port {
    unsafe { accessor!(port -> Port, (*self.base.event)) }
  }

  pub fn new(reason : u8,
         time : xproto::Timestamp,
         drawable : xproto::Drawable,
         port : Port) -> VideoNotifyEvent {
    unsafe {
      let raw = malloc(32 as size_t) as *mut video_notify_event;
      (*raw).reason = reason;
      (*raw).time = time;
      (*raw).drawable = drawable;
      (*raw).port = port;
      VideoNotifyEvent { base : Event { event : raw as *mut video_notify_event }}
    }
  }
}

impl PortNotifyEvent {
  pub fn time(&mut self) -> xproto::Timestamp {
    unsafe { accessor!(time -> xproto::Timestamp, (*self.base.event)) }
  }

  pub fn port(&mut self) -> Port {
    unsafe { accessor!(port -> Port, (*self.base.event)) }
  }

  pub fn attribute(&mut self) -> xproto::Atom {
    unsafe { accessor!(attribute -> xproto::Atom, (*self.base.event)) }
  }

  pub fn value(&mut self) -> i32 {
    unsafe { accessor!(value -> i32, (*self.base.event)) }
  }

  pub fn new(time : xproto::Timestamp,
         port : Port,
         attribute : xproto::Atom,
         value : i32) -> PortNotifyEvent {
    unsafe {
      let raw = malloc(32 as size_t) as *mut port_notify_event;
      (*raw).time = time;
      (*raw).port = port;
      (*raw).attribute = attribute;
      (*raw).value = value;
      PortNotifyEvent { base : Event { event : raw as *mut port_notify_event }}
    }
  }
}
pub fn QueryExtension<'r> (c : &'r Connection) -> QueryExtensionCookie<'r> {
  unsafe {
    let cookie = xcb_xv_query_extension(c.get_raw_conn());
    QueryExtensionCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn QueryExtensionUnchecked<'r> (c : &'r Connection) -> QueryExtensionCookie<'r> {
  unsafe {
    let cookie = xcb_xv_query_extension_unchecked(c.get_raw_conn());
    QueryExtensionCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl QueryExtensionReply {
  pub fn major(&mut self) -> u16 {
    unsafe { accessor!(major -> u16, (*self.base.reply)) }
  }

  pub fn minor(&mut self) -> u16 {
    unsafe { accessor!(minor -> u16, (*self.base.reply)) }
  }

}
impl_reply_cookie!(QueryExtensionCookie<'s>, mk_reply_query_extension_reply, QueryExtensionReply, xcb_xv_query_extension_reply);

pub struct QueryAdaptorsReply { base:  base::Reply<query_adaptors_reply> }
fn mk_reply_query_adaptors_reply(reply:*mut query_adaptors_reply) -> QueryAdaptorsReply { QueryAdaptorsReply { base : base::mk_reply(reply) } }
pub fn QueryAdaptors<'r> (c : &'r Connection,
                      window : xproto::Window) -> QueryAdaptorsCookie<'r> {
  unsafe {
    let cookie = xcb_xv_query_adaptors(c.get_raw_conn(),
        window as ffi::xproto::window); //1
    QueryAdaptorsCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn QueryAdaptorsUnchecked<'r> (c : &'r Connection,
                               window : xproto::Window) -> QueryAdaptorsCookie<'r> {
  unsafe {
    let cookie = xcb_xv_query_adaptors_unchecked(c.get_raw_conn(),
        window as ffi::xproto::window); //1
    QueryAdaptorsCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl QueryAdaptorsReply {
  pub fn info(&mut self) -> AdaptorInfoIterator {
    unsafe { accessor!(AdaptorInfoIterator, xcb_xv_query_adaptors_info_iterator, (*self.base.reply)) }
  }

}
impl_reply_cookie!(QueryAdaptorsCookie<'s>, mk_reply_query_adaptors_reply, QueryAdaptorsReply, xcb_xv_query_adaptors_reply);

pub struct QueryEncodingsReply { base:  base::Reply<query_encodings_reply> }
fn mk_reply_query_encodings_reply(reply:*mut query_encodings_reply) -> QueryEncodingsReply { QueryEncodingsReply { base : base::mk_reply(reply) } }
pub fn QueryEncodings<'r> (c : &'r Connection,
                       port : Port) -> QueryEncodingsCookie<'r> {
  unsafe {
    let cookie = xcb_xv_query_encodings(c.get_raw_conn(),
        port as port); //1
    QueryEncodingsCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn QueryEncodingsUnchecked<'r> (c : &'r Connection,
                                port : Port) -> QueryEncodingsCookie<'r> {
  unsafe {
    let cookie = xcb_xv_query_encodings_unchecked(c.get_raw_conn(),
        port as port); //1
    QueryEncodingsCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl QueryEncodingsReply {
  pub fn info(&mut self) -> EncodingInfoIterator {
    unsafe { accessor!(EncodingInfoIterator, xcb_xv_query_encodings_info_iterator, (*self.base.reply)) }
  }

}
impl_reply_cookie!(QueryEncodingsCookie<'s>, mk_reply_query_encodings_reply, QueryEncodingsReply, xcb_xv_query_encodings_reply);

pub fn GrabPort<'r> (c : &'r Connection,
                 port : Port,
                 time : xproto::Timestamp) -> GrabPortCookie<'r> {
  unsafe {
    let cookie = xcb_xv_grab_port(c.get_raw_conn(),
        port as port, //1
        time as ffi::xproto::timestamp); //2
    GrabPortCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GrabPortUnchecked<'r> (c : &'r Connection,
                          port : Port,
                          time : xproto::Timestamp) -> GrabPortCookie<'r> {
  unsafe {
    let cookie = xcb_xv_grab_port_unchecked(c.get_raw_conn(),
        port as port, //1
        time as ffi::xproto::timestamp); //2
    GrabPortCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GrabPortReply {
  pub fn result(&mut self) -> u8 {
    unsafe { accessor!(result -> u8, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GrabPortCookie<'s>, mk_reply_grab_port_reply, GrabPortReply, xcb_xv_grab_port_reply);

pub fn UngrabPortChecked<'r> (c : &'r Connection,
                          port : Port,
                          time : xproto::Timestamp) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xv_ungrab_port_checked(c.get_raw_conn(),
        port as port, //1
        time as ffi::xproto::timestamp); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn UngrabPort<'r> (c : &'r Connection,
                   port : Port,
                   time : xproto::Timestamp) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xv_ungrab_port(c.get_raw_conn(),
        port as port, //1
        time as ffi::xproto::timestamp); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn PutVideoChecked<'r> (c : &'r Connection,
                        port : Port,
                        drawable : xproto::Drawable,
                        gc : xproto::Gcontext,
                        vid_x : i16,
                        vid_y : i16,
                        vid_w : u16,
                        vid_h : u16,
                        drw_x : i16,
                        drw_y : i16,
                        drw_w : u16,
                        drw_h : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xv_put_video_checked(c.get_raw_conn(),
        port as port, //1
        drawable as ffi::xproto::drawable, //2
        gc as ffi::xproto::gcontext, //3
        vid_x as i16, //4
        vid_y as i16, //5
        vid_w as u16, //6
        vid_h as u16, //7
        drw_x as i16, //8
        drw_y as i16, //9
        drw_w as u16, //10
        drw_h as u16); //11
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn PutVideo<'r> (c : &'r Connection,
                 port : Port,
                 drawable : xproto::Drawable,
                 gc : xproto::Gcontext,
                 vid_x : i16,
                 vid_y : i16,
                 vid_w : u16,
                 vid_h : u16,
                 drw_x : i16,
                 drw_y : i16,
                 drw_w : u16,
                 drw_h : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xv_put_video(c.get_raw_conn(),
        port as port, //1
        drawable as ffi::xproto::drawable, //2
        gc as ffi::xproto::gcontext, //3
        vid_x as i16, //4
        vid_y as i16, //5
        vid_w as u16, //6
        vid_h as u16, //7
        drw_x as i16, //8
        drw_y as i16, //9
        drw_w as u16, //10
        drw_h as u16); //11
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn PutStillChecked<'r> (c : &'r Connection,
                        port : Port,
                        drawable : xproto::Drawable,
                        gc : xproto::Gcontext,
                        vid_x : i16,
                        vid_y : i16,
                        vid_w : u16,
                        vid_h : u16,
                        drw_x : i16,
                        drw_y : i16,
                        drw_w : u16,
                        drw_h : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xv_put_still_checked(c.get_raw_conn(),
        port as port, //1
        drawable as ffi::xproto::drawable, //2
        gc as ffi::xproto::gcontext, //3
        vid_x as i16, //4
        vid_y as i16, //5
        vid_w as u16, //6
        vid_h as u16, //7
        drw_x as i16, //8
        drw_y as i16, //9
        drw_w as u16, //10
        drw_h as u16); //11
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn PutStill<'r> (c : &'r Connection,
                 port : Port,
                 drawable : xproto::Drawable,
                 gc : xproto::Gcontext,
                 vid_x : i16,
                 vid_y : i16,
                 vid_w : u16,
                 vid_h : u16,
                 drw_x : i16,
                 drw_y : i16,
                 drw_w : u16,
                 drw_h : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xv_put_still(c.get_raw_conn(),
        port as port, //1
        drawable as ffi::xproto::drawable, //2
        gc as ffi::xproto::gcontext, //3
        vid_x as i16, //4
        vid_y as i16, //5
        vid_w as u16, //6
        vid_h as u16, //7
        drw_x as i16, //8
        drw_y as i16, //9
        drw_w as u16, //10
        drw_h as u16); //11
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetVideoChecked<'r> (c : &'r Connection,
                        port : Port,
                        drawable : xproto::Drawable,
                        gc : xproto::Gcontext,
                        vid_x : i16,
                        vid_y : i16,
                        vid_w : u16,
                        vid_h : u16,
                        drw_x : i16,
                        drw_y : i16,
                        drw_w : u16,
                        drw_h : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xv_get_video_checked(c.get_raw_conn(),
        port as port, //1
        drawable as ffi::xproto::drawable, //2
        gc as ffi::xproto::gcontext, //3
        vid_x as i16, //4
        vid_y as i16, //5
        vid_w as u16, //6
        vid_h as u16, //7
        drw_x as i16, //8
        drw_y as i16, //9
        drw_w as u16, //10
        drw_h as u16); //11
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn GetVideo<'r> (c : &'r Connection,
                 port : Port,
                 drawable : xproto::Drawable,
                 gc : xproto::Gcontext,
                 vid_x : i16,
                 vid_y : i16,
                 vid_w : u16,
                 vid_h : u16,
                 drw_x : i16,
                 drw_y : i16,
                 drw_w : u16,
                 drw_h : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xv_get_video(c.get_raw_conn(),
        port as port, //1
        drawable as ffi::xproto::drawable, //2
        gc as ffi::xproto::gcontext, //3
        vid_x as i16, //4
        vid_y as i16, //5
        vid_w as u16, //6
        vid_h as u16, //7
        drw_x as i16, //8
        drw_y as i16, //9
        drw_w as u16, //10
        drw_h as u16); //11
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetStillChecked<'r> (c : &'r Connection,
                        port : Port,
                        drawable : xproto::Drawable,
                        gc : xproto::Gcontext,
                        vid_x : i16,
                        vid_y : i16,
                        vid_w : u16,
                        vid_h : u16,
                        drw_x : i16,
                        drw_y : i16,
                        drw_w : u16,
                        drw_h : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xv_get_still_checked(c.get_raw_conn(),
        port as port, //1
        drawable as ffi::xproto::drawable, //2
        gc as ffi::xproto::gcontext, //3
        vid_x as i16, //4
        vid_y as i16, //5
        vid_w as u16, //6
        vid_h as u16, //7
        drw_x as i16, //8
        drw_y as i16, //9
        drw_w as u16, //10
        drw_h as u16); //11
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn GetStill<'r> (c : &'r Connection,
                 port : Port,
                 drawable : xproto::Drawable,
                 gc : xproto::Gcontext,
                 vid_x : i16,
                 vid_y : i16,
                 vid_w : u16,
                 vid_h : u16,
                 drw_x : i16,
                 drw_y : i16,
                 drw_w : u16,
                 drw_h : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xv_get_still(c.get_raw_conn(),
        port as port, //1
        drawable as ffi::xproto::drawable, //2
        gc as ffi::xproto::gcontext, //3
        vid_x as i16, //4
        vid_y as i16, //5
        vid_w as u16, //6
        vid_h as u16, //7
        drw_x as i16, //8
        drw_y as i16, //9
        drw_w as u16, //10
        drw_h as u16); //11
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn StopVideoChecked<'r> (c : &'r Connection,
                         port : Port,
                         drawable : xproto::Drawable) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xv_stop_video_checked(c.get_raw_conn(),
        port as port, //1
        drawable as ffi::xproto::drawable); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn StopVideo<'r> (c : &'r Connection,
                  port : Port,
                  drawable : xproto::Drawable) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xv_stop_video(c.get_raw_conn(),
        port as port, //1
        drawable as ffi::xproto::drawable); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn SelectVideoNotifyChecked<'r> (c : &'r Connection,
                                 drawable : xproto::Drawable,
                                 onoff : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xv_select_video_notify_checked(c.get_raw_conn(),
        drawable as ffi::xproto::drawable, //1
        onoff as u8); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn SelectVideoNotify<'r> (c : &'r Connection,
                          drawable : xproto::Drawable,
                          onoff : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xv_select_video_notify(c.get_raw_conn(),
        drawable as ffi::xproto::drawable, //1
        onoff as u8); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn SelectPortNotifyChecked<'r> (c : &'r Connection,
                                port : Port,
                                onoff : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xv_select_port_notify_checked(c.get_raw_conn(),
        port as port, //1
        onoff as u8); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn SelectPortNotify<'r> (c : &'r Connection,
                         port : Port,
                         onoff : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xv_select_port_notify(c.get_raw_conn(),
        port as port, //1
        onoff as u8); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn QueryBestSize<'r> (c : &'r Connection,
                      port : Port,
                      vid_w : u16,
                      vid_h : u16,
                      drw_w : u16,
                      drw_h : u16,
                      motion : u8) -> QueryBestSizeCookie<'r> {
  unsafe {
    let cookie = xcb_xv_query_best_size(c.get_raw_conn(),
        port as port, //1
        vid_w as u16, //2
        vid_h as u16, //3
        drw_w as u16, //4
        drw_h as u16, //5
        motion as u8); //6
    QueryBestSizeCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn QueryBestSizeUnchecked<'r> (c : &'r Connection,
                               port : Port,
                               vid_w : u16,
                               vid_h : u16,
                               drw_w : u16,
                               drw_h : u16,
                               motion : u8) -> QueryBestSizeCookie<'r> {
  unsafe {
    let cookie = xcb_xv_query_best_size_unchecked(c.get_raw_conn(),
        port as port, //1
        vid_w as u16, //2
        vid_h as u16, //3
        drw_w as u16, //4
        drw_h as u16, //5
        motion as u8); //6
    QueryBestSizeCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl QueryBestSizeReply {
  pub fn actual_width(&mut self) -> u16 {
    unsafe { accessor!(actual_width -> u16, (*self.base.reply)) }
  }

  pub fn actual_height(&mut self) -> u16 {
    unsafe { accessor!(actual_height -> u16, (*self.base.reply)) }
  }

}
impl_reply_cookie!(QueryBestSizeCookie<'s>, mk_reply_query_best_size_reply, QueryBestSizeReply, xcb_xv_query_best_size_reply);

pub fn SetPortAttributeChecked<'r> (c : &'r Connection,
                                port : Port,
                                attribute : xproto::Atom,
                                value : i32) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xv_set_port_attribute_checked(c.get_raw_conn(),
        port as port, //1
        attribute as ffi::xproto::atom, //2
        value as i32); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn SetPortAttribute<'r> (c : &'r Connection,
                         port : Port,
                         attribute : xproto::Atom,
                         value : i32) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xv_set_port_attribute(c.get_raw_conn(),
        port as port, //1
        attribute as ffi::xproto::atom, //2
        value as i32); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetPortAttribute<'r> (c : &'r Connection,
                         port : Port,
                         attribute : xproto::Atom) -> GetPortAttributeCookie<'r> {
  unsafe {
    let cookie = xcb_xv_get_port_attribute(c.get_raw_conn(),
        port as port, //1
        attribute as ffi::xproto::atom); //2
    GetPortAttributeCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetPortAttributeUnchecked<'r> (c : &'r Connection,
                                  port : Port,
                                  attribute : xproto::Atom) -> GetPortAttributeCookie<'r> {
  unsafe {
    let cookie = xcb_xv_get_port_attribute_unchecked(c.get_raw_conn(),
        port as port, //1
        attribute as ffi::xproto::atom); //2
    GetPortAttributeCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetPortAttributeReply {
  pub fn value(&mut self) -> i32 {
    unsafe { accessor!(value -> i32, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetPortAttributeCookie<'s>, mk_reply_get_port_attribute_reply, GetPortAttributeReply, xcb_xv_get_port_attribute_reply);

pub struct QueryPortAttributesReply { base:  base::Reply<query_port_attributes_reply> }
fn mk_reply_query_port_attributes_reply(reply:*mut query_port_attributes_reply) -> QueryPortAttributesReply { QueryPortAttributesReply { base : base::mk_reply(reply) } }
pub fn QueryPortAttributes<'r> (c : &'r Connection,
                            port : Port) -> QueryPortAttributesCookie<'r> {
  unsafe {
    let cookie = xcb_xv_query_port_attributes(c.get_raw_conn(),
        port as port); //1
    QueryPortAttributesCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn QueryPortAttributesUnchecked<'r> (c : &'r Connection,
                                     port : Port) -> QueryPortAttributesCookie<'r> {
  unsafe {
    let cookie = xcb_xv_query_port_attributes_unchecked(c.get_raw_conn(),
        port as port); //1
    QueryPortAttributesCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl QueryPortAttributesReply {
  pub fn text_size(&mut self) -> u32 {
    unsafe { accessor!(text_size -> u32, (*self.base.reply)) }
  }

  pub fn attributes(&mut self) -> AttributeInfoIterator {
    unsafe { accessor!(AttributeInfoIterator, xcb_xv_query_port_attributes_attributes_iterator, (*self.base.reply)) }
  }

}
impl_reply_cookie!(QueryPortAttributesCookie<'s>, mk_reply_query_port_attributes_reply, QueryPortAttributesReply, xcb_xv_query_port_attributes_reply);

pub struct ListImageFormatsReply { base:  base::Reply<list_image_formats_reply> }
fn mk_reply_list_image_formats_reply(reply:*mut list_image_formats_reply) -> ListImageFormatsReply { ListImageFormatsReply { base : base::mk_reply(reply) } }
pub fn ListImageFormats<'r> (c : &'r Connection,
                         port : Port) -> ListImageFormatsCookie<'r> {
  unsafe {
    let cookie = xcb_xv_list_image_formats(c.get_raw_conn(),
        port as port); //1
    ListImageFormatsCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn ListImageFormatsUnchecked<'r> (c : &'r Connection,
                                  port : Port) -> ListImageFormatsCookie<'r> {
  unsafe {
    let cookie = xcb_xv_list_image_formats_unchecked(c.get_raw_conn(),
        port as port); //1
    ListImageFormatsCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl ListImageFormatsReply {
  pub fn format(&mut self) -> ImageFormatInfoIterator {
    unsafe { accessor!(ImageFormatInfoIterator, xcb_xv_list_image_formats_format_iterator, (*self.base.reply)) }
  }

}
impl_reply_cookie!(ListImageFormatsCookie<'s>, mk_reply_list_image_formats_reply, ListImageFormatsReply, xcb_xv_list_image_formats_reply);

pub struct QueryImageAttributesReply { base:  base::Reply<query_image_attributes_reply> }
fn mk_reply_query_image_attributes_reply(reply:*mut query_image_attributes_reply) -> QueryImageAttributesReply { QueryImageAttributesReply { base : base::mk_reply(reply) } }
pub fn QueryImageAttributes<'r> (c : &'r Connection,
                             port : Port,
                             id : u32,
                             width : u16,
                             height : u16) -> QueryImageAttributesCookie<'r> {
  unsafe {
    let cookie = xcb_xv_query_image_attributes(c.get_raw_conn(),
        port as port, //1
        id as u32, //2
        width as u16, //3
        height as u16); //4
    QueryImageAttributesCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn QueryImageAttributesUnchecked<'r> (c : &'r Connection,
                                      port : Port,
                                      id : u32,
                                      width : u16,
                                      height : u16) -> QueryImageAttributesCookie<'r> {
  unsafe {
    let cookie = xcb_xv_query_image_attributes_unchecked(c.get_raw_conn(),
        port as port, //1
        id as u32, //2
        width as u16, //3
        height as u16); //4
    QueryImageAttributesCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl QueryImageAttributesReply {
  pub fn data_size(&mut self) -> u32 {
    unsafe { accessor!(data_size -> u32, (*self.base.reply)) }
  }

  pub fn width(&mut self) -> u16 {
    unsafe { accessor!(width -> u16, (*self.base.reply)) }
  }

  pub fn height(&mut self) -> u16 {
    unsafe { accessor!(height -> u16, (*self.base.reply)) }
  }

  pub fn pitches(&mut self) -> Vec<u32> {
    unsafe { accessor!(u32, xcb_xv_query_image_attributes_pitches_length, xcb_xv_query_image_attributes_pitches, (*self.base.reply)) }
  }

  pub fn offsets(&mut self) -> Vec<u32> {
    unsafe { accessor!(u32, xcb_xv_query_image_attributes_offsets_length, xcb_xv_query_image_attributes_offsets, (*self.base.reply)) }
  }

}
impl_reply_cookie!(QueryImageAttributesCookie<'s>, mk_reply_query_image_attributes_reply, QueryImageAttributesReply, xcb_xv_query_image_attributes_reply);

pub fn PutImageChecked<'r> (c : &'r Connection,
                        port : Port,
                        drawable : xproto::Drawable,
                        gc : xproto::Gcontext,
                        id : u32,
                        src_x : i16,
                        src_y : i16,
                        src_w : u16,
                        src_h : u16,
                        drw_x : i16,
                        drw_y : i16,
                        drw_w : u16,
                        drw_h : u16,
                        width : u16,
                        height : u16,
                        data : &[u8]) -> base::VoidCookie<'r> {
  unsafe {
    let data_len = data.len();
    let data_ptr = data.as_ptr();
    let cookie = xcb_xv_put_image_checked(c.get_raw_conn(),
        port as port, //1
        drawable as ffi::xproto::drawable, //2
        gc as ffi::xproto::gcontext, //3
        id as u32, //4
        src_x as i16, //5
        src_y as i16, //6
        src_w as u16, //7
        src_h as u16, //8
        drw_x as i16, //9
        drw_y as i16, //10
        drw_w as u16, //11
        drw_h as u16, //12
        width as u16, //13
        height as u16, //14
        data_len as u32, //15
        data_ptr as *mut u8); //16
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn PutImage<'r> (c : &'r Connection,
                 port : Port,
                 drawable : xproto::Drawable,
                 gc : xproto::Gcontext,
                 id : u32,
                 src_x : i16,
                 src_y : i16,
                 src_w : u16,
                 src_h : u16,
                 drw_x : i16,
                 drw_y : i16,
                 drw_w : u16,
                 drw_h : u16,
                 width : u16,
                 height : u16,
                 data : &[u8]) -> base::VoidCookie<'r> {
  unsafe {
    let data_len = data.len();
    let data_ptr = data.as_ptr();
    let cookie = xcb_xv_put_image(c.get_raw_conn(),
        port as port, //1
        drawable as ffi::xproto::drawable, //2
        gc as ffi::xproto::gcontext, //3
        id as u32, //4
        src_x as i16, //5
        src_y as i16, //6
        src_w as u16, //7
        src_h as u16, //8
        drw_x as i16, //9
        drw_y as i16, //10
        drw_w as u16, //11
        drw_h as u16, //12
        width as u16, //13
        height as u16, //14
        data_len as u32, //15
        data_ptr as *mut u8); //16
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn ShmPutImageChecked<'r> (c : &'r Connection,
                           port : Port,
                           drawable : xproto::Drawable,
                           gc : xproto::Gcontext,
                           shmseg : shm::Seg,
                           id : u32,
                           offset : u32,
                           src_x : i16,
                           src_y : i16,
                           src_w : u16,
                           src_h : u16,
                           drw_x : i16,
                           drw_y : i16,
                           drw_w : u16,
                           drw_h : u16,
                           width : u16,
                           height : u16,
                           send_event : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xv_shm_put_image_checked(c.get_raw_conn(),
        port as port, //1
        drawable as ffi::xproto::drawable, //2
        gc as ffi::xproto::gcontext, //3
        shmseg as ffi::shm::seg, //4
        id as u32, //5
        offset as u32, //6
        src_x as i16, //7
        src_y as i16, //8
        src_w as u16, //9
        src_h as u16, //10
        drw_x as i16, //11
        drw_y as i16, //12
        drw_w as u16, //13
        drw_h as u16, //14
        width as u16, //15
        height as u16, //16
        send_event as u8); //17
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn ShmPutImage<'r> (c : &'r Connection,
                    port : Port,
                    drawable : xproto::Drawable,
                    gc : xproto::Gcontext,
                    shmseg : shm::Seg,
                    id : u32,
                    offset : u32,
                    src_x : i16,
                    src_y : i16,
                    src_w : u16,
                    src_h : u16,
                    drw_x : i16,
                    drw_y : i16,
                    drw_w : u16,
                    drw_h : u16,
                    width : u16,
                    height : u16,
                    send_event : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xv_shm_put_image(c.get_raw_conn(),
        port as port, //1
        drawable as ffi::xproto::drawable, //2
        gc as ffi::xproto::gcontext, //3
        shmseg as ffi::shm::seg, //4
        id as u32, //5
        offset as u32, //6
        src_x as i16, //7
        src_y as i16, //8
        src_w as u16, //9
        src_h as u16, //10
        drw_x as i16, //11
        drw_y as i16, //12
        drw_w as u16, //13
        drw_h as u16, //14
        width as u16, //15
        height as u16, //16
        send_event as u8); //17
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

