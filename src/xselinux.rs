/*
 * This file generated automatically from xselinux.xml by r_client.py.
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
use ffi::xselinux::*;
use std::option::Option;
use std::iter::Iterator;

use xproto;
pub struct  QueryVersionCookie<'s> { pub base : base::Cookie<'s, query_version_cookie> }

/** Opcode for xcb_selinux_query_version. */
pub static XCB_SELINUX_QUERY_VERSION : u8 = 0;
pub struct QueryVersionReply { base:  base::Reply<query_version_reply> }
fn mk_reply_query_version_reply(reply:*mut query_version_reply) -> QueryVersionReply { QueryVersionReply { base : base::mk_reply(reply) } }
/** Opcode for xcb_selinux_set_device_create_context. */
pub static XCB_SELINUX_SET_DEVICE_CREATE_CONTEXT : u8 = 1;
pub struct  GetDeviceCreateContextCookie<'s> { pub base : base::Cookie<'s, get_device_create_context_cookie> }

/** Opcode for xcb_selinux_get_device_create_context. */
pub static XCB_SELINUX_GET_DEVICE_CREATE_CONTEXT : u8 = 2;
/** Opcode for xcb_selinux_set_device_context. */
pub static XCB_SELINUX_SET_DEVICE_CONTEXT : u8 = 3;
pub struct  GetDeviceContextCookie<'s> { pub base : base::Cookie<'s, get_device_context_cookie> }

/** Opcode for xcb_selinux_get_device_context. */
pub static XCB_SELINUX_GET_DEVICE_CONTEXT : u8 = 4;
/** Opcode for xcb_selinux_set_window_create_context. */
pub static XCB_SELINUX_SET_WINDOW_CREATE_CONTEXT : u8 = 5;
pub struct  GetWindowCreateContextCookie<'s> { pub base : base::Cookie<'s, get_window_create_context_cookie> }

/** Opcode for xcb_selinux_get_window_create_context. */
pub static XCB_SELINUX_GET_WINDOW_CREATE_CONTEXT : u8 = 6;
pub struct  GetWindowContextCookie<'s> { pub base : base::Cookie<'s, get_window_context_cookie> }

/** Opcode for xcb_selinux_get_window_context. */
pub static XCB_SELINUX_GET_WINDOW_CONTEXT : u8 = 7;
pub type ListItemIterator = list_item_iterator;

/** Opcode for xcb_selinux_set_property_create_context. */
pub static XCB_SELINUX_SET_PROPERTY_CREATE_CONTEXT : u8 = 8;
pub struct  GetPropertyCreateContextCookie<'s> { pub base : base::Cookie<'s, get_property_create_context_cookie> }

/** Opcode for xcb_selinux_get_property_create_context. */
pub static XCB_SELINUX_GET_PROPERTY_CREATE_CONTEXT : u8 = 9;
/** Opcode for xcb_selinux_set_property_use_context. */
pub static XCB_SELINUX_SET_PROPERTY_USE_CONTEXT : u8 = 10;
pub struct  GetPropertyUseContextCookie<'s> { pub base : base::Cookie<'s, get_property_use_context_cookie> }

/** Opcode for xcb_selinux_get_property_use_context. */
pub static XCB_SELINUX_GET_PROPERTY_USE_CONTEXT : u8 = 11;
pub struct  GetPropertyContextCookie<'s> { pub base : base::Cookie<'s, get_property_context_cookie> }

/** Opcode for xcb_selinux_get_property_context. */
pub static XCB_SELINUX_GET_PROPERTY_CONTEXT : u8 = 12;
pub struct  GetPropertyDataContextCookie<'s> { pub base : base::Cookie<'s, get_property_data_context_cookie> }

/** Opcode for xcb_selinux_get_property_data_context. */
pub static XCB_SELINUX_GET_PROPERTY_DATA_CONTEXT : u8 = 13;
pub struct  ListPropertiesCookie<'s> { pub base : base::Cookie<'s, list_properties_cookie> }

