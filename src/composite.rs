/*
 * This file generated automatically from composite.xml by r_client.py.
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
use ffi::composite::*;
use std::option::Option;
use std::iter::Iterator;

use xproto;
use render;
use shape;
use xfixes;

pub type redirect = c_uint;//{
    pub static XCB_COMPOSITE_REDIRECT_AUTOMATIC : redirect = 1;
    pub static XCB_COMPOSITE_REDIRECT_MANUAL : redirect = 2;
//}
pub struct  QueryVersionCookie<'s> { pub base : base::Cookie<'s, query_version_cookie> }

/** Opcode for xcb_composite_query_version. */
pub static XCB_COMPOSITE_QUERY_VERSION : u8 = 0;
pub struct QueryVersionReply { base:  base::Reply<query_version_reply> }
fn mk_reply_query_version_reply(reply:*mut query_version_reply) -> QueryVersionReply { QueryVersionReply { base : base::mk_reply(reply) } }
/** Opcode for xcb_composite_redirect_window. */
pub static XCB_COMPOSITE_REDIRECT_WINDOW : u8 = 1;
/** Opcode for xcb_composite_redirect_subwindows. */
pub static XCB_COMPOSITE_REDIRECT_SUBWINDOWS : u8 = 2;
/** Opcode for xcb_composite_unredirect_window. */
pub static XCB_COMPOSITE_UNREDIRECT_WINDOW : u8 = 3;
/** Opcode for xcb_composite_unredirect_subwindows. */
pub static XCB_COMPOSITE_UNREDIRECT_SUBWINDOWS : u8 = 4;
/** Opcode for xcb_composite_create_region_from_border_clip. */
pub static XCB_COMPOSITE_CREATE_REGION_FROM_BORDER_CLIP : u8 = 5;
/** Opcode for xcb_composite_name_window_pixmap. */
pub static XCB_COMPOSITE_NAME_WINDOW_PIXMAP : u8 = 6;
pub struct  GetOverlayWindowCookie<'s> { pub base : base::Cookie<'s, get_overlay_window_cookie> }

