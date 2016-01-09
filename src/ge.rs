/*
 * This file generated automatically from ge.xml by r_client.py.
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
use ffi::ge::*;
use std::option::Option;
use std::iter::Iterator;

pub struct  QueryVersionCookie<'s> { pub base : base::Cookie<'s, xcb_genericevent_query_version_cookie_t> }

/** Opcode for xcb_genericevent_query_version. */
pub static XCB_GENERICEVENT_QUERY_VERSION : u8 = 0;
pub struct QueryVersionReply { base:  base::Reply<xcb_genericevent_query_version_reply_t> }
fn mk_reply_xcb_genericevent_query_version_reply_t(reply:*mut xcb_genericevent_query_version_reply_t) -> QueryVersionReply { QueryVersionReply { base : base::mk_reply(reply) } }
pub fn QueryVersion<'r> (c : &'r Connection,
                     client_major_version : u16,
                     client_minor_version : u16) -> QueryVersionCookie<'r> {
  unsafe {
    let cookie = xcb_genericevent_query_version(c.get_raw_conn(),
        client_major_version as u16, //1
        client_minor_version as u16); //2
    QueryVersionCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn QueryVersionUnchecked<'r> (c : &'r Connection,
                              client_major_version : u16,
                              client_minor_version : u16) -> QueryVersionCookie<'r> {
  unsafe {
    let cookie = xcb_genericevent_query_version_unchecked(c.get_raw_conn(),
        client_major_version as u16, //1
        client_minor_version as u16); //2
    QueryVersionCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl QueryVersionReply {
  pub fn major_version(&mut self) -> u16 {
    unsafe { accessor!(major_version -> u16, (*self.base.reply)) }
  }

  pub fn minor_version(&mut self) -> u16 {
    unsafe { accessor!(minor_version -> u16, (*self.base.reply)) }
  }

}
impl_reply_cookie!(QueryVersionCookie<'s>, mk_reply_xcb_genericevent_query_version_reply_t, QueryVersionReply, xcb_genericevent_query_version_reply);


