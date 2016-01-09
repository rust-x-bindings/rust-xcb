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
pub struct  GetVersionCookie<'s> { pub base : base::Cookie<'s, get_version_cookie> }

/** Opcode for xcb_test_get_version. */
pub static XCB_TEST_GET_VERSION : u8 = 0;
pub struct GetVersionReply { base:  base::Reply<get_version_reply> }
fn mk_reply_get_version_reply(reply:*mut get_version_reply) -> GetVersionReply { GetVersionReply { base : base::mk_reply(reply) } }

pub type cursor = c_uint;//{
    pub static XCB_TEST_CURSOR_NONE : cursor = 0;
    pub static XCB_TEST_CURSOR_CURRENT : cursor = 1;
//}
pub struct  CompareCursorCookie<'s> { pub base : base::Cookie<'s, compare_cursor_cookie> }

/** Opcode for xcb_test_compare_cursor. */
pub static XCB_TEST_COMPARE_CURSOR : u8 = 1;
pub struct CompareCursorReply { base:  base::Reply<compare_cursor_reply> }
fn mk_reply_compare_cursor_reply(reply:*mut compare_cursor_reply) -> CompareCursorReply { CompareCursorReply { base : base::mk_reply(reply) } }
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
impl_reply_cookie!(GetVersionCookie<'s>, mk_reply_get_version_reply, GetVersionReply, xcb_test_get_version_reply);

pub fn CompareCursor<'r> (c : &'r Connection,
                      window : xproto::Window,
                      cursor : xproto::Cursor) -> CompareCursorCookie<'r> {
  unsafe {
    let cookie = xcb_test_compare_cursor(c.get_raw_conn(),
        window as ffi::xproto::window, //1
        cursor as ffi::xproto::cursor); //2
    CompareCursorCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn CompareCursorUnchecked<'r> (c : &'r Connection,
                               window : xproto::Window,
                               cursor : xproto::Cursor) -> CompareCursorCookie<'r> {
  unsafe {
    let cookie = xcb_test_compare_cursor_unchecked(c.get_raw_conn(),
        window as ffi::xproto::window, //1
        cursor as ffi::xproto::cursor); //2
    CompareCursorCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl CompareCursorReply {
  pub fn same(&mut self) -> u8 {
    unsafe { accessor!(same -> u8, (*self.base.reply)) }
  }

}
impl_reply_cookie!(CompareCursorCookie<'s>, mk_reply_compare_cursor_reply, CompareCursorReply, xcb_test_compare_cursor_reply);

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
        root as ffi::xproto::window, //4
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
        root as ffi::xproto::window, //4
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

