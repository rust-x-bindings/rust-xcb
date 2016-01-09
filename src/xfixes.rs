/*
 * This file generated automatically from xfixes.xml by r_client.py.
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
use ffi::xfixes::*;
use std::option::Option;
use std::iter::Iterator;

use xproto;
use render;
use shape;
pub struct  QueryVersionCookie<'s> { pub base : base::Cookie<'s, xcb_xfixes_query_version_cookie_t> }

/** Opcode for xcb_xfixes_query_version. */
pub static XCB_XFIXES_QUERY_VERSION : u8 = 0;
pub struct QueryVersionReply { base:  base::Reply<xcb_xfixes_query_version_reply_t> }
fn mk_reply_xcb_xfixes_query_version_reply_t(reply:*mut xcb_xfixes_query_version_reply_t) -> QueryVersionReply { QueryVersionReply { base : base::mk_reply(reply) } }

pub type xcb_xfixes_save_set_mode_t = c_uint;//{
    pub static XCB_XFIXES_SAVE_SET_MODE_INSERT : xcb_xfixes_save_set_mode_t = 1;
    pub static XCB_XFIXES_SAVE_SET_MODE_DELETE : xcb_xfixes_save_set_mode_t = 2;
//}

pub type xcb_xfixes_save_set_target_t = c_uint;//{
    pub static XCB_XFIXES_SAVE_SET_TARGET_NEAREST : xcb_xfixes_save_set_target_t = 1;
    pub static XCB_XFIXES_SAVE_SET_TARGET_ROOT : xcb_xfixes_save_set_target_t = 2;
//}

pub type xcb_xfixes_save_set_mapping_t = c_uint;//{
    pub static XCB_XFIXES_SAVE_SET_MAPPING_MAP : xcb_xfixes_save_set_mapping_t = 1;
    pub static XCB_XFIXES_SAVE_SET_MAPPING_UNMAP : xcb_xfixes_save_set_mapping_t = 2;
//}
/** Opcode for xcb_xfixes_change_save_set. */
pub static XCB_XFIXES_CHANGE_SAVE_SET : u8 = 1;

pub type xcb_xfixes_selection_event_t = c_uint;//{
    pub static XCB_XFIXES_SELECTION_EVENT_SET_SELECTION_OWNER : xcb_xfixes_selection_event_t = 1;
    pub static XCB_XFIXES_SELECTION_EVENT_SELECTION_WINDOW_DESTROY : xcb_xfixes_selection_event_t = 2;
    pub static XCB_XFIXES_SELECTION_EVENT_SELECTION_CLIENT_CLOSE : xcb_xfixes_selection_event_t = 3;
//}

pub type xcb_xfixes_selection_event_mask_t = c_uint;//{
    pub static XCB_XFIXES_SELECTION_EVENT_MASK_SET_SELECTION_OWNER : xcb_xfixes_selection_event_mask_t = 1;
    pub static XCB_XFIXES_SELECTION_EVENT_MASK_SELECTION_WINDOW_DESTROY : xcb_xfixes_selection_event_mask_t = 2;
    pub static XCB_XFIXES_SELECTION_EVENT_MASK_SELECTION_CLIENT_CLOSE : xcb_xfixes_selection_event_mask_t = 4;
//}
/** Opcode for xcb_xfixes_selection_notify. */
pub static XCB_XFIXES_SELECTION_NOTIFY : u8 = 0;
pub struct SelectionNotifyEvent {pub base : base::Event<xcb_xfixes_selection_notify_event_t>}
/** Opcode for xcb_xfixes_select_selection_input. */
pub static XCB_XFIXES_SELECT_SELECTION_INPUT : u8 = 2;

pub type xcb_xfixes_cursor_notify_t = c_uint;//{
    pub static XCB_XFIXES_CURSOR_NOTIFY_DISPLAY_CURSOR : xcb_xfixes_cursor_notify_t = 1;
//}

pub type xcb_xfixes_cursor_notify_mask_t = c_uint;//{
    pub static XCB_XFIXES_CURSOR_NOTIFY_MASK_DISPLAY_CURSOR : xcb_xfixes_cursor_notify_mask_t = 1;
//}
/** Opcode for xcb_xfixes_cursor_notify. */
pub static XCB_XFIXES_CURSOR_NOTIFY : u8 = 1;
pub struct CursorNotifyEvent {pub base : base::Event<xcb_xfixes_cursor_notify_event_t>}
/** Opcode for xcb_xfixes_select_cursor_input. */
pub static XCB_XFIXES_SELECT_CURSOR_INPUT : u8 = 3;
pub struct  GetCursorImageCookie<'s> { pub base : base::Cookie<'s, xcb_xfixes_get_cursor_image_cookie_t> }

/** Opcode for xcb_xfixes_get_cursor_image. */
pub static XCB_XFIXES_GET_CURSOR_IMAGE : u8 = 4;
pub type RegionIterator = xcb_xfixes_region_iterator_t;

/** Opcode for xcb_xfixes_bad_region. */
pub static XCB_XFIXES_BAD_REGION : u8 = 0;
pub struct BadRegionError { pub base : base::Error<xcb_xfixes_bad_region_error_t> }

