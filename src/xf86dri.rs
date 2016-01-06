/*
 * This file generated automatically from xf86dri.xml by r_client.py.
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
use ffi::xf86dri::*;
use std::option::Option;
use std::iter::Iterator;

pub struct DrmClipRect {pub base : base::Struct<drm_clip_rect> }

pub type DrmClipRectIterator = drm_clip_rect_iterator;

pub struct  QueryVersionCookie<'s> { pub base : base::Cookie<'s, query_version_cookie> }

/** Opcode for xcb_xf86dri_query_version. */
pub static XCB_XF86DRI_QUERY_VERSION : u8 = 0;
pub struct QueryVersionReply { base:  base::Reply<query_version_reply> }
fn mk_reply_query_version_reply(reply:*mut query_version_reply) -> QueryVersionReply { QueryVersionReply { base : base::mk_reply(reply) } }
pub struct  QueryDirectRenderingCapableCookie<'s> { pub base : base::Cookie<'s, query_direct_rendering_capable_cookie> }

/** Opcode for xcb_xf86dri_query_direct_rendering_capable. */
pub static XCB_XF86DRI_QUERY_DIRECT_RENDERING_CAPABLE : u8 = 1;
pub struct QueryDirectRenderingCapableReply { base:  base::Reply<query_direct_rendering_capable_reply> }
fn mk_reply_query_direct_rendering_capable_reply(reply:*mut query_direct_rendering_capable_reply) -> QueryDirectRenderingCapableReply { QueryDirectRenderingCapableReply { base : base::mk_reply(reply) } }
pub struct  OpenConnectionCookie<'s> { pub base : base::Cookie<'s, open_connection_cookie> }

/** Opcode for xcb_xf86dri_open_connection. */
pub static XCB_XF86DRI_OPEN_CONNECTION : u8 = 2;
/** Opcode for xcb_xf86dri_close_connection. */
pub static XCB_XF86DRI_CLOSE_CONNECTION : u8 = 3;
pub struct  GetClientDriverNameCookie<'s> { pub base : base::Cookie<'s, get_client_driver_name_cookie> }

/** Opcode for xcb_xf86dri_get_client_driver_name. */
pub static XCB_XF86DRI_GET_CLIENT_DRIVER_NAME : u8 = 4;
pub struct  CreateContextCookie<'s> { pub base : base::Cookie<'s, create_context_cookie> }

/** Opcode for xcb_xf86dri_create_context. */
pub static XCB_XF86DRI_CREATE_CONTEXT : u8 = 5;
pub struct CreateContextReply { base:  base::Reply<create_context_reply> }
fn mk_reply_create_context_reply(reply:*mut create_context_reply) -> CreateContextReply { CreateContextReply { base : base::mk_reply(reply) } }
/** Opcode for xcb_xf86dri_destroy_context. */
pub static XCB_XF86DRI_DESTROY_CONTEXT : u8 = 6;
pub struct  CreateDrawableCookie<'s> { pub base : base::Cookie<'s, create_drawable_cookie> }

/** Opcode for xcb_xf86dri_create_drawable. */
pub static XCB_XF86DRI_CREATE_DRAWABLE : u8 = 7;
pub struct CreateDrawableReply { base:  base::Reply<create_drawable_reply> }
fn mk_reply_create_drawable_reply(reply:*mut create_drawable_reply) -> CreateDrawableReply { CreateDrawableReply { base : base::mk_reply(reply) } }
/** Opcode for xcb_xf86dri_destroy_drawable. */
pub static XCB_XF86DRI_DESTROY_DRAWABLE : u8 = 8;
pub struct  GetDrawableInfoCookie<'s> { pub base : base::Cookie<'s, get_drawable_info_cookie> }

/** Opcode for xcb_xf86dri_get_drawable_info. */
pub static XCB_XF86DRI_GET_DRAWABLE_INFO : u8 = 9;
pub struct  GetDeviceInfoCookie<'s> { pub base : base::Cookie<'s, get_device_info_cookie> }

/** Opcode for xcb_xf86dri_get_device_info. */
pub static XCB_XF86DRI_GET_DEVICE_INFO : u8 = 10;
pub struct  AuthConnectionCookie<'s> { pub base : base::Cookie<'s, auth_connection_cookie> }

