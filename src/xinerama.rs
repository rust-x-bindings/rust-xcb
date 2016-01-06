/*
 * This file generated automatically from xinerama.xml by r_client.py.
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
use ffi::xinerama::*;
use std::option::Option;
use std::iter::Iterator;

use xproto;
pub struct ScreenInfo {pub base : base::Struct<screen_info> }

pub type ScreenInfoIterator = screen_info_iterator;

pub struct  QueryVersionCookie<'s> { pub base : base::Cookie<'s, query_version_cookie> }

/** Opcode for xcb_xinerama_query_version. */
pub static XCB_XINERAMA_QUERY_VERSION : u8 = 0;
pub struct QueryVersionReply { base:  base::Reply<query_version_reply> }
fn mk_reply_query_version_reply(reply:*mut query_version_reply) -> QueryVersionReply { QueryVersionReply { base : base::mk_reply(reply) } }
pub struct  GetStateCookie<'s> { pub base : base::Cookie<'s, get_state_cookie> }

/** Opcode for xcb_xinerama_get_state. */
pub static XCB_XINERAMA_GET_STATE : u8 = 1;
pub struct GetStateReply { base:  base::Reply<get_state_reply> }
fn mk_reply_get_state_reply(reply:*mut get_state_reply) -> GetStateReply { GetStateReply { base : base::mk_reply(reply) } }
pub struct  GetScreenCountCookie<'s> { pub base : base::Cookie<'s, get_screen_count_cookie> }

/** Opcode for xcb_xinerama_get_screen_count. */
pub static XCB_XINERAMA_GET_SCREEN_COUNT : u8 = 2;
pub struct GetScreenCountReply { base:  base::Reply<get_screen_count_reply> }
fn mk_reply_get_screen_count_reply(reply:*mut get_screen_count_reply) -> GetScreenCountReply { GetScreenCountReply { base : base::mk_reply(reply) } }
pub struct  GetScreenSizeCookie<'s> { pub base : base::Cookie<'s, get_screen_size_cookie> }

/** Opcode for xcb_xinerama_get_screen_size. */
pub static XCB_XINERAMA_GET_SCREEN_SIZE : u8 = 3;
pub struct GetScreenSizeReply { base:  base::Reply<get_screen_size_reply> }
fn mk_reply_get_screen_size_reply(reply:*mut get_screen_size_reply) -> GetScreenSizeReply { GetScreenSizeReply { base : base::mk_reply(reply) } }
pub struct  IsActiveCookie<'s> { pub base : base::Cookie<'s, is_active_cookie> }

/** Opcode for xcb_xinerama_is_active. */
pub static XCB_XINERAMA_IS_ACTIVE : u8 = 4;
pub struct IsActiveReply { base:  base::Reply<is_active_reply> }
fn mk_reply_is_active_reply(reply:*mut is_active_reply) -> IsActiveReply { IsActiveReply { base : base::mk_reply(reply) } }
pub struct  QueryScreensCookie<'s> { pub base : base::Cookie<'s, query_screens_cookie> }

/** Opcode for xcb_xinerama_query_screens. */
pub static XCB_XINERAMA_QUERY_SCREENS : u8 = 5;

impl ScreenInfo {
  pub fn x_org(&mut self) -> i16 {
    unsafe { accessor!(x_org -> i16, self.base.strct) }
  }

  pub fn y_org(&mut self) -> i16 {
    unsafe { accessor!(y_org -> i16, self.base.strct) }
  }

  pub fn width(&mut self) -> u16 {
    unsafe { accessor!(width -> u16, self.base.strct) }
  }

  pub fn height(&mut self) -> u16 {
    unsafe { accessor!(height -> u16, self.base.strct) }
  }

}

