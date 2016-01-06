/*
 * This file generated automatically from record.xml by r_client.py.
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
use ffi::record::*;
use std::option::Option;
use std::iter::Iterator;

pub type Context = context;

pub type ContextIterator = context_iterator;

pub type Range8Iterator = range_8_iterator;

pub type Range16Iterator = range_16_iterator;

pub type ExtRangeIterator = ext_range_iterator;

pub type RangeIterator = range_iterator;

pub type ElementHeaderIterator = element_header_iterator;


pub type h_type = c_uint;//{
    pub static XCB_RECORD_H_TYPE_FROM_SERVER_TIME : h_type = 1;
    pub static XCB_RECORD_H_TYPE_FROM_CLIENT_TIME : h_type = 2;
    pub static XCB_RECORD_H_TYPE_FROM_CLIENT_SEQUENCE : h_type = 4;
//}
pub type ClientSpec = client_spec;

pub type ClientSpecIterator = client_spec_iterator;


pub type cs = c_uint;//{
    pub static XCB_RECORD_CS_CURRENT_CLIENTS : cs = 1;
    pub static XCB_RECORD_CS_FUTURE_CLIENTS : cs = 2;
    pub static XCB_RECORD_CS_ALL_CLIENTS : cs = 3;
//}
pub type ClientInfoIterator = client_info_iterator;

/** Opcode for xcb_record_bad_context. */
pub static XCB_RECORD_BAD_CONTEXT : u8 = 0;
pub struct BadContextError { pub base : base::Error<bad_context_error> }
pub struct  QueryVersionCookie<'s> { pub base : base::Cookie<'s, query_version_cookie> }

/** Opcode for xcb_record_query_version. */
pub static XCB_RECORD_QUERY_VERSION : u8 = 0;
pub struct QueryVersionReply { base:  base::Reply<query_version_reply> }
fn mk_reply_query_version_reply(reply:*mut query_version_reply) -> QueryVersionReply { QueryVersionReply { base : base::mk_reply(reply) } }
/** Opcode for xcb_record_create_context. */
pub static XCB_RECORD_CREATE_CONTEXT : u8 = 1;
/** Opcode for xcb_record_register_clients. */
pub static XCB_RECORD_REGISTER_CLIENTS : u8 = 2;
/** Opcode for xcb_record_unregister_clients. */
pub static XCB_RECORD_UNREGISTER_CLIENTS : u8 = 3;
pub struct  GetContextCookie<'s> { pub base : base::Cookie<'s, get_context_cookie> }

/** Opcode for xcb_record_get_context. */
pub static XCB_RECORD_GET_CONTEXT : u8 = 4;
pub struct  EnableContextCookie<'s> { pub base : base::Cookie<'s, enable_context_cookie> }

/** Opcode for xcb_record_enable_context. */
pub static XCB_RECORD_ENABLE_CONTEXT : u8 = 5;
/** Opcode for xcb_record_disable_context. */
pub static XCB_RECORD_DISABLE_CONTEXT : u8 = 6;
/** Opcode for xcb_record_free_context. */
pub static XCB_RECORD_FREE_CONTEXT : u8 = 7;

impl Iterator for ContextIterator {
    type Item = Context;
    fn next(&mut self) -> Option<Context> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut context_iterator = mem::transmute(self);
            let data = (*iter).data;
            xcb_record_context_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct Range8 {pub base : base::Struct<range_8> }


impl Range8 {
  pub fn first(&mut self) -> u8 {
    unsafe { accessor!(first -> u8, self.base.strct) }
  }

  pub fn last(&mut self) -> u8 {
    unsafe { accessor!(last -> u8, self.base.strct) }
  }

}

impl Iterator for Range8Iterator {
    type Item = Range8;
    fn next(&mut self) -> Option<Range8> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut range_8_iterator = mem::transmute(self);
            let data = (*iter).data;
            xcb_record_range_8_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct Range16 {pub base : base::Struct<range_16> }


impl Range16 {
  pub fn first(&mut self) -> u16 {
    unsafe { accessor!(first -> u16, self.base.strct) }
  }

