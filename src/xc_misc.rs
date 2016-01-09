/*
 * This file generated automatically from xc_misc.xml by r_client.py.
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
use ffi::xc_misc::*;
use std::option::Option;
use std::iter::Iterator;

pub struct  GetVersionCookie<'s> { pub base : base::Cookie<'s, xcb_xc_misc_get_version_cookie_t> }

/** Opcode for xcb_xc_misc_get_version. */
pub const XCB_XC_MISC_GET_VERSION : u8 = 0;
pub struct GetVersionReply { base:  base::Reply<xcb_xc_misc_get_version_reply_t> }
fn mk_reply_xcb_xc_misc_get_version_reply_t(reply:*mut xcb_xc_misc_get_version_reply_t) -> GetVersionReply { GetVersionReply { base : base::mk_reply(reply) } }
pub struct  GetXidRangeCookie<'s> { pub base : base::Cookie<'s, xcb_xc_misc_get_xid_range_cookie_t> }

/** Opcode for xcb_xc_misc_get_xid_range. */
pub const XCB_XC_MISC_GET_XID_RANGE : u8 = 1;
pub struct GetXidRangeReply { base:  base::Reply<xcb_xc_misc_get_xid_range_reply_t> }
fn mk_reply_xcb_xc_misc_get_xid_range_reply_t(reply:*mut xcb_xc_misc_get_xid_range_reply_t) -> GetXidRangeReply { GetXidRangeReply { base : base::mk_reply(reply) } }
pub struct  GetXidListCookie<'s> { pub base : base::Cookie<'s, xcb_xc_misc_get_xid_list_cookie_t> }

/** Opcode for xcb_xc_misc_get_xid_list. */
pub const XCB_XC_MISC_GET_XID_LIST : u8 = 2;
pub fn GetVersion<'r> (c : &'r Connection,
                   client_major_version : u16,
                   client_minor_version : u16) -> GetVersionCookie<'r> {
  unsafe {
    let cookie = xcb_xc_misc_get_version(c.get_raw_conn(),
        client_major_version as u16, //1
        client_minor_version as u16); //2
    GetVersionCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetVersionUnchecked<'r> (c : &'r Connection,
                            client_major_version : u16,
                            client_minor_version : u16) -> GetVersionCookie<'r> {
  unsafe {
    let cookie = xcb_xc_misc_get_version_unchecked(c.get_raw_conn(),
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
impl_reply_cookie!(GetVersionCookie<'s>, mk_reply_xcb_xc_misc_get_version_reply_t, GetVersionReply, xcb_xc_misc_get_version_reply);

pub fn GetXidRange<'r> (c : &'r Connection) -> GetXidRangeCookie<'r> {
  unsafe {
    let cookie = xcb_xc_misc_get_xid_range(c.get_raw_conn());
    GetXidRangeCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetXidRangeUnchecked<'r> (c : &'r Connection) -> GetXidRangeCookie<'r> {
  unsafe {
    let cookie = xcb_xc_misc_get_xid_range_unchecked(c.get_raw_conn());
    GetXidRangeCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetXidRangeReply {
  pub fn start_id(&mut self) -> u32 {
    unsafe { accessor!(start_id -> u32, (*self.base.reply)) }
  }

  pub fn count(&mut self) -> u32 {
    unsafe { accessor!(count -> u32, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetXidRangeCookie<'s>, mk_reply_xcb_xc_misc_get_xid_range_reply_t, GetXidRangeReply, xcb_xc_misc_get_xid_range_reply);

pub struct GetXidListReply { base:  base::Reply<xcb_xc_misc_get_xid_list_reply_t> }
fn mk_reply_xcb_xc_misc_get_xid_list_reply_t(reply:*mut xcb_xc_misc_get_xid_list_reply_t) -> GetXidListReply { GetXidListReply { base : base::mk_reply(reply) } }
pub fn GetXidList<'r> (c : &'r Connection,
                   count : u32) -> GetXidListCookie<'r> {
  unsafe {
    let cookie = xcb_xc_misc_get_xid_list(c.get_raw_conn(),
        count as u32); //1
    GetXidListCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetXidListUnchecked<'r> (c : &'r Connection,
                            count : u32) -> GetXidListCookie<'r> {
  unsafe {
    let cookie = xcb_xc_misc_get_xid_list_unchecked(c.get_raw_conn(),
        count as u32); //1
    GetXidListCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetXidListReply {
  pub fn ids(&mut self) -> Vec<u32> {
    unsafe { accessor!(u32, xcb_xc_misc_get_xid_list_ids_length, xcb_xc_misc_get_xid_list_ids, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetXidListCookie<'s>, mk_reply_xcb_xc_misc_get_xid_list_reply_t, GetXidListReply, xcb_xc_misc_get_xid_list_reply);