impl Iterator for ScreenInfoIterator {
    type Item = ScreenInfo;
    fn next(&mut self) -> Option<ScreenInfo> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut screen_info_iterator = mem::transmute(self);
            let data = (*iter).data;
            xcb_xinerama_screen_info_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub fn QueryVersion<'r> (c : &'r Connection,
                     major : u8,
                     minor : u8) -> QueryVersionCookie<'r> {
  unsafe {
    let cookie = xcb_xinerama_query_version(c.get_raw_conn(),
        major as u8, //1
        minor as u8); //2
    QueryVersionCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn QueryVersionUnchecked<'r> (c : &'r Connection,
                              major : u8,
                              minor : u8) -> QueryVersionCookie<'r> {
  unsafe {
    let cookie = xcb_xinerama_query_version_unchecked(c.get_raw_conn(),
        major as u8, //1
        minor as u8); //2
    QueryVersionCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl QueryVersionReply {
  pub fn major(&mut self) -> u16 {
    unsafe { accessor!(major -> u16, (*self.base.reply)) }
  }

  pub fn minor(&mut self) -> u16 {
    unsafe { accessor!(minor -> u16, (*self.base.reply)) }
  }

}
impl_reply_cookie!(QueryVersionCookie<'s>, mk_reply_query_version_reply, QueryVersionReply, xcb_xinerama_query_version_reply);

pub fn GetState<'r> (c : &'r Connection,
                 window : xproto::Window) -> GetStateCookie<'r> {
  unsafe {
    let cookie = xcb_xinerama_get_state(c.get_raw_conn(),
        window as ffi::xproto::window); //1
    GetStateCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetStateUnchecked<'r> (c : &'r Connection,
                          window : xproto::Window) -> GetStateCookie<'r> {
  unsafe {
    let cookie = xcb_xinerama_get_state_unchecked(c.get_raw_conn(),
        window as ffi::xproto::window); //1
    GetStateCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetStateReply {
  pub fn state(&mut self) -> u8 {
    unsafe { accessor!(state -> u8, (*self.base.reply)) }
  }

  pub fn window(&mut self) -> xproto::Window {
    unsafe { accessor!(window -> xproto::Window, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetStateCookie<'s>, mk_reply_get_state_reply, GetStateReply, xcb_xinerama_get_state_reply);

pub fn GetScreenCount<'r> (c : &'r Connection,
                       window : xproto::Window) -> GetScreenCountCookie<'r> {
  unsafe {
    let cookie = xcb_xinerama_get_screen_count(c.get_raw_conn(),
        window as ffi::xproto::window); //1
    GetScreenCountCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetScreenCountUnchecked<'r> (c : &'r Connection,
                                window : xproto::Window) -> GetScreenCountCookie<'r> {
  unsafe {
    let cookie = xcb_xinerama_get_screen_count_unchecked(c.get_raw_conn(),
        window as ffi::xproto::window); //1
    GetScreenCountCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetScreenCountReply {
  pub fn screen_count(&mut self) -> u8 {
    unsafe { accessor!(screen_count -> u8, (*self.base.reply)) }
  }

  pub fn window(&mut self) -> xproto::Window {
    unsafe { accessor!(window -> xproto::Window, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetScreenCountCookie<'s>, mk_reply_get_screen_count_reply, GetScreenCountReply, xcb_xinerama_get_screen_count_reply);

pub fn GetScreenSize<'r> (c : &'r Connection,
                      window : xproto::Window,
                      screen : u32) -> GetScreenSizeCookie<'r> {
  unsafe {
    let cookie = xcb_xinerama_get_screen_size(c.get_raw_conn(),
        window as ffi::xproto::window, //1
        screen as u32); //2
    GetScreenSizeCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetScreenSizeUnchecked<'r> (c : &'r Connection,
                               window : xproto::Window,
                               screen : u32) -> GetScreenSizeCookie<'r> {
  unsafe {
    let cookie = xcb_xinerama_get_screen_size_unchecked(c.get_raw_conn(),
        window as ffi::xproto::window, //1
        screen as u32); //2
    GetScreenSizeCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetScreenSizeReply {
  pub fn width(&mut self) -> u32 {
    unsafe { accessor!(width -> u32, (*self.base.reply)) }
  }

  pub fn height(&mut self) -> u32 {
    unsafe { accessor!(height -> u32, (*self.base.reply)) }
  }

  pub fn window(&mut self) -> xproto::Window {
    unsafe { accessor!(window -> xproto::Window, (*self.base.reply)) }
  }

  pub fn screen(&mut self) -> u32 {
    unsafe { accessor!(screen -> u32, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetScreenSizeCookie<'s>, mk_reply_get_screen_size_reply, GetScreenSizeReply, xcb_xinerama_get_screen_size_reply);

pub fn IsActive<'r> (c : &'r Connection) -> IsActiveCookie<'r> {
  unsafe {
    let cookie = xcb_xinerama_is_active(c.get_raw_conn());
    IsActiveCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn IsActiveUnchecked<'r> (c : &'r Connection) -> IsActiveCookie<'r> {
  unsafe {
    let cookie = xcb_xinerama_is_active_unchecked(c.get_raw_conn());
    IsActiveCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl IsActiveReply {
  pub fn state(&mut self) -> u32 {
    unsafe { accessor!(state -> u32, (*self.base.reply)) }
  }

}
impl_reply_cookie!(IsActiveCookie<'s>, mk_reply_is_active_reply, IsActiveReply, xcb_xinerama_is_active_reply);

pub struct QueryScreensReply { base:  base::Reply<query_screens_reply> }
fn mk_reply_query_screens_reply(reply:*mut query_screens_reply) -> QueryScreensReply { QueryScreensReply { base : base::mk_reply(reply) } }
pub fn QueryScreens<'r> (c : &'r Connection) -> QueryScreensCookie<'r> {
  unsafe {
    let cookie = xcb_xinerama_query_screens(c.get_raw_conn());
    QueryScreensCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn QueryScreensUnchecked<'r> (c : &'r Connection) -> QueryScreensCookie<'r> {
  unsafe {
    let cookie = xcb_xinerama_query_screens_unchecked(c.get_raw_conn());
    QueryScreensCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl QueryScreensReply {
  pub fn screen_info(&mut self) -> ScreenInfoIterator {
    unsafe { accessor!(ScreenInfoIterator, xcb_xinerama_query_screens_screen_info_iterator, (*self.base.reply)) }
  }

}
impl_reply_cookie!(QueryScreensCookie<'s>, mk_reply_query_screens_reply, QueryScreensReply, xcb_xinerama_query_screens_reply);


