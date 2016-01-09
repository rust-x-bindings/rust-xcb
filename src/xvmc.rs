/*
 * This file generated automatically from xvmc.xml by r_client.py.
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
use ffi::xvmc::*;
use std::option::Option;
use std::iter::Iterator;

use xproto;
use shm;
use xv;
pub type Context = context;

pub type ContextIterator = context_iterator;

pub type SurfaceIterator = surface_iterator;

pub type SubpictureIterator = subpicture_iterator;

pub type SurfaceInfoIterator = surface_info_iterator;

pub struct  QueryVersionCookie<'s> { pub base : base::Cookie<'s, query_version_cookie> }

/** Opcode for xcb_xvmc_query_version. */
pub static XCB_XVMC_QUERY_VERSION : u8 = 0;
pub struct QueryVersionReply { base:  base::Reply<query_version_reply> }
fn mk_reply_query_version_reply(reply:*mut query_version_reply) -> QueryVersionReply { QueryVersionReply { base : base::mk_reply(reply) } }
pub struct  ListSurfaceTypesCookie<'s> { pub base : base::Cookie<'s, list_surface_types_cookie> }

/** Opcode for xcb_xvmc_list_surface_types. */
pub static XCB_XVMC_LIST_SURFACE_TYPES : u8 = 1;
pub struct  CreateContextCookie<'s> { pub base : base::Cookie<'s, create_context_cookie> }

/** Opcode for xcb_xvmc_create_context. */
pub static XCB_XVMC_CREATE_CONTEXT : u8 = 2;
/** Opcode for xcb_xvmc_destroy_context. */
pub static XCB_XVMC_DESTROY_CONTEXT : u8 = 3;
pub struct  CreateSurfaceCookie<'s> { pub base : base::Cookie<'s, create_surface_cookie> }

/** Opcode for xcb_xvmc_create_surface. */
pub static XCB_XVMC_CREATE_SURFACE : u8 = 4;
/** Opcode for xcb_xvmc_destroy_surface. */
pub static XCB_XVMC_DESTROY_SURFACE : u8 = 5;
pub struct  CreateSubpictureCookie<'s> { pub base : base::Cookie<'s, create_subpicture_cookie> }

/** Opcode for xcb_xvmc_create_subpicture. */
pub static XCB_XVMC_CREATE_SUBPICTURE : u8 = 6;
/** Opcode for xcb_xvmc_destroy_subpicture. */
pub static XCB_XVMC_DESTROY_SUBPICTURE : u8 = 7;
pub struct  ListSubpictureTypesCookie<'s> { pub base : base::Cookie<'s, list_subpicture_types_cookie> }

/** Opcode for xcb_xvmc_list_subpicture_types. */
pub static XCB_XVMC_LIST_SUBPICTURE_TYPES : u8 = 8;

impl Iterator for ContextIterator {
    type Item = Context;
    fn next(&mut self) -> Option<Context> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut context_iterator = mem::transmute(self);
            let data = (*iter).data;
            xcb_xvmc_context_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub type Surface = surface;


impl Iterator for SurfaceIterator {
    type Item = Surface;
    fn next(&mut self) -> Option<Surface> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut surface_iterator = mem::transmute(self);
            let data = (*iter).data;
            xcb_xvmc_surface_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub type Subpicture = subpicture;


impl Iterator for SubpictureIterator {
    type Item = Subpicture;
    fn next(&mut self) -> Option<Subpicture> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut subpicture_iterator = mem::transmute(self);
            let data = (*iter).data;
            xcb_xvmc_subpicture_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct SurfaceInfo {pub base : base::Struct<surface_info> }


impl SurfaceInfo {
  pub fn id(&mut self) -> Surface {
    unsafe { accessor!(id -> Surface, self.base.strct) }
  }

  pub fn chroma_format(&mut self) -> u16 {
    unsafe { accessor!(chroma_format -> u16, self.base.strct) }
  }

  pub fn pad0(&mut self) -> u16 {
    unsafe { accessor!(pad0 -> u16, self.base.strct) }
  }