pub type xcb_xfixes_region_enum_t = c_uint;//{
    pub static XCB_XFIXES_REGION_NONE : xcb_xfixes_region_enum_t = 1;
//}
/** Opcode for xcb_xfixes_create_region. */
pub static XCB_XFIXES_CREATE_REGION : u8 = 5;
/** Opcode for xcb_xfixes_create_region_from_bitmap. */
pub static XCB_XFIXES_CREATE_REGION_FROM_BITMAP : u8 = 6;
/** Opcode for xcb_xfixes_create_region_from_window. */
pub static XCB_XFIXES_CREATE_REGION_FROM_WINDOW : u8 = 7;
/** Opcode for xcb_xfixes_create_region_from_gc. */
pub static XCB_XFIXES_CREATE_REGION_FROM_GC : u8 = 8;
/** Opcode for xcb_xfixes_create_region_from_picture. */
pub static XCB_XFIXES_CREATE_REGION_FROM_PICTURE : u8 = 9;
/** Opcode for xcb_xfixes_destroy_region. */
pub static XCB_XFIXES_DESTROY_REGION : u8 = 10;
/** Opcode for xcb_xfixes_set_region. */
pub static XCB_XFIXES_SET_REGION : u8 = 11;
/** Opcode for xcb_xfixes_copy_region. */
pub static XCB_XFIXES_COPY_REGION : u8 = 12;
/** Opcode for xcb_xfixes_union_region. */
pub static XCB_XFIXES_UNION_REGION : u8 = 13;
/** Opcode for xcb_xfixes_intersect_region. */
pub static XCB_XFIXES_INTERSECT_REGION : u8 = 14;
/** Opcode for xcb_xfixes_subtract_region. */
pub static XCB_XFIXES_SUBTRACT_REGION : u8 = 15;
/** Opcode for xcb_xfixes_invert_region. */
pub static XCB_XFIXES_INVERT_REGION : u8 = 16;
/** Opcode for xcb_xfixes_translate_region. */
pub static XCB_XFIXES_TRANSLATE_REGION : u8 = 17;
/** Opcode for xcb_xfixes_region_extents. */
pub static XCB_XFIXES_REGION_EXTENTS : u8 = 18;
pub struct  FetchRegionCookie<'s> { pub base : base::Cookie<'s, xcb_xfixes_fetch_region_cookie_t> }

/** Opcode for xcb_xfixes_fetch_region. */
pub static XCB_XFIXES_FETCH_REGION : u8 = 19;
/** Opcode for xcb_xfixes_set_gc_clip_region. */
pub static XCB_XFIXES_SET_GC_CLIP_REGION : u8 = 20;
/** Opcode for xcb_xfixes_set_window_shape_region. */
pub static XCB_XFIXES_SET_WINDOW_SHAPE_REGION : u8 = 21;
/** Opcode for xcb_xfixes_set_picture_clip_region. */
pub static XCB_XFIXES_SET_PICTURE_CLIP_REGION : u8 = 22;
/** Opcode for xcb_xfixes_set_cursor_name. */
pub static XCB_XFIXES_SET_CURSOR_NAME : u8 = 23;
pub struct  GetCursorNameCookie<'s> { pub base : base::Cookie<'s, xcb_xfixes_get_cursor_name_cookie_t> }

/** Opcode for xcb_xfixes_get_cursor_name. */
pub static XCB_XFIXES_GET_CURSOR_NAME : u8 = 24;
pub struct  GetCursorImageAndNameCookie<'s> { pub base : base::Cookie<'s, xcb_xfixes_get_cursor_image_and_name_cookie_t> }

