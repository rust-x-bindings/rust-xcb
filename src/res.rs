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
pub struct Client {pub base : base::Struct<client> }

pub type ClientIterator = client_iterator;

pub type TypeIterator = type_iterator;

pub struct  QueryVersionCookie<'s> { pub base : base::Cookie<'s, query_version_cookie> }

/** Opcode for xcb_res_query_version. */
pub static XCB_RES_QUERY_VERSION : u8 = 0;
pub struct QueryVersionReply { base:  base::Reply<query_version_reply> }
fn mk_reply_query_version_reply(reply:*mut query_version_reply) -> QueryVersionReply { QueryVersionReply { base : base::mk_reply(reply) } }
pub struct  QueryClientsCookie<'s> { pub base : base::Cookie<'s, query_clients_cookie> }

/** Opcode for xcb_res_query_clients. */
pub static XCB_RES_QUERY_CLIENTS : u8 = 1;
pub struct  QueryClientResourcesCookie<'s> { pub base : base::Cookie<'s, query_client_resources_cookie> }

/** Opcode for xcb_res_query_client_resources. */
pub static XCB_RES_QUERY_CLIENT_RESOURCES : u8 = 2;
pub struct  QueryClientPixmapBytesCookie<'s> { pub base : base::Cookie<'s, query_client_pixmap_bytes_cookie> }

/** Opcode for xcb_res_query_client_pixmap_bytes. */
pub static XCB_RES_QUERY_CLIENT_PIXMAP_BYTES : u8 = 3;
pub struct QueryClientPixmapBytesReply { base:  base::Reply<query_client_pixmap_bytes_reply> }
fn mk_reply_query_client_pixmap_bytes_reply(reply:*mut query_client_pixmap_bytes_reply) -> QueryClientPixmapBytesReply { QueryClientPixmapBytesReply { base : base::mk_reply(reply) } }

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
            let iter: *mut client_iterator = mem::transmute(self);
            let data = (*iter).data;
            xcb_res_client_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct Type {pub base : base::Struct<type_> }


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
            let iter: *mut type_iterator = mem::transmute(self);
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
impl_reply_cookie!(QueryVersionCookie<'s>, mk_reply_query_version_reply, QueryVersionReply, xcb_res_query_version_reply);

pub struct QueryClientsReply { base:  base::Reply<query_clients_reply> }
fn mk_reply_query_clients_reply(reply:*mut query_clients_reply) -> QueryClientsReply { QueryClientsReply { base : base::mk_reply(reply) } }
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
impl_reply_cookie!(QueryClientsCookie<'s>, mk_reply_query_clients_reply, QueryClientsReply, xcb_res_query_clients_reply);

pub struct QueryClientResourcesReply { base:  base::Reply<query_client_resources_reply> }
fn mk_reply_query_client_resources_reply(reply:*mut query_client_resources_reply) -> QueryClientResourcesReply { QueryClientResourcesReply { base : base::mk_reply(reply) } }
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
impl_reply_cookie!(QueryClientResourcesCookie<'s>, mk_reply_query_client_resources_reply, QueryClientResourcesReply, xcb_res_query_client_resources_reply);

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
impl_reply_cookie!(QueryClientPixmapBytesCookie<'s>, mk_reply_query_client_pixmap_bytes_reply, QueryClientPixmapBytesReply, xcb_res_query_client_pixmap_bytes_reply);


