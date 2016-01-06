/*
 * This file generated automatically from bigreq.xml by r_client.py.
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
use ffi::bigreq::*;
use std::option::Option;
use std::iter::Iterator;

pub struct  EnableCookie<'s> { pub base : base::Cookie<'s, enable_cookie> }

/** Opcode for xcb_big_requests_enable. */
pub static XCB_BIG_REQUESTS_ENABLE : u8 = 0;
pub struct EnableReply { base:  base::Reply<enable_reply> }
fn mk_reply_enable_reply(reply:*mut enable_reply) -> EnableReply { EnableReply { base : base::mk_reply(reply) } }
pub fn Enable<'r> (c : &'r Connection) -> EnableCookie<'r> {
  unsafe {
    let cookie = xcb_big_requests_enable(c.get_raw_conn());
    EnableCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn EnableUnchecked<'r> (c : &'r Connection) -> EnableCookie<'r> {
  unsafe {
    let cookie = xcb_big_requests_enable_unchecked(c.get_raw_conn());
    EnableCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl EnableReply {
  pub fn maximum_request_length(&mut self) -> u32 {
    unsafe { accessor!(maximum_request_length -> u32, (*self.base.reply)) }
  }

}
impl_reply_cookie!(EnableCookie<'s>, mk_reply_enable_reply, EnableReply, xcb_big_requests_enable_reply);