  pub fn last(&mut self) -> u16 {
    unsafe { accessor!(last -> u16, self.base.strct) }
  }

}

impl Iterator for Range16Iterator {
    type Item = Range16;
    fn next(&mut self) -> Option<Range16> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut range_16_iterator = mem::transmute(self);
            let data = (*iter).data;
            xcb_record_range_16_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct ExtRange {pub base : base::Struct<ext_range> }


impl ExtRange {
  pub fn major(&self) -> Range8 {
    unsafe { mem::transmute(self.base.strct.major) }
  }
  pub fn minor(&self) -> Range16 {
    unsafe { mem::transmute(self.base.strct.minor) }
  }
}

impl Iterator for ExtRangeIterator {
    type Item = ExtRange;
    fn next(&mut self) -> Option<ExtRange> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut ext_range_iterator = mem::transmute(self);
            let data = (*iter).data;
            xcb_record_ext_range_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct Range {pub base : base::Struct<range> }


impl Range {
  pub fn core_requests(&self) -> Range8 {
    unsafe { mem::transmute(self.base.strct.core_requests) }
  }
  pub fn core_replies(&self) -> Range8 {
    unsafe { mem::transmute(self.base.strct.core_replies) }
  }
  pub fn ext_requests(&self) -> ExtRange {
    unsafe { mem::transmute(self.base.strct.ext_requests) }
  }
  pub fn ext_replies(&self) -> ExtRange {
    unsafe { mem::transmute(self.base.strct.ext_replies) }
  }
  pub fn delivered_events(&self) -> Range8 {
    unsafe { mem::transmute(self.base.strct.delivered_events) }
  }
  pub fn device_events(&self) -> Range8 {
    unsafe { mem::transmute(self.base.strct.device_events) }
  }
  pub fn errors(&self) -> Range8 {
    unsafe { mem::transmute(self.base.strct.errors) }
  }
  pub fn client_started(&mut self) -> u8 {
    unsafe { accessor!(client_started -> u8, self.base.strct) }
  }

  pub fn client_died(&mut self) -> u8 {
    unsafe { accessor!(client_died -> u8, self.base.strct) }
  }

}

impl Iterator for RangeIterator {
    type Item = Range;
    fn next(&mut self) -> Option<Range> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut range_iterator = mem::transmute(self);
            let data = (*iter).data;
            xcb_record_range_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub type ElementHeader = element_header;


impl Iterator for ElementHeaderIterator {
    type Item = ElementHeader;
    fn next(&mut self) -> Option<ElementHeader> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut element_header_iterator = mem::transmute(self);
            let data = (*iter).data;
            xcb_record_element_header_next(iter);
            Some(mem::transmute(*data))
        }
    }
}


impl Iterator for ClientSpecIterator {
    type Item = ClientSpec;
    fn next(&mut self) -> Option<ClientSpec> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut client_spec_iterator = mem::transmute(self);
            let data = (*iter).data;
            xcb_record_client_spec_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct ClientInfo {pub base : base::Struct<client_info> }


impl ClientInfo {
  pub fn client_resource(&mut self) -> ClientSpec {
    unsafe { accessor!(client_resource -> ClientSpec, self.base.strct) }
  }

