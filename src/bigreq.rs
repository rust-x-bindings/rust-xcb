/*
 * This file generated automatically from bigreq.xml by r_client.py.
 * Edit at your peril.
 */

//Make the compiler quiet
#[allow(unused_imports)];
#[allow(unused_unsafe)];
use std;
use std::libc::*;
use std::{cast,num,ptr,str,libc};
use std::to_bytes::ToBytes;
use ffi::base::*;
use base;
use base::*;
use ffi;
use ffi::bigreq::*;
use std::option::Option;
use std::iterator::Iterator;

pub type EnableCookie<'self> = base::Cookie<'self, enable_cookie>;

/** Opcode for xcb_big_requests_enable. */
pub static XCB_BIG_REQUESTS_ENABLE : u8 = 0;
pub type EnableReply = base::Reply<enable_reply>;
pub fn Enable<'r> (c : &'r Connection) -> EnableCookie<'r> {
  unsafe {
    let cookie = xcb_big_requests_enable(c.get_raw_conn());
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}
pub fn EnableUnchecked<'r> (c : &'r Connection) -> EnableCookie<'r> {
  unsafe {
    let cookie = xcb_big_requests_enable_unchecked(c.get_raw_conn());
    Cookie {cookie:cookie,conn:c,checked:false}
  }
}

impl base::Reply<enable_reply> {
  pub fn maximum_request_length(&self) -> u32 {
    unsafe { accessor!(maximum_request_length -> u32, (*self.reply)) }
  }

}
impl_reply_cookie!(EnableCookie<'self>, enable_reply, EnableReply, xcb_big_requests_enable_reply)


