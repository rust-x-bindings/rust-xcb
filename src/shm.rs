//
// This file generated automatically from shm.xml by r_client.py.
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
use ffi::shm::*;
use std::option::Option;
use std::iter::Iterator;

use xproto;
pub type Seg = xcb_shm_seg_t;

pub type SegIterator = xcb_shm_seg_iterator_t;

/// Opcode for xcb_shm_completion.
pub const XCB_SHM_COMPLETION : u8 = 0;
pub struct CompletionEvent {pub base : base::Event<xcb_shm_completion_event_t>}
/// Opcode for xcb_shm_bad_seg.
pub const XCB_SHM_BAD_SEG : u8 = 0;
pub struct BadSegError { pub base : base::Error<xcb_shm_bad_seg_error_t> }
pub struct  QueryVersionCookie<'s> { pub base : base::Cookie<'s, xcb_shm_query_version_cookie_t> }

/// Opcode for xcb_shm_query_version.
pub const XCB_SHM_QUERY_VERSION : u8 = 0;
pub struct QueryVersionReply { base:  base::Reply<xcb_shm_query_version_reply_t> }
fn mk_reply_xcb_shm_query_version_reply_t(reply:*mut xcb_shm_query_version_reply_t) -> QueryVersionReply { QueryVersionReply { base : base::mk_reply(reply) } }
/// Opcode for xcb_shm_attach.
pub const XCB_SHM_ATTACH : u8 = 1;
/// Opcode for xcb_shm_detach.
pub const XCB_SHM_DETACH : u8 = 2;
/// Opcode for xcb_shm_put_image.
pub const XCB_SHM_PUT_IMAGE : u8 = 3;
pub struct  GetImageCookie<'s> { pub base : base::Cookie<'s, xcb_shm_get_image_cookie_t> }

/// Opcode for xcb_shm_get_image.
pub const XCB_SHM_GET_IMAGE : u8 = 4;
pub struct GetImageReply { base:  base::Reply<xcb_shm_get_image_reply_t> }
fn mk_reply_xcb_shm_get_image_reply_t(reply:*mut xcb_shm_get_image_reply_t) -> GetImageReply { GetImageReply { base : base::mk_reply(reply) } }
/// Opcode for xcb_shm_create_pixmap.
pub const XCB_SHM_CREATE_PIXMAP : u8 = 5;

impl Iterator for SegIterator {
    type Item = Seg;
    fn next(&mut self) -> Option<Seg> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_shm_seg_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_shm_seg_next(iter);
            Some(mem::transmute(*data))
        }
    }
}


impl CompletionEvent {
  pub fn drawable(&mut self) -> xproto::Drawable {
    unsafe { accessor!(drawable -> xproto::Drawable, (*self.base.event)) }
  }

  pub fn minor_event(&mut self) -> u16 {
    unsafe { accessor!(minor_event -> u16, (*self.base.event)) }
  }

  pub fn major_event(&mut self) -> u8 {
    unsafe { accessor!(major_event -> u8, (*self.base.event)) }
  }

  pub fn shmseg(&mut self) -> Seg {
    unsafe { accessor!(shmseg -> Seg, (*self.base.event)) }
  }

  pub fn offset(&mut self) -> u32 {
    unsafe { accessor!(offset -> u32, (*self.base.event)) }
  }

