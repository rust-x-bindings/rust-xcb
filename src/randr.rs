/*
 * This file generated automatically from randr.xml by r_client.py.
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
use ffi::randr::*;
use std::option::Option;
use std::iter::Iterator;

use xproto;
use render;
pub type Mode = xcb_randr_mode_t;

pub type ModeIterator = xcb_randr_mode_iterator_t;

pub type CrtcIterator = xcb_randr_crtc_iterator_t;

pub type OutputIterator = xcb_randr_output_iterator_t;

/** Opcode for xcb_randr_bad_output. */
pub static XCB_RANDR_BAD_OUTPUT : u8 = 0;
pub struct BadOutputError { pub base : base::Error<xcb_randr_bad_output_error_t> }
/** Opcode for xcb_randr_bad_crtc. */
pub static XCB_RANDR_BAD_CRTC : u8 = 1;
pub struct BadCrtcError { pub base : base::Error<xcb_randr_bad_crtc_error_t> }
/** Opcode for xcb_randr_bad_mode. */
pub static XCB_RANDR_BAD_MODE : u8 = 2;
pub struct BadModeError { pub base : base::Error<xcb_randr_bad_mode_error_t> }

pub type xcb_randr_rotation_t = c_uint;//{
    pub static XCB_RANDR_ROTATION_ROTATE_0 : xcb_randr_rotation_t = 1;
    pub static XCB_RANDR_ROTATION_ROTATE_90 : xcb_randr_rotation_t = 2;
    pub static XCB_RANDR_ROTATION_ROTATE_180 : xcb_randr_rotation_t = 4;
    pub static XCB_RANDR_ROTATION_ROTATE_270 : xcb_randr_rotation_t = 8;
    pub static XCB_RANDR_ROTATION_REFLECT_X : xcb_randr_rotation_t = 16;
    pub static XCB_RANDR_ROTATION_REFLECT_Y : xcb_randr_rotation_t = 32;
//}
pub struct ScreenSize {pub base : base::Struct<xcb_randr_screen_size_t> }

pub type ScreenSizeIterator = xcb_randr_screen_size_iterator_t;

pub type RefreshRatesIterator = xcb_randr_refresh_rates_iterator_t;

pub struct  QueryVersionCookie<'s> { pub base : base::Cookie<'s, xcb_randr_query_version_cookie_t> }

/** Opcode for xcb_randr_query_version. */
pub static XCB_RANDR_QUERY_VERSION : u8 = 0;
pub struct QueryVersionReply { base:  base::Reply<xcb_randr_query_version_reply_t> }
fn mk_reply_xcb_randr_query_version_reply_t(reply:*mut xcb_randr_query_version_reply_t) -> QueryVersionReply { QueryVersionReply { base : base::mk_reply(reply) } }

pub type xcb_randr_set_config_t = c_uint;//{
    pub static XCB_RANDR_SET_CONFIG_SUCCESS : xcb_randr_set_config_t = 0;
    pub static XCB_RANDR_SET_CONFIG_INVALID_CONFIG_TIME : xcb_randr_set_config_t = 1;
    pub static XCB_RANDR_SET_CONFIG_INVALID_TIME : xcb_randr_set_config_t = 2;
    pub static XCB_RANDR_SET_CONFIG_FAILED : xcb_randr_set_config_t = 3;
//}
pub struct  SetScreenConfigCookie<'s> { pub base : base::Cookie<'s, xcb_randr_set_screen_config_cookie_t> }

/** Opcode for xcb_randr_set_screen_config. */
pub static XCB_RANDR_SET_SCREEN_CONFIG : u8 = 2;
pub struct SetScreenConfigReply { base:  base::Reply<xcb_randr_set_screen_config_reply_t> }
fn mk_reply_xcb_randr_set_screen_config_reply_t(reply:*mut xcb_randr_set_screen_config_reply_t) -> SetScreenConfigReply { SetScreenConfigReply { base : base::mk_reply(reply) } }

pub type xcb_randr_notify_mask_t = c_uint;//{
    pub static XCB_RANDR_NOTIFY_MASK_SCREEN_CHANGE : xcb_randr_notify_mask_t = 1;
    pub static XCB_RANDR_NOTIFY_MASK_CRTC_CHANGE : xcb_randr_notify_mask_t = 2;
    pub static XCB_RANDR_NOTIFY_MASK_OUTPUT_CHANGE : xcb_randr_notify_mask_t = 4;
    pub static XCB_RANDR_NOTIFY_MASK_OUTPUT_PROPERTY : xcb_randr_notify_mask_t = 8;
//}
/** Opcode for xcb_randr_select_input. */
pub static XCB_RANDR_SELECT_INPUT : u8 = 4;
pub struct  GetScreenInfoCookie<'s> { pub base : base::Cookie<'s, xcb_randr_get_screen_info_cookie_t> }

/** Opcode for xcb_randr_get_screen_info. */
pub static XCB_RANDR_GET_SCREEN_INFO : u8 = 5;
pub struct  GetScreenSizeRangeCookie<'s> { pub base : base::Cookie<'s, xcb_randr_get_screen_size_range_cookie_t> }

/** Opcode for xcb_randr_get_screen_size_range. */
pub static XCB_RANDR_GET_SCREEN_SIZE_RANGE : u8 = 6;
pub struct GetScreenSizeRangeReply { base:  base::Reply<xcb_randr_get_screen_size_range_reply_t> }
fn mk_reply_xcb_randr_get_screen_size_range_reply_t(reply:*mut xcb_randr_get_screen_size_range_reply_t) -> GetScreenSizeRangeReply { GetScreenSizeRangeReply { base : base::mk_reply(reply) } }
/** Opcode for xcb_randr_set_screen_size. */
pub static XCB_RANDR_SET_SCREEN_SIZE : u8 = 7;

pub type xcb_randr_mode_flag_t = c_uint;//{
    pub static XCB_RANDR_MODE_FLAG_HSYNC_POSITIVE : xcb_randr_mode_flag_t = 1;
    pub static XCB_RANDR_MODE_FLAG_HSYNC_NEGATIVE : xcb_randr_mode_flag_t = 2;
    pub static XCB_RANDR_MODE_FLAG_VSYNC_POSITIVE : xcb_randr_mode_flag_t = 4;
    pub static XCB_RANDR_MODE_FLAG_VSYNC_NEGATIVE : xcb_randr_mode_flag_t = 8;
    pub static XCB_RANDR_MODE_FLAG_INTERLACE : xcb_randr_mode_flag_t = 16;
    pub static XCB_RANDR_MODE_FLAG_DOUBLE_SCAN : xcb_randr_mode_flag_t = 32;
    pub static XCB_RANDR_MODE_FLAG_CSYNC : xcb_randr_mode_flag_t = 64;
    pub static XCB_RANDR_MODE_FLAG_CSYNC_POSITIVE : xcb_randr_mode_flag_t = 128;
    pub static XCB_RANDR_MODE_FLAG_CSYNC_NEGATIVE : xcb_randr_mode_flag_t = 256;
    pub static XCB_RANDR_MODE_FLAG_HSKEW_PRESENT : xcb_randr_mode_flag_t = 512;
    pub static XCB_RANDR_MODE_FLAG_BCAST : xcb_randr_mode_flag_t = 1024;
    pub static XCB_RANDR_MODE_FLAG_PIXEL_MULTIPLEX : xcb_randr_mode_flag_t = 2048;
    pub static XCB_RANDR_MODE_FLAG_DOUBLE_CLOCK : xcb_randr_mode_flag_t = 4096;
    pub static XCB_RANDR_MODE_FLAG_HALVE_CLOCK : xcb_randr_mode_flag_t = 8192;
//}
pub struct ModeInfo {pub base : base::Struct<xcb_randr_mode_info_t> }

pub type ModeInfoIterator = xcb_randr_mode_info_iterator_t;

pub struct  GetScreenResourcesCookie<'s> { pub base : base::Cookie<'s, xcb_randr_get_screen_resources_cookie_t> }

/** Opcode for xcb_randr_get_screen_resources. */
pub static XCB_RANDR_GET_SCREEN_RESOURCES : u8 = 8;

pub type xcb_randr_connection_t = c_uint;//{
    pub static XCB_RANDR_CONNECTION_CONNECTED : xcb_randr_connection_t = 1;
    pub static XCB_RANDR_CONNECTION_DISCONNECTED : xcb_randr_connection_t = 2;
    pub static XCB_RANDR_CONNECTION_UNKNOWN : xcb_randr_connection_t = 3;
//}
pub struct  GetOutputInfoCookie<'s> { pub base : base::Cookie<'s, xcb_randr_get_output_info_cookie_t> }

/** Opcode for xcb_randr_get_output_info. */
pub static XCB_RANDR_GET_OUTPUT_INFO : u8 = 9;
pub struct  ListOutputPropertiesCookie<'s> { pub base : base::Cookie<'s, xcb_randr_list_output_properties_cookie_t> }

/** Opcode for xcb_randr_list_output_properties. */
pub static XCB_RANDR_LIST_OUTPUT_PROPERTIES : u8 = 10;
pub struct  QueryOutputPropertyCookie<'s> { pub base : base::Cookie<'s, xcb_randr_query_output_property_cookie_t> }

/** Opcode for xcb_randr_query_output_property. */
pub static XCB_RANDR_QUERY_OUTPUT_PROPERTY : u8 = 11;
/** Opcode for xcb_randr_configure_output_property. */
pub static XCB_RANDR_CONFIGURE_OUTPUT_PROPERTY : u8 = 12;
/** Opcode for xcb_randr_change_output_property. */
pub static XCB_RANDR_CHANGE_OUTPUT_PROPERTY : u8 = 13;
/** Opcode for xcb_randr_delete_output_property. */
pub static XCB_RANDR_DELETE_OUTPUT_PROPERTY : u8 = 14;
pub struct  GetOutputPropertyCookie<'s> { pub base : base::Cookie<'s, xcb_randr_get_output_property_cookie_t> }

