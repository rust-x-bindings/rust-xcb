/*
 * This file generated automatically from res.xml by r_client.py.
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
use ffi::res::*;
use std::option::Option;
use std::iter::Iterator;

use xproto;
pub struct Client {pub base : base::Struct<xcb_res_client_t> }

pub type ClientIterator = xcb_res_client_iterator_t;

pub type TypeIterator = xcb_res_type_iterator_t;

pub struct  QueryVersionCookie<'s> { pub base : base::Cookie<'s, xcb_res_query_version_cookie_t> }

/** Opcode for xcb_res_query_version. */
pub static XCB_RES_QUERY_VERSION : u8 = 0;
pub struct QueryVersionReply { base:  base::Reply<xcb_res_query_version_reply_t> }
fn mk_reply_xcb_res_query_version_reply_t(reply:*mut xcb_res_query_version_reply_t) -> QueryVersionReply { QueryVersionReply { base : base::mk_reply(reply) } }
pub struct  QueryClientsCookie<'s> { pub base : base::Cookie<'s, xcb_res_query_clients_cookie_t> }

/** Opcode for xcb_res_query_clients. */
pub static XCB_RES_QUERY_CLIENTS : u8 = 1;
pub struct  QueryClientResourcesCookie<'s> { pub base : base::Cookie<'s, xcb_res_query_client_resources_cookie_t> }

/** Opcode for xcb_res_query_client_resources. */
pub static XCB_RES_QUERY_CLIENT_RESOURCES : u8 = 2;
pub struct  QueryClientPixmapBytesCookie<'s> { pub base : base::Cookie<'s, xcb_res_query_client_pixmap_bytes_cookie_t> }

/** Opcode for xcb_res_query_client_pixmap_bytes. */
pub static XCB_RES_QUERY_CLIENT_PIXMAP_BYTES : u8 = 3;
pub struct QueryClientPixmapBytesReply { base:  base::Reply<xcb_res_query_client_pixmap_bytes_reply_t> }
fn mk_reply_xcb_res_query_client_pixmap_bytes_reply_t(reply:*mut xcb_res_query_client_pixmap_bytes_reply_t) -> QueryClientPixmapBytesReply { QueryClientPixmapBytesReply { base : base::mk_reply(reply) } }

impl Client {
  pub fn resource_base(&mut self) -> u32 {
    unsafe { accessor!(resource_base -> u32, self.base.strct) }
  }

  pub fn resource_mask(&mut self) -> u32 {
    unsafe { accessor!(resource_mask -> u32, self.base.strct) }
  }

}

impl Iterator for ClientIterator {
    type Item = Client;
    fn next(&mut self) -> Option<Client> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_res_client_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_res_client_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct Type {pub base : base::Struct<xcb_res_type_t> }


impl Type {
  pub fn resource_type(&mut self) -> xproto::Atom {
    unsafe { accessor!(resource_type -> xproto::Atom, self.base.strct) }
  }