/** Opcode for xcb_composite_get_overlay_window. */
pub static XCB_COMPOSITE_GET_OVERLAY_WINDOW : u8 = 7;
pub struct GetOverlayWindowReply { base:  base::Reply<get_overlay_window_reply> }
fn mk_reply_get_overlay_window_reply(reply:*mut get_overlay_window_reply) -> GetOverlayWindowReply { GetOverlayWindowReply { base : base::mk_reply(reply) } }
/** Opcode for xcb_composite_release_overlay_window. */
pub static XCB_COMPOSITE_RELEASE_OVERLAY_WINDOW : u8 = 8;
pub fn QueryVersion<'r> (c : &'r Connection,
                     client_major_version : u32,
                     client_minor_version : u32) -> QueryVersionCookie<'r> {
  unsafe {
    let cookie = xcb_composite_query_version(c.get_raw_conn(),
        client_major_version as u32, //1
        client_minor_version as u32); //2
    QueryVersionCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn QueryVersionUnchecked<'r> (c : &'r Connection,
                              client_major_version : u32,
                              client_minor_version : u32) -> QueryVersionCookie<'r> {
  unsafe {
    let cookie = xcb_composite_query_version_unchecked(c.get_raw_conn(),
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
impl_reply_cookie!(QueryVersionCookie<'s>, mk_reply_query_version_reply, QueryVersionReply, xcb_composite_query_version_reply);

pub fn RedirectWindowChecked<'r> (c : &'r Connection,
                              window : xproto::Window,
                              update : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_composite_redirect_window_checked(c.get_raw_conn(),
        window as ffi::xproto::window, //1
        update as u8); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn RedirectWindow<'r> (c : &'r Connection,
                       window : xproto::Window,
                       update : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_composite_redirect_window(c.get_raw_conn(),
        window as ffi::xproto::window, //1
        update as u8); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn RedirectSubwindowsChecked<'r> (c : &'r Connection,
                                  window : xproto::Window,
                                  update : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_composite_redirect_subwindows_checked(c.get_raw_conn(),
        window as ffi::xproto::window, //1
        update as u8); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn RedirectSubwindows<'r> (c : &'r Connection,
                           window : xproto::Window,
                           update : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_composite_redirect_subwindows(c.get_raw_conn(),
        window as ffi::xproto::window, //1
        update as u8); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn UnredirectWindowChecked<'r> (c : &'r Connection,
                                window : xproto::Window,
                                update : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_composite_unredirect_window_checked(c.get_raw_conn(),
        window as ffi::xproto::window, //1
        update as u8); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn UnredirectWindow<'r> (c : &'r Connection,
                         window : xproto::Window,
                         update : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_composite_unredirect_window(c.get_raw_conn(),
        window as ffi::xproto::window, //1
        update as u8); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn UnredirectSubwindowsChecked<'r> (c : &'r Connection,
                                    window : xproto::Window,
                                    update : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_composite_unredirect_subwindows_checked(c.get_raw_conn(),
        window as ffi::xproto::window, //1
        update as u8); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn UnredirectSubwindows<'r> (c : &'r Connection,
                             window : xproto::Window,
                             update : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_composite_unredirect_subwindows(c.get_raw_conn(),
        window as ffi::xproto::window, //1
        update as u8); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn CreateRegionFromBorderClipChecked<'r> (c : &'r Connection,
                                          region : xfixes::Region,
                                          window : xproto::Window) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_composite_create_region_from_border_clip_checked(c.get_raw_conn(),
        region as ffi::xfixes::region, //1
        window as ffi::xproto::window); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn CreateRegionFromBorderClip<'r> (c : &'r Connection,
                                   region : xfixes::Region,
                                   window : xproto::Window) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_composite_create_region_from_border_clip(c.get_raw_conn(),
        region as ffi::xfixes::region, //1
        window as ffi::xproto::window); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn NameWindowPixmapChecked<'r> (c : &'r Connection,
                                window : xproto::Window,
                                pixmap : xproto::Pixmap) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_composite_name_window_pixmap_checked(c.get_raw_conn(),
        window as ffi::xproto::window, //1
        pixmap as ffi::xproto::pixmap); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn NameWindowPixmap<'r> (c : &'r Connection,
                         window : xproto::Window,
                         pixmap : xproto::Pixmap) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_composite_name_window_pixmap(c.get_raw_conn(),
        window as ffi::xproto::window, //1
        pixmap as ffi::xproto::pixmap); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetOverlayWindow<'r> (c : &'r Connection,
                         window : xproto::Window) -> GetOverlayWindowCookie<'r> {
  unsafe {
    let cookie = xcb_composite_get_overlay_window(c.get_raw_conn(),
        window as ffi::xproto::window); //1
    GetOverlayWindowCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetOverlayWindowUnchecked<'r> (c : &'r Connection,
                                  window : xproto::Window) -> GetOverlayWindowCookie<'r> {
  unsafe {
    let cookie = xcb_composite_get_overlay_window_unchecked(c.get_raw_conn(),
        window as ffi::xproto::window); //1
    GetOverlayWindowCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetOverlayWindowReply {
  pub fn overlay_win(&mut self) -> xproto::Window {
    unsafe { accessor!(overlay_win -> xproto::Window, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetOverlayWindowCookie<'s>, mk_reply_get_overlay_window_reply, GetOverlayWindowReply, xcb_composite_get_overlay_window_reply);

pub fn ReleaseOverlayWindowChecked<'r> (c : &'r Connection,
                                    window : xproto::Window) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_composite_release_overlay_window_checked(c.get_raw_conn(),
        window as ffi::xproto::window); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn ReleaseOverlayWindow<'r> (c : &'r Connection,
                             window : xproto::Window) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_composite_release_overlay_window(c.get_raw_conn(),
        window as ffi::xproto::window); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