/** Opcode for xcb_xf86dri_auth_connection. */
pub static XCB_XF86DRI_AUTH_CONNECTION : u8 = 11;
pub struct AuthConnectionReply { base:  base::Reply<auth_connection_reply> }
fn mk_reply_auth_connection_reply(reply:*mut auth_connection_reply) -> AuthConnectionReply { AuthConnectionReply { base : base::mk_reply(reply) } }

impl DrmClipRect {
  pub fn x1(&mut self) -> i16 {
    unsafe { accessor!(x1 -> i16, self.base.strct) }
  }

  pub fn y1(&mut self) -> i16 {
    unsafe { accessor!(y1 -> i16, self.base.strct) }
  }

  pub fn x2(&mut self) -> i16 {
    unsafe { accessor!(x2 -> i16, self.base.strct) }
  }

  pub fn x3(&mut self) -> i16 {
    unsafe { accessor!(x3 -> i16, self.base.strct) }
  }

}

impl Iterator for DrmClipRectIterator {
    type Item = DrmClipRect;
    fn next(&mut self) -> Option<DrmClipRect> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut drm_clip_rect_iterator = mem::transmute(self);
            let data = (*iter).data;
            xcb_xf86dri_drm_clip_rect_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub fn QueryVersion<'r> (c : &'r Connection) -> QueryVersionCookie<'r> {
  unsafe {
    let cookie = xcb_xf86dri_query_version(c.get_raw_conn());
    QueryVersionCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn QueryVersionUnchecked<'r> (c : &'r Connection) -> QueryVersionCookie<'r> {
  unsafe {
    let cookie = xcb_xf86dri_query_version_unchecked(c.get_raw_conn());
    QueryVersionCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl QueryVersionReply {
  pub fn dri_major_version(&mut self) -> u16 {
    unsafe { accessor!(dri_major_version -> u16, (*self.base.reply)) }
  }

  pub fn dri_minor_version(&mut self) -> u16 {
    unsafe { accessor!(dri_minor_version -> u16, (*self.base.reply)) }
  }

  pub fn dri_minor_patch(&mut self) -> u32 {
    unsafe { accessor!(dri_minor_patch -> u32, (*self.base.reply)) }
  }

}
impl_reply_cookie!(QueryVersionCookie<'s>, mk_reply_query_version_reply, QueryVersionReply, xcb_xf86dri_query_version_reply);

pub fn QueryDirectRenderingCapable<'r> (c : &'r Connection,
                                    screen : u32) -> QueryDirectRenderingCapableCookie<'r> {
  unsafe {
    let cookie = xcb_xf86dri_query_direct_rendering_capable(c.get_raw_conn(),
        screen as u32); //1
    QueryDirectRenderingCapableCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn QueryDirectRenderingCapableUnchecked<'r> (c : &'r Connection,
                                             screen : u32) -> QueryDirectRenderingCapableCookie<'r> {
  unsafe {
    let cookie = xcb_xf86dri_query_direct_rendering_capable_unchecked(c.get_raw_conn(),
        screen as u32); //1
    QueryDirectRenderingCapableCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl QueryDirectRenderingCapableReply {
  pub fn is_capable(&mut self) -> u8 {
    unsafe { accessor!(is_capable -> u8, (*self.base.reply)) }
  }

}
impl_reply_cookie!(QueryDirectRenderingCapableCookie<'s>, mk_reply_query_direct_rendering_capable_reply, QueryDirectRenderingCapableReply, xcb_xf86dri_query_direct_rendering_capable_reply);

pub struct OpenConnectionReply { base:  base::Reply<open_connection_reply> }
fn mk_reply_open_connection_reply(reply:*mut open_connection_reply) -> OpenConnectionReply { OpenConnectionReply { base : base::mk_reply(reply) } }
pub fn OpenConnection<'r> (c : &'r Connection,
                       screen : u32) -> OpenConnectionCookie<'r> {
  unsafe {
    let cookie = xcb_xf86dri_open_connection(c.get_raw_conn(),
        screen as u32); //1
    OpenConnectionCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn OpenConnectionUnchecked<'r> (c : &'r Connection,
                                screen : u32) -> OpenConnectionCookie<'r> {
  unsafe {
    let cookie = xcb_xf86dri_open_connection_unchecked(c.get_raw_conn(),
        screen as u32); //1
    OpenConnectionCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl OpenConnectionReply {
  pub fn sarea_handle_low(&mut self) -> u32 {
    unsafe { accessor!(sarea_handle_low -> u32, (*self.base.reply)) }
  }

  pub fn sarea_handle_high(&mut self) -> u32 {
    unsafe { accessor!(sarea_handle_high -> u32, (*self.base.reply)) }
  }

  pub fn bus_id(&mut self) -> String {
    unsafe { accessor!(str, xcb_xf86dri_open_connection_bus_id_length, xcb_xf86dri_open_connection_bus_id, (*self.base.reply)) }
  }

}
impl_reply_cookie!(OpenConnectionCookie<'s>, mk_reply_open_connection_reply, OpenConnectionReply, xcb_xf86dri_open_connection_reply);

pub fn CloseConnectionChecked<'r> (c : &'r Connection,
                               screen : u32) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xf86dri_close_connection_checked(c.get_raw_conn(),
        screen as u32); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn CloseConnection<'r> (c : &'r Connection,
                        screen : u32) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xf86dri_close_connection(c.get_raw_conn(),
        screen as u32); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub struct GetClientDriverNameReply { base:  base::Reply<get_client_driver_name_reply> }
fn mk_reply_get_client_driver_name_reply(reply:*mut get_client_driver_name_reply) -> GetClientDriverNameReply { GetClientDriverNameReply { base : base::mk_reply(reply) } }
pub fn GetClientDriverName<'r> (c : &'r Connection,
                            screen : u32) -> GetClientDriverNameCookie<'r> {
  unsafe {
    let cookie = xcb_xf86dri_get_client_driver_name(c.get_raw_conn(),
        screen as u32); //1
    GetClientDriverNameCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetClientDriverNameUnchecked<'r> (c : &'r Connection,
                                     screen : u32) -> GetClientDriverNameCookie<'r> {
  unsafe {
    let cookie = xcb_xf86dri_get_client_driver_name_unchecked(c.get_raw_conn(),
        screen as u32); //1
    GetClientDriverNameCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetClientDriverNameReply {
  pub fn client_driver_major_version(&mut self) -> u32 {
    unsafe { accessor!(client_driver_major_version -> u32, (*self.base.reply)) }
  }

  pub fn client_driver_minor_version(&mut self) -> u32 {
    unsafe { accessor!(client_driver_minor_version -> u32, (*self.base.reply)) }
  }

  pub fn client_driver_patch_version(&mut self) -> u32 {
    unsafe { accessor!(client_driver_patch_version -> u32, (*self.base.reply)) }
  }

  pub fn client_driver_name(&mut self) -> String {
    unsafe { accessor!(str, xcb_xf86dri_get_client_driver_name_client_driver_name_length, xcb_xf86dri_get_client_driver_name_client_driver_name, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetClientDriverNameCookie<'s>, mk_reply_get_client_driver_name_reply, GetClientDriverNameReply, xcb_xf86dri_get_client_driver_name_reply);

pub fn CreateContext<'r> (c : &'r Connection,
                      screen : u32,
                      visual : u32,
                      context : u32) -> CreateContextCookie<'r> {
  unsafe {
    let cookie = xcb_xf86dri_create_context(c.get_raw_conn(),
        screen as u32, //1
        visual as u32, //2
        context as u32); //3
    CreateContextCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn CreateContextUnchecked<'r> (c : &'r Connection,
                               screen : u32,
                               visual : u32,
                               context : u32) -> CreateContextCookie<'r> {
  unsafe {
    let cookie = xcb_xf86dri_create_context_unchecked(c.get_raw_conn(),
        screen as u32, //1
        visual as u32, //2
        context as u32); //3
    CreateContextCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl CreateContextReply {
  pub fn hw_context(&mut self) -> u32 {
    unsafe { accessor!(hw_context -> u32, (*self.base.reply)) }
  }

}
impl_reply_cookie!(CreateContextCookie<'s>, mk_reply_create_context_reply, CreateContextReply, xcb_xf86dri_create_context_reply);

pub fn DestroyContextChecked<'r> (c : &'r Connection,
                              screen : u32,
                              context : u32) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xf86dri_destroy_context_checked(c.get_raw_conn(),
        screen as u32, //1
        context as u32); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn DestroyContext<'r> (c : &'r Connection,
                       screen : u32,
                       context : u32) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xf86dri_destroy_context(c.get_raw_conn(),
        screen as u32, //1
        context as u32); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn CreateDrawable<'r> (c : &'r Connection,
                       screen : u32,
                       drawable : u32) -> CreateDrawableCookie<'r> {
  unsafe {
    let cookie = xcb_xf86dri_create_drawable(c.get_raw_conn(),
        screen as u32, //1
        drawable as u32); //2
    CreateDrawableCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn CreateDrawableUnchecked<'r> (c : &'r Connection,
                                screen : u32,
                                drawable : u32) -> CreateDrawableCookie<'r> {
  unsafe {
    let cookie = xcb_xf86dri_create_drawable_unchecked(c.get_raw_conn(),
        screen as u32, //1
        drawable as u32); //2
    CreateDrawableCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl CreateDrawableReply {
  pub fn hw_drawable_handle(&mut self) -> u32 {
    unsafe { accessor!(hw_drawable_handle -> u32, (*self.base.reply)) }
  }

}
impl_reply_cookie!(CreateDrawableCookie<'s>, mk_reply_create_drawable_reply, CreateDrawableReply, xcb_xf86dri_create_drawable_reply);

pub fn DestroyDrawableChecked<'r> (c : &'r Connection,
                               screen : u32,
                               drawable : u32) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xf86dri_destroy_drawable_checked(c.get_raw_conn(),
        screen as u32, //1
        drawable as u32); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn DestroyDrawable<'r> (c : &'r Connection,
                        screen : u32,
                        drawable : u32) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xf86dri_destroy_drawable(c.get_raw_conn(),
        screen as u32, //1
        drawable as u32); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub struct GetDrawableInfoReply { base:  base::Reply<get_drawable_info_reply> }
fn mk_reply_get_drawable_info_reply(reply:*mut get_drawable_info_reply) -> GetDrawableInfoReply { GetDrawableInfoReply { base : base::mk_reply(reply) } }
pub fn GetDrawableInfo<'r> (c : &'r Connection,
                        screen : u32,
                        drawable : u32) -> GetDrawableInfoCookie<'r> {
  unsafe {
    let cookie = xcb_xf86dri_get_drawable_info(c.get_raw_conn(),
        screen as u32, //1
        drawable as u32); //2
    GetDrawableInfoCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetDrawableInfoUnchecked<'r> (c : &'r Connection,
                                 screen : u32,
                                 drawable : u32) -> GetDrawableInfoCookie<'r> {
  unsafe {
    let cookie = xcb_xf86dri_get_drawable_info_unchecked(c.get_raw_conn(),
        screen as u32, //1
        drawable as u32); //2
    GetDrawableInfoCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetDrawableInfoReply {
  pub fn drawable_table_index(&mut self) -> u32 {
    unsafe { accessor!(drawable_table_index -> u32, (*self.base.reply)) }
  }

  pub fn drawable_table_stamp(&mut self) -> u32 {
    unsafe { accessor!(drawable_table_stamp -> u32, (*self.base.reply)) }
  }

  pub fn drawable_origin_X(&mut self) -> i16 {
    unsafe { accessor!(drawable_origin_X -> i16, (*self.base.reply)) }
  }

  pub fn drawable_origin_Y(&mut self) -> i16 {
    unsafe { accessor!(drawable_origin_Y -> i16, (*self.base.reply)) }
  }

  pub fn drawable_size_W(&mut self) -> i16 {
    unsafe { accessor!(drawable_size_W -> i16, (*self.base.reply)) }
  }

  pub fn drawable_size_H(&mut self) -> i16 {
    unsafe { accessor!(drawable_size_H -> i16, (*self.base.reply)) }
  }

  pub fn back_x(&mut self) -> i16 {
    unsafe { accessor!(back_x -> i16, (*self.base.reply)) }
  }

  pub fn back_y(&mut self) -> i16 {
    unsafe { accessor!(back_y -> i16, (*self.base.reply)) }
  }

  pub fn clip_rects(&mut self) -> DrmClipRectIterator {
    unsafe { accessor!(DrmClipRectIterator, xcb_xf86dri_get_drawable_info_clip_rects_iterator, (*self.base.reply)) }
  }

  pub fn back_clip_rects(&mut self) -> DrmClipRectIterator {
    unsafe { accessor!(DrmClipRectIterator, xcb_xf86dri_get_drawable_info_back_clip_rects_iterator, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetDrawableInfoCookie<'s>, mk_reply_get_drawable_info_reply, GetDrawableInfoReply, xcb_xf86dri_get_drawable_info_reply);

pub struct GetDeviceInfoReply { base:  base::Reply<get_device_info_reply> }
fn mk_reply_get_device_info_reply(reply:*mut get_device_info_reply) -> GetDeviceInfoReply { GetDeviceInfoReply { base : base::mk_reply(reply) } }
pub fn GetDeviceInfo<'r> (c : &'r Connection,
                      screen : u32) -> GetDeviceInfoCookie<'r> {
  unsafe {
    let cookie = xcb_xf86dri_get_device_info(c.get_raw_conn(),
        screen as u32); //1
    GetDeviceInfoCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetDeviceInfoUnchecked<'r> (c : &'r Connection,
                               screen : u32) -> GetDeviceInfoCookie<'r> {
  unsafe {
    let cookie = xcb_xf86dri_get_device_info_unchecked(c.get_raw_conn(),
        screen as u32); //1
    GetDeviceInfoCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetDeviceInfoReply {
  pub fn framebuffer_handle_low(&mut self) -> u32 {
    unsafe { accessor!(framebuffer_handle_low -> u32, (*self.base.reply)) }
  }

  pub fn framebuffer_handle_high(&mut self) -> u32 {
    unsafe { accessor!(framebuffer_handle_high -> u32, (*self.base.reply)) }
  }

  pub fn framebuffer_origin_offset(&mut self) -> u32 {
    unsafe { accessor!(framebuffer_origin_offset -> u32, (*self.base.reply)) }
  }

  pub fn framebuffer_size(&mut self) -> u32 {
    unsafe { accessor!(framebuffer_size -> u32, (*self.base.reply)) }
  }

  pub fn framebuffer_stride(&mut self) -> u32 {
    unsafe { accessor!(framebuffer_stride -> u32, (*self.base.reply)) }
  }

  pub fn device_private(&mut self) -> Vec<u32> {
    unsafe { accessor!(u32, xcb_xf86dri_get_device_info_device_private_length, xcb_xf86dri_get_device_info_device_private, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetDeviceInfoCookie<'s>, mk_reply_get_device_info_reply, GetDeviceInfoReply, xcb_xf86dri_get_device_info_reply);

pub fn AuthConnection<'r> (c : &'r Connection,
                       screen : u32,
                       magic : u32) -> AuthConnectionCookie<'r> {
  unsafe {
    let cookie = xcb_xf86dri_auth_connection(c.get_raw_conn(),
        screen as u32, //1
        magic as u32); //2
    AuthConnectionCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn AuthConnectionUnchecked<'r> (c : &'r Connection,
                                screen : u32,
                                magic : u32) -> AuthConnectionCookie<'r> {
  unsafe {
    let cookie = xcb_xf86dri_auth_connection_unchecked(c.get_raw_conn(),
        screen as u32, //1
        magic as u32); //2
    AuthConnectionCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl AuthConnectionReply {
  pub fn authenticated(&mut self) -> u32 {
    unsafe { accessor!(authenticated -> u32, (*self.base.reply)) }
  }

}
impl_reply_cookie!(AuthConnectionCookie<'s>, mk_reply_auth_connection_reply, AuthConnectionReply, xcb_xf86dri_auth_connection_reply);


