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
pub type Op = op;

pub type OpIterator = op_iterator;

pub type KindIterator = kind_iterator;


pub type so = c_uint;//{
    pub static XCB_SHAPE_SO_SET : so = 1;
    pub static XCB_SHAPE_SO_UNION : so = 2;
    pub static XCB_SHAPE_SO_INTERSECT : so = 3;
    pub static XCB_SHAPE_SO_SUBTRACT : so = 4;
    pub static XCB_SHAPE_SO_INVERT : so = 5;
//}

pub type sk = c_uint;//{
    pub static XCB_SHAPE_SK_BOUNDING : sk = 1;
    pub static XCB_SHAPE_SK_CLIP : sk = 2;
    pub static XCB_SHAPE_SK_INPUT : sk = 3;
//}
/** Opcode for xcb_shape_notify. */
pub static XCB_SHAPE_NOTIFY : u8 = 0;
pub type NotifyEvent = base::Event<notify_event>;
pub type QueryVersionCookie<'s> = base::Cookie<'s, query_version_cookie>;

/** Opcode for xcb_shape_query_version. */
pub static XCB_SHAPE_QUERY_VERSION : u8 = 0;
pub type QueryVersionReply = base::Reply<query_version_reply>;
/** Opcode for xcb_shape_rectangles. */
pub static XCB_SHAPE_RECTANGLES : u8 = 1;
/** Opcode for xcb_shape_mask. */
pub static XCB_SHAPE_MASK : u8 = 2;
/** Opcode for xcb_shape_combine. */
pub static XCB_SHAPE_COMBINE : u8 = 3;
/** Opcode for xcb_shape_offset. */
pub static XCB_SHAPE_OFFSET : u8 = 4;
pub type QueryExtentsCookie<'s> = base::Cookie<'s, query_extents_cookie>;

/** Opcode for xcb_shape_query_extents. */
pub static XCB_SHAPE_QUERY_EXTENTS : u8 = 5;
pub type QueryExtentsReply = base::Reply<query_extents_reply>;
/** Opcode for xcb_shape_select_input. */
pub static XCB_SHAPE_SELECT_INPUT : u8 = 6;
pub type InputSelectedCookie<'s> = base::Cookie<'s, input_selected_cookie>;

/** Opcode for xcb_shape_input_selected. */
pub static XCB_SHAPE_INPUT_SELECTED : u8 = 7;
pub type InputSelectedReply = base::Reply<input_selected_reply>;
pub type GetRectanglesCookie<'s> = base::Cookie<'s, get_rectangles_cookie>;

/** Opcode for xcb_shape_get_rectangles. */
pub static XCB_SHAPE_GET_RECTANGLES : u8 = 8;

impl<'s, Op> Iterator<&'s Op> for OpIterator {
    pub fn next(&mut self) -> Option<&'s Op> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *mut op_iterator = mem::transmute(self);
            let data = (*iter).data;
            xcb_shape_op_next(iter);
            Some(mem::transmute(data))
        }
    }
}

pub type Kind = kind;


impl<'s, Kind> Iterator<&'s Kind> for KindIterator {
    pub fn next(&mut self) -> Option<&'s Kind> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *mut kind_iterator = mem::transmute(self);
            let data = (*iter).data;
            xcb_shape_kind_next(iter);
            Some(mem::transmute(data))
        }
    }
}


impl base::Event<notify_event> {
  pub fn shape_kind(&self) -> Kind {
    unsafe { accessor!(shape_kind -> Kind, (*self.event)) }
  }

  pub fn affected_window(&self) -> xproto::Window {
    unsafe { accessor!(affected_window -> xproto::Window, (*self.event)) }
  }

  pub fn extents_x(&self) -> i16 {
    unsafe { accessor!(extents_x -> i16, (*self.event)) }
  }

  pub fn extents_y(&self) -> i16 {
    unsafe { accessor!(extents_y -> i16, (*self.event)) }
  }

  pub fn extents_width(&self) -> u16 {
    unsafe { accessor!(extents_width -> u16, (*self.event)) }
  }

  pub fn extents_height(&self) -> u16 {
    unsafe { accessor!(extents_height -> u16, (*self.event)) }
  }

  pub fn server_time(&self) -> xproto::Timestamp {
    unsafe { accessor!(server_time -> xproto::Timestamp, (*self.event)) }
  }