/** Opcode for xcb_xfixes_get_cursor_image_and_name. */
pub static XCB_XFIXES_GET_CURSOR_IMAGE_AND_NAME : u8 = 25;
/** Opcode for xcb_xfixes_change_cursor. */
pub static XCB_XFIXES_CHANGE_CURSOR : u8 = 26;
/** Opcode for xcb_xfixes_change_cursor_by_name. */
pub static XCB_XFIXES_CHANGE_CURSOR_BY_NAME : u8 = 27;
/** Opcode for xcb_xfixes_expand_region. */
pub static XCB_XFIXES_EXPAND_REGION : u8 = 28;
/** Opcode for xcb_xfixes_hide_cursor. */
pub static XCB_XFIXES_HIDE_CURSOR : u8 = 29;
/** Opcode for xcb_xfixes_show_cursor. */
pub static XCB_XFIXES_SHOW_CURSOR : u8 = 30;
pub fn QueryVersion<'r> (c : &'r Connection,
                     client_major_version : u32,
                     client_minor_version : u32) -> QueryVersionCookie<'r> {
  unsafe {
    let cookie = xcb_xfixes_query_version(c.get_raw_conn(),
        client_major_version as u32, //1
        client_minor_version as u32); //2
    QueryVersionCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn QueryVersionUnchecked<'r> (c : &'r Connection,
                              client_major_version : u32,
                              client_minor_version : u32) -> QueryVersionCookie<'r> {
  unsafe {
    let cookie = xcb_xfixes_query_version_unchecked(c.get_raw_conn(),
        client_major_version as u32, //1
        client_minor_version as u32); //2
    QueryVersionCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl QueryVersionReply {
  pub fn major_version(&mut self) -> u32 {
    unsafe { accessor!(major_version -> u32, (*self.base.reply)) }
  }

  pub fn minor_version(&mut self) -> u32 {
    unsafe { accessor!(minor_version -> u32, (*self.base.reply)) }
  }

}
impl_reply_cookie!(QueryVersionCookie<'s>, mk_reply_xcb_xfixes_query_version_reply_t, QueryVersionReply, xcb_xfixes_query_version_reply);

pub fn ChangeSaveSetChecked<'r> (c : &'r Connection,
                             mode : u8,
                             target : u8,
                             map : u8,
                             window : xproto::Window) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xfixes_change_save_set_checked(c.get_raw_conn(),
        mode as u8, //1
        target as u8, //2
        map as u8, //3
        window as ffi::xproto::xcb_window_t); //4
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn ChangeSaveSet<'r> (c : &'r Connection,
                      mode : u8,
                      target : u8,
                      map : u8,
                      window : xproto::Window) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xfixes_change_save_set(c.get_raw_conn(),
        mode as u8, //1
        target as u8, //2
        map as u8, //3
        window as ffi::xproto::xcb_window_t); //4
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl SelectionNotifyEvent {
  pub fn subtype(&mut self) -> u8 {
    unsafe { accessor!(subtype -> u8, (*self.base.event)) }
  }

  pub fn window(&mut self) -> xproto::Window {
    unsafe { accessor!(window -> xproto::Window, (*self.base.event)) }
  }

  pub fn owner(&mut self) -> xproto::Window {
    unsafe { accessor!(owner -> xproto::Window, (*self.base.event)) }
  }

  pub fn selection(&mut self) -> xproto::Atom {
    unsafe { accessor!(selection -> xproto::Atom, (*self.base.event)) }
  }

  pub fn timestamp(&mut self) -> xproto::Timestamp {
    unsafe { accessor!(timestamp -> xproto::Timestamp, (*self.base.event)) }
  }

  pub fn selection_timestamp(&mut self) -> xproto::Timestamp {
    unsafe { accessor!(selection_timestamp -> xproto::Timestamp, (*self.base.event)) }
  }

  pub fn new(subtype : u8,
         window : xproto::Window,
         owner : xproto::Window,
         selection : xproto::Atom,
         timestamp : xproto::Timestamp,
         selection_timestamp : xproto::Timestamp) -> SelectionNotifyEvent {
    unsafe {
      let raw = malloc(32 as size_t) as *mut xcb_xfixes_selection_notify_event_t;
      (*raw).subtype = subtype;
      (*raw).window = window;
      (*raw).owner = owner;
      (*raw).selection = selection;
      (*raw).timestamp = timestamp;
      (*raw).selection_timestamp = selection_timestamp;
      SelectionNotifyEvent { base : Event { event : raw as *mut xcb_xfixes_selection_notify_event_t }}
    }
  }
}
pub fn SelectSelectionInputChecked<'r> (c : &'r Connection,
                                    window : xproto::Window,
                                    selection : xproto::Atom,
                                    event_mask : u32) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xfixes_select_selection_input_checked(c.get_raw_conn(),
        window as ffi::xproto::xcb_window_t, //1
        selection as ffi::xproto::xcb_atom_t, //2
        event_mask as u32); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn SelectSelectionInput<'r> (c : &'r Connection,
                             window : xproto::Window,
                             selection : xproto::Atom,
                             event_mask : u32) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xfixes_select_selection_input(c.get_raw_conn(),
        window as ffi::xproto::xcb_window_t, //1
        selection as ffi::xproto::xcb_atom_t, //2
        event_mask as u32); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl CursorNotifyEvent {
  pub fn subtype(&mut self) -> u8 {
    unsafe { accessor!(subtype -> u8, (*self.base.event)) }
  }

  pub fn window(&mut self) -> xproto::Window {
    unsafe { accessor!(window -> xproto::Window, (*self.base.event)) }
  }

  pub fn cursor_serial(&mut self) -> u32 {
    unsafe { accessor!(cursor_serial -> u32, (*self.base.event)) }
  }

  pub fn timestamp(&mut self) -> xproto::Timestamp {
    unsafe { accessor!(timestamp -> xproto::Timestamp, (*self.base.event)) }
  }

  pub fn name(&mut self) -> xproto::Atom {
    unsafe { accessor!(name -> xproto::Atom, (*self.base.event)) }
  }

  pub fn new(subtype : u8,
         window : xproto::Window,
         cursor_serial : u32,
         timestamp : xproto::Timestamp,
         name : xproto::Atom) -> CursorNotifyEvent {
    unsafe {
      let raw = malloc(32 as size_t) as *mut xcb_xfixes_cursor_notify_event_t;
      (*raw).subtype = subtype;
      (*raw).window = window;
      (*raw).cursor_serial = cursor_serial;
      (*raw).timestamp = timestamp;
      (*raw).name = name;
      CursorNotifyEvent { base : Event { event : raw as *mut xcb_xfixes_cursor_notify_event_t }}
    }
  }
}
pub fn SelectCursorInputChecked<'r> (c : &'r Connection,
                                 window : xproto::Window,
                                 event_mask : u32) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xfixes_select_cursor_input_checked(c.get_raw_conn(),
        window as ffi::xproto::xcb_window_t, //1
        event_mask as u32); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn SelectCursorInput<'r> (c : &'r Connection,
                          window : xproto::Window,
                          event_mask : u32) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xfixes_select_cursor_input(c.get_raw_conn(),
        window as ffi::xproto::xcb_window_t, //1
        event_mask as u32); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub struct GetCursorImageReply { base:  base::Reply<xcb_xfixes_get_cursor_image_reply_t> }
