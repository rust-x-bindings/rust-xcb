/*
 * This file generated automatically from shape.xml by r_client.py.
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
use ffi::shape::*;
use std::option::Option;
use std::iter::Iterator;

use xproto;
pub type Op = xcb_shape_op_t;

pub type OpIterator = xcb_shape_op_iterator_t;

pub type KindIterator = xcb_shape_kind_iterator_t;


pub type xcb_shape_so_t = c_uint;//{
    pub static XCB_SHAPE_SO_SET : xcb_shape_so_t = 1;
    pub static XCB_SHAPE_SO_UNION : xcb_shape_so_t = 2;
    pub static XCB_SHAPE_SO_INTERSECT : xcb_shape_so_t = 3;
    pub static XCB_SHAPE_SO_SUBTRACT : xcb_shape_so_t = 4;
    pub static XCB_SHAPE_SO_INVERT : xcb_shape_so_t = 5;
//}

pub type xcb_shape_sk_t = c_uint;//{
    pub static XCB_SHAPE_SK_BOUNDING : xcb_shape_sk_t = 1;
    pub static XCB_SHAPE_SK_CLIP : xcb_shape_sk_t = 2;
    pub static XCB_SHAPE_SK_INPUT : xcb_shape_sk_t = 3;
//}
/** Opcode for xcb_shape_notify. */
pub static XCB_SHAPE_NOTIFY : u8 = 0;
pub struct NotifyEvent {pub base : base::Event<xcb_shape_notify_event_t>}
pub struct  QueryVersionCookie<'s> { pub base : base::Cookie<'s, xcb_shape_query_version_cookie_t> }

/** Opcode for xcb_shape_query_version. */
pub static XCB_SHAPE_QUERY_VERSION : u8 = 0;
pub struct QueryVersionReply { base:  base::Reply<xcb_shape_query_version_reply_t> }
fn mk_reply_xcb_shape_query_version_reply_t(reply:*mut xcb_shape_query_version_reply_t) -> QueryVersionReply { QueryVersionReply { base : base::mk_reply(reply) } }
/** Opcode for xcb_shape_rectangles. */
pub static XCB_SHAPE_RECTANGLES : u8 = 1;
/** Opcode for xcb_shape_mask. */
pub static XCB_SHAPE_MASK : u8 = 2;
/** Opcode for xcb_shape_combine. */
pub static XCB_SHAPE_COMBINE : u8 = 3;
/** Opcode for xcb_shape_offset. */
pub static XCB_SHAPE_OFFSET : u8 = 4;
pub struct  QueryExtentsCookie<'s> { pub base : base::Cookie<'s, xcb_shape_query_extents_cookie_t> }

/** Opcode for xcb_shape_query_extents. */
pub static XCB_SHAPE_QUERY_EXTENTS : u8 = 5;
pub struct QueryExtentsReply { base:  base::Reply<xcb_shape_query_extents_reply_t> }
fn mk_reply_xcb_shape_query_extents_reply_t(reply:*mut xcb_shape_query_extents_reply_t) -> QueryExtentsReply { QueryExtentsReply { base : base::mk_reply(reply) } }
/** Opcode for xcb_shape_select_input. */
pub static XCB_SHAPE_SELECT_INPUT : u8 = 6;
pub struct  InputSelectedCookie<'s> { pub base : base::Cookie<'s, xcb_shape_input_selected_cookie_t> }

/** Opcode for xcb_shape_input_selected. */
pub static XCB_SHAPE_INPUT_SELECTED : u8 = 7;
pub struct InputSelectedReply { base:  base::Reply<xcb_shape_input_selected_reply_t> }
fn mk_reply_xcb_shape_input_selected_reply_t(reply:*mut xcb_shape_input_selected_reply_t) -> InputSelectedReply { InputSelectedReply { base : base::mk_reply(reply) } }
pub struct  GetRectanglesCookie<'s> { pub base : base::Cookie<'s, xcb_shape_get_rectangles_cookie_t> }

/** Opcode for xcb_shape_get_rectangles. */
pub static XCB_SHAPE_GET_RECTANGLES : u8 = 8;

impl Iterator for OpIterator {
    type Item = Op;
    fn next(&mut self) -> Option<Op> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_shape_op_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_shape_op_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub type Kind = xcb_shape_kind_t;


impl Iterator for KindIterator {
    type Item = Kind;
    fn next(&mut self) -> Option<Kind> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_shape_kind_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_shape_kind_next(iter);
            Some(mem::transmute(*data))
        }
    }
}


