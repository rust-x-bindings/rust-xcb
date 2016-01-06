/*
 * This file generated automatically from screensaver.xml by r_client.py.
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
use ffi::screensaver::*;
use std::option::Option;
use std::iter::Iterator;

use xproto;

pub type kind = c_uint;//{
    pub static XCB_SCREENSAVER_KIND_BLANKED : kind = 1;
    pub static XCB_SCREENSAVER_KIND_INTERNAL : kind = 2;
    pub static XCB_SCREENSAVER_KIND_EXTERNAL : kind = 3;
//}

pub type event = c_uint;//{
    pub static XCB_SCREENSAVER_EVENT_NOTIFY_MASK : event = 1;
    pub static XCB_SCREENSAVER_EVENT_CYCLE_MASK : event = 2;
//}

pub type state = c_uint;//{
    pub static XCB_SCREENSAVER_STATE_OFF : state = 1;
    pub static XCB_SCREENSAVER_STATE_ON : state = 2;
    pub static XCB_SCREENSAVER_STATE_CYCLE : state = 3;
    pub static XCB_SCREENSAVER_STATE_DISABLED : state = 4;
//}
pub struct  QueryVersionCookie<'s> { pub base : base::Cookie<'s, query_version_cookie> }

/** Opcode for xcb_screensaver_query_version. */
pub static XCB_SCREENSAVER_QUERY_VERSION : u8 = 0;
pub struct QueryVersionReply { base:  base::Reply<query_version_reply> }
fn mk_reply_query_version_reply(reply:*mut query_version_reply) -> QueryVersionReply { QueryVersionReply { base : base::mk_reply(reply) } }
pub struct  QueryInfoCookie<'s> { pub base : base::Cookie<'s, query_info_cookie> }