fn mk_reply_xcb_xfixes_get_cursor_image_reply_t(reply:*mut xcb_xfixes_get_cursor_image_reply_t) -> GetCursorImageReply { GetCursorImageReply { base : base::mk_reply(reply) } }
pub fn GetCursorImage<'r> (c : &'r Connection) -> GetCursorImageCookie<'r> {
  unsafe {
    let cookie = xcb_xfixes_get_cursor_image(c.get_raw_conn());
    GetCursorImageCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetCursorImageUnchecked<'r> (c : &'r Connection) -> GetCursorImageCookie<'r> {
  unsafe {
    let cookie = xcb_xfixes_get_cursor_image_unchecked(c.get_raw_conn());
    GetCursorImageCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetCursorImageReply {
  pub fn x(&mut self) -> i16 {
    unsafe { accessor!(x -> i16, (*self.base.reply)) }
  }

  pub fn y(&mut self) -> i16 {
    unsafe { accessor!(y -> i16, (*self.base.reply)) }
  }

  pub fn height(&mut self) -> u16 {
    unsafe { accessor!(height -> u16, (*self.base.reply)) }
  }

  pub fn xhot(&mut self) -> u16 {
    unsafe { accessor!(xhot -> u16, (*self.base.reply)) }
  }

  pub fn yhot(&mut self) -> u16 {
    unsafe { accessor!(yhot -> u16, (*self.base.reply)) }
  }

  pub fn cursor_serial(&mut self) -> u32 {
    unsafe { accessor!(cursor_serial -> u32, (*self.base.reply)) }
  }

  pub fn cursor_image(&mut self) -> Vec<u32> {
    unsafe { accessor!(u32, xcb_xfixes_get_cursor_image_cursor_image_length, xcb_xfixes_get_cursor_image_cursor_image, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetCursorImageCookie<'s>, mk_reply_xcb_xfixes_get_cursor_image_reply_t, GetCursorImageReply, xcb_xfixes_get_cursor_image_reply);

pub type Region = xcb_xfixes_region_t;


impl Iterator for RegionIterator {
    type Item = Region;
    fn next(&mut self) -> Option<Region> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xfixes_region_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xfixes_region_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub fn CreateRegionChecked<'r> (c : &'r Connection,
                            region : Region,
                            rectangles : &[xproto::Rectangle]) -> base::VoidCookie<'r> {
  unsafe {
    let rectangles_len = rectangles.len();
    let rectangles_ptr = rectangles.as_ptr();
    let cookie = xcb_xfixes_create_region_checked(c.get_raw_conn(),
        region as xcb_xfixes_region_t, //1
        rectangles_len as u32, //2
        rectangles_ptr as *mut ffi::xproto::xcb_rectangle_t); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn CreateRegion<'r> (c : &'r Connection,
                     region : Region,
                     rectangles : &[xproto::Rectangle]) -> base::VoidCookie<'r> {
  unsafe {
    let rectangles_len = rectangles.len();
    let rectangles_ptr = rectangles.as_ptr();
    let cookie = xcb_xfixes_create_region(c.get_raw_conn(),
        region as xcb_xfixes_region_t, //1
        rectangles_len as u32, //2
        rectangles_ptr as *mut ffi::xproto::xcb_rectangle_t); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn CreateRegionFromBitmapChecked<'r> (c : &'r Connection,
                                      region : Region,
                                      bitmap : xproto::Pixmap) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xfixes_create_region_from_bitmap_checked(c.get_raw_conn(),
        region as xcb_xfixes_region_t, //1
        bitmap as ffi::xproto::xcb_pixmap_t); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn CreateRegionFromBitmap<'r> (c : &'r Connection,
                               region : Region,
                               bitmap : xproto::Pixmap) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xfixes_create_region_from_bitmap(c.get_raw_conn(),
        region as xcb_xfixes_region_t, //1
        bitmap as ffi::xproto::xcb_pixmap_t); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn CreateRegionFromWindowChecked<'r> (c : &'r Connection,
                                      region : Region,
                                      window : xproto::Window,
                                      kind : shape::Kind) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xfixes_create_region_from_window_checked(c.get_raw_conn(),
        region as xcb_xfixes_region_t, //1
        window as ffi::xproto::xcb_window_t, //2
        kind as ffi::shape::xcb_shape_kind_t); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn CreateRegionFromWindow<'r> (c : &'r Connection,
                               region : Region,
                               window : xproto::Window,
                               kind : shape::Kind) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xfixes_create_region_from_window(c.get_raw_conn(),
        region as xcb_xfixes_region_t, //1
        window as ffi::xproto::xcb_window_t, //2
        kind as ffi::shape::xcb_shape_kind_t); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn CreateRegionFromGcChecked<'r> (c : &'r Connection,
                                  region : Region,
                                  gc : xproto::Gcontext) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xfixes_create_region_from_gc_checked(c.get_raw_conn(),
        region as xcb_xfixes_region_t, //1
        gc as ffi::xproto::xcb_gcontext_t); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn CreateRegionFromGc<'r> (c : &'r Connection,
                           region : Region,
                           gc : xproto::Gcontext) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xfixes_create_region_from_gc(c.get_raw_conn(),
        region as xcb_xfixes_region_t, //1
        gc as ffi::xproto::xcb_gcontext_t); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn CreateRegionFromPictureChecked<'r> (c : &'r Connection,
                                       region : Region,
                                       picture : render::Picture) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xfixes_create_region_from_picture_checked(c.get_raw_conn(),
        region as xcb_xfixes_region_t, //1
        picture as ffi::render::xcb_render_picture_t); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn CreateRegionFromPicture<'r> (c : &'r Connection,
                                region : Region,
                                picture : render::Picture) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xfixes_create_region_from_picture(c.get_raw_conn(),
        region as xcb_xfixes_region_t, //1
        picture as ffi::render::xcb_render_picture_t); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn DestroyRegionChecked<'r> (c : &'r Connection,
                             region : Region) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xfixes_destroy_region_checked(c.get_raw_conn(),
        region as xcb_xfixes_region_t); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn DestroyRegion<'r> (c : &'r Connection,
                      region : Region) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xfixes_destroy_region(c.get_raw_conn(),
        region as xcb_xfixes_region_t); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn SetRegionChecked<'r> (c : &'r Connection,
                         region : Region,
                         rectangles : &[xproto::Rectangle]) -> base::VoidCookie<'r> {
  unsafe {
    let rectangles_len = rectangles.len();
    let rectangles_ptr = rectangles.as_ptr();
    let cookie = xcb_xfixes_set_region_checked(c.get_raw_conn(),
        region as xcb_xfixes_region_t, //1
        rectangles_len as u32, //2
        rectangles_ptr as *mut ffi::xproto::xcb_rectangle_t); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn SetRegion<'r> (c : &'r Connection,
                  region : Region,
                  rectangles : &[xproto::Rectangle]) -> base::VoidCookie<'r> {
  unsafe {
    let rectangles_len = rectangles.len();
    let rectangles_ptr = rectangles.as_ptr();
    let cookie = xcb_xfixes_set_region(c.get_raw_conn(),
        region as xcb_xfixes_region_t, //1
        rectangles_len as u32, //2
        rectangles_ptr as *mut ffi::xproto::xcb_rectangle_t); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn CopyRegionChecked<'r> (c : &'r Connection,
                          source : Region,
                          destination : Region) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xfixes_copy_region_checked(c.get_raw_conn(),
        source as xcb_xfixes_region_t, //1
        destination as xcb_xfixes_region_t); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn CopyRegion<'r> (c : &'r Connection,
                   source : Region,
                   destination : Region) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xfixes_copy_region(c.get_raw_conn(),
        source as xcb_xfixes_region_t, //1
        destination as xcb_xfixes_region_t); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn UnionRegionChecked<'r> (c : &'r Connection,
                           source1 : Region,
                           source2 : Region,
                           destination : Region) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xfixes_union_region_checked(c.get_raw_conn(),
        source1 as xcb_xfixes_region_t, //1
        source2 as xcb_xfixes_region_t, //2
        destination as xcb_xfixes_region_t); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn UnionRegion<'r> (c : &'r Connection,
                    source1 : Region,
                    source2 : Region,
                    destination : Region) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xfixes_union_region(c.get_raw_conn(),
        source1 as xcb_xfixes_region_t, //1
        source2 as xcb_xfixes_region_t, //2
        destination as xcb_xfixes_region_t); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn IntersectRegionChecked<'r> (c : &'r Connection,
                               source1 : Region,
                               source2 : Region,
                               destination : Region) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xfixes_intersect_region_checked(c.get_raw_conn(),
        source1 as xcb_xfixes_region_t, //1
        source2 as xcb_xfixes_region_t, //2
        destination as xcb_xfixes_region_t); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn IntersectRegion<'r> (c : &'r Connection,
                        source1 : Region,
                        source2 : Region,
                        destination : Region) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xfixes_intersect_region(c.get_raw_conn(),
        source1 as xcb_xfixes_region_t, //1
        source2 as xcb_xfixes_region_t, //2
        destination as xcb_xfixes_region_t); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn SubtractRegionChecked<'r> (c : &'r Connection,
                              source1 : Region,
                              source2 : Region,
                              destination : Region) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xfixes_subtract_region_checked(c.get_raw_conn(),
        source1 as xcb_xfixes_region_t, //1
        source2 as xcb_xfixes_region_t, //2
        destination as xcb_xfixes_region_t); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn SubtractRegion<'r> (c : &'r Connection,
                       source1 : Region,
                       source2 : Region,
                       destination : Region) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xfixes_subtract_region(c.get_raw_conn(),
        source1 as xcb_xfixes_region_t, //1
        source2 as xcb_xfixes_region_t, //2
        destination as xcb_xfixes_region_t); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn InvertRegionChecked<'r> (c : &'r Connection,
                            source : Region,
                            bounds : xproto::Rectangle,
                            destination : Region) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xfixes_invert_region_checked(c.get_raw_conn(),
        source as xcb_xfixes_region_t, //1
        bounds.base.strct, //2
        destination as xcb_xfixes_region_t); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn InvertRegion<'r> (c : &'r Connection,
                     source : Region,
                     bounds : xproto::Rectangle,
                     destination : Region) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xfixes_invert_region(c.get_raw_conn(),
        source as xcb_xfixes_region_t, //1
        bounds.base.strct, //2
        destination as xcb_xfixes_region_t); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn TranslateRegionChecked<'r> (c : &'r Connection,
                               region : Region,
                               dx : i16,
                               dy : i16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xfixes_translate_region_checked(c.get_raw_conn(),
        region as xcb_xfixes_region_t, //1
        dx as i16, //2
        dy as i16); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn TranslateRegion<'r> (c : &'r Connection,
                        region : Region,
                        dx : i16,
                        dy : i16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xfixes_translate_region(c.get_raw_conn(),
        region as xcb_xfixes_region_t, //1
        dx as i16, //2
        dy as i16); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn RegionExtentsChecked<'r> (c : &'r Connection,
                             source : Region,
                             destination : Region) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xfixes_region_extents_checked(c.get_raw_conn(),
        source as xcb_xfixes_region_t, //1
        destination as xcb_xfixes_region_t); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn RegionExtents<'r> (c : &'r Connection,
                      source : Region,
                      destination : Region) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xfixes_region_extents(c.get_raw_conn(),
        source as xcb_xfixes_region_t, //1
        destination as xcb_xfixes_region_t); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub struct FetchRegionReply { base:  base::Reply<xcb_xfixes_fetch_region_reply_t> }
