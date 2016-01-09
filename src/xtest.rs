/*
 * This file generated automatically from xtest.xml by r_client.py.
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
use ffi::xtest::*;
use std::option::Option;
use std::iter::Iterator;

use xproto;
pub struct  GetVersionCookie<'s> { pub base : base::Cookie<'s, xcb_test_get_version_cookie_t> }

/** Opcode for xcb_test_get_version. */
pub static XCB_TEST_GET_VERSION : u8 = 0;
pub struct GetVersionReply { base:  base::Reply<xcb_test_get_version_reply_t> }
fn mk_reply_xcb_test_get_version_reply_t(reply:*mut xcb_test_get_version_reply_t) -> GetVersionReply { GetVersionReply { base : base::mk_reply(reply) } }

pub type xcb_test_cursor_t = c_uint;//{
    pub static XCB_TEST_CURSOR_NONE : xcb_test_cursor_t = 0;
    pub static XCB_TEST_CURSOR_CURRENT : xcb_test_cursor_t = 1;
//}
pub struct  CompareCursorCookie<'s> { pub base : base::Cookie<'s, xcb_test_compare_cursor_cookie_t> }

/** Opcode for xcb_test_compare_cursor. */
pub static XCB_TEST_COMPARE_CURSOR : u8 = 1;
pub struct CompareCursorReply { base:  base::Reply<xcb_test_compare_cursor_reply_t> }
fn mk_reply_xcb_test_compare_cursor_reply_t(reply:*mut xcb_test_compare_cursor_reply_t) -> CompareCursorReply { CompareCursorReply { base : base::mk_reply(reply) } }
/** Opcode for xcb_test_fake_input. */
pub static XCB_TEST_FAKE_INPUT : u8 = 2;
/** Opcode for xcb_test_grab_control. */
pub static XCB_TEST_GRAB_CONTROL : u8 = 3;
pub fn GetVersion<'r> (c : &'r Connection,
                   major_version : u8,
                   minor_version : u16) -> GetVersionCookie<'r> {
  unsafe {
    let cookie = xcb_test_get_version(c.get_raw_conn(),
        major_version as u8, //1
        minor_version as u16); //2
    GetVersionCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetVersionUnchecked<'r> (c : &'r Connection,
                            major_version : u8,
                            minor_version : u16) -> GetVersionCookie<'r> {
  unsafe {
    let cookie = xcb_test_get_version_unchecked(c.get_raw_conn(),
        major_version as u8, //1
        minor_version as u16); //2
    GetVersionCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetVersionReply {
  pub fn major_version(&mut self) -> u8 {
    unsafe { accessor!(major_version -> u8, (*self.base.reply)) }
  }

  pub fn minor_version(&mut self) -> u16 {
    unsafe { accessor!(minor_version -> u16, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetVersionCookie<'s>, mk_reply_xcb_test_get_version_reply_t, GetVersionReply, xcb_test_get_version_reply);

pub fn CompareCursor<'r> (c : &'r Connection,
                      window : xproto::Window,
                      cursor : xproto::Cursor) -> CompareCursorCookie<'r> {
  unsafe {
    let cookie = xcb_test_compare_cursor(c.get_raw_conn(),
        window as ffi::xproto::xcb_window_t, //1
        cursor as ffi::xproto::xcb_cursor_t); //2
    CompareCursorCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn CompareCursorUnchecked<'r> (c : &'r Connection,
                               window : xproto::Window,
                               cursor : xproto::Cursor) -> CompareCursorCookie<'r> {
  unsafe {
    let cookie = xcb_test_compare_cursor_unchecked(c.get_raw_conn(),
        window as ffi::xproto::xcb_window_t, //1
        cursor as ffi::xproto::xcb_cursor_t); //2
    CompareCursorCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl CompareCursorReply {
  pub fn same(&mut self) -> u8 {
    unsafe { accessor!(same -> u8, (*self.base.reply)) }
  }

}
impl_reply_cookie!(CompareCursorCookie<'s>, mk_reply_xcb_test_compare_cursor_reply_t, CompareCursorReply, xcb_test_compare_cursor_reply);

pub fn FakeInputChecked<'r> (c : &'r Connection,
                         type_ : u8,
                         detail : u8,
                         time : u32,
                         root : xproto::Window,
                         rootX : i16,
                         rootY : i16,
                         deviceid : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_test_fake_input_checked(c.get_raw_conn(),
        type_ as u8, //1
        detail as u8, //2
        time as u32, //3
        root as ffi::xproto::xcb_window_t, //4
        rootX as i16, //5
        rootY as i16, //6
        deviceid as u8); //7
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn FakeInput<'r> (c : &'r Connection,
                  type_ : u8,
                  detail : u8,
                  time : u32,
                  root : xproto::Window,
                  rootX : i16,
                  rootY : i16,
                  deviceid : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_test_fake_input(c.get_raw_conn(),
        type_ as u8, //1
        detail as u8, //2
        time as u32, //3
        root as ffi::xproto::xcb_window_t, //4
        rootX as i16, //5
        rootY as i16, //6
        deviceid as u8); //7
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GrabControlChecked<'r> (c : &'r Connection,
                           impervious : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_test_grab_control_checked(c.get_raw_conn(),
        impervious as u8); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn GrabControl<'r> (c : &'r Connection,
                    impervious : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_test_grab_control(c.get_raw_conn(),
        impervious as u8); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