/** Opcode for xcb_randr_get_output_property. */
pub static XCB_RANDR_GET_OUTPUT_PROPERTY : u8 = 15;
pub struct  CreateModeCookie<'s> { pub base : base::Cookie<'s, xcb_randr_create_mode_cookie_t> }

/** Opcode for xcb_randr_create_mode. */
pub static XCB_RANDR_CREATE_MODE : u8 = 16;
pub struct CreateModeReply { base:  base::Reply<xcb_randr_create_mode_reply_t> }
fn mk_reply_xcb_randr_create_mode_reply_t(reply:*mut xcb_randr_create_mode_reply_t) -> CreateModeReply { CreateModeReply { base : base::mk_reply(reply) } }
/** Opcode for xcb_randr_destroy_mode. */
pub static XCB_RANDR_DESTROY_MODE : u8 = 17;
/** Opcode for xcb_randr_add_output_mode. */
pub static XCB_RANDR_ADD_OUTPUT_MODE : u8 = 18;
/** Opcode for xcb_randr_delete_output_mode. */
pub static XCB_RANDR_DELETE_OUTPUT_MODE : u8 = 19;
pub struct  GetCrtcInfoCookie<'s> { pub base : base::Cookie<'s, xcb_randr_get_crtc_info_cookie_t> }

/** Opcode for xcb_randr_get_crtc_info. */
pub static XCB_RANDR_GET_CRTC_INFO : u8 = 20;
pub struct  SetCrtcConfigCookie<'s> { pub base : base::Cookie<'s, xcb_randr_set_crtc_config_cookie_t> }

/** Opcode for xcb_randr_set_crtc_config. */
pub static XCB_RANDR_SET_CRTC_CONFIG : u8 = 21;
pub struct SetCrtcConfigReply { base:  base::Reply<xcb_randr_set_crtc_config_reply_t> }
fn mk_reply_xcb_randr_set_crtc_config_reply_t(reply:*mut xcb_randr_set_crtc_config_reply_t) -> SetCrtcConfigReply { SetCrtcConfigReply { base : base::mk_reply(reply) } }
pub struct  GetCrtcGammaSizeCookie<'s> { pub base : base::Cookie<'s, xcb_randr_get_crtc_gamma_size_cookie_t> }

/** Opcode for xcb_randr_get_crtc_gamma_size. */
pub static XCB_RANDR_GET_CRTC_GAMMA_SIZE : u8 = 22;
pub struct GetCrtcGammaSizeReply { base:  base::Reply<xcb_randr_get_crtc_gamma_size_reply_t> }
fn mk_reply_xcb_randr_get_crtc_gamma_size_reply_t(reply:*mut xcb_randr_get_crtc_gamma_size_reply_t) -> GetCrtcGammaSizeReply { GetCrtcGammaSizeReply { base : base::mk_reply(reply) } }
pub struct  GetCrtcGammaCookie<'s> { pub base : base::Cookie<'s, xcb_randr_get_crtc_gamma_cookie_t> }

/** Opcode for xcb_randr_get_crtc_gamma. */
pub static XCB_RANDR_GET_CRTC_GAMMA : u8 = 23;
/** Opcode for xcb_randr_set_crtc_gamma. */
pub static XCB_RANDR_SET_CRTC_GAMMA : u8 = 24;
pub struct  GetScreenResourcesCurrentCookie<'s> { pub base : base::Cookie<'s, xcb_randr_get_screen_resources_current_cookie_t> }

/** Opcode for xcb_randr_get_screen_resources_current. */
pub static XCB_RANDR_GET_SCREEN_RESOURCES_CURRENT : u8 = 25;
/** Opcode for xcb_randr_set_crtc_transform. */
pub static XCB_RANDR_SET_CRTC_TRANSFORM : u8 = 26;
pub struct  GetCrtcTransformCookie<'s> { pub base : base::Cookie<'s, xcb_randr_get_crtc_transform_cookie_t> }

/** Opcode for xcb_randr_get_crtc_transform. */
pub static XCB_RANDR_GET_CRTC_TRANSFORM : u8 = 27;
pub struct  GetPanningCookie<'s> { pub base : base::Cookie<'s, xcb_randr_get_panning_cookie_t> }

/** Opcode for xcb_randr_get_panning. */
pub static XCB_RANDR_GET_PANNING : u8 = 28;
pub struct GetPanningReply { base:  base::Reply<xcb_randr_get_panning_reply_t> }
fn mk_reply_xcb_randr_get_panning_reply_t(reply:*mut xcb_randr_get_panning_reply_t) -> GetPanningReply { GetPanningReply { base : base::mk_reply(reply) } }
pub struct  SetPanningCookie<'s> { pub base : base::Cookie<'s, xcb_randr_set_panning_cookie_t> }

/** Opcode for xcb_randr_set_panning. */
pub static XCB_RANDR_SET_PANNING : u8 = 29;
pub struct SetPanningReply { base:  base::Reply<xcb_randr_set_panning_reply_t> }
fn mk_reply_xcb_randr_set_panning_reply_t(reply:*mut xcb_randr_set_panning_reply_t) -> SetPanningReply { SetPanningReply { base : base::mk_reply(reply) } }
/** Opcode for xcb_randr_set_output_primary. */
pub static XCB_RANDR_SET_OUTPUT_PRIMARY : u8 = 30;
pub struct  GetOutputPrimaryCookie<'s> { pub base : base::Cookie<'s, xcb_randr_get_output_primary_cookie_t> }

/** Opcode for xcb_randr_get_output_primary. */
pub static XCB_RANDR_GET_OUTPUT_PRIMARY : u8 = 31;
pub struct GetOutputPrimaryReply { base:  base::Reply<xcb_randr_get_output_primary_reply_t> }
fn mk_reply_xcb_randr_get_output_primary_reply_t(reply:*mut xcb_randr_get_output_primary_reply_t) -> GetOutputPrimaryReply { GetOutputPrimaryReply { base : base::mk_reply(reply) } }
/** Opcode for xcb_randr_screen_change_notify. */
pub static XCB_RANDR_SCREEN_CHANGE_NOTIFY : u8 = 0;
pub struct ScreenChangeNotifyEvent {pub base : base::Event<xcb_randr_screen_change_notify_event_t>}

pub type xcb_randr_notify_t = c_uint;//{
    pub static XCB_RANDR_NOTIFY_CRTC_CHANGE : xcb_randr_notify_t = 0;
    pub static XCB_RANDR_NOTIFY_OUTPUT_CHANGE : xcb_randr_notify_t = 1;
    pub static XCB_RANDR_NOTIFY_OUTPUT_PROPERTY : xcb_randr_notify_t = 2;
//}
pub struct CrtcChange {pub base : base::Struct<xcb_randr_crtc_change_t> }

pub type CrtcChangeIterator = xcb_randr_crtc_change_iterator_t;

pub type OutputChangeIterator = xcb_randr_output_change_iterator_t;

pub type OutputPropertyIterator = xcb_randr_output_property_iterator_t;

pub type NotifyDataIterator = xcb_randr_notify_data_iterator_t;

/** Opcode for xcb_randr_notify. */
pub static XCB_RANDR_NOTIFY : u8 = 1;
pub struct NotifyEvent {pub base : base::Event<xcb_randr_notify_event_t>}

impl Iterator for ModeIterator {
    type Item = Mode;
    fn next(&mut self) -> Option<Mode> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_randr_mode_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_randr_mode_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub type Crtc = xcb_randr_crtc_t;


impl Iterator for CrtcIterator {
    type Item = Crtc;
    fn next(&mut self) -> Option<Crtc> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_randr_crtc_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_randr_crtc_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub type Output = xcb_randr_output_t;


impl Iterator for OutputIterator {
    type Item = Output;
    fn next(&mut self) -> Option<Output> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_randr_output_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_randr_output_next(iter);
            Some(mem::transmute(*data))
        }
    }
}


impl ScreenSize {
  pub fn width(&mut self) -> u16 {
    unsafe { accessor!(width -> u16, self.base.strct) }
  }

  pub fn height(&mut self) -> u16 {
    unsafe { accessor!(height -> u16, self.base.strct) }
  }

  pub fn mwidth(&mut self) -> u16 {
    unsafe { accessor!(mwidth -> u16, self.base.strct) }
  }

  pub fn mheight(&mut self) -> u16 {
    unsafe { accessor!(mheight -> u16, self.base.strct) }
  }

}