fn mk_reply_xcb_xfixes_fetch_region_reply_t(reply:*mut xcb_xfixes_fetch_region_reply_t) -> FetchRegionReply { FetchRegionReply { base : base::mk_reply(reply) } }
pub fn FetchRegion<'r> (c : &'r Connection,
                    region : Region) -> FetchRegionCookie<'r> {
  unsafe {
    let cookie = xcb_xfixes_fetch_region(c.get_raw_conn(),
        region as xcb_xfixes_region_t); //1
    FetchRegionCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn FetchRegionUnchecked<'r> (c : &'r Connection,
                             region : Region) -> FetchRegionCookie<'r> {
  unsafe {
    let cookie = xcb_xfixes_fetch_region_unchecked(c.get_raw_conn(),
        region as xcb_xfixes_region_t); //1
    FetchRegionCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl FetchRegionReply {
  pub fn extents(&self) -> xproto::Rectangle {
    unsafe { mem::transmute((*self.base.reply).extents) }
  }
  pub fn rectangles(&mut self) -> xproto::RectangleIterator {
    unsafe { accessor!(xproto::RectangleIterator, xcb_xfixes_fetch_region_rectangles_iterator, (*self.base.reply)) }
  }

}
impl_reply_cookie!(FetchRegionCookie<'s>, mk_reply_xcb_xfixes_fetch_region_reply_t, FetchRegionReply, xcb_xfixes_fetch_region_reply);

pub fn SetGcClipRegionChecked<'r> (c : &'r Connection,
                               gc : xproto::Gcontext,
                               region : Region,
                               x_origin : i16,
                               y_origin : i16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xfixes_set_gc_clip_region_checked(c.get_raw_conn(),
        gc as ffi::xproto::xcb_gcontext_t, //1
        region as xcb_xfixes_region_t, //2
        x_origin as i16, //3
        y_origin as i16); //4
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn SetGcClipRegion<'r> (c : &'r Connection,
                        gc : xproto::Gcontext,
                        region : Region,
                        x_origin : i16,
                        y_origin : i16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xfixes_set_gc_clip_region(c.get_raw_conn(),
        gc as ffi::xproto::xcb_gcontext_t, //1
        region as xcb_xfixes_region_t, //2
        x_origin as i16, //3
        y_origin as i16); //4
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn SetWindowShapeRegionChecked<'r> (c : &'r Connection,
                                    dest : xproto::Window,
                                    dest_kind : shape::Kind,
                                    x_offset : i16,
                                    y_offset : i16,
                                    region : Region) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xfixes_set_window_shape_region_checked(c.get_raw_conn(),
        dest as ffi::xproto::xcb_window_t, //1
        dest_kind as ffi::shape::xcb_shape_kind_t, //2
        x_offset as i16, //3
        y_offset as i16, //4
        region as xcb_xfixes_region_t); //5
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn SetWindowShapeRegion<'r> (c : &'r Connection,
                             dest : xproto::Window,
                             dest_kind : shape::Kind,
                             x_offset : i16,
                             y_offset : i16,
                             region : Region) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xfixes_set_window_shape_region(c.get_raw_conn(),
        dest as ffi::xproto::xcb_window_t, //1
        dest_kind as ffi::shape::xcb_shape_kind_t, //2
        x_offset as i16, //3
        y_offset as i16, //4
        region as xcb_xfixes_region_t); //5
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn SetPictureClipRegionChecked<'r> (c : &'r Connection,
                                    picture : render::Picture,
                                    region : Region,
                                    x_origin : i16,
                                    y_origin : i16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xfixes_set_picture_clip_region_checked(c.get_raw_conn(),
        picture as ffi::render::xcb_render_picture_t, //1
        region as xcb_xfixes_region_t, //2
        x_origin as i16, //3
        y_origin as i16); //4
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn SetPictureClipRegion<'r> (c : &'r Connection,
                             picture : render::Picture,
                             region : Region,
                             x_origin : i16,
                             y_origin : i16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xfixes_set_picture_clip_region(c.get_raw_conn(),
        picture as ffi::render::xcb_render_picture_t, //1
        region as xcb_xfixes_region_t, //2
        x_origin as i16, //3
        y_origin as i16); //4
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn SetCursorNameChecked<'r> (c : &'r Connection,
                             cursor : xproto::Cursor,
                             name : &str) -> base::VoidCookie<'r> {
  unsafe {
    let name = (name).as_bytes();
    let name_len = name.len();
    let name_ptr = name.as_ptr();
    let cookie = xcb_xfixes_set_cursor_name_checked(c.get_raw_conn(),
        cursor as ffi::xproto::xcb_cursor_t, //1
        name_len as u16, //2
        name_ptr as *mut c_char); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn SetCursorName<'r> (c : &'r Connection,
                      cursor : xproto::Cursor,
                      name : &str) -> base::VoidCookie<'r> {
  unsafe {
    let name = (name).as_bytes();
    let name_len = name.len();
    let name_ptr = name.as_ptr();
    let cookie = xcb_xfixes_set_cursor_name(c.get_raw_conn(),
        cursor as ffi::xproto::xcb_cursor_t, //1
        name_len as u16, //2
        name_ptr as *mut c_char); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub struct GetCursorNameReply { base:  base::Reply<xcb_xfixes_get_cursor_name_reply_t> }
fn mk_reply_xcb_xfixes_get_cursor_name_reply_t(reply:*mut xcb_xfixes_get_cursor_name_reply_t) -> GetCursorNameReply { GetCursorNameReply { base : base::mk_reply(reply) } }
pub fn GetCursorName<'r> (c : &'r Connection,
                      cursor : xproto::Cursor) -> GetCursorNameCookie<'r> {
  unsafe {
    let cookie = xcb_xfixes_get_cursor_name(c.get_raw_conn(),
        cursor as ffi::xproto::xcb_cursor_t); //1
    GetCursorNameCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetCursorNameUnchecked<'r> (c : &'r Connection,
                               cursor : xproto::Cursor) -> GetCursorNameCookie<'r> {
  unsafe {
    let cookie = xcb_xfixes_get_cursor_name_unchecked(c.get_raw_conn(),
        cursor as ffi::xproto::xcb_cursor_t); //1
    GetCursorNameCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetCursorNameReply {
  pub fn atom(&mut self) -> xproto::Atom {
    unsafe { accessor!(atom -> xproto::Atom, (*self.base.reply)) }
  }

  pub fn name(&mut self) -> String {
    unsafe { accessor!(str, xcb_xfixes_get_cursor_name_name_length, xcb_xfixes_get_cursor_name_name, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetCursorNameCookie<'s>, mk_reply_xcb_xfixes_get_cursor_name_reply_t, GetCursorNameReply, xcb_xfixes_get_cursor_name_reply);

pub struct GetCursorImageAndNameReply { base:  base::Reply<xcb_xfixes_get_cursor_image_and_name_reply_t> }
fn mk_reply_xcb_xfixes_get_cursor_image_and_name_reply_t(reply:*mut xcb_xfixes_get_cursor_image_and_name_reply_t) -> GetCursorImageAndNameReply { GetCursorImageAndNameReply { base : base::mk_reply(reply) } }
pub fn GetCursorImageAndName<'r> (c : &'r Connection) -> GetCursorImageAndNameCookie<'r> {
  unsafe {
    let cookie = xcb_xfixes_get_cursor_image_and_name(c.get_raw_conn());
    GetCursorImageAndNameCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetCursorImageAndNameUnchecked<'r> (c : &'r Connection) -> GetCursorImageAndNameCookie<'r> {
  unsafe {
    let cookie = xcb_xfixes_get_cursor_image_and_name_unchecked(c.get_raw_conn());
    GetCursorImageAndNameCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetCursorImageAndNameReply {
  pub fn x(&mut self) -> i16 {
    unsafe { accessor!(x -> i16, (*self.base.reply)) }
  }

  pub fn y(&mut self) -> i16 {
    unsafe { accessor!(y -> i16, (*self.base.reply)) }
  }

  pub fn height(&mut self) -> u16 {
    unsafe { accessor!(height -> u16, (*self.base.reply)) }
  }

  pub fn xhot(&mut self) -> u16 {
    unsafe { accessor!(xhot -> u16, (*self.base.reply)) }
  }

  pub fn yhot(&mut self) -> u16 {
    unsafe { accessor!(yhot -> u16, (*self.base.reply)) }
  }

  pub fn cursor_serial(&mut self) -> u32 {
    unsafe { accessor!(cursor_serial -> u32, (*self.base.reply)) }
  }

  pub fn cursor_atom(&mut self) -> xproto::Atom {
    unsafe { accessor!(cursor_atom -> xproto::Atom, (*self.base.reply)) }
  }

  pub fn name(&mut self) -> String {
    unsafe { accessor!(str, xcb_xfixes_get_cursor_image_and_name_name_length, xcb_xfixes_get_cursor_image_and_name_name, (*self.base.reply)) }
  }

  pub fn cursor_image(&mut self) -> Vec<u32> {
    unsafe { accessor!(u32, xcb_xfixes_get_cursor_image_and_name_cursor_image_length, xcb_xfixes_get_cursor_image_and_name_cursor_image, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetCursorImageAndNameCookie<'s>, mk_reply_xcb_xfixes_get_cursor_image_and_name_reply_t, GetCursorImageAndNameReply, xcb_xfixes_get_cursor_image_and_name_reply);

pub fn ChangeCursorChecked<'r> (c : &'r Connection,
                            source : xproto::Cursor,
                            destination : xproto::Cursor) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xfixes_change_cursor_checked(c.get_raw_conn(),
        source as ffi::xproto::xcb_cursor_t, //1
        destination as ffi::xproto::xcb_cursor_t); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn ChangeCursor<'r> (c : &'r Connection,
                     source : xproto::Cursor,
                     destination : xproto::Cursor) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xfixes_change_cursor(c.get_raw_conn(),
        source as ffi::xproto::xcb_cursor_t, //1
        destination as ffi::xproto::xcb_cursor_t); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn ChangeCursorByNameChecked<'r> (c : &'r Connection,
                                  src : xproto::Cursor,
                                  name : &str) -> base::VoidCookie<'r> {
  unsafe {
    let name = (name).as_bytes();
    let name_len = name.len();
    let name_ptr = name.as_ptr();
    let cookie = xcb_xfixes_change_cursor_by_name_checked(c.get_raw_conn(),
        src as ffi::xproto::xcb_cursor_t, //1
        name_len as u16, //2
        name_ptr as *mut c_char); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn ChangeCursorByName<'r> (c : &'r Connection,
                           src : xproto::Cursor,
                           name : &str) -> base::VoidCookie<'r> {
  unsafe {
    let name = (name).as_bytes();
    let name_len = name.len();
    let name_ptr = name.as_ptr();
    let cookie = xcb_xfixes_change_cursor_by_name(c.get_raw_conn(),
        src as ffi::xproto::xcb_cursor_t, //1
        name_len as u16, //2
        name_ptr as *mut c_char); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn ExpandRegionChecked<'r> (c : &'r Connection,
                            source : Region,
                            destination : Region,
                            left : u16,
                            right : u16,
                            top : u16,
                            bottom : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xfixes_expand_region_checked(c.get_raw_conn(),
        source as xcb_xfixes_region_t, //1
        destination as xcb_xfixes_region_t, //2
        left as u16, //3
        right as u16, //4
        top as u16, //5
        bottom as u16); //6
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn ExpandRegion<'r> (c : &'r Connection,
                     source : Region,
                     destination : Region,
                     left : u16,
                     right : u16,
                     top : u16,
                     bottom : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xfixes_expand_region(c.get_raw_conn(),
        source as xcb_xfixes_region_t, //1
        destination as xcb_xfixes_region_t, //2
        left as u16, //3
        right as u16, //4
        top as u16, //5
        bottom as u16); //6
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn HideCursorChecked<'r> (c : &'r Connection,
                          window : xproto::Window) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xfixes_hide_cursor_checked(c.get_raw_conn(),
        window as ffi::xproto::xcb_window_t); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn HideCursor<'r> (c : &'r Connection,
                   window : xproto::Window) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xfixes_hide_cursor(c.get_raw_conn(),
        window as ffi::xproto::xcb_window_t); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn ShowCursorChecked<'r> (c : &'r Connection,
                          window : xproto::Window) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xfixes_show_cursor_checked(c.get_raw_conn(),
        window as ffi::xproto::xcb_window_t); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn ShowCursor<'r> (c : &'r Connection,
                   window : xproto::Window) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xfixes_show_cursor(c.get_raw_conn(),
        window as ffi::xproto::xcb_window_t); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