  pub fn ranges(&mut self) -> RangeIterator {
    unsafe { accessor!(RangeIterator, xcb_record_client_info_ranges_iterator, self.base.strct) }
  }

}

impl Iterator for ClientInfoIterator {
    type Item = ClientInfo;
    fn next(&mut self) -> Option<ClientInfo> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut client_info_iterator = mem::transmute(self);
            let data = (*iter).data;
            xcb_record_client_info_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub fn QueryVersion<'r> (c : &'r Connection,
                     major_version : u16,
                     minor_version : u16) -> QueryVersionCookie<'r> {
  unsafe {
    let cookie = xcb_record_query_version(c.get_raw_conn(),
        major_version as u16, //1
        minor_version as u16); //2
    QueryVersionCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn QueryVersionUnchecked<'r> (c : &'r Connection,
                              major_version : u16,
                              minor_version : u16) -> QueryVersionCookie<'r> {
  unsafe {
    let cookie = xcb_record_query_version_unchecked(c.get_raw_conn(),
        major_version as u16, //1
        minor_version as u16); //2
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
impl_reply_cookie!(QueryVersionCookie<'s>, mk_reply_query_version_reply, QueryVersionReply, xcb_record_query_version_reply);

pub fn CreateContextChecked<'r> (c : &'r Connection,
                             context : Context,
                             element_header : ElementHeader,
                             client_specs : &[ClientSpec],
                             ranges : &[Range]) -> base::VoidCookie<'r> {
  unsafe {
    let client_specs_len = client_specs.len();
    let client_specs_ptr = client_specs.as_ptr();
    let ranges_len = ranges.len();
    let ranges_ptr = ranges.as_ptr();
    let cookie = xcb_record_create_context_checked(c.get_raw_conn(),
        context as context, //1
        element_header as element_header, //2
        client_specs_len as u32, //3
        ranges_len as u32, //4
        client_specs_ptr as *mut client_spec, //5
        ranges_ptr as *mut range); //6
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn CreateContext<'r> (c : &'r Connection,
                      context : Context,
                      element_header : ElementHeader,
                      client_specs : &[ClientSpec],
                      ranges : &[Range]) -> base::VoidCookie<'r> {
  unsafe {
    let client_specs_len = client_specs.len();
    let client_specs_ptr = client_specs.as_ptr();
    let ranges_len = ranges.len();
    let ranges_ptr = ranges.as_ptr();
    let cookie = xcb_record_create_context(c.get_raw_conn(),
        context as context, //1
        element_header as element_header, //2
        client_specs_len as u32, //3
        ranges_len as u32, //4
        client_specs_ptr as *mut client_spec, //5
        ranges_ptr as *mut range); //6
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn RegisterClientsChecked<'r> (c : &'r Connection,
                               context : Context,
                               element_header : ElementHeader,
                               client_specs : &[ClientSpec],
                               ranges : &[Range]) -> base::VoidCookie<'r> {
  unsafe {
    let client_specs_len = client_specs.len();
    let client_specs_ptr = client_specs.as_ptr();
    let ranges_len = ranges.len();
    let ranges_ptr = ranges.as_ptr();
    let cookie = xcb_record_register_clients_checked(c.get_raw_conn(),
        context as context, //1
        element_header as element_header, //2
        client_specs_len as u32, //3
        ranges_len as u32, //4
        client_specs_ptr as *mut client_spec, //5
        ranges_ptr as *mut range); //6
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn RegisterClients<'r> (c : &'r Connection,
                        context : Context,
                        element_header : ElementHeader,
                        client_specs : &[ClientSpec],
                        ranges : &[Range]) -> base::VoidCookie<'r> {
  unsafe {
    let client_specs_len = client_specs.len();
    let client_specs_ptr = client_specs.as_ptr();
    let ranges_len = ranges.len();
    let ranges_ptr = ranges.as_ptr();
    let cookie = xcb_record_register_clients(c.get_raw_conn(),
        context as context, //1
        element_header as element_header, //2
        client_specs_len as u32, //3
        ranges_len as u32, //4
        client_specs_ptr as *mut client_spec, //5
        ranges_ptr as *mut range); //6
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn UnregisterClientsChecked<'r> (c : &'r Connection,
                                 context : Context,
                                 client_specs : &[ClientSpec]) -> base::VoidCookie<'r> {
  unsafe {
    let client_specs_len = client_specs.len();
    let client_specs_ptr = client_specs.as_ptr();
    let cookie = xcb_record_unregister_clients_checked(c.get_raw_conn(),
        context as context, //1
        client_specs_len as u32, //2
        client_specs_ptr as *mut client_spec); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn UnregisterClients<'r> (c : &'r Connection,
                          context : Context,
                          client_specs : &[ClientSpec]) -> base::VoidCookie<'r> {
  unsafe {
    let client_specs_len = client_specs.len();
    let client_specs_ptr = client_specs.as_ptr();
    let cookie = xcb_record_unregister_clients(c.get_raw_conn(),
        context as context, //1
        client_specs_len as u32, //2
        client_specs_ptr as *mut client_spec); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub struct GetContextReply { base:  base::Reply<get_context_reply> }
fn mk_reply_get_context_reply(reply:*mut get_context_reply) -> GetContextReply { GetContextReply { base : base::mk_reply(reply) } }
pub fn GetContext<'r> (c : &'r Connection,
                   context : Context) -> GetContextCookie<'r> {
  unsafe {
    let cookie = xcb_record_get_context(c.get_raw_conn(),
        context as context); //1
    GetContextCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetContextUnchecked<'r> (c : &'r Connection,
                            context : Context) -> GetContextCookie<'r> {
  unsafe {
    let cookie = xcb_record_get_context_unchecked(c.get_raw_conn(),
        context as context); //1
    GetContextCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetContextReply {
  pub fn enabled(&mut self) -> u8 {
    unsafe { accessor!(enabled -> u8, (*self.base.reply)) }
  }

  pub fn element_header(&mut self) -> ElementHeader {
    unsafe { accessor!(element_header -> ElementHeader, (*self.base.reply)) }
  }

  pub fn intercepted_clients(&mut self) -> ClientInfoIterator {
    unsafe { accessor!(ClientInfoIterator, xcb_record_get_context_intercepted_clients_iterator, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetContextCookie<'s>, mk_reply_get_context_reply, GetContextReply, xcb_record_get_context_reply);

pub struct EnableContextReply { base:  base::Reply<enable_context_reply> }
fn mk_reply_enable_context_reply(reply:*mut enable_context_reply) -> EnableContextReply { EnableContextReply { base : base::mk_reply(reply) } }
pub fn EnableContext<'r> (c : &'r Connection,
                      context : Context) -> EnableContextCookie<'r> {
  unsafe {
    let cookie = xcb_record_enable_context(c.get_raw_conn(),
        context as context); //1
    EnableContextCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn EnableContextUnchecked<'r> (c : &'r Connection,
                               context : Context) -> EnableContextCookie<'r> {
  unsafe {
    let cookie = xcb_record_enable_context_unchecked(c.get_raw_conn(),
        context as context); //1
    EnableContextCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl EnableContextReply {
  pub fn category(&mut self) -> u8 {
    unsafe { accessor!(category -> u8, (*self.base.reply)) }
  }

  pub fn element_header(&mut self) -> ElementHeader {
    unsafe { accessor!(element_header -> ElementHeader, (*self.base.reply)) }
  }

  pub fn client_swapped(&mut self) -> u8 {
    unsafe { accessor!(client_swapped -> u8, (*self.base.reply)) }
  }

  pub fn xid_base(&mut self) -> u32 {
    unsafe { accessor!(xid_base -> u32, (*self.base.reply)) }
  }

  pub fn server_time(&mut self) -> u32 {
    unsafe { accessor!(server_time -> u32, (*self.base.reply)) }
  }

  pub fn rec_sequence_num(&mut self) -> u32 {
    unsafe { accessor!(rec_sequence_num -> u32, (*self.base.reply)) }
  }

  pub fn data(&mut self) -> Vec<u8> {
    unsafe { accessor!(u8, xcb_record_enable_context_data_length, xcb_record_enable_context_data, (*self.base.reply)) }
  }

}
impl_reply_cookie!(EnableContextCookie<'s>, mk_reply_enable_context_reply, EnableContextReply, xcb_record_enable_context_reply);

pub fn DisableContextChecked<'r> (c : &'r Connection,
                              context : Context) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_record_disable_context_checked(c.get_raw_conn(),
        context as context); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn DisableContext<'r> (c : &'r Connection,
                       context : Context) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_record_disable_context(c.get_raw_conn(),
        context as context); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn FreeContextChecked<'r> (c : &'r Connection,
                           context : Context) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_record_free_context_checked(c.get_raw_conn(),
        context as context); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn FreeContext<'r> (c : &'r Connection,
                    context : Context) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_record_free_context(c.get_raw_conn(),
        context as context); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