impl NotifyEvent {
  pub fn shape_kind(&mut self) -> Kind {
    unsafe { accessor!(shape_kind -> Kind, (*self.base.event)) }
  }

  pub fn affected_window(&mut self) -> xproto::Window {
    unsafe { accessor!(affected_window -> xproto::Window, (*self.base.event)) }
  }

  pub fn extents_x(&mut self) -> i16 {
    unsafe { accessor!(extents_x -> i16, (*self.base.event)) }
  }

  pub fn extents_y(&mut self) -> i16 {
    unsafe { accessor!(extents_y -> i16, (*self.base.event)) }
  }

  pub fn extents_width(&mut self) -> u16 {
    unsafe { accessor!(extents_width -> u16, (*self.base.event)) }
  }

  pub fn extents_height(&mut self) -> u16 {
    unsafe { accessor!(extents_height -> u16, (*self.base.event)) }
  }

  pub fn server_time(&mut self) -> xproto::Timestamp {
    unsafe { accessor!(server_time -> xproto::Timestamp, (*self.base.event)) }
  }

  pub fn shaped(&mut self) -> u8 {
    unsafe { accessor!(shaped -> u8, (*self.base.event)) }
  }

  pub fn new(shape_kind : Kind,
         affected_window : xproto::Window,
         extents_x : i16,
         extents_y : i16,
         extents_width : u16,
         extents_height : u16,
         server_time : xproto::Timestamp,
         shaped : u8) -> NotifyEvent {
    unsafe {
      let raw = malloc(32 as size_t) as *mut xcb_shape_notify_event_t;
      (*raw).shape_kind = shape_kind;
      (*raw).affected_window = affected_window;
      (*raw).extents_x = extents_x;
      (*raw).extents_y = extents_y;
      (*raw).extents_width = extents_width;
      (*raw).extents_height = extents_height;
      (*raw).server_time = server_time;
      (*raw).shaped = shaped;
      NotifyEvent { base : Event { event : raw as *mut xcb_shape_notify_event_t }}
    }
  }
}
pub fn QueryVersion<'r> (c : &'r Connection) -> QueryVersionCookie<'r> {
  unsafe {
    let cookie = xcb_shape_query_version(c.get_raw_conn());
    QueryVersionCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn QueryVersionUnchecked<'r> (c : &'r Connection) -> QueryVersionCookie<'r> {
  unsafe {
    let cookie = xcb_shape_query_version_unchecked(c.get_raw_conn());
    QueryVersionCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl QueryVersionReply {
  pub fn major_version(&mut self) -> u16 {
    unsafe { accessor!(major_version -> u16, (*self.base.reply)) }
  }

  pub fn minor_version(&mut self) -> u16 {
    unsafe { accessor!(minor_version -> u16, (*self.base.reply)) }
  }

}
impl_reply_cookie!(QueryVersionCookie<'s>, mk_reply_xcb_shape_query_version_reply_t, QueryVersionReply, xcb_shape_query_version_reply);

pub fn RectanglesChecked<'r> (c : &'r Connection,
                          operation : Op,
                          destination_kind : Kind,
                          ordering : u8,
                          destination_window : xproto::Window,
                          x_offset : i16,
                          y_offset : i16,
                          rectangles : &[xproto::Rectangle]) -> base::VoidCookie<'r> {
  unsafe {
    let rectangles_len = rectangles.len();
    let rectangles_ptr = rectangles.as_ptr();
    let cookie = xcb_shape_rectangles_checked(c.get_raw_conn(),
        operation as xcb_shape_op_t, //1
        destination_kind as xcb_shape_kind_t, //2
        ordering as u8, //3
        destination_window as ffi::xproto::xcb_window_t, //4
        x_offset as i16, //5
        y_offset as i16, //6
        rectangles_len as u32, //7
        rectangles_ptr as *mut ffi::xproto::xcb_rectangle_t); //8
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn Rectangles<'r> (c : &'r Connection,
                   operation : Op,
                   destination_kind : Kind,
                   ordering : u8,
                   destination_window : xproto::Window,
                   x_offset : i16,
                   y_offset : i16,
                   rectangles : &[xproto::Rectangle]) -> base::VoidCookie<'r> {
  unsafe {
    let rectangles_len = rectangles.len();
    let rectangles_ptr = rectangles.as_ptr();
    let cookie = xcb_shape_rectangles(c.get_raw_conn(),
        operation as xcb_shape_op_t, //1
        destination_kind as xcb_shape_kind_t, //2
        ordering as u8, //3
        destination_window as ffi::xproto::xcb_window_t, //4
        x_offset as i16, //5
        y_offset as i16, //6
        rectangles_len as u32, //7
        rectangles_ptr as *mut ffi::xproto::xcb_rectangle_t); //8
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn MaskChecked<'r> (c : &'r Connection,
                    operation : Op,
                    destination_kind : Kind,
                    destination_window : xproto::Window,
                    x_offset : i16,
                    y_offset : i16,
                    source_bitmap : xproto::Pixmap) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_shape_mask_checked(c.get_raw_conn(),
        operation as xcb_shape_op_t, //1
        destination_kind as xcb_shape_kind_t, //2
        destination_window as ffi::xproto::xcb_window_t, //3
        x_offset as i16, //4
        y_offset as i16, //5
        source_bitmap as ffi::xproto::xcb_pixmap_t); //6
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn Mask<'r> (c : &'r Connection,
             operation : Op,
             destination_kind : Kind,
             destination_window : xproto::Window,
             x_offset : i16,
             y_offset : i16,
             source_bitmap : xproto::Pixmap) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_shape_mask(c.get_raw_conn(),
        operation as xcb_shape_op_t, //1
        destination_kind as xcb_shape_kind_t, //2
        destination_window as ffi::xproto::xcb_window_t, //3
        x_offset as i16, //4
        y_offset as i16, //5
        source_bitmap as ffi::xproto::xcb_pixmap_t); //6
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn CombineChecked<'r> (c : &'r Connection,
                       operation : Op,
                       destination_kind : Kind,
                       source_kind : Kind,
                       destination_window : xproto::Window,
                       x_offset : i16,
                       y_offset : i16,
                       source_window : xproto::Window) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_shape_combine_checked(c.get_raw_conn(),
        operation as xcb_shape_op_t, //1
        destination_kind as xcb_shape_kind_t, //2
        source_kind as xcb_shape_kind_t, //3
        destination_window as ffi::xproto::xcb_window_t, //4
        x_offset as i16, //5
        y_offset as i16, //6
        source_window as ffi::xproto::xcb_window_t); //7
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn Combine<'r> (c : &'r Connection,
                operation : Op,
                destination_kind : Kind,
                source_kind : Kind,
                destination_window : xproto::Window,
                x_offset : i16,
                y_offset : i16,
                source_window : xproto::Window) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_shape_combine(c.get_raw_conn(),
        operation as xcb_shape_op_t, //1
        destination_kind as xcb_shape_kind_t, //2
        source_kind as xcb_shape_kind_t, //3
        destination_window as ffi::xproto::xcb_window_t, //4
        x_offset as i16, //5
        y_offset as i16, //6
        source_window as ffi::xproto::xcb_window_t); //7
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn OffsetChecked<'r> (c : &'r Connection,
                      destination_kind : Kind,
                      destination_window : xproto::Window,
                      x_offset : i16,
                      y_offset : i16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_shape_offset_checked(c.get_raw_conn(),
        destination_kind as xcb_shape_kind_t, //1
        destination_window as ffi::xproto::xcb_window_t, //2
        x_offset as i16, //3
        y_offset as i16); //4
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn Offset<'r> (c : &'r Connection,
               destination_kind : Kind,
               destination_window : xproto::Window,
               x_offset : i16,
               y_offset : i16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_shape_offset(c.get_raw_conn(),
        destination_kind as xcb_shape_kind_t, //1
        destination_window as ffi::xproto::xcb_window_t, //2
        x_offset as i16, //3
        y_offset as i16); //4
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn QueryExtents<'r> (c : &'r Connection,
                     destination_window : xproto::Window) -> QueryExtentsCookie<'r> {
  unsafe {
    let cookie = xcb_shape_query_extents(c.get_raw_conn(),
        destination_window as ffi::xproto::xcb_window_t); //1
    QueryExtentsCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn QueryExtentsUnchecked<'r> (c : &'r Connection,
                              destination_window : xproto::Window) -> QueryExtentsCookie<'r> {
  unsafe {
    let cookie = xcb_shape_query_extents_unchecked(c.get_raw_conn(),
        destination_window as ffi::xproto::xcb_window_t); //1
    QueryExtentsCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl QueryExtentsReply {
  pub fn bounding_shaped(&mut self) -> u8 {
    unsafe { accessor!(bounding_shaped -> u8, (*self.base.reply)) }
  }

  pub fn clip_shaped(&mut self) -> u8 {
    unsafe { accessor!(clip_shaped -> u8, (*self.base.reply)) }
  }

  pub fn bounding_shape_extents_x(&mut self) -> i16 {
    unsafe { accessor!(bounding_shape_extents_x -> i16, (*self.base.reply)) }
  }

  pub fn bounding_shape_extents_y(&mut self) -> i16 {
    unsafe { accessor!(bounding_shape_extents_y -> i16, (*self.base.reply)) }
  }

  pub fn bounding_shape_extents_width(&mut self) -> u16 {
    unsafe { accessor!(bounding_shape_extents_width -> u16, (*self.base.reply)) }
  }

  pub fn bounding_shape_extents_height(&mut self) -> u16 {
    unsafe { accessor!(bounding_shape_extents_height -> u16, (*self.base.reply)) }
  }

  pub fn clip_shape_extents_x(&mut self) -> i16 {
    unsafe { accessor!(clip_shape_extents_x -> i16, (*self.base.reply)) }
  }

  pub fn clip_shape_extents_y(&mut self) -> i16 {
    unsafe { accessor!(clip_shape_extents_y -> i16, (*self.base.reply)) }
  }

  pub fn clip_shape_extents_width(&mut self) -> u16 {
    unsafe { accessor!(clip_shape_extents_width -> u16, (*self.base.reply)) }
  }

  pub fn clip_shape_extents_height(&mut self) -> u16 {
    unsafe { accessor!(clip_shape_extents_height -> u16, (*self.base.reply)) }
  }

}
impl_reply_cookie!(QueryExtentsCookie<'s>, mk_reply_xcb_shape_query_extents_reply_t, QueryExtentsReply, xcb_shape_query_extents_reply);

pub fn SelectInputChecked<'r> (c : &'r Connection,
                           destination_window : xproto::Window,
                           enable : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_shape_select_input_checked(c.get_raw_conn(),
        destination_window as ffi::xproto::xcb_window_t, //1
        enable as u8); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn SelectInput<'r> (c : &'r Connection,
                    destination_window : xproto::Window,
                    enable : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_shape_select_input(c.get_raw_conn(),
        destination_window as ffi::xproto::xcb_window_t, //1
        enable as u8); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn InputSelected<'r> (c : &'r Connection,
                      destination_window : xproto::Window) -> InputSelectedCookie<'r> {
  unsafe {
    let cookie = xcb_shape_input_selected(c.get_raw_conn(),
        destination_window as ffi::xproto::xcb_window_t); //1
    InputSelectedCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn InputSelectedUnchecked<'r> (c : &'r Connection,
                               destination_window : xproto::Window) -> InputSelectedCookie<'r> {
  unsafe {
    let cookie = xcb_shape_input_selected_unchecked(c.get_raw_conn(),
        destination_window as ffi::xproto::xcb_window_t); //1
    InputSelectedCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl InputSelectedReply {
  pub fn enabled(&mut self) -> u8 {
    unsafe { accessor!(enabled -> u8, (*self.base.reply)) }
  }

}
impl_reply_cookie!(InputSelectedCookie<'s>, mk_reply_xcb_shape_input_selected_reply_t, InputSelectedReply, xcb_shape_input_selected_reply);

pub struct GetRectanglesReply { base:  base::Reply<xcb_shape_get_rectangles_reply_t> }
fn mk_reply_xcb_shape_get_rectangles_reply_t(reply:*mut xcb_shape_get_rectangles_reply_t) -> GetRectanglesReply { GetRectanglesReply { base : base::mk_reply(reply) } }
pub fn GetRectangles<'r> (c : &'r Connection,
                      window : xproto::Window,
                      source_kind : Kind) -> GetRectanglesCookie<'r> {
  unsafe {
    let cookie = xcb_shape_get_rectangles(c.get_raw_conn(),
        window as ffi::xproto::xcb_window_t, //1
        source_kind as xcb_shape_kind_t); //2
    GetRectanglesCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetRectanglesUnchecked<'r> (c : &'r Connection,
                               window : xproto::Window,
                               source_kind : Kind) -> GetRectanglesCookie<'r> {
  unsafe {
    let cookie = xcb_shape_get_rectangles_unchecked(c.get_raw_conn(),
        window as ffi::xproto::xcb_window_t, //1
        source_kind as xcb_shape_kind_t); //2
    GetRectanglesCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetRectanglesReply {
  pub fn ordering(&mut self) -> u8 {
    unsafe { accessor!(ordering -> u8, (*self.base.reply)) }
  }

  pub fn rectangles(&mut self) -> xproto::RectangleIterator {
    unsafe { accessor!(xproto::RectangleIterator, xcb_shape_get_rectangles_rectangles_iterator, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetRectanglesCookie<'s>, mk_reply_xcb_shape_get_rectangles_reply_t, GetRectanglesReply, xcb_shape_get_rectangles_reply);


