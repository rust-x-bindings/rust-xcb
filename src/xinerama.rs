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
pub type ScreenInfo = base::Struct<screen_info>;

pub type ScreenInfoIterator = screen_info_iterator;

pub type QueryVersionCookie<'s> = base::Cookie<'s, query_version_cookie>;

/** Opcode for xcb_xinerama_query_version. */
pub static XCB_XINERAMA_QUERY_VERSION : u8 = 0;
pub type QueryVersionReply = base::Reply<query_version_reply>;
pub type GetStateCookie<'s> = base::Cookie<'s, get_state_cookie>;

/** Opcode for xcb_xinerama_get_state. */
pub static XCB_XINERAMA_GET_STATE : u8 = 1;
pub type GetStateReply = base::Reply<get_state_reply>;
pub type GetScreenCountCookie<'s> = base::Cookie<'s, get_screen_count_cookie>;

/** Opcode for xcb_xinerama_get_screen_count. */
pub static XCB_XINERAMA_GET_SCREEN_COUNT : u8 = 2;
pub type GetScreenCountReply = base::Reply<get_screen_count_reply>;
pub type GetScreenSizeCookie<'s> = base::Cookie<'s, get_screen_size_cookie>;

/** Opcode for xcb_xinerama_get_screen_size. */
pub static XCB_XINERAMA_GET_SCREEN_SIZE : u8 = 3;
pub type GetScreenSizeReply = base::Reply<get_screen_size_reply>;
pub type IsActiveCookie<'s> = base::Cookie<'s, is_active_cookie>;

/** Opcode for xcb_xinerama_is_active. */
pub static XCB_XINERAMA_IS_ACTIVE : u8 = 4;
pub type IsActiveReply = base::Reply<is_active_reply>;
pub type QueryScreensCookie<'s> = base::Cookie<'s, query_screens_cookie>;

/** Opcode for xcb_xinerama_query_screens. */
pub static XCB_XINERAMA_QUERY_SCREENS : u8 = 5;

impl base::Struct<screen_info> {
  pub fn x_org(&self) -> i16 {
    unsafe { accessor!(x_org -> i16, s.strct) }
  }

  pub fn y_org(&self) -> i16 {
    unsafe { accessor!(y_org -> i16, s.strct) }
  }

  pub fn width(&self) -> u16 {
    unsafe { accessor!(width -> u16, s.strct) }
  }

  pub fn height(&self) -> u16 {
    unsafe { accessor!(height -> u16, s.strct) }
  }

}

impl<'s, ScreenInfo> Iterator<&'s ScreenInfo> for ScreenInfoIterator {
    pub fn next(&mut self) -> Option<&'s ScreenInfo> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *mut screen_info_iterator = mem::transmute(self);
            let data = (*iter).data;
            xcb_xinerama_screen_info_next(iter);
            Some(mem::transmute(data))
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
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn QueryVersionUnchecked<'r> (c : &'r Connection,
                              major : u8,
                              minor : u8) -> QueryVersionCookie<'r> {
  unsafe {
    let cookie = xcb_xinerama_query_version_unchecked(c.get_raw_conn(),
        major as u8, //1
        minor as u8); //2
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}

impl base::Reply<query_version_reply> {
  pub fn major(&self) -> u16 {
    unsafe { accessor!(major -> u16, (*self.reply)) }
  }

  pub fn minor(&self) -> u16 {
    unsafe { accessor!(minor -> u16, (*self.reply)) }
  }

}
impl_reply_cookie!(QueryVersionCookie<'s>, query_version_reply, QueryVersionReply, xcb_xinerama_query_version_reply)

pub fn GetState<'r> (c : &'r Connection,
                 window : xproto::Window) -> GetStateCookie<'r> {
  unsafe {
    let cookie = xcb_xinerama_get_state(c.get_raw_conn(),
        window as ffi::xproto::window); //1
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn GetStateUnchecked<'r> (c : &'r Connection,
                          window : xproto::Window) -> GetStateCookie<'r> {
  unsafe {
    let cookie = xcb_xinerama_get_state_unchecked(c.get_raw_conn(),
        window as ffi::xproto::window); //1
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}

impl base::Reply<get_state_reply> {
  pub fn state(&self) -> u8 {
    unsafe { accessor!(state -> u8, (*self.reply)) }
  }

