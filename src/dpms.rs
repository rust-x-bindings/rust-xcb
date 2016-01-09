/*
 * This file generated automatically from dpms.xml by r_client.py.
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
use ffi::dpms::*;
use std::option::Option;
use std::iter::Iterator;

pub struct  GetVersionCookie<'s> { pub base : base::Cookie<'s, xcb_dpms_get_version_cookie_t> }

/** Opcode for xcb_dpms_get_version. */
pub const XCB_DPMS_GET_VERSION : u8 = 0;
pub struct GetVersionReply { base:  base::Reply<xcb_dpms_get_version_reply_t> }
fn mk_reply_xcb_dpms_get_version_reply_t(reply:*mut xcb_dpms_get_version_reply_t) -> GetVersionReply { GetVersionReply { base : base::mk_reply(reply) } }
pub struct  CapableCookie<'s> { pub base : base::Cookie<'s, xcb_dpms_capable_cookie_t> }

/** Opcode for xcb_dpms_capable. */
pub const XCB_DPMS_CAPABLE : u8 = 1;
pub struct CapableReply { base:  base::Reply<xcb_dpms_capable_reply_t> }
fn mk_reply_xcb_dpms_capable_reply_t(reply:*mut xcb_dpms_capable_reply_t) -> CapableReply { CapableReply { base : base::mk_reply(reply) } }
pub struct  GetTimeoutsCookie<'s> { pub base : base::Cookie<'s, xcb_dpms_get_timeouts_cookie_t> }

/** Opcode for xcb_dpms_get_timeouts. */
pub const XCB_DPMS_GET_TIMEOUTS : u8 = 2;
pub struct GetTimeoutsReply { base:  base::Reply<xcb_dpms_get_timeouts_reply_t> }
fn mk_reply_xcb_dpms_get_timeouts_reply_t(reply:*mut xcb_dpms_get_timeouts_reply_t) -> GetTimeoutsReply { GetTimeoutsReply { base : base::mk_reply(reply) } }
/** Opcode for xcb_dpms_set_timeouts. */
pub const XCB_DPMS_SET_TIMEOUTS : u8 = 3;
/** Opcode for xcb_dpms_enable. */
pub const XCB_DPMS_ENABLE : u8 = 4;
/** Opcode for xcb_dpms_disable. */
pub const XCB_DPMS_DISABLE : u8 = 5;

pub type xcb_dpms_dpms_mode_t = c_uint;//{
    pub const XCB_DPMS_DPMS_MODE_ON : xcb_dpms_dpms_mode_t = 1;
    pub const XCB_DPMS_DPMS_MODE_STANDBY : xcb_dpms_dpms_mode_t = 2;
    pub const XCB_DPMS_DPMS_MODE_SUSPEND : xcb_dpms_dpms_mode_t = 3;
    pub const XCB_DPMS_DPMS_MODE_OFF : xcb_dpms_dpms_mode_t = 4;
//}
/** Opcode for xcb_dpms_force_level. */
pub const XCB_DPMS_FORCE_LEVEL : u8 = 6;
pub struct  InfoCookie<'s> { pub base : base::Cookie<'s, xcb_dpms_info_cookie_t> }