/** Opcode for xcb_selinux_list_properties. */
pub static XCB_SELINUX_LIST_PROPERTIES : u8 = 14;
/** Opcode for xcb_selinux_set_selection_create_context. */
pub static XCB_SELINUX_SET_SELECTION_CREATE_CONTEXT : u8 = 15;
pub struct  GetSelectionCreateContextCookie<'s> { pub base : base::Cookie<'s, get_selection_create_context_cookie> }

/** Opcode for xcb_selinux_get_selection_create_context. */
pub static XCB_SELINUX_GET_SELECTION_CREATE_CONTEXT : u8 = 16;
/** Opcode for xcb_selinux_set_selection_use_context. */
pub static XCB_SELINUX_SET_SELECTION_USE_CONTEXT : u8 = 17;
pub struct  GetSelectionUseContextCookie<'s> { pub base : base::Cookie<'s, get_selection_use_context_cookie> }

/** Opcode for xcb_selinux_get_selection_use_context. */
pub static XCB_SELINUX_GET_SELECTION_USE_CONTEXT : u8 = 18;
pub struct  GetSelectionContextCookie<'s> { pub base : base::Cookie<'s, get_selection_context_cookie> }

/** Opcode for xcb_selinux_get_selection_context. */
pub static XCB_SELINUX_GET_SELECTION_CONTEXT : u8 = 19;
pub struct  GetSelectionDataContextCookie<'s> { pub base : base::Cookie<'s, get_selection_data_context_cookie> }

/** Opcode for xcb_selinux_get_selection_data_context. */
pub static XCB_SELINUX_GET_SELECTION_DATA_CONTEXT : u8 = 20;
pub struct  ListSelectionsCookie<'s> { pub base : base::Cookie<'s, list_selections_cookie> }

/** Opcode for xcb_selinux_list_selections. */
pub static XCB_SELINUX_LIST_SELECTIONS : u8 = 21;
pub struct  GetClientContextCookie<'s> { pub base : base::Cookie<'s, get_client_context_cookie> }