  pub fn shaped(&self) -> u8 {
    unsafe { accessor!(shaped -> u8, (*self.event)) }
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
      let raw = malloc(32u as size_t) as *mut notify_event;
      (*raw).shape_kind = shape_kind;
      (*raw).affected_window = affected_window;
      (*raw).extents_x = extents_x;
      (*raw).extents_y = extents_y;
      (*raw).extents_width = extents_width;
      (*raw).extents_height = extents_height;
      (*raw).server_time = server_time;
      (*raw).shaped = shaped;
      Event { event : raw as *mut notify_event }
    }
  }
}
pub fn QueryVersion<'r> (c : &'r Connection) -> QueryVersionCookie<'r> {
  unsafe {
    let cookie = xcb_shape_query_version(c.get_raw_conn());
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn QueryVersionUnchecked<'r> (c : &'r Connection) -> QueryVersionCookie<'r> {
  unsafe {
    let cookie = xcb_shape_query_version_unchecked(c.get_raw_conn());
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}

impl base::Reply<query_version_reply> {
  pub fn major_version(&self) -> u16 {
    unsafe { accessor!(major_version -> u16, (*self.reply)) }
  }

  pub fn minor_version(&self) -> u16 {
    unsafe { accessor!(minor_version -> u16, (*self.reply)) }
  }

}
impl_reply_cookie!(QueryVersionCookie<'s>, query_version_reply, QueryVersionReply, xcb_shape_query_version_reply)

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
    let rectangles_ptr = std::vec::raw::to_ptr(rectangles);
    let cookie = xcb_shape_rectangles_checked(c.get_raw_conn(),
        operation as op, //1
        destination_kind as kind, //2
        ordering as u8, //3
        destination_window as ffi::xproto::window, //4
        x_offset as i16, //5
        y_offset as i16, //6
        rectangles_len as u32, //7
        rectangles_ptr as *mut ffi::xproto::rectangle); //8
    Cookie {cookie:cookie,conn:c,checked:true}
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
    let rectangles_ptr = std::vec::raw::to_ptr(rectangles);
    let cookie = xcb_shape_rectangles(c.get_raw_conn(),
        operation as op, //1
        destination_kind as kind, //2
        ordering as u8, //3
        destination_window as ffi::xproto::window, //4
        x_offset as i16, //5
        y_offset as i16, //6
        rectangles_len as u32, //7
        rectangles_ptr as *mut ffi::xproto::rectangle); //8
    Cookie {cookie:cookie,conn:c,checked:false}
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
        operation as op, //1
        destination_kind as kind, //2
        destination_window as ffi::xproto::window, //3
        x_offset as i16, //4
        y_offset as i16, //5
        source_bitmap as ffi::xproto::pixmap); //6
    Cookie {cookie:cookie,conn:c,checked:true}
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
        operation as op, //1
        destination_kind as kind, //2
        destination_window as ffi::xproto::window, //3
        x_offset as i16, //4
        y_offset as i16, //5
        source_bitmap as ffi::xproto::pixmap); //6
    Cookie {cookie:cookie,conn:c,checked:false}
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
        operation as op, //1
        destination_kind as kind, //2
        source_kind as kind, //3
        destination_window as ffi::xproto::window, //4
        x_offset as i16, //5
        y_offset as i16, //6
        source_window as ffi::xproto::window); //7
    Cookie {cookie:cookie,conn:c,checked:true}
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
        operation as op, //1
        destination_kind as kind, //2
        source_kind as kind, //3
        destination_window as ffi::xproto::window, //4
        x_offset as i16, //5
        y_offset as i16, //6
        source_window as ffi::xproto::window); //7
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn OffsetChecked<'r> (c : &'r Connection,
                      destination_kind : Kind,
                      destination_window : xproto::Window,
                      x_offset : i16,
                      y_offset : i16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_shape_offset_checked(c.get_raw_conn(),
        destination_kind as kind, //1
        destination_window as ffi::xproto::window, //2
        x_offset as i16, //3
        y_offset as i16); //4
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn Offset<'r> (c : &'r Connection,
               destination_kind : Kind,
               destination_window : xproto::Window,
               x_offset : i16,
               y_offset : i16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_shape_offset(c.get_raw_conn(),
        destination_kind as kind, //1
        destination_window as ffi::xproto::window, //2
        x_offset as i16, //3
        y_offset as i16); //4
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn QueryExtents<'r> (c : &'r Connection,
                     destination_window : xproto::Window) -> QueryExtentsCookie<'r> {
  unsafe {
    let cookie = xcb_shape_query_extents(c.get_raw_conn(),
        destination_window as ffi::xproto::window); //1
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn QueryExtentsUnchecked<'r> (c : &'r Connection,
                              destination_window : xproto::Window) -> QueryExtentsCookie<'r> {
  unsafe {
    let cookie = xcb_shape_query_extents_unchecked(c.get_raw_conn(),
        destination_window as ffi::xproto::window); //1
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}