/** Opcode for xcb_dpms_info. */
pub const XCB_DPMS_INFO : u8 = 7;
pub struct InfoReply { base:  base::Reply<xcb_dpms_info_reply_t> }
fn mk_reply_xcb_dpms_info_reply_t(reply:*mut xcb_dpms_info_reply_t) -> InfoReply { InfoReply { base : base::mk_reply(reply) } }
pub fn GetVersion<'r> (c : &'r Connection,
                   client_major_version : u16,
                   client_minor_version : u16) -> GetVersionCookie<'r> {
  unsafe {
    let cookie = xcb_dpms_get_version(c.get_raw_conn(),
        client_major_version as u16, //1
        client_minor_version as u16); //2
    GetVersionCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetVersionUnchecked<'r> (c : &'r Connection,
                            client_major_version : u16,
                            client_minor_version : u16) -> GetVersionCookie<'r> {
  unsafe {
    let cookie = xcb_dpms_get_version_unchecked(c.get_raw_conn(),
        client_major_version as u16, //1
        client_minor_version as u16); //2
    GetVersionCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetVersionReply {
  pub fn server_major_version(&mut self) -> u16 {
    unsafe { accessor!(server_major_version -> u16, (*self.base.reply)) }
  }

  pub fn server_minor_version(&mut self) -> u16 {
    unsafe { accessor!(server_minor_version -> u16, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetVersionCookie<'s>, mk_reply_xcb_dpms_get_version_reply_t, GetVersionReply, xcb_dpms_get_version_reply);

pub fn Capable<'r> (c : &'r Connection) -> CapableCookie<'r> {
  unsafe {
    let cookie = xcb_dpms_capable(c.get_raw_conn());
    CapableCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn CapableUnchecked<'r> (c : &'r Connection) -> CapableCookie<'r> {
  unsafe {
    let cookie = xcb_dpms_capable_unchecked(c.get_raw_conn());
    CapableCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl CapableReply {
  pub fn capable(&mut self) -> u8 {
    unsafe { accessor!(capable -> u8, (*self.base.reply)) }
  }

}
impl_reply_cookie!(CapableCookie<'s>, mk_reply_xcb_dpms_capable_reply_t, CapableReply, xcb_dpms_capable_reply);

pub fn GetTimeouts<'r> (c : &'r Connection) -> GetTimeoutsCookie<'r> {
  unsafe {
    let cookie = xcb_dpms_get_timeouts(c.get_raw_conn());
    GetTimeoutsCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetTimeoutsUnchecked<'r> (c : &'r Connection) -> GetTimeoutsCookie<'r> {
  unsafe {
    let cookie = xcb_dpms_get_timeouts_unchecked(c.get_raw_conn());
    GetTimeoutsCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetTimeoutsReply {
  pub fn standby_timeout(&mut self) -> u16 {
    unsafe { accessor!(standby_timeout -> u16, (*self.base.reply)) }
  }

  pub fn suspend_timeout(&mut self) -> u16 {
    unsafe { accessor!(suspend_timeout -> u16, (*self.base.reply)) }
  }

  pub fn off_timeout(&mut self) -> u16 {
    unsafe { accessor!(off_timeout -> u16, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetTimeoutsCookie<'s>, mk_reply_xcb_dpms_get_timeouts_reply_t, GetTimeoutsReply, xcb_dpms_get_timeouts_reply);

pub fn SetTimeoutsChecked<'r> (c : &'r Connection,
                           standby_timeout : u16,
                           suspend_timeout : u16,
                           off_timeout : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_dpms_set_timeouts_checked(c.get_raw_conn(),
        standby_timeout as u16, //1
        suspend_timeout as u16, //2
        off_timeout as u16); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn SetTimeouts<'r> (c : &'r Connection,
                    standby_timeout : u16,
                    suspend_timeout : u16,
                    off_timeout : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_dpms_set_timeouts(c.get_raw_conn(),
        standby_timeout as u16, //1
        suspend_timeout as u16, //2
        off_timeout as u16); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn EnableChecked<'r> (c : &'r Connection) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_dpms_enable_checked(c.get_raw_conn());
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn Enable<'r> (c : &'r Connection) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_dpms_enable(c.get_raw_conn());
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn DisableChecked<'r> (c : &'r Connection) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_dpms_disable_checked(c.get_raw_conn());
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn Disable<'r> (c : &'r Connection) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_dpms_disable(c.get_raw_conn());
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn ForceLevelChecked<'r> (c : &'r Connection,
                          power_level : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_dpms_force_level_checked(c.get_raw_conn(),
        power_level as u16); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn ForceLevel<'r> (c : &'r Connection,
                   power_level : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_dpms_force_level(c.get_raw_conn(),
        power_level as u16); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn Info<'r> (c : &'r Connection) -> InfoCookie<'r> {
  unsafe {
    let cookie = xcb_dpms_info(c.get_raw_conn());
    InfoCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn InfoUnchecked<'r> (c : &'r Connection) -> InfoCookie<'r> {
  unsafe {
    let cookie = xcb_dpms_info_unchecked(c.get_raw_conn());
    InfoCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl InfoReply {
  pub fn power_level(&mut self) -> u16 {
    unsafe { accessor!(power_level -> u16, (*self.base.reply)) }
  }

  pub fn state(&mut self) -> u8 {
    unsafe { accessor!(state -> u8, (*self.base.reply)) }
  }

}
impl_reply_cookie!(InfoCookie<'s>, mk_reply_xcb_dpms_info_reply_t, InfoReply, xcb_dpms_info_reply);