  pub fn new(drawable : xproto::Drawable,
         minor_event : u16,
         major_event : u8,
         shmseg : Seg,
         offset : u32) -> CompletionEvent {
    unsafe {
      let raw = malloc(32 as size_t) as *mut xcb_shm_completion_event_t;
      (*raw).drawable = drawable;
      (*raw).minor_event = minor_event;
      (*raw).major_event = major_event;
      (*raw).shmseg = shmseg;
      (*raw).offset = offset;
      CompletionEvent { base : Event { event : raw as *mut xcb_shm_completion_event_t }}
    }
  }
}
pub fn QueryVersion<'r> (c : &'r Connection) -> QueryVersionCookie<'r> {
  unsafe {
    let cookie = xcb_shm_query_version(c.get_raw_conn());
    QueryVersionCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn QueryVersionUnchecked<'r> (c : &'r Connection) -> QueryVersionCookie<'r> {
  unsafe {
    let cookie = xcb_shm_query_version_unchecked(c.get_raw_conn());
    QueryVersionCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl QueryVersionReply {
  pub fn shared_pixmaps(&mut self) -> u8 {
    unsafe { accessor!(shared_pixmaps -> u8, (*self.base.reply)) }
  }

  pub fn major_version(&mut self) -> u16 {
    unsafe { accessor!(major_version -> u16, (*self.base.reply)) }
  }

  pub fn minor_version(&mut self) -> u16 {
    unsafe { accessor!(minor_version -> u16, (*self.base.reply)) }
  }

  pub fn uid(&mut self) -> u16 {
    unsafe { accessor!(uid -> u16, (*self.base.reply)) }
  }

  pub fn gid(&mut self) -> u16 {
    unsafe { accessor!(gid -> u16, (*self.base.reply)) }
  }

  pub fn pixmap_format(&mut self) -> u8 {
    unsafe { accessor!(pixmap_format -> u8, (*self.base.reply)) }
  }

}
impl_reply_cookie!(QueryVersionCookie<'s>, mk_reply_xcb_shm_query_version_reply_t, QueryVersionReply, xcb_shm_query_version_reply);

pub fn AttachChecked<'r> (c : &'r Connection,
                      shmseg : Seg,
                      shmid : u32,
                      read_only : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_shm_attach_checked(c.get_raw_conn(),
        shmseg as xcb_shm_seg_t, //1
        shmid as u32, //2
        read_only as u8); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn Attach<'r> (c : &'r Connection,
               shmseg : Seg,
               shmid : u32,
               read_only : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_shm_attach(c.get_raw_conn(),
        shmseg as xcb_shm_seg_t, //1
        shmid as u32, //2
        read_only as u8); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn DetachChecked<'r> (c : &'r Connection,
                      shmseg : Seg) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_shm_detach_checked(c.get_raw_conn(),
        shmseg as xcb_shm_seg_t); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn Detach<'r> (c : &'r Connection,
               shmseg : Seg) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_shm_detach(c.get_raw_conn(),
        shmseg as xcb_shm_seg_t); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn PutImageChecked<'r> (c : &'r Connection,
                        drawable : xproto::Drawable,
                        gc : xproto::Gcontext,
                        total_width : u16,
                        total_height : u16,
                        src_x : u16,
                        src_y : u16,
                        src_width : u16,
                        src_height : u16,
                        dst_x : i16,
                        dst_y : i16,
                        depth : u8,
                        format : u8,
                        send_event : u8,
                        shmseg : Seg,
                        offset : u32) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_shm_put_image_checked(c.get_raw_conn(),
        drawable as ffi::xproto::xcb_drawable_t, //1
        gc as ffi::xproto::xcb_gcontext_t, //2
        total_width as u16, //3
        total_height as u16, //4
        src_x as u16, //5
        src_y as u16, //6
        src_width as u16, //7
        src_height as u16, //8
        dst_x as i16, //9
        dst_y as i16, //10
        depth as u8, //11
        format as u8, //12
        send_event as u8, //13
        shmseg as xcb_shm_seg_t, //14
        offset as u32); //15
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn PutImage<'r> (c : &'r Connection,
                 drawable : xproto::Drawable,
                 gc : xproto::Gcontext,
                 total_width : u16,
                 total_height : u16,
                 src_x : u16,
                 src_y : u16,
                 src_width : u16,
                 src_height : u16,
                 dst_x : i16,
                 dst_y : i16,
                 depth : u8,
                 format : u8,
                 send_event : u8,
                 shmseg : Seg,
                 offset : u32) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_shm_put_image(c.get_raw_conn(),
        drawable as ffi::xproto::xcb_drawable_t, //1
        gc as ffi::xproto::xcb_gcontext_t, //2
        total_width as u16, //3
        total_height as u16, //4
        src_x as u16, //5
        src_y as u16, //6
        src_width as u16, //7
        src_height as u16, //8
        dst_x as i16, //9
        dst_y as i16, //10
        depth as u8, //11
        format as u8, //12
        send_event as u8, //13
        shmseg as xcb_shm_seg_t, //14
        offset as u32); //15
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetImage<'r> (c : &'r Connection,
                 drawable : xproto::Drawable,
                 x : i16,
                 y : i16,
                 width : u16,
                 height : u16,
                 plane_mask : u32,
                 format : u8,
                 shmseg : Seg,
                 offset : u32) -> GetImageCookie<'r> {
  unsafe {
    let cookie = xcb_shm_get_image(c.get_raw_conn(),
        drawable as ffi::xproto::xcb_drawable_t, //1
        x as i16, //2
        y as i16, //3
        width as u16, //4
        height as u16, //5
        plane_mask as u32, //6
        format as u8, //7
        shmseg as xcb_shm_seg_t, //8
        offset as u32); //9
    GetImageCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetImageUnchecked<'r> (c : &'r Connection,
                          drawable : xproto::Drawable,
                          x : i16,
                          y : i16,
                          width : u16,
                          height : u16,
                          plane_mask : u32,
                          format : u8,
                          shmseg : Seg,
                          offset : u32) -> GetImageCookie<'r> {
  unsafe {
    let cookie = xcb_shm_get_image_unchecked(c.get_raw_conn(),
        drawable as ffi::xproto::xcb_drawable_t, //1
        x as i16, //2
        y as i16, //3
        width as u16, //4
        height as u16, //5
        plane_mask as u32, //6
        format as u8, //7
        shmseg as xcb_shm_seg_t, //8
        offset as u32); //9
    GetImageCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetImageReply {
  pub fn depth(&mut self) -> u8 {
    unsafe { accessor!(depth -> u8, (*self.base.reply)) }
  }

  pub fn visual(&mut self) -> xproto::Visualid {
    unsafe { accessor!(visual -> xproto::Visualid, (*self.base.reply)) }
  }

  pub fn size(&mut self) -> u32 {
    unsafe { accessor!(size -> u32, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetImageCookie<'s>, mk_reply_xcb_shm_get_image_reply_t, GetImageReply, xcb_shm_get_image_reply);

pub fn CreatePixmapChecked<'r> (c : &'r Connection,
                            pid : xproto::Pixmap,
                            drawable : xproto::Drawable,
                            width : u16,
                            height : u16,
                            depth : u8,
                            shmseg : Seg,
                            offset : u32) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_shm_create_pixmap_checked(c.get_raw_conn(),
        pid as ffi::xproto::xcb_pixmap_t, //1
        drawable as ffi::xproto::xcb_drawable_t, //2
        width as u16, //3
        height as u16, //4
        depth as u8, //5
        shmseg as xcb_shm_seg_t, //6
        offset as u32); //7
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn CreatePixmap<'r> (c : &'r Connection,
                     pid : xproto::Pixmap,
                     drawable : xproto::Drawable,
                     width : u16,
                     height : u16,
                     depth : u8,
                     shmseg : Seg,
                     offset : u32) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_shm_create_pixmap(c.get_raw_conn(),
        pid as ffi::xproto::xcb_pixmap_t, //1
        drawable as ffi::xproto::xcb_drawable_t, //2
        width as u16, //3
        height as u16, //4
        depth as u8, //5
        shmseg as xcb_shm_seg_t, //6
        offset as u32); //7
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