  pub fn count(&mut self) -> u32 {
    unsafe { accessor!(count -> u32, self.base.strct) }
  }

}

impl Iterator for TypeIterator {
    type Item = Type;
    fn next(&mut self) -> Option<Type> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_res_type_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_res_type_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub fn QueryVersion<'r> (c : &'r Connection,
                     client_major : u8,
                     client_minor : u8) -> QueryVersionCookie<'r> {
  unsafe {
    let cookie = xcb_res_query_version(c.get_raw_conn(),
        client_major as u8, //1
        client_minor as u8); //2
    QueryVersionCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn QueryVersionUnchecked<'r> (c : &'r Connection,
                              client_major : u8,
                              client_minor : u8) -> QueryVersionCookie<'r> {
  unsafe {
    let cookie = xcb_res_query_version_unchecked(c.get_raw_conn(),
        client_major as u8, //1
        client_minor as u8); //2
    QueryVersionCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl QueryVersionReply {
  pub fn server_major(&mut self) -> u16 {
    unsafe { accessor!(server_major -> u16, (*self.base.reply)) }
  }

  pub fn server_minor(&mut self) -> u16 {
    unsafe { accessor!(server_minor -> u16, (*self.base.reply)) }
  }

}
impl_reply_cookie!(QueryVersionCookie<'s>, mk_reply_xcb_res_query_version_reply_t, QueryVersionReply, xcb_res_query_version_reply);

pub struct QueryClientsReply { base:  base::Reply<xcb_res_query_clients_reply_t> }
fn mk_reply_xcb_res_query_clients_reply_t(reply:*mut xcb_res_query_clients_reply_t) -> QueryClientsReply { QueryClientsReply { base : base::mk_reply(reply) } }
pub fn QueryClients<'r> (c : &'r Connection) -> QueryClientsCookie<'r> {
  unsafe {
    let cookie = xcb_res_query_clients(c.get_raw_conn());
    QueryClientsCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn QueryClientsUnchecked<'r> (c : &'r Connection) -> QueryClientsCookie<'r> {
  unsafe {
    let cookie = xcb_res_query_clients_unchecked(c.get_raw_conn());
    QueryClientsCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl QueryClientsReply {
  pub fn clients(&mut self) -> ClientIterator {
    unsafe { accessor!(ClientIterator, xcb_res_query_clients_clients_iterator, (*self.base.reply)) }
  }

}
impl_reply_cookie!(QueryClientsCookie<'s>, mk_reply_xcb_res_query_clients_reply_t, QueryClientsReply, xcb_res_query_clients_reply);

pub struct QueryClientResourcesReply { base:  base::Reply<xcb_res_query_client_resources_reply_t> }
fn mk_reply_xcb_res_query_client_resources_reply_t(reply:*mut xcb_res_query_client_resources_reply_t) -> QueryClientResourcesReply { QueryClientResourcesReply { base : base::mk_reply(reply) } }
pub fn QueryClientResources<'r> (c : &'r Connection,
                             xid : u32) -> QueryClientResourcesCookie<'r> {
  unsafe {
    let cookie = xcb_res_query_client_resources(c.get_raw_conn(),
        xid as u32); //1
    QueryClientResourcesCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn QueryClientResourcesUnchecked<'r> (c : &'r Connection,
                                      xid : u32) -> QueryClientResourcesCookie<'r> {
  unsafe {
    let cookie = xcb_res_query_client_resources_unchecked(c.get_raw_conn(),
        xid as u32); //1
    QueryClientResourcesCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl QueryClientResourcesReply {
  pub fn types(&mut self) -> TypeIterator {
    unsafe { accessor!(TypeIterator, xcb_res_query_client_resources_types_iterator, (*self.base.reply)) }
  }

}
impl_reply_cookie!(QueryClientResourcesCookie<'s>, mk_reply_xcb_res_query_client_resources_reply_t, QueryClientResourcesReply, xcb_res_query_client_resources_reply);

pub fn QueryClientPixmapBytes<'r> (c : &'r Connection,
                               xid : u32) -> QueryClientPixmapBytesCookie<'r> {
  unsafe {
    let cookie = xcb_res_query_client_pixmap_bytes(c.get_raw_conn(),
        xid as u32); //1
    QueryClientPixmapBytesCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn QueryClientPixmapBytesUnchecked<'r> (c : &'r Connection,
                                        xid : u32) -> QueryClientPixmapBytesCookie<'r> {
  unsafe {
    let cookie = xcb_res_query_client_pixmap_bytes_unchecked(c.get_raw_conn(),
        xid as u32); //1
    QueryClientPixmapBytesCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl QueryClientPixmapBytesReply {
  pub fn bytes(&mut self) -> u32 {
    unsafe { accessor!(bytes -> u32, (*self.base.reply)) }
  }

  pub fn bytes_overflow(&mut self) -> u32 {
    unsafe { accessor!(bytes_overflow -> u32, (*self.base.reply)) }
  }

}
impl_reply_cookie!(QueryClientPixmapBytesCookie<'s>, mk_reply_xcb_res_query_client_pixmap_bytes_reply_t, QueryClientPixmapBytesReply, xcb_res_query_client_pixmap_bytes_reply);