/** Opcode for xcb_selinux_get_client_context. */
pub static XCB_SELINUX_GET_CLIENT_CONTEXT : u8 = 22;
pub fn QueryVersion<'r> (c : &'r Connection,
                     client_major : u8,
                     client_minor : u8) -> QueryVersionCookie<'r> {
  unsafe {
    let cookie = xcb_selinux_query_version(c.get_raw_conn(),
        client_major as u8, //1
        client_minor as u8); //2
    QueryVersionCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn QueryVersionUnchecked<'r> (c : &'r Connection,
                              client_major : u8,
                              client_minor : u8) -> QueryVersionCookie<'r> {
  unsafe {
    let cookie = xcb_selinux_query_version_unchecked(c.get_raw_conn(),
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
impl_reply_cookie!(QueryVersionCookie<'s>, mk_reply_query_version_reply, QueryVersionReply, xcb_selinux_query_version_reply);

pub fn SetDeviceCreateContextChecked<'r> (c : &'r Connection,
                                      context : &str) -> base::VoidCookie<'r> {
  unsafe {
    let context = (context).as_bytes();
    let context_len = context.len();
    let context_ptr = context.as_ptr();
    let cookie = xcb_selinux_set_device_create_context_checked(c.get_raw_conn(),
        context_len as u32, //1
        context_ptr as *mut c_char); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn SetDeviceCreateContext<'r> (c : &'r Connection,
                               context : &str) -> base::VoidCookie<'r> {
  unsafe {
    let context = (context).as_bytes();
    let context_len = context.len();
    let context_ptr = context.as_ptr();
    let cookie = xcb_selinux_set_device_create_context(c.get_raw_conn(),
        context_len as u32, //1
        context_ptr as *mut c_char); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub struct GetDeviceCreateContextReply { base:  base::Reply<get_device_create_context_reply> }
fn mk_reply_get_device_create_context_reply(reply:*mut get_device_create_context_reply) -> GetDeviceCreateContextReply { GetDeviceCreateContextReply { base : base::mk_reply(reply) } }
pub fn GetDeviceCreateContext<'r> (c : &'r Connection) -> GetDeviceCreateContextCookie<'r> {
  unsafe {
    let cookie = xcb_selinux_get_device_create_context(c.get_raw_conn());
    GetDeviceCreateContextCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetDeviceCreateContextUnchecked<'r> (c : &'r Connection) -> GetDeviceCreateContextCookie<'r> {
  unsafe {
    let cookie = xcb_selinux_get_device_create_context_unchecked(c.get_raw_conn());
    GetDeviceCreateContextCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetDeviceCreateContextReply {
  pub fn context(&mut self) -> String {
    unsafe { accessor!(str, xcb_selinux_get_device_create_context_context_length, xcb_selinux_get_device_create_context_context, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetDeviceCreateContextCookie<'s>, mk_reply_get_device_create_context_reply, GetDeviceCreateContextReply, xcb_selinux_get_device_create_context_reply);

pub fn SetDeviceContextChecked<'r> (c : &'r Connection,
                                device : u32,
                                context : &str) -> base::VoidCookie<'r> {
  unsafe {
    let context = (context).as_bytes();
    let context_len = context.len();
    let context_ptr = context.as_ptr();
    let cookie = xcb_selinux_set_device_context_checked(c.get_raw_conn(),
        device as u32, //1
        context_len as u32, //2
        context_ptr as *mut c_char); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn SetDeviceContext<'r> (c : &'r Connection,
                         device : u32,
                         context : &str) -> base::VoidCookie<'r> {
  unsafe {
    let context = (context).as_bytes();
    let context_len = context.len();
    let context_ptr = context.as_ptr();
    let cookie = xcb_selinux_set_device_context(c.get_raw_conn(),
        device as u32, //1
        context_len as u32, //2
        context_ptr as *mut c_char); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub struct GetDeviceContextReply { base:  base::Reply<get_device_context_reply> }
fn mk_reply_get_device_context_reply(reply:*mut get_device_context_reply) -> GetDeviceContextReply { GetDeviceContextReply { base : base::mk_reply(reply) } }
pub fn GetDeviceContext<'r> (c : &'r Connection,
                         device : u32) -> GetDeviceContextCookie<'r> {
  unsafe {
    let cookie = xcb_selinux_get_device_context(c.get_raw_conn(),
        device as u32); //1
    GetDeviceContextCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetDeviceContextUnchecked<'r> (c : &'r Connection,
                                  device : u32) -> GetDeviceContextCookie<'r> {
  unsafe {
    let cookie = xcb_selinux_get_device_context_unchecked(c.get_raw_conn(),
        device as u32); //1
    GetDeviceContextCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetDeviceContextReply {
  pub fn context(&mut self) -> String {
    unsafe { accessor!(str, xcb_selinux_get_device_context_context_length, xcb_selinux_get_device_context_context, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetDeviceContextCookie<'s>, mk_reply_get_device_context_reply, GetDeviceContextReply, xcb_selinux_get_device_context_reply);

pub fn SetWindowCreateContextChecked<'r> (c : &'r Connection,
                                      context : &str) -> base::VoidCookie<'r> {
  unsafe {
    let context = (context).as_bytes();
    let context_len = context.len();
    let context_ptr = context.as_ptr();
    let cookie = xcb_selinux_set_window_create_context_checked(c.get_raw_conn(),
        context_len as u32, //1
        context_ptr as *mut c_char); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn SetWindowCreateContext<'r> (c : &'r Connection,
                               context : &str) -> base::VoidCookie<'r> {
  unsafe {
    let context = (context).as_bytes();
    let context_len = context.len();
    let context_ptr = context.as_ptr();
    let cookie = xcb_selinux_set_window_create_context(c.get_raw_conn(),
        context_len as u32, //1
        context_ptr as *mut c_char); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub struct GetWindowCreateContextReply { base:  base::Reply<get_window_create_context_reply> }
fn mk_reply_get_window_create_context_reply(reply:*mut get_window_create_context_reply) -> GetWindowCreateContextReply { GetWindowCreateContextReply { base : base::mk_reply(reply) } }
pub fn GetWindowCreateContext<'r> (c : &'r Connection) -> GetWindowCreateContextCookie<'r> {
  unsafe {
    let cookie = xcb_selinux_get_window_create_context(c.get_raw_conn());
    GetWindowCreateContextCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetWindowCreateContextUnchecked<'r> (c : &'r Connection) -> GetWindowCreateContextCookie<'r> {
  unsafe {
    let cookie = xcb_selinux_get_window_create_context_unchecked(c.get_raw_conn());
    GetWindowCreateContextCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetWindowCreateContextReply {
  pub fn context(&mut self) -> String {
    unsafe { accessor!(str, xcb_selinux_get_window_create_context_context_length, xcb_selinux_get_window_create_context_context, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetWindowCreateContextCookie<'s>, mk_reply_get_window_create_context_reply, GetWindowCreateContextReply, xcb_selinux_get_window_create_context_reply);

pub struct GetWindowContextReply { base:  base::Reply<get_window_context_reply> }
fn mk_reply_get_window_context_reply(reply:*mut get_window_context_reply) -> GetWindowContextReply { GetWindowContextReply { base : base::mk_reply(reply) } }
pub fn GetWindowContext<'r> (c : &'r Connection,
                         window : xproto::Window) -> GetWindowContextCookie<'r> {
  unsafe {
    let cookie = xcb_selinux_get_window_context(c.get_raw_conn(),
        window as ffi::xproto::window); //1
    GetWindowContextCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetWindowContextUnchecked<'r> (c : &'r Connection,
                                  window : xproto::Window) -> GetWindowContextCookie<'r> {
  unsafe {
    let cookie = xcb_selinux_get_window_context_unchecked(c.get_raw_conn(),
        window as ffi::xproto::window); //1
    GetWindowContextCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetWindowContextReply {
  pub fn context(&mut self) -> String {
    unsafe { accessor!(str, xcb_selinux_get_window_context_context_length, xcb_selinux_get_window_context_context, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetWindowContextCookie<'s>, mk_reply_get_window_context_reply, GetWindowContextReply, xcb_selinux_get_window_context_reply);

pub struct ListItem {pub base : base::Struct<list_item> }


impl ListItem {
  pub fn name(&mut self) -> xproto::Atom {
    unsafe { accessor!(name -> xproto::Atom, self.base.strct) }
  }

  pub fn object_context(&mut self) -> String {
    unsafe { accessor!(str, xcb_selinux_list_item_object_context_length, xcb_selinux_list_item_object_context, self.base.strct) }
  }

  pub fn data_context(&mut self) -> String {
    unsafe { accessor!(str, xcb_selinux_list_item_data_context_length, xcb_selinux_list_item_data_context, self.base.strct) }
  }

}

impl Iterator for ListItemIterator {
    type Item = ListItem;
    fn next(&mut self) -> Option<ListItem> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut list_item_iterator = mem::transmute(self);
            let data = (*iter).data;
            xcb_selinux_list_item_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub fn SetPropertyCreateContextChecked<'r> (c : &'r Connection,
                                        context : &str) -> base::VoidCookie<'r> {
  unsafe {
    let context = (context).as_bytes();
    let context_len = context.len();
    let context_ptr = context.as_ptr();
    let cookie = xcb_selinux_set_property_create_context_checked(c.get_raw_conn(),
        context_len as u32, //1
        context_ptr as *mut c_char); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn SetPropertyCreateContext<'r> (c : &'r Connection,
                                 context : &str) -> base::VoidCookie<'r> {
  unsafe {
    let context = (context).as_bytes();
    let context_len = context.len();
    let context_ptr = context.as_ptr();
    let cookie = xcb_selinux_set_property_create_context(c.get_raw_conn(),
        context_len as u32, //1
        context_ptr as *mut c_char); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub struct GetPropertyCreateContextReply { base:  base::Reply<get_property_create_context_reply> }
fn mk_reply_get_property_create_context_reply(reply:*mut get_property_create_context_reply) -> GetPropertyCreateContextReply { GetPropertyCreateContextReply { base : base::mk_reply(reply) } }
pub fn GetPropertyCreateContext<'r> (c : &'r Connection) -> GetPropertyCreateContextCookie<'r> {
  unsafe {
    let cookie = xcb_selinux_get_property_create_context(c.get_raw_conn());
    GetPropertyCreateContextCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetPropertyCreateContextUnchecked<'r> (c : &'r Connection) -> GetPropertyCreateContextCookie<'r> {
  unsafe {
    let cookie = xcb_selinux_get_property_create_context_unchecked(c.get_raw_conn());
    GetPropertyCreateContextCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetPropertyCreateContextReply {
  pub fn context(&mut self) -> String {
    unsafe { accessor!(str, xcb_selinux_get_property_create_context_context_length, xcb_selinux_get_property_create_context_context, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetPropertyCreateContextCookie<'s>, mk_reply_get_property_create_context_reply, GetPropertyCreateContextReply, xcb_selinux_get_property_create_context_reply);

pub fn SetPropertyUseContextChecked<'r> (c : &'r Connection,
                                     context : &str) -> base::VoidCookie<'r> {
  unsafe {
    let context = (context).as_bytes();
    let context_len = context.len();
    let context_ptr = context.as_ptr();
    let cookie = xcb_selinux_set_property_use_context_checked(c.get_raw_conn(),
        context_len as u32, //1
        context_ptr as *mut c_char); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn SetPropertyUseContext<'r> (c : &'r Connection,
                              context : &str) -> base::VoidCookie<'r> {
  unsafe {
    let context = (context).as_bytes();
    let context_len = context.len();
    let context_ptr = context.as_ptr();
    let cookie = xcb_selinux_set_property_use_context(c.get_raw_conn(),
        context_len as u32, //1
        context_ptr as *mut c_char); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub struct GetPropertyUseContextReply { base:  base::Reply<get_property_use_context_reply> }
fn mk_reply_get_property_use_context_reply(reply:*mut get_property_use_context_reply) -> GetPropertyUseContextReply { GetPropertyUseContextReply { base : base::mk_reply(reply) } }
pub fn GetPropertyUseContext<'r> (c : &'r Connection) -> GetPropertyUseContextCookie<'r> {
  unsafe {
    let cookie = xcb_selinux_get_property_use_context(c.get_raw_conn());
    GetPropertyUseContextCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetPropertyUseContextUnchecked<'r> (c : &'r Connection) -> GetPropertyUseContextCookie<'r> {
  unsafe {
    let cookie = xcb_selinux_get_property_use_context_unchecked(c.get_raw_conn());
    GetPropertyUseContextCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetPropertyUseContextReply {
  pub fn context(&mut self) -> String {
    unsafe { accessor!(str, xcb_selinux_get_property_use_context_context_length, xcb_selinux_get_property_use_context_context, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetPropertyUseContextCookie<'s>, mk_reply_get_property_use_context_reply, GetPropertyUseContextReply, xcb_selinux_get_property_use_context_reply);

pub struct GetPropertyContextReply { base:  base::Reply<get_property_context_reply> }
fn mk_reply_get_property_context_reply(reply:*mut get_property_context_reply) -> GetPropertyContextReply { GetPropertyContextReply { base : base::mk_reply(reply) } }
pub fn GetPropertyContext<'r> (c : &'r Connection,
                           window : xproto::Window,
                           property : xproto::Atom) -> GetPropertyContextCookie<'r> {
  unsafe {
    let cookie = xcb_selinux_get_property_context(c.get_raw_conn(),
        window as ffi::xproto::window, //1
        property as ffi::xproto::atom); //2
    GetPropertyContextCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetPropertyContextUnchecked<'r> (c : &'r Connection,
                                    window : xproto::Window,
                                    property : xproto::Atom) -> GetPropertyContextCookie<'r> {
  unsafe {
    let cookie = xcb_selinux_get_property_context_unchecked(c.get_raw_conn(),
        window as ffi::xproto::window, //1
        property as ffi::xproto::atom); //2
    GetPropertyContextCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetPropertyContextReply {
  pub fn context(&mut self) -> String {
    unsafe { accessor!(str, xcb_selinux_get_property_context_context_length, xcb_selinux_get_property_context_context, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetPropertyContextCookie<'s>, mk_reply_get_property_context_reply, GetPropertyContextReply, xcb_selinux_get_property_context_reply);

pub struct GetPropertyDataContextReply { base:  base::Reply<get_property_data_context_reply> }
fn mk_reply_get_property_data_context_reply(reply:*mut get_property_data_context_reply) -> GetPropertyDataContextReply { GetPropertyDataContextReply { base : base::mk_reply(reply) } }
pub fn GetPropertyDataContext<'r> (c : &'r Connection,
                               window : xproto::Window,
                               property : xproto::Atom) -> GetPropertyDataContextCookie<'r> {
  unsafe {
    let cookie = xcb_selinux_get_property_data_context(c.get_raw_conn(),
        window as ffi::xproto::window, //1
        property as ffi::xproto::atom); //2
    GetPropertyDataContextCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetPropertyDataContextUnchecked<'r> (c : &'r Connection,
                                        window : xproto::Window,
                                        property : xproto::Atom) -> GetPropertyDataContextCookie<'r> {
  unsafe {
    let cookie = xcb_selinux_get_property_data_context_unchecked(c.get_raw_conn(),
        window as ffi::xproto::window, //1
        property as ffi::xproto::atom); //2
    GetPropertyDataContextCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetPropertyDataContextReply {
  pub fn context(&mut self) -> String {
    unsafe { accessor!(str, xcb_selinux_get_property_data_context_context_length, xcb_selinux_get_property_data_context_context, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetPropertyDataContextCookie<'s>, mk_reply_get_property_data_context_reply, GetPropertyDataContextReply, xcb_selinux_get_property_data_context_reply);

pub struct ListPropertiesReply { base:  base::Reply<list_properties_reply> }
fn mk_reply_list_properties_reply(reply:*mut list_properties_reply) -> ListPropertiesReply { ListPropertiesReply { base : base::mk_reply(reply) } }
pub fn ListProperties<'r> (c : &'r Connection,
                       window : xproto::Window) -> ListPropertiesCookie<'r> {
  unsafe {
    let cookie = xcb_selinux_list_properties(c.get_raw_conn(),
        window as ffi::xproto::window); //1
    ListPropertiesCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn ListPropertiesUnchecked<'r> (c : &'r Connection,
                                window : xproto::Window) -> ListPropertiesCookie<'r> {
  unsafe {
    let cookie = xcb_selinux_list_properties_unchecked(c.get_raw_conn(),
        window as ffi::xproto::window); //1
    ListPropertiesCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl ListPropertiesReply {
  pub fn properties(&mut self) -> ListItemIterator {
    unsafe { accessor!(ListItemIterator, xcb_selinux_list_properties_properties_iterator, (*self.base.reply)) }
  }

}
impl_reply_cookie!(ListPropertiesCookie<'s>, mk_reply_list_properties_reply, ListPropertiesReply, xcb_selinux_list_properties_reply);

pub fn SetSelectionCreateContextChecked<'r> (c : &'r Connection,
                                         context : &str) -> base::VoidCookie<'r> {
  unsafe {
    let context = (context).as_bytes();
    let context_len = context.len();
    let context_ptr = context.as_ptr();
    let cookie = xcb_selinux_set_selection_create_context_checked(c.get_raw_conn(),
        context_len as u32, //1
        context_ptr as *mut c_char); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn SetSelectionCreateContext<'r> (c : &'r Connection,
                                  context : &str) -> base::VoidCookie<'r> {
  unsafe {
    let context = (context).as_bytes();
    let context_len = context.len();
    let context_ptr = context.as_ptr();
    let cookie = xcb_selinux_set_selection_create_context(c.get_raw_conn(),
        context_len as u32, //1
        context_ptr as *mut c_char); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub struct GetSelectionCreateContextReply { base:  base::Reply<get_selection_create_context_reply> }
fn mk_reply_get_selection_create_context_reply(reply:*mut get_selection_create_context_reply) -> GetSelectionCreateContextReply { GetSelectionCreateContextReply { base : base::mk_reply(reply) } }
pub fn GetSelectionCreateContext<'r> (c : &'r Connection) -> GetSelectionCreateContextCookie<'r> {
  unsafe {
    let cookie = xcb_selinux_get_selection_create_context(c.get_raw_conn());
    GetSelectionCreateContextCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetSelectionCreateContextUnchecked<'r> (c : &'r Connection) -> GetSelectionCreateContextCookie<'r> {
  unsafe {
    let cookie = xcb_selinux_get_selection_create_context_unchecked(c.get_raw_conn());
    GetSelectionCreateContextCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetSelectionCreateContextReply {
  pub fn context(&mut self) -> String {
    unsafe { accessor!(str, xcb_selinux_get_selection_create_context_context_length, xcb_selinux_get_selection_create_context_context, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetSelectionCreateContextCookie<'s>, mk_reply_get_selection_create_context_reply, GetSelectionCreateContextReply, xcb_selinux_get_selection_create_context_reply);

pub fn SetSelectionUseContextChecked<'r> (c : &'r Connection,
                                      context : &str) -> base::VoidCookie<'r> {
  unsafe {
    let context = (context).as_bytes();
    let context_len = context.len();
    let context_ptr = context.as_ptr();
    let cookie = xcb_selinux_set_selection_use_context_checked(c.get_raw_conn(),
        context_len as u32, //1
        context_ptr as *mut c_char); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn SetSelectionUseContext<'r> (c : &'r Connection,
                               context : &str) -> base::VoidCookie<'r> {
  unsafe {
    let context = (context).as_bytes();
    let context_len = context.len();
    let context_ptr = context.as_ptr();
    let cookie = xcb_selinux_set_selection_use_context(c.get_raw_conn(),
        context_len as u32, //1
        context_ptr as *mut c_char); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub struct GetSelectionUseContextReply { base:  base::Reply<get_selection_use_context_reply> }
fn mk_reply_get_selection_use_context_reply(reply:*mut get_selection_use_context_reply) -> GetSelectionUseContextReply { GetSelectionUseContextReply { base : base::mk_reply(reply) } }
pub fn GetSelectionUseContext<'r> (c : &'r Connection) -> GetSelectionUseContextCookie<'r> {
  unsafe {
    let cookie = xcb_selinux_get_selection_use_context(c.get_raw_conn());
    GetSelectionUseContextCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetSelectionUseContextUnchecked<'r> (c : &'r Connection) -> GetSelectionUseContextCookie<'r> {
  unsafe {
    let cookie = xcb_selinux_get_selection_use_context_unchecked(c.get_raw_conn());
    GetSelectionUseContextCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetSelectionUseContextReply {
  pub fn context(&mut self) -> String {
    unsafe { accessor!(str, xcb_selinux_get_selection_use_context_context_length, xcb_selinux_get_selection_use_context_context, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetSelectionUseContextCookie<'s>, mk_reply_get_selection_use_context_reply, GetSelectionUseContextReply, xcb_selinux_get_selection_use_context_reply);

pub struct GetSelectionContextReply { base:  base::Reply<get_selection_context_reply> }
fn mk_reply_get_selection_context_reply(reply:*mut get_selection_context_reply) -> GetSelectionContextReply { GetSelectionContextReply { base : base::mk_reply(reply) } }
pub fn GetSelectionContext<'r> (c : &'r Connection,
                            selection : xproto::Atom) -> GetSelectionContextCookie<'r> {
  unsafe {
    let cookie = xcb_selinux_get_selection_context(c.get_raw_conn(),
        selection as ffi::xproto::atom); //1
    GetSelectionContextCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetSelectionContextUnchecked<'r> (c : &'r Connection,
                                     selection : xproto::Atom) -> GetSelectionContextCookie<'r> {
  unsafe {
    let cookie = xcb_selinux_get_selection_context_unchecked(c.get_raw_conn(),
        selection as ffi::xproto::atom); //1
    GetSelectionContextCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetSelectionContextReply {
  pub fn context(&mut self) -> String {
    unsafe { accessor!(str, xcb_selinux_get_selection_context_context_length, xcb_selinux_get_selection_context_context, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetSelectionContextCookie<'s>, mk_reply_get_selection_context_reply, GetSelectionContextReply, xcb_selinux_get_selection_context_reply);

pub struct GetSelectionDataContextReply { base:  base::Reply<get_selection_data_context_reply> }
fn mk_reply_get_selection_data_context_reply(reply:*mut get_selection_data_context_reply) -> GetSelectionDataContextReply { GetSelectionDataContextReply { base : base::mk_reply(reply) } }
pub fn GetSelectionDataContext<'r> (c : &'r Connection,
                                selection : xproto::Atom) -> GetSelectionDataContextCookie<'r> {
  unsafe {
    let cookie = xcb_selinux_get_selection_data_context(c.get_raw_conn(),
        selection as ffi::xproto::atom); //1
    GetSelectionDataContextCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetSelectionDataContextUnchecked<'r> (c : &'r Connection,
                                         selection : xproto::Atom) -> GetSelectionDataContextCookie<'r> {
  unsafe {
    let cookie = xcb_selinux_get_selection_data_context_unchecked(c.get_raw_conn(),
        selection as ffi::xproto::atom); //1
    GetSelectionDataContextCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetSelectionDataContextReply {
  pub fn context(&mut self) -> String {
    unsafe { accessor!(str, xcb_selinux_get_selection_data_context_context_length, xcb_selinux_get_selection_data_context_context, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetSelectionDataContextCookie<'s>, mk_reply_get_selection_data_context_reply, GetSelectionDataContextReply, xcb_selinux_get_selection_data_context_reply);

pub struct ListSelectionsReply { base:  base::Reply<list_selections_reply> }
fn mk_reply_list_selections_reply(reply:*mut list_selections_reply) -> ListSelectionsReply { ListSelectionsReply { base : base::mk_reply(reply) } }
pub fn ListSelections<'r> (c : &'r Connection) -> ListSelectionsCookie<'r> {
  unsafe {
    let cookie = xcb_selinux_list_selections(c.get_raw_conn());
    ListSelectionsCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn ListSelectionsUnchecked<'r> (c : &'r Connection) -> ListSelectionsCookie<'r> {
  unsafe {
    let cookie = xcb_selinux_list_selections_unchecked(c.get_raw_conn());
    ListSelectionsCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl ListSelectionsReply {
  pub fn selections(&mut self) -> ListItemIterator {
    unsafe { accessor!(ListItemIterator, xcb_selinux_list_selections_selections_iterator, (*self.base.reply)) }
  }

}
impl_reply_cookie!(ListSelectionsCookie<'s>, mk_reply_list_selections_reply, ListSelectionsReply, xcb_selinux_list_selections_reply);

pub struct GetClientContextReply { base:  base::Reply<get_client_context_reply> }
fn mk_reply_get_client_context_reply(reply:*mut get_client_context_reply) -> GetClientContextReply { GetClientContextReply { base : base::mk_reply(reply) } }
pub fn GetClientContext<'r> (c : &'r Connection,
                         resource : u32) -> GetClientContextCookie<'r> {
  unsafe {
    let cookie = xcb_selinux_get_client_context(c.get_raw_conn(),
        resource as u32); //1
    GetClientContextCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetClientContextUnchecked<'r> (c : &'r Connection,
                                  resource : u32) -> GetClientContextCookie<'r> {
  unsafe {
    let cookie = xcb_selinux_get_client_context_unchecked(c.get_raw_conn(),
        resource as u32); //1
    GetClientContextCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetClientContextReply {
  pub fn context(&mut self) -> String {
    unsafe { accessor!(str, xcb_selinux_get_client_context_context_length, xcb_selinux_get_client_context_context, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetClientContextCookie<'s>, mk_reply_get_client_context_reply, GetClientContextReply, xcb_selinux_get_client_context_reply);