  pub fn window(&self) -> xproto::Window {
    unsafe { accessor!(window -> xproto::Window, (*self.reply)) }
  }

}
impl_reply_cookie!(GetStateCookie<'s>, get_state_reply, GetStateReply, xcb_xinerama_get_state_reply)

pub fn GetScreenCount<'r> (c : &'r Connection,
                       window : xproto::Window) -> GetScreenCountCookie<'r> {
  unsafe {
    let cookie = xcb_xinerama_get_screen_count(c.get_raw_conn(),
        window as ffi::xproto::window); //1
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn GetScreenCountUnchecked<'r> (c : &'r Connection,
                                window : xproto::Window) -> GetScreenCountCookie<'r> {
  unsafe {
    let cookie = xcb_xinerama_get_screen_count_unchecked(c.get_raw_conn(),
        window as ffi::xproto::window); //1
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}

impl base::Reply<get_screen_count_reply> {
  pub fn screen_count(&self) -> u8 {
    unsafe { accessor!(screen_count -> u8, (*self.reply)) }
  }

  pub fn window(&self) -> xproto::Window {
    unsafe { accessor!(window -> xproto::Window, (*self.reply)) }
  }

}
impl_reply_cookie!(GetScreenCountCookie<'s>, get_screen_count_reply, GetScreenCountReply, xcb_xinerama_get_screen_count_reply)

pub fn GetScreenSize<'r> (c : &'r Connection,
                      window : xproto::Window,
                      screen : u32) -> GetScreenSizeCookie<'r> {
  unsafe {
    let cookie = xcb_xinerama_get_screen_size(c.get_raw_conn(),
        window as ffi::xproto::window, //1
        screen as u32); //2
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn GetScreenSizeUnchecked<'r> (c : &'r Connection,
                               window : xproto::Window,
                               screen : u32) -> GetScreenSizeCookie<'r> {
  unsafe {
    let cookie = xcb_xinerama_get_screen_size_unchecked(c.get_raw_conn(),
        window as ffi::xproto::window, //1
        screen as u32); //2
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}

impl base::Reply<get_screen_size_reply> {
  pub fn width(&self) -> u32 {
    unsafe { accessor!(width -> u32, (*self.reply)) }
  }

  pub fn height(&self) -> u32 {
    unsafe { accessor!(height -> u32, (*self.reply)) }
  }

  pub fn window(&self) -> xproto::Window {
    unsafe { accessor!(window -> xproto::Window, (*self.reply)) }
  }

  pub fn screen(&self) -> u32 {
    unsafe { accessor!(screen -> u32, (*self.reply)) }
  }

}
impl_reply_cookie!(GetScreenSizeCookie<'s>, get_screen_size_reply, GetScreenSizeReply, xcb_xinerama_get_screen_size_reply)

pub fn IsActive<'r> (c : &'r Connection) -> IsActiveCookie<'r> {
  unsafe {
    let cookie = xcb_xinerama_is_active(c.get_raw_conn());
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn IsActiveUnchecked<'r> (c : &'r Connection) -> IsActiveCookie<'r> {
  unsafe {
    let cookie = xcb_xinerama_is_active_unchecked(c.get_raw_conn());
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}

impl base::Reply<is_active_reply> {
  pub fn state(&self) -> u32 {
    unsafe { accessor!(state -> u32, (*self.reply)) }
  }

}
impl_reply_cookie!(IsActiveCookie<'s>, is_active_reply, IsActiveReply, xcb_xinerama_is_active_reply)

pub type QueryScreensReply = base::Reply<query_screens_reply>;
pub fn QueryScreens<'r> (c : &'r Connection) -> QueryScreensCookie<'r> {
  unsafe {
    let cookie = xcb_xinerama_query_screens(c.get_raw_conn());
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn QueryScreensUnchecked<'r> (c : &'r Connection) -> QueryScreensCookie<'r> {
  unsafe {
    let cookie = xcb_xinerama_query_screens_unchecked(c.get_raw_conn());
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}

impl base::Reply<query_screens_reply> {
  pub fn screen_info(&self) -> ScreenInfoIterator {
    unsafe { accessor!(ScreenInfoIterator, xcb_xinerama_query_screens_screen_info_iterator, (*self.reply)) }
  }

}
impl_reply_cookie!(QueryScreensCookie<'s>, query_screens_reply, QueryScreensReply, xcb_xinerama_query_screens_reply)


