//
// This file generated automatically from xevie.xml by r_client.py.
// Edit at your peril.
//

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
use ffi::xevie::*;
use std::option::Option;
use std::iter::Iterator;

pub struct  QueryVersionCookie<'s> { pub base : base::Cookie<'s, xcb_xevie_query_version_cookie_t> }

/// Opcode for xcb_xevie_query_version.
pub const XCB_XEVIE_QUERY_VERSION : u8 = 0;
pub struct QueryVersionReply { base:  base::Reply<xcb_xevie_query_version_reply_t> }
fn mk_reply_xcb_xevie_query_version_reply_t(reply:*mut xcb_xevie_query_version_reply_t) -> QueryVersionReply { QueryVersionReply { base : base::mk_reply(reply) } }
pub struct  StartCookie<'s> { pub base : base::Cookie<'s, xcb_xevie_start_cookie_t> }

/// Opcode for xcb_xevie_start.
pub const XCB_XEVIE_START : u8 = 1;
pub struct StartReply { base:  base::Reply<xcb_xevie_start_reply_t> }
fn mk_reply_xcb_xevie_start_reply_t(reply:*mut xcb_xevie_start_reply_t) -> StartReply { StartReply { base : base::mk_reply(reply) } }
pub struct  EndCookie<'s> { pub base : base::Cookie<'s, xcb_xevie_end_cookie_t> }

/// Opcode for xcb_xevie_end.
pub const XCB_XEVIE_END : u8 = 2;
pub struct EndReply { base:  base::Reply<xcb_xevie_end_reply_t> }
fn mk_reply_xcb_xevie_end_reply_t(reply:*mut xcb_xevie_end_reply_t) -> EndReply { EndReply { base : base::mk_reply(reply) } }

pub type xcb_xevie_datatype_t = c_uint;//{
    pub const XCB_XEVIE_DATATYPE_UNMODIFIED : xcb_xevie_datatype_t = 1;
    pub const XCB_XEVIE_DATATYPE_MODIFIED : xcb_xevie_datatype_t = 2;
//}
pub struct Event {pub base : base::Struct<xcb_xevie_event_t> }

pub type EventIterator = xcb_xevie_event_iterator_t;

pub struct  SendCookie<'s> { pub base : base::Cookie<'s, xcb_xevie_send_cookie_t> }

/// Opcode for xcb_xevie_send.
pub const XCB_XEVIE_SEND : u8 = 3;
pub struct SendReply { base:  base::Reply<xcb_xevie_send_reply_t> }
fn mk_reply_xcb_xevie_send_reply_t(reply:*mut xcb_xevie_send_reply_t) -> SendReply { SendReply { base : base::mk_reply(reply) } }
pub struct  SelectInputCookie<'s> { pub base : base::Cookie<'s, xcb_xevie_select_input_cookie_t> }

/// Opcode for xcb_xevie_select_input.
pub const XCB_XEVIE_SELECT_INPUT : u8 = 4;
pub struct SelectInputReply { base:  base::Reply<xcb_xevie_select_input_reply_t> }
fn mk_reply_xcb_xevie_select_input_reply_t(reply:*mut xcb_xevie_select_input_reply_t) -> SelectInputReply { SelectInputReply { base : base::mk_reply(reply) } }
pub fn QueryVersion<'r> (c : &'r Connection,
                     client_major_version : u16,
                     client_minor_version : u16) -> QueryVersionCookie<'r> {
  unsafe {
    let cookie = xcb_xevie_query_version(c.get_raw_conn(),
        client_major_version as u16, //1
        client_minor_version as u16); //2
    QueryVersionCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn QueryVersionUnchecked<'r> (c : &'r Connection,
                              client_major_version : u16,
                              client_minor_version : u16) -> QueryVersionCookie<'r> {
  unsafe {
    let cookie = xcb_xevie_query_version_unchecked(c.get_raw_conn(),
        client_major_version as u16, //1
        client_minor_version as u16); //2
    QueryVersionCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl QueryVersionReply {
  pub fn server_major_version(&mut self) -> u16 {
    unsafe { accessor!(server_major_version -> u16, (*self.base.reply)) }
  }

  pub fn server_minor_version(&mut self) -> u16 {
    unsafe { accessor!(server_minor_version -> u16, (*self.base.reply)) }
  }

}
impl_reply_cookie!(QueryVersionCookie<'s>, mk_reply_xcb_xevie_query_version_reply_t, QueryVersionReply, xcb_xevie_query_version_reply);

pub fn Start<'r> (c : &'r Connection,
              screen : u32) -> StartCookie<'r> {
  unsafe {
    let cookie = xcb_xevie_start(c.get_raw_conn(),
        screen as u32); //1
    StartCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn StartUnchecked<'r> (c : &'r Connection,
                       screen : u32) -> StartCookie<'r> {
  unsafe {
    let cookie = xcb_xevie_start_unchecked(c.get_raw_conn(),
        screen as u32); //1
    StartCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl StartReply {
}
impl_reply_cookie!(StartCookie<'s>, mk_reply_xcb_xevie_start_reply_t, StartReply, xcb_xevie_start_reply);

pub fn End<'r> (c : &'r Connection,
            cmap : u32) -> EndCookie<'r> {
  unsafe {
    let cookie = xcb_xevie_end(c.get_raw_conn(),
        cmap as u32); //1
    EndCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn EndUnchecked<'r> (c : &'r Connection,
                     cmap : u32) -> EndCookie<'r> {
  unsafe {
    let cookie = xcb_xevie_end_unchecked(c.get_raw_conn(),
        cmap as u32); //1
    EndCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl EndReply {
}
impl_reply_cookie!(EndCookie<'s>, mk_reply_xcb_xevie_end_reply_t, EndReply, xcb_xevie_end_reply);


impl Event {
}

impl Iterator for EventIterator {
    type Item = Event;
    fn next(&mut self) -> Option<Event> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_xevie_event_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_xevie_event_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub fn Send<'r> (c : &'r Connection,
             event : Event,
             data_type : u32) -> SendCookie<'r> {
  unsafe {
    let cookie = xcb_xevie_send(c.get_raw_conn(),
        event.base.strct, //1
        data_type as u32); //2
    SendCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn SendUnchecked<'r> (c : &'r Connection,
                      event : Event,
                      data_type : u32) -> SendCookie<'r> {
  unsafe {
    let cookie = xcb_xevie_send_unchecked(c.get_raw_conn(),
        event.base.strct, //1
        data_type as u32); //2
    SendCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl SendReply {
}
impl_reply_cookie!(SendCookie<'s>, mk_reply_xcb_xevie_send_reply_t, SendReply, xcb_xevie_send_reply);

pub fn SelectInput<'r> (c : &'r Connection,
                    event_mask : u32) -> SelectInputCookie<'r> {
  unsafe {
    let cookie = xcb_xevie_select_input(c.get_raw_conn(),
        event_mask as u32); //1
    SelectInputCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn SelectInputUnchecked<'r> (c : &'r Connection,
                             event_mask : u32) -> SelectInputCookie<'r> {
  unsafe {
    let cookie = xcb_xevie_select_input_unchecked(c.get_raw_conn(),
        event_mask as u32); //1
    SelectInputCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl SelectInputReply {
}
impl_reply_cookie!(SelectInputCookie<'s>, mk_reply_xcb_xevie_select_input_reply_t, SelectInputReply, xcb_xevie_select_input_reply);