  pub fn max_width(&mut self) -> u16 {
    unsafe { accessor!(max_width -> u16, self.base.strct) }
  }

  pub fn max_height(&mut self) -> u16 {
    unsafe { accessor!(max_height -> u16, self.base.strct) }
  }

  pub fn subpicture_max_width(&mut self) -> u16 {
    unsafe { accessor!(subpicture_max_width -> u16, self.base.strct) }
  }

  pub fn subpicture_max_height(&mut self) -> u16 {
    unsafe { accessor!(subpicture_max_height -> u16, self.base.strct) }
  }

  pub fn mc_type(&mut self) -> u32 {
    unsafe { accessor!(mc_type -> u32, self.base.strct) }
  }

  pub fn flags(&mut self) -> u32 {
    unsafe { accessor!(flags -> u32, self.base.strct) }
  }

}

impl Iterator for SurfaceInfoIterator {
    type Item = SurfaceInfo;
    fn next(&mut self) -> Option<SurfaceInfo> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut surface_info_iterator = mem::transmute(self);
            let data = (*iter).data;
            xcb_xvmc_surface_info_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub fn QueryVersion<'r> (c : &'r Connection) -> QueryVersionCookie<'r> {
  unsafe {
    let cookie = xcb_xvmc_query_version(c.get_raw_conn());
    QueryVersionCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn QueryVersionUnchecked<'r> (c : &'r Connection) -> QueryVersionCookie<'r> {
  unsafe {
    let cookie = xcb_xvmc_query_version_unchecked(c.get_raw_conn());
    QueryVersionCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl QueryVersionReply {
  pub fn major(&mut self) -> u32 {
    unsafe { accessor!(major -> u32, (*self.base.reply)) }
  }

  pub fn minor(&mut self) -> u32 {
    unsafe { accessor!(minor -> u32, (*self.base.reply)) }
  }

}
impl_reply_cookie!(QueryVersionCookie<'s>, mk_reply_query_version_reply, QueryVersionReply, xcb_xvmc_query_version_reply);

pub struct ListSurfaceTypesReply { base:  base::Reply<list_surface_types_reply> }
fn mk_reply_list_surface_types_reply(reply:*mut list_surface_types_reply) -> ListSurfaceTypesReply { ListSurfaceTypesReply { base : base::mk_reply(reply) } }
pub fn ListSurfaceTypes<'r> (c : &'r Connection,
                         port_id : xv::Port) -> ListSurfaceTypesCookie<'r> {
  unsafe {
    let cookie = xcb_xvmc_list_surface_types(c.get_raw_conn(),
        port_id as ffi::xv::port); //1
    ListSurfaceTypesCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn ListSurfaceTypesUnchecked<'r> (c : &'r Connection,
                                  port_id : xv::Port) -> ListSurfaceTypesCookie<'r> {
  unsafe {
    let cookie = xcb_xvmc_list_surface_types_unchecked(c.get_raw_conn(),
        port_id as ffi::xv::port); //1
    ListSurfaceTypesCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl ListSurfaceTypesReply {
  pub fn surfaces(&mut self) -> SurfaceInfoIterator {
    unsafe { accessor!(SurfaceInfoIterator, xcb_xvmc_list_surface_types_surfaces_iterator, (*self.base.reply)) }
  }

}
impl_reply_cookie!(ListSurfaceTypesCookie<'s>, mk_reply_list_surface_types_reply, ListSurfaceTypesReply, xcb_xvmc_list_surface_types_reply);

pub struct CreateContextReply { base:  base::Reply<create_context_reply> }
fn mk_reply_create_context_reply(reply:*mut create_context_reply) -> CreateContextReply { CreateContextReply { base : base::mk_reply(reply) } }
pub fn CreateContext<'r> (c : &'r Connection,
                      context_id : Context,
                      port_id : xv::Port,
                      surface_id : Surface,
                      width : u16,
                      height : u16,
                      flags : u32) -> CreateContextCookie<'r> {
  unsafe {
    let cookie = xcb_xvmc_create_context(c.get_raw_conn(),
        context_id as context, //1
        port_id as ffi::xv::port, //2
        surface_id as surface, //3
        width as u16, //4
        height as u16, //5
        flags as u32); //6
    CreateContextCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn CreateContextUnchecked<'r> (c : &'r Connection,
                               context_id : Context,
                               port_id : xv::Port,
                               surface_id : Surface,
                               width : u16,
                               height : u16,
                               flags : u32) -> CreateContextCookie<'r> {
  unsafe {
    let cookie = xcb_xvmc_create_context_unchecked(c.get_raw_conn(),
        context_id as context, //1
        port_id as ffi::xv::port, //2
        surface_id as surface, //3
        width as u16, //4
        height as u16, //5
        flags as u32); //6
    CreateContextCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl CreateContextReply {
  pub fn width_actual(&mut self) -> u16 {
    unsafe { accessor!(width_actual -> u16, (*self.base.reply)) }
  }

  pub fn height_actual(&mut self) -> u16 {
    unsafe { accessor!(height_actual -> u16, (*self.base.reply)) }
  }

  pub fn flags_return(&mut self) -> u32 {
    unsafe { accessor!(flags_return -> u32, (*self.base.reply)) }
  }

  pub fn priv_data(&mut self) -> Vec<u32> {
    unsafe { accessor!(u32, xcb_xvmc_create_context_priv_data_length, xcb_xvmc_create_context_priv_data, (*self.base.reply)) }
  }

}
impl_reply_cookie!(CreateContextCookie<'s>, mk_reply_create_context_reply, CreateContextReply, xcb_xvmc_create_context_reply);

pub fn DestroyContextChecked<'r> (c : &'r Connection,
                              context_id : Context) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xvmc_destroy_context_checked(c.get_raw_conn(),
        context_id as context); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn DestroyContext<'r> (c : &'r Connection,
                       context_id : Context) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xvmc_destroy_context(c.get_raw_conn(),
        context_id as context); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub struct CreateSurfaceReply { base:  base::Reply<create_surface_reply> }
fn mk_reply_create_surface_reply(reply:*mut create_surface_reply) -> CreateSurfaceReply { CreateSurfaceReply { base : base::mk_reply(reply) } }
pub fn CreateSurface<'r> (c : &'r Connection,
                      surface_id : Surface,
                      context_id : Context) -> CreateSurfaceCookie<'r> {
  unsafe {
    let cookie = xcb_xvmc_create_surface(c.get_raw_conn(),
        surface_id as surface, //1
        context_id as context); //2
    CreateSurfaceCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn CreateSurfaceUnchecked<'r> (c : &'r Connection,
                               surface_id : Surface,
                               context_id : Context) -> CreateSurfaceCookie<'r> {
  unsafe {
    let cookie = xcb_xvmc_create_surface_unchecked(c.get_raw_conn(),
        surface_id as surface, //1
        context_id as context); //2
    CreateSurfaceCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl CreateSurfaceReply {
  pub fn priv_data(&mut self) -> Vec<u32> {
    unsafe { accessor!(u32, xcb_xvmc_create_surface_priv_data_length, xcb_xvmc_create_surface_priv_data, (*self.base.reply)) }
  }

}
impl_reply_cookie!(CreateSurfaceCookie<'s>, mk_reply_create_surface_reply, CreateSurfaceReply, xcb_xvmc_create_surface_reply);

pub fn DestroySurfaceChecked<'r> (c : &'r Connection,
                              surface_id : Surface) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xvmc_destroy_surface_checked(c.get_raw_conn(),
        surface_id as surface); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn DestroySurface<'r> (c : &'r Connection,
                       surface_id : Surface) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xvmc_destroy_surface(c.get_raw_conn(),
        surface_id as surface); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub struct CreateSubpictureReply { base:  base::Reply<create_subpicture_reply> }
fn mk_reply_create_subpicture_reply(reply:*mut create_subpicture_reply) -> CreateSubpictureReply { CreateSubpictureReply { base : base::mk_reply(reply) } }
pub fn CreateSubpicture<'r> (c : &'r Connection,
                         subpicture_id : Subpicture,
                         context : Context,
                         xvimage_id : u32,
                         width : u16,
                         height : u16) -> CreateSubpictureCookie<'r> {
  unsafe {
    let cookie = xcb_xvmc_create_subpicture(c.get_raw_conn(),
        subpicture_id as subpicture, //1
        context as context, //2
        xvimage_id as u32, //3
        width as u16, //4
        height as u16); //5
    CreateSubpictureCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn CreateSubpictureUnchecked<'r> (c : &'r Connection,
                                  subpicture_id : Subpicture,
                                  context : Context,
                                  xvimage_id : u32,
                                  width : u16,
                                  height : u16) -> CreateSubpictureCookie<'r> {
  unsafe {
    let cookie = xcb_xvmc_create_subpicture_unchecked(c.get_raw_conn(),
        subpicture_id as subpicture, //1
        context as context, //2
        xvimage_id as u32, //3
        width as u16, //4
        height as u16); //5
    CreateSubpictureCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl CreateSubpictureReply {
  pub fn width_actual(&mut self) -> u16 {
    unsafe { accessor!(width_actual -> u16, (*self.base.reply)) }
  }

  pub fn height_actual(&mut self) -> u16 {
    unsafe { accessor!(height_actual -> u16, (*self.base.reply)) }
  }

  pub fn num_palette_entries(&mut self) -> u16 {
    unsafe { accessor!(num_palette_entries -> u16, (*self.base.reply)) }
  }

  pub fn entry_bytes(&mut self) -> u16 {
    unsafe { accessor!(entry_bytes -> u16, (*self.base.reply)) }
  }

  pub fn component_order(&self) -> Vec<u8> {
    unsafe { ((*self.base.reply).component_order).to_vec() }
  }

  pub fn priv_data(&mut self) -> Vec<u32> {
    unsafe { accessor!(u32, xcb_xvmc_create_subpicture_priv_data_length, xcb_xvmc_create_subpicture_priv_data, (*self.base.reply)) }
  }

}
impl_reply_cookie!(CreateSubpictureCookie<'s>, mk_reply_create_subpicture_reply, CreateSubpictureReply, xcb_xvmc_create_subpicture_reply);

pub fn DestroySubpictureChecked<'r> (c : &'r Connection,
                                 subpicture_id : Subpicture) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xvmc_destroy_subpicture_checked(c.get_raw_conn(),
        subpicture_id as subpicture); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn DestroySubpicture<'r> (c : &'r Connection,
                          subpicture_id : Subpicture) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_xvmc_destroy_subpicture(c.get_raw_conn(),
        subpicture_id as subpicture); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub struct ListSubpictureTypesReply { base:  base::Reply<list_subpicture_types_reply> }
fn mk_reply_list_subpicture_types_reply(reply:*mut list_subpicture_types_reply) -> ListSubpictureTypesReply { ListSubpictureTypesReply { base : base::mk_reply(reply) } }
pub fn ListSubpictureTypes<'r> (c : &'r Connection,
                            port_id : xv::Port,
                            surface_id : Surface) -> ListSubpictureTypesCookie<'r> {
  unsafe {
    let cookie = xcb_xvmc_list_subpicture_types(c.get_raw_conn(),
        port_id as ffi::xv::port, //1
        surface_id as surface); //2
    ListSubpictureTypesCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn ListSubpictureTypesUnchecked<'r> (c : &'r Connection,
                                     port_id : xv::Port,
                                     surface_id : Surface) -> ListSubpictureTypesCookie<'r> {
  unsafe {
    let cookie = xcb_xvmc_list_subpicture_types_unchecked(c.get_raw_conn(),
        port_id as ffi::xv::port, //1
        surface_id as surface); //2
    ListSubpictureTypesCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl ListSubpictureTypesReply {
  pub fn types(&mut self) -> xv::ImageFormatInfoIterator {
    unsafe { accessor!(xv::ImageFormatInfoIterator, xcb_xvmc_list_subpicture_types_types_iterator, (*self.base.reply)) }
  }

}
impl_reply_cookie!(ListSubpictureTypesCookie<'s>, mk_reply_list_subpicture_types_reply, ListSubpictureTypesReply, xcb_xvmc_list_subpicture_types_reply);