/** Opcode for xcb_screensaver_query_info. */
pub static XCB_SCREENSAVER_QUERY_INFO : u8 = 1;
pub struct QueryInfoReply { base:  base::Reply<query_info_reply> }
fn mk_reply_query_info_reply(reply:*mut query_info_reply) -> QueryInfoReply { QueryInfoReply { base : base::mk_reply(reply) } }
/** Opcode for xcb_screensaver_select_input. */
pub static XCB_SCREENSAVER_SELECT_INPUT : u8 = 2;
/** Opcode for xcb_screensaver_set_attributes. */
pub static XCB_SCREENSAVER_SET_ATTRIBUTES : u8 = 3;
/** Opcode for xcb_screensaver_unset_attributes. */
pub static XCB_SCREENSAVER_UNSET_ATTRIBUTES : u8 = 4;
/** Opcode for xcb_screensaver_suspend. */
pub static XCB_SCREENSAVER_SUSPEND : u8 = 5;
/** Opcode for xcb_screensaver_notify. */
pub static XCB_SCREENSAVER_NOTIFY : u8 = 0;
pub struct NotifyEvent {pub base : base::Event<notify_event>}
pub fn QueryVersion<'r> (c : &'r Connection,
                     client_major_version : u8,
                     client_minor_version : u8) -> QueryVersionCookie<'r> {
  unsafe {
    let cookie = xcb_screensaver_query_version(c.get_raw_conn(),
        client_major_version as u8, //1
        client_minor_version as u8); //2
    QueryVersionCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn QueryVersionUnchecked<'r> (c : &'r Connection,
                              client_major_version : u8,
                              client_minor_version : u8) -> QueryVersionCookie<'r> {
  unsafe {
    let cookie = xcb_screensaver_query_version_unchecked(c.get_raw_conn(),
        client_major_version as u8, //1
        client_minor_version as u8); //2
    QueryVersionCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl QueryVersionReply {
  pub fn server_major_version(&mut self) -> u16 {
    unsafe { accessor!(server_major_version -> u16, (*self.base.reply)) }
  }

  pub fn server_minor_version(&mut self) -> u16 {
    unsafe { accessor!(server_minor_version -> u16, (*self.base.reply)) }
  }

}
impl_reply_cookie!(QueryVersionCookie<'s>, mk_reply_query_version_reply, QueryVersionReply, xcb_screensaver_query_version_reply);

pub fn QueryInfo<'r> (c : &'r Connection,
                  drawable : xproto::Drawable) -> QueryInfoCookie<'r> {
  unsafe {
    let cookie = xcb_screensaver_query_info(c.get_raw_conn(),
        drawable as ffi::xproto::drawable); //1
    QueryInfoCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn QueryInfoUnchecked<'r> (c : &'r Connection,
                           drawable : xproto::Drawable) -> QueryInfoCookie<'r> {
  unsafe {
    let cookie = xcb_screensaver_query_info_unchecked(c.get_raw_conn(),
        drawable as ffi::xproto::drawable); //1
    QueryInfoCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl QueryInfoReply {
  pub fn state(&mut self) -> u8 {
    unsafe { accessor!(state -> u8, (*self.base.reply)) }
  }

  pub fn saver_window(&mut self) -> xproto::Window {
    unsafe { accessor!(saver_window -> xproto::Window, (*self.base.reply)) }
  }

  pub fn ms_until_server(&mut self) -> u32 {
    unsafe { accessor!(ms_until_server -> u32, (*self.base.reply)) }
  }

  pub fn ms_since_user_input(&mut self) -> u32 {
    unsafe { accessor!(ms_since_user_input -> u32, (*self.base.reply)) }
  }

  pub fn event_mask(&mut self) -> u32 {
    unsafe { accessor!(event_mask -> u32, (*self.base.reply)) }
  }

  pub fn kind(&mut self) -> u8 {
    unsafe { accessor!(kind -> u8, (*self.base.reply)) }
  }

}
impl_reply_cookie!(QueryInfoCookie<'s>, mk_reply_query_info_reply, QueryInfoReply, xcb_screensaver_query_info_reply);

pub fn SelectInputChecked<'r> (c : &'r Connection,
                           drawable : xproto::Drawable,
                           event_mask : u32) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_screensaver_select_input_checked(c.get_raw_conn(),
        drawable as ffi::xproto::drawable, //1
        event_mask as u32); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn SelectInput<'r> (c : &'r Connection,
                    drawable : xproto::Drawable,
                    event_mask : u32) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_screensaver_select_input(c.get_raw_conn(),
        drawable as ffi::xproto::drawable, //1
        event_mask as u32); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn SetAttributesChecked<'r> (c : &'r Connection,
                             drawable : xproto::Drawable,
                             x : i16,
                             y : i16,
                             width : u16,
                             height : u16,
                             border_width : u16,
                             class : u8,
                             depth : u8,
                             visual : xproto::Visualid,
                             value_list : &[(u32,u32)]) -> base::VoidCookie<'r> {
  unsafe {
    let mut value_list_copy = value_list.to_vec();
    let (value_list_mask, value_list_vec) = pack_bitfield(&mut value_list_copy);
    let value_list_ptr = value_list_vec.as_ptr();
    let cookie = xcb_screensaver_set_attributes_checked(c.get_raw_conn(),
        drawable as ffi::xproto::drawable, //1
        x as i16, //2
        y as i16, //3
        width as u16, //4
        height as u16, //5
        border_width as u16, //6
        class as u8, //7
        depth as u8, //8
        visual as ffi::xproto::visualid, //9
        value_list_mask as u32, //10
        value_list_ptr as *mut u32); //11
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn SetAttributes<'r> (c : &'r Connection,
                      drawable : xproto::Drawable,
                      x : i16,
                      y : i16,
                      width : u16,
                      height : u16,
                      border_width : u16,
                      class : u8,
                      depth : u8,
                      visual : xproto::Visualid,
                      value_list : &[(u32,u32)]) -> base::VoidCookie<'r> {
  unsafe {
    let mut value_list_copy = value_list.to_vec();
    let (value_list_mask, value_list_vec) = pack_bitfield(&mut value_list_copy);
    let value_list_ptr = value_list_vec.as_ptr();
    let cookie = xcb_screensaver_set_attributes(c.get_raw_conn(),
        drawable as ffi::xproto::drawable, //1
        x as i16, //2
        y as i16, //3
        width as u16, //4
        height as u16, //5
        border_width as u16, //6
        class as u8, //7
        depth as u8, //8
        visual as ffi::xproto::visualid, //9
        value_list_mask as u32, //10
        value_list_ptr as *mut u32); //11
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn UnsetAttributesChecked<'r> (c : &'r Connection,
                               drawable : xproto::Drawable) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_screensaver_unset_attributes_checked(c.get_raw_conn(),
        drawable as ffi::xproto::drawable); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn UnsetAttributes<'r> (c : &'r Connection,
                        drawable : xproto::Drawable) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_screensaver_unset_attributes(c.get_raw_conn(),
        drawable as ffi::xproto::drawable); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn SuspendChecked<'r> (c : &'r Connection,
                       suspend : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_screensaver_suspend_checked(c.get_raw_conn(),
        suspend as u8); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn Suspend<'r> (c : &'r Connection,
                suspend : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_screensaver_suspend(c.get_raw_conn(),
        suspend as u8); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl NotifyEvent {
  pub fn code(&mut self) -> u8 {
    unsafe { accessor!(code -> u8, (*self.base.event)) }
  }

  pub fn state(&mut self) -> u8 {
    unsafe { accessor!(state -> u8, (*self.base.event)) }
  }

  pub fn sequence_number(&mut self) -> u16 {
    unsafe { accessor!(sequence_number -> u16, (*self.base.event)) }
  }

  pub fn time(&mut self) -> xproto::Timestamp {
    unsafe { accessor!(time -> xproto::Timestamp, (*self.base.event)) }
  }

  pub fn root(&mut self) -> xproto::Window {
    unsafe { accessor!(root -> xproto::Window, (*self.base.event)) }
  }

  pub fn window(&mut self) -> xproto::Window {
    unsafe { accessor!(window -> xproto::Window, (*self.base.event)) }
  }

  pub fn kind(&mut self) -> u8 {
    unsafe { accessor!(kind -> u8, (*self.base.event)) }
  }

  pub fn forced(&mut self) -> u8 {
    unsafe { accessor!(forced -> u8, (*self.base.event)) }
  }

  pub fn new(code : u8,
         state : u8,
         sequence_number : u16,
         time : xproto::Timestamp,
         root : xproto::Window,
         window : xproto::Window,
         kind : u8,
         forced : u8) -> NotifyEvent {
    unsafe {
      let raw = malloc(32 as size_t) as *mut notify_event;
      (*raw).code = code;
      (*raw).state = state;
      (*raw).sequence_number = sequence_number;
      (*raw).time = time;
      (*raw).root = root;
      (*raw).window = window;
      (*raw).kind = kind;
      (*raw).forced = forced;
      NotifyEvent { base : Event { event : raw as *mut notify_event }}
    }
  }
}