impl Iterator for ScreenSizeIterator {
    type Item = ScreenSize;
    fn next(&mut self) -> Option<ScreenSize> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_randr_screen_size_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_randr_screen_size_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct RefreshRates {pub base : base::Struct<xcb_randr_refresh_rates_t> }


impl RefreshRates {
  pub fn rates(&mut self) -> Vec<u16> {
    unsafe { accessor!(u16, xcb_randr_refresh_rates_rates_length, xcb_randr_refresh_rates_rates, self.base.strct) }
  }

}

impl Iterator for RefreshRatesIterator {
    type Item = RefreshRates;
    fn next(&mut self) -> Option<RefreshRates> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_randr_refresh_rates_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_randr_refresh_rates_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub fn QueryVersion<'r> (c : &'r Connection,
                     major_version : u32,
                     minor_version : u32) -> QueryVersionCookie<'r> {
  unsafe {
    let cookie = xcb_randr_query_version(c.get_raw_conn(),
        major_version as u32, //1
        minor_version as u32); //2
    QueryVersionCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn QueryVersionUnchecked<'r> (c : &'r Connection,
                              major_version : u32,
                              minor_version : u32) -> QueryVersionCookie<'r> {
  unsafe {
    let cookie = xcb_randr_query_version_unchecked(c.get_raw_conn(),
        major_version as u32, //1
        minor_version as u32); //2
    QueryVersionCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl QueryVersionReply {
  pub fn major_version(&mut self) -> u32 {
    unsafe { accessor!(major_version -> u32, (*self.base.reply)) }
  }

  pub fn minor_version(&mut self) -> u32 {
    unsafe { accessor!(minor_version -> u32, (*self.base.reply)) }
  }

}
impl_reply_cookie!(QueryVersionCookie<'s>, mk_reply_xcb_randr_query_version_reply_t, QueryVersionReply, xcb_randr_query_version_reply);

pub fn SetScreenConfig<'r> (c : &'r Connection,
                        window : xproto::Window,
                        timestamp : xproto::Timestamp,
                        config_timestamp : xproto::Timestamp,
                        sizeID : u16,
                        rotation : u16,
                        rate : u16) -> SetScreenConfigCookie<'r> {
  unsafe {
    let cookie = xcb_randr_set_screen_config(c.get_raw_conn(),
        window as ffi::xproto::xcb_window_t, //1
        timestamp as ffi::xproto::xcb_timestamp_t, //2
        config_timestamp as ffi::xproto::xcb_timestamp_t, //3
        sizeID as u16, //4
        rotation as u16, //5
        rate as u16); //6
    SetScreenConfigCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn SetScreenConfigUnchecked<'r> (c : &'r Connection,
                                 window : xproto::Window,
                                 timestamp : xproto::Timestamp,
                                 config_timestamp : xproto::Timestamp,
                                 sizeID : u16,
                                 rotation : u16,
                                 rate : u16) -> SetScreenConfigCookie<'r> {
  unsafe {
    let cookie = xcb_randr_set_screen_config_unchecked(c.get_raw_conn(),
        window as ffi::xproto::xcb_window_t, //1
        timestamp as ffi::xproto::xcb_timestamp_t, //2
        config_timestamp as ffi::xproto::xcb_timestamp_t, //3
        sizeID as u16, //4
        rotation as u16, //5
        rate as u16); //6
    SetScreenConfigCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl SetScreenConfigReply {
  pub fn status(&mut self) -> u8 {
    unsafe { accessor!(status -> u8, (*self.base.reply)) }
  }

  pub fn new_timestamp(&mut self) -> xproto::Timestamp {
    unsafe { accessor!(new_timestamp -> xproto::Timestamp, (*self.base.reply)) }
  }

  pub fn config_timestamp(&mut self) -> xproto::Timestamp {
    unsafe { accessor!(config_timestamp -> xproto::Timestamp, (*self.base.reply)) }
  }

  pub fn root(&mut self) -> xproto::Window {
    unsafe { accessor!(root -> xproto::Window, (*self.base.reply)) }
  }

  pub fn subpixel_order(&mut self) -> u16 {
    unsafe { accessor!(subpixel_order -> u16, (*self.base.reply)) }
  }

}
impl_reply_cookie!(SetScreenConfigCookie<'s>, mk_reply_xcb_randr_set_screen_config_reply_t, SetScreenConfigReply, xcb_randr_set_screen_config_reply);

pub fn SelectInputChecked<'r> (c : &'r Connection,
                           window : xproto::Window,
                           enable : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_randr_select_input_checked(c.get_raw_conn(),
        window as ffi::xproto::xcb_window_t, //1
        enable as u16); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn SelectInput<'r> (c : &'r Connection,
                    window : xproto::Window,
                    enable : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_randr_select_input(c.get_raw_conn(),
        window as ffi::xproto::xcb_window_t, //1
        enable as u16); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub struct GetScreenInfoReply { base:  base::Reply<xcb_randr_get_screen_info_reply_t> }
fn mk_reply_xcb_randr_get_screen_info_reply_t(reply:*mut xcb_randr_get_screen_info_reply_t) -> GetScreenInfoReply { GetScreenInfoReply { base : base::mk_reply(reply) } }
pub fn GetScreenInfo<'r> (c : &'r Connection,
                      window : xproto::Window) -> GetScreenInfoCookie<'r> {
  unsafe {
    let cookie = xcb_randr_get_screen_info(c.get_raw_conn(),
        window as ffi::xproto::xcb_window_t); //1
    GetScreenInfoCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetScreenInfoUnchecked<'r> (c : &'r Connection,
                               window : xproto::Window) -> GetScreenInfoCookie<'r> {
  unsafe {
    let cookie = xcb_randr_get_screen_info_unchecked(c.get_raw_conn(),
        window as ffi::xproto::xcb_window_t); //1
    GetScreenInfoCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetScreenInfoReply {
  pub fn rotations(&mut self) -> u8 {
    unsafe { accessor!(rotations -> u8, (*self.base.reply)) }
  }

  pub fn root(&mut self) -> xproto::Window {
    unsafe { accessor!(root -> xproto::Window, (*self.base.reply)) }
  }

  pub fn timestamp(&mut self) -> xproto::Timestamp {
    unsafe { accessor!(timestamp -> xproto::Timestamp, (*self.base.reply)) }
  }

  pub fn config_timestamp(&mut self) -> xproto::Timestamp {
    unsafe { accessor!(config_timestamp -> xproto::Timestamp, (*self.base.reply)) }
  }

  pub fn sizeID(&mut self) -> u16 {
    unsafe { accessor!(sizeID -> u16, (*self.base.reply)) }
  }

  pub fn rotation(&mut self) -> u16 {
    unsafe { accessor!(rotation -> u16, (*self.base.reply)) }
  }

  pub fn rate(&mut self) -> u16 {
    unsafe { accessor!(rate -> u16, (*self.base.reply)) }
  }

  pub fn sizes(&mut self) -> ScreenSizeIterator {
    unsafe { accessor!(ScreenSizeIterator, xcb_randr_get_screen_info_sizes_iterator, (*self.base.reply)) }
  }

  pub fn rates(&mut self) -> RefreshRatesIterator {
    unsafe { accessor!(RefreshRatesIterator, xcb_randr_get_screen_info_rates_iterator, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetScreenInfoCookie<'s>, mk_reply_xcb_randr_get_screen_info_reply_t, GetScreenInfoReply, xcb_randr_get_screen_info_reply);

pub fn GetScreenSizeRange<'r> (c : &'r Connection,
                           window : xproto::Window) -> GetScreenSizeRangeCookie<'r> {
  unsafe {
    let cookie = xcb_randr_get_screen_size_range(c.get_raw_conn(),
        window as ffi::xproto::xcb_window_t); //1
    GetScreenSizeRangeCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetScreenSizeRangeUnchecked<'r> (c : &'r Connection,
                                    window : xproto::Window) -> GetScreenSizeRangeCookie<'r> {
  unsafe {
    let cookie = xcb_randr_get_screen_size_range_unchecked(c.get_raw_conn(),
        window as ffi::xproto::xcb_window_t); //1
    GetScreenSizeRangeCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetScreenSizeRangeReply {
  pub fn min_width(&mut self) -> u16 {
    unsafe { accessor!(min_width -> u16, (*self.base.reply)) }
  }

  pub fn min_height(&mut self) -> u16 {
    unsafe { accessor!(min_height -> u16, (*self.base.reply)) }
  }

  pub fn max_width(&mut self) -> u16 {
    unsafe { accessor!(max_width -> u16, (*self.base.reply)) }
  }

  pub fn max_height(&mut self) -> u16 {
    unsafe { accessor!(max_height -> u16, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetScreenSizeRangeCookie<'s>, mk_reply_xcb_randr_get_screen_size_range_reply_t, GetScreenSizeRangeReply, xcb_randr_get_screen_size_range_reply);

pub fn SetScreenSizeChecked<'r> (c : &'r Connection,
                             window : xproto::Window,
                             width : u16,
                             height : u16,
                             mm_width : u32,
                             mm_height : u32) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_randr_set_screen_size_checked(c.get_raw_conn(),
        window as ffi::xproto::xcb_window_t, //1
        width as u16, //2
        height as u16, //3
        mm_width as u32, //4
        mm_height as u32); //5
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn SetScreenSize<'r> (c : &'r Connection,
                      window : xproto::Window,
                      width : u16,
                      height : u16,
                      mm_width : u32,
                      mm_height : u32) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_randr_set_screen_size(c.get_raw_conn(),
        window as ffi::xproto::xcb_window_t, //1
        width as u16, //2
        height as u16, //3
        mm_width as u32, //4
        mm_height as u32); //5
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl ModeInfo {
  pub fn id(&mut self) -> u32 {
    unsafe { accessor!(id -> u32, self.base.strct) }
  }

  pub fn width(&mut self) -> u16 {
    unsafe { accessor!(width -> u16, self.base.strct) }
  }

  pub fn height(&mut self) -> u16 {
    unsafe { accessor!(height -> u16, self.base.strct) }
  }

  pub fn dot_clock(&mut self) -> u32 {
    unsafe { accessor!(dot_clock -> u32, self.base.strct) }
  }

  pub fn hsync_start(&mut self) -> u16 {
    unsafe { accessor!(hsync_start -> u16, self.base.strct) }
  }

  pub fn hsync_end(&mut self) -> u16 {
    unsafe { accessor!(hsync_end -> u16, self.base.strct) }
  }

  pub fn htotal(&mut self) -> u16 {
    unsafe { accessor!(htotal -> u16, self.base.strct) }
  }

  pub fn hskew(&mut self) -> u16 {
    unsafe { accessor!(hskew -> u16, self.base.strct) }
  }

  pub fn vsync_start(&mut self) -> u16 {
    unsafe { accessor!(vsync_start -> u16, self.base.strct) }
  }

  pub fn vsync_end(&mut self) -> u16 {
    unsafe { accessor!(vsync_end -> u16, self.base.strct) }
  }

  pub fn vtotal(&mut self) -> u16 {
    unsafe { accessor!(vtotal -> u16, self.base.strct) }
  }

  pub fn name_len(&mut self) -> u16 {
    unsafe { accessor!(name_len -> u16, self.base.strct) }
  }

  pub fn mode_flags(&mut self) -> u32 {
    unsafe { accessor!(mode_flags -> u32, self.base.strct) }
  }

}

impl Iterator for ModeInfoIterator {
    type Item = ModeInfo;
    fn next(&mut self) -> Option<ModeInfo> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_randr_mode_info_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_randr_mode_info_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct GetScreenResourcesReply { base:  base::Reply<xcb_randr_get_screen_resources_reply_t> }
fn mk_reply_xcb_randr_get_screen_resources_reply_t(reply:*mut xcb_randr_get_screen_resources_reply_t) -> GetScreenResourcesReply { GetScreenResourcesReply { base : base::mk_reply(reply) } }
pub fn GetScreenResources<'r> (c : &'r Connection,
                           window : xproto::Window) -> GetScreenResourcesCookie<'r> {
  unsafe {
    let cookie = xcb_randr_get_screen_resources(c.get_raw_conn(),
        window as ffi::xproto::xcb_window_t); //1
    GetScreenResourcesCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetScreenResourcesUnchecked<'r> (c : &'r Connection,
                                    window : xproto::Window) -> GetScreenResourcesCookie<'r> {
  unsafe {
    let cookie = xcb_randr_get_screen_resources_unchecked(c.get_raw_conn(),
        window as ffi::xproto::xcb_window_t); //1
    GetScreenResourcesCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetScreenResourcesReply {
  pub fn timestamp(&mut self) -> xproto::Timestamp {
    unsafe { accessor!(timestamp -> xproto::Timestamp, (*self.base.reply)) }
  }

  pub fn config_timestamp(&mut self) -> xproto::Timestamp {
    unsafe { accessor!(config_timestamp -> xproto::Timestamp, (*self.base.reply)) }
  }

  pub fn crtcs(&mut self) -> Vec<u32> {
    unsafe { accessor!(u32, xcb_randr_get_screen_resources_crtcs_length, xcb_randr_get_screen_resources_crtcs, (*self.base.reply)) }
  }

  pub fn outputs(&mut self) -> Vec<u32> {
    unsafe { accessor!(u32, xcb_randr_get_screen_resources_outputs_length, xcb_randr_get_screen_resources_outputs, (*self.base.reply)) }
  }

  pub fn modes(&mut self) -> ModeInfoIterator {
    unsafe { accessor!(ModeInfoIterator, xcb_randr_get_screen_resources_modes_iterator, (*self.base.reply)) }
  }

  pub fn names(&mut self) -> Vec<u8> {
    unsafe { accessor!(u8, xcb_randr_get_screen_resources_names_length, xcb_randr_get_screen_resources_names, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetScreenResourcesCookie<'s>, mk_reply_xcb_randr_get_screen_resources_reply_t, GetScreenResourcesReply, xcb_randr_get_screen_resources_reply);

pub struct GetOutputInfoReply { base:  base::Reply<xcb_randr_get_output_info_reply_t> }
fn mk_reply_xcb_randr_get_output_info_reply_t(reply:*mut xcb_randr_get_output_info_reply_t) -> GetOutputInfoReply { GetOutputInfoReply { base : base::mk_reply(reply) } }
pub fn GetOutputInfo<'r> (c : &'r Connection,
                      output : Output,
                      config_timestamp : xproto::Timestamp) -> GetOutputInfoCookie<'r> {
  unsafe {
    let cookie = xcb_randr_get_output_info(c.get_raw_conn(),
        output as xcb_randr_output_t, //1
        config_timestamp as ffi::xproto::xcb_timestamp_t); //2
    GetOutputInfoCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetOutputInfoUnchecked<'r> (c : &'r Connection,
                               output : Output,
                               config_timestamp : xproto::Timestamp) -> GetOutputInfoCookie<'r> {
  unsafe {
    let cookie = xcb_randr_get_output_info_unchecked(c.get_raw_conn(),
        output as xcb_randr_output_t, //1
        config_timestamp as ffi::xproto::xcb_timestamp_t); //2
    GetOutputInfoCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetOutputInfoReply {
  pub fn status(&mut self) -> u8 {
    unsafe { accessor!(status -> u8, (*self.base.reply)) }
  }

  pub fn timestamp(&mut self) -> xproto::Timestamp {
    unsafe { accessor!(timestamp -> xproto::Timestamp, (*self.base.reply)) }
  }

  pub fn crtc(&mut self) -> Crtc {
    unsafe { accessor!(crtc -> Crtc, (*self.base.reply)) }
  }

  pub fn mm_width(&mut self) -> u32 {
    unsafe { accessor!(mm_width -> u32, (*self.base.reply)) }
  }

  pub fn mm_height(&mut self) -> u32 {
    unsafe { accessor!(mm_height -> u32, (*self.base.reply)) }
  }

  pub fn connection(&mut self) -> u8 {
    unsafe { accessor!(connection -> u8, (*self.base.reply)) }
  }

  pub fn subpixel_order(&mut self) -> u8 {
    unsafe { accessor!(subpixel_order -> u8, (*self.base.reply)) }
  }

  pub fn num_preferred(&mut self) -> u16 {
    unsafe { accessor!(num_preferred -> u16, (*self.base.reply)) }
  }

  pub fn crtcs(&mut self) -> Vec<Output> {
    unsafe { accessor!(Output, xcb_randr_get_output_info_crtcs_length, xcb_randr_get_output_info_crtcs, (*self.base.reply)) }
  }

  pub fn modes(&mut self) -> Vec<Output> {
    unsafe { accessor!(Output, xcb_randr_get_output_info_modes_length, xcb_randr_get_output_info_modes, (*self.base.reply)) }
  }

  pub fn clones(&mut self) -> Vec<Output> {
    unsafe { accessor!(Output, xcb_randr_get_output_info_clones_length, xcb_randr_get_output_info_clones, (*self.base.reply)) }
  }

  pub fn name(&mut self) -> Vec<u8> {
    unsafe { accessor!(u8, xcb_randr_get_output_info_name_length, xcb_randr_get_output_info_name, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetOutputInfoCookie<'s>, mk_reply_xcb_randr_get_output_info_reply_t, GetOutputInfoReply, xcb_randr_get_output_info_reply);

pub struct ListOutputPropertiesReply { base:  base::Reply<xcb_randr_list_output_properties_reply_t> }
fn mk_reply_xcb_randr_list_output_properties_reply_t(reply:*mut xcb_randr_list_output_properties_reply_t) -> ListOutputPropertiesReply { ListOutputPropertiesReply { base : base::mk_reply(reply) } }
pub fn ListOutputProperties<'r> (c : &'r Connection,
                             output : Output) -> ListOutputPropertiesCookie<'r> {
  unsafe {
    let cookie = xcb_randr_list_output_properties(c.get_raw_conn(),
        output as xcb_randr_output_t); //1
    ListOutputPropertiesCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn ListOutputPropertiesUnchecked<'r> (c : &'r Connection,
                                      output : Output) -> ListOutputPropertiesCookie<'r> {
  unsafe {
    let cookie = xcb_randr_list_output_properties_unchecked(c.get_raw_conn(),
        output as xcb_randr_output_t); //1
    ListOutputPropertiesCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl ListOutputPropertiesReply {
  pub fn atoms(&mut self) -> Vec<xproto::Atom> {
    unsafe { accessor!(xproto::Atom, xcb_randr_list_output_properties_atoms_length, xcb_randr_list_output_properties_atoms, (*self.base.reply)) }
  }

}
impl_reply_cookie!(ListOutputPropertiesCookie<'s>, mk_reply_xcb_randr_list_output_properties_reply_t, ListOutputPropertiesReply, xcb_randr_list_output_properties_reply);

pub struct QueryOutputPropertyReply { base:  base::Reply<xcb_randr_query_output_property_reply_t> }
fn mk_reply_xcb_randr_query_output_property_reply_t(reply:*mut xcb_randr_query_output_property_reply_t) -> QueryOutputPropertyReply { QueryOutputPropertyReply { base : base::mk_reply(reply) } }
pub fn QueryOutputProperty<'r> (c : &'r Connection,
                            output : Output,
                            property : xproto::Atom) -> QueryOutputPropertyCookie<'r> {
  unsafe {
    let cookie = xcb_randr_query_output_property(c.get_raw_conn(),
        output as xcb_randr_output_t, //1
        property as ffi::xproto::xcb_atom_t); //2
    QueryOutputPropertyCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn QueryOutputPropertyUnchecked<'r> (c : &'r Connection,
                                     output : Output,
                                     property : xproto::Atom) -> QueryOutputPropertyCookie<'r> {
  unsafe {
    let cookie = xcb_randr_query_output_property_unchecked(c.get_raw_conn(),
        output as xcb_randr_output_t, //1
        property as ffi::xproto::xcb_atom_t); //2
    QueryOutputPropertyCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl QueryOutputPropertyReply {
  pub fn pending(&mut self) -> u8 {
    unsafe { accessor!(pending -> u8, (*self.base.reply)) }
  }

  pub fn range(&mut self) -> u8 {
    unsafe { accessor!(range -> u8, (*self.base.reply)) }
  }

  pub fn immutable(&mut self) -> u8 {
    unsafe { accessor!(immutable -> u8, (*self.base.reply)) }
  }

  pub fn validValues(&mut self) -> Vec<i32> {
    unsafe { accessor!(i32, xcb_randr_query_output_property_valid_values_length, xcb_randr_query_output_property_valid_values, (*self.base.reply)) }
  }

}
impl_reply_cookie!(QueryOutputPropertyCookie<'s>, mk_reply_xcb_randr_query_output_property_reply_t, QueryOutputPropertyReply, xcb_randr_query_output_property_reply);

pub fn ConfigureOutputPropertyChecked<'r> (c : &'r Connection,
                                       output : Output,
                                       property : xproto::Atom,
                                       pending : u8,
                                       range : u8,
                                       values : &[i32]) -> base::VoidCookie<'r> {
  unsafe {
    let values_len = values.len();
    let values_ptr = values.as_ptr();
    let cookie = xcb_randr_configure_output_property_checked(c.get_raw_conn(),
        output as xcb_randr_output_t, //1
        property as ffi::xproto::xcb_atom_t, //2
        pending as u8, //3
        range as u8, //4
        values_len as u32, //5
        values_ptr as *mut i32); //6
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn ConfigureOutputProperty<'r> (c : &'r Connection,
                                output : Output,
                                property : xproto::Atom,
                                pending : u8,
                                range : u8,
                                values : &[i32]) -> base::VoidCookie<'r> {
  unsafe {
    let values_len = values.len();
    let values_ptr = values.as_ptr();
    let cookie = xcb_randr_configure_output_property(c.get_raw_conn(),
        output as xcb_randr_output_t, //1
        property as ffi::xproto::xcb_atom_t, //2
        pending as u8, //3
        range as u8, //4
        values_len as u32, //5
        values_ptr as *mut i32); //6
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn ChangeOutputPropertyChecked<'r> (c : &'r Connection,
                                    output : Output,
                                    property : xproto::Atom,
                                    type_ : xproto::Atom,
                                    format : u8,
                                    mode : u8,
                                    data : &[u8]) -> base::VoidCookie<'r> {
  unsafe {
    let data_len = data.len();
    let data_ptr = data.as_ptr();
    let cookie = xcb_randr_change_output_property_checked(c.get_raw_conn(),
        output as xcb_randr_output_t, //1
        property as ffi::xproto::xcb_atom_t, //2
        type_ as ffi::xproto::xcb_atom_t, //3
        format as u8, //4
        mode as u8, //5
        data_len as u32, //6
        data_ptr as *mut c_void); //7
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn ChangeOutputProperty<'r> (c : &'r Connection,
                             output : Output,
                             property : xproto::Atom,
                             type_ : xproto::Atom,
                             format : u8,
                             mode : u8,
                             data : &[u8]) -> base::VoidCookie<'r> {
  unsafe {
    let data_len = data.len();
    let data_ptr = data.as_ptr();
    let cookie = xcb_randr_change_output_property(c.get_raw_conn(),
        output as xcb_randr_output_t, //1
        property as ffi::xproto::xcb_atom_t, //2
        type_ as ffi::xproto::xcb_atom_t, //3
        format as u8, //4
        mode as u8, //5
        data_len as u32, //6
        data_ptr as *mut c_void); //7
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn DeleteOutputPropertyChecked<'r> (c : &'r Connection,
                                    output : Output,
                                    property : xproto::Atom) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_randr_delete_output_property_checked(c.get_raw_conn(),
        output as xcb_randr_output_t, //1
        property as ffi::xproto::xcb_atom_t); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn DeleteOutputProperty<'r> (c : &'r Connection,
                             output : Output,
                             property : xproto::Atom) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_randr_delete_output_property(c.get_raw_conn(),
        output as xcb_randr_output_t, //1
        property as ffi::xproto::xcb_atom_t); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub struct GetOutputPropertyReply { base:  base::Reply<xcb_randr_get_output_property_reply_t> }
fn mk_reply_xcb_randr_get_output_property_reply_t(reply:*mut xcb_randr_get_output_property_reply_t) -> GetOutputPropertyReply { GetOutputPropertyReply { base : base::mk_reply(reply) } }
pub fn GetOutputProperty<'r> (c : &'r Connection,
                          output : Output,
                          property : xproto::Atom,
                          type_ : xproto::Atom,
                          long_offset : u32,
                          long_length : u32,
                          delete : u8,
                          pending : u8) -> GetOutputPropertyCookie<'r> {
  unsafe {
    let cookie = xcb_randr_get_output_property(c.get_raw_conn(),
        output as xcb_randr_output_t, //1
        property as ffi::xproto::xcb_atom_t, //2
        type_ as ffi::xproto::xcb_atom_t, //3
        long_offset as u32, //4
        long_length as u32, //5
        delete as u8, //6
        pending as u8); //7
    GetOutputPropertyCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetOutputPropertyUnchecked<'r> (c : &'r Connection,
                                   output : Output,
                                   property : xproto::Atom,
                                   type_ : xproto::Atom,
                                   long_offset : u32,
                                   long_length : u32,
                                   delete : u8,
                                   pending : u8) -> GetOutputPropertyCookie<'r> {
  unsafe {
    let cookie = xcb_randr_get_output_property_unchecked(c.get_raw_conn(),
        output as xcb_randr_output_t, //1
        property as ffi::xproto::xcb_atom_t, //2
        type_ as ffi::xproto::xcb_atom_t, //3
        long_offset as u32, //4
        long_length as u32, //5
        delete as u8, //6
        pending as u8); //7
    GetOutputPropertyCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetOutputPropertyReply {
  pub fn format(&mut self) -> u8 {
    unsafe { accessor!(format -> u8, (*self.base.reply)) }
  }

  pub fn type_(&mut self) -> xproto::Atom {
    unsafe { accessor!(type_ -> xproto::Atom, (*self.base.reply)) }
  }

  pub fn bytes_after(&mut self) -> u32 {
    unsafe { accessor!(bytes_after -> u32, (*self.base.reply)) }
  }

  pub fn data(&mut self) -> Vec<u8> {
    unsafe { accessor!(u8, xcb_randr_get_output_property_data_length, xcb_randr_get_output_property_data, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetOutputPropertyCookie<'s>, mk_reply_xcb_randr_get_output_property_reply_t, GetOutputPropertyReply, xcb_randr_get_output_property_reply);

pub fn CreateMode<'r> (c : &'r Connection,
                   window : xproto::Window,
                   mode_info : ModeInfo,
                   name : &str) -> CreateModeCookie<'r> {
  unsafe {
    let name = (name).as_bytes();
    let name_len = name.len();
    let name_ptr = name.as_ptr();
    let cookie = xcb_randr_create_mode(c.get_raw_conn(),
        window as ffi::xproto::xcb_window_t, //1
        mode_info.base.strct, //2
        name_len as u32, //3
        name_ptr as *mut c_char); //4
    CreateModeCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn CreateModeUnchecked<'r> (c : &'r Connection,
                            window : xproto::Window,
                            mode_info : ModeInfo,
                            name : &str) -> CreateModeCookie<'r> {
  unsafe {
    let name = (name).as_bytes();
    let name_len = name.len();
    let name_ptr = name.as_ptr();
    let cookie = xcb_randr_create_mode_unchecked(c.get_raw_conn(),
        window as ffi::xproto::xcb_window_t, //1
        mode_info.base.strct, //2
        name_len as u32, //3
        name_ptr as *mut c_char); //4
    CreateModeCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl CreateModeReply {
  pub fn mode(&mut self) -> Mode {
    unsafe { accessor!(mode -> Mode, (*self.base.reply)) }
  }

}
impl_reply_cookie!(CreateModeCookie<'s>, mk_reply_xcb_randr_create_mode_reply_t, CreateModeReply, xcb_randr_create_mode_reply);

pub fn DestroyModeChecked<'r> (c : &'r Connection,
                           mode : Mode) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_randr_destroy_mode_checked(c.get_raw_conn(),
        mode as xcb_randr_mode_t); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn DestroyMode<'r> (c : &'r Connection,
                    mode : Mode) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_randr_destroy_mode(c.get_raw_conn(),
        mode as xcb_randr_mode_t); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn AddOutputModeChecked<'r> (c : &'r Connection,
                             output : Output,
                             mode : Mode) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_randr_add_output_mode_checked(c.get_raw_conn(),
        output as xcb_randr_output_t, //1
        mode as xcb_randr_mode_t); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn AddOutputMode<'r> (c : &'r Connection,
                      output : Output,
                      mode : Mode) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_randr_add_output_mode(c.get_raw_conn(),
        output as xcb_randr_output_t, //1
        mode as xcb_randr_mode_t); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn DeleteOutputModeChecked<'r> (c : &'r Connection,
                                output : Output,
                                mode : Mode) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_randr_delete_output_mode_checked(c.get_raw_conn(),
        output as xcb_randr_output_t, //1
        mode as xcb_randr_mode_t); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn DeleteOutputMode<'r> (c : &'r Connection,
                         output : Output,
                         mode : Mode) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_randr_delete_output_mode(c.get_raw_conn(),
        output as xcb_randr_output_t, //1
        mode as xcb_randr_mode_t); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub struct GetCrtcInfoReply { base:  base::Reply<xcb_randr_get_crtc_info_reply_t> }
fn mk_reply_xcb_randr_get_crtc_info_reply_t(reply:*mut xcb_randr_get_crtc_info_reply_t) -> GetCrtcInfoReply { GetCrtcInfoReply { base : base::mk_reply(reply) } }
pub fn GetCrtcInfo<'r> (c : &'r Connection,
                    crtc : Crtc,
                    config_timestamp : xproto::Timestamp) -> GetCrtcInfoCookie<'r> {
  unsafe {
    let cookie = xcb_randr_get_crtc_info(c.get_raw_conn(),
        crtc as xcb_randr_crtc_t, //1
        config_timestamp as ffi::xproto::xcb_timestamp_t); //2
    GetCrtcInfoCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetCrtcInfoUnchecked<'r> (c : &'r Connection,
                             crtc : Crtc,
                             config_timestamp : xproto::Timestamp) -> GetCrtcInfoCookie<'r> {
  unsafe {
    let cookie = xcb_randr_get_crtc_info_unchecked(c.get_raw_conn(),
        crtc as xcb_randr_crtc_t, //1
        config_timestamp as ffi::xproto::xcb_timestamp_t); //2
    GetCrtcInfoCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetCrtcInfoReply {
  pub fn status(&mut self) -> u8 {
    unsafe { accessor!(status -> u8, (*self.base.reply)) }
  }

  pub fn timestamp(&mut self) -> xproto::Timestamp {
    unsafe { accessor!(timestamp -> xproto::Timestamp, (*self.base.reply)) }
  }

  pub fn x(&mut self) -> i16 {
    unsafe { accessor!(x -> i16, (*self.base.reply)) }
  }

  pub fn y(&mut self) -> i16 {
    unsafe { accessor!(y -> i16, (*self.base.reply)) }
  }

  pub fn width(&mut self) -> u16 {
    unsafe { accessor!(width -> u16, (*self.base.reply)) }
  }

  pub fn height(&mut self) -> u16 {
    unsafe { accessor!(height -> u16, (*self.base.reply)) }
  }

  pub fn mode(&mut self) -> Mode {
    unsafe { accessor!(mode -> Mode, (*self.base.reply)) }
  }

  pub fn rotation(&mut self) -> u16 {
    unsafe { accessor!(rotation -> u16, (*self.base.reply)) }
  }

  pub fn rotations(&mut self) -> u16 {
    unsafe { accessor!(rotations -> u16, (*self.base.reply)) }
  }

  pub fn outputs(&mut self) -> Vec<Output> {
    unsafe { accessor!(Output, xcb_randr_get_crtc_info_outputs_length, xcb_randr_get_crtc_info_outputs, (*self.base.reply)) }
  }

  pub fn possible(&mut self) -> Vec<Output> {
    unsafe { accessor!(Output, xcb_randr_get_crtc_info_possible_length, xcb_randr_get_crtc_info_possible, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetCrtcInfoCookie<'s>, mk_reply_xcb_randr_get_crtc_info_reply_t, GetCrtcInfoReply, xcb_randr_get_crtc_info_reply);

pub fn SetCrtcConfig<'r> (c : &'r Connection,
                      crtc : Crtc,
                      timestamp : xproto::Timestamp,
                      config_timestamp : xproto::Timestamp,
                      x : i16,
                      y : i16,
                      mode : Mode,
                      rotation : u16,
                      outputs : &[Output]) -> SetCrtcConfigCookie<'r> {
  unsafe {
    let outputs_len = outputs.len();
    let outputs_ptr = outputs.as_ptr();
    let cookie = xcb_randr_set_crtc_config(c.get_raw_conn(),
        crtc as xcb_randr_crtc_t, //1
        timestamp as ffi::xproto::xcb_timestamp_t, //2
        config_timestamp as ffi::xproto::xcb_timestamp_t, //3
        x as i16, //4
        y as i16, //5
        mode as xcb_randr_mode_t, //6
        rotation as u16, //7
        outputs_len as u32, //8
        outputs_ptr as *mut xcb_randr_output_t); //9
    SetCrtcConfigCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn SetCrtcConfigUnchecked<'r> (c : &'r Connection,
                               crtc : Crtc,
                               timestamp : xproto::Timestamp,
                               config_timestamp : xproto::Timestamp,
                               x : i16,
                               y : i16,
                               mode : Mode,
                               rotation : u16,
                               outputs : &[Output]) -> SetCrtcConfigCookie<'r> {
  unsafe {
    let outputs_len = outputs.len();
    let outputs_ptr = outputs.as_ptr();
    let cookie = xcb_randr_set_crtc_config_unchecked(c.get_raw_conn(),
        crtc as xcb_randr_crtc_t, //1
        timestamp as ffi::xproto::xcb_timestamp_t, //2
        config_timestamp as ffi::xproto::xcb_timestamp_t, //3
        x as i16, //4
        y as i16, //5
        mode as xcb_randr_mode_t, //6
        rotation as u16, //7
        outputs_len as u32, //8
        outputs_ptr as *mut xcb_randr_output_t); //9
    SetCrtcConfigCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl SetCrtcConfigReply {
  pub fn status(&mut self) -> u8 {
    unsafe { accessor!(status -> u8, (*self.base.reply)) }
  }

  pub fn timestamp(&mut self) -> xproto::Timestamp {
    unsafe { accessor!(timestamp -> xproto::Timestamp, (*self.base.reply)) }
  }

}
impl_reply_cookie!(SetCrtcConfigCookie<'s>, mk_reply_xcb_randr_set_crtc_config_reply_t, SetCrtcConfigReply, xcb_randr_set_crtc_config_reply);

pub fn GetCrtcGammaSize<'r> (c : &'r Connection,
                         crtc : Crtc) -> GetCrtcGammaSizeCookie<'r> {
  unsafe {
    let cookie = xcb_randr_get_crtc_gamma_size(c.get_raw_conn(),
        crtc as xcb_randr_crtc_t); //1
    GetCrtcGammaSizeCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetCrtcGammaSizeUnchecked<'r> (c : &'r Connection,
                                  crtc : Crtc) -> GetCrtcGammaSizeCookie<'r> {
  unsafe {
    let cookie = xcb_randr_get_crtc_gamma_size_unchecked(c.get_raw_conn(),
        crtc as xcb_randr_crtc_t); //1
    GetCrtcGammaSizeCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetCrtcGammaSizeReply {
  pub fn size(&mut self) -> u16 {
    unsafe { accessor!(size -> u16, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetCrtcGammaSizeCookie<'s>, mk_reply_xcb_randr_get_crtc_gamma_size_reply_t, GetCrtcGammaSizeReply, xcb_randr_get_crtc_gamma_size_reply);

pub struct GetCrtcGammaReply { base:  base::Reply<xcb_randr_get_crtc_gamma_reply_t> }
fn mk_reply_xcb_randr_get_crtc_gamma_reply_t(reply:*mut xcb_randr_get_crtc_gamma_reply_t) -> GetCrtcGammaReply { GetCrtcGammaReply { base : base::mk_reply(reply) } }
pub fn GetCrtcGamma<'r> (c : &'r Connection,
                     crtc : Crtc) -> GetCrtcGammaCookie<'r> {
  unsafe {
    let cookie = xcb_randr_get_crtc_gamma(c.get_raw_conn(),
        crtc as xcb_randr_crtc_t); //1
    GetCrtcGammaCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetCrtcGammaUnchecked<'r> (c : &'r Connection,
                              crtc : Crtc) -> GetCrtcGammaCookie<'r> {
  unsafe {
    let cookie = xcb_randr_get_crtc_gamma_unchecked(c.get_raw_conn(),
        crtc as xcb_randr_crtc_t); //1
    GetCrtcGammaCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetCrtcGammaReply {
  pub fn red(&mut self) -> Vec<u16> {
    unsafe { accessor!(u16, xcb_randr_get_crtc_gamma_red_length, xcb_randr_get_crtc_gamma_red, (*self.base.reply)) }
  }

  pub fn green(&mut self) -> Vec<u16> {
    unsafe { accessor!(u16, xcb_randr_get_crtc_gamma_green_length, xcb_randr_get_crtc_gamma_green, (*self.base.reply)) }
  }

  pub fn blue(&mut self) -> Vec<u16> {
    unsafe { accessor!(u16, xcb_randr_get_crtc_gamma_blue_length, xcb_randr_get_crtc_gamma_blue, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetCrtcGammaCookie<'s>, mk_reply_xcb_randr_get_crtc_gamma_reply_t, GetCrtcGammaReply, xcb_randr_get_crtc_gamma_reply);

pub fn SetCrtcGammaChecked<'r> (c : &'r Connection,
                            crtc : Crtc,
                            red : &[u16],
                            green : &[u16],
                            blue : &[u16]) -> base::VoidCookie<'r> {
  unsafe {
    let red_len = red.len();
    let red_ptr = red.as_ptr();
    let green_ptr = green.as_ptr();
    let blue_ptr = blue.as_ptr();
    let cookie = xcb_randr_set_crtc_gamma_checked(c.get_raw_conn(),
        crtc as xcb_randr_crtc_t, //1
        red_len as u16, //2
        red_ptr as *mut u16, //3
        green_ptr as *mut u16, //4
        blue_ptr as *mut u16); //5
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn SetCrtcGamma<'r> (c : &'r Connection,
                     crtc : Crtc,
                     red : &[u16],
                     green : &[u16],
                     blue : &[u16]) -> base::VoidCookie<'r> {
  unsafe {
    let red_len = red.len();
    let red_ptr = red.as_ptr();
    let green_ptr = green.as_ptr();
    let blue_ptr = blue.as_ptr();
    let cookie = xcb_randr_set_crtc_gamma(c.get_raw_conn(),
        crtc as xcb_randr_crtc_t, //1
        red_len as u16, //2
        red_ptr as *mut u16, //3
        green_ptr as *mut u16, //4
        blue_ptr as *mut u16); //5
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub struct GetScreenResourcesCurrentReply { base:  base::Reply<xcb_randr_get_screen_resources_current_reply_t> }
fn mk_reply_xcb_randr_get_screen_resources_current_reply_t(reply:*mut xcb_randr_get_screen_resources_current_reply_t) -> GetScreenResourcesCurrentReply { GetScreenResourcesCurrentReply { base : base::mk_reply(reply) } }
pub fn GetScreenResourcesCurrent<'r> (c : &'r Connection,
                                  window : xproto::Window) -> GetScreenResourcesCurrentCookie<'r> {
  unsafe {
    let cookie = xcb_randr_get_screen_resources_current(c.get_raw_conn(),
        window as ffi::xproto::xcb_window_t); //1
    GetScreenResourcesCurrentCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetScreenResourcesCurrentUnchecked<'r> (c : &'r Connection,
                                           window : xproto::Window) -> GetScreenResourcesCurrentCookie<'r> {
  unsafe {
    let cookie = xcb_randr_get_screen_resources_current_unchecked(c.get_raw_conn(),
        window as ffi::xproto::xcb_window_t); //1
    GetScreenResourcesCurrentCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetScreenResourcesCurrentReply {
  pub fn timestamp(&mut self) -> xproto::Timestamp {
    unsafe { accessor!(timestamp -> xproto::Timestamp, (*self.base.reply)) }
  }

  pub fn config_timestamp(&mut self) -> xproto::Timestamp {
    unsafe { accessor!(config_timestamp -> xproto::Timestamp, (*self.base.reply)) }
  }

  pub fn crtcs(&mut self) -> Vec<u32> {
    unsafe { accessor!(u32, xcb_randr_get_screen_resources_current_crtcs_length, xcb_randr_get_screen_resources_current_crtcs, (*self.base.reply)) }
  }

  pub fn outputs(&mut self) -> Vec<u32> {
    unsafe { accessor!(u32, xcb_randr_get_screen_resources_current_outputs_length, xcb_randr_get_screen_resources_current_outputs, (*self.base.reply)) }
  }

  pub fn modes(&mut self) -> ModeInfoIterator {
    unsafe { accessor!(ModeInfoIterator, xcb_randr_get_screen_resources_current_modes_iterator, (*self.base.reply)) }
  }

  pub fn names(&mut self) -> Vec<u8> {
    unsafe { accessor!(u8, xcb_randr_get_screen_resources_current_names_length, xcb_randr_get_screen_resources_current_names, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetScreenResourcesCurrentCookie<'s>, mk_reply_xcb_randr_get_screen_resources_current_reply_t, GetScreenResourcesCurrentReply, xcb_randr_get_screen_resources_current_reply);

pub fn SetCrtcTransformChecked<'r> (c : &'r Connection,
                                crtc : Crtc,
                                transform : render::Transform,
                                filter_name : &str,
                                filter_params : &[render::Fixed]) -> base::VoidCookie<'r> {
  unsafe {
    let filter_name = (filter_name).as_bytes();
    let filter_name_len = filter_name.len();
    let filter_name_ptr = filter_name.as_ptr();
    let filter_params_len = filter_params.len();
    let filter_params_ptr = filter_params.as_ptr();
    let cookie = xcb_randr_set_crtc_transform_checked(c.get_raw_conn(),
        crtc as xcb_randr_crtc_t, //1
        transform.base.strct, //2
        filter_name_len as u16, //3
        filter_name_ptr as *mut c_char, //4
        filter_params_len as u32, //5
        filter_params_ptr as *mut ffi::render::xcb_render_fixed_t); //6
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn SetCrtcTransform<'r> (c : &'r Connection,
                         crtc : Crtc,
                         transform : render::Transform,
                         filter_name : &str,
                         filter_params : &[render::Fixed]) -> base::VoidCookie<'r> {
  unsafe {
    let filter_name = (filter_name).as_bytes();
    let filter_name_len = filter_name.len();
    let filter_name_ptr = filter_name.as_ptr();
    let filter_params_len = filter_params.len();
    let filter_params_ptr = filter_params.as_ptr();
    let cookie = xcb_randr_set_crtc_transform(c.get_raw_conn(),
        crtc as xcb_randr_crtc_t, //1
        transform.base.strct, //2
        filter_name_len as u16, //3
        filter_name_ptr as *mut c_char, //4
        filter_params_len as u32, //5
        filter_params_ptr as *mut ffi::render::xcb_render_fixed_t); //6
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub struct GetCrtcTransformReply { base:  base::Reply<xcb_randr_get_crtc_transform_reply_t> }
fn mk_reply_xcb_randr_get_crtc_transform_reply_t(reply:*mut xcb_randr_get_crtc_transform_reply_t) -> GetCrtcTransformReply { GetCrtcTransformReply { base : base::mk_reply(reply) } }
pub fn GetCrtcTransform<'r> (c : &'r Connection,
                         crtc : Crtc) -> GetCrtcTransformCookie<'r> {
  unsafe {
    let cookie = xcb_randr_get_crtc_transform(c.get_raw_conn(),
        crtc as xcb_randr_crtc_t); //1
    GetCrtcTransformCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetCrtcTransformUnchecked<'r> (c : &'r Connection,
                                  crtc : Crtc) -> GetCrtcTransformCookie<'r> {
  unsafe {
    let cookie = xcb_randr_get_crtc_transform_unchecked(c.get_raw_conn(),
        crtc as xcb_randr_crtc_t); //1
    GetCrtcTransformCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetCrtcTransformReply {
  pub fn pending_transform(&self) -> render::Transform {
    unsafe { mem::transmute((*self.base.reply).pending_transform) }
  }
  pub fn has_transforms(&mut self) -> u8 {
    unsafe { accessor!(has_transforms -> u8, (*self.base.reply)) }
  }

  pub fn current_transform(&self) -> render::Transform {
    unsafe { mem::transmute((*self.base.reply).current_transform) }
  }
  pub fn pending_filter_name(&mut self) -> String {
    unsafe { accessor!(str, xcb_randr_get_crtc_transform_pending_filter_name_length, xcb_randr_get_crtc_transform_pending_filter_name, (*self.base.reply)) }
  }

  pub fn pending_params(&mut self) -> Vec<render::Fixed> {
    unsafe { accessor!(render::Fixed, xcb_randr_get_crtc_transform_pending_params_length, xcb_randr_get_crtc_transform_pending_params, (*self.base.reply)) }
  }

  pub fn current_filter_name(&mut self) -> String {
    unsafe { accessor!(str, xcb_randr_get_crtc_transform_current_filter_name_length, xcb_randr_get_crtc_transform_current_filter_name, (*self.base.reply)) }
  }

  pub fn current_params(&mut self) -> Vec<render::Fixed> {
    unsafe { accessor!(render::Fixed, xcb_randr_get_crtc_transform_current_params_length, xcb_randr_get_crtc_transform_current_params, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetCrtcTransformCookie<'s>, mk_reply_xcb_randr_get_crtc_transform_reply_t, GetCrtcTransformReply, xcb_randr_get_crtc_transform_reply);

pub fn GetPanning<'r> (c : &'r Connection,
                   crtc : Crtc) -> GetPanningCookie<'r> {
  unsafe {
    let cookie = xcb_randr_get_panning(c.get_raw_conn(),
        crtc as xcb_randr_crtc_t); //1
    GetPanningCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetPanningUnchecked<'r> (c : &'r Connection,
                            crtc : Crtc) -> GetPanningCookie<'r> {
  unsafe {
    let cookie = xcb_randr_get_panning_unchecked(c.get_raw_conn(),
        crtc as xcb_randr_crtc_t); //1
    GetPanningCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetPanningReply {
  pub fn status(&mut self) -> u8 {
    unsafe { accessor!(status -> u8, (*self.base.reply)) }
  }

  pub fn timestamp(&mut self) -> xproto::Timestamp {
    unsafe { accessor!(timestamp -> xproto::Timestamp, (*self.base.reply)) }
  }

  pub fn left(&mut self) -> u16 {
    unsafe { accessor!(left -> u16, (*self.base.reply)) }
  }

  pub fn top(&mut self) -> u16 {
    unsafe { accessor!(top -> u16, (*self.base.reply)) }
  }

  pub fn width(&mut self) -> u16 {
    unsafe { accessor!(width -> u16, (*self.base.reply)) }
  }

  pub fn height(&mut self) -> u16 {
    unsafe { accessor!(height -> u16, (*self.base.reply)) }
  }

  pub fn track_left(&mut self) -> u16 {
    unsafe { accessor!(track_left -> u16, (*self.base.reply)) }
  }

  pub fn track_top(&mut self) -> u16 {
    unsafe { accessor!(track_top -> u16, (*self.base.reply)) }
  }

  pub fn track_width(&mut self) -> u16 {
    unsafe { accessor!(track_width -> u16, (*self.base.reply)) }
  }

  pub fn track_height(&mut self) -> u16 {
    unsafe { accessor!(track_height -> u16, (*self.base.reply)) }
  }

  pub fn border_left(&mut self) -> i16 {
    unsafe { accessor!(border_left -> i16, (*self.base.reply)) }
  }

  pub fn border_top(&mut self) -> i16 {
    unsafe { accessor!(border_top -> i16, (*self.base.reply)) }
  }

  pub fn border_right(&mut self) -> i16 {
    unsafe { accessor!(border_right -> i16, (*self.base.reply)) }
  }

  pub fn border_bottom(&mut self) -> i16 {
    unsafe { accessor!(border_bottom -> i16, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetPanningCookie<'s>, mk_reply_xcb_randr_get_panning_reply_t, GetPanningReply, xcb_randr_get_panning_reply);

pub fn SetPanning<'r> (c : &'r Connection,
                   crtc : Crtc,
                   timestamp : xproto::Timestamp,
                   left : u16,
                   top : u16,
                   width : u16,
                   height : u16,
                   track_left : u16,
                   track_top : u16,
                   track_width : u16,
                   track_height : u16,
                   border_left : i16,
                   border_top : i16,
                   border_right : i16,
                   border_bottom : i16) -> SetPanningCookie<'r> {
  unsafe {
    let cookie = xcb_randr_set_panning(c.get_raw_conn(),
        crtc as xcb_randr_crtc_t, //1
        timestamp as ffi::xproto::xcb_timestamp_t, //2
        left as u16, //3
        top as u16, //4
        width as u16, //5
        height as u16, //6
        track_left as u16, //7
        track_top as u16, //8
        track_width as u16, //9
        track_height as u16, //10
        border_left as i16, //11
        border_top as i16, //12
        border_right as i16, //13
        border_bottom as i16); //14
    SetPanningCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn SetPanningUnchecked<'r> (c : &'r Connection,
                            crtc : Crtc,
                            timestamp : xproto::Timestamp,
                            left : u16,
                            top : u16,
                            width : u16,
                            height : u16,
                            track_left : u16,
                            track_top : u16,
                            track_width : u16,
                            track_height : u16,
                            border_left : i16,
                            border_top : i16,
                            border_right : i16,
                            border_bottom : i16) -> SetPanningCookie<'r> {
  unsafe {
    let cookie = xcb_randr_set_panning_unchecked(c.get_raw_conn(),
        crtc as xcb_randr_crtc_t, //1
        timestamp as ffi::xproto::xcb_timestamp_t, //2
        left as u16, //3
        top as u16, //4
        width as u16, //5
        height as u16, //6
        track_left as u16, //7
        track_top as u16, //8
        track_width as u16, //9
        track_height as u16, //10
        border_left as i16, //11
        border_top as i16, //12
        border_right as i16, //13
        border_bottom as i16); //14
    SetPanningCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl SetPanningReply {
  pub fn status(&mut self) -> u8 {
    unsafe { accessor!(status -> u8, (*self.base.reply)) }
  }

  pub fn timestamp(&mut self) -> xproto::Timestamp {
    unsafe { accessor!(timestamp -> xproto::Timestamp, (*self.base.reply)) }
  }

}
impl_reply_cookie!(SetPanningCookie<'s>, mk_reply_xcb_randr_set_panning_reply_t, SetPanningReply, xcb_randr_set_panning_reply);

pub fn SetOutputPrimaryChecked<'r> (c : &'r Connection,
                                window : xproto::Window,
                                output : Output) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_randr_set_output_primary_checked(c.get_raw_conn(),
        window as ffi::xproto::xcb_window_t, //1
        output as xcb_randr_output_t); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn SetOutputPrimary<'r> (c : &'r Connection,
                         window : xproto::Window,
                         output : Output) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_randr_set_output_primary(c.get_raw_conn(),
        window as ffi::xproto::xcb_window_t, //1
        output as xcb_randr_output_t); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetOutputPrimary<'r> (c : &'r Connection,
                         window : xproto::Window) -> GetOutputPrimaryCookie<'r> {
  unsafe {
    let cookie = xcb_randr_get_output_primary(c.get_raw_conn(),
        window as ffi::xproto::xcb_window_t); //1
    GetOutputPrimaryCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetOutputPrimaryUnchecked<'r> (c : &'r Connection,
                                  window : xproto::Window) -> GetOutputPrimaryCookie<'r> {
  unsafe {
    let cookie = xcb_randr_get_output_primary_unchecked(c.get_raw_conn(),
        window as ffi::xproto::xcb_window_t); //1
    GetOutputPrimaryCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetOutputPrimaryReply {
  pub fn output(&mut self) -> Output {
    unsafe { accessor!(output -> Output, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetOutputPrimaryCookie<'s>, mk_reply_xcb_randr_get_output_primary_reply_t, GetOutputPrimaryReply, xcb_randr_get_output_primary_reply);


impl ScreenChangeNotifyEvent {
  pub fn rotation(&mut self) -> u8 {
    unsafe { accessor!(rotation -> u8, (*self.base.event)) }
  }

  pub fn timestamp(&mut self) -> xproto::Timestamp {
    unsafe { accessor!(timestamp -> xproto::Timestamp, (*self.base.event)) }
  }

  pub fn config_timestamp(&mut self) -> xproto::Timestamp {
    unsafe { accessor!(config_timestamp -> xproto::Timestamp, (*self.base.event)) }
  }

  pub fn root(&mut self) -> xproto::Window {
    unsafe { accessor!(root -> xproto::Window, (*self.base.event)) }
  }

  pub fn request_window(&mut self) -> xproto::Window {
    unsafe { accessor!(request_window -> xproto::Window, (*self.base.event)) }
  }

  pub fn sizeID(&mut self) -> u16 {
    unsafe { accessor!(sizeID -> u16, (*self.base.event)) }
  }

  pub fn subpixel_order(&mut self) -> u16 {
    unsafe { accessor!(subpixel_order -> u16, (*self.base.event)) }
  }

  pub fn width(&mut self) -> u16 {
    unsafe { accessor!(width -> u16, (*self.base.event)) }
  }

  pub fn height(&mut self) -> u16 {
    unsafe { accessor!(height -> u16, (*self.base.event)) }
  }

  pub fn mwidth(&mut self) -> u16 {
    unsafe { accessor!(mwidth -> u16, (*self.base.event)) }
  }

  pub fn mheight(&mut self) -> u16 {
    unsafe { accessor!(mheight -> u16, (*self.base.event)) }
  }

  pub fn new(rotation : u8,
         timestamp : xproto::Timestamp,
         config_timestamp : xproto::Timestamp,
         root : xproto::Window,
         request_window : xproto::Window,
         sizeID : u16,
         subpixel_order : u16,
         width : u16,
         height : u16,
         mwidth : u16,
         mheight : u16) -> ScreenChangeNotifyEvent {
    unsafe {
      let raw = malloc(32 as size_t) as *mut xcb_randr_screen_change_notify_event_t;
      (*raw).rotation = rotation;
      (*raw).timestamp = timestamp;
      (*raw).config_timestamp = config_timestamp;
      (*raw).root = root;
      (*raw).request_window = request_window;
      (*raw).sizeID = sizeID;
      (*raw).subpixel_order = subpixel_order;
      (*raw).width = width;
      (*raw).height = height;
      (*raw).mwidth = mwidth;
      (*raw).mheight = mheight;
      ScreenChangeNotifyEvent { base : Event { event : raw as *mut xcb_randr_screen_change_notify_event_t }}
    }
  }
}

impl CrtcChange {
  pub fn timestamp(&mut self) -> xproto::Timestamp {
    unsafe { accessor!(timestamp -> xproto::Timestamp, self.base.strct) }
  }

  pub fn window(&mut self) -> xproto::Window {
    unsafe { accessor!(window -> xproto::Window, self.base.strct) }
  }

  pub fn crtc(&mut self) -> Crtc {
    unsafe { accessor!(crtc -> Crtc, self.base.strct) }
  }

  pub fn mode(&mut self) -> Mode {
    unsafe { accessor!(mode -> Mode, self.base.strct) }
  }

  pub fn rotation(&mut self) -> u16 {
    unsafe { accessor!(rotation -> u16, self.base.strct) }
  }

  pub fn x(&mut self) -> i16 {
    unsafe { accessor!(x -> i16, self.base.strct) }
  }

  pub fn y(&mut self) -> i16 {
    unsafe { accessor!(y -> i16, self.base.strct) }
  }

  pub fn width(&mut self) -> u16 {
    unsafe { accessor!(width -> u16, self.base.strct) }
  }

  pub fn height(&mut self) -> u16 {
    unsafe { accessor!(height -> u16, self.base.strct) }
  }

}

impl Iterator for CrtcChangeIterator {
    type Item = CrtcChange;
    fn next(&mut self) -> Option<CrtcChange> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_randr_crtc_change_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_randr_crtc_change_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct OutputChange {pub base : base::Struct<xcb_randr_output_change_t> }


impl OutputChange {
  pub fn timestamp(&mut self) -> xproto::Timestamp {
    unsafe { accessor!(timestamp -> xproto::Timestamp, self.base.strct) }
  }

  pub fn config_timestamp(&mut self) -> xproto::Timestamp {
    unsafe { accessor!(config_timestamp -> xproto::Timestamp, self.base.strct) }
  }

  pub fn window(&mut self) -> xproto::Window {
    unsafe { accessor!(window -> xproto::Window, self.base.strct) }
  }

  pub fn output(&mut self) -> Output {
    unsafe { accessor!(output -> Output, self.base.strct) }
  }

  pub fn crtc(&mut self) -> Crtc {
    unsafe { accessor!(crtc -> Crtc, self.base.strct) }
  }

  pub fn mode(&mut self) -> Mode {
    unsafe { accessor!(mode -> Mode, self.base.strct) }
  }

  pub fn rotation(&mut self) -> u16 {
    unsafe { accessor!(rotation -> u16, self.base.strct) }
  }

  pub fn connection(&mut self) -> u8 {
    unsafe { accessor!(connection -> u8, self.base.strct) }
  }

  pub fn subpixel_order(&mut self) -> u8 {
    unsafe { accessor!(subpixel_order -> u8, self.base.strct) }
  }

}

impl Iterator for OutputChangeIterator {
    type Item = OutputChange;
    fn next(&mut self) -> Option<OutputChange> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_randr_output_change_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_randr_output_change_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct OutputProperty {pub base : base::Struct<xcb_randr_output_property_t> }


impl OutputProperty {
  pub fn window(&mut self) -> xproto::Window {
    unsafe { accessor!(window -> xproto::Window, self.base.strct) }
  }

  pub fn output(&mut self) -> Output {
    unsafe { accessor!(output -> Output, self.base.strct) }
  }

  pub fn atom(&mut self) -> xproto::Atom {
    unsafe { accessor!(atom -> xproto::Atom, self.base.strct) }
  }

  pub fn timestamp(&mut self) -> xproto::Timestamp {
    unsafe { accessor!(timestamp -> xproto::Timestamp, self.base.strct) }
  }

  pub fn status(&mut self) -> u8 {
    unsafe { accessor!(status -> u8, self.base.strct) }
  }

}

impl Iterator for OutputPropertyIterator {
    type Item = OutputProperty;
    fn next(&mut self) -> Option<OutputProperty> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_randr_output_property_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_randr_output_property_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct NotifyData {pub base : base::Struct<xcb_randr_notify_data_t>}

impl Iterator for NotifyDataIterator {
    type Item = NotifyData;
    fn next(&mut self) -> Option<NotifyData> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_randr_notify_data_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_randr_notify_data_next(iter);
            Some(mem::transmute(*data))
        }
    }
}


impl NotifyEvent {
  pub fn subCode(&mut self) -> u8 {
    unsafe { accessor!(subCode -> u8, (*self.base.event)) }
  }

  pub fn u(&self) -> NotifyData {
    unsafe { mem::transmute((*self.base.event).u) }
  }
  pub fn new(subCode : u8,
         u : NotifyData) -> NotifyEvent {
    unsafe {
      let raw = malloc(32 as size_t) as *mut xcb_randr_notify_event_t;
      (*raw).subCode = subCode;
      (*raw).u = u.base.strct;
      NotifyEvent { base : Event { event : raw as *mut xcb_randr_notify_event_t }}
    }
  }
}