impl base::Reply<query_extents_reply> {
  pub fn bounding_shaped(&self) -> u8 {
    unsafe { accessor!(bounding_shaped -> u8, (*self.reply)) }
  }

  pub fn clip_shaped(&self) -> u8 {
    unsafe { accessor!(clip_shaped -> u8, (*self.reply)) }
  }

  pub fn bounding_shape_extents_x(&self) -> i16 {
    unsafe { accessor!(bounding_shape_extents_x -> i16, (*self.reply)) }
  }

  pub fn bounding_shape_extents_y(&self) -> i16 {
    unsafe { accessor!(bounding_shape_extents_y -> i16, (*self.reply)) }
  }

  pub fn bounding_shape_extents_width(&self) -> u16 {
    unsafe { accessor!(bounding_shape_extents_width -> u16, (*self.reply)) }
  }

  pub fn bounding_shape_extents_height(&self) -> u16 {
    unsafe { accessor!(bounding_shape_extents_height -> u16, (*self.reply)) }
  }

  pub fn clip_shape_extents_x(&self) -> i16 {
    unsafe { accessor!(clip_shape_extents_x -> i16, (*self.reply)) }
  }

  pub fn clip_shape_extents_y(&self) -> i16 {
    unsafe { accessor!(clip_shape_extents_y -> i16, (*self.reply)) }
  }

  pub fn clip_shape_extents_width(&self) -> u16 {
    unsafe { accessor!(clip_shape_extents_width -> u16, (*self.reply)) }
  }

  pub fn clip_shape_extents_height(&self) -> u16 {
    unsafe { accessor!(clip_shape_extents_height -> u16, (*self.reply)) }
  }

}
impl_reply_cookie!(QueryExtentsCookie<'s>, query_extents_reply, QueryExtentsReply, xcb_shape_query_extents_reply)

pub fn SelectInputChecked<'r> (c : &'r Connection,
                           destination_window : xproto::Window,
                           enable : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_shape_select_input_checked(c.get_raw_conn(),
        destination_window as ffi::xproto::window, //1
        enable as u8); //2
    Cookie {cookie:cookie,conn:c,checked:true}
  }
}
pub fn SelectInput<'r> (c : &'r Connection,
                    destination_window : xproto::Window,
                    enable : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_shape_select_input(c.get_raw_conn(),
        destination_window as ffi::xproto::window, //1
        enable as u8); //2
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn InputSelected<'r> (c : &'r Connection,
                      destination_window : xproto::Window) -> InputSelectedCookie<'r> {
  unsafe {
    let cookie = xcb_shape_input_selected(c.get_raw_conn(),
        destination_window as ffi::xproto::window); //1
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn InputSelectedUnchecked<'r> (c : &'r Connection,
                               destination_window : xproto::Window) -> InputSelectedCookie<'r> {
  unsafe {
    let cookie = xcb_shape_input_selected_unchecked(c.get_raw_conn(),
        destination_window as ffi::xproto::window); //1
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}

impl base::Reply<input_selected_reply> {
  pub fn enabled(&self) -> u8 {
    unsafe { accessor!(enabled -> u8, (*self.reply)) }
  }

}
impl_reply_cookie!(InputSelectedCookie<'s>, input_selected_reply, InputSelectedReply, xcb_shape_input_selected_reply)

pub type GetRectanglesReply = base::Reply<get_rectangles_reply>;
pub fn GetRectangles<'r> (c : &'r Connection,
                      window : xproto::Window,
                      source_kind : Kind) -> GetRectanglesCookie<'r> {
  unsafe {
    let cookie = xcb_shape_get_rectangles(c.get_raw_conn(),
        window as ffi::xproto::window, //1
        source_kind as kind); //2
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn GetRectanglesUnchecked<'r> (c : &'r Connection,
                               window : xproto::Window,
                               source_kind : Kind) -> GetRectanglesCookie<'r> {
  unsafe {
    let cookie = xcb_shape_get_rectangles_unchecked(c.get_raw_conn(),
        window as ffi::xproto::window, //1
        source_kind as kind); //2
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}

impl base::Reply<get_rectangles_reply> {
  pub fn ordering(&self) -> u8 {
    unsafe { accessor!(ordering -> u8, (*self.reply)) }
  }

  pub fn rectangles(&self) -> xproto::RectangleIterator {
    unsafe { accessor!(xproto::RectangleIterator, xcb_shape_get_rectangles_rectangles_iterator, (*self.reply)) }
  }

}
impl_reply_cookie!(GetRectanglesCookie<'s>, get_rectangles_reply, GetRectanglesReply, xcb_shape_get_rectangles_reply)


